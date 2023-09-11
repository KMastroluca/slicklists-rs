# Slicklist-rs: A Collection of Learning Experiences 🎓🦀

Welcome to **Slicklist-rs**, a repository showcasing my journey through the world of Rust and data structures. This project is primarily a CV piece, demonstrating my abilities as a programmer. However, it is important to note that the data structures presented here are not intended for practical use.

Feel free to explore the various linked lists and other data structures I've created, such as the **BadStack** (a.k.a. **I32List**). While I doubt you'll find these creations particularly useful, if you do happen to stumble upon something of interest, I encourage you to fork this repository and use the code however you see fit.

Remember, this project is a testament to my learning experience, and as such, it may not be the most polished or efficient. But who knows? You might just find a hidden gem amidst the chaos. Happy exploring! 🚀

---

## The BadStack (a.k.a. I32List) 📚🔗

Welcome to the **BadStack**, or the **I32List**. This is a simple linked list data structure in Rust, serving as a learning experience and a display of my programming abilities. However, it's far from perfect and not intended for practical use.

### Features 🌟

The BadStack is a single-linked list storing `i32` values, with the following features:

- A ```rust new()``` function to create an empty list[1].
- A `rust push()` function to add elements to the list[1].
- A `rust pop()` function to remove elements from the list[1].
- A custom `Drop` implementation to clean up the list when it goes out of scope[1].

### Problems 😅

The BadStack has its share of issues:

1. The `I32Link` enum has two variants: `Empty` and `More(Box<I32Node>)`, leading to allocating a node that just says "I'm not actually a Node"[3].
2. The implementation is inefficient and impractical, as it uses `Box` for heap allocation[5].
3. The BadStack lacks iterators, making it difficult to traverse the list.

This data structure is just one of several in this series, and it's important to note that it's not meant for actual use. It's a learning experience and a testament to my journey as a programmer.