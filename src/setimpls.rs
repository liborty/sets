use crate::{SType,Set,MutSetOps,trivindex};
use indxvec::{MinMax,Indices,Vecops};

/// Associated functions for conversions and self operations returning Set<T> = Self
impl<T> Set<T> where T: Copy+PartialOrd+Default {

    /// Associated constant EMPTYSET, unique for each concrete end-type T
    pub const EMPTYSET:Set<T> = Set{ stype:SType::Empty, ascending:true, data:Vec::new(), index:Vec::new() };

    /// all in one Initialiser creates a new Set
    /// of self_type, from slice d, in asc order 
    pub fn new(set_type: SType, d: &[T], asc:bool) -> Self {  
        if d.is_empty() { return Set::EMPTYSET }; // no data
        match set_type {
            SType::Empty => Set::EMPTYSET, // empty self specified
            SType::Unordered => Set{ stype:SType::Unordered, ascending:true, data:d.to_vec(), index:Vec::new() }, 
            SType::Ordered => Set{ stype:SType::Ordered, ascending:asc, data:d.sortm(asc), index:Vec::new() },
            SType::Indexed =>  Set{ stype:SType::Indexed, ascending:asc, data:d.to_vec(), 
                index: if asc { d.mergesort_indexed() } else { d.mergesort_indexed().revs() } },
            SType::Ranked => Set{ stype:SType::Ranked, ascending:asc, data:d.to_vec(), 
                index: if asc { d.mergesort_indexed().invindex() } else { d.mergesort_indexed().revs().invindex() } } }
    }

    /// Creates a new empty Set
    pub fn new_empty() -> Self { Self::EMPTYSET }

    /// Initialiser - creates a new SType::Unordered Set from data
    pub fn new_unordered(d: &[T]) -> Self {  
        if !d.is_empty() { // have some data
            Set{ stype:SType::Unordered, ascending:true, data:d.to_vec(), index:Vec::new() } } 
        else { Set::EMPTYSET } 
    }

    /// Initialiser - creates a new SType::Ordered Set in asc order from data 
    pub fn new_ordered(d: &[T], asc:bool) -> Self {  
        if !d.is_empty() { // have some data
            Set{ stype:SType::Ordered, ascending:asc, data:d.sortm(asc), index:Vec::new() } }
        else { Set::EMPTYSET } 
    }

    /// Initialiser - creates a new SType::Indexed Set in asc order from data 
    pub fn new_indexed(d: &[T], asc:bool) -> Self {  
        if !d.is_empty() { // have some data
            Set{ stype:SType::Indexed, ascending:asc, data:d.to_vec(), 
                index: if asc { d.mergesort_indexed() } else { d.mergesort_indexed().revs() } } }
        else { Set::EMPTYSET } 
    }

    /// Initialiser - creates a new SType::Ranked Set in asc order from data 
    pub fn new_ranked(d: &[T], asc:bool) -> Self {  
        if !d.is_empty() { // have some data
            Set{ stype:SType::Ranked, ascending:asc, data:d.to_vec(), 
                index: if asc { d.mergesort_indexed().invindex() } else { d.mergesort_indexed().revs().invindex() } } }
        else { Set::EMPTYSET } 
    }

    /// Converter - to SType::Unordered Set
    /// Caution: this just throws away the valuable index!
    pub fn to_unordered(&self) -> Self { 
        match self.stype {
            SType::Empty => Set::EMPTYSET, // no op 
            // ascending field has no meaning for unordered, so just inherit it
            _ => Self{ stype:SType::Unordered, ascending:self.ascending, data:self.data.clone(), index:Vec::new() }
        }
    }

    /// Converts any Set type to ordered
    pub fn to_ordered(&self, asc:bool) -> Self {
        match self.stype {
            SType::Empty => Set::EMPTYSET, 
            SType::Unordered => Self{ stype:SType::Ordered, ascending:asc, data:self.data.sortm(asc), index:Vec::new()},
            SType::Ordered => if self.ascending == asc { self.clone() } // just a copy
                else { Self{ stype:SType::Ordered, ascending:asc, data:self.data.revs(), index:Vec::new() } },
            SType::Indexed => Self{ stype:SType::Ordered, ascending:asc, 
                data:self.index.unindex(&self.data, self.ascending == asc), index:Vec::new() },
            SType::Ranked => Self{ stype:SType::Ordered, ascending:asc, 
                data:self.index.invindex().unindex(&self.data, self.ascending == asc), index:Vec::new() },
        }    
    }

    /// Converts any Set type to indexed
    pub fn to_indexed(&self,asc:bool) -> Self {
        match self.stype {
            SType::Empty => Set::EMPTYSET,
            SType::Unordered => Self{ stype:SType::Indexed, ascending:asc, data:self.data.clone(), 
                index: if asc {self.data.mergesort_indexed()} else {self.data.mergesort_indexed().revs()} },
            SType::Ordered => Self{ stype:SType::Indexed, ascending:asc, data:self.data.clone(), 
                index: trivindex(self.ascending == asc,self.data.len()) },
            SType::Indexed =>  if self.ascending == asc { self.clone() } // no op
                else { Self{ stype:SType::Indexed, ascending:asc, data:self.data.clone(),
                    index: self.index.revs() } },
            SType::Ranked => Self{ stype:SType::Indexed, ascending:asc, data:self.data.clone(),             
                index: if self.ascending == asc {self.index.invindex()} else {self.index.invindex().revs()}}
        }    
    }

    /// Converts any Set type to ranked
    pub fn to_ranked(&self,asc:bool) -> Self {
        match self.stype {
            SType::Empty => Set::EMPTYSET,
            SType::Unordered => Self{ stype:SType::Ranked, ascending:asc, data:self.data.clone(), 
                index: if asc {self.data.mergesort_indexed().invindex()} 
                    else {self.data.mergesort_indexed().revs().invindex()} },
            SType::Ordered => Self{ stype:SType::Ranked, ascending:asc, data:self.data.clone(), 
                index: trivindex(self.ascending == asc,self.data.len()) },
            SType::Indexed => Self{ stype:SType::Ranked, ascending:asc, data:self.data.clone(),             
                index: if self.ascending == asc {self.index.invindex()} 
                    else {self.index.revs().invindex()}}, 
            SType::Ranked => if self.ascending == asc { self.clone() } // no op
                else { Self{ stype:SType::Ranked, ascending:asc, data:self.data.clone(),
                    index: {self.index.complindex()} } }
        }    
    }

    /// General converter: s -> Set of the same type and order as self
    /// self only serves as a template for the type and order and is not involved in the conversion
    pub fn to_same(&self, s:&Self) -> Self { 
        match self.stype { 
            SType::Empty => Set::EMPTYSET, //  was Default::default()
            SType::Unordered => s.to_unordered(), 
            SType::Ordered => s.to_ordered(self.ascending),
            SType::Indexed => s.to_indexed(self.ascending),
            SType::Ranked => s.to_ranked(self.ascending)
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
        if scopy.mdelete(item) { scopy } else { self.clone() }
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

    /// Union of two selfs  
    pub fn union(&self, s: &Self) -> Self {
        let mut scopy =  self.clone();
        scopy.munion(s);
        scopy
    }
    
    /// Intersection of two selfs
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
            SType::Empty => Default::default(),
            SType::Unordered => self.data.minmax(),  
            SType::Ordered => {
                let last = self.data.len()-1;
                if self.ascending { MinMax{min:self.data[0],minindex:0,max:self.data[last],maxindex:last} }
                else { MinMax{min:self.data[last],minindex:last,max:self.data[0],maxindex:0} } 
            },
            SType::Indexed => {
                let last = self.data.len()-1;
                let firstval = self.data[self.index[0]];
                let lastval = self.data[self.index[last]];
                if self.ascending { MinMax{min:firstval,minindex:self.index[0],max:lastval,maxindex:self.index[last]} }
                else { MinMax{min:lastval,minindex:self.index[last],max:firstval,maxindex:self.index[0]} }
            }, 
            SType::Ranked => {
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
            SType::Empty => None,
            SType::Unordered => self.data.member(m), // from indxvec ,
            SType::Ordered => if self.ascending { self.data.memsearch(m)}
                else {self.data.memsearchdesc(m)},     
            SType::Indexed => if self.ascending { self.data.memsearch_indexed(&self.index,m) }
                else { self.data.memsearchdesc_indexed(&self.index,m) },
            SType::Ranked => if self.ascending { self.data.memsearch_indexed(&self.index.invindex(),m) }
                else { self.data.memsearchdesc_indexed(&self.index.invindex(),m) }, 
            }       
    }       
    
    /// True if m is a member of the self
    /// Throws away the subscript found by `search`
    pub fn member(&self, m: T) -> bool {
        self.search(m).is_some() 
    }
    
    /// Mostly for non-members. Index of the next item in order, or self.len(). 
    /// SType::Unordered selfs return self.data.len() as 'not found'.
    pub fn position(&self, m:T)  -> usize {
        match self.stype {
            SType::Empty => 0_usize,
            SType::Unordered => self.data.len(),
            SType::Ordered => if self.ascending { self.data.binsearch(m)}
                else {self.data.binsearchdesc(m)},     
            SType::Indexed => if self.ascending { self.data.binsearch_indexed(&self.index,m) }
                else { self.data.binsearchdesc_indexed(&self.index,m) },
            SType::Ranked => if self.ascending { self.data.binsearch_indexed(&self.index.invindex(),m) }
            else { self.data.binsearchdesc_indexed(&self.index.invindex(),m) },   
        }
    }   
}