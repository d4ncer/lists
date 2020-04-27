use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Nil,
    Cons(Box<Node>),
}

struct Node {
    value: i32,
    next: Link,
}

impl List {
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

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Nil) {
            Link::Nil => Option::None,
            Link::Cons(node) => {
                self.head = node.next;
                Option::Some(node.value)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Nil);
        while let Link::Cons(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Nil);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), Option::None);

        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);

        assert_eq!(list.pop(), Option::Some(4));
        assert_eq!(list.pop(), Option::Some(3));
        assert_eq!(list.pop(), Option::Some(2));
        assert_eq!(list.pop(), Option::Some(1));

        assert_eq!(list.pop(), Option::None);
        assert_eq!(list.pop(), Option::None);
    }
}
