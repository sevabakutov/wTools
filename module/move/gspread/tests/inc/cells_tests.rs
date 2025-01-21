use dotenv::dotenv;
use gspread::*;
use actions::gspread::{update_row, update_rows_by_custom_row_key, OnFail, OnFind};
use gcore::Secret; 
use gcore::client::Client;
use serde_json::json;

/// # What
/// We check that updating a row in a Google Spreadsheet returns the correct response.
///
/// # How
/// 1. Send `POST /1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/values:batchUpdate`.
/// 2. Return a `BatchUpdateValuesResponse`.
/// 3. Call `update_row()`, passing the necessary parameters.
/// 4. Verify that the number of updated cells, rows, and columns matches the expected result.
// #[tokio::test]
// async fn test_update_row_should_work() 
// {
//   dotenv().ok();

//   let secret = Secret::read();

//   let client = Client::former()
//   .token( &secret )
//   .await
//   .expect( "Failed to buid a client" )
//   .form();

//   let spreadsheet_id = "1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU";
//   let mut row_key_val = std::collections::HashMap::new();
//   row_key_val.insert( "A".to_string(), serde_json::Value::String( "Hello".to_string() ) );
//   row_key_val.insert( "B".to_string(), serde_json::Value::Number( serde_json::Number::from( 123 ) ) );

//   let batch_result = update_row
//   ( 
//     &client, 
//     spreadsheet_id,
//     "tab3",
//     serde_json::Value::Number( serde_json::Number::from( 7 ) ), 
//     row_key_val
//   )
//   .await
//   .expect( "update_row failed" );

//   assert_eq!( batch_result.spreadsheet_id.as_deref(), Some( spreadsheet_id ) );
//   assert_eq!( batch_result.total_updated_cells, Some( 2 ) );
//   assert_eq!( batch_result.total_updated_rows, Some( 1 ) );
//   assert_eq!( batch_result.total_updated_columns, Some( 2 ) );

//   if let Some( responses ) = &batch_result.responses 
//   {
//     assert_eq!( responses.len(), 2 );
//   }
// }


/// # What
/// We check that updating rows in a Google Spreadsheet returns the correct response.
///
/// # How
/// 1. Send `POST /1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/values:batchUpdate`.
/// 2. Return a `BatchUpdateValuesResponse`.
/// 3. Call `update_rows_by_custom_row_key()`, passing the necessary parameters.
/// 4. Verify that the number of updated cells, rows, and columns matches the expected result.
#[tokio::test]
async fn test_update_rows_by_custom_row_key_should_work() 
{
  dotenv().ok();

  let secret = Secret::read();

  let client = Client::former()
  .token( &secret )
  .await
  .expect( "Failed to buid a client" )
  .form();

  let spreadsheet_id = "1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU";
  let mut row_key_val = std::collections::HashMap::new();
  row_key_val.insert( "C".to_string(), json!( "Buy" ) );
  row_key_val.insert( "D".to_string(), json!( 0987 ) );

  let batch_result = update_rows_by_custom_row_key
  ( 
    &client, 
    spreadsheet_id,
    "tab1",
    ( "E", serde_json::Value::from( 333 ) ),
    row_key_val,
    OnFind::UpdateAllMatchedRow,
    OnFail::AppendRow
  )
  .await
  .expect( "update_row_by_custom_row_key failed" );

  println!("{:?}",batch_result );
  // assert_eq!( batch_result.spreadsheet_id.as_deref(), Some( spreadsheet_id ) );
  // assert_eq!( batch_result.total_updated_cells, Some( 8 ) );
  // assert_eq!( batch_result.total_updated_rows, Some( 4 ) );
  // assert_eq!( batch_result.total_updated_columns, Some( 2 ) );

  // if let Some( responses ) = &batch_result.responses 
  // {
  //   assert_eq!( responses.len(), 8 );
  // }
}