//!
//! Action for the command "cell set".
//!
//! Updates the value of a selected cell in the specified Google Sheet.
//!


mod private
{
  use crate::*;
  use actions::gspread::
  { 
    set_cell, 
    Error, 
    Result 
  };

  pub async fn action
  (
    client : &GspreadClient,
    spreadsheet_id : &str,
    sheet_name : &str,
    cell_id : &str,
    value : &str
  ) -> Result< i32 >
  {
    match set_cell( client, spreadsheet_id, sheet_name, cell_id, value ).await
    {
      Ok( response ) =>
      {
        match response.updated_cells 
        {
          Some( amount ) => Ok( amount ),
          None => Err( Error::CellError( "Some problem with cell updating".to_string() ) )
        }
      },
      Err( error ) => Err( error )
    }
  }
}

crate::mod_interface!
{
  own use action;
}