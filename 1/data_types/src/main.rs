use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let x = 1;
    println!("{}", type_of(x));
    let y: i8 = 1;
    println!("{}", type_of(y));
    let z: i16 = 1;
    println!("{}", type_of(z));

    let k = 1.0;
    println!("{}", type_of(k));
    let l: f32 = 1.0;
    println!("{}", type_of(l));

    let t = true;
    println!("{}", type_of(t));
    let f: bool = false;
    println!("{}", type_of(f));

    let c = 'c';
    println!("{}", type_of(c));
    let z: char = 'z';
    println!("{}", type_of(z));
}
