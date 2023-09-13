use super::Message;

#[derive(PartialEq, Eq)]
pub struct MessagesList {
    pub messages: Vec<Message>,
}

impl MessagesList {
    pub fn new() -> Self {
        Self { messages: Vec::new() }
    }

    pub fn from(messages: Vec<Message>) -> Self {
        Self {
            messages
        }
    }

    pub fn push(&mut self, message:Message) {
        self.messages.insert(0, message)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Message> {
        self.messages.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Message> {
        self.messages.iter_mut()
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn clear(&mut self) {
        self.messages.clear()
    }

    pub fn send_out(&self) -> Vec<Message> {
        let mut buf = Vec::new();
        for i in self.iter() {
            buf.insert(0, i.clone())
        };
        buf
    }

}

unsafe impl Send for MessagesList {}