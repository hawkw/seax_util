use std::fmt;
use std::ops;

/// Represents a location within a source code file
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "unstable",
    unstable(feature = "location") )]
pub struct Location {
    #[cfg_attr(feature = "unstable",
        unstable(feature = "location") )]
    pub col: isize,
    #[cfg_attr(feature = "unstable",
        unstable(feature = "location") )]
    pub line: isize
}

/// Annotates a value with a `Location`
#[cfg_attr(feature = "unstable",
    unstable(feature = "location") )]
pub struct AtLocation<T> {
    #[cfg_attr(feature = "unstable",
        unstable(feature = "location") )]
    pub location: Location,
    #[cfg_attr(feature = "unstable",
        unstable(feature = "location") )]
    pub value: T
}

impl Location {
    /// Returns the absolute position in the file.
    #[cfg_attr(feature = "unstable",
        unstable(feature = "location") )]
    pub fn absolute(&self) -> isize { self.col + self.line }
}

impl fmt::Display for Location {
    #[cfg_attr(feature = "unstable",
        unstable(feature = "location") )]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.col)
    }
}

impl<T> fmt::Display for AtLocation<T> where T: fmt::Display {
    #[cfg_attr(feature = "unstable",
        unstable(feature = "location") )]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} at {}", self.value, self.location)
    }
}

impl<T> ops::Deref for AtLocation<T> {
    type Target = T;
    #[cfg_attr(feature = "unstable",
        unstable(feature = "location") )]
    fn deref(&self) -> &T {
        &self.value
    }
}
