use std::sync::atomic::AtomicPtr;
use std::sync::atomic::AtomicI32;
use std::ptr;

// A simple function to demonstrate module functionality
pub fn print_hello() {
    println!("Hello from linked_list module!");
}

pub struct Node<T> {
    pub data: T, 
    
    // AtomicPtr holds a raw pointer and provides atomic CAS operations.
    pub next: AtomicPtr<Node<T>>,

    // AtomicI32 is the idiomatic Rust equivalent for thread-safe integer state.
    pub state: AtomicI32,
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Node {
            data: val,
            next: AtomicPtr::new(ptr::null_mut()),
            state: AtomicI32::new(0),
        }
    }

    pub fn empty() -> Self 
    where 
        T: Default,
    {
        Node {
            data: T::default(),
            next: AtomicPtr::new(ptr::null_mut()), 
            state: AtomicI32::new(0),
        }
    }
}