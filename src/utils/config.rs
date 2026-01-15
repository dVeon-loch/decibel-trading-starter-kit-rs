//! Configuration values for Decibel trading.
//!
//! All values loaded from .env file with fallbacks to documented defaults.
//!
//! TODO: Documentation references

use std::{ffi::os_str::Display, ops::Not};

use aptos_rust_sdk_v2::{AccountAddress, crypto::Ed25519PrivateKey};
use eyre::{Context, ContextCompat, bail};
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
    pub api_wallet_private_key: Ed25519PrivateKey,
    /// Will be defaulted to [DEFAULT_REST_API_BASE_URL] if nothing is defined
    pub rest_api_base_url: Url,
    /// Will be defaulted to [DEFAULT_WEBSOCKET_URL] if nothing is defined
    pub websocket_url: Url,
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
        let mut errors = vec![];

        let package_address = Self::parse_address_or(
            Self::parse_env_var("PACKAGE_ADDRESS"),
            DEFAULT_PACKAGE_ADDRESS,
        );

        let fullnode_url =
            Self::parse_url_or(Self::parse_env_var("FULLNODE_URL"), DEFAULT_FULLNODE_URL);

        let api_wallet_address = Self::parse_env_var("API_WALLET_ADDRESS")
            .inspect_err(|err| {
                errors.push(format!(
                    "API_WALLET_ADDRESS is required in .env file. Error: {err}"
                ))
            })
            .map(|a| {
                Some(AccountAddress::from_hex(&a).inspect_err(|err| {
                    errors.push(format!(
                        "API_WALLET_ADDRESS could not be parsed as AccountAddress. Error: {err}"
                    ))
                }))
            })
            .unwrap_or_default();

        let api_wallet_private_key = Self::parse_env_var("API_WALLET_PRIVATE_KEY")
            .inspect_err(|err| {
                errors.push(format!(
                    "API_WALLET_PRIVATE_KEY is required in .env file. Error: {err}"
                ))
            })
            .map(|key_str| {
                // Decibel seems to add a prefix to their private keys
                Some(
                    Ed25519PrivateKey::from_hex(&key_str.replace("ed25519-priv-", "")).inspect_err(
                        |err| {
                            errors.push(format!(
                    "API_WALLET_PRIVATE_KEY could not be parsed as Ed25519PrivateKey. Error: {err}"
                ))
                        },
                    ),
                )
            })
            .unwrap_or_default();

        let api_bearer_token = Self::parse_env_var("API_BEARER_TOKEN")
            .inspect_err(|err| {
                errors.push(format!(
                    "API_BEARER_TOKEN is required in .env file. Error: {err}"
                ))
            })
            .map(|key_str| SecretString::from(key_str));

        let rest_api_base_url = Self::parse_url_or(
            Self::parse_env_var("REST_API_BASE_URL"),
            DEFAULT_REST_API_BASE_URL,
        );

        let websocket_url =
            Self::parse_url_or(Self::parse_env_var("WEBSOCKET_URL"), DEFAULT_WEBSOCKET_URL);

        let subaccount_address = Self::parse_env_var("SUBACCOUNT_ADDRESS")
            .map_or(None, |a| AccountAddress::from_hex(a).ok());

        let market_address = Self::parse_env_var("MARKET_ADDRESS")
            .map_or(None, |a| AccountAddress::from_hex(a).ok());

        let market_name = Self::parse_env_var("MARKET_NAME").ok();

        if !errors.is_empty() {
            tracing::error!("\nConfiguration errors found:\n");
            for error in &errors {
                tracing::error!(error);
            }
            tracing::error!(
                "\nPlease check your .env file and ensure all required values are set.\n"
            );

            bail!(
                "Failed to build config from environment due to errors: \n{:#?}\n",
                errors
            );
        }

        Ok(Self {
            package_address,
            fullnode_url,
            api_wallet_address: api_wallet_address
                .expect("This error state should have been validated")
                .expect("This error state should have been validated"),
            api_wallet_private_key: api_wallet_private_key
                .expect("This error state should have been validated")
                .expect("This error state should have been validated"),
            rest_api_base_url,
            websocket_url,
            api_bearer_token: api_bearer_token
                .expect("This error state should have been validated"),
            subaccount_address,
            market_address,
            market_name,
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

    fn parse_url_or(parse_result: eyre::Result<String>, default: &str) -> Url {
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

use std::fmt;

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"
            -----------------------------------------
            Config {{

                Package Address:        {}

                Fullnode URL:           {}
                
                API Wallet Address:     {}

                API Wallet Private Key: {:#?}

                Rest API Base URL:      {}

                Websocket URL:          {}

                API Bearer Token:       {:#?}

                Subaccount Address:     {:#?}

                Market Address:         {:#?}

                Market Name:            {:#?}
            }}
            -----------------------------------------
            "#,
            self.package_address,
            self.fullnode_url,
            self.api_wallet_address,
            self.api_wallet_private_key,
            self.rest_api_base_url,
            self.websocket_url,
            self.api_bearer_token,
            self.subaccount_address,
            self.market_address,
            self.market_name,
        )
    }
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
