//!
//! Client of API.
//!

mod private
{
  // use std::error::Error;
  use crate::*;
  use actions::gspread::{Error, Result};
  // use error_tools::for_app::Ok;
  use secret::Secret;
  use google_sheets4 as sheets4;
  use sheets4::
  {
    Sheets,
    hyper_rustls,
    hyper_util
  };
  use sheets4::yup_oauth2::
  {
    self,
    ApplicationSecret
  };
  use hyper_util::client::legacy::connect::HttpConnector;

  pub use hyper_util::client::legacy::Client;

  /// # SheetsType
  ///
  /// A type alias for `Sheets<hyper_rustls::HttpsConnector<HttpConnector>>` representing HTTP connector.
  pub type SheetsType = Sheets< hyper_rustls::HttpsConnector< HttpConnector > >;


  // TODO: Implement
  pub struct GspreadClient 
  {
    pub hub : Option<SheetsType>,
    endpoint: Option<String>

  }

  // TODO: Implement.
  impl GspreadClient 
  {
    // Should be a default implementation.
    // pub fn new() -> GspreadClient 
    // {

    // }

    pub fn builder() -> GspreadClientBuilder
    {
      GspreadClientBuilder::new()
    }

    // pub async fn update
    // (
    //   &self,
    //   spreadsheet_id : &str,
    //   sheet_name : &str,
    //   cell_id : &str,
    //   value : &str
    // ) -> Result< Error >
    // {

    // }

    // pub async fn get
    // (
    //   &self,
    //   spreadsheet_id : &str,
    //   sheet_name : &str
    // ) -> Result< Error >
    // {

    // }

    // pub async fn batch_update
    // (
    //   &self,
    //   spreadsheet_id : &str,
    //   sheet_name : &str,
    // ) -> Result< Error >
    // {

    // }
  }

  pub struct GspreadClientBuilder
  {
    pub secret : Option<Secret>,
    pub endpoint : Option<String>
  }

  impl GspreadClientBuilder
  {
    pub fn new() -> Self
    {
      Self
      {
        secret : None,
        endpoint : None
      }
    }

    pub fn with_endpoint< S: Into< String > >
    (
      mut self,
      endpoint : S
    ) -> Self
    {
      self.endpoint = Some( endpoint.into() );
      self
    }

    pub fn with_secret
    (
      mut self,
      secret : Secret
    ) -> Self
    {
      self.secret = Some( secret );
      self
    }

    pub async fn build(self) -> Result< GspreadClient >
    {
      if self.endpoint.is_some() && self.secret.is_some()
      {
        return Err( Error::HubError("Build error. You can not have enpoint and secret both at the same time. Endpoint is used for mock testing, and secret for normal usage.".to_string()) );
      }

      if self.endpoint.is_some()
      {
        return Ok
        (
          GspreadClient 
          {
            hub : None,
            endpoint : self.endpoint
          }
        );
      }

      if self.secret.is_some()
      {
        let hub = hub( self.secret.unwrap() ).await?;
        return Ok
        (
          GspreadClient
          {
            hub : Some(hub),
            endpoint : None
          }  
        )
      }

      Err( Error::HubError( "Build error. Endpoint or secret were not set up. You can use with_secret method for normal usage or with_endpoint method for mock testing.".to_string() ) )
    }
  }

  /// # `hub`
  ///
  /// Initializes and configures a Google Sheets client (`SheetsType`) using the provided secrets.
  ///
  /// ## Parameters:
  /// - `secrets`:  
  ///   A reference to a `Secret` struct containing credentials and URIs required for authentication.
  ///
  /// ## Returns:
  /// - `Result<SheetsType, Box<dyn Error>>`:  
  ///
  /// ## Example:
  /// ```rust
  /// async fn example() -> Result<(), Box<dyn Error>> 
  /// {
  ///   let secrets = Secret::read();
  ///   let hub = hub(&secrets).await?;
  ///
  ///   // Use `hub` to interact with the Google Sheets API.
  ///   Ok(())
  /// }
  /// ```
  pub async fn hub( secrets: Secret ) -> Result< SheetsType >
  {
    let secret: ApplicationSecret = ApplicationSecret
    {
      client_id : secrets.CLIENT_ID.clone(),
      auth_uri : secrets.AUTH_URI.clone(),
      token_uri : secrets.TOKEN_URI.clone(),
      client_secret : secrets.CLIENT_SECRET.clone(),
      .. Default::default()
    };

    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
      secret,
      yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .build()
    .await
    .map_err( | _ | Error::HubError( "Auth error.".to_string() ) )?;

    let client = Client::builder
    (
      hyper_util::rt::TokioExecutor::new()
    )
    .build
    (
      hyper_rustls::HttpsConnectorBuilder::new()
      .with_native_roots()
      .unwrap()
      .https_or_http()
      .enable_http1()
      .build()
    );

    Ok( Sheets::new( client, auth ) )
  }


}

crate::mod_interface!
{
  exposed use
  {
    hub,
    Client,
    SheetsType
  };
}