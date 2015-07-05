/*!
Miscellaneous utility code.
*/
use std::ffi::CString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub trait BoolUtil {
    fn as_either<T>(&self, if_true: T, if_false: T) -> T;
}

impl BoolUtil for bool {
    fn as_either<T>(&self, if_true: T, if_false: T) -> T {
        if *self { if_true } else { if_false }
    }
}

pub trait CheckedFrom<T> {
    fn checked_from(T) -> Self;
}

impl CheckedFrom<usize> for u32 {
    fn checked_from(v: usize) -> u32 {
        if ! (::std::u32::MIN as usize <= v) {
            panic!("underflow on conversion from usize {} to u32", v);
        } else if ! (v <= ::std::u32::MAX as usize) {
            panic!("overflow on conversion from usize {} to u32", v);
        } else {
            v as u32
        }
    }
}

impl CheckedFrom<usize> for i32 {
    fn checked_from(v: usize) -> i32 {
        if ! (v <= ::std::i32::MAX as usize) {
            panic!("overflow on conversion from usize {} to i32", v);
        } else {
            v as i32
        }
    }
}

pub trait CheckedInto<T> {
    fn checked_into(self) -> T;
}

impl<T, U> CheckedInto<U> for T where U: CheckedFrom<T> {
    fn checked_into(self) -> U {
        CheckedFrom::checked_from(self)
    }
}

pub trait PathBufExt {
    fn normalize_path_sep(&mut self);
}

impl PathBufExt for PathBuf {
    fn normalize_path_sep(&mut self) {
        // TODO: also resolve to some sort of absolute path.
        let s = ::std::mem::replace(self, PathBuf::new())
            .into_os_string()
            .into_string().unwrap_or_else(|oss| oss.to_string_lossy().into_owned());
        let mut bs = s.into_bytes();
        for b in &mut bs {
            if *b == '/' as u8 { *b = '\\' as u8 }
        }

        // Back we go!
        let p = String::from_utf8(bs).unwrap().into();
        *self = p;
    }
}

/**
This is because I am too lazy to write my own damn monad type.

This makes a `Result<Option<T>, E>` behave exactly like `Option<T>`, except that errors will always short-circuit.
*/
pub trait ResultOptionExt<T, E> {
    fn ro_or_else<F>(self, f: F) -> Result<Option<T>, E>
    where F: FnOnce() -> Result<Option<T>, E>;

    fn ro_and_then<T1, F>(self, f: F) -> Result<Option<T1>, E>
    where F: FnOnce(T) -> Result<Option<T1>, E>;

    fn ro_map<T1, F>(self, f: F) -> Result<Option<T1>, E>
    where F: FnOnce(T) -> T1;
}

impl<T, E> ResultOptionExt<T, E> for Result<Option<T>, E> {
    fn ro_or_else<F>(self, f: F) -> Result<Option<T>, E>
    where F: FnOnce() -> Result<Option<T>, E> {
        match self {
            Ok(Some(v)) => Ok(Some(v)),
            Ok(None) => f(),
            Err(err) => Err(err)
        }
    }

    fn ro_and_then<T1, F>(self, f: F) -> Result<Option<T1>, E>
    where F: FnOnce(T) -> Result<Option<T1>, E> {
        match self {
            Ok(Some(v)) => f(v),
            Ok(None) => Ok(None),
            Err(err) => Err(err)
        }
    }

    fn ro_map<T1, F>(self, f: F) -> Result<Option<T1>, E>
    where F: FnOnce(T) -> T1 {
        match self {
            Ok(Some(v)) => Ok(Some(f(v))),
            Ok(None) => Ok(None),
            Err(err) => Err(err)
        }
    }
}

pub trait ToCStr {
    fn to_c_str(&self) -> CString;
}

impl<'a> ToCStr for &'a str {
    fn to_c_str(&self) -> CString {
        CString::new(*self).unwrap()
    }
}

impl ToCStr for String {
    fn to_c_str(&self) -> CString {
        CString::new(self.clone()).unwrap()
    }
}

pub trait TryFrom<T> {
    fn try_from(T) -> Option<Self>;
}

pub trait TryInto<U> {
    fn try_into(self) -> Option<U>;
}

impl<T, U> TryInto<U> for T where U: TryFrom<T> {
    fn try_into(self) -> Option<U> {
        TryFrom::try_from(self)
    }
}

pub fn read_lines<P: AsRef<Path>>(path: P) -> io::Lines<io::BufReader<fs::File>> {
    use std::io::BufRead;
    io::BufReader::new(fs::File::open(path).unwrap()).lines()
}
