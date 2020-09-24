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
use std::collections::LinkedList;
use std::io::{self, Read, Write};

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                             impl {En,De}code for LinkedList<T>                             │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl<T> Encode for LinkedList<T>
where
    T: Encode,
    T::Error: From<io::Error>,
{
    type Error = T::Error;

    fn size(&self) -> Result<usize, Self::Error> {
        if self.len() > u16::MAX as usize {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "list.len() > u16::MAX").into());
        }
       
        let mut size = (self.len() as u16).fast_size();
        for elem in self {
            size += elem.size()?;
        }

        Ok(size)
    }

    fn fast_size(&self) -> usize {
        if self.len() > u16::MAX as usize {
            0
        } else {
            (self.len() as u16).fast_size() + self.front().map(|elem| elem.fast_size() * self.len()).unwrap_or(0)
        }
    }

    fn encode_into<W: Write>(&self, mut writer: W) -> Result<(), Self::Error> {
        if self.len() > u16::MAX as usize {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "buf.len() > u16::MAX").into())
        } else {
            (self.len() as u16).encode_into(&mut writer)?;
            for elem in self {
                elem.encode_into(&mut writer)?;
            }

            Ok(())
        }
    }
}

impl<T> Decode for LinkedList<T>
where
    T: Decode,
    T::Error: From<io::Error>,
{
    fn decode_with_len_from<R: Read>(mut reader: R) -> Result<(Self, usize), Self::Error> {
        let (len, mut read) = u16::decode_with_len_from(&mut reader)?;

        let mut list = LinkedList::new();
        for _ in 0..len {
            let (elem, readb) = T::decode_with_len_from(&mut reader)?;
            list.push_back(elem);
            read += readb;
        }

        Ok((list, read))
    }
}
