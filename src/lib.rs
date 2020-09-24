/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                                                                            │ *
 * │ This Source Code Form is subject to the terms of the Mozilla Public                        │ *
 * │ License, v. 2.0. If a copy of the MPL was not distributed with this                        │ *
 * │ file, You can obtain one at http://mozilla.org/MPL/2.0/.                                   │ *
 * │                                                                                            │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                       Configuration                                        │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

#![feature(min_const_generics, min_specialization)]

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                          Imports                                           │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

mod exts;

use core::mem;
use std::io::{self, Read, Write};

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                         Interfaces                                         │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

pub trait Encode {
    type Error;

    fn size(&self) -> Result<usize, Self::Error> {
        Ok(self.fast_size())
    }

    fn fast_size(&self) -> usize;

    fn encode(&self) -> Result<Vec<u8>, Self::Error> {
        let mut buf = Vec::with_capacity(self.size()?);
        self.encode_into(&mut buf)?;

        Ok(buf)
    }

    fn encode_into<W: Write>(&self, writer: W) -> Result<(), Self::Error>;
}

pub trait Decode: Encode + Sized {
    fn decode(buf: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::decode_with_len(buf)?.0)
    }

    fn decode_from<R: Read>(reader: R) -> Result<Self, Self::Error> {
        Ok(Self::decode_with_len_from(reader)?.0)
    }

    fn decode_with_len(buf: &[u8]) -> Result<(Self, usize), Self::Error> {
        Self::decode_with_len_from(buf)
    }

    fn decode_with_len_from<R: Read>(reader: R) -> Result<(Self, usize), Self::Error>;
}

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                     impl Encode for &T                                     │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl<T: Encode + ?Sized> Encode for &T {
    type Error = T::Error;
   
    fn size(&self) -> Result<usize, Self::Error> {
        (**self).size()
    }

    fn fast_size(&self) -> usize {
        (**self).fast_size()
    }

    fn encode(&self) -> Result<Vec<u8>, Self::Error> {
        (**self).encode()
    }

    fn encode_into<W: Write>(&self, writer: W) -> Result<(), Self::Error> {
        (**self).encode_into(writer)
    }
}

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                impl {En,De}code for Box<T>                                 │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl<T: Encode + ?Sized> Encode for Box<T> {
    type Error = T::Error;

    fn size(&self) -> Result<usize, Self::Error> {
        (**self).size()
    }

    fn fast_size(&self) -> usize {
        (**self).fast_size()
    }

    fn encode(&self) -> Result<Vec<u8>, Self::Error> {
        (**self).encode()
    }

    fn encode_into<W: Write>(&self, writer: W) -> Result<(), Self::Error> {
        (**self).encode_into(writer)
    }
}

impl<T: Decode + ?Sized> Decode for Box<T> {
    fn decode(buf: &[u8]) -> Result<Self, Self::Error> {
        T::decode(buf).map(Box::new)
    }

    fn decode_from<R: Read>(reader: R) -> Result<Self, Self::Error> {
        T::decode_from(reader).map(Box::new)
    }

    fn decode_with_len(buf: &[u8]) -> Result<(Self, usize), Self::Error> {
        let (val, len) = T::decode_with_len(buf)?;
        Ok((Box::new(val), len))
    }

    fn decode_with_len_from<R: Read>(reader: R) -> Result<(Self, usize), Self::Error> {
        let (val, len) = T::decode_with_len_from(reader)?;
        Ok((Box::new(val), len))
    }
}

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                               impl {En,De}code for Option<T>                               │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl<T: Encode> Encode for Option<T>
where
    T::Error: From<io::Error>,
{
    type Error = T::Error;

    fn size(&self) -> Result<usize, Self::Error> {
        if let Some(val) = self {
            Ok(val.size()? + true.fast_size())
        } else {
            Ok(false.fast_size())
        }
    }

    fn fast_size(&self) -> usize {
        if let Some(val) = self {
            val.fast_size() + true.fast_size()
        } else {
            false.fast_size()
        }
    }

    fn encode_into<W: Write>(&self, mut writer: W) -> Result<(), Self::Error> {
        if let Some(val) = self {
            true.encode_into(&mut writer)?;
            val.encode_into(writer)
        } else {
            false.encode_into(&mut writer)?;
            Ok(())
        }
    }
}

impl<T: Decode> Decode for Option<T>
where
    T::Error: From<io::Error>,
{
    fn decode_with_len_from<R: Read>(mut reader: R) -> Result<(Self, usize), Self::Error> {
        match bool::decode_with_len_from(&mut reader)? {
            (true, read1) => {
                let (val, read2) = T::decode_with_len_from(reader)?;
                Ok((Some(val), read1 + read2))
            }
            (false, read) => Ok((None, read))
        }
    }
}

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                           Macros                                           │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

macro_rules! primitive {
    ($primitive:ty) => {
        impl Encode for $primitive {
            type Error = io::Error;

            fn fast_size(&self) -> usize {
                mem::size_of::<$primitive>()
            }

            fn encode_into<W: Write>(&self, mut writer: W) -> Result<(), Self::Error> {
                writer.write_all(&self.to_le_bytes())
            }
        }

        impl Decode for $primitive {
            fn decode_with_len(buf: &[u8]) -> Result<(Self, usize), Self::Error> {
                let bytes = <[u8; mem::size_of::<$primitive>()]>::decode(buf)?;
                Ok((Self::from_le_bytes(bytes), mem::size_of::<$primitive>()))
            }

            fn decode_with_len_from<R: Read>(reader: R) -> Result<(Self, usize), Self::Error> {
                let bytes = <[u8; mem::size_of::<$primitive>()]>::decode_from(reader)?;
                Ok((Self::from_le_bytes(bytes), mem::size_of::<$primitive>()))
            }
        }
    };
}

macro_rules! tuple {
    ($($idx:tt: $name:ident),+) => {
        impl<Err, $($name),+> Encode for ($($name),+)
        where
            $($name: Encode<Error = Err>,)+
        {
            type Error = Err;

            fn size(&self) -> Result<usize, Self::Error> {
                Ok($(self.$idx.size()? +)+ 0)
            }

            fn fast_size(&self) -> usize {
                $(self.$idx.fast_size() +)+ 0
            }

            fn encode_into<W: Write>(&self, mut writer: W) -> Result<(), Self::Error> {
                $(self.$idx.encode_into(&mut writer)?;)+
                Ok(())
            }
        }

        impl<Err, $($name),+> Decode for ($($name),+)
        where
            $($name: Encode<Error = Err> + Decode,)+
        {
            #[allow(clippy::eval_order_dependence)]
            fn decode_with_len_from<R: Read>(mut reader: R) -> Result<(Self, usize), Self::Error> {
                let mut len = 0;
                let val = (
                    $({
                        let (val, read) = <$name>::decode_with_len_from(&mut reader)?;
                        len += read;
                        val
                    },)+
                );

                Ok((val, len))
            }
        }
    };
}

macro_rules! tuples {
    ($idx1:tt: $name1:ident, $idx2:tt: $name2:ident $(, $($idx:tt: $name:ident),+)?) => {
        tuples!(@INTERNAL; ($idx1: $name1, $idx2: $name2) ($($($idx: $name),+)?));
    };

    (@INTERNAL;
     ($($idx:tt: $name:ident),+) ()
    ) => {
        tuple!($($idx: $name),+);
    };

    (@INTERNAL;
     ($($idx:tt: $name:ident),+) ($oidx:tt: $oname:ident $(, $($ridx:tt: $rname:ident),+)?)
    ) => {
        tuple!($($idx: $name),+);
        tuples!(@INTERNAL; ($($idx: $name),+, $oidx: $oname) ($($($ridx: $rname),+)?));
    };
}

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                       primitive!(..)                                       │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

primitive!(u8);
primitive!(u16);
primitive!(u32);
primitive!(u64);
primitive!(u128);

primitive!(i8);
primitive!(i16);
primitive!(i32);
primitive!(i64);
primitive!(i128);

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                        tuples!(..)                                         │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

tuples!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K, 11: L, 12: M);

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                 impl {En,De}code for bool                                  │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl Encode for bool {
    type Error = io::Error;

    fn fast_size(&self) -> usize {
        (*self as u8).fast_size()
    }

    fn encode_into<W: Write>(&self, writer: W) -> Result<(), Self::Error> {
        (*self as u8).encode_into(writer)
    }
}

impl Decode for bool {
    fn decode_with_len(buf: &[u8]) -> Result<(Self, usize), Self::Error> {
        let (val, len) = u8::decode_with_len(buf)?;
        match val {
            0 => Ok((false, len)),
            1 => Ok((true, len)),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "expected 0u8 or 1u8")),
        }
    }

    fn decode_with_len_from<R: Read>(reader: R) -> Result<(Self, usize), Self::Error> {
        let (val, len) = u8::decode_with_len_from(reader)?;
        match val {
            0 => Ok((false, len)),
            1 => Ok((true, len)),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "expected 0u8 or 1u8")),
        }
    }
}

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                             impl {En,De}code for {usize,isize}                             │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl Encode for usize {
    type Error = io::Error;

    fn fast_size(&self) -> usize {
        (*self as u64).fast_size()
    }

    fn encode_into<W: Write>(&self, writer: W) -> Result<(), Self::Error> {
        (*self as u64).encode_into(writer)
    }
}

impl Encode for isize {
    type Error = io::Error;

    fn fast_size(&self) -> usize {
        (*self as i64).fast_size()
    }

    fn encode_into<W: Write>(&self, writer: W) -> Result<(), Self::Error> {
        (*self as i64).encode_into(writer)
    }
}

impl Decode for usize {
    fn decode_with_len(buf: &[u8]) -> Result<(Self, usize), Self::Error> {
        let (num, len) = u64::decode_with_len(buf)?;
        Ok((num as usize, len))
    }

    fn decode_with_len_from<R: Read>(reader: R) -> Result<(Self, usize), Self::Error> {
        let (num, len) = u64::decode_with_len_from(reader)?;
        Ok((num as usize, len))
    }
}

impl Decode for isize {
    fn decode_with_len(buf: &[u8]) -> Result<(Self, usize), Self::Error> {
        let (num, len) = i64::decode_with_len(buf)?;
        Ok((num as isize, len))
    }

    fn decode_with_len_from<R: Read>(reader: R) -> Result<(Self, usize), Self::Error> {
        let (num, len) = i64::decode_with_len_from(reader)?;
        Ok((num as isize, len))
    }
}

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                impl {En,De}code for [u8; _]                                │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl<const LEN: usize> Encode for [u8; LEN] {
    type Error = io::Error;

    fn fast_size(&self) -> usize {
        LEN
    }

    fn encode_into<W: Write>(&self, mut writer: W) -> Result<(), Self::Error> {
        writer.write_all(self)
    }
}

impl<const LEN: usize> Decode for [u8; LEN] {
    fn decode_with_len(buf: &[u8]) -> Result<(Self, usize), Self::Error> {
        if buf.len() < LEN {
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "not enough data"))
        } else {
            let mut bytes = [0; LEN];
            bytes[..].copy_from_slice(&buf[..LEN]);

            Ok((bytes, LEN))
        }
    }

    fn decode_with_len_from<R: Read>(mut reader: R) -> Result<(Self, usize), Self::Error> {
        let mut bytes = [0; LEN];
        reader.read_exact(&mut bytes)?;

        Ok((bytes, LEN))
    }
}
