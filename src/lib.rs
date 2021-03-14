pub type Child<T> = Box<Option<Node<T>>>;

#[derive(Debug)]
pub struct Node<T: std::cmp::Ord> {
    pub data: T,
    pub left: Child<T>,
    pub right: Child<T>,
}

impl<T> Node<T>
where
    T: std::cmp::Ord,
{
    pub fn new(data: T) -> Self {
        Node {
            data,
            left: Box::new(None),
            right: Box::new(None),
        }
    }
}

#[derive(Debug)]
pub struct Tree<T: std::cmp::Ord> {
    pub root: Child<T>,
}

impl<T> Tree<T>
where
    T: std::cmp::Ord,
{
    pub fn new() -> Self {
        Tree {
            root: Box::new(None),
        }
    }

    pub fn insert(&mut self, data: T) {
        fn insert_inner<U>(root: &mut Child<U>, data: U)
        where
            U: std::cmp::Ord,
        {
            match **root {
                Some(ref mut node) => {
                    if (*node).data > data {
                        insert_inner(&mut (*node).left, data);
                    } else if (*node).data < data {
                        insert_inner(&mut (*node).right, data);
                    } else {
                        return;
                    }
                }
                None => {
                    let new_node = Node::<U>::new(data);
                    **root = Some(new_node);
                }
            }
        }

        insert_inner(&mut (*self).root, data)
    }

    pub fn delete(&mut self, data: T) {
        fn delete_innner<U>(root: &mut Child<U>, data: U)
        where
            U: std::cmp::Ord,
        {
            match **root {
                Some(ref mut node) => {
                    if (*node).data > data {
                        delete_innner(&mut (*node).left, data);
                    } else if (*node).data < data {
                        delete_innner(&mut (*node).right, data);
                    } else {
                        (**root).take().map(|node| {
                            **root = *(node.right);
                            let mut t = &mut *root;

                            while let Some(ref mut node) = **t {
                                t = &mut (*node).left;
                            }

                            **t = *(node.left);
                        });
                    }
                }
                None => return,
            }
        }

        delete_innner(&mut (*self).root, data)
    }

    pub fn lookup(&mut self, data: T) -> bool {
        fn lookup_inner<U>(root: &Child<U>, data: U) -> bool
        where
            U: std::cmp::Ord,
        {
            match **root {
                Some(ref node) => {
                    if (*node).data > data {
                        lookup_inner(&(*node).left, data)
                    } else if (*node).data < data {
                        lookup_inner(&(*node).right, data)
                    } else {
                        true
                    }
                }
                None => false,
            }
        }

        lookup_inner(&(*self).root, data)
    }

    fn traverse_tree (
        root: &Child<T>,
        f: &mut std::fmt::Formatter
    ) -> std::result::Result<(), std::fmt::Error> where T: std::cmp::Ord + std::fmt::Display {
        match **root {
            Some(ref node) => {
                Tree::<T>::traverse_tree(&(*node).left, f)?;
                write!(f, "{} ", node.data)?;
                Tree::<T>::traverse_tree(&(*node).right, f)?;
                
                Ok(())
            },
            None => Ok(())
        }
    }
}

impl <T> std::fmt::Display for Tree<T> where T: std::cmp::Ord + std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Tree::<T>::traverse_tree(&self.root, f)?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_insertion() {
        let mut bst = Tree::<i32>::new();

        bst.insert(9);
        bst.insert(67);
        bst.insert(236);
        bst.insert(19);
        bst.insert(53);

        assert_eq!(format!("{}", bst), "9 19 53 67 236 ");
    }

#[test]
    fn check_deletion() {
        let mut bst = Tree::<i32>::new();

        bst.insert(89);
        bst.insert(23);
        bst.insert(25);
        bst.insert(1);
        bst.insert(15);
        bst.insert(91);
        bst.insert(123);
        bst.insert(85);
        bst.insert(30);
        bst.insert(62);
        bst.insert(84);

        assert_eq!(format!("{}", bst), "1 15 23 25 30 62 84 85 89 91 123 ");

        bst.delete(23);
        assert_eq!(format!("{}", bst), "1 15 25 30 62 84 85 89 91 123 ");

        bst.delete(25);
        assert_eq!(format!("{}", bst), "1 15 30 62 84 85 89 91 123 ");

        bst.delete(91);
        assert_eq!(format!("{}", bst), "1 15 30 62 84 85 89 123 ");
}

#[test]
    fn check_lookup() {
        let mut bst = Tree::<i32>::new();

        assert_eq!(bst.lookup(7), false);

        bst.insert(34);
        bst.insert(99);
        bst.insert(63);
        bst.insert(89);
        bst.insert(63);
        bst.insert(47);
        bst.insert(17);
        bst.insert(91);
        bst.insert(37);
        bst.insert(74);
        bst.insert(91);

        assert_eq!(format!("{}", bst), "17 34 37 47 63 74 89 91 99 ");

        assert_eq!(bst.lookup(89), true);
        assert_eq!(bst.lookup(5), false);
        assert_eq!(bst.lookup(91), true);
        assert_eq!(bst.lookup(22), false);

        bst.delete(89);
        assert_eq!(format!("{}", bst), "17 34 37 47 63 74 91 99 ");
        assert_eq!(bst.lookup(89), false);
        
        bst.insert(89);
        assert_eq!(format!("{}", bst), "17 34 37 47 63 74 89 91 99 ");
        assert_eq!(bst.lookup(89), true);
    }
}

