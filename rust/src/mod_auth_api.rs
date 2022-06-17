use ::libc;
extern "C" {
    pub type request_st;
    fn array_free_data(a: *mut array);
    fn buffer_eq_slen(
        b: *const buffer,
        s: *const libc::c_char,
        slen: size_t,
    ) -> libc::c_int;
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn http_header_env_set(
        r: *mut request_st,
        k: *const libc::c_char,
        klen: uint32_t,
        v: *const libc::c_char,
        vlen: uint32_t,
    );
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_auth_backend_t {
    pub name: *const libc::c_char,
    pub basic: Option::<
        unsafe extern "C" fn(
            *mut request_st,
            *mut libc::c_void,
            *const http_auth_require_t,
            *const buffer,
            *const libc::c_char,
        ) -> handler_t,
    >,
    pub digest: Option::<
        unsafe extern "C" fn(
            *mut request_st,
            *mut libc::c_void,
            *mut http_auth_info_t,
        ) -> handler_t,
    >,
    pub p_d: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_auth_info_t {
    pub dalgo: libc::c_int,
    pub dlen: libc::c_uint,
    pub username: *const libc::c_char,
    pub ulen: size_t,
    pub realm: *const libc::c_char,
    pub rlen: size_t,
    pub userhash: libc::c_int,
    pub digest: [libc::c_uchar; 32],
    pub userbuf: [libc::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_auth_require_t {
    pub scheme: *const http_auth_scheme_t,
    pub realm: *const buffer,
    pub nonce_secret: *const buffer,
    pub valid_user: uint8_t,
    pub userhash: uint8_t,
    pub algorithm: libc::c_int,
    pub user: array,
    pub group: array,
    pub host: array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_auth_scheme_t {
    pub name: *const libc::c_char,
    pub checkfn: Option::<
        unsafe extern "C" fn(
            *mut request_st,
            *mut libc::c_void,
            *const http_auth_require_t,
            *const http_auth_backend_t,
        ) -> handler_t,
    >,
    pub p_d: *mut libc::c_void,
}
pub type http_auth_digest_type = libc::c_uint;
pub const HTTP_AUTH_DIGEST_SHA512_256: http_auth_digest_type = 8;
pub const HTTP_AUTH_DIGEST_SHA256: http_auth_digest_type = 4;
pub const HTTP_AUTH_DIGEST_MD5: http_auth_digest_type = 2;
pub const HTTP_AUTH_DIGEST_SESS: http_auth_digest_type = 1;
pub const HTTP_AUTH_DIGEST_NONE: http_auth_digest_type = 0;
static mut http_auth_schemes: [http_auth_scheme_t; 8] = [http_auth_scheme_t {
    name: 0 as *const libc::c_char,
    checkfn: None,
    p_d: 0 as *const libc::c_void as *mut libc::c_void,
}; 8];
#[no_mangle]
#[cold]
pub unsafe extern "C" fn http_auth_scheme_get(
    mut name: *const buffer,
) -> *const http_auth_scheme_t {
    let mut i: libc::c_int = 0 as libc::c_int;
    while !(http_auth_schemes[i as usize].name).is_null()
        && 0 as libc::c_int != strcmp(http_auth_schemes[i as usize].name, (*name).ptr)
    {
        i += 1;
    }
    return if !(http_auth_schemes[i as usize].name).is_null() {
        http_auth_schemes.as_mut_ptr().offset(i as isize)
    } else {
        0 as *mut http_auth_scheme_t
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn http_auth_scheme_set(mut scheme: *const http_auth_scheme_t) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while !(http_auth_schemes[i as usize].name).is_null() {
        i = i.wrapping_add(1);
    }
    if !((i as libc::c_ulong)
        < (::std::mem::size_of::<[http_auth_scheme_t; 8]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<http_auth_scheme_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong))
    {
        ck_assert_failed(
            b"src/mod_auth_api.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int as libc::c_uint,
            b"i<(sizeof(http_auth_schemes)/sizeof(http_auth_scheme_t))-1\0" as *const u8
                as *const libc::c_char,
        );
    }
    memcpy(
        http_auth_schemes.as_mut_ptr().offset(i as isize) as *mut libc::c_void,
        scheme as *const libc::c_void,
        ::std::mem::size_of::<http_auth_scheme_t>() as libc::c_ulong,
    );
}
static mut http_auth_backends: [http_auth_backend_t; 12] = [http_auth_backend_t {
    name: 0 as *const libc::c_char,
    basic: None,
    digest: None,
    p_d: 0 as *const libc::c_void as *mut libc::c_void,
}; 12];
#[no_mangle]
#[cold]
pub unsafe extern "C" fn http_auth_backend_get(
    mut name: *const buffer,
) -> *const http_auth_backend_t {
    let mut i: libc::c_int = 0 as libc::c_int;
    while !(http_auth_backends[i as usize].name).is_null()
        && 0 as libc::c_int != strcmp(http_auth_backends[i as usize].name, (*name).ptr)
    {
        i += 1;
    }
    return if !(http_auth_backends[i as usize].name).is_null() {
        http_auth_backends.as_mut_ptr().offset(i as isize)
    } else {
        0 as *mut http_auth_backend_t
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn http_auth_backend_set(mut backend: *const http_auth_backend_t) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while !(http_auth_backends[i as usize].name).is_null() {
        i = i.wrapping_add(1);
    }
    if !((i as libc::c_ulong)
        < (::std::mem::size_of::<[http_auth_backend_t; 12]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<http_auth_backend_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong))
    {
        ck_assert_failed(
            b"src/mod_auth_api.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            b"i<(sizeof(http_auth_backends)/sizeof(http_auth_backend_t))-1\0"
                as *const u8 as *const libc::c_char,
        );
    }
    memcpy(
        http_auth_backends.as_mut_ptr().offset(i as isize) as *mut libc::c_void,
        backend as *const libc::c_void,
        ::std::mem::size_of::<http_auth_backend_t>() as libc::c_ulong,
    );
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn http_auth_dumbdata_reset() {
    memset(
        http_auth_schemes.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[http_auth_scheme_t; 8]>() as libc::c_ulong,
    );
    memset(
        http_auth_backends.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[http_auth_backend_t; 12]>() as libc::c_ulong,
    );
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn http_auth_require_init() -> *mut http_auth_require_t {
    let mut require: *mut http_auth_require_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<http_auth_require_t>() as libc::c_ulong,
    ) as *mut http_auth_require_t;
    if require.is_null() {
        ck_assert_failed(
            b"src/mod_auth_api.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int as libc::c_uint,
            b"((void*)0) != require\0" as *const u8 as *const libc::c_char,
        );
    }
    return require;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn http_auth_require_free(require: *mut http_auth_require_t) {
    array_free_data(&mut (*require).user);
    array_free_data(&mut (*require).group);
    array_free_data(&mut (*require).host);
    free(require as *mut libc::c_void);
}
unsafe extern "C" fn http_auth_array_contains(
    a: *const array,
    k: *const libc::c_char,
    klen: size_t,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut used: size_t = (*a).used as size_t;
    while i < used {
        if buffer_eq_slen(&mut (**((*a).data).offset(i as isize)).key, k, klen) != 0 {
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn http_auth_match_rules(
    require: *const http_auth_require_t,
    user: *const libc::c_char,
    group: *const libc::c_char,
    host: *const libc::c_char,
) -> libc::c_int {
    if !user.is_null()
        && ((*require).valid_user as libc::c_int != 0
            || http_auth_array_contains(&(*require).user, user, strlen(user)) != 0)
    {
        return 1 as libc::c_int;
    }
    if !group.is_null()
        && http_auth_array_contains(&(*require).group, group, strlen(group)) != 0
    {
        return 1 as libc::c_int;
    }
    if !host.is_null()
        && http_auth_array_contains(&(*require).host, host, strlen(host)) != 0
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn http_auth_setenv(
    r: *mut request_st,
    mut username: *const libc::c_char,
    mut ulen: size_t,
    mut auth_type: *const libc::c_char,
    mut alen: size_t,
) {
    http_header_env_set(
        r,
        b"REMOTE_USER\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
        username,
        ulen as uint32_t,
    );
    http_header_env_set(
        r,
        b"AUTH_TYPE\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
        auth_type,
        alen as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn http_auth_digest_len(mut algo: libc::c_int) -> libc::c_uint {
    if algo
        & (HTTP_AUTH_DIGEST_SHA256 as libc::c_int
            | HTTP_AUTH_DIGEST_SHA512_256 as libc::c_int) != 0
    {
        return 32 as libc::c_int as libc::c_uint;
    }
    if algo & HTTP_AUTH_DIGEST_MD5 as libc::c_int != 0 {
        return 16 as libc::c_int as libc::c_uint;
    }
    return 0 as libc::c_int as libc::c_uint;
}
