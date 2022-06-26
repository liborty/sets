# Sets

[<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/sets/HEAD?logo=github">](https://github.com/liborty/sets)
[<img alt="crates.io" src="https://img.shields.io/crates/v/sets?logo=rust">](https://crates.io/crates/sets)
[<img alt="crates.io" src="https://img.shields.io/crates/d/sets?logo=rust">](https://crates.io/crates/sets)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/sets?logo=rust&logoColor=white">](https://docs.rs/sets/)

## Description

This crate defines `Structs: Set, OrderedSet, IndexedSet, RankedSet` and methods acting on them. These `Struct`s are type-safe wrappers for the more primitive imported functions and methods from crate `indxvec`.

The main capabilities of `sets` include: efficient sorting, ranking, merging, searching and indices manipulations. These methods work with generic vectors or slices of primitive end types. They will also work with any arbitrarily complex user end type, as long as the required traits `PartialOrd` and `Copy`, are implemented for it (by the user).

## Usage

Insert into your `Cargo.toml` file [dependencies] section: `sets = "^1"`  
Import into your source file(s) the four `Struct`s for the four different types of sets and the two traits `SetOps` and `MutSetOps`. The following 'use' declaration imports everything:

```rust
use sets::{Set,OrderedSet,IndexedSet,RankedSet,SetOps,MutSetOps};
```

## Initialisers and Converters

`from_slice(), from_set(), from_indexed, from_ranked`

Initialisers and converters are associated with their type `Struct`s, hence the `::` syntax is necessary, e.g.:

```rust
// Unordered set from slice v
let s = Set::from_slice(&v);
// Automatically creates a descending sort index for v
let si = IndexedSet::from_slice(&v,false);
```

Example use of methods from the traits `SetOps`, and `MutSetOps`:

```rust
// Mutable set su with unique elements from s 
let mut su = s.nonrepeat();
// su mutated-reversed into the opposite order  
su.mreverse; 
```

It is highly recommended to read and run `tests/tests.rs` for many more examples of usage. Use a single thread to run them. It may be a bit slower but it will write the results in the right order:

```bash
cargo test --release -- --test-threads=1 --nocapture --color always
```

## Trait SetOps

Implements the following methods for all four types of sets (`Struct`s):

```rust
pub trait SetOps<T> {
    /// reverses the vector of explicit sets and index of indexed sets
    fn reverse(&self) -> Self;
    /// Deletes any repetitions
    fn nonrepeat(&self) -> Self;
    /// Finds minimum, its first index, maximum, its first index  
    fn infsup(&self) -> MinMax<T>; 
    /// True if m is a member of the set
    fn member(&self, m: T) -> bool;
    /// Some(index) of the first item found, or None.
    fn search(&self, m: T)  -> Option<usize>;    
    /// Union of two sets of the same type
    fn union(&self, s: &Self) -> Self;
    /// Intersection of two sets of the same type
    fn intersection(&self, s: &Self) -> OrderedSet<T>;
    /// Removing s from self (i.e. self-s)
    fn difference(&self, s: &Self) -> OrderedSet<T>;
}
```

 Some of these methods are more efficient for the ordered and indexed sets, rather than for the unordered sets. For example, `member` and `search` are then able to use binary search. Union is like the classical merge but only one copy of items that were present in both input sets is kept. To remove repetitions from a single set at any other time, use `nonrepeat`.

`intersection` and `difference`, when applied to IndexedSet(s) and RankedSet(s) return an OrderedSet as a result. This result can be explicitly converted to other types of sets when needed. 

`Union` returns the same type as the one to which it is applied. Thus, for example, union of two (unordered) `Set`s will produce another unordered `Set` (just their concatenation).

## Trait MutSetOps

Here 'm' in the methods' names stands for 'mutable'. They overwrite the mutable set to which they are applied with the result. Thus they are not *functional* but in the context of handling large vectors, they are often simpler and more efficient. At the price of destroying the previous contents of self, of course.

Implements the following methods for all four types of sets:

```rust
pub trait MutSetOps<T> {
    /// reverses the vector of explicit sets and index of indexed sets
    fn mreverse(&mut self);
    /// Deletes any repetitions
    fn mnonrepeat(&mut self); 
    /// Union of two sets of the same type
    fn munion(&mut self, s: &Self);
    /// Intersection of two sets of the same type
    fn mintersection(&mut self, s: &Self);
    /// Removing s from self (i.e. self-s)
    fn mdifference(&mut self, s: &Self);
}
```

## Release Notes (Latest First)

**Version 1.0.5** - Documentation improvements.

**Version 1.0.4** - `nonrepeat` now always returns an OrderedSet. Clarified `settest`.

**Version 1.0.3** - updated to be compatible with `indxvec` version 1.2.1. Improved `munion`.

**Version 1.0.2** - some changes to printing to reflect changes to `indxvec`.

**Version 1.0.1** - some tidying up of code, no changes of functionality.

**Version 1.0.0** - stable version with some minor improvements to `README.md` (this document). Updated to `indxvec = "^1"` and Rust edition 2021.
