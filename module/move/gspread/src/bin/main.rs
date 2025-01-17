use std::error::Error;
use clap::Parser;
use dotenv::dotenv;

use gspread::
{
  commands::{ self, Cli, CliCommand },
  secret::Secret, GspreadClient,
};

#[ tokio::main ]
async fn main() -> Result< (), Box< dyn Error > >
{
  dotenv().ok();

  let secret = Secret::read();

  let hub = GspreadClient::builder()
  .with_secret( &secret )
  .build()
  .await?;

  let cli = Cli::parse();

  match cli.command
  {
    CliCommand::GSpread( cmd ) =>
    {
      commands::gspread::command( &hub, cmd ).await;
    }
  }

  Ok( () )
}
