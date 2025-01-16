
mod private
{
  use crate::*;
  
  pub struct GspreadHub {
    pub hub: i32
  }
}

crate::mod_interface!
{
  own use
  {
    GspreadHub,
  };
}
