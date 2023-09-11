use std::sync::Arc;

/*
*
* ThreadList is a pub struct that represents a singly linked list of elements of type T.
* It contains a single field, head, which is of type Link<T>.
*
* Link<T> is an enumeration that represents either None (if the link is empty)
* or a linked list node containing an element of type T and a reference to the next node.
*
* Node<T> is a struct that represents a node in the linked list,
* containing an element of type T and a reference to the next node.
*/

pub struct ThreadList<T> {
    head: Link<T>,
}

type Link<T> = Option<Arc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

/*
* The ThreadList struct has the following methods:
*
* new() creates a new, empty ThreadList<T>.
*
* prepend(elem: T) creates a new ThreadList<T> with the specified element
* as its head, and the original head as its tail.
*
* tail() creates a new ThreadList<T> with the same head as the original, but with the tail set to the original tail.
*
* head() returns the element at the head of the list, or None if the list is empty.
*
* The ThreadList struct also implements the Iterator trait,
* which allows it to be iterated over. The iter() method returns an Iter<'_, T> object,
* which can be used to iterate over the elements of the list.
*/

impl<T> ThreadList<T> {
    pub fn new() -> Self {
        ThreadList { head: None }
    }

    pub fn prepend(&self, elem: T) -> ThreadList<T> {
        ThreadList {
            head: Some(Arc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> ThreadList<T> {
        ThreadList {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

// Iter Implement
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> ThreadList<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

/*
* The Recursive Destructor for ThreadList<T> is implemented as a Drop trait implementation.
*
* When the ThreadList is dropped, it moves the current head into a mutable variable
* and then loops over each node, taking ownership of that node and letting the last
* one drop.
*
* This ensures that the linked list is properly deallocated,
* even if there are still references to the nodes.
*/
impl<T> Drop for ThreadList<T> {
    fn drop(&mut self) {
        // Move current head into mutable variable;
        let mut head = self.head.take();
        // loop over each node, if the node exists (can be unwrapped),
        // take ownership of that node, and let the last one drop
        // otherwise, we're at the end of the list, and we should stop
        while let Some(node) = head {
            if let Ok(mut node) = Arc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::ThreadList;

    #[test]
    fn basics() {
        let tl = ThreadList::new();
        assert_eq!(tl.head(), None);

        let tl = tl.prepend(1).prepend(2).prepend(3);
        assert_eq!(tl.head(), Some(&3));

        let tl = tl.tail();
        assert_eq!(tl.head(), Some(&2));

        let tl = tl.tail();
        assert_eq!(tl.head(), Some(&1));

        let tl = tl.tail();
        assert_eq!(tl.head(), None);
    }

    #[test]
    fn iter() {
        let tl = ThreadList::new().prepend(1).prepend(2).prepend(3);

        let mut iter = tl.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
