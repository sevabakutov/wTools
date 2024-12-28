//!
//! Set command -> set specified values in specified columns in specified row
//! 

mod private
{
  use crate::*;
  use actions::gspread::
  {
    Error,
    Result,
    update_row
  };
  use google_sheets4::api::
  {
    BatchUpdateValuesRequest, 
    ValueRange
  };
  use ser::
  { 
    Deserialize, 
    JsonValue 
  };
  use std::collections::HashMap;

  /// TODO: Add documentation
  #[ derive( Deserialize, Debug ) ]
  struct ParsedJson
  {
    #[ serde( flatten ) ]
    columns : HashMap< String, String >
  }
  
  /// TODO: Add documentation
  fn parse_json
  (
    json_str : &str
  ) -> Result< ParsedJson > 
  {
    let parsed_json = serde_json::from_str::< HashMap< String, String > >( json_str )
    .map_err( | error | format!( "Failed to parse JSON: {}", error ) )
    .and_then
    ( | map | 
      {
        for ( col_name, _value ) in &map 
        {
          if !col_name.chars().all( | c | c.is_alphabetic() && c.is_uppercase() ) 
          {
            return Err
            ( 
              format!( "Invalid column name: {}. Allowed only uppercase alphabetic letters (A-Z)", col_name )
            );
          }
        }
        Ok( map )
        });
    
    match parsed_json
    {
      Ok( map ) => Ok( ParsedJson{ columns: map } ),
      Err( error ) => Err( Error::InvalidJSON( error ) )
    }
    
  }

  /// Check availables keys.
  /// Available keys: "id" -> row's id
  fn check_select_row_by_key
  (
    key : &str
  ) -> Result< () > 
  {
    let keys = vec![ "id" ];
    if keys.contains( &key )
    {
      Ok( () )
    } 
    else 
    {
      Err
      ( 
        Error::ParseError( format!( "Invalid select_row_by_key: '{}'. Allowed keys: {:?}", key, keys ) ) 
      )
    }
  }

  
  // fn check_col_names
  // (
  //   s : &str
  // ) -> Result< () >
  // {
  //   if s.chars().all( | c | c.is_ascii_uppercase() ) 
  //   {
  //     Ok( () )
  //   } 
  //   else 
  //   {
  //     Err
  //     ( 
  //       Error::ParseError( format!( "The string '{}' contains invalid characters. Only uppercase letters (A-Z) are allowed.", s ) ) 
  //     )
  //   }
  // }

  pub async fn action
  (
    hub : &SheetsType,
    select_row_by_key : &str,
    json_str : &str,
    spreadsheet_id : &str,
    table_name : &str
  ) -> Result< i32 >
  {
    check_select_row_by_key( select_row_by_key )?;

    match parse_json( json_str )
    {
      Ok( parsed_json ) => 
      match update_row( parsed_json.columns )
      {
        Ok( ( _, values ) ) => 
        {
          match values.total_updated_cells 
          {
            Some( val ) => Ok( val ),
            None => Ok( 0 ),
          }
        }
      }
      Err( error ) => Err( error ),
    }

    let secret = Secret::read();
    let hub = hub( &secret ).await.context( "Failed to create a hub" );

    let req = BatchUpdateValuesRequest
    {
      value_input_option: Some( "USER_ENTERED".to_string() ),
      data: Some( value_ranges ),
      include_values_in_response: Some( true ),
      ..Default::default()
    };

    match hub
    .spreadsheets()
    .values_batch_update( req, spreadsheet_id )
    .doit()
    .await
    {
      Ok( ( _, values ) ) => 
      {
        match values.total_updated_cells 
        {
          Some( val ) => Ok( val ),
          None => Ok( 0 ),
        }
      }
      Err( error ) => Err( Error::ApiError( error ) ),
    }

    // let row_id = pairs
    // .columns
    // .remove( select_row_by_key )
    // .ok_or_else( || Error::ParseError( format!( "Key '{}' not found in JSON", select_row_by_key ) ) )?;

    // wrap_row()
    

    // // Also read about it and change values
    // let req = BatchUpdateValuesRequest
    // {
    //   value_input_option: Some( "USER_ENTERED".to_string() ),
    //   data: Some( value_ranges ),
    //   include_values_in_response: Some( true ),
    //   ..Default::default()
    // };

    // match hub
    // .spreadsheets()
    // .values_batch_update( req, spreadsheet_id )
    // .doit()
    // .await
    // {
    //   Ok( ( _, values ) ) => 
    //   {
    //     match values.total_updated_cells 
    //     {
    //       Some( val ) => Ok( val ),
    //       None => Ok( 0 ),
    //     }
    //   }
    //   Err( error ) => Err( Error::ApiError( error ) ),
    // }
  }
  
}

crate::mod_interface!
{
  own use action;
}