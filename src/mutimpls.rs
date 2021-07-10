use crate::{trivindex,Set,OrderedSet,IndexedSet,RankedSet,MutSetOps};
use indxvec::{Indices,merge::*};

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
    fn mnonrepeat(&mut self) { self.v = sansrepeat(&self.v) }

    /// Union of two unordered sets, assigned to self  
    fn munion(&mut self, s: &Self) {
        let s1 = sortm(&self.v,true);
        let s2 = sortm(&s.v,true);
        self.v = unite(&s1,&s2) 
    }

    /// Intersection of two unordered sets, assigned to self
    fn mintersection(&mut self, s: &Self) {
        let s1 = sortm(&self,true);
        let s2 = sortm(&s,true);
        self.v = intersect(&s1,&s2) 
    }

    /// Complement of s in self (i.e. self -= s)
    fn mdifference(&mut self, s: &Self) {
        let s1 = sortm(&self,true);
        let s2 = sortm(&s,true); 
        self.v = diff(&s1,&s2)  
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
    fn mnonrepeat(&mut self) { self.v = sansrepeat(&self.v) }  

    /// Ascending union of two ordered sets, reassigned to self 
    fn munion(&mut self, s: &Self) { 
        if self.ascending {
            if s.ascending {  self.v = unite(&self.v,&s.v) }
            else { self.v = unite(&self.v, &revs(&s.v)) }
        }
        else {
            self.ascending = true; // the result is always ascending
            if s.ascending {  self.v = unite(&revs(&self.v),&s.v) }
            else { self.v = unite(&revs(&self.v), &revs(&s.v)) } 
        }
    }     

    /// Ascending intersection of two sets, assigned to the self 
    fn mintersection(&mut self, s: &Self) {
        if self.ascending {
            if s.ascending {  self.v = intersect(&self.v,&s.v) }
            else { self.v = intersect(&self.v, &revs(&s.v)) }
        }
        else {
            self.ascending = true; // the result is always ascending
            if s.ascending {  self.v = intersect(&revs(&self.v),&s.v) } 
            else { self.v = intersect(&revs(&self.v), &revs(&s.v)) }  
        }
    }

    /// Ascending complement of s in self (i.e. self-s)
    fn mdifference(&mut self, s: &Self) {
        if self.ascending {
            if s.ascending {  self.v = diff(&self.v,&s.v) } 
            else { self.v = diff(&self.v, &revs(&s.v)) } 
        }
        else {
            self.ascending = true; 
            if s.ascending {  self.v = diff(&revs(&self.v),&s.v) } 
            else { self.v = diff(&revs(&self.v), &revs(&s.v)) }  
        }
    }
}

/// These are generally better than OrderedSet(s) for bulky end types, as
/// there is not so much of moving them around.
impl<T> MutSetOps<T> for IndexedSet<T> where T: Copy+PartialOrd {

    /// just reverse the index
    fn mreverse(&mut self) {
        self.ascending = !self.ascending;
        self.i = self.i.revindex() 
        }
    
    /// deletes repetitions.
    fn mnonrepeat(&mut self) { 
        let clean = &sansrepeat(&self.v);
        self.v = clean.to_vec();
        self.i = sortidx(&clean)       
    } 

    /// Union of two IndexedSets reassigned to self.  
    /// self will be ascending ordered  
    fn munion(&mut self, s: &Self) {         
        if self.ascending {         
            if s.ascending { self.v = unite_indexed(&self.v,&self.i,&s.v, &s.i) }
            else { self.v = unite_indexed(&self.v,  &self.i,&s.v, &s.i.revindex() ) }     
        }
        else {
            self.ascending = true; 
            if s.ascending { self.v = unite_indexed(&self.v, &self.i.revindex(),&s.v,&s.i) } 
            else { self.v = unite_indexed(&self.v, &self.i.revindex(), &s.v, &s.i.revindex()) }  
        }
        // result index will be of the new size but in all cases trivial and ascending
        self.i = trivindex(true,self.v.len()); 
    }      
    
    /// Intersection of two IndexedSets
    fn mintersection(&mut self, s: &Self) {
        if self.ascending {
            if s.ascending { self.v = intersect_indexed(&self.v,&self.i,&s.v, &s.i) }
            else { self.v = intersect_indexed(&self.v,  &self.i,&s.v, &s.i.revindex() ) }     
        }
        else {
            self.ascending = true; 
            if s.ascending { self.v = intersect_indexed(&self.v, &self.i.revindex(),&s.v,&s.i) } 
            else { self.v = intersect_indexed(&self.v, &self.i.revindex(), &s.v, &s.i.revindex()) }  
        }
        // result index will be of the new size but in all cases trivial and ascending
        self.i = trivindex(true,self.v.len()); 
    }
    
    /// Complement of s in self (i.e. self-s)
    fn mdifference(&mut self, s: &Self) {
        if self.ascending {
            if s.ascending { self.v = diff_indexed(&self.v,&self.i,&s.v, &s.i) }
            else { self.v = diff_indexed(&self.v,  &self.i,&s.v, &s.i.revindex() ) }     
        }
        else {
            self.ascending = true; 
            if s.ascending { self.v = intersect_indexed(&self.v, &self.i.revindex(),&s.v,&s.i) } 
            else { self.v = intersect_indexed(&self.v, &self.i.revindex(), &s.v, &s.i.revindex()) }  
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
        self.i = self.i.complindex() 
    }
        
    /// deletes repetitions.
    fn mnonrepeat(&mut self) { 
        let clean = &sansrepeat(&self.v);
        self.v = clean.to_vec();
        self.i = rank(&clean, self.ascending )       
    } 
    
    /// Union of two IndexedSets.  
    fn munion(&mut self, s: &Self) {         
        if self.ascending {
            if s.ascending { self.v = unite_indexed(&self.v,&self.i.invindex(),&s.v, &s.i.invindex()) }
            else { self.v = unite_indexed(&self.v,  &self.i.invindex(),&s.v, &s.i.invindex().revindex() ) }     
        }
        else {
            self.ascending = true; 
            if s.ascending { self.v = unite_indexed(&self.v, &self.i.invindex().revindex(),&s.v,&s.i.invindex()) } 
            else { self.v = unite_indexed(&self.v, &self.i.invindex().revindex(), &s.v, &s.i.invindex().revindex()) }  
        }
        // result ranks will be of the new size but in all cases trivial and ascending
        self.i = trivindex(true,self.v.len()); 
    }      
        
    /// Intersection of two IndexedSets
    fn mintersection(&mut self, s: &Self) {
        if self.ascending {
            if s.ascending { self.v = intersect_indexed(&self.v,&self.i.invindex(),&s.v, &s.i.invindex()) }
            else { self.v = intersect_indexed(&self.v,  &self.i.invindex(),&s.v, &s.i.invindex().revindex() ) }     
        }
        else {
            self.ascending = true; 
            if s.ascending { self.v = intersect_indexed(&self.v, &self.i.invindex().revindex(),&s.v,&s.i.invindex()) } 
            else { self.v = intersect_indexed(&self.v, &self.i.invindex().revindex(), &s.v, &s.i.invindex().revindex()) }  
        }
        // result ranks will be of the new size but in all cases trivial and ascending
        self.i = trivindex(true,self.v.len()); 
    }
        
    /// Complement of s in self (i.e. self-s)
    fn mdifference(&mut self, s: &Self) {
        if self.ascending {
            if s.ascending { self.v = diff_indexed(&self.v,&self.i.invindex(),&s.v, &s.i.invindex()) }
            else { self.v = diff_indexed(&self.v,  &self.i.invindex(),&s.v, &s.i.invindex().revindex() ) }     
        }
        else {
            self.ascending = true; 
            if s.ascending { self.v = intersect_indexed(&self.v, &self.i.invindex().revindex(),&s.v,&s.i.invindex()) } 
            else { self.v = intersect_indexed(&self.v, &self.i.invindex().revindex(), &s.v, &s.i.invindex().revindex()) }  
        }
        // result ranks will be of the new size but in all cases trivial and ascending
        self.i = trivindex(true,self.v.len()); 
    }
}