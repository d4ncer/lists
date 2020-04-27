use std::mem;

pub struct List<T> {
    head: Link<T>,
}

enum Link<T> {
    Nil,
    Cons(Box<Node<T>>),
}

struct Node<T> {
    value: T,
    next: Link<T>,
}

impl List<i32> {
    pub fn new() -> Self {
        List { head: Link::Nil }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            value: elem,
            next: mem::replace(&mut self.head, Link::Nil),
        };

        self.head = Link::Cons(Box::new(new_node));
    }
}
