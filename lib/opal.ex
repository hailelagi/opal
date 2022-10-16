defmodule Opal do
  @moduledoc """
  Documentation for `Opal`.
  """
  use Rustler, otp_app: :opal, crate: "opal"

  alias Opal.Key

  @spec get_pci_public_key() :: {:ok, Key.t()} | {:error, any()}
  def get_pci_public_key, do: error()

  @spec create_message(binary()) :: {:ok, map()} | {:error, any()}
  def create_message(_), do: error()

  @spec encrypt(binary(), map()) :: {:ok, binary()} | {:error, any()}
  def encrypt(_, _), do: error()

  defp error, do: :erlang.nif_error("not implemented")
end
