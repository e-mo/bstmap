use std::fmt::Debug;
use super::node::NodeLink;
// This enum type is used as a return type for the recursive remove
// function used internally by Node. The returned varient tells the calling Node
// what action it should take with the data returned.  
pub enum RemoveAction<T: Ord + Debug, V: Debug> {
    // A return value has been found.
    // Pass this value along.
    Return(Option<V>),
    // A direct child node was the node
    // to be removed. Update child node
    // and pass along old Node's value as Return
    UpdateNode(NodeLink<T, V>),
}

// This enum type is used as a return type for the recursive insert  
// functions used internally by Node.  
// The returned varient tells the BstMap if it should increment its  
// len (a new node was inserted), or if a Node was simply updated and
// the len should remain unchanged. 
pub enum InsertAction {
    Increment,
    None,
}
