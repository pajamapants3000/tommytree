//
// File   : tree.rs
// Purpose: do the actual printing
// Program: tommytree
// Goal   : print out directory structure for easy visualization
// License: MIT; See LICENSE

// Bring into namespace
use std::io;//{{{
use std::io::stdout;
use std::io::BufWriter;

mod ancestry; // or use ancestry::* ? Are they the same?
//}}}

/// Recursive printer
pub fn print_recurs<T, P>(current: P, names_len: Vec<i32>, depth: i32,//{{{
                get_children: T where T: Fn<P>(P) -> Children<P>, P: Ancestry) {
    if depth == 0 || depth > (depth +1) {
        return;
    }
    // cd name
    // let name_bytes: &[u8] = name.bytes().collect(); -- need this?
    let mut stdout_buf = BufWriter::new(stdout());
    let mut len: i32 = 0;
    for ch as &[u8] in name.chars() { // this probably won't work; try it, then
        stdout_buf.write(ch);       // if not, drop the "as &[u8]" and just use
        len += 1;                   // this to get len, then write with .bytes()
    }
    names_len.push(len);

    let children = get_children(name);

    {
        stdout_buf.lock();
        stdout_buf.flush();
    }

    loop {
        if let Some(child) = children.pop() {

            stdout_buf().write(b"\n");

            for _len in &names_len[..names_len.len()] { // do -1 upper bound?
                for _ in [1.._len.abs().div(2)] {       // make fn?
                    stdout_buf().write(b" ");
                }
                if _len > 0 {
                    stdout_buf().write(b"|");
                } else {
                    stdout_buf().write(b" ");
                }
                stdout_buf().write(b"  ");
            }

            for _ in [1..len.abs().div(2)] {            // make fn?
                stdout_buf().write(b" ");
            }

            if children.len() == 0 {
                stdout_buf().write(b"-");
                names_len[..names_len.len()] = -names_len[..names_len.len()]
            } else {
                stdout_buf().write(b"-");
            }

            stdout_buf().write(b"- ");

            {
                stdout_buf.lock();
                stdout_buf.flush();
            }

            print_recurs(child, names_len, depth-1);

        } else { break; }                       // children.pop() returns None
    }
    // cd .. ?
}
//}}}
