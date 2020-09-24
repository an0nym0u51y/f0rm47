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
use chrono::{Date, DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc};
use std::io::{self, Read, Write};

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                             impl {En,De}code for DateTime<Utc>                             │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl Encode for DateTime<Utc> {
    type Error = io::Error;

    fn fast_size(&self) -> usize {
        self.naive_utc().fast_size()
    }

    fn encode_into<W: Write>(&self, writer: W) -> Result<(), Self::Error> {
        self.naive_utc().encode_into(writer)
    }
}

impl Decode for DateTime<Utc> {
    fn decode_with_len_from<R: Read>(reader: R) -> Result<(Self, usize), Self::Error> {
        let (datetime, len) = NaiveDateTime::decode_with_len_from(reader)?;
        Ok((Self::from_utc(datetime, Utc), len))
    }
}

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                             impl {En,De}code for NaiveDateTime                             │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl Encode for NaiveDateTime {
    type Error = io::Error;

    fn fast_size(&self) -> usize {
        self.date().fast_size() + self.time().fast_size()
    }

    fn encode_into<W: Write>(&self, mut writer: W) -> Result<(), Self::Error> {
        self.date().encode_into(&mut writer)?;
        self.time().encode_into(&mut writer)
    }
}

impl Decode for NaiveDateTime {
    fn decode_with_len_from<R: Read>(mut reader: R) -> Result<(Self, usize), Self::Error> {
        let (date, read1) = NaiveDate::decode_with_len_from(&mut reader)?;
        let (time, read2) = NaiveTime::decode_with_len_from(&mut reader)?;
        Ok((Self::new(date, time), read1 + read2))
    }
}

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                               impl {En,De}code for Date<Utc>                               │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl Encode for Date<Utc> {
    type Error = io::Error;

    fn fast_size(&self) -> usize {
        self.naive_utc().fast_size()
    }

    fn encode_into<W: Write>(&self, writer: W) -> Result<(), Self::Error> {
        self.naive_utc().encode_into(writer)
    }
}

impl Decode for Date<Utc> {
    fn decode_with_len_from<R: Read>(reader: R) -> Result<(Self, usize), Self::Error> {
        let (date, len) = NaiveDate::decode_with_len_from(reader)?;
        Ok((Self::from_utc(date, Utc), len))
    }
}

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                               impl {En,De}code for NaiveDate                               │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl Encode for NaiveDate {
    type Error = io::Error;

    fn fast_size(&self) -> usize {
        self.num_days_from_ce().fast_size()
    }

    fn encode_into<W: Write>(&self, writer: W) -> Result<(), Self::Error> {
        self.num_days_from_ce().encode_into(writer)
    }
}

impl Decode for NaiveDate {
    fn decode_with_len(buf: &[u8]) -> Result<(Self, usize), Self::Error> {
        let (days, len) = i32::decode_with_len(buf)?;
        Ok((Self::from_num_days_from_ce(days), len))
    }

    fn decode_with_len_from<R: Read>(reader: R) -> Result<(Self, usize), Self::Error> {
        let (days, len) = i32::decode_with_len_from(reader)?;
        Ok((Self::from_num_days_from_ce(days), len))
    }
}

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                               impl {En,De}code for NaiveTime                               │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

impl Encode for NaiveTime {
    type Error = io::Error;

    fn fast_size(&self) -> usize {
        self.num_seconds_from_midnight().fast_size()
    }

    fn encode_into<W: Write>(&self, writer: W) -> Result<(), Self::Error> {
        self.num_seconds_from_midnight().encode_into(writer)
    }
}

impl Decode for NaiveTime {
    fn decode_with_len(buf: &[u8]) -> Result<(Self, usize), Self::Error> {
        let (secs, len) = u32::decode_with_len(buf)?;
        Ok((Self::from_num_seconds_from_midnight(secs, 0), len))
    }

    fn decode_with_len_from<R: Read>(reader: R) -> Result<(Self, usize), Self::Error> {
        let (secs, len) = u32::decode_with_len_from(reader)?;
        Ok((Self::from_num_seconds_from_midnight(secs, 0), len))
    }
}
