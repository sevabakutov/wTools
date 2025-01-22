//!
//! Google Sheets API actions.
//!
//! This module also contains the definition of Google Sheets Error.
//!

mod private
{
  use regex::Regex;
use serde_json::json;
  use std::collections::HashMap;

  use crate::*;
  use gcore::error::{ Error, Result };
  use gcore::client::
  {
    Client, 
    Dimension, 
    ValueRange, 
    ValueInputOption, 
    ValueRenderOption, 
    UpdateValuesResponse, 
    ValuesAppendResponse,
    BatchUpdateValuesRequest, 
    BatchUpdateValuesResponse, 
  };

  /// # check_variant
  /// 
  /// Checks if passed variant is correct.
  /// 
  /// ## Returns:
  ///  - `Result< () >`
  /// 
  /// ## Errors:
  /// 
  /// Can occur if passed varaint is not alllowed.
  pub fn check_variant
  ( 
    variant: &str,
    allowed : Vec< &str > 
  ) -> Result< () >
  {
    if allowed.contains( &variant )
    {
      Ok( () )
    }
    else
    {
      Err
      ( 
        Error::ParseError( format!( "Not suchvariant: {}. Allowed: {:?}", variant, allowed ) )
      )
    }
  }

  /// # parse_json
  /// 
  /// Parse passed json to HashMap< String, serde_json::Value >
  /// 
  /// ## Returns
  ///  - `Result< HashMap< String, serde_json::Value > >`
  /// 
  /// ## Errors
  /// 
  /// Can occur if the passed json is not valid.
  pub fn parse_json
  ( 
    json_str : &str 
  ) -> Result< HashMap< String, serde_json::Value > >
  {
    let parsed_json: HashMap< String, serde_json::Value > = serde_json::from_str( json_str )
    .map_err( | error | Error::InvalidJSON( format!( "Failed to parse JSON: {}", error ) ) )?;

    Ok( parsed_json )
  }
  
  /// # `get_spreadsheet_id_from_url`
  ///
  /// Retrieves the spreadsheet ID from the provided Google Sheets URL.
  ///
  /// ## Parameters:
  /// - `url`:  
  ///   A `&str` containing the full URL of the Google spreadsheet.  
  ///
  /// ## Returns:
  /// - `Result<&str>`
  ///
  /// ## Errors:
  /// - `Error::InvalidUrl`:  
  ///   Occurs when the URL does not match the expected format.  
  ///   Suggests copying the entire URL directly from the browser.
  pub fn get_spreadsheet_id_from_url
  (
    url : &str
  ) -> Result< &str >
  {

    let re = Regex::new( r"d/([^/]+)/edit" ).unwrap();
    if let Some( captures ) = re.captures( url )
    {
      if let Some( id ) = captures.get( 1 )
      {
        return Ok( id.as_str() );
      }
    }

    Err
    ( 
      Error::InvalidUrl( "Wrong url format.\nFix: copy sheet's the whole url from your browser. Usage: --url '<your copied url>'".to_string() ) 
    )
  }

  /// # `update_row`
  ///
  /// Updates a specific row in a Google Sheet with the provided values.
  ///
  /// ## Parameters:
  /// - `client`:  
  ///   A reference to the [`Client`] client configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:  
  ///   A `&str` specifying the name of the sheet.
  /// - `row_key`:  
  ///   A `serde_json::Value` representing the row's key (e.g., the row index).
  /// - `row_key_val`:  
  ///   A `HashMap<String, serde_json::Value>` where:  
  ///   - Key: The column name (e.g., "A", "B").  
  ///   - Value: The new value to set in the corresponding cell.
  ///
  /// ## Returns:
  /// - Result<[`BatchUpdateValuesResponse`]>
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, e.g., due to invalid input or insufficient permissions.
  pub async fn update_row
  (
    client : &Client<'_>,
    spreadsheet_id : &str,
    sheet_name : &str,
    row_key : serde_json::Value,
    row_key_val : HashMap< String, serde_json::Value >
  ) -> Result< BatchUpdateValuesResponse >
  {
    let mut value_ranges = Vec::with_capacity( row_key_val.len() );

    for ( col_name, value ) in row_key_val 
    {
      value_ranges.push
      (
        ValueRange
        { 
          major_dimension: Some( Dimension::Row ),
          values: Some( vec![ vec![ value ] ] ),
          range: Some( format!( "{}!{}{}", sheet_name, col_name, row_key ) ),
        }
      )
    }

    let request = BatchUpdateValuesRequest
    {
      data : value_ranges,
      value_input_option : ValueInputOption::UserEntered,
      include_values_in_response : Some( true ),
      response_value_render_option : Some( ValueRenderOption::FormattedValue ),
      response_date_time_render_option : Default::default()
    };

    match client
    .spreadsheet()
    .values_batch_update( spreadsheet_id, request )
    .doit()
    .await
    {
      Ok( response ) => Ok( response ),
      Err( error ) => Err( error )
    }
  }


  /// # `update_rows_by_custom_row_key`
  ///
  /// Updates a specific row or rows in a Google Sheet with the provided values.
  ///
  /// ## Parameters:
  /// - `client`:  
  ///   A reference to the [`Client`] client configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:  
  ///   A `&str` specifying the name of the sheet.
  /// - `key_by`:  
  ///   A `( &str, serde_json::Value )` a pair of column key and its value.
  /// - `row_key_val`:  
  ///   A `HashMap<String, serde_json::Value>` where:  
  ///   - Key: The column name (e.g., "A", "B").  
  ///   - Value: The new value to set in the corresponding cell.
  /// - `update_range_at_all_match_cells`
  ///   A `bool` If true, updates the rows with all match cells. Otherwise updates row with the first match cell.
  /// - `raise_error_on_fail`
  ///   Returns an error if there were not found any matches.
  ///
  /// ## Returns:
  /// - Result< [`BatchUpdateValuesResponse`] >
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, e.g., due to invalid input or insufficient permissions.
  pub async fn update_rows_by_custom_row_key
  (
    client : &Client<'_>,
    spreadsheet_id : &str,
    sheet_name : &str,
    key_by : ( &str, serde_json::Value ), 
    row_key_val : HashMap< String, serde_json::Value >,
    on_find : OnFind,
    on_fail : OnFail
  ) -> Result< BatchUpdateValuesResponse >
  {
    // Getting provided column.
    let range = format!( "{}!{}:{}", sheet_name, key_by.0, key_by.0 );

    let value_range = client
    .spreadsheet()
    .values_get( spreadsheet_id, &range )
    .major_dimension( Dimension::Column )
    .value_render_option( ValueRenderOption::UnformattedValue )
    .doit()
    .await
    .map_err( | err | Error::ApiError( err.to_string() ) )?;

    let values = match value_range.values
    {
      Some( values ) => values,
      None =>
      {
        match on_fail
        {
          OnFail::Nothing => return Ok( BatchUpdateValuesResponse::default() ),
          OnFail::AppendRow =>
          {
            let _ = append_row( client, spreadsheet_id, sheet_name, &row_key_val ).await?;
            let response = BatchUpdateValuesResponse
            {
              spreadsheet_id : Some( spreadsheet_id.to_string() ),
              total_updated_rows : Some( 1 ),
              total_updated_sheets : Some( 1 ),
              ..Default::default()
            };

            return Ok( response );
          }
          OnFail::Error => return Err( Error::ApiError( "Not such value in the sheet.".to_string() ) )
        }
      }
    };

    // Counting mathces.
    let row_keys: Vec< usize > = values[0]
    .iter()
    .enumerate()
    .filter( | &( _, val ) | { *val == key_by.1 } )
    .map( | ( i, _ ) | i )
    .collect();

    if row_keys.is_empty()
    {
      match on_fail
      {
        OnFail::Nothing => return Ok( BatchUpdateValuesResponse::default() ),
        OnFail::AppendRow =>
        {
          let _ = append_row( client, spreadsheet_id, sheet_name, &row_key_val ).await?;
          let response = BatchUpdateValuesResponse
          {
            spreadsheet_id : Some( spreadsheet_id.to_string() ),
            total_updated_rows : Some( 1 ),
            total_updated_sheets : Some( 1 ),
            ..Default::default()
          };

          return Ok( response );
        }
        OnFail::Error => return Err( Error::ApiError( "Not such value in the sheet.".to_string() ) )
      }
    }

    // Preparing value ranges.
    let mut value_ranges = Vec::with_capacity( row_key_val.len() );
    let range = match on_find
    {
      OnFind::UpdateAllMatchedRow => row_keys,
      OnFind::UpdateFirstMatchedRow => vec![ *row_keys.first().unwrap() ],
      OnFind::UpdateLastMatchedRow => vec![ *row_keys.last().unwrap() ]
    };

    for row_key in range
    {
      for ( col_name, value ) in &row_key_val 
      {
        value_ranges.push
        (
          ValueRange
          { 
            major_dimension: Some( Dimension::Row ),
            values: Some( vec![ vec![ value.clone() ] ] ),
            range: Some( format!( "{}!{}{}", sheet_name, col_name, row_key + 1 ) ),
          }
        );
      }
    }

    // Making HTTP request.
    let request = BatchUpdateValuesRequest
    {
      data : value_ranges,
      value_input_option : ValueInputOption::UserEntered,
      include_values_in_response : Some( true ),
      response_value_render_option : Some( ValueRenderOption::FormattedValue ),
      response_date_time_render_option : Default::default()
    };

    match client
    .spreadsheet()
    .values_batch_update( spreadsheet_id, request )
    .doit()
    .await
    {
      Ok( response ) => Ok( response ),
      Err( error ) => Err( error )
    }

  }



  /// # `append_row`
  ///
  /// Append a new row at the end of the sheet.
  /// If provided range is empty, it will put values begining from A index.
  /// More information you can find here [append docs](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/append)
  ///
  /// ## Parameters:
  /// - `client`:  
  ///   A reference to the [`Client`] client configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:  
  ///   A `&str` specifying the name of the sheet whose header is to be retrieved.
  /// - `row_key_val`: 
  ///   A `HashMap<String, serde_json::Value>` where:  
  ///   - Key: The column name (e.g., "A", "B").  
  ///   - Value: The new value to set in the corresponding cell.
  /// 
  /// ## Returns:
  /// - `Result< ValuesAppendResponse >`
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, such as an invalid spreadsheet ID
  ///   or insufficient permissions.
  pub async fn append_row
  (
    client : &Client<'_>,
    spreadsheet_id : &str,
    sheet_name : &str,
    row_key_val : &HashMap< String, serde_json::Value >
  ) -> Result< ValuesAppendResponse >
  {
    // Sort column indexes, from A -> ZZZ
    let mut columns: Vec< &String > = row_key_val.keys().collect();
    columns.sort_by_key( | col | _column_label_to_number( col ) );

    let min_idx = _column_label_to_number( columns.first().unwrap() );
    let max_idx = _column_label_to_number( columns.last().unwrap() );

    // Creating a row, Vec< serde_json::Value >
    let mut row = Vec::with_capacity( max_idx - min_idx + 1 );

    for idx in min_idx..=max_idx
    {
      let col_label = _number_to_column_label( idx );
      let val = row_key_val
      .get( &col_label )
      .cloned()
      .unwrap_or_else( || serde_json::Value::String( "".to_string() ));
      
      row.push( val );
    }

    // Creating request.
    let range = format!( "{}!A1", sheet_name );
    let value_range = ValueRange
    {
      major_dimension : Some( Dimension::Row ),
      values : Some( vec![ row ] ),
      range : None
    };

    match client
    .spreadsheet()
    .append( spreadsheet_id, &range, value_range )
    .doit()
    .await
    {
      Ok( response ) => Ok( response ),
      Err( error ) => Err( Error::ApiError( error.to_string() ) )
    }

  }

  /// Converts number to column label.
  fn _number_to_column_label( mut num : usize ) -> String
  {
    let mut chars = Vec::new();
    while num > 0
    {
      let remainder = ( num - 1 ) % 26;
      let c = ( b'A' + remainder as u8 ) as char;
      chars.push( c );
      num = ( num - 1 ) / 26;
    }
    chars.reverse();
    chars.into_iter().collect()
  }
  /// Converts label to number.
  fn _column_label_to_number( col : &str ) -> usize
  {
    let mut result = 0;
    for c in col.chars()
    {
      let digit = c as usize - 'A' as usize + 1;
      result = result * 26 + digit
    }
    result
  }



  /// # `get_header`
  ///
  /// Retrieves the header row of a specific sheet.
  ///
  /// ## Parameters:
  /// - `client`:  
  ///   A reference to the [`Client`] client configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:  
  ///   A `&str` specifying the name of the sheet whose header is to be retrieved.
  ///
  /// ## Returns:
  /// - `Result<Vec<Vec<serde_json::Value>>>`
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, such as an invalid spreadsheet ID
  ///   or insufficient permissions.
  pub async fn get_header
  (

    client : &Client<'_>,
    spreadsheet_id : &str,
    sheet_name : &str, 
  ) -> Result< Vec< serde_json::Value > >
  {
    let range = format!( "{}!A1:ZZZ1", sheet_name );

    match client
    .spreadsheet()
    .values_get( spreadsheet_id, &range )
    .doit()
    .await
    {
      Ok( response ) =>
      {
        match response.values
        {
          Some( values ) => Ok( values[0].clone() ),
          None => Ok( Vec::new() )
        }
      } 
      Err( error ) => Err( error )
    }
    
  }

  /// # `get_rows`
  ///
  /// Retrieves all rows (excluding the header) from a specific sheet.
  ///
  /// ## Parameters:
  /// - `client`:  
  ///   A reference to the `Client` client configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:  
  ///   A `&str` specifying the name of the sheet whose rows are to be retrieved.
  ///
  /// ## Returns:
  /// - `Result<Vec<Vec<serde_json::Value>>>`
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, such as an invalid spreadsheet ID
  ///   or insufficient permissions.
  pub async fn get_rows
  (
    client : &Client<'_>,
    spreadsheet_id : &str,
    sheet_name : &str, 
  ) -> Result< Vec< Vec< serde_json::Value > > >
  {
    let range = format!( "{}!A2:ZZZ", sheet_name );

    match client
    .spreadsheet()
    .values_get( spreadsheet_id, &range )
    .value_render_option( ValueRenderOption::UnformattedValue )
    .doit()
    .await
    {
      Ok( response ) => 
      {
        match response.values
        {
          Some( values ) => Ok( values ),
          None => Ok( Vec::new() )
        }
      }
      Err( error ) => Err( error )
    }
    
  }

  /// # `get_cell`
  ///
  /// Retrieves the value of a specific cell from a Google Sheet.
  ///
  /// ## Parameters:
  /// - `client`:  
  ///   A reference to the [`Client`] client configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:  
  ///   A `&str` specifying the name of the sheet where the cell is located.
  /// - `cell_id`:  
  ///   A `&str` representing the cell ID in the format `A1`, where `A` is the column and `1` is the row.
  ///
  /// ## Returns:
  /// - `Result<serde_json::Value>`:
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, such as an invalid spreadsheet ID
  ///   or insufficient permissions.
  pub async fn get_cell
  (
    client : &Client<'_>,
    spreadsheet_id : &str,
    sheet_name : &str,
    cell_id : &str
  ) -> Result< serde_json::Value >
  {
    let range = format!( "{}!{}", sheet_name, cell_id );

    match client
    .spreadsheet()
    .values_get( spreadsheet_id, &range )
    .doit()
    .await
    {
      Ok( response ) =>
      {
        match response.values
        {
          Some( values ) => Ok( values[0][0].clone() ),
          None => Ok( json!( "" ) )
        }
      }
      Err( error ) => Err( error )
    }
  }

  /// # `set_cell`
  ///
  /// Updates the value of a specific cell in a Google Sheet.
  ///
  /// ## Parameters:
  /// - `client`:  
  ///   A reference to the `Client` client configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:  
  ///   A `&str` specifying the name of the sheet where the cell is located.
  /// - `cell_id`:  
  ///   A `&str` representing the cell ID in the format `A1`, where `A` is the column and `1` is the row.
  /// - `value`:  
  ///   A `serde_json::Value` containing the new value to update in the cell.
  ///
  /// ## Returns:
  /// - Result<[`UpdateValuesResponse`]>
  /// 
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, such as invalid input or insufficient permissions.
  pub async fn set_cell
  (
    client : &Client<'_>,
    spreadsheet_id : &str,
    sheet_name : &str,
    cell_id : &str,
    value : serde_json::Value
  ) -> Result< UpdateValuesResponse >
  {
    let range = format!( "{}!{}", sheet_name, cell_id );

    let value_range = ValueRange
    {
      values : Some( vec![ vec![ value ] ] ),
      ..ValueRange::default()
    };

    match client
    .spreadsheet()
    .values_update( value_range, spreadsheet_id, &range )
    .doit()
    .await
    {
      Ok( response ) => Ok( response ),
      Err( error ) => Err( error )
    }
  }

  /// Action to do if one or more rows were found.
  pub enum OnFind
  {
    /// Update first matched row.
    UpdateFirstMatchedRow,
    /// Update last matched row.
    UpdateLastMatchedRow,
    /// Update all matched rows.
    UpdateAllMatchedRow,
  }

  /// Action to do if row was not find.
  pub enum OnFail
  {
    /// Returns error.
    Error,
    /// Does nothing.
    Nothing,
    /// Append provided row at the and of sheet.
    AppendRow,
  }
  
    
}

crate::mod_interface!
{
  own use
  {
    OnFind,
    OnFail,
    set_cell,
    get_cell,
    get_rows,
    update_row,
    get_header,
    append_row,
    update_rows_by_custom_row_key,
    get_spreadsheet_id_from_url,
    parse_json,
    check_variant
  };
}