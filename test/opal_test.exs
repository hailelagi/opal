defmodule OpalTest do
  use ExUnit.Case
  doctest Opal

  alias Opal.Key

  setup do
    msg = ~S"""
    {"hello":"world"}
    """

    [msg: msg]
  end

  test "it gets a key" do
    assert {:ok, %Key{public_key: key}} = Opal.get_pci_public_key()
    assert is_binary(key)
  end

  test "it creates a PGP message", %{msg: msg} do

    assert {:ok, _} = Opal.create_message(msg)
  end

  test "it encrypts arbitrary data with a key", %{msg: msg} do
    {:ok, %Key{public_key: key}} = Opal.get_pci_public_key()
    {:ok, msg} = Opal.create_message(msg)

    assert {:ok, data} = Opal.encrypt(key, msg)
    IO.inspect(data)
  end
end
