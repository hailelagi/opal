defmodule Opal.Key do
  @moduledoc """
    see: https://docs.sequoia-pgp.org/sequoia_openpgp/packet/enum.Key.html#key-variants
  """
  @type t :: %__MODULE__{
          public_key: String.t()
          # todo: serialise other key fields
        }

  @fields ~w[public_key]a
  @enforce_keys @fields

  defstruct @fields
end
