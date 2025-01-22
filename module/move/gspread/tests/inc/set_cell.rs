//!
//! Tests for `set_cell` function.
//! 

use dotenv::dotenv;
use serde_json::json;
use gspread::{actions::gspread::set_cell, gcore::{client::Client, Secret}}; 

const SPREADSHEET_ID: &'static str  = "1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU";

/// # What
/// We check that setting a value in a specific cell of a Google Spreadsheet works correctly.
///
/// # How
/// 1. Create a client.
/// 2. Send a PUT request to /{spreadsheet_id}/values/{range}.
/// 3. Check results.
#[tokio::test]
async fn test_mock_set_cell_should_work() 
{
  dotenv().ok();
  let secret = Secret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "failed to form a client." )
  .form();

  let result = set_cell
  ( 
    &client, 
    SPREADSHEET_ID, 
    "tab9", 
    "A1", 
    json!( "Val" ) 
  )
  .await
  .expect( "set_cell failed with mock" );

  assert_eq!( result.spreadsheet_id.as_deref(), Some( SPREADSHEET_ID ) );
  assert_eq!( result.updated_rows, Some( 1 ) );
  assert_eq!( result.updated_columns, Some( 1 ) );
  assert_eq!( result.updated_cells, Some( 1 ) );
}

/// # What
/// We test that `set_cell` function will return error with bad cell_id.
///
/// # How
/// 1. Create a client.
/// 2. Send a PUT request to /{spreadsheet_id}/values/{range}.
#[ tokio::test ]
#[ should_panic ]
async fn test_mock_set_cell_bad_cell_id_should_panic() 
{
  dotenv().ok();
  let secret = Secret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "failed to form a client." )
  .form();

  let _result = set_cell( &client, SPREADSHEET_ID, "tab2", "AAAAA1", json!( "Val" ) )
  .await
  .expect( "set_cell failed with mock. Ok." );
}