use crate::{SetOps,Set,OrderedSet,IndexedSet,RankedSet};
use indxvec::{MinMax,Indices,merge::*};

impl<T> SetOps<T> for Set<T> where T:Copy+PartialOrd {

    fn reverse(&self) -> Self {
        Self { v: revs(&self.v) }
    }

    /// Deletes any repetitions
    fn nonrepeat(&self) -> Self { 
        Self { v: sansrepeat(&self.v) }
    }

    /// Finds minimum, minimum's first index, maximum, maximum's first index of &[T] 
    fn infsup(&self) -> MinMax<T> {
        minmax(&self.v) 
    }

    /// True if m is a member of the set
    fn member(&self, m: T) -> bool  { 
        let opt = member(&self.v,m);
        opt.is_some()     
    }

    /// Search a Set for m. Returns index of the first m.
    fn search(&self, m: T)  -> Option<usize>  {
        member(&self.v,m)
    }

    /// Union of two unordered sets of the same type - 
    fn union(&self, s: &Self) -> OrderedSet<T>  {
        let s1 = sortm(self,true);
        let s2 = sortm(s,true);
        OrderedSet { ascending:true,v:unite(&s1,&s2) }
    }

    /// Intersection of two sets of the same type
    fn intersection(&self, s: &Self) -> OrderedSet<T>  {
        let s1 = sortm(self,true);
        let s2 = sortm(s,true);
        OrderedSet { ascending:true,v:intersect(&s1,&s2) }
    }

    /// Complement of s in self (i.e. self-s)
    fn difference(&self, s: &Self) -> OrderedSet<T>  {
        let s1 = sortm(self,true);
        let s2 = sortm(s,true); 
        OrderedSet { ascending:true,v:diff(&s1,&s2) } 
    }
    
}

impl<T> SetOps<T> for OrderedSet<T> where T: Copy+PartialOrd {

    fn reverse(&self) -> Self where T: Copy {
        Self { ascending: !self.ascending, v: revs(&self.v) }
    }

    /// Deletes any repetitions, in either order
    fn nonrepeat(&self) -> Self { 
        Self { ascending: self.ascending, v: sansrepeat(&self.v) }
    }

    /// Finds minimum, minimum's first index, maximum, maximum's first index
    /// Much simpler for OrderedSet than for Set.
    fn infsup(&self) -> MinMax<T> {
        let last = self.v.len()-1;
        if self.ascending { MinMax{min:self.v[0],minindex:0,max:self.v[last],maxindex:last} }
        else { MinMax{min:self.v[last],minindex:last,max:self.v[0],maxindex:0} }
    }

    /// True if m is a member of the set
    fn member(&self, m: T) -> bool { 
        let opt = if self.ascending { memsearch(&self.v,m) }
            else { memsearchdesc(&self.v,m) };
        opt.is_some()     
    }

    /// Search a Set for m. Returns index of the first m.
    fn search(&self, m: T)  -> Option<usize> where T: PartialOrd {
        if self.ascending { memsearch(&self.v,m) }
        else { memsearchdesc(&self.v,m) }        
    }

    /// Union of two ordered sets  
    fn union(&self, s: &Self) -> OrderedSet<T> { 
        let rself; let rs; 
        OrderedSet { ascending:true, v:unite(
            if self.ascending {&self.v} else {rself = revs(&self.v); &rself},
            if s.ascending {&s.v} else {rs = revs(&s.v); &rs}) } 
    } 

    /// Intersection of two sets of the same type
    fn intersection(&self, s: &Self) -> OrderedSet<T> {
        let rself; let rs; 
        OrderedSet { ascending:true, v:intersect(
            if self.ascending {&self.v} else {rself = revs(&self.v); &rself},
            if s.ascending {&s.v} else {rs = revs(&s.v); &rs}) } 
    }

    /// Complement of s in self (i.e. self-s)
    fn difference(&self, s: &Self) -> OrderedSet<T> {
        let rself; let rs; 
        OrderedSet { ascending:true, v:diff(
            if self.ascending {&self.v} else {rself = revs(&self.v); &rself},
            if s.ascending {&s.v} else {rs = revs(&s.v); &rs}) } 
    }
}

/// These are generally better than OrderedSet(s) for bulky end types, as
/// there is not so much of moving them around.
impl<T> SetOps<T> for IndexedSet<T> where T: Copy+PartialOrd {

    /// just reverse the index
    fn reverse(&self) -> Self where T: Copy {
        Self { ascending: !self.ascending, v: self.v.to_vec(), i: self.i.revindex() }
        }
    
    /// Deletes repetitions.
    fn nonrepeat(&self) -> Self { 
        let clean = &sansrepeat(&self.v);
        Self { ascending: self.ascending, v: clean.to_vec(), i:sortidx(clean)  }      
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
        let opt = if self.ascending { memsearch_indexed(&self.v,&self.i,m) }
            else { memsearchdesc_indexed(&self.v,&self.i,m) };
        opt.is_some()     
    }
    
    /// Search a Set for m. Returns index of the first m.
    fn search(&self, m: T)  -> Option<usize>  {
        if self.ascending { memsearch_indexed(&self.v,&self.i,m) }
        else { memsearchdesc_indexed(&self.v,&self.i,m) }        
    }
    
    /// Union of two IndexedSets. Returns an OrderedSet. 
    fn union(&self, s: &Self) -> OrderedSet<T>  { 
        if self.ascending {
            if s.ascending { OrderedSet{ ascending:self.ascending, v: unite_indexed(&self.v,&self.i,&s.v, &s.i)} }
            else { OrderedSet { ascending:true, v: unite_indexed(&self.v,  &self.i,&s.v, &s.i.revindex() ) }     }
        }
        else if s.ascending { OrderedSet { ascending:true, v:unite_indexed(&self.v, &self.i.revindex(),&s.v,&s.i) } }
            else { OrderedSet { ascending:true, v:unite_indexed(&self.v, &self.i.revindex(), &s.v, &s.i.revindex()) } } 
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
    /// but it is not the same as a literal reversal of the ranks!
    fn reverse(&self) -> Self {
        Self { ascending: !self.ascending, v: self.v.to_vec(), i: self.i.complindex() }
    }
        
    /// Deletes repetitions.
    fn nonrepeat(&self) -> Self { 
        let clean = &sansrepeat(&self.v);
        Self { ascending: self.ascending, v: clean.to_vec(), i:rank(clean, self.ascending )  }      
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
        let opt = if self.ascending { memsearch_indexed(&self.v,&self.i.invindex(),m) }
                else { memsearchdesc_indexed(&self.v,&self.i.invindex(),m) };
            opt.is_some()     
        }
        
    /// Search a Set for m. Returns index of the first m.
    fn search(&self, m: T)  -> Option<usize> {
        if self.ascending { memsearch_indexed(&self.v,&self.i.invindex(),m) }
        else { memsearchdesc_indexed(&self.v,&self.i.invindex(),m) }        
    }
        
    /// Union of two RankedSets. Returns an OrderedSet 
    fn union(&self, s: &Self) -> OrderedSet<T> { 
        if self.ascending {
            if s.ascending { OrderedSet{ ascending:self.ascending, v: unite_indexed(&self.v,&self.i.invindex(),&s.v, &s.i.invindex())} }
            else { OrderedSet { ascending:true, v: unite_indexed(&self.v,  &self.i.invindex(),&s.v, &s.i.invindex().revindex() ) }     }
        }
        else if s.ascending { OrderedSet { ascending:true, v:unite_indexed(&self.v, &self.i.invindex().revindex(),&s.v,&s.i.invindex()) } }
            else { OrderedSet { ascending:true, v:unite_indexed(&self.v, &self.i.invindex().revindex(), &s.v, &s.i.invindex().revindex()) } } 
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