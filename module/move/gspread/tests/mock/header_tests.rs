use dotenv::dotenv;
use gspread::{actions::gspread::get_header, GspreadClient, Secret};
use httpmock::prelude::*;
use gspread::ser::JsonValue;


#[tokio::test]
async fn test_get_header_with_mock_should_work() {
  dotenv().ok();

  let secret = Secret::read();

  let server = MockServer::start();

  let body = r#"
  {
    "range": "tab2!A1:Z1",
    "majorDimension": "ROWS",
    "values": [
      ["ID", "Name", "Email"]
    ]
  }
  "#;

  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/v4/spreadsheets/12345/values/tab2!A1:Z1" );
    then.status(200)
      .header( "Content-Type", "application/json" )
      .body( body );
  });

  let client = GspreadClient::builder()
  .with_endpoint( server.url("") )
  .with_secret( &secret )
  .build()
  .await
  .expect( "Some error while building the client." );

  let header = get_header( &client, "12345", "tab2" )
  .await
  .expect( "get_header failed" );

  mock.assert();

  assert_eq!( header.len(), 1, "Header should have one row" );
  assert_eq!( header[0].len(), 3, "Header row should have 3 columns" );

  assert_eq!( header[0][0], JsonValue::String( "ID".to_string() ) );
  assert_eq!( header[0][1], JsonValue::String( "Name".to_string() ) );
  assert_eq!( header[0][2], JsonValue::String( "Email".to_string() ) );
}