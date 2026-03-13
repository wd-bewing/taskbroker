# FIPS 140-3 Compliance

This document describes how to build and run taskbroker in a FIPS 140-3 compliant configuration. When the `fips` feature is enabled and `FIPS_MODE=1` is set at runtime, the OpenSSL FIPS provider is loaded at startup; Kafka TLS (and any future gRPC TLS) then uses FIPS-approved cryptography.

## Prerequisites

- **OpenSSL 3.x** built with FIPS support, or a FIPS-capable distribution (e.g. RHEL 9 / UBI9 with FIPS-enabled OpenSSL).
- **Environment**: Set `OPENSSL_MODULES` to the directory containing the FIPS provider (e.g. `/usr/lib64/ossl-modules` on RHEL/UBI, or the path from your OpenSSL build).

## Building with FIPS support

Build with the `fips` feature:

```sh
cargo build --release --features fips
```

For FIPS builds, **rdkafka** (Kafka TLS) must link against OpenSSL 3.x built with FIPS. If OpenSSL is not in a default system path, set:

- `OPENSSL_DIR` – prefix of your FIPS OpenSSL install
- `OPENSSL_LIB_DIR` – e.g. `$OPENSSL_DIR/lib`
- `OPENSSL_INCLUDE_DIR` – e.g. `$OPENSSL_DIR/include`

Example:

```sh
export OPENSSL_DIR=/opt/openssl-fips
export OPENSSL_LIB_DIR=/opt/openssl-fips/lib
export OPENSSL_INCLUDE_DIR=/opt/openssl-fips/include
cargo build --release --features fips
```

## Running in FIPS mode

1. Set the environment so the FIPS provider can be loaded:
   - `OPENSSL_MODULES` – path to the directory containing `fips.so` (or your platform’s FIPS provider).

2. Enable FIPS in the process:
   - `FIPS_MODE=1`

3. Start the binary (built with `--features fips`). Example:

```sh
export OPENSSL_MODULES=/usr/lib64/ossl-modules
export FIPS_MODE=1
./target/release/taskbroker -c ./config/config.yaml
```

If the FIPS provider loads successfully, the log will include:

`FIPS 140-3: OpenSSL FIPS provider loaded successfully`

If `FIPS_MODE=1` is set but the provider fails to load (e.g. wrong OpenSSL build or missing `OPENSSL_MODULES`), the process exits with an error.

## What changes in FIPS mode

- **Startup**: When `FIPS_MODE=1`, the process loads the OpenSSL FIPS provider before creating Kafka clients or gRPC.
- **Kafka TLS**: rdkafka’s SSL feature uses the same OpenSSL; when linked against a FIPS-built OpenSSL and with the FIPS provider loaded, TLS uses FIPS-approved algorithms.
- **gRPC**: The current gRPC server does not use TLS. If TLS is added in the future, it must use the same FIPS OpenSSL stack.

## Notes

- **Validation**: This setup uses a FIPS-validated cryptographic module (OpenSSL’s FIPS provider). Taskbroker as an application is not itself FIPS 140-3 validated; formal product validation would require the CMVP process.
- **Default build**: The default build does **not** enable FIPS. Only with `--features fips` does the binary load the FIPS provider when `FIPS_MODE=1` is set.
