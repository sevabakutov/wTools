//!
//! Module to manage with secrets.
//!

mod private
{
  use crate::*;
  use std::
  {
    env,
    sync::OnceLock,
  };

  use error_tools::typed::Error;
  use ser::DisplayFromStr;

  /// # Secret's Errors
  /// 
  /// This enumeration defines errors that can occur while working with secrets.
  /// 
  /// **Errors:**
  /// 
  /// - `SecretFileIllformed`
  ///   - Occurs when the secret file is not properly formatted.
  ///   - Associated data:
  ///     - `dotenv::Error`: Provides details about the specific formatting issue.
  /// 
  /// - `VariableMissing`
  ///   - Indicates that a required variable is missing from the secret configuration.
  ///   - Associated data:
  ///     - `&'static str`: The name of the missing variable.
  /// 
  /// - `VariableIllformed`
  ///   - Signals an issue while processing a specific secret variable.
  ///   - Associated data:
  ///     - `&'static str`: The name of the variable that caused the issue.
  ///     - `String`: Detailed error message or explanation.
  #[ ser::serde_as ]
  #[ derive( Debug, Error, ser::Serialize ) ]
  #[ serde( tag = "type", content = "data" ) ]
  pub enum Error
  {
    #[ error( "Secret file is illformed\n{0}" ) ]
    SecretFileIllformed
    (
      #[ from ]
      #[ serde_as( as = "DisplayFromStr" ) ]
      dotenv::Error
    ),

    #[ error( "Secret missing the variable {0}" ) ]
    VariableMissing( &'static str ),

    #[ error( "Secret error processing in the variable {0}\n{1}" ) ]
    VariableIllformed( &'static str, String ),

  }

  /// # Result
  ///
  /// A type alias for `std::result::Result` with the error type `Error`.
  pub type Result< R > = std::result::Result< R, Error >;

  /// # Secret
  ///
  /// A struct that represents configuration secrets loaded from a `.env` file.
  ///
  /// This structure contains essential fields required for authentication and token management,
  /// such as client credentials and URIs.
  ///
  /// ## Fields
  ///
  /// - `CLIENT_SECRET`  
  ///   - A `String` containing the client secret used for authentication.
  /// - `CLIENT_ID`  
  ///   - A `String` containing the client ID associated with the application.
  /// - `AUTH_URI`  
  ///   - A `String` representing the authentication URI used for OAuth2 flows.  
  ///   - Defaults to `"https://accounts.google.com/o/oauth2/auth"` if not specified in the `.env` file.
  /// - `TOKEN_URI`  
  ///   - A `String` representing the token URI used to retrieve OAuth2 tokens.  
  ///   - Defaults to `"https://oauth2.googleapis.com/token"` if not specified in the `.env` file.
  ///
  /// ## Usage
  ///
  /// The `Secret` struct is intended to be loaded from a `.env` file using the `dotenv` crate.
  /// It provides methods for loading and accessing these secrets within the application.
  ///
  /// Example of fields in a `.env` file:
  /// ```text
  /// CLIENT_SECRET=your_client_secret
  /// CLIENT_ID=your_client_id
  /// AUTH_URI=https://accounts.google.com/o/oauth2/auth
  /// TOKEN_URI=https://oauth2.googleapis.com/token
  /// ```
  #[ derive( Debug ) ]
  #[ allow( non_snake_case ) ]
  pub struct Secret
  {
    pub CLIENT_SECRET : String,
    pub CLIENT_ID: String,
    pub AUTH_URI : String,
    pub TOKEN_URI : String,
  }

  impl Secret
  {
    #[ allow( non_snake_case ) ]
    pub fn load() -> Result< Self >
    {
      let path = "./.secret/.env";

      let r = dotenv::from_path( path );
      if let Err( ref err ) = r
      {
        if !matches!( err, dotenv::Error::Io(_) )
        {
          return Err( r.expect_err( &format!( "Failed to load {path}" ) ).into() );
        }
      }

      let config = Self
      {
        CLIENT_SECRET : var( "CLIENT_SECRET", None )?,
        CLIENT_ID : var( "CLIENT_ID", None )?,
        AUTH_URI : var ( "AUTH_URI", Some( DEFAULT_AUTH_URI ) )?,
        TOKEN_URI : var ( "TOKEN_URI", Some( DEFAULT_TOKEN_URI ) )?
      };
      Ok( config )
    }

    pub fn read() -> Secret
    {
      Self::load().unwrap_or_else( | err |
      {
        let example = include_str!("../../.secret/readme.md");
        let explanation = format!
        (
                  r#" = Lack of secrets

Failed to load secret or some its parameters.
{err}

 = Fix

Add missing secret to .env file in .secret directory. Example: MISSING_SECRET=YOUR_MISSING_SECRET

 = More information

{example}
"#
        );
        panic!( "{}", explanation );
       })
    }

    pub fn get() -> &'static Secret
    {
      static INSTANCE : OnceLock< Secret > = OnceLock::new();
      INSTANCE.get_or_init( || Self::read() )
    }

  }

  /// # `var`
  ///
  /// Retrieves the value of an environment variable, or returns a default value if the variable is not set.
  ///
  /// **Parameters:**
  /// - `name`:  
  ///   A `&'static str` specifying the name of the environment variable to retrieve.
  /// - `default`:  
  ///   An `Option<&'static str>` containing the default value to return if the variable is not set.  
  ///   If `None`, an error is returned when the variable is missing.
  ///
  /// **Returns:**  
  /// - `Result<String>`:  
  fn var
  (
    name : &'static str,
    default : Option< &'static str >,
  ) -> Result < String >
  {
    match env::var( name )
    {
      Ok( val ) => Ok ( val ),
      Err( _ ) =>
      {
        if let Some( default_value ) = default
        {
          Ok( default_value.to_string() )
        }
        else
        {
          Err ( Error::VariableMissing( name ) )
        }
      }
    }
  }

  /// # `_var_path`
  ///
  /// Retrieves the value of an environment variable, interprets it as a path, and converts it to an absolute path.
  ///
  /// **Parameters:**
  /// - `name`:  
  ///   A `&'static str` specifying the name of the environment variable to retrieve.
  /// - `default`:  
  ///   An `Option<&'static str>` containing the default value to use if the variable is not set.  
  ///   If `None`, an error is returned when the variable is missing.
  ///
  /// **Returns:**  
  /// - `Result<pth::AbsolutePath>`
  fn _var_path
  (
    name : &'static str,
    default : Option<&'static str>,
  ) -> Result < pth::AbsolutePath >
  {
    let p = var( name, default )?;
    pth::AbsolutePath::from_paths( ( pth::CurrentPath, p ) )
    .map_err( |e| Error::VariableIllformed( name, e.to_string() ) )
  }

}

crate::mod_interface!
{
  own use
  {
    Error,
    Result,
  };

  orphan use
  {
    Secret,
  };
}