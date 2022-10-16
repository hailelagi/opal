# NIF for Elixir.Opal

Provides bindings to `sequoia-openpgp`.

## Installation

After runnning `cargo build` you may run into a dependency/build error
You may need the rust binding for the low-level cryptographic libary [nettle](https://gitlab.com/sequoia-pgp/nettle-sys)

## TODO

- impl `Decode` for `openpgp::Message`
- encrypt the PGP message
