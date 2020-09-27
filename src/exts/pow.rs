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
use pow::Proofs;
use std::collections::BTreeMap;
use std::io::{self, Read, Write};

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                impl {En,De}code for Proofs                                 │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl Encode for Proofs {
    type Error = io::Error;

    fn fast_size(&self) -> usize {
        self.description().fast_size()
            + (self.levels() as u16).fast_size()
            + (self.proofs() as u16).fast_size()
            + self.as_nodes().fast_size()
    }

    fn encode_into<W: Write>(&self, mut writer: W) -> Result<(), Self::Error> {
        if self.levels() > u16::MAX as usize || self.proofs() > u16::MAX as usize {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "levels and proofs should be < u16::MAX"))
        } else {
            self.description().encode_into(&mut writer)?;
            (self.levels() as u16).encode_into(&mut writer)?;
            (self.proofs() as u16).encode_into(&mut writer)?;
            self.as_nodes().encode_into(&mut writer)
        }
    }
}

impl Decode for Proofs {
    fn decode_with_read_from<R: Read>(mut reader: R) -> Result<(Self, usize), Self::Error> {
        let (desc, read1) = Vec::<u8>::decode_with_read_from(&mut reader)?;
        let (levels, read2) = u16::decode_with_read_from(&mut reader)?;
        let (proofs, read3) = u16::decode_with_read_from(&mut reader)?;
        let (nodes, read4) = BTreeMap::<usize, [u8; 32]>::decode_with_read_from(&mut reader)?;

        Ok((Proofs::new(desc, levels as usize, proofs as usize, nodes), read1 + read2 + read3 + read4))
    }
}

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                          #[test]                                           │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

#[cfg(test)]
#[test]
fn proofs() {
    use pow::Tree;

    let tree = Tree::new("foobar", 8);
    let proofs = tree.gen_proofs_with(64);
    assert_eq!(proofs.fast_size(), "foobar".len() + 8 + proofs.as_nodes().len() * 40);

    let encoded = proofs.encode().unwrap();
    let decoded = Proofs::decode(&encoded).unwrap();
    assert!(decoded.verify().is_ok());
    assert_eq!(decoded.levels(), 8);
    assert_eq!(decoded.proofs(), 64);
}
