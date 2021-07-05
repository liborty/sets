#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]

// use anyhow::{Result};
use sets::{Set,OrderedSet,IndexedSet,RankedSet,functions::*};
use indxvec::{Indices,merge::*};

#[test]
fn indxvec() -> () { 
   let v = vec![1.,14.,2.,13.,3.,12.,4.,11.,5.,10.,6.,9.,7.,8.,16.];
   let sv = Set::from_slice(&v);  
   println!("{}",sv); // Display of Set  
   println!("{}",OrderedSet::from_slice(&v,true)); // sorted data but index lost
   println!("{}",OrderedSet::from_set(&sv,false)); // descending sorted data, index lost  
   let ix = IndexedSet::from_set(&sv,false);    
   println!("{}",&ix); 
   println!("{}",OrderedSet::from_indexed(&ix,true));
   let rx = RankedSet::from_slice(&v,false);
   println!("{}",&rx);
   println!("{}",IndexedSet::from_ranked(&rx,true)); 
   println!("{}",OrderedSet::from_ranked(&rx,true)); 
   println!("{}",RankedSet::from_indexed(&ix,true));
   ()
}
