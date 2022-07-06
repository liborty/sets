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

/// Associated functions for conversions returning Set<T> = Self
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
                index: trivindex(self.ascending == asc,self.data.len()) }
        }    
    }
 
    /// Simply copies the slice and throws away its index
    pub fn from_indexed(s: &IndexedSet<T>) -> Self {
        Set{ v: s.v.to_vec() }
    }
    /// Simply copies the slice and throws away the ranks
    pub fn from_ranked(s: &RankedSet<T>) -> Self {
        Set{ v: s.v.to_vec() }
    }  
}

/// Ordered Set, holding an explicitly sorted (ascending or descending) generic Vec<T>. 
/// Often is the final result of some set operations.
pub struct OrderedSet<T> {
    /// Ascending order (true), descending (false)
    pub ascending: bool,
    /// Ordered data vector
    pub v: Vec<T>,
}
/// Display trait implemented for struct OrderedSet.
impl<T: std::fmt::Display> std::fmt::Display for OrderedSet<T> where T:Copy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let n = self.v.len();
        if n == 0 { return writeln!(f,"[]") }
        let ascdesc = if self.ascending { String::from("Ascending") }
            else { String::from("Descending") };  
        writeln!(f, "{} Ordered Set:\n{}", ascdesc, self.v.gr())
    }
}

impl<T> Deref for OrderedSet<T> {
    type Target = Vec<T>; 
    fn deref(&self) -> &Self::Target {
        &self.v
    }
}
impl<T> DerefMut for OrderedSet<T> { 
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}

/// Implementation of Clone trait for struct OrderedSet.    
impl<T> Clone for OrderedSet<T> where T:Clone {
    fn clone(&self) -> Self {
        OrderedSet{ ascending: self.ascending, v: self.v.to_vec() }
    }
}
impl<T> OrderedSet<T>  where T: Copy {

    /// Constructor from an ascending ordered slice
    pub fn from_asc_slice(s: &[T]) -> Self where T:PartialOrd+Copy {        
        OrderedSet{ ascending:true, v: s.to_vec() }
    }

    /// Constructor from a descending ordered slice
    pub fn from_desc_slice(s: &[T]) -> Self where T:PartialOrd+Copy {        
        OrderedSet{ ascending:false, v: s.to_vec() }
    }

    /// Initialiser, explicitly sorts an unordered slice
    pub fn from_slice(s: &[T], asc: bool) -> Self where T:PartialOrd+Copy {
        OrderedSet{ ascending:asc, v: s.sortm(asc) }
    }
    /// Initialiser, explicitly sorts an unordered Set
    pub fn from_set(s: &Set<T>, asc: bool) -> Self where T:PartialOrd+Copy {
        OrderedSet{ ascending:asc, v: s.v.sortm(asc) }
    }
    /// Uses index to build explicitly ordered set
    pub fn from_indexed(s: &IndexedSet<T>, asc: bool) -> Self where T:PartialOrd+Copy {
        OrderedSet{ ascending:asc, v: s.i.unindex(&s.v,asc == s.ascending) }
    }
    /// Uses ranks to build explicitly ordered set
    pub fn from_ranked(s: &RankedSet<T>, asc: bool) -> Self where T:PartialOrd+Copy {
        OrderedSet{ ascending:asc, v: s.i.invindex().unindex(&s.v,asc == s.ascending) }
    }
}

/// Struct holding an (unordered)git set and its sort index. 
/// Thus it is an index ordered set (ascending or descending).
pub struct IndexedSet<T> {
    /// Ascending order (true), descending (false)    
    pub ascending: bool,
    /// Unordered data vector
    pub v: Vec<T>,
    /// Index giving the data sort order
    pub i: Vec<usize>,
}
/// Display implemented for struct IndexedSet.
impl<'a,T: std::fmt::Display> std::fmt::Display for IndexedSet<T> where T:Copy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let n = self.v.len();
        if n == 0 { return writeln!(f,"[]") }
        let s = if self.ascending { String::from("Ascending") }
            else { String::from("Descending") };  
        writeln!(f, "{} Indexed Set\nSet:   {}\nIndex: {}",
            s, self.v.gr(), self.i.gr())
    }
}
/// Implementation of Clone trait for struct IndexedSet.    
impl<T> Clone for IndexedSet<T> where T:Clone {
    fn clone(&self) -> Self {
        IndexedSet{ ascending: self.ascending, v: self.v.to_vec(), i: self.i.to_vec() }
    }
}

impl<'a,T> IndexedSet<T> where T:Copy {

    /// Initialiser, indexsorts an unordered slice
    pub fn from_slice(s: &'a[T], asc:bool) -> Self where T:PartialOrd+Copy {
        if asc { IndexedSet{ ascending:true, v:s.to_vec(), i:s.sortidx() } }
        else { IndexedSet{ ascending:false, v:s.to_vec(), i:s.sortidx().revindex() } }
    }
    /// Initialiser, indexsorts an unordered Set
    pub fn from_set(s: &'a Set<T>, asc: bool) -> Self where T:PartialOrd+Copy {
        if asc { IndexedSet{ ascending:true, v:s.v.to_vec(), i:s.sortidx() } }
        else { IndexedSet{ ascending:false, v:s.v.to_vec(), i:s.v.sortidx().revindex() } }     
    }
    /// From Oredered, the sort index will be trivial 
    pub fn from_ordered(s: &'a OrderedSet<T>, asc: bool) -> Self where T:PartialOrd+Copy {        
        IndexedSet{ ascending:asc, v:s.v.to_vec(), i:trivindex(asc == s.ascending,s.len()) } 
    }
    /// Converts ranks to sort index
    pub fn from_ranked(s: &'a RankedSet<T>, asc: bool) -> Self where T:PartialOrd+Copy {
        if asc == s.ascending { IndexedSet{ ascending: asc, v: s.v.to_vec(), i:s.i.invindex() } }
        else  { IndexedSet{ ascending: asc, v: s.v.to_vec(), i:s.i.complindex().invindex() } }     
    }
}

/// Struct holding an unordered set 
/// and a vector of its ranks (ascending or descending).
pub struct RankedSet<T> {
    /// Ascending order (true), descending (false)
    pub ascending: bool,
    /// Unordered data vector
    pub v: Vec<T>,
    /// Index giving the data ranks
    pub i: Vec<usize>,
}
/// Display implemented for struct RankedSet.
impl<'a,T: std::fmt::Display> std::fmt::Display for RankedSet<T> where T:Copy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let n = self.v.len();
        if n == 0 { return writeln!(f,"[]") }
        let s = if self.ascending { String::from("Ascending") }
            else { String::from("Descending") };  
        writeln!(f, "{} Ranked Set\nSet:   {}\nRanks: {}", s, self.v.gr(),self.i.gr())
    }
}
/// Implementation of Clone trait for struct RankedSet.    
impl<T> Clone for RankedSet<T> where T:Clone {
    fn clone(&self) -> Self {
        RankedSet{ ascending: self.ascending, v: self.v.to_vec(), i: self.i.to_vec() }
    }
}

/// Associated functions for conversions, returning RankedSet
impl<T> RankedSet<T> where T:Copy {

    /// Initialiser, ranks an unordered slice
    pub fn from_slice(s: &[T], asc:bool) -> Self where T:PartialOrd+Copy {
        RankedSet{ ascending:asc, v:s.to_vec(), i:s.rank(asc) }
    }
    /// Initialiser, ranks an unordered Set
    pub fn from_set(s: &Set<T>, asc: bool) -> Self where T:PartialOrd+Copy {
        RankedSet{ ascending:asc, v:s.v.to_vec(), i:s.rank(asc) } 
    }        
    /// From Ordered - the index will be trivial 
    pub fn from_ordered(s: &OrderedSet<T>, asc: bool) -> Self where T:PartialOrd+Copy {       
        RankedSet{ ascending:asc, v:s.v.to_vec(), i:trivindex(asc == s.ascending,s.len()) } 
    }
    /// Converts sort index to ranks
    pub fn from_indexed(s: &IndexedSet<T>, asc: bool) -> Self where T:PartialOrd+Copy {
        if asc == s.ascending { RankedSet{ ascending: asc, v: s.v.to_vec(), i:s.i.invindex() } }
        else { RankedSet{ ascending: asc, v: s.v.to_vec(), i:s.i.invindex().complindex() } }     
    }
}

/// Required methods for all four of the set structs.
pub trait SetOps<T>  where Self: MutSetOps<T> + Sized {
    /// reverses the vector of explicit sets and index of indexed sets
    fn reverse(&self) -> Self;
    /// Deletes any repetitions
    fn nonrepeat(&self) -> Self;git merge
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
