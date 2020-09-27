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
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                            impl {En,De}code for {Ip,Socket}Addr                            │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl Encode for IpAddr {
    type Error = io::Error;

    fn fast_size(&self) -> usize {
        match self {
            IpAddr::V4(addr) => 4u8.fast_size() + addr.fast_size(),
            IpAddr::V6(addr) => 6u8.fast_size() + addr.fast_size(),
        }
    }

    fn encode_into<W: Write>(&self, mut writer: W) -> Result<(), Self::Error> {
        match self {
            IpAddr::V4(addr) => {
                4u8.encode_into(&mut writer)?;
                addr.encode_into(writer)
            }
            IpAddr::V6(addr) => {
                6u8.encode_into(&mut writer)?;
                addr.encode_into(writer)
            }
        }
    }
}

impl Encode for SocketAddr {
    type Error = io::Error;

    fn fast_size(&self) -> usize {
        match self {
            SocketAddr::V4(addr) => 4u8.fast_size() + addr.fast_size(),
            SocketAddr::V6(addr) => 4u8.fast_size() + addr.fast_size(),
        }
    }

    fn encode_into<W: Write>(&self, mut writer: W) -> Result<(), Self::Error> {
        match self {
            SocketAddr::V4(addr) => {
                4u8.encode_into(&mut writer)?;
                addr.encode_into(writer)
            }
            SocketAddr::V6(addr) => {
                6u8.encode_into(&mut writer)?;
                addr.encode_into(writer)
            }
        }
    }
}

impl Decode for IpAddr {
    fn decode_with_read_from<R: Read>(mut reader: R) -> Result<(Self, usize), Self::Error> {
        let (tag, read1) = u8::decode_with_read_from(&mut reader)?;
        match tag {
            4 => {
                let (addr, read2) = Ipv4Addr::decode_with_read_from(&mut reader)?;
                Ok((IpAddr::V4(addr), read1 + read2))
            },
            6 => {
                let (addr, read2) = Ipv6Addr::decode_with_read_from(&mut reader)?;
                Ok((IpAddr::V6(addr), read1 + read2))
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "expected '4' or '6'")),
        }
    }
}

impl Decode for SocketAddr {
    fn decode_with_read_from<R: Read>(mut reader: R) -> Result<(Self, usize), Self::Error> {
        let (tag, read1) = u8::decode_with_read_from(&mut reader)?;
        match tag {
            4 => {
                let (addr, read2) = SocketAddrV4::decode_with_read_from(&mut reader)?;
                Ok((SocketAddr::V4(addr), read1 + read2))
            },
            6 => {
                let (addr, read2) = SocketAddrV6::decode_with_read_from(&mut reader)?;
                Ok((SocketAddr::V6(addr), read1 + read2))
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "expected '4' or '6'")),
        }
    }
}

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                             impl {En,De}code for Ip{v4,v6}Addr                             │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl Encode for Ipv4Addr {
    type Error = io::Error;

    fn fast_size(&self) -> usize {
        self.octets().fast_size()
    }

    fn encode_into<W: Write>(&self, writer: W) -> Result<(), Self::Error> {
        self.octets().encode_into(writer)
    }
}

impl Encode for Ipv6Addr {
    type Error = io::Error;

    fn fast_size(&self) -> usize {
        self.octets().fast_size()
    }

    fn encode_into<W: Write>(&self, writer: W) -> Result<(), Self::Error> {
        self.octets().encode_into(writer)
    }
}

impl Decode for Ipv4Addr {
    fn decode_with_read_from<R: Read>(reader: R) -> Result<(Self, usize), Self::Error> {
        let (bytes, len) = <[u8; 4]>::decode_with_read_from(reader)?;
        Ok((Self::from(bytes), len))
    }
}

impl Decode for Ipv6Addr {
    fn decode_with_read_from<R: Read>(reader: R) -> Result<(Self, usize), Self::Error> {
        let (bytes, len) = <[u8; 16]>::decode_with_read_from(reader)?;
        Ok((Self::from(bytes), len))
    }
}

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                           impl {En,De}code for SocketAddr{V4,V6}                           │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl Encode for SocketAddrV4 {
    type Error = io::Error;

    fn fast_size(&self) -> usize {
        self.ip().fast_size() + self.port().fast_size()
    }

    fn encode_into<W: Write>(&self, mut writer: W) -> Result<(), Self::Error> {
        self.ip().encode_into(&mut writer)?;
        self.port().encode_into(&mut writer)?;

        Ok(())
    }
}

impl Encode for SocketAddrV6 {
    type Error = io::Error;

    fn fast_size(&self) -> usize {
        self.ip().fast_size()
            + self.port().fast_size()
            + self.flowinfo().fast_size()
            + self.scope_id().fast_size()
    }

    fn encode_into<W: Write>(&self, mut writer: W) -> Result<(), Self::Error> {
        self.ip().encode_into(&mut writer)?;
        self.port().encode_into(&mut writer)?;
        self.flowinfo().encode_into(&mut writer)?;
        self.scope_id().encode_into(&mut writer)?;

        Ok(())
    }
}

impl Decode for SocketAddrV4 {
    fn decode_with_read_from<R: Read>(mut reader: R) -> Result<(Self, usize), Self::Error> {
        let (ip, read1) = Ipv4Addr::decode_with_read_from(&mut reader)?;
        let (port, read2) = u16::decode_with_read_from(&mut reader)?;

        Ok((Self::new(ip, port), read1 + read2))
    }
}

impl Decode for SocketAddrV6 {
    fn decode_with_read_from<R: Read>(mut reader: R) -> Result<(Self, usize), Self::Error> {
        let (ip, read1) = Ipv6Addr::decode_with_read_from(&mut reader)?;
        let (port, read2) = u16::decode_with_read_from(&mut reader)?;
        let (flowinfo, read3) = u32::decode_with_read_from(&mut reader)?;
        let (scope_id, read4) = u32::decode_with_read_from(&mut reader)?;

        Ok((Self::new(ip, port, flowinfo, scope_id), read1 + read2 + read3 + read4))
    }
}
