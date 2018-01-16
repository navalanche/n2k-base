pub mod header;
pub mod j1393;

use self::header::N2kHeader;

/// An N2kMessage has a header and a body. The header is encoded into a 32-bit header field
/// for a CAN-frame. The body is a sequence of up to 8 bytes. Longer messages need to be split into
/// multiple CAN-frames with a counter at the start of each body.
/// The life-time of an N2kMessage is coupled to the liftime of its underlying data sequence.
pub struct N2kMessage {
    pub header: N2kHeader,
    pub len: usize,
    pub body: [u8; 8],
}

impl N2kMessage {
    /// create an N2kMessage
    pub fn new(header: N2kHeader, body: &[u8]) -> N2kMessage {
        let len = body.len();

        let mut copy = [0; 8];
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