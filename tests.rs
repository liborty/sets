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
   println!("{}",OrderedSet::from_slice(&sv,true)); // sorted data but index lost
   println!("{}",OrderedSet::from_slice(&sv,false)); // sorted data but index lost      
   println!("{}",IndexedSet::from_slice(&sv,true)); 
   println!("{}",IndexedSet::from_slice(&sv,false));   
   println!("{}",RankedSet::from_slice(&sv,true));
   println!("{}",RankedSet::from_slice(&sv,false));  
/*
   println!("Sorted:       {}",Set(&rank(&v,false).invindex().unindex(&v,false)));   
   println!("Ranks:        {}",Set(&rank(&v,true))); // how to get ranks
   println!("Ranks rev:    {}",Set(&revs(&rank(&v,true)))); // reverse funtion reverses any vector
   println!("Ranks desc:   {}",Set(&rank(&v,false))); // descending ranks, not the same as ranks reversed!!   
   println!("Sort index:   {}",Set(&sortidx(&v))); // sortindex, can be unindexed at anytime
   println!("Ranks to idx: {}",Set(&rank(&v,true).invindex()));  // ascending sort index from ranks
   println!("Sort ix desc: {}",Set(&rank(&v,false).invindex())); // descending sort index from ranks
   println!("Idx to ranks: {}",Set(&rank(&v,false).invindex().invindex())); // even inverses = original
   println!("Sorted rev:   {}",Set(&sortm(&v,false))); // descending sort, index lost
   println!("Sorted rev:   {}",Set(&revs(&sortm(&v,true)))); // the above simply reversed
   println!("Sorted rev:   {}",Set(&sortidx(&v).unindex(&v,false))); // more efficient reversal  
   println!("Sorted rev:   {}",Set(&rank(&v,false).invindex().unindex(&v,true))); // odd falses = reversed
   println!("Spearman corr against itself: {}",rank(&v,true).ucorrelation(&rank(&v,true))); //  1 for any Vec
   println!("Spearman corr against reversed: {}",rank(&v,true).ucorrelation(&rank(&v,false))); // -1 for any Vec
   let (vm,vi) = merge_indexed(&v,&sortidx(&v),&v,&sortidx(&v)); // merge two vecs using their sort indices
   println!("Twice sorted, Concatenated and Merged:\n{}",Set(&vi.unindex(&vm,true))); 
   println!("Searched for {}, found at: {}",14.0,binsearch(&vi.unindex(&vm,true),14.0)); // binary search 
   println!("{}", IndexedSet{ascending:true,v:vm,i:vi});
     let (min,minix,max,maxi) = minmax(&v);
   println!("Min {}, minidx {}, max {}, maxidx {}",min,minix,max,maxi);
    println!("Ranks to f64:\n{:?}",&rank(&v,true).indx_to_f64());  
   */
   ()
}
