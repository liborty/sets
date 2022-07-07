#![warn(missing_docs)]
//! Operations on  Unordered Sets, Ordered Sets, Indexed Sets, Ranked Sets

/// Mutable set operations, implemented for &mut Set
pub mod mutimpls;

use indxvec::{MinMax,Printing,Indices,Vecops, here};

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

/// Implementation of Display trait for struct Set.
impl std::fmt::Display for SType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f,"{}",self.to_str())
        }
    }

pub struct Set<T> {
    pub stype: SType,
    pub ascending: bool,
    pub data: Vec<T>,
    pub index: Vec<usize>
}

// static EMPTYSET:Set<f64> = Set{ stype:SType::Empty, ascending:true, data:vec![], index:vec![]};

/// Default values for empty Set<T>
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
            _ => panic!("{} Unrecognised Set field {}",here!(),self.stype)
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
    pub fn unordered_from_slice(s: &[T]) -> Self { 
        let mut newset = Set::default(); // returns this empty set for zero lentgh s
        if s.len() > 0 { // have some data, so modify the newset with it
            newset.stype = SType::Unordered;
            newset.data = s.to_vec(); 
        }; 
        newset
    }

    /// Converter - to unordered Set
    /// Caution: this just throws away the valuable index!
    pub fn to_unordered(&self) -> Self { 
        match self.stype {
            Empty => *self, // empty set is unique
            Unordered => *self, // no op
            Ordered => Self{ stype:SType::Unordered, ascending:true, data:self.data, index:vec![] },
            Indexed => Self{ stype:SType::Unordered, ascending:true, data:self.data, index:vec![] }, 
            Ranked => Self{ stype:SType::Unordered, ascending:true, data:self.data, index:vec![] }
        }
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

    /// General converter: s -> Set of the same type and order as self
    pub fn to_self(&self,s: &Self) -> Self { 
        // if self.stype = s.stype { return *s }; // nothing to do
        match self.stype {
            Empty => Default::default(), // empty set
            Unordered => s.to_unordered(), 
            Ordered => s.to_ordered(self.ascending),
            Indexed => s.to_indexed(self.ascending),
            Ranked => s.to_ranked(self.ascending)
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
    pub fn infsup(&self) -> MinMax<T> where T: Default {
        match self.stype {
            Empty => Default::default(),
            Unordered => self.data.minmax(),  
            Ordered => {
                let last = self.data.len()-1;
                if self.ascending { MinMax{min:self.data[0],minindex:0,max:self.data[last],maxindex:last} }
                else { MinMax{min:self.data[last],minindex:last,max:self.data[0],maxindex:0} } 
            },
            Indexed => {
                let last = self.data.len()-1;
                let firstval = self.data[self.index[0]];
                let lastval = self.data[self.index[last]];
                if self.ascending { MinMax{min:firstval,minindex:self.index[0],max:lastval,maxindex:self.index[last]} }
                else { MinMax{min:lastval,minindex:self.index[last],max:firstval,maxindex:self.index[0]} }
            }, 
            Ranked => {
                let last = self.data.len()-1;
                let si = self.index.invindex(); // ranks -> sort index
                let firstval = self.data[si[0]];
                let lastval = self.data[si[last]];
                    if self.ascending { MinMax{min:firstval,minindex:si[0],max:lastval,maxindex:si[last]} }
                    else { MinMax{min:lastval,minindex:si[last],max:firstval,maxindex:si[0]} }
            }
        }      
    }
    
    /// Search a Set self for m.
    /// Returns the subscript of the first m or None   
    pub fn search(&self, m: T) -> Option<usize> { 
        match self.stype {
            Empty => None,
            Unordered => self.data.member(m), // from indxvec ,
            Ordered => if self.ascending { self.data.memsearch(m)}
                else {self.data.memsearchdesc(m)},     
            Indexed => if self.ascending { self.data.memsearch_indexed(&self.index,m) }
                else { self.data.memsearchdesc_indexed(&self.index,m) },
            Ranked => if self.ascending { self.data.memsearch_indexed(&self.index.invindex(),m) }
                else { self.data.memsearchdesc_indexed(&self.index.invindex(),m) }, 
            }       
    }       
    
    /// True if m is a member of the set
    /// Throws away the subscript found by `search`
    pub fn member(&self, m: T) -> bool {
        self.search(m).is_some() 
    }
    
    /// Mostly for non-members. Index of the next item in order, or self.len(). 
    /// Unordered sets return self.data.len() as 'not found'.
    pub fn position(&self, m:T)  -> usize {
        match self.stype {
            Empty => 0_usize,
            Unordered => self.data.len(),
            Ordered => if self.ascending { self.data.binsearch(m)}
                else {self.data.binsearchdesc(m)},     
            Indexed => if self.ascending { self.data.binsearch_indexed(&self.index,m) }
                else { self.data.binsearchdesc_indexed(&self.index,m) },
            Ranked => if self.ascending { self.data.binsearch_indexed(&self.index.invindex(),m) }
            else { self.data.binsearchdesc_indexed(&self.index.invindex(),m) },   
        }
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
