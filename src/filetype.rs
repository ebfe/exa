use file::File;
use std::io;

use ansi_term::{Paint, Colour, Plain, Style, Red, Green, Yellow, Blue, Cyan, Fixed};

static Grey: Colour = Fixed(244);

pub enum FileType {
    Normal, Directory, Executable, Immediate, Compiled, Symlink, Special,
    Image, Video, Music, Lossless, Compressed, Document, Temp, Crypto,
}

static IMAGE_TYPES: &'static [&'static str] = &[
    "png", "jpeg", "jpg", "gif", "bmp", "tiff", "tif",
    "ppm", "pgm", "pbm", "pnm", "webp", "raw", "arw",
    "svg", "stl", "eps", "dvi", "ps", "cbr",
    "cbz", "xpm", "ico" ];

static VIDEO_TYPES: &'static [&'static str] = &[
    "avi", "flv", "m2v", "mkv", "mov", "mp4", "mpeg",
    "mpg", "ogm", "ogv", "vob", "wmv" ];

static MUSIC_TYPES: &'static [&'static str] = &[
    "aac", "m4a", "mp3", "ogg" ];

static MUSIC_LOSSLESS: &'static [&'static str] = &[
    "alac", "ape", "flac", "wav" ];

static COMPRESSED_TYPES: &'static [&'static str] = &[
    "zip", "tar", "Z", "gz", "bz2", "a", "ar", "7z",
    "iso", "dmg", "tc", "rar", "par" ];

static DOCUMENT_TYPES: &'static [&'static str] = &[
    "djvu", "doc", "docx", "eml", "eps", "odp", "ods",
    "odt", "pdf", "ppt", "pptx", "xls", "xlsx" ];

static TEMP_TYPES: &'static [&'static str] = &[
    "tmp", "swp", "swo", "swn", "bak" ];

static CRYPTO_TYPES: &'static [&'static str] = &[
    "asc", "gpg", "sig", "signature", "pgp" ];

static COMPILED_TYPES: &'static [&'static str] = &[
    "class", "elc", "hi", "o", "pyc" ];
    
static BUILD_TYPES: &'static [&'static str] = &[
    "Makefile", "Cargo.toml", "SConstruct", "CMakeLists.txt",
    "build.gradle", "Rakefile", "Gruntfile.js",
    "Gruntfile.coffee" ];

impl FileType {
    pub fn style(&self) -> Style {
        match *self {
            Normal => Plain,
            Directory => Blue.bold(),
            Symlink => Cyan.normal(),
            Special => Yellow.normal(),
            Executable => Green.bold(),
            Image => Fixed(133).normal(),
            Video => Fixed(135).normal(),
            Music => Fixed(92).normal(),
            Lossless => Fixed(93).normal(),
            Crypto => Fixed(109).normal(),
            Document => Fixed(105).normal(),
            Compressed => Red.normal(),
            Temp => Grey.normal(),
            Immediate => Yellow.bold().underline(),
            Compiled => Fixed(137).normal(),
        }
    }
}

pub trait HasType {
    fn get_type(&self) -> FileType;
}

impl<'a> HasType for File<'a> {
    fn get_type(&self) -> FileType {
        let name = self.name.as_slice();
        if self.stat.kind == io::TypeDirectory {
            return Directory;
        }
        else if self.stat.kind == io::TypeSymlink {
            return Symlink;
        }
        else if self.stat.kind == io::TypeBlockSpecial || self.stat.kind == io::TypeNamedPipe || self.stat.kind == io::TypeUnknown {
            return Special;
        }
        else if self.stat.perm.contains(io::UserExecute) {
            return Executable;
        }
        else if name.starts_with("README") || BUILD_TYPES.iter().any(|&s| s == name) {
            return Immediate;
        }
        else if self.ext.is_some() {
            let e = self.ext.clone().unwrap();
            let ext = e.as_slice();
            if IMAGE_TYPES.iter().any(|&s| s == ext) {
                return Image;
            }
            else if VIDEO_TYPES.iter().any(|&s| s == ext) {
                return Video;
            }
            else if MUSIC_TYPES.iter().any(|&s| s == ext) {
                return Music;
            }
            else if MUSIC_LOSSLESS.iter().any(|&s| s == ext) {
                return Lossless;
            }
            else if CRYPTO_TYPES.iter().any(|&s| s == ext) {
                return Crypto;
            }
            else if DOCUMENT_TYPES.iter().any(|&s| s == ext) {
                return Document;
            }
            else if COMPRESSED_TYPES.iter().any(|&s| s == ext) {
                return Compressed;
            }
            else if self.is_tmpfile() || TEMP_TYPES.iter().any(|&s| s == ext) {
                return Temp;
            }
            
            let source_files = self.get_source_files();
            if source_files.len() == 0 {
                return Normal;
            }
            else if source_files.iter().any(|path| self.dir.contains(path)) {
                return Temp;
            }
            else {
                if COMPILED_TYPES.iter().any(|&s| s == ext) {
                    return Compiled;
                }
                else {
                    return Normal;
                }
            }
        }
        return Normal;  // no filetype
    }
}
