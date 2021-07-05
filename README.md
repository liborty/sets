# Sets ![Crates.io](https://img.shields.io/crates/v/sets?logo=rust) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/liborty/sets/HEAD?logo=github)  

## Usage

Insert into your Cargo.toml file [dependencies] section:

```rust
sets = "^0.1" 
```

Import into your source file(s) macro `here`, structs and functions, as needed.
  
```rust
use sets::{here,Set,OrderedSet,IndexedSet,RankedSet,functions::*};
```

See tests/tests.rs for examples of usage. 

## Description

`Sets` consists mostly of structs `Set, OrderedSet, IndexedSet, RankedSet` , which are type-safe wrappers for the more primitive functions and methods of crate `indxvec`.

The main capabilities of `sets` include: efficient sorting, ranking, merging, searching and indices manipulations. More set operations will be added in future releases. The structs contain generic vectors `Vec<T>` and/or generic slices `&[T]`. Thus they will work with vectors/slices of primitive types but also on any arbitrarily complex end type `T`. As long as the required traits, mostly just `PartialOrd` and `Copy`, are implemented for `T`.

## Functions

Most of the functions are associated with the supplied structs: `Set, OrderedSet, IndexedSet, RankedSet`. Thus they need to be called as in: ```Set::from_slice(&v);```  
Some additional helper functions are in the module `src/functions.rs`.

## Release Notes (Latest First)

**Version 0.1.1** -

**Version 0.1.0** - first version, includes creation and conversions of the structs representing the four types of sets.
