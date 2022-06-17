use ::libc;
extern "C" {
    pub type fdlog_st;
    pub type pcre2_real_match_data_8;
    pub type pcre2_real_code_8;
    pub type pcre2_real_general_context_8;
    pub type pcre2_real_compile_context_8;
    pub type pcre2_real_match_context_8;
    fn buffer_string_prepare_copy(b: *mut buffer, size: size_t) -> *mut libc::c_char;
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_int(b: *mut buffer, val: intmax_t);
    fn burl_append(
        b: *mut buffer,
        str: *const libc::c_char,
        len: size_t,
        flags: libc::c_int,
    );
    fn log_error(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        fmt: *const libc::c_char,
        _: ...
    );
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn pcre2_match_data_free_8(_: *mut pcre2_match_data_8);
    fn pcre2_compile_8(
        _: PCRE2_SPTR8,
        _: size_t,
        _: uint32_t,
        _: *mut libc::c_int,
        _: *mut size_t,
        _: *mut pcre2_compile_context_8,
    ) -> *mut pcre2_code_8;
    fn pcre2_jit_compile_8(_: *mut pcre2_code_8, _: uint32_t) -> libc::c_int;
    fn pcre2_get_ovector_pointer_8(_: *mut pcre2_match_data_8) -> *mut size_t;
    fn pcre2_match_8(
        _: *const pcre2_code_8,
        _: PCRE2_SPTR8,
        _: size_t,
        _: size_t,
        _: uint32_t,
        _: *mut pcre2_match_data_8,
        _: *mut pcre2_match_context_8,
    ) -> libc::c_int;
    fn pcre2_pattern_info_8(
        _: *const pcre2_code_8,
        _: uint32_t,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    fn pcre2_get_error_message_8(
        _: libc::c_int,
        _: *mut PCRE2_UCHAR8,
        _: size_t,
    ) -> libc::c_int;
    fn pcre2_match_data_create_8(
        _: uint32_t,
        _: *mut pcre2_general_context_8,
    ) -> *mut pcre2_match_data_8;
    fn pcre2_code_free_8(_: *mut pcre2_code_8);
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __intmax_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type intmax_t = __intmax_t;
pub type log_error_st = fdlog_st;
pub type handler_t = libc::c_uint;
pub const HANDLER_ERROR: handler_t = 4;
pub const HANDLER_WAIT_FOR_EVENT: handler_t = 3;
pub const HANDLER_COMEBACK: handler_t = 2;
pub const HANDLER_FINISHED: handler_t = 1;
pub const HANDLER_GO_ON: handler_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub ptr: *mut libc::c_char,
    pub used: uint32_t,
    pub size: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct burl_parts_t {
    pub scheme: *const buffer,
    pub authority: *const buffer,
    pub port: libc::c_ushort,
    pub path: *const buffer,
    pub query: *const buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cond_match_t {
    pub comp_value: *const buffer,
    pub match_data: *mut pcre2_real_match_data_8,
    pub captures: libc::c_int,
    pub matches: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pcre_keyvalue {
    pub code: *mut pcre2_code_8,
    pub match_data: *mut pcre2_real_match_data_8,
    pub value: buffer,
}
pub type pcre2_code_8 = pcre2_real_code_8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pcre_keyvalue_ctx {
    pub cache: *mut cond_match_t,
    pub burl: *mut burl_parts_t,
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub ovec: *mut libc::c_void,
    pub subject: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pcre_keyvalue_buffer {
    pub kv: *mut pcre_keyvalue,
    pub used: uint32_t,
    pub x0: libc::c_int,
    pub x1: libc::c_int,
    pub cfgidx: libc::c_int,
}
pub type pcre2_match_data_8 = pcre2_real_match_data_8;
pub type pcre2_general_context_8 = pcre2_real_general_context_8;
pub type PCRE2_UCHAR8 = uint8_t;
pub type pcre2_compile_context_8 = pcre2_real_compile_context_8;
pub type PCRE2_SPTR8 = *const PCRE2_UCHAR8;
pub const BURL_ENCODE_PSNDE: burl_recoding_e = 32;
pub const BURL_DECODE_B64U: burl_recoding_e = 128;
pub const BURL_ENCODE_B64U: burl_recoding_e = 64;
pub const BURL_TOLOWER: burl_recoding_e = 1;
pub const BURL_ENCODE_NONE: burl_recoding_e = 4;
pub const BURL_ENCODE_NDE: burl_recoding_e = 16;
pub const BURL_ENCODE_ALL: burl_recoding_e = 8;
pub type pcre2_match_context_8 = pcre2_real_match_context_8;
pub type burl_recoding_e = libc::c_uint;
pub const BURL_TOUPPER: burl_recoding_e = 2;
#[inline]
unsafe extern "C" fn buffer_clear(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn light_isdigit(mut c: libc::c_int) -> libc::c_int {
    return ((c as uint32_t).wrapping_sub('0' as i32 as libc::c_uint)
        <= ('9' as i32 - '0' as i32) as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn buffer_is_unset(mut b: *const buffer) -> libc::c_int {
    return (0 as libc::c_int as libc::c_uint == (*b).used) as libc::c_int;
}
#[inline]
unsafe extern "C" fn buffer_is_blank(mut b: *const buffer) -> libc::c_int {
    return ((*b).used < 2 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn buffer_clen(mut b: *const buffer) -> uint32_t {
    return ((*b).used)
        .wrapping_sub(
            (0 as libc::c_int as libc::c_uint != (*b).used) as libc::c_int
                as libc::c_uint,
        );
}
static mut keyvalue_match_data: *mut pcre2_real_match_data_8 = 0
    as *const pcre2_real_match_data_8 as *mut pcre2_real_match_data_8;
#[no_mangle]
#[cold]
pub unsafe extern "C" fn pcre_keyvalue_buffer_init() -> *mut pcre_keyvalue_buffer {
    let mut kvb: *mut pcre_keyvalue_buffer = 0 as *mut pcre_keyvalue_buffer;
    kvb = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<pcre_keyvalue_buffer>() as libc::c_ulong,
    ) as *mut pcre_keyvalue_buffer;
    if kvb.is_null() {
        ck_assert_failed(
            b"src/keyvalue.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int as libc::c_uint,
            b"((void*)0) != kvb\0" as *const u8 as *const libc::c_char,
        );
    }
    return kvb;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn pcre_keyvalue_buffer_append(
    mut errh: *mut log_error_st,
    mut kvb: *mut pcre_keyvalue_buffer,
    mut key: *const buffer,
    mut value: *const buffer,
    pcre_jit: libc::c_int,
) -> libc::c_int {
    let mut kv: *mut pcre_keyvalue = 0 as *mut pcre_keyvalue;
    if 0 as libc::c_int as libc::c_uint == (*kvb).used & 3 as libc::c_int as libc::c_uint
    {
        (*kvb)
            .kv = realloc(
            (*kvb).kv as *mut libc::c_void,
            (((*kvb).used).wrapping_add(4 as libc::c_int as libc::c_uint)
                as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<pcre_keyvalue>() as libc::c_ulong),
        ) as *mut pcre_keyvalue;
        if ((*kvb).kv).is_null() {
            ck_assert_failed(
                b"src/keyvalue.c\0" as *const u8 as *const libc::c_char,
                61 as libc::c_int as libc::c_uint,
                b"((void*)0) != kvb->kv\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    let fresh0 = (*kvb).used;
    (*kvb).used = ((*kvb).used).wrapping_add(1);
    kv = ((*kvb).kv).offset(fresh0 as isize);
    memcpy(
        &mut (*kv).value as *mut buffer as *mut libc::c_void,
        value as *const libc::c_void,
        ::std::mem::size_of::<buffer>() as libc::c_ulong,
    );
    let mut errcode: libc::c_int = 0;
    let mut erroff: size_t = 0;
    let mut errbuf: [PCRE2_UCHAR8; 1024] = [0; 1024];
    (*kv)
        .code = pcre2_compile_8(
        (*key).ptr as PCRE2_SPTR8,
        buffer_clen(key) as size_t,
        0x80000 as libc::c_uint,
        &mut errcode,
        &mut erroff,
        0 as *mut pcre2_compile_context_8,
    );
    if ((*kv).code).is_null() {
        pcre2_get_error_message_8(
            errcode,
            errbuf.as_mut_ptr(),
            ::std::mem::size_of::<[PCRE2_UCHAR8; 1024]>() as libc::c_ulong,
        );
        log_error(
            errh,
            b"src/keyvalue.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int as libc::c_uint,
            b"pcre2_compile: %s at offset %zu, regex: %s\0" as *const u8
                as *const libc::c_char,
            errbuf.as_mut_ptr() as *mut libc::c_char,
            erroff,
            (*key).ptr,
        );
        return 0 as libc::c_int;
    }
    if pcre_jit != 0 {
        errcode = pcre2_jit_compile_8((*kv).code, 0x1 as libc::c_uint);
        if 0 as libc::c_int != errcode && errcode != -(45 as libc::c_int) {
            pcre2_get_error_message_8(
                errcode,
                errbuf.as_mut_ptr(),
                ::std::mem::size_of::<[PCRE2_UCHAR8; 1024]>() as libc::c_ulong,
            );
            log_error(
                errh,
                b"src/keyvalue.c\0" as *const u8 as *const libc::c_char,
                90 as libc::c_int as libc::c_uint,
                b"pcre2_jit_compile: %s, regex: %s\0" as *const u8
                    as *const libc::c_char,
                errbuf.as_mut_ptr() as *mut libc::c_char,
                (*key).ptr,
            );
        }
    }
    let mut captures: uint32_t = 0;
    errcode = pcre2_pattern_info_8(
        (*kv).code,
        4 as libc::c_int as uint32_t,
        &mut captures as *mut uint32_t as *mut libc::c_void,
    );
    if 0 as libc::c_int != errcode {
        pcre2_get_error_message_8(
            errcode,
            errbuf.as_mut_ptr(),
            ::std::mem::size_of::<[PCRE2_UCHAR8; 1024]>() as libc::c_ulong,
        );
        log_error(
            errh,
            b"src/keyvalue.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int as libc::c_uint,
            b"pcre2_pattern_info: %s, regex: %s\0" as *const u8 as *const libc::c_char,
            errbuf.as_mut_ptr() as *mut libc::c_char,
            (*key).ptr,
        );
        return 0 as libc::c_int;
    } else {
        if captures > 19 as libc::c_int as libc::c_uint {
            log_error(
                errh,
                b"src/keyvalue.c\0" as *const u8 as *const libc::c_char,
                105 as libc::c_int as libc::c_uint,
                b"Too many captures in regex, use (?:...) instead of (...): %s\0"
                    as *const u8 as *const libc::c_char,
                (*key).ptr,
            );
            return 0 as libc::c_int;
        }
    }
    if keyvalue_match_data.is_null() {
        keyvalue_match_data = pcre2_match_data_create_8(
            20 as libc::c_int as uint32_t,
            0 as *mut pcre2_general_context_8,
        );
        if keyvalue_match_data.is_null() {
            ck_assert_failed(
                b"src/keyvalue.c\0" as *const u8 as *const libc::c_char,
                114 as libc::c_int as libc::c_uint,
                b"keyvalue_match_data\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    (*kv).match_data = keyvalue_match_data;
    return 1 as libc::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn pcre_keyvalue_buffer_free(mut kvb: *mut pcre_keyvalue_buffer) {
    let mut kv: *mut pcre_keyvalue = (*kvb).kv;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut used: libc::c_int = (*kvb).used as libc::c_int;
    while i < used {
        if !((*kv).code).is_null() {
            pcre2_code_free_8((*kv).code);
        }
        if !keyvalue_match_data.is_null() {
            pcre2_match_data_free_8(keyvalue_match_data);
            keyvalue_match_data = 0 as *mut pcre2_real_match_data_8;
        }
        i += 1;
        kv = kv.offset(1);
    }
    if !((*kvb).kv).is_null() {
        free((*kvb).kv as *mut libc::c_void);
    }
    free(kvb as *mut libc::c_void);
}
unsafe extern "C" fn pcre_keyvalue_buffer_append_match(
    mut b: *mut buffer,
    mut ctx: *const pcre_keyvalue_ctx,
    mut num: libc::c_uint,
    mut flags: libc::c_int,
) {
    if num < (*ctx).n as libc::c_uint {
        let mut ovec: *const size_t = (*ctx).ovec as *mut size_t;
        num <<= 1 as libc::c_int;
        let off: size_t = *ovec.offset(num as isize);
        let len: size_t = (*ovec
            .offset(num.wrapping_add(1 as libc::c_int as libc::c_uint) as isize))
            .wrapping_sub(off);
        burl_append(b, ((*ctx).subject).offset(off as isize), len, flags);
    }
}
unsafe extern "C" fn pcre_keyvalue_buffer_append_ctxmatch(
    mut b: *mut buffer,
    mut ctx: *const pcre_keyvalue_ctx,
    mut num: libc::c_uint,
    mut flags: libc::c_int,
) {
    let cache: *const cond_match_t = (*ctx).cache;
    if cache.is_null() {
        return;
    }
    if num < (*cache).captures as libc::c_uint {
        let mut ovec: *const size_t = (*cache).matches as *mut size_t;
        num <<= 1 as libc::c_int;
        let off: size_t = *ovec.offset(num as isize);
        let len: size_t = (*ovec
            .offset(num.wrapping_add(1 as libc::c_int as libc::c_uint) as isize))
            .wrapping_sub(off);
        burl_append(b, ((*(*cache).comp_value).ptr).offset(off as isize), len, flags);
    }
}
unsafe extern "C" fn pcre_keyvalue_buffer_subst_ext(
    mut b: *mut buffer,
    mut pattern: *const libc::c_char,
    mut ctx: *const pcre_keyvalue_ctx,
) -> libc::c_int {
    let mut p: *const libc::c_uchar = (pattern as *mut libc::c_uchar)
        .offset(2 as libc::c_int as isize);
    let mut flags: libc::c_int = 0 as libc::c_int;
    while light_isdigit(*p as libc::c_int) == 0 && *p as libc::c_int != '}' as i32
        && *p as libc::c_int != '\u{0}' as i32
    {
        if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'e' as i32
            && *p.offset(1 as libc::c_int as isize) as libc::c_int == 's' as i32
            && *p.offset(2 as libc::c_int as isize) as libc::c_int == 'c' as i32
        {
            p = p.offset(3 as libc::c_int as isize);
            if *p.offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32 {
                flags |= BURL_ENCODE_ALL as libc::c_int;
                p = p.offset(1 as libc::c_int as isize);
            } else if 0 as libc::c_int
                    == strncmp(
                        p as *const libc::c_char,
                        b"ape:\0" as *const u8 as *const libc::c_char,
                        4 as libc::c_int as libc::c_ulong,
                    )
                {
                flags |= BURL_ENCODE_ALL as libc::c_int;
                p = p.offset(4 as libc::c_int as isize);
            } else if 0 as libc::c_int
                    == strncmp(
                        p as *const libc::c_char,
                        b"nde:\0" as *const u8 as *const libc::c_char,
                        4 as libc::c_int as libc::c_ulong,
                    )
                {
                flags |= BURL_ENCODE_NDE as libc::c_int;
                p = p.offset(4 as libc::c_int as isize);
            } else if 0 as libc::c_int
                    == strncmp(
                        p as *const libc::c_char,
                        b"psnde:\0" as *const u8 as *const libc::c_char,
                        6 as libc::c_int as libc::c_ulong,
                    )
                {
                flags |= BURL_ENCODE_PSNDE as libc::c_int;
                p = p.offset(6 as libc::c_int as isize);
            } else {
                p = strchr(p as *const libc::c_char, ':' as i32) as *const libc::c_uchar;
                if p.is_null() {
                    return -(1 as libc::c_int);
                }
                p = p.offset(1);
            }
        } else if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'n' as i32
                && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'o' as i32
            {
            p = p.offset(2 as libc::c_int as isize);
            if 0 as libc::c_int
                == strncmp(
                    p as *const libc::c_char,
                    b"esc:\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                )
            {
                flags |= BURL_ENCODE_NONE as libc::c_int;
                p = p.offset(4 as libc::c_int as isize);
            } else if 0 as libc::c_int
                    == strncmp(
                        p as *const libc::c_char,
                        b"escape:\0" as *const u8 as *const libc::c_char,
                        7 as libc::c_int as libc::c_ulong,
                    )
                {
                flags |= BURL_ENCODE_NONE as libc::c_int;
                p = p.offset(7 as libc::c_int as isize);
            } else {
                p = strchr(p as *const libc::c_char, ':' as i32) as *const libc::c_uchar;
                if p.is_null() {
                    return -(1 as libc::c_int);
                }
                p = p.offset(1);
            }
        } else if *p.offset(0 as libc::c_int as isize) as libc::c_int == 't' as i32
                && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'o' as i32
            {
            p = p.offset(2 as libc::c_int as isize);
            if 0 as libc::c_int
                == strncmp(
                    p as *const libc::c_char,
                    b"lower:\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                )
            {
                flags |= BURL_TOLOWER as libc::c_int;
                p = p.offset(6 as libc::c_int as isize);
            } else if 0 as libc::c_int
                    == strncmp(
                        p as *const libc::c_char,
                        b"upper:\0" as *const u8 as *const libc::c_char,
                        6 as libc::c_int as libc::c_ulong,
                    )
                {
                flags |= BURL_TOLOWER as libc::c_int;
                p = p.offset(6 as libc::c_int as isize);
            } else {
                p = strchr(p as *const libc::c_char, ':' as i32) as *const libc::c_uchar;
                if p.is_null() {
                    return -(1 as libc::c_int);
                }
                p = p.offset(1);
            }
        } else if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'u' as i32
                && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'r' as i32
                && *p.offset(2 as libc::c_int as isize) as libc::c_int == 'l' as i32
                && *p.offset(3 as libc::c_int as isize) as libc::c_int == '.' as i32
            {
            let burl: *const burl_parts_t = (*ctx).burl;
            p = p.offset(4 as libc::c_int as isize);
            if 0 as libc::c_int
                == strncmp(
                    p as *const libc::c_char,
                    b"scheme}\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as libc::c_ulong,
                )
            {
                if !((*burl).scheme).is_null() {
                    burl_append(
                        b,
                        (*(*burl).scheme).ptr,
                        buffer_clen((*burl).scheme) as size_t,
                        flags,
                    );
                }
                p = p.offset(6 as libc::c_int as isize);
            } else if 0 as libc::c_int
                    == strncmp(
                        p as *const libc::c_char,
                        b"authority}\0" as *const u8 as *const libc::c_char,
                        10 as libc::c_int as libc::c_ulong,
                    )
                {
                if !((*burl).authority).is_null() {
                    burl_append(
                        b,
                        (*(*burl).authority).ptr,
                        buffer_clen((*burl).authority) as size_t,
                        flags,
                    );
                }
                p = p.offset(9 as libc::c_int as isize);
            } else if 0 as libc::c_int
                    == strncmp(
                        p as *const libc::c_char,
                        b"port}\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    )
                {
                buffer_append_int(b, (*burl).port as libc::c_int as intmax_t);
                p = p.offset(4 as libc::c_int as isize);
            } else if 0 as libc::c_int
                    == strncmp(
                        p as *const libc::c_char,
                        b"path}\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    )
                {
                let target: *const buffer = (*burl).path;
                let len: uint32_t = buffer_clen(target);
                let ptr: *const libc::c_char = (*target).ptr;
                let qmark: *const libc::c_char = memchr(
                    ptr as *const libc::c_void,
                    '?' as i32,
                    len as libc::c_ulong,
                ) as *const libc::c_char;
                burl_append(
                    b,
                    ptr,
                    (if !qmark.is_null() {
                        qmark.offset_from(ptr) as libc::c_long as uint32_t
                    } else {
                        len
                    }) as size_t,
                    flags,
                );
                p = p.offset(4 as libc::c_int as isize);
            } else if 0 as libc::c_int
                    == strncmp(
                        p as *const libc::c_char,
                        b"query}\0" as *const u8 as *const libc::c_char,
                        6 as libc::c_int as libc::c_ulong,
                    )
                {
                if !((*burl).query).is_null() {
                    burl_append(
                        b,
                        (*(*burl).query).ptr,
                        buffer_clen((*burl).query) as size_t,
                        flags,
                    );
                }
                p = p.offset(5 as libc::c_int as isize);
            } else {
                p = strchr(p as *const libc::c_char, '}' as i32) as *const libc::c_uchar;
                if p.is_null() {
                    return -(1 as libc::c_int);
                }
            }
            break;
        } else if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'q' as i32
                && *p.offset(1 as libc::c_int as isize) as libc::c_int == 's' as i32
                && *p.offset(2 as libc::c_int as isize) as libc::c_int == 'a' as i32
                && *p.offset(3 as libc::c_int as isize) as libc::c_int == '}' as i32
            {
            let mut qs: *const buffer = (*(*ctx).burl).query;
            if !qs.is_null() && buffer_is_unset(qs) == 0 {
                if !(strchr((*b).ptr, '?' as i32)).is_null() {
                    if buffer_is_blank(qs) == 0 {
                        buffer_append_string_len(
                            b,
                            b"&\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        );
                    }
                } else {
                    buffer_append_string_len(
                        b,
                        b"?\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    );
                }
                burl_append(b, (*qs).ptr, buffer_clen(qs) as size_t, flags);
            }
            p = p.offset(3 as libc::c_int as isize);
            break;
        } else if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'e' as i32
                && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'n' as i32
                && *p.offset(2 as libc::c_int as isize) as libc::c_int == 'c' as i32
                && 0 as libc::c_int
                    == strncmp(
                        (p as *const libc::c_char).offset(3 as libc::c_int as isize),
                        b"b64u:\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    )
            {
            flags |= BURL_ENCODE_B64U as libc::c_int;
            p = p.offset(8 as libc::c_int as isize);
        } else if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'd' as i32
                && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'e' as i32
                && *p.offset(2 as libc::c_int as isize) as libc::c_int == 'c' as i32
                && 0 as libc::c_int
                    == strncmp(
                        (p as *const libc::c_char).offset(3 as libc::c_int as isize),
                        b"b64u:\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    )
            {
            flags |= BURL_DECODE_B64U as libc::c_int;
            p = p.offset(8 as libc::c_int as isize);
        } else {
            p = p.offset(1);
        }
    }
    if *p as libc::c_int == '\u{0}' as i32 {
        return -(1 as libc::c_int);
    }
    if *p as libc::c_int != '}' as i32 {
        let mut num: libc::c_uint = (*p as libc::c_int - '0' as i32) as libc::c_uint;
        p = p.offset(1);
        if light_isdigit(*p as libc::c_int) != 0 {
            let fresh1 = p;
            p = p.offset(1);
            num = num
                .wrapping_mul(10 as libc::c_int as libc::c_uint)
                .wrapping_add((*fresh1 as libc::c_int - '0' as i32) as libc::c_uint);
        }
        if *p as libc::c_int != '}' as i32 {
            p = strchr(p as *const libc::c_char, '}' as i32) as *const libc::c_uchar;
            if p.is_null() {
                return -(1 as libc::c_int);
            }
        }
        if 0 as libc::c_int == flags {
            flags = BURL_ENCODE_PSNDE as libc::c_int;
        }
        if *pattern.offset(0 as libc::c_int as isize) as libc::c_int == '$' as i32 {
            pcre_keyvalue_buffer_append_match(b, ctx, num, flags);
        } else {
            pcre_keyvalue_buffer_append_ctxmatch(b, ctx, num, flags);
        };
    }
    return (p
        .offset(1 as libc::c_int as isize)
        .offset_from(pattern as *mut libc::c_uchar) as libc::c_long
        - 2 as libc::c_int as libc::c_long) as libc::c_int;
}
unsafe extern "C" fn pcre_keyvalue_buffer_subst(
    mut b: *mut buffer,
    mut patternb: *const buffer,
    mut ctx: *const pcre_keyvalue_ctx,
) {
    let mut pattern: *const libc::c_char = (*patternb).ptr;
    let pattern_len: size_t = buffer_clen(patternb) as size_t;
    let mut start: size_t = 0 as libc::c_int as size_t;
    buffer_clear(b);
    let mut k: size_t = 0 as libc::c_int as size_t;
    while k.wrapping_add(1 as libc::c_int as libc::c_ulong) < pattern_len {
        if *pattern.offset(k as isize) as libc::c_int == '$' as i32
            || *pattern.offset(k as isize) as libc::c_int == '%' as i32
        {
            buffer_append_string_len(
                b,
                pattern.offset(start as isize),
                k.wrapping_sub(start),
            );
            if *pattern
                .offset(k.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int == '{' as i32
            {
                let mut num: libc::c_int = pcre_keyvalue_buffer_subst_ext(
                    b,
                    pattern.offset(k as isize),
                    ctx,
                );
                if num < 0 as libc::c_int {
                    return;
                }
                k = (k as libc::c_ulong).wrapping_add(num as size_t) as size_t as size_t;
            } else if light_isdigit(
                    *(pattern as *mut libc::c_uchar)
                        .offset(
                            k.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int,
                ) != 0
                {
                let mut num_0: libc::c_uint = (*pattern
                    .offset(k.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_uint)
                    .wrapping_sub('0' as i32 as libc::c_uint);
                if *pattern.offset(k as isize) as libc::c_int == '$' as i32 {
                    pcre_keyvalue_buffer_append_match(b, ctx, num_0, 0 as libc::c_int);
                } else {
                    pcre_keyvalue_buffer_append_ctxmatch(
                        b,
                        ctx,
                        num_0,
                        0 as libc::c_int,
                    );
                };
            } else {
                buffer_append_string_len(
                    b,
                    pattern.offset(k as isize),
                    (if *pattern.offset(k as isize) as libc::c_int
                        == *pattern
                            .offset(
                                k.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int
                    {
                        1 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) as size_t,
                );
            }
            k = k.wrapping_add(1);
            start = k.wrapping_add(1 as libc::c_int as libc::c_ulong);
        }
        k = k.wrapping_add(1);
    }
    buffer_append_string_len(
        b,
        pattern.offset(start as isize),
        pattern_len.wrapping_sub(start),
    );
}
#[no_mangle]
pub unsafe extern "C" fn pcre_keyvalue_buffer_process(
    mut kvb: *const pcre_keyvalue_buffer,
    mut ctx: *mut pcre_keyvalue_ctx,
    mut input: *const buffer,
    mut result: *mut buffer,
) -> handler_t {
    let mut kv: *const pcre_keyvalue = (*kvb).kv;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut used: libc::c_int = (*kvb).used as libc::c_int;
    while i < used {
        let mut n: libc::c_int = pcre2_match_8(
            (*kv).code,
            (*input).ptr as PCRE2_SPTR8,
            buffer_clen(input) as size_t,
            0 as libc::c_int as size_t,
            0 as libc::c_int as uint32_t,
            (*kv).match_data,
            0 as *mut pcre2_match_context_8,
        );
        if n < 0 as libc::c_int {
            if n != -(1 as libc::c_int) {
                return HANDLER_ERROR;
            }
        } else if buffer_is_blank(&(*kv).value) != 0 {
            (*ctx).m = i;
            return HANDLER_GO_ON;
        } else {
            (*ctx).m = i;
            (*ctx).n = n;
            (*ctx).subject = (*input).ptr;
            (*ctx)
                .ovec = pcre2_get_ovector_pointer_8((*kv).match_data)
                as *mut libc::c_void;
            pcre_keyvalue_buffer_subst(result, &(*kv).value, ctx);
            return HANDLER_FINISHED;
        }
        i += 1;
        kv = kv.offset(1);
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn pcre_keyvalue_burl_percent_toupper(mut b: *mut buffer) {
    let s: *const libc::c_uchar = (*b).ptr as *mut libc::c_uchar;
    let used: libc::c_int = buffer_clen(b) as libc::c_int;
    let mut n1: libc::c_uint = 0;
    let mut n2: libc::c_uint = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < used {
        if *s.offset(i as isize) as libc::c_int == '%' as i32
            && {
                n1 = (*s.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    - '0' as i32) as libc::c_uint;
                n1 <= 9 as libc::c_int as libc::c_uint
                    || {
                        n1 = ((*s.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                            & 0xdf as libc::c_int) - 'A' as i32) as libc::c_uint;
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
                        n2 = ((*s.offset((i + 2 as libc::c_int) as isize) as libc::c_int
                            & 0xdf as libc::c_int) - 'A' as i32) as libc::c_uint;
                        (if n2 <= 5 as libc::c_int as libc::c_uint {
                            n2 = n2.wrapping_add(10 as libc::c_int as libc::c_uint);
                            n2
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) != 0
                    }
            }
        {
            if *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int >= 'a' as i32 {
                let ref mut fresh2 = *((*b).ptr).offset((i + 1 as libc::c_int) as isize);
                *fresh2 = (*fresh2 as libc::c_int & 0xdf as libc::c_int) as libc::c_char;
            }
            if *s.offset((i + 2 as libc::c_int) as isize) as libc::c_int >= 'a' as i32 {
                let ref mut fresh3 = *((*b).ptr).offset((i + 2 as libc::c_int) as isize);
                *fresh3 = (*fresh3 as libc::c_int & 0xdf as libc::c_int) as libc::c_char;
            }
            i += 2 as libc::c_int;
        }
        i += 1;
    }
}
unsafe extern "C" fn pcre_keyvalue_burl_percent_percent_toupper(mut b: *mut buffer) {
    let s: *const libc::c_uchar = (*b).ptr as *mut libc::c_uchar;
    let used: libc::c_int = buffer_clen(b) as libc::c_int;
    let mut n1: libc::c_uint = 0;
    let mut n2: libc::c_uint = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < used {
        if *s.offset(i as isize) as libc::c_int == '%' as i32
            && *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '%' as i32
            && {
                n1 = (*s.offset((i + 2 as libc::c_int) as isize) as libc::c_int
                    - '0' as i32) as libc::c_uint;
                n1 <= 9 as libc::c_int as libc::c_uint
                    || {
                        n1 = ((*s.offset((i + 2 as libc::c_int) as isize) as libc::c_int
                            & 0xdf as libc::c_int) - 'A' as i32) as libc::c_uint;
                        (if n1 <= 5 as libc::c_int as libc::c_uint {
                            n1 = n1.wrapping_add(10 as libc::c_int as libc::c_uint);
                            n1
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) != 0
                    }
            }
            && {
                n2 = (*s.offset((i + 3 as libc::c_int) as isize) as libc::c_int
                    - '0' as i32) as libc::c_uint;
                n2 <= 9 as libc::c_int as libc::c_uint
                    || {
                        n2 = ((*s.offset((i + 3 as libc::c_int) as isize) as libc::c_int
                            & 0xdf as libc::c_int) - 'A' as i32) as libc::c_uint;
                        (if n2 <= 5 as libc::c_int as libc::c_uint {
                            n2 = n2.wrapping_add(10 as libc::c_int as libc::c_uint);
                            n2
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) != 0
                    }
            }
        {
            if *s.offset((i + 2 as libc::c_int) as isize) as libc::c_int >= 'a' as i32 {
                let ref mut fresh4 = *((*b).ptr).offset((i + 2 as libc::c_int) as isize);
                *fresh4 = (*fresh4 as libc::c_int & 0xdf as libc::c_int) as libc::c_char;
            }
            if *s.offset((i + 3 as libc::c_int) as isize) as libc::c_int >= 'a' as i32 {
                let ref mut fresh5 = *((*b).ptr).offset((i + 3 as libc::c_int) as isize);
                *fresh5 = (*fresh5 as libc::c_int & 0xdf as libc::c_int) as libc::c_char;
            }
            i += 3 as libc::c_int;
        }
        i += 1;
    }
}
static mut hex_chars_uc: [libc::c_char; 17] = unsafe {
    *::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"0123456789ABCDEF\0")
};
unsafe extern "C" fn pcre_keyvalue_burl_percent_high_UTF8(
    mut b: *mut buffer,
    mut t: *mut buffer,
) {
    let s: *const libc::c_uchar = (*b).ptr as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let used: libc::c_int = buffer_clen(b) as libc::c_int;
    let mut count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut j: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < used {
        if *s.offset(i as isize) as libc::c_int > 0x7f as libc::c_int {
            count = count.wrapping_add(1);
        }
        i += 1;
    }
    if 0 as libc::c_int as libc::c_uint == count {
        return;
    }
    p = buffer_string_prepare_copy(
        t,
        (used as libc::c_uint)
            .wrapping_add(count.wrapping_mul(2 as libc::c_int as libc::c_uint)) as size_t,
    ) as *mut libc::c_uchar;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < used {
        if *s.offset(i_0 as isize) as libc::c_int <= 0x7f as libc::c_int {
            *p.offset(j as isize) = *s.offset(i_0 as isize);
        } else {
            *p.offset(j as isize) = '%' as i32 as libc::c_uchar;
            j = j.wrapping_add(1);
            *p
                .offset(
                    j as isize,
                ) = hex_chars_uc[(*s.offset(i_0 as isize) as libc::c_int
                >> 4 as libc::c_int & 0xf as libc::c_int) as usize] as libc::c_uchar;
            j = j.wrapping_add(1);
            *p
                .offset(
                    j as isize,
                ) = hex_chars_uc[(*s.offset(i_0 as isize) as libc::c_int
                & 0xf as libc::c_int) as usize] as libc::c_uchar;
        }
        i_0 += 1;
        j = j.wrapping_add(1);
    }
    buffer_copy_string_len(b, p as *mut libc::c_char, j as size_t);
}
unsafe extern "C" fn pcre_keyvalue_burl_percent_percent_high_UTF8(
    mut b: *mut buffer,
    mut t: *mut buffer,
) {
    let s: *const libc::c_uchar = (*b).ptr as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let used: libc::c_int = buffer_clen(b) as libc::c_int;
    let mut count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut j: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < used {
        if *s.offset(i as isize) as libc::c_int > 0x7f as libc::c_int {
            count = count.wrapping_add(1);
        }
        i += 1;
    }
    if 0 as libc::c_int as libc::c_uint == count {
        return;
    }
    p = buffer_string_prepare_copy(
        t,
        (used as libc::c_uint)
            .wrapping_add(count.wrapping_mul(3 as libc::c_int as libc::c_uint)) as size_t,
    ) as *mut libc::c_uchar;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < used {
        if *s.offset(i_0 as isize) as libc::c_int <= 0x7f as libc::c_int {
            *p.offset(j as isize) = *s.offset(i_0 as isize);
        } else {
            *p.offset(j as isize) = '%' as i32 as libc::c_uchar;
            j = j.wrapping_add(1);
            *p.offset(j as isize) = '%' as i32 as libc::c_uchar;
            j = j.wrapping_add(1);
            *p
                .offset(
                    j as isize,
                ) = hex_chars_uc[(*s.offset(i_0 as isize) as libc::c_int
                >> 4 as libc::c_int & 0xf as libc::c_int) as usize] as libc::c_uchar;
            j = j.wrapping_add(1);
            *p
                .offset(
                    j as isize,
                ) = hex_chars_uc[(*s.offset(i_0 as isize) as libc::c_int
                & 0xf as libc::c_int) as usize] as libc::c_uchar;
        }
        i_0 += 1;
        j = j.wrapping_add(1);
    }
    buffer_copy_string_len(b, p as *mut libc::c_char, j as size_t);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn pcre_keyvalue_burl_normalize_key(
    mut k: *mut buffer,
    mut t: *mut buffer,
) {
    pcre_keyvalue_burl_percent_toupper(k);
    pcre_keyvalue_burl_percent_high_UTF8(k, t);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn pcre_keyvalue_burl_normalize_value(
    mut v: *mut buffer,
    mut t: *mut buffer,
) {
    pcre_keyvalue_burl_percent_percent_toupper(v);
    pcre_keyvalue_burl_percent_percent_high_UTF8(v, t);
}
