#![allow(dead_code)]
mod min_heap;
use min_heap::MinHeap;

fn main() {
    let mut h = MinHeap::empty(10);
    h.push(16);
    h.push(5);
    h.push(-3);
    println!("{:?}",h.poll().unwrap());
    println!("{:?}",h.poll().unwrap());
    println!("{:?}",h.poll().unwrap());
    println!("{:?}",h.peek());
}

