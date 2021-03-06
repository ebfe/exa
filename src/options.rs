extern crate getopts;

use file::File;
use std::cmp::lexical_ordering;
use column::{Column, Permissions, FileName, FileSize, User, Group, HardLinks, Inode, Blocks};
use std::ascii::StrAsciiExt;

pub enum SortField {
    Name, Extension, Size
}

pub struct Options {
    pub showInvisibles: bool,
    pub sortField: SortField,
    pub reverse: bool,
    pub dirs: Vec<String>,
    pub columns: Vec<Column>,
    pub header: bool,
}

impl SortField {
    fn from_word(word: String) -> SortField {
        match word.as_slice() {
            "name" => Name,
            "size" => Size,
            "ext"  => Extension,
            _      => fail!("Invalid sorting order"),
        }
    }
}

impl Options {
    pub fn getopts(args: Vec<String>) -> Result<Options, getopts::Fail_> {
        let opts = [
            getopts::optflag("a", "all", "show dot-files"),
            getopts::optflag("b", "binary", "use binary prefixes in file sizes"),
            getopts::optflag("g", "group", "show group as well as user"),
            getopts::optflag("h", "header", "show a header row at the top"),
            getopts::optflag("i", "inode", "show each file's inode number"),
            getopts::optflag("l", "links", "show number of hard links"),
            getopts::optflag("r", "reverse", "reverse order of files"),
            getopts::optopt("s", "sort", "field to sort by", "WORD"),
            getopts::optflag("S", "blocks", "show number of file system blocks"),
        ];

        match getopts::getopts(args.tail(), opts) {
            Err(f) => Err(f),
            Ok(matches) => Ok(Options {
                showInvisibles: matches.opt_present("all"),
                reverse: matches.opt_present("reverse"),
                header: matches.opt_present("header"),
                sortField: matches.opt_str("sort").map(|word| SortField::from_word(word)).unwrap_or(Name),
                dirs: if matches.free.is_empty() { vec![ ".".to_string() ] } else { matches.free.clone() },
                columns: Options::columns(matches),
            })
        }
    }

    fn columns(matches: getopts::Matches) -> Vec<Column> {
        let mut columns = vec![];

        if matches.opt_present("inode") {
            columns.push(Inode);
        }

        columns.push(Permissions);

        if matches.opt_present("links") {
            columns.push(HardLinks);
        }
        
        columns.push(FileSize(matches.opt_present("binary")));

        if matches.opt_present("blocks") {
            columns.push(Blocks);
        }

        columns.push(User);

        if matches.opt_present("group") {
            columns.push(Group);
        }

        columns.push(FileName);

        return columns;
    }

    fn should_display(&self, f: &File) -> bool {
        if self.showInvisibles {
            true
        } else {
            !f.name.as_slice().starts_with(".")
        }
    }

    pub fn transform_files<'a>(&self, unordered_files: &'a Vec<File<'a>>) -> Vec<&'a File<'a>> {
        let mut files: Vec<&'a File<'a>> = unordered_files.iter()
            .filter(|&f| self.should_display(f))
            .collect();

        match self.sortField {
            Name => files.sort_by(|a, b| a.parts.cmp(&b.parts)),
            Size => files.sort_by(|a, b| a.stat.size.cmp(&b.stat.size)),
            Extension => files.sort_by(|a, b| {
                let exts = a.ext.clone().map(|e| e.as_slice().to_ascii_lower()).cmp(&b.ext.clone().map(|e| e.as_slice().to_ascii_lower()));
                let names = a.name.as_slice().to_ascii_lower().cmp(&b.name.as_slice().to_ascii_lower());
                lexical_ordering(exts, names)
            }),
        }

        if self.reverse {
            files.reverse();
        }

        return files;
    }
}
