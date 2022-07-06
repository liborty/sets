#![warn(missing_docs)]
//! Operations on Sets, Ordered Sets, Indexed Sets, Ranked Sets

/// Set operations, implemented for the four types of sets 
pub mod traitimpls;
/// Mutable set operations, implemented for the four types of sets
pub mod mutimpls;

use std::ops::{Deref,DerefMut};
use indxvec::{MinMax,Printing,Indices,Vecops};

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

pub enum SType {
    Empty,
    Unordered,
    Ordered,
    Indexed,
    Ranked
}

pub struct Set<T> {
    pub stype: SType,
    pub ascending: bool,
    pub data: Vec<T>,
    pub index: Vec<usize>
}

// static EMPTYSET:Set<f64> = Set{ stype:SType::Empty, ascending:true, data:vec![], index:vec![]};

/// Default values for Set<T>
/// Note that the data and index Vecs are empty 
/// but still of the end types T and <usize> respectively
impl<T> Default for Set<T> {
    fn default() -> Self { 
        Set { stype:SType::Empty, ascending:true, data:vec![], index:vec![] }
    }
}

/// Implementation of Display trait for struct Set.
impl<T: std::fmt::Display> std::fmt::Display for Set<T> where T:Copy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.stype {
            Empty =>  writeln!(f,"Empty Set"),
            Unordered => writeln!(f, "Unordered Set\n{}",self.data.gr()),
            Ordered => writeln!(f, "Ordered {} Set\nData: {}",ascdesc(self.ascending),self.data.gr()),
            Indexed => writeln!(f, "Indexed {} Set\nData: {}\nIndex: {}",ascdesc(self.ascending),self.data.gr(),self.index.yl()),
            Ranked => writeln!(f, "Ranked {} Set\nData: {}\nRanks: {}",ascdesc(self.ascending),self.data.gr(),self.index.yl()),
        }
    }
}

/// Implementation of Clone trait for struct Set.    
impl<T> Clone for Set<T> where T:Clone {
    fn clone(&self) -> Self {
        Set { stype:self.stype, ascending:self.ascending, data:self.data.to_vec(), index:self.index.to_vec() }
    }
}

/// Associated functions for conversions and set operations returning Set<T> = Self
impl<T> Set<T> where T: Copy+PartialOrd {

    /// Initialiser - creates a new Unordered Vec
    pub fn from_slice(s: &[T]) -> Self {
        let mut newset = Set::default();
        newset.stype = SType::Unordered;
        newset.data = s.to_vec();
        newset
    }

    pub fn to_ordered(&self,asc:bool) -> Self {
        match self.stype {
            Empty => *self, // empty set is unique
            Unordered => Self{ stype:SType::Ordered, ascending:asc, data:self.data.sortm(asc), index:vec![] },
            Ordered => *self, // no op
            Indexed => Self{ stype:SType::Ordered, ascending:asc, 
                data:self.index.unindex(&self.data, asc),
                index:vec![] },
            Ranked => Self{ stype:SType::Ordered, ascending:asc, 
                data:self.index.invindex().unindex(&self.data, asc),
                index:vec![] },
        }    
    }

    pub fn to_indexed(&self,asc:bool) -> Self {
        match self.stype {
            Empty => *self,
            Unordered => Self{ stype:SType::Indexed, ascending:asc, data:self.data, 
                index: if asc {self.data.sortidx()} else {self.data.sortidx().revs()} },
            Ordered => Self{ stype:SType::Indexed, ascending:asc, data:self.data, 
                index: trivindex(self.ascending == asc,self.data.len()) },
            Indexed => *self, 
            Ranked => Self{ stype:SType::Indexed, ascending:asc, data:self.data,             
                index: if self.ascending == asc {self.index.invindex()} 
                    else {self.index.invindex().revs()}}
        }    
    }

    pub fn to_ranked(&self,asc:bool) -> Self {
        match self.stype {
            Empty => *self,
            Unordered => Self{ stype:SType::Ranked, ascending:asc, data:self.data, 
                index: if asc {self.data.sortidx().invindex()} 
                    else {self.data.sortidx().revs().invindex()} },
            Ordered => Self{ stype:SType::Ranked, ascending:asc, data:self.data, 
                index: trivindex(self.ascending == asc,self.data.len()) },
            Indexed => Self{ stype:SType::Ranked, ascending:asc, data:self.data,             
                index: if self.ascending == asc {self.index.invindex()} 
                    else {self.index.revs().invindex()}}, 
            Ranked => *self
        }    
    }

    /// Inserts an item of the same end-type to self
    pub fn insert(&self, item:T) -> Self {
        let mut scopy = self.clone();
        scopy.minsert(item);
        scopy 
    }

    /// Deletes an item of the same end-type from self
    pub fn delete(&self, item:T) -> Self {
        let mut scopy = self.clone();
        if scopy.mdelete(item) { scopy } else { *self }
    }    

    /// Reverses a vec by iterating over only half of its length
    /// and swapping the items
    pub fn reverse(&self) -> Self { 
        let mut scopy = self.clone();
        scopy.mreverse();
        scopy
    }
 
    /// Deletes any repetitions
    pub fn nonrepeat(&self) -> Self { 
        let mut scopy =  self.clone();
        scopy.mnonrepeat();
        scopy
    }

    /// Union of two sets  
    pub fn union(&self, s: &Self) -> Self {
        let mut scopy =  self.clone();
        scopy.munion(s);
        scopy
    }
    
    /// Intersection of two sets
    pub fn intersection(&self, s: &Self) -> Self {
        let mut scopy = self.clone();
        scopy.mintersection(s);
        scopy
    }
    
    /// Complement of s in self (i.e. self -= s)
    pub fn difference(&self, s: &Self) -> Self {
        let mut scopy = self.clone();
        scopy.mdifference(s);
        scopy
    }

    /// Finds minimum, minimum's first index, maximum, maximum's first index  
    pub fn infsup(&self) -> MinMax<T> {
        match self.stype {
            Empty => *self,
            Unordered => Self{ stype:SType::Ranked, ascending:asc, data:self.data, 
                index: if asc {self.data.sortidx().invindex()} 
                    else {self.data.sortidx().revs().invindex()} },
            Ordered => Self{ stype:SType::Ranked, ascending:asc, data:self.data, 
                index: trivindex(self.ascending == asc,self.data.len()) },
            Indexed => Self{ stype:SType::Ranked, ascending:asc, data:self.data,             
                index: if self.ascending == asc {self.index.invindex()} 
                    else {self.index.revs().invindex()}}, 
            Ranked => *self
        }      
    } 

}   


/// Required methods for all four of the set structs.
pub trait SetOps<T>  where Self: MutSetOps<T> + Sized {
    /// reverses the vector of explicit sets and index of indexed sets
    fn reverse(&self) -> Self;
    /// Deletes any repetitions
    fn nonrepeat(&self) -> Self;
    /// fn nonrepeat(&self) -> Self;  
    /// Finds minimum, minimum's first index, maximum, maximum's first index  
    fn infsup(&self) -> MinMax<T>; 
    /// True if m is a member of the set
    fn member(&self, m: T) -> bool;
    /// Some(index) of the first item found, or None.
    fn search(&self, m: T)  -> Option<usize>; 
    /// Index of the next item in order, or self.len(). Mostly for non-members.
    /// For unordered sets returns self.len(), too.
    fn position(&self, m: T)  -> usize;       
    /// Union of two sets of the same type
    fn union(&self, s: &Self) -> Self;
    /// Intersection of two sets of the same type
    fn intersection(&self, s: &Self) -> OrderedSet<T>;
    /// Removing s from self (i.e. self-s)
    fn difference(&self, s: &Self) -> OrderedSet<T>;
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
