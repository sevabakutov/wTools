//!
//! Set cells tests.
//! In these examples:
//!   - url is /v4/spreadsheets/{spreadsheet_id}}/values/{range}
//!   - everything is fake: spreadsheet_id, sheet's name, range and response json
//! 

use dotenv::dotenv;
use gspread::{actions::gspread::update_row, GspreadClient, Secret};
use httpmock::prelude::*;

#[tokio::test]
async fn test_update_row_with_mock() {
  dotenv().ok();

  let secret = Secret::read();

  let server = MockServer::start();

  let mock_patch_a = server.mock( |when, then | {
    when.method( POST );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .body
      (
        r#"
        {
          "spreadsheetId": "12345",
          "updatedRange": "tab2!A5",
          "updatedRows": 1,
          "updatedColumns": 1,
          "updatedCells": 1,
          "updatedData": {
            "range": "tab2!A5",
            "values": [["Hello"]]
          }
        }
        "#
      );
  });

  let mock_patch_b = server.mock( |when, then | {
    when.method( POST );
    then.status( 200 )
      .header("Content-Type", "application/json")
      .body
      (
        r#"
        {
          "spreadsheetId": "12345",
          "updatedRange": "tab2!B5",
          "updatedRows": 1,
          "updatedColumns": 1,
          "updatedCells": 1,
          "updatedData": {
            "range": "tab2!B5",
            "values": [["World"]]
          }
        }
        "#
      );
  });

  let client = GspreadClient::builder()
  .with_endpoint( server.url("") )
  .with_secret( &secret )
  .build()
  .await
  .expect( "Some error while building the client." );

  let mut row_key_val = std::collections::HashMap::new();
  row_key_val.insert("A".to_string(), "Hello".to_string());
  row_key_val.insert("B".to_string(), "World".to_string());

  let batch_result = update_row(
      &client,
      "12345",
      "tab2",
      "5",
      row_key_val
  )
  .await
  .expect( "update_row failed in mock test" );

  mock_patch_a.assert();
  mock_patch_b.assert();

  assert_eq!( batch_result.spreadsheet_id.as_deref(), Some( "12345" ) );
  assert_eq!( batch_result.total_updated_cells, Some( 2 ) );
  assert_eq!( batch_result.total_updated_rows, Some( 2 ) );
  assert_eq!( batch_result.total_updated_columns, Some( 2 ) );

  if let Some( responses ) = &batch_result.responses 
  {
    assert_eq!( responses.len(), 2 );
  }
  
  println!( "Batch update result: {:?}", batch_result );
}