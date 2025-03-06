//!
//! Collection of subcommands fo command "cell"
//!

mod private
{

  use clap::Subcommand;
  use crate::*;

  use gcore::client::Client;
  use gcore::Secret;
  use actions;
  use actions::utils::get_spreadsheet_id_from_url;

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
  ///   Example: `'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}'`.
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
  /// --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' \
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
  ///   Example: `'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}'`.
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
  /// --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' \
  /// --tab tab1 \
  /// --cell A1 \
  /// --val 13
  /// ```
  #[ derive( Debug, Subcommand ) ]
  #[ command( long_about = "\n\nSubcommands for the `CELL` command, used to interact with individual cells in a Google Sheet." ) ]
  pub enum Commands
  {
<<<<<<< HEAD
    /// Command to get a value from a sheet's cell
    #[ command( name = "get" ) ]
=======
    #[ command( name = "get", about = "Retrieves a single cell.", long_about = r#"
---------------------------------------------------------------------------------------------------------------
                                              CELL GET                                                         
---------------------------------------------------------------------------------------------------------------
 ● Description:                                                                                                
   ↓ ↓ ↓ ↓ ↓ ↓                                                                                                 
                                                                                                               
 Retrieves a single cell specified by the `--cell` argument in A1 notation.                                    
                                                                                                               
---------------------------------------------------------------------------------------------------------------
 ● Command example:                                                                                            
   ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓                                                                                             
                                                                                                               
 cargo run gspread cell get \                                                                                            
         --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' \  
         --tab tab1 \                                                                                          
         --cell A1                                                                                             
                                                                                                               
---------------------------------------------------------------------------------------------------------------
 ● Output:  Prints the value of the cell:                                                                      
   ↓ ↓ ↓ ↓                                                                                                     
                                                                                                               
 Value: "Name"                                                                                                 
                                                                                                               
---------------------------------------------------------------------------------------------------------------
 ● Errors:                                                                                                     
   ↓ ↓ ↓ ↓                                                                                                     
                                                                                                               
  ◦ Error::ApiError:                                                                                           
    ----------------------------------------------------------------                                         
     Occurs if the Google Sheets API returns an error,                                                       
     such as an invalid spreadsheet ID, insufficient permissions                                             
     or invalid sheet name.                                                                                  
    ----------------------------------------------------------------                                         
                                                                                                               
  ◦ Error::InvalidURL:                                                                                         
    ----------------------------------------------------------------------                                   
     Occurs when you pass a URL with an invalid spreadsheet format.                                          
    ----------------------------------------------------------------------                                   
                                                                                                               
---------------------------------------------------------------------------------------------------------------
    "# ) ]
>>>>>>> updstream/alpha
    Get
    {
      #[ arg( long, help = "Full URL of Google Sheet.\n\
      It has to be inside of '' to avoid parse errors.\n\
<<<<<<< HEAD
      Example: 'https://docs.google.com/spreadsheets/d/your_spreadsheet_id/edit?gid=0#gid=0'" ) ]
=======
      Example: 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}'" ) ]
>>>>>>> updstream/alpha
      url : String,

      #[ arg( long, help = "Sheet name.\nExample: Sheet1" ) ]
      tab : String,

      #[ arg( long, help = "Cell id. You can set it in format:\n \
      - A1, where A is column name and 1 is row number\n\
      Example: --cell A4" ) ]
      cell : String,
    },

<<<<<<< HEAD
    /// Command to set a new value to a sheet's cell.
    #[ command( name = "set" ) ]
=======
    #[ command( name = "set", about = "Updates a single cell.", long_about = r#"
---------------------------------------------------------------------------------------------------------------
                                             CELL SET                                                          
---------------------------------------------------------------------------------------------------------------
● Description:                                                                                                
   ↓ ↓ ↓ ↓ ↓ ↓                                                                                                 
                                                                                                               
 Updates a single cell specified by `--cell` (in A1 notation) and `--val`.                                     
                                                                                                               
---------------------------------------------------------------------------------------------------------------
 ● Command example:                                                                                            
   ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓                                                                                             
                                                                                                               
 cargo run gspread cell set \                                                                                  
         --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' \  
         --tab tab1 \                                                                                          
         --cell A1 \                                                                                           
         --val 'New Value'                                                                                     
                                                                                                               
---------------------------------------------------------------------------------------------------------------
 ● Output:  Prints a message indicating the number of cells updated:                                           
   ↓ ↓ ↓ ↓                                                                                                     
                                                                                                               
 You successfully update 1 cell!                                                                               
                                                                                                               
---------------------------------------------------------------------------------------------------------------
 ● Errors:                                                                                                     
   ↓ ↓ ↓ ↓                                                                                                     
                                                                                                               
  ◦ Error::ApiError:                                                                                           
    ----------------------------------------------------------------                                         
     Occurs if the Google Sheets API returns an error,                                                       
     such as an invalid spreadsheet ID, insufficient permissions                                             
     or invalid sheet name.                                                                                
    ----------------------------------------------------------------                                         
                                                                                                               
  ◦ Error::ParseError:                                                                                         
    ---------------------------------------------------------                                               
     Occurs when serde_json::Value parse error                                                               
    ---------------------------------------------------------                                                
                                                                                                               
  ◦ Error::InvalidURL:                                                                                         
    ----------------------------------------------------------------------                                   
     Occurs when you pass a URL with an invalid spreadsheet format.                                          
    ----------------------------------------------------------------------                                   
                                                                                                               
---------------------------------------------------------------------------------------------------------------
    "# ) ]
>>>>>>> updstream/alpha
    Set
    {
      #[ arg( long, help = "Full URL of Google Sheet.\n\
      It has to be inside of '' to avoid parse errors.\n\
<<<<<<< HEAD
      Example: 'https://docs.google.com/spreadsheets/d/your_spreadsheet_id/edit?gid=0#gid=0'" ) ]
=======
      Example: 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}'" ) ]
>>>>>>> updstream/alpha
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
  ///   A `Client` type.
  /// - `commands`:  
  ///   A variant of the `Commands` enum specifying the operation to execute.
  ///
  /// ## Errors:
  /// - Prints an error message if the spreadsheet ID extraction, retrieval, or update fails.
  pub async fn command< S : Secret >
  (
    client : &Client< '_, S >,
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