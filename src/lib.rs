pub mod traitimpls;

use std::ops::{Deref,DerefMut};
use indxvec::{Indices,merge::*};

// const EMPTYIDX:Vec<usize> = vec![];

/// Unordered set holding a generic Vec<T>.
/// Usually is the initial input.
// #[derive(Clone, PartialEq, Eq)]
pub struct Set<T> {
    pub v: Vec<T>
} 

/// helper function to stringify a generic vector for display, without recourse to debug
fn writevec<T>(v:&[T]) -> String where T: std::fmt::Display {
    let mut s = String::from("\x1B[01;92m[ ");
    for x in v { s.push_str(&x.to_string()); s.push_str(" ") };
    s.push_str("]\x1B[0m");
    s
}

/// Implementation of Display trait for struct Set.
impl<T: std::fmt::Display> std::fmt::Display for Set<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Unordered Set:\n{}",writevec(&self.v))
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
    /// Simply clones the slice and throws away its index
    pub fn from_indexed(s: &IndexedSet<T>) -> Self {
        Set{ v: s.v.to_vec() }
    }
    /// Simply clones the slice and throws away the ranks
    pub fn from_ranked(s: &RankedSet<T>) -> Self {
        Set{ v: s.v.to_vec() }
    }
 
    /* 
    /// Sort index for an unordered set
    pub fn sortidx(self) -> Vec<usize> where T:PartialOrd {
        mergesort(&self.v,0,self.v.len())
    }
    /// Reverse by reverse iteration
    pub fn revs(self) -> Vec<T> { 
        self.v.iter().rev().map(|&x| x).collect::<Vec<T>>() 
    }   
    */
}

/// Ordered Set, holding an explicitly sorted (ascending or descending) generic Vec<T>. 
/// Often is the output of some process.
pub struct OrderedSet<T> {
    pub ascending: bool,
    pub v: Vec<T>,
}
/// Display trait implemented for struct OrderedSet.
impl<T: std::fmt::Display> std::fmt::Display for OrderedSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let n = self.v.len();
        if n == 0 { return writeln!(f,"[]") }
        let s = if self.ascending { String::from("Ascending") }
            else { String::from("Descending") };  
        writeln!(f, "{} Ordered Set:\n{}", s, writevec(&self.v) )
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

    /// Initiliser, explicitly sorts an unordered slice
    pub fn from_slice(s: &[T], asc: bool) -> Self where T:PartialOrd+Copy {
        OrderedSet{ ascending:asc, v: sortm(s,asc) }
    }
    /// Initiliser, explicitly sorts an unordered Set
    pub fn from_set(s: &Set<T>, asc: bool) -> Self where T:PartialOrd+Copy {
        OrderedSet{ ascending:asc, v: sortm(&s.v,asc) }
    }
    /// Uses index to build explicitly ordered set
    pub fn from_indexed(s: &IndexedSet<T>, asc: bool) -> Self where T:PartialOrd+Copy {
        let order = if asc == s.ascending { true } else { false };
        OrderedSet{ ascending:asc, v: s.i.unindex(s.v,order) }
    }
    /// Uses ranks to build explicitly ordered set
    pub fn from_ranked(s: &RankedSet<T>, asc: bool) -> Self where T:PartialOrd+Copy {
        let order = if asc == s.ascending { true } else { false };
        OrderedSet{ ascending:asc, v: s.i.invindex().unindex(s.v,order) }
    }
}

/// Struct holding a borrowed unordered set and its sort index. 
/// Thus it is an index ordered set (ascending or descending).
pub struct IndexedSet<'a,T> {
    pub ascending: bool,
    pub v: &'a[T],
    pub i: Vec<usize>,
}
/// Display implemented for struct IndexedSet.
impl<'a,T: std::fmt::Display> std::fmt::Display for IndexedSet<'a,T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let n = self.v.len();
        if n == 0 { return writeln!(f,"[]") }
        let s = if self.ascending { String::from("Ascending") }
            else { String::from("Descending") };  
        writeln!(f, "{} Indexed Set\nSet:   {}\nIndex: {}",
            s, writevec(&self.v), writevec(&self.i) )
    }
}

impl<'a,T> IndexedSet<'a,T> {
    /// Initiliser, indexsorts an unordered slice
    pub fn from_slice(s: &'a[T], asc:bool) -> Self where T:PartialOrd+Copy {
        if asc { IndexedSet{ ascending:true, v:s, i:sortidx(s) } }
        else { IndexedSet{ ascending:false, v:s, i:revs(&sortidx(s)) } }
    }
    /// Initiliser, indexsorts an unordered Set
    pub fn from_set(s: &'a Set<T>, asc: bool) -> Self where T:PartialOrd+Copy {
        if asc { IndexedSet{ ascending:true, v:&s.v, i:sortidx(&s.v) } }
        else { IndexedSet{ ascending:false, v:&s.v, i:revs(&sortidx(&s.v)) } }     
    }
    /// From Oredered is not often needed, as the index will be trivial 
    pub fn from_ordered(s: &'a OrderedSet<T>, asc: bool) -> Self where T:PartialOrd+Copy {
        let n = s.len();
        let mut idx:Vec<usize> = vec![0;s.len()];
        if asc == s.ascending { for i in 0..n { idx[i] = i } }          
        else { for i in (0..n).rev() { idx[i] = i } };
        IndexedSet{ ascending:asc, v:&s.v, i:idx } 
    }
    /// Converts ranks to sort index
    pub fn from_ranked(s: &'a RankedSet<T>, asc: bool) -> Self where T:PartialOrd+Copy {
        if asc == s.ascending { IndexedSet{ ascending: asc, v: s.v, i:s.i.invindex() } }
        else  { IndexedSet{ ascending: asc, v: s.v, i:revs(&s.i.invindex()) } }     
    }
}

/// Struct holding a borrowed unordered set 
/// and a vector of its ranks (ascending or descending).
pub struct RankedSet<'a,T> {
    pub ascending: bool,
    pub v: &'a[T],
    pub i: Vec<usize>,
}
/// Display implemented for struct IndexedSet.
impl<'a,T: std::fmt::Display> std::fmt::Display for RankedSet<'a,T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let n = self.v.len();
        if n == 0 { return writeln!(f,"[]") }
        let s = if self.ascending { String::from("Ascending") }
            else { String::from("Descending") };  
        writeln!(f, "{} Ranked Set\nSet:   {}\nRanks: {}", s, writevec(&self.v), writevec(&self.i) )
    }
}
impl<'a,T> RankedSet<'a,T> {
    /// Initiliser, ranks an unordered slice
    pub fn from_slice(s: &'a[T], asc:bool) -> Self where T:PartialOrd+Copy {
        RankedSet{ ascending:asc, v:s, i:rank(s,asc) }
    }
    /// Initiliser, ranks an unordered Set
    pub fn from_set(s: &'a Set<T>, asc: bool) -> Self where T:PartialOrd+Copy {
        RankedSet{ ascending:asc, v:&s.v, i:rank(s,asc) } 
    }        
    /// From Oredered is not often needed, as the index will be trivial 
    pub fn from_ordered(s: &'a OrderedSet<T>, asc: bool) -> Self where T:PartialOrd+Copy {
        let n = s.len();
        let mut idx:Vec<usize> = vec![0;s.len()];
        if asc == s.ascending { for i in 0..n { idx[i] = i } }          
        else { for i in (0..n).rev() { idx[i] = i } };
        RankedSet{ ascending:asc, v:&s.v, i:idx } 
    }
    /// Converts sort index to ranks
    pub fn from_indexed(s: &'a IndexedSet<T>, asc: bool) -> Self where T:PartialOrd+Copy {
        if asc == s.ascending { RankedSet{ ascending: asc, v: s.v, i:s.i.invindex() } }
        else  { RankedSet{ ascending: asc, v: s.v, i:s.i.invindex().complindex() } }     
    }
}

/// Methods for the set structs.
pub trait SetOps<T> where T: Copy {
    /// Deletes any repetitions
    fn nonrepeat(&self) -> Self where T: PartialOrd+Copy;
    /// Finds minimum, minimum's first index, maximum, maximum's first index  
    fn infsup(&self) -> (T, usize, T, usize) where T: PartialOrd+Copy; 
    /// True if m is a member of the set
    fn member(&self, m: T) -> bool where T: PartialOrd; 
    /// Search of a set, returns Some(index) of the last item found, or None.
    fn search(&self, m: T)  -> Option<usize> where T: PartialOrd;    
    /// Union of two sets of the same type
    fn union(&self, s: &Self) -> OrderedSet<T> where T: PartialOrd;
    /// Intersection of two sets of the same type
    fn intersection(&self, s: &Self) -> OrderedSet<T> where T: PartialOrd;
    /// Removing s from self (i.e. self-s)
    fn difference(&self, s: &Self) -> OrderedSet<T> where T: PartialOrd;
}
