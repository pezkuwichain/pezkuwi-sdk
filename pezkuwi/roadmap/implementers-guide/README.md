# The Pezkuwi Parachain Host Implementers' Guide

The implementers' guide is compiled from several source files with [`mdBook`](https://github.com/rust-lang/mdBook).

## Hosted build

This is available [here](https://paritytech.github.io/pezkuwi-sdk/book/).

## Local build

To view it locally, run the following (from the `pezkuwi/` directory):

Ensure graphviz is installed:

```sh
brew install graphviz # for macOS
sudo apt-get install graphviz # for Ubuntu/Debian
```

Then install and build the book:

```sh
cargo install mdbook mdbook-linkcheck mdbook-graphviz mdbook-mermaid mdbook-last-changed
mdbook serve roadmap/implementers-guide
```

and in a second terminal window run:

```sh
open http://localhost:3000
```

## Specification

See also the Pezkuwi specification [hosted](https://spec.pezkuwi.network/), and its [source](https://github.com/w3f/pezkuwi-spec).
