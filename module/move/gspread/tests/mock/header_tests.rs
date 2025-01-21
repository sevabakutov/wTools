use httpmock::prelude::*;

use serde_json::json;
use gspread::actions::gspread::get_header;
use gspread::gcore::client::
{
  Client, 
  Dimension, 
  ValueRange
};

/// # What
/// We check that requesting the header row (first row) of a sheet in a Google Spreadsheet
/// returns the correct set of column values.
///
/// # How
/// 1. Start a `MockServer` to send `GET /12345/values/tab2!A1:Z1`.
/// 2. Return a predefined `ValueRange` containing `"ID"`, `"Name"`, and `"Email"`.
/// 3. Call `get_header()`, passing the table and sheet.
/// 4. Verify that the returned header row has exactly three columns as expected.
#[tokio::test]
async fn test_get_header_with_mock_should_work() 
{
  let spreadsheet_id = "12345";
  let body = ValueRange
  {
    major_dimension : Some( Dimension::Row ),
    range : Some( "tab2!A1:Z1".to_string() ),
    values : Some( vec![ vec![ json!( "ID" ), json!( "Name" ), json!( "Email" ) ] ] )
  };

  let server = MockServer::start();

  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/12345/values/tab2!A1:Z1" );
    then.status(200)
      .header( "Content-Type", "application/json" )
      .json_body_obj( &body );
  });

  let client = Client::former()
  .endpoint( server.url("") )
  .form();

  let header = get_header( &client, spreadsheet_id, "tab2" )
  .await
  .expect( "get_header failed" );

  mock.assert();

  assert_eq!( header.len(), 1, "Header should have one row" );
  assert_eq!( header[0].len(), 3, "Header row should have 3 columns" );

  assert_eq!( header[0][0], serde_json::Value::String( "ID".to_string() ) );
  assert_eq!( header[0][1], serde_json::Value::String( "Name".to_string() ) );
  assert_eq!( header[0][2], serde_json::Value::String( "Email".to_string() ) );
}