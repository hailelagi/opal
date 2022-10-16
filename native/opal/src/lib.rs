use std::io;
use std::io::Read;

use openpgp::serialize::Marshal;
use sequoia_openpgp as openpgp;

use openpgp::{Message, message};
use openpgp::armor::{Reader, ReaderMode};
use openpgp::parse::Parse;
use openpgp::packet::prelude::*;
use openpgp::types::Curve;

use rustler::{Atom, NifResult, NifStruct};

mod atoms { rustler::atoms! {ok, error} }

#[derive(NifStruct)]
#[module = "Opal.Key"]
pub struct OpalKey {
    pub public_key: String,
}

// TODO: impl trait bound
#[derive(NifStruct)]
#[module = "Opal.Message"]
pub struct OpalMessage {
    pub message: Message,
}

#[rustler::nif]
fn get_pci_public_key() -> NifResult<(Atom, OpalKey)> {
  // todo: use match convert anyhow::Error -> rustler::Error
  let key = Key4::generate_ecc(true, Curve::Ed25519).unwrap();
  let key: Key<key::SecretParts, key::PrimaryRole> = Key::from(key);

  Ok((atoms::ok(), OpalKey{public_key: key.parts_into_public().to_string()}))
}

#[rustler::nif]
fn create_message(text: String) -> NifResult<(Atom, OpalMessage)> {
let mut cursor = io::Cursor::new(&text);
let mut reader = Reader::from_reader(&mut cursor, ReaderMode::VeryTolerant);

let mut buf = Vec::new();
reader.read_to_end(&mut buf).unwrap();


let message = Message::from_bytes(&buf).unwrap();
// TODO:let message = message.serialize(o)

Ok((atoms::ok(), OpalMessage{message}))
}


/*
/* - readKey({ armoredKey: atob(publicKey) }) */
- encrypt({message, encryptionKeys: decodedPublicKey})
*/
/*
#[rustler::nif]
fn encrypt() {

}

*/

rustler::init!("Elixir.Opal", [get_pci_public_key]);

