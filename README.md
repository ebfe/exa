exa
===

exa is a replacement for `ls` written in Rust.

[![Build status](https://travis-ci.org/ogham/exa.svg)](https://travis-ci.org/ogham/exa)

Screenshot
----------

![Screenshot of exa](https://raw.githubusercontent.com/ogham/exa/master/screenshot.png)

Options
-------

- **-a**, **--all**: show dot files
- **-b**, **--binary**: use binary (power of two) file sizes
- **-g**, **--group**: show group as well as user
- **-h**, **--header**: show a header row
- **-i**, **--inode**: show inode number column
- **-l**, **--links**: show number of hard links column
- **-r**, **--reverse**: reverse sort order
- **-s**, **--sort=(name, size, ext)**: field to sort by
- **-S**, **--blocks**: show number of file system blocks


Installation
------------

exa is written in [Rust](http://www.rust-lang.org). It compiles with Rust 0.11, the latest version - 0.10 will not do, as there have been too many breaking changes since. You will also need [Cargo](http://crates.io), the Rust package manager. Once you have them both set up, a simple `cargo build` will pull in all the dependencies and compile exa.
