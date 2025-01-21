//!
//! Command "header"
//!

mod private
{
  use std::fmt;
  use crate::*;
  use gcore::client::Client;
  use commands::gspread::CommonArgs;
  use actions;
  use actions::gspread::get_spreadsheet_id_from_url;
  use format_tools::AsTable;
  use utils::display_table::display_header;

  /// # Report
  ///
  /// A structure to display the retrieved header in the console using `format_tools`.
  ///
  /// ## Fields:
  /// - `header`:  
  ///   A `Vec<RowWrapper>` representing the retrieved header rows.
  ///
  /// ## Usage:
  /// This structure is used in conjunction with the `fmt::Display` trait to render the header in a formatted table view.
  #[ derive( Debug ) ]
  pub struct Report
  {
    pub header : Vec< RowWrapper >
  }

  impl fmt::Display for Report
  {
    /// Formats the header for display by calling the `display_header` function,
    /// which uses appropriate functions from `format_tools`.
    ///
    /// ## Parameters:
    /// - `f`:  
    ///   A mutable reference to the `fmt::Formatter` used to write the formatted output.
    ///
    /// ## Returns:
    /// - `fmt::Result` 
    fn fmt
    (
      &self,
      f : &mut fmt::Formatter
    ) -> fmt::Result
    {
      display_header( &AsTable::new( &self.header ), f )
    }
  }

  /// # `command`
  ///
  /// Processes the `header` command by retrieving the header (first row) from a specified Google Sheet
  /// and displaying it in a table format in the console.
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
  /// - Prints an error message if the spreadsheet ID extraction or header retrieval fails.
  pub async fn command
  (
    client : &Client<'_>,
    args : CommonArgs,
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

        match actions::gspread_get_header::action
        (
          client,
          spreadsheet_id,
          tab.as_str()
        )
        .await
        {
          Ok( header ) =>
            {
              let header_wrapped = header
              .into_iter()
              .map( | row | RowWrapper{ max_len: row.len(), row } )
              .collect();
              
              println!( "Header:\n{}", Report{ header: header_wrapped } );
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

