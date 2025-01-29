//!
//! Command copy
//! 

mod private
{
  use clap::Parser;

  use crate::*;
  use gcore::client::Client;
  use actions::
  {
    self, 
    utils::get_spreadsheet_id_from_url
  };

  /// # Args
  ///
  /// Structure containing arguments of `copy` command.
  ///
  /// ## Fields:
  /// - `url`:  
  ///   The full URL of the Google Sheet.  
  ///   Example: `'https://docs.google.com/spreadsheets/d/your_spreadsheet_id/edit?gid=0#gid=0'`
  /// - `sheet_id`:  
  ///   Source sheet id.  
  ///   Example: `1484163460`
  /// - `dest`:  
  ///   Destination spreadsheet url.  
  ///   Example: `https://docs.google.com/spreadsheets/d/your_spreadsheet_id/edit?gid=0#gid=0`
  #[ derive( Debug, Parser ) ]
  pub struct Args
  {
    #[ arg( long, help = "Full URL of Google Sheet.\n\
    It has to be inside of '' to avoid parse errors.\n\
    Example: 'https://docs.google.com/spreadsheets/d/your_spreadsheet_id/edit?gid=0#gid=0'" ) ]
    pub url : String,

    #[ arg( long, help = "Source Sheet id. You can find it in a sheet url, in the 'gid' query parameter.\n\
    Example: https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={SHEET_ID}#gid={SHEET_ID}\n\
    Sheet Id Example: 1484163460" ) ]
    pub sheet_id : String,

    #[ arg( long, help = "Destination spreadsheet id. 
    It has to be inside of '' to avoid parse errors.\n\
    Example: 'https://docs.google.com/spreadsheets/d/your_spreadsheet_id/edit?gid=0#gid=0'" ) ]
    pub dest : String
  }

  pub async fn command
  (
    client : &Client<'_>,
    args : Args
  )
  {
    match args
    {
      Args { url, sheet_id, dest } =>
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

        let dest = match get_spreadsheet_id_from_url( &dest ) 
        {
          Ok( id ) => id,
          Err( error ) => 
          {
            eprintln!( "Error extracting spreadsheet ID: {}", error );
            return;
          }
        };

        match actions::gspread_copy::action
        (
          client,
          spreadsheet_id,
          &sheet_id,
          dest
        )
        .await
        {
          Ok( title ) => println!( "A sheet was successfully copied to a new one with title '{title}'" ),
          Err( error ) => eprintln!( "Error:\n{error}" )
        }
      }
    }
  }

}

crate::mod_interface!
{
  own use
  {
    Args,
    command
  };
}