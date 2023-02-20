#[derive(Clone)]
pub struct Info {
    field: i32,
}

impl Info {
    pub fn new(field: i32) -> Self {
        Info { field: field }
    }
}

#[derive(Clone)]
pub struct Prototype {
    field: i32,
    info: Info,
}

impl Prototype {
    pub fn new(field: i32) -> Self {
        Prototype {
            field: field,
            info: Info::new(2),
        }
    }

    pub fn print(&self) {
        println!("field = {} info = {}", self.field, self.info.field)
    }
}

fn main() {
    let mut prototype = Prototype::new(1);
    print!("Prototype: ");
    prototype.print();

    let clone = prototype.clone();
    print!("Clone: ");
    clone.print();

    prototype.info.field = 3;

    print!("Prototype: ");
    prototype.print();

    print!("Clone: ");
    clone.print();
}
