use std::{cell::RefCell, rc::Rc};

trait Observer {
    fn update(&self, subject: &dyn Subject);
}

trait Subject {
    fn get_state(&self) -> i32;
    fn attach(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn detach(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn notify(&self);
}

struct Publisher {
    state: i32,
    observers: Vec<Rc<RefCell<dyn Observer>>>,
}

impl Publisher {
    pub fn new() -> Self {
        Publisher {
            state: 0,
            observers: Vec::new(),
        }
    }

    pub fn operation(&mut self) {
        self.state += 1;
        print!("Subject {}:", self.state);
        self.notify();
    }
}

impl Subject for Publisher {
    fn get_state(&self) -> i32 {
        self.state
    }

    fn attach(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }

    fn detach(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        if let Some(idx) = self.observers.iter().position(|x| {
            &*x.borrow() as *const dyn Observer == &*observer.borrow() as *const dyn Observer
        }) {
            self.observers.remove(idx);
        }
    }

    fn notify(&self) {
        for observer in self.observers.iter() {
            observer.borrow_mut().update(self);
        }
        println!("");
    }
}

pub struct FirstObserver;
impl Observer for FirstObserver {
    fn update(&self, subject: &dyn Subject) {
        if subject.get_state() % 2 == 0 {
            print!(" FirstObserver");
        }
    }
}

pub struct SecondObserver;
impl Observer for SecondObserver {
    fn update(&self, subject: &dyn Subject) {
        if subject.get_state() % 3 == 0 {
            print!(" SecondObserver");
        }
    }
}

fn main() {
    let mut publisher = Publisher::new();
    let first_observer = Rc::new(RefCell::new(FirstObserver));
    let second_observer = Rc::new(RefCell::new(SecondObserver));

    publisher.attach(first_observer.clone());
    publisher.attach(second_observer.clone());

    publisher.operation();
    publisher.operation();
    publisher.operation();

    publisher.detach(first_observer.clone());
    publisher.operation();
}
