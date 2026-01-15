/// Funds an account using the private Netna faucet.
///
/// Faucet URL discovered from Java example:
/// <https://faucet-dev-netna-us-central1-410192433417.us-central1.run.app>
///
/// ### Arguments
/// - `address` - The address to fund (with or without 0x prefix)
/// - `amount` - Amount in Octas (default: 10,000,000,000 = 100 APT)
pub async fn fund_from_netna_faucet(address: &str, amount: Option<u64>) -> eyre::Result<()> {
    todo!()
}

/// Alias for backwards compatibility.
pub use fund_from_netna_faucet as fund_account;
