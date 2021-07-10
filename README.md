# Sets ![Crates.io](https://img.shields.io/crates/v/sets?logo=rust) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/liborty/sets/HEAD?logo=github)  

## Usage

Insert into your Cargo.toml file [dependencies] section: `sets = "^0.1"`  
Import into your source file(s) the structs and trait:  
```use sets::{Set,OrderedSet,IndexedSet,RankedSet,SetOps,MutSetOps};```

The initialisers and conversions are associated with their structs, e.g.:  
```let s = Set::from_slice(&v);```

The rest are methods of the trait SetOps, e.g.:  
`let su = s.nonrepeat(); // new set with unique elements`

See tests/tests.rs for example usage.

## Description

Crate `sets` consists mostly of structs `Set, OrderedSet, IndexedSet, RankedSet`, which are type-safe wrappers for the more primitive imported functions and methods from crate `indxvec`.

The main capabilities of `sets` include: efficient sorting, ranking, merging, searching and indices manipulations. The structs contain generic vectors `Vec<T>`. Thus they will work with vectors/slices of primitive types but also on any arbitrarily complex end type `T`, as long as the required traits `PartialOrd` and `Copy`, are implemented for `T`.

## Trait SetOps

Implements methods:  
`reverse, nonrepeat, infsup, member, search, union, intersection, difference`,    
for all four types of sets. Some of these methods are more efficient for the ordered and indexed sets, rather than for the unordered sets. For example, `member` and `search` are then able to use binary search. Union is like the classical merge but only one copy of items that were present in both input sets is kept. To remove repetitions from a set, use `nonrepeat`.

Union, interesection and difference applied to IndexedSet(s) or RankedSet(s) for now return only OrderedSet(s). Should this not be what is wanted, convert the result, or better still, use `munion, minteresection and mdifference`, (see below), which do not have this restriction.

## Trait MutSetOps

Implements methods:  
`mreverse, mnonrepeat, munion, mintersection, mdifference`,  
for all four types of sets. They overwrite the mutable set to which they are applied with the result. Thus they are not *functional* but in this context of handling potentially large vectors, they are in some cases simpler and more efficient.

## Release Notes (Latest First)

**Version 0.1.6** - implemented `MutSetOps` for all set types and added some tests.

**Version 0.1.5** - implemented `SetOps` for `RankedSet`, making the implementations now complete. Future work: adding  mutable sets.

**Version 0.1.4** - updated readme, implemented `SetOps` for `IndexedSet`.

**Version 0.1.3** - fixed readme typos, improved tests, implemented `SetOps` for `OrderedSet`.

**Version 0.1.2** - implemented `SetOps` trait for struct `Set`. The other three structs will follow in the next versions.

**Version 0.1.1** - competed the associated functions for all initiations and conversions of the four set structs.

**Version 0.1.0** - first version, includes creation and conversions of the structs representing the four types of sets.
