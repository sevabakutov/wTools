


mod private
{
  use crate::*;
  use gcore::client::Client;
  use gcore::error::{ Error, Result };
  use actions::gspread::
  {
    check_variant, 
    parse_json, 
    update_rows_by_custom_row_key, 
    OnFail, 
    OnFind
  };

  /// # parse_key_by
  /// 
  /// Parse a provided string to ( &str, serde_json::Value )
  /// 
  /// ## Errors
  /// 
  /// Can occur if passed string is not valid.
  fn parse_key_by( s: &str ) -> Result< ( &str, serde_json::Value ) >
  {
    let result: ( &str, serde_json::Value ) = serde_json::from_str( s )
    .map_err( | err | Error::ParseError( format!( "Failed to parse key_by. {}", err ) ) )?;
    
    Ok( result )
  }

  /// # parse_on_find
  /// 
  /// Parse provided string to OnFind's variant.
  /// 
  /// ## Errors
  /// 
  /// Can occur if variant is not allowed.
  fn parse_on_find( on_find: &str ) -> Result< OnFind >
  {
    check_variant( on_find, vec![ "first", "last", "all" ] )?;
    match on_find
    {
      "first" => Ok( OnFind::UpdateFirstMatchedRow ),
      "last" => Ok( OnFind::UpdateLastMatchedRow ),
      "all" => Ok( OnFind::UpdateAllMatchedRow ),
      &_ => Err( Error::ParseError( format!( "OnFind prase error." ) ) )
    }
  }

  /// # parse_on_fail
  /// 
  /// Parse provided string to OnFail's variant.
  /// 
  /// ## Errors
  /// 
  /// Can occur if variant is not allowed.
  fn parse_on_fail( on_fail: &str ) -> Result< OnFail >
  {
    check_variant( on_fail, vec![ "none", "error", "append" ] )?;
    match on_fail
    {
      "none" => Ok( OnFail::Nothing ),
      "error" => Ok( OnFail::Error ),
      "append" => Ok( OnFail::AppendRow ),
      &_ => Err( Error::ParseError( format!( "OnFail parse error." ) ) )
    }
  }

  pub async fn action
  (
    client : &Client<'_>,
    spreadsheet_id : &str,
    sheet_name : &str,
    key_by : &str,
    json_str : &str,
    on_find : &str,
    on_fail : &str
  ) -> Result< u32 >
  {
    let key_by = match parse_key_by( key_by )
    {
      Ok( val ) => val,
      Err( error ) => return Err( error ),
    };

    let on_find = parse_on_find( on_find )?;
    let on_fail = parse_on_fail( on_fail )?;

    match parse_json( json_str )
    {
      Ok( parsed_json ) =>
      {
        match update_rows_by_custom_row_key
        ( 
          client, 
          spreadsheet_id, 
          sheet_name, 
          key_by, 
          parsed_json, 
          on_find, 
          on_fail 
        ).await
        {
          Ok( response ) => Ok
          ( 
            match response.responses
            {
              Some( _ ) => match response.total_updated_cells
              {
                Some( amount ) => amount,
                None => 0
              },
              None => 0,
            } 
          ),
          Err( error ) => Err( error )
        }
      },

      Err( error ) => Err( error )
    }
  }
}

crate::mod_interface!
{
  own use
  {
    action
  };
}