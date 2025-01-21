

mod private
{
  use clap::Subcommand;
  use crate::*;
  use actions::*;
  use gspread::get_spreadsheet_id_from_url;
  use gcore::client::Client;

  /// # Commands
  ///
  /// Subcommands for the `ROW` command.
  ///
  /// ## Variants:
  ///
  /// ### `Append`
  /// Appends a new row to at the end of Google Sheet.
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
  /// - `json`:  
  ///   A string containing the key-value pairs for the new row.  
  ///   The keys are column names (only uppercase Latin letters, e.g. `"A"`, `"B"`, etc.),  
  ///   and the values are strings or other JSON-compatible data.  
  ///   Depending on the shell, you may need to escape quotes.  
  ///   Examples:  
  ///   1. `--json '{"A": "value1", "B": "value2"}'`  
  ///   2. `--json "{\\\"A\\\": \\\"value1\\\", \\\"B\\\": \\\"value2\\\"}"`
  ///
  /// **Example:**
  /// ```bash
  /// gspread cells append \
  ///   --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0' \
  ///   --tab 'Sheet1' \
  ///   --json '{"A": "Hello", "B": "World"}'
  /// ```
  ///
  /// ### `UpdateCustom`
  /// Updates one or more rows in a Google Sheet based on a custom key (or condition),
  /// and offers control over what to do if no rows are found.
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
  /// - `json`:  
  ///   A JSON string of column-value pairs that you want to update.  
  ///   The keys should be valid column names (uppercase letters only),  
  ///   and values are JSON-compatible.  
  ///   Example:  
  ///   `--json '{"A": "10", "B": "Some text"}'`
  ///
  /// - `key_by`:  
  ///   An expression specifying **which** rows to match.  
  ///   Example:  
  ///   or  
  ///   `--key-by '["columnX", value_to_find]'`  
  ///
  /// - `on_fail`:  
  ///   What to do if **no rows are found** matching the key.  
  ///   Possible values might be `Error`, `AppendRow`, or `Nothing` (depending on your logic).  
  ///
  /// - `on_find`:  
  ///   What to do if **one or multiple** rows are found.  
  ///   Possible values might be `UpdateFirstMatchedRow`, `UpdateLastMatchedRow`, or `UpdateAllMatchedRow`.
  ///
  /// **Example:**
  /// ```bash
  /// gspread cells update-custom \
  ///   --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0' \
  ///   --tab tab1 \
  ///   --json '{"A": "newVal", "B": "updatedVal"}' \
  ///   --key-by '["C", 12]' \
  ///   --on_fail append \
  ///   --on_find all
  /// ```
  #[ derive( Debug, Subcommand ) ]
  pub enum Commands
  {
    /// Appends a new row to at the end of Google Sheet.
    ///
    /// **Example:**
    /// ```bash
    /// gspread cells append \
    ///   --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0' \
    ///   --tab 'tab1' \
    ///   --json '{"A": "Hello", "B": "World"}'
    /// ```
    #[ command( name = "append" ) ]
    Append
    {
      #[ arg( long, help = "Full URL of Google Sheet.\n\
      It has to be inside of '' to avoid parse errors.\n\
      Example: 'https://docs.google.com/spreadsheets/d/your_spreadsheet_id/edit?gid=0#gid=0'" ) ]
      url : String,

      #[ arg( long, help = "Sheet name.\nExample: Sheet1" ) ]
      tab : String,

      #[ arg( long, help = "Value range. 
      The key is a column name (not a header name, but a column name, which can only contain Latin letters).
      Depending on the shell, different handling might be required.\n\
      Examples:\n\
      1. --json '{\"A\": \"1\", \"B\": \"2\"}'\n\
      2. --json '{\\\"A\\\": \\\"1\\\", \\\"B\\\": \\\"2\\\"}'\n" ) ]
      json : String
    },

    /// Updates one or more rows in a Google Sheet based on a custom key,
    /// with control over how to handle matches or missing rows.
    ///
    /// **Example:**
    /// ```bash
    /// gspread cells update-custom \
    ///   --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0' \
    ///   --tab 'tab1' \
    ///   --json '{"A": "newVal", "B": "updatedVal"}' \
    ///   --key-by '["C", 12]' \
    ///   --on_fail error \
    ///   --on_find first
    /// ```
    #[ command( name = "update-custom" ) ]
    UpdateCustom
    {
      #[ arg( long, help = "Full URL of Google Sheet.\n\
      It has to be inside of '' to avoid parse errors.\n\
      Example: 'https://docs.google.com/spreadsheets/d/your_spreadsheet_id/edit?gid=0#gid=0'" ) ]
      url : String,

      #[ arg( long, help = "Sheet name.\nExample: Sheet1" ) ]
      tab : String,

      #[ arg( long, help = "Value range. 
      The key is a column name (not a header name, but a column name, which can only contain Latin letters).
      Depending on the shell, different handling might be required.\n\
      Examples:\n\
      1. --json '{\"A\": \"1\", \"B\": \"2\"}'\n\
      2. --json '{\\\"A\\\": \\\"1\\\", \\\"B\\\": \\\"2\\\"}'\n" ) ]
      json : String,

      #[ arg( long, help = "A string with key pair view, like [\"A\", \"new_val\"], where is a column index." ) ]
      key_by : String,

      #[ arg( long, help = "Action to take if no rows are found.
      Available: 
        - none - Does nothing.
        - error - Return an error.
        - append - Append a new row at the end of sheet." ) ]
      on_fail : String,

      #[ arg( long, help = "Action to take if one or more rows are found.
      Available: 
        - all - Update all matched rows, with provided values.
        - first - Update first matched row with provided values.
        - last - Update last matched row with provided data." ) ]
      on_find : String
    }
  }

  pub async fn command
  (
    client : &Client<'_>,
    commands : Commands
  )
  {
    match commands
    {
      Commands::Append { url, tab, json } =>
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

        match actions::gspread_row_append::action( client, spreadsheet_id, &tab, &json ).await
        {
          Ok( updated_cells ) => println!
          ( 
            "Row was successfully append at the end of the sheet! Amount of updated cells: {} ",
            updated_cells
          ),

          Err( error ) => eprintln!( "Error\n{}", error )
        }
      },

      Commands::UpdateCustom { url, tab, json, key_by, on_fail, on_find } =>
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

        match actions::gspread_row_update_custom::action
        ( 
          client, 
          spreadsheet_id, 
          &tab, 
          &key_by, 
          &json, 
          &on_find, 
          &on_fail 
        ).await
        {
          Ok( updated_cells ) => println!( "Rows were successfully update. Updated cells: {}", updated_cells ),
          Err( error ) => eprintln!( "Error\n{}", error )
        }
      }
    }
  }
}

crate::mod_interface!
{
  own use
  {
    Commands,
    command
  };
}