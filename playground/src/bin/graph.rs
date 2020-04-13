#[derive (Debug)]
enum List {
  // Cons (Rc <RefCell <i32>>, Rc <List>),
  Cons (i32, RefCell <Rc <List>>),
  // Cons (i32, Rc <RefCell <List>>),
  Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

impl List {
  fn tail (&self) -> Option <&RefCell <Rc <List>>> {
    match self {
      Cons (_, item) => Some (item),
      Nil => None,
    }
  }
}

pub fn main() {
  // let value = Rc::new (RefCell::new (5));
  // let a = Rc::new (Cons (Rc::clone (&value), Rc::new (Nil)));
  // let b = Cons (Rc::new (RefCell::new (6)), Rc::clone (&a));
  // let c = Cons (Rc::new (RefCell::new (10)), Rc::clone (&a));

  // *(value.borrow_mut()) += 10;

  // println!("a after = {:?}", a);
  // println!("b after = {:?}", b);
  // println!("c after = {:?}", c);

  let a = Rc::new (Cons (5, RefCell::new (Rc::new (Nil))));

  // println!("a ini", )

  let b = Rc::new (Cons (10, RefCell::new (Rc::clone (&a))));

  if let Some (link) = a.tail() {
    *(link.borrow_mut()) = Rc::clone (&b);
  }

  println!("a = {:?}", a);
}