//!
//! Google Sheets API actions.
//!
//! This module also contains the definition of Google Sheets Error.
//!

mod private
{
  use regex::Regex;
  use error_tools::typed::Error;
  use derive_tools::AsRefStr;
  use serde_json::json;
  use crate::*;
  use ser::
  {
    DisplayFromStr, 
    JsonValue
  };
  use std::collections::HashMap;
  use google_sheets4::api::
  {
    BatchUpdateValuesRequest, 
    BatchUpdateValuesResponse, 
    UpdateValuesResponse, 
    ValueRange
  };

  /// # Error
  ///
  /// Represents errors that can occur while interacting with the Google Sheets API 
  /// or during related operations in the application.
  ///
  /// ## Variants:
  ///
  /// ### `ApiError`
  ///
  /// Represents an error returned by the Google Sheets API.
  ///
  /// **Details:**  
  /// This error occurs when the API returns a specific error message.  
  /// The error message from the Google Sheets API is stored and displayed.
  ///
  /// **Fields:**  
  /// - `google_sheets4::Error`:  
  ///   The raw error returned by the API.
  ///
  /// ### `HubError`
  ///
  /// Represents an error that occurs while initializing Google Sheets Hub.
  ///
  /// **Details:**  
  /// This error indicates that the application failed to properly configure with the Google Sheets Hub.
  ///
  /// **Fields:**  
  /// - `String`:  
  ///   A detailed error message describing the issue.
  ///
  /// ### `InvalidUrl`
  ///
  /// Represents an error caused by an invalid URL format.
  ///
  /// **Details:**  
  /// This error occurs when the provided URL does not match the expected format.
  ///
  /// **Fields:**  
  /// - `String`:  
  ///   The invalid URL or a message describing the issue.
  ///
  /// ### `CellError`
  ///
  /// Represents an error related to a cell in the spreadsheet.
  ///
  /// **Details:**  
  /// This error indicates that a cell was not retrieved or updated successfully.
  ///
  /// **Fields:**  
  /// - `String`:  
  ///   A message describing the issue with the cell.
  ///
  /// ### `InvalidJSON`
  ///
  /// Represents an error caused by invalid JSON input or parsing issues.
  ///
  /// **Details:**  
  /// This error occurs when the provided JSON data does not conform to the expected structure or format.
  ///
  /// **Fields:**  
  /// - `String`:  
  ///   A detailed error message describing the JSON issue.
  ///
  /// ### `ParseError`
  ///
  /// Represents a generic parsing error.
  ///
  /// **Details:**  
  /// This error is raised when a string or other input cannot be parsed into the expected format or structure.
  ///
  /// **Fields:**  
  /// - `String`:  
  ///   A message describing the parse error.
  #[ ser::serde_as ]
  #[ derive( Debug, Error, AsRefStr, ser::Serialize ) ]
  #[ serde( tag = "type", content = "data" ) ]
  pub enum Error
  {
    /// Represents an error returned by the Google Sheets API.
    /// 
    /// # Details
    /// This error occurs when the API returns a specific error message.
    /// The error message from the Google Sheets API is stored and displayed.
    /// 
    /// # Fields
    /// - `google_sheets4::Error`: The raw error returned by the API.
    #[ error( "Google Sheets returned error:\n{0}" ) ]
    ApiError
    (
      #[ from ]
      #[ serde_as( as = "DisplayFromStr" ) ]
      google_sheets4::Error
    ),

    #[ error( "Mock Google API returned error:\n{0}" ) ]
    MockApiError
    (
      String
    ),

    /// Represents an error that occurs while initializing Google Sheets Hub.
    /// 
    /// # Details
    /// This error indicates that the application failed to properly configure with the Google Sheets Hub.
    /// 
    /// # Fields
    /// - `String`: A detailed error message describing the issue.
    #[ error( "Hub Error:\n{0}" ) ]
    HubError
    (
      String
    ),

    /// Represents an error caused by an invalid URL format.
    /// 
    /// # Details
    /// This error occurs when the provided URL does not match the expected format
    /// 
    /// # Fields
    /// - `String`: The invalid URL or a message describing the issue.
    #[ error( "Invalid URL format:\n{0}" ) ]
    InvalidUrl
    (
      String
    ),

    /// Represents an error related to a cell in the spreadsheet.
    /// 
    /// # Details
    /// This error indicates that a cell was not got or updated
    /// 
    /// # Fields
    /// - `String`: A message describing the issue with the cell.
    #[ error( "Cell error:\n{0}" ) ]
    CellError
    (
      String
    ),

    /// Represents an error caused by invalid JSON input or parsing issues.
    /// 
    /// # Details
    /// This error occurs when the provided JSON data does not conform to the expected
    /// structure or format.
    /// 
    /// # Fields
    /// - `String`: A detailed error message describing the JSON issue.
    #[ error( "Invalid JSON format:\n{0}" ) ]
    InvalidJSON
    (
      String
    ),

    /// Represents a generic parsing error.
    /// 
    /// # Details
    /// This error is raised when a string or other input cannot be parsed
    /// into the expected format or structure.
    /// 
    /// # Fields
    /// - `String`: A message describing the parse error.
    #[ error( "Parse error:\n{0}" ) ]
    ParseError
    (
      String
    )
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
  /// - `hub`:  
  ///   A reference to the `SheetsType` client configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:  
  ///   A `&str` specifying the name of the sheet.
  /// - `row_key`:  
  ///   A `&str` representing the row's key (e.g., the row index).
  /// - `row_key_val`:  
  ///   A `HashMap<String, String>` where:  
  ///   - Key: The column name (e.g., "A", "B").  
  ///   - Value: The new value to set in the corresponding cell.
  ///
  /// ## Returns:
  /// - `Result<BatchUpdateValuesResponse>`
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, e.g., due to invalid input or insufficient permissions.
  ///
  /// ## Notes:
  /// - The `value_input_option` is set to `"USER_ENTERED"`, meaning the input values will be parsed as if entered by a user.
  pub async fn update_row
  (
    client : &GspreadClient,
    // hub : &SheetsType,
    spreadsheet_id : &str,
    sheet_name : &str,
    row_key : &str,
    row_key_val : HashMap< String, String >
  ) -> Result< BatchUpdateValuesResponse >
  {
    match client
    {
      GspreadClient::GoogleHub( hub ) =>
      {
        // Cretaing JSON with values to update. 
        let mut value_ranges = Vec::with_capacity( row_key_val.len() );

        for ( col_name, value ) in row_key_val {
          value_ranges.push
          (
            ValueRange 
            { 
              major_dimension: Some( String::from( "ROWS" ) ),
              values: Some( vec![ vec![ json!( value ) ] ] ),
              range: Some( format!( "{}!{}{}", sheet_name, col_name, row_key ) ), 
            }
          )
        }

        // Creating request.
        let req = BatchUpdateValuesRequest
        {
          value_input_option: Some( "USER_ENTERED".to_string() ),
          data: Some( value_ranges ),
          include_values_in_response: Some( true ),
          ..Default::default()
        };

        // Making HTTP request.
        match hub
        .spreadsheets()
        .values_batch_update( req, spreadsheet_id )
        .doit()
        .await
        {
          Ok( ( _, response ) ) => Ok( response ),
          Err( error ) => Err( Error::ApiError( error ) ),
        }
      },

      GspreadClient::MockHub(mock_client) => 
      {
        let mut update_responses = Vec::with_capacity( row_key_val.len() );

        for (col_name, value) in row_key_val 
        {
          let range_str = format!( "{}!{}{}", sheet_name, col_name, row_key );
          let value_range = ValueRange 
          {
            major_dimension: Some( "ROWS".to_string() ),
            values: Some( vec![ vec![ json!( value ) ] ] ),
            range: Some( range_str.clone() ),
          };

          let update_result = mock_client
          .spreadsheet()
          .values_update( value_range, spreadsheet_id, &range_str )
          .doit()
          .await?;

          update_responses.push( update_result );
        }
        
        let total_updated_cells = update_responses.iter().fold( 0, | acc, r | {
            acc + r.updated_cells.unwrap_or( 0 )
        });
        let total_updated_rows = update_responses.iter().fold( 0, | acc, r | {
            acc + r.updated_rows.unwrap_or( 0 )
        });
        let total_updated_cols = update_responses.iter().fold( 0, | acc, r | {
            acc + r.updated_columns.unwrap_or( 0 )
        });

        let batch_response = BatchUpdateValuesResponse 
        {
          spreadsheet_id: Some( spreadsheet_id.to_string() ),
          total_updated_cells: Some( total_updated_cells ),
          total_updated_rows: Some( total_updated_rows ),
          total_updated_columns: Some( total_updated_cols ),
          total_updated_sheets: None,
          responses: Some( update_responses.into_iter().map( | ur| ur.into() ).collect() ),
        };

        Ok( batch_response )
      }
    }
    
  }

  /// # `get_header`
  ///
  /// Retrieves the header row of a specific sheet.
  ///
  /// ## Parameters:
  /// - `hub`:  
  ///   A reference to the `SheetsType` client configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:  
  ///   A `&str` specifying the name of the sheet whose header is to be retrieved.
  ///
  /// ## Returns:
  /// - `Result<Vec<Vec<JsonValue>>>`
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, such as an invalid spreadsheet ID
  ///   or insufficient permissions.
  pub async fn get_header
  (

    client : &GspreadClient,
    // hub : &SheetsType,
    spreadsheet_id : &str,
    sheet_name : &str, 
  ) -> Result< Vec< Vec< JsonValue > > >
  {
    let range = format!( "{}!A1:Z1", sheet_name );

    match client
    {
      GspreadClient::GoogleHub( hub ) => 
      {
        match hub
        .spreadsheets()
        .values_get( spreadsheet_id, &range )
        .doit()
        .await
        {
          Ok( ( _, response ) ) =>
          {
            match response.values
            {
              Some( values ) => Ok( values ),
              None => Ok( Vec::new() )
            }
          },
          Err( error ) => Err( Error::ApiError( error ) )
        }
      },

      GspreadClient::MockHub( mock_client ) => 
      {
        match mock_client
        .spreadsheet()
        .values_get( spreadsheet_id, &range )
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
          },
          Err( error ) => Err( error )
        }
      }
    }
    
  }

  /// # `get_rows`
  ///
  /// Retrieves all rows (excluding the header) from a specific sheet.
  ///
  /// ## Parameters:
  /// - `hub`:  
  ///   A reference to the `SheetsType` client configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:  
  ///   A `&str` specifying the name of the sheet whose rows are to be retrieved.
  ///
  /// ## Returns:
  /// - `Result<Vec<Vec<JsonValue>>>`
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, such as an invalid spreadsheet ID
  ///   or insufficient permissions.
  pub async fn get_rows
  (
    client : &GspreadClient,
    // hub : &SheetsType,
    spreadsheet_id : &str,
    sheet_name : &str, 
  ) -> Result< Vec< Vec< JsonValue > > >
  {
    let range = format!( "{}!A2:Z", sheet_name );

    match client
    {
      GspreadClient::GoogleHub( hub ) => 
      {
        match hub
        .spreadsheets()
        .values_get( spreadsheet_id, &range )
        .doit()
        .await
        {
          Ok( ( _, response ) ) =>
          {
            match response.values
            {
              Some( values ) => Ok( values ),
              None => Ok( Vec::new() )
            }
          },
          Err( error ) => Err( Error::ApiError( error ) )
        }
      },

      GspreadClient::MockHub( mock_server ) => 
      {
        match mock_server
        .spreadsheet()
        .values_get( spreadsheet_id, &range )
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
          },
          Err( error ) => Err( error )
        }
      }
    }
    
  }

  /// # `get_cell`
  ///
  /// Retrieves the value of a specific cell from a Google Sheet.
  ///
  /// ## Parameters:
  /// - `hub`:  
  ///   A reference to the `SheetsType` client configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:  
  ///   A `&str` specifying the name of the sheet where the cell is located.
  /// - `cell_id`:  
  ///   A `&str` representing the cell ID in the format `A1`, where `A` is the column and `1` is the row.
  ///
  /// ## Returns:
  /// - `Result<JsonValue>`:
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, such as an invalid spreadsheet ID
  ///   or insufficient permissions.
  pub async fn get_cell
  (
    client : &GspreadClient,
    spreadsheet_id : &str,
    sheet_name : &str,
    cell_id : &str
  ) -> Result< JsonValue >
  {
    let range = format!( "{}!{}", sheet_name, cell_id );

    // Making HTTP request.
    match client
    {
      GspreadClient::GoogleHub( hub ) => 
      {
        match hub
        .spreadsheets()
        .values_get( spreadsheet_id, &range )
        .doit()
        .await
        {
          Ok( ( _, response ) ) => 
          match response.values
          {
            Some( values ) => Ok( values.get( 0 ).unwrap().get( 0 ).unwrap().clone() ),
            None => Ok( JsonValue::Null.clone() )
          }
          Err( error ) => Err( Error::ApiError( error ) )
        }    
      },

      GspreadClient::MockHub( mock_client ) =>
      {
        match mock_client
        .spreadsheet()
        .values_get( spreadsheet_id, &range )
        .doit()
        .await
        {
          Ok( response ) => 
          match response.values
          {
            Some( values ) => Ok( values.get( 0 ).unwrap().get( 0 ).unwrap().clone() ),
            None => Ok( JsonValue::Null.clone() )
          }
          Err( error ) => Err( error )
        }
      }
    }
  }

  /// # `set_cell`
  ///
  /// Updates the value of a specific cell in a Google Sheet.
  ///
  /// ## Parameters:
  /// - `hub`:  
  ///   A reference to the `SheetsType` client configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:  
  ///   A `&str` specifying the name of the sheet where the cell is located.
  /// - `cell_id`:  
  ///   A `&str` representing the cell ID in the format `A1`, where `A` is the column and `1` is the row.
  /// - `value`:  
  ///   A `&str` containing the new value to update in the cell.
  ///
  /// ## Returns:
  /// - `Result<UpdateValuesResponse>`
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, such as invalid input or insufficient permissions.
  pub async fn set_cell
  (
    // hub : &SheetsType,
    client : &GspreadClient,
    spreadsheet_id : &str,
    sheet_name : &str,
    cell_id : &str,
    value : &str
  ) -> Result< UpdateValuesResponse >
  {
    // Creating JSON with value to update.
    let value_range = ValueRange
    {
      values : Some( vec![ vec![ json!( value ) ] ] ),
      ..ValueRange::default()
    };

    let range = format!( "{}!{}", sheet_name, cell_id );

    match client
    {
      GspreadClient::GoogleHub( hub ) => 
      {
        match hub
        .spreadsheets()
        .values_update( value_range, spreadsheet_id, &range )
        .value_input_option( "USER_ENTERED" )
        .doit()
        .await
        {
          Ok( ( _, response) ) => Ok( response ),
          Err( error) => Err( Error::ApiError( error ) )
        }
      },

      GspreadClient::MockHub( mock_client ) => 
      {
        match mock_client
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
    
  }

  /// Type alias for `std::result::Result< T, Error >`.
  pub type Result< T > = std::result::Result< T, Error >;
}

crate::mod_interface!
{
  own use
  {
    Error,
    Result,
    set_cell,
    get_cell,
    get_rows,
    update_row,
    get_header,
    get_spreadsheet_id_from_url,
  };
}