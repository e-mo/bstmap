use std::{fmt::{Display, Debug}, cmp::Ordering};
use super::action::*;

// Used to simplify remove_first and remove_last functions. 
pub enum NodePosition {
    First,
    Last,
}

pub type NodeLink<T, V> = Option<Box<Node<T, V>>>;
// Internal Node used by BstMap to structure binary tree.  
#[derive(Debug)]
pub struct Node<T: Ord + Debug, V: Debug> {
    pub key: T,
    pub value: V,
    pub left: NodeLink<T, V>,
    pub right: NodeLink<T, V>,
}


impl<'a, T: Ord + Debug, V: Debug> Node<T, V> {
    // Returns a new Node with no children. 
    pub fn new(key: T, value: V) -> Self {
        Self {
            key,
            value,
            left: None,
            right: None,
        }
    }

    // Fills passed vector with every key/value pair as owned values,  
    // consuming the BstMap.
    pub fn fill_owned_vec(self, vec: &mut Vec<(T, V)>) {
        if let Some(node) = self.left {
            node.fill_owned_vec(vec);
        }

        vec.push((self.key, self.value));

        if let Some(node) = self.right {
            node.fill_owned_vec(vec);
        }
    }

    // Fills passed vector with every key/value pair as borrowed values.
    pub fn fill_ref_vec(&'a self, vec: &mut Vec<(&'a T, &'a V)>) {
        if let Some(ref node) = self.left {
            node.fill_ref_vec(vec);
        }

        vec.push((&self.key, &self.value));

        if let Some(ref node) = self.right {
            node.fill_ref_vec(vec);
        }
    }

    // Fills passed vector with every key/value pair as mutable values.
    pub fn fill_mut_vec(&'a mut self, vec: &mut Vec<(&'a T, &'a mut V)>) {
        if let Some(ref mut node) = self.left {
            node.fill_mut_vec(vec);
        }

        vec.push((&self.key, &mut self.value));

        if let Some(ref mut node) = self.right {
            node.fill_mut_vec(vec);
        }
    }

    // Recurse function which traverses the tree until it finds the
    // proper location to insert key/value pair.
    //
    // If key already exists, old value is clobbered. 
    pub fn insert(&mut self, key: T, value: V) -> InsertAction {

        let node_link: &mut NodeLink<T, V> = match key.cmp(&self.key) {
            Ordering::Greater => &mut self.right,
            Ordering::Less => &mut self.left,
            // We match the insert key. Clobber the old value
            // and pass a None action since no Node was added.
            Ordering::Equal => {
                self.value = value;
                return InsertAction::None;
            }
        };

        // Either call recursively or insert child
        if let Some(node) = node_link {
            node.insert(key, value)
        } else {
            *node_link = Some(Box::new(Node::new(key, value)));
            InsertAction::Increment
        }
    }

    // Recurse function which traverses the tree until it finds the
    // proper location to insert key/value pair.
    //
    // If key already exists, func is called to update the existing value
    // instead of clobbering.
    pub fn insert_or<F>(&mut self, key: T, value: V, mut func: F) -> InsertAction
            where F: FnMut(&mut V) {

        let node_link: &mut NodeLink<T, V> = match key.cmp(&self.key) {
            Ordering::Greater => &mut self.right,
            Ordering::Less => &mut self.left,
            // We match the insert key. Call the provided
            // update function and pass a None action since no
            // Node was added.
            Ordering::Equal => {
                func(&mut self.value);
                return InsertAction::None;
            }
        };

        // Either call recursively or insert child
        if let Some(node) = node_link {
            node.insert_or(key, value, func)
        } else {
            *node_link = Some(Box::new(Node::new(key, value)));
            InsertAction::Increment
        }
    }

    // Returns reference to value refferred to by key. Returns None
    // if key is not found. 
    pub fn get(&self, key: T) -> Option<&V> {

        let node_link: &NodeLink<T, V> = match key.cmp(&self.key) {
            Ordering::Greater => &self.right,
            Ordering::Less => &self.left,
            // Return a reference to our value
            Ordering::Equal => return Some(&self.value),
        };

        if let Some(node) = node_link {
            node.get(key)
        } else {
            None
        }
        
    }

    // Returns mutable reference to value refferred to by key.
    // Returns None if key is not found. 
    pub fn get_mut(&mut self, key: T) -> Option<&mut V> {

        let node_link: &mut NodeLink<T, V> = match key.cmp(&self.key) {
            Ordering::Greater => &mut self.right,
            Ordering::Less => &mut self.right,
            Ordering::Equal => return Some(&mut self.value),
        };

        if let Some(node) = node_link {
            node.get_mut(key)
        } else { None }
    }

    // Returns the logical "first" key/value pair. (farthest left)
    // Recursively calls to the left until it reaches the end. 
    pub fn first_key_value(&self) -> Option<(&T, &V)> {
        if let Some(ref node) = self.left {
            node.first_key_value()
        } else {
            Some((&self.key, &self.value))
        }
    }

    // Returns the logical "last" key/value pair. (farthest right)
    // Recursively calls to the right until it reaches the end. 
    pub fn last_key_value(&self) -> Option<(&T, &V)> {
        if let Some(ref node) = self.right {
            node.last_key_value()
        } else {
            Some((&self.key, &self.value))
        }
    }

    // Recursively seeks a Node to remove. If desired Node is reached,
    // a replacement strategy is chosen based on the number of children 
    // the Node has. 
    //
    // The only complicated scenario is if the Node has two children,
    // where the chosen strategy is to find the Node's inline successor
    // to take its place. 
    pub fn remove(&mut self, key: T) -> RemoveAction<T, V> {

        let node_link: &mut NodeLink<T, V> = match key.cmp(&self.key) {
            Ordering::Greater => &mut self.right,
            Ordering::Less => &mut self.left,
            // That's us! Return the node that is going to replace us.
            Ordering::Equal => {
                return RemoveAction::UpdateNode(self.replacement_node());
            }
        };

        if let Some(node) = node_link {
            let action = node.remove(key);
            match action {
                // Just pass action along
                // Nothing to do
                RemoveAction::Return(_) => action,
                // Grab the value out of the old node
                // Replace child with new node
                // Pass along value from old node
                RemoveAction::UpdateNode(new_node) => {
                    let value = node_link.take().unwrap().value;
                    *node_link = new_node;
                    RemoveAction::Return(Some(value))
                }
            }
        } else {
            // Otherwise no match is possible
            RemoveAction::Return(None)
        }
    }

    // Seeks a Node to replace the current one. 
    fn replacement_node(&mut self) -> NodeLink<T, V> {
        match self.has_children() {
            // I am a leaf. Whoosh.
            // Replace my NodeLink with None
            (false, false) => None,
            // If I have only left or right child,
            // replace my Nodelink with one of them
            (true, false) => self.left.take(),
            (false, true) => self.right.take(),
            // I have two children and I have to
            // replace myself with my nearest successor
            (true, true) => {
                // Pick up our nodes since they no longer need to be owned by self
                let left = self.left.take();
                let mut right = self.right.take().unwrap();

                // If our right node has no left node, it is the successor
                // Move self left node to successor left node.
                // Then return successor node to replace us. 
                if right.is_successor() {
                    right.left = left;
                    Some(right)
                } 

                // Otherwise we need to go looking for the successor
                else {
                    // First call to get_successor is to the right child.
                    // All further recursive calls will be to the left child.
                    let mut replacement = right.get_successor().unwrap();
                    // We move our children over to our replacement
                    // and return the replacement
                    replacement.left = left;
                    replacement.right = Some(right);
                    Some(replacement)
                }
            }
        }
    }

    // When looking for the successor, the first node we find that
    // has no left child node is the successor.
    fn is_successor(&self) -> bool { !self.left.is_some() }

    // I am not the successor, but is my left node pointing
    // to the successor?
    //
    // Return successor if so, otherwise call recursively on
    // left node
    fn get_successor(&mut self) -> NodeLink<T, V> {
        // Safe to unwrap here
        // None is impossible (I hope)
        let left = self.left.as_mut().unwrap();
        if left.is_successor() {
            // Take the successor node, and assign self.left to
            // successor's right node if there is one.
            let mut successor = self.left.take().unwrap();
            if successor.right.is_some() { self.left = successor.right.take(); }

            // Wrap successor back up
            Some(successor)
        } 
        else {
            left.get_successor()
        }
    }

    // Remove a node at NodePosition::First or NodePosition::Last.
    pub fn remove_position(&mut self, pos: NodePosition) -> RemoveAction<T, V> {

        // Are we looking left or right?
        let node_link: &mut NodeLink<T, V> = match pos {
            NodePosition::First => &mut self.left,
            NodePosition::Last => &mut self.right, 
        };

        // If there is a node there...
        if let Some(node) = node_link {
            let action = node.remove_position(pos);
            match action {
                RemoveAction::Return(_) => action,
                RemoveAction::UpdateNode(new_node) => {
                    let value = node_link.take().unwrap().value;
                    *node_link = new_node;
                    RemoveAction::Return(Some(value))
                }
            }
        } 
        // Otherwise its us! Find and pass on our replacement.
        else { RemoveAction::UpdateNode(self.replacement_node()) }
    }

    fn has_children(&self) -> (bool, bool) {
        (self.left.is_some(), self.right.is_some())
    }
}

// trait impl

impl<T: Ord + Debug, V: Debug> Display for Node<T, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key_left = match &self.left {
            Some(node) => format!("{:?}", node.key),
            None => "None".to_string(),
        };
        let key_right = match &self.right {
            Some(node) => format!("{:?}", node.key),
            None => "None".to_string(),
        };
        let node_left = match &self.left {
            Some(node) => format!("{}", node),
            None => String::new()
        };
        let node_right = match &self.right {
            Some(node) => format!("{}", node),
            None => String::new(),
        };
        write!(f, 
               "\n\n[BSTMap::Node @ {:p}]\
                  \n      key: {:?}\
                  \n    value: {:?}\
                  \n left key: {}\
                  \nright key: {}{}{}", 
               self,
               self.key, 
               self.value, 
               key_left, 
               key_right,
               node_left,
               node_right)
    }
}
