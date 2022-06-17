use ::libc;
extern "C" {
    pub type connection;
    fn buffer_extend(b: *mut buffer, x: size_t) -> *mut libc::c_char;
    fn buffer_free_ptr(b: *mut buffer);
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_str2(
        b: *mut buffer,
        s1: *const libc::c_char,
        len1: size_t,
        s2: *const libc::c_char,
        len2: size_t,
    );
    fn buffer_append_iovec(b: *mut buffer, iov: *const const_iovec, n: size_t);
    fn buffer_append_strftime(
        b: *mut buffer,
        format: *const libc::c_char,
        tm: *const tm,
    );
    fn li_itostrn(buf: *mut libc::c_char, buf_len: size_t, val: intmax_t) -> size_t;
    fn li_utostrn(buf: *mut libc::c_char, buf_len: size_t, val: uintmax_t) -> size_t;
    fn buffer_append_string_c_escaped(
        b: *mut buffer,
        s: *const libc::c_char,
        s_len: size_t,
    );
    fn ck_strerror_s(s: *mut libc::c_char, maxsize: rsize_t, errnum: errno_t) -> errno_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn syslog(__pri: libc::c_int, __fmt: *const libc::c_char, _: ...);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uint32_t = libc::c_uint;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type rsize_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type unix_time64_t = time_t;
pub type unix_timespec64_t = timespec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fdlog_st {
    pub mode: C2RustUnnamed,
    pub fd: libc::c_int,
    pub b: buffer,
    pub fn_0: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub ptr: *mut libc::c_char,
    pub used: uint32_t,
    pub size: uint32_t,
}
pub type C2RustUnnamed = libc::c_uint;
pub const FDLOG_PIPE: C2RustUnnamed = 3;
pub const FDLOG_SYSLOG: C2RustUnnamed = 2;
pub const FDLOG_FD: C2RustUnnamed = 1;
pub const FDLOG_FILE: C2RustUnnamed = 0;
pub type log_error_st = fdlog_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct const_iovec {
    pub iov_base: *const libc::c_void,
    pub iov_len: size_t,
}
pub type errno_t = libc::c_int;
pub type va_list = __builtin_va_list;
#[inline]
unsafe extern "C" fn buffer_clear(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
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
unsafe extern "C" fn buffer_string_space(mut b: *const buffer) -> uint32_t {
    return if (*b).size != 0 {
        ((*b).size)
            .wrapping_sub(
                (*b).used
                    | (0 as libc::c_int as libc::c_uint == (*b).used) as libc::c_int
                        as libc::c_uint,
            )
    } else {
        0 as libc::c_int as libc::c_uint
    };
}
#[inline]
unsafe extern "C" fn buffer_truncate(mut b: *mut buffer, mut len: uint32_t) {
    *((*b).ptr).offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    (*b).used = len.wrapping_add(1 as libc::c_int as libc::c_uint);
}
static mut log_stderrh: fdlog_st = {
    let mut init = fdlog_st {
        mode: FDLOG_FD,
        fd: 2 as libc::c_int,
        b: {
            let mut init = buffer {
                ptr: 0 as *const libc::c_char as *mut libc::c_char,
                used: 0 as libc::c_int as uint32_t,
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        fn_0: 0 as *const libc::c_char,
    };
    init
};
static mut log_errh: *mut fdlog_st = unsafe {
    &log_stderrh as *const fdlog_st as *mut fdlog_st
};
static mut log_tlast: unix_time64_t = 0 as libc::c_int as unix_time64_t;
#[no_mangle]
pub static mut log_con_jqueue: *mut connection = 0 as *const connection
    as *mut connection;
#[no_mangle]
pub static mut log_epoch_secs: unix_time64_t = 0 as libc::c_int as unix_time64_t;
#[no_mangle]
pub static mut log_monotonic_secs: unix_time64_t = 0 as libc::c_int as unix_time64_t;
#[no_mangle]
pub unsafe extern "C" fn write_all(
    mut fd: libc::c_int,
    buf: *const libc::c_void,
    mut count: size_t,
) -> ssize_t {
    let mut written: ssize_t = 0 as libc::c_int as ssize_t;
    let mut wr: ssize_t = 0;
    loop {
        wr = write(
            fd,
            (buf as *const libc::c_char).offset(written as isize) as *const libc::c_void,
            count,
        );
        if !(if wr > 0 as libc::c_int as libc::c_long {
            written += wr;
            count = (count as libc::c_ulong).wrapping_sub(wr as libc::c_ulong) as size_t
                as size_t;
            count
        } else {
            (wr < 0 as libc::c_int as libc::c_long
                && *__errno_location() == 4 as libc::c_int) as libc::c_int
                as libc::c_ulong
        } != 0)
        {
            break;
        }
    }
    if (0 as libc::c_int as libc::c_ulong == count) as libc::c_int as libc::c_long != 0 {
        return written
    } else {
        if 0 as libc::c_int as libc::c_long == wr {
            *__errno_location() = 5 as libc::c_int;
        }
        return -(1 as libc::c_int) as ssize_t;
    };
}
unsafe extern "C" fn log_buffer_timestamp(b: *mut buffer) {
    if -(2 as libc::c_int) as libc::c_long == log_tlast {
        let mut ts: unix_timespec64_t = {
            let mut init = timespec {
                tv_sec: 0 as libc::c_int as __time_t,
                tv_nsec: 0 as libc::c_int as __syscall_slong_t,
            };
            init
        };
        clock_gettime(0 as libc::c_int, &mut ts);
        let mut tm: tm = tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: 0 as *const libc::c_char,
        };
        buffer_append_strftime(
            b,
            b"%F %T\0" as *const u8 as *const libc::c_char,
            localtime_r(&mut ts.tv_sec, &mut tm),
        );
        buffer_append_string_len(
            b,
            b".000000000: \0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        let mut n: [libc::c_char; 22] = [0; 22];
        let nlen: size_t = li_utostrn(
            n.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong,
            ts.tv_nsec as libc::c_ulong,
        );
        memcpy(
            ((*b).ptr)
                .offset(buffer_clen(b) as isize)
                .offset(-(nlen as isize))
                .offset(-(2 as libc::c_int as isize)) as *mut libc::c_void,
            n.as_mut_ptr() as *const libc::c_void,
            nlen,
        );
    } else {
        static mut tlen: uint32_t = 0;
        static mut tstr: [libc::c_char; 24] = [0; 24];
        if (log_tlast != log_epoch_secs) as libc::c_int as libc::c_long != 0 {
            let mut tm_0: tm = tm {
                tm_sec: 0,
                tm_min: 0,
                tm_hour: 0,
                tm_mday: 0,
                tm_mon: 0,
                tm_year: 0,
                tm_wday: 0,
                tm_yday: 0,
                tm_isdst: 0,
                tm_gmtoff: 0,
                tm_zone: 0 as *const libc::c_char,
            };
            log_tlast = log_epoch_secs;
            tlen = strftime(
                tstr.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong,
                b"%F %T\0" as *const u8 as *const libc::c_char,
                localtime_r(&mut log_tlast, &mut tm_0),
            ) as uint32_t;
            tstr[tlen as usize] = ':' as i32 as libc::c_char;
            tlen = tlen.wrapping_add(1);
            tstr[tlen as usize] = ' ' as i32 as libc::c_char;
            tlen = tlen.wrapping_add(1);
        }
        buffer_copy_string_len(b, tstr.as_mut_ptr(), tlen as size_t);
    };
}
unsafe extern "C" fn log_buffer_prefix(
    b: *mut buffer,
    filename: *const libc::c_char,
    line: libc::c_uint,
) {
    let mut lstr: [libc::c_char; 22] = [0; 22];
    let mut iov: [const_iovec; 5] = [
        {
            let mut init = const_iovec {
                iov_base: b"(\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                iov_len: (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: filename as *const libc::c_void,
                iov_len: strlen(filename),
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: b".\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                iov_len: (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: lstr.as_mut_ptr() as *const libc::c_void,
                iov_len: li_itostrn(
                    lstr.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong,
                    line as intmax_t,
                ),
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: b") \0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                iov_len: (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            };
            init
        },
    ];
    buffer_append_iovec(
        b,
        iov.as_mut_ptr(),
        (::std::mem::size_of::<[const_iovec; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<const_iovec>() as libc::c_ulong),
    );
}
unsafe extern "C" fn log_buffer_append_encoded(
    b: *mut buffer,
    s: *const libc::c_char,
    n: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n && ' ' as i32 <= *s.offset(i as isize) as libc::c_int
        && *s.offset(i as isize) as libc::c_int <= '~' as i32
    {
        i = i.wrapping_add(1);
    }
    if i == n {
        buffer_append_string_len(b, s, n);
    } else {
        buffer_append_string_c_escaped(b, s, n);
    };
}
unsafe extern "C" fn log_buffer_vsprintf(
    b: *mut buffer,
    fmt: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) {
    let mut blen: size_t = buffer_clen(b) as size_t;
    let mut bsp: size_t = (buffer_string_space(b))
        .wrapping_add(1 as libc::c_int as libc::c_uint) as size_t;
    let mut s: *mut libc::c_char = ((*b).ptr).offset(blen as isize);
    let mut n: size_t = 0;
    let mut aptry: ::std::ffi::VaListImpl;
    aptry = ap.clone();
    n = vsnprintf(s, bsp, fmt, aptry.as_va_list()) as size_t;
    if n < bsp {
        buffer_truncate(b, blen.wrapping_add(n) as uint32_t);
    } else {
        s = buffer_extend(b, n);
        vsnprintf(
            s,
            n.wrapping_add(1 as libc::c_int as libc::c_ulong),
            fmt,
            ap.as_va_list(),
        );
    }
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n && ' ' as i32 <= *s.offset(i as isize) as libc::c_int
        && *s.offset(i as isize) as libc::c_int <= '~' as i32
    {
        i = i.wrapping_add(1);
    }
    if i == n {
        return;
    }
    let src: *mut libc::c_char = malloc(n) as *mut libc::c_char;
    memcpy(src as *mut libc::c_void, s as *const libc::c_void, n);
    buffer_truncate(b, blen as uint32_t);
    buffer_append_string_c_escaped(b, src, n);
    free(src as *mut libc::c_void);
}
unsafe extern "C" fn log_buffer_prepare(
    errh: *const log_error_st,
    filename: *const libc::c_char,
    line: libc::c_uint,
) -> *mut buffer {
    let b: *mut buffer = &mut (*log_errh).b;
    buffer_clear(b);
    if (*errh).mode as libc::c_uint != FDLOG_SYSLOG as libc::c_int as libc::c_uint {
        if -(1 as libc::c_int) == (*errh).fd {
            return 0 as *mut buffer;
        }
        log_buffer_timestamp(b);
    }
    log_buffer_prefix(b, filename, line);
    return b;
}
unsafe extern "C" fn log_error_write(errh: *const log_error_st, b: *mut buffer) {
    if (*errh).mode as libc::c_uint != FDLOG_SYSLOG as libc::c_int as libc::c_uint {
        buffer_append_string_len(
            b,
            b"\n\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        write_all((*errh).fd, (*b).ptr as *const libc::c_void, buffer_clen(b) as size_t);
    } else {
        syslog(3 as libc::c_int, b"%s\0" as *const u8 as *const libc::c_char, (*b).ptr);
    };
}
#[inline(never)]
unsafe extern "C" fn log_error_append_strerror(b: *mut buffer, errnum: libc::c_int) {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut rc: errno_t = ck_strerror_s(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        errnum,
    );
    if 0 as libc::c_int == rc || rc == 34 as libc::c_int {
        buffer_append_str2(
            b,
            b": \0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            buf.as_mut_ptr(),
            strlen(buf.as_mut_ptr()),
        );
    }
}
unsafe extern "C" fn log_error_va_list_impl(
    mut errh: *const log_error_st,
    filename: *const libc::c_char,
    line: libc::c_uint,
    fmt: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
    perr: libc::c_int,
) {
    let errnum: libc::c_int = *__errno_location();
    if errh.is_null() {
        errh = log_errh;
    }
    let b: *mut buffer = log_buffer_prepare(errh, filename, line);
    if b.is_null() {
        return;
    }
    log_buffer_vsprintf(b, fmt, ap.as_va_list());
    if perr != 0 {
        log_error_append_strerror(b, errnum);
    }
    log_error_write(errh, b);
    buffer_clear(b);
    *__errno_location() = errnum;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn log_error(
    errh: *mut log_error_st,
    filename: *const libc::c_char,
    line: libc::c_uint,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    log_error_va_list_impl(errh, filename, line, fmt, ap.as_va_list(), 0 as libc::c_int);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn log_perror(
    errh: *mut log_error_st,
    filename: *const libc::c_char,
    line: libc::c_uint,
    fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    log_error_va_list_impl(errh, filename, line, fmt, ap.as_va_list(), 1 as libc::c_int);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn log_error_multiline(
    mut errh: *mut log_error_st,
    filename: *const libc::c_char,
    line: libc::c_uint,
    multiline: *const libc::c_char,
    len: size_t,
    fmt: *const libc::c_char,
    mut args: ...
) {
    if 0 as libc::c_int as libc::c_ulong == len {
        return;
    }
    let errnum: libc::c_int = *__errno_location();
    if errh.is_null() {
        errh = log_errh;
    }
    let b: *mut buffer = log_buffer_prepare(errh, filename, line);
    if b.is_null() {
        return;
    }
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    log_buffer_vsprintf(b, fmt, ap.as_va_list());
    let prefix_len: uint32_t = buffer_clen(b);
    let end: *const libc::c_char = multiline.offset(len as isize);
    let mut pos: *const libc::c_char = multiline;
    while pos < end {
        let current_line: *const libc::c_char = pos;
        pos = strchr(pos, '\n' as i32);
        if pos.is_null() {
            pos = end;
        }
        let mut n: size_t = pos.offset_from(current_line) as libc::c_long as size_t;
        if n != 0
            && *current_line
                .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int == '\r' as i32
        {
            n = n.wrapping_sub(1);
        }
        buffer_truncate(b, prefix_len);
        log_buffer_append_encoded(b, current_line, n);
        log_error_write(errh, b);
        pos = pos.offset(1);
    }
    buffer_clear(b);
    *__errno_location() = errnum;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn log_set_global_errh(
    errh: *mut log_error_st,
    ts_high_precision: libc::c_int,
) -> *mut log_error_st {
    log_tlast = (if ts_high_precision != 0 {
        -(2 as libc::c_int)
    } else {
        -(1 as libc::c_int)
    }) as unix_time64_t;
    buffer_free_ptr(&mut log_stderrh.b);
    log_errh = if !errh.is_null() { errh } else { &mut log_stderrh };
    return log_errh;
}
