use std::fmt;

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

impl fmt::Display for Location {
    #[cfg_attr(feature = "unstable",
        unstable(feature = "location") )]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.col)
    }
}

impl Location {
    /// Returns the absolute position in the file.
    #[cfg_attr(feature = "unstable",
        unstable(feature = "location") )]
    pub fn absolute(&self) -> isize { self.col + self.line }
}
