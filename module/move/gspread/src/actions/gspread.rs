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
  /// **Example:**  
  /// ```
  /// Error::ApiError(google_sheets4::Error::new(...))
  /// ```
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
  /// **Example:**  
  /// ```
  /// Error::HubError("Failed to initialize hub.".to_string())
  /// ```
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
  /// **Example:**  
  /// ```
  /// Error::InvalidUrl("Invalid spreadsheet URL.".to_string())
  /// ```
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
  /// **Example:**  
  /// ```
  /// Error::CellError("Failed to update cell A1.".to_string())
  /// ```
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
  /// **Example:**  
  /// ```
  /// Error::InvalidJSON("Missing required field in JSON.".to_string())
  /// ```
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
  ///
  /// **Example:**  
  /// ```
  /// Error::ParseError("Failed to parse date string.".to_string())
  /// ```
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
  /// ## Example:
  /// ```rust
  /// use std::collections::HashMap;
  /// 
  /// async fn example(hub: &SheetsType, spreadsheet_id: &str, sheet_name: &str) -> Result<(), Error> 
  /// {
  ///   let mut row_key_val = HashMap::new();
  ///   row_key_val.insert("A".to_string(), "New Value 1".to_string());
  ///   row_key_val.insert("B".to_string(), "New Value 2".to_string());
  /// 
  ///   match update_row(hub, spreadsheet_id, sheet_name, "1", row_key_val).await 
  ///   {
  ///     Ok(response) => println!("Row updated successfully: {:?}", response),
  ///     Err(error) => eprintln!("Failed to update row: {}", error),
  ///   }
  ///   Ok(())
  /// }
  /// ```
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, e.g., due to invalid input or insufficient permissions.
  ///
  /// ## Notes:
  /// - The `value_input_option` is set to `"USER_ENTERED"`, meaning the input values will be parsed as if entered by a user.
  pub async fn update_row
  (
    hub : &SheetsType,
    spreadsheet_id : &str,
    sheet_name : &str,
    row_key : &str,
    row_key_val : HashMap< String, String >
  ) -> Result< BatchUpdateValuesResponse >
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
  /// ## Example:
  /// ```rust
  /// async fn example(hub: &SheetsType, spreadsheet_id: &str, sheet_name: &str) 
  /// {
  ///   match get_header(hub, spreadsheet_id, sheet_name).await 
  ///   {
  ///     Ok(header) => 
  ///     {
  ///       println!("Header: {:?}", header);
  ///     }
  ///     Err(error) => 
  ///     {
  ///       eprintln!("Failed to retrieve header: {}", error);
  ///     }
  ///   }
  /// }
  /// ```
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, such as an invalid spreadsheet ID
  ///   or insufficient permissions.
  pub async fn get_header
  (
    hub : &SheetsType,
    spreadsheet_id : &str,
    sheet_name : &str, 
  ) -> Result< Vec< Vec< JsonValue > > >
  {
    // Making HTTP request.
    match hub
    .spreadsheets()
    .values_get( spreadsheet_id, format!( "{}!A1:Z1", sheet_name ).as_str() )
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
  /// ## Example:
  /// ```rust
  /// async fn example(hub: &SheetsType, spreadsheet_id: &str, sheet_name: &str) 
  /// {
  ///   match get_rows(hub, spreadsheet_id, sheet_name).await 
  ///   {
  ///     Ok(rows) => 
  ///     {
  ///       println!("Rows: {:?}", rows);
  ///     }
  ///     Err(error) => 
  ///     {
  ///       eprintln!("Failed to retrieve rows: {}", error);
  ///     }
  ///   }
  /// }
  /// ```
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, such as an invalid spreadsheet ID
  ///   or insufficient permissions.
  pub async fn get_rows
  (
    hub : &SheetsType,
    spreadsheet_id : &str,
    sheet_name : &str, 
  ) -> Result< Vec< Vec< JsonValue > > >
  {
    // Making HTTP request.
    match hub
    .spreadsheets()
    .values_get( spreadsheet_id, format!( "{}!A2:Z", sheet_name ).as_str() )
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
  /// ## Example:
  /// ```rust
  /// async fn example(hub: &SheetsType, spreadsheet_id: &str, sheet_name: &str, cell_id: &str) 
  /// {
  ///   match get_cell(hub, spreadsheet_id, sheet_name, cell_id).await 
  ///   {
  ///     Ok(value) => println!("Cell value: {:?}", value),
  ///     Err(error) => eprintln!("Failed to retrieve cell value: {}", error),
  ///   }
  /// }
  /// ```
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, such as an invalid spreadsheet ID
  ///   or insufficient permissions.
  pub async fn get_cell
  (
    hub : &SheetsType,
    spreadsheet_id : &str,
    sheet_name : &str,
    cell_id : &str
  ) -> Result< JsonValue >
  {
    // Making HTTP request.
    match hub
    .spreadsheets()
    .values_get( spreadsheet_id, format!( "{}!{}", sheet_name, cell_id ).as_str() )
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
  /// ## Example:
  /// ```rust
  /// async fn example(hub: &SheetsType, spreadsheet_id: &str, sheet_name: &str, cell_id: &str, value: &str) 
  /// {
  ///   match set_cell(hub, spreadsheet_id, sheet_name, cell_id, value).await 
  ///   {
  ///     Ok(response) => println!("Cell updated successfully: {:?}", response),
  ///     Err(error) => eprintln!("Failed to update cell: {}", error),
  ///   }
  /// }
  /// ```
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, such as invalid input or insufficient permissions.
  pub async fn set_cell
  (
    hub : &SheetsType,
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

    // Making HTTP request.
    match hub
    .spreadsheets()
    .values_update( value_range, spreadsheet_id, format!( "{}!{}", sheet_name, cell_id ).as_str() )
    .value_input_option( "USER_ENTERED" )
    .doit()
    .await
    {
      Ok( ( _, response) ) => Ok( response ),
      Err( error) => Err( Error::ApiError( error ) )
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