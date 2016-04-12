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

/// Recursive printer - Starting to look a lot better!
pub fn print_recurs<T, P>(current: P, prefix: &mut String, depth: i32,//{{{
                get_children: T where T: Fn<P>(P) -> Children<P>, P: Ancestry) {

    // Return if we have exceeded max depth
    if depth == 0 || depth > (depth +1) {
        return;
    }

    let mut stdout_buf = BufWriter::new(stdout());
    let name = current.get_name();
    let mut len: i32 = 0;

    for ch in name.chars() {
        write!(stdout_buf, "{}", ch);
        len += 1;
    }

    for _ in [0..len.div(2)] {
        prefix.append(' ');
    }
    if len.div(2) == (len - 1).div(2) {     // add space for odd lengths
        prefix.append(' ');
    }

    let children = get_children(current);

    {   // flush before entering loop
        stdout_buf.lock();
        stdout_buf.flush();
    }

    if let children.pop() = Some(child) {

        write!(stdout_buf, "\n{}|\n{}", prefix);

        let child_prefix = prefix.clone();

        if children.len() == 0 {        // currently on the last child
            write!(stdout_buf, "-");    // bot-right corner character
            child_prefix.std_append("   ");
        } else {
            write!(stdout_buf, "-");    // right-facing 'T' character
            child_prefix.std_append("|  ");
        }

        {
            stdout_buf.lock();
            stdout_buf.flush();
        }

        print_recurs(child, child_prefix, depth-1, get_children);
    }
}
//}}}
