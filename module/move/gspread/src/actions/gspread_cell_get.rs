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
  use client::SheetsType;
  use ser::JsonValue;

  pub async fn action
  (
    hub : &SheetsType,
    spreadsheet_id : &str,
    sheet_name : &str,
    cell_id : &str,
  ) -> Result< JsonValue >
  {
    match get_cell( hub, spreadsheet_id, sheet_name, cell_id ).await
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