mod private
{
  use crate::*;
  use actions::gspread::{ Error, Result };
  use client::{ hub, SheetsType };
  use client::mock::MockClient;

  pub enum GspreadClient 
  {
    GoogleHub(SheetsType),
    MockHub(MockClient),
  }

  impl GspreadClient 
  {
    pub fn builder<'a>() -> GspreadClientBuilder<'a>
    {
      GspreadClientBuilder::new()
    }
  }

  pub struct GspreadClientBuilder<'a>
  {
    pub secret : Option<&'a Secret>,
    pub endpoint : Option<String>
  }

  impl<'a> GspreadClientBuilder<'a>
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
      secret : &'a Secret
    ) -> Self
    {
      self.secret = Some( &secret );
      self
    }

    pub async fn build(self) -> Result< GspreadClient >
    {
      if self.secret.is_none()
      {
        return Err( Error::HubError( "Failed to build GspreadClient. You have not set up a secret.".to_string() ) )
      }

      if self.endpoint.is_some()
      {
        return Ok( GspreadClient::MockHub( MockClient::new( self.endpoint.unwrap() ) ) );
      }

      let hub = hub( self.secret.unwrap() ).await?;
      return Ok( GspreadClient::GoogleHub( hub ) );
    }
  }
}

crate::mod_interface!
{
  exposed use
  {
    GspreadClient,
    GspreadClientBuilder
  };
}