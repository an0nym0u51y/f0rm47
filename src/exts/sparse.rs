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

use crate::{Decode, DecodeRef, Encode};
use sparse::Proof;
use std::io::{self, Read, Write};

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                 impl {En,De}code for Proof                                 │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl Encode for Proof {
    type Error = io::Error;

    fn fast_size(&self) -> usize {
        Proof::size(self)
    }

    fn encode_into<W: Write>(&self, writer: W) -> Result<(), Self::Error> {
        self.as_bytes().encode_into(writer)
    }
}

impl Decode for Proof {
    fn decode_with_len(buf: &[u8]) -> Result<(Self, usize), Self::Error> {
        let (buf, read) = <[u8]>::decode_ref_with_len(buf)?;
        if let Ok(proof) = Proof::from_bytes(buf) {
            Ok((proof, read))
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "invalid proof data"))
        }
    }

    fn decode_with_len_from<R: Read>(reader: R) -> Result<(Self, usize), Self::Error> {
        let (buf, read) = Vec::<u8>::decode_with_len_from(reader)?;
        if let Ok(proof) = Proof::from_bytes(&buf) {
            Ok((proof, read))
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "invalid proof data"))
        }
    }
}
