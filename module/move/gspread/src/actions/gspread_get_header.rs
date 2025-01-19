//!
//! Action for the command "header".
//!
//! Retrieves the header (first row) from the specified Google Sheet.
//!


mod private
{
  use crate::*;
  use actions::gspread::
  {
    get_header, 
    Result
  };
  use client::client::Client;
use ser::JsonValue;

  pub async fn action
  (
    client : &Client,
    spreadsheet_id : &str,
    sheet_name : &str
  ) -> Result< Vec< Vec< JsonValue > > >
  {
    match get_header( client, spreadsheet_id, sheet_name ).await
    {
      Ok( result ) => Ok( result ),
      Err( error ) => Err( error )
    }
  }

}

crate::mod_interface!
{
  own use action;
}