use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let a = Rc::new(RefCell::new(5)); // Combine `Rc` and `RefCell`

    let b = Rc::clone(&a);
    *a.borrow_mut() = 10; // Modify the value through `RefCell`

    println!("a: {}", a.borrow()); // Prints "a: 10"
    println!("b: {}", b.borrow()); // Prints "b: 10" since `Rc` shares the reference
}

