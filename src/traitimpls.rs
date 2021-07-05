use crate::{SetOps,Set,OrderedSet,IndexedSet,RankedSet};
use indxvec::{merge::*};

impl<T> SetOps<T> for Set<T> where T: Copy, Vec<T> : IntoIterator {

    /// Finds minimum, minimum's first index, maximum, maximum's first index of &[T] 
    fn infsup(self) -> (T, usize, T, usize) where T: PartialOrd {
        minmax(&self.v)  
    } 
    /// Search a Set for m. Returns index of the first m.
    fn search(self, m: T)  -> Option<usize> where T: PartialOrd {
        let s = &self.v;
        for (i,&item) in s.iter().enumerate() {
            if item == m { return Some(i) } 
        }
        None     
    }     
    /// True if m is a member of the set
    fn member(&self, m: T) -> bool where T: PartialOrd {
        let i = binsearch(&sortm(&self.v, true), m);
        if i == 0  { return false }; 
        if self.v[i-1] < m { return false };
        true  
    }
    /// Union of two sets of the same type
    fn union(self, s: Self) -> Self where T: PartialOrd {
        Self { v:[self.v,s.v].concat() }
    }
    /// Intersection of two sets of the same type
    fn intersection(self, s: Self) -> Self where T: PartialOrd {
        let s1 = sortm(&self,true);
        let s2 = sortm(&s,true);
        self // todo: Self { v: common(&s1,&s2) }
    }
    /// Complement of s in self (i.e. self-s)
    fn complement(self, s: Self) -> Self where T: PartialOrd {
        let s1 = sortm(&self,true);
        let s2 = sortm(&s,true); 
        self // todo: Self { v: remove(&s1,&s2) }
    }
    
}