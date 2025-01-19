//!
//! Collection of Google Sheets API commands.
//!


mod private
{

  use clap::{ Subcommand, Parser };
use client::client::Client;

  use crate::*;
  use commands::
  {
    gspread_header,
    gspread_rows,
    gspread_cell,
    gspread_cells
  };

  /// # CommonArgs
  ///
  /// Structure containing common command-line arguments for `gspread` commands.
  ///
  /// ## Fields:
  /// - `url`:  
  ///   The full URL of the Google Sheet.  
  ///   Example: `'https://docs.google.com/spreadsheets/d/your_spreadsheet_id/edit?gid=0#gid=0'`
  /// - `tab`:  
  ///   The name of the specific sheet to target.  
  ///   Example: `Sheet1`
  #[ derive( Debug, Parser ) ]
  pub struct CommonArgs
  {
    #[ arg( long, help = "Full URL of Google Sheet.\n\
    It has to be inside of '' to avoid parse errors.\n\
    Example: 'https://docs.google.com/spreadsheets/d/your_spreadsheet_id/edit?gid=0#gid=0'" ) ]
    pub url : String,

    #[ arg( long, help = "Sheet name.\nExample: Sheet1" ) ]
    pub tab : String
  }

  /// # Command
  ///
  /// Enum representing all available `gspread` commands.
  ///
  /// ## Variants:
  /// - `Header`: Retrieves the header (first row) of a specific sheet.
  /// - `Rows`: Retrieves all rows (excluding the header) from a specific sheet.
  /// - `Cell`: Retrieves or updates a single cell in a sheet.
  /// - `Cells`: Updates multiple cells in a specific row.
  ///
  /// ## Examples:
  /// - Retrieve the header:
  /// ```bash
  /// gspread header --url 'https://docs.google.com/spreadsheets/d/.../edit?gid=0#gid=0' --tab Sheet1
  /// ```
  /// - Retrieve all rows:
  /// ```bash
  /// gspread rows --url 'https://docs.google.com/spreadsheets/d/.../edit?gid=0#gid=0' --tab Sheet1
  /// ```
  /// - Retrieve a single cell:
  /// ```bash
  /// gspread cell get --url 'https://docs.google.com/spreadsheets/d/.../edit?gid=0#gid=0' --tab Sheet1 --cell A1
  /// ```
  /// - Update a single cell:
  /// ```bash
  /// gspread cell set --url 'https://docs.google.com/spreadsheets/d/.../edit?gid=0#gid=0' --tab Sheet1 --cell A1 --val NewVal
  /// ```
  /// - Update multiple cells in a single row:
  /// ```bash
  /// gspread cells set
  /// --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0' --tab Sheet1 --select-row-by-key "id" --json '{"id": "2", "A": "1", "B": "2"}'
  /// ```
  #[ derive( Debug, Subcommand ) ]
  pub enum Command
  {
    /// Retrieves the header (first row) of a specific sheet.
    ///
    /// **Example:**
    /// 
    /// gspread header
    /// --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0'
    /// --tab tab1
    #[ command ( name = "header" ) ]
    Header
    (
      CommonArgs
    ),

    /// Retrieves all raws of a specific sheet but not header.
    /// 
    /// **Example**:
    /// 
    /// gspread rows
    /// --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0'
    /// --tab tab1
    #[ command( name = "rows" ) ]
    Rows
    (
      CommonArgs
    ),

    /// Retrieves or updates a single cell in specific sheet.
    #[ command ( subcommand, name = "cell" ) ]
    Cell
    (
      gspread_cell::Commands
    ),

    /// Updates multiple values in a single row of a specific sheet.
    #[ command ( subcommand, name = "cells" ) ]
    Cells
    (
      gspread_cells::Commands
    )

  }

  /// # `command`
  ///
  /// Executes the appropriate `gspread` command.
  ///
  /// ## Parameters:
  /// - `client`:  
  ///   A `GspreadClient` enum.
  ///   - `Variants`: 
  ///     `SheetsType` variant is used for interacting with the Google Sheets API. 
  ///     `MockClient` variant is used for mock testing.
  /// - `command`:  
  ///   The `Command` enum specifying which operation to execute.
  pub async fn command
  (
    client : &Client,
    command : Command,
  )
  {
    match command
    {
      Command::Header( header_command ) =>
      {
        gspread_header::command( client, header_command ).await;
      },

      Command::Rows( rows_command ) =>
      {
        gspread_rows::command( client, rows_command ).await;
      },

      Command::Cell( cell_command ) =>
      {
        gspread_cell::command( client, cell_command ).await;
      },

      Command::Cells( cells_command) =>
      {
        gspread_cells::command( client, cells_command ).await;
      },
    }
  }

}

crate::mod_interface!
{
  own use
  {
    CommonArgs,
    Command,
    command,
  };
}