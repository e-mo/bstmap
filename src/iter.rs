use std::fmt::Debug;
use crate::BstMap;

// Iterator implementations for BstMap.
// All iterators are constructed by filling a vector with key/value 
// pairs from the map. 
//
// All three flavors of iterator can be constructed, including destructive. 

// Owned Iterator
pub struct Iter<T: Ord + Debug, V: Debug> {
    pairs: Vec<(T, V)>,
}

impl<T: Ord + Debug, V: Debug> Iterator for Iter<T, V> {
    type Item = (T, V);
    fn next(&mut self) -> Option<Self::Item> {
        if self.pairs.len() == 0 { None } 
        else {
            let pair = self.pairs.swap_remove(0);
            Some(pair)
        }
    }
}

// Reference Iterator
pub struct IterRef<'a, T: Ord + Debug, V: Debug> {
    pairs: Vec<(&'a T, &'a V)>,
    index: usize,
    len: usize,
}

impl<'a, T: Ord + Debug, V: Debug> Iterator for IterRef<'a, T, V> {
    type Item = (&'a T, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.len { None } 
        else {
            let pair = self.pairs[self.index];
            self.index += 1;
            Some(pair)
        }
    }
}

// Mutable Iterator
pub struct IterMut<'a, T: Ord + Debug, V: Debug> {
    pairs: Vec::<(&'a T, &'a mut V)>,
}

impl<'a, T: Ord + Debug, V: Debug> Iterator for IterMut<'a, T, V> {
    type Item = (&'a T, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pairs.len() == 0 { None }
        else {
            let pair = self.pairs.swap_remove(0);
            Some(pair)
        }
    }
}

// IntoIterator impl for three states of BstMap

impl<T: Ord + Debug + Debug, V: Debug> IntoIterator for BstMap<T, V> {
    type Item = (T, V);
    type IntoIter = Iter<T, V>;

    fn into_iter(self) -> Self::IntoIter {
        let mut pairs = Vec::new();
        if self.head.is_some() {
            self.head.unwrap().fill_owned_vec(&mut pairs);
        }

        Iter {
            pairs,
        }
    }
}

impl<'a, T: Ord + Debug + Debug, V: Debug> IntoIterator for &'a BstMap<T, V> {
    type Item = (&'a T, &'a V);
    type IntoIter = IterRef<'a, T, V>;

    fn into_iter(self) -> Self::IntoIter {
        let mut pairs = Vec::new();
        if self.head.is_some() {
            self.head.as_ref().unwrap().fill_ref_vec(&mut pairs);
        }

        IterRef {
            pairs,
            index: 0,
            len: self.len,
        }
    }
}

impl<'a, T: Ord + Debug + Debug, V: Debug> IntoIterator for &'a mut BstMap<T, V> {
    type Item = (&'a T, &'a mut V);
    type IntoIter = IterMut<'a, T, V>;

    fn into_iter(self) -> Self::IntoIter {
        let mut pairs = Vec::new();
        if self.head.is_some() {
            self.head.as_mut().unwrap().fill_mut_vec(&mut pairs);
        }

        IterMut {
            pairs,
        }
    }
}
