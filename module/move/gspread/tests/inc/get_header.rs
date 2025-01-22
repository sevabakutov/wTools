//!
//! Tests for `get_header()` function.
//! It can return only one of the common errors.
//! 

use dotenv::dotenv;
use gspread::gcore::Secret;
use gspread::actions::gspread::get_header;
use gspread::gcore::client::Client;

const SPREADSHEET_ID: &'static str  = "1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU";

/// # What
/// We check that requesting the header row (first row) of a sheet in a Google Spreadsheet
/// returns the correct set of column values.
/// 
/// It works:
///  - With the whole header, 
///  - With empty columns between columns,
///  - With empty column at the start.
///  - With empty header.
///
/// # How
/// 1. Create a client.
/// 2. Call `get_header()` function wich sends a GET request to /{spreadshett_id}/values/{range}.
/// 3. Check results.
#[tokio::test]
async fn test_get_header_should_work() 
{
  dotenv().ok();
  let secret = Secret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "failed to form a client." )
  .form();

  let header = get_header( &client, SPREADSHEET_ID, "tab2" )
  .await
  .expect( "get_header failed" );

  assert_eq!( header.len(), 3, "Header row should have 3 columns" );
  assert_eq!( header[0], serde_json::Value::String( "Name".to_string() ) );
  assert_eq!( header[1], serde_json::Value::String( "Surname".to_string() ) );
  assert_eq!( header[2], serde_json::Value::String( "Age".to_string() ) );
}

#[tokio::test]
async fn test_get_header_with_empty_column_betwee_columns_should_work() 
{
  dotenv().ok();
  let secret = Secret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "failed to form a client." )
  .form();

  let header = get_header( &client, SPREADSHEET_ID, "tab3" )
  .await
  .expect( "get_header failed" );

  assert_eq!( header.len(), 3, "Header row should have 3 columns" );
  assert_eq!( header[0], serde_json::Value::String( "Name".to_string() ) );
  assert_eq!( header[1], serde_json::Value::String( "".to_string() ) );
  assert_eq!( header[2], serde_json::Value::String( "Surname".to_string() ) );
}

#[tokio::test]
async fn test_get_header_with_empty_first_column_should_work() 
{
  dotenv().ok();
  let secret = Secret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "failed to form a client." )
  .form();

  let header = get_header( &client, SPREADSHEET_ID, "tab4" )
  .await
  .expect( "get_header failed" );

  assert_eq!( header.len(), 3, "Header row should have 3 columns" );
  assert_eq!( header[0], serde_json::Value::String( "".to_string() ) );
  assert_eq!( header[1], serde_json::Value::String( "header2".to_string() ) );
  assert_eq!( header[2], serde_json::Value::String( "header3".to_string() ) );
}

#[tokio::test]
async fn test_get_header_with_empty_columns_should_work() 
{
  dotenv().ok();
  let secret = Secret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "failed to form a client." )
  .form();

  let header = get_header( &client, SPREADSHEET_ID, "tab6" )
  .await
  .expect( "get_header failed" );

  assert_eq!( header.len(), 0, "Header row should have 0 columns" );
}