//!
//! Copy command action 
//!

mod private
{
  use crate::*;
  use actions::gspread::copy_to;
  use gcore::
  {
    client::Client, 
    error::Result
  };

  pub async fn action
  (
    client : &Client<'_>,
    spreadsheet_id : &str,
    sheet_id : &str,
    dest : &str
  ) -> Result< String >
  {
    match copy_to
    (
      client, 
      spreadsheet_id, 
      sheet_id, 
      dest
    )
    .await
    {
      Ok( response ) =>
      {
        let title = response.title.unwrap_or_default();
        Ok( title )
      },
      Err( error ) => Err( error )
    }
  }
}

crate::mod_interface!
{
  own use
  {
    action
  };
}