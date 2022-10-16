#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

/*
SCOPE -

- getPCIPublicKey()
- readKey({ armoredKey: atob(publicKey) })
- createMessage({ text: JSON.stringify(dataToEncrypt) })
- encrypt({
    message,
    encryptionKeys: decodedPublicKey,
  }).then((ciphertext) => {
    return {
      encryptedMessage: btoa(ciphertext),
      keyId,
    }
  })
*/

rustler::init!("Elixir.Opal", [add]);
