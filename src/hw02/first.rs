// Binary Search Tree first implementation
#[derive(Debug)]
enum Link {
    More(Box<Node>),
    Empty,
}

#[derive(Debug)]
struct Node {
    element: i32,
    left: Link,
    right: Link,
}

#[derive(Debug)]
pub struct BST {
    root: Link,
}

impl BST {
    // Create a new BST.
    pub fn new() -> Self {
        return BST {
            root: Link::Empty,
        };
    }

    // Insert a element into the binary search tree.
    pub fn insert(&mut self, element: i32) -> bool {
        return self.root.insert(element);
    }

    // Search whether an element is already inside the binary search tree.
    pub fn search(&self, element: i32) -> bool {
        return self.root.search(element);
    }
}

impl Link {
    fn insert(&mut self, element: i32) -> bool {
        return match self {
            Self::Empty => {
                *self = Self::More(Box::new(
                    Node {
                        element,
                        left: Link::Empty,
                        right: Link::Empty,
                    }
                ));
                true
            },
            Self::More(b) => {
                if b.element == element {
                    false
                } else if element < b.element {
                    b.left.insert(element)
                } else {
                    b.right.insert(element)
                }
            }
        };
    }

    fn search(&self, element: i32) -> bool {
        return match self {
            Self::Empty => false,
            Self::More(b) => {
                if b.element == element {
                    true
                } else if element < b.element {
                    b.left.search(element)
                } else {
                    b.right.search(element)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bst() {
        let mut bst = BST::new();
        assert_eq!(bst.search(3), false);

        bst.insert(1);
        bst.insert(2);
        bst.insert(3);
        assert_eq!(bst.search(3), true);
    }
}