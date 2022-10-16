use sequoia_openpgp as openpgp;
// use openpgp::cert::prelude::*;

use openpgp::packet::prelude::*;
// use sequoia_openpgp::parse::Parse;
// use openpgp::serialize::Serialize;
use openpgp::types::Curve;

use rustler::{Atom, NifResult, NifStruct};

mod atoms { rustler::atoms! {ok, error} }

#[derive(NifStruct)]
#[module = "Opal.Key"]
pub struct OpalKey {
    pub public_key: String,
}

/*
SCOPE -

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

/* - getPCIPublicKey() */
#[rustler::nif]
fn get_pci_public_key() -> NifResult<(Atom, OpalKey)> {
  let key = Key4::generate_ecc(true, Curve::Ed25519).unwrap();
  let key: Key<key::SecretParts, key::PrimaryRole> = Key::from(key);

  Ok((atoms::ok(), OpalKey{public_key: key.parts_into_public().to_string()}))
}

rustler::init!("Elixir.Opal", [get_pci_public_key]);
