# Opal

An openPGP encryption and decryption library via rust NIF bindings. Backend currently uses [sequoia](https://crates.io/crates/sequoia-openpgp). 

This project started as a part of [spawnfest 2022](https://github.com/spawnfest/opal)

## Installation

add`opal` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:opal, "~> 0.1.0"}
  ]
end
```
