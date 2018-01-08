use n2k::N2kMessage;
use n2k::header::N2kHeader;

/// IsoAddressClaim
/// See <https://www.kvaser.com/about-can/higher-layer-protocols/j1939-introduction/>
/// for an explanation.
///
/// WIP
pub struct N2kName {
    data: [u8; 8]
}

impl N2kName {
    /// WIP
    pub fn new() -> N2kName {
        N2kName { data: [0x21, 0x04, 0xb4, 0x2f, 0x00, 0x91, 0x79, 0xc0] }
    }
}

impl N2kName {
    pub fn get_data<'a>(&'a self) -> &'a [u8] {
        &self.data
    }
}

/// Create an ISO Address Claim message
pub fn create_iso_address_claim<'a>(priority: u8, source: u8, name: &'a N2kName) -> N2kMessage<'a> {
    let header = N2kHeader::new(priority, source, 255, 60928);
    N2kMessage::new(header, name.get_data())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_iso_address_claim_message() {
        let name = N2kName::new();
        let priority = 6;
        let source = 10;
        let aap = create_iso_address_claim(priority, source, &name);
        let h: u32 = aap.get_header().into();
        assert_eq!(h, 0x18EEFF0A);
        assert_eq!(aap.get_body(), [0x21, 0x04, 0xb4, 0x2f, 0x00, 0x91, 0x79, 0xc0]);
    }
}