use ::libc;
extern "C" {
    pub type connection;
    pub type plugin;
    pub type cond_match_t;
    pub type cond_cache_t;
    fn buffer_commit(b: *mut buffer, size: size_t);
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn chunkqueue_steal_with_tempfiles(
        dest: *mut chunkqueue,
        src: *mut chunkqueue,
        len: off_t,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn chunkqueue_steal(dest: *mut chunkqueue, src: *mut chunkqueue, len: off_t);
    fn chunkqueue_append_mem_to_tempfile(
        cq: *mut chunkqueue,
        mem: *const libc::c_char,
        len: size_t,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn chunkqueue_append_buffer_commit(cq: *mut chunkqueue);
    fn chunkqueue_append_buffer_open_sz(cq: *mut chunkqueue, sz: size_t) -> *mut buffer;
    fn chunkqueue_append_buffer(cq: *mut chunkqueue, mem: *mut buffer);
    fn chunkqueue_append_mem(cq: *mut chunkqueue, mem: *const libc::c_char, len: size_t);
    fn chunkqueue_append_file_fd(
        cq: *mut chunkqueue,
        fn_0: *const buffer,
        fd: libc::c_int,
        offset: off_t,
        len: off_t,
    );
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn hex2int(c: libc::c_uchar) -> libc::c_char;
    fn stat_cache_entry_refchg(data: *mut libc::c_void, mod_0: libc::c_int);
    fn log_error(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        fmt: *const libc::c_char,
        _: ...
    );
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pread(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __nbytes: size_t,
        __offset: __off64_t,
    ) -> ssize_t;
    fn __errno_location() -> *mut libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
}
pub type __int8_t = libc::c_schar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type off_t = __off64_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintmax_t = __uintmax_t;
pub type unix_time64_t = time_t;
pub type unix_timespec64_t = timespec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct request_st {
    pub state: request_state_t,
    pub http_status: libc::c_int,
    pub h2state: uint32_t,
    pub h2id: uint32_t,
    pub h2_rwin: int32_t,
    pub h2_swin: int32_t,
    pub http_method: http_method_t,
    pub http_version: http_version_t,
    pub handler_module: *const plugin,
    pub plugin_ctx: *mut *mut libc::c_void,
    pub con: *mut connection,
    pub conditional_is_valid: uint32_t,
    pub cond_cache: *mut cond_cache_t,
    pub cond_match: *mut *mut cond_match_t,
    pub cond_match_data: *mut cond_match_t,
    pub conf: request_config,
    pub rqst_header_len: uint32_t,
    pub rqst_htags: uint64_t,
    pub rqst_headers: array,
    pub uri: request_uri,
    pub physical: physical,
    pub env: array,
    pub reqbody_length: off_t,
    pub te_chunked: off_t,
    pub resp_body_scratchpad: off_t,
    pub http_host: *mut buffer,
    pub server_name: *const buffer,
    pub target: buffer,
    pub target_orig: buffer,
    pub pathinfo: buffer,
    pub server_name_buf: buffer,
    pub resp_header_len: uint32_t,
    pub resp_htags: uint64_t,
    pub resp_headers: array,
    pub resp_body_finished: libc::c_char,
    pub resp_body_started: libc::c_char,
    pub resp_send_chunked: libc::c_char,
    pub resp_decode_chunked: libc::c_char,
    pub resp_header_repeated: libc::c_char,
    pub loops_per_request: libc::c_char,
    pub keep_alive: int8_t,
    pub async_callback: libc::c_char,
    pub tmp_buf: *mut buffer,
    pub gw_dechunk: *mut response_dechunk,
    pub bytes_written_ckpt: off_t,
    pub bytes_read_ckpt: off_t,
    pub start_hp: unix_timespec64_t,
    pub error_handler_saved_status: libc::c_int,
    pub error_handler_saved_method: http_method_t,
    pub write_queue: chunkqueue,
    pub read_queue: chunkqueue,
    pub reqbody_queue: chunkqueue,
    pub tmp_sce: *mut stat_cache_entry,
    pub cond_captures: libc::c_int,
}
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
pub struct buffer {
    pub ptr: *mut libc::c_char,
    pub used: uint32_t,
    pub size: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chunkqueue {
    pub first: *mut chunk,
    pub last: *mut chunk,
    pub bytes_in: off_t,
    pub bytes_out: off_t,
    pub tempdirs: *const array,
    pub upload_temp_file_size: off_t,
    pub tempdir_idx: libc::c_uint,
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
pub struct chunk {
    pub next: *mut chunk,
    pub type_0: C2RustUnnamed_1,
    pub mem: *mut buffer,
    pub offset: off_t,
    pub file: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub length: off_t,
    pub fd: libc::c_int,
    pub is_temp: libc::c_int,
    pub mmap: C2RustUnnamed_0,
    pub ref_0: *mut libc::c_void,
    pub refchg: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub start: *mut libc::c_char,
    pub length: size_t,
    pub offset: off_t,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const FILE_CHUNK: C2RustUnnamed_1 = 1;
pub const MEM_CHUNK: C2RustUnnamed_1 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct response_dechunk {
    pub gw_chunked: off_t,
    pub b: buffer,
    pub done: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct physical {
    pub path: buffer,
    pub basedir: buffer,
    pub doc_root: buffer,
    pub rel_path: buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct request_uri {
    pub scheme: buffer,
    pub authority: buffer,
    pub path: buffer,
    pub query: buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct request_config {
    pub http_parseopts: libc::c_uint,
    pub max_request_field_size: uint32_t,
    pub mimetypes: *const array,
    pub document_root: *const buffer,
    pub server_name: *const buffer,
    pub server_tag: *const buffer,
    pub errh: *mut fdlog_st,
    pub max_request_size: libc::c_uint,
    pub max_keep_alive_requests: libc::c_ushort,
    pub max_keep_alive_idle: libc::c_ushort,
    pub max_read_idle: libc::c_ushort,
    pub max_write_idle: libc::c_ushort,
    pub stream_request_body: libc::c_ushort,
    pub stream_response_body: libc::c_ushort,
    pub high_precision_timestamps: libc::c_uchar,
    pub allow_http11: libc::c_uchar,
    pub follow_symlink: libc::c_uchar,
    pub etag_flags: libc::c_uchar,
    pub force_lowercase_filenames: libc::c_uchar,
    pub use_xattr: libc::c_uchar,
    pub range_requests: libc::c_uchar,
    pub error_intercept: libc::c_uchar,
    pub h2proto: libc::c_uchar,
    pub log_file_not_found: libc::c_uchar,
    pub log_request_header: libc::c_uchar,
    pub log_request_handling: libc::c_uchar,
    pub log_response_header: libc::c_uchar,
    pub log_condition_handling: libc::c_uchar,
    pub log_timeouts: libc::c_uchar,
    pub log_state_handling: libc::c_uchar,
    pub log_request_header_on_error: libc::c_uchar,
    pub bytes_per_second: libc::c_uint,
    pub global_bytes_per_second: libc::c_uint,
    pub global_bytes_per_second_cnt_ptr: *mut off_t,
    pub error_handler: *const buffer,
    pub error_handler_404: *const buffer,
    pub errorfile_prefix: *const buffer,
    pub serrh: *mut fdlog_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fdlog_st {
    pub mode: C2RustUnnamed_2,
    pub fd: libc::c_int,
    pub b: buffer,
    pub fn_0: *const libc::c_char,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const FDLOG_PIPE: C2RustUnnamed_2 = 3;
pub const FDLOG_SYSLOG: C2RustUnnamed_2 = 2;
pub const FDLOG_FD: C2RustUnnamed_2 = 1;
pub const FDLOG_FILE: C2RustUnnamed_2 = 0;
pub type http_version_t = libc::c_int;
pub const HTTP_VERSION_2: http_version_t = 2;
pub const HTTP_VERSION_1_1: http_version_t = 1;
pub const HTTP_VERSION_1_0: http_version_t = 0;
pub const HTTP_VERSION_UNSET: http_version_t = -1;
pub type request_state_t = libc::c_uint;
pub const CON_STATE_CLOSE: request_state_t = 10;
pub const CON_STATE_ERROR: request_state_t = 9;
pub const CON_STATE_RESPONSE_END: request_state_t = 8;
pub const CON_STATE_WRITE: request_state_t = 7;
pub const CON_STATE_RESPONSE_START: request_state_t = 6;
pub const CON_STATE_HANDLE_REQUEST: request_state_t = 5;
pub const CON_STATE_READ_POST: request_state_t = 4;
pub const CON_STATE_REQUEST_END: request_state_t = 3;
pub const CON_STATE_READ: request_state_t = 2;
pub const CON_STATE_REQUEST_START: request_state_t = 1;
pub const CON_STATE_CONNECT: request_state_t = 0;
pub type log_error_st = fdlog_st;
#[inline]
unsafe extern "C" fn chunkqueue_length(mut cq: *const chunkqueue) -> off_t {
    return (*cq).bytes_in - (*cq).bytes_out;
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
unsafe extern "C" fn buffer_clear(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
}
#[inline(never)]
unsafe extern "C" fn http_chunk_len_append(cq: *mut chunkqueue, mut len: uintmax_t) {
    let mut buf: [libc::c_char; 24] = [0; 24];
    let mut i: libc::c_int = ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong
        as libc::c_int;
    i -= 1;
    buf[i as usize] = '\n' as i32 as libc::c_char;
    i -= 1;
    buf[i as usize] = '\r' as i32 as libc::c_char;
    loop {
        i -= 1;
        buf[i
            as usize] = (*::std::mem::transmute::<
            &[u8; 17],
            &[libc::c_char; 17],
        >(b"0123456789abcdef\0"))[(len & 0xf as libc::c_int as libc::c_ulong) as usize];
        len >>= 4 as libc::c_int;
        if !(len != 0) {
            break;
        }
    }
    chunkqueue_append_mem(
        cq,
        buf.as_mut_ptr().offset(i as isize),
        (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
            .wrapping_sub(i as libc::c_ulong),
    );
}
#[inline(never)]
unsafe extern "C" fn http_chunk_len_append_tempfile(
    cq: *mut chunkqueue,
    mut len: uintmax_t,
    errh: *mut log_error_st,
) -> libc::c_int {
    let mut buf: [libc::c_char; 24] = [0; 24];
    let mut i: libc::c_int = ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong
        as libc::c_int;
    i -= 1;
    buf[i as usize] = '\n' as i32 as libc::c_char;
    i -= 1;
    buf[i as usize] = '\r' as i32 as libc::c_char;
    loop {
        i -= 1;
        buf[i
            as usize] = (*::std::mem::transmute::<
            &[u8; 17],
            &[libc::c_char; 17],
        >(b"0123456789abcdef\0"))[(len & 0xf as libc::c_int as libc::c_ulong) as usize];
        len >>= 4 as libc::c_int;
        if !(len != 0) {
            break;
        }
    }
    return chunkqueue_append_mem_to_tempfile(
        cq,
        buf.as_mut_ptr().offset(i as isize),
        (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
            .wrapping_sub(i as libc::c_ulong),
        errh,
    );
}
#[inline(never)]
unsafe extern "C" fn http_chunk_append_read_fd_range(
    r: *mut request_st,
    fn_0: *const buffer,
    fd: libc::c_int,
    mut offset: off_t,
    mut len: off_t,
) -> libc::c_int {
    let cq: *mut chunkqueue = &mut (*r).write_queue;
    if (*r).resp_send_chunked != 0 {
        http_chunk_len_append(cq, len as uintmax_t);
    }
    let b: *mut buffer = chunkqueue_append_buffer_open_sz(
        cq,
        (len + 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long)
            as size_t,
    );
    let mut rd: ssize_t = 0;
    let foff: off_t = offset;
    offset = 0 as libc::c_int as off_t;
    loop {
        rd = pread(
            fd,
            ((*b).ptr).offset(offset as isize) as *mut libc::c_void,
            (len - offset) as size_t,
            foff + offset,
        );
        if !(if rd > 0 as libc::c_int as libc::c_long {
            offset += rd;
            (offset != len) as libc::c_int
        } else {
            (*__errno_location() == 4 as libc::c_int) as libc::c_int
        } != 0)
        {
            break;
        }
    }
    buffer_commit(b, offset as size_t);
    if (*r).resp_send_chunked != 0 {
        buffer_append_string_len(
            b,
            b"\r\n\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    chunkqueue_append_buffer_commit(cq);
    return if rd >= 0 as libc::c_int as libc::c_long {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn http_chunk_append_file_ref_range(
    r: *mut request_st,
    sce: *mut stat_cache_entry,
    offset: off_t,
    mut len: off_t,
) {
    let cq: *mut chunkqueue = &mut (*r).write_queue;
    if (*sce).st.st_size - offset < len {
        len = (*sce).st.st_size - offset;
    }
    if len <= 0 as libc::c_int as libc::c_long {
        return;
    }
    if (*r).resp_send_chunked != 0 {
        http_chunk_len_append(cq, len as uintmax_t);
    }
    let fn_0: *const buffer = &mut (*sce).name;
    let fd: libc::c_int = (*sce).fd;
    chunkqueue_append_file_fd(cq, fn_0, fd, offset, len);
    if fd >= 0 as libc::c_int {
        let d: *mut chunk = (*cq).last;
        (*d).file.ref_0 = sce as *mut libc::c_void;
        (*d)
            .file
            .refchg = Some(
            stat_cache_entry_refchg
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> (),
        );
        stat_cache_entry_refchg(sce as *mut libc::c_void, 1 as libc::c_int);
    }
    if (*r).resp_send_chunked != 0 {
        chunkqueue_append_mem(
            cq,
            b"\r\n\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn http_chunk_append_file_fd_range(
    r: *mut request_st,
    fn_0: *const buffer,
    fd: libc::c_int,
    offset: off_t,
    len: off_t,
) {
    let cq: *mut chunkqueue = &mut (*r).write_queue;
    if (*r).resp_send_chunked != 0 {
        http_chunk_len_append(cq, len as uintmax_t);
    }
    chunkqueue_append_file_fd(cq, fn_0, fd, offset, len);
    if (*r).resp_send_chunked != 0 {
        chunkqueue_append_mem(
            cq,
            b"\r\n\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn http_chunk_append_file_fd(
    r: *mut request_st,
    fn_0: *const buffer,
    fd: libc::c_int,
    sz: off_t,
) -> libc::c_int {
    if sz > 32768 as libc::c_int as libc::c_long || (*r).resp_send_chunked == 0 {
        http_chunk_append_file_fd_range(r, fn_0, fd, 0 as libc::c_int as off_t, sz);
        return 0 as libc::c_int;
    }
    let mut rc: libc::c_int = if 0 as libc::c_int as libc::c_long != sz {
        http_chunk_append_read_fd_range(r, fn_0, fd, 0 as libc::c_int as off_t, sz)
    } else {
        0 as libc::c_int
    };
    close(fd);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn http_chunk_append_file_ref(
    r: *mut request_st,
    sce: *mut stat_cache_entry,
) -> libc::c_int {
    let sz: off_t = (*sce).st.st_size;
    if sz > 32768 as libc::c_int as libc::c_long || (*r).resp_send_chunked == 0 {
        http_chunk_append_file_ref_range(r, sce, 0 as libc::c_int as off_t, sz);
        return 0 as libc::c_int;
    }
    let fn_0: *const buffer = &mut (*sce).name;
    let fd: libc::c_int = (*sce).fd;
    let mut rc: libc::c_int = if 0 as libc::c_int as libc::c_long != sz {
        http_chunk_append_read_fd_range(r, fn_0, fd, 0 as libc::c_int as off_t, sz)
    } else {
        0 as libc::c_int
    };
    return rc;
}
#[inline(never)]
unsafe extern "C" fn http_chunk_append_to_tempfile(
    r: *mut request_st,
    mem: *const libc::c_char,
    len: size_t,
) -> libc::c_int {
    let cq: *mut chunkqueue = &mut (*r).write_queue;
    let errh: *mut log_error_st = (*r).conf.errh;
    if (*r).resp_send_chunked as libc::c_int != 0
        && 0 as libc::c_int != http_chunk_len_append_tempfile(cq, len, errh)
    {
        return -(1 as libc::c_int);
    }
    if 0 as libc::c_int != chunkqueue_append_mem_to_tempfile(cq, mem, len, errh) {
        return -(1 as libc::c_int);
    }
    if (*r).resp_send_chunked as libc::c_int != 0
        && 0 as libc::c_int
            != chunkqueue_append_mem_to_tempfile(
                cq,
                b"\r\n\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                errh,
            )
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[inline(never)]
unsafe extern "C" fn http_chunk_append_cq_to_tempfile(
    r: *mut request_st,
    src: *mut chunkqueue,
    len: size_t,
) -> libc::c_int {
    let cq: *mut chunkqueue = &mut (*r).write_queue;
    let errh: *mut log_error_st = (*r).conf.errh;
    if (*r).resp_send_chunked as libc::c_int != 0
        && 0 as libc::c_int != http_chunk_len_append_tempfile(cq, len, errh)
    {
        return -(1 as libc::c_int);
    }
    if 0 as libc::c_int != chunkqueue_steal_with_tempfiles(cq, src, len as off_t, errh) {
        return -(1 as libc::c_int);
    }
    if (*r).resp_send_chunked as libc::c_int != 0
        && 0 as libc::c_int
            != chunkqueue_append_mem_to_tempfile(
                cq,
                b"\r\n\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                errh,
            )
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn http_chunk_uses_tempfile(
    cq: *const chunkqueue,
    len: size_t,
) -> libc::c_int {
    let c: *const chunk = (*cq).last;
    return (!c.is_null()
        && (*c).type_0 as libc::c_uint == FILE_CHUNK as libc::c_int as libc::c_uint
        && (*c).file.is_temp != 0
        || (chunkqueue_length(cq) as libc::c_ulong).wrapping_add(len)
            > 65536 as libc::c_int as libc::c_ulong) as libc::c_int;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn http_chunk_append_buffer(
    r: *mut request_st,
    mem: *mut buffer,
) -> libc::c_int {
    let mut len: size_t = (if !mem.is_null() {
        buffer_clen(mem)
    } else {
        0 as libc::c_int as libc::c_uint
    }) as size_t;
    if 0 as libc::c_int as libc::c_ulong == len {
        return 0 as libc::c_int;
    }
    let cq: *mut chunkqueue = &mut (*r).write_queue;
    if http_chunk_uses_tempfile(cq, len) != 0 {
        let mut rc: libc::c_int = http_chunk_append_to_tempfile(r, (*mem).ptr, len);
        buffer_clear(mem);
        return rc;
    }
    if (*r).resp_send_chunked != 0 {
        http_chunk_len_append(cq, len);
    }
    chunkqueue_append_buffer(cq, mem);
    if (*r).resp_send_chunked != 0 {
        chunkqueue_append_mem(
            cq,
            b"\r\n\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn http_chunk_append_mem(
    r: *mut request_st,
    mem: *const libc::c_char,
    len: size_t,
) -> libc::c_int {
    if 0 as libc::c_int as libc::c_ulong == len {
        return 0 as libc::c_int;
    }
    if mem.is_null() {
        ck_assert_failed(
            b"src/http_chunk.c\0" as *const u8 as *const libc::c_char,
            246 as libc::c_int as libc::c_uint,
            b"((void*)0) != mem\0" as *const u8 as *const libc::c_char,
        );
    }
    let cq: *mut chunkqueue = &mut (*r).write_queue;
    if http_chunk_uses_tempfile(cq, len) != 0 {
        return http_chunk_append_to_tempfile(r, mem, len);
    }
    if (*r).resp_send_chunked != 0 {
        http_chunk_len_append(cq, len);
    }
    chunkqueue_append_mem(cq, mem, len);
    if (*r).resp_send_chunked != 0 {
        chunkqueue_append_mem(
            cq,
            b"\r\n\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn http_chunk_transfer_cqlen(
    r: *mut request_st,
    src: *mut chunkqueue,
    len: size_t,
) -> libc::c_int {
    if 0 as libc::c_int as libc::c_ulong == len {
        return 0 as libc::c_int;
    }
    let cq: *mut chunkqueue = &mut (*r).write_queue;
    if http_chunk_uses_tempfile(cq, len) != 0 {
        return http_chunk_append_cq_to_tempfile(r, src, len);
    }
    if (*r).resp_send_chunked != 0 {
        http_chunk_len_append(cq, len);
    }
    chunkqueue_steal(cq, src, len as off_t);
    if (*r).resp_send_chunked != 0 {
        chunkqueue_append_mem(
            cq,
            b"\r\n\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn http_chunk_close(r: *mut request_st) {
    if (*r).resp_send_chunked == 0 {
        return;
    }
    if !((*r).gw_dechunk).is_null() {
        if (*(*r).gw_dechunk).done == 0 {
            (*r).keep_alive = 0 as libc::c_int as int8_t;
        }
    } else {
        chunkqueue_append_mem(
            &mut (*r).write_queue,
            b"0\r\n\r\n\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    };
}
unsafe extern "C" fn http_chunk_decode_append_data(
    r: *mut request_st,
    mut mem: *const libc::c_char,
    mut len: off_t,
) -> libc::c_int {
    if (*(*r).gw_dechunk).done != 0 {
        return -(1 as libc::c_int);
    }
    let h: *mut buffer = &mut (*(*r).gw_dechunk).b;
    let mut te_chunked: off_t = (*(*r).gw_dechunk).gw_chunked;
    while len != 0 {
        if 0 as libc::c_int as libc::c_long == te_chunked {
            let mut p: *const libc::c_char = 0 as *const libc::c_char;
            let mut s: *mut libc::c_uchar = mem as *mut libc::c_uchar;
            let mut hsz: off_t = 0;
            if buffer_is_blank(h) != 0 {
                p = memchr(mem as *const libc::c_void, '\n' as i32, len as size_t)
                    as *const libc::c_char;
                if !p.is_null() {
                    p = p.offset(1);
                    hsz = p.offset_from(mem) as libc::c_long;
                } else {
                    if len >= 1024 as libc::c_int as libc::c_long {
                        log_error(
                            (*r).conf.errh,
                            b"src/http_chunk.c\0" as *const u8 as *const libc::c_char,
                            313 as libc::c_int as libc::c_uint,
                            b"chunked header line too long\0" as *const u8
                                as *const libc::c_char,
                        );
                        return -(1 as libc::c_int);
                    }
                    buffer_append_string_len(h, mem, len as uint32_t as size_t);
                    break;
                }
            } else {
                let mut hlen: uint32_t = buffer_clen(h);
                p = strchr((*h).ptr, '\n' as i32);
                if !p.is_null() {
                    p = p.offset(1);
                    hsz = p.offset_from((*h).ptr) as libc::c_long;
                } else {
                    p = memchr(mem as *const libc::c_void, '\n' as i32, len as size_t)
                        as *const libc::c_char;
                    hsz = if !p.is_null() {
                        p = p.offset(1);
                        p.offset_from(mem) as libc::c_long
                    } else {
                        len
                    };
                    if ((1024 as libc::c_int as libc::c_uint).wrapping_sub(hlen)
                        as off_t) < hsz
                    {
                        log_error(
                            (*r).conf.errh,
                            b"src/http_chunk.c\0" as *const u8 as *const libc::c_char,
                            330 as libc::c_int as libc::c_uint,
                            b"chunked header line too long\0" as *const u8
                                as *const libc::c_char,
                        );
                        return -(1 as libc::c_int);
                    }
                    buffer_append_string_len(h, mem, hsz as size_t);
                    if p.is_null() {
                        break;
                    }
                    mem = mem.offset(hsz as isize);
                    len -= hsz;
                    hsz = 0 as libc::c_int as off_t;
                }
                s = (*h).ptr as *mut libc::c_uchar;
            }
            let mut u: libc::c_uchar = 0;
            loop {
                u = hex2int(*s) as libc::c_uchar;
                if !(u as libc::c_int != 0xff as libc::c_int) {
                    break;
                }
                if te_chunked
                    > ((1 as libc::c_ulonglong)
                        << (8 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<off_t>() as libc::c_ulong,
                            )
                            .wrapping_sub(5 as libc::c_int as libc::c_ulong)) as off_t
                        - 1 as libc::c_int as libc::c_long
                        - 2 as libc::c_int as libc::c_long
                {
                    log_error(
                        (*r).conf.errh,
                        b"src/http_chunk.c\0" as *const u8 as *const libc::c_char,
                        345 as libc::c_int as libc::c_uint,
                        b"chunked data size too large\0" as *const u8
                            as *const libc::c_char,
                    );
                    return -(1 as libc::c_int);
                }
                te_chunked <<= 4 as libc::c_int;
                te_chunked |= u as libc::c_long;
                s = s.offset(1);
            }
            if s as *mut libc::c_char == mem as *mut libc::c_char
                || s as *mut libc::c_char == (*h).ptr
            {
                return -(1 as libc::c_int);
            }
            while *s as libc::c_int == ' ' as i32 || *s as libc::c_int == '\t' as i32 {
                s = s.offset(1);
            }
            if *s as libc::c_int != '\r' as i32 && *s as libc::c_int != ';' as i32 {
                log_error(
                    (*r).conf.errh,
                    b"src/http_chunk.c\0" as *const u8 as *const libc::c_char,
                    355 as libc::c_int as libc::c_uint,
                    b"chunked header invalid chars\0" as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            if 0 as libc::c_int as libc::c_long == te_chunked {
                if len - hsz >= 2 as libc::c_int as libc::c_long
                    && *p.offset(0 as libc::c_int as isize) as libc::c_int == '\r' as i32
                    && *p.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32
                {
                    if len - hsz > 2 as libc::c_int as libc::c_long {
                        return -(1 as libc::c_int);
                    }
                    buffer_clear(h);
                    (*(*r).gw_dechunk).done = (*r).http_status;
                    break;
                } else {
                    let mut mlen: uint32_t = buffer_clen(h);
                    mlen = if (*r).conf.max_request_field_size > mlen {
                        ((*r).conf.max_request_field_size).wrapping_sub(mlen)
                    } else {
                        0 as libc::c_int as libc::c_uint
                    };
                    if (mlen as off_t) < len {
                        if (*r).resp_send_chunked != 0 {
                            (*r).keep_alive = 0 as libc::c_int as int8_t;
                        }
                        (*(*r).gw_dechunk).done = (*r).http_status;
                        buffer_append_string_len(h, mem, mlen as size_t);
                        p = strrchr((*h).ptr, '\n' as i32);
                        if !p.is_null() {
                            buffer_truncate(
                                h,
                                p.offset(1 as libc::c_int as isize).offset_from((*h).ptr)
                                    as libc::c_long as uint32_t,
                            );
                            if *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                != '\r' as i32
                            {
                                buffer_append_string_len(
                                    h,
                                    b"\r\n\0" as *const u8 as *const libc::c_char,
                                    (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                                        as uint32_t)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                );
                            }
                        } else {
                            buffer_clear(h);
                            buffer_append_string_len(
                                h,
                                b"0\r\n\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                                    as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                            );
                        }
                        buffer_append_string_len(
                            h,
                            b"\r\n\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        );
                        break;
                    } else {
                        buffer_append_string_len(h, mem, len as uint32_t as size_t);
                        p = strstr(
                            (*h).ptr,
                            b"\r\n\r\n\0" as *const u8 as *const libc::c_char,
                        );
                        if !p.is_null() {
                            (*(*r).gw_dechunk).done = (*r).http_status;
                            if *p.offset(4 as libc::c_int as isize) as libc::c_int
                                != '\u{0}' as i32
                            {
                                return -(1 as libc::c_int);
                            }
                        }
                        break;
                    }
                }
            } else {
                mem = mem.offset(hsz as isize);
                len -= hsz;
                te_chunked += 2 as libc::c_int as libc::c_long;
                buffer_clear(h);
                if 0 as libc::c_int as libc::c_long == len {
                    break;
                }
            }
        }
        if te_chunked >= 2 as libc::c_int as libc::c_long {
            let mut clen: off_t = te_chunked - 2 as libc::c_int as libc::c_long;
            if clen > len {
                clen = len;
            }
            if (*r).resp_send_chunked == 0
                && 0 as libc::c_int != http_chunk_append_mem(r, mem, clen as size_t)
            {
                return -(1 as libc::c_int);
            }
            mem = mem.offset(clen as isize);
            len -= clen;
            te_chunked -= clen;
            if !(te_chunked == 2 as libc::c_int as libc::c_long) {
                continue;
            }
            if len >= 2 as libc::c_int as libc::c_long {
                if *mem.offset(0 as libc::c_int as isize) as libc::c_int != '\r' as i32
                    || *mem.offset(1 as libc::c_int as isize) as libc::c_int
                        != '\n' as i32
                {
                    return -(1 as libc::c_int);
                }
                mem = mem.offset(2 as libc::c_int as isize);
                len -= 2 as libc::c_int as libc::c_long;
                te_chunked = 0 as libc::c_int as off_t;
            } else {
                if !(len == 1 as libc::c_int as libc::c_long) {
                    continue;
                }
                if *mem.offset(0 as libc::c_int as isize) as libc::c_int != '\r' as i32 {
                    return -(1 as libc::c_int);
                }
                te_chunked = 1 as libc::c_int as off_t;
                break;
            }
        } else if 1 as libc::c_int as libc::c_long == te_chunked {
            if *mem.offset(0 as libc::c_int as isize) as libc::c_int != '\n' as i32 {
                return -(1 as libc::c_int);
            }
            mem = mem.offset(1);
            len -= 1;
            te_chunked = 0 as libc::c_int as off_t;
        }
    }
    if (*(*r).gw_dechunk).done != 0 {
        (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
    }
    (*(*r).gw_dechunk).gw_chunked = te_chunked;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn http_chunk_decode_append_buffer(
    r: *mut request_st,
    mem: *mut buffer,
) -> libc::c_int {
    if 0 as libc::c_int
        != http_chunk_decode_append_data(r, (*mem).ptr, buffer_clen(mem) as off_t)
    {
        return -(1 as libc::c_int);
    }
    if (*r).resp_send_chunked != 0 {
        (*r).resp_send_chunked = 0 as libc::c_int as libc::c_char;
        let mut rc: libc::c_int = http_chunk_append_buffer(r, mem);
        (*r).resp_send_chunked = 1 as libc::c_int as libc::c_char;
        return rc;
    } else {
        buffer_clear(mem);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn http_chunk_decode_append_mem(
    r: *mut request_st,
    mem: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    if 0 as libc::c_int != http_chunk_decode_append_data(r, mem, len as off_t) {
        return -(1 as libc::c_int);
    }
    if (*r).resp_send_chunked != 0 {
        (*r).resp_send_chunked = 0 as libc::c_int as libc::c_char;
        let mut rc: libc::c_int = http_chunk_append_mem(r, mem, len);
        (*r).resp_send_chunked = 1 as libc::c_int as libc::c_char;
        return rc;
    }
    return 0 as libc::c_int;
}
