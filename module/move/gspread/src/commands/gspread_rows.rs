//!
//! Command "rows"
//!

mod private
{
  use crate::*;
  use actions;
  use gcore::Secret;
  use gcore::client::Client;
  use commands::gspread::CommonArgs;
  use actions::utils::get_spreadsheet_id_from_url;
  use debug::
  { 
    Report, 
    RowWrapper 
  };

  /// # `command`
  ///
  /// Processes the `rows` command by retrieving rows from a specified Google Sheet
  /// and displaying them in a table format in the console.
  ///
  /// ## Errors:
  /// - Prints an error message if the spreadsheet ID extraction or row retrieval fails.
  pub async fn command< S : Secret >
  (
    client : &Client< '_, S >,
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

<<<<<<< HEAD
        match actions::gspread_get_rows::action
=======
        match actions::gspread_rows_get::action
>>>>>>> updstream/alpha
        (
          client,
          spreadsheet_id,
          tab.as_str()
        )
        .await
        {
          Ok( rows ) =>
          {
<<<<<<< HEAD
            let max_len = rows.iter().map(|row| row.len()).max().unwrap_or(0);
            let rows_wrapped: Vec<RowWrapper> = rows
            .into_iter()
            .map(|row| RowWrapper { row, max_len })
            .collect();

            println!( "Rows:\n{}", Report{ rows: rows_wrapped } );
=======
            let max_len = rows
            .iter()
            .map( | row | row.len() )
            .max()
            .unwrap_or( 0 );

            let rows_wrapped: Vec< RowWrapper > = rows
            .into_iter()
            .map( | row | RowWrapper { row, max_len } )
            .collect();

            println!( "Rows:\n{}", Report{ rows : rows_wrapped } );
>>>>>>> updstream/alpha
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
