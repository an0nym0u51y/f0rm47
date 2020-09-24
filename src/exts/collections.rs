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
use core::hash::Hash;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::io::{self, Read, Write};

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                               impl {En,De}code for BTreeMap                                │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl<K, V, E> Encode for BTreeMap<K, V>
where
    K: Encode<Error = E>,
    V: Encode<Error = E>,
    E: From<io::Error>,
{
    type Error = E;

    fn size(&self) -> Result<usize, Self::Error> {
        if self.len() > u16::MAX as usize {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "map.len() > u16::MAX").into());
        }

        let mut size = (self.len() as u16).size()?;
        for (key, value) in self {
            size += key.size()?;
            size += value.size()?;
        }

        Ok(size)
    }

    fn fast_size(&self) -> usize {
        if self.is_empty() {
            (0 as u16).fast_size()
        } else {
            let len = self.len();
            let (key, value) = self.iter().next().unwrap();

            (len as u16).fast_size() + (key.fast_size() + value.fast_size()) * len
        }
    }

    fn encode_into<W: Write>(&self, mut writer: W) -> Result<(), Self::Error> {
        if self.len() > u16::MAX as usize {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "map.len() > u16::MAX").into())
        } else {
            (self.len() as u16).encode_into(&mut writer)?;
            for (key, value) in self {
                key.encode_into(&mut writer)?;
                value.encode_into(&mut writer)?;
            }

            Ok(())
        }
    }
}

impl<K, V, E> Decode for BTreeMap<K, V>
where
    K: Encode<Error = E> + Decode + Ord,
    V: Encode<Error = E> + Decode,
    E: From<io::Error>,
{
    fn decode_with_len_from<R: Read>(mut reader: R) -> Result<(Self, usize), Self::Error> {
        let (len, mut read) = u16::decode_with_len_from(&mut reader)?;

        let mut map = BTreeMap::new();
        for _ in 0..len {
            let (key, readb) = K::decode_with_len_from(&mut reader)?;
            read += readb;

            let (value, readb) = V::decode_with_len_from(&mut reader)?;
            read += readb;

            map.insert(key, value);
        }

        Ok((map, read))
    }
}

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                impl {En,De}code for HashMap                                │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl<K, V, E> Encode for HashMap<K, V>
where
    K: Encode<Error = E>,
    V: Encode<Error = E>,
    E: From<io::Error>,
{
    type Error = E;

    fn size(&self) -> Result<usize, Self::Error> {
        if self.len() > u16::MAX as usize {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "map.len() > u16::MAX").into());
        }

        let mut size = (self.len() as u16).size()?;
        for (key, value) in self {
            size += key.size()?;
            size += value.size()?;
        }

        Ok(size)
    }

    fn fast_size(&self) -> usize {
        if self.is_empty() {
            (0 as u16).fast_size()
        } else {
            let len = self.len();
            let (key, value) = self.iter().next().unwrap();

            (len as u16).fast_size() + (key.fast_size() + value.fast_size()) * len
        }
    }

    fn encode_into<W: Write>(&self, mut writer: W) -> Result<(), Self::Error> {
        if self.len() > u16::MAX as usize {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "map.len() > u16::MAX").into())
        } else {
            (self.len() as u16).encode_into(&mut writer)?;
            for (key, value) in self {
                key.encode_into(&mut writer)?;
                value.encode_into(&mut writer)?;
            }

            Ok(())
        }
    }
}

impl<K, V, E> Decode for HashMap<K, V>
where
    K: Encode<Error = E> + Decode + Hash + Eq,
    V: Encode<Error = E> + Decode,
    E: From<io::Error>,
{
    fn decode_with_len_from<R: Read>(mut reader: R) -> Result<(Self, usize), Self::Error> {
        let (len, mut read) = u16::decode_with_len_from(&mut reader)?;

        let mut map = HashMap::with_capacity(len as usize);
        for _ in 0..len {
            let (key, readb) = K::decode_with_len_from(&mut reader)?;
            read += readb;

            let (value, readb) = V::decode_with_len_from(&mut reader)?;
            read += readb;

            map.insert(key, value);
        }

        Ok((map, read))
    }
}

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                               impl {En,De}code for BTreeSet                                │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl<T> Encode for BTreeSet<T>
where
    T: Encode,
    T::Error: From<io::Error>,
{
    type Error = T::Error;

    fn size(&self) -> Result<usize, Self::Error> {
        if self.len() > u16::MAX as usize {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "set.len() > u16::MAX").into());
        }

        let mut size = (self.len() as u16).size()?;
        for val in self {
            size += val.size()?;
        }

        Ok(size)
    }

    fn fast_size(&self) -> usize {
        if self.is_empty() {
            (0 as u16).fast_size()
        } else {
            let len = self.len();
            let val = self.iter().next().unwrap();

            (len as u16).fast_size() + val.fast_size() * len
        }
    }

    fn encode_into<W: Write>(&self, mut writer: W) -> Result<(), Self::Error> {
        if self.len() > u16::MAX as usize {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "set.len() > u16::MAX").into())
        } else {
            (self.len() as u16).encode_into(&mut writer)?;
            for val in self {
                val.encode_into(&mut writer)?;
            }

            Ok(())
        }
    }
}

impl<T> Decode for BTreeSet<T>
where
    T: Decode + Ord,
    T::Error: From<io::Error>,
{
    fn decode_with_len_from<R: Read>(mut reader: R) -> Result<(Self, usize), Self::Error> {
        let (len, mut read) = u16::decode_with_len_from(&mut reader)?;

        let mut set = BTreeSet::new();
        for _ in 0..len {
            let (val, readb) = T::decode_with_len_from(&mut reader)?;
            read += readb;

            set.insert(val);
        }

        Ok((set, read))
    }
}

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                impl {En,De}code for HashSet                                │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl<T> Encode for HashSet<T>
where
    T: Encode,
    T::Error: From<io::Error>,
{
    type Error = T::Error;

    fn size(&self) -> Result<usize, Self::Error> {
        if self.len() > u16::MAX as usize {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "set.len() > u16::MAX").into());
        }

        let mut size = (self.len() as u16).size()?;
        for val in self {
            size += val.size()?;
        }

        Ok(size)
    }

    fn fast_size(&self) -> usize {
        if self.is_empty() {
            (0 as u16).fast_size()
        } else {
            let len = self.len();
            let val = self.iter().next().unwrap();

            (len as u16).fast_size() + val.fast_size() * len
        }
    }

    fn encode_into<W: Write>(&self, mut writer: W) -> Result<(), Self::Error> {
        if self.len() > u16::MAX as usize {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "set.len() > u16::MAX").into())
        } else {
            (self.len() as u16).encode_into(&mut writer)?;
            for val in self {
                val.encode_into(&mut writer)?;
            }

            Ok(())
        }
    }
}

impl<T> Decode for HashSet<T>
where
    T: Decode + Hash + Eq,
    T::Error: From<io::Error>,
{
    fn decode_with_len_from<R: Read>(mut reader: R) -> Result<(Self, usize), Self::Error> {
        let (len, mut read) = u16::decode_with_len_from(&mut reader)?;

        let mut set = HashSet::with_capacity(len as usize);
        for _ in 0..len {
            let (val, readb) = T::decode_with_len_from(&mut reader)?;
            read += readb;

            set.insert(val);
        }

        Ok((set, read))
    }
}
