#![warn(missing_docs)]
//! Operations on  Unordered Sets, Ordered Sets, Indexed Sets, Ranked Sets

/// Associated functions for struct Set
pub mod setimpls;
/// Mutable set operations, implemented for &mut Set
pub mod mutimpls;

use indxvec::{Printing};

/// Constructs a trivial index (for already sorted sets), 
/// of required ascending or descending order and size
pub fn trivindex(asc:bool,n:usize) -> Vec<usize> { 
    if asc { Vec::from_iter(0..n) } 
    else { Vec::from_iter((0..n).rev()) }
}

/// Display helper function
pub fn ascdesc(asc:bool) -> &'static str {
    if asc { "Ascending" } else { "Descending" }
}

/// The five types of sets
#[derive(Default,Clone,Copy)]
pub enum SType {
    /// empty set
    #[default]
    Empty,
    /// unordered set
    Unordered,
    /// ordered set
    Ordered,
    /// indexed set
    Indexed,
    /// ranked set
    Ranked
}

/// Implementation of Display trait for struct Set.
impl std::fmt::Display for SType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f,"{}",self.to_str())
        }
}

/// The struct type for sets
#[derive(Default,Clone)]
pub struct Set<T> {
    /// type of the set
    pub stype: SType,
    /// order: ascending (true), descending (false)
    pub ascending: bool,
    /// data Vec
    pub data: Vec<T>,
    /// index Vec
    pub index: Vec<usize>
}

/// Implementation of Display trait for struct Set.
impl<T: std::fmt::Display> std::fmt::Display for Set<T> where T:Copy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.stype {
            SType::Empty =>  writeln!(f,"Empty Set"),
            SType::Unordered => writeln!(f, "Unordered Set\nData: {}",self.data.gr()),
            SType::Ordered => writeln!(f, "Ordered {} Set\nData: {}",ascdesc(self.ascending),self.data.gr()),
            SType::Indexed => writeln!(f, "Indexed {} Set\nData: {}\nIndex: {}",ascdesc(self.ascending),self.data.gr(),self.index.yl()),
            SType::Ranked => writeln!(f, "Ranked {} Set\nData: {}\nRanks: {}",ascdesc(self.ascending),self.data.gr(),self.index.yl()),
            // _ => panic!("{} Unrecognised Set field {}",here!(),self.stype)
        }
    }
}

/*
/// Implementation of Clone trait for struct Set.    
impl<T> Clone for Set<T> where T:Copy+PartialOrd+Default {
    fn clone(&self) -> Self {
        match self.stype {
            // empty set is a unique constant (modulo T). 
            // no need to make another copy of it, be it default valued, with Default::default()
            SType::Empty => Set::EMPTYSET, 
            _ => Set { stype:self.stype, ascending:self.ascending, data:self.data.to_vec(), index:self.index.to_vec() }
        }
    }
}
*/

/// Mutable methods for &mut Set<T>
pub trait MutSetOps<T> {
    /// Makes a Set unordered
    fn munordered(&mut self);
    /// Makes a Set ordered
    fn mordered(&mut self, quantify: impl Copy + Fn(&T) -> f64, asc:bool);
    /// Makes any Set indexed
    fn mindexed(&mut self, quantify: impl Copy + Fn(&T) -> f64, asc:bool);
    /// Converts any Set type to ranked
    fn mranked(&mut self,asc:bool);
    /// General converter: s -> Set of the same type and order as self
    fn msame(&mut self, s:&mut Self, quantify: impl Copy + Fn(&T) -> f64);
    /// Deletes the first item from self
    fn mdelete(&mut self, item:T) -> bool;
    /// Deletes all occurrences of a matching item from self, returns their count
    fn mdeleteall(&mut self, item:T) -> usize;
    /// Inserts an item of the same end-type to self
    fn minsert(&mut self, item:T);
    /// reverses the vector of explicit sets and index of indexed sets
    fn mreverse(&mut self);
    /// Deletes all repetitions
    fn mnonrepeat(&mut self); 
    /// Union of two sets of the same type
    fn munion(&mut self, s: &Self);
    /// Intersection of two sets of the same type
    fn mintersection(&mut self, s: &Self);
    /// Removing s from self (i.e. self-s)
    fn mdifference(&mut self, s: &Self);
}
