#![cfg_attr(not(test), no_std)]
#![feature(result_flattening)]

mod checksum_calculator;
mod constants;
#[cfg(feature = "embedded")]
pub mod embedded;
pub mod receiver;
pub mod transmitter;

#[cfg(test)]
mod tests {
    extern crate alloc;

    use crate::transmitter::Transmitter;

    use super::{
        checksum_calculator::{self, ChecksumCalculator},
        constants::START_BYTE,
        receiver::{Receiver, ReceiverState},
    };
    use alloc::boxed::Box;

    struct MockChecksumCalculator(u16);

    impl checksum_calculator::ChecksumCalculator for MockChecksumCalculator {
        fn calculate(&self, _data: &[u8]) -> u16 {
            self.0
        }
    }

    #[test]
    fn test_receiver() {
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
        let mut receiver = Receiver::new(checksum_calculator);
        let mut received_payload: Option<([u8; 8192], u16)> = None;

        for byte in protocol_message {
            let state = receiver.put_byte(byte);
            if let ReceiverState::Complete(Ok(payload)) = state {
                received_payload = Some(payload);
                break;
            }
        }

        // then
        let received_payload = received_payload.unwrap();
        assert_eq!(
            &received_payload.0[..received_payload.1.into()],
            message.as_bytes()
        );
    }

    #[test]
    fn test_transmitter() {
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
        let transmitter = Transmitter::new(checksum_calculator);
        let sended_protocol_message = transmitter.prepare_to_transmit(payload).ok().unwrap();

        // then
        assert_eq!(sended_protocol_message, protocol_message)
    }
}
