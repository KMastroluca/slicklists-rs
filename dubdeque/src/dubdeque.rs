
use std::rc::Rc;
use std::cell::{Ref, RefCell, RefMut};

pub struct DubDeque<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>
}


impl<T> Node<T> {
    fn new(elem:T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            next: None,
            prev: None
        }))
    }
}

impl<T> DubDeque<T> {
    pub fn new() -> Self {
        DubDeque { head: None, tail: None }
    }

    pub fn push_front(&mut self, elem: T) {

        let new_head = Node::new(elem);

        match self.head.take() {
            Some(old_head) => {
                // Non empty list, need to connect the old head
                old_head.borrow_mut().prev = Some(new_head.clone()); // +1 New head
                new_head.borrow_mut().next = Some(old_head); // +1 Old head
                self.head = Some(new_head); // +1 new head -1 old head
            }
            None => {
                // Empty List, Need To Set The Tail
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }

    }


    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }



    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }


    pub fn push_back(&mut self, elem: T) {

        let new_tail = Node::new(elem);

        match self.tail.take() {
            Some(old_tail) => {
                // Non empty list, need to connect the old head
                old_tail.borrow_mut().next = Some(new_tail.clone()); // +1 New head
                new_tail.borrow_mut().prev = Some(old_tail); // +1 Old head
                self.tail = Some(new_tail); // +1 new head -1 old head
            }
            None => {
                // Empty List, Need To Set The Tail
                self.tail = Some(new_tail.clone());
                self.head = Some(new_tail);
            }
        }

    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    } 

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    } 

}


impl<T> Drop for DubDeque<T> {
    fn drop(&mut self) { // Pop all the nodes out!
        while self.pop_front().is_some() {}
    }
}



#[cfg(test)]
mod test {
    use super::DubDeque;



    #[test]
    fn basics() {
        let mut dd = DubDeque::new();

        // Check Empty List Behaves Right
        assert_eq!(dd.pop_front(), None);

        // Populate list
        dd.push_front(1);
        dd.push_front(2);
        dd.push_front(3);

        // Check Normal Removal
        assert_eq!(dd.pop_front(), Some(3));
        assert_eq!(dd.pop_front(), Some(2));

        // Push More To Make Sure Nothings Corrupted
        dd.push_front(4);
        dd.push_front(5);

        // Check Removal Again
        assert_eq!(dd.pop_front(), Some(5));
        assert_eq!(dd.pop_front(), Some(4));

        // Check Exaustion
        assert_eq!(dd.pop_front(), Some(1));
        assert_eq!(dd.pop_front(), None);

    }

}