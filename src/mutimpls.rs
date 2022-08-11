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
    
    /// Deletes an item from self
    /// Returns false if item not found 
    fn mdelete(&mut self, item:T) -> bool where Self:Sized {
        match self.stype {
            SType::Empty => false, // empty set
            SType::Unordered => {
                if let Some(i) = self.data.member(item,true) {
                    // don't care about order, swap_remove swaps in the last item, fast
                    self.data.swap_remove(i); true }
                else { false }
            }, 
            SType::Ordered => {
                let r = self.data.binsearch(&item);
                if r.start == r.end { false } else { 
                    self.data.remove(r.start); true } // remove + shift, preserves ordering
            },
            SType::Indexed => {
                self.data = self.index.unindex(&self.data,self.ascending); // make ordered data
                let r = self.data.binsearch(&item);
                if r.start == r.end { false } else { // success 
                    self.data.remove(r.start); // remove + shift, preserves ordering     
                    self.index = trivindex(self.ascending,self.data.len());
                    true }
            },
            SType::Ranked => {
                let sortindex = self.index.invindex();
                self.data = sortindex.unindex(&self.data,self.ascending); // make ordered data
                let r = self.data.binsearch(&item);
                if r.start == r.end { false } else { // success 
                    self.data.remove(r.start); // remove + shift, preserves ordering     
                    self.index = trivindex(self.ascending,self.data.len());
                    true }
            },
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
                let range = self.data.binsearch(&item);
                self.data.insert(range.start,item); // shifts the rest  
            },
            SType::Indexed => {
                let irange = self.data.binsearch_indexed(&self.index,&item); 
                // simply push the item to the end of unordered data self.data
                self.data.push(item);
                // and insert its subscipt into the right place in the sort index    
                self.index.insert(irange.start,self.data.len()-1);                

            }
            SType::Ranked => {
               // invert the rank index to get the sort index position
                let irange = self.data.binsearch_indexed(&self.index.invindex(),&item);
                // simply push the new item to the end of unordered data self.data
                self.data.push(item);
                // and insert its index position (rank) into the same place in the rank index    
                self.index.push(irange.start);
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
