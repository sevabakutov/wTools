

mod private
{
  use clap::Subcommand;
  use debug::{Report, RowWrapper};
  use serde_json::json;
  
  use crate::*;
  use gcore::client::Client;
  use actions::
  {
    self,
    utils::get_spreadsheet_id_from_url
  };

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
  /// gspread row append \
  ///   --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0' \
  ///   --tab 'tab1' \
  ///   --json '{"A": "Hello", "B": "World"}'
  /// ```
  /// 
  /// ### `Update`
  /// Updates a specific row.
  /// 
  /// **Arguments**
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
  ///   `--json '{"id": 2, "A": 10, "B": "Some text"}'`
  /// 
  /// - `select_row_by_key`:  
  ///   A string specifying the identifier of the row to update.  
  ///   Example: `"id"`.
  /// 
  /// **Example:**
  /// ```bash
  /// gspread row update \
  /// --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0' \
  /// --tab tab1 \
  /// --select-row-by-key "id" \
  /// --json '{"id": 2, "A": 1, "B": 2}'
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
  /// gspread row update-custom \
  ///   --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0' \
  ///   --tab tab1 \
  ///   --json '{"A": "newVal", "B": "updatedVal"}' \
  ///   --key-by '["C", 12]' \
  ///   --on_fail append \
  ///   --on_find all
  /// ```
  /// 
  /// ### `Get`
  /// Retreives a specific row from a Google Sheet.
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
  /// - `row-key`:
  ///   Row key (id). The range starts from 1.
  ///   Example:
  ///   `row-key 2`
  /// 
  /// **Example:**
  /// 
  /// gspread row get
  /// --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0' \
  /// --tab 'tab1'
  /// 
  /// ### `GetCustom`
  /// Retrieves one or more rows from a Google Sheet based on a custom key condition,
  /// specifying how to handle multiple matches.
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
  /// - `key_by`:  
  ///   A JSON array of the form `[<column>, <value>]`, defining which rows to match.  
  ///   For instance, if you pass `["A", "Hello"]`, the function will look in column `A`  
  ///   for cells whose value equals `"Hello"`.  
  ///   Example:  
  ///   `--key-by '["C", 12]'`
  ///
  /// - `on_find`:  
  ///   Defines how to handle situations where multiple rows match the key.  
  ///   Possible values (depending on your logic):  
  ///   - `all`: Return **all** matched rows,  
  ///   - `first`: Return **only the first** matched row,  
  ///   - `last`: Return **only the last** matched row.
  ///
  /// **Example:**
  /// ```bash
  /// gspread row get-custom \
  ///   --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0' \
  ///   --tab 'Sheet1' \
  ///   --key-by '["C", 12]' \
  ///   --on-find all
  /// ```
  #[ derive( Debug, Subcommand ) ]
  pub enum Commands
  {
    /// Appends a new row to at the end of Google Sheet.
    ///
    /// **Example:**
    /// 
    /// gspread row append
    ///   --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0'
    ///   --tab 'tab1'
    ///   --json '{"A": "Hello", "B": "World"}'
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
      1. --json '{\"A\": 1, \"B\": \"Hello\"}'\n\
      2. --json '{\\\"A\\\": 1, \\\"B\\\": \\\"Hello\\\"}'\n" ) ]
      json : String
    },

    /// Updates a specific row in a Google Sheet with a given set of values.
    /// 
    /// **Example**:
    /// 
    /// gspread row update
    /// --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0'
    /// --tab tab1
    /// --select-row-by-key "id"
    /// --json '{"id": 2, "A": 1, "B": 2}'
    #[ command( name = "update" ) ]
    Update
    {
      #[ arg( long, help = "Identifier of a row. Available identifiers: id (row's unique identifier).\n\
      Example: --select_row_by_key \"id\"" ) ]
      select_row_by_key : String,
      
      #[ arg( long, help = "Value range. It must contain select_row_by_key.
      The key is a column name (not a header name, but a column name, which can only contain Latin letters).
      Every key and value must be a string.
      Depending on the shell, different handling might be required.\n\
      Examples:\n\
      1. --json '{\"id\": 3, \"A\": 1, \"B\": 2}'\n\
      3. --json '{\\\"id\\\": 3, \\\"A\\\": \\\"Hello\\\", \\\"B\\\": true}'\n" ) ]
      json : String,

      #[ arg( long, help = "Full URL of Google Sheet.\n\
      It has to be inside of '' to avoid parse errors.\n\
      Example: 'https://docs.google.com/spreadsheets/d/your_spreadsheet_id/edit?gid=0#gid=0'" ) ]
      url : String,

      #[ arg( long, help = "Sheet name.\nExample: Sheet1" ) ]
      tab : String
    },

    /// Updates one or more rows in a Google Sheet based on a custom key,
    /// with control over how to handle matches or missing rows.
    ///
    /// **Example:**
    /// 
    /// gspread row update-custom
    ///   --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0'
    ///   --tab 'tab1'
    ///   --json '{"A": "newVal", "B": "updatedVal"}'
    ///   --key-by '["C", 12]'
    ///   --on-fail error
    ///   --on-find first
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
      1. --json '{\"A\": 1, \"B\": 2}'\n\
      2. --json '{\\\"A\\\": \\\"Hello\\\", \\\"B\\\": \\\"World\\\"}'\n" ) ]
      json : String,

      #[ arg( long, help = "A string with key pair view, like [\"A\", \"new_val\"], where A is a column index." ) ]
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
    },

    /// Retreives a specific row from a Google Sheet.
    /// 
    /// **Example:**
    /// 
    /// gspread row get
    /// --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0' \
    /// --tab 'tab1'
    /// --row-key 2
    #[ command( name = "get" ) ]
    Get
    {
      #[ arg( long, help = "Full URL of Google Sheet.\n\
      It has to be inside of '' to avoid parse errors.\n\
      Example: 'https://docs.google.com/spreadsheets/d/your_spreadsheet_id/edit?gid=0#gid=0'" ) ]
      url : String,

      #[ arg( long, help = "Sheet name.\nExample: Sheet1" ) ]
      tab : String,

      #[ arg( long, help = "A row key. Example: row_key=2" ) ]
      row_key : u32,
    },

    /// Retrieves one or more rows from a Google Sheet based on a custom key condition,
    /// specifying how to handle multiple matches.
    /// 
    /// **Example:**
    /// 
    /// gspread row get-custom
    ///   --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0' \
    ///   --tab 'tab1'
    ///   --key-by '["C", 12]'
    ///   --on-find all
    #[ command( name = "get-custom" ) ]
    GetCustom
    {
      #[ arg( long, help = "Full URL of Google Sheet.\n\
      It has to be inside of '' to avoid parse errors.\n\
      Example: 'https://docs.google.com/spreadsheets/d/your_spreadsheet_id/edit?gid=0#gid=0'" ) ]
      url : String,

      #[ arg( long, help = "Sheet name.\nExample: Sheet1" ) ]
      tab : String,

      #[ arg( long, help = "A string with key pair view, like [\"A\", \"val\"], where A is a column index." ) ]
      key_by : String,

      #[ arg( long, help = "Action to take if one or more rows are found.
      Available: 
        - all - Retreive all matched rows.
        - first - Retreive first matched row.
        - last - Retreive last matched row." ) ]
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
          Ok( val ) => 
          {
            match val
            {
              0 => println!( "Row key was not found, provided action has worked." ),
              _ => println!( "{} cells were sucsessfully updated!", val )
            }
          },
          Err( error ) => eprintln!( "Error\n{}", error )
        }
      },

      Commands::Update { select_row_by_key, json, url, tab } =>
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

        match actions::gspread_row_update::action
        (
          client,
          &select_row_by_key,
          &json,
          spreadsheet_id,
          &tab
        )
        .await
        {
          Ok( val ) => println!( "{} cells were sucsessfully updated!", val ),
          Err( error ) => println!( "Error:\n{}", error )
        }
      },

      Commands::Get { url, tab, row_key } => 
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

        match actions::gspread_row_get::action
        (
          client, 
          spreadsheet_id, 
          &tab,
          json!( row_key )
        )
        .await
        {
          Ok( row ) => 
          {
            let row_wrapped = RowWrapper
            {
              max_len : row.len(), 
              row : row
            };

            println!( "Row:\n{}", Report{ rows: vec![ row_wrapped ] } );
          },
          Err( error ) => eprintln!( "Error:\n{}", error ),
        }
      }

      Commands::GetCustom { url, tab, key_by, on_find } =>
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

        match actions::gspread_row_get_custom::action
        (
          client, 
          spreadsheet_id, 
          &tab, 
          &key_by, 
          &on_find
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
    Commands,
    command
  };
}