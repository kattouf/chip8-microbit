pub trait ChecksumCalculator {
    fn calculate(&self, data: &[u8]) -> u16;
}

pub struct CRCChecksumCalculator(crc::Crc<u16<>>);

impl CRCChecksumCalculator {
    pub fn new() -> Self {
        CRCChecksumCalculator(crc::Crc::<u16>::new(&crc::CRC_16_IBM_SDLC))
    }
}

impl ChecksumCalculator for CRCChecksumCalculator {
    fn calculate(&self, data: &[u8]) -> u16 {
        self.0.checksum(data)
    }
}
