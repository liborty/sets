# Sets

[<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/sets/HEAD?logo=github">](https://github.com/liborty/sets)
[<img alt="crates.io" src="https://img.shields.io/crates/v/sets?logo=rust">](https://crates.io/crates/sets)
[<img alt="crates.io" src="https://img.shields.io/crates/d/sets?logo=rust">](https://crates.io/crates/sets)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/sets?logo=rust&logoColor=white">](https://docs.rs/sets/)

## Description

This crate defines `Structs: Set, OrderedSet, IndexedSet, RankedSet` and methods acting on them. These structs are type-safe wrappers for the more primitive imported functions and methods from crate `indxvec`.

The main capabilities of `sets` include: efficient sorting, ranking, merging, searching and indices manipulations. The structs contain generic vectors `Vec<T>`. Thus they will work with vectors or slices of primitive end types but also with any arbitrarily complex end type `T`, as long as the required traits `PartialOrd` and `Copy`, are implemented for `T`.

## Usage

Insert into your Cargo.toml file [dependencies] section: `sets = "^1"`  
Import into your source file(s) the four structs for the four different types of sets and the two traits `SetOps` and `MutSetOps`. The following 'use' declaration imports everything:

```rust
use sets::{Set,OrderedSet,IndexedSet,RankedSet,SetOps,MutSetOps};
```

The initialisers and convertors are associated with their structs, hence the `::` syntax, e.g.:

```rust
let s = Set::from_slice(&v);
```

Example use of methods from the traits `SetOps`, and `MutSetOps`:

```rust
// new mutable set with unique elements  
let mut su = s.nonrepeat();
// transformed in place into the opposite order  
su.mreverse; 
```

It is highly recommended to read and run `tests/tests.rs` for many more examples of usage. Use a single thread to run them. It may be a bit slower but it will write the results in the right order:

`cargo test --release -- --test-threads=1 --nocapture --color always`

## Trait SetOps

Implements the following methods for all four types of sets (`Struct`s):

`reverse, nonrepeat, infsup, member, search, union, intersection, difference`.

 Some of these methods are more efficient for the ordered and indexed sets, rather than for the unordered sets. For example, `member` and `search` are then able to use binary search. Union is like the classical merge but only one copy of items that were present in both input sets is kept. To remove repetitions from a single set at any other time, use `nonrepeat`.

`intersection` and `difference`, when applied to IndexedSet(s) and RankedSet(s) return an OrderedSet as a result. This result can be explicitly converted to other types of sets when needed. 

`Union` returns the same type as the one to which it is applied. Thus, for example, union of two (unordered) `Set`s will produce another unordered `Set` (just their concatenation).

`munion, minteresection and mdifference`, (where 'm' stands for 'mutable', see below), will overwrite `self` with the resulting set of the same type.

## Trait MutSetOps

Implements the following methods for all four types of sets:

`mreverse, mnonrepeat, munion, mintersection, mdifference`.

They overwrite the mutable set to which they are applied with the result. Thus they are not *functional* but in the context of handling large vectors, they are often simpler and more efficient.

## Release Notes (Latest First)

**Version 1.0.3** - updated to be compatible with `indxvec` version 1.2.1. Improved `munion`.

**Version 1.0.2** - some changes to printing to reflect changes to `indxvec`.

**Version 1.0.1** - some tidying up of code, no changes of functionality.

**Version 1.0.0** - stable version with some minor improvements to README.md (this document). Updated to `indxvec = "^1"` and Rust edition 2021.

**Version 0.1.8** - `infsup` now returns Struct MinMax (defined in crate `sets`).

**Version 0.1.7** - just some cosmetic cleaning up. No change of functionality from the previous version.

**Version 0.1.6** - implemented `MutSetOps` for all set types and added some tests.

**Version 0.1.5** - implemented `SetOps` for `RankedSet`, making the implementations now complete. Future work: adding  mutable sets.

**Version 0.1.4** - updated readme, implemented `SetOps` for `IndexedSet`.

**Version 0.1.3** - fixed readme typos, improved tests, implemented `SetOps` for `OrderedSet`.

**Version 0.1.2** - implemented `SetOps` trait for Struct `Set`. The other three Structs will follow in the next versions.

**Version 0.1.1** - competed the associated functions for all initiations and conversions of the four set Structs.

**Version 0.1.0** - first version, includes creation and conversions of the Structs representing the four types of sets.
