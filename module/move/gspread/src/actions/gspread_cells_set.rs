//!
//! Set command -> set specified values in specified columns in specified row
//! 

mod private
{
  use crate::*;
  use actions::gspread::
  {
    Error,
    Result
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

  /// Structure for --json value
  #[ derive( Deserialize, Debug ) ]
  struct ParsedJson
  {
    #[ serde( flatten ) ]
    columns : HashMap< String, String >
  }
  
  /// Parse --json value
  fn parse_json
  (
    json_str : &str
  ) -> Result< ParsedJson > 
  {
    serde_json::from_str::< ParsedJson >( json_str ).map_err
    (
      | error | Error::InvalidJSON( format!( "Failed to parse JSON: {}", error ) )
    )
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

  fn is_all_uppercase_letters
  (
    s : &str
  ) -> Result< () >
  {
    if s.chars().all( | c | c.is_ascii_uppercase() ) 
    {
      Ok( () )
    } 
    else 
    {
      Err
      ( 
        Error::ParseError( format!( "The string '{}' contains invalid characters. Only uppercase letters (A-Z) are allowed.", s ) ) 
      )
    }
  }

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

    let mut pairs = parse_json( json_str )?;

    let row_id = pairs
    .columns
    .remove( select_row_by_key )
    .ok_or_else( || Error::ParseError( format!( "Key '{}' not found in JSON", select_row_by_key ) ) )?;


    let mut value_ranges= Vec::new();

    for ( key, value ) in pairs.columns.into_iter()
    {
      is_all_uppercase_letters( key.as_str() )?;
      value_ranges.push
      ( 
        ValueRange
        { 
          range: Some( format!( "{}!{}{}", table_name, key, row_id ) ), 
          values: Some( vec![ vec![ JsonValue::String( value.to_string() ) ] ] ),
          ..Default::default() 
        } 
      );
    };

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
  }
  
}

crate::mod_interface!
{
  own use action;
}