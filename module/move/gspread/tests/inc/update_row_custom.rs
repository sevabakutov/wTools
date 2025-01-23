//!
//! Tests to update
//! 

use dotenv::dotenv;
use gspread::
{
  actions::gspread::
  {
    update_rows_by_custom_row_key, 
    OnFail, 
    OnFind
  }, gcore::{client::Client, ApplicationSecret}
};
use serde_json::json;

const SPREADSHEET_ID: &'static str  = "1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU";

/// # What
/// We check that updating rows in a Google Spreadsheet returns the correct response.
///
/// # How
/// 1. Create a client
/// 2. Call `update_rows_by_custom_row_key`.
/// 3. Check results.
#[tokio::test]
async fn test_update_rows_by_custom_row_key_on_fail_nothing_should_work() 
{
  dotenv().ok();
  let secret = ApplicationSecret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "failed to form a client." )
  .form();

  let mut row_key_val = std::collections::HashMap::new();
  row_key_val.insert( "A".to_string(), json!( "Hello" ) );
  row_key_val.insert( "B".to_string(), json!( 123 ) );

  let _batch_result = update_rows_by_custom_row_key
  (
    &client,
    SPREADSHEET_ID,
    "tab10",
    ( "E", json!( 122 ) ),
    row_key_val,
    OnFind::UpdateFirstMatchedRow,
    OnFail::Nothing
  )
  .await
  .expect( "update_rows_by_custom_row_key failed" );
}



/// # What
/// We check that updating rows in a Google Spreadsheet returns the correct response.
///
/// # How
/// 1. Create a client.
/// 2. Call `update_rows_by_custom_row_key`.
#[ tokio::test ]
#[ should_panic ]
async fn test_update_rows_by_custom_row_key_on_fail_error_should_panic() 
{
  dotenv().ok();
  let secret = ApplicationSecret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "failed to form a client." )
  .form();

  let mut row_key_val = std::collections::HashMap::new();
  row_key_val.insert( "A".to_string(), json!( "Hello" ) );
  row_key_val.insert( "B".to_string(), json!( 123 ) );

  let _batch_result = update_rows_by_custom_row_key
  (
    &client,
    SPREADSHEET_ID,
    "tab10",
    ( "E", json!( 122 ) ),
    row_key_val,
    OnFind::UpdateFirstMatchedRow,
    OnFail::Error
  )
  .await
  .expect( "update_rows_by_custom_row_key failed" );
}

/// # What
/// We test that in case where we didn't find passed cell, OnFail::AppendRow in works correct.
///
/// # How
/// 1. Create a client
/// 2. Call `update_rows_by_custom_row_key`.
/// 3. Check results.
#[tokio::test]
async fn test_update_rows_by_custom_row_key_on_find_append_row_should_work() 
{
  dotenv().ok();
  let secret = ApplicationSecret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "failed to form a client." )
  .form();

  let mut row_key_val = std::collections::HashMap::new();
  row_key_val.insert( "A".to_string(), json!( "Hello" ) );
  row_key_val.insert( "B".to_string(), json!( 123 ) );

  let mut row_key_val = std::collections::HashMap::new();
  row_key_val.insert( "A".to_string(), json!( "Hello" ) );
  row_key_val.insert( "B".to_string(), json!( 123 ) );

  let _batch_result = update_rows_by_custom_row_key
  (
    &client,
    SPREADSHEET_ID,
    "tab10",
    ( "E", json!( 122 ) ),
    row_key_val,
    OnFind::UpdateFirstMatchedRow,
    OnFail::AppendRow
  )
  .await
  .expect( "update_rows_by_custom_row_key failed" );
}

/// # What
/// We test that in case where we didn't find passed cell, OnFail::AppendRow in works correct.
///
/// # How
/// 1. Create a client
/// 2. Call `update_rows_by_custom_row_key`.
/// 3. Check resaults.
#[tokio::test]
async fn test_update_rows_by_custom_row_key_on_find_update_first_row_should_work() 
{
  dotenv().ok();
  let secret = ApplicationSecret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "failed to form a client." )
  .form();

  let mut row_key_val = std::collections::HashMap::new();
  row_key_val.insert( "A".to_string(), json!( "Hello" ) );
  row_key_val.insert( "B".to_string(), json!( 123 ) );

  let _batch_result = update_rows_by_custom_row_key
  (
    &client,
    SPREADSHEET_ID,
    "tab10",
    ( "E", json!( 12 ) ),
    row_key_val,
    OnFind::UpdateFirstMatchedRow,
    OnFail::Error
  )
  .await
  .expect( "update_rows_by_custom_row_key failed" );
}

/// # What
/// We test that in case where we didn't find passed cell, OnFail::UpdateAllMatchesRows in works correct.
///
/// # How
/// 1. Create a client
/// 2. Call `update_rows_by_custom_row_key`.
/// 3. Check resaults.
#[tokio::test]
async fn test_update_rows_by_custom_row_key_on_find_update_all_rows_should_work() 
{
  dotenv().ok();
  let secret = ApplicationSecret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "failed to form a client." )
  .form();

  let mut row_key_val = std::collections::HashMap::new();
  row_key_val.insert( "A".to_string(), json!( "Hello" ) );
  row_key_val.insert( "B".to_string(), json!( 123 ) );

  let mut row_key_val = std::collections::HashMap::new();
  row_key_val.insert( "A".to_string(), json!( "Hello" ) );
  row_key_val.insert( "B".to_string(), json!( 123 ) );

  let _batch_result = update_rows_by_custom_row_key
  (
    &client,
    SPREADSHEET_ID,
    "tab10",
    ( "E", json!( 12 ) ),
    row_key_val,
    OnFind::UpdateAllMatchedRow,
    OnFail::Error
  )
  .await
  .expect( "update_rows_by_custom_row_key failed" );
}

/// # What
/// We test that in case where we find passed cell, OnFail::UpdateLastMatchedRow in works correct.
///
/// # How
/// 1. Create a client
/// 2. Call `update_rows_by_custom_row_key`.
/// 3. Check resaults.
#[tokio::test]
async fn test_mock_update_rows_by_custom_row_key_on_find_update_last_row_should_work() 
{
  dotenv().ok();
  let secret = ApplicationSecret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "failed to form a client." )
  .form();

  let mut row_key_val = std::collections::HashMap::new();
  row_key_val.insert( "A".to_string(), json!( "Hello" ) );
  row_key_val.insert( "B".to_string(), json!( 123 ) );

  let _batch_result = update_rows_by_custom_row_key
  (
    &client,
    SPREADSHEET_ID,
    "tab10",
    ( "E", json!( 12 ) ),
    row_key_val,
    OnFind::UpdateLastMatchedRow,
    OnFail::Error
  )
  .await
  .expect( "update_rows_by_custom_row_key failed" );
}