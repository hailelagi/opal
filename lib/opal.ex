defmodule Opal do
  @moduledoc """
  Documentation for `Opal`.
  """
  use Rustler, otp_app: :opal, crate: "opal"

  alias Opal.Key

  @spec get_pci_public_key() :: {:ok, Key.t()} | {:error, any()}
  def get_pci_public_key, do: error()

  @spec encrypt(map()) :: {:ok, binary()} | {:error, any()}
  def encrypt(_), do: error()

  defp error, do: :erlang.nif_error("not implemented")
end
