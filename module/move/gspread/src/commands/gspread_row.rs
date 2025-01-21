

mod private
{
  use clap::Subcommand;

use crate::{actions::{self, gspread::get_spreadsheet_id_from_url}, gcore::client::Client};

  #[ derive( Debug, Subcommand ) ]
  pub enum Commands
  {
    #[ command( name= "append" ) ]
    Append
    {
      #[ arg( long ) ]
      url : String,

      #[ arg( long ) ]
      tab : String,

      #[ arg( long ) ]
      json : String
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