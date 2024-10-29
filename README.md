# Impact Protocols

This repository contains to SINE's implementations of impact-related protocols and data models.

As of right now, all the protocols and data models are implemented in Rust and they are all concerned with the exchange of emissions data. Specifically, the repository includes implementations of [the PACT Data Model](https://wbcsd.github.io/data-exchange-protocol/v2/),[the iLEAP Data Model](https://sine-fdn.github.io/ileap-extension/), and a demo API implementing the [PACT Data Exchange Protocol](https://wbcsd.github.io/data-exchange-protocol/v2/) with [iLEAP support](https://sine-fdn.github.io/ileap-extension/).

## Contents

This repository includes three crates:

- [`pact-data-model`](./pact-data-model) - a Rust implementation of the PACT data model
- [`ileap-data-model`](./ileap-data-model) - a Rust implementation of the iLEAP data model
- [`demo-api`](./demo-api) - a demo implementation of the PACT data exchange protocol enriched with an iLEAP-specific endpoint and iLEAP demo data.

> [!WARNING]
> The `demo-api` is currently WIP. Significant refactoring will take place in the near future.

## Contribute

We welcome contributions to this repository in the form of issues and pull requests.

## Licenses

Each crate in the workspace has its own license. Please refer to the `LICENSE` and `cargo.toml` files in each crate for more information.
