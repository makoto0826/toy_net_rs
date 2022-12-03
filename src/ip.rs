pub struct InternetProtocolV4<'a> {
    pub data: &'a [u8],
}

impl<'a> InternetProtocolV4<'a> {
    pub fn decode(data: &[u8], size: usize) -> InternetProtocolV4 {
        let byte = data[0];
        // let version = (byte & 0xF0) >> 4;
        let header_size = 4 * ((byte & 0x0F) as usize);

        InternetProtocolV4 {
            data: &data[header_size..size],
        }
    }
}
