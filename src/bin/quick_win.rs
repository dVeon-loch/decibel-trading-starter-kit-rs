use ansi_term::Colour::Green;
use decibel_trading_starter_kit_rs::{logging, utils::config::Config};
use tracing::Level;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    logging::init_logging(Level::INFO)?;

    let config = Config::from_env()?;

    let success_message = Green
        .bold()
        .underline()
        .paint("Config successfully validated!")
        .to_string();

    println!("{success_message}");

    println!("{config}");

    Ok(())
}
