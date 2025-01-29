//!
//! CLI actions of the tool.
//!

mod private {}

crate::mod_interface!
{
  layer utils;
  layer gspread;
  layer gspread_header_get;
  layer gspread_rows_get;
  layer gspread_cell_get;
  layer gspread_cell_set;
  layer gspread_row_get;
  layer gspread_row_get_custom;
  layer gspread_row_update;
  layer gspread_row_append;
  layer gspread_row_update_custom;
}

