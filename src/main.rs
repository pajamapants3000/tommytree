//
// File   : main.rs
// Program: tommytree
// Purpose: print out directory structure for easy visualization
// License: MIT; See LICENSE

use std::io

fn main() {

}

/// This is terrible! Use bufferring? How to we keep track of
/// our parent, and/or their identity?
fn print_recurs<T>(name: &str, names_len: Vec<i32>, depth: i32,
                get_children: T where T: Fn(&str) -> Vec<&str>) {
    if depth == 0 || depth > (depth +1) {
        return;
    }
    let name_bytes: &[u8] = name.bytes().collect();
    let len = stdout().write(name_bytes);
    let children = get_children(name);
    names_len.push(len);

    loop {
        match children.pop() {
            None        => break,
            Some(child) => {
                stdout().write(b"\n");
                for len in &names_len[..names_len.len()] {
                    let i: i32 = 0;
                    while len.abs().div(2) > i {    // make fn
                        stdout().write(b" ");
                        i += 1;
                    }
                    if len > 0 {
                        stdout().write(b"|");
                    } else {
                        stdout().write(b" ");
                    }
                    stdout().write(b"  ");
                }
                while len.abs().div(2) > (i - ) {    // make fn
                    stdout().write(b" ");
                    i += 1;
                }
                if children.len() == 0 {
                    stdout().write(b"-");
                    names_len[..names_len.len()] = -names_len[..names_len.len()]
                } else {
                    stdout().write(b"-");
                }
                stdout().write(b"- ");
                
                print_recurs(child, names_len, depth-1);
            }
        }
    }
}




}

