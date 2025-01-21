use gspread::{actions::gspread::append_row, gcore::client::{Client, Dimension, UpdateValuesResponse, ValueRange, ValuesAppendResponse}};
use httpmock::prelude::*;
use serde_json::json;

use std::collections::HashMap;

/// # What 
/// We test appending a row at the and of a sheet.
/// 
/// # How
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Call `append_row` function wich sends a POST request to /{spreadshett_id}/values/{range}:append
/// 4. Check results.
#[ tokio::test ]
async fn test_mock_append_row_should_work()
{  
  let spreadsheet_id = "12345";
  let value_range = ValueRange
  {
    major_dimension : Some( Dimension::Row ),
    range : None,
    values : Some( vec![ vec![ json!( 1 ), json!( 2 ), json!( 3 ) ] ] )
  };
  let updates = UpdateValuesResponse
  {
    spreadsheet_id : Some( spreadsheet_id.to_string() ),
    updated_range : Some( "tab2!A3:C3".to_string() ),
    updated_rows : Some( 1 ),
    updated_columns : Some( 3 ),
    updated_cells : Some( 3 ),
    updated_data : Some( value_range.clone() )
  };
  let body = ValuesAppendResponse
  {
    spreadsheet_id : Some( spreadsheet_id.to_string() ),
    table_range : None,
    updates : Some( updates )
  };

  // 1. Start a mock server.
  let server = MockServer::start();
  let mock = server.mock( | when, then | {
    when.method( POST )
      .path( "/12345/values/tab2!A:C:append" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &body );
  } );

  // 2. Create a client.
  let endpoint = server.url("" );
  let client = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Call `append_row`.
  let mut row_key_val = HashMap::new();
  row_key_val.insert( "A".to_string(), json!( 1 ) );
  row_key_val.insert( "B".to_string(), json!( 2 ) );
  row_key_val.insert( "C".to_string(), json!( 3 ) );

  let response = append_row( &client, spreadsheet_id, "tab2", &row_key_val )
  .await
  .expect( "append_row failed." );

  // 4. Check results.
  mock.assert();

  assert_eq!( response.spreadsheet_id, Some( spreadsheet_id.to_string() ) );
  assert_eq!( response.table_range, None );
  assert_eq!
  ( 
    response.updates.unwrap().updated_data.unwrap().values, 
    Some( value_range.values.unwrap() ) 
  );
}

#[ tokio::test ]
async fn test_mock_append_row_to_existed_one_should_work()
{  
  let spreadsheet_id = "12345";
  let value_range = ValueRange
  {
    major_dimension : Some( Dimension::Row ),
    range : None,
    values : Some( vec![ vec![ json!( 1 ), json!( 2 ), json!( 3 ) ] ] )
  };
  let updates = UpdateValuesResponse
  {
    spreadsheet_id : Some( spreadsheet_id.to_string() ),
    updated_range : Some( "tab2!A3:C3".to_string() ),
    updated_rows : Some( 1 ),
    updated_columns : Some( 3 ),
    updated_cells : Some( 3 ),
    updated_data : Some( value_range.clone() )
  };
  let body = ValuesAppendResponse
  {
    spreadsheet_id : Some( spreadsheet_id.to_string() ),
    table_range : None,
    updates : Some( updates )
  };

  // 1. Start a mock server.
  let server = MockServer::start();
  let mock = server.mock( | when, then | {
    when.method( POST )
      .path( "/12345/values/tab2!A:C:append" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &body );
  } );

  // 2. Create a client.
  let endpoint = server.url("" );
  let client = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Call `append_row`.
  let mut row_key_val = HashMap::new();
  row_key_val.insert( "A".to_string(), json!( 1 ) );
  row_key_val.insert( "B".to_string(), json!( 2 ) );
  row_key_val.insert( "C".to_string(), json!( 3 ) );

  let response = append_row( &client, spreadsheet_id, "tab2", &row_key_val )
  .await
  .expect( "append_row failed." );

  // 4. Check results.
  mock.assert();

  assert_eq!( response.spreadsheet_id, Some( spreadsheet_id.to_string() ) );
  assert_eq!( response.table_range, None );
  assert_eq!
  ( 
    response.updates.unwrap().updated_data.unwrap().values, 
    Some( value_range.values.unwrap() ) 
  );
}