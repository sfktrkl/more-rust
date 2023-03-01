struct Originator {
    state: String,
}
impl Originator {
    pub fn new(state: String) -> Self {
        println!("Originator: {}", state);
        Originator { state: state }
    }

    pub fn operation(&mut self) {
        self.state = format!("{}{}", self.state, "-operation");
        println!("Originator: {}", self.state);
    }

    fn save(&self) -> Box<Memento> {
        Box::new(Memento::new(self.state.clone()))
    }

    fn restore(&mut self, memento: Box<Memento>) {
        self.state = memento.get_state();
        println!("Originator: {}", self.state);
    }
}

struct Memento {
    state: String,
}
impl Memento {
    pub fn new(state: String) -> Self {
        Memento { state: state }
    }

    fn get_state(&self) -> String {
        self.state.clone()
    }
}

struct Caretaker {
    originator: Box<Originator>,
    history: Vec<Box<Memento>>,
}
impl Caretaker {
    pub fn new(originator: Box<Originator>) -> Caretaker {
        Caretaker {
            originator: originator,
            history: Vec::new(),
        }
    }

    fn backup(&mut self) {
        self.history.push(self.originator.save())
    }

    fn undo(&mut self) {
        let memento = self.history.remove(self.history.len() - 1);
        self.originator.restore(memento);
    }
}

fn main() {
    let originator = Originator::new(String::from("original"));
    let mut caretaker = Caretaker::new(Box::new(originator));

    caretaker.backup();
    caretaker.originator.operation();

    caretaker.backup();
    caretaker.originator.operation();

    caretaker.undo();
    caretaker.undo();
}
