pub struct Product {
    parts: Vec<String>,
}

impl Product {
    pub fn new() -> Self {
        Product {
            parts: Vec::<String>::new(),
        }
    }

    pub fn add(&mut self, part: String) {
        self.parts.push(part)
    }

    pub fn list_parts(&self) {
        print!("Created: ");
        for i in 0..self.parts.len() {
            print!("{}", self.parts[i]);
            if self.parts.len() > (i + 1) {
                print!(", ")
            }
        }
        println!("");
    }
}

pub trait Builder {
    fn build_part_a(&mut self);
    fn build_part_b(&mut self);
    fn build_part_c(&mut self);
}

pub struct ProductBuilder {
    product: Product,
}

impl ProductBuilder {
    pub fn new() -> Self {
        ProductBuilder {
            product: Product::new(),
        }
    }

    pub fn get_product(&mut self) -> Product {
        std::mem::replace(&mut self.product, Product::new())
    }
}

impl Builder for ProductBuilder {
    fn build_part_a(&mut self) {
        self.product.add(String::from("Part A"));
    }

    fn build_part_b(&mut self) {
        self.product.add(String::from("Part B"));
    }

    fn build_part_c(&mut self) {
        self.product.add(String::from("Part C"));
    }
}

pub struct Director;
impl Director {
    pub fn build_minimal_product(&self, builder: &mut impl Builder) {
        builder.build_part_a();
    }

    pub fn build_full_product(&self, builder: &mut impl Builder) {
        builder.build_part_a();
        builder.build_part_b();
        builder.build_part_c();
    }
}

fn main() {
    let mut builder = ProductBuilder::new();
    let director = Director;

    director.build_minimal_product(&mut builder);
    builder.get_product().list_parts();

    director.build_minimal_product(&mut builder);
    builder.build_part_b();
    builder.get_product().list_parts();

    director.build_full_product(&mut builder);
    builder.get_product().list_parts();
}
