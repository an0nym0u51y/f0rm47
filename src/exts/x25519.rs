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
use std::io::{self, Read, Write};
use x25519::PublicKey;

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
        Ok((PublicKey::from(bytes), read))
    }

    fn decode_with_read_from<R: Read>(reader: R) -> Result<(Self, usize), Self::Error> {
        let (bytes, read) = <[u8; 32]>::decode_with_read_from(reader)?;
        Ok((PublicKey::from(bytes), read))
    }
}
