## Usecase/Example

In order to create [secure financial systems]((https://www.precisely.com/blog/data-security/pci-compliance-standards-pci-dss)),
it is [sometimes recommended](https://pcidss.com/listing-category/encryption-key-management/
data at rest is encrypted with AES, which is provided by the excellent
 [erlang crypto stdlib](https://www.erlang.org/doc/man/crypto.html#crypto_one_time_aead-6).

For data travelling over a network, PGP has traditionally been preffered. This library tries to provide a simple api library for this.

For example from Circle's [documentation](https://developers.circle.com/docs/accept-card-payments-online):

```javascript
import { createMessage, encrypt, readKey } from 'openpgp'

// Object to be encrypted
interface CardDetails {
 number?: string,    // required when storing card details
 cvv?: string        // required when cardVerification is set to cvv
}

// Encrypted result
interface EncryptedValue {
 encryptedData: string,
 keyId: string
}
 
const pciEncryptionKey = await getPCIPublicKey()
 
/**
* Encrypt card data function
*/
return async function(dataToEncrypt: CardDetails): Promise<EncryptedValue> {
 const decodedPublicKey = await readKey({ armoredKey: atob(publicKey) })
  const message = await createMessage({ text: JSON.stringify(dataToEncrypt) })
  return encrypt({
    message,
    encryptionKeys: decodedPublicKey,
  }).then((ciphertext) => {
    return {
      encryptedMessage: btoa(ciphertext),
      keyId,
    }
  })
}
```

might be achieved in elixir with opal like so:

```elixir
with {:ok, %Key{public_key: key}} <- Opal.get_pci_public_key(), 
     {:ok, json_data} <- Jason.encode(%{hello: "world"}),
     {:ok, message} <- Opal.create_message(json_data),
     {:ok, data} <- Opal.encrypt(key, message) do
       # todo: fix me!
       data
     else
     # error handing
     end
```
