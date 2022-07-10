use std::{process, error::Error, future::pending};
use clap::Parser;
use resmand::{ResourceManager, parser::CliArgs};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = match CliArgs::try_parse() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error parsing: {e}");
            process::exit(1);
        }
    };
    let mut manager = ResourceManager::from_args(&args);
    manager.init();
    manager.run_server().await?;
    pending::<()>().await;
    Ok(())
}
