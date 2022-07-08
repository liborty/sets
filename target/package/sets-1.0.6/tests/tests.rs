#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]

// use anyhow::{Result};
use sets::{Set,OrderedSet,IndexedSet,RankedSet,SetOps,MutSetOps};
use indxvec::{Printing,Indices,Vecops};

#[test]
fn conversions() { 
   let v = vec![1.,14.,2.,13.,3.,12.,4.,11.,5.,10.,10.,6.,9.,7.,8.,16.];
   let setv = Set::from_slice(&v);  
   println!("{}",setv); // Display of Set  
   println!("Slice-> {}",OrderedSet::from_slice(&v,true)); // sorted data but index lost
   println!("Set-> {}",OrderedSet::from_set(&setv,false)); // descending sorted data, index lost  
   let ix = IndexedSet::from_set(&setv,false);    
   println!("Set-> {}",&ix);
   let ss = OrderedSet::from_indexed(&ix,false); 
   println!("Indexed-> {}",ss); 
   println!("Ordered-> {}",IndexedSet::from_ordered(&ss,false)); 
   println!("Ordered-> {}",RankedSet::from_ordered(&ss,true)); 
   let rx = RankedSet::from_slice(&v,false);
   println!("Slice->{}",&rx);
   println!("Ranked->{}",IndexedSet::from_ranked(&rx,true)); 
   println!("Ranked->{}",OrderedSet::from_ranked(&rx,true)); 
   println!("Indexed->{}",RankedSet::from_indexed(&ix,true));
}

#[test]
fn settest() { 
   let v = vec![1.,14.,2.,13.,3.,12.,4.,11.,5.,10.,10.,6.,9.,7.,8.,16.];
   println!("Data: {}\n",v.bl()); // Display of Set   
   let sv = Set::from_slice(&v); 
   println!("Where is {}? at {}\n",12.bl(),sv.search(12.0).map_or_else(||"None".rd(),|x|x.gr()));  
   let setv = RankedSet::from_slice(&v,false);  
   println!("{}",setv); // Display of Set
   println!("Reverse-> {}",setv.reverse()); 
   println!("Nonrepeat-> {}",setv.nonrepeat()); // Display of Set    
   println!("Is {} a member? {}\n",0.0.bl(),setv.member(0.0).gr());
   println!("Where is {} (from descending ranked set)? at {}\n",12.bl(),setv.search(12.0).map_or_else(||"None".rd(),|x|x.gr())); 
   println!("Infsup: {}\n",setv.infsup());
   let setw = RankedSet::from_slice(&[20.,19.,18.,17.,16.,15.],true);
   println!("{}",setw);
   let us = setw.union(&setv);
   println!("Union-> {}",&us);
   println!("Intersection-> {}",setw.intersection(&setv));
   println!("Difference-> {}",setv.difference(&setw));
}

#[test]
fn mutabletest() { 
    let v = vec![1.,14.,2.,13.,3.,12.,4.,11.,5.,10.,10.,6.,9.,7.,8.,16.];
    let mut setu = Set::from_slice(&v);
    println!("{}",setu);
    setu.minsert(10.5);
    println!("Inserted 10.5 to {}",setu); // Display of Set 
    let mut setr = RankedSet::from_set(&setu,false);  
    println!("{}",setr); // Display of RankedSet
    let setr2 = RankedSet::from_slice(&[20.,19.,18.,17.,16.,15.],false);
    println!("New {}",setr2); // Display of ascending ranked set
    setr.munion(&setr2); 
    setr.mreverse();
    println!("Union->  {}",&setr);
    println!("{}",&OrderedSet::from_ranked(&setr,false));        
    setr.mdelete(10.5);
    println!("Deleted 10.5 from {}",&setr); 
    println!("{}",&OrderedSet::from_ranked(&setr,true));
    let mut seti = IndexedSet::from_ranked(&setr,false); 
    seti.minsert(16.5);
    println!("Inserted 16.5 to {}",seti);
    println!("{}",&OrderedSet::from_indexed(&seti,false));
    seti.mdelete(4.);
    println!("Deleted 4 from {}",seti);  
    let mut seto = OrderedSet::from_indexed(&seti,false);
    println!("{}",seto); 
    seto.minsert(9.5);
    println!("Inserted 9.5 to {}",seto); 
    seto.mdelete(9.5);
    println!("Deleted 9.5 from {}",seto);                
    setr.mintersection(&setr2);
    println!("Intersection-> {}",&setr);
    setr.mdifference(&setr2);
    println!("Difference-> {}",&setr);
   
    
}
 
#[test]
fn nlptest() { 
   let sentence1 = "Alphabetic ordering puts capital Letters first.";
   let sentence2 = "It sorts by Letters, ordering by Letters"; 
   let v1 = sentence1.split(' ').collect::<Vec<_>>();
   let v2 = sentence2.split(' ').collect::<Vec<_>>();  
   let setv = RankedSet::from_slice(&v1,false);  
   println!("{}",setv); // Display of Set
   println!("Reverse-> {}",setv.reverse()); 
   println!("Nonrepeat-> {}",setv.nonrepeat()); // Display of Set    
   println!("Is {} a member? {}\n",&"Spain",setv.member("Spain")); 
   println!("Infsup: {}",setv.infsup());
   let setw = RankedSet::from_slice(&v2,true);
   println!("{}",setw);
   let us = setw.union(&setv);
   println!("Union-> {}",us);
   println!("Intersection-> {}",setw.intersection(&setv));
   println!("Difference-> {}",setv.difference(&setw));
}