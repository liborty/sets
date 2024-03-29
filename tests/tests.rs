#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]

// use anyhow::{Result};
use sets::{Set,MutSetOps};
use indxvec::{Printing,Indices,Vecops};

#[test]
fn conversions() { 
   let v = vec![1.,14.,2.,13.,3.,12.,4.,11.,5.,10.,10.,6.,9.,7.,8.,16.];
   let setv = Set::new_unordered(&v);  
   println!("{}",setv); // Display setv 
   println!("{}",setv.to_ordered(true)); // ordered, ascending  
   println!("{}",setv.to_indexed(false)); // indexed, descending
   let rset = setv.to_ranked(false).to_ordered(false);
   println!("{}",rset); 
   println!("{}",setv.to_same(&rset)); 
}

#[test]
fn settest() { 
   let v = vec![1.,14.,2.,13.,3.,12.,4.,11.,5.,10.,10.,6.,9.,7.,8.,16.];
   println!("Data: {}\n",v.bl()); // Display of Set   
   let sv = Set::new_unordered(&v); 
   println!("Where is {}? at {}\n",12.bl(),sv.search(12.0).map_or_else(||"None".rd(),|x|x.gr()));  
   let setv = sv.to_ranked(false);  
   println!("{}",setv); // Display of Set
   println!("Reverse-> {}",setv.reverse()); 
   println!("Nonrepeat-> {}",setv.nonrepeat()); // Display of Set    
   println!("Is {} a member? {}\n",0.0.bl(),setv.member(0.0).gr());
   println!("Where is {} (from descending ranked set)? at {}\n",12.bl(),setv.search(12.0).map_or_else(||"None".rd(),|x|x.gr())); 
   println!("Infsup: {}\n",setv.infsup());
   let setw = Set::new_unordered(&[20.,19.,18.,17.,16.,15.]);
   println!("{}",setw);
   let us = setw.union(&setv);
   println!("Union-> {}",&us);
   println!("Intersection-> {}",setw.intersection(&setv));
   println!("Difference-> {}",setv.difference(&setw));
}

#[test]
fn mutabletest() { 
    let v = vec![1.,14.,2.,13.,3.,12.,4.,11.,5.,10.,10.,6.,9.,7.,8.,16.];
    let mut setu = Set::new_unordered(&v);
    println!("{}",setu);
    setu.minsert(10.5);
    println!("Inserted 10.5 to {}",setu); // Display of Set 
    let mut setr = setu.to_ranked(false);  
    println!("{}",setr); // Display of RankedSet
    setr.mnonrepeat();
    println!("Mnonrepeat {}",setr); //    
    let setr2 = Set::new_unordered(&[20.,19.,18.,17.,16.,15.]);
    println!("New {}",setr2); // Display of ascending ranked set
    setr.munion(&setr2); 
    setr.mreverse();
    println!("Union->  {}",&setr);
    println!("{}",setr.to_ranked(false));        
    setr.mdelete(10.5);
    println!("Deleted 10.5 from {}",&setr); 
    println!("{}",Set::to_ordered(&setr,true));

    let mut seti = Set::to_indexed(&setr,false); 
    seti.minsert(16.5);
    println!("Inserted 16.5 to {}",seti);
    println!("{}",Set::to_ordered(&seti,false));
    seti.mdelete(4.);
    println!("Deleted 4 from {}",seti);  
    let mut seto = Set::to_ordered(&seti,false);
    println!("{}",seto); 
    seto.minsert(9.5);
    println!("Inserted 9.5 to {}",seto);
    let mut union = seti.union(&seto);
    println!("Union {}",union);  
    let count = union.mdeleteall(16.);  
    println!("Deleted {} 16s from {}",count,union);                
    setr.mintersection(&setr2);
    println!("Intersection of\n{}\n{} -> {}",&setr,setr2,&setr);
    setr.mdifference(&setr2);
    println!("Difference-> {}",&setr);
   
    
}
 
#[test]
fn nlptest() { 
   let sentence1 = "Alphabetic ordering puts punctuation first first and capital Letters first .";
   let v1 = sentence1.split(' ').collect::<Vec<_>>();
   let sentence2 = "It sorts by Letters, ordering by Letters";  
   let v2 = sentence2.split(' ').collect::<Vec<_>>();  
   let setv = Set::new_ranked(&v1,true);  
   println!("{}",setv); // Display of Set
   println!("Reverse-> {}",setv.reverse()); 
   println!("Nonrepeat-> {}",setv.nonrepeat()); // Display of Set    
   println!("Is {} a member? {}\n",&"Spain",setv.member("Spain").rd()); 
   println!("Infsup of original data: {}",setv.infsup());
   let setw = Set::new_indexed(&v2,true);
   println!("\nNew {}",setw);
   let us = setw.union(&setv);
   println!("Union-> {}",us);
   println!("Intersection-> {}",setw.intersection(&setv));
   let mut diff = setv.difference(&setw);
   println!("Difference-> {}",&diff);
   diff.mnonrepeat();
   println!("Nonrepeat -> {}",diff);   
}