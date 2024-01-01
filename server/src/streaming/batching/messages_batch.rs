use crate::streaming::models::messages_batch::MessagesBatch;
use bytes::{BufMut, Bytes};
use iggy::models::messages::Message;

impl MessagesBatch {
    pub fn new(base_offset: u64, length: u32, last_offset_delta: u32, messages: Bytes) -> Self {
        Self {
            base_offset,
            length,
            last_offset_delta,
            messages,
        }
    }
    pub fn get_last_offset(&self) -> u64 {
        self.base_offset + self.last_offset_delta as u64
    }
    pub fn messages_to_batch(
        base_offset: u64,
        last_offset_delta: u32,
        messages: Vec<Message>,
    ) -> Self {
        let payload: Vec<_> = messages
            .iter()
            .flat_map(|message| message.to_bytes())
            .collect();
        let len = 8 + 4 + 4 + payload.len() as u32;
        Self::new(base_offset, len, last_offset_delta, Bytes::from(payload))
    }

    pub fn get_size_bytes(&self) -> u32 {
        return 8 + 4 + 4 + self.messages.len() as u32;
    }
    pub fn extend(&self, bytes: &mut Vec<u8>) {
        bytes.put_u64_le(self.base_offset);
        bytes.put_u32_le(self.length);
        bytes.put_u32_le(self.last_offset_delta);
        bytes.extend(&self.messages);
    }
}
