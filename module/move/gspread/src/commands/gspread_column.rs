//!
//! Command column.
//! 

mod private
{
  use clap::Subcommand;
  use crate::*;
  use gcore::client::Client;
  use debug::{ RowWrapper, Report };
  use actions::
  {
    self,
    utils::get_spreadsheet_id_from_url
  };


  /// # Commands
  /// 
  /// Subcommands for `COLUMN` command
  /// 
  /// ## Variants:
  /// 
  /// ### `Get`
  /// Retreive a column from a Google Sheet.
  /// 
  /// **Arguments:**
  /// - `url`:  
  ///   The full URL of the Google Sheet.  
  ///   Example:  
  ///   `--url 'https://docs.google.com/spreadsheets/d/your_spreadsheet_id/edit?gid=0#gid=0'`
  ///
  /// - `tab`:  
  ///   The name of the specific sheet (tab) in the Google Spreadsheet.  
  ///   Example:  
  ///   `--tab 'Sheet1'`
  /// 
  /// - `column_id`:
  ///   Column id. In the range from A to ZZZ.
  ///   Example:
  ///   `--column-id=A`
  /// 
  /// **Example:**
  /// ```bash
  /// gspread column get
  ///   --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0' \
  ///   --tab 'tab1' \
  ///   --column-id 'A'
  /// ```
  #[ derive( Debug, Subcommand ) ]
  pub enum Commands
  {
    /// Retreive a column from a Google Sheet.
    /// 
    /// **Example:**
    /// 
    /// gspread column get
    ///   --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0'
    ///   --tab 'tab1'
    ///   --column-id 'A'
    #[ command( name = "get" ) ]
    Get
    {
      /// Retreive a column from Google Sheet
      #[ arg( long, help = "Full URL of Google Sheet.\n\
      It has to be inside of '' to avoid parse errors.\n\
      Example: 'https://docs.google.com/spreadsheets/d/your_spreadsheet_id/edit?gid=0#gid=0'" ) ]
      url : String,

      #[ arg( long, help = "Sheet name.\nExample: Sheet1" ) ]
      tab : String,

      #[ arg( long, help = "Column id, in range from A to ZZZ" ) ]
      column_id : String
    }
  }

  /// # `command`
  ///
  /// Executes the specified subcommand for the `COLUMN` command.
  ///
  /// ## Parameters:
  /// - `client`:  
  ///   A `Client` type.
  /// - `commands`:  
  ///   A variant of the `Commands` enum specifying the operation to execute.
  ///
  /// ## Errors:
  /// - Prints an error message if the spreadsheet ID extraction, retrieval, or update fails.
  pub async fn command
  (
    client : &Client<'_>,
    commands : Commands
  )
  {
    match commands
    {
      Commands::Get { url, tab, column_id } =>
      {
        let spreadsheet_id = match get_spreadsheet_id_from_url( &url ) 
        {
          Ok( id ) => id,
          Err( error ) => 
          {
            eprintln!( "Error extracting spreadsheet ID: {}", error );
            return;
          }
        };

        match actions::gspread_column_get::action
        (
          client, 
          spreadsheet_id, 
          &tab, 
          &column_id
        )
        .await
        {
          Ok( column ) =>
          {
            let column_wrapped = column
            .into_iter()
            .map( | row | RowWrapper{ row : vec![ row ], max_len : 1 } )
            .collect();

            println!( "Column:\n{}", Report{ rows : column_wrapped } )
          }
          Err( error ) => eprintln!( "Error:\n{}", error )
        }
      }
    }
  }

}

crate::mod_interface!
{
  own use
  {
    command,
    Commands
  };
}