use ::libc;
extern "C" {
    pub type request_st;
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
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
}
pub type __uint32_t = libc::c_uint;
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
pub struct http_vhostdb_backend_t {
    pub name: *const libc::c_char,
    pub query: Option::<
        unsafe extern "C" fn(
            *mut request_st,
            *mut libc::c_void,
            *mut buffer,
        ) -> libc::c_int,
    >,
    pub p_d: *mut libc::c_void,
}
static mut http_vhostdb_backends: [http_vhostdb_backend_t; 8] = [http_vhostdb_backend_t {
    name: 0 as *const libc::c_char,
    query: None,
    p_d: 0 as *const libc::c_void as *mut libc::c_void,
}; 8];
#[no_mangle]
#[cold]
pub unsafe extern "C" fn http_vhostdb_dumbdata_reset() {
    memset(
        http_vhostdb_backends.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[http_vhostdb_backend_t; 8]>() as libc::c_ulong,
    );
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn http_vhostdb_backend_get(
    mut name: *const buffer,
) -> *const http_vhostdb_backend_t {
    let mut i: libc::c_int = 0 as libc::c_int;
    while !(http_vhostdb_backends[i as usize].name).is_null()
        && 0 as libc::c_int
            != strcmp(http_vhostdb_backends[i as usize].name, (*name).ptr)
    {
        i += 1;
    }
    return if !(http_vhostdb_backends[i as usize].name).is_null() {
        http_vhostdb_backends.as_mut_ptr().offset(i as isize)
    } else {
        0 as *mut http_vhostdb_backend_t
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn http_vhostdb_backend_set(
    mut backend: *const http_vhostdb_backend_t,
) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while !(http_vhostdb_backends[i as usize].name).is_null() {
        i = i.wrapping_add(1);
    }
    if !((i as libc::c_ulong)
        < (::std::mem::size_of::<[http_vhostdb_backend_t; 8]>() as libc::c_ulong)
            .wrapping_div(
                ::std::mem::size_of::<http_vhostdb_backend_t>() as libc::c_ulong,
            )
            .wrapping_sub(1 as libc::c_int as libc::c_ulong))
    {
        ck_assert_failed(
            b"src/mod_vhostdb_api.c\0" as *const u8 as *const libc::c_char,
            39 as libc::c_int as libc::c_uint,
            b"i < (sizeof(http_vhostdb_backends)/sizeof(http_vhostdb_backend_t))-1\0"
                as *const u8 as *const libc::c_char,
        );
    }
    memcpy(
        http_vhostdb_backends.as_mut_ptr().offset(i as isize) as *mut libc::c_void,
        backend as *const libc::c_void,
        ::std::mem::size_of::<http_vhostdb_backend_t>() as libc::c_ulong,
    );
}
