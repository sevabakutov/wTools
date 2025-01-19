//!
//! Action for the command "cell get".
//!
//! Retrieves the value of a selected cell from the specified Google Sheet.
//!

mod private
{
  use crate::*;
  use actions::gspread::
  {
    get_cell, 
    Result
  };
  use client::client::Client;
use ser::JsonValue;

  pub async fn action
  (
    client : &Client,
    spreadsheet_id : &str,
    sheet_name : &str,
    cell_id : &str,
  ) -> Result< JsonValue >
  {
    match get_cell( client, spreadsheet_id, sheet_name, cell_id ).await
    {
      Ok( value ) => Ok( value ),
      Err( error ) => Err( error )
    }
  }
}

crate::mod_interface!
{
  own use action;
}