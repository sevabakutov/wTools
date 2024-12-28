


mod private
{
  use super::*;
  use crate::*;

  use std::borrow::Cow;
  use actions::gspread::Result;
  use format_tools::TableWithFields;
  
  /// Struct to keep row and its id.
  // #[ derive( Debug, Clone ) ]
  // pub struct RowWrapper
  // {
  //   /// Row's id
  //   pub row_key : usize,
  //   /// Row wich is represented as an array of typle, where tuple is a pair of key value, where key is a column's name and value is a value of a cell.
  //   pub row : Vec< ( String, String ) >,
  // }

  // impl RowWrapper
  // {
  //   /// Just constructor
  //   pub fn new( row_key : usize, row : Vec< ( String, String ) > ) -> Self
  //   {
  //     Self
  //     {
  //       row_key,
  //       row,
  //     }
  //   }
  // }

  // impl TableWithFields for RowWrapper {}
  // impl Fields< &'_ str, Option< Cow< '_, str > > >
  // for RowWrapper
  // {
  //   type Key< 'k > = &'k str;
  //   type Val< 'v > = Option< Cow< 'v, str > >;

  //   fn fields( &self ) -> impl IteratorTrait< Item= ( &'_ str, Option<Cow<'_, str > > ) >
  //   {
  //     self.row.iter().map( | ( key, value ) | {
  //       ( key.as_str(), Some( Cow::Borrowed( value.as_str() ) ) )
  //     } )
  //   }
  // }

  
}


// crate::mod_interface!
// {
//   own use
//   {

//   };
// }