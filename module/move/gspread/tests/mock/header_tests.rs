use httpmock::prelude::*;
use reqwest;

#[ tokio::test ]
async fn oauth2_first_endpoint_with_mock()
{
  let server = MockServer::start();
  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/o/oauth2/auth" )
      .query_param( "scope", "https://www.googleapis.com/auth/drive.readonly" )
      .query_param( "access_type", "offline" )
      .query_param( "redirect_uri", "http://localhost:44444" )
      .query_param( "response_type", "code" )
      .query_param( "client_id", "YOUR_CLIENT_ID" );
    then.status( 302 );
  });

  let response = reqwest::get
  ( 
    server.url
    ( 
    "/o/oauth2/auth?\
    scope=https://www.googleapis.com/auth/drive.readonly&\
    access_type=offline&\
    redirect_uri=http://localhost:44444&\
    response_type=code&\
    client_id=YOUR_CLIENT_ID"
    ) 
  )
  .await
  .unwrap();
  
  mock.assert();

  assert_eq!( response.status(), 302 );
}


#[ tokio::test ]
async fn oauth2_second_endpoint_with_mock()
{
  let server = MockServer::start();

  // url == first endpoint
  let mock = server.mock( | when, then | {
    when.path( "/o/oauth2/auth" )
      .query_param( "scope", "https://..." );
    then.status( 200 );
  } );

  // in real program at that point we have to open generated url and give access to our program from browser
  let response = reqwest::get( server.url( "/o/oauth2/auth?scope=https://..." ) ).await.unwrap();

  mock.assert();

  assert_eq!( response.status(), 200 );
}

// #[ tokio::test ]
// async fn get_header_with_mock_test()
// {
//   let server = MockServer::start();
//   let mock = server.mock( | when, then | {
//     when.method( GET )
//       .path( "/spreadsheets/d/1234" )
//       .query_param( "tab1!A1:Z1" )
//     then.status( 200 )
//       .header( "content-type", "application/json" )
//       .body( r#"["Name", "Surname", "Age"]"# );
//   });

//   let response = reqwest::get( server.url( "/spreadsheets/d/1234?tab1!A1:Z1" ) )
//   .await
//   .unwrap();

//   mock.assert();

//   assert_eq!( response.status(), 200 )
// }