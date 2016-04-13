//
// File   : path.rs
// Purpose: implement ancestry for (filesystem) Path type
// Program: tommytree
// Goal   : print out directory structure for easy visualization
// License: MIT; See LICENSE

// Bring in to namespace
use std::path::PathBuf as Child;//{{{
use std::path::MAIN_SEPARATOR;
use std::fs::ReadDir;
use std::iter::{Iterator, empty};

use ancestry::Ancestry;
//}}}
/// Our child iterator; uses ReadDir, which iterates over DirEntry types
//struct Children(std::fs::ReadDir);
type Children = ReadDir;

/// Modify ReadDir to return Child types
impl Iterator for Children {//{{{
    type Item = Child;
    fn next(&mut self) -> Option<Child> {
        self.next().map(|dirent| dirent.path())
    }
}
//}}}
/// Apply Ancestry to our Child type (PathBuf)
impl Ancestry for Child {//{{{
    type C = Children;              // iterator over Child objects

    fn my_name(&self) -> &str {
        self.to_str().rsplit(MAIN_SEPARATOR).next()
    }

    fn my_children<C>(&self) -> C {
        self.read_dir().unwrap_or(empty())
    }

//}}}
}

//{{{
/* lines removed from get_children: I don't think I need any of this,
 * but I'm saving it for now
if self.is_symlink() or !self.is_dir() {
    return std::iter::empty();    // return empty iterator
}
/// Helper for get_children: test if path is symlink
#[allow(dead_code)]
fn is_symlink(s: PathBuf) -> bool {//{{{
    if let Ok(md) = s.metadata() {
        md.file_type().is_symlink()
    } else { true }     // return true if any errors to stop processing;
}                       // tree will post name and not attempt to process
//}}}                   // children
*/

