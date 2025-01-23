use std::error::Error;
use clap::Parser;
use dotenv::dotenv;

use gspread::*;
use gcore::ApplicationSecret;
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

  let secret = ApplicationSecret::read();

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
