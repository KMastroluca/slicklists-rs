# Slicklist-rs: A Collection of Learning Experiences 🎓🦀

[![Security Audit](https://github.com/KMastroluca/slicklists-rs/actions/workflows/security-audit.yml/badge.svg?branch=main)](https://github.com/KMastroluca/slicklists-rs/actions/workflows/security-audit.yml)

[![Style & Formatting](https://github.com/KMastroluca/slicklists-rs/actions/workflows/style-formatting.yml/badge.svg)](https://github.com/KMastroluca/slicklists-rs/actions/workflows/style-formatting.yml)

Welcome to **Slicklist-rs**, a repository showcasing my journey through the world of Rust and data structures. This project is primarily a CV piece, demonstrating my abilities as a programmer. However, it is important to note that the data structures presented here are not intended for practical use.

Feel free to explore the various linked lists and other data structures I've created, such as the **BadStack** (a.k.a. **I32List**). While I doubt you'll find these creations particularly useful, if you do happen to stumble upon something of interest, I encourage you to fork this repository and use the code however you see fit.

Remember, this project is a testament to my learning experience, and as such, it may not be the most polished or efficient. But who knows? You might just find a hidden gem amidst the chaos. Happy exploring! 🚀

---

## The BadStack (a.k.a. I32List) 📚🔗

This is a simple linked list data structure in Rust, serving as a learning experience and a display of my programming abilities. However, it's far from perfect and not intended for practical use.

### Features? 🌟

The BadStack is a single-linked list storing `i32` values, with the following features:

- A `new()` function to create an empty list.
- A `push()` function to add elements to the list.
- A `pop()` function to remove elements from the list.
- A custom `Drop` implementation to clean up the list when it goes out of scope.

### Problems! 😅

The BadStack has its share of issues:

1. The `I32Link` enum has two variants: `Empty` and `More(Box<I32Node>)`, leading to allocating a node that just says "I'm not actually a Node. In other words, its just a bad implementation of Option. Which i use in later lists.
2. The implementation is inefficient and impractical, as it uses `Box` for heap allocation.
3. The BadStack lacks iterators, making it difficult to traverse the list.

This data structure is just one of several in this series, and it's important to note that it's not meant for actual use. It's a learning experience and a testament to my journey as a programmer.

---

# RefList

A simple linked list implementation using `Rc` in Rust.

## Structs

- `RefList<T>`: A linked list with elements of type `T`.
- `Node<T>`: A node in the list containing a reference to the element and a reference to the next node in the list.

## Methods

### RefList

- `new() -> Self`: Create a new empty list.
- `prepend(&self, elem: T) -> RefList<T>`: Add a node with a reference to the next node to the beginning of the list.
- `tail(&self) -> RefList<T>`: Get a reference to the tail of the list.
- `head(&self) -> Option<&T>`: Get the first element of the list, if it exists.

### Iter

- `iter(&self) -> Iter<'_, T>`: Create a new iterator for the list.

## Traits

### Iterator for Iter

- `next(&mut self) -> Option<Self::Item>`: Return the next element in the list and update the next field accordingly.

## Tests

- `basics()`: Test basic functionality of the RefList.
- `iter()`: Test the iterator for the RefList.

---

# ThreadList Documentation

## Overview

The `ThreadList<T>` struct represents a singly linked list of elements of type `T`. It contains a single field, `head`, which is of type `Link<T>`. It is a thread-safe data structure due to its use of `std::sync::Arc` for shared ownership of nodes.

```rust 
pub struct ThreadList<T> { head: Link<T>, }
```

## Data Structures

The `Link<T>` is an enumeration that represents either `None` (if the link is empty) or a linked list node containing an element of type `T` and a reference to the next node.

```rust 
type Link<T> = Option<Arc<Node<T>>>;```


The `Node<T>` struct represents a node in the linked list, containing an element of type `T` and a reference to the next node.

```rust 
struct Node<T> { elem: T, next: Link<T>, }```

## Methods

The `ThreadList` struct has several methods:

- `new()`: creates a new, empty `ThreadList<T>`.

```rust 
pub fn new() -> Self { ThreadList { head: None } }```

- `prepend(elem: T)`: creates a new `ThreadList<T>` with the specified element as its head, and the original head as its tail.

```rust 
pub fn prepend(&self, elem: T) -> ThreadList<T> { 
    ThreadList { 
        head: Some(Arc::new(Node { elem, next: self.head.clone(), })), 
    } 
}```

- `tail()`: creates a new `ThreadList<T>` with the same head as the original, but with the tail set to the original tail.

```rust 
pub fn tail(&self) -> ThreadList<T> { 
    ThreadList { 
        head: self.head.as_ref().and_then(|node| node.next.clone()), 
    } 
}```


- `head()`: returns the element at the head of the list, or `None` if the list is empty.

```rust pub fn head(&self) -> Option<&T> { 
    self.head.as_ref().map(|node| &node.elem) 
}```

## Iterator Implementation

The `ThreadList` struct implements the `Iterator` trait, which allows it to be iterated over. The `iter()` method returns an `Iter<'_, T>` object, which can be used to iterate over the elements of the list.

```rust 
impl<T> ThreadList<T> {
     pub fn iter(&self) -> Iter<'_, T> { 
        Iter { next: self.head.as_deref(), } 
     } 
}```

The `Iter<'a, T>` struct, and the `Iterator` trait implementation for `Iter<'a, T>`:

```rust 
pub struct Iter<'a, T> { next: Option<&'a Node<T>>, }

impl<'a, T> Iterator for Iter<'a, T> { 
    type Item = &'a T

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}```

## Recursive Destructor

When the `ThreadList` is dropped, it moves the current head into a mutable variable and then loops over each node, taking ownership of that node and letting the last one drop. This ensures that the linked list is properly deallocated, even if there are still references to the nodes.


```rust

impl<T> Drop for ThreadList<T> { 
    fn drop(&mut self) { let mut head = self.head.take(); 
    while let Some(node) = head {
         if let Ok(mut node) = Arc::try_unwrap(node) { head = node.next.take(); } else { break; } 
         } 
    }
}

```

---

# DubDeque

`DubDeque` is a double-ended queue (deque) implemented in Rust.

## Structure

The primary structure is `DubDeque<T>`, which consists of a `head` and `tail` that are of type `Link<T>`. `Link<T>` is an alias for `Option<Rc<RefCell<Node<T>>>>`.

The `Node<T>` structure represents each element in the deque. It holds an element of type `T` and pointers to the next and previous nodes.

## Methods

### `new() -> Self`

Creates a new `DubDeque` with `head` and `tail` initialized to `None`.

### `push_front(&mut self, elem: T)`

Adds an element to the front of the deque. If the deque is not empty, this method connects the old head to the new element. If the deque is empty, the new element becomes both the `head` and `tail`.

### `pop_front(&mut self) -> Option<T>`

Removes and returns the front element of the deque. If the deque is not empty, it sets the next element as the new head. If the deque is empty, it returns `None`.

### `peek_front(&self) -> Option<Ref<T>>`

Returns a reference to the front element of the deque, or `None` if the deque is empty.

### `push_back(&mut self, elem: T)`

Adds an element to the back of the deque. If the deque is not empty, this method connects the old tail to the new element. If the deque is empty, the new element becomes both the `head` and `tail`.

### `pop_back(&mut self) -> Option<T>`

Removes and returns the back element of the deque. If the deque is not empty, it sets the previous element as the new tail. If the deque is empty, it returns `None`.

### `peek_back(&self) -> Option<Ref<T>>`

Returns a reference to the back element of the deque, or `None` if the deque is empty.

### `peek_back_mut(&mut self) -> Option<RefMut<T>>`

Returns a mutable reference to the back element of the deque, or `None` if the deque is empty.

### `peek_front_mut(&mut self) -> Option<RefMut<T>>`

Returns a mutable reference to the front element of the deque, or `None` if the deque is empty.

## Iteration

The `DubDeque` structure also provides an `into_iter(self) -> IntoIter<T>` method which returns an `IntoIter<T>` object for iterating over the deque. `IntoIter<T>` implements the `Iterator` trait, allowing elements to be accessed using the `next(&mut self) -> Option<Self::Item>` method. `IntoIter<T>` also implements the `DoubleEndedIterator` trait, providing the `next_back(&mut self) -> Option<Self::Item>` method for reversed iteration.

## Drop Trait

The `Drop` trait is implemented for `DubDeque<T>`. This ensures that all nodes are popped out when a `DubDeque<T>` object goes out of scope.

## Testing

The module includes tests for basic operations such as `push_front`, `pop_front`, and iteration via `into_iter`.


---

# ToughList

`ToughList` is a singly-linked list implemented in Rust.

## Structure

The primary structure is `ToughList<T>`, which consists of a `head` that is of type `Link<T>`. `Link<T>` is an alias for `Option<Box<Node<T>>>`.

The `Node<T>` structure represents each element in the list. It holds an element of type `T` and a pointer to the next node.

## Methods

### `new() -> Self`

Creates a new `ToughList` with `head` initialized to `None`.

### `push(&mut self, elem: T)`

Adds an element to the top of the list. If the list is not empty, this method connects the current head to the new element. If the list is empty, the new element becomes the new head.

### `pop(&mut self) -> Option<T>`

Removes and returns the top element of the list. If the list is not empty, it sets the next element as the new head. If the list is empty, it returns `None`.

### `peek(&self) -> Option<&T>`

Returns a reference to the top element of the list, or `None` if the list is empty.

### `peek_mut(&mut self) -> Option<&mut T>`

Returns a mutable reference to the top element of the list, or `None` if the list is empty.

## Iteration

The `ToughList` structure provides an `into_iter(self) -> IntoIter<T>` method which returns an `IntoIter<T>` object for iterating over the list. `IntoIter<T>` implements the `Iterator` trait, allowing elements to be accessed using the `next(&mut self) -> Option<Self::Item>` method. `ToughList` also provides `iter(&self) -> Iter<'_, T>` and `iter_mut(&mut self) -> IterMut<'_, T>` methods for creating iterators over the list.

## Drop Trait

The `Drop` trait is implemented for `ToughList<T>`. This ensures that all nodes are popped off when a `ToughList<T>` object goes out of scope.

## Testing

The module includes tests for basic operations such as `push`, `pop`, `peek`, and iteration using `into_iter`, `iter`, and `iter_mut`.

---

# UnsafeList

`UnsafeList` is a singly-linked list implemented in Rust using unsafe operations.

## Structure

The primary structure is `UnsafeList<T>`, which consists of a `head` of type `Link<T>` and a `tail` of type `*mut Node<T>`. `Link<T>` is an alias for `*mut Node<T>`.

The `Node<T>` structure represents each element in the list. It holds an element of type `T` and a pointer to the next node.

## Methods

### `new() -> Self`

Creates a new `UnsafeList` with `head` and `tail` initialized to null pointers.

### `push(&mut self, elem: T)`

Adds an element to the top of the list. This method uses unsafe operations to create a new node and update the pointers accordingly.

### `pop(&mut self) -> Option<T>`

Removes and returns the top element of the list. This method uses unsafe operations to free the memory of the popped node and update the pointers.

### `peek(&self) -> Option<&T>`

Returns a reference to the top element of the list, or `None` if the list is empty. This method uses unsafe operations to access the element without modifying the list.

### `peek_mut(&mut self) -> Option<&mut T>`

Returns a mutable reference to the top element of the list, or `None` if the list is empty. This method uses unsafe operations to access the element without modifying the list.

## Iteration

The `UnsafeList` structure provides an `into_iter(self) -> IntoIter<T>` method which returns an `IntoIter<T>` object for iterating over the list. `IntoIter<T>` implements the `Iterator` trait, allowing elements to be accessed using the `next(&mut self) -> Option<Self::Item>` method. `UnsafeList` also provides `iter(&self) -> Iter<'_, T>` and `iter_mut(&mut self) -> IterMut<'_, T>` methods for creating iterators over the list.

## Drop Trait

The `Drop` trait is implemented for `UnsafeList<T>`. This ensures that all nodes are popped off when an `UnsafeList<T>` object goes out of scope.

## Testing

The module includes tests for basic operations such as `push`, `pop`, `peek`, and iteration using `into_iter`, `iter`, and `iter_mut`.

---

