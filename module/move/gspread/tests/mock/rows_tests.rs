use dotenv::dotenv;
use gspread::{actions::gspread::get_rows, GspreadClient, Secret};
use httpmock::prelude::*;
use gspread::ser::JsonValue;

#[tokio::test]
async fn test_get_rows_with_mock() {
  dotenv().ok();

  let secret = Secret::read();

  let server = MockServer::start();

  let body = r#"
  {
    "range": "tab2!A2:Z999",
    "majorDimension": "ROWS",
    "values": [
      ["Row2Col1", "Row2Col2"],
      ["Row3Col1", "Row3Col2"]
    ]
  }
  "#;

  let mock = server.mock( | when, then | {
    when.method(GET)
      .path("/v4/spreadsheets/12345/values/tab2!A2:Z");
    then.status(200)
      .header("Content-Type", "application/json")
      .body(body);
  } );

  let client = GspreadClient::builder()
  .with_endpoint( server.url("" ) )
  .with_secret( &secret )
  .build()
  .await
  .expect( "Some error while building the client." );

  let rows = get_rows( &client, "12345", "tab2" )
  .await
  .expect( "get_rows failed" );

  mock.assert();

  assert_eq!( rows.len(), 2 );
  assert_eq!( rows[0].len(), 2 );
  assert_eq!( rows[0][0], JsonValue::String( "Row2Col1".to_string() ) );
  assert_eq!( rows[0][1], JsonValue::String( "Row2Col2".to_string() ) );

  assert_eq!( rows[1].len(), 2);
  assert_eq!( rows[1][0], JsonValue::String( "Row3Col1".to_string() ) );
  assert_eq!( rows[1][1], JsonValue::String( "Row3Col2".to_string() ) );
}