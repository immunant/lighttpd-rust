use ::libc;
extern "C" {
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_int(b: *mut buffer, val: intmax_t);
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intmax_t = __intmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub ptr: *mut libc::c_char,
    pub used: uint32_t,
    pub size: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type C2RustUnnamed = libc::c_uint;
pub const ETAG_USE_SIZE: C2RustUnnamed = 4;
pub const ETAG_USE_MTIME: C2RustUnnamed = 2;
pub const ETAG_USE_INODE: C2RustUnnamed = 1;
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
#[inline]
unsafe extern "C" fn buffer_clear(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn dekhash(
    mut str: *const libc::c_char,
    len: uint32_t,
    mut hash: uint32_t,
) -> uint32_t {
    let s: *const libc::c_uchar = str as *const libc::c_uchar;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < len {
        hash = hash << 5 as libc::c_int ^ hash >> 27 as libc::c_int
            ^ *s.offset(i as isize) as libc::c_uint;
        i = i.wrapping_add(1);
    }
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn http_etag_matches(
    etag: *const buffer,
    mut s: *const libc::c_char,
    weak_ok: libc::c_int,
) -> libc::c_int {
    if '*' as i32 == *s.offset(0 as libc::c_int as isize) as libc::c_int
        && '\u{0}' as i32 == *s.offset(1 as libc::c_int as isize) as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if buffer_is_blank(etag) != 0 {
        return 0 as libc::c_int;
    }
    let mut etag_sz: uint32_t = buffer_clen(etag);
    let mut etag_ptr: *const libc::c_char = (*etag).ptr;
    if *etag_ptr.offset(0 as libc::c_int as isize) as libc::c_int == 'W' as i32
        && *etag_ptr.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        if weak_ok == 0 {
            return 0 as libc::c_int;
        }
        etag_ptr = etag_ptr.offset(2 as libc::c_int as isize);
        etag_sz = (etag_sz as libc::c_uint)
            .wrapping_sub(2 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
    }
    while *s != 0 {
        while *s as libc::c_int == ' ' as i32 || *s as libc::c_int == '\t' as i32
            || *s as libc::c_int == ',' as i32
        {
            s = s.offset(1);
        }
        if if *s.offset(0 as libc::c_int as isize) as libc::c_int == 'W' as i32
            && *s.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            s = s.offset(2 as libc::c_int as isize);
            weak_ok
        } else {
            1 as libc::c_int
        } != 0
        {
            if 0 as libc::c_int == strncmp(s, etag_ptr, etag_sz as libc::c_ulong)
                || *s as libc::c_int == '*' as i32
            {
                s = s
                    .offset(
                        (if *s as libc::c_int != '*' as i32 {
                            etag_sz
                        } else {
                            1 as libc::c_int as libc::c_uint
                        }) as isize,
                    );
                if *s as libc::c_int == '\u{0}' as i32 || *s as libc::c_int == ' ' as i32
                    || *s as libc::c_int == '\t' as i32
                    || *s as libc::c_int == ',' as i32
                {
                    return 1 as libc::c_int;
                }
            }
        }
        while *s as libc::c_int != '\u{0}' as i32 && *s as libc::c_int != ',' as i32 {
            s = s.offset(1);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn http_etag_remix(
    etag: *mut buffer,
    str: *const libc::c_char,
    len: uint32_t,
) {
    let mut h: uint32_t = dekhash(str, len, len);
    buffer_copy_string_len(
        etag,
        b"\"\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(etag, h as intmax_t);
    buffer_append_string_len(
        etag,
        b"\"\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn http_etag_create(
    etag: *mut buffer,
    st: *const stat,
    flags: libc::c_int,
) {
    if 0 as libc::c_int == flags {
        return;
    }
    let mut x: [uint64_t; 4] = [0; 4];
    let mut len: uint32_t = 0 as libc::c_int as uint32_t;
    if flags & ETAG_USE_INODE as libc::c_int != 0 {
        let fresh0 = len;
        len = len.wrapping_add(1);
        x[fresh0 as usize] = (*st).st_ino;
    }
    if flags & ETAG_USE_SIZE as libc::c_int != 0 {
        let fresh1 = len;
        len = len.wrapping_add(1);
        x[fresh1 as usize] = (*st).st_size as uint64_t;
    }
    if flags & ETAG_USE_MTIME as libc::c_int != 0 {
        let fresh2 = len;
        len = len.wrapping_add(1);
        x[fresh2 as usize] = (*st).st_mtim.tv_sec as uint64_t;
        let fresh3 = len;
        len = len.wrapping_add(1);
        x[fresh3 as usize] = (*st).st_mtim.tv_nsec as uint64_t;
    }
    buffer_clear(etag);
    http_etag_remix(etag, x.as_mut_ptr() as *mut libc::c_char, len << 3 as libc::c_int);
}
