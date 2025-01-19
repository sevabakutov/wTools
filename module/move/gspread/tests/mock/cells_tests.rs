use gspread::*;
use actions::gspread::update_row; 
use gcore::client::
{
  Client,
  Dimension, 
  ValueRange,
  BatchUpdateValuesResponse 
};
use httpmock::prelude::*;
use serde_json::json;

/// # What
/// We check that updating a row in a Google Spreadsheet returns the correct response.
///
/// # How
/// 1. Start a `MockServer` to send `POST /12345/values:batchUpdate`.
/// 2. Return a predefined `BatchUpdateValuesResponse`.
/// 3. Call `update_row()`, passing the necessary parameters.
/// 4. Verify that the number of updated cells, rows, and columns matches the expected result.
#[tokio::test]
async fn test_update_row_with_mock() {
  let spreadsheet_id = "12345";

  let value_ranges = vec!
  [
    ValueRange
    {
      major_dimension : Some( Dimension::Row ),
      range : Some( "tab2!A5".to_string() ),
      values : Some( vec![ vec![ json!( "Hello" ) ] ] )
    },

    ValueRange
    {
      major_dimension : Some( Dimension::Row ),
      range : Some( "tab2!A7".to_string() ),
      values : Some( vec![ vec![ json!( 123 ) ] ] )
    },
  ];

  let response_body = BatchUpdateValuesResponse
  {
    spreadsheet_id : Some( spreadsheet_id.to_string() ),
    total_updated_rows : Some( 2 ),
    total_updated_columns : Some( 1 ),
    total_updated_cells : Some( 2 ),
    total_updated_sheets : Some( 1 ),
    responses : Some( value_ranges )
  };

  let server = MockServer::start();

  let mock = server.mock( |when, then | {
    when.method( POST )
      .path( "/12345/values:batchUpdate" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &response_body );
  });

  let client = Client::former()
  .endpoint( server.url("") )
  .form();

  let mut row_key_val = std::collections::HashMap::new();
  row_key_val.insert( "A".to_string(), serde_json::Value::String( "Hello".to_string() ) );
  row_key_val.insert( "B".to_string(), serde_json::Value::Number( serde_json::Number::from( 123 ) ) );

  let batch_result = update_row
  ( 
    &client, 
    spreadsheet_id, 
    "tab2", 
    serde_json::Value::String( "5".to_string() ), 
    row_key_val
  )
  .await
  .expect( "update_row failed in mock test" );

  mock.assert();

  assert_eq!( batch_result.spreadsheet_id.as_deref(), Some( spreadsheet_id ) );
  assert_eq!( batch_result.total_updated_cells, Some( 2 ) );
  assert_eq!( batch_result.total_updated_rows, Some( 2 ) );
  assert_eq!( batch_result.total_updated_columns, Some( 1 ) );

  if let Some( responses ) = &batch_result.responses 
  {
    assert_eq!( responses.len(), 2 );
  }
}