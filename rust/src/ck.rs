use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn abort() -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror_r(
        __errnum: libc::c_int,
        __buf: *mut libc::c_char,
        __buflen: size_t,
    ) -> *mut libc::c_char;
    fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type rsize_t = libc::c_ulong;
pub type errno_t = libc::c_int;
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[no_mangle]
pub unsafe extern "C" fn ck_memclear_s(
    s: *mut libc::c_void,
    smax: rsize_t,
    mut n: rsize_t,
) -> errno_t {
    if s.is_null() {
        return 22 as libc::c_int;
    }
    if (18446744073709551615 as libc::c_ulong >> 1 as libc::c_int) < smax {
        return 7 as libc::c_int;
    }
    let mut rc: errno_t = 0 as libc::c_int;
    if (18446744073709551615 as libc::c_ulong >> 1 as libc::c_int) < n {
        rc = 22 as libc::c_int;
        n = smax;
    }
    if smax < n {
        rc = 75 as libc::c_int;
        n = smax;
    }
    explicit_bzero(s, n);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn ck_strerror_s(
    s: *mut libc::c_char,
    maxsize: rsize_t,
    errnum: errno_t,
) -> errno_t {
    if s.is_null() || 0 as libc::c_int as libc::c_ulong == maxsize
        || (18446744073709551615 as libc::c_ulong >> 1 as libc::c_int) < maxsize
    {
        return 22 as libc::c_int;
    }
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut errstr: *const libc::c_char = strerror_r(
        errnum,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    if !errstr.is_null() {
        let errlen: size_t = strlen(errstr);
        if errlen < maxsize {
            memcpy(
                s as *mut libc::c_void,
                errstr as *const libc::c_void,
                errlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            return 0 as libc::c_int;
        } else {
            memcpy(
                s as *mut libc::c_void,
                errstr as *const libc::c_void,
                maxsize.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            *s
                .offset(
                    maxsize.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = '\u{0}' as i32 as libc::c_char;
        }
    } else if (snprintf(
            s,
            maxsize,
            b"Unknown error %d\0" as *const u8 as *const libc::c_char,
            errnum,
        ) as rsize_t) < maxsize
        {
        return 0 as libc::c_int
    }
    if maxsize > 3 as libc::c_int as libc::c_ulong {
        memcpy(
            s.offset(maxsize as isize).offset(-(4 as libc::c_int as isize))
                as *mut libc::c_void,
            b"...\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as libc::c_ulong,
        );
    }
    return 34 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ck_memeq_const_time(
    mut a: *const libc::c_void,
    mut alen: size_t,
    mut b: *const libc::c_void,
    mut blen: size_t,
) -> libc::c_int {
    let av: *const libc::c_uchar = (if alen != 0 {
        a
    } else {
        b"\0" as *const u8 as *const libc::c_char as *const libc::c_void
    }) as *const libc::c_uchar;
    let bv: *const libc::c_uchar = (if blen != 0 {
        b
    } else {
        b"\0" as *const u8 as *const libc::c_char as *const libc::c_void
    }) as *const libc::c_uchar;
    let mut lim: size_t = (if alen >= blen { alen } else { blen })
        .wrapping_add(0x3f as libc::c_int as libc::c_ulong)
        & !(0x3f as libc::c_int) as libc::c_ulong;
    let mut diff: libc::c_int = (alen != blen) as libc::c_int;
    alen = (alen as libc::c_ulong)
        .wrapping_sub(
            (alen != 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_ulong,
        ) as size_t as size_t;
    blen = (blen as libc::c_ulong)
        .wrapping_sub(
            (blen != 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_ulong,
        ) as size_t as size_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut j: size_t = 0 as libc::c_int as size_t;
    while lim != 0 {
        diff
            |= *av.offset(i as isize) as libc::c_int
                ^ *bv.offset(j as isize) as libc::c_int;
        i = (i as libc::c_ulong).wrapping_add((i < alen) as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        j = (j as libc::c_ulong).wrapping_add((j < blen) as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        lim = lim.wrapping_sub(1);
    }
    return (0 as libc::c_int == diff) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ck_memeq_const_time_fixed_len(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
    len: size_t,
) -> libc::c_int {
    let av: *const libc::c_uchar = a as *const libc::c_uchar;
    let bv: *const libc::c_uchar = b as *const libc::c_uchar;
    let mut diff: libc::c_int = 0 as libc::c_int;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < len {
        diff
            |= *av.offset(i as isize) as libc::c_int
                ^ *bv.offset(i as isize) as libc::c_int;
        i = i.wrapping_add(1);
    }
    return (0 as libc::c_int == diff) as libc::c_int;
}
#[inline(never)]
unsafe extern "C" fn ck_bt_stderr(
    mut filename: *const libc::c_char,
    mut line: libc::c_uint,
    mut msg: *const libc::c_char,
    mut fmt: *const libc::c_char,
) {
    fprintf(stderr, fmt, filename, line, msg);
    fflush(stderr);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ck_bt(
    mut filename: *const libc::c_char,
    mut line: libc::c_uint,
    mut msg: *const libc::c_char,
) {
    ck_bt_stderr(
        filename,
        line,
        msg,
        b"%s.%u: %s\n\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ck_bt_abort(
    mut filename: *const libc::c_char,
    mut line: libc::c_uint,
    mut msg: *const libc::c_char,
) -> ! {
    ck_bt(filename, line, msg);
    abort();
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ck_assert_failed(
    mut filename: *const libc::c_char,
    mut line: libc::c_uint,
    mut msg: *const libc::c_char,
) -> ! {
    ck_bt_stderr(
        filename,
        line,
        msg,
        b"%s.%u: assertion failed: %s\n\0" as *const u8 as *const libc::c_char,
    );
    abort();
}
