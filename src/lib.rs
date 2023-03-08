//! # bstmap
//!
//! A map implemented with a binary search tree.
//! 
//! # Example
//!
//! ```
//! // Implements a standard map interface
//! use bstmap::BstMap;
//!
//! let mut map = BstMap::new();
//!
//! // Insert a key/value pair
//! map.insert("ten", 10);
//! assert!(*map.get("ten").unwrap() == 10);
//!
//! // Insert a key/value pair, or
//! // get a mutable reference to existing value
//! // if key exists.
//! map.insert_or("ten", 20, |v| {
//!     *v += 1;
//! });
//! assert!(*map.get("ten").unwrap() == 11); 
//! 
//! // Remove operations return the removed value
//! let removed = map.remove("ten").unwrap();
//! assert!(removed == 11);
//! assert!(map.is_empty());
//! ```
use std::{ops::Index, fmt::{Display, Debug}};

mod iter;
mod node;
mod action;
use iter::*;
use node::*;
use action::*;

/// BstMap instance struct.  
/// Short for "Binary Search Tree Map."
#[derive(Debug)]
pub struct BstMap<T: Ord + Debug, V: Debug> {
    len: usize,
    head: Option<Node<T, V>>,
}

impl<T: Ord + Debug + Debug, V: Debug> BstMap<T, V> {
    /// Creates an empty `BstMap`.
    ///
    /// ```
    /// # use bstmap::BstMap;
    /// let map = BstMap::<&str, u8>::new();
    /// assert!(map.is_empty()); // Yup, she's empty.
    /// ```
    pub fn new() -> Self {
        Self {
            len: 0,
            head: None,
        }
    }

    /// Empties map contents.
    ///
    /// ```
    /// # use bstmap::BstMap;
    /// let mut map: BstMap<u32, u32> = BstMap::new();
    /// map.insert(16, 86);
    /// assert!(!map.is_empty()); // Not empty!
    /// map.clear();
    /// assert!(map.is_empty());  // Yup, empty again.
    /// ```
    pub fn clear(&mut self) { 
        self.len = 0;
        self.head = None;
    }

    /// Returns true if map is empty.
    pub fn is_empty(&self) -> bool { self.len == 0 }

    /// Returns number of map entries.
    pub fn len(&self) -> usize { self.len }

    /// Returns `Iterator` over contents of map   
    /// in key/value tuples `(key: &'a T, value: &'a V)`.
    ///  
    /// No guaranteed ordering.
    pub fn iter(&self) -> IterRef<T, V> {
        self.into_iter()
    }

    /// Returns mutable value `Iterator` over contents of map   
    /// in key/value tuples `(key: &'a T, value: &'a mut V)`.
    ///  
    /// No guaranteed ordering.
    pub fn iter_mut(&mut self) -> IterMut<T, V> {
        self.into_iter()
    }

    /// Inserts a key/value pair into map.
    /// If key exists, existing value is clobbered. 
    pub fn insert(&mut self, key: T, value: V) {
        match self.head {
            Some(ref mut node) => {
                // Check returned InsertAction to see if we
                // need to increment len
                if let InsertAction::Increment = node.insert(key, value) {
                    self.len += 1;
                }
            }
            // First node! 
            None => {
                self.head = Some(Node::new(key, value));
                self.len += 1;
            }
        }
    }

    /// Inserts a key/value pair into map, and  
    /// also accepts a `FnMut(&mut V)` function pointer  
    /// which is called and passed the existing value if key already exists.  
    ///  
    /// Exiting value can be mutated inside of passed function.  
    ///   
    /// ```
    /// # use bstmap::BstMap;
    /// # let mut map: BstMap<u32, u32> = BstMap::new();
    /// map.insert(10, 10);
    /// // Attempt to insert another entry with same key, new value.
    /// map.insert_or(10, 20, |v| { *v += 1; });
    /// // Value was updated to 11 inside of closure instead of 20.
    /// assert!(*map.get(10).unwrap() == 11); // Pass!
    /// ```
    pub fn insert_or<F>(&mut self, key: T, value: V, func: F) 
            where F: FnMut(&mut V) {

        match self.head {
            Some(ref mut node) => {
                // Check returned InsertAction to see if we
                // need to increment len
                if let InsertAction::Increment = node.insert_or(key, value, func) {
                    self.len += 1;
                }
            }
            // First node! 
            None => {
                self.head = Some(Node::new(key, value));
                self.len += 1;
            }
        }
    }

    /// Returns `Some(&value)` associated with key,  
    /// or `None` if key wasn't found. 
    pub fn get(&self, key: T) -> Option<&V> {
        if let Some(node) = &self.head {
            node.get(key) 
        } 
        else { None }
    }

    /// Returns `Some(&mut value)` associated with key,  
    /// or `None` if key wasn't found. 
    pub fn get_mut(&mut self, key: T) -> Option<&mut V> {
        if let Some(ref mut node) = self.head {
            node.get_mut(key) 
        } 
        else { None }
    }

    /// Returns "first" key/value pair as sorted by key.
    pub fn first_key_value(&self) -> Option<(&T, &V)> {
        if let Some(node) = &self.head {
            node.first_key_value()
        } 
        else { None }
    }

    /// Returns "last" key/value pair as sorted by key.
    pub fn last_key_value(&self) -> Option<(&T, &V)> {
        if let Some(node) = &self.head {
            node.last_key_value()
        } 
        else { None }
    }

    /// Removes entry and returns the `Some(value)` associated  
    /// with key.  
    /// Returns `None` if key wasn't found.
    pub fn remove(&mut self, key: T) -> Option<V> {
        if let Some(ref mut node) = self.head {
            // Check what action we should take with return value
            // from remove call. 
            match node.remove(key) {
                // Just a return value which may be Some or None
                RemoveAction::Return(value) => {
                    // If the value actually contains Some,
                    // decrement our len because a node was
                    // removed
                    if value.is_some() { self.len -= 1 }
                    value
                }
                // A call to update a child node which means
                // our head was the removed node. Update head with
                // passed node. 
                RemoveAction::UpdateNode(node) => {
                    self.len -= 1;
                    let old_head = self.head.take().unwrap();
                    self.head = {
                        // The node isn't boxed at the top level, so
                        // we strip the box
                        if let Some(node) = node {
                            Some(*node)
                        // Otherwise the last node was removed and
                        // the head should now point to None. 
                        } else { None }
                    };
                    Some(old_head.value)
                }
            }
        } 
        else { None }
    }

    pub fn remove_first(&mut self) -> Option<V> {
        self._remove_position(NodePosition::First)
    }

    pub fn remove_last(&mut self) -> Option<V> {
        self._remove_position(NodePosition::Last)
    }

    fn _remove_position(&mut self, position: NodePosition) -> Option<V> {
        if let Some(ref mut node) = self.head {
            // As long as we have a head, some node is going to get
            // removed in this process, so we can decrement now.
            self.len -= 1;
            match node.remove_position(position) {
                // We know value is Some because as long
                // as the list has a head node, something is
                // going to be returned, and a node muset have
                // been removed.
                RemoveAction::Return(value) => value,
                RemoveAction::UpdateNode(node) => {
                    let old_head = self.head.take().unwrap();
                    self.head = {
                        // The node isn't boxed at the top level, so
                        // we strip the box
                        if let Some(node) = node {
                            Some(*node)
                        // Otherwise the last node was removed and
                        // the head should now point to None. 
                        } else { None }
                    };
                    Some(old_head.value)
                }
            }
        } 
        // Can't remove anything if the three doesn't even have a head.
        else { None }
    }
}

// Trait Impls
impl<T: Ord + Debug + Debug, V: Debug> Index<T> for BstMap<T, V> {
    type Output = V;

    fn index(&self, key: T) -> &Self::Output {
        self.get(key).expect("no entry found for key")
    }
}

impl<T: Ord + Debug, V: Debug> Display for BstMap<T, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let node_display = match &self.head {
            Some(node) => format!("{}", node),
            None => String::new()
        };
        write!(f,
               "[BstMap @ {:p}]\
              \n      len: {}\
              \n head key: {:?}{}",
               self,               
               self.len, 
               self.head.as_ref().unwrap().key,
               node_display)
    }
}


#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn insert_and_get() {
        let mut map = BstMap::new();
        map.insert(0, 0);
        assert!(*map.get(0).unwrap() == 0);
        map.insert(1, 1);
        assert!(*map.get(1).unwrap() == 1);
        map.insert(1, 2);
        assert!(*map.get(1).unwrap() == 2);
    }

    #[test]
    fn insert_or() {
        let mut map = BstMap::new();
        map.insert(0, 0);
        assert!(*map.get(0).unwrap() == 0);
        map.insert_or(0, 10, |v| { *v += 1; });
        assert!(*map.get(0).unwrap() == 1);
    }

    #[test]
    fn clear() {
        let mut map = BstMap::new();
        map.insert(0, 0);
        map.insert(1, 0);
        map.insert(2, 0);
        map.insert(3, 0);
        map.insert(4, 0);
        map.insert(5, 0);
        map.insert(6, 0);
        map.insert(7, 0);
        assert!(!map.is_empty());
    }

    #[test]
    fn get_mut() {
        let mut map = BstMap::new();
        map.insert(0, 0);
        let val = map.get_mut(0).unwrap();
        *val += 1;
        assert!(*map.get(0).unwrap() == 1);
    }

    #[test]
    fn first_last_key_value() {
        let mut map = BstMap::new();
        map.insert(2, 2);
        map.insert(1, 1);
        map.insert(3, 3);
        let (key, value) = map.first_key_value().unwrap();
        assert!(*key == 1 && *value == 1); 
        let (key, value) = map.last_key_value().unwrap();
        assert!(*key == 3 && *value == 3); 
    }

    #[test]
    fn remove() {
        let mut map = BstMap::new();
        map.insert(10, "head");
        map.insert(14, "last removed");
        map.insert(13, "new head");
        map.insert(16, "last");
        map.insert(15, "second last");
        map.insert(1, "first");
        map.insert(3, "third first");
        map.insert(2, "second first");

        let value = map.remove(99);
        assert!(value.is_none());

        let value = map.remove(10).unwrap();
        assert!(value == "head");

        let value = map.remove_first().unwrap();
        assert!(value == "first");

        let value = map.remove_first().unwrap();
        assert!(value == "second first");

        let value = map.remove_first().unwrap();
        assert!(value == "third first");

        let value = map.remove_first().unwrap();
        assert!(value == "new head");

        let value = map.remove_last().unwrap();
        assert!(value == "last");

        let value = map.remove_last().unwrap();
        assert!(value == "second last");

        let value = map.remove_last().unwrap();
        assert!(value == "last removed");

        assert!(map.is_empty());
    }
}
