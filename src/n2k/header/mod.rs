/// An N2kHeader is just a u32, created using the right fields.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct N2kHeader(u32);

impl N2kHeader {
    /// create an N2kHeader
    pub fn new(priority: u8, source: u8, destination: u8, pgn: u32) -> N2kHeader {
        // TODO: asserts on incoming values (maybe even Result<N2kHeader>)
        let mut id = (priority as u32) << 26 | (pgn << 8) | source as u32;

        let pdu_format = (pgn >> 8) as u8;

        if pdu_format < 240 {
            //addressable, thus set the destination
            id = id | ((destination as u32) << 8);
        }
        N2kHeader(id)
    }
}

/// automatic conversion into a u32
impl Into<u32> for N2kHeader {
    fn into(self) -> u32 {
        self.0
    }
}

/// Basic trait to derive N2k header data. Usually the header is just a simple u32.
/// Implementation provided below.
pub trait N2kHeaderDecoder {
    fn get_pgn(&self) -> u32;
    fn get_source(&self) -> u8;
    fn get_destination(&self) -> u8;
    fn get_priority(&self) -> u8;
}

impl N2kHeaderDecoder for N2kHeader {
    fn get_pgn(&self) -> u32 {
        let pf = get_pdu_format(&self.0);
        let pdu_specific = get_pdu_specific(&self.0);
        let dp = get_data_page(&self.0);

        if pf < 240 {
            (dp as u32) << 16 | (pf as u32) << 8
        } else {
            (dp as u32) << 16 | (pf as u32) << 8 | pdu_specific as u32
        }
    }

    fn get_source(&self) -> u8 {
        self.0 as u8
    }

    fn get_destination(&self) -> u8 {
        let pf = get_pdu_format(&self.0);
        let ps = get_pdu_specific(&self.0);
        if pf < 240 {
            ps
        } else {
            0xFF
        }
    }

    fn get_priority(&self) -> u8 {
        ((self.0 >> 26) & 0x7) as u8
    }
}


fn get_pdu_specific(id: &u32) -> u8 {
    (id >> 8) as u8
}

fn get_pdu_format(id: &u32) -> u8 {
    (id >> 16) as u8
}

fn get_data_page(id: &u32) -> u8 {
    ((id >> 24) & 1 as u32) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_iso_address_claim_header() {
        let id = N2kHeader(0x18EEFF0A);
        assert_eq!(id.get_source(), 10);
        assert_eq!(id.get_pgn(), 60928);
        assert_eq!(id.get_destination(), 255);
        assert_eq!(id.get_priority(), 6);
    }

    #[test]
    fn read_wind_data_header() {
        let id = N2kHeader(0x09FD020A);

        assert_eq!(id.get_source(), 10);
        assert_eq!(id.get_pgn(), 130306);
        assert_eq!(id.get_destination(), 255);
        assert_eq!(id.get_priority(), 2);
    }

    #[test]
    fn create_iso_address_claim_header() {
        let id = N2kHeader::new(
            6,
            10,
            255,
            60928,
        );

        assert_eq!(id, N2kHeader(0x18EEFF0A));
        assert_eq!(id.get_source(), 10);
        assert_eq!(id.get_pgn(), 60928);
        assert_eq!(id.get_destination(), 255);
        assert_eq!(id.get_priority(), 6);
    }

    #[test]
    fn create_wind_data_header() {
        let id = N2kHeader::new(
            2,
            10,
            255,
            130306,
        );
        assert_eq!(id, N2kHeader(0x09FD020A));
        assert_eq!(id.get_source(), 10);
        assert_eq!(id.get_pgn(), 130306);
        assert_eq!(id.get_destination(), 255);
        assert_eq!(id.get_priority(), 2);
    }
}