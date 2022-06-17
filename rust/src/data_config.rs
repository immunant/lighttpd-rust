use ::libc;
extern "C" {
    pub type fdlog_st;
    pub type pcre2_real_match_data_8;
    pub type pcre2_real_code_8;
    pub type pcre2_real_compile_context_8;
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn array_init(n: uint32_t) -> *mut array;
    fn array_copy_array(dst: *mut array, src: *const array);
    fn array_free(a: *mut array);
    fn vector_free(data: *mut libc::c_void);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn pcre2_compile_8(
        _: PCRE2_SPTR8,
        _: size_t,
        _: uint32_t,
        _: *mut libc::c_int,
        _: *mut size_t,
        _: *mut pcre2_compile_context_8,
    ) -> *mut pcre2_code_8;
    fn pcre2_jit_compile_8(_: *mut pcre2_code_8, _: uint32_t) -> libc::c_int;
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
    fn pcre2_code_free_8(_: *mut pcre2_code_8);
    fn log_error(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        fmt: *const libc::c_char,
        _: ...
    );
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub ptr: *mut libc::c_char,
    pub used: uint32_t,
    pub size: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_unset {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
}
pub type data_type_t = libc::c_uint;
pub const TYPE_OTHER: data_type_t = 4;
pub const TYPE_CONFIG: data_type_t = 3;
pub const TYPE_INTEGER: data_type_t = 2;
pub const TYPE_ARRAY: data_type_t = 1;
pub const TYPE_STRING: data_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_methods {
    pub copy: Option::<unsafe extern "C" fn(*const data_unset) -> *mut data_unset>,
    pub free: Option::<unsafe extern "C" fn(*mut data_unset) -> ()>,
    pub insert_dup: Option::<
        unsafe extern "C" fn(*mut data_unset, *mut data_unset) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array {
    pub data: *mut *mut data_unset,
    pub sorted: *mut *mut data_unset,
    pub used: uint32_t,
    pub size: uint32_t,
}
pub type log_error_st = fdlog_st;
pub type config_cond_t = libc::c_uint;
pub const CONFIG_COND_ELSE: config_cond_t = 5;
pub const CONFIG_COND_NOMATCH: config_cond_t = 4;
pub const CONFIG_COND_NE: config_cond_t = 3;
pub const CONFIG_COND_MATCH: config_cond_t = 2;
pub const CONFIG_COND_EQ: config_cond_t = 1;
pub const CONFIG_COND_UNSET: config_cond_t = 0;
pub type comp_key_t = libc::c_uint;
pub const COMP_LAST_ELEMENT: comp_key_t = 13;
pub const COMP_HTTP_REQUEST_HEADER: comp_key_t = 12;
pub const COMP_HTTP_REQUEST_METHOD: comp_key_t = 11;
pub const COMP_HTTP_SCHEME: comp_key_t = 10;
pub const COMP_HTTP_QUERY_STRING: comp_key_t = 9;
pub const COMP_HTTP_REMOTE_IP: comp_key_t = 8;
pub const COMP_HTTP_COOKIE: comp_key_t = 7;
pub const COMP_HTTP_LANGUAGE: comp_key_t = 6;
pub const COMP_HTTP_USER_AGENT: comp_key_t = 5;
pub const COMP_HTTP_REFERER: comp_key_t = 4;
pub const COMP_HTTP_HOST: comp_key_t = 3;
pub const COMP_HTTP_URL: comp_key_t = 2;
pub const COMP_SERVER_SOCKET: comp_key_t = 1;
pub const COMP_UNSET: comp_key_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_config {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
    pub context_ndx: libc::c_int,
    pub comp: comp_key_t,
    pub cond: config_cond_t,
    pub parent: *mut data_config,
    pub prev: *mut data_config,
    pub next: *mut data_config,
    pub string: buffer,
    pub code: *mut libc::c_void,
    pub match_data: *mut pcre2_real_match_data_8,
    pub capture_idx: libc::c_int,
    pub ext: libc::c_int,
    pub comp_tag: buffer,
    pub comp_key: *const libc::c_char,
    pub children: vector_config_weak,
    pub value: *mut array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vector_config_weak {
    pub data: *mut *mut data_config,
    pub used: size_t,
    pub size: size_t,
}
pub type pcre2_code_8 = pcre2_real_code_8;
pub type PCRE2_UCHAR8 = uint8_t;
pub type pcre2_compile_context_8 = pcre2_real_compile_context_8;
pub type PCRE2_SPTR8 = *const PCRE2_UCHAR8;
#[inline]
unsafe extern "C" fn buffer_is_unset(mut b: *const buffer) -> libc::c_int {
    return (0 as libc::c_int as libc::c_uint == (*b).used) as libc::c_int;
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
unsafe extern "C" fn buffer_copy_buffer(mut b: *mut buffer, mut src: *const buffer) {
    buffer_copy_string_len(b, (*src).ptr, buffer_clen(src) as size_t);
}
#[inline]
unsafe extern "C" fn vector_config_weak_init(mut v: *mut vector_config_weak) {
    (*v).data = 0 as *mut *mut data_config;
    (*v).size = 0 as libc::c_int as size_t;
    (*v).used = (*v).size;
}
#[inline]
unsafe extern "C" fn vector_config_weak_clear(mut v: *mut vector_config_weak) {
    if (::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*mut data_config) -> ()>,
    >(0 as *mut libc::c_void))
        .is_some()
    {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < (*v).used {
            (::std::mem::transmute::<
                *mut libc::c_void,
                Option::<unsafe extern "C" fn(*mut data_config) -> ()>,
            >(0 as *mut libc::c_void))
                .expect("non-null function pointer")(*((*v).data).offset(i as isize));
            i = i.wrapping_add(1);
        }
    }
    vector_free((*v).data as *mut libc::c_void);
    vector_config_weak_init(v);
}
#[cold]
unsafe extern "C" fn data_config_copy(mut s: *const data_unset) -> *mut data_unset {
    let mut src: *mut data_config = s as *mut data_config;
    let mut ds: *mut data_config = data_config_init();
    (*ds).comp = (*src).comp;
    if buffer_is_unset(&mut (*src).key) == 0 {
        buffer_copy_buffer(&mut (*ds).key, &mut (*src).key);
        (*ds)
            .comp_key = ((*ds).key.ptr)
            .offset(
                ((*src).comp_key).offset_from((*src).key.ptr) as libc::c_long as isize,
            );
    }
    buffer_copy_buffer(&mut (*ds).comp_tag, &mut (*src).comp_tag);
    array_copy_array((*ds).value, (*src).value);
    return ds as *mut data_unset;
}
#[cold]
unsafe extern "C" fn data_config_free(mut d: *mut data_unset) {
    let mut ds: *mut data_config = d as *mut data_config;
    free((*ds).key.ptr as *mut libc::c_void);
    free((*ds).comp_tag.ptr as *mut libc::c_void);
    array_free((*ds).value);
    vector_config_weak_clear(&mut (*ds).children);
    free((*ds).string.ptr as *mut libc::c_void);
    if !((*ds).code).is_null() {
        pcre2_code_free_8((*ds).code as *mut pcre2_code_8);
    }
    free(d as *mut libc::c_void);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn data_config_init() -> *mut data_config {
    static mut config_fn: data_methods = unsafe {
        {
            let mut init = data_methods {
                copy: Some(
                    data_config_copy
                        as unsafe extern "C" fn(*const data_unset) -> *mut data_unset,
                ),
                free: Some(
                    data_config_free as unsafe extern "C" fn(*mut data_unset) -> (),
                ),
                insert_dup: None,
            };
            init
        }
    };
    let mut ds: *mut data_config = 0 as *mut data_config;
    ds = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<data_config>() as libc::c_ulong,
    ) as *mut data_config;
    if ds.is_null() {
        ck_assert_failed(
            b"src/data_config.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int as libc::c_uint,
            b"ds\0" as *const u8 as *const libc::c_char,
        );
    }
    (*ds).comp_key = b"\0" as *const u8 as *const libc::c_char;
    (*ds).value = array_init(4 as libc::c_int as uint32_t);
    vector_config_weak_init(&mut (*ds).children);
    (*ds).type_0 = TYPE_CONFIG;
    (*ds).fn_0 = &config_fn;
    return ds;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn data_config_pcre_compile(
    dc: *mut data_config,
    pcre_jit: libc::c_int,
    errh: *mut log_error_st,
) -> libc::c_int {
    let mut errcode: libc::c_int = 0;
    let mut erroff: size_t = 0;
    let mut errbuf: [PCRE2_UCHAR8; 1024] = [0; 1024];
    (*dc)
        .code = pcre2_compile_8(
        (*dc).string.ptr as PCRE2_SPTR8,
        buffer_clen(&mut (*dc).string) as size_t,
        0x80000 as libc::c_uint,
        &mut errcode,
        &mut erroff,
        0 as *mut pcre2_compile_context_8,
    ) as *mut libc::c_void;
    if ((*dc).code).is_null() {
        pcre2_get_error_message_8(
            errcode,
            errbuf.as_mut_ptr(),
            ::std::mem::size_of::<[PCRE2_UCHAR8; 1024]>() as libc::c_ulong,
        );
        log_error(
            errh,
            b"src/data_config.c\0" as *const u8 as *const libc::c_char,
            94 as libc::c_int as libc::c_uint,
            b"pcre2_compile: %s at offset %zu, regex: %s\0" as *const u8
                as *const libc::c_char,
            errbuf.as_mut_ptr() as *mut libc::c_char,
            erroff,
            (*dc).string.ptr,
        );
        return 0 as libc::c_int;
    }
    if pcre_jit != 0 {
        errcode = pcre2_jit_compile_8(
            (*dc).code as *mut pcre2_code_8,
            0x1 as libc::c_uint,
        );
        if 0 as libc::c_int != errcode && errcode != -(45 as libc::c_int) {
            pcre2_get_error_message_8(
                errcode,
                errbuf.as_mut_ptr(),
                ::std::mem::size_of::<[PCRE2_UCHAR8; 1024]>() as libc::c_ulong,
            );
            log_error(
                errh,
                b"src/data_config.c\0" as *const u8 as *const libc::c_char,
                104 as libc::c_int as libc::c_uint,
                b"pcre2_jit_compile: %s, regex: %s\0" as *const u8
                    as *const libc::c_char,
                errbuf.as_mut_ptr() as *mut libc::c_char,
                (*dc).string.ptr,
            );
        }
    }
    let mut captures: uint32_t = 0;
    errcode = pcre2_pattern_info_8(
        (*dc).code as *const pcre2_code_8,
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
            b"src/data_config.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int as libc::c_uint,
            b"pcre2_pattern_info: %s, regex: %s\0" as *const u8 as *const libc::c_char,
            errbuf.as_mut_ptr() as *mut libc::c_char,
            (*dc).string.ptr,
        );
        return 0 as libc::c_int;
    } else {
        if captures > 9 as libc::c_int as libc::c_uint {
            log_error(
                errh,
                b"src/data_config.c\0" as *const u8 as *const libc::c_char,
                120 as libc::c_int as libc::c_uint,
                b"Too many captures in regex, use (?:...) instead of (...): %s\0"
                    as *const u8 as *const libc::c_char,
                (*dc).string.ptr,
            );
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
