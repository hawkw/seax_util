//  Seax
//  Copyright 2016 Hawk Weisman.
//
//  Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//  http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//  <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//  option. This file may not be copied, modified, or distributed
//  except according to those terms.
//  
//! # AST
//!
//! Contains a trait definition for an abstract syntax tree (AST) node.
//! Compilers targeting Seax should provide concrete implementations of this
//! trait for their AST node types. This ensures a consistant interface for
//! Seax compilers.

use super::CompileResult;
use super::SymTable;

use std::fmt;

/// Width of one indentation level
#[cfg_attr(feature = "unstable",
    stable(feature = "ast", since = "0.0.2") )]
pub const INDENT: &'static str = "\t";

/// Trait for AST nodes.
#[cfg_attr(feature = "unstable",
    stable(feature = "ast", since = "0.0.1") )]
pub trait ASTNode {
    /// Compile this node to a list of SVM expressions
    #[cfg_attr(feature = "unstable",
        stable(feature = "compile", since = "0.0.1") )]
    fn compile<'a>(&'a self,
                   state: &'a SymTable<'a>)
                   -> CompileResult;

    /// Pretty-print this node to a String.
    ///
    /// This should start with this node indented zero spaces, and recursively
    /// walk the tree downward, increasing the indentation level by `INDENT`
    /// every step.
    #[cfg_attr(feature = "unstable",
        stable(feature = "ast", since = "0.0.1") )]
    fn prettyprint(&self) -> String {
        self.print_level(0)
    }

    /// Pretty-print this node at the desired indent level
    #[cfg_attr(feature = "unstable",
        stable(feature = "ast", since = "0.0.1") )]
    fn print_level(&self, level: usize) -> String;
}

#[cfg_attr(feature = "unstable",
    stable(feature = "ast", since = "0.0.1") )]
impl fmt::Debug for ASTNode {
    #[cfg_attr(feature = "unstable",
        stable(feature = "ast", since = "0.0.1") )]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.prettyprint())
    }
}
