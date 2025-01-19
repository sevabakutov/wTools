use std::error::Error;
use clap::Parser;
use dotenv::dotenv;

use gspread::*;
use gcore::Secret;
use gcore::client::Client;
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
  .token( &secret )
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
