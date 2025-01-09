//!
//! Gspread wrapper for outputting data to console
//!
//! It is used for "header" and "rows" commands
//!

mod private
{
  use format_tools::
  {
    Fields,
    TableWithFields,
  };
  use std::borrow::Cow;
  use crate::*;
  use ser::JsonValue;

  /// Structure to display a raw in console.
  /// 
  /// It implements `TableWithFields` and `Fields` traits.
  /// These implementations allow to display a HTTP requests results in table view in console using `format_tools`.
  ///  
  /// **Fields**
  ///  - `row` : Sheet's row. Header is also a row.
  ///  - `max_len` : Max length of our table. It is necessary for correct displaying of the table.
  #[ derive( Debug, Clone ) ]
  pub struct RowWrapper
  {
    pub row: Vec< JsonValue >,
    pub max_len: usize
  }

  impl TableWithFields for RowWrapper {}
  impl Fields< &'_ str, Option< Cow< '_, str > > >
  for RowWrapper
  {
    type Key< 'k > = &'k str;
    type Val< 'v > = Option< Cow< 'v, str > >;
    fn fields( &self ) -> impl IteratorTrait< Item= ( &'_ str, Option<Cow<'_, str > > ) >
    {
      let mut dst = Vec::new();

      for ( index, value ) in self.row.iter().enumerate()
      {
        let column_name = format!( "{} ", index );
        let title = Box::leak( column_name.into_boxed_str() ) as &str;
        let cleaned: String = value
        .to_string()
        .chars()
        .skip( 1 )
        .take( value.to_string().chars().count() - 2 )
        .collect();

        dst.push( ( title, Some( Cow::Owned( cleaned ) ) ) )
      }

      // adding empty values for missing cells
      for index in self.row.len()..self.max_len
      {
        let column_name = format!( "{}", index );
        let title = Box::leak( column_name.into_boxed_str() ) as &str;
        dst.push( ( title, Some( Cow::Owned( "".to_string() ) ) ) );
      }
      dst.into_iter()
    }
  }

}

crate::mod_interface!
{
  own use
  {
    RowWrapper
  };
}