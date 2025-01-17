
mod private
{
  use crate::*;
  use reqwest::StatusCode;
  use actions::gspread::{Result, Error};
  use google_sheets4::api::{ValueRange, UpdateValuesResponse};

  #[ derive( Clone ) ]
  pub struct MockClient 
  {
    pub endpoint : String,
  }

  impl MockClient {
    pub fn new( endpoint : String ) -> Self 
    {
      Self
      {
        endpoint : endpoint,
      }
    }

    pub fn spreadsheet( &self ) -> MockSpreadSheetMethod<'_>
    {
      MockSpreadSheetMethod
      {
        mock_client : self
      }
    }
  }

  pub struct MockSpreadSheetMethod<'a>
  {
    pub mock_client : &'a MockClient
  }

  impl<'a> MockSpreadSheetMethod<'a>
  {
    pub fn values_update
    ( 
      &self,
      request : ValueRange,
      spreadsheet_id : &'a str,
      range : &'a str
    ) -> MockValueUpdateCall
    {
      MockValueUpdateCall
      {
        endpoint : self.mock_client.endpoint.clone(),
        _spreadsheet_id : spreadsheet_id,
        _request : request,
        _range : range
      }
    }

    pub fn values_get
    (
      &self,
      spreadsheet_id : &'a str,
      range : &'a str
    ) -> MockValueGetCall
    {
      let full_path = format!
      (
        "{}/v4/spreadsheets/{}/values/{}",
        self.mock_client.endpoint,
        spreadsheet_id,
        range
      );

      MockValueGetCall 
      { 
        endpoint : full_path, 
        _spreadsheet_id : spreadsheet_id, 
        _range : range 
      }
    }

  }

  pub struct MockValueUpdateCall<'a>
  {
    pub endpoint : String,
    pub _spreadsheet_id : &'a str,
    pub _request : ValueRange,
    pub _range : &'a str
  }

  impl MockValueUpdateCall<'_>
  {
    pub async fn doit( &self ) -> Result< UpdateValuesResponse >
    {
      let body = serde_json::to_string(&self._request)
      .map_err(|err| Error::MockApiError(format!("Failed to serialize request: {}", err)))?;

      let response = reqwest::Client::new()
      .post( &self.endpoint )
      .header( "Content-Type", "application/json" )
      .body( body )
      .send()
      .await
      .map_err( | err | Error::MockApiError( format!( "Failed to send request: {:?}", err ) ) )?;

      let response_status = response.status();
      
      if response_status == StatusCode::OK
      {
        let update_values_response = UpdateValuesResponse
        {
          spreadsheet_id : Some( self._spreadsheet_id.to_string() ),
          updated_cells : Some( 1 ),
          updated_columns : Some( 1 ),
          updated_data : Some( self._request.clone() ),
          updated_range : Some( self._range.to_string() ),
          updated_rows : Some( 1 )
        };

        return Ok( update_values_response );
      }
      
      match response.text().await
      {
        Ok( text ) => Err( Error::MockApiError( format!( "Status code: {:?}, {}", response_status, text ) ) ),
        Err( error ) => Err( Error::MockApiError( format!("Await error: {:?}", error) ) )
      }
      
    }

  }

  pub struct MockValueGetCall<'a>
  {
    endpoint : String,
    _spreadsheet_id : &'a str,
    _range : &'a str
  }

  impl MockValueGetCall<'_>
  {
    pub async fn doit( &self ) -> Result< ValueRange >
    {
      let response = reqwest::get( &self.endpoint )
      .await
      .map_err( | err | Error::MockApiError( format!("Failed to send request: {:?}", err) ))?;

      let response_status = response.status();

      if response_status == StatusCode::OK 
      {
        let text_body = response
        .text()
        .await
        .map_err( | err | Error::MockApiError( format!( "Failed to read response text: {:?}", err ) ) )?;

        let json_value = serde_json::from_str( &text_body )
        .map_err( | err | {
            Error::MockApiError( format!( "Failed to parse JSON into Value: {:?}", err ) )
        } )?;

        let value_range: ValueRange = serde_json::from_value( json_value )
        .map_err( | err | {
            Error::MockApiError( format!( "Failed to parse JSON into ValueRange: {:?}", err ) )
        } )?;

        Ok( value_range )
      } 
      else 
      {
        Err( Error::MockApiError( format!( "Status code: {}", response_status ) ) )
      }

    }
  }

}

crate::mod_interface!
{
  own use
  {
    MockClient,
  };
}