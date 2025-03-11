use httpmock::prelude::*;
use gspread::
{
	actions::gspread::
	{
		delete_rows, 
		RowRange
	}, 
	gcore::
	{
		methods::batch_update::BatchUpdateResponse,
		ApplicationSecret, 
		Client
	}
};



// This test is going to fail because 
#[ tokio::test ]
async fn test_delete_rows_all_should_work()
{
	let spreadsheet_id = "12345";
	let sheet_id = "12345";
	let range = RowRange::All;

	let response = BatchUpdateResponse
	{
		spreadsheet_id : Some( spreadsheet_id.to_string() ),
		replies : None,
		updated_spreadsheet : None
	};

	// 1. Start server.
	let server = MockServer::start();
	let _ = server.mock( | when, then | {
		when.method( POST )
			.path( "/12345:batchUpdate" );
		then
			.status( 200 )
			.header( "Content-Type", "application/json" )
			.json_body_obj( &response );
	} );

	let endpoint = server.url( "" );
	let client : Client< '_, ApplicationSecret > = Client::former()
	.endpoint( &*endpoint )
	.form();

	_ = delete_rows( &client, spreadsheet_id, sheet_id, range )
	.await
	.expect( "Failed delete rows test" );
}