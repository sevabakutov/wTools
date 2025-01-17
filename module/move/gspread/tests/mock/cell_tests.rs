use gspread::actions::gspread::set_cell;
use gspread::{actions::gspread::get_cell, GspreadClient};
use httpmock::prelude::*;
use dotenv::dotenv;
use gspread::secret::Secret;
use gspread::ser::JsonValue;
use serde_json::json;

#[tokio::test]
async fn test_get_cell_with_mock_should_work() {
  dotenv().ok();

  let secret = Secret::read();

  let server = MockServer::start();

  let body = r#"
  {
    "range": "tab2!R2C1:R2C1",
    "majorDimension": "ROWS",
    "values": [
      [ "Steeve" ]
    ]
  }
  "#;

  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/v4/spreadsheets/12345/values/tab2!R2C1" );
    then
      .status( 200 )
      .header( "Content-Type", "application/json" )
      .body( body );
  } );

  let client = GspreadClient::builder()
  .with_endpoint( server.url("") )
  .with_secret( &secret )
  .build()
  .await
  .expect( "Some error while building the client." );

  let result = get_cell( &client, "12345", "tab2", "R2C1" )
  .await
  .expect( "get_cell failed" );

  mock.assert();

  assert_eq!( result, JsonValue::String( "Steeve".to_string() ) );
}

#[tokio::test]
async fn test_set_cell_with_mock_should_work() {
  dotenv().ok();

  let secret = Secret::read();

  let server = MockServer::start();

  let response_body = r#"
  {
    "spreadsheetId": "12345",
    "updatedRange": "tab2!A1",
    "updatedRows": 1,
    "updatedColumns": 1,
    "updatedCells": 1,
    "updatedData": {
      "range": "tab2!A1",
      "values": [
        ["TestValue"]
      ]
    }
  }
  "#;

  let mock = server.mock( | when, then | {
    when.method( POST )
      .path( "/v4/spreadsheets/12345/values/tab2!A1" )
      .query_param( "valueInputOption", "USER_ENTERED" );
    then
      .status( 200 )
      .header( "Content-Type", "application/json" )
      .body( response_body );
  });

  let client = GspreadClient::builder()
  .with_endpoint( server.url("") )
  .with_secret( &secret )
  .build()
  .await
  .expect( "Some error while building the client." );

  let result = set_cell( &client, "12345", "tab2", "A1", "Val" )
  .await
  .expect( "set_cell failed with mock" );

  mock.assert();

  assert_eq!( result.spreadsheet_id.as_deref(), Some( "12345" ) );
  assert_eq!( result.updated_range.as_deref(), Some( "tab2!A1" ) );
  assert_eq!( result.updated_rows, Some( 1 ) );
  assert_eq!( result.updated_columns, Some( 1 ) );
  assert_eq!( result.updated_cells, Some( 1 ) );

  if let Some( updated_data ) = &result.updated_data 
  {
    let values = updated_data.values.as_ref().unwrap();
    assert_eq!( values, &vec![ vec![ json!( "Val" ) ] ] );
  }
}