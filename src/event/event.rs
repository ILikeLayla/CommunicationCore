use std::sync::{Arc, Mutex};

pub enum Level {
    Hurry,
    Medium,
    Least
}

pub enum State<T, E> {
    Waiting,
    Finished(Result<T, E>)
}

pub struct Event<T, E> {
    to_do: fn() -> Result<T, E>,
    call: Arc<Mutex<State<T, E>>>,
    pub level: Level,
}

impl<T, E> Event<T, E> {
    pub fn new(to_do: fn() -> Result<T, E>, level: Level, call: Arc<Mutex<State<T, E>>>) -> Self {
        Self {
            to_do, level, call
        }
    }

    pub fn make(&self) {
        let out = (self.to_do)();
        *self.call.lock().unwrap() = State::Finished(out);
    }
}

pub trait Nothing<T> {
    fn nothing() -> T;
}

impl<T: Nothing<T>, E> Event<T, E> {
    pub fn delete(&mut self) {
        self.to_do = || {Ok(T::nothing())}
    }
}

impl Nothing<String> for String {
    fn nothing() -> String {
        "".to_string()
    }
}