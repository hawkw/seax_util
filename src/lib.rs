#![crate_name = "seax_util"]
#![crate_type = "lib"]
#![cfg_attr(feature = "unstable", feature(vec_push_all))]
#![cfg_attr(feature = "unstable", feature(staged_api))]
#![cfg_attr(feature = "unstable", staged_api)]
#![cfg_attr(feature = "unstable", stable(feature = "util", since = "0.0.1"))]

//! Utility library containing code for building programs involving the
//! [Seax](hawkweisman.me/seax) platform. Seax is a virtual-machine-based
//! runtime environment for functional programming languages.
//!
//! This crate contains the following modules:
//!
//! + `cell`: Contains the definitions of all Seax Virtual Machine cell types,
//!   including instruction, atom, and list cells.
//! + `list`: Contains the singly-linked list and stack implementations used
//!   by the Seax VM internally.
//! + `bytecode`: Contains functions for encoding and decoding Seax VM cells
//!   to and from Seax bytecode
//! + `compiler_tools`: Contains reusable code for implementing compilers
//!   targeting the Seax platform, including traits for abstract syntax trees
//!   and symbol tables.

#[macro_use] extern crate log;
#[cfg(test)] extern crate quickcheck;
extern crate byteorder;

/// Contains singly-linked list and stack implementations.
///
/// `List<T>` is a singly-linked `cons` list.
/// `Stack<T>` is a trait providing stack operations(`push()`, `pop()`, and
/// `peek()`), and an implementation for `List`.
#[cfg_attr(feature = "unstable",
    stable(feature = "list", since="0.1.0") )]
#[macro_use] pub mod list;

/// Seax VM cell and instruction types.
///
/// A cell in the VM can be either an atom (single item, either unsigned
/// int, signed int, float, or string), a pointer to a list cell, or an
/// instruction.
#[cfg_attr(feature = "unstable",
    stable(feature = "cell", since = "0.1.0") )]
#[macro_use] pub mod cell;

/// Functions for encoding and decoding Seax bytecode.
#[cfg_attr(feature = "unstable",
    unstable(feature = "bytecode") )]
pub mod bytecode;

/// General-purpose code for compilers targeting the Seax platform.
#[cfg_attr(feature = "unstable",
    stable(feature = "compile", since = "0.0.1") )]
pub mod compiler_tools;

// Reexports
pub use self::list::{List, Stack};
pub use self::list::List::{Cons,Nil};
pub use self::cell::{SVMCell,Atom,Inst};
