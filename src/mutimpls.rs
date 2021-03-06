#![warn(missing_docs)]
use crate::{trivindex,SType,Set,MutSetOps};
use indxvec::{F64,Indices,Vecops,Mutops};

impl<T> MutSetOps<T> for Set<T> where T:Copy+PartialOrd+Default {

    /// Makes a Set unordered
    /// Caution: this just throws away the valuable index!
    fn munordered(&mut self) { 
        match self.stype {
            SType::Empty | SType::Unordered => return, // no op
            SType::Ordered => (), // leave data as is, just change SType below
            SType::Indexed | SType::Ranked  => self.index = Vec::new() // remove the index
        }
        self.stype = SType::Unordered;
        // ascending field has no meaning for unordered, so leaving it as it is 
    }

    /// Makes a Set ordered
    fn mordered(&mut self, asc:bool) where F64:From<T> {
        match self.stype {
            SType::Empty => return, // no op
            SType::Unordered => { self.data.muthashsort(); if !asc { self.data.mutrevs() } },
            SType::Ordered => if self.ascending != asc { self.data.mutrevs() }, 
            SType::Indexed => { 
                self.data = self.index.unindex(&self.data, self.ascending == asc);
                self.index = Vec::new(); },
            SType::Ranked => {
                self.data = self.index.invindex().unindex(&self.data, self.ascending == asc);
                self.index = Vec::new(); }
        } 
        self.stype = SType::Ordered; // new SType 
        self.ascending = asc;  // new ordering    
    }

    /// Makes any Set indexed
    fn mindexed(&mut self,asc:bool) where F64:From<T> { 
        match self.stype { 
            SType::Empty => return, // empty set, no op 
            SType::Unordered => {                 
                self.index = self.data.hashsort_indexed();
                if !asc { self.index.mutrevs(); }; },
            SType::Ordered => self.index = trivindex(self.ascending == asc,self.data.len()),
            SType::Indexed => if self.ascending != asc { self.index.mutrevs() },
            SType::Ranked => {
                if self.ascending != asc { self.index = self.index.complindex() }; 
                self.index = self.index.invindex(); }, 
        }
        self.stype = SType::Indexed; // new SType 
        self.ascending = asc;  // new ordering 
    }

    /// Converts any Set type to ranked
    fn mranked(&mut self,asc:bool) {
        match self.stype {
            SType::Empty => return, // empty set, no op 
            SType::Unordered =>  {                 
                self.index = self.data.mergesort_indexed().invindex();
                if !asc { self.index.complindex(); }; },
            SType::Ordered => self.index = trivindex(self.ascending == asc,self.data.len()),
            SType::Indexed => {
                if self.ascending != asc { self.index.mutrevs() }; 
                self.index = self.index.invindex(); }, 
            SType::Ranked => if self.ascending != asc { self.index = self.index.complindex() }
        } 
        self.stype = SType::Ranked; // new SType 
        self.ascending = asc;  // new ordering    
    }

    /// General converter: s -> Set of the same type and order as self
    /// self only serves as a template for the type and order and is not involved in the conversion
    fn msame(&mut self, s:&mut Self) where F64:From<T> { 
        match self.stype { 
            SType::Empty => *s = Set::EMPTYSET, //  was Default::default()
            SType::Unordered => s.munordered(), 
            SType::Ordered => s.mordered(self.ascending),
            SType::Indexed => s.mindexed(self.ascending),
            SType::Ranked => s.mranked(self.ascending)
        }
    }       

    /// Deletes an item v of the same end-type from self
    /// Returns false if item not found 
    fn mdelete(&mut self, item:T) -> bool where Self:Sized {
        match self.stype {
            SType::Empty => false, // empty set
            SType::Unordered => {
                if let Some(i) = self.search(item) {
                    // don't care about order, swap_remove swaps in the last item, fast
                    self.data.swap_remove(i); true }
                else { false }
            }, 
            SType::Ordered => {
                if let Some(i) = self.search(item) {
                    self.data.remove(i); true } // preserve ordering
                else { false }  
            },
            SType::Indexed => {
                let mut rankindex = self.index.invindex();
                if let Some(ix) = if self.ascending { 
                    self.data.memsearch_indexed(&self.index,item) }
                else { self.data.memsearchdesc_indexed(&self.index,item) } 
                {  // if let success      
                    self.data.remove(self.index[ix]); 
                    rankindex.remove(self.index[ix]);
                    // repare the whole rank index
                    if self.ascending {
                        for (j,&val) in self.data.iter().enumerate() { 
                            if val > item { rankindex[j] -= 1 };
                        } 
                    }
                    else {
                        for (j,&val) in self.data.iter().enumerate() { 
                            if val < item { rankindex[j] -= 1 };                
                        }
                    }
                    self.index = rankindex.invindex();
                    true 
                } 
                else { false } // if let failure
            },
            SType::Ranked => {
                let sortindex = self.index.invindex();
                if let Some(ix) = if self.ascending { 
                    self.data.memsearch_indexed(&sortindex,item) }
                else { self.data.memsearchdesc_indexed(&sortindex,item) } 
                {   // memsearch(desc) suceeded, finding subscript ix of item    
                    self.data.remove(sortindex[ix]); 
                    // rank index is also in data order
                    self.index.remove(sortindex[ix]); 
                    // repare the whole rank index
                    if self.ascending {
                        for (j,&val) in self.data.iter().enumerate() { 
                            if val > item { self.index[j] -= 1 };
                        } 
                    }
                    else {
                        for (j,&val) in self.data.iter().enumerate() { 
                            if val < item { self.index[j] -= 1 };                
                        }
                    }
                    true 
                } 
                else { false } // memsearch(desc) failed
            } 
        }
    }  

    /// Inserts an item v of the same end-type to self
    fn minsert(&mut self, item:T) {
        match self.stype {
            SType::Empty => {  // initially empty set
                self.stype = crate::SType::Ordered;
                self.data.push(item);
            },
            SType::Unordered => self.data.push(item), 
            SType::Ordered => {
                // binsearch finds the right sort position
                let i = if self.ascending { self.data.binsearch(item) }
                else { self.data.binsearchdesc(item) };
                self.data.insert(i,item); // shifts the rest  
            },
            SType::Indexed => {
                let ix = if self.ascending { self.data.binsearch_indexed(&self.index,item) }
                else { self.data.binsearchdesc_indexed(&self.index,item) };
                // simply push the item to the end of unordered data self.data
                self.data.push(item);
                // and insert its subscipt into the right place ix in the sort index    
                self.index.insert(ix,self.data.len()-1);                

            }
            SType::Ranked => {
               // have to invert the rank index to get the required sort index
                let ix = if self.ascending { self.data.binsearch_indexed(&self.index.invindex(),item) }
                else { self.data.binsearchdesc_indexed(&self.index.invindex(),item) };
                // simply push the new item to the end of unordered data self.data
                self.data.push(item);
               // and insert its subscipt into the same place in the rank index    
                self.index.push(ix);
            }
        };
    }

    /// Reverses a vec by iterating over only half of its length
    /// and swapping the items
    fn mreverse(&mut self) { 
        match self.stype {
            SType::Empty => Default::default(), // empty set
            SType::Unordered => self.data.mutrevs(), 
            SType::Ordered => {        
                self.ascending = !self.ascending;
                self.data.mutrevs(); 
            },
            SType::Indexed => {
                self.ascending = !self.ascending;
                self.index.mutrevs(); 
            },
            SType::Ranked => {
                self.ascending = !self.ascending;
                self.index = self.index.complindex();                
            }
        }
    }

    /// Deletes any repetitions
    fn mnonrepeat(&mut self) {
        if self.data.len() < 2 { return }; // nothing to be done here
        match self.stype {
            SType::Empty => (), // empty set, do nothing
            SType::Unordered => { // sort data 
                self.data = self.data.sortm(true);
                self.data.dedup(); 
            }, 
            SType::Ordered =>  self.data.dedup(),
            SType::Indexed => { // spoofed by sorted data and trivial index
                let mut orddata = self.index.unindex(&self.data,self.ascending);
                orddata.dedup();
                self.data = orddata; // resets data to ordered
                self.index = trivindex(self.ascending, self.data.len());
            },
            SType::Ranked => { // spoofed by sorted data and trivial index
                let mut orddata = self.index.invindex().unindex(&self.data,self.ascending);
                orddata.dedup();
                self.data = orddata; // resets data to ordered
                self.index = trivindex(self.ascending, self.data.len());       
            }
        }
    }

    /// sets union
    fn munion(&mut self, s: &Self) {
        let mut selford = self.to_ordered(true);
        let sord = s.to_ordered(true);
        selford.data = selford.data.merge(&sord.data);
        *self = self.to_same(&selford); // back to original type and order 
    }

    /// Intersection of two unordered sets, assigned to self
    fn mintersection(&mut self, s: &Self) {
        let mut selford = self.to_ordered(true);
        let sord = s.to_ordered(true);
        selford.data = selford.data.intersect(&sord.data);
        *self = self.to_same(&selford); // back to original type and order 
    }

    /// Complement of s in self (i.e. self -= s)
    fn mdifference(&mut self, s: &Self) {
        let mut selford = self.to_ordered(true);
        let sord = s.to_ordered(true);
        selford.data = selford.data.diff(&sord.data);
        *self = self.to_same(&selford); // back to original type and order
    }    
}

/*
impl<T> MutSetOps<T> for OrderedSet<T> where T:Copy+PartialOrd {

    /// Reverses a vec by iterating over only half of its length
    /// and swapping the items
    fn mreverse(&mut self) { 
        self.ascending = !self.ascending;
        let n = self.data.len();
        for i in 0..n/2 { self.swap(i,n-i-1) } 
    }

    /// Deletes any repetitions
    fn mnonrepeat(&mut self) { self.data = self.data.sansrepeat() }  

    /// Ascending union of two ordered sets, reassigned to self 
    fn munion(&mut self, s: &Self) { 
        // the result will be always ascending
        if !self.ascending { self.ascending = true; self.data = self.data.revs() };
        if s.ascending { self.data = self.data.merge(&s.data) } 

        else { self.data = self.data.merge(&s.data.revs()); }; 
    } 

    /// Ascending intersection of two sets, assigned to the self 
    fn mintersection(&mut self, s: &Self) {
        // the result will be always ascending
        if !self.ascending { self.ascending = true; self.data = self.data.revs() }; 
        if s.ascending { self.data = self.data.intersect(&s.data )}
        else { self.data = self.data.intersect(&s.data.revs()) };  
    }

    /// Ascending complement of s in self (i.e. self-s)
    fn mdifference(&mut self, s: &Self) {
        // the result will be always ascending
        if !self.ascending { self.ascending = true;  self.data = self.data.revs() };
        if s.ascending { self.data  = self.data.diff(&s.data) }
        else { self.data  = self.data.diff(&s.data.revs()) };
    }
}

/// These are generally better than OrderedSet(s) for bulky end types, as
/// there is no moving of data around.
impl<T> MutSetOps<T> for IndexedSet<T> where T: Copy+PartialOrd {
    


    /// Union of two IndexedSets reassigned to self.  
    /// Will be always ascending ordered.  
    fn munion(&mut self, s: &Self) {         
        if self.ascending {         
            if s.ascending { (self.data,self.index) = self.data.merge_indexed(&self.index,&s.data, &s.index) }
            else { (self.data,self.index) = self.data.merge_indexed(&self.index, &s.data, &s.index.revindex() ) }     
        }
        else {
            self.ascending = true; 
            if s.ascending { (self.data,self.index) = self.data.merge_indexed( &self.index.revindex(),&s.data,&s.index) } 
            else { (self.data,self.index) = self.data.merge_indexed(&self.index.revindex(), &s.data, &s.index.revindex()) }  
        }
    }      
    
    /// Intersection of two IndexedSets
    fn mintersection(&mut self, s: &Self) {
        if self.ascending {
            if s.ascending { self.data = self.data.indexntersect_indexed(&self.index,&s.data, &s.index) }
            else { self.data = self.data.indexntersect_indexed(&self.index,&s.data, &s.index.revindex() ) }     
        }
        else {
            self.ascending = true; 
            if s.ascending { self.data = self.data.indexntersect_indexed(&self.index.revindex(),&s.data,&s.index) } 
            else { self.data = self.data.indexntersect_indexed(&self.index.revindex(), &s.data, &s.index.revindex()) }  
        }
        // result index will be of the new size but in all cases trivial and ascending
        self.index = trivindex(true,self.data.len()); 
    }
    
    /// Complement of s in self (i.e. self-s)
    fn mdifference(&mut self, s: &Self) {
        if self.ascending {
            if s.ascending { self.data = self.data.diff_indexed(&self.index,&s.data, &s.index) }
            else { self.data = self.data.diff_indexed(&self.index,&s.data, &s.index.revindex() ) }     
        }
        else {
            self.ascending = true; 
            if s.ascending { self.data = self.data.indexntersect_indexed(&self.index.revindex(),&s.data,&s.index) } 
            else { self.data = self.data.indexntersect_indexed(&self.index.revindex(), &s.data, &s.index.revindex()) }  
        }
        // result index will be of the new size but in all cases trivial and ascending
        self.index = trivindex(true,self.data.len()); 
    }
}

/// The primitive functions from `indxvec` all expect indexed sets, 
/// so for now we convert from ranks to sort indices using `.indexnvindex()`.
/// Even though that is a simple operation, for lots of set operations, 
/// it will be slightly quicker to work in IndexedSet(s) 
/// and only to rank the final result.
impl<T> MutSetOps<T> for RankedSet<T> where T: Copy+PartialOrd {

    /// Inserts an item v of the same end-type to self
    fn minsert(&mut self, item:T) {

    }
    
    /// just make the ranks descending
    fn mreverse(&mut self) {
        self.ascending = !self.ascending;
        self.index = self.index.complindex() // `complindex` reverses the ranks
    }
        
    /// deletes repetitions.
    fn mnonrepeat(&mut self) { 
        let clean = self.data.sansrepeat();
        self.data = clean.to_vec();
        // rebuild the ranks (can do better)
        self.index = clean.rank(self.ascending)      
    } 
    
    /// Union of two RankedSets. 
    /// Converts ranks to sort indices with `invindex`, merges, then converts back to ranks
    /// Data `self.data` is simply concatenated
    fn munion(&mut self, s: &Self) {
        if !self.ascending { self.ascending = true; self.index = self.index.complindex() }
        if s.ascending { (self.data,self.index) = 
            self.data.merge_indexed( &self.index.indexnvindex(), &s.data, &s.index.indexnvindex() ); }
        else { (self.data,self.index) = 
            self.data.merge_indexed( &self.index.indexnvindex(), &s.data, &s.index.indexnvindex().revindex() ); };        
        self.index = self.index.indexnvindex(); // invert back to ranks index
    }      
        
    /// Intersection of two RankedSets
    fn mintersection(&mut self, s: &Self) {
        if self.ascending {
            if s.ascending { self.data = self.data.indexntersect_indexed(&self.index.indexnvindex(),&s.data, &s.index.indexnvindex()) }
            else { self.data = self.data.indexntersect_indexed(  &self.index.indexnvindex(),&s.data, &s.index.indexnvindex().revindex() ) }     
        }
        else {
            self.ascending = true; 
            if s.ascending { self.data = 
                self.data.indexntersect_indexed( &self.index.indexnvindex().revindex(),&s.data,&s.index.indexnvindex()) } 
            else { self.data = self.data.indexntersect_indexed( &self.index.indexnvindex().revindex(), &s.data, &s.index.indexnvindex().revindex()) }  
        }
        // result ranks will be of the new size but in all cases trivial and ascending
        self.index = trivindex(true,self.data.len()); 
    }
        
    /// Complement of s in self (i.e. self-s)
    fn mdifference(&mut self, s: &Self) {
        if self.ascending {
            if s.ascending { self.data = self.data.diff_indexed(&self.index.indexnvindex(),&s.data, &s.index.indexnvindex()) }
            else { self.data = self.data.diff_indexed( &self.index.indexnvindex(),&s.data, &s.index.indexnvindex().revindex() ) }     
        }
        else {
            self.ascending = true; 
            if s.ascending { self.data = self.data.diff_indexed( &self.index.indexnvindex().revindex(),&s.data,&s.index.indexnvindex()) } 
            else { self.data = self.data.diff_indexed( &self.index.indexnvindex().revindex(), &s.data, &s.index.indexnvindex().revindex()) }  
        }
        // result ranks will be of the new size, trivial and ascending
        self.index = trivindex(true,self.data.len()); 
    }
}
*/