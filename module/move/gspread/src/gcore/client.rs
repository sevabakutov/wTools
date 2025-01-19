mod private
{

  use reqwest;
  use yup_oauth2;
  use former::Former;
 
  use crate::*;
  use gcore::Secret;
  use gcore::error::{ Error, Result };
  use ser::
  { 
    self, 
    Serialize, 
    Deserialize 
  };
  
  /// # Gspread Client
  ///
  /// A struct that represents a client for interacting with Google Spreadsheets.
  ///
  /// This structure encapsulates the essential information and methods needed to
  /// authenticate and send requests to the Google Sheets API. It uses the [`Former`]
  /// procedural macro to provide builder-like functionality, allowing you to
  /// configure fields (like `token` and `endpoint`) before finalizing an instance.
  ///
  /// ## Fields
  ///
  /// - `token`  
  ///   - A `String` representing the OAuth2 access token needed to perform requests
  ///     against the Google Sheets API.  
  ///   - Typically set using the `token(&Secret)` method (see below).
  ///
  /// - `endpoint`  
  ///   - A `String` specifying the base API endpoint for Google Sheets.  
  ///   - Defaults to `"https://sheets.googleapis.com/v4/spreadsheets"` if no value
  ///     is provided.
  /// 
  /// ## Methods
  /// 
  /// - **`spreadsheet` → [`SpreadSheetValuesMethod`]**
  ///   Returns  [`SpreadSheetValuesMethod`].
  ///
  /// ## Usage
  ///
  /// An instance of `Client` can be created via its `Former` implementation. You have to
  /// set the `token` dynamically by providing a [`Secret`] to the `token( &Secret )`
  /// method, which handles OAuth2 authentication under the hood.
  /// You can use this client also for mock testing. In such case you need to provide `endpoint`
  /// using `endpoint( url )` and there is no need to set `token`.
  /// 
  /// ```rust,ignore
  /// # use gspread::*;
  /// # use gcore::Secret;
  /// # use gcore::client::Client;
  /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// dotenv().ok();
  /// let secret = Secret::read();
  ///
  /// // Build a new Client with an OAuth2 token
  /// let client = Client::former()
  ///     .token(&secret)
  ///     .await? 
  ///     .form();
  ///
  /// # Ok(())
  /// # }
  /// ```
  /// 
  /// Once the `Client` is fully constructed, you can use the `spreadsheet()` method
  /// to access various Google Sheets API operations, such as reading or updating
  /// spreadsheet cells.
  #[ derive( Former ) ]
  pub struct Client
  {
    #[ former( default = "" ) ]
    #[ scalar( setter = false ) ]
    token : String,
    #[ former( default = "https://sheets.googleapis.com/v4/spreadsheets".to_string() ) ]
    endpoint : String,
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
    pub async fn token( mut self, secret : &Secret ) -> Result< Self >
    {
      debug_assert!( self.storage.token.is_none() );

      let secret: yup_oauth2::ApplicationSecret = yup_oauth2::ApplicationSecret
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

      let scopes = vec!
      [ 
        "https://www.googleapis.com/auth/spreadsheets",
        "https://www.googleapis.com/auth/spreadsheets.readonly" 
      ];

      let access_token = authenticator
      .token( &scopes )
      .await
      .map_err( | err | Error::AuthError( err.to_string() ) )?;

      let token = access_token.token().unwrap();
      
      self.storage.token = Some( token.to_string() );

      Ok( self )
    }
  }


  /// # SpreadSheetValuesMethod
  ///
  /// A helper struct that provides methods for working with spreadsheet values in the
  /// Google Sheets API. This struct is associated with a given [`Client`] instance and
  /// offers specialized methods for retrieving and updating data within a spreadsheet.
  ///
  /// ## Fields
  ///
  /// - `client`  
  ///   - A reference to a [`Client`] object.  
  ///   - Used to perform authenticated HTTP requests against the Google Sheets API.
  ///
  /// ## Methods
  ///
  /// - **`values_get(spreadsheet_id, range)` → [`ValuesGetMethod`]**  
  ///   Creates a new request object that retrieves the values within the specified `range`
  ///   of the spreadsheet identified by `spreadsheet_id`. 
  ///
  /// - **`values_update(value_range, spreadsheet_id, range)` → [`ValuesUpdateMethod`]**  
  ///   Creates a new request object that updates the values within the specified `range`
  ///   of the spreadsheet identified by `spreadsheet_id`, using the provided `value_range`.
  ///
  /// - **`values_batch_update(spreadsheet_id, req)` → [`ValuesBatchUpdateMethod`]**  
  ///   Creates a new request object that performs multiple updates on the spreadsheet
  ///   identified by `spreadsheet_id`, based on the instructions defined in
  ///   `BatchUpdateValuesRequest`.
  ///
  /// ## Usage
  ///
  /// This struct is usually obtained by calling the `spreadsheet()` method on a
  /// fully-initialized [`Client`] instance:

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
      }
    }
  }

  /// # ValuesGetMethod
  ///
  /// Represents a specialized request builder for retrieving values from a Google Spreadsheet.
  ///
  /// This struct is constructed internally by the library when calling
  /// [`SpreadSheetValuesMethod::values_get`]. It holds references and parameters
  /// required to execute a `GET` request against the Google Sheets API to fetch
  /// spreadsheet data.
  ///
  /// ## Fields
  ///
  /// - `client`  
  ///   A reference to the [`Client`] used for sending authenticated requests.
  /// - `_spreadsheet_id`  
  ///   The `String` ID of the spreadsheet from which values are fetched.
  /// - `_range`  
  ///   The `String` representing the cell range (e.g. `"A1:B10"`) to retrieve values for.
  /// - `_major_dimension`  
  ///   An optional [`Dimension`] that specifies whether the range is in rows or columns.
  /// - `_value_render_option`  
  ///   An optional [`ValueRenderOption`] that indicates how values should be
  ///   rendered in the response (e.g., formatted, unformatted or formula).
  /// - `_date_time_render_option`  
  ///   An optional [`DateTimeRenderOption`] specifying how date/time values are
  ///   rendered in the response.
  ///
  /// ## Method
  ///
  /// - `doit()`  
  ///   Sends the configured request to the Google Sheets API to retrieve the
  ///   specified range of values. Returns a [`ValueRange`] on success, or an
  ///   [`Error`] if the API request fails.
  ///
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
    /// Executes the request configured by `ValuesGetMethod`.
    ///
    /// Performs an HTTP `GET` to retrieve values for the configured spreadsheet range.
    /// On success, returns the [`ValueRange`] containing the fetched data.
    /// If the request fails or the response cannot be parsed, returns an [`Error`].
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

      let response = reqwest::Client::new()
      .get( endpoint )
      .query( &query )
      .bearer_auth( &self.client.token )
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


  /// # ValuesUpdateMethod
  ///
  /// Represents a specialized request builder for updating values in a Google Spreadsheet.
  ///
  /// This struct is constructed internally by the library when calling
  /// [`SpreadSheetValuesMethod::values_update`]. It holds references and parameters
  /// required to execute a `PUT` request against the Google Sheets API to modify
  /// spreadsheet data.
  ///
  /// ## Fields
  ///
  /// - `client`  
  ///   A reference to the [`Client`] used for sending authenticated requests.
  /// - `_value_range`  
  ///   A [`ValueRange`] describing the new data to be written to the spreadsheet.
  /// - `_spreadsheet_id`  
  ///   A `&str` denoting the spreadsheet’s identifier.
  /// - `_range`  
  ///   A `&str` specifying the cell range (e.g. `"A1:B10"`) where the values should be updated.
  /// - `_value_input_option`  
  ///   A [`ValueInputOption`] that indicates how the input data should be parsed
  ///   (e.g., as user-entered or raw data).
  /// - `_include_values_in_response`  
  ///   An optional `bool` indicating whether the updated values should be
  ///   returned in the response.
  /// - `_response_value_render_option`  
  ///   An optional [`ValueRenderOption`] that specifies how updated values should
  ///   be rendered in the response.
  /// - `_response_date_time_render_option`  
  ///   An optional [`DateTimeRenderOption`] that specifies how date/time values
  ///   should be rendered in the response if `_include_values_in_response` is `true`.
  ///
  /// ## Method
  ///
  /// - `doit()`  
  ///   Sends the configured request to the Google Sheets API to update the specified
  ///   range with new data. Returns an [`UpdateValuesResponse`] on success, or an
  ///   [`Error`] if the API request fails.
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
    /// Executes the request configured by `ValuesUpdateMethod`.
    ///
    /// Performs an HTTP `PUT` to update spreadsheet values within the specified range.
    /// On success, returns an [`UpdateValuesResponse`] describing the result of the
    /// update operation. If the request fails or parsing the response is unsuccessful,
    /// an [`Error`] is returned.
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

      let response = reqwest::Client::new()
      .put( endpoint )
      .query( &query )
      .json( &self._value_range )
      .bearer_auth( &self.client.token )
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

  /// # ValuesBatchUpdateMethod
  ///
  /// Represents a specialized request builder for performing batch updates
  /// of values in a Google Spreadsheet.
  ///
  /// This struct is constructed internally by the library when calling
  /// [`SpreadSheetValuesMethod::values_batch_update`]. It holds the information
  /// required to execute a `POST` request to apply multiple updates in a single
  /// call to the Google Sheets API.
  ///
  /// ## Fields
  ///
  /// - `client`  
  ///   A reference to the [`Client`] used for sending authenticated requests.
  /// - `_spreadsheet_id`  
  ///   The `String` ID of the spreadsheet to be updated.
  /// - `_request`  
  ///   A [`BatchUpdateValuesRequest`] containing multiple update instructions.
  ///
  /// ## Method
  ///
  /// - `doit()`  
  ///   Sends the configured request to the Google Sheets API to perform multiple
  ///   updates on the target spreadsheet. Returns a [`BatchUpdateValuesResponse`]
  ///   on success, or an [`Error`] if the API request fails.
  pub struct ValuesBatchUpdateMethod<'a>
  {
    pub client : &'a Client,
    pub _spreadsheet_id : String,
    pub _request : BatchUpdateValuesRequest
  }

  impl ValuesBatchUpdateMethod<'_>
  {
    /// Executes the request configured by `ValuesBatchUpdateMethod`.
    ///
    /// Performs an HTTP `POST` to apply a batch of updates to the specified
    /// spreadsheet. On success, returns a [`BatchUpdateValuesResponse`] containing
    /// details about the applied updates. If the request fails or the response
    /// cannot be parsed, an [`Error`] is returned.
    pub async fn doit( &self ) -> Result< BatchUpdateValuesResponse >
    {
      let endpoint = format!
      (
        "{}/{}/values:batchUpdate",
        self.client.endpoint,
        self._spreadsheet_id
      );

      let response = reqwest::Client::new()
      .post( endpoint )
      .json( &self._request )
      .bearer_auth( &self.client.token )
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

  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct UpdateValuesResponse
  {
    #[ serde( rename = "spreadsheetId" ) ]
    pub spreadsheet_id : Option< String >,
    #[ serde( rename = "updatedRange" ) ]
    pub updated_range : Option< String >,
    #[ serde( rename = "updatedRows" ) ]
    pub updated_rows : Option< u32 >,
    #[ serde( rename = "updatedColumns" ) ]
    pub updated_columns : Option< u32 >,
    #[ serde( rename = "updatedCells" ) ]
    pub updated_cells : Option< u32 >,
    #[ serde( rename = "updatedData" ) ]
    pub updated_data : Option< ValueRange >
  }

  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct BatchUpdateValuesResponse
  {
    #[ serde( rename = "spreadsheetId" ) ]
    pub spreadsheet_id : Option< String >,
    #[ serde( rename = "totalUpdatedRows" ) ]
    pub total_updated_rows : Option< u32 >,
    #[ serde( rename = "totalUpdatedColumns" ) ]
    pub total_updated_columns : Option< u32 >,
    #[ serde( rename = "totalUpdatedCells" ) ]
    pub total_updated_cells : Option< u32 >,
    #[ serde( rename = "totalUpdatedSheets" ) ]
    pub total_updated_sheets : Option< u32 >,
    pub responses : Option< Vec< ValueRange > >
  }
   /// Determines how dates should be rendered in the output.
  #[ derive( Debug, Clone, Copy, Serialize ) ]
  pub enum DateTimeRenderOption
  {
    /// Instructs date, time, datetime, and duration fields to be output as doubles in "serial number" format, as popularized by Lotus 1-2-3. The whole number portion of the value (left of the decimal) counts the days since December 30th 1899. The fractional portion (right of the decimal) counts the time as a fraction of the day. For example, January 1st 1900 at noon would be 2.5, 2 because it's 2 days after December 30th 1899, and .5 because noon is half a day. February 1st 1900 at 3pm would be 33.625. This correctly treats the year 1900 as not a leap year.
    #[ serde( rename = "SERIAL_NUMBER" ) ]
    SerialNumber,
    /// Instructs date, time, datetime, and duration fields to be output as strings in their given number format (which depends on the spreadsheet locale).
    #[ serde( rename = "FORMATTED_STRING" ) ]
    FormattedString
  }

  /// Determines how values should be rendered in the output.
  #[ derive( Debug, Clone, Copy, Serialize ) ]
  pub enum ValueRenderOption
  {
    /// Values will be calculated & formatted in the response according to the cell's formatting. Formatting is based on the spreadsheet's locale, not the requesting user's locale. For example, if A1 is 1.23 and A2 is =A1 and formatted as currency, then A2 would return "$1.23".
    #[ serde( rename = "FORMATTED_VALUE" ) ]
    FormattedValue,
    /// Values will be calculated, but not formatted in the reply. For example, if A1 is 1.23 and A2 is =A1 and formatted as currency, then A2 would return the number 1.23.
    #[ serde( rename = "UNFORMATTED_VALUE" ) ]
    UnformattedValue,
    /// Values will not be calculated. The reply will include the formulas. For example, if A1 is 1.23 and A2 is =A1 and formatted as currency, then A2 would return "=A1".
    ///
    /// Sheets treats date and time values as decimal values. This lets you perform arithmetic on them in formulas. For more information on interpreting date and time values, see About date & time values.
    #[ serde( rename = "FORMULA" ) ]
    Formula
  }

  /// Determines how input data should be interpreted.
  #[ derive( Debug, Clone, Copy, Default, Serialize ) ]
  pub enum ValueInputOption
  {
    /// The values the user has entered will not be parsed and will be stored as-is.
    #[ default ]
    #[ serde( rename = "RAW" ) ]
    Raw,
    /// The values will be parsed as if the user typed them into the UI. Numbers will stay as numbers, but strings may be converted to numbers, dates, etc. following the same rules that are applied when entering text into a cell via the Google Sheets UI.
    #[ serde( rename = "USER_ENTERED" ) ]
    UserEntered
  }

  /// Indicates which dimension an operation should apply to.
  #[ derive( Debug, Clone, Copy, Serialize, Deserialize ) ]
  pub enum Dimension 
  {
    /// Operates on the rows of a sheet.
    #[ serde( rename = "ROWS" ) ]
    Row,
    /// Operates on the columns of a sheet.
    #[ serde( rename = "COLUMNS" ) ]
    Column,
  }

  /// Data within a range of the spreadsheet.
  #[ derive( Debug, Clone, Default, serde::Serialize, serde::Deserialize ) ]
  pub struct ValueRange
  {
    /// The range the values cover, in A1 notation. For output, this range indicates the entire requested range, even though the values will exclude trailing rows and columns. When appending values, this field represents the range to search for a table, after which values will be appended.
    pub range : Option< String >,
    /// The major dimension of the values.
    /// For output, if the spreadsheet data is: A1=1,B1=2,A2=3,B2=4, then requesting range=A1:B2,majorDimension=ROWS will return [[1,2],[3,4]], whereas requesting range=A1:B2,majorDimension=COLUMNS will return [[1,3],[2,4]].
    ///
    /// For input, with range=A1:B2,majorDimension=ROWS then [[1,2],[3,4]] will set A1=1,B1=2,A2=3,B2=4. With range=A1:B2,majorDimension=COLUMNS then [[1,2],[3,4]] will set A1=1,B1=3,A2=2,B2=4.
    ///
    /// When writing, if this field is not set, it defaults to ROWS.
    #[ serde( rename = "majorDimension" ) ]
    pub major_dimension : Option< Dimension >,
    /// The data that was read or to be written. This is an array of arrays, the outer array representing all the data and each inner array representing a major dimension. Each item in the inner array corresponds with one cell.
    ///
    /// For output, empty trailing rows and columns will not be included.
    /// 
    /// For input, supported value types are: bool, string, and double. Null values will be skipped. To set a cell to an empty value, set the string value to an empty string.
    pub values : Option< Vec< Vec< serde_json::Value > > >
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