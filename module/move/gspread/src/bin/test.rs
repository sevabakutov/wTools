use dotenv::dotenv;
use gspread::actions::gspread::{delete_rows, RowRange};
use gspread::*;
use gcore::ApplicationSecret;
use gcore::client::
{
  Auth,
  Client
};

#[ tokio::main ]
async fn main()
{
  dotenv().ok();

  let secret = ApplicationSecret::read();
  
  let auth = Auth::new( &secret );
  
  let client = Client::former()
  .auth( auth )
  .form();

  let spreadsheet_id = "15_AKiE2nWUYopNpSLsE_G1tBsiZRNoSfdU8VIuostLc";
  let sheet_id = "0";
  let range = RowRange::All;

  match delete_rows( &client, spreadsheet_id, sheet_id, range ).await
  {
    Ok( _ ) => println!("Row was deleted"),
    Err( error ) => eprintln!( "{error}" )
  }
}