use crate::{SetOps,Set,OrderedSet,IndexedSet,RankedSet};
use indxvec::{merge::*};

impl<T> SetOps<T> for Set<T> where T: Copy {

    /// Deletes any repetitions
    fn nonrepeat(&self) -> Self where T: PartialOrd+Copy { 
        Self { v: sansrepeat(&self.v) }
    }

    /// Finds minimum, minimum's first index, maximum, maximum's first index of &[T] 
    fn infsup(&self) -> (T, usize, T, usize) where T: PartialOrd {
        let (min,mini,max,maxi) = minmax(&self.v);
        (min,mini,max,maxi)  
    }

    /// True if m is a member of the set
    fn member(&self, m: T) -> bool where T: PartialOrd { 
        let opt = member(&self.v,m);
        opt.is_some()     
    }

    /// Search a Set for m. Returns index of the first m.
    fn search(&self, m: T)  -> Option<usize> where T: PartialOrd {
        member(&self.v,m)
    }

    /// Union of two unordered sets of the same type - 
    fn union(&self, s: &Self) -> OrderedSet<T> where T: PartialOrd {
        let s1 = sortm(&self,true);
        let s2 = sortm(&s,true);
        OrderedSet { ascending:true,v:unite(&s1,&s2) }
    }

    /// Intersection of two sets of the same type
    fn intersection(&self, s: &Self) -> OrderedSet<T> where T: PartialOrd {
        let s1 = sortm(&self,true);
        let s2 = sortm(&s,true);
        OrderedSet { ascending:true,v:intersect(&s1,&s2) }
    }

    /// Complement of s in self (i.e. self-s)
    fn difference(&self, s: &Self) -> OrderedSet<T> where T: PartialOrd {
        let s1 = sortm(&self,true);
        let s2 = sortm(&s,true); 
        OrderedSet { ascending:true,v:diff(&s1,&s2) } 
    }
    
}