//!
//! Tests for `get_rows` function.
//! 

use dotenv::dotenv;
use gspread::gcore::ApplicationSecret;

use serde_json::json;
use gspread::actions::gspread::get_rows;
use gspread::gcore::client::Client;

const SPREADSHEET_ID: &'static str  = "1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU";

/// # What
/// We check that requesting all rows from the second row onward (below the header)
/// correctly parses the response and returns the expected result.
/// 
/// It works:
///  - With the whole rows.
///  - With rows with empty columns.
///  - With empty rows in the middle.
///  - With empty table.
///
/// # How
/// 1. Create a client.
/// 2. Call `get_rows` which sends a GET request to "/{spreadsheet_id}/values/{range}".
/// 3. Check results.
#[tokio::test]
async fn test_get_rows_should_work() 
{
  dotenv().ok();
  let secret = ApplicationSecret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "failed to form a client." )
  .form();

  let rows = get_rows( &client, SPREADSHEET_ID, "tab2" )
  .await
  .expect( "get_rows failed" );

  assert_eq!( rows.len(), 2 );
  assert_eq!( rows[0].len(), 3 );
  assert_eq!( rows[0][0], json!( "name1" ) );
  assert_eq!( rows[0][1], json!( "surname1" ) );
  assert_eq!( rows[0][2], json!( 20 ) );

  assert_eq!( rows[1].len(), 3);
  assert_eq!( rows[1][0], json!( "name2" ) );
  assert_eq!( rows[1][1], json!( "surname2" ) );
  assert_eq!( rows[1][2], json!( 85 ) );
}

#[ tokio::test ]
async fn test_mock_get_rows_with_empty_columns() 
{
  dotenv().ok();
  let secret = ApplicationSecret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "failed to form a client." )
  .form();

  let rows = get_rows( &client, SPREADSHEET_ID, "tab3" )
  .await
  .expect( "get_rows failed" );

  assert_eq!( rows.len(), 2 );
  assert_eq!( rows[0].len(), 3 );
  assert_eq!( rows[0][0], json!( "name1" ) );
  assert_eq!( rows[0][1], json!( "" ) );
  assert_eq!( rows[0][2], json!( 10 ) );

  assert_eq!( rows[1].len(), 3);
  assert_eq!( rows[1][0], json!( "name2" ) );
  assert_eq!( rows[1][1], json!( "" ) );
  assert_eq!( rows[1][2], json!( 11 ) );
}

#[ tokio::test ]
async fn test_get_rows_with_empty_row_in_the_middle() 
{
  dotenv().ok();
  let secret = ApplicationSecret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "failed to form a client." )
  .form();

  let rows = get_rows( &client, SPREADSHEET_ID, "tab4" )
  .await
  .expect( "get_rows failed" );

  assert_eq!( rows.len(), 3 );

  assert_eq!( rows[0].len(), 3 );
  assert_eq!( rows[0][0], json!( "name1" ) );
  assert_eq!( rows[0][1], json!( "surname1" ) );
  assert_eq!( rows[0][2], json!( 10 ) );

  assert_eq!( rows[1].len(), 0 );

  assert_eq!( rows[2].len(), 3);
  assert_eq!( rows[2][0], json!( "name3" ) );
  assert_eq!( rows[2][1], json!( "surname3" ) );
  assert_eq!( rows[2][2], json!( 12 ) );
}

#[ tokio::test ]
async fn test_mock_get_rows_empty_should_work() 
{
  dotenv().ok();
  let secret = ApplicationSecret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "failed to form a client." )
  .form();

  let rows = get_rows( &client, SPREADSHEET_ID, "tab1" )
  .await
  .expect( "get_rows failed" );

  assert_eq!( rows.len(), 0 );
}