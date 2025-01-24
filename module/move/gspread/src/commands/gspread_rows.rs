//!
//! Command "rows"
//!

mod private
{
  use crate::*;
  use actions;
  use gcore::client::Client;
  use debug::{ Report, RowWrapper };
  use commands::gspread::CommonArgs;
  use actions::utils::get_spreadsheet_id_from_url;

  /// # `command`
  ///
  /// Processes the `rows` command by retrieving rows from a specified Google Sheet
  /// and displaying them in a table format in the console.
  ///
  /// ## Parameters:
  /// - `client`:  
  ///   A `GspreadClient` enum.
  ///   - `Variants`: 
  ///     `SheetsType` variant is used for interacting with the Google Sheets API. 
  ///     `MockClient` variant is used for mock testing.
  /// - `args`:  
  ///   A `CommonArgs` instance containing the sheet's URL and tab name.
  ///
  /// ## Errors:
  /// - Prints an error message if the spreadsheet ID extraction or row retrieval fails.
  pub async fn command
  (
    client : &Client<'_>,
    args : CommonArgs
  )
  {
    match args
    {
      CommonArgs { url, tab } =>
      {
        let spreadsheet_id = match get_spreadsheet_id_from_url( url.as_str() ) 
        {
          Ok( id ) => id,
          Err( error ) => 
          {
            eprintln!( "Error extracting spreadsheet ID: {}", error );
            return;
          }
        };

        match actions::gspread_rows_get::action
        (
          client,
          spreadsheet_id,
          tab.as_str()
        )
        .await
        {
          Ok( rows ) =>
          {
            let max_len = rows
            .iter()
            .map( | row | row.len() )
            .max()
            .unwrap_or( 0 );

            let rows_wrapped: Vec< RowWrapper > = rows
            .into_iter()
            .map( | row | RowWrapper { row, max_len } )
            .collect();

            println!( "Rows:\n{}", Report{ rows: rows_wrapped } );
          }
          Err( error ) => eprintln!( "Error:\n{}", error ),
        }
      }
    }
  }
}

crate::mod_interface!
{
  own use
  {
    command
  };
}
