use ::libc;
extern "C" {
    fn buffer_string_prepare_append(b: *mut buffer, size: size_t) -> *mut libc::c_char;
    fn buffer_commit(b: *mut buffer, size: size_t);
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
}
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type int_fast32_t = libc::c_long;
pub type uint_fast32_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub ptr: *mut libc::c_char,
    pub used: uint32_t,
    pub size: uint32_t,
}
pub type base64_charset = libc::c_uint;
pub const BASE64_URL: base64_charset = 1;
pub const BASE64_STANDARD: base64_charset = 0;
static mut base64_standard_table: [libc::c_char; 66] = unsafe {
    *::std::mem::transmute::<
        &[u8; 66],
        &[libc::c_char; 66],
    >(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=\0")
};
static mut base64_standard_reverse_table: [libc::c_schar; 128] = [
    -(1 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    62 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    63 as libc::c_int as libc::c_schar,
    52 as libc::c_int as libc::c_schar,
    53 as libc::c_int as libc::c_schar,
    54 as libc::c_int as libc::c_schar,
    55 as libc::c_int as libc::c_schar,
    56 as libc::c_int as libc::c_schar,
    57 as libc::c_int as libc::c_schar,
    58 as libc::c_int as libc::c_schar,
    59 as libc::c_int as libc::c_schar,
    60 as libc::c_int as libc::c_schar,
    61 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(3 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    0 as libc::c_int as libc::c_schar,
    1 as libc::c_int as libc::c_schar,
    2 as libc::c_int as libc::c_schar,
    3 as libc::c_int as libc::c_schar,
    4 as libc::c_int as libc::c_schar,
    5 as libc::c_int as libc::c_schar,
    6 as libc::c_int as libc::c_schar,
    7 as libc::c_int as libc::c_schar,
    8 as libc::c_int as libc::c_schar,
    9 as libc::c_int as libc::c_schar,
    10 as libc::c_int as libc::c_schar,
    11 as libc::c_int as libc::c_schar,
    12 as libc::c_int as libc::c_schar,
    13 as libc::c_int as libc::c_schar,
    14 as libc::c_int as libc::c_schar,
    15 as libc::c_int as libc::c_schar,
    16 as libc::c_int as libc::c_schar,
    17 as libc::c_int as libc::c_schar,
    18 as libc::c_int as libc::c_schar,
    19 as libc::c_int as libc::c_schar,
    20 as libc::c_int as libc::c_schar,
    21 as libc::c_int as libc::c_schar,
    22 as libc::c_int as libc::c_schar,
    23 as libc::c_int as libc::c_schar,
    24 as libc::c_int as libc::c_schar,
    25 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    26 as libc::c_int as libc::c_schar,
    27 as libc::c_int as libc::c_schar,
    28 as libc::c_int as libc::c_schar,
    29 as libc::c_int as libc::c_schar,
    30 as libc::c_int as libc::c_schar,
    31 as libc::c_int as libc::c_schar,
    32 as libc::c_int as libc::c_schar,
    33 as libc::c_int as libc::c_schar,
    34 as libc::c_int as libc::c_schar,
    35 as libc::c_int as libc::c_schar,
    36 as libc::c_int as libc::c_schar,
    37 as libc::c_int as libc::c_schar,
    38 as libc::c_int as libc::c_schar,
    39 as libc::c_int as libc::c_schar,
    40 as libc::c_int as libc::c_schar,
    41 as libc::c_int as libc::c_schar,
    42 as libc::c_int as libc::c_schar,
    43 as libc::c_int as libc::c_schar,
    44 as libc::c_int as libc::c_schar,
    45 as libc::c_int as libc::c_schar,
    46 as libc::c_int as libc::c_schar,
    47 as libc::c_int as libc::c_schar,
    48 as libc::c_int as libc::c_schar,
    49 as libc::c_int as libc::c_schar,
    50 as libc::c_int as libc::c_schar,
    51 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
];
static mut base64_url_table: [libc::c_char; 66] = unsafe {
    *::std::mem::transmute::<
        &[u8; 66],
        &[libc::c_char; 66],
    >(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_=\0")
};
static mut base64_url_reverse_table: [libc::c_schar; 128] = [
    -(1 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    62 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    52 as libc::c_int as libc::c_schar,
    53 as libc::c_int as libc::c_schar,
    54 as libc::c_int as libc::c_schar,
    55 as libc::c_int as libc::c_schar,
    56 as libc::c_int as libc::c_schar,
    57 as libc::c_int as libc::c_schar,
    58 as libc::c_int as libc::c_schar,
    59 as libc::c_int as libc::c_schar,
    60 as libc::c_int as libc::c_schar,
    61 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(3 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    0 as libc::c_int as libc::c_schar,
    1 as libc::c_int as libc::c_schar,
    2 as libc::c_int as libc::c_schar,
    3 as libc::c_int as libc::c_schar,
    4 as libc::c_int as libc::c_schar,
    5 as libc::c_int as libc::c_schar,
    6 as libc::c_int as libc::c_schar,
    7 as libc::c_int as libc::c_schar,
    8 as libc::c_int as libc::c_schar,
    9 as libc::c_int as libc::c_schar,
    10 as libc::c_int as libc::c_schar,
    11 as libc::c_int as libc::c_schar,
    12 as libc::c_int as libc::c_schar,
    13 as libc::c_int as libc::c_schar,
    14 as libc::c_int as libc::c_schar,
    15 as libc::c_int as libc::c_schar,
    16 as libc::c_int as libc::c_schar,
    17 as libc::c_int as libc::c_schar,
    18 as libc::c_int as libc::c_schar,
    19 as libc::c_int as libc::c_schar,
    20 as libc::c_int as libc::c_schar,
    21 as libc::c_int as libc::c_schar,
    22 as libc::c_int as libc::c_schar,
    23 as libc::c_int as libc::c_schar,
    24 as libc::c_int as libc::c_schar,
    25 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    63 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    26 as libc::c_int as libc::c_schar,
    27 as libc::c_int as libc::c_schar,
    28 as libc::c_int as libc::c_schar,
    29 as libc::c_int as libc::c_schar,
    30 as libc::c_int as libc::c_schar,
    31 as libc::c_int as libc::c_schar,
    32 as libc::c_int as libc::c_schar,
    33 as libc::c_int as libc::c_schar,
    34 as libc::c_int as libc::c_schar,
    35 as libc::c_int as libc::c_schar,
    36 as libc::c_int as libc::c_schar,
    37 as libc::c_int as libc::c_schar,
    38 as libc::c_int as libc::c_schar,
    39 as libc::c_int as libc::c_schar,
    40 as libc::c_int as libc::c_schar,
    41 as libc::c_int as libc::c_schar,
    42 as libc::c_int as libc::c_schar,
    43 as libc::c_int as libc::c_schar,
    44 as libc::c_int as libc::c_schar,
    45 as libc::c_int as libc::c_schar,
    46 as libc::c_int as libc::c_schar,
    47 as libc::c_int as libc::c_schar,
    48 as libc::c_int as libc::c_schar,
    49 as libc::c_int as libc::c_schar,
    50 as libc::c_int as libc::c_schar,
    51 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
];
#[no_mangle]
pub unsafe extern "C" fn li_base64_dec(
    result: *mut libc::c_uchar,
    out_length: size_t,
    in_0: *const libc::c_char,
    in_length: size_t,
    charset: base64_charset,
) -> size_t {
    let mut i: size_t = 0;
    let base64_reverse_table: *const libc::c_schar = if charset as libc::c_uint != 0 {
        base64_url_reverse_table.as_ptr()
    } else {
        base64_standard_reverse_table.as_ptr()
    };
    let mut ch: int_fast32_t = 0 as libc::c_int as int_fast32_t;
    let mut out4: int_fast32_t = 0 as libc::c_int as int_fast32_t;
    let mut out_pos: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < in_length {
        let c: uint_fast32_t = *(in_0 as *mut libc::c_uchar).offset(i as isize)
            as uint_fast32_t;
        ch = (if c < 128 as libc::c_int as libc::c_ulong {
            *base64_reverse_table.offset(c as isize) as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as int_fast32_t;
        if (ch < 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long != 0 {
            if !(-(2 as libc::c_int) as libc::c_long == ch) {
                break;
            }
        } else {
            out4 = out4 << 6 as libc::c_int | ch;
            if i & 3 as libc::c_int as libc::c_ulong == 3 as libc::c_int as libc::c_ulong
            {
                *result
                    .offset(
                        out_pos as isize,
                    ) = (out4 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_long)
                    as libc::c_uchar;
                *result
                    .offset(
                        out_pos.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = (out4 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_long)
                    as libc::c_uchar;
                *result
                    .offset(
                        out_pos.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                    ) = (out4 & 0xff as libc::c_int as libc::c_long) as libc::c_uchar;
                out_pos = (out_pos as libc::c_ulong)
                    .wrapping_add(3 as libc::c_int as libc::c_ulong) as size_t as size_t;
                out4 = 0 as libc::c_int as int_fast32_t;
            }
        }
        i = i.wrapping_add(1);
    }
    let mut current_block_21: u64;
    match if i == in_length || ch == -(3 as libc::c_int) as libc::c_long
        || *in_0.offset(i as isize) as libc::c_int != '\u{0}' as i32
    {
        i & 3 as libc::c_int as libc::c_ulong
    } else {
        1 as libc::c_int as libc::c_ulong
    } {
        3 => {
            let fresh0 = out_pos;
            out_pos = out_pos.wrapping_add(1);
            *result
                .offset(fresh0 as isize) = (out4 >> 10 as libc::c_int) as libc::c_uchar;
            out4 <<= 2 as libc::c_int;
            current_block_21 = 2165623813156138699;
        }
        2 => {
            current_block_21 = 2165623813156138699;
        }
        0 => {
            current_block_21 = 15822148388160763871;
        }
        1 | _ => return 0 as libc::c_int as size_t,
    }
    match current_block_21 {
        2165623813156138699 => {
            let fresh1 = out_pos;
            out_pos = out_pos.wrapping_add(1);
            *result
                .offset(
                    fresh1 as isize,
                ) = (out4 >> 4 as libc::c_int & 0xff as libc::c_int as libc::c_long)
                as libc::c_uchar;
        }
        _ => {}
    }
    if !(out_pos <= out_length) {
        ck_assert_failed(
            b"src/base64.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int as libc::c_uint,
            b"out_pos <= out_length\0" as *const u8 as *const libc::c_char,
        );
    }
    return out_pos;
}
#[no_mangle]
pub unsafe extern "C" fn li_base64_enc(
    out: *mut libc::c_char,
    out_length: size_t,
    in_0: *const libc::c_uchar,
    in_length: size_t,
    charset: base64_charset,
    pad: libc::c_int,
) -> size_t {
    let mut i: size_t = 0;
    let mut out_pos: size_t = 0 as libc::c_int as size_t;
    let mut v: uint_fast32_t = 0;
    let base64_table: *const libc::c_char = if charset as libc::c_uint != 0 {
        base64_url_table.as_ptr()
    } else {
        base64_standard_table.as_ptr()
    };
    let padchar: libc::c_char = *base64_table.offset(64 as libc::c_int as isize);
    if !(in_length <= 3221225469 as libc::c_long as libc::c_ulong) {
        ck_assert_failed(
            b"src/base64.c\0" as *const u8 as *const libc::c_char,
            101 as libc::c_int as libc::c_uint,
            b"in_length <= 3221225469\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(in_length
        .wrapping_add(2 as libc::c_int as libc::c_ulong)
        .wrapping_div(3 as libc::c_int as libc::c_ulong)
        .wrapping_mul(4 as libc::c_int as libc::c_ulong) <= out_length)
    {
        ck_assert_failed(
            b"src/base64.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int as libc::c_uint,
            b"(in_length+2)/3*4 <= out_length\0" as *const u8 as *const libc::c_char,
        );
    }
    i = 2 as libc::c_int as size_t;
    while i < in_length {
        v = ((*in_0.offset(i.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int) << 16 as libc::c_int
            | (*in_0.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int) << 8 as libc::c_int
            | *in_0.offset(i as isize) as libc::c_int) as uint_fast32_t;
        *out
            .offset(
                out_pos.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
            ) = *base64_table
            .offset(
                (v >> 18 as libc::c_int & 0x3f as libc::c_int as libc::c_ulong) as isize,
            );
        *out
            .offset(
                out_pos.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = *base64_table
            .offset(
                (v >> 12 as libc::c_int & 0x3f as libc::c_int as libc::c_ulong) as isize,
            );
        *out
            .offset(
                out_pos.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
            ) = *base64_table
            .offset(
                (v >> 6 as libc::c_int & 0x3f as libc::c_int as libc::c_ulong) as isize,
            );
        *out
            .offset(
                out_pos.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
            ) = *base64_table
            .offset((v & 0x3f as libc::c_int as libc::c_ulong) as isize);
        out_pos = (out_pos as libc::c_ulong)
            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
        i = (i as libc::c_ulong).wrapping_add(3 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    match in_length.wrapping_sub(i.wrapping_sub(2 as libc::c_int as libc::c_ulong)) {
        1 => {
            v = ((*in_0
                .offset(i.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int) << 4 as libc::c_int) as uint_fast32_t;
            *out
                .offset(
                    out_pos.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
                ) = *base64_table
                .offset(
                    (v >> 6 as libc::c_int & 0x3f as libc::c_int as libc::c_ulong)
                        as isize,
                );
            *out
                .offset(
                    out_pos.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *base64_table
                .offset((v & 0x3f as libc::c_int as libc::c_ulong) as isize);
            if pad != 0 {
                let ref mut fresh2 = *out
                    .offset(
                        out_pos.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
                    );
                *fresh2 = padchar;
                *out
                    .offset(
                        out_pos.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                    ) = *fresh2;
                out_pos = (out_pos as libc::c_ulong)
                    .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
            } else {
                out_pos = (out_pos as libc::c_ulong)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            }
        }
        2 => {
            v = ((*in_0
                .offset(i.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int) << 10 as libc::c_int
                | (*in_0
                    .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int) << 2 as libc::c_int) as uint_fast32_t;
            *out
                .offset(
                    out_pos.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
                ) = *base64_table
                .offset(
                    (v >> 12 as libc::c_int & 0x3f as libc::c_int as libc::c_ulong)
                        as isize,
                );
            *out
                .offset(
                    out_pos.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *base64_table
                .offset(
                    (v >> 6 as libc::c_int & 0x3f as libc::c_int as libc::c_ulong)
                        as isize,
                );
            *out
                .offset(
                    out_pos.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                ) = *base64_table
                .offset((v & 0x3f as libc::c_int as libc::c_ulong) as isize);
            if pad != 0 {
                *out
                    .offset(
                        out_pos.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
                    ) = padchar;
                out_pos = (out_pos as libc::c_ulong)
                    .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
            } else {
                out_pos = (out_pos as libc::c_ulong)
                    .wrapping_add(3 as libc::c_int as libc::c_ulong) as size_t as size_t;
            }
        }
        0 | _ => {}
    }
    return out_pos;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_append_base64_enc(
    mut out: *mut buffer,
    mut in_0: *const libc::c_uchar,
    mut in_length: size_t,
    mut charset: base64_charset,
    mut pad: libc::c_int,
) -> *mut libc::c_char {
    let reserve: size_t = in_length
        .wrapping_add(2 as libc::c_int as libc::c_ulong)
        .wrapping_div(3 as libc::c_int as libc::c_ulong)
        .wrapping_mul(4 as libc::c_int as libc::c_ulong);
    let result: *mut libc::c_char = buffer_string_prepare_append(out, reserve);
    let out_pos: size_t = li_base64_enc(result, reserve, in_0, in_length, charset, pad);
    buffer_commit(out, out_pos);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_append_base64_decode(
    mut out: *mut buffer,
    mut in_0: *const libc::c_char,
    mut in_length: size_t,
    mut charset: base64_charset,
) -> *mut libc::c_uchar {
    let reserve: size_t = (3 as libc::c_int as libc::c_ulong)
        .wrapping_mul(in_length.wrapping_div(4 as libc::c_int as libc::c_ulong))
        .wrapping_add(3 as libc::c_int as libc::c_ulong);
    let result: *mut libc::c_uchar = buffer_string_prepare_append(out, reserve)
        as *mut libc::c_uchar;
    let out_pos: size_t = li_base64_dec(result, reserve, in_0, in_length, charset);
    buffer_commit(out, out_pos);
    return if out_pos != 0 || in_length == 0 { result } else { 0 as *mut libc::c_uchar };
}
