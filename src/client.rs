use crate::types::Relay;

pub struct Client;

impl Relay for Client {
    fn send(&mut self, command: String, data: crate::types::Value) {
        todo!()
    }
}
