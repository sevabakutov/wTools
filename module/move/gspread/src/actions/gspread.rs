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
  use crate::*;
  use ser::{DisplayFromStr, JsonValue};
  use std::collections::HashMap;
  use google_sheets4::api::{BatchUpdateValuesResponse, ValueRange};

  #[ ser::serde_as ]
  #[ derive( Debug, Error, AsRefStr, ser::Serialize ) ]
  #[ serde( tag = "type", content = "data" ) ]
  pub enum Error
  {
    #[ error( "Google Sheets returned error:\n{0}" ) ]
    ApiError
    (
      #[ from ]
      #[ serde_as( as = "DisplayFromStr" ) ]
      google_sheets4::Error
    ),

    #[ error( "Invalid URL format:\n{0}" ) ]
    InvalidUrl
    (
      String
    ),

    #[ error( "Cell error:\n{0}" ) ]
    CellError
    (
      String
    ),

    #[ error( "Invalid JSON format:\n{0}" ) ]
    InvalidJSON
    (
      String
    ),

    #[ error( "Parse error:\n{0}" ) ]
    ParseError
    (
      String
    )
  }

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

  /// Function to update a row on a Google Sheet.
  /// 
  /// It converts from HashMap to a row wich is actually sorted array, by column name.
  /// 
  /// **Params**
  ///  - `id` : Row's id.
  ///  - `vales` : Pairs of key value, where key is a clomun's name and value is a value of cell.
  /// 
  /// **Returns**
  ///  - `RowWrapper` object.
  pub async fn update_row
  (
    row_key : usize,
    values : HashMap< String, String >,
    sheet_name : &str
  ) -> Result< Vec< ValueRange > >
  {
    let mut value_ranges = Vec::with_capacity( values.len() );

    for ( col_name, value ) in values {
      value_ranges.push
      (
        ValueRange 
        { 
          major_dimension: Some( String::from( "ROWS" ) ),
          values: Some( vec![ vec![ JsonValue::String( value ) ] ] ),
          range: Some( format!( "{}!{}{}", sheet_name, col_name, row_key ) ), 
        }
      )
    }

    Ok( value_ranges )
  }

  pub type Result< T > = core::result::Result< T, Error >;
}

crate::mod_interface!
{
  own use
  {
    Error,
    Result,
    update_row,
    get_spreadsheet_id_from_url,
  };
}