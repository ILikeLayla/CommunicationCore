use super::{Level, Event, Nothing};

pub struct EventList<T, E> {
    hurry: Vec<Event<T, E>>,
    medium: Vec<Event<T,E>>,
    least: Vec<Event<T, E>>,
}

impl<T, E> EventList<T, E> {
    pub fn new() -> Self {
        Self {
            hurry: Vec::new(),
            medium: Vec::new(),
            least: Vec::new(),
        }
    }

    pub fn join(&mut self, event: Event<T, E>) {
        let point = match event.level {
            Level::Hurry => &mut self.hurry,
            Level::Medium => &mut self.medium,
            Level::Least => &mut self.least
        };
        point.insert(0, event);
    }

    pub fn make(&mut self) {
        if let Some(event) = self.hurry.pop() {
            event.make()
        } else if let Some(event) = self.medium.pop() {
            event.make()
        } else if let Some(event) = self.least.pop() {
            event.make()
        }
    }
}

impl<T: Nothing<T>, E> EventList<T, E> {
    pub fn clear(&mut self) {
        self.clear_hurry();
        self.clear_medium();
        self.clear_least()
    }

    pub fn clear_hurry(&mut self) {
        for i in self.least.iter_mut() {
            i.delete();
            i.make()
        };
        self.hurry.clear()
    }

    pub fn clear_medium(&mut self) {
        for i in self.medium.iter_mut() {
            i.delete();
            i.make()
        };
        self.medium.clear()
    }

    pub fn clear_least(&mut self) {
        for i in self.hurry.iter_mut() {
            i.delete();
            i.make()
        };
        self.medium.clear()
    }
}