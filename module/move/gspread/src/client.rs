//!
//! Client of API.
//!

mod private
{
  use std::error::Error;
  use crate::*;
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
  pub async fn hub( secrets: &Secret ) -> Result< SheetsType, Box< dyn Error > >
  {
    let secret: ApplicationSecret = ApplicationSecret
    {
      client_id : secrets.CLIENT_ID.clone(),
      auth_uri : secrets.AUTH_URI.clone(),
      token_uri : secrets.TOKEN_URI.clone(),
      client_secret : secrets.CLIENT_SECRET.clone(),
      .. Default::default()
    };

    let auth = yup_oauth2::InstalledFlowAuthenticator::builder
    (
      secret,
      yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .build()
    .await
    .unwrap();

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