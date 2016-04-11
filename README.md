# tommytree #

Currently, this README serves to describe the program, but also
to outline it's creation for my own benefit while writing it.


## Purpose ##

This program is mainly intended to get some practice using the Rust
programming language. I noticed in the documentation they used a
utility called tree that seems pretty neat but I don't have on
my computer. Rather than look for it and install it, I though "That
seems like something I could write myself!" So that's what I'm
gonna do.

The tree utility takes a directory structure and displays it in a
visually helpful way, showing the layout using lines to connect
parent to child, recursively.

## Example ##

Say you have two folders in your current working directory,
mydir1 and mydir2. Then, in mydir1 there is another directory
called subdir, and each directory as a file or two named after the
directory, with a letter suffixed. If you called tommytree, you
would see something like:

```
$ tommytree
mydir1
  |
  -- subdir
  |    |
  |    -- subdira
  |    
  -- mydir1a
  |
  -- mydir1b

mydir2
  |
  -- mydir2a
  |
  -- mydir2b
  |
  -- mydir2c

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

Instead of an API that interlocks well with ls, or find, or whatever,
write with an API as simple as possible, with extra "conversion layers"
or "adapter layers" (is there already a name for this?) that translates
(translation later?) the output of, say, ls, to produce input for
tommytree. I can then write convenience scripts that automatically
run ls | tl | tommytree. This way, tommytree can be extended to fit
all sorts of outputs by simply writing a new translation layer.

One problem with this approach is that a lot of the functionality suggested
above precludes this approach. I can at least start with the main program
having this simple design, and add functionality that works with other
utilities or possibly with it's own implementation of them. For example,
tommytree could query the directory structure itself, rather than using
ls or find to do the job - not that this is necessarily a great idea.

Better idea, build the full tommytree as a directory structure
visualizer, but keep the visualizer part separate as a reusable utility.
Do NOT use ls or anything as part of the primary functionality.
