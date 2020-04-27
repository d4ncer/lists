pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: Link::None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Node {
            value: elem,
            next: self.head.take(),
        };

        self.head = Link::Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.value)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
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
        assert_eq!(list.peek(), Option::Some(&1));
        list.peek_mut().map(|value| *value = 42);
        assert_eq!(list.peek(), Option::Some(&42));
        list.push(2);
        list.push(3);
        list.push(4);

        assert_eq!(list.pop(), Option::Some(4));
        assert_eq!(list.pop(), Option::Some(3));
        assert_eq!(list.pop(), Option::Some(2));
        assert_eq!(list.pop(), Option::Some(42));

        assert_eq!(list.pop(), Option::None);
        assert_eq!(list.pop(), Option::None);
    }
}
