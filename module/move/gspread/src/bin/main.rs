use std::error::Error;
use clap::Parser;
use dotenv::dotenv;

use gspread::*;
use secret::Secret;
use client::client::Client;
use commands::
{
  self,
  Cli,
  CliCommand
};


#[ tokio::main ]
async fn main() -> Result< (), Box< dyn Error > >
{
  dotenv().ok();

  let secret = Secret::read();

  let client = Client::former()
  .auth( &secret )
  .await?
  .form();

  let cli = Cli::parse();

  match cli.command
  {
    CliCommand::GSpread( cmd ) =>
    {
      commands::gspread::command( &client, cmd ).await;
    }
  }

  Ok( () )
}
