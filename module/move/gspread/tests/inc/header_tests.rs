use dotenv::dotenv;
use gspread::gcore::Secret;
use gspread::actions::gspread::get_header;
use gspread::gcore::client::Client; 

/// # What
/// We check that requesting the header row (first row) of a sheet in a Google Spreadsheet
/// returns the correct set of column values.
///
/// # How
/// 1. Send `GET /1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/values/tab2!A1:Z1`.
/// 2. Return a `ValueRange`.
/// 3. Call `get_header()`, passing the table and sheet.
/// 4. Verify that the returned header row has exactly three columns as expected.
#[tokio::test]
async fn test_get_header_with_mock_should_work() {
  dotenv().ok();

  let secret = Secret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "Failed to build a client" )
  .form();

  let spreadsheet_id = "1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU";

  let header = get_header
  ( 
    &client, 
    spreadsheet_id, 
    "tab2" 
  )
  .await
  .expect( "get_header failed" );

  assert_eq!( header.len(), 1, "Header should have one row" );
  assert_eq!( header[0].len(), 3, "Header row should have 3 columns" );

  assert_eq!( header[0][0], serde_json::Value::String( "Name".to_string() ) );
  assert_eq!( header[0][1], serde_json::Value::String( "Surname".to_string() ) );
  assert_eq!( header[0][2], serde_json::Value::String( "Age".to_string() ) );
}