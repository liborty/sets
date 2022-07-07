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
#[derive(Default)]
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

// static EMPTYSET:Set<f64> = Set{ stype:SType::Empty, ascending:true, data:vec![], index:vec![]};

/// Default values for empty Set<T>
/// Note that the data and index Vecs are empty 
/// but still of the end types T and <usize> respectively
/// Not needed since Rust 1.62.0
//impl<T> Default for Set<T> {
//    fn default() -> Self { 
//        Set { stype:SType::Empty, ascending:true, data:vec![], index:vec![] }
//    }
//}

/// Implementation of Display trait for struct Set.
impl<T: std::fmt::Display> std::fmt::Display for Set<T> where T:Copy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.stype {
            SType::Empty =>  writeln!(f,"Empty Set"),
            SType::Unordered => writeln!(f, "Unordered Set\n{}",self.data.gr()),
            SType::Ordered => writeln!(f, "Ordered {} Set\nData: {}",ascdesc(self.ascending),self.data.gr()),
            SType::Indexed => writeln!(f, "Indexed {} Set\nData: {}\nIndex: {}",ascdesc(self.ascending),self.data.gr(),self.index.yl()),
            SType::Ranked => writeln!(f, "Ranked {} Set\nData: {}\nRanks: {}",ascdesc(self.ascending),self.data.gr(),self.index.yl()),
            // _ => panic!("{} Unrecognised Set field {}",here!(),self.stype)
        }
    }
}

/// Implementation of Clone trait for struct Set.    
impl<T> Clone for Set<T> where T:Clone {
    fn clone(&self) -> Self {
        Set { stype:self.stype, ascending:self.ascending, data:self.data.to_vec(), index:self.index.to_vec() }
    }
}

/// Mutable methods for all four of the set structs
pub trait MutSetOps<T> {
    /// Deletes an item of the same end-type from self
    fn mdelete(&mut self, item:T) -> bool;
    /// Inserts an item of the same end-type to self
    fn minsert(&mut self, item:T);
    /// reverses the vector of explicit sets and index of indexed sets
    fn mreverse(&mut self);
    /// Deletes any repetitions
    fn mnonrepeat(&mut self); 
    /// Union of two sets of the same type
    fn munion(&mut self, s: &Self);
    /// Intersection of two sets of the same type
    fn mintersection(&mut self, s: &Self);
    /// Removing s from self (i.e. self-s)
    fn mdifference(&mut self, s: &Self);
}
