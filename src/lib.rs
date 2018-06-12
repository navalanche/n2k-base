//!
//! Basic definitions of data structures and traits for N2K networks.
//! See <https://en.wikipedia.org/wiki/NMEA_2000>.
//!

/*
Copyright (C) 2018 Erwin Gribnau

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

#![no_std]
pub mod header;
pub mod j1393;

use self::header::N2kHeader;


// max length chosen to support the length of a product-info message
const N2K_DATA_MAX : usize = 134;
/// An N2kMessage has a header and a body. The header is encoded into a 32-bit header field
/// for a CAN-frame. The body is a sequence of up to 8 bytes. Longer messages need to be split into
/// multiple CAN-frames with a counter at the start of each body.
///
pub struct N2kMessage {
    pub header: N2kHeader,
    pub len: usize,
    //TODO: investigate a good strategy for the data storage (no_std env, so Vec<> is out)
    pub body: [u8; N2K_DATA_MAX],
}

impl N2kMessage {
    /// create an N2kMessage
    pub fn new(header: N2kHeader, body: &[u8]) -> N2kMessage {
        let len = body.len();

        let mut copy = [0; N2K_DATA_MAX];
        for (index, value) in body.iter().enumerate() {
            copy[index] = *value;
        }

        N2kMessage { header: header, len: len, body: copy }
    }

    pub fn get_header(&self) -> N2kHeader {
        self.header
    }

    pub fn get_body(&self) -> &[u8] {
        &self.body[0..self.len]
    }
}