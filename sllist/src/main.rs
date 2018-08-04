use std::ptr;

struct SLList<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> SLList<T> {
    fn new() -> Self {
        SLList { head: None, tail: ptr::null_mut() }
    }

    fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);

        if self.tail.is_null() {
            self.tail = ptr::null_mut();
        }
    }

    fn add(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem: elem,
            next: None,
        });

        let raw_tail: *mut _ = &mut *new_tail;

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }

        self.tail = raw_tail;
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            let node = *n;
            self.head = node.next;

            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            node.elem
        })
    }
}

fn main() {}

mod test {
    use super::SLList;
    #[test]
    fn pop() {
        let mut list = SLList::new();

        list.push(1);
        list.push(2);

        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn add() {
        let mut list = SLList::new();

        list.add(1);
        list.add(2);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), None);
    }
}
