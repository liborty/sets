# Sets [<img alt="crates.io" src="https://img.shields.io/crates/v/sets?logo=rust">](https://crates.io/crates/sets) [<img alt="crates.io" src="https://img.shields.io/crates/d/sets?logo=rust">](https://crates.io/crates/sets) [<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/sets/HEAD?logo=github">](https://github.com/liborty/sets) [![Actions Status](https://github.com/liborty/sets/workflows/test/badge.svg)](https://github.com/liborty/sets/actions)

Set operations, plus efficient sorting, ranking, searching, etc. The aim is to avoid moving data as much as possible. This is done by manipulating indices instead.

## Short Description

This crate defines `Struct: Set`, which wraps five kinds of sets: Empty, Unordered, Ordered, Indexed and Ranked, and methods acting upon them. These methods work on any generic vectors (or slices) of primitive end types `<T>`. Also, on any arbitrarily complex user end type, as long as the required traits `PartialOrd` and `Copy`, are implemented for it (by the user). It adds organisation and type safety to lower level methods from crate `indxvec`.

## Usage

Insert into `Cargo.toml` file, under [dependencies]: `sets = "^1"`  
The following 'use' declaration in source files makes available everything:

```rust
use sets::{Set,MutSetOps};
```

## `Set<T>`

```rust
/// The struct type for sets
#[derive(Default)]
pub struct Set<T> {
    /// type of the set
    pub stype: SType,
    /// order: ascending (true), descending (false)
    pub ascending: bool,
    /// data Vec
    pub data: Vec<T>,
    /// index Vec
    pub index: Vec<usize>
}
```

`Clone` and `Display` traits are implemented for `Set` and `SType`.  
`Default` is derived, thus `Default::default()` generates an empty set.

`SType` specifies one of the five kinds of sets. It is dealt with by 'enumeration generics'.

```rust
/// The five types of sets
#[derive(Default,Clone,Copy)]
pub enum SType {
    /// empty set
    #[default]
    Empty,
    /// unordered set
    Unordered,
    /// ordered set
    Ordered,
    /// indexed set
    Indexed,
    /// ranked set
    Ranked
}
```

### Associated Initialisers

Initialisers are associated with the struct Set, hence to invoke them, the `::` syntax is necessary, e.g. `Set::new(..)`

```rust
    /// all in one Initialiser: creates a new Set
    /// of any self_type SType, from slice d, in asc order 
    pub fn new(set_type: SType, d: &[T], asc:bool) -> Self
```

There are also explicitly named convenience functions for all STypes:
`new_empty, new_unordered, new_ordered, new_indexed, new_ranked`. All the ordered types (i.e. ordered, indexed, ranked) take a bool argument specifying ascending or descending order.

### Converters

```rust
    /// General converter - 
    /// converts s to a Set of the same type and order as self 
    /// (self only serves as a template).
    pub fn to_same(&self, s:&Self) -> Self 
```

Again, we have explicitly named converters:
`to_unordered, to_ordered, to_indexed, to_ranked`.

```rust
   let v = vec![1.,14.,2.,13.,3.,12.];
   let setv = Set::new_unordered(&v);  
   println!("{}",setv); // Display setv 
   // ordered, ascending  
   println!("{}",setv.to_ordered(true)); 
   // indexed, descending
   println!("{}",setv.to_indexed(false)); 
```

It is highly recommended to read and run [`tests/tests.rs`](https://github.com/liborty/sets/blob/main/tests/tests.rs) for more examples of usage. Use a single thread to run them. It may be a bit slower but it will write the results in the right order:

```bash
cargo test --release -- --test-threads=1 --nocapture --color always
```

The output can be seen by clicking the last badge above and then the automated test logs therein.

## Set Functions

 Some of the general methods are more efficient for the ordered and indexed sets, rather than for the unordered sets. For example, `member` and `search` will automatically use the binary search. Union is like the classical merge with duplications across the sets removed. To remove repetitions within a set, use `nonrepeat`.

The STypes of the two operands of union, intersection and difference can be different. However, they are required to have the same end-type `<T>`. This is, perhaps, a useful type discipline. 

## Trait MutSetOps

Here 'm' in the methods' names stands for 'mutable'. They overwrite the mutable set to which they are applied with the result. Thus they are not *functional* but in the context of handling large vectors, they are often simpler and more efficient. At the price of destroying the previous contents of self, of course.

```rust
/// Mutable methods for &mut Set<T>
pub trait MutSetOps<T> {
    /// Makes a Set unordered
    fn munordered(&mut self);
    /// Makes a Set ordered
    fn mordered(&mut self, asc:bool) where f64:From<T>;
    /// Makes any Set indexed
    fn mindexed(&mut self,asc:bool) where f64:From<T>;
    /// Converts any Set type to ranked
    fn mranked(&mut self,asc:bool);
    /// General converter: s -> Set of the same type and order as self
    fn msame(&mut self, s:&mut Self) where f64:From<T>; 
    /// Deletes the first item from self
    fn mdelete(&mut self, item:T) -> bool;
    /// Deletes all occurrences of a matching item from self, returns their count
    fn mdeleteall(&mut self, item:T) -> usize;
    /// Inserts an item of the same end-type to self
    fn minsert(&mut self, item:T);
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

**Version 1.1.4** - Updated dependency `indxvec 1.4`. Added automated github actions test.

**Version 1.1.2** - Updated to indxvec 1.3.3. Pruned and simplified some code. Added `deleteall` to  trait `MutSetOps`.

**Version 1.1.1** - Eliminating unnecessary cloning. Updating to the latest dependency on  indxvec 1.2.8.

**Version 1.1.0** - Joined all four types of sets into one Struct Set. Simplified and generalised code by using enum generics.

**Version 1.0.6** - Added mutable methods `minsert` and `mdelete` to `MutSetOps`, that insert or remove one specific item to/from any of the sets. Added tests of them to `tests/tests.rs`. Updated `indxvec` dependency to its version `1.2.4` or greater.

**Version 1.0.5** - Documentation improvements.

**Version 1.0.4** - `nonrepeat` now always returns an OrderedSet. Clarified `settest`.

**Version 1.0.3** - updated to be compatible with `indxvec` version 1.2.1. Improved `munion`.

**Version 1.0.2** - some changes to printing to reflect changes to `indxvec`.

**Version 1.0.1** - some tidying up of code, no changes of functionality.

**Version 1.0.0** - stable version with some minor improvements to `README.md` (this document). Updated to `indxvec = "^1"` and Rust edition 2021.
