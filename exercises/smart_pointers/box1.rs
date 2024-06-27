// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: the value of the current item and
// the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `todo!()`
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.

use crate::List::{Cons, Nil};

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    pub fn cons_len(&self) -> usize {
        match self {
            Cons(_, next) => 1 + next.cons_len(),
            Nil => 0,
        }
    }
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );

    let user_input: i32 = 32;

    println!(
        "This is a user inputted cons list: {:?}",
        create_user_input_cons_list(user_input)
    )
}

pub fn create_empty_list() -> List {
    return Nil;
}

pub fn create_non_empty_list() -> List {
    return Cons(1, Box::new(Cons(2, Box::new(Nil))));
}
pub fn create_user_input_cons_list(n: i32) -> List {
    if n < 1 {
        return Nil;
    }

    let mut list = Nil;

    for i in (1..=n).rev() {
        list = Cons(i, Box::new(list));
    }

    return list;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }

    #[test]
    fn test_create_user_inputted_cons_list() {
        let input: i32 = 32;
        let list = create_user_input_cons_list(input);
        assert_eq!(list.cons_len(), input as usize);
    }
}
