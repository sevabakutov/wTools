//!
//! Tests for `append_row` function.
//! 

use dotenv::dotenv;
use serde_json::json;
use std::collections::HashMap;
use gspread::
{
  actions::gspread::append_row, 
  gcore::
  { 
    client::Client, 
    ApplicationSecret 
  }
};

const SPREADSHEET_ID: &'static str  = "1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU";

/// # What 
/// We test appending a row at the and of a sheet.
/// 
/// # How
/// 1. Create a client.
/// 2. Call `append_row` function wich sends a POST request to /{spreadshett_id}/values/{range}:append
/// 3. Check results.
#[ tokio::test ]
async fn test_append_row_should_work()
{  
  dotenv().ok();
  let secret = ApplicationSecret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "failed to form a client." )
  .form();

  let mut row_key_val = HashMap::new();
  row_key_val.insert( "A".to_string(), json!( 1 ) );
  row_key_val.insert( "B".to_string(), json!( 2 ) );
  row_key_val.insert( "C".to_string(), json!( 3 ) );

  let _response = append_row( &client, SPREADSHEET_ID, "tab9", &row_key_val )
  .await
  .expect( "append_row failed." );
}


/// # What
/// We test that we can not pass a HashMap with invalid column index.
/// 
/// # How
/// 1. Create a client.
/// 3. Call `append_row` wich sends a POST request to /{spreadsheet_id}/values/{range}:append
#[ tokio::test ]
#[ should_panic ]
async fn test_append_row_with_bad_values_should_panic()
{  
  dotenv().ok();
  let secret = ApplicationSecret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "failed to form a client." )
  .form();

  let mut row_key_val = HashMap::new();
  row_key_val.insert( "AAAAA".to_string(), json!( 1 ) );
  row_key_val.insert( "BBBBA".to_string(), json!( 2 ) );
  row_key_val.insert( "CCCCA".to_string(), json!( 3 ) );

  let _response = append_row( &client, SPREADSHEET_ID, "tab2", &row_key_val )
  .await
  .expect( "append_row failed. Ok!" );
}

#[ tokio::test ]
#[ should_panic ]
async fn test_append_row_with_bad_values2_should_panic()
{  
  dotenv().ok();
  let secret = ApplicationSecret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "failed to form a client." )
  .form();

  let mut row_key_val = HashMap::new();
  row_key_val.insert( "123".to_string(), json!( 1 ) );
  row_key_val.insert( "a".to_string(), json!( 2 ) );
  row_key_val.insert( "qdqwq".to_string(), json!( 3 ) );

  let _response = append_row( &client, SPREADSHEET_ID, "tab9", &row_key_val )
  .await
  .expect( "append_row failed. Ok!" );
}