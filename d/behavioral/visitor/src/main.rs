pub trait Visitor {
    fn visit_first(&self, component: &FirstComponent);
    fn visit_second(&self, component: &SecondComponent);
}

pub struct FirstVisitor;
impl Visitor for FirstVisitor {
    fn visit_first(&self, component: &FirstComponent) {
        println!("{}FirstVisitor", component.first_operation());
    }

    fn visit_second(&self, component: &SecondComponent) {
        println!("{}FirstVisitor", component.second_operation());
    }
}

pub struct SecondVisitor;
impl Visitor for SecondVisitor {
    fn visit_first(&self, component: &FirstComponent) {
        println!("{}SecondVisitor", component.first_operation());
    }

    fn visit_second(&self, component: &SecondComponent) {
        println!("{}SecondVisitor", component.second_operation());
    }
}

pub trait Component {
    fn accept(&self, visitor: &dyn Visitor);
}

pub struct FirstComponent;

impl FirstComponent {
    pub fn first_operation(&self) -> String {
        String::from("A: ")
    }
}

impl Component for FirstComponent {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_first(self);
    }
}

pub struct SecondComponent;

impl SecondComponent {
    pub fn second_operation(&self) -> String {
        String::from("B: ")
    }
}

impl Component for SecondComponent {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_second(self);
    }
}

fn main() {
    let mut components = Vec::<Box<dyn Component>>::new();
    components.push(Box::new(FirstComponent));
    components.push(Box::new(SecondComponent));

    let first_visitor = FirstVisitor;
    for component in components.iter() {
        component.accept(&first_visitor);
    }

    let second_visitor = SecondVisitor;
    for component in components.iter() {
        component.accept(&second_visitor);
    }
}
