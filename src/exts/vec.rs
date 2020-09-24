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

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                impl {En,De}code for Vec<T>                                 │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl<T> Encode for Vec<T>
where
    T: Encode,
    T::Error: From<io::Error>,
{
    type Error = T::Error;

    default fn size(&self) -> Result<usize, Self::Error> {
        if self.len() > u16::MAX as usize {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "vec.len() > u16::MAX").into());
        }

        let mut size = (self.len() as u16).fast_size();
        for elem in self {
            size += elem.size()?;
        }

        Ok(size)
    }

    default fn fast_size(&self) -> usize {
        (self.len() as u16).fast_size() + self.get(0).map(|elem| elem.fast_size() * self.len()).unwrap_or(0)
    }

    default fn encode_into<W: Write>(&self, mut writer: W) -> Result<(), Self::Error> {
        if self.len() > u16::MAX as usize {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "vec.len() > u16::MAX").into())
        } else {
            (self.len() as u16).encode_into(&mut writer)?;
            for elem in self {
                elem.encode_into(&mut writer)?;
            }

            Ok(())
        }
    }
}

impl<T> Decode for Vec<T>
where
    T: Decode,
    T::Error: From<io::Error>,
{
    default fn decode_with_len_from<R: Read>(mut reader: R) -> Result<(Self, usize), Self::Error> {
        let (len, mut read) = u16::decode_with_len_from(&mut reader)?;
        let mut elems = Vec::with_capacity(len as usize);

        for _ in 0..len {
            let (elem, readb) = T::decode_with_len_from(&mut reader)?;
            elems.push(elem);
            read += readb;
        }

        Ok((elems, read))
    }
}

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                impl {En,De}code for Vec<u8>                                │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl Encode for Vec<u8> {
    fn size(&self) -> Result<usize, Self::Error> {
        Ok(self.fast_size())
    }

    fn fast_size(&self) -> usize {
        self.len() + (self.len() as u16).fast_size()
    }

    fn encode_into<W: Write>(&self, mut writer: W) -> Result<(), Self::Error> {
        if self.len() > u16::MAX as usize {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "vec.len() > u16::MAX"))
        } else {
            (self.len() as u16).encode_into(&mut writer)?;
            writer.write_all(&self)
        }
    }
}

impl Decode for Vec<u8> {
    fn decode_with_len_from<R: Read>(mut reader: R) -> Result<(Self, usize), Self::Error> {
        let (len, read) = u16::decode_with_len_from(&mut reader)?;
        let mut data = vec![0; len as usize];
        reader.read_exact(&mut data)?;

        Ok((data, read + len as usize))
    }
}
