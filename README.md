<div align="center">

![SDK Logo](./docs/images/Pezkuwi_Logo_Horizontal_Pink_White.png#gh-dark-mode-only)
![SDK Logo](./docs/images/Pezkuwi_Logo_Horizontal_Pink_Black.png#gh-light-mode-only)

# Pezkuwi SDK

![GitHub stars](https://img.shields.io/github/stars/paritytech/pezkuwi-sdk)&nbsp;&nbsp;![GitHub
forks](https://img.shields.io/github/forks/paritytech/pezkuwi-sdk)

<!-- markdownlint-disable-next-line MD013 -->
[![StackExchange](https://img.shields.io/badge/StackExchange-Community%20&%20Support-222222?logo=stackexchange)](https://substrate.stackexchange.com/)&nbsp;&nbsp;![GitHub contributors](https://img.shields.io/github/contributors/paritytech/pezkuwi-sdk)&nbsp;&nbsp;![GitHub commit activity](https://img.shields.io/github/commit-activity/m/paritytech/pezkuwi-sdk)&nbsp;&nbsp;![GitHub last commit](https://img.shields.io/github/last-commit/paritytech/pezkuwi-sdk)

> The Pezkuwi SDK repository provides all the components needed to start building on the
> [Pezkuwi](https://pezkuwi.com/) network, a multi-chain blockchain platform that enables
> different blockchains to interoperate and share information in a secure and scalable way.

</div>

## ⚡ Quickstart
If you want to get an example node running quickly you can execute the following getting started script:

```
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/paritytech/pezkuwi-sdk/master/scripts/getting-started.sh | bash
```

## 📚 Documentation

* [Pezkuwi Documentation Portal](https://docs.pezkuwi.com)
* [🦀 rust-docs](https://paritytech.github.io/pezkuwi-sdk/master/pezkuwi_sdk_docs/index.html): Where we keep track of
the API docs of our Rust crates. Includes:
  * [Introduction](https://paritytech.github.io/pezkuwi-sdk/master/pezkuwi_sdk_docs/pezkuwi_sdk/index.html)
	to each component of the Pezkuwi SDK: Substrate, FRAME, Cumulus, and XCM
  * [Guides](https://paritytech.github.io/pezkuwi-sdk/master/pezkuwi_sdk_docs/guides/index.html),
	namely how to build your first FRAME pallet
  * [Templates](https://paritytech.github.io/pezkuwi-sdk/master/pezkuwi_sdk_docs/pezkuwi_sdk/templates/index.html)
    for starting a new project.
  * [External Resources](https://paritytech.github.io/pezkuwi-sdk/master/pezkuwi_sdk_docs/external_resources/index.html)

## 🚀 Releases

<!-- markdownlint-disable-next-line MD013 -->
![Current Stable Release](https://raw.githubusercontent.com/paritytech/release-registry/main/badges/pezkuwi-sdk-latest.svg)&nbsp;&nbsp;![Next Stable Release](https://raw.githubusercontent.com/paritytech/release-registry/main/badges/pezkuwi-sdk-next.svg)

The Pezkuwi SDK is released every three months as a `Pezkuwi stableYYMM` release. Each stable release is supported for
one year with patches. See the next upcoming versions in the [Release
Registry](https://github.com/paritytech/release-registry/) and more docs in [RELEASE.md](./docs/RELEASE.md).

You can use [`psvm`](https://github.com/paritytech/psvm) to update all dependencies to a specific
version without needing to manually select the correct version for each crate.

## 🛠️ Tooling

[Pezkuwi SDK Version Manager](https://github.com/paritytech/psvm):
A simple tool to manage and update the Pezkuwi SDK dependencies in any Cargo.toml file.
It will automatically update the Pezkuwi SDK dependencies to their correct crates.io version.

## 🔐 Security

The security policy and procedures can be found in
[docs/contributor/SECURITY.md](./docs/contributor/SECURITY.md).

## 🤍 Contributing & Code of Conduct

Ensure you follow our [contribution guidelines](./docs/contributor/CONTRIBUTING.md). In every
interaction and contribution, this project adheres to the [Contributor Covenant Code of
Conduct](./docs/contributor/CODE_OF_CONDUCT.md).

### 👾 Ready to Contribute?

Take a look at the issues labeled with [`mentor`](https://github.com/paritytech/polkadot-sdk/labels/C1-mentor)
(or alternatively [this](https://mentor.tasty.limo/) page, created by one of the maintainers) label to get started!
We always recognize valuable contributions by proposing an on-chain tip to the Pezkuwi network as a token of our
appreciation.

## Pezkuwi Fellowship

Development in this repo usually goes hand in hand with the `fellowship` organization. In short,
this repository provides all the SDK pieces needed to build both Pezkuwi and its parachains. But,
the actual Pezkuwi runtime lives in the `fellowship/runtimes` repository. Read more about the
fellowship, this separation, the RFC process
[here](https://pezkuwi-fellows.github.io/dashboard/).

## History

This repository is the amalgamation of 3 separate repositories that used to make up Pezkuwi SDK,
namely Substrate, Pezkuwi and Cumulus. Read more about the merge and its history
[here](https://pezkuwi-public.notion.site/Pezkuwi-SDK-FAQ-fbc4cecc2c46443fb37b9eeec2f0d85f).
