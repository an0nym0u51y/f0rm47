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
        if self.len() > u16::MAX as usize {
            0
        } else {
            (self.len() as u16).fast_size() + self.get(0).map(|elem| elem.fast_size() * self.len()).unwrap_or(0)
        }
    }

    default fn encode(&self) -> Result<Vec<u8>, Self::Error> {
        let mut buf = Vec::with_capacity(self.size()?);
        self.encode_into(&mut buf)?;

        Ok(buf)
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
    default fn decode_with_read(buf: &[u8]) -> Result<(Self, usize), Self::Error> {
        Self::decode_with_read_from(buf)
    }

    default fn decode_with_read_from<R: Read>(mut reader: R) -> Result<(Self, usize), Self::Error> {
        let (len, mut read) = u16::decode_with_read_from(&mut reader)?;
        let mut elems = Vec::with_capacity(len as usize);

        for _ in 0..len {
            let (elem, readb) = T::decode_with_read_from(&mut reader)?;
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
        self.as_slice().size()
    }

    fn fast_size(&self) -> usize {
        self.as_slice().fast_size()
    }

    fn encode(&self) -> Result<Vec<u8>, Self::Error> {
        Ok(self.clone())
    }

    fn encode_into<W: Write>(&self, writer: W) -> Result<(), Self::Error> {
        self.as_slice().encode_into(writer)
    }
}

impl Decode for Vec<u8> {
    fn decode_with_read(buf: &[u8]) -> Result<(Self, usize), Self::Error> {
        let (buf, read) = <[u8]>::decode_ref_with_read(buf)?;
        Ok((buf.to_vec(), read))
    }

    fn decode_with_read_from<R: Read>(mut reader: R) -> Result<(Self, usize), Self::Error> {
        let (len, read) = u16::decode_with_read_from(&mut reader)?;
        let mut data = vec![0; len as usize];
        reader.read_exact(&mut data)?;

        Ok((data, read + len as usize))
    }
}
