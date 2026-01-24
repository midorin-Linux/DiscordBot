use crate::models::message::Message;

#[derive(Debug, Clone)]
pub struct ConversationMemory {
    messages: Vec<Message>,
    max_size: usize,
}

impl ConversationMemory {
    pub fn new(max_size: usize) -> Self {
        Self {
            messages: Vec::new(),
            max_size,
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        if self.messages.len() > self.max_size {
            self.messages.remove(0);
        }
    }

    pub fn get_messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }
}