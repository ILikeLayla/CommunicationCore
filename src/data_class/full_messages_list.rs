use super::FullMessage;

#[derive(Clone)]
pub struct FullMessagesList {
    messages: Vec<FullMessage>
}

unsafe impl Send for FullMessagesList {
    
}


impl FullMessagesList {
    pub fn new() -> Self {
        Self { messages: Vec::new() }
    }

    pub fn from(messages: Vec<FullMessage>) -> Self {
        Self {
            messages
        }
    }

    pub fn push(&mut self, message:FullMessage) {
        self.messages.insert(0, message)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, FullMessage> {
        self.messages.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, FullMessage> {
        self.messages.iter_mut()
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn clear(&mut self) {
        self.messages.clear()
    }

    // pub fn send_out(&self) -> Vec<FullMessage> {
    //     let mut buf = Vec::new();
    //     for i in self.iter() {
    //         buf.insert(0, i.clone())
    //     };
    //     buf
    // }

}