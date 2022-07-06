# Sets

[<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/sets/HEAD?logo=github">](https://github.com/liborty/sets)
[<img alt="crates.io" src="https://img.shields.io/crates/v/sets?logo=rust">](https://crates.io/crates/sets)
[<img alt="crates.io" src="https://img.shields.io/crates/d/sets?logo=rust">](https://crates.io/crates/sets)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/sets?logo=rust&logoColor=white">](https://docs.rs/sets/)

## Description

This crate defines `Struct: Set`, encompassing five kinds of sets: Empty, Unordered, Ordered, Indexed, Ranked, and common methods acting on them. This `Struct` is a type-safe wrapper for the more primitive imported functions and methods from crate `indxvec`.

The main capabilities of `Sets` include the usual set operations, plus efficient sorting, ranking, searching, etc. The aim is to avoid moving data as much as possible. This is done by manipulating indices instead. These methods work on generic vectors (or slices) of primitive end types `<T>`. They will also work on any arbitrarily complex user end type, as long as the required traits `PartialOrd` and `Copy`, are implemented for it by the user.

## Usage

Insert into `Cargo.toml` file, under [dependencies]: `sets = "^1"`  
The following 'use' declaration in source files makes available everything:

```rust
use sets::{Set,MutSetOps};
```

## Initialisers and Converters

`unordered_from_slice()` wraps raw data slice &[T] in Set of Unordered type.

`to_unordered, to_ordered, to_indexed, to_ranked` implement conversions to all types of Sets from all types.

Initialisers and converters are associated with the Set Struct, hence the `::` syntax is necessary, e.g.:

```rust
// Unordered set from slice v
let su = Set::from_slice(&v);
// Creates a descending sort index for v
let si = s.to_indexed(false);
// A mutable indexed set msiu, with unique elements, from si 
let mut msiu = si.nonrepeat();
// msiu mutated-reversed in place into the opposite order  
msiu.mreverse; 
```

It is highly recommended to read and run `tests/tests.rs` for many more examples of usage. Use a single thread to run them. It may be a bit slower but it will write the results in the right order:

```bash
cargo test --release -- --test-threads=1 --nocapture --color always
```

## Set Associated Functions

 Some of the methods are more efficient for the ordered and indexed sets, rather than for the unordered sets. For example, `member` and `search` are then able to use binary search. Union is like the classical merge but only one copy of items that were present in both input sets is kept. To remove repetitions from a single set at any other time, use `nonrepeat`.

 The set operations between two operands are required to have the same end-type `<T>`. This is possibly a good type discipline but it could easily be relaxed.

## Trait MutSetOps

Here 'm' in the methods' names stands for 'mutable'. They overwrite the mutable set to which they are applied with the result. Thus they are not *functional* but in the context of handling large vectors, they are often simpler and more efficient. At the price of destroying the previous contents of self, of course.

Implements the following methods for &mut Set:

```rust
pub trait MutSetOps<T> {
    /// Deletes an item of the same end-type from self
    fn mdelete(&mut self, item:T) -> bool;
    /// Inserts an item of the same end-type to self
    fn minsert(&mut self, item:T);
    /// reverses the explicit sets, 
    /// or index of indexed sets
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

**Version 1.1.0** - Joined all four types of sets into one Struct Set. Simplified code using enum generics. 

**Version 1.0.6** - Added mutable methods `minsert` and `mdelete` to `MutSetOps`, that insert or remove one specific item to/from any of the sets. Added tests of them to `tests/tests.rs`. Updated `indxvec` dependency to its version `1.2.4` or greater.

**Version 1.0.5** - Documentation improvements.

**Version 1.0.4** - `nonrepeat` now always returns an OrderedSet. Clarified `settest`.

**Version 1.0.3** - updated to be compatible with `indxvec` version 1.2.1. Improved `munion`.

**Version 1.0.2** - some changes to printing to reflect changes to `indxvec`.

**Version 1.0.1** - some tidying up of code, no changes of functionality.

**Version 1.0.0** - stable version with some minor improvements to `README.md` (this document). Updated to `indxvec = "^1"` and Rust edition 2021.
