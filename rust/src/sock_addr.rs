use ::libc;
extern "C" {
    pub type fdlog_st;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    static in6addr_any: in6_addr;
    static in6addr_loopback: in6_addr;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn buffer_copy_string(b: *mut buffer, s: *const libc::c_char);
    fn buffer_append_string(b: *mut buffer, s: *const libc::c_char);
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_int(b: *mut buffer, val: intmax_t);
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn getnameinfo(
        __sa: *const sockaddr,
        __salen: socklen_t,
        __host: *mut libc::c_char,
        __hostlen: socklen_t,
        __serv: *mut libc::c_char,
        __servlen: socklen_t,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn inet_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
    ) -> libc::c_int;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    fn log_error(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        fmt: *const libc::c_char,
        _: ...
    );
    fn log_perror(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        fmt: *const libc::c_char,
        _: ...
    );
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __intmax_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type intmax_t = __intmax_t;
pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_0 = 263;
pub const IPPROTO_MPTCP: C2RustUnnamed_0 = 262;
pub const IPPROTO_RAW: C2RustUnnamed_0 = 255;
pub const IPPROTO_ETHERNET: C2RustUnnamed_0 = 143;
pub const IPPROTO_MPLS: C2RustUnnamed_0 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_0 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_0 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_0 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_0 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_0 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_0 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_0 = 92;
pub const IPPROTO_AH: C2RustUnnamed_0 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_0 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_0 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_0 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_0 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_0 = 33;
pub const IPPROTO_TP: C2RustUnnamed_0 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_0 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_0 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_0 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_0 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_0 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_0 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_0 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_0 = 1;
pub const IPPROTO_IP: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sock_addr {
    pub ipv6: sockaddr_in6,
    pub ipv4: sockaddr_in,
    pub un: sockaddr_un,
    pub plain: sockaddr,
}
pub type log_error_st = fdlog_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub ptr: *mut libc::c_char,
    pub used: uint32_t,
    pub size: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn sock_addr_get_port(
    mut saddr: *const sock_addr,
) -> libc::c_ushort {
    match (*saddr).plain.sa_family as libc::c_int {
        2 => return __bswap_16((*saddr).ipv4.sin_port),
        10 => return __bswap_16((*saddr).ipv6.sin6_port),
        _ => return 0 as libc::c_int as libc::c_ushort,
    };
}
#[no_mangle]
pub unsafe extern "C" fn sock_addr_is_addr_wildcard(
    mut saddr: *const sock_addr,
) -> libc::c_int {
    match (*saddr).plain.sa_family as libc::c_int {
        2 => {
            return ((*saddr).ipv4.sin_addr.s_addr == 0 as libc::c_int as in_addr_t)
                as libc::c_int;
        }
        10 => {
            return (memcmp(
                &(*saddr).ipv6.sin6_addr as *const in6_addr as *const libc::c_void,
                &in6addr_any as *const in6_addr as *const libc::c_void,
                ::std::mem::size_of::<in6_addr>() as libc::c_ulong,
            ) == 0) as libc::c_int;
        }
        _ => return 0 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn sock_addr_is_family_eq(
    mut saddr1: *const sock_addr,
    mut saddr2: *const sock_addr,
) -> libc::c_int {
    return ((*saddr1).plain.sa_family as libc::c_int
        == (*saddr2).plain.sa_family as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sock_addr_is_port_eq(
    mut saddr1: *const sock_addr,
    mut saddr2: *const sock_addr,
) -> libc::c_int {
    if sock_addr_is_family_eq(saddr1, saddr2) == 0 {
        return 0 as libc::c_int;
    }
    match (*saddr1).plain.sa_family as libc::c_int {
        2 => {
            return ((*saddr1).ipv4.sin_port as libc::c_int
                == (*saddr2).ipv4.sin_port as libc::c_int) as libc::c_int;
        }
        10 => {
            return ((*saddr1).ipv6.sin6_port as libc::c_int
                == (*saddr2).ipv6.sin6_port as libc::c_int) as libc::c_int;
        }
        1 => return 1 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn sock_addr_is_addr_eq(
    mut saddr1: *const sock_addr,
    mut saddr2: *const sock_addr,
) -> libc::c_int {
    if sock_addr_is_family_eq(saddr1, saddr2) == 0 {
        return 0 as libc::c_int;
    }
    match (*saddr1).plain.sa_family as libc::c_int {
        2 => {
            return ((*saddr1).ipv4.sin_addr.s_addr == (*saddr2).ipv4.sin_addr.s_addr)
                as libc::c_int;
        }
        10 => {
            return (0 as libc::c_int
                == memcmp(
                    &(*saddr1).ipv6.sin6_addr as *const in6_addr as *const libc::c_void,
                    &(*saddr2).ipv6.sin6_addr as *const in6_addr as *const libc::c_void,
                    ::std::mem::size_of::<in6_addr>() as libc::c_ulong,
                )) as libc::c_int;
        }
        1 => {
            return (0 as libc::c_int
                == strcmp(
                    ((*saddr1).un.sun_path).as_ptr(),
                    ((*saddr2).un.sun_path).as_ptr(),
                )) as libc::c_int;
        }
        _ => return 0 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn sock_addr_is_addr_eq_bits(
    mut a: *const sock_addr,
    mut b: *const sock_addr,
    mut bits: libc::c_int,
) -> libc::c_int {
    match (*a).plain.sa_family as libc::c_int {
        2 => {
            let mut nm: uint32_t = 0;
            if bits > 32 as libc::c_int {
                bits = 32 as libc::c_int;
            }
            nm = __bswap_32(
                !((1 as libc::c_uint)
                    << 32 as libc::c_int
                        - (if 0 as libc::c_int != bits {
                            bits
                        } else {
                            32 as libc::c_int
                        }))
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            );
            if (*b).plain.sa_family as libc::c_int == 2 as libc::c_int {
                return ((*a).ipv4.sin_addr.s_addr & nm == (*b).ipv4.sin_addr.s_addr & nm)
                    as libc::c_int
            } else {
                if (*b).plain.sa_family as libc::c_int == 10 as libc::c_int
                    && ({
                        let mut __a: *const in6_addr = &(*b).ipv6.sin6_addr
                            as *const in6_addr;
                        ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize]
                            == 0 as libc::c_int as libc::c_uint
                            && (*__a).__in6_u.__u6_addr32[1 as libc::c_int as usize]
                                == 0 as libc::c_int as libc::c_uint
                            && (*__a).__in6_u.__u6_addr32[2 as libc::c_int as usize]
                                == __bswap_32(0xffff as libc::c_int as __uint32_t))
                            as libc::c_int
                    }) != 0
                {
                    let mut x: in_addr_t = (*b)
                        .ipv6
                        .sin6_addr
                        .__in6_u
                        .__u6_addr32[3 as libc::c_int as usize];
                    return ((*a).ipv4.sin_addr.s_addr & nm == x & nm) as libc::c_int;
                }
            }
            return 0 as libc::c_int;
        }
        10 => {
            if bits > 128 as libc::c_int {
                bits = 128 as libc::c_int;
            }
            if (*b).plain.sa_family as libc::c_int == 10 as libc::c_int {
                let mut c: *mut uint8_t = &*((*a).ipv6.sin6_addr.__in6_u.__u6_addr8)
                    .as_ptr()
                    .offset(0 as libc::c_int as isize) as *const uint8_t as *mut uint8_t;
                let mut d: *mut uint8_t = &*((*b).ipv6.sin6_addr.__in6_u.__u6_addr8)
                    .as_ptr()
                    .offset(0 as libc::c_int as isize) as *const uint8_t as *mut uint8_t;
                let mut match_0: libc::c_int = 0;
                loop {
                    match_0 = if bits >= 8 as libc::c_int {
                        let fresh0 = c;
                        c = c.offset(1);
                        let fresh1 = d;
                        d = d.offset(1);
                        (*fresh0 as libc::c_int == *fresh1 as libc::c_int) as libc::c_int
                    } else {
                        (*c as libc::c_int >> 8 as libc::c_int - bits
                            == *d as libc::c_int >> 8 as libc::c_int - bits)
                            as libc::c_int
                    };
                    if !(match_0 != 0
                        && {
                            bits -= 8 as libc::c_int;
                            bits > 0 as libc::c_int
                        })
                    {
                        break;
                    }
                }
                return match_0;
            } else {
                if (*b).plain.sa_family as libc::c_int == 2 as libc::c_int
                    && ({
                        let mut __a: *const in6_addr = &(*a).ipv6.sin6_addr
                            as *const in6_addr;
                        ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize]
                            == 0 as libc::c_int as libc::c_uint
                            && (*__a).__in6_u.__u6_addr32[1 as libc::c_int as usize]
                                == 0 as libc::c_int as libc::c_uint
                            && (*__a).__in6_u.__u6_addr32[2 as libc::c_int as usize]
                                == __bswap_32(0xffff as libc::c_int as __uint32_t))
                            as libc::c_int
                    }) != 0
                {
                    let mut nm_0: uint32_t = if bits < 128 as libc::c_int {
                        __bswap_32(
                            !(!(0 as libc::c_uint)
                                >> (if bits > 96 as libc::c_int {
                                    bits - 96 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })),
                        )
                    } else {
                        !(0 as libc::c_uint)
                    };
                    let mut x_0: in_addr_t = (*a)
                        .ipv6
                        .sin6_addr
                        .__in6_u
                        .__u6_addr32[3 as libc::c_int as usize];
                    return (x_0 & nm_0 == (*b).ipv4.sin_addr.s_addr & nm_0)
                        as libc::c_int;
                }
            }
            return 0 as libc::c_int;
        }
        _ => return 0 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn sock_addr_set_port(
    saddr: *mut sock_addr,
    port: libc::c_ushort,
) {
    match (*saddr).plain.sa_family as libc::c_int {
        2 => {
            (*saddr).ipv4.sin_port = __bswap_16(port);
        }
        10 => {
            (*saddr).ipv6.sin6_port = __bswap_16(port);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn sock_addr_assign(
    saddr: *mut sock_addr,
    mut family: libc::c_int,
    mut nport: libc::c_ushort,
    naddr: *const libc::c_void,
) -> libc::c_int {
    match family {
        2 => {
            memset(
                &mut (*saddr).ipv4 as *mut sockaddr_in as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
            );
            (*saddr).ipv4.sin_family = 2 as libc::c_int as sa_family_t;
            (*saddr).ipv4.sin_port = nport;
            memcpy(
                &mut (*saddr).ipv4.sin_addr as *mut in_addr as *mut libc::c_void,
                naddr,
                4 as libc::c_int as libc::c_ulong,
            );
            return 0 as libc::c_int;
        }
        10 => {
            memset(
                &mut (*saddr).ipv6 as *mut sockaddr_in6 as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong,
            );
            (*saddr).ipv6.sin6_family = 10 as libc::c_int as sa_family_t;
            (*saddr).ipv6.sin6_port = nport;
            memcpy(
                &mut (*saddr).ipv6.sin6_addr as *mut in6_addr as *mut libc::c_void,
                naddr,
                16 as libc::c_int as libc::c_ulong,
            );
            return 0 as libc::c_int;
        }
        1 => {
            let mut len: size_t = (strlen(naddr as *mut libc::c_char))
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            if len > ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong {
                *__errno_location() = 36 as libc::c_int;
                return -(1 as libc::c_int);
            }
            memset(
                &mut (*saddr).un as *mut sockaddr_un as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong,
            );
            (*saddr).un.sun_family = 1 as libc::c_int as sa_family_t;
            memcpy(((*saddr).un.sun_path).as_mut_ptr() as *mut libc::c_void, naddr, len);
            return 0 as libc::c_int;
        }
        _ => {
            *__errno_location() = 97 as libc::c_int;
            return -(1 as libc::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn sock_addr_inet_pton(
    saddr: *mut sock_addr,
    str: *const libc::c_char,
    mut family: libc::c_int,
    mut port: libc::c_ushort,
) -> libc::c_int {
    match family {
        2 => {
            memset(
                &mut (*saddr).ipv4 as *mut sockaddr_in as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
            );
            (*saddr).ipv4.sin_family = 2 as libc::c_int as sa_family_t;
            (*saddr).ipv4.sin_port = __bswap_16(port);
            return inet_pton(
                2 as libc::c_int,
                str,
                &mut (*saddr).ipv4.sin_addr as *mut in_addr as *mut libc::c_void,
            );
        }
        10 => {
            memset(
                &mut (*saddr).ipv6 as *mut sockaddr_in6 as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong,
            );
            (*saddr).ipv6.sin6_family = 10 as libc::c_int as sa_family_t;
            (*saddr).ipv6.sin6_port = __bswap_16(port);
            return inet_pton(
                10 as libc::c_int,
                str,
                &mut (*saddr).ipv6.sin6_addr as *mut in6_addr as *mut libc::c_void,
            );
        }
        _ => {
            *__errno_location() = 97 as libc::c_int;
            return -(1 as libc::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn sock_addr_inet_ntop(
    saddr: *const sock_addr,
    buf: *mut libc::c_char,
    mut sz: socklen_t,
) -> *const libc::c_char {
    match (*saddr).plain.sa_family as libc::c_int {
        2 => {
            return inet_ntop(
                2 as libc::c_int,
                &(*saddr).ipv4.sin_addr as *const in_addr as *const libc::c_void,
                buf,
                sz,
            );
        }
        10 => {
            return inet_ntop(
                10 as libc::c_int,
                &(*saddr).ipv6.sin6_addr as *const in6_addr as *const libc::c_void,
                buf,
                sz,
            );
        }
        1 => return ((*saddr).un.sun_path).as_ptr(),
        _ => {
            *__errno_location() = 97 as libc::c_int;
            return 0 as *const libc::c_char;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn sock_addr_inet_ntop_copy_buffer(
    b: *mut buffer,
    saddr: *const sock_addr,
) -> libc::c_int {
    let mut buf: [libc::c_char; 108] = [0; 108];
    let mut s: *const libc::c_char = sock_addr_inet_ntop(
        saddr,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong as socklen_t,
    );
    if s.is_null() {
        return -(1 as libc::c_int);
    }
    buffer_copy_string(b, s);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sock_addr_inet_ntop_append_buffer(
    b: *mut buffer,
    saddr: *const sock_addr,
) -> libc::c_int {
    let mut buf: [libc::c_char; 108] = [0; 108];
    let mut s: *const libc::c_char = sock_addr_inet_ntop(
        saddr,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong as socklen_t,
    );
    if s.is_null() {
        return -(1 as libc::c_int);
    }
    buffer_append_string(b, s);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sock_addr_stringify_append_buffer(
    b: *mut buffer,
    saddr: *const sock_addr,
) -> libc::c_int {
    match (*saddr).plain.sa_family as libc::c_int {
        2 => {
            if 0 as libc::c_int != sock_addr_inet_ntop_append_buffer(b, saddr) {
                return -(1 as libc::c_int);
            }
            buffer_append_string_len(
                b,
                b":\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            buffer_append_int(b, __bswap_16((*saddr).ipv4.sin_port) as intmax_t);
            return 0 as libc::c_int;
        }
        10 => {
            buffer_append_string_len(
                b,
                b"[\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            if 0 as libc::c_int != sock_addr_inet_ntop_append_buffer(b, saddr) {
                buffer_truncate(
                    b,
                    (buffer_clen(b)).wrapping_sub(1 as libc::c_int as libc::c_uint),
                );
                return -(1 as libc::c_int);
            }
            buffer_append_string_len(
                b,
                b"]:\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            buffer_append_int(b, __bswap_16((*saddr).ipv6.sin6_port) as intmax_t);
            return 0 as libc::c_int;
        }
        1 => {
            buffer_append_string(b, ((*saddr).un.sun_path).as_ptr());
            return 0 as libc::c_int;
        }
        _ => return 0 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn sock_addr_nameinfo_append_buffer(
    b: *mut buffer,
    saddr: *const sock_addr,
    errh: *mut log_error_st,
) -> libc::c_int {
    match (*saddr).plain.sa_family as libc::c_int {
        2 => {
            let mut hbuf: [libc::c_char; 256] = [0; 256];
            let mut rc: libc::c_int = getnameinfo(
                &(*saddr).ipv4 as *const sockaddr_in as *const sockaddr,
                ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
                hbuf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                    as socklen_t,
                0 as *mut libc::c_char,
                0 as libc::c_int as socklen_t,
                0 as libc::c_int,
            );
            if 0 as libc::c_int != rc {
                log_error(
                    errh,
                    b"src/sock_addr.c\0" as *const u8 as *const libc::c_char,
                    393 as libc::c_int as libc::c_uint,
                    b"NOTICE: getnameinfo failed; using ip-address instead: %s\0"
                        as *const u8 as *const libc::c_char,
                    gai_strerror(rc),
                );
                sock_addr_inet_ntop_append_buffer(b, saddr);
            } else {
                buffer_append_string(b, hbuf.as_mut_ptr());
            }
            return 0 as libc::c_int;
        }
        10 => {
            let mut hbuf_0: [libc::c_char; 256] = [0; 256];
            let mut rc_0: libc::c_int = getnameinfo(
                &(*saddr).ipv6 as *const sockaddr_in6 as *const sockaddr,
                ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as socklen_t,
                hbuf_0.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                    as socklen_t,
                0 as *mut libc::c_char,
                0 as libc::c_int as socklen_t,
                0 as libc::c_int,
            );
            if 0 as libc::c_int != rc_0 {
                log_error(
                    errh,
                    b"src/sock_addr.c\0" as *const u8 as *const libc::c_char,
                    410 as libc::c_int as libc::c_uint,
                    b"NOTICE: getnameinfo failed; using ip-address instead: %s\0"
                        as *const u8 as *const libc::c_char,
                    gai_strerror(rc_0),
                );
                buffer_append_string_len(
                    b,
                    b"[\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
                sock_addr_inet_ntop_append_buffer(b, saddr);
                buffer_append_string_len(
                    b,
                    b"]\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
            } else {
                buffer_append_string(b, hbuf_0.as_mut_ptr());
            }
            return 0 as libc::c_int;
        }
        _ => {
            log_error(
                errh,
                b"src/sock_addr.c\0" as *const u8 as *const libc::c_char,
                424 as libc::c_int as libc::c_uint,
                b"ERROR: unsupported address-type\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn sock_addr_from_str_hints(
    saddr: *mut sock_addr,
    len: *mut socklen_t,
    str: *const libc::c_char,
    mut family: libc::c_int,
    mut port: libc::c_ushort,
    errh: *mut log_error_st,
) -> libc::c_int {
    match family {
        0 => {
            if 0 as libc::c_int
                == strcmp(str, b"localhost\0" as *const u8 as *const libc::c_char)
            {
                memset(
                    saddr as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
                );
                (*saddr).ipv4.sin_family = 2 as libc::c_int as sa_family_t;
                (*saddr)
                    .ipv4
                    .sin_addr
                    .s_addr = __bswap_32(0x7f000001 as libc::c_int as in_addr_t);
                (*saddr).ipv4.sin_port = __bswap_16(port);
                *len = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
                    as socklen_t;
                return 1 as libc::c_int;
            } else {
                let mut hints: addrinfo = addrinfo {
                    ai_flags: 0,
                    ai_family: 0,
                    ai_socktype: 0,
                    ai_protocol: 0,
                    ai_addrlen: 0,
                    ai_addr: 0 as *mut sockaddr,
                    ai_canonname: 0 as *mut libc::c_char,
                    ai_next: 0 as *mut addrinfo,
                };
                let mut res: *mut addrinfo = 0 as *mut addrinfo;
                let mut rc: libc::c_int = 0;
                memset(
                    &mut hints as *mut addrinfo as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
                );
                hints.ai_family = 0 as libc::c_int;
                hints.ai_socktype = SOCK_STREAM as libc::c_int;
                hints.ai_protocol = IPPROTO_TCP as libc::c_int;
                rc = getaddrinfo(str, 0 as *const libc::c_char, &mut hints, &mut res);
                if 0 as libc::c_int != rc {
                    log_error(
                        errh,
                        b"src/sock_addr.c\0" as *const u8 as *const libc::c_char,
                        454 as libc::c_int as libc::c_uint,
                        b"getaddrinfo failed: %s '%s'\0" as *const u8
                            as *const libc::c_char,
                        gai_strerror(rc),
                        str,
                    );
                    return 0 as libc::c_int;
                }
                memcpy(
                    saddr as *mut libc::c_void,
                    (*res).ai_addr as *const libc::c_void,
                    (*res).ai_addrlen as libc::c_ulong,
                );
                freeaddrinfo(res);
                if 10 as libc::c_int == (*saddr).plain.sa_family as libc::c_int {
                    (*saddr).ipv6.sin6_port = __bswap_16(port);
                    *len = ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong
                        as socklen_t;
                } else {
                    (*saddr).ipv4.sin_port = __bswap_16(port);
                    *len = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
                        as socklen_t;
                }
                return 1 as libc::c_int;
            }
        }
        10 => {
            memset(
                saddr as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong,
            );
            (*saddr).ipv6.sin6_family = 10 as libc::c_int as sa_family_t;
            if 0 as libc::c_int
                == strcmp(str, b"::\0" as *const u8 as *const libc::c_char)
            {
                (*saddr).ipv6.sin6_addr = in6addr_any;
            } else if 0 as libc::c_int
                    == strcmp(str, b"::1\0" as *const u8 as *const libc::c_char)
                {
                (*saddr).ipv6.sin6_addr = in6addr_loopback;
            } else {
                let mut hints_0: addrinfo = addrinfo {
                    ai_flags: 0,
                    ai_family: 0,
                    ai_socktype: 0,
                    ai_protocol: 0,
                    ai_addrlen: 0,
                    ai_addr: 0 as *mut sockaddr,
                    ai_canonname: 0 as *mut libc::c_char,
                    ai_next: 0 as *mut addrinfo,
                };
                let mut res_0: *mut addrinfo = 0 as *mut addrinfo;
                let mut rc_0: libc::c_int = 0;
                memset(
                    &mut hints_0 as *mut addrinfo as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
                );
                hints_0.ai_family = 10 as libc::c_int;
                hints_0.ai_socktype = SOCK_STREAM as libc::c_int;
                hints_0.ai_protocol = IPPROTO_TCP as libc::c_int;
                rc_0 = getaddrinfo(
                    str,
                    0 as *const libc::c_char,
                    &mut hints_0,
                    &mut res_0,
                );
                if 0 as libc::c_int != rc_0 {
                    hints_0.ai_family = 2 as libc::c_int;
                    if -(9 as libc::c_int) == rc_0
                        && 0 as libc::c_int
                            == getaddrinfo(
                                str,
                                0 as *const libc::c_char,
                                &mut hints_0,
                                &mut res_0,
                            )
                    {
                        memcpy(
                            saddr as *mut libc::c_void,
                            (*res_0).ai_addr as *const libc::c_void,
                            (*res_0).ai_addrlen as libc::c_ulong,
                        );
                        (*saddr).ipv4.sin_family = 2 as libc::c_int as sa_family_t;
                        (*saddr).ipv4.sin_port = __bswap_16(port);
                        *len = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
                            as socklen_t;
                        freeaddrinfo(res_0);
                        return 1 as libc::c_int;
                    }
                    log_error(
                        errh,
                        b"src/sock_addr.c\0" as *const u8 as *const libc::c_char,
                        510 as libc::c_int as libc::c_uint,
                        b"getaddrinfo failed: %s '%s'\0" as *const u8
                            as *const libc::c_char,
                        gai_strerror(rc_0),
                        str,
                    );
                    return 0 as libc::c_int;
                }
                memcpy(
                    saddr as *mut libc::c_void,
                    (*res_0).ai_addr as *const libc::c_void,
                    (*res_0).ai_addrlen as libc::c_ulong,
                );
                freeaddrinfo(res_0);
            }
            (*saddr).ipv6.sin6_port = __bswap_16(port);
            *len = ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as socklen_t;
            return 1 as libc::c_int;
        }
        2 => {
            memset(
                saddr as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
            );
            (*saddr).ipv4.sin_family = 2 as libc::c_int as sa_family_t;
            if 0 as libc::c_int
                == strcmp(str, b"0.0.0.0\0" as *const u8 as *const libc::c_char)
            {
                (*saddr)
                    .ipv4
                    .sin_addr
                    .s_addr = __bswap_32(0 as libc::c_int as in_addr_t);
            } else if 0 as libc::c_int
                    == strcmp(str, b"127.0.0.1\0" as *const u8 as *const libc::c_char)
                {
                (*saddr)
                    .ipv4
                    .sin_addr
                    .s_addr = __bswap_32(0x7f000001 as libc::c_int as in_addr_t);
            } else {
                let mut hints_1: addrinfo = addrinfo {
                    ai_flags: 0,
                    ai_family: 0,
                    ai_socktype: 0,
                    ai_protocol: 0,
                    ai_addrlen: 0,
                    ai_addr: 0 as *mut sockaddr,
                    ai_canonname: 0 as *mut libc::c_char,
                    ai_next: 0 as *mut addrinfo,
                };
                let mut res_1: *mut addrinfo = 0 as *mut addrinfo;
                let mut rc_1: libc::c_int = 0;
                memset(
                    &mut hints_1 as *mut addrinfo as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
                );
                hints_1.ai_family = 2 as libc::c_int;
                hints_1.ai_socktype = SOCK_STREAM as libc::c_int;
                hints_1.ai_protocol = IPPROTO_TCP as libc::c_int;
                rc_1 = getaddrinfo(
                    str,
                    0 as *const libc::c_char,
                    &mut hints_1,
                    &mut res_1,
                );
                if 0 as libc::c_int != rc_1 {
                    log_error(
                        errh,
                        b"src/sock_addr.c\0" as *const u8 as *const libc::c_char,
                        543 as libc::c_int as libc::c_uint,
                        b"getaddrinfo failed: %s '%s'\0" as *const u8
                            as *const libc::c_char,
                        gai_strerror(rc_1),
                        str,
                    );
                    return 0 as libc::c_int;
                }
                memcpy(
                    saddr as *mut libc::c_void,
                    (*res_1).ai_addr as *const libc::c_void,
                    (*res_1).ai_addrlen as libc::c_ulong,
                );
                freeaddrinfo(res_1);
            }
            (*saddr).ipv4.sin_port = __bswap_16(port);
            *len = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
            return 1 as libc::c_int;
        }
        1 => {
            memset(
                saddr as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong,
            );
            (*saddr).un.sun_family = 1 as libc::c_int as sa_family_t;
            let mut hostlen: size_t = (strlen(str))
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            if hostlen > ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong {
                log_error(
                    errh,
                    b"src/sock_addr.c\0" as *const u8 as *const libc::c_char,
                    584 as libc::c_int as libc::c_uint,
                    b"unix socket filename too long: %s\0" as *const u8
                        as *const libc::c_char,
                    str,
                );
                return 0 as libc::c_int;
            }
            memcpy(
                ((*saddr).un.sun_path).as_mut_ptr() as *mut libc::c_void,
                str as *const libc::c_void,
                hostlen,
            );
            *len = (((*(0 as *mut sockaddr_un)).sun_path).as_mut_ptr() as size_t)
                .wrapping_add(strlen(((*saddr).un.sun_path).as_mut_ptr()))
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as socklen_t;
            return 1 as libc::c_int;
        }
        _ => {
            log_error(
                errh,
                b"src/sock_addr.c\0" as *const u8 as *const libc::c_char,
                604 as libc::c_int as libc::c_uint,
                b"address family unsupported: %d\0" as *const u8 as *const libc::c_char,
                family,
            );
            return 0 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn sock_addr_from_str_numeric(
    saddr: *mut sock_addr,
    str: *const libc::c_char,
    errh: *mut log_error_st,
) -> libc::c_int {
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut addrlist: *mut addrinfo = 0 as *mut addrinfo;
    let mut result: libc::c_int = 0;
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_flags = 0x4 as libc::c_int | 0x400 as libc::c_int;
    *__errno_location() = 0 as libc::c_int;
    result = getaddrinfo(str, 0 as *const libc::c_char, &mut hints, &mut addrlist);
    if result != 0 as libc::c_int {
        log_perror(
            errh,
            b"src/sock_addr.c\0" as *const u8 as *const libc::c_char,
            638 as libc::c_int as libc::c_uint,
            b"could not parse ip address %s because %s\0" as *const u8
                as *const libc::c_char,
            str,
            gai_strerror(result),
        );
        return result;
    } else {
        if addrlist.is_null() {
            log_error(
                errh,
                b"src/sock_addr.c\0" as *const u8 as *const libc::c_char,
                643 as libc::c_int as libc::c_uint,
                b"Problem in parsing ip address %s:succeeded, but no information returned\0"
                    as *const u8 as *const libc::c_char,
                str,
            );
            return -(1 as libc::c_int);
        } else {
            match (*addrlist).ai_family {
                2 => {
                    memcpy(
                        &mut (*saddr).ipv4 as *mut sockaddr_in as *mut libc::c_void,
                        (*addrlist).ai_addr as *const libc::c_void,
                        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
                    );
                    if !(2 as libc::c_int == (*saddr).plain.sa_family as libc::c_int) {
                        ck_assert_failed(
                            b"src/sock_addr.c\0" as *const u8 as *const libc::c_char,
                            650 as libc::c_int as libc::c_uint,
                            b"2 == saddr->plain.sa_family\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
                10 => {
                    memcpy(
                        &mut (*saddr).ipv6 as *mut sockaddr_in6 as *mut libc::c_void,
                        (*addrlist).ai_addr as *const libc::c_void,
                        ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong,
                    );
                    if !(10 as libc::c_int == (*saddr).plain.sa_family as libc::c_int) {
                        ck_assert_failed(
                            b"src/sock_addr.c\0" as *const u8 as *const libc::c_char,
                            654 as libc::c_int as libc::c_uint,
                            b"10 == saddr->plain.sa_family\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
                _ => {
                    log_error(
                        errh,
                        b"src/sock_addr.c\0" as *const u8 as *const libc::c_char,
                        657 as libc::c_int as libc::c_uint,
                        b"Problem in parsing ip address %s:succeeded, but unknown family\0"
                            as *const u8 as *const libc::c_char,
                        str,
                    );
                    result = -(1 as libc::c_int);
                }
            }
        }
    }
    freeaddrinfo(addrlist);
    return (0 as libc::c_int == result) as libc::c_int;
}
