#![crate_name = "seax_util"]
#![crate_type = "lib"]
#![cfg_attr(feature = "nightly", feature(vec_push_all))]
#![cfg_attr(feature = "nightly", feature(staged_api))]
#![cfg_attr(feature = "nightly", staged_api)]
#![cfg_attr(feature = "nightly", stable(feature = "util", since = "0.0.1"))]

//! Seax Compiler Tools
//! -------------------
//!
//! Library containing general-purpose tools for compiling programs for
//! the [Seax](hawkweisman.me/seax) platform.

#[macro_use] extern crate log;
#[cfg(test)] extern crate quickcheck;
extern crate byteorder;

/// List
/// ----
///
/// Contains singly-linked list and stack implementations.
///
/// `List<T>` is a singly-linked `cons` list.
/// `Stack<T>` is a trait providing stack operations(`push()`, `pop()`, and
/// `peek()`), and an implementation for `List`.
#[cfg_attr(feature = "nightly", stable(feature="list", since="0.1.0"))]
#[macro_use] pub mod list;

/// Cell
/// ----
///
/// Contains the SVM cell and instruction types.
///
/// A cell in the VM can be either an atom (single item, either unsigned
/// int, signed int, float, or string), a pointer to a list cell, or an
/// instruction.
#[cfg_attr(feature = "nightly", stable(feature = "cell", since = "0.1.0"))]
#[macro_use] pub mod cell;

/// Bytecode
/// --------
///
/// Contains code for encoding and decoding SVM cells to and from bytecode.
#[cfg_attr(feature = "nightly", unstable(feature = "bytecode"))]
pub mod bytecode;

/// Compiler Tools
/// --------------
///
/// Contains general-purpose code for compilers targeting the Seax platform.
#[cfg_attr(feature = "nightly", stable(feature = "compile", since = "0.0.1"))]
pub mod bytecode;

// Reexports
pub use self::list::{List, Stack};
pub use self::list::List::{Cons,Nil};
pub use self::cell::{SVMCell,Atom,Inst};
