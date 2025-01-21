//!
//! Collection of subcommands fo command "cell"
//!

mod private
{

  use clap::Subcommand;
use gcore::client::Client;

  use crate::*;
  use actions;
  use actions::gspread::get_spreadsheet_id_from_url;

  /// # Commands
  ///
  /// Subcommands for the `CELL` command, used to interact with individual cells in a Google Sheet.
  ///
  /// ## Variants:
  ///
  /// ### `Get`
  ///
  /// Retrieves the value of a specific cell.
  ///
  /// **Arguments:**
  /// - `url`:  
  ///   The full URL of the Google Sheet.  
  ///   Example: `'https://docs.google.com/spreadsheets/d/your_spreadsheet_id/edit?gid=0#gid=0'`.
  ///
  /// - `tab`:  
  ///   The name of the specific sheet to target.  
  ///   Example: `Sheet1`.
  ///
  /// - `cell`:  
  ///   The ID of the cell in the format `A1`, where `A` is the column and `1` is the row.  
  ///   Example: `A4`.
  ///
  /// **Example:**
  /// ```bash
  /// gspread cell get \
  /// --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0' \
  /// --tab tab1 \
  /// --cell A1
  /// ```
  ///
  /// ### `Set`
  ///
  /// Updates the value of a specific cell.
  ///
  /// **Arguments:**
  /// - `url`:  
  ///   The full URL of the Google Sheet.  
  ///   Example: `'https://docs.google.com/spreadsheets/d/your_spreadsheet_id/edit?gid=0#gid=0'`.
  ///
  /// - `tab`:  
  ///   The name of the specific sheet to target.  
  ///   Example: `Sheet1`.
  ///
  /// - `cell`:  
  ///   The ID of the cell in the format `A1`, where `A` is the column and `1` is the row.  
  ///   Example: `A4`.
  ///
  /// - `val`:  
  ///   The value to set in the specified cell.  
  ///   Example: `hello`.
  ///
  /// **Example:**
  /// ```bash
  /// gspread cell set \
  /// --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0' \
  /// --tab tab1 \
  /// --cell A1 \
  /// --val 13
  /// ```
  #[ derive( Debug, Subcommand ) ]
  pub enum Commands
  {
    /// Retrieves a single cell of a specific sheet.
    /// 
    /// **Example**:
    /// 
    /// gspread cell get
    /// --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0'
    /// --tab tab1
    /// --cell A1
    #[ command( name = "get" ) ]
    Get
    {
      #[ arg( long, help = "Full URL of Google Sheet.\n\
      It has to be inside of '' to avoid parse errors.\n\
      Example: 'https://docs.google.com/spreadsheets/d/your_spreadsheet_id/edit?gid=0#gid=0'" ) ]
      url : String,

      #[ arg( long, help = "Sheet name.\nExample: Sheet1" ) ]
      tab : String,

      #[ arg( long, help = "Cell id. You can set it in format:\n \
      - A1, where A is column name and 1 is row number\n\
      Example: --cell A4" ) ]
      cell : String,
    },

    /// Updates a single cell of a specific sheet.
    /// 
    /// **Example**:
    /// 
    /// gspread cell set
    /// --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0'
    /// --tab tab1
    /// --cell A1
    /// --val 13
    #[ command( name = "set" ) ]
    Set
    {
      #[ arg( long, help = "Full URL of Google Sheet.\n\
      It has to be inside of '' to avoid parse errors.\n\
      Example: 'https://docs.google.com/spreadsheets/d/your_spreadsheet_id/edit?gid=0#gid=0'" ) ]
      url : String,

      #[ arg( long, help = "Sheet name.\nExample: Sheet1" ) ]
      tab : String,

      #[ arg( long, help = "Cell id. You can set it in format:\n \
      - A1, where A is column name and 1 is row number\n\
      Example: --cell A4" ) ]
      cell : String,

      #[ arg( long, help = "Value you want to set. It can be written on any language.\nExample: --val hello" ) ]
      val : String
    }
  }

  /// # `command`
  ///
  /// Executes the specified subcommand for the `CELL` command.
  ///
  /// ## Parameters:
  /// - `client`:  
  ///   A `GspreadClient` enum.
  ///   - `Variants`: 
  ///     `SheetsType` variant is used for interacting with the Google Sheets API. 
  ///     `MockClient` variant is used for mock testing.
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
      Commands::Get { url, tab, cell } =>
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

        match actions::gspread_cell_get::action
        (
          client,
          spreadsheet_id,
          tab.as_str(),
          cell.as_str()
        )
        .await
        {
          Ok( value ) => println!( "Value: {}", value ),
          Err( error ) => println!( "Error:\n{}", error ),
        }
      },

      Commands::Set { url, tab, cell, val } =>
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

        match actions::gspread_cell_set::action
        (
          client,
          spreadsheet_id,
          tab.as_str(),
          cell.as_str(),
          val.as_str()
        )
        .await
        {
          Ok( number ) => println!( "You successfully update {} cell!", number ),
          Err( error ) => println!( "Error:\n{}", error ),
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
    Commands,
  };
}