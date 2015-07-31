use super::CompileResult;
use super::SymTable;

use std::fmt;

/// Width of one indentation level
#[cfg_attr(feature = "nightly",
    stable(feature = "ast", since = "0.0.2")
)]
pub const INDENT: &'static str = "\t";

/// Trait for AST nodes.
#[cfg_attr(feature = "nightly",
    stable(feature = "ast", since = "0.0.1")
)]
pub trait ASTNode {
    /// Compile this node to a list of SVM expressions
    #[cfg_attr(feature = "nightly",
        stable(feature = "compile", since = "0.0.1")
    )]
    fn compile<'a>(&'a self,
                   state: &'a SymTable<'a>
                   )                    -> CompileResult;

    /// Pretty-print this node to a String.
    ///
    /// This should start with this node indented zero spaces, and recursively
    /// walk the tree downward, increasing the indentation level by `INDENT`
    /// every step.
    #[cfg_attr(feature = "nightly",
        stable(feature = "ast", since = "0.0.1")
    )]
    fn prettyprint(&self)               -> String { self.print_level(0) }

    /// Pretty-print this node at the desired indent level
    #[cfg_attr(feature = "nightly",
        stable(feature = "ast", since = "0.0.1")
    )]
    fn print_level(&self, level: usize) -> String;
}

#[cfg_attr(feature = "nightly",
    stable(feature = "ast", since = "0.0.1")
)]
impl fmt::Debug for ASTNode {
    #[cfg_attr(feature = "nightly",
        stable(feature = "ast", since = "0.0.1")
    )]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.prettyprint())
    }
}
