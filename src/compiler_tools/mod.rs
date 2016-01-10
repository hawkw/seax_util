//
//  Seax
//  Copyright 2016 Hawk Weisman.
//
//  Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//  http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//  <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//  option. This file may not be copied, modified, or distributed
//  except according to those terms.
//
use std::hash::Hash;
use super::SVMCell;

#[cfg_attr(feature = "unstable",
    stable(feature = "forktable", since = "0.0.1") )]
pub mod forktable;


#[cfg_attr(feature = "unstable",
    stable(feature = "ast", since = "0.0.1") )]
pub mod ast;

#[cfg_attr(feature = "unstable",
    unstable(feature = "location", issue = "4") )]
pub mod location;

// Reexports
pub use self::forktable::ForkTable;
pub use self::ast::ASTNode;

/// Represents an index into the Seax VM's environment stack.
///
/// Since the environment stack is a stack of stacks, an index is represented
/// by two unsigned integers. The first represents the level in the `$e`
/// stack,  and the second represents the index within the list at that level.
#[cfg_attr(feature = "unstable",
    stable(feature = "scope", since = "0.0.1") )]
pub type Index = (u64, u64);

/// The symbol table for bound names is represented as a
/// `ForkTable` mapping `&str` (names) to `(uint,uint)` tuples,
/// representing the location in the `$e` stack storing the value
/// bound to that name.
#[cfg_attr(feature = "unstable",
    stable(feature = "forktable", since = "0.0.1") )]
pub type SymTable<'a> = forktable::ForkTable<'a, &'a str, Index>;

/// A `CompileResult` is either `Ok(SVMCell)` or `Err(&str)`
#[cfg_attr(feature = "unstable",
    stable(feature = "compile", since = "0.0.1") )]
pub type CompileResult = Result<Vec<SVMCell>, String>;

/// Trait for a symbol table
#[cfg_attr(feature = "unstable",
    stable(feature = "scope", since = "0.0.1") )]
pub trait Scope<K>
where K: Eq + Hash {
    /// Bind a name to a scope.
    ///
    /// Returnsthe indices for that name in the SVM environment.
    #[cfg_attr(feature = "unstable",
        stable(feature = "scope", since = "0.0.1") )]
    fn bind(&mut self, name: K, lvl: u64) -> Index;
    /// Look up a name against a scope.
    ///
    /// Returns the indices for that name in the SVM environment,
    /// or None if that name is unbound.
    #[cfg_attr(feature = "unstable",
        stable(feature = "scope", since = "0.0.1") )]
    fn lookup(&self, name: &K)            -> Option<Index>;
}
