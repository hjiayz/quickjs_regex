//! quickjs libregexp
//! # windows
//! install clang
//! ```sh
//! set CC=clang
//! ```

use std::borrow::Cow;
use std::convert::TryInto;
use std::ffi::c_void;
use std::ffi::CStr;
use std::os::raw::c_char;

extern "C" {
    fn lre_compile(
        byte_code_len: *mut isize,
        error_msg: *mut c_char,
        error_msg_size: isize,
        pattern: *const c_char,
        pattern_len: isize,
        flag: isize,
        ctx: *mut c_void,
    ) -> *mut u8;
    fn lre_exec(
        capture: *mut *mut u8,
        bc_buf: *const u8,
        cbuf: *const u8,
        cindex: isize,
        clen: isize,
        cbuf_type: isize,
        opaque: *mut c_void,
    ) -> isize;
    fn lre_get_capture_count(bc_buf: *const u8) -> isize;
}

#[derive(Copy, Clone)]
pub struct Flag(isize);

pub const GLOBAL: Flag = Flag(1 << 0);
pub const IGNORECASE: Flag = Flag(1 << 1);
pub const MULTILINE: Flag = Flag(1 << 2);
pub const DOTALL: Flag = Flag(1 << 3);
pub const UNICODE: Flag = Flag(1 << 4);
pub const STICKY: Flag = Flag(1 << 5);
pub const NAMED_GROUPS: Flag = Flag(1 << 6);

impl std::ops::BitOr for Flag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Debug)]
pub struct Regex {
    code: Cow<'static, [u8]>,
    count: usize,
}

#[derive(Debug)]
pub enum ComplieError {
    NotUnicode,
    ByteCodeLenIsNeg,
    ParseFail(String),
}

#[derive(Debug)]
pub enum ExecError {
    NotMatch,
    Error,
}

#[inline]
fn cap_to_str<'a>(_s: &'a str, cap: &[usize]) -> Vec<&'a str> {
    let mut iter = cap.iter();
    let mut matchgroups = Vec::new();
    while let Some(start) = iter.next() {
        let end = iter.next().unwrap();
        let slice = unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                *start as *mut u8,
                *end - *start,
            ))
        };
        matchgroups.push(slice);
    }
    matchgroups
}

impl Regex {
    pub fn compile(pattern: &str, flag: Flag) -> Result<Regex, ComplieError> {
        let mut byte_code_len = 0isize;
        const ERROR_MSG_SIZE: isize = 128;
        let mut error_msg = [0 as c_char; ERROR_MSG_SIZE as usize];
        let byte_code_ptr = unsafe {
            lre_compile(
                &mut byte_code_len as *mut isize,
                error_msg.as_mut_ptr(),
                ERROR_MSG_SIZE,
                pattern.as_ptr() as *const c_char,
                pattern.len() as isize,
                flag.0,
                std::ptr::null_mut(),
            )
        };
        if byte_code_ptr == std::ptr::null_mut() {
            return Err(ComplieError::ParseFail(
                unsafe { CStr::from_ptr(error_msg.as_ptr()) }
                    .to_string_lossy()
                    .into_owned(),
            ));
        }
        let byte_code_len = byte_code_len
            .try_into()
            .map_err(|_| ComplieError::ByteCodeLenIsNeg)?;
        let code: Cow<'static, [u8]> =
            Cow::Owned(unsafe { Vec::from_raw_parts(byte_code_ptr, byte_code_len, byte_code_len) });
        let count = unsafe { lre_get_capture_count(code.as_ptr() as *const _) as usize };
        Ok(Regex { code, count })
    }

    pub fn byte_code(&self) -> Vec<u8> {
        self.code.to_vec()
    }

    pub fn capture_count(&self) -> usize {
        self.count
    }

    pub const fn from_static(byte_code: &'static [u8], capture_count: usize) -> Regex {
        Regex {
            code: Cow::Borrowed(byte_code),
            count: capture_count,
        }
    }

    fn exec_inner(&self, s: &str, cindex: isize) -> Result<Vec<usize>, ExecError> {
        let s_ptr = s.as_ptr();
        let mut cap = vec![0usize; 2 * self.count];
        let r = unsafe {
            lre_exec(
                cap.as_mut_ptr() as *mut *mut u8,
                self.code.as_ptr() as *const _,
                s_ptr as *const _,
                cindex,
                s.len() as isize,
                0,
                std::ptr::null_mut() as *mut _,
            )
        };
        match r {
            1 => Ok(cap),
            0 => Err(ExecError::NotMatch),
            _ => Err(ExecError::Error),
        }
    }

    pub fn test(&self, s: &str) -> bool {
        self.exec_inner(s, 0).is_ok()
    }

    pub fn try_match<'a>(&self, s: &'a str) -> Result<Vec<&'a str>, ExecError> {
        self.exec_inner(s, 0).map(|cap| cap_to_str(s, &cap))
    }

    pub fn try_matchn<'a>(&self, s: &'a str, n: usize) -> Result<Vec<Vec<&'a str>>, ExecError> {
        let mut groups = Vec::new();
        let mut cindex = 0isize;
        for _ in 1..=n {
            let cap = match self.exec_inner(s, cindex) {
                Err(ExecError::NotMatch) => break,
                Err(ExecError::Error) => return Err(ExecError::Error),
                Ok(cap) => cap,
            };
            let matchgroups = cap_to_str(s, &cap);
            cindex = cap[1] as isize - (s.as_ptr() as isize);
            groups.push(matchgroups);
        }
        Ok(groups)
    }

    pub fn try_match_all<'a>(&self, s: &'a str) -> Result<Vec<Vec<&'a str>>, ExecError> {
        self.try_matchn(s, std::usize::MAX)
    }

    pub fn try_replace<'a, F>(&self, s: &'a str, f: F) -> Result<String, ExecError>
    where
        F: Fn(&[&'a str]) -> String,
    {
        self.try_replacen(s, f, 1)
    }

    pub fn try_replace_all<'a, F>(&self, s: &'a str, f: F) -> Result<String, ExecError>
    where
        F: Fn(&[&'a str]) -> String,
    {
        self.try_replacen(s, f, std::usize::MAX)
    }

    pub fn try_replacen<'a, F>(&self, s: &'a str, f: F, n: usize) -> Result<String, ExecError>
    where
        F: Fn(&[&'a str]) -> String,
    {
        let mut slice = Vec::new();
        let mut cindex = 0isize;
        for _ in 1..=n {
            let cap = match self.exec_inner(s, cindex) {
                Err(ExecError::NotMatch) => break,
                Err(ExecError::Error) => return Err(ExecError::Error),
                Ok(cap) => cap,
            };
            let start_ptr = unsafe { s.as_ptr().offset(cindex) };
            let start_len = cap[0] - (start_ptr as usize);
            let start = unsafe {
                std::str::from_utf8_unchecked(std::slice::from_raw_parts(start_ptr, start_len))
            };
            let matchgroups = cap_to_str(s, &cap);
            let rep = f(&matchgroups);
            slice.push(Cow::Borrowed(start));
            slice.push(Cow::Owned(rep));
            cindex = cap[1] as isize - (s.as_ptr() as isize);
        }

        let end_len = s.len() - (cindex as usize);
        let end_ptr = unsafe { s.as_ptr().offset(cindex) };
        let end =
            unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(end_ptr, end_len)) };
        slice.push(Cow::Borrowed(end));
        Ok(slice.join(""))
    }
}

