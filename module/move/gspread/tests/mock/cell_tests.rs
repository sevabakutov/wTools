use httpmock::prelude::*;

use serde_json::json;
use gspread::actions::gspread::
{ 
  get_cell, 
  set_cell 
};
use gspread::gcore::client::
{
  Client, 
  Dimension, 
  ValueRange,
  UpdateValuesResponse 
};

/// # What
/// We check that reading a specific cell from a Google Spreadsheet returns the expected result.
///
/// # How
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Send a GET request to "/{spreadsheet_id}/values/{range}".
/// 4. Check for correct results.
#[tokio::test]
async fn test_get_cell_with_mock_should_work() 
{
  // 1. Ceating a server.
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

  // 2. Creating a client.
  let endpoint = server.url("");

  let client = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Sending a PUT request.
  let result = get_cell( &client, "12345", "tab2", "A2" )
  .await
  .expect( "get_cell failed" );

  // 4. Checking results.
  mock.assert();

  assert_eq!( result, serde_json::Value::String( "Steeve".to_string() ) );
}


/// # What
/// We check that setting a value in a specific cell of a Google Spreadsheet works correctly.
///
/// # How
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Send a PUT request to /{spreadsheet_id}/values/{range}?valueInputOption=RAW.
/// 4. Check results.
#[tokio::test]
async fn test_set_cell_with_mock_should_work() 
{
  // 1. Start a mock server.
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

  // 2. Create a client.
  let endpoint = server.url( "" );

  let client = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Send a PUT request.
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

  // 4. Check results.
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