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
    fn decode_with_len_from<R: Read>(reader: R) -> Result<(Self, usize), Self::Error> {
        let (bytes, read) = <[u8; 32]>::decode_with_len_from(reader)?;
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
    fn decode_with_len_from<R: Read>(reader: R) -> Result<(Self, usize), Self::Error> {
        let (bytes, read) = <[u8; 64]>::decode_with_len_from(reader)?;
        Ok((Signature::from(bytes), read))
    }
}
