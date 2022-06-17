use ::libc;
extern "C" {
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_int(b: *mut buffer, val: intmax_t);
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __intmax_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type uint_fast32_t = libc::c_ulong;
pub type intmax_t = __intmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub ptr: *mut libc::c_char,
    pub used: uint32_t,
    pub size: uint32_t,
}
pub type http_method_t = libc::c_int;
pub const HTTP_METHOD_VERSION_CONTROL: http_method_t = 37;
pub const HTTP_METHOD_UPDATEREDIRECTREF: http_method_t = 36;
pub const HTTP_METHOD_UPDATE: http_method_t = 35;
pub const HTTP_METHOD_UNLOCK: http_method_t = 34;
pub const HTTP_METHOD_UNLINK: http_method_t = 33;
pub const HTTP_METHOD_UNCHECKOUT: http_method_t = 32;
pub const HTTP_METHOD_UNBIND: http_method_t = 31;
pub const HTTP_METHOD_SEARCH: http_method_t = 30;
pub const HTTP_METHOD_REPORT: http_method_t = 29;
pub const HTTP_METHOD_REBIND: http_method_t = 28;
pub const HTTP_METHOD_PROPPATCH: http_method_t = 27;
pub const HTTP_METHOD_PROPFIND: http_method_t = 26;
pub const HTTP_METHOD_PATCH: http_method_t = 25;
pub const HTTP_METHOD_ORDERPATCH: http_method_t = 24;
pub const HTTP_METHOD_MOVE: http_method_t = 23;
pub const HTTP_METHOD_MKWORKSPACE: http_method_t = 22;
pub const HTTP_METHOD_MKREDIRECTREF: http_method_t = 21;
pub const HTTP_METHOD_MKCOL: http_method_t = 20;
pub const HTTP_METHOD_MKCALENDAR: http_method_t = 19;
pub const HTTP_METHOD_MKACTIVITY: http_method_t = 18;
pub const HTTP_METHOD_MERGE: http_method_t = 17;
pub const HTTP_METHOD_LOCK: http_method_t = 16;
pub const HTTP_METHOD_LINK: http_method_t = 15;
pub const HTTP_METHOD_LABEL: http_method_t = 14;
pub const HTTP_METHOD_COPY: http_method_t = 13;
pub const HTTP_METHOD_CHECKOUT: http_method_t = 12;
pub const HTTP_METHOD_CHECKIN: http_method_t = 11;
pub const HTTP_METHOD_BIND: http_method_t = 10;
pub const HTTP_METHOD_BASELINE_CONTROL: http_method_t = 9;
pub const HTTP_METHOD_ACL: http_method_t = 8;
pub const HTTP_METHOD_TRACE: http_method_t = 7;
pub const HTTP_METHOD_OPTIONS: http_method_t = 6;
pub const HTTP_METHOD_CONNECT: http_method_t = 5;
pub const HTTP_METHOD_DELETE: http_method_t = 4;
pub const HTTP_METHOD_PUT: http_method_t = 3;
pub const HTTP_METHOD_POST: http_method_t = 2;
pub const HTTP_METHOD_HEAD: http_method_t = 1;
pub const HTTP_METHOD_GET: http_method_t = 0;
pub const HTTP_METHOD_UNSET: http_method_t = -1;
pub const HTTP_METHOD_PRI: http_method_t = -2;
pub type http_version_t = libc::c_int;
pub const HTTP_VERSION_2: http_version_t = 2;
pub const HTTP_VERSION_1_1: http_version_t = 1;
pub const HTTP_VERSION_1_0: http_version_t = 0;
pub const HTTP_VERSION_UNSET: http_version_t = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keyvalue {
    pub key: libc::c_int,
    pub vlen: libc::c_uint,
    pub value: *const libc::c_char,
}
static mut http_versions: [keyvalue; 4] = [keyvalue {
    key: 0,
    vlen: 0,
    value: 0 as *const libc::c_char,
}; 4];
static mut http_methods: [buffer; 40] = [buffer {
    ptr: 0 as *mut libc::c_char,
    used: 0,
    size: 0,
}; 40];
static mut http_status: [keyvalue; 62] = [keyvalue {
    key: 0,
    vlen: 0,
    value: 0 as *const libc::c_char,
}; 62];
#[no_mangle]
pub unsafe extern "C" fn http_method_buf(mut i: http_method_t) -> *const buffer {
    return if (i as libc::c_uint as libc::c_ulong)
        < (::std::mem::size_of::<[buffer; 40]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<buffer>() as libc::c_ulong)
            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
    {
        http_methods.as_ptr().offset(i as libc::c_int as isize)
    } else {
        http_methods
            .as_ptr()
            .offset(i as libc::c_int as isize)
            .offset(
                (::std::mem::size_of::<[buffer; 40]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<buffer>() as libc::c_ulong)
                    as isize,
            )
    };
}
#[inline(never)]
unsafe extern "C" fn keyvalue_from_key(
    mut kv: *const keyvalue,
    k: libc::c_int,
) -> *const keyvalue {
    while (*kv).key != k && (*kv).key != -(1 as libc::c_int) {
        kv = kv.offset(1);
    }
    return kv;
}
#[no_mangle]
pub unsafe extern "C" fn get_http_version_name(
    mut i: libc::c_int,
) -> *const libc::c_char {
    return (*keyvalue_from_key(http_versions.as_ptr(), i)).value;
}
#[no_mangle]
pub unsafe extern "C" fn get_http_method_key(
    mut s: *const libc::c_char,
    slen: size_t,
) -> http_method_t {
    if slen == 3 as libc::c_int as libc::c_ulong
        && *s.offset(0 as libc::c_int as isize) as libc::c_int == 'G' as i32
        && *s.offset(1 as libc::c_int as isize) as libc::c_int == 'E' as i32
        && *s.offset(2 as libc::c_int as isize) as libc::c_int == 'T' as i32
    {
        return HTTP_METHOD_GET;
    }
    let mut kv: *const buffer = http_methods.as_ptr().offset(1 as libc::c_int as isize);
    while (*kv).used != 0
        && (((*kv).used).wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong
            != slen
            || 0 as libc::c_int
                != memcmp(
                    (*kv).ptr as *const libc::c_void,
                    s as *const libc::c_void,
                    slen,
                ))
    {
        kv = kv.offset(1);
    }
    let i: uint_fast32_t = kv.offset_from(http_methods.as_ptr()) as libc::c_long
        as uint_fast32_t;
    return (if i
        < (::std::mem::size_of::<[buffer; 40]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<buffer>() as libc::c_ulong)
            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
    {
        i as http_method_t as libc::c_int
    } else if i
            == (::std::mem::size_of::<[buffer; 40]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<buffer>() as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong)
        {
        HTTP_METHOD_PRI as libc::c_int
    } else {
        HTTP_METHOD_UNSET as libc::c_int
    }) as http_method_t;
}
#[no_mangle]
pub unsafe extern "C" fn http_status_append(b: *mut buffer, status: libc::c_int) {
    if 200 as libc::c_int == status {
        buffer_append_string_len(
            b,
            b"200 OK\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        return;
    }
    let kv: *const keyvalue = keyvalue_from_key(http_status.as_ptr(), status);
    if (0 as libc::c_int as libc::c_uint != (*kv).vlen) as libc::c_int as libc::c_long
        != 0
    {
        buffer_append_string_len(b, (*kv).value, (*kv).vlen as size_t);
    } else {
        buffer_append_int(b, status as intmax_t);
        buffer_append_string_len(
            b,
            b" \0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn http_version_append(b: *mut buffer, version: http_version_t) {
    let kv: *const keyvalue = keyvalue_from_key(
        http_versions.as_ptr(),
        version as libc::c_int,
    );
    if (0 as libc::c_int as libc::c_uint != (*kv).vlen) as libc::c_int as libc::c_long
        != 0
    {
        buffer_append_string_len(b, (*kv).value, (*kv).vlen as size_t);
    }
}
unsafe extern "C" fn run_static_initializers() {
    http_versions = [
        {
            let mut init = keyvalue {
                key: HTTP_VERSION_2 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"HTTP/2.0\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: HTTP_VERSION_1_1 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"HTTP/1.1\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: HTTP_VERSION_1_0 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"HTTP/1.0\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: HTTP_VERSION_UNSET as libc::c_int,
                vlen: 0 as libc::c_int as libc::c_uint,
                value: 0 as *const libc::c_char,
            };
            init
        },
    ];
    http_methods = [
        {
            let mut init = buffer {
                ptr: b"GET\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"HEAD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"POST\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"PUT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"DELETE\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"CONNECT\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"OPTIONS\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"TRACE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"ACL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"BASELINE-CONTROL\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"BIND\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"CHECKIN\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"CHECKOUT\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"COPY\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"LABEL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"LINK\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"LOCK\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"MERGE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"MKACTIVITY\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"MKCALENDAR\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"MKCOL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"MKREDIRECTREF\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"MKWORKSPACE\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"MOVE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"ORDERPATCH\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"PATCH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"PROPFIND\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"PROPPATCH\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"REBIND\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"REPORT\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"SEARCH\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"UNBIND\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"UNCHECKOUT\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"UNLINK\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"UNLOCK\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"UPDATE\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"UPDATEREDIRECTREF\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"VERSION-CONTROL\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"PRI\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                used: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = buffer {
                ptr: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                used: 0 as libc::c_int as uint32_t,
                size: 0 as libc::c_int as uint32_t,
            };
            init
        },
    ];
    http_status = [
        {
            let mut init = keyvalue {
                key: 100 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"100 Continue\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 101 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"101 Switching Protocols\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 102 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"102 Processing\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 103 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"103 Early Hints\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 200 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"200 OK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 201 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"201 Created\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 202 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"202 Accepted\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 203 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 34]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"203 Non-Authoritative Information\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 204 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"204 No Content\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 205 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"205 Reset Content\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 206 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"206 Partial Content\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 207 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"207 Multi-status\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 208 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"208 Already Reported\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 226 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"226 IM Used\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 300 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"300 Multiple Choices\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 301 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"301 Moved Permanently\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 302 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"302 Found\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 303 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"303 See Other\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 304 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"304 Not Modified\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 305 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"305 Use Proxy\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 306 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"306 (Unused)\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 307 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"307 Temporary Redirect\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 308 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"308 Permanent Redirect\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 400 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"400 Bad Request\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 401 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"401 Unauthorized\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 402 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"402 Payment Required\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 403 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"403 Forbidden\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 404 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"404 Not Found\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 405 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"405 Method Not Allowed\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 406 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"406 Not Acceptable\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 407 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 34]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"407 Proxy Authentication Required\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 408 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"408 Request Timeout\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 409 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"409 Conflict\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 410 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"410 Gone\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 411 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"411 Length Required\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 412 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"412 Precondition Failed\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 413 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"413 Payload Too Large\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 414 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"414 URI Too Long\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 415 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"415 Unsupported Media Type\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 416 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"416 Range Not Satisfiable\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 417 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"417 Expectation Failed\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 421 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"421 Misdirected Request\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 422 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"422 Unprocessable Entity\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 423 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"423 Locked\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 424 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"424 Failed Dependency\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 426 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"426 Upgrade Required\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 428 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"428 Precondition Required\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 429 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"429 Too Many Requests\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 431 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"431 Request Header Fields Too Large\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 451 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 34]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"451 Unavailable For Legal Reasons\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 500 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"500 Internal Server Error\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 501 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"501 Not Implemented\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 502 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"502 Bad Gateway\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 503 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"503 Service Unavailable\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 504 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"504 Gateway Timeout\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 505 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"505 HTTP Version Not Supported\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 506 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"506 Variant Also Negotiates\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 507 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"507 Insufficient Storage\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 508 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"508 Loop Detected\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 510 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"510 Not Extended\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: 511 as libc::c_int,
                vlen: (::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                value: b"511 Network Authentication Required\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = keyvalue {
                key: -(1 as libc::c_int),
                vlen: 0 as libc::c_int as libc::c_uint,
                value: 0 as *const libc::c_char,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
