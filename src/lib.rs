pub mod traitimpls;
pub mod mutimpls;

use std::ops::{Deref,DerefMut};
use indxvec::{MinMax,Printing,Indices,Vecops};

// const EMPTYIDX:Vec<usize> = vec![];

/// Constructs a trivial index (for already sorted sets), 
/// of required ascending or descending order and size
pub fn trivindex(asc:bool,n:usize) -> Vec<usize> { 
    if asc { Vec::from_iter(0..n) } 
    else { Vec::from_iter((0..n).rev()) }
}

/// Unordered set holding a generic Vec<T>. 
/// Usually is the initial input.
pub struct Set<T> {
    pub v: Vec<T>
} 

/// Implementation of Display trait for struct Set.
impl<T: std::fmt::Display> std::fmt::Display for Set<T> where T:Copy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Unordered Set:\n{}",self.v.gr())
    }
}

/// Implementation of Deref trait for struct Set.
/// Thus, for instance, calling `OrderedSet::from_slice(&s,true)`,
/// where s is an instance of Set, will dereference s to the vector 
/// contained in s and eventually to its slice and will not throw a type error.
/// Of course, in this particular example, it would have been more correct to invoke 
/// `OrderedSet::from_set(&s,true)` in the first place.
impl<T> Deref for Set<T> {
    type Target = Vec<T>; 
    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

/// Implementation of DerefMut trait for struct Self.
impl<T> DerefMut for Set<T> { 
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}

impl<T> Set<T> where T: Copy {
    /// Initialiser - copies to a new Vec
    pub fn from_slice(s: &[T]) -> Self {
        Set { v: s.to_vec() }
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
    pub ascending: bool,
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

impl<T> OrderedSet<T> {

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
    pub ascending: bool,
    pub v: Vec<T>,
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

impl<'a,T> IndexedSet<T> {
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
    pub ascending: bool,
    pub v: Vec<T>,
    pub i: Vec<usize>,
}
/// Display implemented for struct IndexedSet.
impl<'a,T: std::fmt::Display> std::fmt::Display for RankedSet<T> where T:Copy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let n = self.v.len();
        if n == 0 { return writeln!(f,"[]") }
        let s = if self.ascending { String::from("Ascending") }
            else { String::from("Descending") };  
        writeln!(f, "{} Ranked Set\nSet:   {}\nRanks: {}", s, self.v.gr(),self.i.gr())
    }
}
impl<T> RankedSet<T> {
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

/// Common methods for all four of the set structs.
pub trait SetOps<T> {
    /// reverses the vector of explicit sets and index of indexed sets
    fn reverse(&self) -> Self;
    /// Deletes any repetitions
    fn nonrepeat(&self) -> Self;
    /// Finds minimum, minimum's first index, maximum, maximum's first index  
    fn infsup(&self) -> MinMax<T>; 
    /// True if m is a member of the set
    fn member(&self, m: T) -> bool;
    /// Search of a set, returns Some(index) of the last item found, or None.
    fn search(&self, m: T)  -> Option<usize>;    
    /// Union of two sets of the same type
    fn union(&self, s: &Self) -> Self;
    /// Intersection of two sets of the same type
    fn intersection(&self, s: &Self) -> OrderedSet<T>;
    /// Removing s from self (i.e. self-s)
    fn difference(&self, s: &Self) -> OrderedSet<T>;
}
pub trait MutSetOps<T> {
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
