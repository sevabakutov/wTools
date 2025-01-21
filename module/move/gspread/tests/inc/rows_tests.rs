use std::collections::HashMap;

use dotenv::dotenv;
use gspread::gcore::Secret;
use gspread::actions::gspread::{get_rows, append_row};
use gspread::gcore::client::Client; 

/// # What
/// We check that requesting all rows from the second row onward (below the header)
/// correctly parses the response and returns the expected result.
///
/// # How
/// 1. Send `GET /1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/values/tab2!A2:Z999`.
/// 2. Return `ValueRange`.
/// 3. Call `get_rows()`, passing the table and sheet.
/// 4. Verify that the array of returned rows matches the expected structure and values.
#[tokio::test]
async fn test_get_rows_should_work() 
{
  dotenv().ok();

  let secret = Secret::read();

  let spreadsheet_id = "1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU";

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "Failed to build a client" )
  .form();

  let rows = get_rows
  ( 
    &client, 
    spreadsheet_id, 
    "tab2" 
  )
  .await
  .expect( "get_rows failed" );

  assert_eq!( rows.len(), 2 );
  assert_eq!( rows[0].len(), 3 );
  assert_eq!( rows[0][0], serde_json::Value::String( "Vsevolod".to_string() ) );
  assert_eq!( rows[0][1], serde_json::Value::String( "Bakutov".to_string() ) );
  assert_eq!( rows[0][2], serde_json::Value::String( "20".to_string() ) );

  assert_eq!( rows[1].len(), 3  );
  assert_eq!( rows[1][0], serde_json::Value::String( "Victor".to_string() ) );
  assert_eq!( rows[1][1], serde_json::Value::String( "Ovyanik".to_string() ) );
  assert_eq!( rows[1][2], serde_json::Value::String( "85".to_string() ) );
}


#[ tokio::test ]
async fn append_row_should_work()
{
  dotenv().ok();

  let secret = Secret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "Failed to form a client." )
  .form();

  let spreadheet_id = "1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU";
  let mut row_key_val = HashMap::new();
  row_key_val.insert( "A".to_string(), serde_json::Value::Bool( true ) );
  row_key_val.insert( "C".to_string(), serde_json::Value::Bool( false ) );

  println!( "{:?}", row_key_val );

  let result = append_row
  ( 
    &client, 
    spreadheet_id, 
    "tab8", 
    &row_key_val 
  )
  .await
  .expect( "append_row failed!" );

  assert_eq!( result.spreadsheet_id.unwrap(), spreadheet_id );
  assert_eq!( result.updates.unwrap().updated_cells, Some( 3 ) );
}