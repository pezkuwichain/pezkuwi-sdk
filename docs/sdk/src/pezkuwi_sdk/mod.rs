//! # Pezkuwi SDK
//!
//! [Pezkuwi SDK](https://github.com/paritytech/polkadot-sdk) provides the main resources needed to
//! start building on the [Pezkuwi network](https://pezkuwi.network), a scalable, multi-chain
//! blockchain platform that enables different blockchains to securely interoperate.
//!
//! [![StackExchange](https://img.shields.io/badge/StackExchange-Pezkuwi%20and%20Substrate-222222?logo=stackexchange)](https://substrate.stackexchange.com/)
//!
//! [![awesomeHez](https://img.shields.io/badge/pezkuwi-awesome-e6007a?logo=pezkuwi)](https://github.com/Awsmdot/awesome-dot)
//! [![wiki](https://img.shields.io/badge/pezkuwi-wiki-e6007a?logo=pezkuwi)](https://wiki.pezkuwi.network/)
//! [![forum](https://img.shields.io/badge/pezkuwi-forum-e6007a?logo=pezkuwi)](https://forum.pezkuwi.network/)
//!
//! [![RFCs](https://img.shields.io/badge/fellowship-RFCs-e6007a?logo=pezkuwi)](https://github.com/pezkuwi-fellows/rfcs)
//! [![Runtime](https://img.shields.io/badge/fellowship-runtimes-e6007a?logo=pezkuwi)](https://github.com/pezkuwi-fellows/runtimes)
//! [![Manifesto](https://img.shields.io/badge/fellowship-manifesto-e6007a?logo=pezkuwi)](https://github.com/pezkuwi-fellows/manifesto/blob/main/manifesto.pdf)
//!
//! ## Getting Started
//!
//! The primary way to get started with the Pezkuwi SDK is to start writing a FRAME-based runtime.
//! See:
//!
//! * [`pezkuwi`], to understand what is Pezkuwi as a development platform.
//! * [`substrate`], for an overview of what Substrate as the main blockchain framework of Pezkuwi
//!   SDK.
//! * [`frame`], to learn about how to write blockchain applications aka. "App Chains".
//! * Continue with the [`pezkuwi_sdk_docs`'s "getting started"](crate#getting-started).
//!
//! ## Components
//!
//! #### Substrate
//!
//! [![Substrate-license](https://img.shields.io/badge/License-GPL3%2FApache2.0-blue)](https://github.com/paritytech/polkadot-sdk/blob/master/substrate/LICENSE-APACHE2)
//! [![GitHub
//! Repo](https://img.shields.io/badge/github-substrate-2324CC85)](https://github.com/paritytech/polkadot-sdk/blob/master/substrate)
//!
//! [`substrate`] is the base blockchain framework used to power the Pezkuwi SDK. It is a full
//! toolkit to create sovereign blockchains, including but not limited to those which connect to
//! Pezkuwi as parachains.
//!
//! #### FRAME
//!
//! [![Substrate-license](https://img.shields.io/badge/License-Apache2.0-blue)](https://github.com/paritytech/polkadot-sdk/blob/master/substrate/LICENSE-APACHE2)
//! [![GitHub
//! Repo](https://img.shields.io/badge/github-frame-2324CC85)](https://github.com/paritytech/polkadot-sdk/blob/master/substrate/frame)
//!
//! [`frame`] is the framework used to create Substrate-based application logic, aka. runtimes.
//! Learn more about the distinction of a runtime and node in
//! [`reference_docs::wasm_meta_protocol`].
//!
//! #### Cumulus
//!
//! [![Cumulus-license](https://img.shields.io/badge/License-GPL3-blue)](https://github.com/paritytech/polkadot-sdk/blob/master/cumulus/LICENSE)
//! [![GitHub
//! Repo](https://img.shields.io/badge/github-cumulus-white)](https://github.com/paritytech/polkadot-sdk/blob/master/cumulus)
//!
//! [`cumulus`] transforms FRAME-based runtimes into Pezkuwi-compatible parachain runtimes, and
//! Substrate-based nodes into Pezkuwi/Parachain-compatible nodes.
//!
//! #### XCM
//!
//! [![XCM-license](https://img.shields.io/badge/License-GPL3-blue)](https://github.com/paritytech/polkadot-sdk/blob/master/pezkuwi/LICENSE)
//! [![GitHub
//! Repo](https://img.shields.io/badge/github-XCM-e6007a?logo=pezkuwi)](https://github.com/paritytech/polkadot-sdk/blob/master/pezkuwi/xcm)
//!
//! [`xcm`], short for "cross consensus message", is the primary format that is used for
//! communication between parachains, but is intended to be extensible to other use cases as well.
//!
//! #### Pezkuwi
//!
//! [![Pezkuwi-license](https://img.shields.io/badge/License-GPL3-blue)](https://github.com/paritytech/polkadot-sdk/blob/master/pezkuwi/LICENSE)
//! [![GitHub
//! Repo](https://img.shields.io/badge/github-pezkuwi-e6007a?logo=pezkuwi)](https://github.com/paritytech/polkadot-sdk/blob/master/pezkuwi)
//!
//! [`pezkuwi`] is an implementation of a Pezkuwi node in Rust, by `@paritytech`. The Pezkuwi
//! runtimes are located under the
//! [`pezkuwi-fellows/runtimes`](https://github.com/pezkuwi-fellows/runtimes) repository.
//!
//! ### Binaries
//!
//! The main binaries that are part of the Pezkuwi SDK are:

//! * [`pezkuwi`]: The Pezkuwi relay chain node binary, as noted above.
//! * [`pezkuwi-omni-node`]: A white-labeled parachain collator node. See more in
//!   [`crate::reference_docs::omni_node`].
//! * [`pezkuwi-parachain-bin`]: The collator node used to run collators for all Pezkuwi system
//!   parachains.
//! * [`frame-omni-bencher`]: a benchmarking tool for FRAME-based runtimes. Nodes typically contain
//!   a
//!  `benchmark` subcommand that does the same.
//! * [`chain_spec_builder`]: Utility to build chain-specs Nodes  typically contain a `build-spec`
//!   subcommand that does the same.
//! * [`subkey`]: Substrate's key management utility.
//! * [`substrate-node`](node_cli) is an extensive substrate node that contains the superset of all
//!   runtime and node side features. The corresponding runtime, called [`kitchensink_runtime`]
//!   contains all of the modules that are provided with `FRAME`. This node and runtime is only used
//!   for testing and demonstration.
//!
//! ### Summary
//!
//! The following diagram summarizes how some of the components of Pezkuwi SDK work together:
#![doc = simple_mermaid::mermaid!("../../../mermaid/pezkuwi_sdk_substrate.mmd")]
//!
//! A Substrate-based chain is a blockchain composed of a runtime and a node. As noted above, the
//! runtime is the application logic of the blockchain, and the node is everything else.
//! See [`reference_docs::wasm_meta_protocol`] for an in-depth explanation of this. The
//! former is built with [`frame`], and the latter is built with rest of Substrate.
//!
//! > You can think of a Substrate-based chain as a white-labeled blockchain.
#![doc = simple_mermaid::mermaid!("../../../mermaid/pezkuwi_sdk_pezkuwi.mmd")]
//! Pezkuwi is itself a Substrate-based chain, composed of the exact same two components. It has
//! specialized logic in both the node and the runtime side, but it is not "special" in any way.
//!
//! A parachain is a "special" Substrate-based chain, whereby both the node and the runtime
//! components have became "Pezkuwi-aware" using Cumulus.
#![doc = simple_mermaid::mermaid!("../../../mermaid/pezkuwi_sdk_parachain.mmd")]
//!
//! ## Notable Upstream Crates
//!
//! - [`parity-scale-codec`](https://github.com/paritytech/parity-scale-codec)
//! - [`parity-db`](https://github.com/paritytech/parity-db)
//! - [`trie`](https://github.com/paritytech/trie)
//! - [`parity-common`](https://github.com/paritytech/parity-common)
//!
//! ## Trophy Section: Notable Downstream Projects
//!
//! A list of projects and tools in the blockchain ecosystem that one way or another use parts of
//! the Pezkuwi SDK:
//!
//! * [Avail](https://github.com/availproject/avail)
//! * [Cardano Partner Chains](https://iohk.io/en/blog/posts/2023/11/03/partner-chains-are-coming-to-cardano/)
//! * [Starknet's Madara Sequencer](https://github.com/keep-starknet-strange/madara)
//! * [Polymesh](https://polymesh.network/)
//!
//! [`substrate`]: crate::pezkuwi_sdk::substrate
//! [`frame`]: crate::pezkuwi_sdk::frame_runtime
//! [`cumulus`]: crate::pezkuwi_sdk::cumulus
//! [`pezkuwi`]: crate::pezkuwi_sdk::pezkuwi
//! [`xcm`]: crate::pezkuwi_sdk::xcm
//! [`frame-omni-bencher`]: https://crates.io/crates/frame-omni-bencher
//! [`pezkuwi-parachain-bin`]: https://crates.io/crates/pezkuwi-parachain-bin
//! [`pezkuwi-omni-node`]: https://crates.io/crates/pezkuwi-omni-node

/// Learn about Cumulus, the framework that transforms [`substrate`]-based chains into
/// [`pezkuwi`]-enabled parachains.
pub mod cumulus;
/// Learn about FRAME, the framework used to build Substrate runtimes.
pub mod frame_runtime;
/// Learn about Pezkuwi as a platform.
pub mod pezkuwi;
/// Learn about different ways through which smart contracts can be utilized on top of Substrate,
/// and in the Pezkuwi ecosystem.
pub mod smart_contracts;
/// Learn about Substrate, the main blockchain framework used in the Pezkuwi ecosystem.
pub mod substrate;
/// Index of all the templates that can act as first scaffold for a new project.
pub mod templates;
/// Learn about XCM, the de-facto communication language between different consensus systems.
pub mod xcm;
