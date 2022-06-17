use ::libc;
extern "C" {
    fn ck_memclear_s(s: *mut libc::c_void, smax: rsize_t, n: rsize_t) -> errno_t;
    fn fdevent_open_cloexec(
        pathname: *const libc::c_char,
        symlinks: libc::c_int,
        flags: libc::c_int,
        mode: mode_t,
    ) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn __errno_location() -> *mut libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn rand() -> libc::c_int;
    fn srandom(__seed: libc::c_uint);
    fn random() -> libc::c_long;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn getpid() -> __pid_t;
    fn syscall(__sysno: libc::c_long, _: ...) -> libc::c_long;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
}
pub type __mode_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type rsize_t = libc::c_ulong;
pub type errno_t = libc::c_int;
#[inline]
unsafe extern "C" fn ck_memzero(mut s: *mut libc::c_void, mut n: rsize_t) -> errno_t {
    return ck_memclear_s(s, n, n);
}
unsafe extern "C" fn li_getentropy(
    mut buf: *mut libc::c_void,
    mut buflen: size_t,
) -> libc::c_int {
    if buflen <= 256 as libc::c_int as libc::c_ulong {
        let mut num: libc::c_int = syscall(
            318 as libc::c_int as libc::c_long,
            buf,
            buflen,
            0 as libc::c_int,
        ) as libc::c_int;
        if num == buflen as libc::c_int {
            return 0 as libc::c_int;
        }
        if num < 0 as libc::c_int {
            return num;
        }
    }
    *__errno_location() = 5 as libc::c_int;
    return -(1 as libc::c_int);
}
unsafe extern "C" fn li_rand_device_bytes(
    mut buf: *mut libc::c_uchar,
    mut num: libc::c_int,
) -> libc::c_int {
    static mut devices: [*const libc::c_char; 2] = [
        b"/dev/urandom\0" as *const u8 as *const libc::c_char,
        b"/dev/random\0" as *const u8 as *const libc::c_char,
    ];
    if 0 as libc::c_int == li_getentropy(buf as *mut libc::c_void, num as size_t) {
        return 1 as libc::c_int;
    }
    let mut u: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (u as libc::c_ulong)
        < (::std::mem::size_of::<[*const libc::c_char; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        let mut fd: libc::c_int = fdevent_open_cloexec(
            devices[u as usize],
            1 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int as mode_t,
        );
        if fd >= 0 as libc::c_int {
            let mut rd: ssize_t = 0 as libc::c_int as ssize_t;
            let mut entropy: libc::c_int = 0;
            if 0 as libc::c_int
                == ioctl(
                    fd,
                    ((2 as libc::c_uint)
                        << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int
                            + 14 as libc::c_int
                        | (('R' as i32) << 0 as libc::c_int + 8 as libc::c_int)
                            as libc::c_uint
                        | ((0 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                        as libc::c_ulong
                        | (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
                    &mut entropy as *mut libc::c_int,
                ) && entropy >= num * 8 as libc::c_int
            {
                rd = read(fd, buf as *mut libc::c_void, num as size_t);
            }
            close(fd);
            if rd == num as libc::c_long {
                return 1 as libc::c_int;
            }
        }
        u = u.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
static mut li_rand_inited: libc::c_int = 0;
static mut xsubi: [libc::c_ushort; 3] = [0; 3];
#[cold]
unsafe extern "C" fn li_rand_init() {
    let mut u: libc::c_uint = 0;
    li_rand_inited = 1 as libc::c_int;
    if 1 as libc::c_int
        == li_rand_device_bytes(
            xsubi.as_mut_ptr() as *mut libc::c_uchar,
            ::std::mem::size_of::<[libc::c_ushort; 3]>() as libc::c_ulong as libc::c_int,
        )
    {
        u = (xsubi[0 as libc::c_int as usize] as libc::c_uint) << 16 as libc::c_int
            | xsubi[1 as libc::c_int as usize] as libc::c_uint;
    } else {
        srand((time(0 as *mut time_t) ^ getpid() as libc::c_long) as libc::c_uint);
        u = 0 as libc::c_int as libc::c_uint;
        while (u as libc::c_ulong)
            < ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong
        {
            xsubi[u as usize] = (rand() & 0xffff as libc::c_int) as libc::c_ushort;
            u = u.wrapping_add(1);
        }
        u = (xsubi[0 as libc::c_int as usize] as libc::c_uint) << 16 as libc::c_int
            | xsubi[1 as libc::c_int as usize] as libc::c_uint;
    }
    srand(u);
    srandom(u);
}
#[no_mangle]
pub unsafe extern "C" fn li_rand_reseed() {
    if li_rand_inited != 0 {
        li_rand_init();
    }
}
#[no_mangle]
pub unsafe extern "C" fn li_rand_pseudo() -> libc::c_int {
    if li_rand_inited == 0 {
        li_rand_init();
    }
    return random() as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn li_rand_pseudo_bytes(
    mut buf: *mut libc::c_uchar,
    mut num: libc::c_int,
) {
    if li_rand_inited == 0 {
        li_rand_init();
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < num {
        *buf
            .offset(
                i as isize,
            ) = (li_rand_pseudo() & 0xff as libc::c_int) as libc::c_uchar;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn li_rand_bytes(
    mut buf: *mut libc::c_uchar,
    mut num: libc::c_int,
) -> libc::c_int {
    if 1 as libc::c_int == li_rand_device_bytes(buf, num) {
        return 1 as libc::c_int
    } else {
        li_rand_pseudo_bytes(buf, num);
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn li_rand_cleanup() {
    ck_memzero(
        xsubi.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_ushort; 3]>() as libc::c_ulong,
    );
}
