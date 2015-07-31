#![crate_name = "seax_util"]
#![crate_type = "lib"]
#![cfg_attr(feature = "nightly", feature(vec_push_all) )]
#![cfg_attr(feature = "nightly", feature(staged_api) )]
#![cfg_attr(feature = "nightly", staged_api)]
#![cfg_attr(feature = "nightly", stable(feature = "tools", since = "0.0.1"))]

//! Seax Compiler Tools
//! -------------------
//!
//! Library containing general-purpose tools for compiling programs for
//! the [Seax](hawkweisman.me/seax) platform.

#[macro_use] extern crate log;
#[cfg(test)] extern crate quickcheck;
extern crate byteorder;

/// Singly-linked list and stack implementations.
///
/// `List<T>` is a singly-linked `cons` list.
/// `Stack<T>` is a trait providing stack operations(`push()`, `pop()`, and
/// `peek()`), and an implementation for `List`.
#[cfg_attr(feature = "nightly",
    stable(feature="list", since="0.1.0")
)]
#[macro_use]
pub mod list;

// Reexports
pub use self::list::{List, Stack};
pub use self::list::List::{Cons,Nil};
pub use self::cell::{SVMCell,Atom,Inst};

/// SVM cell types.
///
/// A cell in the VM can be either an atom (single item, either unsigned
/// int, signed int, float, or string), a pointer to a list cell, or an
/// instruction.
#[macro_use]
#[cfg_attr(feature = "nightly",
    stable(feature="vm_core", since="0.1.2")
)]
pub mod cell;

#[cfg_attr(feature = "nightly",
    unstable(feature="bytecode")
)]
pub mod bytecode;

#[cfg_attr(feature = "nightly",
    stable(feature = "forktable", since = "0.0.1")
)]
pub mod forktable;
#[cfg_attr(feature = "nightly",
    stable(feature = "ast", since = "0.0.1")
)]
pub mod ast;

#[cfg_attr(feature = "nightly",
    stable(feature = "scope", since = "0.0.1")
)]
pub type Index = (u64, u64);

/// The symbol table for bound names is represented as a
/// `ForkTable` mapping `&str` (names) to `(uint,uint)` tuples,
/// representing the location in the `$e` stack storing the value
/// bound to that name.
#[cfg_attr(feature = "nightly",
    stable(feature = "forktable", since = "0.0.1")
)]
pub type SymTable<'a> = self::forktable::ForkTable<'a, &'a str, Index>;

/// A `CompileResult` is either `Ok(SVMCell)` or `Err(&str)`
#[cfg_attr(feature = "nightly",
    stable(feature = "compile", since = "0.0.1")
)]
pub type CompileResult = Result<Vec<cell::SVMCell>, String>;

/// Trait for a symbol table
#[cfg_attr(feature = "nightly",
    stable(feature = "scope", since = "0.0.1")
)]
pub trait Scope<K>
    where K: Eq + std::hash::Hash
{
    /// Bind a name to a scope.
    ///
    /// Returnsthe indices for that name in the SVM environment.
    #[cfg_attr(feature = "nightly",
        stable(feature = "scope", since = "0.0.1")
    )]
    fn bind(&mut self, name: K, lvl: u64) -> Index;
    /// Look up a name against a scope.
    ///
    /// Returns the indices for that name in the SVM environment,
    /// or None if that name is unbound.
    #[cfg_attr(feature = "nightly",
        stable(feature = "scope", since = "0.0.1")
    )]
    fn lookup(&self, name: &K)            -> Option<Index>;
}
