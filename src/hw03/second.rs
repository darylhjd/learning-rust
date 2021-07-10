// Binary Search Tree second implementation
type Link<T> = Option<Box<Node<T>>>;

trait InsertSearch<T> {
    fn insert_value(&mut self, e: T) -> bool;
    fn search_value(&self, e: T) -> bool;
}

#[derive(Debug)]
struct Node<T> {
    element: T,
    left: Link<T>,
    right: Link<T>,
}

#[derive(Debug)]
pub struct BST<T> {
    root: Link<T>,
}

impl<T> BST<T>
    where T: Ord {
    // Create a new BST.
    pub fn new() -> Self {
        return BST {
            root: None,
        };
    }

    // Insert a element into the binary search_value tree.
    pub fn insert(&mut self, element: T) -> bool {
        return self.root.insert_value(element);
    }

    // Search whether an element is already inside the binary search_value tree.
    pub fn search(&self, element: T) -> bool {
        return self.root.search_value(element);
    }
}

impl<T> InsertSearch<T> for Link<T>
    where T: Ord {
    fn insert_value(&mut self, element: T) -> bool {
        return match self {
            None => {
                *self = Some(Box::new(
                    Node {
                        element,
                        left: None,
                        right: None,
                    }
                ));
                true
            }
            Some(b) => {
                if b.element == element {
                    false
                } else if element < b.element {
                    b.left.insert_value(element)
                } else {
                    b.right.insert_value(element)
                }
            }
        };
    }

    fn search_value(&self, element: T) -> bool {
        return match self {
            None => false,
            Some(b) => {
                if b.element == element {
                    true
                } else if element < b.element {
                    b.left.search_value(element)
                } else {
                    b.right.search_value(element)
                }
            }
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bst() {
        let mut bst = BST::new();
        assert_eq!(bst.search(4), false);

        bst.insert(2);
        bst.insert(3);
        bst.insert(4);
        assert_eq!(bst.search(4), true);

        let mut bst_letter = BST::new();
        assert_eq!(bst_letter.search('z'), false);

        bst_letter.insert('a');
        bst_letter.insert('b');
        bst_letter.insert('z');
        assert_eq!(bst_letter.search('z'), true);
    }
}