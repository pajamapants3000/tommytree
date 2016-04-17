# tree #

## Purpose ##

This program is mainly intended to get some practice using the Rust
programming language. I noticed in the documentation they used a
utility called tree that seems pretty neat but I don't have on
my computer. Rather than look for it and install it, I thought "That
seems like something I could write myself!" So that's what I'm
gonna do.

The tree utility takes a directory structure and displays it in a
visually helpful way, showing the layout using lines to connect
parent to child, recursively.

## Example ##

Say you have two folders in your current working directory,
mydir1 and mydir2. Then, in mydir1 there is another directory
called subdir, and each directory as a file or two named after the
directory, with a letter suffixed. If you called tree with no
arguments, you would see something like:

```
$ tree
mydir1
  |
  |- subdir
  |    |
  |    -- subdira
  |    
  |- subdir1a
  |
  -- subdir1b

mydir2
  |
  |- subdir2a
  |
  |- subdir2b
  |
  -- subdir2c

```

## Features ##

Calling the utility with no arguments should produce an output like this;
however, what if the output were very large? Should there be defaults as
far as how the output is displayed for very large directory structures?
Should it use a pager? Perhaps that is another program I could write - a
pager. But anyway... There's that.

What other features should be present?

+ Specify maximum depth
+ Specify root directory
+ Collapse directories larger than specified size
+ Show directories only, no files
+ Show only certain filetypes
+ Sort by name, time, directories first/last, etc.
+ ...?

## Design ##

I originally wanted this to work for any nested structure, but
I found this rather difficult to represent in Rust. I did manage to do
this for Vec<&str> specifically, but a general T: Iterator eludes (and
frustrates) me. It may be possible, but not with my current understanding
of Rust. But this still gives me plenty to play around with even just
applying to filesystem directory structure.

