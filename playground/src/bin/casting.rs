use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub enum MyOption<T> {
    Some(T),
    None,
}

impl<T> MyOption<T> {
    pub fn as_ref(&self) -> MyOption<&T> {
        println!("type of self = {}", type_of(self));
        match *self {
            MyOption::Some(ref x) => MyOption::Some(x),
            MyOption::None => MyOption::None,
        }
    }
}

pub fn test(&x: &i32) {
    println!("type of x = {}", type_of(x));
}

fn main() {
    let s = String::from("hi there");
    // let s = 5;
    // println!("type of s = {}", type_of(s));
    // test(&s);
    let some = MyOption::Some(&s);
    
    #[allow(unused_variables)]
    let some2 = some.as_ref();
    match some {
      MyOption::Some(x) => println!("type of x = {}", type_of(x)),
      MyOption::None => {},
    }
}