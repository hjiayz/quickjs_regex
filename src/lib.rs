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
    code: Vec<u8>,
    count: usize,
}

impl Regex {
    pub fn complie(pattern: &str, flag: Flag) -> Result<Regex, String> {
        if flag.0 & UNICODE.0 == 0 {
            return Err("please set flag UNICODE".to_string());
        }
        //let cpattern = CString::new(pattern).map_err(|_|"create cstring failed".to_string())?;
        let mut byte_code_len = 0isize;
        const ERROR_MSG_SIZE: isize = 64;
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
            return Err(unsafe { CStr::from_ptr(error_msg.as_ptr()) }
                .to_string_lossy()
                .into_owned());
        }
        let byte_code_len = byte_code_len
            .try_into()
            .map_err(|_| "byte_code_len error".to_string())?;
        let code = unsafe { Vec::from_raw_parts(byte_code_ptr, byte_code_len, byte_code_len) };
        let count = unsafe { lre_get_capture_count(code.as_ptr() as *const _) as usize };
        Ok(Regex { code, count })
    }
    pub fn exec<'a>(&self, s: &'a str) -> Result<Option<Vec<&'a str>>, String> {
        let s_ptr = s.as_ptr();
        let mut cap = vec![0usize; 2 * self.count];
        let r = unsafe {
            lre_exec(
                cap.as_mut_ptr() as *mut *mut u8,
                self.code.as_ptr() as *const _,
                s_ptr as *const _,
                0,
                s.len() as isize,
                0,
                std::ptr::null_mut() as *mut _,
            )
        };
        match r {
            1 => (),
            0 => return Ok(None),
            _ => return Err("test exec failed".to_string()),
        };
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
        Ok(Some(matchgroups))
    }
}

#[test]
fn test_regex() {
    let reg = "(Α)123456".repeat(1);
    let text = "α123456".repeat(1000);
    let regex = Regex::complie(&reg, IGNORECASE | UNICODE).unwrap();
    let result = regex.exec(&text).unwrap().unwrap();
    assert!(result.len() == 2);

    let reg = "/(\0)123456".repeat(1);
    let text = "/\0123456".repeat(1000);
    let regex = Regex::complie(&reg, IGNORECASE | UNICODE).unwrap();
    let result = regex.exec(&text).unwrap().unwrap();
    assert!(result.len() == 2);
}
