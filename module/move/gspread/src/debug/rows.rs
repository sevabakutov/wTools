


mod private
{
  use super::*;
  use crate::*;

  use std::borrow::Cow;
  use std::collections::HashMap;
  use actions::gspread::Result;
  use format_tools::TableWithFields;
  
  /// Struct to keep row and its id.
  #[ derive( Debug, Clone ) ]
  pub struct RowWrapper
  {
    /// Row's id
    pub id : usize,
    /// Row wich is represented as an array of typle, where tuple is a pair of key value, where key is a column's name and value is a value of a cell.
    pub row : Vec< ( String, String ) >,
  }

  impl RowWrapper
  {
    /// Just constructor
    pub fn new( id : usize, row : Vec< ( String, String ) > ) -> Self
    {
      Self
      {
        id,
        row,
      }
    }
  }

  impl TableWithFields for RowWrapper {}
  impl Fields< &'_ str, Option< Cow< '_, str > > >
  for RowWrapper
  {
    type Key< 'k > = &'k str;
    type Val< 'v > = Option< Cow< 'v, str > >;

    fn fields( &self ) -> impl IteratorTrait< Item= ( &'_ str, Option<Cow<'_, str > > ) >
    {
      self.row.iter().map( | ( key, value ) | {
        ( key.as_str(), Some( Cow::Borrowed( value.as_str() ) ) )
      } )
      // let mut dst = Vec::new();

      // for ( index, value ) in self.row.iter().enumerate()
      // {
      //   let column_name = format!( "{} ", index );
      //   let title = Box::leak( column_name.into_boxed_str() ) as &str;
      //   let cleaned: String = value
      //   .to_string()
      //   .chars()
      //   .skip( 1 )
      //   .take( value.to_string().chars().count() - 2 )
      //   .collect();

      //   dst.push( ( title, Some( Cow::Owned( cleaned ) ) ) )
      // }

      // //adding empty values for missing cells
      // for index in self.row.len()..self.max_len
      // {
      //   let column_name = format!( "{}", index );
      //   let title = Box::leak( column_name.into_boxed_str() ) as &str;
      //   dst.push( ( title, Some( Cow::Owned( "".to_string() ) ) ) );
      // }

      // dst.into_iter()
    }
  }

  /// Function to wrap a row.
  /// 
  /// It converts from HashMap to a row wich is actually sorted array, by column name.
  /// 
  /// **Params**
  ///  - `id` : Row's id.
  ///  - `vales` : Pairs of key value, where key is a clomun's name and value is a value of cell.
  /// 
  /// **Returns**
  ///  - `RowWrapper` object.
  pub fn wrap_row
  (
    id : usize,
    values : HashMap< String, String >
  ) -> Result< RowWrapper >
  {
    let mut row: Vec< ( String, String ) > = values.into_iter().collect();
    
    row.sort_by( | a, b | a.0.cmp( &b.0 ) );

    Ok ( RowWrapper::new( id, row ) )
  }
}

crate::mod_interface!
{
  own use
  {
    wrap_row,
    RowWrapper
  };
}