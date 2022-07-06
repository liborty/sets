use crate::{trivindex,Set,MutSetOps};
use indxvec::{Indices,Vecops};

impl<T> MutSetOps<T> for Set<T> where T:Copy+PartialOrd {

    /// Deletes an item v of the same end-type from self
    /// Returns false if item not found 
    fn mdelete(&mut self, item:T) -> bool
        where Self:Sized,T:Copy+PartialOrd {
        if let Some(i) = self.search(item) {
        // don't care about order, swap_remove swaps in the last item, fast
        self.data.swap_remove(i); true }
        else { false }
    }  

    /// Inserts an item v of the same end-type to self
    fn minsert(&mut self, item:T) {
        // unordered set, so just pushing v to the end
        self.push(item)  
    }

    /// Reverses a vec by iterating over only half of its length
    /// and swapping the items
    fn mreverse(&mut self) { 
        let n = self.data.len();
        for i in 0..n/2 { self.swap(i,n-i-1) }
    }

    /// Deletes any repetitions
    fn mnonrepeat(&mut self) { self.data = self.data.sansrepeat() }

    /// Union of two unordered sets, concat assigned to self  
    fn munion(&mut self, s: &Self) {
        self.data = self.unite_unsorted(s); 
    }

    /// Intersection of two unordered sets, assigned to self
    fn mintersection(&mut self, s: &Self) {
        let s1 = self.sortm(true);
        let s2 = s.sortm(true);
        self.data = s1.intersect(&s2); 
    }

    /// Complement of s in self (i.e. self -= s)
    fn mdifference(&mut self, s: &Self) {
        let s1 = self.sortm(true);
        let s2 = s.sortm(true); 
        self.data = s1.diff(&s2);  
    }    
}

impl<T> MutSetOps<T> for OrderedSet<T> where T:Copy+PartialOrd {

    /// Deletes an item v of the same end-type from self
    /// Returns false if item not found 
    fn mdelete(&mut self, item:T) -> bool {
        if let Some(i) = self.search(item) {
        self.data.remove(i); true } // preserve ordering
        else { false }
    }  

    /// Inserts an item v of the same end-type to self
    fn minsert(&mut self, item:T) {
        // binsearch finds the right sort position
        let i = if self.ascending { self.binsearch(item) }
        else { self.binsearchdesc(item) };
        self.data.indexnsert(i,item); 
    }

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
        if s.ascending { self.data = self.data.indexntersect(&s.data )}
        else { self.data = self.data.indexntersect(&s.data.revs()) };  
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

    /// Deletes an item v of the same end-type from self
    /// Returns false if the item is not found 
    fn mdelete(&mut self, item:T) -> bool {
        let mut rankindex = self.index.indexnvindex();
        if let Some(ix) = if self.ascending { 
            self.data.memsearch_indexed(&self.index,item) }
        else { self.data.memsearchdesc_indexed(&self.index,item) } 
        {        
            self.data.remove(self.index[ix]); 
            rankindex.remove(self.index[ix]);
            // repare the whole rank index
            if self.ascending {
                for (j,&val) in self.data.indexter().enumerate() { 
                    if val > item { rankindex[j] -= 1 };
                } 
            }
            else {
                for (j,&val) in self.data.indexter().enumerate() { 
                    if val < item { rankindex[j] -= 1 };                
                }
            }
            self.index = rankindex.indexnvindex();
            true 
        } 
        else { false }
    }

    /// Inserts an item v of the same end-type to self
    fn minsert(&mut self, item:T) {
        let ix = if self.ascending { self.data.binsearch_indexed(&self.index,item) }
        else { self.data.binsearchdesc_indexed(&self.index,item) };
        // simply push the item to the end of unordered data self.data
        self.data.push(item);
        // and insert its subscipt into the right place ix in the sort index    
        self.index.indexnsert(ix,self.data.len()-1);
    }

    /// just reverse the index
    fn mreverse(&mut self) {
        self.ascending = !self.ascending;
        self.index = self.index.revindex(); 
        }
    
    /// deletes repetitions.
    fn mnonrepeat(&mut self) { 
        self.data = self.data.sansrepeat();
        self.index = self.data.sortidx();       
    } 

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

    /// Deletes an item v of the same end-type from self
    /// Returns false if the item is not found 
    fn mdelete(&mut self, item:T) -> bool {
        let sortindex = self.index.indexnvindex();
        if let Some(ix) = if self.ascending { 
            self.data.memsearch_indexed(&sortindex,item) }
        else { self.data.memsearchdesc_indexed(&sortindex,item) } 
        {        
            self.data.remove(sortindex[ix]); 
            self.index.remove(sortindex[ix]);
            // repare the whole rank index
            if self.ascending {
                for (j,&val) in self.data.indexter().enumerate() { 
                    if val > item { self.index[j] -= 1 };
                } 
            }
            else {
                for (j,&val) in self.data.indexter().enumerate() { 
                    if val < item { self.index[j] -= 1 };                
                }
            }
            true 
        } 
        else { false }
    }  

    /// Inserts an item v of the same end-type to self
    fn minsert(&mut self, item:T) {
        // have to invert the rank index to get the required sort index
        let ix = if self.ascending { self.data.binsearch_indexed(&self.index.indexnvindex(),item) }
        else { self.data.binsearchdesc_indexed(&self.index.indexnvindex(),item) };
        // simply push the new item to the end of unordered data self.data
        self.data.push(item);
        // and insert its subscipt into the same place in the rank index    
        self.index.push(ix);
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