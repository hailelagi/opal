use sequoia_openpgp as openpgp;
use openpgp::cert::prelude::*;

use openpgp::packet::prelude::*;
use sequoia_openpgp::parse::Parse;
use openpgp::serialize::Serialize;
use openpgp::types::Curve;

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

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

/* - getPCIPublicKey() */
#[rustler::nif]
fn get_pci_public_key() Key {
  Key::from(Key4::generate_ecc(true, Curve::Ed25519)?);
}

// Generate a new certificate.  It has secret key material.
let (cert, _) = CertBuilder::new()
    .generate()?;

let pk = cert.primary_key().key();
assert!(pk.has_secret());

// Serializing a `Key<key::PublicParts, _>` drops the secret key
// material.
let mut bytes = Vec::new();
Packet::from(pk.clone()).serialize(&mut bytes);
let p : Packet = Packet::from_bytes(&bytes)?;

if let Packet::PublicKey(key) = p {
    assert!(! key.has_secret());
} else {
    unreachable!();
}

rustler::init!("Elixir.Opal", [add]);
