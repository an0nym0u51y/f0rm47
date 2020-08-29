/**************************************************************************************************
 *                                                                                                *
 * This Source Code Form is subject to the terms of the Mozilla Public                            *
 * License, v. 2.0. If a copy of the MPL was not distributed with this                            *
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.                                       *
 *                                                                                                *
 **************************************************************************************************/

// =========================================== Imports ========================================== \\

use core::mem;
use std::collections::BTreeMap;

#[cfg(feature = "thiserror")]
use thiserror::Error;

// ========================================= Interfaces ========================================= \\

pub trait Encode {
    fn encode<'buf>(&self, buf: &'buf mut [u8]) -> Result<(usize, &'buf mut [u8])>;
}

pub trait Decode<'buf>: Sized {
    fn decode(buf: &'buf [u8]) -> Result<(Self, &'buf [u8])>;
}

// ============================================ Types =========================================== \\

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
#[cfg_attr(feature = "thiserror", derive(Error))]
pub enum Error {
    #[cfg(feature = "sparse")]
    #[cfg_attr(feature = "thiserror", error("{0}"))]
    Sp4rs3(sparse::Error),
}

// ======================================== macro_rules! ======================================== \\

macro_rules! assert_min_len {
    ($buf:ident, $min:expr) => {
        // TODO
    };
}

macro_rules! assert_max_len {
    ($buf:ident, $max:expr) => {
        // TODO
    };
}

macro_rules! encode_len {
    ($buf:ident, $len:expr) => {
        {
            let len = $len;
            if len > u16::MAX as usize {
                // TODO
            }

            (len as u16).encode($buf)?
        }
    };
}

macro_rules! decode_len {
    ($buf:ident) => {
        {
            let (len, buf) = u16::decode($buf)?;
            (len as usize, buf)
        }
    };
}

macro_rules! primitive {
    ($primitive:ty) => {
        impl Encode for $primitive {
            fn encode<'buf>(&self, buf: &'buf mut [u8]) -> Result<(usize, &'buf mut [u8])> {
                self.to_le_bytes().encode(buf)
            }
        }

        impl<'buf> Decode<'buf> for $primitive {
            fn decode(buf: &'buf [u8]) -> Result<(Self, &'buf [u8])> {
                let (bytes, buf) = <[u8; mem::size_of::<$primitive>()]>::decode(buf)?;
                Ok((<$primitive>::from_le_bytes(bytes), buf))
            }
        }
    };
}

macro_rules! bytes {
    ([u8; $size:literal]) => {
        impl Encode for [u8; $size] {
            fn encode<'buf>(&self, buf: &'buf mut [u8]) -> Result<(usize, &'buf mut [u8])> {
                assert_min_len!(buf, $size);

                buf[0..$size].copy_from_slice(self);

                Ok(($size, &mut buf[$size..]))
            }
        }

        impl<'buf> Decode<'buf> for [u8; $size] {
            fn decode(buf: &'buf [u8]) -> Result<(Self, &'buf [u8])> {
                assert_min_len!(buf, $size);

                let mut bytes = [0; $size];
                bytes[..].copy_from_slice(&buf[0..$size]);

                Ok((bytes, &buf[$size..]))
            }
        }
    };
}

// ========================================= primitive! ========================================= \\

primitive!(u16);
primitive!(u32);
primitive!(u64);

primitive!(i16);
primitive!(i32);
primitive!(i64);

// =========================================== bytes! =========================================== \\

bytes!([u8; 2]);
bytes!([u8; 4]);
bytes!([u8; 8]);
bytes!([u8; 16]);
bytes!([u8; 32]);
bytes!([u8; 64]);

// ========================================= impl Encode ======================================== \\

#[cfg(feature = "chrono")]
impl Encode for chrono::DateTime<chrono::Utc> {
    fn encode<'buf>(&self, buf: &'buf mut [u8]) -> Result<(usize, &'buf mut [u8])> {
        self.naive_utc().encode(buf)
    }
}

#[cfg(feature = "chrono")]
impl Encode for chrono::NaiveDateTime {
    fn encode<'buf>(&self, buf: &'buf mut [u8]) -> Result<(usize, &'buf mut [u8])> {
        self.timestamp().encode(buf)
    }
}

#[cfg(feature = "pow")]
impl Encode for pow::Proofs {
    fn encode<'buf>(&self, buf: &'buf mut [u8]) -> Result<(usize, &'buf mut [u8])> {
        let desc = self.description();
        let (mut bytes, buf) = desc.encode(buf)?;

        let levels = self.levels();
        let (octets, buf) = encode_len!(buf, levels);
        bytes += octets;

        let proofs = self.proofs();
        let (octets, buf) = encode_len!(buf, proofs);
        bytes += octets;

        let nodes = self.as_nodes();
        let (octets, mut buf) = encode_len!(buf, nodes.len());
        bytes += octets;

        assert_min_len!(buf, 34 * nodes.len());
        for (node, hash) in nodes {
            let (octets, rest) = encode_len!(buf, *node);
            bytes += octets;
            buf = rest;

            let (octets, rest) = hash.encode(buf)?;
            bytes += octets;
            buf = rest;
        }

        Ok((bytes, buf))
    }
}

#[cfg(feature = "sparse")]
impl Encode for sparse::Hash {
    fn encode<'buf>(&self, buf: &'buf mut [u8]) -> Result<(usize, &'buf mut [u8])> {
        <[u8; 32]>::encode(self.as_bytes(), buf)
    }
}

#[cfg(feature = "sparse")]
impl Encode for sparse::Proof {
    fn encode<'buf>(&self, buf: &'buf mut [u8]) -> Result<(usize, &'buf mut [u8])> {
        self.as_bytes().encode(buf)
    }
}

impl Encode for [u8] {
    fn encode<'buf>(&self, buf: &'buf mut [u8]) -> Result<(usize, &'buf mut [u8])> {
        let (bytes, buf) = encode_len!(buf, self.len());

        buf[0..self.len()].copy_from_slice(self);

        Ok((bytes + self.len(), &mut buf[self.len()..]))
    }
}

impl<T: Encode> Encode for [T] {
    fn encode<'buf>(&self, buf: &'buf mut [u8]) -> Result<(usize, &'buf mut [u8])> {
        let (mut bytes, mut buf) = encode_len!(buf, self.len());

        for elem in self {
            let (octets, rest) = elem.encode(buf)?;
            bytes += octets;
            buf = rest;
        }

        Ok((bytes, buf))
    }
}

// ========================================= impl Decode ======================================== \\

#[cfg(feature = "chrono")]
impl<'buf> Decode<'buf> for chrono::DateTime<chrono::Utc> {
    fn decode(buf: &'buf [u8]) -> Result<(Self, &'buf [u8])> {
        let (time, buf) = chrono::NaiveDateTime::decode(buf)?;
        Ok((Self::from_utc(time, chrono::Utc), buf))
    }
}

#[cfg(feature = "chrono")]
impl<'buf> Decode<'buf> for chrono::NaiveDateTime {
    fn decode(buf: &'buf [u8]) -> Result<(Self, &'buf [u8])> {
        let (time, buf) = i64::decode(buf)?;
        Ok((Self::from_timestamp(time, 0), buf))
    }
}

#[cfg(feature = "pow")]
impl<'buf> Decode<'buf> for pow::Proofs {
    fn decode(buf: &'buf [u8]) -> Result<(Self, &'buf [u8])> {
        let (desc, buf) = <&[u8]>::decode(buf)?;
        let (levels, buf) = decode_len!(buf);
        let (proofs, buf) = decode_len!(buf);
        let (len, mut buf) = decode_len!(buf);

        let mut nodes = BTreeMap::new();
        for _ in 0..len {
            let (node, rest) = decode_len!(buf);
            buf = rest;

            let (hash, rest) = <[u8; 32]>::decode(buf)?;
            buf = rest;

            nodes.insert(node, hash);
        }

        Ok((Self::new(desc, levels, proofs, nodes), buf))
    }
}

#[cfg(feature = "sparse")]
impl<'buf> Decode<'buf> for sparse::Hash {
    fn decode(buf: &'buf [u8]) -> Result<(Self, &'buf [u8])> {
        let (bytes, rest) = <[u8; 32]>::decode(buf)?;

        Ok((Self::from(bytes), rest))
    }
}

#[cfg(feature = "sparse")]
impl<'buf> Decode<'buf> for sparse::Proof {
    fn decode(buf: &'buf [u8]) -> Result<(Self, &'buf [u8])> {
        let (bytes, rest) = <&[u8]>::decode(buf)?;
       
        Ok((Self::from_bytes(bytes)?, rest))
    }
}

impl<'buf> Decode<'buf> for &'buf [u8] {
    fn decode(buf: &'buf [u8]) -> Result<(Self, &'buf [u8])> {
        let (len, buf) = decode_len!(buf);
        assert_min_len!(buf, len);

        Ok(buf.split_at(len))
    }
}

impl<'buf> Decode<'buf> for Vec<u8> {
    fn decode(buf: &'buf [u8]) -> Result<(Self, &'buf [u8])> {
        let (len, buf) = decode_len!(buf);
        assert_min_len!(buf, len);

        let (bytes, rest) = buf.split_at(len);
        Ok((Vec::from(bytes), rest))
    }
}

impl<'buf, T: Decode<'buf>> Decode<'buf> for Vec<T> {
    fn decode(buf: &'buf [u8]) -> Result<(Self, &'buf [u8])> {
        let (len, mut buf) = decode_len!(buf);

        let mut elems = Vec::with_capacity(len);
        for _ in 0..len {
            let (elem, rest) = T::decode(buf)?;

            elems.push(elem);
            buf = rest;
        }

        Ok((elems, buf))
    }
}

// ========================================== impl From ========================================= \\

#[cfg(feature = "sparse")]
impl From<sparse::Error> for Error {
    fn from(error: sparse::Error) -> Self {
        Error::Sp4rs3(error)
    }
}
