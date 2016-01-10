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
//
//! # Location
//!
//! Code for annotating AST nodes with source code locations. This is based
//! loosely on ideas from code written by Markus Westerlind for the Embed
//! programming language
//! (https://github.com/Marwes/embed_lang/blob/master/base/src/ast.rs#L99).
use std::fmt;
use std::ops;

/// Represents a location within a source code file
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "unstable",
    unstable(feature = "location", issue = "4") )]
pub struct Location {
    #[cfg_attr(feature = "unstable",
        unstable(feature = "location", issue = "4") )]
    pub col: isize,
    #[cfg_attr(feature = "unstable",
        unstable(feature = "location", issue = "4") )]
    pub line: isize
}

/// Annotates a value with a `Location`
#[cfg_attr(feature = "unstable",
        unstable(feature = "location", issue = "4") )]
pub struct AtLocation<T> {
    #[cfg_attr(feature = "unstable",
        unstable(feature = "location", issue = "4") )]
    pub location: Location,
    #[cfg_attr(feature = "unstable",
        unstable(feature = "location", issue = "4") )]
    pub value: T
}

impl Location {
    /// Returns the absolute position in the file.
    #[cfg_attr(feature = "unstable",
        unstable(feature = "location", issue = "4") )]
    pub fn absolute(&self) -> isize { self.col + self.line }
}

impl fmt::Display for Location {
    #[cfg_attr(feature = "unstable",
        unstable(feature = "location", issue = "4") )]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.col)
    }
}

impl<T> fmt::Display for AtLocation<T> where T: fmt::Display {
    #[cfg_attr(feature = "unstable",
        unstable(feature = "location", issue = "4") )]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} at {}", self.value, self.location)
    }
}

impl<T> ops::Deref for AtLocation<T> {
    type Target = T;
    #[cfg_attr(feature = "unstable",
        unstable(feature = "location", issue = "4") )]
    fn deref(&self) -> &T {
        &self.value
    }
}
