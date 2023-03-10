#![cfg_attr(not(test), no_std)]
#![feature(result_flattening)]

mod checksum_calculator;
mod constants;
pub mod decoder;
pub mod encoder;
#[cfg(feature = "embedded")]
pub mod receiver;
#[cfg(feature = "embedded")]
pub mod transmitter;

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::{
        checksum_calculator::{self, ChecksumCalculator},
        constants::START_BYTE,
        decoder::Decoder,
        encoder::Encoder,
    };
    use alloc::boxed::Box;

    struct MockChecksumCalculator(u16);

    impl checksum_calculator::ChecksumCalculator for MockChecksumCalculator {
        fn calculate(&self, _data: &[u8]) -> u16 {
            self.0
        }
    }

    #[test]
    fn test_encoder_decoder_integration() {
        // given
        let message = "jepa";

        let encoder = Encoder::default();
        let mut decoder = Decoder::default();

        // when
        let encoded_data = encoder.encode(message.as_bytes());

        decoder.put_bytes(encoded_data.ok().unwrap().as_slice());
        let decoded_data = decoder.decoded_data().unwrap();

        // then
        assert_eq!(decoded_data, message.as_bytes());
    }

    #[test]
    fn test_decoder() {
        // given
        let checksum_value = 1337u16;
        let message = "jepa";

        let header: &[u8] = &[START_BYTE];
        let payload: &[u8] = message.as_bytes();
        let length: &[u8] = &(payload.len() as u16).to_ne_bytes();
        let checksum: &[u8] = &checksum_value.to_ne_bytes();

        let protocol_message = [header, length, payload, checksum].concat();

        // when
        let checksum_calculator: Box<dyn ChecksumCalculator> =
            Box::new(MockChecksumCalculator(checksum_value));
        let mut decoder = Decoder::new(checksum_calculator);

        decoder.put_bytes(protocol_message.as_slice());

        // then
        let decoded_payload = decoder.decoded_data().unwrap();
        assert_eq!(payload, decoded_payload);
    }

    #[test]
    fn test_encoder() {
        // given
        let checksum_value = 1337u16;
        let message = "jepa";

        let header: &[u8] = &[START_BYTE];
        let payload: &[u8] = message.as_bytes();
        let length: &[u8] = &(payload.len() as u16).to_ne_bytes();
        let checksum: &[u8] = &checksum_value.to_ne_bytes();

        let protocol_message = [header, length, payload, checksum].concat();

        // when
        let checksum_calculator: Box<dyn ChecksumCalculator> =
            Box::new(MockChecksumCalculator(checksum_value));
        let encoder = Encoder::new(checksum_calculator);
        let sended_protocol_message = encoder.encode(payload).ok().unwrap();

        // then
        assert_eq!(sended_protocol_message, protocol_message)
    }
}
