use std::rc::Rc;

/*
The RefList struct has the following methods:

new(): Create a new empty list.
prepend(elem: T) -> RefList<T>: Add a node with a reference to the next node to the beginning of the list.
tail() -> RefList<T>: Get a reference to the tail of the list.
head() -> Option<&T>: Get the first element of the list, if it exists.
The Iter struct is an iterator for iterating over the elements of the list. It has a single field, next, which is a reference to the next node in the list. The iter() method creates a new iterator for the list.

The Iterator for Iter implements the next() method, which returns the next element in the list and updates the next field accordingly.

*/

// A simple linked list implementation using Rc
pub struct RefList<T> {
    head: Link<T>,
}

// A type representing a connection between nodes in the list, using the Rc trait
type Link<T> = Option<Rc<Node<T>>>;

// A struct representing a node in the list,
// containing a reference to the element and a reference to the next node in the list
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> RefList<T> {
    // Create a new empty list
    pub fn new() -> Self {
        RefList { head: None }
    }

    // Add a node with a reference to the next node to the beginning of the list
    pub fn prepend(&self, elem: T) -> RefList<T> {
        RefList {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        }
    }

    // Add a node with a reference to the next node to the end of the list
    pub fn tail(&self) -> RefList<T> {
        RefList {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    // Return a reference to the head of the current list
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

// Implementation of the Iter style iterator
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

// Implement the Iterator trait for the RefList iterator
impl<T> RefList<T> {
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

// Implementation of the RefList drop trait
// which uses a custom recursive destructor pattern
impl<T> Drop for RefList<T> {
    fn drop(&mut self) {
        // Move current head into mutable variable;
        let mut head = self.head.take();
        // loop over each node, if the node exists (can be unwrapped),
        // take ownership of that node, and let the last one drop
        // otherwise, we're at the end of the list, and we should stop
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::RefList;

    #[test]
    fn basics() {
        let rl = RefList::new();
        assert_eq!(rl.head(), None);

        let rl = rl.prepend(1).prepend(2).prepend(3);
        assert_eq!(rl.head(), Some(&3));

        let rl = rl.tail();
        assert_eq!(rl.head(), Some(&2));

        let rl = rl.tail();
        assert_eq!(rl.head(), Some(&1));

        let rl = rl.tail();
        assert_eq!(rl.head(), None);
    }

    #[test]
    fn iter() {
        let rl = RefList::new().prepend(1).prepend(2).prepend(3);

        let mut iter = rl.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
