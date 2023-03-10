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
        decoder::{Decoder, DecodingState},
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

        for byte in protocol_message {
            decoder.put_byte(byte);

            if let DecodingState::Complete(_) = decoder.get_state() {
                break;
            }
        }

        // then
        let decoded_payload = decoder.take_decoded_data().unwrap();
        assert_eq!(payload, decoded_payload.as_slice());
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
