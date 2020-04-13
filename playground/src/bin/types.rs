// Here could also do list: &Vec<i32>
fn largest_i32 (list: &[i32]) -> i32 {
  let mut largest: i32 = (*list)[0];

  // item: i32
  // list.iter() returns references,
  // so in a way we destructure the reference
  for &item in list.iter() {
      if item > largest {
          largest = item;
      }
  }

  largest
}

fn largest_char (list: &[char]) -> char {
  let mut largest: char = (*list)[0];

  // item: &i32
  for item in list.iter() {
      if (*item) > largest {
          // Need to dereference the reference
          largest = *item
      }
  }

  largest
}

fn largest_copy<T: PartialOrd + Copy> (list: &[T]) -> T {
  // Works because T is Copy
  let mut largest: T = list[0];

  for &item in list.iter() {
      if item > largest {
          largest = item;
      }
  }

  largest
}

fn largest_copy_ref<T: PartialOrd + Copy> (list: &[T]) -> T {
  let mut largest: &T = &(*list)[0];

  for item in list.iter() {
      if item > largest {
          largest = item;
      }
  }

  *largest
}

fn largest_string (list: &[String]) -> String {
  let mut largest: &String = &((*list)[0]);

  for item in list.iter() {
      if (*item) > (*largest) {
          largest = item;
      }
  }

  // Works because String is Clone
  largest.clone()
}

// Inefficient implementation
fn largest_clone_ineff<T: PartialOrd + Clone> (list: &[T]) -> T {
  let mut largest: T = (*list)[0].clone();

  for item in list.iter() {
      if (*item) > largest {
          largest = item.clone();
      }
  }

  largest
}

// Similar to largest_string
fn largest_clone<T: PartialOrd + Clone> (list: &[T]) -> T {
  let mut largest: &T = &((*list)[0]);

  for item in list.iter() {
      if (*item) > (*largest) {
          largest = item;
      }
  }

  largest.clone()
}

// Best implementation - works in alll cases
fn largest<T: PartialOrd> (list: &[T]) -> &T {
  let mut largest: &T = &((*list)[0]);

  for item in list.iter() {
      if (*item) > (*largest) {
          largest = item;
      }
  }

  largest
}

pub fn main() {
  // Vector of i32
  let mut number_list = vec![34, 50, 25, 100, 65];
  number_list.push (200);
  let result_i32 = largest_i32 (&number_list);
  let result_copy = largest_copy (&number_list);
  let result_copy_ref = largest_copy_ref (&number_list);
  let result = largest (&number_list);

  assert_eq!(result_i32, result_copy);
  assert_eq!(result_copy, result_copy_ref);
  assert_eq!(result_copy_ref, *result);
  println!("The largest number is {}", result);

  // Array of char
  let char_list = ['y', 'm', 'a', 'q'];
  let result_char = largest_char (&char_list);
  let result_copy = largest_copy (&char_list);
  let result_copy_ref = largest_copy_ref (&char_list);
  let result = largest (&char_list);

  assert_eq!(result_char, result_copy);
  assert_eq!(result_copy, result_copy_ref);
  assert_eq!(result_copy_ref, *result);
  println!("The largest char is {}", result);

  // Vector of String
  let string_list = vec![String::from("ab"), String::from("aa")];
  println!("1st string larger than 2nd: {}", string_list[0] > string_list[1]);
  let result_string = largest_string (&string_list);
  let result_clone_ineff = largest_clone_ineff (&string_list);
  let result_clone = largest_clone (&string_list);
  let result = largest (&string_list);

  assert_eq!(result_string, result_clone_ineff);
  assert_eq!(result_clone_ineff, result_clone);
  assert_eq!(result_clone, result.clone());
  println!("The largest string is {}", result);

  // let a = String::from("hellooo");
  // let b = &a;
  // println!("hello {}", a.deref());
}