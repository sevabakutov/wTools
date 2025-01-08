//!
//! Action for command "header"
//!
//! It returns header (first row)
//!


mod private
{
  use crate::*;
  use client::SheetsType;
  use actions::gspread::
  {
    get_header, 
    Result
  };
  use ser::JsonValue;

  pub async fn action
  (
    hub : &SheetsType,
    spreadsheet_id : &str,
    sheet_name : &str
  ) -> Result< Vec< Vec< JsonValue > > >
  {
    match get_header( hub, spreadsheet_id, sheet_name ).await
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