use ::libc;
extern "C" {
    fn buffer_string_prepare_copy(b: *mut buffer, size: size_t) -> *mut libc::c_char;
    fn buffer_string_prepare_append(b: *mut buffer, size: size_t) -> *mut libc::c_char;
    fn buffer_commit(b: *mut buffer, size: size_t);
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_path_simplify(b: *mut buffer);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn buffer_append_base64_decode(
        out: *mut buffer,
        in_0: *const libc::c_char,
        in_length: size_t,
        charset: base64_charset,
    ) -> *mut libc::c_uchar;
    fn buffer_append_base64_enc(
        out: *mut buffer,
        in_0: *const libc::c_uchar,
        in_length: size_t,
        charset: base64_charset,
        pad: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub ptr: *mut libc::c_char,
    pub used: uint32_t,
    pub size: uint32_t,
}
pub type burl_opts_e = libc::c_uint;
pub const HTTP_PARSEOPT_METHOD_GET_BODY: burl_opts_e = 32768;
pub const HTTP_PARSEOPT_URL_NORMALIZE_QUERY_20_PLUS: burl_opts_e = 4096;
pub const HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REJECT: burl_opts_e = 2048;
pub const HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REMOVE: burl_opts_e = 1024;
pub const HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_REJECT: burl_opts_e = 512;
pub const HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_DECODE: burl_opts_e = 256;
pub const HTTP_PARSEOPT_URL_NORMALIZE_PATH_BACKSLASH_TRANS: burl_opts_e = 128;
pub const HTTP_PARSEOPT_URL_NORMALIZE_CTRLS_REJECT: burl_opts_e = 64;
pub const HTTP_PARSEOPT_URL_NORMALIZE_REQUIRED: burl_opts_e = 32;
pub const HTTP_PARSEOPT_URL_NORMALIZE_UNRESERVED: burl_opts_e = 16;
pub const HTTP_PARSEOPT_URL_NORMALIZE: burl_opts_e = 8;
pub const HTTP_PARSEOPT_HOST_NORMALIZE: burl_opts_e = 4;
pub const HTTP_PARSEOPT_HOST_STRICT: burl_opts_e = 2;
pub const HTTP_PARSEOPT_HEADER_STRICT: burl_opts_e = 1;
pub type burl_recoding_e = libc::c_uint;
pub const BURL_DECODE_B64U: burl_recoding_e = 128;
pub const BURL_ENCODE_B64U: burl_recoding_e = 64;
pub const BURL_ENCODE_PSNDE: burl_recoding_e = 32;
pub const BURL_ENCODE_NDE: burl_recoding_e = 16;
pub const BURL_ENCODE_ALL: burl_recoding_e = 8;
pub const BURL_ENCODE_NONE: burl_recoding_e = 4;
pub const BURL_TOUPPER: burl_recoding_e = 2;
pub const BURL_TOLOWER: burl_recoding_e = 1;
pub type base64_charset = libc::c_uint;
pub const BASE64_URL: base64_charset = 1;
pub const BASE64_STANDARD: base64_charset = 0;
#[inline]
unsafe extern "C" fn light_isdigit(mut c: libc::c_int) -> libc::c_int {
    return ((c as uint32_t).wrapping_sub('0' as i32 as libc::c_uint)
        <= ('9' as i32 - '0' as i32) as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn light_isxdigit(mut c: libc::c_int) -> libc::c_int {
    return (light_isdigit(c) != 0
        || (c as uint32_t | 0x20 as libc::c_int as libc::c_uint)
            .wrapping_sub('a' as i32 as libc::c_uint)
            <= ('f' as i32 - 'a' as i32) as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn light_isalpha(mut c: libc::c_int) -> libc::c_int {
    return ((c as uint32_t | 0x20 as libc::c_int as libc::c_uint)
        .wrapping_sub('a' as i32 as libc::c_uint)
        <= ('z' as i32 - 'a' as i32) as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn light_isalnum(mut c: libc::c_int) -> libc::c_int {
    return (light_isdigit(c) != 0 || light_isalpha(c) != 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn buffer_clen(mut b: *const buffer) -> uint32_t {
    return ((*b).used)
        .wrapping_sub(
            (0 as libc::c_int as libc::c_uint != (*b).used) as libc::c_int
                as libc::c_uint,
        );
}
#[inline]
unsafe extern "C" fn buffer_truncate(mut b: *mut buffer, mut len: uint32_t) {
    *((*b).ptr).offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    (*b).used = len.wrapping_add(1 as libc::c_int as libc::c_uint);
}
static mut hex_chars_uc: [libc::c_char; 17] = unsafe {
    *::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"0123456789ABCDEF\0")
};
static mut encoded_chars_http_uri_reqd: [libc::c_char; 256] = [
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
];
unsafe extern "C" fn burl_is_unreserved(c: libc::c_int) -> libc::c_int {
    return (light_isalnum(c) != 0 || c == '-' as i32 || c == '.' as i32
        || c == '_' as i32 || c == '~' as i32) as libc::c_int;
}
unsafe extern "C" fn burl_normalize_basic_unreserved_fix(
    mut b: *mut buffer,
    mut t: *mut buffer,
    mut i: libc::c_int,
    mut qs: libc::c_int,
) -> libc::c_int {
    let mut j: libc::c_int = i;
    let used: libc::c_int = buffer_clen(b) as libc::c_int;
    let s: *const libc::c_uchar = (*b).ptr as *mut libc::c_uchar;
    let p: *mut libc::c_uchar = buffer_string_prepare_copy(
        t,
        (i + (used - i) * 3 as libc::c_int + 1 as libc::c_int) as size_t,
    ) as *mut libc::c_uchar;
    let mut n1: libc::c_uint = 0;
    let mut n2: libc::c_uint = 0;
    memcpy(p as *mut libc::c_void, s as *const libc::c_void, i as size_t);
    while i < used {
        if encoded_chars_http_uri_reqd[*s.offset(i as isize) as usize] == 0 {
            if *s.offset(i as isize) as libc::c_int == '?' as i32
                && -(1 as libc::c_int) == qs
            {
                qs = j;
            }
            *p.offset(j as isize) = *s.offset(i as isize);
        } else if *s.offset(i as isize) as libc::c_int == '%' as i32
                && {
                    n1 = (*s.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                        - '0' as i32) as libc::c_uint;
                    n1 <= 9 as libc::c_int as libc::c_uint
                        || {
                            n1 = ((*s.offset((i + 1 as libc::c_int) as isize)
                                as libc::c_int & 0xdf as libc::c_int) - 'A' as i32)
                                as libc::c_uint;
                            (if n1 <= 5 as libc::c_int as libc::c_uint {
                                n1 = n1.wrapping_add(10 as libc::c_int as libc::c_uint);
                                n1
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) != 0
                        }
                }
                && {
                    n2 = (*s.offset((i + 2 as libc::c_int) as isize) as libc::c_int
                        - '0' as i32) as libc::c_uint;
                    n2 <= 9 as libc::c_int as libc::c_uint
                        || {
                            n2 = ((*s.offset((i + 2 as libc::c_int) as isize)
                                as libc::c_int & 0xdf as libc::c_int) - 'A' as i32)
                                as libc::c_uint;
                            (if n2 <= 5 as libc::c_int as libc::c_uint {
                                n2 = n2.wrapping_add(10 as libc::c_int as libc::c_uint);
                                n2
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) != 0
                        }
                }
            {
            let x: libc::c_uint = n1 << 4 as libc::c_int | n2;
            if burl_is_unreserved(x as libc::c_int) != 0 {
                *p.offset(j as isize) = x as libc::c_uchar;
            } else {
                *p.offset(j as isize) = '%' as i32 as libc::c_uchar;
                j += 1;
                *p.offset(j as isize) = hex_chars_uc[n1 as usize] as libc::c_uchar;
                j += 1;
                *p.offset(j as isize) = hex_chars_uc[n2 as usize] as libc::c_uchar;
                if x >= 0xf5 as libc::c_int as libc::c_uint
                    || x | 0x1 as libc::c_int as libc::c_uint
                        == 0xc1 as libc::c_int as libc::c_uint
                {
                    qs = -(2 as libc::c_int);
                }
            }
            i += 2 as libc::c_int;
        } else {
            if *s.offset(i as isize) as libc::c_int == '#' as i32 {
                break;
            }
            *p.offset(j as isize) = '%' as i32 as libc::c_uchar;
            j += 1;
            *p
                .offset(
                    j as isize,
                ) = hex_chars_uc[(*s.offset(i as isize) as libc::c_int
                >> 4 as libc::c_int & 0xf as libc::c_int) as usize] as libc::c_uchar;
            j += 1;
            *p
                .offset(
                    j as isize,
                ) = hex_chars_uc[(*s.offset(i as isize) as libc::c_int
                & 0xf as libc::c_int) as usize] as libc::c_uchar;
            if *s.offset(i as isize) as libc::c_int >= 0xf5 as libc::c_int
                || *s.offset(i as isize) as libc::c_int | 0x1 as libc::c_int
                    == 0xc1 as libc::c_int
            {
                qs = -(2 as libc::c_int);
            }
        }
        i += 1;
        j += 1;
    }
    buffer_copy_string_len(b, p as *mut libc::c_char, j as size_t);
    return qs;
}
unsafe extern "C" fn burl_normalize_basic_unreserved(
    mut b: *mut buffer,
    mut t: *mut buffer,
) -> libc::c_int {
    let s: *const libc::c_uchar = (*b).ptr as *mut libc::c_uchar;
    let used: libc::c_int = buffer_clen(b) as libc::c_int;
    let mut n1: libc::c_uint = 0;
    let mut n2: libc::c_uint = 0;
    let mut x: libc::c_uint = 0;
    let mut qs: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < used {
        if encoded_chars_http_uri_reqd[*s.offset(i as isize) as usize] == 0 {
            if *s.offset(i as isize) as libc::c_int == '?' as i32
                && -(1 as libc::c_int) == qs
            {
                qs = i;
            }
        } else if *s.offset(i as isize) as libc::c_int == '%' as i32
                && {
                    n1 = (*s.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                        - '0' as i32) as libc::c_uint;
                    n1 <= 9 as libc::c_int as libc::c_uint
                        || {
                            n1 = ((*s.offset((i + 1 as libc::c_int) as isize)
                                as libc::c_int & 0xdf as libc::c_int) - 'A' as i32)
                                as libc::c_uint;
                            (if n1 <= 5 as libc::c_int as libc::c_uint {
                                n1 = n1.wrapping_add(10 as libc::c_int as libc::c_uint);
                                n1
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) != 0
                        }
                }
                && {
                    n2 = (*s.offset((i + 2 as libc::c_int) as isize) as libc::c_int
                        - '0' as i32) as libc::c_uint;
                    n2 <= 9 as libc::c_int as libc::c_uint
                        || {
                            n2 = ((*s.offset((i + 2 as libc::c_int) as isize)
                                as libc::c_int & 0xdf as libc::c_int) - 'A' as i32)
                                as libc::c_uint;
                            (if n2 <= 5 as libc::c_int as libc::c_uint {
                                n2 = n2.wrapping_add(10 as libc::c_int as libc::c_uint);
                                n2
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) != 0
                        }
                }
                && {
                    x = n1 << 4 as libc::c_int | n2;
                    burl_is_unreserved(x as libc::c_int) == 0
                }
            {
            if x >= 0xf5 as libc::c_int as libc::c_uint
                || x | 0x1 as libc::c_int as libc::c_uint
                    == 0xc1 as libc::c_int as libc::c_uint
            {
                qs = -(2 as libc::c_int);
            }
            if *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int >= 'a' as i32 {
                let ref mut fresh0 = *((*b).ptr).offset((i + 1 as libc::c_int) as isize);
                *fresh0 = (*fresh0 as libc::c_int & 0xdf as libc::c_int) as libc::c_char;
            }
            if *s.offset((i + 2 as libc::c_int) as isize) as libc::c_int >= 'a' as i32 {
                let ref mut fresh1 = *((*b).ptr).offset((i + 2 as libc::c_int) as isize);
                *fresh1 = (*fresh1 as libc::c_int & 0xdf as libc::c_int) as libc::c_char;
            }
            i += 2 as libc::c_int;
        } else if *s.offset(i as isize) as libc::c_int == '#' as i32 {
            buffer_truncate(b, i as size_t as uint32_t);
            break;
        } else {
            qs = burl_normalize_basic_unreserved_fix(b, t, i, qs);
            break;
        }
        i += 1;
    }
    return qs;
}
unsafe extern "C" fn burl_normalize_basic_required_fix(
    mut b: *mut buffer,
    mut t: *mut buffer,
    mut i: libc::c_int,
    mut qs: libc::c_int,
) -> libc::c_int {
    let mut j: libc::c_int = i;
    let used: libc::c_int = buffer_clen(b) as libc::c_int;
    let s: *const libc::c_uchar = (*b).ptr as *mut libc::c_uchar;
    let p: *mut libc::c_uchar = buffer_string_prepare_copy(
        t,
        (i + (used - i) * 3 as libc::c_int + 1 as libc::c_int) as size_t,
    ) as *mut libc::c_uchar;
    let mut n1: libc::c_uint = 0;
    let mut n2: libc::c_uint = 0;
    memcpy(p as *mut libc::c_void, s as *const libc::c_void, i as size_t);
    while i < used {
        if encoded_chars_http_uri_reqd[*s.offset(i as isize) as usize] == 0 {
            if *s.offset(i as isize) as libc::c_int == '?' as i32
                && -(1 as libc::c_int) == qs
            {
                qs = j;
            }
            *p.offset(j as isize) = *s.offset(i as isize);
        } else if *s.offset(i as isize) as libc::c_int == '%' as i32
                && {
                    n1 = (*s.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                        - '0' as i32) as libc::c_uint;
                    n1 <= 9 as libc::c_int as libc::c_uint
                        || {
                            n1 = ((*s.offset((i + 1 as libc::c_int) as isize)
                                as libc::c_int & 0xdf as libc::c_int) - 'A' as i32)
                                as libc::c_uint;
                            (if n1 <= 5 as libc::c_int as libc::c_uint {
                                n1 = n1.wrapping_add(10 as libc::c_int as libc::c_uint);
                                n1
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) != 0
                        }
                }
                && {
                    n2 = (*s.offset((i + 2 as libc::c_int) as isize) as libc::c_int
                        - '0' as i32) as libc::c_uint;
                    n2 <= 9 as libc::c_int as libc::c_uint
                        || {
                            n2 = ((*s.offset((i + 2 as libc::c_int) as isize)
                                as libc::c_int & 0xdf as libc::c_int) - 'A' as i32)
                                as libc::c_uint;
                            (if n2 <= 5 as libc::c_int as libc::c_uint {
                                n2 = n2.wrapping_add(10 as libc::c_int as libc::c_uint);
                                n2
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) != 0
                        }
                }
            {
            let x: libc::c_uint = n1 << 4 as libc::c_int | n2;
            if encoded_chars_http_uri_reqd[x as usize] == 0
                && (if qs < 0 as libc::c_int {
                    (x != '/' as i32 as libc::c_uint && x != '?' as i32 as libc::c_uint)
                        as libc::c_int
                } else {
                    (x != '&' as i32 as libc::c_uint && x != '=' as i32 as libc::c_uint
                        && x != ';' as i32 as libc::c_uint
                        && x != '+' as i32 as libc::c_uint) as libc::c_int
                }) != 0
            {
                *p.offset(j as isize) = x as libc::c_uchar;
            } else {
                *p.offset(j as isize) = '%' as i32 as libc::c_uchar;
                j += 1;
                *p.offset(j as isize) = hex_chars_uc[n1 as usize] as libc::c_uchar;
                j += 1;
                *p.offset(j as isize) = hex_chars_uc[n2 as usize] as libc::c_uchar;
                if x >= 0xf5 as libc::c_int as libc::c_uint
                    || x | 0x1 as libc::c_int as libc::c_uint
                        == 0xc1 as libc::c_int as libc::c_uint
                {
                    qs = -(2 as libc::c_int);
                }
            }
            i += 2 as libc::c_int;
        } else {
            if *s.offset(i as isize) as libc::c_int == '#' as i32 {
                break;
            }
            *p.offset(j as isize) = '%' as i32 as libc::c_uchar;
            j += 1;
            *p
                .offset(
                    j as isize,
                ) = hex_chars_uc[(*s.offset(i as isize) as libc::c_int
                >> 4 as libc::c_int & 0xf as libc::c_int) as usize] as libc::c_uchar;
            j += 1;
            *p
                .offset(
                    j as isize,
                ) = hex_chars_uc[(*s.offset(i as isize) as libc::c_int
                & 0xf as libc::c_int) as usize] as libc::c_uchar;
            if *s.offset(i as isize) as libc::c_int >= 0xf5 as libc::c_int
                || *s.offset(i as isize) as libc::c_int | 0x1 as libc::c_int
                    == 0xc1 as libc::c_int
            {
                qs = -(2 as libc::c_int);
            }
        }
        i += 1;
        j += 1;
    }
    buffer_copy_string_len(b, p as *mut libc::c_char, j as size_t);
    return qs;
}
unsafe extern "C" fn burl_normalize_basic_required(
    mut b: *mut buffer,
    mut t: *mut buffer,
) -> libc::c_int {
    let s: *const libc::c_uchar = (*b).ptr as *mut libc::c_uchar;
    let used: libc::c_int = buffer_clen(b) as libc::c_int;
    let mut n1: libc::c_uint = 0;
    let mut n2: libc::c_uint = 0;
    let mut x: libc::c_uint = 0;
    let mut qs: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < used {
        if encoded_chars_http_uri_reqd[*s.offset(i as isize) as usize] == 0 {
            if *s.offset(i as isize) as libc::c_int == '?' as i32
                && -(1 as libc::c_int) == qs
            {
                qs = i;
            }
        } else if *s.offset(i as isize) as libc::c_int == '%' as i32
                && {
                    n1 = (*s.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                        - '0' as i32) as libc::c_uint;
                    n1 <= 9 as libc::c_int as libc::c_uint
                        || {
                            n1 = ((*s.offset((i + 1 as libc::c_int) as isize)
                                as libc::c_int & 0xdf as libc::c_int) - 'A' as i32)
                                as libc::c_uint;
                            (if n1 <= 5 as libc::c_int as libc::c_uint {
                                n1 = n1.wrapping_add(10 as libc::c_int as libc::c_uint);
                                n1
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) != 0
                        }
                }
                && {
                    n2 = (*s.offset((i + 2 as libc::c_int) as isize) as libc::c_int
                        - '0' as i32) as libc::c_uint;
                    n2 <= 9 as libc::c_int as libc::c_uint
                        || {
                            n2 = ((*s.offset((i + 2 as libc::c_int) as isize)
                                as libc::c_int & 0xdf as libc::c_int) - 'A' as i32)
                                as libc::c_uint;
                            (if n2 <= 5 as libc::c_int as libc::c_uint {
                                n2 = n2.wrapping_add(10 as libc::c_int as libc::c_uint);
                                n2
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) != 0
                        }
                }
                && {
                    x = n1 << 4 as libc::c_int | n2;
                    encoded_chars_http_uri_reqd[x as usize] as libc::c_int != 0
                        || (if qs < 0 as libc::c_int {
                            (x == '/' as i32 as libc::c_uint
                                || x == '?' as i32 as libc::c_uint) as libc::c_int
                        } else {
                            (x == '&' as i32 as libc::c_uint
                                || x == '=' as i32 as libc::c_uint
                                || x == ';' as i32 as libc::c_uint
                                || x == '+' as i32 as libc::c_uint) as libc::c_int
                        }) != 0
                }
            {
            if x >= 0xf5 as libc::c_int as libc::c_uint
                || x | 0x1 as libc::c_int as libc::c_uint
                    == 0xc1 as libc::c_int as libc::c_uint
            {
                qs = -(2 as libc::c_int);
            }
            if *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int >= 'a' as i32 {
                let ref mut fresh2 = *((*b).ptr).offset((i + 1 as libc::c_int) as isize);
                *fresh2 = (*fresh2 as libc::c_int & 0xdf as libc::c_int) as libc::c_char;
            }
            if *s.offset((i + 2 as libc::c_int) as isize) as libc::c_int >= 'a' as i32 {
                let ref mut fresh3 = *((*b).ptr).offset((i + 2 as libc::c_int) as isize);
                *fresh3 = (*fresh3 as libc::c_int & 0xdf as libc::c_int) as libc::c_char;
            }
            i += 2 as libc::c_int;
        } else if *s.offset(i as isize) as libc::c_int == '#' as i32 {
            buffer_truncate(b, i as size_t as uint32_t);
            break;
        } else {
            qs = burl_normalize_basic_required_fix(b, t, i, qs);
            break;
        }
        i += 1;
    }
    return qs;
}
unsafe extern "C" fn burl_contains_ctrls(mut b: *const buffer) -> libc::c_int {
    let s: *const libc::c_char = (*b).ptr;
    let used: libc::c_int = buffer_clen(b) as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < used {
        if *s.offset(i as isize) as libc::c_int == '%' as i32
            && ((*s.offset((i + 1 as libc::c_int) as isize) as libc::c_int) < '2' as i32
                || *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    == '7' as i32
                    && *s.offset((i + 2 as libc::c_int) as isize) as libc::c_int
                        == 'F' as i32)
        {
            return 1 as libc::c_int;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn burl_normalize_qs20_to_plus_fix(
    mut b: *mut buffer,
    mut i: libc::c_int,
) {
    let s: *mut libc::c_char = (*b).ptr;
    let used: libc::c_int = buffer_clen(b) as libc::c_int;
    let mut j: libc::c_int = i;
    while i < used {
        *s.offset(j as isize) = *s.offset(i as isize);
        if *s.offset(i as isize) as libc::c_int == '%' as i32
            && *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '2' as i32
            && *s.offset((i + 2 as libc::c_int) as isize) as libc::c_int == '0' as i32
        {
            *s.offset(j as isize) = '+' as i32 as libc::c_char;
            i += 2 as libc::c_int;
        }
        i += 1;
        j += 1;
    }
    buffer_truncate(b, j as uint32_t);
}
unsafe extern "C" fn burl_normalize_qs20_to_plus(
    mut b: *mut buffer,
    mut qs: libc::c_int,
) {
    let s: *const libc::c_char = (*b).ptr;
    let used: libc::c_int = if qs < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        buffer_clen(b) as libc::c_int
    };
    let mut i: libc::c_int = 0;
    if qs < 0 as libc::c_int {
        return;
    }
    i = qs + 1 as libc::c_int;
    while i < used {
        if *s.offset(i as isize) as libc::c_int == '%' as i32
            && *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '2' as i32
            && *s.offset((i + 2 as libc::c_int) as isize) as libc::c_int == '0' as i32
        {
            break;
        }
        i += 1;
    }
    if i != used {
        burl_normalize_qs20_to_plus_fix(b, i);
    }
}
unsafe extern "C" fn burl_normalize_2F_to_slash_fix(
    mut b: *mut buffer,
    mut qs: libc::c_int,
    mut i: libc::c_int,
) -> libc::c_int {
    let s: *mut libc::c_char = (*b).ptr;
    let blen: libc::c_int = buffer_clen(b) as libc::c_int;
    let used: libc::c_int = if qs < 0 as libc::c_int { blen } else { qs };
    let mut j: libc::c_int = i;
    while i < used {
        *s.offset(j as isize) = *s.offset(i as isize);
        if *s.offset(i as isize) as libc::c_int == '%' as i32
            && *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '2' as i32
            && *s.offset((i + 2 as libc::c_int) as isize) as libc::c_int == 'F' as i32
        {
            *s.offset(j as isize) = '/' as i32 as libc::c_char;
            i += 2 as libc::c_int;
        }
        i += 1;
        j += 1;
    }
    if qs >= 0 as libc::c_int {
        let qslen: libc::c_int = blen - qs;
        memmove(
            s.offset(j as isize) as *mut libc::c_void,
            s.offset(qs as isize) as *const libc::c_void,
            qslen as size_t,
        );
        qs = j;
        j += qslen;
    }
    buffer_truncate(b, j as uint32_t);
    return qs;
}
unsafe extern "C" fn burl_normalize_2F_to_slash(
    mut b: *mut buffer,
    mut qs: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let s: *const libc::c_char = (*b).ptr;
    let used: libc::c_int = if qs < 0 as libc::c_int {
        buffer_clen(b) as libc::c_int
    } else {
        qs
    };
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < used {
        if *s.offset(i as isize) as libc::c_int == '%' as i32
            && *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '2' as i32
            && *s.offset((i + 2 as libc::c_int) as isize) as libc::c_int == 'F' as i32
        {
            return if flags & HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_DECODE as libc::c_int
                != 0
            {
                burl_normalize_2F_to_slash_fix(b, qs, i)
            } else {
                -(2 as libc::c_int)
            };
        }
        i += 1;
    }
    return qs;
}
unsafe extern "C" fn burl_normalize_path(
    mut b: *mut buffer,
    mut t: *mut buffer,
    mut qs: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let s: *const libc::c_uchar = (*b).ptr as *mut libc::c_uchar;
    let used: libc::c_int = buffer_clen(b) as libc::c_int;
    let mut path_simplify: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = if qs < 0 as libc::c_int { used } else { qs };
    while i < len {
        if *s.offset(i as isize) as libc::c_int == '.' as i32
            && (*s.offset((i + 1 as libc::c_int) as isize) as libc::c_int != '.' as i32
                || {
                    i += 1;
                    i != 0
                })
            && (*s.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '/' as i32
                || *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    == '?' as i32
                || *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    == '\u{0}' as i32)
        {
            path_simplify = 1 as libc::c_int;
            break;
        } else {
            while i < len && *s.offset(i as isize) as libc::c_int != '/' as i32 {
                i += 1;
            }
            if *s.offset(i as isize) as libc::c_int == '/' as i32
                && *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    == '/' as i32
            {
                path_simplify = 1 as libc::c_int;
                break;
            } else {
                i += 1;
            }
        }
    }
    if path_simplify != 0 {
        if flags & HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REJECT as libc::c_int != 0 {
            return -(2 as libc::c_int);
        }
        if qs >= 0 as libc::c_int {
            buffer_copy_string_len(
                t,
                ((*b).ptr).offset(qs as isize),
                (used - qs) as size_t,
            );
            buffer_truncate(b, qs as uint32_t);
        }
        buffer_path_simplify(b);
        if qs >= 0 as libc::c_int {
            qs = buffer_clen(b) as libc::c_int;
            buffer_append_string_len(b, (*t).ptr, buffer_clen(t) as size_t);
        }
    }
    return qs;
}
#[no_mangle]
pub unsafe extern "C" fn burl_normalize(
    mut b: *mut buffer,
    mut t: *mut buffer,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut qs: libc::c_int = 0;
    qs = if flags & HTTP_PARSEOPT_URL_NORMALIZE_REQUIRED as libc::c_int != 0 {
        burl_normalize_basic_required(b, t)
    } else {
        burl_normalize_basic_unreserved(b, t)
    };
    if -(2 as libc::c_int) == qs {
        return -(2 as libc::c_int);
    }
    if flags & HTTP_PARSEOPT_URL_NORMALIZE_CTRLS_REJECT as libc::c_int != 0 {
        if burl_contains_ctrls(b) != 0 {
            return -(2 as libc::c_int);
        }
    }
    if flags
        & (HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_DECODE as libc::c_int
            | HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_REJECT as libc::c_int) != 0
    {
        qs = burl_normalize_2F_to_slash(b, qs, flags);
        if -(2 as libc::c_int) == qs {
            return -(2 as libc::c_int);
        }
    }
    if flags
        & (HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REMOVE as libc::c_int
            | HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REJECT as libc::c_int) != 0
    {
        qs = burl_normalize_path(b, t, qs, flags);
        if -(2 as libc::c_int) == qs {
            return -(2 as libc::c_int);
        }
    }
    if flags & HTTP_PARSEOPT_URL_NORMALIZE_QUERY_20_PLUS as libc::c_int != 0 {
        if qs >= 0 as libc::c_int {
            burl_normalize_qs20_to_plus(b, qs);
        }
    }
    return qs;
}
unsafe extern "C" fn burl_append_encode_nde(
    b: *mut buffer,
    str: *const libc::c_char,
    len: size_t,
) {
    let p: *mut libc::c_char = buffer_string_prepare_append(
        b,
        len.wrapping_mul(3 as libc::c_int as libc::c_ulong),
    );
    let mut n1: libc::c_uint = 0;
    let mut n2: libc::c_uint = 0;
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < len {
        if *str.offset(i as isize) as libc::c_int == '%' as i32
            && {
                n1 = (*str
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int - '0' as i32) as libc::c_uint;
                n1 <= 9 as libc::c_int as libc::c_uint
                    || {
                        n1 = ((*str
                            .offset(
                                i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                            ) as libc::c_int & 0xdf as libc::c_int) - 'A' as i32)
                            as libc::c_uint;
                        (if n1 <= 5 as libc::c_int as libc::c_uint {
                            n1 = n1.wrapping_add(10 as libc::c_int as libc::c_uint);
                            n1
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) != 0
                    }
            }
            && {
                n2 = (*str
                    .offset(i.wrapping_add(2 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int - '0' as i32) as libc::c_uint;
                n2 <= 9 as libc::c_int as libc::c_uint
                    || {
                        n2 = ((*str
                            .offset(
                                i.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                            ) as libc::c_int & 0xdf as libc::c_int) - 'A' as i32)
                            as libc::c_uint;
                        (if n2 <= 5 as libc::c_int as libc::c_uint {
                            n2 = n2.wrapping_add(10 as libc::c_int as libc::c_uint);
                            n2
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) != 0
                    }
            }
        {
            let x: libc::c_uint = n1 << 4 as libc::c_int | n2;
            if burl_is_unreserved(x as libc::c_int) != 0 {
                *p.offset(j as isize) = x as libc::c_char;
            } else {
                *p.offset(j as isize) = '%' as i32 as libc::c_char;
                j += 1;
                *p
                    .offset(
                        j as isize,
                    ) = *str
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
                j += 1;
                *p
                    .offset(
                        j as isize,
                    ) = *str
                    .offset(i.wrapping_add(2 as libc::c_int as libc::c_uint) as isize);
            }
            i = i.wrapping_add(2 as libc::c_int as libc::c_uint);
        } else if burl_is_unreserved(*str.offset(i as isize) as libc::c_int) != 0 {
            *p.offset(j as isize) = *str.offset(i as isize);
        } else {
            *p.offset(j as isize) = '%' as i32 as libc::c_char;
            j += 1;
            *p
                .offset(
                    j as isize,
                ) = hex_chars_uc[(*str.offset(i as isize) as libc::c_int
                >> 4 as libc::c_int & 0xf as libc::c_int) as usize];
            j += 1;
            *p
                .offset(
                    j as isize,
                ) = hex_chars_uc[(*str.offset(i as isize) as libc::c_int
                & 0xf as libc::c_int) as usize];
        }
        i = i.wrapping_add(1);
        j += 1;
    }
    buffer_commit(b, j as size_t);
}
unsafe extern "C" fn burl_append_encode_psnde(
    b: *mut buffer,
    str: *const libc::c_char,
    len: size_t,
) {
    let p: *mut libc::c_char = buffer_string_prepare_append(
        b,
        len.wrapping_mul(3 as libc::c_int as libc::c_ulong),
    );
    let mut n1: libc::c_uint = 0;
    let mut n2: libc::c_uint = 0;
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < len {
        if *str.offset(i as isize) as libc::c_int == '%' as i32
            && {
                n1 = (*str
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int - '0' as i32) as libc::c_uint;
                n1 <= 9 as libc::c_int as libc::c_uint
                    || {
                        n1 = ((*str
                            .offset(
                                i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                            ) as libc::c_int & 0xdf as libc::c_int) - 'A' as i32)
                            as libc::c_uint;
                        (if n1 <= 5 as libc::c_int as libc::c_uint {
                            n1 = n1.wrapping_add(10 as libc::c_int as libc::c_uint);
                            n1
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) != 0
                    }
            }
            && {
                n2 = (*str
                    .offset(i.wrapping_add(2 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int - '0' as i32) as libc::c_uint;
                n2 <= 9 as libc::c_int as libc::c_uint
                    || {
                        n2 = ((*str
                            .offset(
                                i.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                            ) as libc::c_int & 0xdf as libc::c_int) - 'A' as i32)
                            as libc::c_uint;
                        (if n2 <= 5 as libc::c_int as libc::c_uint {
                            n2 = n2.wrapping_add(10 as libc::c_int as libc::c_uint);
                            n2
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) != 0
                    }
            }
        {
            let x: libc::c_uint = n1 << 4 as libc::c_int | n2;
            if burl_is_unreserved(x as libc::c_int) != 0 {
                *p.offset(j as isize) = x as libc::c_char;
            } else {
                *p.offset(j as isize) = '%' as i32 as libc::c_char;
                j += 1;
                *p
                    .offset(
                        j as isize,
                    ) = *str
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
                j += 1;
                *p
                    .offset(
                        j as isize,
                    ) = *str
                    .offset(i.wrapping_add(2 as libc::c_int as libc::c_uint) as isize);
            }
            i = i.wrapping_add(2 as libc::c_int as libc::c_uint);
        } else if burl_is_unreserved(*str.offset(i as isize) as libc::c_int) != 0
                || *str.offset(i as isize) as libc::c_int == '/' as i32
            {
            *p.offset(j as isize) = *str.offset(i as isize);
        } else {
            *p.offset(j as isize) = '%' as i32 as libc::c_char;
            j += 1;
            *p
                .offset(
                    j as isize,
                ) = hex_chars_uc[(*str.offset(i as isize) as libc::c_int
                >> 4 as libc::c_int & 0xf as libc::c_int) as usize];
            j += 1;
            *p
                .offset(
                    j as isize,
                ) = hex_chars_uc[(*str.offset(i as isize) as libc::c_int
                & 0xf as libc::c_int) as usize];
        }
        i = i.wrapping_add(1);
        j += 1;
    }
    buffer_commit(b, j as size_t);
}
unsafe extern "C" fn burl_append_encode_all(
    b: *mut buffer,
    str: *const libc::c_char,
    len: size_t,
) {
    let p: *mut libc::c_char = buffer_string_prepare_append(
        b,
        len.wrapping_mul(3 as libc::c_int as libc::c_ulong),
    );
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < len {
        if burl_is_unreserved(*str.offset(i as isize) as libc::c_int) != 0 {
            *p.offset(j as isize) = *str.offset(i as isize);
        } else {
            *p.offset(j as isize) = '%' as i32 as libc::c_char;
            j += 1;
            *p
                .offset(
                    j as isize,
                ) = hex_chars_uc[(*str.offset(i as isize) as libc::c_int
                >> 4 as libc::c_int & 0xf as libc::c_int) as usize];
            j += 1;
            *p
                .offset(
                    j as isize,
                ) = hex_chars_uc[(*str.offset(i as isize) as libc::c_int
                & 0xf as libc::c_int) as usize];
        }
        i = i.wrapping_add(1);
        j += 1;
    }
    buffer_commit(b, j as size_t);
}
unsafe extern "C" fn burl_offset_tolower(b: *mut buffer, off: size_t) {
    let mut p: *mut libc::c_char = ((*b).ptr).offset(off as isize);
    while *p.offset(0 as libc::c_int as isize) != 0 {
        if (*p.offset(0 as libc::c_int as isize) as uint32_t)
            .wrapping_sub('A' as i32 as libc::c_uint)
            <= ('Z' as i32 - 'A' as i32) as libc::c_uint
        {
            let ref mut fresh4 = *p.offset(0 as libc::c_int as isize);
            *fresh4 = (*fresh4 as libc::c_int | 0x20 as libc::c_int) as libc::c_char;
        } else if *p.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32
                && light_isxdigit(*p.offset(1 as libc::c_int as isize) as libc::c_int)
                    != 0
                && light_isxdigit(*p.offset(2 as libc::c_int as isize) as libc::c_int)
                    != 0
            {
            p = p.offset(2 as libc::c_int as isize);
        }
        p = p.offset(1);
    }
}
unsafe extern "C" fn burl_offset_toupper(b: *mut buffer, off: size_t) {
    let mut p: *mut libc::c_char = ((*b).ptr).offset(off as isize);
    while *p.offset(0 as libc::c_int as isize) != 0 {
        if (*p.offset(0 as libc::c_int as isize) as uint32_t)
            .wrapping_sub('a' as i32 as libc::c_uint)
            <= ('z' as i32 - 'a' as i32) as libc::c_uint
        {
            let ref mut fresh5 = *p.offset(0 as libc::c_int as isize);
            *fresh5 = (*fresh5 as libc::c_int & 0xdf as libc::c_int) as libc::c_char;
        } else if *p.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32
                && light_isxdigit(*p.offset(1 as libc::c_int as isize) as libc::c_int)
                    != 0
                && light_isxdigit(*p.offset(2 as libc::c_int as isize) as libc::c_int)
                    != 0
            {
            p = p.offset(2 as libc::c_int as isize);
        }
        p = p.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn burl_append(
    b: *mut buffer,
    str: *const libc::c_char,
    len: size_t,
    flags: libc::c_int,
) {
    let mut off: size_t = 0 as libc::c_int as size_t;
    if 0 as libc::c_int as libc::c_ulong == len {
        return;
    }
    if 0 as libc::c_int == flags {
        buffer_append_string_len(b, str, len);
        return;
    }
    if flags & (BURL_TOUPPER as libc::c_int | BURL_TOLOWER as libc::c_int) != 0 {
        off = buffer_clen(b) as size_t;
    }
    if flags & BURL_ENCODE_NONE as libc::c_int != 0 {
        buffer_append_string_len(b, str, len);
    } else if flags & BURL_ENCODE_ALL as libc::c_int != 0 {
        burl_append_encode_all(b, str, len);
    } else if flags & BURL_ENCODE_NDE as libc::c_int != 0 {
        burl_append_encode_nde(b, str, len);
    } else if flags & BURL_ENCODE_PSNDE as libc::c_int != 0 {
        burl_append_encode_psnde(b, str, len);
    } else if flags & BURL_ENCODE_B64U as libc::c_int != 0 {
        let mut s: *const libc::c_uchar = str as *const libc::c_uchar;
        buffer_append_base64_enc(b, s, len, BASE64_URL, 0 as libc::c_int);
    } else if flags & BURL_DECODE_B64U as libc::c_int != 0 {
        buffer_append_base64_decode(b, str, len, BASE64_URL);
    }
    if flags & (BURL_TOLOWER as libc::c_int | BURL_TOUPPER as libc::c_int) != 0 {
        if flags & BURL_TOLOWER as libc::c_int != 0 {
            burl_offset_tolower(b, off);
        } else {
            burl_offset_toupper(b, off);
        };
    }
}
