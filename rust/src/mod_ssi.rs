use ::libc;
extern "C" {
    pub type server;
    pub type connection;
    pub type pcre2_real_match_data_8;
    fn fdevent_fork_execve(
        name: *const libc::c_char,
        argv: *mut *mut libc::c_char,
        envp: *mut *mut libc::c_char,
        fdin: libc::c_int,
        fdout: libc::c_int,
        fderr: libc::c_int,
        dfd: libc::c_int,
    ) -> pid_t;
    fn fdevent_waitpid(
        pid: pid_t,
        status: *mut libc::c_int,
        nb: libc::c_int,
    ) -> libc::c_int;
    fn buffer_free_ptr(b: *mut buffer);
    fn buffer_copy_string(b: *mut buffer, s: *const libc::c_char);
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_string(b: *mut buffer, s: *const libc::c_char);
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_str2(
        b: *mut buffer,
        s1: *const libc::c_char,
        len1: size_t,
        s2: *const libc::c_char,
        len2: size_t,
    );
    fn buffer_append_int(b: *mut buffer, val: intmax_t);
    fn buffer_append_strftime(
        b: *mut buffer,
        format: *const libc::c_char,
        tm: *const tm,
    );
    fn buffer_eq_icase_ssn(
        a: *const libc::c_char,
        b: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
    fn buffer_is_equal(a: *const buffer, b: *const buffer) -> libc::c_int;
    fn buffer_append_string_encoded(
        b: *mut buffer,
        s: *const libc::c_char,
        s_len: size_t,
        encoding: buffer_encoding_t,
    );
    fn buffer_urldecode_path(b: *mut buffer);
    fn buffer_is_valid_UTF8(b: *const buffer) -> libc::c_int;
    fn buffer_path_simplify(b: *mut buffer);
    fn buffer_to_lower(b: *mut buffer);
    fn buffer_copy_path_len2(
        b: *mut buffer,
        s1: *const libc::c_char,
        len1: size_t,
        s2: *const libc::c_char,
        len2: size_t,
    );
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    static mut log_epoch_secs: unix_time64_t;
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
    fn array_init(n: uint32_t) -> *mut array;
    fn array_free(a: *mut array);
    fn array_reset_data_strings(a: *mut array);
    fn array_get_element_klen(
        a: *const array,
        key: *const libc::c_char,
        klen: uint32_t,
    ) -> *const data_unset;
    fn array_get_buf_ptr(
        a: *mut array,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn array_match_value_suffix(a: *const array, b: *const buffer) -> *const buffer;
    fn chunk_buffer_acquire() -> *mut buffer;
    fn chunk_buffer_release(b: *mut buffer);
    fn chunk_buffer_prepare_append(b: *mut buffer, sz: size_t) -> size_t;
    fn chunkqueue_append_file_fd(
        cq: *mut chunkqueue,
        fn_0: *const buffer,
        fd: libc::c_int,
        offset: off_t,
        len: off_t,
    );
    fn chunkqueue_append_mem(cq: *mut chunkqueue, mem: *const libc::c_char, len: size_t);
    fn chunkqueue_append_mem_to_tempfile(
        cq: *mut chunkqueue,
        mem: *const libc::c_char,
        len: size_t,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn chunkqueue_update_file(cq: *mut chunkqueue, c: *mut chunk, len: off_t);
    fn chunkqueue_reset(cq: *mut chunkqueue);
    fn http_cgi_headers(
        r: *mut request_st,
        opts: *mut http_cgi_opts,
        cb: http_cgi_header_append_cb,
        vdata: *mut libc::c_void,
    ) -> libc::c_int;
    fn http_chunk_transfer_cqlen(
        r: *mut request_st,
        src: *mut chunkqueue,
        len: size_t,
    ) -> libc::c_int;
    fn http_etag_create(etag: *mut buffer, st: *const stat, flags: libc::c_int);
    fn http_header_response_set(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
        v: *const libc::c_char,
        vlen: uint32_t,
    );
    fn http_header_request_get(
        r: *const request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn stat_cache_path_contains_symlink(
        name: *const buffer,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn stat_cache_open_rdonly_fstat(
        name: *const buffer,
        st: *mut stat,
        symlinks: libc::c_int,
    ) -> libc::c_int;
    fn config_plugin_values_init(
        srv: *mut server,
        p_d: *mut libc::c_void,
        cpk: *const config_plugin_keys_t,
        mname: *const libc::c_char,
    ) -> libc::c_int;
    fn config_check_cond(r: *mut request_st, context_ndx: libc::c_int) -> libc::c_int;
    fn http_response_set_last_modified(
        r: *mut request_st,
        lmtime: unix_time64_t,
    ) -> *const buffer;
    fn http_response_handle_cachable(
        r: *mut request_st,
        lmod: *const buffer,
        lmtime: unix_time64_t,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
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
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intmax_t = __intmax_t;
pub type unix_time64_t = time_t;
pub type unix_timespec64_t = timespec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin {
    pub data: *mut libc::c_void,
    pub handle_uri_raw: Option::<
        unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    >,
    pub handle_uri_clean: Option::<
        unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    >,
    pub handle_docroot: Option::<
        unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    >,
    pub handle_physical: Option::<
        unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    >,
    pub handle_request_env: Option::<
        unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    >,
    pub handle_request_done: Option::<
        unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    >,
    pub handle_subrequest_start: Option::<
        unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    >,
    pub handle_subrequest: Option::<
        unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    >,
    pub handle_response_start: Option::<
        unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    >,
    pub handle_request_reset: Option::<
        unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    >,
    pub handle_connection_accept: Option::<
        unsafe extern "C" fn(*mut connection, *mut libc::c_void) -> handler_t,
    >,
    pub handle_connection_shut_wr: Option::<
        unsafe extern "C" fn(*mut connection, *mut libc::c_void) -> handler_t,
    >,
    pub handle_connection_close: Option::<
        unsafe extern "C" fn(*mut connection, *mut libc::c_void) -> handler_t,
    >,
    pub handle_trigger: Option::<
        unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    >,
    pub handle_sighup: Option::<
        unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    >,
    pub handle_waitpid: Option::<
        unsafe extern "C" fn(
            *mut server,
            *mut libc::c_void,
            pid_t,
            libc::c_int,
        ) -> handler_t,
    >,
    pub init: Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
    pub priv_defaults: Option::<
        unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    >,
    pub set_defaults: Option::<
        unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    >,
    pub worker_init: Option::<
        unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    >,
    pub cleanup: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub name: *const libc::c_char,
    pub version: size_t,
    pub lib: *mut libc::c_void,
}
pub type handler_t = libc::c_uint;
pub const HANDLER_ERROR: handler_t = 4;
pub const HANDLER_WAIT_FOR_EVENT: handler_t = 3;
pub const HANDLER_COMEBACK: handler_t = 2;
pub const HANDLER_FINISHED: handler_t = 1;
pub const HANDLER_GO_ON: handler_t = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cond_match_t {
    pub comp_value: *const buffer,
    pub match_data: *mut pcre2_real_match_data_8,
    pub captures: libc::c_int,
    pub matches: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cond_cache_t {
    pub result: int8_t,
    pub local_result: int8_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type buffer_encoding_t = libc::c_uint;
pub const ENCODING_MINIMAL_XML: buffer_encoding_t = 3;
pub const ENCODING_HTML: buffer_encoding_t = 2;
pub const ENCODING_REL_URI_PART: buffer_encoding_t = 1;
pub const ENCODING_REL_URI: buffer_encoding_t = 0;
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
pub struct http_cgi_opts_t {
    pub authorizer: libc::c_int,
    pub break_scriptfilename_for_php: libc::c_int,
    pub docroot: *const buffer,
    pub strip_request_uri: *const buffer,
}
pub type http_cgi_opts = http_cgi_opts_t;
pub type http_cgi_header_append_cb = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_char,
        size_t,
        *const libc::c_char,
        size_t,
    ) -> libc::c_int,
>;
pub type http_header_e = libc::c_uint;
pub const HTTP_HEADER_X_XSS_PROTECTION: http_header_e = 58;
pub const HTTP_HEADER_X_FRAME_OPTIONS: http_header_e = 57;
pub const HTTP_HEADER_X_FORWARDED_PROTO: http_header_e = 56;
pub const HTTP_HEADER_X_FORWARDED_FOR: http_header_e = 55;
pub const HTTP_HEADER_X_CONTENT_TYPE_OPTIONS: http_header_e = 54;
pub const HTTP_HEADER_WWW_AUTHENTICATE: http_header_e = 53;
pub const HTTP_HEADER_VARY: http_header_e = 52;
pub const HTTP_HEADER_USER_AGENT: http_header_e = 51;
pub const HTTP_HEADER_UPGRADE_INSECURE_REQUESTS: http_header_e = 50;
pub const HTTP_HEADER_UPGRADE: http_header_e = 49;
pub const HTTP_HEADER_TRANSFER_ENCODING: http_header_e = 48;
pub const HTTP_HEADER_TE: http_header_e = 47;
pub const HTTP_HEADER_STRICT_TRANSPORT_SECURITY: http_header_e = 46;
pub const HTTP_HEADER_STATUS: http_header_e = 45;
pub const HTTP_HEADER_SET_COOKIE: http_header_e = 44;
pub const HTTP_HEADER_SERVER: http_header_e = 43;
pub const HTTP_HEADER_REFERRER_POLICY: http_header_e = 42;
pub const HTTP_HEADER_REFERER: http_header_e = 41;
pub const HTTP_HEADER_RANGE: http_header_e = 40;
pub const HTTP_HEADER_PRAGMA: http_header_e = 39;
pub const HTTP_HEADER_P3P: http_header_e = 38;
pub const HTTP_HEADER_ONION_LOCATION: http_header_e = 37;
pub const HTTP_HEADER_LOCATION: http_header_e = 36;
pub const HTTP_HEADER_LINK: http_header_e = 35;
pub const HTTP_HEADER_LAST_MODIFIED: http_header_e = 34;
pub const HTTP_HEADER_IF_UNMODIFIED_SINCE: http_header_e = 33;
pub const HTTP_HEADER_IF_RANGE: http_header_e = 32;
pub const HTTP_HEADER_IF_NONE_MATCH: http_header_e = 31;
pub const HTTP_HEADER_IF_MODIFIED_SINCE: http_header_e = 30;
pub const HTTP_HEADER_IF_MATCH: http_header_e = 29;
pub const HTTP_HEADER_HTTP2_SETTINGS: http_header_e = 28;
pub const HTTP_HEADER_HOST: http_header_e = 27;
pub const HTTP_HEADER_FORWARDED: http_header_e = 26;
pub const HTTP_HEADER_EXPIRES: http_header_e = 25;
pub const HTTP_HEADER_EXPECT_CT: http_header_e = 24;
pub const HTTP_HEADER_EXPECT: http_header_e = 23;
pub const HTTP_HEADER_ETAG: http_header_e = 22;
pub const HTTP_HEADER_DNT: http_header_e = 21;
pub const HTTP_HEADER_DATE: http_header_e = 20;
pub const HTTP_HEADER_COOKIE: http_header_e = 19;
pub const HTTP_HEADER_CONTENT_TYPE: http_header_e = 18;
pub const HTTP_HEADER_CONTENT_SECURITY_POLICY: http_header_e = 17;
pub const HTTP_HEADER_CONTENT_RANGE: http_header_e = 16;
pub const HTTP_HEADER_CONTENT_LOCATION: http_header_e = 15;
pub const HTTP_HEADER_CONTENT_LENGTH: http_header_e = 14;
pub const HTTP_HEADER_CONTENT_ENCODING: http_header_e = 13;
pub const HTTP_HEADER_CONNECTION: http_header_e = 12;
pub const HTTP_HEADER_CACHE_CONTROL: http_header_e = 11;
pub const HTTP_HEADER_AUTHORIZATION: http_header_e = 10;
pub const HTTP_HEADER_ALT_USED: http_header_e = 9;
pub const HTTP_HEADER_ALT_SVC: http_header_e = 8;
pub const HTTP_HEADER_ALLOW: http_header_e = 7;
pub const HTTP_HEADER_AGE: http_header_e = 6;
pub const HTTP_HEADER_ACCESS_CONTROL_ALLOW_ORIGIN: http_header_e = 5;
pub const HTTP_HEADER_ACCEPT_RANGES: http_header_e = 4;
pub const HTTP_HEADER_ACCEPT_LANGUAGE: http_header_e = 3;
pub const HTTP_HEADER_ACCEPT_ENCODING: http_header_e = 2;
pub const HTTP_HEADER_ACCEPT: http_header_e = 1;
pub const HTTP_HEADER_OTHER: http_header_e = 0;
pub type config_values_type_t = libc::c_uint;
pub const T_CONFIG_UNSUPPORTED: config_values_type_t = 12;
pub const T_CONFIG_DEPRECATED: config_values_type_t = 11;
pub const T_CONFIG_LOCAL: config_values_type_t = 10;
pub const T_CONFIG_ARRAY_VLIST: config_values_type_t = 9;
pub const T_CONFIG_ARRAY_KVSTRING: config_values_type_t = 8;
pub const T_CONFIG_ARRAY_KVARRAY: config_values_type_t = 7;
pub const T_CONFIG_ARRAY_KVANY: config_values_type_t = 6;
pub const T_CONFIG_ARRAY: config_values_type_t = 5;
pub const T_CONFIG_BOOL: config_values_type_t = 4;
pub const T_CONFIG_INT: config_values_type_t = 3;
pub const T_CONFIG_SHORT: config_values_type_t = 2;
pub const T_CONFIG_STRING: config_values_type_t = 1;
pub const T_CONFIG_UNSET: config_values_type_t = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const T_CONFIG_SCOPE_CONNECTION: C2RustUnnamed_3 = 2;
pub const T_CONFIG_SCOPE_SERVER: C2RustUnnamed_3 = 1;
pub const T_CONFIG_SCOPE_UNSET: C2RustUnnamed_3 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_plugin_value {
    pub k_id: libc::c_int,
    pub vtype: config_values_type_t,
    pub v: v_u,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union v_u {
    pub v: *mut libc::c_void,
    pub a: *const array,
    pub b: *const buffer,
    pub s: *const libc::c_char,
    pub u: libc::c_uint,
    pub shrt: libc::c_ushort,
    pub d: libc::c_double,
    pub o: off_t,
    pub u2: [uint32_t; 2],
}
pub type config_plugin_value_t = config_plugin_value;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_plugin_keys_t {
    pub k: *const libc::c_char,
    pub klen: uint8_t,
    pub ktype: uint8_t,
    pub scope: uint8_t,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_4 = 8;
pub const _ISpunct: C2RustUnnamed_4 = 4;
pub const _IScntrl: C2RustUnnamed_4 = 2;
pub const _ISblank: C2RustUnnamed_4 = 1;
pub const _ISgraph: C2RustUnnamed_4 = 32768;
pub const _ISprint: C2RustUnnamed_4 = 16384;
pub const _ISspace: C2RustUnnamed_4 = 8192;
pub const _ISxdigit: C2RustUnnamed_4 = 4096;
pub const _ISdigit: C2RustUnnamed_4 = 2048;
pub const _ISalpha: C2RustUnnamed_4 = 1024;
pub const _ISlower: C2RustUnnamed_4 = 512;
pub const _ISupper: C2RustUnnamed_4 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_config {
    pub ssi_extension: *const array,
    pub content_type: *const buffer,
    pub conditional_requests: libc::c_ushort,
    pub ssi_exec: libc::c_ushort,
    pub ssi_recursion_max: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_data {
    pub id: libc::c_int,
    pub nconfig: libc::c_int,
    pub cvlist: *mut config_plugin_value_t,
    pub self_0: *mut plugin,
    pub defaults: plugin_config,
    pub conf: plugin_config,
    pub ssi_vars: *mut array,
    pub ssi_cgi_env: *mut array,
    pub stat_fn: buffer,
    pub timefmt: buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct handler_ctx {
    pub ssi_vars: *mut array,
    pub ssi_cgi_env: *mut array,
    pub stat_fn: *mut buffer,
    pub timefmt: *mut buffer,
    pub sizefmt: libc::c_int,
    pub if_level: libc::c_int,
    pub if_is_false_level: libc::c_int,
    pub if_is_false: libc::c_int,
    pub if_is_false_endif: libc::c_int,
    pub ssi_recursion_depth: libc::c_ushort,
    pub wq: chunkqueue,
    pub errh: *mut log_error_st,
    pub conf: plugin_config,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssi_tokenizer_t {
    pub input: *const libc::c_char,
    pub offset: size_t,
    pub size: size_t,
    pub in_brace: libc::c_int,
    pub depth: libc::c_int,
    pub p: *mut handler_ctx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssi_val_t {
    pub str_0: buffer,
    pub type_0: C2RustUnnamed_5,
    pub bo: libc::c_int,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const SSI_TYPE_STRING: C2RustUnnamed_5 = 2;
pub const SSI_TYPE_BOOL: C2RustUnnamed_5 = 1;
pub const SSI_TYPE_UNSET: C2RustUnnamed_5 = 0;
pub const SSI_COMMENT: C2RustUnnamed_8 = 13;
pub const SSI_ENDIF: C2RustUnnamed_8 = 11;
pub const SSI_ELIF: C2RustUnnamed_8 = 9;
pub const SSI_ELSE: C2RustUnnamed_8 = 10;
pub const SSI_IF: C2RustUnnamed_8 = 8;
pub const SSI_EXEC: C2RustUnnamed_8 = 12;
pub const SSI_PRINTENV: C2RustUnnamed_8 = 6;
pub const SSI_CONFIG: C2RustUnnamed_8 = 5;
pub const SSI_SET: C2RustUnnamed_8 = 7;
pub const SSI_INCLUDE: C2RustUnnamed_8 = 3;
pub const SSI_FLASTMOD: C2RustUnnamed_8 = 4;
pub const SSI_FSIZE: C2RustUnnamed_8 = 2;
pub const SSI_ECHO_SCRIPT_URL: C2RustUnnamed_6 = 8;
pub const SSI_ECHO_SCRIPT_URI: C2RustUnnamed_6 = 7;
pub const SSI_ECHO_DOCUMENT_URI: C2RustUnnamed_6 = 4;
pub const SSI_ECHO_DOCUMENT_NAME: C2RustUnnamed_6 = 3;
pub const SSI_ECHO_DATE_GMT: C2RustUnnamed_6 = 1;
pub const SSI_ECHO_LAST_MODIFIED: C2RustUnnamed_6 = 5;
pub const SSI_ECHO_DATE_LOCAL: C2RustUnnamed_6 = 2;
pub const SSI_ECHO_USER_NAME: C2RustUnnamed_6 = 6;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const SSI_ECHO_UNSET: C2RustUnnamed_6 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub var: *const libc::c_char,
    pub type_0: C2RustUnnamed_6,
}
pub const SSI_ECHO: C2RustUnnamed_8 = 1;
pub type C2RustUnnamed_8 = libc::c_uint;
pub const SSI_UNSET: C2RustUnnamed_8 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub var: *const libc::c_char,
    pub type_0: C2RustUnnamed_8,
}
#[inline]
unsafe extern "C" fn buffer_clear(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn light_isdigit(mut c: libc::c_int) -> libc::c_int {
    return ((c as uint32_t).wrapping_sub('0' as i32 as libc::c_uint)
        <= ('9' as i32 - '0' as i32) as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn light_isalpha(mut c: libc::c_int) -> libc::c_int {
    return ((c as uint32_t | 0x20 as libc::c_int as libc::c_uint)
        .wrapping_sub('a' as i32 as libc::c_uint)
        <= ('z' as i32 - 'a' as i32) as libc::c_uint) as libc::c_int;
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
unsafe extern "C" fn buffer_string_space(mut b: *const buffer) -> uint32_t {
    return if (*b).size != 0 {
        ((*b).size)
            .wrapping_sub(
                (*b).used
                    | (0 as libc::c_int as libc::c_uint == (*b).used) as libc::c_int
                        as libc::c_uint,
            )
    } else {
        0 as libc::c_int as libc::c_uint
    };
}
#[inline]
unsafe extern "C" fn buffer_copy_buffer(mut b: *mut buffer, mut src: *const buffer) {
    buffer_copy_string_len(b, (*src).ptr, buffer_clen(src) as size_t);
}
#[inline]
unsafe extern "C" fn array_set_key_value(
    a: *mut array,
    k: *const libc::c_char,
    klen: uint32_t,
    v: *const libc::c_char,
    vlen: uint32_t,
) {
    buffer_copy_string_len(array_get_buf_ptr(a, k, klen), v, vlen as size_t);
}
#[inline]
unsafe extern "C" fn chunkqueue_length(mut cq: *const chunkqueue) -> off_t {
    return (*cq).bytes_in - (*cq).bytes_out;
}
unsafe extern "C" fn handler_ctx_init(
    mut p: *mut plugin_data,
    mut errh: *mut log_error_st,
) -> *mut handler_ctx {
    let mut hctx: *mut handler_ctx = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<handler_ctx>() as libc::c_ulong,
    ) as *mut handler_ctx;
    if hctx.is_null() {
        ck_assert_failed(
            b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int as libc::c_uint,
            b"hctx\0" as *const u8 as *const libc::c_char,
        );
    }
    (*hctx).errh = errh;
    (*hctx).timefmt = &mut (*p).timefmt;
    (*hctx).stat_fn = &mut (*p).stat_fn;
    (*hctx).ssi_vars = (*p).ssi_vars;
    (*hctx).ssi_cgi_env = (*p).ssi_cgi_env;
    memcpy(
        &mut (*hctx).conf as *mut plugin_config as *mut libc::c_void,
        &mut (*p).conf as *mut plugin_config as *const libc::c_void,
        ::std::mem::size_of::<plugin_config>() as libc::c_ulong,
    );
    return hctx;
}
unsafe extern "C" fn handler_ctx_free(mut hctx: *mut handler_ctx) {
    chunkqueue_reset(&mut (*hctx).wq);
    free(hctx as *mut libc::c_void);
}
static mut include_file_last_mtime: unix_time64_t = 0 as libc::c_int as unix_time64_t;
#[cold]
unsafe extern "C" fn mod_ssi_init() -> *mut libc::c_void {
    let mut p: *mut plugin_data = 0 as *mut plugin_data;
    p = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<plugin_data>() as libc::c_ulong,
    ) as *mut plugin_data;
    if p.is_null() {
        ck_assert_failed(
            b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int as libc::c_uint,
            b"p\0" as *const u8 as *const libc::c_char,
        );
    }
    (*p).ssi_vars = array_init(8 as libc::c_int as uint32_t);
    (*p).ssi_cgi_env = array_init(32 as libc::c_int as uint32_t);
    return p as *mut libc::c_void;
}
#[cold]
unsafe extern "C" fn mod_ssi_free(mut p_d: *mut libc::c_void) {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    array_free((*p).ssi_vars);
    array_free((*p).ssi_cgi_env);
    free((*p).timefmt.ptr as *mut libc::c_void);
    free((*p).stat_fn.ptr as *mut libc::c_void);
}
unsafe extern "C" fn mod_ssi_merge_config_cpv(
    pconf: *mut plugin_config,
    cpv: *const config_plugin_value_t,
) {
    match (*cpv).k_id {
        0 => {
            (*pconf).ssi_extension = (*cpv).v.a;
        }
        1 => {
            (*pconf).content_type = (*cpv).v.b;
        }
        2 => {
            (*pconf).conditional_requests = (*cpv).v.u as libc::c_ushort;
        }
        3 => {
            (*pconf).ssi_exec = (*cpv).v.u as libc::c_ushort;
        }
        4 => {
            (*pconf).ssi_recursion_max = (*cpv).v.shrt;
        }
        _ => return,
    };
}
unsafe extern "C" fn mod_ssi_merge_config(
    pconf: *mut plugin_config,
    mut cpv: *const config_plugin_value_t,
) {
    loop {
        mod_ssi_merge_config_cpv(pconf, cpv);
        cpv = cpv.offset(1);
        if !((*cpv).k_id != -(1 as libc::c_int)) {
            break;
        }
    };
}
unsafe extern "C" fn mod_ssi_patch_config(r: *mut request_st, p: *mut plugin_data) {
    memcpy(
        &mut (*p).conf as *mut plugin_config as *mut libc::c_void,
        &mut (*p).defaults as *mut plugin_config as *const libc::c_void,
        ::std::mem::size_of::<plugin_config>() as libc::c_ulong,
    );
    let mut i: libc::c_int = 1 as libc::c_int;
    let mut used: libc::c_int = (*p).nconfig;
    while i < used {
        if config_check_cond(
            r,
            (*((*p).cvlist).offset(i as isize)).k_id as uint32_t as libc::c_int,
        ) != 0
        {
            mod_ssi_merge_config(
                &mut (*p).conf,
                ((*p).cvlist)
                    .offset(
                        (*((*p).cvlist).offset(i as isize))
                            .v
                            .u2[0 as libc::c_int as usize] as isize,
                    ),
            );
        }
        i += 1;
    }
}
static mut cpk: [config_plugin_keys_t; 6] = [config_plugin_keys_t {
    k: 0 as *const libc::c_char,
    klen: 0,
    ktype: 0,
    scope: 0,
}; 6];
#[cold]
unsafe extern "C" fn mod_ssi_set_defaults(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    if config_plugin_values_init(
        srv,
        p as *mut libc::c_void,
        cpk.as_ptr(),
        b"mod_ssi\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return HANDLER_ERROR;
    }
    let mut i: libc::c_int = ((*((*p).cvlist).offset(0 as libc::c_int as isize))
        .v
        .u2[1 as libc::c_int as usize] == 0) as libc::c_int;
    while i < (*p).nconfig {
        let mut cpv: *mut config_plugin_value_t = ((*p).cvlist)
            .offset(
                (*((*p).cvlist).offset(i as isize)).v.u2[0 as libc::c_int as usize]
                    as isize,
            );
        while -(1 as libc::c_int) != (*cpv).k_id {
            let mut current_block_3: u64;
            match (*cpv).k_id {
                0 => {
                    current_block_3 = 1841672684692190573;
                }
                1 => {
                    if buffer_is_blank((*cpv).v.b) != 0 {
                        (*cpv).v.b = 0 as *const buffer;
                    }
                    current_block_3 = 1841672684692190573;
                }
                2 => {
                    current_block_3 = 12727495758234751296;
                }
                3 => {
                    current_block_3 = 12727495758234751296;
                }
                4 => {
                    current_block_3 = 8480471748305462526;
                }
                _ => {
                    current_block_3 = 1841672684692190573;
                }
            }
            match current_block_3 {
                12727495758234751296 => {
                    current_block_3 = 8480471748305462526;
                }
                _ => {}
            }
            match current_block_3 {
                8480471748305462526 => {}
                _ => {}
            }
            cpv = cpv.offset(1);
        }
        i += 1;
    }
    (*p).defaults.ssi_exec = 1 as libc::c_int as libc::c_ushort;
    if (*p).nconfig > 0 as libc::c_int
        && (*(*p).cvlist).v.u2[1 as libc::c_int as usize] != 0
    {
        let mut cpv_0: *const config_plugin_value_t = ((*p).cvlist)
            .offset((*(*p).cvlist).v.u2[0 as libc::c_int as usize] as isize);
        if -(1 as libc::c_int) != (*cpv_0).k_id {
            mod_ssi_merge_config(&mut (*p).defaults, cpv_0);
        }
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn ssi_val_tobool(mut B: *const ssi_val_t) -> libc::c_int {
    return if (*B).type_0 as libc::c_uint == SSI_TYPE_BOOL as libc::c_int as libc::c_uint
    {
        (*B).bo
    } else {
        (buffer_is_blank(&(*B).str_0) == 0) as libc::c_int
    };
}
unsafe extern "C" fn ssi_eval_expr_cmp(
    v1: *const ssi_val_t,
    v2: *const ssi_val_t,
    cond: libc::c_int,
) -> libc::c_int {
    let mut cmp: libc::c_int = if (*v1).type_0 as libc::c_uint
        != SSI_TYPE_BOOL as libc::c_int as libc::c_uint
        && (*v2).type_0 as libc::c_uint != SSI_TYPE_BOOL as libc::c_int as libc::c_uint
    {
        strcmp(
            if !((*v1).str_0.ptr).is_null() {
                (*v1).str_0.ptr as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            if !((*v2).str_0.ptr).is_null() {
                (*v2).str_0.ptr as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        )
    } else {
        ssi_val_tobool(v1) - ssi_val_tobool(v2)
    };
    match cond {
        3 => return (cmp == 0 as libc::c_int) as libc::c_int,
        4 => return (cmp != 0 as libc::c_int) as libc::c_int,
        6 => return (cmp >= 0 as libc::c_int) as libc::c_int,
        5 => return (cmp > 0 as libc::c_int) as libc::c_int,
        8 => return (cmp <= 0 as libc::c_int) as libc::c_int,
        7 => return (cmp < 0 as libc::c_int) as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn ssi_eval_expr_cmp_bool(
    v1: *const ssi_val_t,
    v2: *const ssi_val_t,
    cond: libc::c_int,
) -> libc::c_int {
    return if cond == 2 as libc::c_int {
        (ssi_val_tobool(v1) != 0 || ssi_val_tobool(v2) != 0) as libc::c_int
    } else {
        (ssi_val_tobool(v1) != 0 && ssi_val_tobool(v2) != 0) as libc::c_int
    };
}
unsafe extern "C" fn ssi_eval_expr_append_val(
    b: *mut buffer,
    mut s: *const libc::c_char,
    slen: size_t,
) {
    if buffer_is_blank(b) != 0 {
        buffer_append_string_len(b, s, slen);
    } else if slen != 0 {
        buffer_append_str2(
            b,
            b" \0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            s,
            slen,
        );
    }
}
unsafe extern "C" fn ssi_expr_tokenizer(
    t: *mut ssi_tokenizer_t,
    token: *mut buffer,
) -> libc::c_int {
    let mut i: size_t = 0;
    while (*t).offset < (*t).size
        && (*((*t).input).offset((*t).offset as isize) as libc::c_int == ' ' as i32
            || *((*t).input).offset((*t).offset as isize) as libc::c_int == '\t' as i32)
    {
        (*t).offset = ((*t).offset).wrapping_add(1);
    }
    if (*t).offset >= (*t).size {
        return 0 as libc::c_int;
    }
    if *((*t).input).offset((*t).offset as isize) as libc::c_int == '\u{0}' as i32 {
        log_error(
            (*(*t).p).errh,
            b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
            286 as libc::c_int as libc::c_uint,
            b"pos: %zu foobar\0" as *const u8 as *const libc::c_char,
            ((*t).offset).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        return -(1 as libc::c_int);
    }
    match *((*t).input).offset((*t).offset as isize) as libc::c_int {
        61 => {
            (*t).offset = ((*t).offset).wrapping_add(1);
            return 3 as libc::c_int;
        }
        62 => {
            if *((*t).input)
                .offset(
                    ((*t).offset).wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as libc::c_int == '=' as i32
            {
                (*t)
                    .offset = ((*t).offset as libc::c_ulong)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                return 6 as libc::c_int;
            } else {
                (*t)
                    .offset = ((*t).offset as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
                return 5 as libc::c_int;
            }
        }
        60 => {
            if *((*t).input)
                .offset(
                    ((*t).offset).wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as libc::c_int == '=' as i32
            {
                (*t)
                    .offset = ((*t).offset as libc::c_ulong)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                return 8 as libc::c_int;
            } else {
                (*t)
                    .offset = ((*t).offset as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
                return 7 as libc::c_int;
            }
        }
        33 => {
            if *((*t).input)
                .offset(
                    ((*t).offset).wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as libc::c_int == '=' as i32
            {
                (*t)
                    .offset = ((*t).offset as libc::c_ulong)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                return 4 as libc::c_int;
            } else {
                (*t)
                    .offset = ((*t).offset as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
                return 9 as libc::c_int;
            }
        }
        38 => {
            if *((*t).input)
                .offset(
                    ((*t).offset).wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as libc::c_int == '&' as i32
            {
                (*t)
                    .offset = ((*t).offset as libc::c_ulong)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                return 1 as libc::c_int;
            } else {
                log_error(
                    (*(*t).p).errh,
                    b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                    332 as libc::c_int as libc::c_uint,
                    b"pos: %zu missing second &\0" as *const u8 as *const libc::c_char,
                    ((*t).offset).wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                return -(1 as libc::c_int);
            }
        }
        124 => {
            if *((*t).input)
                .offset(
                    ((*t).offset).wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as libc::c_int == '|' as i32
            {
                (*t)
                    .offset = ((*t).offset as libc::c_ulong)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                return 2 as libc::c_int;
            } else {
                log_error(
                    (*(*t).p).errh,
                    b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                    342 as libc::c_int as libc::c_uint,
                    b"pos: %zu missing second |\0" as *const u8 as *const libc::c_char,
                    ((*t).offset).wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                return -(1 as libc::c_int);
            }
        }
        40 => {
            (*t).offset = ((*t).offset).wrapping_add(1);
            (*t).in_brace += 1;
            return 10 as libc::c_int;
        }
        41 => {
            (*t).offset = ((*t).offset).wrapping_add(1);
            (*t).in_brace -= 1;
            return 11 as libc::c_int;
        }
        39 => {
            i = 1 as libc::c_int as size_t;
            while *((*t).input).offset(((*t).offset).wrapping_add(i) as isize)
                as libc::c_int != 0
                && *((*t).input).offset(((*t).offset).wrapping_add(i) as isize)
                    as libc::c_int != '\'' as i32
            {
                i = i.wrapping_add(1);
            }
            if *((*t).input).offset(((*t).offset).wrapping_add(i) as isize) != 0 {
                ssi_eval_expr_append_val(
                    token,
                    ((*t).input)
                        .offset((*t).offset as isize)
                        .offset(1 as libc::c_int as isize),
                    i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                (*t)
                    .offset = ((*t).offset as libc::c_ulong)
                    .wrapping_add(i.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as size_t as size_t;
                return 12 as libc::c_int;
            } else {
                log_error(
                    (*(*t).p).errh,
                    b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                    365 as libc::c_int as libc::c_uint,
                    b"pos: %zu missing closing quote\0" as *const u8
                        as *const libc::c_char,
                    ((*t).offset).wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                return -(1 as libc::c_int);
            }
        }
        36 => {
            let mut var: *const libc::c_char = 0 as *const libc::c_char;
            let mut varlen: size_t = 0;
            if *((*t).input)
                .offset(
                    ((*t).offset).wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as libc::c_int == '{' as i32
            {
                i = 2 as libc::c_int as size_t;
                while *((*t).input).offset(((*t).offset).wrapping_add(i) as isize)
                    as libc::c_int != 0
                    && *((*t).input).offset(((*t).offset).wrapping_add(i) as isize)
                        as libc::c_int != '}' as i32
                {
                    i = i.wrapping_add(1);
                }
                if *((*t).input).offset(((*t).offset).wrapping_add(i) as isize)
                    as libc::c_int != '}' as i32
                {
                    log_error(
                        (*(*t).p).errh,
                        b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                        377 as libc::c_int as libc::c_uint,
                        b"pos: %zu missing closing curly-brace\0" as *const u8
                            as *const libc::c_char,
                        ((*t).offset).wrapping_add(1 as libc::c_int as libc::c_ulong),
                    );
                    return -(1 as libc::c_int);
                }
                i = i.wrapping_add(1);
                var = ((*t).input)
                    .offset((*t).offset as isize)
                    .offset(2 as libc::c_int as isize);
                varlen = i.wrapping_sub(3 as libc::c_int as libc::c_ulong);
            } else {
                i = 1 as libc::c_int as size_t;
                while light_isalpha(
                    *((*t).input).offset(((*t).offset).wrapping_add(i) as isize)
                        as libc::c_int,
                ) != 0
                    || *((*t).input).offset(((*t).offset).wrapping_add(i) as isize)
                        as libc::c_int == '_' as i32
                    || i > 1 as libc::c_int as libc::c_ulong
                        && light_isdigit(
                            *((*t).input).offset(((*t).offset).wrapping_add(i) as isize)
                                as libc::c_int,
                        ) != 0
                {
                    i = i.wrapping_add(1);
                }
                var = ((*t).input)
                    .offset((*t).offset as isize)
                    .offset(1 as libc::c_int as isize);
                varlen = i.wrapping_sub(1 as libc::c_int as libc::c_ulong);
            }
            let mut ds: *const data_string = 0 as *const data_string;
            ds = array_get_element_klen((*(*t).p).ssi_cgi_env, var, varlen as uint32_t)
                as *const data_string;
            if !ds.is_null()
                || {
                    ds = array_get_element_klen(
                        (*(*t).p).ssi_vars,
                        var,
                        varlen as uint32_t,
                    ) as *const data_string;
                    !ds.is_null()
                }
            {
                ssi_eval_expr_append_val(
                    token,
                    (*ds).value.ptr,
                    buffer_clen(&(*ds).value) as size_t,
                );
            }
            (*t)
                .offset = ((*t).offset as libc::c_ulong).wrapping_add(i) as size_t
                as size_t;
            return 12 as libc::c_int;
        }
        _ => {
            i = 0 as libc::c_int as size_t;
            while *(*__ctype_b_loc())
                .offset(
                    *((*t).input as *mut libc::c_uchar)
                        .offset(((*t).offset).wrapping_add(i) as isize) as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISgraph as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                let mut d: libc::c_char = *((*t).input)
                    .offset(((*t).offset).wrapping_add(i) as isize);
                match d as libc::c_int {
                    32 | 9 | 41 | 40 | 39 | 61 | 33 | 60 | 62 | 38 | 124 => {
                        break;
                    }
                    _ => {}
                }
                i = i.wrapping_add(1);
            }
            ssi_eval_expr_append_val(
                token,
                ((*t).input).offset((*t).offset as isize),
                i,
            );
            (*t)
                .offset = ((*t).offset as libc::c_ulong).wrapping_add(i) as size_t
                as size_t;
            return 12 as libc::c_int;
        }
    };
}
unsafe extern "C" fn ssi_eval_expr_step(
    t: *mut ssi_tokenizer_t,
    v: *mut ssi_val_t,
) -> libc::c_int {
    buffer_clear(&mut (*v).str_0);
    (*v).type_0 = SSI_TYPE_UNSET;
    let mut next: libc::c_int = 0;
    let level: libc::c_int = (*t).in_brace;
    next = ssi_expr_tokenizer(t, &mut (*v).str_0);
    match next {
        12 => {
            loop {
                next = ssi_expr_tokenizer(t, &mut (*v).str_0);
                if !(next == 12 as libc::c_int) {
                    break;
                }
            }
            return next;
        }
        10 => {
            if (*t).in_brace > 16 as libc::c_int {
                return -(1 as libc::c_int);
            }
            next = ssi_eval_expr_loop(t, v);
            if next == 11 as libc::c_int && level == (*t).in_brace {
                let mut result: libc::c_int = ssi_val_tobool(v);
                next = ssi_eval_expr_step(t, v);
                (*v).bo = result;
                (*v).type_0 = SSI_TYPE_BOOL;
                return if next == 1 as libc::c_int || next == 2 as libc::c_int
                    || next == 11 as libc::c_int || 0 as libc::c_int == next
                {
                    next
                } else {
                    -(1 as libc::c_int)
                };
            } else {
                return -(1 as libc::c_int)
            }
        }
        11 => {
            return if (*t).in_brace >= 0 as libc::c_int {
                11 as libc::c_int
            } else {
                -(1 as libc::c_int)
            };
        }
        9 => {
            (*t).depth += 1;
            if (*t).depth > 16 as libc::c_int {
                return -(1 as libc::c_int);
            }
            next = ssi_eval_expr_step(t, v);
            (*t).depth -= 1;
            if -(1 as libc::c_int) == next {
                return next;
            }
            (*v).bo = (ssi_val_tobool(v) == 0) as libc::c_int;
            (*v).type_0 = SSI_TYPE_BOOL;
            return next;
        }
        _ => return next,
    };
}
unsafe extern "C" fn ssi_eval_expr_loop_cmp(
    t: *mut ssi_tokenizer_t,
    v1: *mut ssi_val_t,
    mut cond: libc::c_int,
) -> libc::c_int {
    let mut v2: ssi_val_t = {
        let mut init = ssi_val_t {
            str_0: {
                let mut init = buffer {
                    ptr: 0 as *mut libc::c_char,
                    used: 0 as libc::c_int as uint32_t,
                    size: 0 as libc::c_int as uint32_t,
                };
                init
            },
            type_0: SSI_TYPE_UNSET,
            bo: 0 as libc::c_int,
        };
        init
    };
    let mut next: libc::c_int = ssi_eval_expr_step(t, &mut v2);
    if -(1 as libc::c_int) != next {
        (*v1).bo = ssi_eval_expr_cmp(v1, &mut v2, cond);
        (*v1).type_0 = SSI_TYPE_BOOL;
    }
    buffer_free_ptr(&mut v2.str_0);
    return next;
}
unsafe extern "C" fn ssi_eval_expr_loop(
    t: *mut ssi_tokenizer_t,
    v1: *mut ssi_val_t,
) -> libc::c_int {
    let mut next: libc::c_int = ssi_eval_expr_step(t, v1);
    match next {
        1 | 2 => {}
        3 | 4 | 5 | 6 | 7 | 8 => {
            next = ssi_eval_expr_loop_cmp(t, v1, next);
            if next == 11 as libc::c_int || 0 as libc::c_int == next {
                return next;
            }
            if next != 1 as libc::c_int && next != 2 as libc::c_int {
                log_error(
                    (*(*t).p).errh,
                    b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                    490 as libc::c_int as libc::c_uint,
                    b"pos: %zu parser failed somehow near here\0" as *const u8
                        as *const libc::c_char,
                    ((*t).offset).wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                return -(1 as libc::c_int);
            }
        }
        _ => return next,
    }
    let mut v2: ssi_val_t = {
        let mut init = ssi_val_t {
            str_0: {
                let mut init = buffer {
                    ptr: 0 as *mut libc::c_char,
                    used: 0 as libc::c_int as uint32_t,
                    size: 0 as libc::c_int as uint32_t,
                };
                init
            },
            type_0: SSI_TYPE_UNSET,
            bo: 0 as libc::c_int,
        };
        init
    };
    let mut current_block_12: u64;
    loop {
        let mut cond: libc::c_int = next;
        next = ssi_eval_expr_step(t, &mut v2);
        match next {
            3 | 4 | 5 | 6 | 7 | 8 => {
                next = ssi_eval_expr_loop_cmp(t, &mut v2, next);
                if -(1 as libc::c_int) == next {
                    current_block_12 = 13586036798005543211;
                } else {
                    current_block_12 = 11307063007268554308;
                }
            }
            1 | 2 | 0 | 11 => {
                current_block_12 = 11307063007268554308;
            }
            _ => {
                current_block_12 = 13586036798005543211;
            }
        }
        match current_block_12 {
            11307063007268554308 => {
                (*v1).bo = ssi_eval_expr_cmp_bool(v1, &mut v2, cond);
                (*v1).type_0 = SSI_TYPE_BOOL;
            }
            _ => {}
        }
        if !(next == 1 as libc::c_int || next == 2 as libc::c_int) {
            break;
        }
    }
    buffer_free_ptr(&mut v2.str_0);
    return next;
}
unsafe extern "C" fn ssi_eval_expr(
    mut p: *mut handler_ctx,
    mut expr: *const libc::c_char,
) -> libc::c_int {
    let mut t: ssi_tokenizer_t = ssi_tokenizer_t {
        input: 0 as *const libc::c_char,
        offset: 0,
        size: 0,
        in_brace: 0,
        depth: 0,
        p: 0 as *mut handler_ctx,
    };
    t.input = expr;
    t.offset = 0 as libc::c_int as size_t;
    t.size = strlen(expr);
    t.in_brace = 0 as libc::c_int;
    t.depth = 0 as libc::c_int;
    t.p = p;
    let mut v: ssi_val_t = {
        let mut init = ssi_val_t {
            str_0: {
                let mut init = buffer {
                    ptr: 0 as *mut libc::c_char,
                    used: 0 as libc::c_int as uint32_t,
                    size: 0 as libc::c_int as uint32_t,
                };
                init
            },
            type_0: SSI_TYPE_UNSET,
            bo: 0 as libc::c_int,
        };
        init
    };
    let mut rc: libc::c_int = ssi_eval_expr_loop(&mut t, &mut v);
    rc = if 0 as libc::c_int == rc && 0 as libc::c_int == t.in_brace
        && 0 as libc::c_int == t.depth
    {
        ssi_val_tobool(&mut v)
    } else {
        -(1 as libc::c_int)
    };
    buffer_free_ptr(&mut v.str_0);
    return rc;
}
unsafe extern "C" fn ssi_env_add(
    mut venv: *mut libc::c_void,
    mut key: *const libc::c_char,
    mut klen: size_t,
    mut val: *const libc::c_char,
    mut vlen: size_t,
) -> libc::c_int {
    array_set_key_value(
        venv as *mut array,
        key,
        klen as uint32_t,
        val,
        vlen as uint32_t,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn build_ssi_cgi_vars(
    r: *mut request_st,
    p: *mut handler_ctx,
) -> libc::c_int {
    let mut opts: http_cgi_opts = {
        let mut init = http_cgi_opts_t {
            authorizer: 0 as libc::c_int,
            break_scriptfilename_for_php: 0 as libc::c_int,
            docroot: 0 as *const buffer,
            strip_request_uri: 0 as *const buffer,
        };
        init
    };
    let mut vb_auth: *mut buffer = http_header_request_get(
        r,
        HTTP_HEADER_AUTHORIZATION,
        b"Authorization\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    let mut b_auth: buffer = buffer {
        ptr: 0 as *mut libc::c_char,
        used: 0,
        size: 0,
    };
    if !vb_auth.is_null() {
        memcpy(
            &mut b_auth as *mut buffer as *mut libc::c_void,
            vb_auth as *const libc::c_void,
            ::std::mem::size_of::<buffer>() as libc::c_ulong,
        );
        memset(
            vb_auth as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<buffer>() as libc::c_ulong,
        );
    }
    array_reset_data_strings((*p).ssi_cgi_env);
    if 0 as libc::c_int
        != http_cgi_headers(
            r,
            &mut opts,
            Some(
                ssi_env_add
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        size_t,
                        *const libc::c_char,
                        size_t,
                    ) -> libc::c_int,
            ),
            (*p).ssi_cgi_env as *mut libc::c_void,
        )
    {
        (*r).http_status = 400 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if !vb_auth.is_null() {
        memcpy(
            vb_auth as *mut libc::c_void,
            &mut b_auth as *mut buffer as *const libc::c_void,
            ::std::mem::size_of::<buffer>() as libc::c_ulong,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mod_ssi_timefmt(
    b: *mut buffer,
    mut timefmtb: *mut buffer,
    mut t: unix_time64_t,
    mut localtm: libc::c_int,
) {
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let timefmt: *const libc::c_char = if buffer_is_blank(timefmtb) != 0 {
        b"%a, %d %b %Y %T %Z\0" as *const u8 as *const libc::c_char
    } else {
        (*timefmtb).ptr as *const libc::c_char
    };
    buffer_append_strftime(
        b,
        timefmt,
        if localtm != 0 {
            localtime_r(&mut t, &mut tm)
        } else {
            gmtime_r(&mut t, &mut tm)
        },
    );
    if buffer_is_blank(b) != 0 {
        buffer_copy_string_len(
            b,
            b"(none)\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
}
unsafe extern "C" fn process_ssi_stmt(
    r: *mut request_st,
    p: *mut handler_ctx,
    l: *mut *const libc::c_char,
    mut n: size_t,
    st: *mut stat,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut ssicmd: size_t = 0 as libc::c_int as size_t;
    let mut tb: *mut buffer = 0 as *mut buffer;
    static mut ssicmds: [C2RustUnnamed_9; 14] = [
        {
            let mut init = C2RustUnnamed_9 {
                var: b"echo\0" as *const u8 as *const libc::c_char,
                type_0: SSI_ECHO,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_9 {
                var: b"include\0" as *const u8 as *const libc::c_char,
                type_0: SSI_INCLUDE,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_9 {
                var: b"flastmod\0" as *const u8 as *const libc::c_char,
                type_0: SSI_FLASTMOD,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_9 {
                var: b"fsize\0" as *const u8 as *const libc::c_char,
                type_0: SSI_FSIZE,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_9 {
                var: b"config\0" as *const u8 as *const libc::c_char,
                type_0: SSI_CONFIG,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_9 {
                var: b"printenv\0" as *const u8 as *const libc::c_char,
                type_0: SSI_PRINTENV,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_9 {
                var: b"set\0" as *const u8 as *const libc::c_char,
                type_0: SSI_SET,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_9 {
                var: b"if\0" as *const u8 as *const libc::c_char,
                type_0: SSI_IF,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_9 {
                var: b"elif\0" as *const u8 as *const libc::c_char,
                type_0: SSI_ELIF,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_9 {
                var: b"endif\0" as *const u8 as *const libc::c_char,
                type_0: SSI_ENDIF,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_9 {
                var: b"else\0" as *const u8 as *const libc::c_char,
                type_0: SSI_ELSE,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_9 {
                var: b"exec\0" as *const u8 as *const libc::c_char,
                type_0: SSI_EXEC,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_9 {
                var: b"comment\0" as *const u8 as *const libc::c_char,
                type_0: SSI_COMMENT,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_9 {
                var: 0 as *const libc::c_char,
                type_0: SSI_UNSET,
            };
            init
        },
    ];
    i = 0 as libc::c_int as size_t;
    while !(ssicmds[i as usize].var).is_null() {
        if 0 as libc::c_int
            == strcmp(*l.offset(1 as libc::c_int as isize), ssicmds[i as usize].var)
        {
            ssicmd = ssicmds[i as usize].type_0 as size_t;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    let cq: *mut chunkqueue = &mut (*p).wq;
    let mut current_block_280: u64;
    match ssicmd {
        1 => {
            let mut var: libc::c_int = 0 as libc::c_int;
            let mut var_val: *const libc::c_char = 0 as *const libc::c_char;
            static mut echovars: [C2RustUnnamed_7; 9] = [
                {
                    let mut init = C2RustUnnamed_7 {
                        var: b"DATE_GMT\0" as *const u8 as *const libc::c_char,
                        type_0: SSI_ECHO_DATE_GMT,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_7 {
                        var: b"DATE_LOCAL\0" as *const u8 as *const libc::c_char,
                        type_0: SSI_ECHO_DATE_LOCAL,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_7 {
                        var: b"DOCUMENT_NAME\0" as *const u8 as *const libc::c_char,
                        type_0: SSI_ECHO_DOCUMENT_NAME,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_7 {
                        var: b"DOCUMENT_URI\0" as *const u8 as *const libc::c_char,
                        type_0: SSI_ECHO_DOCUMENT_URI,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_7 {
                        var: b"LAST_MODIFIED\0" as *const u8 as *const libc::c_char,
                        type_0: SSI_ECHO_LAST_MODIFIED,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_7 {
                        var: b"USER_NAME\0" as *const u8 as *const libc::c_char,
                        type_0: SSI_ECHO_USER_NAME,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_7 {
                        var: b"SCRIPT_URI\0" as *const u8 as *const libc::c_char,
                        type_0: SSI_ECHO_SCRIPT_URI,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_7 {
                        var: b"SCRIPT_URL\0" as *const u8 as *const libc::c_char,
                        type_0: SSI_ECHO_SCRIPT_URL,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_7 {
                        var: 0 as *const libc::c_char,
                        type_0: SSI_ECHO_UNSET,
                    };
                    init
                },
            ];
            i = 2 as libc::c_int as size_t;
            while i < n {
                if 0 as libc::c_int
                    == strcmp(
                        *l.offset(i as isize),
                        b"var\0" as *const u8 as *const libc::c_char,
                    )
                {
                    let mut j: libc::c_int = 0;
                    var_val = *l
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    j = 0 as libc::c_int;
                    while !(echovars[j as usize].var).is_null() {
                        if 0 as libc::c_int
                            == strcmp(
                                *l
                                    .offset(
                                        i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                    ),
                                echovars[j as usize].var,
                            )
                        {
                            var = echovars[j as usize].type_0 as libc::c_int;
                            break;
                        } else {
                            j += 1;
                        }
                    }
                } else if !(0 as libc::c_int
                        == strcmp(
                            *l.offset(i as isize),
                            b"encoding\0" as *const u8 as *const libc::c_char,
                        ))
                    {
                    log_error(
                        (*r).conf.errh,
                        b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                        757 as libc::c_int as libc::c_uint,
                        b"ssi: unknown attribute for %s %s\0" as *const u8
                            as *const libc::c_char,
                        *l.offset(1 as libc::c_int as isize),
                        *l.offset(i as isize),
                    );
                }
                i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
            }
            if !((*p).if_is_false != 0) {
                if var_val.is_null() {
                    log_error(
                        (*r).conf.errh,
                        b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                        765 as libc::c_int as libc::c_uint,
                        b"ssi: %s var is missing\0" as *const u8 as *const libc::c_char,
                        *l.offset(1 as libc::c_int as isize),
                    );
                } else {
                    match var {
                        6 => {
                            tb = (*r).tmp_buf;
                            buffer_clear(tb);
                            let mut pw: *mut passwd = 0 as *mut passwd;
                            pw = getpwuid((*st).st_uid);
                            if pw.is_null() {
                                buffer_append_int(tb, (*st).st_uid as intmax_t);
                            } else {
                                buffer_copy_string(tb, (*pw).pw_name);
                            }
                            chunkqueue_append_mem(
                                cq,
                                (*tb).ptr,
                                buffer_clen(tb) as size_t,
                            );
                        }
                        5 | 2 | 1 => {
                            tb = (*r).tmp_buf;
                            buffer_clear(tb);
                            mod_ssi_timefmt(
                                tb,
                                (*p).timefmt,
                                if var == SSI_ECHO_LAST_MODIFIED as libc::c_int {
                                    (*st).st_mtim.tv_sec
                                } else {
                                    log_epoch_secs
                                },
                                (var != SSI_ECHO_DATE_GMT as libc::c_int) as libc::c_int,
                            );
                            chunkqueue_append_mem(
                                cq,
                                (*tb).ptr,
                                buffer_clen(tb) as size_t,
                            );
                        }
                        3 => {
                            let mut sl: *mut libc::c_char = 0 as *mut libc::c_char;
                            sl = strrchr((*r).physical.path.ptr, '/' as i32);
                            if sl.is_null() {
                                chunkqueue_append_mem(
                                    cq,
                                    (*r).physical.path.ptr,
                                    buffer_clen(&mut (*r).physical.path) as size_t,
                                );
                            } else {
                                chunkqueue_append_mem(
                                    cq,
                                    sl.offset(1 as libc::c_int as isize),
                                    strlen(sl.offset(1 as libc::c_int as isize)),
                                );
                            }
                        }
                        4 => {
                            chunkqueue_append_mem(
                                cq,
                                (*r).uri.path.ptr,
                                buffer_clen(&mut (*r).uri.path) as size_t,
                            );
                        }
                        7 => {
                            if buffer_is_blank(&mut (*r).uri.scheme) == 0
                                && buffer_is_blank(&mut (*r).uri.authority) == 0
                            {
                                chunkqueue_append_mem(
                                    cq,
                                    (*r).uri.scheme.ptr,
                                    buffer_clen(&mut (*r).uri.scheme) as size_t,
                                );
                                chunkqueue_append_mem(
                                    cq,
                                    b"://\0" as *const u8 as *const libc::c_char,
                                    (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                                        as uint32_t)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                );
                                chunkqueue_append_mem(
                                    cq,
                                    (*r).uri.authority.ptr,
                                    buffer_clen(&mut (*r).uri.authority) as size_t,
                                );
                                chunkqueue_append_mem(
                                    cq,
                                    (*r).target.ptr,
                                    buffer_clen(&mut (*r).target) as size_t,
                                );
                                if buffer_is_blank(&mut (*r).uri.query) == 0 {
                                    chunkqueue_append_mem(
                                        cq,
                                        b"?\0" as *const u8 as *const libc::c_char,
                                        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                            as uint32_t)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                    );
                                    chunkqueue_append_mem(
                                        cq,
                                        (*r).uri.query.ptr,
                                        buffer_clen(&mut (*r).uri.query) as size_t,
                                    );
                                }
                            }
                        }
                        8 => {
                            chunkqueue_append_mem(
                                cq,
                                (*r).target.ptr,
                                buffer_clen(&mut (*r).target) as size_t,
                            );
                            if buffer_is_blank(&mut (*r).uri.query) == 0 {
                                chunkqueue_append_mem(
                                    cq,
                                    b"?\0" as *const u8 as *const libc::c_char,
                                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                        as uint32_t)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                );
                                chunkqueue_append_mem(
                                    cq,
                                    (*r).uri.query.ptr,
                                    buffer_clen(&mut (*r).uri.query) as size_t,
                                );
                            }
                        }
                        _ => {
                            let mut ds: *const data_string = 0 as *const data_string;
                            ds = array_get_element_klen(
                                (*p).ssi_cgi_env,
                                var_val,
                                strlen(var_val) as uint32_t,
                            ) as *const data_string;
                            if !ds.is_null()
                                || {
                                    ds = array_get_element_klen(
                                        (*p).ssi_vars,
                                        var_val,
                                        strlen(var_val) as uint32_t,
                                    ) as *const data_string;
                                    !ds.is_null()
                                }
                            {
                                chunkqueue_append_mem(
                                    cq,
                                    (*ds).value.ptr,
                                    buffer_clen(&(*ds).value) as size_t,
                                );
                            } else {
                                chunkqueue_append_mem(
                                    cq,
                                    b"(none)\0" as *const u8 as *const libc::c_char,
                                    (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                                        as uint32_t)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                );
                            }
                        }
                    }
                }
            }
        }
        3 | 4 | 2 => {
            let mut file_path: *const libc::c_char = 0 as *const libc::c_char;
            let mut virt_path: *const libc::c_char = 0 as *const libc::c_char;
            let mut stb: stat = stat {
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
            i = 2 as libc::c_int as size_t;
            while i < n {
                if 0 as libc::c_int
                    == strcmp(
                        *l.offset(i as isize),
                        b"file\0" as *const u8 as *const libc::c_char,
                    )
                {
                    file_path = *l
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                } else if 0 as libc::c_int
                        == strcmp(
                            *l.offset(i as isize),
                            b"virtual\0" as *const u8 as *const libc::c_char,
                        )
                    {
                    virt_path = *l
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                } else {
                    log_error(
                        (*r).conf.errh,
                        b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                        862 as libc::c_int as libc::c_uint,
                        b"ssi: unknown attribute for %s %s\0" as *const u8
                            as *const libc::c_char,
                        *l.offset(1 as libc::c_int as isize),
                        *l.offset(i as isize),
                    );
                }
                i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
            }
            if file_path.is_null() && virt_path.is_null() {
                log_error(
                    (*r).conf.errh,
                    b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                    868 as libc::c_int as libc::c_uint,
                    b"ssi: %s file or virtual is missing\0" as *const u8
                        as *const libc::c_char,
                    *l.offset(1 as libc::c_int as isize),
                );
            } else if !file_path.is_null() && !virt_path.is_null() {
                log_error(
                    (*r).conf.errh,
                    b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                    874 as libc::c_int as libc::c_uint,
                    b"ssi: %s only one of file and virtual is allowed here\0"
                        as *const u8 as *const libc::c_char,
                    *l.offset(1 as libc::c_int as isize),
                );
            } else if !((*p).if_is_false != 0) {
                tb = (*r).tmp_buf;
                if !file_path.is_null() {
                    buffer_copy_string(tb, file_path);
                    buffer_urldecode_path(tb);
                    if buffer_is_valid_UTF8(tb) == 0 {
                        log_error(
                            (*r).conf.errh,
                            b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                            889 as libc::c_int as libc::c_uint,
                            b"SSI invalid UTF-8 after url-decode: %s\0" as *const u8
                                as *const libc::c_char,
                            (*tb).ptr,
                        );
                        current_block_280 = 8769351727396938758;
                    } else {
                        buffer_path_simplify(tb);
                        let mut sl_0: *mut libc::c_char = strrchr(
                            (*r).physical.path.ptr,
                            '/' as i32,
                        );
                        if sl_0.is_null() {
                            current_block_280 = 8769351727396938758;
                        } else {
                            buffer_copy_path_len2(
                                (*p).stat_fn,
                                (*r).physical.path.ptr,
                                (sl_0.offset_from((*r).physical.path.ptr) as libc::c_long
                                    + 1 as libc::c_int as libc::c_long) as size_t,
                                (*tb).ptr,
                                buffer_clen(tb) as size_t,
                            );
                            current_block_280 = 10945915984064580713;
                        }
                    }
                } else {
                    buffer_clear(tb);
                    if *virt_path.offset(0 as libc::c_int as isize) as libc::c_int
                        != '/' as i32
                    {
                        let sl_1: *const libc::c_char = strrchr(
                            (*r).uri.path.ptr,
                            '/' as i32,
                        );
                        buffer_copy_string_len(
                            tb,
                            (*r).uri.path.ptr,
                            (sl_1.offset_from((*r).uri.path.ptr) as libc::c_long
                                + 1 as libc::c_int as libc::c_long) as size_t,
                        );
                    }
                    buffer_append_string(tb, virt_path);
                    buffer_urldecode_path(tb);
                    if buffer_is_valid_UTF8(tb) == 0 {
                        log_error(
                            (*r).conf.errh,
                            b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                            913 as libc::c_int as libc::c_uint,
                            b"SSI invalid UTF-8 after url-decode: %s\0" as *const u8
                                as *const libc::c_char,
                            (*tb).ptr,
                        );
                        current_block_280 = 8769351727396938758;
                    } else {
                        buffer_path_simplify(tb);
                        let mut sep: *const libc::c_char = 0 as *const libc::c_char;
                        let mut sep2: *const libc::c_char = 0 as *const libc::c_char;
                        sep = (*r).uri.path.ptr;
                        sep2 = (*tb).ptr;
                        i = 0 as libc::c_int as size_t;
                        while *sep.offset(i as isize) as libc::c_int != 0
                            && *sep.offset(i as isize) as libc::c_int
                                == *sep2.offset(i as isize) as libc::c_int
                        {
                            i = i.wrapping_add(1);
                        }
                        while i != 0 as libc::c_int as libc::c_ulong
                            && {
                                i = i.wrapping_sub(1);
                                *sep.offset(i as isize) as libc::c_int != '/' as i32
                            }
                        {}
                        if (*r).conf.force_lowercase_filenames != 0 {
                            buffer_to_lower(tb);
                        }
                        let mut remain: uint32_t = (buffer_clen(&mut (*r).uri.path)
                            as libc::c_ulong)
                            .wrapping_sub(i) as uint32_t;
                        let mut plen: uint32_t = buffer_clen(&mut (*r).physical.path);
                        if plen >= remain
                            && (if (*r).conf.force_lowercase_filenames == 0 {
                                (0 as libc::c_int
                                    == memcmp(
                                        ((*r).physical.path.ptr)
                                            .offset(plen as isize)
                                            .offset(-(remain as isize)) as *const libc::c_void,
                                        ((*r).physical.rel_path.ptr).offset(i as isize)
                                            as *const libc::c_void,
                                        remain as libc::c_ulong,
                                    )) as libc::c_int
                            } else {
                                buffer_eq_icase_ssn(
                                    ((*r).physical.path.ptr)
                                        .offset(plen as isize)
                                        .offset(-(remain as isize)),
                                    ((*r).physical.rel_path.ptr).offset(i as isize),
                                    remain as size_t,
                                )
                            }) != 0
                        {
                            buffer_copy_path_len2(
                                (*p).stat_fn,
                                (*r).physical.path.ptr,
                                plen.wrapping_sub(remain) as size_t,
                                ((*tb).ptr).offset(i as isize),
                                (buffer_clen(tb) as libc::c_ulong).wrapping_sub(i),
                            );
                        } else {
                            buffer_copy_path_len2(
                                (*p).stat_fn,
                                (*r).physical.doc_root.ptr,
                                buffer_clen(&mut (*r).physical.doc_root) as size_t,
                                (*tb).ptr,
                                buffer_clen(tb) as size_t,
                            );
                        }
                        current_block_280 = 10945915984064580713;
                    }
                }
                match current_block_280 {
                    8769351727396938758 => {}
                    _ => {
                        if !((*r).conf.follow_symlink == 0
                            && 0 as libc::c_int
                                != stat_cache_path_contains_symlink(
                                    (*p).stat_fn,
                                    (*r).conf.errh,
                                ))
                        {
                            let mut fd: libc::c_int = stat_cache_open_rdonly_fstat(
                                (*p).stat_fn,
                                &mut stb,
                                (*r).conf.follow_symlink as libc::c_int,
                            );
                            if fd >= 0 as libc::c_int {
                                match ssicmd {
                                    2 => {
                                        buffer_clear(tb);
                                        if (*p).sizefmt != 0 {
                                            let mut j_0: libc::c_int = 0 as libc::c_int;
                                            let mut abr: [*const libc::c_char; 6] = [
                                                b" B\0" as *const u8 as *const libc::c_char,
                                                b" kB\0" as *const u8 as *const libc::c_char,
                                                b" MB\0" as *const u8 as *const libc::c_char,
                                                b" GB\0" as *const u8 as *const libc::c_char,
                                                b" TB\0" as *const u8 as *const libc::c_char,
                                                0 as *const libc::c_char,
                                            ];
                                            let mut s: off_t = stb.st_size;
                                            j_0 = 0 as libc::c_int;
                                            while s > 1024 as libc::c_int as libc::c_long
                                                && !(abr[(j_0 + 1 as libc::c_int) as usize]).is_null()
                                            {
                                                s /= 1024 as libc::c_int as libc::c_long;
                                                j_0 += 1;
                                            }
                                            buffer_append_int(tb, s);
                                            buffer_append_string_len(
                                                tb,
                                                abr[j_0 as usize],
                                                (if j_0 != 0 { 3 as libc::c_int } else { 2 as libc::c_int })
                                                    as size_t,
                                            );
                                        } else {
                                            buffer_append_int(tb, stb.st_size);
                                        }
                                        chunkqueue_append_mem(
                                            cq,
                                            (*tb).ptr,
                                            buffer_clen(tb) as size_t,
                                        );
                                    }
                                    4 => {
                                        buffer_clear(tb);
                                        mod_ssi_timefmt(
                                            tb,
                                            (*p).timefmt,
                                            stb.st_mtim.tv_sec,
                                            1 as libc::c_int,
                                        );
                                        chunkqueue_append_mem(
                                            cq,
                                            (*tb).ptr,
                                            buffer_clen(tb) as size_t,
                                        );
                                    }
                                    3 => {
                                        if include_file_last_mtime < stb.st_mtim.tv_sec {
                                            ::std::ptr::write_volatile(
                                                &mut include_file_last_mtime as *mut unix_time64_t,
                                                stb.st_mtim.tv_sec,
                                            );
                                        }
                                        if !file_path.is_null()
                                            || 0 as libc::c_int
                                                == (*p).conf.ssi_recursion_max as libc::c_int
                                        {
                                            chunkqueue_append_file_fd(
                                                cq,
                                                (*p).stat_fn,
                                                fd,
                                                0 as libc::c_int as off_t,
                                                stb.st_size,
                                            );
                                            fd = -(1 as libc::c_int);
                                        } else {
                                            let mut upsave: buffer = buffer {
                                                ptr: 0 as *mut libc::c_char,
                                                used: 0,
                                                size: 0,
                                            };
                                            let mut ppsave: buffer = buffer {
                                                ptr: 0 as *mut libc::c_char,
                                                used: 0,
                                                size: 0,
                                            };
                                            let mut prpsave: buffer = buffer {
                                                ptr: 0 as *mut libc::c_char,
                                                used: 0,
                                                size: 0,
                                            };
                                            if (*p).ssi_recursion_depth as libc::c_int
                                                >= (*p).conf.ssi_recursion_max as libc::c_int
                                            {
                                                chunkqueue_append_mem(
                                                    cq,
                                                    b"(error: include directives recurse deeper than pre-defined ssi.recursion-max)\0"
                                                        as *const u8 as *const libc::c_char,
                                                    (::std::mem::size_of::<[libc::c_char; 78]>()
                                                        as libc::c_ulong as uint32_t)
                                                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                                );
                                            } else if buffer_is_equal(
                                                    &mut (*r).physical.path,
                                                    (*p).stat_fn,
                                                ) != 0
                                                {
                                                chunkqueue_append_mem(
                                                    cq,
                                                    b"(error: include directives create an infinite loop)\0"
                                                        as *const u8 as *const libc::c_char,
                                                    (::std::mem::size_of::<[libc::c_char; 52]>()
                                                        as libc::c_ulong as uint32_t)
                                                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                                );
                                            } else {
                                                upsave = (*r).uri.path;
                                                ppsave = (*r).physical.path;
                                                prpsave = (*r).physical.rel_path;
                                                (*r).physical.path = *(*p).stat_fn;
                                                memset(
                                                    (*p).stat_fn as *mut libc::c_void,
                                                    0 as libc::c_int,
                                                    ::std::mem::size_of::<buffer>() as libc::c_ulong,
                                                );
                                                memset(
                                                    &mut (*r).uri.path as *mut buffer as *mut libc::c_void,
                                                    0 as libc::c_int,
                                                    ::std::mem::size_of::<buffer>() as libc::c_ulong,
                                                );
                                                buffer_copy_buffer(&mut (*r).uri.path, tb);
                                                (*r).physical.rel_path = (*r).uri.path;
                                                close(fd);
                                                fd = -(1 as libc::c_int);
                                                (*p)
                                                    .ssi_recursion_depth = ((*p).ssi_recursion_depth)
                                                    .wrapping_add(1);
                                                mod_ssi_process_file(r, p, &mut stb);
                                                (*p)
                                                    .ssi_recursion_depth = ((*p).ssi_recursion_depth)
                                                    .wrapping_sub(1);
                                                free((*r).uri.path.ptr as *mut libc::c_void);
                                                (*r).uri.path = upsave;
                                                (*r).physical.rel_path = prpsave;
                                                free((*(*p).stat_fn).ptr as *mut libc::c_void);
                                                *(*p).stat_fn = (*r).physical.path;
                                                (*r).physical.path = ppsave;
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                if fd >= 0 as libc::c_int {
                                    close(fd);
                                }
                            } else {
                                log_perror(
                                    (*r).conf.errh,
                                    b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                                    1060 as libc::c_int as libc::c_uint,
                                    b"ssi: stating %s failed\0" as *const u8
                                        as *const libc::c_char,
                                    (*(*p).stat_fn).ptr,
                                );
                            }
                        }
                    }
                }
            }
        }
        7 => {
            let mut key: *const libc::c_char = 0 as *const libc::c_char;
            let mut val: *const libc::c_char = 0 as *const libc::c_char;
            i = 2 as libc::c_int as size_t;
            while i < n {
                if 0 as libc::c_int
                    == strcmp(
                        *l.offset(i as isize),
                        b"var\0" as *const u8 as *const libc::c_char,
                    )
                {
                    key = *l
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                } else if 0 as libc::c_int
                        == strcmp(
                            *l.offset(i as isize),
                            b"value\0" as *const u8 as *const libc::c_char,
                        )
                    {
                    val = *l
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                } else {
                    log_error(
                        (*r).conf.errh,
                        b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                        1073 as libc::c_int as libc::c_uint,
                        b"ssi: unknown attribute for %s %s\0" as *const u8
                            as *const libc::c_char,
                        *l.offset(1 as libc::c_int as isize),
                        *l.offset(i as isize),
                    );
                }
                i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
            }
            if !((*p).if_is_false != 0) {
                if !key.is_null() && !val.is_null() {
                    array_set_key_value(
                        (*p).ssi_vars,
                        key,
                        strlen(key) as uint32_t,
                        val,
                        strlen(val) as uint32_t,
                    );
                } else if !key.is_null() || !val.is_null() {
                    log_error(
                        (*r).conf.errh,
                        b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                        1083 as libc::c_int as libc::c_uint,
                        b"ssi: var and value have to be set in <!--#set %s=%s -->\0"
                            as *const u8 as *const libc::c_char,
                        *l.offset(1 as libc::c_int as isize),
                        *l.offset(2 as libc::c_int as isize),
                    );
                } else {
                    log_error(
                        (*r).conf.errh,
                        b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                        1086 as libc::c_int as libc::c_uint,
                        b"ssi: var and value have to be set in <!--#set var=... value=... -->\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        5 => {
            if !((*p).if_is_false != 0) {
                i = 2 as libc::c_int as size_t;
                while i < n {
                    if 0 as libc::c_int
                        == strcmp(
                            *l.offset(i as isize),
                            b"timefmt\0" as *const u8 as *const libc::c_char,
                        )
                    {
                        buffer_copy_string(
                            (*p).timefmt,
                            *l
                                .offset(
                                    i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                ),
                        );
                    } else if 0 as libc::c_int
                            == strcmp(
                                *l.offset(i as isize),
                                b"sizefmt\0" as *const u8 as *const libc::c_char,
                            )
                        {
                        if 0 as libc::c_int
                            == strcmp(
                                *l
                                    .offset(
                                        i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                    ),
                                b"abbrev\0" as *const u8 as *const libc::c_char,
                            )
                        {
                            (*p).sizefmt = 1 as libc::c_int;
                        } else if 0 as libc::c_int
                                == strcmp(
                                    *l
                                        .offset(
                                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                        ),
                                    b"bytes\0" as *const u8 as *const libc::c_char,
                                )
                            {
                            (*p).sizefmt = 0 as libc::c_int;
                        } else {
                            log_error(
                                (*r).conf.errh,
                                b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                                1103 as libc::c_int as libc::c_uint,
                                b"ssi: unknown value for attribute '%s' for %s %s\0"
                                    as *const u8 as *const libc::c_char,
                                *l.offset(i as isize),
                                *l.offset(1 as libc::c_int as isize),
                                *l
                                    .offset(
                                        i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                    ),
                            );
                        }
                    } else {
                        log_error(
                            (*r).conf.errh,
                            b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                            1108 as libc::c_int as libc::c_uint,
                            b"ssi: unknown attribute for %s %s\0" as *const u8
                                as *const libc::c_char,
                            *l.offset(1 as libc::c_int as isize),
                            *l.offset(i as isize),
                        );
                    }
                    i = (i as libc::c_ulong)
                        .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                }
            }
        }
        6 => {
            if !((*p).if_is_false != 0) {
                tb = (*r).tmp_buf;
                buffer_clear(tb);
                i = 0 as libc::c_int as size_t;
                while i < (*(*p).ssi_vars).used as libc::c_ulong {
                    let mut ds_0: *mut data_string = *((*(*p).ssi_vars).sorted)
                        .offset(i as isize) as *mut data_string;
                    buffer_append_str2(
                        tb,
                        (*ds_0).key.ptr,
                        buffer_clen(&mut (*ds_0).key) as size_t,
                        b"=\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    );
                    buffer_append_string_encoded(
                        tb,
                        (*ds_0).value.ptr,
                        buffer_clen(&mut (*ds_0).value) as size_t,
                        ENCODING_MINIMAL_XML,
                    );
                    buffer_append_string_len(
                        tb,
                        b"\n\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    );
                    i = i.wrapping_add(1);
                }
                i = 0 as libc::c_int as size_t;
                while i < (*(*p).ssi_cgi_env).used as libc::c_ulong {
                    let mut ds_1: *mut data_string = *((*(*p).ssi_cgi_env).sorted)
                        .offset(i as isize) as *mut data_string;
                    buffer_append_str2(
                        tb,
                        (*ds_1).key.ptr,
                        buffer_clen(&mut (*ds_1).key) as size_t,
                        b"=\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    );
                    buffer_append_string_encoded(
                        tb,
                        (*ds_1).value.ptr,
                        buffer_clen(&mut (*ds_1).value) as size_t,
                        ENCODING_MINIMAL_XML,
                    );
                    buffer_append_string_len(
                        tb,
                        b"\n\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    );
                    i = i.wrapping_add(1);
                }
                chunkqueue_append_mem(cq, (*tb).ptr, buffer_clen(tb) as size_t);
            }
        }
        12 => {
            let mut cmd: *const libc::c_char = 0 as *const libc::c_char;
            let mut pid: pid_t = 0;
            let mut c: *mut chunk = 0 as *mut chunk;
            let mut args: [*mut libc::c_char; 4] = [0 as *mut libc::c_char; 4];
            let mut errh: *mut log_error_st = (*p).errh;
            if !((*p).conf.ssi_exec == 0) {
                i = 2 as libc::c_int as size_t;
                while i < n {
                    if 0 as libc::c_int
                        == strcmp(
                            *l.offset(i as isize),
                            b"cmd\0" as *const u8 as *const libc::c_char,
                        )
                    {
                        cmd = *l
                            .offset(
                                i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            );
                    } else {
                        log_error(
                            errh,
                            b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                            1149 as libc::c_int as libc::c_uint,
                            b"ssi: unknown attribute for %s %s\0" as *const u8
                                as *const libc::c_char,
                            *l.offset(1 as libc::c_int as isize),
                            *l.offset(i as isize),
                        );
                    }
                    i = (i as libc::c_ulong)
                        .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                }
                if !((*p).if_is_false != 0) {
                    if !cmd.is_null() {
                        if !(0 as libc::c_int
                            != chunkqueue_append_mem_to_tempfile(
                                cq,
                                b"\0" as *const u8 as *const libc::c_char,
                                0 as libc::c_int as size_t,
                                errh,
                            ))
                        {
                            c = (*cq).last;
                            let ref mut fresh0 = *(&mut *args
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize) as *mut *mut libc::c_char
                                as *mut *const libc::c_char);
                            *fresh0 = b"/bin/sh\0" as *const u8 as *const libc::c_char;
                            let ref mut fresh1 = *(&mut *args
                                .as_mut_ptr()
                                .offset(1 as libc::c_int as isize) as *mut *mut libc::c_char
                                as *mut *const libc::c_char);
                            *fresh1 = b"-c\0" as *const u8 as *const libc::c_char;
                            let ref mut fresh2 = *(&mut *args
                                .as_mut_ptr()
                                .offset(2 as libc::c_int as isize) as *mut *mut libc::c_char
                                as *mut *const libc::c_char);
                            *fresh2 = cmd;
                            args[3 as libc::c_int as usize] = 0 as *mut libc::c_char;
                            let mut status: libc::c_int = 0 as libc::c_int;
                            let mut stb_0: stat = stat {
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
                            stb_0.st_size = 0 as libc::c_int as __off_t;
                            let mut serrh_fd: libc::c_int = if !((*r).conf.serrh)
                                .is_null()
                            {
                                (*(*r).conf.serrh).fd
                            } else {
                                -(1 as libc::c_int)
                            };
                            pid = fdevent_fork_execve(
                                args[0 as libc::c_int as usize],
                                args.as_mut_ptr(),
                                0 as *mut *mut libc::c_char,
                                -(1 as libc::c_int),
                                (*c).file.fd,
                                serrh_fd,
                                -(1 as libc::c_int),
                            );
                            if -(1 as libc::c_int) == pid {
                                log_perror(
                                    errh,
                                    b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                                    1178 as libc::c_int as libc::c_uint,
                                    b"spawning exec failed: %s\0" as *const u8
                                        as *const libc::c_char,
                                    cmd,
                                );
                            } else if fdevent_waitpid(pid, &mut status, 0 as libc::c_int)
                                    < 0 as libc::c_int
                                {
                                log_perror(
                                    errh,
                                    b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                                    1180 as libc::c_int as libc::c_uint,
                                    b"waitpid failed\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                if !(status & 0x7f as libc::c_int == 0 as libc::c_int) {
                                    log_error(
                                        errh,
                                        b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                                        1189 as libc::c_int as libc::c_uint,
                                        b"process exited abnormally: %s\0" as *const u8
                                            as *const libc::c_char,
                                        cmd,
                                    );
                                }
                                0 as libc::c_int == fstat((*c).file.fd, &mut stb_0);
                            }
                            chunkqueue_update_file(cq, c, stb_0.st_size);
                        }
                    }
                }
            }
        }
        8 => {
            let mut expr: *const libc::c_char = 0 as *const libc::c_char;
            i = 2 as libc::c_int as size_t;
            while i < n {
                if 0 as libc::c_int
                    == strcmp(
                        *l.offset(i as isize),
                        b"expr\0" as *const u8 as *const libc::c_char,
                    )
                {
                    expr = *l
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                } else {
                    log_error(
                        (*r).conf.errh,
                        b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                        1204 as libc::c_int as libc::c_uint,
                        b"ssi: unknown attribute for %s %s\0" as *const u8
                            as *const libc::c_char,
                        *l.offset(1 as libc::c_int as isize),
                        *l.offset(i as isize),
                    );
                }
                i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
            }
            if expr.is_null() {
                log_error(
                    (*r).conf.errh,
                    b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                    1210 as libc::c_int as libc::c_uint,
                    b"ssi: %s expr missing\0" as *const u8 as *const libc::c_char,
                    *l.offset(1 as libc::c_int as isize),
                );
            } else {
                if (*p).if_is_false == 0
                    && ((*p).if_is_false_level == 0 as libc::c_int
                        || (*p).if_level < (*p).if_is_false_level)
                {
                    match ssi_eval_expr(p, expr) {
                        -1 | 0 => {
                            (*p).if_is_false = 1 as libc::c_int;
                            (*p).if_is_false_level = (*p).if_level;
                        }
                        1 => {
                            (*p).if_is_false = 0 as libc::c_int;
                        }
                        _ => {}
                    }
                }
                (*p).if_level += 1;
            }
        }
        10 => {
            (*p).if_level -= 1;
            if (*p).if_is_false != 0 {
                if (*p).if_level == (*p).if_is_false_level
                    && (*p).if_is_false_endif == 0 as libc::c_int
                {
                    (*p).if_is_false = 0 as libc::c_int;
                }
            } else {
                (*p).if_is_false = 1 as libc::c_int;
                (*p).if_is_false_level = (*p).if_level;
            }
            (*p).if_level += 1;
        }
        9 => {
            let mut expr_0: *const libc::c_char = 0 as *const libc::c_char;
            i = 2 as libc::c_int as size_t;
            while i < n {
                if 0 as libc::c_int
                    == strcmp(
                        *l.offset(i as isize),
                        b"expr\0" as *const u8 as *const libc::c_char,
                    )
                {
                    expr_0 = *l
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                } else {
                    log_error(
                        (*r).conf.errh,
                        b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                        1256 as libc::c_int as libc::c_uint,
                        b"ssi: unknown attribute for %s %s\0" as *const u8
                            as *const libc::c_char,
                        *l.offset(1 as libc::c_int as isize),
                        *l.offset(i as isize),
                    );
                }
                i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
            }
            if expr_0.is_null() {
                log_error(
                    (*r).conf.errh,
                    b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                    1262 as libc::c_int as libc::c_uint,
                    b"ssi: %s expr missing\0" as *const u8 as *const libc::c_char,
                    *l.offset(1 as libc::c_int as isize),
                );
            } else {
                (*p).if_level -= 1;
                if (*p).if_level == (*p).if_is_false_level {
                    if (*p).if_is_false != 0
                        && (*p).if_is_false_endif == 0 as libc::c_int
                    {
                        match ssi_eval_expr(p, expr_0) {
                            -1 | 0 => {
                                (*p).if_is_false = 1 as libc::c_int;
                                (*p).if_is_false_level = (*p).if_level;
                            }
                            1 => {
                                (*p).if_is_false = 0 as libc::c_int;
                            }
                            _ => {}
                        }
                    } else {
                        (*p).if_is_false = 1 as libc::c_int;
                        (*p).if_is_false_level = (*p).if_level;
                        (*p).if_is_false_endif = 1 as libc::c_int;
                    }
                }
                (*p).if_level += 1;
            }
        }
        11 => {
            (*p).if_level -= 1;
            if (*p).if_level == (*p).if_is_false_level {
                (*p).if_is_false = 0 as libc::c_int;
                (*p).if_is_false_endif = 0 as libc::c_int;
            }
        }
        13 => {}
        _ => {
            log_error(
                (*r).conf.errh,
                b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
                1305 as libc::c_int as libc::c_uint,
                b"ssi: unknown ssi-command: %s\0" as *const u8 as *const libc::c_char,
                *l.offset(1 as libc::c_int as isize),
            );
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mod_ssi_parse_ssi_stmt_value(
    s: *const libc::c_uchar,
    len: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let c: libc::c_int = if *s.offset(0 as libc::c_int as isize) as libc::c_int
        == '"' as i32
    {
        '"' as i32
    } else if *s.offset(0 as libc::c_int as isize) as libc::c_int == '\'' as i32 {
        '\'' as i32
    } else {
        0 as libc::c_int
    };
    if 0 as libc::c_int != c {
        n = 1 as libc::c_int;
        while n < len {
            if *s.offset(n as isize) as libc::c_int == c {
                return n + 1 as libc::c_int;
            }
            if *s.offset(n as isize) as libc::c_int == '\\' as i32 {
                if n + 1 as libc::c_int == len {
                    return 0 as libc::c_int;
                }
                n += 1;
            }
            n += 1;
        }
        return 0 as libc::c_int;
    } else {
        n = 0 as libc::c_int;
        while n < len {
            if *(*__ctype_b_loc()).offset(*s.offset(n as isize) as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                return n;
            }
            if *s.offset(n as isize) as libc::c_int == '\\' as i32 {
                if n + 1 as libc::c_int == len {
                    return 0 as libc::c_int;
                }
                n += 1;
            }
            n += 1;
        }
        return n;
    };
}
unsafe extern "C" fn mod_ssi_parse_ssi_stmt_offlen(
    mut o: *mut libc::c_int,
    s: *const libc::c_uchar,
    len: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = 5 as libc::c_int;
    *o.offset(0 as libc::c_int as isize) = n;
    while light_isalpha(*s.offset(n as isize) as libc::c_int) != 0 {
        n += 1;
    }
    *o.offset(1 as libc::c_int as isize) = n - *o.offset(0 as libc::c_int as isize);
    if 0 as libc::c_int == *o.offset(1 as libc::c_int as isize) {
        return -(1 as libc::c_int);
    }
    if n + 3 as libc::c_int == len {
        return 2 as libc::c_int;
    }
    if *(*__ctype_b_loc()).offset(*s.offset(n as isize) as libc::c_int as isize)
        as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        return -(1 as libc::c_int);
    }
    loop {
        n += 1;
        if !(*(*__ctype_b_loc()).offset(*s.offset(n as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0)
        {
            break;
        }
    }
    if n + 3 as libc::c_int == len {
        return 2 as libc::c_int;
    }
    *o.offset(2 as libc::c_int as isize) = n;
    while light_isalpha(*s.offset(n as isize) as libc::c_int) != 0 {
        n += 1;
    }
    *o.offset(3 as libc::c_int as isize) = n - *o.offset(2 as libc::c_int as isize);
    if 0 as libc::c_int == *o.offset(3 as libc::c_int as isize)
        || {
            let fresh3 = n;
            n = n + 1;
            *s.offset(fresh3 as isize) as libc::c_int != '=' as i32
        }
    {
        return -(1 as libc::c_int);
    }
    *o.offset(4 as libc::c_int as isize) = n;
    *o
        .offset(
            5 as libc::c_int as isize,
        ) = mod_ssi_parse_ssi_stmt_value(
        s.offset(n as isize),
        len - n - 3 as libc::c_int,
    );
    if 0 as libc::c_int == *o.offset(5 as libc::c_int as isize) {
        return -(1 as libc::c_int);
    }
    n += *o.offset(5 as libc::c_int as isize);
    if n + 3 as libc::c_int == len {
        return 6 as libc::c_int;
    }
    if *(*__ctype_b_loc()).offset(*s.offset(n as isize) as libc::c_int as isize)
        as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        return -(1 as libc::c_int);
    }
    loop {
        n += 1;
        if !(*(*__ctype_b_loc()).offset(*s.offset(n as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0)
        {
            break;
        }
    }
    if n + 3 as libc::c_int == len {
        return 6 as libc::c_int;
    }
    *o.offset(6 as libc::c_int as isize) = n;
    while light_isalpha(*s.offset(n as isize) as libc::c_int) != 0 {
        n += 1;
    }
    *o.offset(7 as libc::c_int as isize) = n - *o.offset(6 as libc::c_int as isize);
    if 0 as libc::c_int == *o.offset(7 as libc::c_int as isize)
        || {
            let fresh4 = n;
            n = n + 1;
            *s.offset(fresh4 as isize) as libc::c_int != '=' as i32
        }
    {
        return -(1 as libc::c_int);
    }
    *o.offset(8 as libc::c_int as isize) = n;
    *o
        .offset(
            9 as libc::c_int as isize,
        ) = mod_ssi_parse_ssi_stmt_value(
        s.offset(n as isize),
        len - n - 3 as libc::c_int,
    );
    if 0 as libc::c_int == *o.offset(9 as libc::c_int as isize) {
        return -(1 as libc::c_int);
    }
    n += *o.offset(9 as libc::c_int as isize);
    if n + 3 as libc::c_int == len {
        return 10 as libc::c_int;
    }
    if *(*__ctype_b_loc()).offset(*s.offset(n as isize) as libc::c_int as isize)
        as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        return -(1 as libc::c_int);
    }
    loop {
        n += 1;
        if !(*(*__ctype_b_loc()).offset(*s.offset(n as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0)
        {
            break;
        }
    }
    if n + 3 as libc::c_int == len {
        return 10 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mod_ssi_parse_ssi_stmt(
    r: *mut request_st,
    p: *mut handler_ctx,
    s: *mut libc::c_char,
    mut len: libc::c_int,
    st: *mut stat,
) {
    let mut o: [libc::c_int; 10] = [0; 10];
    let mut m: libc::c_int = 0;
    let n: libc::c_int = mod_ssi_parse_ssi_stmt_offlen(
        o.as_mut_ptr(),
        s as *mut libc::c_uchar,
        len,
    );
    let mut l: [*mut libc::c_char; 6] = [
        s,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
    ];
    if -(1 as libc::c_int) == n {
        if len >= 16 as libc::c_int
            && 0 as libc::c_int
                == memcmp(
                    s.offset(5 as libc::c_int as isize) as *const libc::c_void,
                    b"comment\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
            && (*s.offset(12 as libc::c_int as isize) as libc::c_int == ' ' as i32
                || *s.offset(12 as libc::c_int as isize) as libc::c_int == '\t' as i32)
        {
            return;
        }
        chunkqueue_append_mem(&mut (*p).wq, s, len as size_t);
        return;
    }
    m = 0 as libc::c_int;
    while m < n {
        let mut ptr: *mut libc::c_char = s.offset(o[m as usize] as isize);
        match *ptr as libc::c_int {
            34 | 39 => {
                ptr = ptr.offset(1);
                *ptr
                    .offset(
                        (o[(m + 1 as libc::c_int) as usize] - 2 as libc::c_int) as isize,
                    ) = '\u{0}' as i32 as libc::c_char;
            }
            _ => {
                *ptr
                    .offset(
                        o[(m + 1 as libc::c_int) as usize] as isize,
                    ) = '\u{0}' as i32 as libc::c_char;
            }
        }
        l[(1 as libc::c_int + (m >> 1 as libc::c_int)) as usize] = ptr;
        m == 4 as libc::c_int || m == 8 as libc::c_int;
        m += 2 as libc::c_int;
    }
    process_ssi_stmt(
        r,
        p,
        l.as_mut_ptr() as *mut *const libc::c_char,
        (1 as libc::c_int + (n >> 1 as libc::c_int)) as size_t,
        st,
    );
}
unsafe extern "C" fn mod_ssi_stmt_len(
    mut s: *const libc::c_char,
    len: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut sq: libc::c_int = 0 as libc::c_int;
    let mut dq: libc::c_int = 0 as libc::c_int;
    let mut bs: libc::c_int = 0 as libc::c_int;
    n = 5 as libc::c_int;
    while n < len {
        match *s.offset(n as isize) as libc::c_int {
            45 => {
                if sq == 0 && dq == 0 && (n + 2 as libc::c_int) < len
                    && *s.offset((n + 1 as libc::c_int) as isize) as libc::c_int
                        == '-' as i32
                    && *s.offset((n + 2 as libc::c_int) as isize) as libc::c_int
                        == '>' as i32
                {
                    return n + 3 as libc::c_int;
                }
            }
            34 => {
                if sq == 0 && (dq == 0 || bs == 0) {
                    dq = (dq == 0) as libc::c_int;
                }
            }
            39 => {
                if dq == 0 && (sq == 0 || bs == 0) {
                    sq = (sq == 0) as libc::c_int;
                }
            }
            92 => {
                if sq != 0 || dq != 0 {
                    bs = (bs == 0) as libc::c_int;
                }
            }
            _ => {}
        }
        n += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mod_ssi_read_fd(
    r: *mut request_st,
    p: *mut handler_ctx,
    st: *mut stat,
    mut fd: libc::c_int,
) {
    let mut rd: ssize_t = 0;
    let mut offset: size_t = 0;
    let mut pretag: size_t = 0;
    let b: *mut buffer = chunk_buffer_acquire();
    let cq: *mut chunkqueue = &mut (*p).wq;
    let bufsz: size_t = 8192 as libc::c_int as size_t;
    chunk_buffer_prepare_append(
        b,
        bufsz.wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    let buf: *mut libc::c_char = (*b).ptr;
    offset = 0 as libc::c_int as size_t;
    pretag = 0 as libc::c_int as size_t;
    loop {
        rd = read(
            fd,
            buf.offset(offset as isize) as *mut libc::c_void,
            bufsz.wrapping_sub(offset),
        );
        if !((0 as libc::c_int as libc::c_long) < rd) {
            break;
        }
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut prelen: size_t = 0 as libc::c_int as size_t;
        let mut len: size_t = 0;
        offset = (offset as libc::c_ulong).wrapping_add(rd as size_t) as size_t
            as size_t;
        loop {
            s = memchr(
                buf.offset(prelen as isize) as *const libc::c_void,
                '<' as i32,
                offset.wrapping_sub(prelen),
            ) as *mut libc::c_char;
            if s.is_null() {
                break;
            }
            prelen = s.offset_from(buf) as libc::c_long as size_t;
            if prelen.wrapping_add(5 as libc::c_int as libc::c_ulong) <= offset {
                if !(0 as libc::c_int
                    != memcmp(
                        s.offset(1 as libc::c_int as isize) as *const libc::c_void,
                        b"!--#\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong,
                    ))
                {
                    if prelen.wrapping_sub(pretag) != 0 && (*p).if_is_false == 0 {
                        chunkqueue_append_mem(
                            cq,
                            buf.offset(pretag as isize),
                            prelen.wrapping_sub(pretag),
                        );
                    }
                    len = mod_ssi_stmt_len(
                        buf.offset(prelen as isize),
                        offset.wrapping_sub(prelen) as libc::c_int,
                    ) as size_t;
                    if len != 0 {
                        mod_ssi_parse_ssi_stmt(
                            r,
                            p,
                            buf.offset(prelen as isize),
                            len as libc::c_int,
                            st,
                        );
                        prelen = (prelen as libc::c_ulong)
                            .wrapping_add(
                                len.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            ) as size_t as size_t;
                        pretag = prelen.wrapping_add(1 as libc::c_int as libc::c_ulong);
                        if pretag == offset {
                            pretag = 0 as libc::c_int as size_t;
                            offset = pretag;
                            break;
                        }
                    } else if 0 as libc::c_int as libc::c_ulong == prelen
                            && offset == bufsz
                        {
                        chunkqueue_append_mem(
                            cq,
                            b"<!-- [an error occurred: directive too long] \0"
                                as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        );
                        if *buf
                            .offset(
                                offset.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int == '-' as i32
                            && *buf
                                .offset(
                                    offset.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as isize,
                                ) as libc::c_int == '-' as i32
                        {
                            chunkqueue_append_mem(
                                cq,
                                b"--\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                                    as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                            );
                        } else if *buf
                                .offset(
                                    offset.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as isize,
                                ) as libc::c_int == '-' as i32
                            {
                            chunkqueue_append_mem(
                                cq,
                                b"-\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                    as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                            );
                        }
                        pretag = 0 as libc::c_int as size_t;
                        offset = pretag;
                        break;
                    } else {
                        offset = (offset as libc::c_ulong).wrapping_sub(prelen) as size_t
                            as size_t;
                        memmove(
                            buf as *mut libc::c_void,
                            buf.offset(prelen as isize) as *const libc::c_void,
                            offset,
                        );
                        pretag = 0 as libc::c_int as size_t;
                        break;
                    }
                }
            } else if prelen.wrapping_add(1 as libc::c_int as libc::c_ulong) == offset
                    || 0 as libc::c_int
                        == memcmp(
                            s.offset(1 as libc::c_int as isize) as *const libc::c_void,
                            b"!--\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            offset
                                .wrapping_sub(prelen)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        )
                {
                if prelen.wrapping_sub(pretag) != 0 && (*p).if_is_false == 0 {
                    chunkqueue_append_mem(
                        cq,
                        buf.offset(pretag as isize),
                        prelen.wrapping_sub(pretag),
                    );
                }
                offset = (offset as libc::c_ulong).wrapping_sub(prelen) as size_t
                    as size_t;
                memcpy(
                    buf as *mut libc::c_void,
                    buf.offset(prelen as isize) as *const libc::c_void,
                    offset,
                );
                pretag = 0 as libc::c_int as size_t;
                break;
            }
            prelen = prelen.wrapping_add(1);
        }
        if offset == bufsz {
            if (*p).if_is_false == 0 {
                chunkqueue_append_mem(
                    cq,
                    buf.offset(pretag as isize),
                    offset.wrapping_sub(pretag),
                );
            }
            pretag = 0 as libc::c_int as size_t;
            offset = pretag;
        }
        if !((*cq).last).is_null()
            && (*(*cq).last).type_0 as libc::c_uint
                == MEM_CHUNK as libc::c_int as libc::c_uint
            && buffer_string_space((*(*cq).last).mem)
                < 1023 as libc::c_int as libc::c_uint
        {
            http_chunk_transfer_cqlen(r, cq, chunkqueue_length(cq) as size_t);
        }
    }
    if 0 as libc::c_int as libc::c_long != rd {
        log_perror(
            (*r).conf.errh,
            b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
            1542 as libc::c_int as libc::c_uint,
            b"read(): %s\0" as *const u8 as *const libc::c_char,
            (*r).physical.path.ptr,
        );
    }
    if offset.wrapping_sub(pretag) != 0 {
        if (*p).if_is_false == 0 {
            chunkqueue_append_mem(
                cq,
                buf.offset(pretag as isize),
                offset.wrapping_sub(pretag),
            );
        }
    }
    chunk_buffer_release(b);
    http_chunk_transfer_cqlen(r, cq, chunkqueue_length(cq) as size_t);
}
unsafe extern "C" fn mod_ssi_process_file(
    r: *mut request_st,
    p: *mut handler_ctx,
    st: *mut stat,
) -> libc::c_int {
    let mut fd: libc::c_int = stat_cache_open_rdonly_fstat(
        &mut (*r).physical.path,
        st,
        (*r).conf.follow_symlink as libc::c_int,
    );
    if -(1 as libc::c_int) == fd {
        log_perror(
            (*r).conf.errh,
            b"src/mod_ssi.c\0" as *const u8 as *const libc::c_char,
            1561 as libc::c_int as libc::c_uint,
            b"open(): %s\0" as *const u8 as *const libc::c_char,
            (*r).physical.path.ptr,
        );
        return -(1 as libc::c_int);
    }
    mod_ssi_read_fd(r, p, st, fd);
    close(fd);
    return 0 as libc::c_int;
}
unsafe extern "C" fn mod_ssi_handle_request(
    r: *mut request_st,
    p: *mut handler_ctx,
) -> libc::c_int {
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
    buffer_clear((*p).timefmt);
    array_reset_data_strings((*p).ssi_vars);
    array_reset_data_strings((*p).ssi_cgi_env);
    build_ssi_cgi_vars(r, p);
    ::std::ptr::write_volatile(
        &mut include_file_last_mtime as *mut unix_time64_t,
        0 as libc::c_int as unix_time64_t,
    );
    if mod_ssi_process_file(r, p, &mut st) != 0 {
        return -(1 as libc::c_int);
    }
    (*r).resp_body_started = 1 as libc::c_int as libc::c_char;
    (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
    if ((*p).conf.content_type).is_null() {
        http_header_response_set(
            r,
            HTTP_HEADER_CONTENT_TYPE,
            b"Content-Type\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            b"text/html\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    } else {
        http_header_response_set(
            r,
            HTTP_HEADER_CONTENT_TYPE,
            b"Content-Type\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            (*(*p).conf.content_type).ptr,
            buffer_clen((*p).conf.content_type),
        );
    }
    if (*p).conf.conditional_requests != 0 {
        if st.st_mtim.tv_sec < include_file_last_mtime {
            st.st_mtim.tv_sec = include_file_last_mtime;
        }
        http_etag_create((*r).tmp_buf, &mut st, (*r).conf.etag_flags as libc::c_int);
        http_header_response_set(
            r,
            HTTP_HEADER_ETAG,
            b"ETag\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            (*(*r).tmp_buf).ptr,
            buffer_clen((*r).tmp_buf),
        );
        let mtime: *const buffer = http_response_set_last_modified(r, st.st_mtim.tv_sec);
        if HANDLER_FINISHED as libc::c_int
            == http_response_handle_cachable(r, mtime, st.st_mtim.tv_sec)
        {
            chunkqueue_reset(&mut (*r).write_queue);
        }
    }
    ::std::ptr::write_volatile(
        &mut include_file_last_mtime as *mut unix_time64_t,
        0 as libc::c_int as unix_time64_t,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn mod_ssi_physical_path(
    mut r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    if !((*r).handler_module).is_null() {
        return HANDLER_GO_ON;
    }
    mod_ssi_patch_config(r, p);
    if ((*p).conf.ssi_extension).is_null() {
        return HANDLER_GO_ON;
    }
    if !(array_match_value_suffix((*p).conf.ssi_extension, &mut (*r).physical.path))
        .is_null()
    {
        let ref mut fresh5 = *((*r).plugin_ctx).offset((*p).id as isize);
        *fresh5 = handler_ctx_init(p, (*r).conf.errh) as *mut libc::c_void;
        (*r).handler_module = (*p).self_0;
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_ssi_handle_subrequest(
    mut r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    let mut hctx: *mut handler_ctx = *((*r).plugin_ctx).offset((*p).id as isize)
        as *mut handler_ctx;
    if hctx.is_null() {
        return HANDLER_GO_ON;
    }
    if mod_ssi_handle_request(r, hctx) != 0 {
        (*r).http_status = 500 as libc::c_int;
        (*r).handler_module = 0 as *const plugin;
    }
    return HANDLER_FINISHED;
}
unsafe extern "C" fn mod_ssi_handle_request_reset(
    r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    let mut hctx: *mut handler_ctx = *((*r).plugin_ctx).offset((*p).id as isize)
        as *mut handler_ctx;
    if !hctx.is_null() {
        handler_ctx_free(hctx);
        let ref mut fresh6 = *((*r).plugin_ctx).offset((*p).id as isize);
        *fresh6 = 0 as *mut libc::c_void;
    }
    return HANDLER_GO_ON;
}
#[no_mangle]
pub unsafe extern "C" fn mod_ssi_plugin_init(mut p: *mut plugin) -> libc::c_int {
    (*p).version = 0x10440 as libc::c_int as size_t;
    (*p).name = b"ssi\0" as *const u8 as *const libc::c_char;
    (*p)
        .init = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
    >(Some(mod_ssi_init as unsafe extern "C" fn() -> *mut libc::c_void));
    (*p)
        .handle_subrequest_start = Some(
        mod_ssi_physical_path
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_subrequest = Some(
        mod_ssi_handle_subrequest
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_request_reset = Some(
        mod_ssi_handle_request_reset
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .set_defaults = Some(
        mod_ssi_set_defaults
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    (*p).cleanup = Some(mod_ssi_free as unsafe extern "C" fn(*mut libc::c_void) -> ());
    return 0 as libc::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    cpk = [
        {
            let mut init = config_plugin_keys_t {
                k: b"ssi.extension\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_VLIST as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"ssi.content-type\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"ssi.conditional-requests\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"ssi.exec\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"ssi.recursion-max\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: 0 as *const libc::c_char,
                klen: 0 as libc::c_int as uint8_t,
                ktype: T_CONFIG_UNSET as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_UNSET as libc::c_int as uint8_t,
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
