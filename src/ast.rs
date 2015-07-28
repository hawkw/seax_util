use super::CompileResult;
use super::SymTable;

use std::fmt;

/// Trait for AST nodes.
#[stable(feature = "ast", since = "0.0.1")]
pub trait ASTNode {
    /// Compile this node to a list of SVM expressions
    #[unstable(feature="compile")]
    fn compile<'a>(&'a self,
                   state: &'a SymTable<'a>
                   )                    -> CompileResult;

    /// Pretty-print this node
    #[stable(feature = "ast", since = "0.0.1")]
    fn prettyprint(&self)               -> String { self.print_level(0) }

    /// Pretty-print this node at the desired indent level
    #[stable(feature = "ast", since = "0.0.1")]
    fn print_level(&self, level: usize) -> String;
}

#[stable(feature = "ast", since = "0.0.1")]
impl fmt::Debug for ASTNode {
    #[stable(feature = "ast", since = "0.0.1")]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.prettyprint())
    }
}
