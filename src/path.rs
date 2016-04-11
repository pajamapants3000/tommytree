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
use std::iter::Iterator;
//}}}
/// Our child iterator; uses ReadDir, which iterates over DirEntry types
struct Children(std::fs::ReadDir);

/// Modify ReadDir to return Child types
impl Iterator for Children {//{{{
    type Item = Child;

    fn next(&mut self) -> Option(Item) {
        if let Ok(dir_entry) = self.0.next() {
            dir_entry.path()
        } else {
            panic!("error obtaining contents of directory");
        }
    }
}
//}}}
/// Apply Ancestry to our Child type
impl Ancestry for Child {//{{{
    type C = Children;              // iterator over Child objects
    fn my_name(&self) -> &str {
        let name_str = self.to_str();
        let rev_comp = name_str.rsplit(MAIN_SEPARATOR);
        rev_comp.next()
    }
    fn get_children<C>(&self) -> C {
        if self.is_symlink() or !self.is_dir() {
            return Vec::<i8>::new().into_iter();
        }
        if let Ok(children) = Children(self.read_dir()) {
            children
        } else {
            panic!("error: failed to generate children iterator");
        }
    }
//}}}
    /// Helper for get_children: test if path is symlink
    fn is_symlink(&self) -> bool {//{{{
        if let Ok(md) = self.metadata() {
            md.file_type().is_symlink()
        } else { false }    // return false if any errors to stop processing
    }
//}}}
}


