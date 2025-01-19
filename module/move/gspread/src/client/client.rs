mod private
{

  use crate::*;
  use actions::gspread::{Error, Result};
use former::Former;
  use reqwest;
  use serde::{Deserialize, Serialize};
  use ser::JsonValue;
  use yup_oauth2::{authenticator::Authenticator, ApplicationSecret};
  use hyper_util::client::legacy::connect::HttpConnector;
  use hyper_rustls::HttpsConnector;

  
  /// Gspread Client. 
  #[ derive( Former ) ]
  pub struct Client
  {
    #[ scalar( setter = false ) ]
    auth : Authenticator< HttpsConnector< HttpConnector > >,
    #[ former( default = "https://sheets.googleapis.com/v4/spreadsheets".to_string() ) ]
    endpoint : String,
    #[ former( default = vec![ 
      "https://www.googleapis.com/auth/spreadsheets",
      "https://www.googleapis.com/auth/spreadsheets.readonly"
      ] 
    ) ]
    scopes : Vec< &'static str >
  }

  impl Client
  {
    pub fn spreadsheet( &self ) -> SpreadSheetValuesMethod
    {
      SpreadSheetValuesMethod
      {
        client : self
      }
    }
  }


  // Custom initialization for auth field.
  impl< Definition > ClientFormer< Definition >
  where
    Definition : former::FormerDefinition< Storage = ClientFormerStorage >,
  {
    pub async fn auth( mut self, secret : &Secret ) -> Result< Self >
    {
      debug_assert!( self.storage.auth.is_none() );

      let secret: ApplicationSecret = ApplicationSecret
      {
        client_id : secret.CLIENT_ID.clone(),
        auth_uri : secret.AUTH_URI.clone(),
        token_uri : secret.TOKEN_URI.clone(),
        client_secret : secret.CLIENT_SECRET.clone(),
        .. Default::default()
      };

      let authenticator  = yup_oauth2::InstalledFlowAuthenticator::builder(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
      )
      .build()
      .await
      .map_err( | err | Error::AuthError( err.to_string() ) )?;

      self.storage.auth = Some( authenticator );

      Ok( self )
    }
  }


  pub struct SpreadSheetValuesMethod<'a>
  {
    client : &'a Client,
  }

  impl SpreadSheetValuesMethod<'_>
  {
    pub fn values_get
    (
      &self,
      spreadsheet_id : &str,
      range : &str
    ) -> ValuesGetMethod
    {
      ValuesGetMethod
      {
        client : self.client,
        _spreadsheet_id : spreadsheet_id.to_string(),
        _range : range.to_string(),
        _major_dimension : Default::default(),
        _value_render_option : Default::default(),
        _date_time_render_option : Default::default()
      }
    }

    pub fn values_update<'a>
    ( 
      &'a self,
      value_range : ValueRange,
      spreadsheet_id : &'a str,
      range : &'a str 
    ) -> ValuesUpdateMethod<'a>
    {
      ValuesUpdateMethod
      {
        client : self.client,
        _value_range : value_range,
        _spreadsheet_id : spreadsheet_id,
        _range : range,
        _value_input_option : ValueInputOption::default(),
        _include_values_in_response : Default::default(),
        _response_value_render_option : Default::default(),
        _response_date_time_render_option : Default::default()
      }
    }

    pub fn values_batch_update
    ( 
      &self,
      spreadsheet_id : &str,
      req : BatchUpdateValuesRequest,
    ) -> ValuesBatchUpdateMethod
    {
      ValuesBatchUpdateMethod
      {
        client : self.client,
        _spreadsheet_id : spreadsheet_id.to_string(),
        _request : req,
        // _value_input_option : ValueInputOption::default(),
        // _include_values_in_response : Default::default(),
        // _response_date_time_render_option : Default::default(),
        // _response_value_render_option : Default::default()
      }
    }
  }

  pub struct ValuesGetMethod<'a>
  {
    client : &'a Client,
    _spreadsheet_id : String,
    _range : String,
    _major_dimension : Option< Dimension >,
    _value_render_option : Option< ValueRenderOption >,
    _date_time_render_option : Option< DateTimeRenderOption >
  }

  impl ValuesGetMethod<'_>
  {
    pub async fn doit( &self ) -> Result< ValueRange >
    {
      let endpoint = format!
      ( 
        "{}/{}/values/{}", 
        self.client.endpoint, 
        self._spreadsheet_id, 
        self._range 
      );

      let query = GetValuesRequest
      {
        major_dimension : self._major_dimension,
        value_render_option : self._value_render_option,
        date_time_render_option : self._date_time_render_option
      };

      let access_token = self
      .client
      .auth
      .token( &self.client.scopes )
      .await
      .map_err( | err | Error::AuthError( err.to_string() ) )?;

      let token = access_token.token().unwrap();

      let response = reqwest::Client::new()
      .get( endpoint )
      .query( &query )
      .bearer_auth( token )
      .send()
      .await
      .map_err( | err | Error::ApiError( err.to_string() ) )?;

      if !response.status().is_success()
      {
        let response_text = response
        .text()
        .await
        .map_err( | err | Error::ParseError( err.to_string() ) )?;

        return Err( Error::ApiError( response_text ) )
      }

      let value_range = response.json::< ValueRange >()
      .await
      .map_err( | err | Error::ParseError( err.to_string() ))?;

      Ok( value_range )
    }
  }

  pub struct ValuesUpdateMethod<'a>
  {
    client : &'a Client,
    _value_range : ValueRange,
    _spreadsheet_id : &'a str,
    _range : &'a str,
    _value_input_option : ValueInputOption,
    _include_values_in_response : Option< bool >,
    _response_value_render_option : Option< ValueRenderOption >,
    _response_date_time_render_option : Option< DateTimeRenderOption >
  }

  impl ValuesUpdateMethod<'_>
  {
    pub async fn doit( &self ) -> Result< UpdateValuesResponse >
    {
      let endpoint = format!
      (
        "{}/{}/values/{}", 
        self.client.endpoint, 
        self._spreadsheet_id, 
        self._range
      );

      let query = UpdateValuesRequest
      {
        value_input_option : self._value_input_option,
        include_values_in_response : self._include_values_in_response,
        response_value_render_option : self._response_value_render_option,
        response_date_time_render_option : self._response_date_time_render_option
      };

      let access_token = self
      .client
      .auth
      .token( &self.client.scopes )
      .await
      .map_err( | err | Error::AuthError( err.to_string() ))?;

      let token = access_token.token().unwrap();

      let response = reqwest::Client::new()
      .post( endpoint )
      .query( &query )
      .json( &self._value_range )
      .bearer_auth( token )
      .send()
      .await
      .map_err( | err | Error::ApiError( err.to_string() ) )?;

      if !response.status().is_success()
      {
        let response_text = response
        .text()
        .await
        .map_err( | err | Error::ParseError( err.to_string() ) )?;

        return Err( Error::ApiError( response_text ) );
      }

      let parsed_response = response.json::< UpdateValuesResponse >()
      .await
      .map_err( | err | Error::ParseError( err.to_string() ) )?;

      Ok( parsed_response )
    }

  }

  pub struct ValuesBatchUpdateMethod<'a>
  {
    pub client : &'a Client,
    pub _spreadsheet_id : String,
    pub _request : BatchUpdateValuesRequest
    // pub _data : Vec< ValueRange >,
    // pub _value_input_option : ValueInputOption,
    // pub _include_values_in_response : Option< bool >,
    // pub _response_value_render_option : Option< ValueRenderOption >,
    // pub _response_date_time_render_option : Option< DateTimeRenderOption >,
  }

  impl ValuesBatchUpdateMethod<'_>
  {
    pub async fn doit( &self ) -> Result< BatchUpdateValuesResponse >
    {
      let endpoint = format!
      (
        "{}/{}/values:batchUpdate",
        self.client.endpoint,
        self._spreadsheet_id
      );

      let access_token = self
      .client
      .auth
      .token( &self.client.scopes )
      .await
      .map_err( | err | Error::AuthError( err.to_string() ) )?;

      let token = access_token.token().unwrap();

      let response = reqwest::Client::new()
      .post( endpoint )
      .json( &self._request )
      .bearer_auth( token )
      .send()
      .await
      .map_err( | err | Error::ApiError( err.to_string() ) )?;

      if !response.status().is_success()
      {
        let response_text = response
        .text()
        .await
        .map_err( | err | Error::ParseError( err.to_string() ) )?;

        return Err( Error::ApiError( response_text ) );  
      }

      let parsed_response = response.json::< BatchUpdateValuesResponse >()
      .await
      .map_err( | err | Error::ParseError( err.to_string() ) )?;

      Ok( parsed_response )
    }
  }

  #[ derive( Debug, Serialize ) ]
  pub struct GetValuesRequest
  {
    #[ serde( rename = "majorDimension" ) ]
    major_dimension : Option< Dimension >,
    #[ serde( rename = "valueRenderOption" ) ]
    value_render_option : Option< ValueRenderOption >,
    #[ serde( rename = "dateTimeRenderOption" ) ]
    date_time_render_option : Option< DateTimeRenderOption >
  }

  #[ derive( Debug, Serialize ) ]
  pub struct UpdateValuesRequest
  {
    #[ serde( rename = "valueInputOption" )]
    value_input_option : ValueInputOption,
    #[ serde( rename = "includeValuesInResponse" ) ]
    include_values_in_response : Option< bool >,
    #[ serde( rename = "responseValueRenderOption" ) ]
    response_value_render_option : Option< ValueRenderOption >,
    #[ serde( rename = "responseDateTimeRenderOption" ) ]
    response_date_time_render_option : Option< DateTimeRenderOption >
  }

  #[ derive( Debug, Serialize ) ]
  pub struct BatchUpdateValuesRequest 
  {
    pub data : Vec< ValueRange >,
    #[ serde( rename = "valueInputOption" ) ]
    pub value_input_option : ValueInputOption,
    #[ serde( rename = "includeValuesInResponse" ) ]
    pub include_values_in_response : Option< bool >,
    #[ serde( rename = "responseValueRenderOption" ) ]
    pub response_value_render_option : Option< ValueRenderOption >,
    #[ serde( rename = "responseDateTimeRenderOption" ) ]
    pub response_date_time_render_option : Option< DateTimeRenderOption >,
  }

  #[ derive( Debug, Deserialize ) ]
  pub struct UpdateValuesResponse
  {
    pub spreadsheet_id : Option< String >,
    pub updated_range : Option< String >,
    pub updated_rows : Option< u32 >,
    pub updated_columns : Option< u32 >,
    pub updated_cells : Option< u32 >,
    pub updated_data : Option< ValueRange >
  }

  #[ derive( Debug, Deserialize ) ]
  pub struct BatchUpdateValuesResponse
  {
    pub spreadsheet_id : Option< String >,
    pub total_updated_rows : Option< u32 >,
    pub total_updated_columns : Option< u32 >,
    pub total_updated_cells : Option< u32 >,
    pub total_updated_sheets : Option< u32 >,
    pub responses : Option< Vec< ValueRange > >
  }

  #[ derive( Debug, Clone, Copy, Serialize ) ]
  pub enum DateTimeRenderOption
  {
    #[ serde( rename = "SERIAL_NUMBER" ) ]
    SerialNumber,
    #[ serde( rename = "FORMATTED_STRING" ) ]
    FormattedString
  }

  #[ derive( Debug, Clone, Copy, Serialize ) ]
  pub enum ValueRenderOption
  {
    #[ serde( rename = "FORMATTED_VALUE" ) ]
    FormattedValue,
    #[ serde( rename = "UNFORMATTED_VALUE" ) ]
    UnformattedValue,
    #[ serde( rename = "FORMULA" ) ]
    Formula
  }

  #[ derive( Debug, Clone, Copy, Default, Serialize ) ]
  pub enum ValueInputOption
  {
    #[ default ]
    #[ serde( rename = "RAW" ) ]
    Raw,
    #[ serde( rename = "USER_ENTERED" ) ]
    UserEntered
  }

  #[ derive( Debug, Clone, Copy, Serialize, Deserialize ) ]
  pub enum Dimension 
  {
    #[ serde( rename = "ROWS" ) ]
    Row,
    #[ serde( rename = "COLUMNS" ) ]
    Column,
  }

  ///  Values which google returns or we send to google api.
  #[ derive( Debug, Clone, Default, Serialize, Deserialize ) ]
  pub struct ValueRange
  {
    /// Range. For example A1
    pub range : Option< String >,
    /// Indicates dimension.
    #[ serde( rename = "majorDimension" ) ]
    pub major_dimension : Option< Dimension >,
    /// Double vector.
    pub values : Option< Vec< Vec< JsonValue > > >
  }

}


crate::mod_interface!
{
  own use
  {
    Client,
    Dimension,
    ValueRange,
    ValueInputOption,
    ValueRenderOption,
    UpdateValuesResponse,
    BatchUpdateValuesRequest,
    BatchUpdateValuesResponse,
  };
}