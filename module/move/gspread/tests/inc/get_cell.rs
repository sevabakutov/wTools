//!
//! Tests for `get_cell` function.
//! 

use dotenv::dotenv;
use serde_json::json;
use gspread::
{
  actions::gspread::get_cell, 
  gcore::{client::Client, Secret}
};

const SPREADSHEET_ID: &'static str  = "1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU";

/// # What
/// We check that reading a specific cell from a Google Spreadsheet returns the expected result.
///
/// # How
/// 1. Create a client.
/// 2. Send a GET request to "/{spreadsheet_id}/values/{range}".
/// 3. Check for correct results.
#[tokio::test]
async fn test_get_cell_should_work() 
{
  dotenv().ok();
  let secret = Secret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "failed to form a client." )
  .form();

  let result = get_cell( &client, SPREADSHEET_ID, "tab2", "A2" )
  .await
  .expect( "get_cell failed" );

  assert_eq!( result, json!( "name1" ) );
}

#[tokio::test]
async fn test_get_empty_cell_should_work() 
{
  dotenv().ok();
  let secret = Secret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "failed to form a client." )
  .form();

  let result = get_cell( &client, SPREADSHEET_ID, "tab2", "A10" )
  .await
  .expect( "get_cell failed" );

  assert_eq!( result, json!( "" ) );
}

/// # What
/// We test that function has to return an error if invalid cellid was provideed.
/// 
/// # How
/// 1. Create a client
/// 2. Call `get_cell` and pass there a bad cell id. 
#[ tokio::test ]
#[ should_panic ]
async fn test_get_cell_with_bad_range_should_panic() 
{
  dotenv().ok();
  let secret = Secret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "failed to form a client." )
  .form();

  let _result = get_cell( &client, SPREADSHEET_ID, "tab2", "AAAA2" )
  .await
  .expect( "get_cell failed" );
}