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
    pub fn new_from_parts(
        _unique_number: u32, // 21 bits, 0..21
        _manufacturer: u8, // 11 bits, 21..32
        _dev_instance_lower: u8, // 3 bits, 32..35
        _dev_instance_upper: u8, // 5 bits, 35..40
        _dev_function: u8, // 8 bits, 40..48
        _reserverd: u8, // 1 bit, 48..49
        _dev_class: u8, //7 bits, 49..56
        _sys_instance: u8, //4 bits, 56..60
        _industry_group: u8, // 3 bits, start 4, 60..63
        _reserved: u8 // 1 bit, 63..64
    ) -> N2kName {
        let mut data = [0x21, 0x04, 0xb4, 0x2f, 0x00, 0x91, 0x79, 0x00];

        let _industry_group = _industry_group & 0x70; //mask the bits

        data[7] = data[7] | _industry_group;

        N2kName { data: data }
    }
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
pub fn create_iso_address_claim(priority: u8, source: u8, name: &N2kName) -> N2kMessage {
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