use crate::types::{Transport, TransportError, Encoder, Decoder};

pub struct MarionetteTransport<T: Decoder + Encoder> {
	message_id: i16,
	protocol: i32,
	de: T
}

impl<T: Decoder + Encoder> Transport for MarionetteTransport<T> {
    fn message_id(&self) -> i16 {
        todo!()
    }

    fn close(self) -> Result<(), TransportError> {
        todo!()
    }

    fn send(&mut self, command: String, data: crate::types::Value) -> Result<crate::types::Response, TransportError> {
        todo!()
    }

    fn receive(&mut self) -> Result<Vec<u8>, TransportError> {
        todo!()
    }
}
