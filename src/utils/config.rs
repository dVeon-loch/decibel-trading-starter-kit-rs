//! Configuration values for Decibel trading.
//!
//! All values loaded from .env file with fallbacks to documented defaults.
//!
//! TODO: Documentation references

use std::ops::Not;

use aptos_rust_sdk_v2::AccountAddress;
use eyre::{Context, bail};
use reqwest::{Body, Method, Response, Url, header::HeaderMap};
use secrecy::SecretString;

/// Blockchain Configuration
const DEFAULT_PACKAGE_ADDRESS: &str =
    "0xb8a5788314451ce4d2fbbad32e1bad88d4184b73943b7fe5166eab93cf1a5a95";
const DEFAULT_FULLNODE_URL: &str = "https://api.netna.staging.aptoslabs.com/v1";

/// API Configuration
const DEFAULT_REST_API_BASE_URL: &str = "https://api.netna.aptoslabs.com/decibel";
const DEFAULT_WEBSOCKET_URL: &str = "wss://api.netna.aptoslabs.com/decibel/ws";

#[derive(Debug)]
pub struct Config {
    /// Will be defaulted to [DEFAULT_PACKAGE_ADDRESS] if nothing is defined
    pub package_address: AccountAddress,
    /// Will be defaulted to [DEFAULT_FULLNODE_URL] if nothing is defined
    pub fullnode_url: Url,
    /// Must be user-defined
    pub api_wallet_address: AccountAddress,
    /// Must be user-defined
    pub api_wallet_private_key: SecretString,
    /// Will be defaulted to [DEFAULT_REST_API_BASE_URL] if nothing is defined
    pub rest_api_base_url: String,
    /// Will be defaulted to [DEFAULT_WEBSOCKET_URL] if nothing is defined
    pub websocket_url: String,
    /// Must be user-defined
    pub api_bearer_token: SecretString,
    /// Will be defaulted to [None] if nothing is defined or if the defined value fails [AccountAddress::from_hex]
    pub subaccount_address: Option<AccountAddress>,
    /// Will be defaulted to [None] if nothing is defined or if the defined value fails [AccountAddress::from_hex]
    pub market_address: Option<AccountAddress>,
    /// Will be defaulted to [None] if nothing is defined
    pub market_name: Option<String>,
}

impl Config {
    /// Constructs the configuration from the available environment variables.
    /// Panics if:
    /// - Any of the following environment variables: [API_WALLET_ADDRESS, API_WALLET_PRIVATE_KEY, API_BEARER_TOKEN] are not defined or are malformed
    /// - A provided optional variable is malformed
    /// - A fallback default for any of the optional [AccountAddress] variables fails [AccountAddress::from_hex], this should be reported to a code maintainer
    pub fn from_env() -> eyre::Result<Self> {
        Ok(Self {
            package_address: Self::parse_address_or(
                Self::parse_env_var("PACKAGE_ADDRESS"),
                DEFAULT_PACKAGE_ADDRESS,
            ),
            fullnode_url: Self::parse_url_or_default(
                Self::parse_env_var("FULLNODE_URL"),
                DEFAULT_FULLNODE_URL,
            ),
            api_wallet_address: AccountAddress::from_hex(Self::parse_env_var(
                "API_WALLET_ADDRESS",
            )?)?,
            api_wallet_private_key: SecretString::from(Self::parse_env_var(
                "API_WALLET_PRIVATE_KEY",
            )?),
            rest_api_base_url: Self::parse_env_var("REST_API_BASE_URL")
                .unwrap_or(DEFAULT_REST_API_BASE_URL.to_string()),
            websocket_url: Self::parse_env_var("WEBSOCKET_URL")
                .unwrap_or(DEFAULT_WEBSOCKET_URL.to_string()),
            api_bearer_token: SecretString::from(Self::parse_env_var("API_BEARER_TOKEN")?),
            subaccount_address: Self::parse_env_var("SUBACCOUNT_ADDRESS")
                .map_or(None, |a| AccountAddress::from_hex(a).ok()),
            market_address: Self::parse_env_var("MARKET_ADDRESS")
                .map_or(None, |a| AccountAddress::from_hex(a).ok()),
            market_name: Self::parse_env_var("MARKET_NAME").ok(),
        })
    }

    fn parse_env_var(env_var: &str) -> eyre::Result<String> {
        match dotenvy::var(env_var)
            .wrap_err(format!("{env_var} environment variable not defined"))?
            .trim()
        {
            s if s.is_empty().not() => Ok(s.to_string()),
            _ => bail!("{} environment variable is defined but is empty", env_var),
        }
    }

    fn parse_address_or(parse_result: eyre::Result<String>, default: &str) -> AccountAddress {
        parse_result.map_or_else(
            |_| {
                AccountAddress::from_hex(default)
                    .expect("Internal error: Error parsing default fallback address")
            },
            |a| {
                AccountAddress::from_hex(&a).expect(&format!(
                    "User error: Error parsing user-provided address: {a}"
                ))
            },
        )
    }

    fn parse_url_or_default(parse_result: eyre::Result<String>, default: &str) -> Url {
        parse_result.map_or_else(
            |_| Url::parse(default).expect("Internal error: Error parsing default fallback url"),
            |a| {
                Url::parse(&a).expect(&format!(
                    "User error: Error parsing user-provided address: {a}"
                ))
            },
        )
    }
}

/// Validates that all required configuration values are present.
///
/// # Errors
/// Returns an error if critical values are missing.
pub fn validate_config() -> eyre::Result<()> {
    todo!()
}

/// Prints current configuration (with secrets masked).
pub fn print_config() {
    todo!()
}

pub struct RequestInit {
    pub method: Method,
    pub headers: Option<HeaderMap>,
    pub body: Option<Body>,
}

/// Makes an authenticated fetch request to the Decibel API.
///
/// Automatically includes the Bearer token if [`API_BEARER_TOKEN`] is configured.
pub async fn authenticated_fetch(url: &str, options: RequestInit) -> eyre::Result<Response> {
    todo!()
}
