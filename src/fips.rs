//! FIPS 140-3 support: load OpenSSL FIPS provider at startup when FIPS_MODE=1.
//!
//! When the `fips` Cargo feature is enabled and `FIPS_MODE=1` is set in the environment,
//! the OpenSSL FIPS provider is loaded before any other crypto (e.g. rdkafka TLS).
//! Kafka TLS and any future gRPC TLS will then use the FIPS provider.

use std::env;

const FIPS_MODE_ENV: &str = "FIPS_MODE";

/// Loads the OpenSSL FIPS provider at startup when FIPS_MODE=1. Must be called
/// before any TLS or other crypto. When the `fips` feature is disabled, this is a no-op.
///
/// Returns Ok(()) if FIPS is not requested, or if the provider was loaded successfully.
/// Returns Err when FIPS_MODE=1 and the provider failed to load (e.g. OpenSSL not built
/// with FIPS, or OPENSSL_MODULES not set).
#[cfg(feature = "fips")]
pub fn ensure_fips_provider() -> Result<(), String> {
    if env::var(FIPS_MODE_ENV).as_deref() != Ok("1") {
        return Ok(());
    }

    let provider = openssl::provider::Provider::load(None, "fips").map_err(|e| {
        let msg = e
            .errors()
            .iter()
            .filter_map(|e| e.reason().map(String::from))
            .collect::<Vec<_>>()
            .join("; ");
        format!("Failed to load OpenSSL FIPS provider: {msg}")
    })?;

    // Keep the provider loaded for the process lifetime.
    std::mem::forget(provider);
    tracing::info!("FIPS 140-3: OpenSSL FIPS provider loaded successfully");
    Ok(())
}

#[cfg(not(feature = "fips"))]
pub fn ensure_fips_provider() -> Result<(), String> {
    Ok(())
}
