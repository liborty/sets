pub mod functions;

use std::ops::{Deref,DerefMut};
use indxvec::{Indices,merge::*};
use self::functions::*;

// const EMPTYIDX:Vec<usize> = vec![];

/// macro `here!()` gives `&str` with the current `file:line path::function` for error messages.
#[macro_export]
macro_rules! here {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        // For function name only:
        // let fnct = match &name[..name.len()-3].rfind(':') {
        //    Some(pos) => &name[pos + 1..name.len() - 3],
        //    None => &name[..name.len()-3],
        // };
        format!("\n{}:{} {}", file!(), line!(), &name[..name.len()-3])
    }}
}

/// Unordered set holding a generic Vec<T>.
/// Usually is the initial input.
// #[derive(Clone, PartialEq, Eq)]
pub struct Set<T> {
    pub v: Vec<T>
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
        writeln!(f, "{} Indexed Set\nSet:   {}\nIndex: {}", s, writevec(&self.v), writevec(&self.i) )
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
    /// Converts sort index to ranks
    pub fn from_indexed(s: &'a IndexedSet<T>, asc: bool) -> Self where T:PartialOrd+Copy {
        if asc == s.ascending { RankedSet{ ascending: asc, v: s.v, i:s.i.invindex() } }
        else  { RankedSet{ ascending: asc, v: s.v, i:s.i.invindex().complindex() } }     
    }
}



/*
/// Methods to manipulate indices of `Vec<usize>` type.
pub trait SetOps<T> where  T: Copy {
    // Vec<T> : IntoIterator { 
    /// Finds minimum, minimum's first index, maximum, maximum's first index of &[T] 
    fn minmax(self) -> (T, usize, T, usize) where T: PartialOrd+Copy+IntoIterator; 
    /// Binary search of a sorted list (in ascending order).
    fn binsearch(self, val: T)  -> usize where T: PartialOrd;
    /// Merges two ascending sorted generic vectors.
    fn merge(self, v2: Self) -> Self where T: PartialOrd;  
}
*/
