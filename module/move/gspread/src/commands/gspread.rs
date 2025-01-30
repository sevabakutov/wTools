//!
//! Collection of Google Sheets API commands.
//!


mod private
{

  use clap::{ Subcommand, Parser };
  use gcore::client::Client;

  use crate::{gcore::Secret, *};
  use commands::
  {
    gspread_header,
    gspread_row,
    gspread_rows,
    gspread_cell,
    gspread_column,
    gspread_clear,
    gspread_clear_custom,
    gspread_copy
  };

  /// # CommonArgs
  ///
  /// Structure containing common command-line arguments for `gspread` commands.
  ///
  /// ## Fields:
  /// - `url`:  
  ///   The full URL of the Google Sheet.  
  ///   Example: `'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}'`
  /// - `tab`:  
  ///   The name of the specific sheet to target.  
  ///   Example: `Sheet1`
  #[ derive( Debug, Parser ) ]
  pub struct CommonArgs
  {
    #[ arg( long, help = "Full URL of Google Sheet.\n\
    It has to be inside of '' to avoid parse errors.\n\
    Example: 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}'" ) ]
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
  /// - `Row`: Updates or appends rows.
  /// - `Column`: Retrives a column. 
  /// - `Clear`: Clears a sheet.
  /// - `ClearCustom`: Clears a range specified bu row key and on-find arguments.
  /// - `Copy`: Copies a spreadsheet's sheet to other spreadsheet.
  ///
  /// ## Examples:
  /// - Retrieve the header:
  /// ```bash
  /// gspread header --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' --tab Sheet1
  /// ```
  /// - Retrieve all rows:
  /// ```bash
  /// gspread rows --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' --tab Sheet1
  /// ```
  /// - Retrieve a single cell:
  /// ```bash
  /// gspread cell get --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' --tab Sheet1 --cell A1
  /// ```
  /// - Update a single cell:
  /// ```bash
  /// gspread cell set --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' --tab Sheet1 --cell A1 --val NewVal
  /// ```
  /// - Update multiple cells in a single row:
  /// ```bash
  /// gspread cells set
  /// --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' --tab Sheet1 --select-row-by-key "id" --json '{"id": "2", "A": "1", "B": "2"}'
  /// ```
  /// - Update rows:
  /// ```bash
  /// gspread row update-custom 
  /// --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' --tab tab8 --json '{"A": "1", "B": "2"}' --key-by '["A", 800]' --on-fail append --on-find all
  /// ```
  /// - Append a new row:
  /// ```bash
  /// gspread row append 
  /// --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' --tab tab8 --json '{ "D": 800, "F": 400, "H": 200 }'
  /// ```
  /// - Retrive a column:
  /// ```bash
  /// gspread column get 
  /// --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' --tab tab8 --column-id 'A'
  /// ```
  /// - Clear sheet:
  /// ```bash
  /// gspread clear 
  /// --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' --tab tab8
  /// ```
  /// - Clear a range specified by row key:
  /// ```bash
  /// gspread clear-custom 
  /// --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' --tab tab1 --key-by '["A", 4]' --on-find all
  /// ```
  /// - Copy a sheet from a specified spreadsheet to the other one.
  /// ```bash
  /// gspread copy
  /// --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' --sheet-id 1484163460
  /// --dest 'https://docs.google.com/spreadsheets/d/{dest_spreadsheet_id}/edit?gid={dest_sheet_id}#gid={dest_sheet_id}'
  /// ```
  #[ derive( Debug, Subcommand ) ]
  pub enum Command
  {
    #[command( name = "header", about = "Retrieves the header (first row) of a specific sheet.", long_about = r#"
    
Retrieves the header (first row) of a specific sheet.

Example:  gspread header \
          --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' \
          --tab tab1
    "# )]
    Header( CommonArgs ),

    #[ command( name = "rows", about = "Retrieves all raws of a specific sheet but not header.", long_about = r#"
    
Retrieves all raws of a specific sheet but not header.

Example:  gspread rows \
          --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' \
          --tab tab1
    "# ) ]
    Rows( CommonArgs ),

    #[ command ( subcommand, name = "cell", about = "Retrieves or updates a single cell in specific sheet." ) ]
    Cell( gspread_cell::Commands ),

    #[ command( subcommand, name = "row", about = "Update or append a row." ) ]
    Row( gspread_row::Commands  ),

    #[ command( subcommand, name = "column", about = "Retreive a column." ) ]
    Column( gspread_column::Commands ),

    #[ command( name = "clear", about = "Clears a sheet.", long_about = r#"
  
Clears a sheet.

Example:  gspread clear \
          --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' \
          --tab tab1
    "# ) ]
    Clear( CommonArgs ),

    #[ command( name = "clear-custom", about = "Clears range sprecified by `key-by` and `on-find` action.", long_about = r#"
    
Clears range sprecified by `key-by` and `on-find` action.

Example:  gspread clear-custom \
          --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' \
          --tab tab1 \
          --key-by '["A", 4]' \
          --on-find all
    "# ) ]
    ClearCustom( gspread_clear_custom::Args ),

    #[ command( name = "copy", about = "Copies a spreadsheet's sheet to the other spreadsheet.", long_about = r#"

Copies a spreadsheet's sheet to the other spreadsheet.

Example:  gspread copy \
          --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' \
          --sheet-id 1484163460 \
          --dest 'https://docs.google.com/spreadsheets/d/{dest_spreadsheet_id}/edit?gid={dest_sheet_id}#gid={dest_sheet_id}'
    "# )]
    Copy( gspread_copy::Args )

  }

  /// # `command`
  ///
  /// Executes the appropriate `gspread` command.
  pub async fn command<S: Secret>
  (
    client : &Client<'_, S>,
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

      Command::Row( row_command ) =>
      {
        gspread_row::command( client, row_command ).await;
      },

      Command::Column( column_command ) =>
      {
        gspread_column::command( client, column_command ).await;
      },

      Command::Clear( clear_command ) => 
      {
        gspread_clear::command( client, clear_command ).await;
      },

      Command::ClearCustom( args ) =>
      {
        gspread_clear_custom::command( client, args ).await;
      },

      Command::Copy( args ) =>
      {
        gspread_copy::command( client, args ).await;
      }
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