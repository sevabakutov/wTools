
mod private
{
  pub const GOOGLE_API_URL: &'static str = "https://sheets.googleapis.com/v4/spreadsheets";
}

crate::mod_interface!
{
  prelude use
  {
    GOOGLE_API_URL,
  };
}