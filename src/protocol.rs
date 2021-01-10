use crate::types::{Decoder, DecoderError, Encoder, Response};

pub struct Proto3EncoderDecoder;

impl Encoder for Proto3EncoderDecoder{
    fn encode(
        &self,
        t: impl crate::types::Transport,
        command: String,
        value: crate::types::Value,
    ) -> Result<Vec<u8>, crate::types::EncoderError> {
        todo!()
    }
}

impl Decoder for Proto3EncoderDecoder {
    fn decode(& self, buf: Vec<u8>) -> Result<Response, DecoderError> {
        todo!()
    }
}
