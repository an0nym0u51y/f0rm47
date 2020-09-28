/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                                                                            │ *
 * │ This Source Code Form is subject to the terms of the Mozilla Public                        │ *
 * │ License, v. 2.0. If a copy of the MPL was not distributed with this                        │ *
 * │ file, You can obtain one at http://mozilla.org/MPL/2.0/.                                   │ *
 * │                                                                                            │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                          Imports                                           │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

use crate::{Decode, Encode};
use ed25519::{PublicKey, Signature};
use std::io::{self, Read, Write};

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                               impl {En,De}code for PublicKey                               │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl Encode for PublicKey {
    type Error = io::Error;

    fn fast_size(&self) -> usize {
        self.to_bytes().fast_size()
    }

    fn encode_into<W: Write>(&self, writer: W) -> Result<(), Self::Error> {
        self.to_bytes().encode_into(writer)
    }
}

impl Decode for PublicKey {
    fn decode_with_read(buf: &[u8]) -> Result<(Self, usize), Self::Error> {
        let (bytes, read) = <[u8; 32]>::decode_with_read(buf)?;
        if let Ok(key) = PublicKey::from_bytes(&bytes) {
            Ok((key, read))
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "invalid public key"))
        }
    }

    fn decode_with_read_from<R: Read>(reader: R) -> Result<(Self, usize), Self::Error> {
        let (bytes, read) = <[u8; 32]>::decode_with_read_from(reader)?;
        if let Ok(key) = PublicKey::from_bytes(&bytes) {
            Ok((key, read))
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "invalid public key"))
        }
    }
}

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                               impl {En,De}code for Signature                               │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl Encode for Signature {
    type Error = io::Error;

    fn fast_size(&self) -> usize {
        self.to_bytes().fast_size()
    }

    fn encode_into<W: Write>(&self, writer: W) -> Result<(), Self::Error> {
        self.to_bytes().encode_into(writer)
    }
}

impl Decode for Signature {
    fn decode_with_read_from<R: Read>(reader: R) -> Result<(Self, usize), Self::Error> {
        let (bytes, read) = <[u8; 64]>::decode_with_read_from(reader)?;
        Ok((Signature::from(bytes), read))
    }
}

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                          #[test]                                           │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

#[cfg(test)]
#[test]
fn public_key() {
    use rand::rngs::OsRng;
    use ed25519::{Keypair, PUBLIC_KEY_LENGTH};

    let keypair = Keypair::generate(&mut OsRng);
    let pubkey = keypair.public;
    assert_eq!(pubkey.fast_size(), PUBLIC_KEY_LENGTH);

    let encoded = pubkey.encode().unwrap();
    assert_eq!(encoded, pubkey.as_bytes());

    let decoded = PublicKey::decode(&encoded).unwrap();
    assert_eq!(decoded, pubkey);
}

#[cfg(test)]
#[test]
fn signature() {
    use rand::rngs::OsRng;
    use ed25519::{Keypair, Signer, SIGNATURE_LENGTH};

    let keypair = Keypair::generate(&mut OsRng);
    let msg = [0, 1, 2, 3, 4, 5, 6, 7];
    let signature = keypair.sign(&msg);
    assert_eq!(signature.fast_size(), SIGNATURE_LENGTH);

    let encoded = signature.encode().unwrap();
    assert_eq!(encoded, signature.to_bytes());

    let decoded = Signature::decode(&encoded).unwrap();
    assert_eq!(decoded, signature);
}
