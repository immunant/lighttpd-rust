use ::libc;
extern "C" {
    fn buffer_extend(b: *mut buffer, x: size_t) -> *mut libc::c_char;
    fn buffer_copy_string(b: *mut buffer, s: *const libc::c_char);
    fn sock_addr_inet_ntop_copy_buffer(
        b: *mut buffer,
        saddr: *const sock_addr,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub ptr: *mut libc::c_char,
    pub used: uint32_t,
    pub size: uint32_t,
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sock_addr {
    pub ipv6: sockaddr_in6,
    pub ipv4: sockaddr_in,
    pub un: sockaddr_un,
    pub plain: sockaddr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub s: [libc::c_char; 47],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub ipv6: in6_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub s: [libc::c_char; 17],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub ipv4: in_addr,
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
unsafe extern "C" fn buffer_truncate(mut b: *mut buffer, mut len: uint32_t) {
    *((*b).ptr).offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    (*b).used = len.wrapping_add(1 as libc::c_int as libc::c_uint);
}
#[inline]
unsafe extern "C" fn buffer_blank(mut b: *mut buffer) {
    if !((*b).ptr).is_null() {
        buffer_truncate(b, 0 as libc::c_int as uint32_t);
    } else {
        buffer_extend(b, 0 as libc::c_int as size_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn sock_addr_cache_inet_ntop_copy_buffer(
    b: *mut buffer,
    saddr: *const sock_addr,
) -> libc::c_int {
    static mut ndx4: libc::c_int = 0;
    static mut ntop4_cache: [C2RustUnnamed_3; 4] = [C2RustUnnamed_3 {
        ipv4: in_addr { s_addr: 0 },
    }; 4];
    static mut ntop4_strs: [C2RustUnnamed_2; 4] = [C2RustUnnamed_2 { s: [0; 17] }; 4];
    static mut ndx6: libc::c_int = 0;
    static mut ntop6_cache: [C2RustUnnamed_1; 4] = [C2RustUnnamed_1 {
        ipv6: in6_addr {
            __in6_u: C2RustUnnamed {
                __u6_addr8: [0; 16],
            },
        },
    }; 4];
    static mut ntop6_strs: [C2RustUnnamed_0; 4] = [C2RustUnnamed_0 { s: [0; 47] }; 4];
    match (*saddr).plain.sa_family as libc::c_int {
        2 => {
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                if ntop4_cache[i as usize].ipv4.s_addr == (*saddr).ipv4.sin_addr.s_addr {
                    buffer_copy_string(b, (ntop4_strs[i as usize].s).as_mut_ptr());
                    return 0 as libc::c_int;
                }
                i += 1;
            }
        }
        10 => {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < 4 as libc::c_int {
                if 0 as libc::c_int
                    == memcmp(
                        (ntop6_cache[i_0 as usize].ipv6.__in6_u.__u6_addr8).as_mut_ptr()
                            as *const libc::c_void,
                        ((*saddr).ipv6.sin6_addr.__in6_u.__u6_addr8).as_ptr()
                            as *const libc::c_void,
                        16 as libc::c_int as libc::c_ulong,
                    )
                {
                    buffer_copy_string(b, (ntop6_strs[i_0 as usize].s).as_mut_ptr());
                    return 0 as libc::c_int;
                }
                i_0 += 1;
            }
        }
        _ => {}
    }
    if 0 as libc::c_int != sock_addr_inet_ntop_copy_buffer(b, saddr) {
        buffer_blank(b);
        return -(1 as libc::c_int);
    }
    match (*saddr).plain.sa_family as libc::c_int {
        2 => {
            ntop4_cache[ndx4 as usize].ipv4.s_addr = (*saddr).ipv4.sin_addr.s_addr;
            memcpy(
                ntop4_strs.as_mut_ptr().offset(ndx4 as isize) as *mut libc::c_void,
                (*b).ptr as *const libc::c_void,
                (buffer_clen(b)).wrapping_add(1 as libc::c_int as libc::c_uint)
                    as libc::c_ulong,
            );
            ndx4 += 1;
            if ndx4 == 4 as libc::c_int {
                ndx4 = 0 as libc::c_int;
            }
        }
        10 => {
            memcpy(
                (ntop6_cache[ndx6 as usize].ipv6.__in6_u.__u6_addr8).as_mut_ptr()
                    as *mut libc::c_void,
                ((*saddr).ipv6.sin6_addr.__in6_u.__u6_addr8).as_ptr()
                    as *const libc::c_void,
                16 as libc::c_int as libc::c_ulong,
            );
            memcpy(
                ntop6_strs.as_mut_ptr().offset(ndx6 as isize) as *mut libc::c_void,
                (*b).ptr as *const libc::c_void,
                (buffer_clen(b)).wrapping_add(1 as libc::c_int as libc::c_uint)
                    as libc::c_ulong,
            );
            ndx6 += 1;
            if ndx6 == 4 as libc::c_int {
                ndx6 = 0 as libc::c_int;
            }
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
