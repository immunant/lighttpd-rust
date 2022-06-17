use ::libc;
extern "C" {
    pub type fdlog_st;
    pub type fdevents;
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_eq_icase_ssn(
        a: *const libc::c_char,
        b: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
    fn buffer_eq_slen(
        b: *const buffer,
        s: *const libc::c_char,
        slen: size_t,
    ) -> libc::c_int;
    fn buffer_is_equal(a: *const buffer, b: *const buffer) -> libc::c_int;
    fn buffer_append_path_len(b: *mut buffer, a: *const libc::c_char, alen: size_t);
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn array_get_element_klen(
        a: *const array,
        key: *const libc::c_char,
        klen: uint32_t,
    ) -> *const data_unset;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    static mut log_monotonic_secs: unix_time64_t;
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
    fn fdevent_fdnode_event_del(ev: *mut fdevents, fdn: *mut fdnode);
    fn fdevent_fdnode_event_set(
        ev: *mut fdevents,
        fdn: *mut fdnode,
        events: libc::c_int,
    );
    fn fdevent_register(
        ev: *mut fdevents,
        fd: libc::c_int,
        handler: fdevent_handler,
        ctx: *mut libc::c_void,
    ) -> *mut fdnode;
    fn fdevent_unregister(ev: *mut fdevents, fd: libc::c_int);
    fn fdevent_open_cloexec(
        pathname: *const libc::c_char,
        symlinks: libc::c_int,
        flags: libc::c_int,
        mode: mode_t,
    ) -> libc::c_int;
    fn http_etag_create(etag: *mut buffer, st: *const stat, flags: libc::c_int);
    fn splaytree_splay(t: *mut splay_tree, key: libc::c_int) -> *mut splay_tree;
    fn splaytree_insert(
        t: *mut splay_tree,
        key: libc::c_int,
        data: *mut libc::c_void,
    ) -> *mut splay_tree;
    fn splaytree_delete(t: *mut splay_tree, key: libc::c_int) -> *mut splay_tree;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn inotify_add_watch(
        __fd: libc::c_int,
        __name: *const libc::c_char,
        __mask: uint32_t,
    ) -> libc::c_int;
    fn inotify_rm_watch(__fd: libc::c_int, __wd: libc::c_int) -> libc::c_int;
    fn inotify_init1(__flags: libc::c_int) -> libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __ino64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ino_t = __ino64_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint32_t = __uint32_t;
pub type uintptr_t = libc::c_ulong;
pub type unix_time64_t = time_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fdnode_st {
    pub handler: fdevent_handler,
    pub ctx: *mut libc::c_void,
    pub fd: libc::c_int,
    pub events: libc::c_int,
    pub fde_ndx: libc::c_int,
}
pub type fdevent_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> handler_t,
>;
pub type handler_t = libc::c_uint;
pub const HANDLER_ERROR: handler_t = 4;
pub const HANDLER_WAIT_FOR_EVENT: handler_t = 3;
pub const HANDLER_COMEBACK: handler_t = 2;
pub const HANDLER_FINISHED: handler_t = 1;
pub const HANDLER_GO_ON: handler_t = 0;
pub type fdnode = fdnode_st;
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
pub type stat_cache_st = stat;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat_cache_entry {
    pub name: buffer,
    pub stat_ts: unix_time64_t,
    pub fd: libc::c_int,
    pub refcnt: libc::c_int,
    pub fam_dir: *mut libc::c_void,
    pub etag: buffer,
    pub content_type: buffer,
    pub st: stat,
}
pub const STAT_CACHE_ENGINE_NONE: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat_cache {
    pub stat_cache_engine: libc::c_int,
    pub files: *mut splay_tree,
    pub scf: *mut stat_cache_fam,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat_cache_fam {
    pub dirs: *mut splay_tree,
    pub wds: *mut splay_tree,
    pub errh: *mut log_error_st,
    pub ev: *mut fdevents,
    pub fdn: *mut fdnode,
    pub fd: libc::c_int,
}
pub type splay_tree = tree_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tree_node {
    pub left: *mut tree_node,
    pub right: *mut tree_node,
    pub key: libc::c_int,
    pub data: *mut libc::c_void,
}
pub const STAT_CACHE_ENGINE_FAM: C2RustUnnamed = 2;
pub const STAT_CACHE_ENGINE_INOTIFY: C2RustUnnamed = 2;
pub const STAT_CACHE_ENGINE_SIMPLE: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inotify_event {
    pub wd: libc::c_int,
    pub mask: uint32_t,
    pub cookie: uint32_t,
    pub len: uint32_t,
    pub name: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fam_dir_entry {
    pub name: buffer,
    pub refcnt: libc::c_int,
    pub req: FAMRequest,
    pub stat_ts: unix_time64_t,
    pub st_dev: dev_t,
    pub st_ino: ino_t,
    pub fam_parent: *mut fam_dir_entry,
}
pub type FAMRequest = libc::c_int;
pub const FAMMoved: FAMCodes = 6;
pub const FAMDeleted: FAMCodes = 2;
pub const FAMChanged: FAMCodes = 1;
pub const FAMCreated: FAMCodes = 5;
pub const IN_CLOEXEC: C2RustUnnamed_0 = 524288;
pub const IN_NONBLOCK: C2RustUnnamed_0 = 2048;
pub type C2RustUnnamed = libc::c_uint;
pub const STAT_CACHE_ENGINE_KQUEUE: C2RustUnnamed = 2;
pub type C2RustUnnamed_0 = libc::c_uint;
pub type FAMCodes = libc::c_uint;
#[inline]
unsafe extern "C" fn buffer_clear(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
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
unsafe extern "C" fn buffer_truncate(mut b: *mut buffer, mut len: uint32_t) {
    *((*b).ptr).offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    (*b).used = len.wrapping_add(1 as libc::c_int as libc::c_uint);
}
#[inline]
unsafe extern "C" fn djbhash(
    mut str: *const libc::c_char,
    len: uint32_t,
    mut hash: uint32_t,
) -> uint32_t {
    let s: *const libc::c_uchar = str as *const libc::c_uchar;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < len {
        hash = (hash << 5 as libc::c_int).wrapping_add(hash)
            ^ *s.offset(i as isize) as libc::c_uint;
        i = i.wrapping_add(1);
    }
    return hash;
}
#[inline]
unsafe extern "C" fn splaytree_djbhash(
    mut str: *const libc::c_char,
    len: uint32_t,
) -> int32_t {
    return (djbhash(str, len, 5381 as libc::c_int as uint32_t)
        & !((1 as libc::c_int as uint32_t) << 31 as libc::c_int)) as int32_t;
}
static mut sc: stat_cache = stat_cache {
    stat_cache_engine: 0,
    files: 0 as *const splay_tree as *mut splay_tree,
    scf: 0 as *const stat_cache_fam as *mut stat_cache_fam,
};
unsafe extern "C" fn stat_cache_sptree_find(
    sptree: *mut *mut splay_tree,
    name: *const libc::c_char,
    mut len: uint32_t,
) -> *mut libc::c_void {
    let ndx: libc::c_int = splaytree_djbhash(name, len);
    *sptree = splaytree_splay(*sptree, ndx);
    return if !(*sptree).is_null() && (**sptree).key == ndx {
        (**sptree).data
    } else {
        0 as *mut libc::c_void
    };
}
unsafe extern "C" fn fam_dir_entry_init(
    mut name: *const libc::c_char,
    mut len: size_t,
) -> *mut fam_dir_entry {
    let fam_dir: *mut fam_dir_entry = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<fam_dir_entry>() as libc::c_ulong,
    ) as *mut fam_dir_entry;
    if fam_dir.is_null() {
        ck_assert_failed(
            b"src/stat_cache.c\0" as *const u8 as *const libc::c_char,
            244 as libc::c_int as libc::c_uint,
            b"((void*)0) != fam_dir\0" as *const u8 as *const libc::c_char,
        );
    }
    buffer_copy_string_len(&mut (*fam_dir).name, name, len);
    (*fam_dir).refcnt = 0 as libc::c_int;
    return fam_dir;
}
unsafe extern "C" fn fam_dir_entry_free(mut fam_dir: *mut fam_dir_entry) {
    if fam_dir.is_null() {
        return;
    }
    free((*fam_dir).name.ptr as *mut libc::c_void);
    free(fam_dir as *mut libc::c_void);
}
unsafe extern "C" fn fam_dir_invalidate_node(mut fam_dir: *mut fam_dir_entry) {
    (*fam_dir).stat_ts = 0 as libc::c_int as unix_time64_t;
    if !((*fam_dir).fam_parent).is_null() {
        (*(*fam_dir).fam_parent).refcnt -= 1;
        (*fam_dir).fam_parent = 0 as *mut fam_dir_entry;
    }
}
unsafe extern "C" fn fam_dir_tag_refcnt(
    mut t: *mut splay_tree,
    mut keys: *mut libc::c_int,
    mut ndx: *mut libc::c_int,
) {
    if *ndx == 512 as libc::c_int {
        return;
    }
    if !((*t).left).is_null() {
        fam_dir_tag_refcnt((*t).left, keys, ndx);
    }
    if !((*t).right).is_null() {
        fam_dir_tag_refcnt((*t).right, keys, ndx);
    }
    if *ndx == 512 as libc::c_int {
        return;
    }
    let fam_dir: *mut fam_dir_entry = (*t).data as *mut fam_dir_entry;
    if 0 as libc::c_int == (*fam_dir).refcnt {
        fam_dir_invalidate_node(fam_dir);
        let fresh0 = *ndx;
        *ndx = *ndx + 1;
        *keys.offset(fresh0 as isize) = (*t).key;
    }
}
#[inline(never)]
unsafe extern "C" fn fam_dir_periodic_cleanup() {
    let scf: *mut stat_cache_fam = sc.scf;
    let mut max_ndx: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut keys: [libc::c_int; 512] = [0; 512];
    while !((*scf).dirs).is_null() {
        max_ndx = 0 as libc::c_int;
        fam_dir_tag_refcnt((*scf).dirs, keys.as_mut_ptr(), &mut max_ndx);
        i = 0 as libc::c_int;
        while i < max_ndx {
            let ndx: libc::c_int = keys[i as usize];
            (*scf).dirs = splaytree_splay((*scf).dirs, ndx);
            let mut node: *mut splay_tree = (*scf).dirs;
            if !node.is_null() && (*node).key == ndx {
                let mut fam_dir: *mut fam_dir_entry = (*node).data as *mut fam_dir_entry;
                (*scf).dirs = splaytree_delete((*scf).dirs, ndx);
                (*scf).wds = splaytree_delete((*scf).wds, (*fam_dir).req);
                inotify_rm_watch((*scf).fd, (*fam_dir).req);
                fam_dir_entry_free(fam_dir);
            }
            i += 1;
        }
        if !(max_ndx as libc::c_ulong
            == (::std::mem::size_of::<[libc::c_int; 512]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong))
        {
            break;
        }
    }
}
unsafe extern "C" fn fam_dir_invalidate_tree(
    mut t: *mut splay_tree,
    mut name: *const libc::c_char,
    mut len: size_t,
) {
    if !((*t).left).is_null() {
        fam_dir_invalidate_tree((*t).left, name, len);
    }
    if !((*t).right).is_null() {
        fam_dir_invalidate_tree((*t).right, name, len);
    }
    let fam_dir: *mut fam_dir_entry = (*t).data as *mut fam_dir_entry;
    let b: *const buffer = &mut (*fam_dir).name;
    let mut blen: size_t = buffer_clen(b) as size_t;
    if blen > len && *((*b).ptr).offset(len as isize) as libc::c_int == '/' as i32
        && 0 as libc::c_int
            == memcmp((*b).ptr as *const libc::c_void, name as *const libc::c_void, len)
    {
        fam_dir_invalidate_node(fam_dir);
    }
}
unsafe extern "C" fn stat_cache_handle_fdevent_in(mut scf: *mut stat_cache_fam) {
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut rd: libc::c_int = 0;
    loop {
        rd = read(
            (*scf).fd,
            buf.as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        ) as libc::c_int;
        if rd <= 0 as libc::c_int {
            if -(1 as libc::c_int) == rd && *__errno_location() != 4 as libc::c_int
                && *__errno_location() != 11 as libc::c_int
            {
                log_perror(
                    (*scf).errh,
                    b"src/stat_cache.c\0" as *const u8 as *const libc::c_char,
                    370 as libc::c_int as libc::c_uint,
                    b"inotify error\0" as *const u8 as *const libc::c_char,
                );
            }
            break;
        } else {
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < rd {
                let in_0: *mut inotify_event = (buf.as_mut_ptr() as uintptr_t)
                    .wrapping_add(i as libc::c_ulong) as *mut inotify_event;
                let mut len: uint32_t = (*in_0).len;
                if len as libc::c_ulong
                    > ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                {
                    break;
                }
                i = (i as libc::c_ulong)
                    .wrapping_add(
                        (::std::mem::size_of::<inotify_event>() as libc::c_ulong)
                            .wrapping_add(len as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                if i > rd {
                    break;
                }
                if (*in_0).mask & 0x100 as libc::c_int as libc::c_uint != 0 {
                    continue;
                }
                if (*in_0).mask & 0x4000 as libc::c_int as libc::c_uint != 0 {
                    log_error(
                        (*scf).errh,
                        b"src/stat_cache.c\0" as *const u8 as *const libc::c_char,
                        385 as libc::c_int as libc::c_uint,
                        b"inotify queue overflow\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    (*scf).wds = splaytree_splay((*scf).wds, (*in_0).wd);
                    if ((*scf).wds).is_null() || (*(*scf).wds).key != (*in_0).wd {
                        continue;
                    }
                    let mut fam_dir: *mut fam_dir_entry = (*(*scf).wds).data
                        as *mut fam_dir_entry;
                    if !fam_dir.is_null() {
                        if (*fam_dir).req != (*in_0).wd {
                            continue;
                        }
                        let mut code: libc::c_int = 0 as libc::c_int;
                        if (*in_0).mask
                            & (0x4 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint
                            != 0
                        {
                            code = FAMChanged as libc::c_int;
                        } else if (*in_0).mask
                                & (0x200 as libc::c_int | 0x400 as libc::c_int
                                    | 0x2000 as libc::c_int) as libc::c_uint != 0
                            {
                            code = FAMDeleted as libc::c_int;
                        } else if (*in_0).mask
                                & (0x800 as libc::c_int | 0x40 as libc::c_int)
                                    as libc::c_uint != 0
                            {
                            code = FAMMoved as libc::c_int;
                        }
                        if len != 0 {
                            loop {
                                len = len.wrapping_sub(1);
                                if !(len != 0
                                    && *((*in_0).name)
                                        .as_mut_ptr()
                                        .offset(
                                            len.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                        ) as libc::c_int == '\u{0}' as i32)
                                {
                                    break;
                                }
                            }
                        }
                        stat_cache_handle_fdevent_fn(
                            scf,
                            fam_dir,
                            ((*in_0).name).as_mut_ptr(),
                            len,
                            code,
                        );
                    }
                }
            }
            if !((rd as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<inotify_event>() as libc::c_ulong)
                .wrapping_add(255 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                > ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
            {
                break;
            }
        }
    };
}
unsafe extern "C" fn stat_cache_handle_fdevent_fn(
    scf: *mut stat_cache_fam,
    mut fam_dir: *mut fam_dir_entry,
    fn_0: *const libc::c_char,
    fnlen: uint32_t,
    mut code: libc::c_int,
) {
    if fnlen != 0 {
        let n: *mut buffer = &mut (*fam_dir).name;
        let mut fam_link: *mut fam_dir_entry = 0 as *mut fam_dir_entry;
        let mut len: uint32_t = 0;
        match code {
            5 => return,
            1 | 2 | 6 => {
                len = buffer_clen(n);
                buffer_append_path_len(n, fn_0, fnlen as size_t);
                stat_cache_invalidate_entry((*n).ptr, buffer_clen(n));
                fam_link = stat_cache_sptree_find(
                    &mut (*scf).dirs,
                    (*n).ptr,
                    buffer_clen(n),
                ) as *mut fam_dir_entry;
                if !fam_link.is_null() && buffer_is_equal(&mut (*fam_link).name, n) == 0
                {
                    fam_link = 0 as *mut fam_dir_entry;
                }
                buffer_truncate(n, len);
                if !fam_link.is_null() {
                    stat_cache_invalidate_entry((*n).ptr, len);
                    code = FAMDeleted as libc::c_int;
                    fam_dir = fam_link;
                } else {
                    return
                }
            }
            _ => return,
        }
    }
    match code {
        1 => {
            stat_cache_invalidate_entry(
                (*fam_dir).name.ptr,
                buffer_clen(&mut (*fam_dir).name),
            );
        }
        2 | 6 => {
            stat_cache_delete_tree(
                (*fam_dir).name.ptr,
                buffer_clen(&mut (*fam_dir).name),
            );
            fam_dir_invalidate_node(fam_dir);
            if !((*scf).dirs).is_null() {
                fam_dir_invalidate_tree(
                    (*scf).dirs,
                    (*fam_dir).name.ptr,
                    buffer_clen(&mut (*fam_dir).name) as size_t,
                );
            }
            fam_dir_periodic_cleanup();
        }
        _ => {}
    };
}
unsafe extern "C" fn stat_cache_handle_fdevent(
    mut ctx: *mut libc::c_void,
    mut revent: libc::c_int,
) -> handler_t {
    let scf: *mut stat_cache_fam = ctx as *mut stat_cache_fam;
    if revent & 0x1 as libc::c_int != 0 {
        stat_cache_handle_fdevent_in(scf);
    }
    if revent & (0x10 as libc::c_int | 0x2000 as libc::c_int) != 0 {
        log_error(
            (*scf).errh,
            b"src/stat_cache.c\0" as *const u8 as *const libc::c_char,
            556 as libc::c_int as libc::c_uint,
            b"FAM connection closed; disabling stat_cache.\0" as *const u8
                as *const libc::c_char,
        );
        fdevent_fdnode_event_del((*scf).ev, (*scf).fdn);
        fdevent_unregister((*scf).ev, (*scf).fd);
        (*scf).fdn = 0 as *mut fdnode;
        close((*scf).fd);
        (*scf).fd = -(1 as libc::c_int);
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn stat_cache_init_fam(
    mut ev: *mut fdevents,
    mut errh: *mut log_error_st,
) -> *mut stat_cache_fam {
    let mut scf: *mut stat_cache_fam = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<stat_cache_fam>() as libc::c_ulong,
    ) as *mut stat_cache_fam;
    if scf.is_null() {
        ck_assert_failed(
            b"src/stat_cache.c\0" as *const u8 as *const libc::c_char,
            574 as libc::c_int as libc::c_uint,
            b"scf\0" as *const u8 as *const libc::c_char,
        );
    }
    (*scf).fd = -(1 as libc::c_int);
    (*scf).ev = ev;
    (*scf).errh = errh;
    (*scf).fd = inotify_init1(IN_NONBLOCK as libc::c_int | IN_CLOEXEC as libc::c_int);
    if (*scf).fd < 0 as libc::c_int {
        log_perror(
            errh,
            b"src/stat_cache.c\0" as *const u8 as *const libc::c_char,
            582 as libc::c_int as libc::c_uint,
            b"inotify_init1()\0" as *const u8 as *const libc::c_char,
        );
        free(scf as *mut libc::c_void);
        return 0 as *mut stat_cache_fam;
    }
    (*scf)
        .fdn = fdevent_register(
        (*scf).ev,
        (*scf).fd,
        Some(
            stat_cache_handle_fdevent
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> handler_t,
        ),
        scf as *mut libc::c_void,
    );
    fdevent_fdnode_event_set(
        (*scf).ev,
        (*scf).fdn,
        0x1 as libc::c_int | 0x2000 as libc::c_int,
    );
    return scf;
}
unsafe extern "C" fn stat_cache_free_fam(mut scf: *mut stat_cache_fam) {
    if scf.is_null() {
        return;
    }
    while !((*scf).wds).is_null() {
        let mut node: *mut splay_tree = (*scf).wds;
        (*scf).wds = splaytree_delete((*scf).wds, (*node).key);
    }
    while !((*scf).dirs).is_null() {
        let mut node_0: *mut splay_tree = (*scf).dirs;
        fam_dir_entry_free((*node_0).data as *mut fam_dir_entry);
        (*scf).dirs = splaytree_delete((*scf).dirs, (*node_0).key);
    }
    if -(1 as libc::c_int) != (*scf).fd {
        close((*scf).fd);
    }
    free(scf as *mut libc::c_void);
}
unsafe extern "C" fn fam_dir_monitor(
    mut scf: *mut stat_cache_fam,
    mut fn_0: *mut libc::c_char,
    mut dirlen: uint32_t,
    mut st: *mut stat,
) -> *mut fam_dir_entry {
    if ((*scf).fdn).is_null() {
        return 0 as *mut fam_dir_entry;
    }
    let fn_is_dir: libc::c_int = ((*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int;
    if *fn_0.offset(dirlen.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
        as libc::c_int == '/' as i32
    {
        dirlen = dirlen.wrapping_sub(1);
    }
    if 0 as libc::c_int as libc::c_uint == dirlen {
        dirlen = 1 as libc::c_int as uint32_t;
    }
    if fn_is_dir == 0 {
        loop {
            dirlen = dirlen.wrapping_sub(1);
            if !(*fn_0.offset(dirlen as isize) as libc::c_int != '/' as i32) {
                break;
            }
        }
        if 0 as libc::c_int as libc::c_uint == dirlen {
            dirlen = 1 as libc::c_int as uint32_t;
        }
    }
    let mut dir_ndx: libc::c_int = splaytree_djbhash(fn_0, dirlen);
    let mut fam_dir: *mut fam_dir_entry = 0 as *mut fam_dir_entry;
    (*scf).dirs = splaytree_splay((*scf).dirs, dir_ndx);
    if !((*scf).dirs).is_null() && (*(*scf).dirs).key == dir_ndx {
        fam_dir = (*(*scf).dirs).data as *mut fam_dir_entry;
        if buffer_eq_slen(&mut (*fam_dir).name, fn_0, dirlen as size_t) == 0 {
            return 0 as *mut fam_dir_entry;
        }
    }
    let cur_ts: unix_time64_t = log_monotonic_secs;
    let mut lst: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut ck_dir: libc::c_int = fn_is_dir;
    if fn_is_dir == 0
        && (fam_dir.is_null()
            || cur_ts - (*fam_dir).stat_ts >= 16 as libc::c_int as libc::c_long)
    {
        ck_dir = 1 as libc::c_int;
        *fn_0.offset(dirlen as isize) = '\u{0}' as i32 as libc::c_char;
        if 0 as libc::c_int != lstat(fn_0, &mut lst) {
            *fn_0.offset(dirlen as isize) = '/' as i32 as libc::c_char;
            return 0 as *mut fam_dir_entry;
        }
        if !(lst.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint)
        {
            st = &mut lst;
        } else if 0 as libc::c_int != stat(fn_0, st) {
            *fn_0.offset(dirlen as isize) = '/' as i32 as libc::c_char;
            return 0 as *mut fam_dir_entry;
        }
        *fn_0.offset(dirlen as isize) = '/' as i32 as libc::c_char;
    }
    let mut ck_lnk: libc::c_int = (0 as *mut libc::c_void as *mut fam_dir_entry
        == fam_dir) as libc::c_int;
    if ck_dir != 0 && !fam_dir.is_null() {
        if (*st).st_dev != (*fam_dir).st_dev || (*st).st_ino != (*fam_dir).st_ino {
            ck_lnk = 1 as libc::c_int;
            if !((*scf).dirs).is_null() {
                fam_dir_invalidate_tree((*scf).dirs, fn_0, dirlen as size_t);
            }
            if fn_is_dir == 0 {
                stat_cache_update_entry(fn_0, dirlen, st, 0 as *const buffer);
            }
            stat_cache_invalidate_dir_tree(fn_0, dirlen as size_t);
            (*scf).wds = splaytree_delete((*scf).wds, (*fam_dir).req);
            if 0 as libc::c_int != inotify_rm_watch((*scf).fd, (*fam_dir).req)
                || {
                    (*fam_dir)
                        .req = inotify_add_watch(
                        (*scf).fd,
                        (*fam_dir).name.ptr,
                        (0x4 as libc::c_int | 0x100 as libc::c_int | 0x200 as libc::c_int
                            | 0x400 as libc::c_int | 0x2 as libc::c_int
                            | 0x800 as libc::c_int | 0x40 as libc::c_int
                            | 0x4000000 as libc::c_int | 0x1000000 as libc::c_int)
                            as uint32_t,
                    );
                    0 as libc::c_int
                        != ((*fam_dir).req < 0 as libc::c_int) as libc::c_int
                }
            {
                (*fam_dir).stat_ts = 0 as libc::c_int as unix_time64_t;
                return 0 as *mut fam_dir_entry;
            }
            (*fam_dir).st_dev = (*st).st_dev;
            (*fam_dir).st_ino = (*st).st_ino;
            (*scf)
                .wds = splaytree_insert(
                (*scf).wds,
                (*fam_dir).req,
                fam_dir as *mut libc::c_void,
            );
        }
        (*fam_dir).stat_ts = cur_ts;
    }
    if fam_dir.is_null() {
        fam_dir = fam_dir_entry_init(fn_0, dirlen as size_t);
        (*fam_dir)
            .req = inotify_add_watch(
            (*scf).fd,
            (*fam_dir).name.ptr,
            (0x4 as libc::c_int | 0x100 as libc::c_int | 0x200 as libc::c_int
                | 0x400 as libc::c_int | 0x2 as libc::c_int | 0x800 as libc::c_int
                | 0x40 as libc::c_int | 0x4000000 as libc::c_int
                | 0x1000000 as libc::c_int) as uint32_t,
        );
        if 0 as libc::c_int != ((*fam_dir).req < 0 as libc::c_int) as libc::c_int {
            log_perror(
                (*scf).errh,
                b"src/stat_cache.c\0" as *const u8 as *const libc::c_char,
                745 as libc::c_int as libc::c_uint,
                b"monitoring dir failed: %s file: %s\0" as *const u8
                    as *const libc::c_char,
                (*fam_dir).name.ptr,
                fn_0,
            );
            fam_dir_entry_free(fam_dir);
            return 0 as *mut fam_dir_entry;
        }
        (*scf)
            .dirs = splaytree_insert((*scf).dirs, dir_ndx, fam_dir as *mut libc::c_void);
        (*scf)
            .wds = splaytree_insert(
            (*scf).wds,
            (*fam_dir).req,
            fam_dir as *mut libc::c_void,
        );
        (*fam_dir).stat_ts = cur_ts;
        (*fam_dir).st_dev = (*st).st_dev;
        (*fam_dir).st_ino = (*st).st_ino;
    }
    if ck_lnk != 0 {
        if fn_is_dir != 0 {
            let mut e: libc::c_char = *fn_0.offset(dirlen as isize);
            *fn_0.offset(dirlen as isize) = '\u{0}' as i32 as libc::c_char;
            if 0 as libc::c_int != lstat(fn_0, &mut lst) {
                *fn_0.offset(dirlen as isize) = e;
                return 0 as *mut fam_dir_entry;
            }
            *fn_0.offset(dirlen as isize) = e;
        }
        if !((*fam_dir).fam_parent).is_null() {
            (*(*fam_dir).fam_parent).refcnt -= 1;
            (*fam_dir).fam_parent = 0 as *mut fam_dir_entry;
        }
        if lst.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
        {
            (*fam_dir).fam_parent = fam_dir_monitor(scf, fn_0, dirlen, &mut lst);
        }
    }
    (*fam_dir).refcnt += 1;
    return fam_dir;
}
unsafe extern "C" fn stat_cache_entry_init() -> *mut stat_cache_entry {
    let mut sce: *mut stat_cache_entry = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<stat_cache_entry>() as libc::c_ulong,
    ) as *mut stat_cache_entry;
    if sce.is_null() {
        ck_assert_failed(
            b"src/stat_cache.c\0" as *const u8 as *const libc::c_char,
            797 as libc::c_int as libc::c_uint,
            b"((void*)0) != sce\0" as *const u8 as *const libc::c_char,
        );
    }
    (*sce).fd = -(1 as libc::c_int);
    (*sce).refcnt = 1 as libc::c_int;
    return sce;
}
unsafe extern "C" fn stat_cache_entry_free(mut data: *mut libc::c_void) {
    let mut sce: *mut stat_cache_entry = data as *mut stat_cache_entry;
    if sce.is_null() {
        return;
    }
    (*sce).refcnt -= 1;
    if (*sce).refcnt != 0 {
        return;
    }
    if !((*sce).fam_dir).is_null() {
        let ref mut fresh1 = (*((*sce).fam_dir as *mut fam_dir_entry)).refcnt;
        *fresh1 -= 1;
    }
    free((*sce).name.ptr as *mut libc::c_void);
    free((*sce).etag.ptr as *mut libc::c_void);
    if (*sce).content_type.size != 0 {
        free((*sce).content_type.ptr as *mut libc::c_void);
    }
    if (*sce).fd >= 0 as libc::c_int {
        close((*sce).fd);
    }
    free(sce as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn stat_cache_entry_refchg(
    mut data: *mut libc::c_void,
    mut mod_0: libc::c_int,
) {
    let sce: *mut stat_cache_entry = data as *mut stat_cache_entry;
    if mod_0 < 0 as libc::c_int && 1 as libc::c_int == (*sce).refcnt {
        stat_cache_entry_free(data);
    } else {
        (*sce).refcnt += mod_0;
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn stat_cache_init(
    mut ev: *mut fdevents,
    mut errh: *mut log_error_st,
) -> libc::c_int {
    if sc.stat_cache_engine == STAT_CACHE_ENGINE_FAM as libc::c_int {
        sc.scf = stat_cache_init_fam(ev, errh);
        if (sc.scf).is_null() {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn stat_cache_free() {
    let mut sptree: *mut splay_tree = sc.files;
    while !sptree.is_null() {
        stat_cache_entry_free((*sptree).data);
        sptree = splaytree_delete(sptree, (*sptree).key);
    }
    sc.files = 0 as *mut splay_tree;
    stat_cache_free_fam(sc.scf);
    sc.scf = 0 as *mut stat_cache_fam;
    sc.stat_cache_engine = STAT_CACHE_ENGINE_SIMPLE as libc::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn stat_cache_xattrname(mut name: *const libc::c_char) {}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn stat_cache_choose_engine(
    mut stat_cache_string: *const buffer,
    mut errh: *mut log_error_st,
) -> libc::c_int {
    if buffer_is_blank(stat_cache_string) != 0 {
        sc.stat_cache_engine = STAT_CACHE_ENGINE_SIMPLE as libc::c_int;
    } else if buffer_eq_slen(
            stat_cache_string,
            b"simple\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
        sc.stat_cache_engine = STAT_CACHE_ENGINE_SIMPLE as libc::c_int;
    } else if buffer_eq_slen(
            stat_cache_string,
            b"inotify\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
        sc.stat_cache_engine = STAT_CACHE_ENGINE_INOTIFY as libc::c_int;
    } else if buffer_eq_slen(
            stat_cache_string,
            b"fam\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
        sc.stat_cache_engine = STAT_CACHE_ENGINE_FAM as libc::c_int;
    } else if buffer_eq_slen(
            stat_cache_string,
            b"disable\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
            || buffer_eq_slen(
                stat_cache_string,
                b"none\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
        {
        sc.stat_cache_engine = STAT_CACHE_ENGINE_NONE as libc::c_int;
    } else {
        log_error(
            errh,
            b"src/stat_cache.c\0" as *const u8 as *const libc::c_char,
            927 as libc::c_int as libc::c_uint,
            b"server.stat-cache-engine can be one of \"disable\", \"simple\", \"inotify\", \"fam\", but not: %s\0"
                as *const u8 as *const libc::c_char,
            (*stat_cache_string).ptr,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stat_cache_mimetype_by_ext(
    mimetypes: *const array,
    name: *const libc::c_char,
    nlen: uint32_t,
) -> *const buffer {
    let end: *const libc::c_char = name.offset(nlen as isize);
    let used: uint32_t = (*mimetypes).used;
    if used < 16 as libc::c_int as libc::c_uint {
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while i < used {
            let mut ds: *const data_string = *((*mimetypes).data).offset(i as isize)
                as *mut data_string;
            let klen: size_t = buffer_clen(&(*ds).key) as size_t;
            if klen <= nlen as libc::c_ulong
                && buffer_eq_icase_ssn(end.offset(-(klen as isize)), (*ds).key.ptr, klen)
                    != 0
            {
                return &(*ds).value;
            }
            i = i.wrapping_add(1);
        }
    } else {
        let mut s: *const libc::c_char = 0 as *const libc::c_char;
        let mut ds_0: *const data_string = 0 as *const data_string;
        if nlen != 0 {
            s = end.offset(-(1 as libc::c_int as isize));
            while s != name && *s as libc::c_int != '/' as i32 {
                s = s.offset(-1);
            }
            if *s as libc::c_int == '/' as i32 {
                s = s.offset(1);
            }
        } else {
            s = name;
        }
        ds_0 = array_get_element_klen(
            mimetypes,
            s,
            end.offset_from(s) as libc::c_long as uint32_t,
        ) as *const data_string;
        if !ds_0.is_null() {
            return &(*ds_0).value;
        }
        loop {
            s = s.offset(1);
            if !(s < end) {
                break;
            }
            while *s as libc::c_int != '.' as i32
                && {
                    s = s.offset(1);
                    s != end
                }
            {}
            if s == end {
                break;
            }
            ds_0 = array_get_element_klen(
                mimetypes,
                s,
                end.offset_from(s) as libc::c_long as uint32_t,
            ) as *const data_string;
            if !ds_0.is_null() {
                return &(*ds_0).value;
            }
            s = s.offset(1);
            if !(s < end) {
                continue;
            }
            if *s as libc::c_int == '.' as i32 {
                s = s.offset(-1);
            } else {
                ds_0 = array_get_element_klen(
                    mimetypes,
                    s,
                    end.offset_from(s) as libc::c_long as uint32_t,
                ) as *const data_string;
                if !ds_0.is_null() {
                    return &(*ds_0).value;
                }
            }
        }
        ds_0 = array_get_element_klen(
            mimetypes,
            b"\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        ) as *const data_string;
        if !ds_0.is_null() {
            return &(*ds_0).value;
        }
    }
    return 0 as *const buffer;
}
#[no_mangle]
pub unsafe extern "C" fn stat_cache_content_type_get_by_ext(
    mut sce: *mut stat_cache_entry,
    mut mimetypes: *const array,
) -> *const buffer {
    if buffer_is_blank(&mut (*sce).content_type) == 0 {
        return &mut (*sce).content_type;
    }
    if !((*sce).st.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint)
    {
        return 0 as *const buffer;
    }
    let mtype: *const buffer = stat_cache_mimetype_by_ext(
        mimetypes,
        (*sce).name.ptr,
        buffer_clen(&mut (*sce).name),
    );
    if !mtype.is_null() {
        (*sce).content_type.ptr = (*mtype).ptr;
        (*sce).content_type.used = (*mtype).used;
    } else {
        buffer_clear(&mut (*sce).content_type);
    }
    return &mut (*sce).content_type;
}
#[no_mangle]
pub unsafe extern "C" fn stat_cache_etag_get(
    mut sce: *mut stat_cache_entry,
    mut flags: libc::c_int,
) -> *const buffer {
    if buffer_is_blank(&mut (*sce).etag) == 0 {
        return &mut (*sce).etag;
    }
    if (*sce).st.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
        || (*sce).st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint
    {
        if 0 as libc::c_int == flags {
            return 0 as *const buffer;
        }
        http_etag_create(&mut (*sce).etag, &mut (*sce).st, flags);
        return &mut (*sce).etag;
    }
    return 0 as *const buffer;
}
unsafe extern "C" fn stat_cache_stat_eq(
    sta: *const stat,
    stb: *const stat,
) -> libc::c_int {
    return ((*sta).st_mtim.tv_nsec == (*stb).st_mtim.tv_nsec
        && (*sta).st_mtim.tv_sec == (*stb).st_mtim.tv_sec
        && (*sta).st_size == (*stb).st_size && (*sta).st_ino == (*stb).st_ino
        && (*sta).st_dev == (*stb).st_dev) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stat_cache_update_entry(
    mut name: *const libc::c_char,
    mut len: uint32_t,
    mut st: *const stat,
    mut etagb: *const buffer,
) {
    if sc.stat_cache_engine == STAT_CACHE_ENGINE_NONE as libc::c_int {
        return;
    }
    if !(0 as libc::c_int as libc::c_uint != len) {
        ck_assert_failed(
            b"src/stat_cache.c\0" as *const u8 as *const libc::c_char,
            1095 as libc::c_int as libc::c_uint,
            b"0 != len\0" as *const u8 as *const libc::c_char,
        );
    }
    if *name.offset(len.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
        as libc::c_int == '/' as i32
    {
        len = len.wrapping_sub(1);
        if 0 as libc::c_int as libc::c_uint == len {
            len = 1 as libc::c_int as uint32_t;
        }
    }
    let mut sptree: *mut *mut splay_tree = &mut sc.files;
    let mut sce: *mut stat_cache_entry = stat_cache_sptree_find(sptree, name, len)
        as *mut stat_cache_entry;
    if !sce.is_null() && buffer_eq_slen(&mut (*sce).name, name, len as size_t) != 0 {
        if stat_cache_stat_eq(&mut (*sce).st, st) == 0 {
            buffer_clear(&mut (*sce).etag);
            if !etagb.is_null() {
                buffer_copy_string_len(
                    &mut (*sce).etag,
                    (*etagb).ptr,
                    buffer_clen(etagb) as size_t,
                );
            }
            if (*sce).fd >= 0 as libc::c_int {
                if 1 as libc::c_int == (*sce).refcnt {
                    close((*sce).fd);
                    (*sce).fd = -(1 as libc::c_int);
                } else {
                    (*sce).refcnt -= 1;
                    sce = stat_cache_entry_init();
                    (**sptree).data = sce as *mut libc::c_void;
                    buffer_copy_string_len(&mut (*sce).name, name, len as size_t);
                }
            }
            (*sce).st = *st;
        }
        (*sce).stat_ts = log_monotonic_secs;
    }
}
#[no_mangle]
pub unsafe extern "C" fn stat_cache_delete_entry(
    mut name: *const libc::c_char,
    mut len: uint32_t,
) {
    if sc.stat_cache_engine == STAT_CACHE_ENGINE_NONE as libc::c_int {
        return;
    }
    if !(0 as libc::c_int as libc::c_uint != len) {
        ck_assert_failed(
            b"src/stat_cache.c\0" as *const u8 as *const libc::c_char,
            1129 as libc::c_int as libc::c_uint,
            b"0 != len\0" as *const u8 as *const libc::c_char,
        );
    }
    if *name.offset(len.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
        as libc::c_int == '/' as i32
    {
        len = len.wrapping_sub(1);
        if 0 as libc::c_int as libc::c_uint == len {
            len = 1 as libc::c_int as uint32_t;
        }
    }
    let mut sptree: *mut *mut splay_tree = &mut sc.files;
    let mut sce: *mut stat_cache_entry = stat_cache_sptree_find(sptree, name, len)
        as *mut stat_cache_entry;
    if !sce.is_null() && buffer_eq_slen(&mut (*sce).name, name, len as size_t) != 0 {
        stat_cache_entry_free(sce as *mut libc::c_void);
        *sptree = splaytree_delete(*sptree, (**sptree).key);
    }
}
#[no_mangle]
pub unsafe extern "C" fn stat_cache_invalidate_entry(
    mut name: *const libc::c_char,
    mut len: uint32_t,
) {
    let mut sptree: *mut *mut splay_tree = &mut sc.files;
    let mut sce: *mut stat_cache_entry = stat_cache_sptree_find(sptree, name, len)
        as *mut stat_cache_entry;
    if !sce.is_null() && buffer_eq_slen(&mut (*sce).name, name, len as size_t) != 0 {
        (*sce).stat_ts = 0 as libc::c_int as unix_time64_t;
        if !((*sce).fam_dir).is_null() {
            let ref mut fresh2 = (*((*sce).fam_dir as *mut fam_dir_entry)).refcnt;
            *fresh2 -= 1;
            (*sce).fam_dir = 0 as *mut libc::c_void;
        }
    }
}
unsafe extern "C" fn stat_cache_invalidate_dir_tree_walk(
    mut t: *mut splay_tree,
    mut name: *const libc::c_char,
    mut len: size_t,
) {
    if !((*t).left).is_null() {
        stat_cache_invalidate_dir_tree_walk((*t).left, name, len);
    }
    if !((*t).right).is_null() {
        stat_cache_invalidate_dir_tree_walk((*t).right, name, len);
    }
    let b: *const buffer = &mut (*((*t).data as *mut stat_cache_entry)).name;
    let blen: size_t = buffer_clen(b) as size_t;
    if blen > len && *((*b).ptr).offset(len as isize) as libc::c_int == '/' as i32
        && 0 as libc::c_int
            == memcmp((*b).ptr as *const libc::c_void, name as *const libc::c_void, len)
    {
        let mut sce: *mut stat_cache_entry = (*t).data as *mut stat_cache_entry;
        (*sce).stat_ts = 0 as libc::c_int as unix_time64_t;
        if !((*sce).fam_dir).is_null() {
            let ref mut fresh3 = (*((*sce).fam_dir as *mut fam_dir_entry)).refcnt;
            *fresh3 -= 1;
            (*sce).fam_dir = 0 as *mut libc::c_void;
        }
    }
}
unsafe extern "C" fn stat_cache_invalidate_dir_tree(
    mut name: *const libc::c_char,
    mut len: size_t,
) {
    let sptree: *mut splay_tree = sc.files;
    if !sptree.is_null() {
        stat_cache_invalidate_dir_tree_walk(sptree, name, len);
    }
}
unsafe extern "C" fn stat_cache_tag_dir_tree(
    mut t: *mut splay_tree,
    mut name: *const libc::c_char,
    mut len: size_t,
    mut keys: *mut libc::c_int,
    mut ndx: *mut libc::c_int,
) {
    if *ndx == 8192 as libc::c_int {
        return;
    }
    if !((*t).left).is_null() {
        stat_cache_tag_dir_tree((*t).left, name, len, keys, ndx);
    }
    if !((*t).right).is_null() {
        stat_cache_tag_dir_tree((*t).right, name, len, keys, ndx);
    }
    if *ndx == 8192 as libc::c_int {
        return;
    }
    let b: *const buffer = &mut (*((*t).data as *mut stat_cache_entry)).name;
    let blen: size_t = buffer_clen(b) as size_t;
    if blen > len && *((*b).ptr).offset(len as isize) as libc::c_int == '/' as i32
        && 0 as libc::c_int
            == memcmp((*b).ptr as *const libc::c_void, name as *const libc::c_void, len)
    {
        let fresh4 = *ndx;
        *ndx = *ndx + 1;
        *keys.offset(fresh4 as isize) = (*t).key;
    }
}
#[inline(never)]
unsafe extern "C" fn stat_cache_prune_dir_tree(
    mut name: *const libc::c_char,
    mut len: size_t,
) {
    let mut sptree: *mut splay_tree = sc.files;
    let mut max_ndx: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut keys: [libc::c_int; 8192] = [0; 8192];
    while !sptree.is_null() {
        max_ndx = 0 as libc::c_int;
        stat_cache_tag_dir_tree(sptree, name, len, keys.as_mut_ptr(), &mut max_ndx);
        i = 0 as libc::c_int;
        while i < max_ndx {
            let ndx: libc::c_int = keys[i as usize];
            sptree = splaytree_splay(sptree, ndx);
            let mut node: *mut splay_tree = sptree;
            if !node.is_null() && (*node).key == ndx {
                stat_cache_entry_free((*node).data);
                sptree = splaytree_delete(sptree, ndx);
            }
            i += 1;
        }
        if !(max_ndx as libc::c_ulong
            == (::std::mem::size_of::<[libc::c_int; 8192]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong))
        {
            break;
        }
    }
    sc.files = sptree;
}
unsafe extern "C" fn stat_cache_delete_tree(
    mut name: *const libc::c_char,
    mut len: uint32_t,
) {
    stat_cache_delete_entry(name, len);
    stat_cache_prune_dir_tree(name, len as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn stat_cache_delete_dir(
    mut name: *const libc::c_char,
    mut len: uint32_t,
) {
    if !(0 as libc::c_int as libc::c_uint != len) {
        ck_assert_failed(
            b"src/stat_cache.c\0" as *const u8 as *const libc::c_char,
            1231 as libc::c_int as libc::c_uint,
            b"0 != len\0" as *const u8 as *const libc::c_char,
        );
    }
    if *name.offset(len.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
        as libc::c_int == '/' as i32
    {
        len = len.wrapping_sub(1);
        if 0 as libc::c_int as libc::c_uint == len {
            len = 1 as libc::c_int as uint32_t;
        }
    }
    stat_cache_delete_tree(name, len);
    if sc.stat_cache_engine == STAT_CACHE_ENGINE_FAM as libc::c_int {
        let mut sptree: *mut *mut splay_tree = &mut (*sc.scf).dirs;
        let mut fam_dir: *mut fam_dir_entry = stat_cache_sptree_find(sptree, name, len)
            as *mut fam_dir_entry;
        if !fam_dir.is_null()
            && buffer_eq_slen(&mut (*fam_dir).name, name, len as size_t) != 0
        {
            fam_dir_invalidate_node(fam_dir);
        }
        if !(*sptree).is_null() {
            fam_dir_invalidate_tree(*sptree, name, len as size_t);
        }
        fam_dir_periodic_cleanup();
    }
}
#[no_mangle]
pub unsafe extern "C" fn stat_cache_get_entry(
    name: *const buffer,
) -> *mut stat_cache_entry {
    let mut sce: *mut stat_cache_entry = 0 as *mut stat_cache_entry;
    let mut final_slash: libc::c_int = 0 as libc::c_int;
    let mut len: size_t = buffer_clen(name) as size_t;
    if !(0 as libc::c_int as libc::c_ulong != len) {
        ck_assert_failed(
            b"src/stat_cache.c\0" as *const u8 as *const libc::c_char,
            1262 as libc::c_int as libc::c_uint,
            b"0 != len\0" as *const u8 as *const libc::c_char,
        );
    }
    if *((*name).ptr)
        .offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int == '/' as i32
    {
        final_slash = 1 as libc::c_int;
        len = len.wrapping_sub(1);
        if 0 as libc::c_int as libc::c_ulong == len {
            len = 1 as libc::c_int as size_t;
        }
    }
    if *((*name).ptr).offset(0 as libc::c_int as isize) as libc::c_int != '/' as i32 {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut stat_cache_entry;
    }
    let cur_ts: unix_time64_t = log_monotonic_secs;
    let file_ndx: libc::c_int = splaytree_djbhash((*name).ptr, len as uint32_t);
    sc.files = splaytree_splay(sc.files, file_ndx);
    let mut sptree: *mut splay_tree = sc.files;
    if !sptree.is_null() && (*sptree).key == file_ndx {
        sce = (*sptree).data as *mut stat_cache_entry;
        if buffer_eq_slen(&mut (*sce).name, (*name).ptr, len) != 0 {
            if sc.stat_cache_engine == STAT_CACHE_ENGINE_SIMPLE as libc::c_int {
                if (*sce).stat_ts == cur_ts {
                    if final_slash != 0
                        && !((*sce).st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                            == 0o40000 as libc::c_int as libc::c_uint)
                    {
                        *__errno_location() = 20 as libc::c_int;
                        return 0 as *mut stat_cache_entry;
                    }
                    return sce;
                }
            } else if sc.stat_cache_engine == STAT_CACHE_ENGINE_FAM as libc::c_int
                    && !((*sce).fam_dir).is_null()
                {
                if cur_ts - (*sce).stat_ts < 16 as libc::c_int as libc::c_long {
                    if final_slash != 0
                        && !((*sce).st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                            == 0o40000 as libc::c_int as libc::c_uint)
                    {
                        *__errno_location() = 20 as libc::c_int;
                        return 0 as *mut stat_cache_entry;
                    }
                    return sce;
                }
            }
        } else {
            sce = 0 as *mut stat_cache_entry;
        }
    }
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if -(1 as libc::c_int) == stat((*name).ptr, &mut st) {
        return 0 as *mut stat_cache_entry;
    }
    if sce.is_null() {
        if final_slash != 0
            && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint
        {
            *__errno_location() = 20 as libc::c_int;
            return 0 as *mut stat_cache_entry;
        }
        sce = stat_cache_entry_init();
        buffer_copy_string_len(&mut (*sce).name, (*name).ptr, len);
        if !sptree.is_null() && (*sptree).key == file_ndx {
            stat_cache_entry_free((*sptree).data);
            (*sptree).data = sce as *mut libc::c_void;
        } else {
            sc.files = splaytree_insert(sptree, file_ndx, sce as *mut libc::c_void);
        }
    } else {
        buffer_clear(&mut (*sce).etag);
        if (*sce).fd >= 0 as libc::c_int
            && stat_cache_stat_eq(&mut (*sce).st, &mut st) == 0
        {
            if 1 as libc::c_int == (*sce).refcnt {
                close((*sce).fd);
                (*sce).fd = -(1 as libc::c_int);
            } else {
                (*sce).refcnt -= 1;
                sce = stat_cache_entry_init();
                (*sptree).data = sce as *mut libc::c_void;
                buffer_copy_string_len(&mut (*sce).name, (*name).ptr, len);
            }
        }
    }
    (*sce).st = st;
    if sc.stat_cache_engine == STAT_CACHE_ENGINE_FAM as libc::c_int {
        if !((*sce).fam_dir).is_null() {
            let ref mut fresh5 = (*((*sce).fam_dir as *mut fam_dir_entry)).refcnt;
            *fresh5 -= 1;
        }
        (*sce)
            .fam_dir = fam_dir_monitor(sc.scf, (*name).ptr, len as uint32_t, &mut st)
            as *mut libc::c_void;
    }
    (*sce).stat_ts = cur_ts;
    return sce;
}
#[no_mangle]
pub unsafe extern "C" fn stat_cache_get_entry_open(
    name: *const buffer,
    symlinks: libc::c_int,
) -> *mut stat_cache_entry {
    let sce: *mut stat_cache_entry = stat_cache_get_entry(name);
    if sce.is_null() {
        return 0 as *mut stat_cache_entry;
    }
    if (*sce).fd >= 0 as libc::c_int {
        return sce;
    }
    if (*sce).st.st_size > 0 as libc::c_int as libc::c_long {
        (*sce).fd = stat_cache_open_rdonly_fstat(name, &mut (*sce).st, symlinks);
        buffer_clear(&mut (*sce).etag);
    }
    return sce;
}
#[no_mangle]
pub unsafe extern "C" fn stat_cache_path_stat(
    name: *const buffer,
) -> *const stat_cache_st {
    let sce: *const stat_cache_entry = stat_cache_get_entry(name);
    return if !sce.is_null() { &(*sce).st } else { 0 as *const stat };
}
#[no_mangle]
pub unsafe extern "C" fn stat_cache_path_isdir(mut name: *const buffer) -> libc::c_int {
    let sce: *const stat_cache_entry = stat_cache_get_entry(name);
    return (!sce.is_null()
        && (if (*sce).st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint
        {
            1 as libc::c_int
        } else {
            *__errno_location() = 20 as libc::c_int;
            0 as libc::c_int
        }) != 0) as libc::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn stat_cache_path_contains_symlink(
    mut name: *const buffer,
    mut errh: *mut log_error_st,
) -> libc::c_int {
    let mut len: size_t = buffer_clen(name) as size_t;
    if !(0 as libc::c_int as libc::c_ulong != len) {
        ck_assert_failed(
            b"src/stat_cache.c\0" as *const u8 as *const libc::c_char,
            1425 as libc::c_int as libc::c_uint,
            b"0 != len\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(*((*name).ptr).offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32) {
        ck_assert_failed(
            b"src/stat_cache.c\0" as *const u8 as *const libc::c_char,
            1426 as libc::c_int as libc::c_uint,
            b"name->ptr[0] == '/'\0" as *const u8 as *const libc::c_char,
        );
    }
    if 1 as libc::c_int as libc::c_ulong == len {
        return 0 as libc::c_int;
    }
    if len >= 4096 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    memcpy(
        buf.as_mut_ptr() as *mut libc::c_void,
        (*name).ptr as *const libc::c_void,
        len,
    );
    let mut s_cur: *mut libc::c_char = buf.as_mut_ptr().offset(len as isize);
    loop {
        *s_cur = '\u{0}' as i32 as libc::c_char;
        let mut st: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        if 0 as libc::c_int == lstat(buf.as_mut_ptr(), &mut st) {
            if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o120000 as libc::c_int as libc::c_uint
            {
                return 1 as libc::c_int;
            }
        } else {
            log_perror(
                errh,
                b"src/stat_cache.c\0" as *const u8 as *const libc::c_char,
                1443 as libc::c_int as libc::c_uint,
                b"lstat failed for: %s\0" as *const u8 as *const libc::c_char,
                buf.as_mut_ptr(),
            );
            return -(1 as libc::c_int);
        }
        s_cur = strrchr(buf.as_mut_ptr(), '/' as i32);
        if !(s_cur > buf.as_mut_ptr()) {
            break;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stat_cache_open_rdonly_fstat(
    mut name: *const buffer,
    mut st: *mut stat,
    mut symlinks: libc::c_int,
) -> libc::c_int {
    let fd: libc::c_int = fdevent_open_cloexec(
        (*name).ptr,
        symlinks,
        0 as libc::c_int,
        0 as libc::c_int as mode_t,
    );
    if fd >= 0 as libc::c_int {
        if 0 as libc::c_int == fstat(fd, st) {
            return fd
        } else {
            let errnum: libc::c_int = *__errno_location();
            close(fd);
            *__errno_location() = errnum;
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn stat_cache_tag_old_entries(
    t: *mut splay_tree,
    keys: *mut libc::c_int,
    ndx: *mut libc::c_int,
    max_age: time_t,
    cur_ts: unix_time64_t,
) {
    if *ndx == 8192 as libc::c_int {
        return;
    }
    if !((*t).left).is_null() {
        stat_cache_tag_old_entries((*t).left, keys, ndx, max_age, cur_ts);
    }
    if !((*t).right).is_null() {
        stat_cache_tag_old_entries((*t).right, keys, ndx, max_age, cur_ts);
    }
    if *ndx == 8192 as libc::c_int {
        return;
    }
    let sce: *const stat_cache_entry = (*t).data as *const stat_cache_entry;
    if cur_ts - (*sce).stat_ts > max_age {
        let fresh6 = *ndx;
        *ndx = *ndx + 1;
        *keys.offset(fresh6 as isize) = (*t).key;
    }
}
unsafe extern "C" fn stat_cache_periodic_cleanup(
    max_age: time_t,
    cur_ts: unix_time64_t,
) {
    let mut sptree: *mut splay_tree = sc.files;
    let mut max_ndx: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut keys: [libc::c_int; 8192] = [0; 8192];
    while !sptree.is_null() {
        max_ndx = 0 as libc::c_int;
        stat_cache_tag_old_entries(
            sptree,
            keys.as_mut_ptr(),
            &mut max_ndx,
            max_age,
            cur_ts,
        );
        i = 0 as libc::c_int;
        while i < max_ndx {
            let mut ndx: libc::c_int = keys[i as usize];
            sptree = splaytree_splay(sptree, ndx);
            if !sptree.is_null() && (*sptree).key == ndx {
                stat_cache_entry_free((*sptree).data);
                sptree = splaytree_delete(sptree, ndx);
            }
            i += 1;
        }
        if !(max_ndx as libc::c_ulong
            == (::std::mem::size_of::<[libc::c_int; 8192]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong))
        {
            break;
        }
    }
    sc.files = sptree;
}
#[no_mangle]
pub unsafe extern "C" fn stat_cache_trigger_cleanup() {
    let mut max_age: time_t = 2 as libc::c_int as time_t;
    if STAT_CACHE_ENGINE_FAM as libc::c_int == sc.stat_cache_engine {
        if log_monotonic_secs & 0x1f as libc::c_int as libc::c_long != 0 {
            return;
        }
        max_age = 32 as libc::c_int as time_t;
        fam_dir_periodic_cleanup();
    }
    stat_cache_periodic_cleanup(max_age, log_monotonic_secs);
}
