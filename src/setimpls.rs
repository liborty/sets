use crate::{SType,Set,MutSetOps,trivindex};
use indxvec::{MinMax,Indices,Vecops};

/// Associated functions for conversions and set operations returning Set<T> = Self
impl<T> Set<T> where T: Copy+PartialOrd+Default {

    /// Initialiser - creates a new SType::Unordered Vec
    pub fn unordered_from_slice(s: &[T]) -> Self { 
        let mut newset = Set::default(); // returns this SType::Empty set for zero lentgh s
        if !s.is_empty() { // have some data, so modify the newset with it
            newset.stype = SType::Unordered;
            newset.data = s.to_vec(); 
        }; 
        newset
    }

    /// Converter - to SType::Unordered Set
    /// Caution: this just throws away the valuable index!
    pub fn to_unordered(&self) -> Self { 
        match self.stype {
            SType::Empty => self.clone(), // SType::Empty set is unique
            SType::Unordered => self.clone(), // no op
            SType::Ordered => Self{ stype:SType::Unordered, ascending:true, data:self.data.clone(), index:vec![] },
            SType::Indexed => Self{ stype:SType::Unordered, ascending:true, data:self.data.clone(), index:vec![] }, 
            SType::Ranked => Self{ stype:SType::Unordered, ascending:true, data:self.data.clone(), index:vec![] }
        }
    }

    /// Converts any Set type to ordered
    pub fn to_ordered(&self,asc:bool) -> Self {
        match self.stype {
            SType::Empty => self.clone(), 
            SType::Unordered => Self{ stype:SType::Ordered, ascending:asc, data:self.data.sortm(asc), index:vec![] },
            SType::Ordered => self.clone(), // no op
            SType::Indexed => Self{ stype:SType::Ordered, ascending:asc, 
                data:self.index.unindex(&self.data, asc),
                index:vec![] },
            SType::Ranked => Self{ stype:SType::Ordered, ascending:asc, 
                data:self.index.invindex().unindex(&self.data, asc),
                index:vec![] },
        }    
    }

    /// Converts any Set type to indexed
    pub fn to_indexed(&self,asc:bool) -> Self {
        match self.stype {
            SType::Empty => self.clone(),
            SType::Unordered => Self{ stype:SType::Indexed, ascending:asc, data:self.data.clone(), 
                index: if asc {self.data.sortidx()} else {self.data.sortidx().revs()} },
            SType::Ordered => Self{ stype:SType::Indexed, ascending:asc, data:self.data.clone(), 
                index: trivindex(self.ascending == asc,self.data.len()) },
            SType::Indexed => self.clone(), 
            SType::Ranked => Self{ stype:SType::Indexed, ascending:asc, data:self.data.clone(),             
                index: if self.ascending == asc {self.index.invindex()} 
                    else {self.index.invindex().revs()}}
        }    
    }

    /// Converts any Set type to ranked
    pub fn to_ranked(&self,asc:bool) -> Self {
        match self.stype {
            SType::Empty => self.clone(),
            SType::Unordered => Self{ stype:SType::Ranked, ascending:asc, data:self.data.clone(), 
                index: if asc {self.data.sortidx().invindex()} 
                    else {self.data.sortidx().revs().invindex()} },
            SType::Ordered => Self{ stype:SType::Ranked, ascending:asc, data:self.data.clone(), 
                index: trivindex(self.ascending == asc,self.data.len()) },
            SType::Indexed => Self{ stype:SType::Ranked, ascending:asc, data:self.data.clone(),             
                index: if self.ascending == asc {self.index.invindex()} 
                    else {self.index.revs().invindex()}}, 
            SType::Ranked => self.clone()
        }    
    }

    /// General converter: s -> Set of the same type and order as self
    pub fn to_self(&self,s: &Self) -> Self { 
        // if self.stype = s.stype { return *s }; // nothing to do
        match self.stype {
            SType::Empty => Default::default(), // SType::Empty set
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
            SType::Empty => Default::default(),
            SType::Unordered => self.data.minmax(),  
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
            SType::Empty => None,
            SType::Unordered => self.data.member(m), // from indxvec ,
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
    /// SType::Unordered sets return self.data.len() as 'not found'.
    pub fn position(&self, m:T)  -> usize {
        match self.stype {
            SType::Empty => 0_usize,
            SType::Unordered => self.data.len(),
            Ordered => if self.ascending { self.data.binsearch(m)}
                else {self.data.binsearchdesc(m)},     
            Indexed => if self.ascending { self.data.binsearch_indexed(&self.index,m) }
                else { self.data.binsearchdesc_indexed(&self.index,m) },
            Ranked => if self.ascending { self.data.binsearch_indexed(&self.index.invindex(),m) }
            else { self.data.binsearchdesc_indexed(&self.index.invindex(),m) },   
        }
    }   
}