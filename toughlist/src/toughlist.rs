use std::mem;

// NewType Wrapper Around ToughList For IntoIter Implementation
pub struct IntoIter<T>(ToughList<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct ToughList<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}


// Implement List
impl<T> ToughList<T> {
    // Create A New, Empty List
    pub fn new() -> Self {
        ToughList { head: None } // Return A List Pointing To An Empty link
    }

    // Push A New Link Onto The Top Of The List Stack
    pub fn push(&mut self, elem: T) {
        // Create New Node With New Element
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(), // Grab self.head, while replacing it with I32Link::Empty
        });

        self.head = Some(new_node); // Assign self.head to our new link
    }

    // Pop A Value Off The Top Of The List Stack
    pub fn pop(&mut self) -> Option<T> {
        self.pop_node().take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    // Private: Pop A Node Off The Stack For Use With Custom Drop Fn
    fn pop_node(&mut self) -> Link<T> {
        self.head.take()
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    // IntoIter Implement
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

// Custom Drop Implementation
impl<T> Drop for ToughList<T> {
    fn drop(&mut self) {
        // Iterate Thru Just Popping Off Every Element In The List
        while let Some(_) = self.pop_node() {}
    }
}

// IntoIter Iterator Implementation
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// Iter Implementation
impl<T> ToughList<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        // self.head.as_deref() ------------------v
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            // Which Is The Same as node.next.as_deref()
            self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {

    use super::ToughList;

    #[test]
    fn test_toughlist() {
        let mut tl = ToughList::new();

        tl.push(1);
        tl.push(2);
        tl.push(3);

        assert_eq!(tl.pop(), Some(3));
        assert_eq!(tl.pop(), Some(2));

        tl.push(4);
        tl.push(5);

        assert_eq!(tl.pop(), Some(5));
        assert_eq!(tl.pop(), Some(4));

        assert_eq!(tl.pop(), Some(1));
        assert_eq!(tl.pop(), None);
    }

    #[test]
    fn test_toughlist_peek() {
        let mut tl = ToughList::new();
        assert_eq!(tl.peek(), None);
        assert_eq!(tl.peek_mut(), None);

        tl.push(1);
        tl.push(2);
        tl.push(3);

        assert_eq!(tl.peek(), Some(&3));
        assert_eq!(tl.peek_mut(), Some(&mut 3));

        tl.peek_mut().map(|value| {
            *value = 100;
        });

        assert_eq!(tl.peek(), Some(&100));
        assert_eq!(tl.pop(), Some(100));
    }

    #[test]
    fn test_intoiter() {
        let mut tl = ToughList::new();

        tl.push(1);
        tl.push(2);
        tl.push(3);

        let mut iter = tl.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter() {
        let mut tl = ToughList::new();

        tl.push(1);
        tl.push(2);
        tl.push(3);

        let mut iter = tl.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
