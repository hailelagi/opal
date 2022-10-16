defmodule OpalTest do
  use ExUnit.Case
  doctest Opal

  alias Opal.Key

  test "it gets a key" do
    assert {:ok, %Key{public_key: key}} = Opal.get_pci_public_key()
    assert is_binary(key)
  end
end
