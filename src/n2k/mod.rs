pub mod header;
pub mod j1393;

use self::header::N2kHeader;

/// An N2kMessage has a header and a body. The header is encoded into a 32-bit header field
/// for a CAN-frame. The body is a sequence of up to 8 bytes. Longer messages need to be split into
/// multiple CAN-frames with a counter at the start of each body.
/// The life-time of an N2kMessage is coupled to the liftime of its underlying data sequence.
#[derive(Debug)]
pub struct N2kMessage<'a>  {
    pub header: N2kHeader,
    pub body: &'a[u8],
}

impl <'a> N2kMessage<'a> {
    /// create an N2kMessage
    pub fn new(header: N2kHeader, body: &'a [u8]) -> N2kMessage {
        N2kMessage { header: header, body: body}
    }

    pub fn get_header(&self) -> N2kHeader {
        self.header
    }

    pub fn get_body(&self) -> &'a [u8] {
        self.body
    }
}