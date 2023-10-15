use std::{cell::RefCell, rc::Rc};

struct GlobalStack<T> {
    stack: Rc<RefCell<Vec<T>>>,
}

impl<T> GlobalStack<T> {
    fn new() -> GlobalStack<T> {
        GlobalStack {
            stack: Rc::new(RefCell::new(vec![])),
        }
    }

    fn from_vec(vec: Vec<T>) -> GlobalStack<T> {
        GlobalStack {
            stack: Rc::new(RefCell::new(vec)),
        }
    }
}

impl<T> Default for GlobalStack<T> {
    fn default() -> GlobalStack<T> {
        GlobalStack::new()
    }
}

impl<T> Clone for GlobalStack<T> {
    fn clone(&self) -> Self {
        Self {
            stack: Rc::clone(&self.stack),
        }
    }
}

/// Ok, wow, there are actually a way to have multiple owners of a data..
/// It is Rc which translates to "reference, counted", or basically a smart pointer
/// There also atomic variant : Arc, for use in multi-threading
///
/// Unsafe if used unwisely, e.g. don't crossreference stuff around
fn main() {
    let a = GlobalStack::from_vec(vec![1, 2, 3]);
    println!("a: {:?}", a.stack);
    a.stack.borrow_mut().push(4);
    let b = a.clone(); // do not actually need to be mutable
    println!("a: {:?}", a.stack);
    println!("b: {:?}", b.stack);
    b.stack.borrow_mut().push(5);
    println!("a: {:?}", a.stack);
    println!("b: {:?}", b.stack);
}
