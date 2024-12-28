mod private
{
}

use format_tools::
{
  Fields,
  TableWithFields,
};
use std::borrow::Cow;

pub mod rows;

crate::mod_interface!
{
  exposed use
  {
    rows::
    {
      RowWrapper,
      wrap_row
    }
  };
}
