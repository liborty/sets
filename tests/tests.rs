#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]

// use anyhow::{Result};
use sets::{Set,OrderedSet,IndexedSet,RankedSet,SetOps};
use indxvec::{Indices,merge::*};

#[test]
fn conversions() -> () { 
   let v = vec![1.,14.,2.,13.,3.,12.,4.,11.,5.,10.,10.,6.,9.,7.,8.,16.];
   let setv = Set::from_slice(&v);  
   println!("{}",setv); // Display of Set  
   println!("Slice-> {}",OrderedSet::from_slice(&v,true)); // sorted data but index lost
   println!("Set-> {}",OrderedSet::from_set(&setv,false)); // descending sorted data, index lost  
   let ix = IndexedSet::from_set(&setv,false);    
   println!("Set-> {}",&ix); 
   println!("Indexed-> {}",OrderedSet::from_indexed(&ix,true));
   let rx = RankedSet::from_slice(&v,false);
   println!("Slice->{}",&rx);
   println!("Ranked->{}",IndexedSet::from_ranked(&rx,true)); 
   println!("Ranked->{}",OrderedSet::from_ranked(&rx,true)); 
   println!("Indexed->{}",RankedSet::from_indexed(&ix,true));
   ()
}

#[test]
fn setops() -> () { 
   let v = vec![1.,14.,2.,13.,3.,12.,4.,11.,5.,10.,10.,6.,9.,7.,8.,16.];   
   let setv = Set::from_slice(&v);  
   println!("{}",setv); // Display of Set 
   println!("Nonrepeat-> {}",setv.nonrepeat()); // Display of Set    
   println!("Is {} member? {}",0.0, setv.member(0.0));  
   println!("Infsup: {:?}",setv.infsup());
   let setw = Set::from_slice(&[20.,19.,18.,17.,16.,15.]);
   println!("{}",setw);
   let us = setw.union(&setv);
   println!("Union-> {}",&us);
   println!("Intersection-> {}",setw.intersection(&setv));
   println!("Difference-> {}",setw.difference(&setv));
   // let oset = OrderedSet::from_slice(&v,true);
   // let ixset = IndexedSet::from_set(&setv,false);  
   // let rxset = RankedSet::from_slice(&v,false);

   ()
}