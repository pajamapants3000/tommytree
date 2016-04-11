//
// File   : ancestry.rs
// Purpose: obtain child elements from parent
// Program: tommytree
// Goal   : print out directory structure for easy visualization
// License: MIT; See LICENSE
// TODO: Try to add more generic code that can be reused (ideally this
//  file is larger than tree.rs

/// Get family names
pub trait Ancestry {
    type C: std::iter::Iterator;

    fn my_name(&self) -> &str;
    fn my_children<C>(&self) -> C;
}

