/// Define a private namespace for all its items.
mod private
{
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use std::error::Error as ErrorTrait;

  /// This trait allows adding extra context or information to an error, creating a tuple of the additional
  /// context and the original error. This is particularly useful for error handling when you want to include
  /// more details in the error without losing the original error value.
  ///
  /// The `ErrWith` trait provides methods to wrap an error with additional context, either by using a closure
  /// that generates the context or by directly providing the context.
  ///
  /// ```
  pub trait ErrWith< ReportErr, ReportOk, E >
  {
    /// Takes a closure `f` that returns a value of type `ReportErr`, and uses it to wrap an error of type `(ReportErr, E)`
    /// in the context of a `Result` of type `ReportOk`.
    ///
    /// This method allows you to add additional context to an error by providing a closure that generates the context.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that returns the additional context of type `ReportErr`.
    ///
    /// # Returns
    ///
    /// A `Result` of type `ReportOk` if the original result is `Ok`, or a tuple `(ReportErr, E)` containing the additional
    /// context and the original error if the original result is `Err`.
    /// 
    /// # Errors
    /// 
    /// qqq: errors
    ///
    /// # Example
    ///
    /// ```rust
    /// use error_tools::ErrWith;
    /// let result : Result< (), std::io::Error > = Err( std::io::Error::new( std::io::ErrorKind::Other, "an error occurred" ) );
    /// let result_with_context : Result< (), ( &str, std::io::Error ) > = result.err_with( || "additional context" );
    /// ```
    fn err_with< F >( self, f : F ) -> core::result::Result< ReportOk, ( ReportErr, E ) >
    where
      F : FnOnce() -> ReportErr;

    /// Takes a reference to a `ReportErr` value and uses it to wrap an error of type `(ReportErr, E)`
    /// in the context of a `Result` of type `ReportOk`.
    ///
    /// This method allows you to add additional context to an error by providing a reference to the context.
    ///
    /// # Arguments
    ///
    /// * `report` - A reference to the additional context of type `ReportErr`.
    ///
    /// # Returns
    ///
    /// A `Result` of type `ReportOk` if the original result is `Ok`, or a tuple `(ReportErr, E)` containing the additional
    /// context and the original error if the original result is `Err`.
    /// 
    /// # Errors
    /// 
    /// qqq: Errors
    ///
    /// # Example
    ///
    /// ```rust
    /// use error_tools::ErrWith;
    /// let result : Result< (), std::io::Error > = Err( std::io::Error::new( std::io::ErrorKind::Other, "an error occurred" ) );
    /// let report = "additional context";
    /// let result_with_report : Result< (), ( &str, std::io::Error ) > = result.err_with_report( &report );
    /// ```
    fn err_with_report( self, report : &ReportErr ) -> core::result::Result< ReportOk, ( ReportErr, E ) >
    where
      ReportErr : Clone;

  }

  impl< ReportErr, ReportOk, E, IntoError > ErrWith< ReportErr, ReportOk, E >
  for core::result::Result< ReportOk, IntoError >
  where
    IntoError : Into< E >,
  {

    #[ allow( clippy::implicit_return, clippy::min_ident_chars ) ]
    #[ inline ]
    fn err_with< F >( self, f : F ) -> core::result::Result< ReportOk, ( ReportErr, E ) >
    where
      F : FnOnce() -> ReportErr,
    {
      self.map_err( | error | ( f(), error.into() ) )
    }

    #[ inline( always ) ]
    #[ allow( clippy::implicit_return ) ]
    fn err_with_report( self, report : &ReportErr ) -> core::result::Result< ReportOk, ( ReportErr, E ) >
    where
      ReportErr : Clone,
      Self : Sized,
    {
      self.map_err( | error | ( report.clone(), error.into() ) )
    }

  }

  /// A type alias for a `Result` that contains an error which is a tuple of a report and an original error.
  ///
  /// This is useful when you want to report additional information along with an error. The `ResultWithReport` type
  /// helps in defining such results more concisely.
  pub type ResultWithReport< Report, Error > = Result< Report, ( Report, Error ) >;

//   ///
//   /// Macro to generate an error descriptor.
//   ///
//   /// ### Basic use-case.
//   /// ```rust
//   /// # use error_tools::{ BasicError, err };
//   /// fn f1() -> BasicError
//   /// {
//   ///   return err!( "No attr" );
//   /// }
//   /// ```
//   ///
//
//   #[ macro_export ]
//   macro_rules! err
//   {
//
//     ( $msg : expr ) =>
//     {
//       $crate::BasicError::new( $msg ).into()
//     };
//     ( $msg : expr, $( $arg : expr ),+ $(,)? ) =>
//     {
//       $crate::BasicError::new( format!( $msg, $( $arg ),+ ) ).into()
//     };
//
//   }
//
//   ///
//   /// Macro to return an Err( error ) generating error descriptor.
//   ///
//   /// ### Basic use-case.
//   /// ```rust
//   /// # use error_tools::{ BasicError, return_err };
//   /// fn f1() -> Result< (), BasicError >
//   /// {
//   ///   return_err!( "No attr" );
//   /// }
//   /// ```
//   ///
//
//   #[ macro_export ]
//   macro_rules! return_err
//   {
//
//     ( $msg : expr ) =>
//     {
//       return Result::Err( $crate::err!( $msg ) )
//     };
//     ( $msg : expr, $( $arg : expr ),+ $(,)? ) =>
//     {
//       return Result::Err( $crate::err!( $msg, $( $arg ),+ ) )
//     };
//
//   }
//
//   // zzz : review
//   // xxx : rid of
//
//   /// baic implementation of generic BasicError
//
//   #[ derive( core::fmt::Debug, core::clone::Clone, core::cmp::PartialEq, core::cmp::Eq ) ]
//   pub struct BasicError
//   {
//     msg : String,
//   }
//
//   impl BasicError
//   {
//     /// Constructor expecting message with description.
//     pub fn new< Msg : Into< String > >( msg : Msg ) -> BasicError
//     {
//       BasicError { msg : msg.into() }
//     }
//     /// Message with description getter.
//     pub fn msg( &self ) -> &String
//     {
//       &self.msg
//     }
//   }
//
//   impl core::fmt::Display for BasicError
//   {
//     fn fmt(&self, f: &mut core::fmt::Formatter< '_ >) -> core::fmt::Result
//     {
//       write!( f, "{}", self.msg )
//     }
//   }
//
//   impl ErrorTrait for BasicError
//   {
//     fn description( &self ) -> &str
//     {
//       &self.msg
//     }
//   }
//
//   impl< T > From< BasicError > for Result< T, BasicError >
//   {
//     /// Returns the argument unchanged.
//     #[ inline( always ) ]
//     fn from( src : BasicError ) -> Self
//     {
//       Result::Err( src )
//     }
//   }
//
//   pub use err;
//   pub use return_err;

  // qqq : write standard mod interface without using mod_interface /* aaa : Dmytro : added to each library file */
}

/// Assertions.
#[ cfg( feature = "enabled" ) ]
pub mod assert;

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "error_typed" ) ]
/// Typed exceptions handling mechanism.
pub mod typed;

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "error_untyped" ) ]
/// Untyped exceptions handling mechanism.
pub mod untyped;

#[ cfg( feature = "enabled" ) ]
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ allow( clippy::pub_use ) ]
pub use own::*;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod own
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;

  #[ doc( inline ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use orphan::*;

  #[ doc( inline ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use assert::orphan::*;

  #[ cfg( feature = "error_untyped" ) ]
  #[ doc( inline ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use untyped::orphan::*;

  #[ cfg( feature = "error_typed" ) ]
  #[ doc( inline ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use typed::orphan::*;

  #[ doc( inline ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use private::
  {
    // err,
    // return_err,
    ErrorTrait,
    // BasicError,
  };

  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use super::assert;
  #[ cfg( feature = "error_typed" ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use super::typed;
  #[ cfg( feature = "error_untyped" ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use super::untyped;

}

/// Shared with parent namespace of the module
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  #[ doc( inline ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;

  #[ doc( inline ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use prelude::*;

  // Expose itself.
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use super::super::error;

  #[ doc( inline ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use private::
  {
    ErrWith,
    ResultWithReport,
  };

  #[ doc( inline ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use assert::exposed::*;

  #[ cfg( feature = "error_untyped" ) ]
  #[ doc( inline ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use untyped::exposed::*;

  #[ cfg( feature = "error_typed" ) ]
  #[ doc( inline ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use typed::exposed::*;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;

  // #[ doc( inline ) ]
  // pub use private::
  // {
  //   // err,
  //   // return_err,
  //   ErrorTrait,
  //   // BasicError,
  // };

  #[ doc( inline ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use assert::prelude::*;

  #[ cfg( feature = "error_untyped" ) ]
  #[ doc( inline ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use untyped::prelude::*;

  #[ cfg( feature = "error_typed" ) ]
  #[ doc( inline ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use typed::prelude::*;

}
