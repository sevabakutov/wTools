use gspread::actions::gspread::set_cell;
use gspread::gcore::client::{Client, Dimension, UpdateValuesResponse, ValueRange};
use gspread::actions::gspread::get_cell;
use httpmock::prelude::*;
use gspread::ser::JsonValue;
use serde_json::json;

/// # What
/// We check that reading a specific cell from a Google Spreadsheet returns the expected result.
///
/// # How
/// 1. Start a `MockServer` to send `GET /12345/values/tab2!A2`.
/// 2. Return a predefined `ValueRange` containing the desired cell value.
/// 3. Call `get_cell()`, passing the necessary parameters.
/// 4. Verify that the result matches the expected `"Steeve"`.
#[tokio::test]
async fn test_get_cell_with_mock_should_work() {
  let spreadsheet_id = "12345";
  let body = ValueRange
  {
    major_dimension : Some( Dimension::Row ),
    range : Some( "tab2!A2".to_string() ),
    values : Some( vec![ vec![ json!( "Steeve" ) ] ] )
  };

  let server = MockServer::start();

  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/12345/values/tab2!A2" );
    then
      .status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &body );
  } );

  let client = Client::former()
  .endpoint( server.url("") )
  .form();

  let result = get_cell( &client, spreadsheet_id, "tab2", "A2" )
  .await
  .expect( "get_cell failed" );

  mock.assert();

  assert_eq!( result, JsonValue::String( "Steeve".to_string() ) );
}


/// # What
/// We check that setting a value in a specific cell of a Google Spreadsheet works correctly.
///
/// # How
/// 1. Start a `MockServer` to send `PUT /12345/values/tab2!A1?valueInputOption=RAW`.
/// 2. Return a predefined `UpdateValuesResponse`.
/// 3. Call `set_cell()`, passing the table, sheet, cell, and the value to set.
/// 4. Verify that the number of updated cells, rows, and columns matches the expected result.
#[tokio::test]
async fn test_set_cell_with_mock_should_work() {
  let spreadsheet_id = "12345";
  let range = "tab2!A1";
  let value_range = ValueRange
  {
    major_dimension : Some( Dimension::Row ),
    range : Some( range.to_string() ),
    values : Some( vec![ vec![ json!( "Val" ) ] ] )
  };

  let response_body = UpdateValuesResponse
  {
    spreadsheet_id : Some( spreadsheet_id.to_string() ),
    updated_cells : Some( 1 ),
    updated_columns : Some( 1 ),
    updated_range : Some( range.to_string() ),
    updated_rows : Some( 1 ),
    updated_data : Some( value_range )
  };

  let server = MockServer::start();

  let mock = server.mock( | when, then | {
    when.method( PUT )
      .path( "/12345/values/tab2!A1" )
      .query_param( "valueInputOption", "RAW" );
    then
      .status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &response_body );
  });

  let client = Client::former()
  .endpoint( server.url( "" ) )
  .form();

  let result = set_cell
  ( 
    &client, 
    spreadsheet_id, 
    "tab2", 
    "A1", 
    serde_json::Value::String( "Val".to_string() ) 
  )
  .await
  .expect( "set_cell failed with mock" );

  mock.assert();

  assert_eq!( result.spreadsheet_id.as_deref(), Some( spreadsheet_id ) );
  assert_eq!( result.updated_range.as_deref(), Some( range ) );
  assert_eq!( result.updated_rows, Some( 1 ) );
  assert_eq!( result.updated_columns, Some( 1 ) );
  assert_eq!( result.updated_cells, Some( 1 ) );

  if let Some( updated_data ) = &result.updated_data 
  {
    let values = updated_data.values.as_ref().unwrap();
    assert_eq!( values, &vec![ vec![ json!( "Val" ) ] ] );
  }
}