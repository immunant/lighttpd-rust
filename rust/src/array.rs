use ::libc;
extern "C" {
    fn buffer_free_ptr(b: *mut buffer);
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_str2(
        b: *mut buffer,
        s1: *const libc::c_char,
        len1: size_t,
        s2: *const libc::c_char,
        len2: size_t,
    );
    fn buffer_eq_icase_ssn(
        a: *const libc::c_char,
        b: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn memmove(
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
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type uint_fast32_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub ptr: *mut libc::c_char,
    pub used: uint32_t,
    pub size: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_unset {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
}
pub type data_type_t = libc::c_uint;
pub const TYPE_OTHER: data_type_t = 4;
pub const TYPE_CONFIG: data_type_t = 3;
pub const TYPE_INTEGER: data_type_t = 2;
pub const TYPE_ARRAY: data_type_t = 1;
pub const TYPE_STRING: data_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_methods {
    pub copy: Option::<unsafe extern "C" fn(*const data_unset) -> *mut data_unset>,
    pub free: Option::<unsafe extern "C" fn(*mut data_unset) -> ()>,
    pub insert_dup: Option::<
        unsafe extern "C" fn(*mut data_unset, *mut data_unset) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array {
    pub data: *mut *mut data_unset,
    pub sorted: *mut *mut data_unset,
    pub used: uint32_t,
    pub size: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_string {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
    pub ext: libc::c_int,
    pub value: buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_array {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
    pub value: array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_integer {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
    pub value: libc::c_int,
}
#[inline]
unsafe extern "C" fn buffer_is_unset(mut b: *const buffer) -> libc::c_int {
    return (0 as libc::c_int as libc::c_uint == (*b).used) as libc::c_int;
}
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
unsafe extern "C" fn buffer_copy_buffer(mut b: *mut buffer, mut src: *const buffer) {
    buffer_copy_string_len(b, (*src).ptr, buffer_clen(src) as size_t);
}
#[inline]
unsafe extern "C" fn buffer_clear(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn buffer_reset(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
    if (*b).size > 4096 as libc::c_int as libc::c_uint {
        buffer_free_ptr(b);
    }
}
#[cold]
unsafe extern "C" fn array_data_string_copy(
    mut s: *const data_unset,
) -> *mut data_unset {
    let mut src: *mut data_string = s as *mut data_string;
    let mut ds: *mut data_string = array_data_string_init();
    if buffer_is_unset(&mut (*src).key) == 0 {
        buffer_copy_buffer(&mut (*ds).key, &mut (*src).key);
    }
    buffer_copy_buffer(&mut (*ds).value, &mut (*src).value);
    return ds as *mut data_unset;
}
#[cold]
unsafe extern "C" fn array_data_string_insert_dup(
    mut dst: *mut data_unset,
    mut src: *mut data_unset,
) {
    let mut ds_dst: *mut data_string = dst as *mut data_string;
    let mut ds_src: *mut data_string = src as *mut data_string;
    if buffer_is_blank(&mut (*ds_dst).value) == 0 {
        buffer_append_str2(
            &mut (*ds_dst).value,
            b", \0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            (*ds_src).value.ptr,
            buffer_clen(&mut (*ds_src).value) as size_t,
        );
    } else {
        buffer_copy_buffer(&mut (*ds_dst).value, &mut (*ds_src).value);
    };
}
unsafe extern "C" fn array_data_string_free(mut du: *mut data_unset) {
    let mut ds: *mut data_string = du as *mut data_string;
    free((*ds).key.ptr as *mut libc::c_void);
    free((*ds).value.ptr as *mut libc::c_void);
    free(ds as *mut libc::c_void);
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn array_data_string_init() -> *mut data_string {
    static mut string_fn: data_methods = unsafe {
        {
            let mut init = data_methods {
                copy: Some(
                    array_data_string_copy
                        as unsafe extern "C" fn(*const data_unset) -> *mut data_unset,
                ),
                free: Some(
                    array_data_string_free as unsafe extern "C" fn(*mut data_unset) -> (),
                ),
                insert_dup: Some(
                    array_data_string_insert_dup
                        as unsafe extern "C" fn(*mut data_unset, *mut data_unset) -> (),
                ),
            };
            init
        }
    };
    let mut ds: *mut data_string = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<data_string>() as libc::c_ulong,
    ) as *mut data_string;
    if ds.is_null() {
        ck_assert_failed(
            b"src/array.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int as libc::c_uint,
            b"((void*)0) != ds\0" as *const u8 as *const libc::c_char,
        );
    }
    (*ds).type_0 = TYPE_STRING;
    (*ds).fn_0 = &string_fn;
    return ds;
}
#[cold]
unsafe extern "C" fn array_data_integer_copy(
    mut s: *const data_unset,
) -> *mut data_unset {
    let mut src: *mut data_integer = s as *mut data_integer;
    let mut di: *mut data_integer = array_data_integer_init();
    if buffer_is_unset(&mut (*src).key) == 0 {
        buffer_copy_buffer(&mut (*di).key, &mut (*src).key);
    }
    (*di).value = (*src).value;
    return di as *mut data_unset;
}
unsafe extern "C" fn array_data_integer_free(mut du: *mut data_unset) {
    let mut di: *mut data_integer = du as *mut data_integer;
    free((*di).key.ptr as *mut libc::c_void);
    free(di as *mut libc::c_void);
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn array_data_integer_init() -> *mut data_integer {
    static mut integer_fn: data_methods = unsafe {
        {
            let mut init = data_methods {
                copy: Some(
                    array_data_integer_copy
                        as unsafe extern "C" fn(*const data_unset) -> *mut data_unset,
                ),
                free: Some(
                    array_data_integer_free
                        as unsafe extern "C" fn(*mut data_unset) -> (),
                ),
                insert_dup: None,
            };
            init
        }
    };
    let mut di: *mut data_integer = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<data_integer>() as libc::c_ulong,
    ) as *mut data_integer;
    if di.is_null() {
        ck_assert_failed(
            b"src/array.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int as libc::c_uint,
            b"((void*)0) != di\0" as *const u8 as *const libc::c_char,
        );
    }
    (*di).type_0 = TYPE_INTEGER;
    (*di).fn_0 = &integer_fn;
    return di;
}
#[cold]
unsafe extern "C" fn array_data_array_copy(mut s: *const data_unset) -> *mut data_unset {
    let mut src: *mut data_array = s as *mut data_array;
    let mut da: *mut data_array = array_data_array_init();
    if buffer_is_unset(&mut (*src).key) == 0 {
        buffer_copy_buffer(&mut (*da).key, &mut (*src).key);
    }
    array_copy_array(&mut (*da).value, &mut (*src).value);
    return da as *mut data_unset;
}
unsafe extern "C" fn array_data_array_free(mut du: *mut data_unset) {
    let mut da: *mut data_array = du as *mut data_array;
    free((*da).key.ptr as *mut libc::c_void);
    array_free_data(&mut (*da).value);
    free(da as *mut libc::c_void);
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn array_data_array_init() -> *mut data_array {
    static mut array_fn: data_methods = unsafe {
        {
            let mut init = data_methods {
                copy: Some(
                    array_data_array_copy
                        as unsafe extern "C" fn(*const data_unset) -> *mut data_unset,
                ),
                free: Some(
                    array_data_array_free as unsafe extern "C" fn(*mut data_unset) -> (),
                ),
                insert_dup: None,
            };
            init
        }
    };
    let mut da: *mut data_array = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<data_array>() as libc::c_ulong,
    ) as *mut data_array;
    if da.is_null() {
        ck_assert_failed(
            b"src/array.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int as libc::c_uint,
            b"((void*)0) != da\0" as *const u8 as *const libc::c_char,
        );
    }
    (*da).type_0 = TYPE_ARRAY;
    (*da).fn_0 = &array_fn;
    return da;
}
#[cold]
unsafe extern "C" fn array_extend(a: *mut array, mut n: uint32_t) {
    if !((*a).size <= (2147483647 as libc::c_int as libc::c_uint).wrapping_sub(n)) {
        ck_assert_failed(
            b"src/array.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int as libc::c_uint,
            b"a->size <= (2147483647)-n\0" as *const u8 as *const libc::c_char,
        );
    }
    (*a).size = ((*a).size as libc::c_uint).wrapping_add(n) as uint32_t as uint32_t;
    (*a)
        .data = realloc(
        (*a).data as *mut libc::c_void,
        (::std::mem::size_of::<*mut data_unset>() as libc::c_ulong)
            .wrapping_mul((*a).size as libc::c_ulong),
    ) as *mut *mut data_unset;
    (*a)
        .sorted = realloc(
        (*a).sorted as *mut libc::c_void,
        (::std::mem::size_of::<*mut data_unset>() as libc::c_ulong)
            .wrapping_mul((*a).size as libc::c_ulong),
    ) as *mut *mut data_unset;
    if ((*a).data).is_null() {
        ck_assert_failed(
            b"src/array.c\0" as *const u8 as *const libc::c_char,
            121 as libc::c_int as libc::c_uint,
            b"a->data\0" as *const u8 as *const libc::c_char,
        );
    }
    if ((*a).sorted).is_null() {
        ck_assert_failed(
            b"src/array.c\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int as libc::c_uint,
            b"a->sorted\0" as *const u8 as *const libc::c_char,
        );
    }
    memset(
        ((*a).data).offset((*a).used as isize) as *mut libc::c_void,
        0 as libc::c_int,
        (((*a).size).wrapping_sub((*a).used) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut data_unset>() as libc::c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn array_init(mut n: uint32_t) -> *mut array {
    let mut a: *mut array = 0 as *mut array;
    a = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<array>() as libc::c_ulong,
    ) as *mut array;
    if a.is_null() {
        ck_assert_failed(
            b"src/array.c\0" as *const u8 as *const libc::c_char,
            130 as libc::c_int as libc::c_uint,
            b"a\0" as *const u8 as *const libc::c_char,
        );
    }
    if n != 0 {
        array_extend(a, n);
    }
    return a;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn array_free_data(a: *mut array) {
    if !((*a).sorted).is_null() {
        free((*a).sorted as *mut libc::c_void);
    }
    let data: *mut *mut data_unset = (*a).data;
    let sz: uint32_t = (*a).size;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < sz {
        if !(*data.offset(i as isize)).is_null() {
            ((*(**data.offset(i as isize)).fn_0).free)
                .expect("non-null function pointer")(*data.offset(i as isize));
        }
        i = i.wrapping_add(1);
    }
    free(data as *mut libc::c_void);
    (*a).data = 0 as *mut *mut data_unset;
    (*a).sorted = 0 as *mut *mut data_unset;
    (*a).used = 0 as libc::c_int as uint32_t;
    (*a).size = 0 as libc::c_int as uint32_t;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn array_copy_array(dst: *mut array, src: *const array) {
    array_free_data(dst);
    if 0 as libc::c_int as libc::c_uint == (*src).size {
        return;
    }
    array_extend(dst, (*src).size);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*src).used {
        array_insert_unique(
            dst,
            ((*(**((*src).data).offset(i as isize)).fn_0).copy)
                .expect("non-null function pointer")(*((*src).data).offset(i as isize)),
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn array_free(a: *mut array) {
    if a.is_null() {
        return;
    }
    array_free_data(a);
    free(a as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn array_reset_data_strings(a: *mut array) {
    if a.is_null() {
        return;
    }
    let data: *mut *mut data_string = (*a).data as *mut *mut data_string;
    let used: uint32_t = (*a).used;
    (*a).used = 0 as libc::c_int as uint32_t;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < used {
        let ds: *mut data_string = *data.offset(i as isize);
        buffer_reset(&mut (*ds).key);
        buffer_reset(&mut (*ds).value);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn array_caseless_compare(
    a: *const libc::c_char,
    b: *const libc::c_char,
    len: uint32_t,
) -> libc::c_int {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < len {
        let mut ca: libc::c_uint = *(a as *mut libc::c_uchar).offset(i as isize)
            as libc::c_uint;
        let mut cb: libc::c_uint = *(b as *mut libc::c_uchar).offset(i as isize)
            as libc::c_uint;
        if !(ca == cb) {
            if ca.wrapping_sub('A' as i32 as libc::c_uint)
                <= ('Z' as i32 - 'A' as i32) as libc::c_uint
            {
                ca |= 0x20 as libc::c_int as libc::c_uint;
            }
            if cb.wrapping_sub('A' as i32 as libc::c_uint)
                <= ('Z' as i32 - 'A' as i32) as libc::c_uint
            {
                cb |= 0x20 as libc::c_int as libc::c_uint;
            }
            if !(ca == cb) {
                return ca.wrapping_sub(cb) as libc::c_int;
            }
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn array_keycmp(
    a: *const libc::c_char,
    alen: uint32_t,
    b: *const libc::c_char,
    blen: uint32_t,
) -> libc::c_int {
    return if alen < blen {
        -(1 as libc::c_int)
    } else if alen > blen {
        1 as libc::c_int
    } else {
        array_caseless_compare(a, b, blen)
    };
}
#[cold]
unsafe extern "C" fn array_keycmpb(
    k: *const libc::c_char,
    klen: uint32_t,
    b: *const buffer,
) -> libc::c_int {
    return array_keycmp(
        k,
        klen,
        (*b).ptr,
        ((*b).used).wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
}
unsafe extern "C" fn array_get_index_ext(
    a: *const array,
    ext: libc::c_int,
    k: *const libc::c_char,
    klen: uint32_t,
) -> int32_t {
    let mut lower: uint_fast32_t = 0 as libc::c_int as uint_fast32_t;
    let mut upper: uint_fast32_t = (*a).used as uint_fast32_t;
    while lower != upper {
        let probe: uint_fast32_t = lower
            .wrapping_add(upper)
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
        let x: libc::c_int = (*(*((*a).sorted).offset(probe as isize)
            as *mut data_string))
            .ext;
        let e: libc::c_int = if ext | x != 0 {
            ext
        } else {
            array_keycmpb(k, klen, &mut (**((*a).sorted).offset(probe as isize)).key)
        };
        if e < x {
            upper = probe;
        } else if e > x {
            lower = probe.wrapping_add(1 as libc::c_int as libc::c_ulong);
        } else {
            return probe as int32_t
        }
    }
    return -(lower as libc::c_int) - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn array_get_element_klen_ext(
    a: *const array,
    ext: libc::c_int,
    mut key: *const libc::c_char,
    klen: uint32_t,
) -> *mut data_unset {
    let ipos: int32_t = array_get_index_ext(a, ext, key, klen);
    return if ipos >= 0 as libc::c_int {
        *((*a).sorted).offset(ipos as isize)
    } else {
        0 as *mut data_unset
    };
}
unsafe extern "C" fn array_get_index(
    a: *const array,
    k: *const libc::c_char,
    klen: uint32_t,
) -> int32_t {
    let mut lower: uint_fast32_t = 0 as libc::c_int as uint_fast32_t;
    let mut upper: uint_fast32_t = (*a).used as uint_fast32_t;
    while lower != upper {
        let mut probe: uint_fast32_t = lower
            .wrapping_add(upper)
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
        let b: *const buffer = &mut (**((*a).sorted).offset(probe as isize)).key;
        let mut cmp: libc::c_int = array_keycmp(
            k,
            klen,
            (*b).ptr,
            ((*b).used).wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        if cmp < 0 as libc::c_int {
            upper = probe;
        } else if cmp > 0 as libc::c_int {
            lower = probe.wrapping_add(1 as libc::c_int as libc::c_ulong);
        } else {
            return probe as int32_t
        }
    }
    return -(lower as libc::c_int) - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn array_get_element_klen(
    a: *const array,
    mut key: *const libc::c_char,
    klen: uint32_t,
) -> *const data_unset {
    let ipos: int32_t = array_get_index(a, key, klen);
    return if ipos >= 0 as libc::c_int {
        *((*a).sorted).offset(ipos as isize)
    } else {
        0 as *mut data_unset
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn array_get_data_unset(
    a: *const array,
    mut key: *const libc::c_char,
    klen: uint32_t,
) -> *mut data_unset {
    let ipos: int32_t = array_get_index(a, key, klen);
    return if ipos >= 0 as libc::c_int {
        *((*a).sorted).offset(ipos as isize)
    } else {
        0 as *mut data_unset
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn array_extract_element_klen(
    a: *mut array,
    mut key: *const libc::c_char,
    klen: uint32_t,
) -> *mut data_unset {
    let ipos: int32_t = array_get_index(a, key, klen);
    if ipos < 0 as libc::c_int {
        return 0 as *mut data_unset;
    }
    let entry: *mut data_unset = *((*a).sorted).offset(ipos as isize);
    (*a).used = ((*a).used).wrapping_sub(1);
    let last_ndx: uint32_t = (*a).used;
    if last_ndx != ipos as uint32_t {
        let d: *mut *mut data_unset = ((*a).sorted).offset(ipos as isize);
        memmove(
            d as *mut libc::c_void,
            d.offset(1 as libc::c_int as isize) as *const libc::c_void,
            (last_ndx.wrapping_sub(ipos as uint32_t) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut data_unset>() as libc::c_ulong),
        );
    }
    if entry != *((*a).data).offset(last_ndx as isize) {
        let mut ndx: uint32_t = 0 as libc::c_int as uint32_t;
        while entry != *((*a).data).offset(ndx as isize) {
            ndx = ndx.wrapping_add(1);
        }
        let ref mut fresh0 = *((*a).data).offset(ndx as isize);
        *fresh0 = *((*a).data).offset(last_ndx as isize);
    }
    let ref mut fresh1 = *((*a).data).offset(last_ndx as isize);
    *fresh1 = 0 as *mut data_unset;
    return entry;
}
unsafe extern "C" fn array_get_unused_element(
    a: *mut array,
    t: data_type_t,
) -> *mut data_unset {
    let du: *mut data_unset = if (*a).used < (*a).size {
        *((*a).data).offset((*a).used as isize)
    } else {
        0 as *mut data_unset
    };
    if !du.is_null() && (*du).type_0 as libc::c_uint == t as libc::c_uint {
        let ref mut fresh2 = *((*a).data).offset((*a).used as isize);
        *fresh2 = 0 as *mut data_unset;
        return du;
    }
    return 0 as *mut data_unset;
}
unsafe extern "C" fn array_insert_data_at_pos(
    a: *mut array,
    entry: *mut data_unset,
    pos: uint_fast32_t,
) -> *mut data_unset {
    if (*a).used < (*a).size {
        let prev: *mut data_unset = *((*a).data).offset((*a).used as isize);
        if (prev != 0 as *mut libc::c_void as *mut data_unset) as libc::c_int
            as libc::c_long != 0
        {
            ((*(*prev).fn_0).free).expect("non-null function pointer")(prev);
        }
    } else {
        array_extend(a, 16 as libc::c_int as uint32_t);
    }
    let fresh3 = (*a).used;
    (*a).used = ((*a).used).wrapping_add(1);
    let mut ndx: uint_fast32_t = fresh3 as uint_fast32_t;
    let ref mut fresh4 = *((*a).data).offset(ndx as isize);
    *fresh4 = entry;
    ndx = (ndx as libc::c_ulong).wrapping_sub(pos) as uint_fast32_t as uint_fast32_t;
    let d: *mut *mut data_unset = ((*a).sorted).offset(pos as isize);
    if ndx as libc::c_long != 0 {
        memmove(
            d.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            d as *const libc::c_void,
            ndx.wrapping_mul(::std::mem::size_of::<*mut data_unset>() as libc::c_ulong),
        );
    }
    *d = entry;
    return entry;
}
unsafe extern "C" fn array_insert_integer_at_pos(
    a: *mut array,
    pos: uint_fast32_t,
) -> *mut data_integer {
    let di: *mut data_integer = array_data_integer_init();
    return array_insert_data_at_pos(a, di as *mut data_unset, pos) as *mut data_integer;
}
unsafe extern "C" fn array_insert_string_at_pos(
    a: *mut array,
    pos: uint_fast32_t,
) -> *mut data_string {
    let mut ds: *mut data_string = array_get_unused_element(a, TYPE_STRING)
        as *mut data_string;
    if ds.is_null() {
        ds = array_data_string_init();
    }
    return array_insert_data_at_pos(a, ds as *mut data_unset, pos) as *mut data_string;
}
#[no_mangle]
pub unsafe extern "C" fn array_get_buf_ptr_ext(
    a: *mut array,
    ext: libc::c_int,
    k: *const libc::c_char,
    klen: uint32_t,
) -> *mut buffer {
    let mut ipos: int32_t = array_get_index_ext(a, ext, k, klen);
    if ipos >= 0 as libc::c_int {
        return &mut (*(*((*a).sorted).offset(ipos as isize) as *mut data_string)).value;
    }
    let ds: *mut data_string = array_insert_string_at_pos(
        a,
        (-ipos - 1 as libc::c_int) as uint32_t as uint_fast32_t,
    );
    (*ds).ext = ext;
    buffer_copy_string_len(&mut (*ds).key, k, klen as size_t);
    buffer_clear(&mut (*ds).value);
    return &mut (*ds).value;
}
#[no_mangle]
pub unsafe extern "C" fn array_get_int_ptr(
    a: *mut array,
    k: *const libc::c_char,
    klen: uint32_t,
) -> *mut libc::c_int {
    let mut ipos: int32_t = array_get_index(a, k, klen);
    if ipos >= 0 as libc::c_int {
        return &mut (*(*((*a).sorted).offset(ipos as isize) as *mut data_integer)).value;
    }
    let di: *mut data_integer = array_insert_integer_at_pos(
        a,
        (-ipos - 1 as libc::c_int) as uint32_t as uint_fast32_t,
    );
    buffer_copy_string_len(&mut (*di).key, k, klen as size_t);
    (*di).value = 0 as libc::c_int;
    return &mut (*di).value;
}
#[no_mangle]
pub unsafe extern "C" fn array_get_buf_ptr(
    a: *mut array,
    k: *const libc::c_char,
    klen: uint32_t,
) -> *mut buffer {
    let mut ipos: int32_t = array_get_index(a, k, klen);
    if ipos >= 0 as libc::c_int {
        return &mut (*(*((*a).sorted).offset(ipos as isize) as *mut data_string)).value;
    }
    let ds: *mut data_string = array_insert_string_at_pos(
        a,
        (-ipos - 1 as libc::c_int) as uint32_t as uint_fast32_t,
    );
    buffer_copy_string_len(&mut (*ds).key, k, klen as size_t);
    buffer_clear(&mut (*ds).value);
    return &mut (*ds).value;
}
#[no_mangle]
pub unsafe extern "C" fn array_insert_value(
    a: *mut array,
    v: *const libc::c_char,
    vlen: uint32_t,
) {
    let ds: *mut data_string = array_insert_string_at_pos(a, (*a).used as uint_fast32_t);
    buffer_clear(&mut (*ds).key);
    buffer_copy_string_len(&mut (*ds).value, v, vlen as size_t);
}
#[cold]
unsafe extern "C" fn array_find_or_insert(
    a: *mut array,
    entry: *mut data_unset,
) -> *mut *mut data_unset {
    if entry.is_null() {
        ck_assert_failed(
            b"src/array.c\0" as *const u8 as *const libc::c_char,
            436 as libc::c_int as libc::c_uint,
            b"((void*)0) != entry\0" as *const u8 as *const libc::c_char,
        );
    }
    if buffer_is_unset(&mut (*entry).key) != 0 {
        array_insert_data_at_pos(a, entry, (*a).used as uint_fast32_t);
        return 0 as *mut *mut data_unset;
    }
    let ipos: int32_t = array_get_index(
        a,
        (*entry).key.ptr,
        buffer_clen(&mut (*entry).key),
    );
    if ipos >= 0 as libc::c_int {
        return &mut *((*a).sorted).offset(ipos as isize) as *mut *mut data_unset;
    }
    array_insert_data_at_pos(
        a,
        entry,
        (-ipos - 1 as libc::c_int) as uint32_t as uint_fast32_t,
    );
    return 0 as *mut *mut data_unset;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn array_replace(a: *mut array, entry: *mut data_unset) {
    if (array_find_or_insert(a, entry)).is_null() {
        return;
    }
    let ipos: int32_t = array_get_index(
        a,
        (*entry).key.ptr,
        buffer_clen(&mut (*entry).key),
    );
    if !(ipos >= 0 as libc::c_int) {
        ck_assert_failed(
            b"src/array.c\0" as *const u8 as *const libc::c_char,
            458 as libc::c_int as libc::c_uint,
            b"ipos >= 0\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut old: *mut data_unset = *((*a).sorted).offset(ipos as isize);
    if !(old != entry) {
        ck_assert_failed(
            b"src/array.c\0" as *const u8 as *const libc::c_char,
            460 as libc::c_int as libc::c_uint,
            b"old != entry\0" as *const u8 as *const libc::c_char,
        );
    }
    let ref mut fresh5 = *((*a).sorted).offset(ipos as isize);
    *fresh5 = entry;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*a).used && *((*a).data).offset(i as isize) != old {
        i = i.wrapping_add(1);
    }
    if !(i != (*a).used) {
        ck_assert_failed(
            b"src/array.c\0" as *const u8 as *const libc::c_char,
            465 as libc::c_int as libc::c_uint,
            b"i != a->used\0" as *const u8 as *const libc::c_char,
        );
    }
    let ref mut fresh6 = *((*a).data).offset(i as isize);
    *fresh6 = entry;
    ((*(*old).fn_0).free).expect("non-null function pointer")(old);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn array_insert_unique(a: *mut array, entry: *mut data_unset) {
    let mut old: *mut *mut data_unset = 0 as *mut *mut data_unset;
    old = array_find_or_insert(a, entry);
    if !old.is_null() {
        if ((*(*entry).fn_0).insert_dup).is_some() {
            if !((**old).type_0 as libc::c_uint == (*entry).type_0 as libc::c_uint) {
                ck_assert_failed(
                    b"src/array.c\0" as *const u8 as *const libc::c_char,
                    476 as libc::c_int as libc::c_uint,
                    b"(*old)->type == entry->type\0" as *const u8 as *const libc::c_char,
                );
            }
            ((*(*entry).fn_0).insert_dup)
                .expect("non-null function pointer")(*old, entry);
        }
        ((*(*entry).fn_0).free).expect("non-null function pointer")(entry);
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn array_is_vlist(a: *const array) -> libc::c_int {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*a).used {
        let mut du: *mut data_unset = *((*a).data).offset(i as isize);
        if buffer_is_unset(&mut (*du).key) == 0
            || (*du).type_0 as libc::c_uint != TYPE_STRING as libc::c_int as libc::c_uint
        {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn array_is_kvany(a: *const array) -> libc::c_int {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*a).used {
        let mut du: *mut data_unset = *((*a).data).offset(i as isize);
        if buffer_is_unset(&mut (*du).key) != 0 {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn array_is_kvarray(a: *const array) -> libc::c_int {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*a).used {
        let mut du: *mut data_unset = *((*a).data).offset(i as isize);
        if buffer_is_unset(&mut (*du).key) != 0
            || (*du).type_0 as libc::c_uint != TYPE_ARRAY as libc::c_int as libc::c_uint
        {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn array_is_kvstring(a: *const array) -> libc::c_int {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*a).used {
        let mut du: *mut data_unset = *((*a).data).offset(i as isize);
        if buffer_is_unset(&mut (*du).key) != 0
            || (*du).type_0 as libc::c_uint != TYPE_STRING as libc::c_int as libc::c_uint
        {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn array_match_key_prefix_klen(
    a: *const array,
    s: *const libc::c_char,
    slen: uint32_t,
) -> *mut data_unset {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*a).used {
        let key: *const buffer = &mut (**((*a).data).offset(i as isize)).key;
        let klen: uint32_t = buffer_clen(key);
        if klen <= slen
            && 0 as libc::c_int
                == memcmp(
                    s as *const libc::c_void,
                    (*key).ptr as *const libc::c_void,
                    klen as libc::c_ulong,
                )
        {
            return *((*a).data).offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    return 0 as *mut data_unset;
}
#[no_mangle]
pub unsafe extern "C" fn array_match_key_prefix_nc_klen(
    a: *const array,
    s: *const libc::c_char,
    slen: uint32_t,
) -> *mut data_unset {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*a).used {
        let key: *const buffer = &mut (**((*a).data).offset(i as isize)).key;
        let klen: uint32_t = buffer_clen(key);
        if klen <= slen && buffer_eq_icase_ssn(s, (*key).ptr, klen as size_t) != 0 {
            return *((*a).data).offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    return 0 as *mut data_unset;
}
#[no_mangle]
pub unsafe extern "C" fn array_match_key_prefix(
    a: *const array,
    b: *const buffer,
) -> *mut data_unset {
    return array_match_key_prefix_klen(a, (*b).ptr, buffer_clen(b));
}
#[no_mangle]
pub unsafe extern "C" fn array_match_key_prefix_nc(
    a: *const array,
    b: *const buffer,
) -> *mut data_unset {
    return array_match_key_prefix_nc_klen(a, (*b).ptr, buffer_clen(b));
}
#[no_mangle]
pub unsafe extern "C" fn array_match_value_prefix(
    a: *const array,
    b: *const buffer,
) -> *const buffer {
    let blen: uint32_t = buffer_clen(b);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*a).used {
        let value: *const buffer = &mut (*(*((*a).data).offset(i as isize)
            as *mut data_string))
            .value;
        let vlen: uint32_t = buffer_clen(value);
        if vlen <= blen
            && 0 as libc::c_int
                == memcmp(
                    (*b).ptr as *const libc::c_void,
                    (*value).ptr as *const libc::c_void,
                    vlen as libc::c_ulong,
                )
        {
            return value;
        }
        i = i.wrapping_add(1);
    }
    return 0 as *const buffer;
}
#[no_mangle]
pub unsafe extern "C" fn array_match_value_prefix_nc(
    a: *const array,
    b: *const buffer,
) -> *const buffer {
    let blen: uint32_t = buffer_clen(b);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*a).used {
        let value: *const buffer = &mut (*(*((*a).data).offset(i as isize)
            as *mut data_string))
            .value;
        let vlen: uint32_t = buffer_clen(value);
        if vlen <= blen
            && buffer_eq_icase_ssn((*b).ptr, (*value).ptr, vlen as size_t) != 0
        {
            return value;
        }
        i = i.wrapping_add(1);
    }
    return 0 as *const buffer;
}
#[no_mangle]
pub unsafe extern "C" fn array_match_key_suffix(
    a: *const array,
    b: *const buffer,
) -> *mut data_unset {
    let blen: uint32_t = buffer_clen(b);
    let end: *const libc::c_char = ((*b).ptr).offset(blen as isize);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*a).used {
        let key: *const buffer = &mut (**((*a).data).offset(i as isize)).key;
        let klen: uint32_t = buffer_clen(key);
        if klen <= blen
            && 0 as libc::c_int
                == memcmp(
                    end.offset(-(klen as isize)) as *const libc::c_void,
                    (*key).ptr as *const libc::c_void,
                    klen as libc::c_ulong,
                )
        {
            return *((*a).data).offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    return 0 as *mut data_unset;
}
#[no_mangle]
pub unsafe extern "C" fn array_match_key_suffix_nc(
    a: *const array,
    b: *const buffer,
) -> *mut data_unset {
    let blen: uint32_t = buffer_clen(b);
    let end: *const libc::c_char = ((*b).ptr).offset(blen as isize);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*a).used {
        let key: *const buffer = &mut (**((*a).data).offset(i as isize)).key;
        let klen: uint32_t = buffer_clen(key);
        if klen <= blen
            && buffer_eq_icase_ssn(
                end.offset(-(klen as isize)),
                (*key).ptr,
                klen as size_t,
            ) != 0
        {
            return *((*a).data).offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    return 0 as *mut data_unset;
}
#[no_mangle]
pub unsafe extern "C" fn array_match_value_suffix(
    a: *const array,
    b: *const buffer,
) -> *const buffer {
    let blen: uint32_t = buffer_clen(b);
    let end: *const libc::c_char = ((*b).ptr).offset(blen as isize);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*a).used {
        let value: *const buffer = &mut (*(*((*a).data).offset(i as isize)
            as *mut data_string))
            .value;
        let vlen: uint32_t = buffer_clen(value);
        if vlen <= blen
            && 0 as libc::c_int
                == memcmp(
                    end.offset(-(vlen as isize)) as *const libc::c_void,
                    (*value).ptr as *const libc::c_void,
                    vlen as libc::c_ulong,
                )
        {
            return value;
        }
        i = i.wrapping_add(1);
    }
    return 0 as *const buffer;
}
#[no_mangle]
pub unsafe extern "C" fn array_match_value_suffix_nc(
    a: *const array,
    b: *const buffer,
) -> *const buffer {
    let blen: uint32_t = buffer_clen(b);
    let end: *const libc::c_char = ((*b).ptr).offset(blen as isize);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*a).used {
        let value: *const buffer = &mut (*(*((*a).data).offset(i as isize)
            as *mut data_string))
            .value;
        let vlen: uint32_t = buffer_clen(value);
        if vlen <= blen
            && buffer_eq_icase_ssn(
                end.offset(-(vlen as isize)),
                (*value).ptr,
                vlen as size_t,
            ) != 0
        {
            return value;
        }
        i = i.wrapping_add(1);
    }
    return 0 as *const buffer;
}
#[no_mangle]
pub unsafe extern "C" fn array_match_path_or_ext(
    a: *const array,
    b: *const buffer,
) -> *mut data_unset {
    let blen: uint32_t = buffer_clen(b);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*a).used {
        let key: *const buffer = &mut (**((*a).data).offset(i as isize)).key;
        let klen: uint32_t = buffer_clen(key);
        if klen <= blen
            && 0 as libc::c_int
                == memcmp(
                    (if *(*key).ptr as libc::c_int == '/' as i32 {
                        (*b).ptr
                    } else {
                        ((*b).ptr).offset(blen as isize).offset(-(klen as isize))
                    }) as *const libc::c_void,
                    (*key).ptr as *const libc::c_void,
                    klen as libc::c_ulong,
                )
        {
            return *((*a).data).offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    return 0 as *mut data_unset;
}
