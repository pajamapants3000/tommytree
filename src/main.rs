/*
 * File   : main.rs
 * Purpose: main executable
 * Program: tree
 * About  : prints directory structure in a visually clear tree-like diagram
 * Authors: Tommy Lincoln <pajamapants3000@gmail.com>
 * License: MIT; See LICENSE!
 * Created: 04/16/2016
 * Updated: 04/16/2016
 */

// Bring into namespace{{{
extern crate getopts;
use getopts::Options;
use std::env;

use std::io::{Error, Write, stdout, BufWriter};
use std::fs::DirEntry;
use std::path::{PathBuf, MAIN_SEPARATOR};

//}}}

fn main() {//{{{
    // Process comand-line arguments
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("d", "depth", "Depth of tree [-1]", "DEPTH");
    opts.optflag("h", "help", "Display usage");
    // Not implemented:
    opts.optopt("x", "exclude", "Exclude entries that match PATTERN []", "PATTERN");
    opts.optopt("s", "size", "Maximum directory size to expand [-1]", "MAXCHILD");
    opts.optflag("D", "dirs", "Only show directories");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("error: {} See usage (-h or --help).", f);
            return;
        },
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let depth = match matches.opt_str("d") {
        Some(x) => x,
        None => "-1".to_string(),
    };
    let depth: i8 = depth.parse().unwrap();     // can use overflow for max?

    let root: &str = match matches.free.len() {
        0 => ".",
        1 => &matches.free[0],
        _ => {
            println!("error: too many arguments!");
            for s in matches.free {
                println!("{}", s);
                println!("(pick just one)");
            }
            return;
        },
    };

    // Okay, we got all our details, let's begin the path-ing
    let pathroot = PathBuf::from(root);
    if !pathroot.is_dir() {
        println!("error: {} is not a directory", root);
        return;
    }
    let pathroot = pathroot.canonicalize()
        .expect("print_tree: unable to resolve path name");

    let prefix = &mut "".to_string();
    print_tree(pathroot, prefix, depth);

    println!("\nGoodbye!");

}
//}}}
fn print_usage(program: &str, opts: Options) {//{{{
    let brief = format!("{} ROOT [options]", program);
    println!("{}", opts.usage(&brief));
}
//}}}
pub fn print_tree(thisdir: PathBuf, prefix: &mut String, depth: i8) {//{{{

    let name = match thisdir.to_str() {
        Some(x) => x.rsplit(MAIN_SEPARATOR).next(),
        None => Some("YYY"),
    };
    let name = match name {
        Some(x) => x,
        None    => "XXX",
    };

    let stdout_handle = stdout();
    let mut stdout_buf = BufWriter::new(stdout_handle);

    let mut len: i32 = 0;
    for ch in name.chars() {
        write!(stdout_buf, "{}", ch).unwrap();
        len += 1;
    }

    if !thisdir.is_dir() { return; }

    let half = match len % 2 {
        0 => len / 2,
        1 => (len / 2) + 1,
        _ => panic!("The universe is unravelling!"),
    };

    let children = thisdir.read_dir()
        .expect(&format!("error while reading contents of {}", name));
    let mut children: Vec<Result<DirEntry, Error>> = children.collect();

    stdout_buf.flush().expect("error writing to stdout");

    // Return if we have exceeded max depth
    if depth == 0 || depth > (depth +1) {
        return;
    }

    // Alt: use Vec<Rc<&str>> and reuse prefix parts?
    let mut my_prefix = prefix.clone();
    for _ in 1..half {
        my_prefix.push(' ');
    }
    let mut child_prefix = my_prefix.clone();
    child_prefix.push_str("|  ");
    let mut emptydir: &mut bool = &mut true;
    loop {
        if let Some(result) = children.pop() {
            *emptydir = false;
            let child = result.expect(&format!("error reading contents of {}", name));
            write!(stdout_buf, "\n{0}|\n{0}", my_prefix).unwrap();

            if !children.is_empty() {
                write!(stdout_buf, "|- ").unwrap();    // right-facing 'T' character
            } else {
                write!(stdout_buf, "*- ").unwrap();    // bot-left-corner character
                let changelen = child_prefix.len() - 3;
                child_prefix.truncate(changelen);
                child_prefix.push_str("   ");
            }

            stdout_buf.flush().expect("error writing to stdout");
            print_tree(child.path(), &mut child_prefix, depth-1);
        } else {
            if *emptydir {
                write!(stdout_buf, "\n{0}|", my_prefix).unwrap();
            }
            break;
        }
    }

}//}}}

