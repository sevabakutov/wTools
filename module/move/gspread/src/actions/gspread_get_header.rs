//!
//! Action for command "header"
//!
//! It returns header (first row)
//!


mod private
{
  use std::fmt;
  use crate::*;
  use client::SheetsType;
  use actions::gspread::
  {
    get_header, Result
  };
  use format_tools::AsTable;
  use util::display_table::display_header;
  use ser::JsonValue;

  #[ derive( Debug ) ]
  pub struct Report
  {
    pub rows : Vec< RowWrapper >
  }

  impl fmt::Display for Report
  {
    fn fmt
    (
      &self,
      f : &mut fmt::Formatter
    ) -> fmt::Result
    {
      display_header( &AsTable::new( &self.rows ), f )
    }
  }

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