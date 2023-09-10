use std::rc::Rc;


pub struct RefList<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>
}


impl<T> RefList<T> {


    pub fn new() -> Self {
        RefList { head: None }
    }

    pub fn prepend(&self, elem:T) -> RefList<T> {
        RefList {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone()
            }))
        }
    }

    pub fn tail(&self) -> RefList<T> {
        RefList { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

}


// Iter Implement
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> RefList<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
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

// Recursive Destructor
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