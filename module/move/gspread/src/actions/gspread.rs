//!
//! Google Sheets API actions.
//!
//! This module also contains the definition of Google Sheets Error.
//!

mod private
{
  use regex::Regex;
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
    BatchUpdateValuesRequest,
    BatchUpdateValuesResponse,
  };
  
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
    client : &Client,
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

    client : &Client,
    spreadsheet_id : &str,
    sheet_name : &str, 
  ) -> Result< Vec< Vec< serde_json::Value > > >
  {
    let range = format!( "{}!A1:ZZZ1", sheet_name );

    match client
    .spreadsheet()
    .values_get( spreadsheet_id, &range )
    .doit()
    .await
    {
      Ok( response ) => Ok( response.values.unwrap() ),
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
    client : &Client,
    spreadsheet_id : &str,
    sheet_name : &str, 
  ) -> Result< Vec< Vec< serde_json::Value > > >
  {
    let range = format!( "{}!A2:Z", sheet_name );

    match client
    .spreadsheet()
    .values_get( spreadsheet_id, &range )
    .doit()
    .await
    {
      Ok( response ) => Ok( response.values.unwrap() ),
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
    client : &Client,
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
      Ok( response ) => Ok
      ( 
        response
        .values
        .unwrap()
        .get( 0 )
        .unwrap()
        .get( 0 )
        .unwrap()
        .clone() 
      ),
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
    client : &Client,
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
    
}

crate::mod_interface!
{
  own use
  {
    set_cell,
    get_cell,
    get_rows,
    update_row,
    get_header,
    get_spreadsheet_id_from_url,
  };
}