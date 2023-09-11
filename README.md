# Sets [![crates.io](https://img.shields.io/crates/v/sets?logo=rust)](https://crates.io/crates/sets) [![crates.io](https://img.shields.io/crates/d/sets?logo=rust)](https://crates.io/crates/sets) [!["GitHub last commit"](https://img.shields.io/github/last-commit/liborty/sets/HEAD?logo=github)](https://github.com/liborty/sets) [![Actions Status](https://github.com/liborty/sets/workflows/test/badge.svg)](https://github.com/liborty/sets/actions)

## Author: Libor Spacek

Written in 100% safe Rust.

Set operations, plus efficient sorting, ranking, searching, etc. The aim is to avoid moving data as much as possible. This is done by manipulating indices instead.

This crate defines `Struct: Set`, which wraps five kinds of sets: Empty, Unordered, Ordered, Indexed and Ranked, and methods acting upon them. These methods work on any generic vectors (or slices) of primitive end types `<T>`. Also, on any arbitrarily complex user end type, as long as the required traits `PartialOrd` and `Copy`, are implemented for it (by the user). It adds organisation and type safety to lower level methods from crate `indxvec`.

## Usage

Insert into `Cargo.toml` file, under [dependencies]: `sets = "^1.2"`  
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

## Release Notes (Latest First)

**Version 1.2.1** - Updated to `indxvec 1.8`. The closure arguments in `MutSetOps` are now simpler. They no longer need to be `&mut`.

**Version 1.2.0** - Updated to `indxvec 1.4.9` and introduced compatible generalizations. No longer requiring users to globally implement `From` trait for all their types T but instead specify conversion closures on per-individual-use basis. A closure is easier to use and here it allows using the most efficient hashsort for the sorting of the sets. This allows custom dynamic conversions. Beware that this breaks previous usage of `mordered`, `mindexed` and `msame` methods of `MutSetOps` trait.
