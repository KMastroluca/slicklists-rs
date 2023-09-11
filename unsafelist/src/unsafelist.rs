use std::mem;
use std::ptr;

pub struct UnsafeList<T> {
    head: Link<T>,
    tail: *mut Node<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem:T,
    next: Link<T>
}


impl<T> UnsafeList<T> {

    pub fn new() -> Self {
        UnsafeList { head: None, tail: ptr::null_mut() }
    }

    pub fn push(&mut self, elem:T) {

        let mut new_tail = Box::new(Node {
            elem:elem,
            next:None
        });

        // Corece a mutable reference to a raw pointer
        let raw_tail: *mut _ =  &mut *new_tail;

        // Check for a null pointer
        if !self.tail.is_null() {
            // if the old tail exists, update it to point to the new tail
            unsafe { // Telling the compiler that we arent wearing any condoms before we put it in
                (*self.tail).next = Some(new_tail);
            }
        } else {
            // otherwise, update the head to point to it
            self.head = Some(new_tail);
        }

        self.tail = raw_tail;
    }


    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {

            let head = *head; // Dereference Head
            self.head = head.next;

            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            head.elem

        } )
    }


}


#[cfg(test)]
mod test {

    use super::UnsafeList;

    #[test]
    fn basics() {

        let mut list = UnsafeList::new();

        // Test Empty List Behaves Correctly
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        // Check Normal Removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push More Values to Test Possible Corruptions
        list.push(4);
        list.push(5);

        // Check Removal And Exaustion
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);


        // Check That We Can Bounce Back From Nothing
        list.push(6);
        list.push(7);


        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);

    }

}