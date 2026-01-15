use decibel_trading_starter_kit_rs::utils::config::Config;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let _config = Config::from_env()?;

    // TODO

    Ok(())
}
