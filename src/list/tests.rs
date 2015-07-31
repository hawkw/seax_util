use super::{List, Stack};
use super::List::{Cons,Nil};

#[test]
fn test_list_length() {
    let full_list: List<i32> = list!(1i32, 2i32, 3i32);
    let empty_list: List<i32> = List::new();
    assert_eq!(full_list.length(), 3);
    assert_eq!(empty_list.length(), 0);
}

#[test]
fn test_list_to_string() {
    let l: List<i32> = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    assert_eq!(l.to_string(), "(1, 2, 3)");
}

#[test]
fn test_stack_length() {
    let full_stack: List<i32> = list!(1i32, 2i32, 3i32);
    let empty_stack: List<i32> = Stack::empty();
    assert_eq!(full_stack.length(), 3);
    assert_eq!(empty_stack.length(), 0);
}

#[test]
fn test_stack_peek() {
    let full_stack: List<i32> = list!(1i32, 2i32, 3i32);
    let empty_stack: List<i32> = Stack::empty();
    assert_eq!(full_stack.peek(), Some(&1));
    assert_eq!(empty_stack.peek(), None);
}

#[test]
fn test_stack_push() {
    let mut s: List<i32> = Stack::empty();
    assert_eq!(s.peek(), None);
    s = s.push(1);
    assert_eq!(s.peek(), Some(&1));
    s = s.push(6);
    assert_eq!(s.peek(), Some(&6));
}

#[test]
fn test_stack_pop() {
    let mut s: List<i32> = Stack::empty();
    assert_eq!(s.peek(), None);
    s = s.push(1);
    assert_eq!(s.peek(), Some(&1));
    s = s.push(6);
    assert_eq!(s.peek(), Some(&6));
    let pop_result = s.pop().unwrap(); // should not break
    s = pop_result.1;
    assert_eq!(s.peek(), Some(&1));
    assert_eq!(pop_result.0, 6);
}

#[test]
fn test_list_usize_indexing() {
    let l: List<isize> = list!(1,2,3,4,5,6);
    assert_eq!(l[0usize],1);
    assert_eq!(l[1usize],2);
    assert_eq!(l[2usize],3);
    assert_eq!(l[3usize],4);
    assert_eq!(l[4usize],5);
    assert_eq!(l[5usize],6);
}

#[test]
fn test_list_isize_indexing() {
    let l: List<isize> = list!(1,2,3,4,5,6);
    assert_eq!(l[0isize],1);
    assert_eq!(l[1isize],2);
    assert_eq!(l[2isize],3);
    assert_eq!(l[3isize],4);
    assert_eq!(l[4isize],5);
    assert_eq!(l[5isize],6);
}

#[test]
fn test_list_macro() {
    let l: List<i32> = list!(1i32, 2i32, 3i32);
    assert_eq!(l.to_string(), "(1, 2, 3)")
}

#[test]
fn test_list_iter() {
    let l: List<isize> = list!(1,2,3,4,5,6);
    let mut string = String::new();
    for item in l.iter() {
        string.push_str((item.to_string() + ", ").as_ref());
    }
    let slice: &str = string.as_ref(); // this is necessary because assert_eq! is weird
    assert_eq!(slice, "1, 2, 3, 4, 5, 6, ")
}
