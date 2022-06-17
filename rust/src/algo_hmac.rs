use ::libc;
extern "C" {
    fn MD5_Update(_: *mut MD5_CTX, _: *const libc::c_void, _: libc::c_uint);
    fn MD5_Init(_: *mut MD5_CTX);
    fn MD5_Final(_: *mut libc::c_uchar, _: *mut MD5_CTX);
}
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct const_iovec {
    pub iov_base: *const libc::c_void,
    pub iov_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5_CTX {
    pub state: [uint32_t; 4],
    pub count: [uint32_t; 2],
    pub buffer: [libc::c_uchar; 64],
}
#[inline]
unsafe extern "C" fn MD5_iov(
    digest: *mut libc::c_uchar,
    iov: *const const_iovec,
    n: size_t,
) {
    let mut ctx: MD5_CTX = MD5_CTX {
        state: [0; 4],
        count: [0; 2],
        buffer: [0; 64],
    };
    MD5_Init(&mut ctx);
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n {
        if (*iov.offset(i as isize)).iov_len != 0 {
            MD5_Update(
                &mut ctx,
                (*iov.offset(i as isize)).iov_base,
                (*iov.offset(i as isize)).iov_len as libc::c_uint,
            );
        }
        i = i.wrapping_add(1);
    }
    MD5_Final(digest, &mut ctx);
}
#[no_mangle]
pub unsafe extern "C" fn li_hmac_md5(
    mut digest: *mut libc::c_uchar,
    secret: *const libc::c_void,
    slen: uint32_t,
    msg: *const libc::c_uchar,
    mlen: uint32_t,
) -> libc::c_int {
    let mut iov: [const_iovec; 2] = [
        {
            let mut init = const_iovec {
                iov_base: secret,
                iov_len: slen as size_t,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: msg as *const libc::c_void,
                iov_len: mlen as size_t,
            };
            init
        },
    ];
    MD5_iov(
        digest,
        iov.as_mut_ptr(),
        (::std::mem::size_of::<[const_iovec; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<const_iovec>() as libc::c_ulong),
    );
    return 1 as libc::c_int;
}
