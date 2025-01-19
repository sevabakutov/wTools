use gspread::{actions::gspread::get_rows, gcore::client::{Client, Dimension, ValueRange}};
use httpmock::prelude::*;
use gspread::ser::JsonValue;
use serde_json::json;

/// # What
/// We check that requesting all rows from the second row onward (below the header)
/// correctly parses the response and returns the expected result.
///
/// # How
/// 1. Start a `MockServer` to send `GET /12345/values/tab2!A2:Z999`.
/// 2. Return a predefined `ValueRange` with multiple rows of data.
/// 3. Call `get_rows()`, passing the table and sheet.
/// 4. Verify that the array of returned rows matches the expected structure and values.
#[tokio::test]
async fn test_get_rows_with_mock() {
  let spreadsheet_id = "12345";
  let body = ValueRange
  {
    major_dimension : Some( Dimension::Row ),
    range : Some( "tab2!A2:Z999".to_string() ),
    values : Some
    ( 
      vec!
      [ 
        vec![ json!( "Row2Col1" ), json!( "Row2Col2" ) ], 
        vec![ json!( "Row3Col1" ), json!( "Row3Col2" ) ] 
      ] 
    )
  };

  let server = MockServer::start();

  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/12345/values/tab2!A2:Z" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &body );
  } );

  let client = Client::former()
  .endpoint( server.url("" ) )
  .form();

  let rows = get_rows( &client, spreadsheet_id, "tab2" )
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