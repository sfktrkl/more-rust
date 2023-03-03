trait TemplateMethod {
    fn template_method(&self) {
        self.base_operation();
        self.hook();
        self.required_operation();
    }

    fn base_operation(&self) {
        print!("TemplateMethod")
    }

    fn required_operation(&self);

    fn hook(&self) {
        print!(" -> ");
    }
}

pub struct FirstClass;
impl TemplateMethod for FirstClass {
    fn required_operation(&self) {
        println!("FirstClass");
    }
}

pub struct SecondClass;
impl TemplateMethod for SecondClass {
    fn required_operation(&self) {
        println!("SecondClass");
    }

    fn hook(&self) {
        print!(" => ");
    }
}

fn main() {
    let first_class = FirstClass;
    first_class.template_method();

    let second_class = SecondClass;
    second_class.template_method();
}
