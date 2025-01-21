use gspread::*;
use actions::gspread::{update_row, update_rows_by_custom_row_key, OnFail, OnFind}; 
use gcore::client::
{
  BatchUpdateValuesResponse, Client, Dimension, ValueRange 
};
use httpmock::prelude::*;
use serde_json::json;




/// # What
/// We check that updating rows in a Google Spreadsheet returns the correct response.
///
/// # How
/// 1. Start a mock server and send `POST /12345/values:batchUpdate`.
/// 2. Return a `BatchUpdateValuesResponse`.
/// 3. Call `update_rows_by_custom_row_key()`, passing the necessary parameters.
/// 4. Verify that the number of updated cells, rows, and columns matches the expected result.
#[tokio::test]
async fn test_update_rows_by_custom_row_key_with_mock() {
  let server = MockServer::start();
  let spreadsheet_id = "12345";

  let get_mock = server.mock( |when, then| {
    when.method( GET )
      .path( "/12345/values/tab1!E:E" )
      .query_param( "majorDimension", "COLUMN" )
      .query_param( "valueRenderOption", "UNFORMATTED_VALUE" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body( json!( {
          "range": "tab1!E:E",
          "majorDimension": "COLUMN",
          "values": [ [ "12", "12", "12", "12" ] ]
      } ) );
  } );

  let mut mocked_value_ranges = vec![];
  for i in 1..=4 
  {
    mocked_value_ranges.push
    (
      ValueRange 
      {
        major_dimension : Some( Dimension::Row ),
        range: Some( format!( "tab1!A{}", i ) ),
        values: Some( vec![ vec![ json!( "Hello" ) ] ] ),
      }
    );
    mocked_value_ranges.push
    (
      ValueRange 
      {
        major_dimension: Some( Dimension::Row ),
        range: Some( format!( "tab1!B{}", i ) ),
        values: Some( vec![ vec![ json!( 123 ) ] ] ),
      }
    );
  }

  let response_body = BatchUpdateValuesResponse 
  {
    spreadsheet_id: Some( spreadsheet_id.to_string() ),
    total_updated_rows: Some( 4 ),
    total_updated_columns: Some( 2 ),
    total_updated_cells: Some( 8 ),
    total_updated_sheets: Some( 1 ),
    responses: Some( mocked_value_ranges ),
  };

  let post_mock = server.mock( | when, then | {
    when.method( POST )
      .path( "/12345/values:batchUpdate" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &response_body );
  } );

  let client = Client::former()
  .endpoint( server.url( "" ) )
  .form();

  let mut row_key_val = std::collections::HashMap::new();
  row_key_val.insert( "A".to_string(), json!( "Hello" ) );
  row_key_val.insert( "B".to_string(), json!( 123 ) );

  let batch_result = update_rows_by_custom_row_key
  (
    &client,
    spreadsheet_id,
    "tab1",
    ( "E", json!( 12 ) ),
    row_key_val,
    OnFind::UpdateAllMatchedRow,
    OnFail::AppendRow
  )
  .await
  .expect("update_rows_by_custom_row_key failed");

  assert_eq!( batch_result.spreadsheet_id.as_deref(), Some( spreadsheet_id ) );
  assert_eq!( batch_result.total_updated_cells, Some( 8 ) );
  assert_eq!( batch_result.total_updated_rows, Some( 4 ) );
  assert_eq!( batch_result.total_updated_columns, Some( 2 ) );
  if let Some( responses ) = &batch_result.responses 
  {
    assert_eq!( responses.len(), 8 );
  }

  get_mock.assert();
  post_mock.assert();
}