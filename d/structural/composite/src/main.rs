pub trait Component {
    fn search(&self, keyword: String);
}

pub struct Leaf {
    name: String,
}

impl Leaf {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Component for Leaf {
    fn search(&self, keyword: String) {
        println!("Searching for {} in {}", keyword, self.name);
    }
}

pub struct Composite {
    name: String,
    components: Vec<Box<dyn Component>>,
}

impl Composite {
    pub fn new(name: String) -> Self {
        Self {
            name,
            components: vec![],
        }
    }

    pub fn add(&mut self, component: impl Component + 'static) {
        self.components.push(Box::new(component));
    }
}

impl Component for Composite {
    fn search(&self, keyword: String) {
        println!("Searching recursively for {} in {}", keyword, self.name);

        for component in self.components.iter() {
            component.search(keyword.clone());
        }
    }
}

fn main() {
    let first_leaf = Leaf::new(String::from("Leaf 1"));
    let second_leaf = Leaf::new(String::from("Leaf 2"));

    let mut first_composite = Composite::new(String::from("Composite 1"));
    first_composite.add(first_leaf);

    let mut second_composite = Composite::new(String::from("Composite 2"));
    second_composite.add(second_leaf);
    second_composite.add(first_composite);

    second_composite.search(String::from("keyword"));
}
