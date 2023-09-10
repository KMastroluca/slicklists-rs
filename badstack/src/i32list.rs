use std::mem;

// Allows Us To Keep I32Node and Link Private, While Allowing
// Access To The Linked List
pub struct I32List {
    head: I32Link,
}

struct I32Node {
    elem: i32,
    next: I32Link,
}

enum I32Link {
    Empty,
    More(Box<I32Node>),
}


// Implement List
impl I32List {
    // Create A New, Empty List
    pub fn new() -> Self {
        I32List {
            head: I32Link::Empty,
        } // Return A List Pointing To An Empty link
    }

    // Push A New Link Onto The Top Of The List Stack
    pub fn push(&mut self, elem: i32) {
        // Create New Node With New Element
        let new_node = Box::new(I32Node {
            elem: elem,
            next: mem::replace(&mut self.head, I32Link::Empty), // Grab self.head, while replacing it with I32Link::Empty
        });

        self.head = I32Link::More(new_node); // Assign self.head to our new link
    }

    // Pop A Value Off The Top Of The List Stack
    pub fn pop(&mut self) -> Option<i32> {
        // Match The Value To Make Sure Theres Something To Pop Off The Stack
        // Mem::Replace This With I32Link::Empty
        match self.pop_node() {
            I32Link::Empty => None,
            I32Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }


    // Private: Pop A Node Off The Stack For Use With Custom Drop Fn
    fn pop_node(&mut self) -> I32Link {
        mem::replace(&mut self.head, I32Link::Empty)
    }
}

// Custom Drop Implementation
impl Drop for I32List {
    fn drop(&mut self) {
        // Iterate Thru Just Popping Off Every Element In The List
        while let I32Link::More(_) = self.pop_node() {}
    }
}


#[cfg(test)]
mod test {

    use super::I32List;

    #[test]
    fn basics() {
        let mut list = I32List::new();

        // Check List Empty
        assert_eq!(list.pop(), None);

        // Populate List
        list.push(1);
        list.push(2);
        list.push(3);

        // Check Normal Removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push More So Make Sure Nothings Corrupted
        list.push(4);
        list.push(5);

        // Check Normal Removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check Exaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
