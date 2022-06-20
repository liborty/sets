use crate::{trivindex,Set,OrderedSet,IndexedSet,RankedSet,MutSetOps};
use indxvec::{Indices,Vecops};

impl<T> MutSetOps<T> for Set<T> where T:Copy+PartialOrd {

    /// Reverses a vec by iterating over only half of its length
    /// and swapping the items
    fn mreverse(&mut self) { 
        let n = self.v.len();
        for i in 0..n/2 {
            let temp = self.v[i];
            self.v[i] = self.v[n-i-1];
            self.v[n-i-1] = temp } 
    }

    /// Deletes any repetitions
    fn mnonrepeat(&mut self) { self.v = self.v.sansrepeat() }

    /// Union of two unordered sets, concat assigned to self  
    fn munion(&mut self, s: &Self) {
        self.v = self.unite_unsorted(s); 
    }

    /// Intersection of two unordered sets, assigned to self
    fn mintersection(&mut self, s: &Self) {
        let s1 = self.sortm(true);
        let s2 = s.sortm(true);
        self.v = s1.intersect(&s2); 
    }

    /// Complement of s in self (i.e. self -= s)
    fn mdifference(&mut self, s: &Self) {
        let s1 = self.sortm(true);
        let s2 = s.sortm(true); 
        self.v = s1.diff(&s2);  
    }    
}

impl<T> MutSetOps<T> for OrderedSet<T> where T:Copy+PartialOrd {

    /// Reverses a vec by iterating over only half of its length
    /// and swapping the items
    fn mreverse(&mut self) { 
        self.ascending = !self.ascending;
        let n = self.v.len();
        for i in 0..n/2 {
            let temp = self.v[i];
            self.v[i] = self.v[n-i-1];
            self.v[n-i-1] = temp } 
    }

    /// Deletes any repetitions
    fn mnonrepeat(&mut self) { self.v = self.v.sansrepeat() }  

    /// Ascending union of two ordered sets, reassigned to self 
    fn munion(&mut self, s: &Self) { 
        // the result will be always ascending
        if !self.ascending { self.ascending = true; self.v = self.v.revs() };
        if s.ascending { self.v = self.v.merge(&s.v) } 

        else { self.v = self.v.merge(&s.v.revs()); }; 
    } 

    /// Ascending intersection of two sets, assigned to the self 
    fn mintersection(&mut self, s: &Self) {
        // the result will be always ascending
        if !self.ascending { self.ascending = true; self.v = self.v.revs() }; 
        if s.ascending { self.v = self.v.intersect(&s.v )}
        else { self.v = self.v.intersect(&s.v.revs()) };  
    }

    /// Ascending complement of s in self (i.e. self-s)
    fn mdifference(&mut self, s: &Self) {
        // the result will be always ascending
        if !self.ascending { self.ascending = true;  self.v = self.v.revs() };
        if s.ascending { self.v  = self.v.diff(&s.v) }
        else { self.v  = self.v.diff(&s.v.revs()) };
    }
}

/// These are generally better than OrderedSet(s) for bulky end types, as
/// there is not so much of moving them around.
impl<T> MutSetOps<T> for IndexedSet<T> where T: Copy+PartialOrd {

    /// just reverse the index
    fn mreverse(&mut self) {
        self.ascending = !self.ascending;
        self.i = self.i.revindex(); 
        }
    
    /// deletes repetitions.
    fn mnonrepeat(&mut self) { 
        self.v = self.v.sansrepeat();
        self.i = self.v.sortidx();       
    } 

    /// Union of two IndexedSets reassigned to self.  
    /// Will be always ascending ordered.  
    fn munion(&mut self, s: &Self) {         
        if self.ascending {         
            if s.ascending { (self.v,self.i) = self.v.merge_indexed(&self.i,&s.v, &s.i) }
            else { (self.v,self.i) = self.v.merge_indexed(&self.i, &s.v, &s.i.revindex() ) }     
        }
        else {
            self.ascending = true; 
            if s.ascending { (self.v,self.i) = self.v.merge_indexed( &self.i.revindex(),&s.v,&s.i) } 
            else { (self.v,self.i) = self.v.merge_indexed(&self.i.revindex(), &s.v, &s.i.revindex()) }  
        }
    }      
    
    /// Intersection of two IndexedSets
    fn mintersection(&mut self, s: &Self) {
        if self.ascending {
            if s.ascending { self.v = self.v.intersect_indexed(&self.i,&s.v, &s.i) }
            else { self.v = self.v.intersect_indexed(&self.i,&s.v, &s.i.revindex() ) }     
        }
        else {
            self.ascending = true; 
            if s.ascending { self.v = self.v.intersect_indexed(&self.i.revindex(),&s.v,&s.i) } 
            else { self.v = self.v.intersect_indexed(&self.i.revindex(), &s.v, &s.i.revindex()) }  
        }
        // result index will be of the new size but in all cases trivial and ascending
        self.i = trivindex(true,self.v.len()); 
    }
    
    /// Complement of s in self (i.e. self-s)
    fn mdifference(&mut self, s: &Self) {
        if self.ascending {
            if s.ascending { self.v = self.v.diff_indexed(&self.i,&s.v, &s.i) }
            else { self.v = self.v.diff_indexed(&self.i,&s.v, &s.i.revindex() ) }     
        }
        else {
            self.ascending = true; 
            if s.ascending { self.v = self.v.intersect_indexed(&self.i.revindex(),&s.v,&s.i) } 
            else { self.v = self.v.intersect_indexed(&self.i.revindex(), &s.v, &s.i.revindex()) }  
        }
        // result index will be of the new size but in all cases trivial and ascending
        self.i = trivindex(true,self.v.len()); 
    }
}

/// The primitive functions from `indxvec` all expect indexed sets, 
/// so for now we convert from ranks to sort indices using `.invindex()`.
/// Even though that is a simple operation, for lots of set operations, 
/// it will be slightly quicker to work in IndexedSet(s) 
/// and only to rank the final result.
impl<T> MutSetOps<T> for RankedSet<T> where T: Copy+PartialOrd {
    /// just make the ranks descending
    fn mreverse(&mut self) {
        self.ascending = !self.ascending;
        self.i = self.i.complindex() // `complindex` reverses the ranks
    }
        
    /// deletes repetitions.
    fn mnonrepeat(&mut self) { 
        let clean = self.v.sansrepeat();
        self.v = clean.to_vec();
        // rebuild the ranks (can do better)
        self.i = clean.rank(self.ascending)      
    } 
    
    /// Union of two RankedSets. 
    /// Converts ranks to sort indices with `invindex`, merges, then converts back to ranks
    /// Data `self.v` is simply concatenated
    fn munion(&mut self, s: &Self) {
        if !self.ascending { self.ascending = true; self.i = self.i.complindex() }
        if s.ascending { (self.v,self.i) = 
            self.v.merge_indexed( &self.i.invindex(), &s.v, &s.i.invindex() ); }
        else { (self.v,self.i) = 
            self.v.merge_indexed( &self.i.invindex(), &s.v, &s.i.invindex().revindex() ); };        
        self.i = self.i.invindex(); // invert back to ranks index
    }      
        
    /// Intersection of two RankedSets
    fn mintersection(&mut self, s: &Self) {
        if self.ascending {
            if s.ascending { self.v = self.v.intersect_indexed(&self.i.invindex(),&s.v, &s.i.invindex()) }
            else { self.v = self.v.intersect_indexed(  &self.i.invindex(),&s.v, &s.i.invindex().revindex() ) }     
        }
        else {
            self.ascending = true; 
            if s.ascending { self.v = 
                self.v.intersect_indexed( &self.i.invindex().revindex(),&s.v,&s.i.invindex()) } 
            else { self.v = self.v.intersect_indexed( &self.i.invindex().revindex(), &s.v, &s.i.invindex().revindex()) }  
        }
        // result ranks will be of the new size but in all cases trivial and ascending
        self.i = trivindex(true,self.v.len()); 
    }
        
    /// Complement of s in self (i.e. self-s)
    fn mdifference(&mut self, s: &Self) {
        if self.ascending {
            if s.ascending { self.v = self.v.diff_indexed(&self.i.invindex(),&s.v, &s.i.invindex()) }
            else { self.v = self.v.diff_indexed( &self.i.invindex(),&s.v, &s.i.invindex().revindex() ) }     
        }
        else {
            self.ascending = true; 
            if s.ascending { self.v = self.v.diff_indexed( &self.i.invindex().revindex(),&s.v,&s.i.invindex()) } 
            else { self.v = self.v.diff_indexed( &self.i.invindex().revindex(), &s.v, &s.i.invindex().revindex()) }  
        }
        // result ranks will be of the new size, trivial and ascending
        self.i = trivindex(true,self.v.len()); 
    }
}