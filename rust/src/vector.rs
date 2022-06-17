use ::libc;
extern "C" {
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn vector_free(mut data: *mut libc::c_void) {
    free(data);
}
#[no_mangle]
pub unsafe extern "C" fn vector_malloc(mut sz: size_t) -> *mut libc::c_void {
    return malloc(sz);
}
#[no_mangle]
pub unsafe extern "C" fn vector_realloc(
    mut data: *mut libc::c_void,
    mut elem_size: size_t,
    mut size: size_t,
    mut used: size_t,
) -> *mut libc::c_void {
    let total_size: size_t = elem_size.wrapping_mul(size);
    let used_size: size_t = elem_size.wrapping_mul(used);
    if !(size <= (18446744073709551615 as libc::c_ulong).wrapping_div(elem_size)) {
        ck_assert_failed(
            b"src/vector.c\0" as *const u8 as *const libc::c_char,
            15 as libc::c_int as libc::c_uint,
            b"size <= SIZE_MAX / elem_size\0" as *const u8 as *const libc::c_char,
        );
    }
    data = realloc(data, total_size);
    if data.is_null() {
        ck_assert_failed(
            b"src/vector.c\0" as *const u8 as *const libc::c_char,
            17 as libc::c_int as libc::c_uint,
            b"NULL != data\0" as *const u8 as *const libc::c_char,
        );
    }
    memset(
        (data as *mut libc::c_char).offset(used_size as isize) as *mut libc::c_void,
        0 as libc::c_int,
        total_size.wrapping_sub(used_size),
    );
    return data;
}
