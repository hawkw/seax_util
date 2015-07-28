#![crate_name = "seax_compiler_tools"]
#![crate_type = "lib"]
#![stable(feature = "tools", since = "0.0.1")]
#![feature(staged_api)]
#![staged_api]

#[macro_use]
extern crate seax_svm as svm;

#[stable(feature = "forktable", since = "0.0.1")]
pub mod forktable;
#[stable(feature = "ast", since = "0.0.1")]
pub mod ast;

#[stable(feature = "scope", since = "0.0.1")]
pub type Index = (u64, u64);

/// The symbol table for bound names is represented as a
/// `ForkTable` mapping `&str` (names) to `(uint,uint)` tuples,
/// representing the location in the `$e` stack storing the value
/// bound to that name.
#[stable(feature = "forktable", since = "0.0.1")]
pub type SymTable<'a> = self::forktable::ForkTable<'a, &'a str, Index>;

/// A `CompileResult` is either `Ok(SVMCell)` or `Err(&str)`
#[stable(feature = "compile", since = "0.0.1")]
pub type CompileResult = Result<Vec<svm::cell::SVMCell>, String>;
