use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type XXH_errorcode = libc::c_uint;
pub const XXH_ERROR: XXH_errorcode = 1;
pub const XXH_OK: XXH_errorcode = 0;
pub type XXH32_hash_t = uint32_t;
pub type xxh_u32 = XXH32_hash_t;
pub type XXH_alignment = libc::c_uint;
pub const XXH_unaligned: XXH_alignment = 1;
pub const XXH_aligned: XXH_alignment = 0;
pub type xxh_u8 = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH32_state_s {
    pub total_len_32: XXH32_hash_t,
    pub large_len: XXH32_hash_t,
    pub v: [XXH32_hash_t; 4],
    pub mem32: [XXH32_hash_t; 4],
    pub memsize: XXH32_hash_t,
    pub reserved: XXH32_hash_t,
}
pub type XXH32_state_t = XXH32_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH32_canonical_t {
    pub digest: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xxh_sa {
    pub x: [libc::c_char; 1],
}
#[no_mangle]
pub unsafe extern "C" fn XXH32(
    mut input: *const libc::c_void,
    mut len: size_t,
    mut seed: XXH32_hash_t,
) -> XXH32_hash_t {
    return XXH32_endian_align(input as *const xxh_u8, len, seed, XXH_unaligned);
}
#[inline(always)]
unsafe extern "C" fn XXH32_endian_align(
    mut input: *const xxh_u8,
    mut len: size_t,
    mut seed: xxh_u32,
    mut align: XXH_alignment,
) -> xxh_u32 {
    let mut h32: xxh_u32 = 0;
    input.is_null();
    if len >= 16 as libc::c_int as libc::c_ulong {
        let bEnd: *const xxh_u8 = input.offset(len as isize);
        let limit: *const xxh_u8 = bEnd.offset(-(15 as libc::c_int as isize));
        let mut v1: xxh_u32 = seed
            .wrapping_add(0x9e3779b1 as libc::c_uint)
            .wrapping_add(0x85ebca77 as libc::c_uint);
        let mut v2: xxh_u32 = seed.wrapping_add(0x85ebca77 as libc::c_uint);
        let mut v3: xxh_u32 = seed.wrapping_add(0 as libc::c_int as libc::c_uint);
        let mut v4: xxh_u32 = seed.wrapping_sub(0x9e3779b1 as libc::c_uint);
        loop {
            v1 = XXH32_round(
                v1,
                XXH_readLE32_align(input as *const libc::c_void, align),
            );
            input = input.offset(4 as libc::c_int as isize);
            v2 = XXH32_round(
                v2,
                XXH_readLE32_align(input as *const libc::c_void, align),
            );
            input = input.offset(4 as libc::c_int as isize);
            v3 = XXH32_round(
                v3,
                XXH_readLE32_align(input as *const libc::c_void, align),
            );
            input = input.offset(4 as libc::c_int as isize);
            v4 = XXH32_round(
                v4,
                XXH_readLE32_align(input as *const libc::c_void, align),
            );
            input = input.offset(4 as libc::c_int as isize);
            if !(input < limit) {
                break;
            }
        }
        h32 = (::std::intrinsics::rotate_left(v1, 1 as libc::c_int as libc::c_uint))
            .wrapping_add(
                ::std::intrinsics::rotate_left(v2, 7 as libc::c_int as libc::c_uint),
            )
            .wrapping_add(
                ::std::intrinsics::rotate_left(v3, 12 as libc::c_int as libc::c_uint),
            )
            .wrapping_add(
                ::std::intrinsics::rotate_left(v4, 18 as libc::c_int as libc::c_uint),
            );
    } else {
        h32 = seed.wrapping_add(0x165667b1 as libc::c_uint);
    }
    h32 = (h32 as libc::c_uint).wrapping_add(len as xxh_u32) as xxh_u32 as xxh_u32;
    return XXH32_finalize(h32, input, len & 15 as libc::c_int as libc::c_ulong, align);
}
unsafe extern "C" fn XXH32_finalize(
    mut h32: xxh_u32,
    mut ptr: *const xxh_u8,
    mut len: size_t,
    mut align: XXH_alignment,
) -> xxh_u32 {
    ptr.is_null();
    if 0 as libc::c_int == 0 {
        len &= 15 as libc::c_int as libc::c_ulong;
        while len >= 4 as libc::c_int as libc::c_ulong {
            h32 = (h32 as libc::c_uint)
                .wrapping_add(
                    (XXH_readLE32_align(ptr as *const libc::c_void, align))
                        .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                ) as xxh_u32 as xxh_u32;
            ptr = ptr.offset(4 as libc::c_int as isize);
            h32 = (::std::intrinsics::rotate_left(
                h32,
                17 as libc::c_int as libc::c_uint,
            ))
                .wrapping_mul(0x27d4eb2f as libc::c_uint);
            len = (len as libc::c_ulong).wrapping_sub(4 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        }
        while len > 0 as libc::c_int as libc::c_ulong {
            let fresh0 = ptr;
            ptr = ptr.offset(1);
            h32 = (h32 as libc::c_uint)
                .wrapping_add(
                    (*fresh0 as libc::c_uint).wrapping_mul(0x165667b1 as libc::c_uint),
                ) as xxh_u32 as xxh_u32;
            h32 = (::std::intrinsics::rotate_left(
                h32,
                11 as libc::c_int as libc::c_uint,
            ))
                .wrapping_mul(0x9e3779b1 as libc::c_uint);
            len = len.wrapping_sub(1);
        }
        return XXH32_avalanche(h32);
    } else {
        's_584: {
            let mut current_block_119: u64;
            match len & 15 as libc::c_int as libc::c_ulong {
                12 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::std::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    current_block_119 = 6153699775205394992;
                }
                8 => {
                    current_block_119 = 6153699775205394992;
                }
                4 => {
                    current_block_119 = 7998700586369207647;
                }
                13 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::std::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    current_block_119 = 2519682908000394751;
                }
                9 => {
                    current_block_119 = 2519682908000394751;
                }
                5 => {
                    current_block_119 = 8638443973614572636;
                }
                14 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::std::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    current_block_119 = 14259835374904613827;
                }
                10 => {
                    current_block_119 = 14259835374904613827;
                }
                6 => {
                    current_block_119 = 2634834160384886366;
                }
                15 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::std::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    current_block_119 = 8935709402465019696;
                }
                11 => {
                    current_block_119 = 8935709402465019696;
                }
                7 => {
                    current_block_119 = 8989328560971620175;
                }
                3 => {
                    current_block_119 = 2549557944114250762;
                }
                2 => {
                    current_block_119 = 14939628500602179151;
                }
                1 => {
                    current_block_119 = 17693336941790597961;
                }
                0 => {
                    current_block_119 = 15873389959211871527;
                }
                _ => {
                    break 's_584;
                }
            }
            match current_block_119 {
                6153699775205394992 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::std::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    current_block_119 = 7998700586369207647;
                }
                2519682908000394751 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::std::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    current_block_119 = 8638443973614572636;
                }
                14259835374904613827 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::std::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    current_block_119 = 2634834160384886366;
                }
                8935709402465019696 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::std::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    current_block_119 = 8989328560971620175;
                }
                _ => {}
            }
            match current_block_119 {
                2634834160384886366 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::std::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    let fresh2 = ptr;
                    ptr = ptr.offset(1);
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (*fresh2 as libc::c_uint)
                                .wrapping_mul(0x165667b1 as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    h32 = (::std::intrinsics::rotate_left(
                        h32,
                        11 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x9e3779b1 as libc::c_uint);
                    let fresh3 = ptr;
                    ptr = ptr.offset(1);
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (*fresh3 as libc::c_uint)
                                .wrapping_mul(0x165667b1 as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    h32 = (::std::intrinsics::rotate_left(
                        h32,
                        11 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x9e3779b1 as libc::c_uint);
                    return XXH32_avalanche(h32);
                }
                8638443973614572636 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::std::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    let fresh1 = ptr;
                    ptr = ptr.offset(1);
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (*fresh1 as libc::c_uint)
                                .wrapping_mul(0x165667b1 as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    h32 = (::std::intrinsics::rotate_left(
                        h32,
                        11 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x9e3779b1 as libc::c_uint);
                    return XXH32_avalanche(h32);
                }
                7998700586369207647 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::std::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    return XXH32_avalanche(h32);
                }
                8989328560971620175 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::std::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    current_block_119 = 2549557944114250762;
                }
                _ => {}
            }
            match current_block_119 {
                2549557944114250762 => {
                    let fresh4 = ptr;
                    ptr = ptr.offset(1);
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (*fresh4 as libc::c_uint)
                                .wrapping_mul(0x165667b1 as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    h32 = (::std::intrinsics::rotate_left(
                        h32,
                        11 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x9e3779b1 as libc::c_uint);
                    current_block_119 = 14939628500602179151;
                }
                _ => {}
            }
            match current_block_119 {
                14939628500602179151 => {
                    let fresh5 = ptr;
                    ptr = ptr.offset(1);
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (*fresh5 as libc::c_uint)
                                .wrapping_mul(0x165667b1 as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    h32 = (::std::intrinsics::rotate_left(
                        h32,
                        11 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x9e3779b1 as libc::c_uint);
                    current_block_119 = 17693336941790597961;
                }
                _ => {}
            }
            match current_block_119 {
                17693336941790597961 => {
                    let fresh6 = ptr;
                    ptr = ptr.offset(1);
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (*fresh6 as libc::c_uint)
                                .wrapping_mul(0x165667b1 as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    h32 = (::std::intrinsics::rotate_left(
                        h32,
                        11 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x9e3779b1 as libc::c_uint);
                }
                _ => {}
            }
            return XXH32_avalanche(h32);
        }
        return h32;
    };
}
unsafe extern "C" fn XXH32_avalanche(mut h32: xxh_u32) -> xxh_u32 {
    h32 ^= h32 >> 15 as libc::c_int;
    h32 = (h32 as libc::c_uint).wrapping_mul(0x85ebca77 as libc::c_uint) as xxh_u32
        as xxh_u32;
    h32 ^= h32 >> 13 as libc::c_int;
    h32 = (h32 as libc::c_uint).wrapping_mul(0xc2b2ae3d as libc::c_uint) as xxh_u32
        as xxh_u32;
    h32 ^= h32 >> 16 as libc::c_int;
    return h32;
}
#[inline(always)]
unsafe extern "C" fn XXH_readLE32_align(
    mut ptr: *const libc::c_void,
    mut align: XXH_alignment,
) -> xxh_u32 {
    if align as libc::c_uint == XXH_unaligned as libc::c_int as libc::c_uint {
        return XXH_readLE32(ptr)
    } else {
        return if 1 as libc::c_int != 0 {
            *(ptr as *const xxh_u32)
        } else {
            XXH_swap32(*(ptr as *const xxh_u32))
        }
    };
}
unsafe extern "C" fn XXH_swap32(mut x: xxh_u32) -> xxh_u32 {
    return x << 24 as libc::c_int & 0xff000000 as libc::c_uint
        | x << 8 as libc::c_int & 0xff0000 as libc::c_int as libc::c_uint
        | x >> 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint
        | x >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint;
}
#[inline(always)]
unsafe extern "C" fn XXH_readLE32(mut ptr: *const libc::c_void) -> xxh_u32 {
    return if 1 as libc::c_int != 0 {
        XXH_read32(ptr)
    } else {
        XXH_swap32(XXH_read32(ptr))
    };
}
unsafe extern "C" fn XXH_read32(mut memPtr: *const libc::c_void) -> xxh_u32 {
    let mut val: xxh_u32 = 0;
    XXH_memcpy(
        &mut val as *mut xxh_u32 as *mut libc::c_void,
        memPtr,
        ::std::mem::size_of::<xxh_u32>() as libc::c_ulong,
    );
    return val;
}
unsafe extern "C" fn XXH_memcpy(
    mut dest: *mut libc::c_void,
    mut src: *const libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return memcpy(dest, src, size);
}
unsafe extern "C" fn XXH32_round(mut acc: xxh_u32, mut input: xxh_u32) -> xxh_u32 {
    acc = (acc as libc::c_uint)
        .wrapping_add(input.wrapping_mul(0x85ebca77 as libc::c_uint)) as xxh_u32
        as xxh_u32;
    acc = ::std::intrinsics::rotate_left(acc, 13 as libc::c_int as libc::c_uint);
    acc = (acc as libc::c_uint).wrapping_mul(0x9e3779b1 as libc::c_uint) as xxh_u32
        as xxh_u32;
    return acc;
}
#[no_mangle]
pub unsafe extern "C" fn XXH32_createState() -> *mut XXH32_state_t {
    return XXH_malloc(::std::mem::size_of::<XXH32_state_t>() as libc::c_ulong)
        as *mut XXH32_state_t;
}
unsafe extern "C" fn XXH_malloc(mut s: size_t) -> *mut libc::c_void {
    return malloc(s);
}
#[no_mangle]
pub unsafe extern "C" fn XXH32_freeState(
    mut statePtr: *mut XXH32_state_t,
) -> XXH_errorcode {
    XXH_free(statePtr as *mut libc::c_void);
    return XXH_OK;
}
unsafe extern "C" fn XXH_free(mut p: *mut libc::c_void) {
    free(p);
}
#[no_mangle]
pub unsafe extern "C" fn XXH32_copyState(
    mut dstState: *mut XXH32_state_t,
    mut srcState: *const XXH32_state_t,
) {
    XXH_memcpy(
        dstState as *mut libc::c_void,
        srcState as *const libc::c_void,
        ::std::mem::size_of::<XXH32_state_t>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn XXH32_reset(
    mut statePtr: *mut XXH32_state_t,
    mut seed: XXH32_hash_t,
) -> XXH_errorcode {
    let mut state: XXH32_state_t = XXH32_state_t {
        total_len_32: 0,
        large_len: 0,
        v: [0; 4],
        mem32: [0; 4],
        memsize: 0,
        reserved: 0,
    };
    memset(
        &mut state as *mut XXH32_state_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<XXH32_state_t>() as libc::c_ulong,
    );
    state
        .v[0 as libc::c_int
        as usize] = seed
        .wrapping_add(0x9e3779b1 as libc::c_uint)
        .wrapping_add(0x85ebca77 as libc::c_uint);
    state.v[1 as libc::c_int as usize] = seed.wrapping_add(0x85ebca77 as libc::c_uint);
    state
        .v[2 as libc::c_int
        as usize] = seed.wrapping_add(0 as libc::c_int as libc::c_uint);
    state.v[3 as libc::c_int as usize] = seed.wrapping_sub(0x9e3779b1 as libc::c_uint);
    XXH_memcpy(
        statePtr as *mut libc::c_void,
        &mut state as *mut XXH32_state_t as *const libc::c_void,
        (::std::mem::size_of::<XXH32_state_t>() as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<XXH32_hash_t>() as libc::c_ulong),
    );
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn XXH32_update(
    mut state: *mut XXH32_state_t,
    mut input: *const libc::c_void,
    mut len: size_t,
) -> XXH_errorcode {
    if input.is_null() {
        return XXH_OK;
    }
    let mut p: *const xxh_u8 = input as *const xxh_u8;
    let bEnd: *const xxh_u8 = p.offset(len as isize);
    (*state)
        .total_len_32 = ((*state).total_len_32 as libc::c_uint)
        .wrapping_add(len as XXH32_hash_t) as XXH32_hash_t as XXH32_hash_t;
    (*state).large_len
        |= ((len >= 16 as libc::c_int as libc::c_ulong) as libc::c_int
            | ((*state).total_len_32 >= 16 as libc::c_int as libc::c_uint)
                as libc::c_int) as XXH32_hash_t;
    if ((*state).memsize as libc::c_ulong).wrapping_add(len)
        < 16 as libc::c_int as libc::c_ulong
    {
        XXH_memcpy(
            (((*state).mem32).as_mut_ptr() as *mut xxh_u8)
                .offset((*state).memsize as isize) as *mut libc::c_void,
            input,
            len,
        );
        (*state)
            .memsize = ((*state).memsize as libc::c_uint)
            .wrapping_add(len as XXH32_hash_t) as XXH32_hash_t as XXH32_hash_t;
        return XXH_OK;
    }
    if (*state).memsize != 0 {
        XXH_memcpy(
            (((*state).mem32).as_mut_ptr() as *mut xxh_u8)
                .offset((*state).memsize as isize) as *mut libc::c_void,
            input,
            (16 as libc::c_int as libc::c_uint).wrapping_sub((*state).memsize) as size_t,
        );
        let mut p32: *const xxh_u32 = ((*state).mem32).as_mut_ptr();
        (*state)
            .v[0 as libc::c_int
            as usize] = XXH32_round(
            (*state).v[0 as libc::c_int as usize],
            XXH_readLE32(p32 as *const libc::c_void),
        );
        p32 = p32.offset(1);
        (*state)
            .v[1 as libc::c_int
            as usize] = XXH32_round(
            (*state).v[1 as libc::c_int as usize],
            XXH_readLE32(p32 as *const libc::c_void),
        );
        p32 = p32.offset(1);
        (*state)
            .v[2 as libc::c_int
            as usize] = XXH32_round(
            (*state).v[2 as libc::c_int as usize],
            XXH_readLE32(p32 as *const libc::c_void),
        );
        p32 = p32.offset(1);
        (*state)
            .v[3 as libc::c_int
            as usize] = XXH32_round(
            (*state).v[3 as libc::c_int as usize],
            XXH_readLE32(p32 as *const libc::c_void),
        );
        p = p
            .offset(
                (16 as libc::c_int as libc::c_uint).wrapping_sub((*state).memsize)
                    as isize,
            );
        (*state).memsize = 0 as libc::c_int as XXH32_hash_t;
    }
    if p <= bEnd.offset(-(16 as libc::c_int as isize)) {
        let limit: *const xxh_u8 = bEnd.offset(-(16 as libc::c_int as isize));
        loop {
            (*state)
                .v[0 as libc::c_int
                as usize] = XXH32_round(
                (*state).v[0 as libc::c_int as usize],
                XXH_readLE32(p as *const libc::c_void),
            );
            p = p.offset(4 as libc::c_int as isize);
            (*state)
                .v[1 as libc::c_int
                as usize] = XXH32_round(
                (*state).v[1 as libc::c_int as usize],
                XXH_readLE32(p as *const libc::c_void),
            );
            p = p.offset(4 as libc::c_int as isize);
            (*state)
                .v[2 as libc::c_int
                as usize] = XXH32_round(
                (*state).v[2 as libc::c_int as usize],
                XXH_readLE32(p as *const libc::c_void),
            );
            p = p.offset(4 as libc::c_int as isize);
            (*state)
                .v[3 as libc::c_int
                as usize] = XXH32_round(
                (*state).v[3 as libc::c_int as usize],
                XXH_readLE32(p as *const libc::c_void),
            );
            p = p.offset(4 as libc::c_int as isize);
            if !(p <= limit) {
                break;
            }
        }
    }
    if p < bEnd {
        XXH_memcpy(
            ((*state).mem32).as_mut_ptr() as *mut libc::c_void,
            p as *const libc::c_void,
            bEnd.offset_from(p) as libc::c_long as size_t,
        );
        (*state).memsize = bEnd.offset_from(p) as libc::c_long as libc::c_uint;
    }
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn XXH32_digest(mut state: *const XXH32_state_t) -> XXH32_hash_t {
    let mut h32: xxh_u32 = 0;
    if (*state).large_len != 0 {
        h32 = (::std::intrinsics::rotate_left(
            (*state).v[0 as libc::c_int as usize],
            1 as libc::c_int as libc::c_uint,
        ))
            .wrapping_add(
                ::std::intrinsics::rotate_left(
                    (*state).v[1 as libc::c_int as usize],
                    7 as libc::c_int as libc::c_uint,
                ),
            )
            .wrapping_add(
                ::std::intrinsics::rotate_left(
                    (*state).v[2 as libc::c_int as usize],
                    12 as libc::c_int as libc::c_uint,
                ),
            )
            .wrapping_add(
                ::std::intrinsics::rotate_left(
                    (*state).v[3 as libc::c_int as usize],
                    18 as libc::c_int as libc::c_uint,
                ),
            );
    } else {
        h32 = ((*state).v[2 as libc::c_int as usize])
            .wrapping_add(0x165667b1 as libc::c_uint);
    }
    h32 = (h32 as libc::c_uint).wrapping_add((*state).total_len_32) as xxh_u32
        as xxh_u32;
    return XXH32_finalize(
        h32,
        ((*state).mem32).as_ptr() as *const xxh_u8,
        (*state).memsize as size_t,
        XXH_aligned,
    );
}
#[no_mangle]
pub unsafe extern "C" fn XXH32_canonicalFromHash(
    mut dst: *mut XXH32_canonical_t,
    mut hash: XXH32_hash_t,
) {
    hash = XXH_swap32(hash);
    XXH_memcpy(
        dst as *mut libc::c_void,
        &mut hash as *mut XXH32_hash_t as *const libc::c_void,
        ::std::mem::size_of::<XXH32_canonical_t>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn XXH32_hashFromCanonical(
    mut src: *const XXH32_canonical_t,
) -> XXH32_hash_t {
    return XXH_readBE32(src as *const libc::c_void);
}
unsafe extern "C" fn XXH_readBE32(mut ptr: *const libc::c_void) -> xxh_u32 {
    return if 1 as libc::c_int != 0 {
        XXH_swap32(XXH_read32(ptr))
    } else {
        XXH_read32(ptr)
    };
}
#[no_mangle]
pub unsafe extern "C" fn XXH_versionNumber() -> libc::c_uint {
    return (0 as libc::c_int * 100 as libc::c_int * 100 as libc::c_int
        + 8 as libc::c_int * 100 as libc::c_int + 1 as libc::c_int) as libc::c_uint;
}
