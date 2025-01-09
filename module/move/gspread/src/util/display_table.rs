//!
//! Module with functions to display HTTP requests results in a table view.
//! 

mod private
{
  use std::fmt;
  use format_tools::
  {
    TableFormatter,
    print,
    output_format,
    TableOutputFormat
  };

  /// Function to display rows in a table view.
  /// 
  /// It calles `display_data` function.
  /// 
  /// **Params**
  ///  - `data` : Data to display.
  ///  - `f` : Formatter.
  /// 
  /// **Return**
  ///  - `Result`
  pub fn display_rows< 'a >
  (
    data :  &'a impl TableFormatter< 'a >,
    f : &mut fmt::Formatter< '_ >
  ) -> fmt::Result
  {
    display_data( data, f, output_format::Table::default() )
  }

  /// Function to display header in a table view.
  /// 
  /// It calles `display_data` function.
  /// 
  /// **Params**
  ///  - `data` : Data to display.
  ///  - `f` : Formatter.
  /// 
  /// **Return**
  ///  - `Result`
  pub fn display_header < 'a >
  (
    data : &'a impl TableFormatter< 'a >,
    f : &mut fmt::Formatter< '_ >
  ) -> fmt::Result
  {
    display_data( data, f, output_format::Table::default() )
  }

  /// Function to display data in a table view.
  /// 
  /// It creates printer and context objects and then passes them to `TableFormatter::fmt`.
  /// 
  /// **Params**
  ///  - `data` : Data to display.
  ///  - `f` : Formatter.
  ///  - `format` : Output format.
  /// 
  /// **Return**
  ///  - `Result`
  pub fn display_data < 'a >
  (
    data : &'a impl TableFormatter< 'a >,
    f : &mut fmt::Formatter< '_ >,
    format : impl TableOutputFormat,
  ) -> fmt::Result
  {
    let printer = print::Printer::with_format( &format );
    let mut context = print::Context::new( f, printer );
    TableFormatter::fmt( data, &mut context )
  }

}

crate::mod_interface!
{
  own use
  {
    display_rows,
    display_header
  };
}