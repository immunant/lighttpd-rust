use ::libc;
extern "C" {
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
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type sha1_quadbyte = uint32_t;
pub type sha1_byte = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SHA_CTX {
    pub state: [sha1_quadbyte; 5],
    pub count: [sha1_quadbyte; 2],
    pub buffer: [sha1_byte; 64],
}
pub type SHA_CTX = _SHA_CTX;
pub type BYTE64QUAD16 = _BYTE64QUAD16;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _BYTE64QUAD16 {
    pub c: [sha1_byte; 64],
    pub l: [sha1_quadbyte; 16],
}
unsafe extern "C" fn SHA1_Transform(
    mut state: *mut sha1_quadbyte,
    mut buffer: *const sha1_byte,
) {
    let mut a: sha1_quadbyte = 0;
    let mut b: sha1_quadbyte = 0;
    let mut c: sha1_quadbyte = 0;
    let mut d: sha1_quadbyte = 0;
    let mut e: sha1_quadbyte = 0;
    let mut src: BYTE64QUAD16 = _BYTE64QUAD16 { c: [0; 64] };
    let mut block: *mut BYTE64QUAD16 = 0 as *mut BYTE64QUAD16;
    memcpy(
        (src.c).as_mut_ptr() as *mut libc::c_void,
        buffer as *const libc::c_void,
        (::std::mem::size_of::<sha1_byte>() as libc::c_ulong)
            .wrapping_mul(64 as libc::c_int as libc::c_ulong),
    );
    block = &mut src;
    a = *state.offset(0 as libc::c_int as isize);
    b = *state.offset(1 as libc::c_int as isize);
    c = *state.offset(2 as libc::c_int as isize);
    d = *state.offset(3 as libc::c_int as isize);
    e = *state.offset(4 as libc::c_int as isize);
    (*block)
        .l[0 as libc::c_int
        as usize] = ((*block).l[0 as libc::c_int as usize] << 24 as libc::c_int
        | (*block).l[0 as libc::c_int as usize] >> 32 as libc::c_int - 24 as libc::c_int)
        & 0xff00ff00 as libc::c_uint
        | ((*block).l[0 as libc::c_int as usize] << 8 as libc::c_int
            | (*block).l[0 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as sha1_quadbyte;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b & (c ^ d) ^ d)
                .wrapping_add((*block).l[0 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[1 as libc::c_int
        as usize] = ((*block).l[1 as libc::c_int as usize] << 24 as libc::c_int
        | (*block).l[1 as libc::c_int as usize] >> 32 as libc::c_int - 24 as libc::c_int)
        & 0xff00ff00 as libc::c_uint
        | ((*block).l[1 as libc::c_int as usize] << 8 as libc::c_int
            | (*block).l[1 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as sha1_quadbyte;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & (b ^ c) ^ c)
                .wrapping_add((*block).l[1 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[2 as libc::c_int
        as usize] = ((*block).l[2 as libc::c_int as usize] << 24 as libc::c_int
        | (*block).l[2 as libc::c_int as usize] >> 32 as libc::c_int - 24 as libc::c_int)
        & 0xff00ff00 as libc::c_uint
        | ((*block).l[2 as libc::c_int as usize] << 8 as libc::c_int
            | (*block).l[2 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as sha1_quadbyte;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e & (a ^ b) ^ b)
                .wrapping_add((*block).l[2 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[3 as libc::c_int
        as usize] = ((*block).l[3 as libc::c_int as usize] << 24 as libc::c_int
        | (*block).l[3 as libc::c_int as usize] >> 32 as libc::c_int - 24 as libc::c_int)
        & 0xff00ff00 as libc::c_uint
        | ((*block).l[3 as libc::c_int as usize] << 8 as libc::c_int
            | (*block).l[3 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as sha1_quadbyte;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d & (e ^ a) ^ a)
                .wrapping_add((*block).l[3 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[4 as libc::c_int
        as usize] = ((*block).l[4 as libc::c_int as usize] << 24 as libc::c_int
        | (*block).l[4 as libc::c_int as usize] >> 32 as libc::c_int - 24 as libc::c_int)
        & 0xff00ff00 as libc::c_uint
        | ((*block).l[4 as libc::c_int as usize] << 8 as libc::c_int
            | (*block).l[4 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as sha1_quadbyte;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c & (d ^ e) ^ e)
                .wrapping_add((*block).l[4 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[5 as libc::c_int
        as usize] = ((*block).l[5 as libc::c_int as usize] << 24 as libc::c_int
        | (*block).l[5 as libc::c_int as usize] >> 32 as libc::c_int - 24 as libc::c_int)
        & 0xff00ff00 as libc::c_uint
        | ((*block).l[5 as libc::c_int as usize] << 8 as libc::c_int
            | (*block).l[5 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as sha1_quadbyte;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b & (c ^ d) ^ d)
                .wrapping_add((*block).l[5 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[6 as libc::c_int
        as usize] = ((*block).l[6 as libc::c_int as usize] << 24 as libc::c_int
        | (*block).l[6 as libc::c_int as usize] >> 32 as libc::c_int - 24 as libc::c_int)
        & 0xff00ff00 as libc::c_uint
        | ((*block).l[6 as libc::c_int as usize] << 8 as libc::c_int
            | (*block).l[6 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as sha1_quadbyte;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & (b ^ c) ^ c)
                .wrapping_add((*block).l[6 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[7 as libc::c_int
        as usize] = ((*block).l[7 as libc::c_int as usize] << 24 as libc::c_int
        | (*block).l[7 as libc::c_int as usize] >> 32 as libc::c_int - 24 as libc::c_int)
        & 0xff00ff00 as libc::c_uint
        | ((*block).l[7 as libc::c_int as usize] << 8 as libc::c_int
            | (*block).l[7 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as sha1_quadbyte;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e & (a ^ b) ^ b)
                .wrapping_add((*block).l[7 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[8 as libc::c_int
        as usize] = ((*block).l[8 as libc::c_int as usize] << 24 as libc::c_int
        | (*block).l[8 as libc::c_int as usize] >> 32 as libc::c_int - 24 as libc::c_int)
        & 0xff00ff00 as libc::c_uint
        | ((*block).l[8 as libc::c_int as usize] << 8 as libc::c_int
            | (*block).l[8 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as sha1_quadbyte;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d & (e ^ a) ^ a)
                .wrapping_add((*block).l[8 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[9 as libc::c_int
        as usize] = ((*block).l[9 as libc::c_int as usize] << 24 as libc::c_int
        | (*block).l[9 as libc::c_int as usize] >> 32 as libc::c_int - 24 as libc::c_int)
        & 0xff00ff00 as libc::c_uint
        | ((*block).l[9 as libc::c_int as usize] << 8 as libc::c_int
            | (*block).l[9 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as sha1_quadbyte;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c & (d ^ e) ^ e)
                .wrapping_add((*block).l[9 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[10 as libc::c_int
        as usize] = ((*block).l[10 as libc::c_int as usize] << 24 as libc::c_int
        | (*block).l[10 as libc::c_int as usize]
            >> 32 as libc::c_int - 24 as libc::c_int) & 0xff00ff00 as libc::c_uint
        | ((*block).l[10 as libc::c_int as usize] << 8 as libc::c_int
            | (*block).l[10 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as sha1_quadbyte;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b & (c ^ d) ^ d)
                .wrapping_add((*block).l[10 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[11 as libc::c_int
        as usize] = ((*block).l[11 as libc::c_int as usize] << 24 as libc::c_int
        | (*block).l[11 as libc::c_int as usize]
            >> 32 as libc::c_int - 24 as libc::c_int) & 0xff00ff00 as libc::c_uint
        | ((*block).l[11 as libc::c_int as usize] << 8 as libc::c_int
            | (*block).l[11 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as sha1_quadbyte;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & (b ^ c) ^ c)
                .wrapping_add((*block).l[11 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[12 as libc::c_int
        as usize] = ((*block).l[12 as libc::c_int as usize] << 24 as libc::c_int
        | (*block).l[12 as libc::c_int as usize]
            >> 32 as libc::c_int - 24 as libc::c_int) & 0xff00ff00 as libc::c_uint
        | ((*block).l[12 as libc::c_int as usize] << 8 as libc::c_int
            | (*block).l[12 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as sha1_quadbyte;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e & (a ^ b) ^ b)
                .wrapping_add((*block).l[12 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[13 as libc::c_int
        as usize] = ((*block).l[13 as libc::c_int as usize] << 24 as libc::c_int
        | (*block).l[13 as libc::c_int as usize]
            >> 32 as libc::c_int - 24 as libc::c_int) & 0xff00ff00 as libc::c_uint
        | ((*block).l[13 as libc::c_int as usize] << 8 as libc::c_int
            | (*block).l[13 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as sha1_quadbyte;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d & (e ^ a) ^ a)
                .wrapping_add((*block).l[13 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[14 as libc::c_int
        as usize] = ((*block).l[14 as libc::c_int as usize] << 24 as libc::c_int
        | (*block).l[14 as libc::c_int as usize]
            >> 32 as libc::c_int - 24 as libc::c_int) & 0xff00ff00 as libc::c_uint
        | ((*block).l[14 as libc::c_int as usize] << 8 as libc::c_int
            | (*block).l[14 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as sha1_quadbyte;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c & (d ^ e) ^ e)
                .wrapping_add((*block).l[14 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[15 as libc::c_int
        as usize] = ((*block).l[15 as libc::c_int as usize] << 24 as libc::c_int
        | (*block).l[15 as libc::c_int as usize]
            >> 32 as libc::c_int - 24 as libc::c_int) & 0xff00ff00 as libc::c_uint
        | ((*block).l[15 as libc::c_int as usize] << 8 as libc::c_int
            | (*block).l[15 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as sha1_quadbyte;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b & (c ^ d) ^ d)
                .wrapping_add((*block).l[15 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(16 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(16 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(16 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(16 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(16 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(16 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(16 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(16 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(16 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & (b ^ c) ^ c)
                .wrapping_add(
                    (*block).l[(16 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(17 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(17 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(17 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(17 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(17 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(17 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(17 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(17 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(17 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e & (a ^ b) ^ b)
                .wrapping_add(
                    (*block).l[(17 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(18 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(18 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(18 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(18 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(18 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(18 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(18 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(18 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(18 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d & (e ^ a) ^ a)
                .wrapping_add(
                    (*block).l[(18 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(19 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(19 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(19 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(19 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(19 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(19 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(19 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(19 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(19 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c & (d ^ e) ^ e)
                .wrapping_add(
                    (*block).l[(19 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(20 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(20 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(20 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(20 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(20 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(20 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(20 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(20 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(20 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(
                    (*block).l[(20 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(21 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(21 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(21 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(21 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(21 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(21 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(21 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(21 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(21 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(
                    (*block).l[(21 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(22 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(22 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(22 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(22 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(22 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(22 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(22 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(22 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(22 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(
                    (*block).l[(22 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(23 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(23 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(23 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(23 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(23 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(23 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(23 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(23 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(23 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(
                    (*block).l[(23 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(24 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(24 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(24 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(24 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(24 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(24 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(24 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(24 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(24 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(
                    (*block).l[(24 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(25 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(25 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(25 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(25 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(25 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(25 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(25 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(25 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(25 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(
                    (*block).l[(25 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(26 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(26 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(26 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(26 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(26 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(26 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(26 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(26 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(26 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(
                    (*block).l[(26 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(27 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(27 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(27 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(27 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(27 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(27 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(27 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(27 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(27 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(
                    (*block).l[(27 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(28 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(28 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(28 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(28 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(28 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(28 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(28 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(28 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(28 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(
                    (*block).l[(28 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(29 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(29 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(29 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(29 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(29 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(29 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(29 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(29 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(29 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(
                    (*block).l[(29 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(30 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(30 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(30 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(30 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(30 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(30 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(30 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(30 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(30 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(
                    (*block).l[(30 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(31 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(31 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(31 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(31 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(31 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(31 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(31 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(31 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(31 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(
                    (*block).l[(31 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(32 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(32 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(32 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(32 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(32 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(32 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(32 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(32 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(32 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(
                    (*block).l[(32 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(33 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(33 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(33 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(33 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(33 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(33 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(33 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(33 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(33 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(
                    (*block).l[(33 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(34 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(34 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(34 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(34 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(34 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(34 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(34 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(34 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(34 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(
                    (*block).l[(34 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(35 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(35 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(35 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(35 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(35 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(35 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(35 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(35 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(35 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(
                    (*block).l[(35 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(36 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(36 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(36 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(36 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(36 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(36 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(36 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(36 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(36 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(
                    (*block).l[(36 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(37 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(37 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(37 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(37 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(37 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(37 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(37 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(37 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(37 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(
                    (*block).l[(37 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(38 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(38 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(38 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(38 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(38 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(38 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(38 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(38 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(38 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(
                    (*block).l[(38 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(39 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(39 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(39 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(39 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(39 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(39 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(39 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(39 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(39 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(
                    (*block).l[(39 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(40 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(40 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(40 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(40 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(40 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(40 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(40 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(40 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(40 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            ((b | c) & d | b & c)
                .wrapping_add(
                    (*block).l[(40 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(41 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(41 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(41 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(41 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(41 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(41 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(41 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(41 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(41 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            ((a | b) & c | a & b)
                .wrapping_add(
                    (*block).l[(41 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(42 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(42 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(42 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(42 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(42 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(42 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(42 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(42 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(42 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            ((e | a) & b | e & a)
                .wrapping_add(
                    (*block).l[(42 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(43 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(43 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(43 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(43 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(43 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(43 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(43 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(43 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(43 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            ((d | e) & a | d & e)
                .wrapping_add(
                    (*block).l[(43 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(44 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(44 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(44 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(44 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(44 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(44 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(44 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(44 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(44 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            ((c | d) & e | c & d)
                .wrapping_add(
                    (*block).l[(44 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(45 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(45 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(45 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(45 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(45 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(45 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(45 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(45 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(45 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            ((b | c) & d | b & c)
                .wrapping_add(
                    (*block).l[(45 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(46 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(46 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(46 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(46 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(46 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(46 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(46 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(46 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(46 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            ((a | b) & c | a & b)
                .wrapping_add(
                    (*block).l[(46 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(47 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(47 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(47 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(47 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(47 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(47 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(47 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(47 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(47 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            ((e | a) & b | e & a)
                .wrapping_add(
                    (*block).l[(47 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(48 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(48 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(48 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(48 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(48 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(48 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(48 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(48 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(48 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            ((d | e) & a | d & e)
                .wrapping_add(
                    (*block).l[(48 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(49 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(49 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(49 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(49 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(49 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(49 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(49 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(49 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(49 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            ((c | d) & e | c & d)
                .wrapping_add(
                    (*block).l[(49 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(50 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(50 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(50 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(50 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(50 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(50 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(50 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(50 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(50 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            ((b | c) & d | b & c)
                .wrapping_add(
                    (*block).l[(50 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(51 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(51 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(51 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(51 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(51 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(51 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(51 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(51 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(51 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            ((a | b) & c | a & b)
                .wrapping_add(
                    (*block).l[(51 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(52 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(52 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(52 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(52 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(52 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(52 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(52 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(52 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(52 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            ((e | a) & b | e & a)
                .wrapping_add(
                    (*block).l[(52 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(53 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(53 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(53 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(53 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(53 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(53 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(53 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(53 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(53 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            ((d | e) & a | d & e)
                .wrapping_add(
                    (*block).l[(53 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(54 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(54 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(54 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(54 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(54 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(54 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(54 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(54 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(54 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            ((c | d) & e | c & d)
                .wrapping_add(
                    (*block).l[(54 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(55 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(55 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(55 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(55 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(55 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(55 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(55 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(55 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(55 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            ((b | c) & d | b & c)
                .wrapping_add(
                    (*block).l[(55 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(56 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(56 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(56 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(56 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(56 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(56 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(56 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(56 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(56 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            ((a | b) & c | a & b)
                .wrapping_add(
                    (*block).l[(56 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(57 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(57 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(57 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(57 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(57 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(57 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(57 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(57 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(57 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            ((e | a) & b | e & a)
                .wrapping_add(
                    (*block).l[(57 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(58 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(58 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(58 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(58 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(58 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(58 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(58 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(58 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(58 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            ((d | e) & a | d & e)
                .wrapping_add(
                    (*block).l[(58 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(59 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(59 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(59 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(59 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(59 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(59 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(59 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(59 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(59 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            ((c | d) & e | c & d)
                .wrapping_add(
                    (*block).l[(59 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(60 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(60 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(60 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(60 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(60 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(60 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(60 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(60 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(60 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(
                    (*block).l[(60 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(61 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(61 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(61 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(61 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(61 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(61 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(61 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(61 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(61 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(
                    (*block).l[(61 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(62 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(62 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(62 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(62 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(62 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(62 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(62 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(62 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(62 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(
                    (*block).l[(62 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(63 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(63 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(63 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(63 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(63 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(63 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(63 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(63 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(63 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(
                    (*block).l[(63 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(64 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(64 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(64 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(64 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(64 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(64 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(64 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(64 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(64 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(
                    (*block).l[(64 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(65 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(65 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(65 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(65 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(65 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(65 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(65 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(65 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(65 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(
                    (*block).l[(65 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(66 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(66 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(66 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(66 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(66 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(66 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(66 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(66 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(66 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(
                    (*block).l[(66 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(67 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(67 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(67 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(67 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(67 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(67 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(67 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(67 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(67 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(
                    (*block).l[(67 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(68 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(68 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(68 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(68 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(68 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(68 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(68 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(68 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(68 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(
                    (*block).l[(68 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(69 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(69 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(69 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(69 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(69 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(69 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(69 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(69 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(69 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(
                    (*block).l[(69 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(70 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(70 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(70 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(70 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(70 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(70 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(70 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(70 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(70 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(
                    (*block).l[(70 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(71 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(71 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(71 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(71 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(71 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(71 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(71 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(71 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(71 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(
                    (*block).l[(71 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(72 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(72 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(72 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(72 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(72 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(72 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(72 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(72 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(72 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(
                    (*block).l[(72 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(73 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(73 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(73 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(73 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(73 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(73 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(73 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(73 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(73 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(
                    (*block).l[(73 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(74 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(74 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(74 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(74 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(74 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(74 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(74 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(74 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(74 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(
                    (*block).l[(74 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(75 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(75 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(75 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(75 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(75 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(75 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(75 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(75 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(75 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(
                    (*block).l[(75 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(76 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(76 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(76 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(76 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(76 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(76 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(76 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(76 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(76 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(
                    (*block).l[(76 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(77 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(77 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(77 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(77 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(77 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(77 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(77 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(77 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(77 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(
                    (*block).l[(77 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(78 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(78 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(78 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(78 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(78 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(78 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(78 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(78 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(78 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(
                    (*block).l[(78 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    (*block)
        .l[(79 as libc::c_int & 15 as libc::c_int)
        as usize] = ((*block)
        .l[(79 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(79 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(79 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block).l[(79 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block)
            .l[(79 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(79 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block)
                .l[(79 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block).l[(79 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(
                    (*block).l[(79 as libc::c_int & 15 as libc::c_int) as usize],
                )
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as sha1_quadbyte as sha1_quadbyte;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh0 = *state.offset(0 as libc::c_int as isize);
    *fresh0 = (*fresh0 as libc::c_uint).wrapping_add(a) as sha1_quadbyte
        as sha1_quadbyte;
    let ref mut fresh1 = *state.offset(1 as libc::c_int as isize);
    *fresh1 = (*fresh1 as libc::c_uint).wrapping_add(b) as sha1_quadbyte
        as sha1_quadbyte;
    let ref mut fresh2 = *state.offset(2 as libc::c_int as isize);
    *fresh2 = (*fresh2 as libc::c_uint).wrapping_add(c) as sha1_quadbyte
        as sha1_quadbyte;
    let ref mut fresh3 = *state.offset(3 as libc::c_int as isize);
    *fresh3 = (*fresh3 as libc::c_uint).wrapping_add(d) as sha1_quadbyte
        as sha1_quadbyte;
    let ref mut fresh4 = *state.offset(4 as libc::c_int as isize);
    *fresh4 = (*fresh4 as libc::c_uint).wrapping_add(e) as sha1_quadbyte
        as sha1_quadbyte;
    e = 0 as libc::c_int as sha1_quadbyte;
    d = e;
    c = d;
    b = c;
    a = b;
}
#[no_mangle]
pub unsafe extern "C" fn SHA1_Init(mut context: *mut SHA_CTX) {
    (*context)
        .state[0 as libc::c_int as usize] = 0x67452301 as libc::c_int as sha1_quadbyte;
    (*context).state[1 as libc::c_int as usize] = 0xefcdab89 as libc::c_uint;
    (*context).state[2 as libc::c_int as usize] = 0x98badcfe as libc::c_uint;
    (*context)
        .state[3 as libc::c_int as usize] = 0x10325476 as libc::c_int as sha1_quadbyte;
    (*context).state[4 as libc::c_int as usize] = 0xc3d2e1f0 as libc::c_uint;
    (*context).count[1 as libc::c_int as usize] = 0 as libc::c_int as sha1_quadbyte;
    (*context)
        .count[0 as libc::c_int as usize] = (*context).count[1 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn SHA1_Update(
    mut context: *mut SHA_CTX,
    mut data: *const sha1_byte,
    mut len: libc::c_uint,
) {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    j = (*context).count[0 as libc::c_int as usize] >> 3 as libc::c_int
        & 63 as libc::c_int as libc::c_uint;
    (*context)
        .count[0 as libc::c_int
        as usize] = ((*context).count[0 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(len << 3 as libc::c_int) as sha1_quadbyte as sha1_quadbyte;
    if (*context).count[0 as libc::c_int as usize] < len << 3 as libc::c_int {
        (*context)
            .count[1 as libc::c_int
            as usize] = ((*context).count[1 as libc::c_int as usize]).wrapping_add(1);
    }
    (*context)
        .count[1 as libc::c_int
        as usize] = ((*context).count[1 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(len >> 29 as libc::c_int) as sha1_quadbyte as sha1_quadbyte;
    if j.wrapping_add(len) > 63 as libc::c_int as libc::c_uint {
        i = (64 as libc::c_int as libc::c_uint).wrapping_sub(j);
        memcpy(
            &mut *((*context).buffer).as_mut_ptr().offset(j as isize) as *mut sha1_byte
                as *mut libc::c_void,
            data as *const libc::c_void,
            i as libc::c_ulong,
        );
        SHA1_Transform(
            ((*context).state).as_mut_ptr(),
            ((*context).buffer).as_mut_ptr() as *const sha1_byte,
        );
        while i.wrapping_add(63 as libc::c_int as libc::c_uint) < len {
            SHA1_Transform(((*context).state).as_mut_ptr(), &*data.offset(i as isize));
            i = i.wrapping_add(64 as libc::c_int as libc::c_uint);
        }
        j = 0 as libc::c_int as libc::c_uint;
    } else {
        i = 0 as libc::c_int as libc::c_uint;
    }
    memcpy(
        &mut *((*context).buffer).as_mut_ptr().offset(j as isize) as *mut sha1_byte
            as *mut libc::c_void,
        &*data.offset(i as isize) as *const sha1_byte as *const libc::c_void,
        len.wrapping_sub(i) as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SHA1_Final(
    mut digest: *mut sha1_byte,
    mut context: *mut SHA_CTX,
) {
    let mut i: sha1_quadbyte = 0;
    let mut j: sha1_quadbyte = 0;
    let mut finalcount: [sha1_byte; 8] = [0; 8];
    i = 0 as libc::c_int as sha1_quadbyte;
    while i < 8 as libc::c_int as libc::c_uint {
        finalcount[i
            as usize] = ((*context)
            .count[(if i >= 4 as libc::c_int as libc::c_uint {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        }) as usize]
            >> (3 as libc::c_int as libc::c_uint)
                .wrapping_sub(i & 3 as libc::c_int as libc::c_uint)
                .wrapping_mul(8 as libc::c_int as libc::c_uint)
            & 255 as libc::c_int as libc::c_uint) as sha1_byte;
        i = i.wrapping_add(1);
    }
    SHA1_Update(
        context,
        b"\x80\0" as *const u8 as *const libc::c_char as *mut sha1_byte,
        1 as libc::c_int as libc::c_uint,
    );
    while (*context).count[0 as libc::c_int as usize]
        & 504 as libc::c_int as libc::c_uint != 448 as libc::c_int as libc::c_uint
    {
        SHA1_Update(
            context,
            b"\0\0" as *const u8 as *const libc::c_char as *mut sha1_byte,
            1 as libc::c_int as libc::c_uint,
        );
    }
    SHA1_Update(context, finalcount.as_mut_ptr(), 8 as libc::c_int as libc::c_uint);
    i = 0 as libc::c_int as sha1_quadbyte;
    while i < 20 as libc::c_int as libc::c_uint {
        *digest
            .offset(
                i as isize,
            ) = ((*context).state[(i >> 2 as libc::c_int) as usize]
            >> (3 as libc::c_int as libc::c_uint)
                .wrapping_sub(i & 3 as libc::c_int as libc::c_uint)
                .wrapping_mul(8 as libc::c_int as libc::c_uint)
            & 255 as libc::c_int as libc::c_uint) as sha1_byte;
        i = i.wrapping_add(1);
    }
    j = 0 as libc::c_int as sha1_quadbyte;
    i = j;
    memset(
        ((*context).buffer).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        64 as libc::c_int as libc::c_ulong,
    );
    memset(
        ((*context).state).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        20 as libc::c_int as libc::c_ulong,
    );
    memset(
        ((*context).count).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        8 as libc::c_int as libc::c_ulong,
    );
    memset(
        &mut finalcount as *mut [sha1_byte; 8] as *mut libc::c_void,
        0 as libc::c_int,
        8 as libc::c_int as libc::c_ulong,
    );
}
