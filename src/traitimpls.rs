use crate::{SetOps,Set,OrderedSet,IndexedSet,RankedSet,MutSetOps};
use indxvec::{MinMax,Indices,Vecops};

impl<T> SetOps<T> for Set<T> where T:Copy+PartialOrd {

    /// Reverses a set
    fn reverse(&self) -> Self {
        Self { v: self.v.revs() }
    } 


    /// Finds minimum, minimum's first index, maximum, maximum's first index of &[T] 
    fn infsup(&self) -> MinMax<T> {
        self.v.minmax() 
    }

    /// True if m is a member of the set
    fn member(&self, m: T) -> bool  { 
        let opt = self.v.member(m); // from indxvec
        opt.is_some()     
    }

    /// Search a Set self for m.
    /// Returns the subscript of the first m or None
    fn search(&self, m: T)  -> Option<usize> {
        self.v.member(m) 
    }

    /// Mostly for non-members. Index of the next item in order, or self.len(). 
    /// For unordered sets returns self.len().
    fn position(&self, _:T)  -> usize {
        self.len()
    }       

    /// Union of two unordered sets of the same end type T 
    fn union(&self, s: &Self) -> Self  { 
        Set { v:self.v.unite_unsorted(&s.v) }
    }

    /// Intersection of two sets of the same type
    fn intersection(&self, s: &Self) -> OrderedSet<T>  {
        let s1 = self.sortm(true);
        let s2 = s.sortm(true);
        OrderedSet { ascending:true,v:s1.intersect(&s2) }
    }

    /// Complement of s in self (i.e. self-s)
    fn difference(&self, s: &Self) -> OrderedSet<T>  {
        let s1 = self.sortm(true);
        let s2 = s.sortm(true); 
        OrderedSet { ascending:true,v:s1.diff(&s2) } 
    }
    
}

impl<T> SetOps<T> for OrderedSet<T> where T: Copy+PartialOrd {

    fn reverse(&self) -> Self where T: Copy {
        Self { ascending: !self.ascending, v: self.v.revs() }
    }

    /// Deletes any repetitions
    fn nonrepeat(&self) -> Self { 
        let mut scopy =  self.clone();
        scopy.mnonrepeat();
        scopy
    }

    /// Finds minimum, minimum's first index, maximum, maximum's first index
    /// Much faster for OrderedSet than for Set.
    fn infsup(&self) -> MinMax<T> {
        let last = self.v.len()-1;
        if self.ascending { MinMax{min:self.v[0],minindex:0,max:self.v[last],maxindex:last} }
        else { MinMax{min:self.v[last],minindex:last,max:self.v[0],maxindex:0} }
    }

    /// True if m is a member of the set
    fn member(&self, m: T) -> bool { 
        let opt = if self.ascending { self.v.memsearch(m) }
            else { self.v.memsearchdesc(m) };
        opt.is_some()     
    }

    /// Search a Set for m. Returns index of the first m.
    fn search(&self, m: T)  -> Option<usize> where T: PartialOrd {
        if self.ascending { self.v.memsearch(m) }
        else { self.v.memsearchdesc(m) }        
    }

    /// Mostly for non-members. Index of the next item in order, or self.len(). 
    /// For unordered sets returns self.len(), too.
    fn position(&self, m: T)  -> usize {
        if self.ascending { self.binsearch(m) }
        else { self.v.binsearchdesc(m) }
    }

    /// Union of two ordered sets  
    fn union(&self, s: &Self) -> OrderedSet<T> { 
        let tmp1; let tmp2;  
        OrderedSet { ascending:true, v: 
            if self.ascending { &self.v } else { tmp1 = self.v.revs(); &tmp1 }       
        .merge( if s.ascending { &s.v } else { tmp2 = s.v.revs(); &tmp2 }) } 
    } 

    /// Intersection of two sets of the same type
    fn intersection(&self, s: &Self) -> OrderedSet<T> {
        let tmp1; let tmp2; 
        OrderedSet { ascending:true, v: 
            if self.ascending { &self.v } else { tmp1 = self.v.revs(); &tmp1 }
            .intersect( if s.ascending { &s.v } else { tmp2 = s.v.revs(); &tmp2 }) } 
    }

    /// Complement of s in self (i.e. self-s)
    fn difference(&self, s: &Self) -> OrderedSet<T> {
        let tmp1; let tmp2; 
        OrderedSet { ascending:true, v: 
            if self.ascending { &self.v } else { tmp1 = self.v.revs(); &tmp1 }
            .diff( if s.ascending { &s.v } else { tmp2 = s.v.revs(); &tmp2 }) } 
    }
}

/// These are generally better than OrderedSet(s) for bulky end types, as
/// there is not so much of moving them around.
impl<T> SetOps<T> for IndexedSet<T> where T: Copy+PartialOrd {

    /// just reverse the index
    fn reverse(&self) -> Self where T: Copy {
        Self { ascending: !self.ascending, v: self.v.to_vec(), i: self.i.revindex() }
    } 
    
    /// Deletes any repetitions
    fn nonrepeat(&self) -> Self { 
        let mut scopy =  self.clone();
        scopy.mnonrepeat();
        scopy
    }
 
    /// Finds minimum, minimum's first index, maximum, maximum's first index
    fn infsup(&self) -> MinMax<T> {
        let last = self.v.len()-1;
        let firstval = self.v[self.i[0]];
        let lastval = self.v[self.i[last]];
        if self.ascending { MinMax{min:firstval,minindex:self.i[0],max:lastval,maxindex:self.i[last]} }
        else { MinMax{min:lastval,minindex:self.i[last],max:firstval,maxindex:self.i[0]} }
    }
    
    /// True if m is a member of the set
    fn member(&self, m: T) -> bool where T: PartialOrd { 
        let opt = if self.ascending { self.v.memsearch_indexed(&self.i,m) }
            else { self.v.memsearchdesc_indexed(&self.i,m) };
        opt.is_some()     
    }
    
    /// Search a Set for m. Returns index of the first m.
    fn search(&self, m: T)  -> Option<usize>  {
        if self.ascending { self.v.memsearch_indexed(&self.i,m) }
        else { self.v.memsearchdesc_indexed(&self.i,m) }        
    }

    /// Mostly for non-members. Index of the next item in order, or self.len(). 
    /// For unordered sets returns self.len(), too.
    fn position(&self, m: T)  -> usize {
        if self.ascending { self.v.binsearch_indexed(&self.i,m) }
        else { self.v.binsearchdesc_indexed(&self.i,m) }        
    }

    /// Union of two IndexedSets. Returns new IndexedSet 
    fn union(&self, s: &Self) -> IndexedSet<T> {
        let tmp1; let tmp2;
        let (data,index) = self.v.merge_indexed(
                if self.ascending { &self.i } else { tmp1 = self.i.revindex(); &tmp1 },
                &s.v, // data remains in previous order
                if s.ascending { &s.i } else { tmp2 = s.i.revindex(); &tmp2 });
            IndexedSet { ascending: true, v: data, i: index }
    }   
    
    /// Intersection of two sets of the same type. 
    /// Via OrderedSet for convenience, for now.
    /// Probably should use intersect_indexed as in `union` above.
    fn intersection(&self, s: &Self) -> OrderedSet<T>  {
        let s1 = OrderedSet::from_indexed(self,true);
        let s2 = OrderedSet::from_indexed(s,true);
        s1.intersection(&s2)
    }
    
    /// Complement of s in self (i.e. self-s)
    fn difference(&self, s: &Self) -> OrderedSet<T> {
        let s1 = OrderedSet::from_indexed(self,true);
        let s2 = OrderedSet::from_indexed(s,true); 
        s1.difference(&s2) 
    }
}

/// For lots of set operations, it is probably better to work in IndexedSet(s) 
/// and then only to rank the final result.
impl<T> SetOps<T> for RankedSet<T> where T: Copy+PartialOrd {

    /// switches between ascending and descending ranks, 
    /// which is what is logically expected here 
    /// but it is not the same as a literal reversal of the ranks index!
    fn reverse(&self) -> Self {
        Self { ascending: !self.ascending, v: self.v.to_vec(), i: self.i.complindex() }
    }     

    /// Deletes any repetitions
    fn nonrepeat(&self) -> Self { 
        let mut scopy =  self.clone();
        scopy.mnonrepeat();
        scopy
    }
         
    /// Finds minimum, minimum's first index, maximum, maximum's first index
    fn infsup(&self) -> MinMax<T> {
        let last = self.v.len()-1;
        let si = self.i.invindex(); // ranks -> sort index
        let firstval = self.v[si[0]];
        let lastval = self.v[si[last]];
            if self.ascending { MinMax{min:firstval,minindex:si[0],max:lastval,maxindex:si[last]} }
            else { MinMax{min:lastval,minindex:si[last],max:firstval,maxindex:si[0]} }
        }
        
    /// True if m is a member of the set
    fn member(&self, m: T) -> bool { 
        let opt = if self.ascending { self.v.memsearch_indexed(&self.i.invindex(),m) }
                else { self.v.memsearchdesc_indexed(&self.i.invindex(),m) };
            opt.is_some()     
        }
        
    /// Search a Set for m. Returns index of the first m.
    fn search(&self, m: T)  -> Option<usize> {
        if self.ascending { self.v.memsearch_indexed(&self.i.invindex(),m) }
        else { self.v.memsearchdesc_indexed(&self.i.invindex(),m) }        
    }

    /// Mostly for non-members. Index of the next item in order, or self.len(). 
    /// For unordered sets returns self.len(), too.
    fn position(&self, m: T)  -> usize {
        if self.ascending { self.v.binsearch_indexed(&self.i.invindex(),m) }
        else { self.v.binsearchdesc_indexed(&self.i.invindex(),m) }        
    }

    /// Union of two RankedSets. Returns new RankedSet 
    fn union(&self, s: &Self) -> RankedSet<T> {
        let tmp1; let tmp2; 
        let (sv,si) = self.v.merge_indexed(
            if self.ascending { tmp1 = self.i.invindex(); &tmp1 } 
            else { tmp1 = self.i.invindex().revindex(); &tmp1 },
            &s.v, // data remains in previous order
            if s.ascending { tmp2 = s.i.invindex(); &tmp2 }
            else { tmp2 = s.i.invindex().revindex(); &tmp2 });
        RankedSet { ascending: true, v: sv, i: si.invindex() } // invert back to ranks index
    }
        
    /// Intersection of two RankedSets. 
    /// Via OrderedSet for convenience, for now.
    /// Todo: Probably should use intersect_indexed as in `union` above.
    fn intersection(&self, s: &Self) -> OrderedSet<T> {
        let s1 = OrderedSet::from_ranked(self,true);
        let s2 = OrderedSet::from_ranked(s,true);
        s1.intersection(&s2)
    }
        
    /// Complement of s in self (i.e. self-s)
    fn difference(&self, s: &Self) -> OrderedSet<T> {
        let s1 = OrderedSet::from_ranked(self,true);
        let s2 = OrderedSet::from_ranked(s,true); 
        s1.difference(&s2) 
    }  

}