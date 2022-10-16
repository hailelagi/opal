defmodule Opal do
  @moduledoc """
  Documentation for `Opal`.
  """
  use Rustler, otp_app: :opal, crate: "opal"

  def get_pci_public_key, do: error()

  defp error, do: :erlang.nif_error("not implemented")
end
