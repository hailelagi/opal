# Opal

An openPGP encryption and decryption library via rust NIF bindings. Backend currently uses [sequoia](https://crates.io/crates/sequoia-openpgp). 

This project started as a part of [spawnfest 2022](https://github.com/spawnfest/opal) as a way for me to learn more about cryptography and rust. All code here is provided "as is", terribe, awful, bad code and you [probably shouldn't be using PGP anyway](https://latacora.micro.blog/2019/07/16/the-pgp-problem.html)


## Installation

add`opal` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:opal, "~> 0.1.0"}
  ]
end
```
