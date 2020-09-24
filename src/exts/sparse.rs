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

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                          #[test]                                           │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

#[cfg(test)]
#[test]
fn sparse_proof() {
    use sparse::{blake3, Tree};

    let foo = blake3::hash(b"foo");
    let bar = blake3::hash(b"bar");
    let baz = blake3::hash(b"baz");

    let mut tree = Tree::new();
    tree.insert(foo);
    tree.insert(bar);
    tree.insert(baz);
    tree.flush();

    let proof = tree.proove(&[foo, baz]).unwrap();

    let encoded1 = proof.as_bytes();
    assert_eq!(proof.fast_size(), encoded1.len());

    let encoded2 = proof.encode().unwrap();
    assert_eq!(encoded1[..], encoded2[2..]);

    let decoded1 = Proof::decode(&encoded2).unwrap();
    let decoded2 = Proof::decode_from(encoded2.as_slice()).unwrap();
    assert_eq!(decoded1, decoded2);
    assert_eq!(decoded1, proof);
}
