mod private
{
}

pub mod gspread_hub;

crate::mod_interface!
{
  exposed use
  {
    gspread_hub::
    {
      GspreadHub,
    }
  };
}