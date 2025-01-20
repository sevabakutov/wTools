use dotenv::dotenv;
use gspread::gcore::Secret;
use gspread::gcore::client::Client;
use gspread::actions::gspread::{ get_cell, set_cell };
use serde_json::json; 

/// # What
/// We check that reading a specific cell from a Google Spreadsheet returns the expected result.
///
/// # How
/// 1. Send `GET /1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/values/tab2!A2`.
/// 2. Return a `ValueRange`.
/// 3. Call `get_cell()`, passing the necessary parameters.
/// 4. Verify that the result matches the expected `"Vsevolod"`.
#[tokio::test]
async fn test_get_cell_should_work() 
{
  dotenv().ok();

  let secret = Secret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "Failed to build client" )
  .form();

  let spreadsheet_id = "1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU";

  let result = get_cell
  ( 
    &client, 
    spreadsheet_id, 
    "tab2", 
    "A2" 
  )
  .await
  .expect( "get_cell failed" );

  assert_eq!( result, serde_json::Value::String( "Vsevolod".to_string() ) );
}


/// # What
/// We check that setting a value in a specific cell of a Google Spreadsheet works correctly.
///
/// # How
/// 1. Send `PUT /1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/values/tab2!A1?valueInputOption=RAW`.
/// 2. Return `UpdateValuesResponse`.
/// 3. Call `set_cell()`, passing the table, sheet, cell, and the value to set.
/// 4. Verify that the number of updated cells, rows, and columns matches the expected result.
#[tokio::test]
async fn test_set_cell_should_work() {
  dotenv().ok();

  let secret = Secret::read();

  let spreadsheet_id = "1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU";

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "Failed to build a client." )
  .form();

  let result = set_cell
  ( 
    &client, 
    spreadsheet_id, 
    "tab4", 
    "A1", 
    serde_json::Value::String( "Val".to_string() ) 
  )
  .await
  .expect( "set_cell failed with mock" );

  assert_eq!( result.spreadsheet_id.as_deref(), Some( spreadsheet_id ) );
  assert_eq!( result.updated_range.as_deref(), Some( "'tab4'!A1" ) );
  assert_eq!( result.updated_rows, Some( 1 ) );
  assert_eq!( result.updated_columns, Some( 1 ) );
  assert_eq!( result.updated_cells, Some( 1 ) );

  if let Some( updated_data ) = &result.updated_data 
  {
    let values = updated_data.values.as_ref().unwrap();
    assert_eq!( values, &vec![ vec![ json!( "Val" ) ] ] );
  }
}