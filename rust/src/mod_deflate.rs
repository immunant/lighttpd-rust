use ::libc;
extern "C" {
    pub type pcre2_real_match_data_8;
    pub type h2con;
    pub type fdevents;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off64_t, __whence: libc::c_int) -> __off64_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn getpid() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn buffer_string_prepare_copy(b: *mut buffer, size: size_t) -> *mut libc::c_char;
    fn buffer_append_string(b: *mut buffer, s: *const libc::c_char);
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_int(b: *mut buffer, val: intmax_t);
    fn li_itostrn(buf: *mut libc::c_char, buf_len: size_t, val: intmax_t) -> size_t;
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
    fn array_match_value_prefix(a: *const array, b: *const buffer) -> *const buffer;
    fn chunkqueue_init(cq: *mut chunkqueue) -> *mut chunkqueue;
    fn chunkqueue_append_chunkqueue(cq: *mut chunkqueue, src: *mut chunkqueue);
    fn chunkqueue_mark_written(cq: *mut chunkqueue, len: off_t);
    fn chunkqueue_remove_finished_chunks(cq: *mut chunkqueue);
    fn chunkqueue_reset(cq: *mut chunkqueue);
    fn buffer_append_str2(
        b: *mut buffer,
        s1: *const libc::c_char,
        len1: size_t,
        s2: *const libc::c_char,
        len2: size_t,
    );
    fn fdevent_open_cloexec(
        pathname: *const libc::c_char,
        symlinks: libc::c_int,
        flags: libc::c_int,
        mode: mode_t,
    ) -> libc::c_int;
    fn fdevent_rename(
        oldpath: *const libc::c_char,
        newpath: *const libc::c_char,
    ) -> libc::c_int;
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
    fn http_chunk_append_file_fd(
        r: *mut request_st,
        fn_0: *const buffer,
        fd: libc::c_int,
        sz: off_t,
    ) -> libc::c_int;
    fn http_chunk_append_file_ref(
        r: *mut request_st,
        sce: *mut stat_cache_entry,
    ) -> libc::c_int;
    fn http_header_str_contains_token(
        s: *const libc::c_char,
        slen: uint32_t,
        m: *const libc::c_char,
        mlen: uint32_t,
    ) -> libc::c_int;
    fn http_header_response_get(
        r: *const request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn http_header_response_unset(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    );
    fn http_header_response_set(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
        v: *const libc::c_char,
        vlen: uint32_t,
    );
    fn http_header_response_append(
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
    fn http_header_env_set_ptr(
        r: *mut request_st,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn http_response_body_clear(r: *mut request_st, preserve_length: libc::c_int);
    fn stat_cache_get_entry(name: *const buffer) -> *mut stat_cache_entry;
    fn stat_cache_get_entry_open(
        name: *const buffer,
        symlinks: libc::c_int,
    ) -> *mut stat_cache_entry;
    fn config_plugin_values_init(
        srv: *mut server,
        p_d: *mut libc::c_void,
        cpk: *const config_plugin_keys_t,
        mname: *const libc::c_char,
    ) -> libc::c_int;
    fn config_check_cond(r: *mut request_st, context_ndx: libc::c_int) -> libc::c_int;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
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
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
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
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intmax_t = __intmax_t;
pub type unix_time64_t = time_t;
pub type unix_timespec64_t = timespec;
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
pub struct server {
    pub plugin_slots: *mut libc::c_void,
    pub config_context: *mut array,
    pub config_captures: libc::c_int,
    pub ev: *mut fdevents,
    pub network_backend_write: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *mut chunkqueue,
            off_t,
            *mut log_error_st,
        ) -> libc::c_int,
    >,
    pub request_env: Option::<unsafe extern "C" fn(*mut request_st) -> handler_t>,
    pub tmp_buf: *mut buffer,
    pub con_opened: libc::c_int,
    pub con_read: libc::c_int,
    pub con_written: libc::c_int,
    pub con_closed: libc::c_int,
    pub max_fds: libc::c_int,
    pub max_fds_lowat: libc::c_int,
    pub max_fds_hiwat: libc::c_int,
    pub cur_fds: libc::c_int,
    pub sockets_disabled: libc::c_int,
    pub lim_conns: uint32_t,
    pub conns: *mut connection,
    pub conns_pool: *mut connection,
    pub errh: *mut log_error_st,
    pub loadts: unix_time64_t,
    pub loadavg: [libc::c_double; 3],
    pub srvconf: server_config,
    pub config_data_base: *mut libc::c_void,
    pub srv_sockets: server_socket_array,
    pub srv_sockets_inherited: server_socket_array,
    pub plugins: C2RustUnnamed,
    pub startup_ts: unix_time64_t,
    pub graceful_expire_ts: unix_time64_t,
    pub uid: uid_t,
    pub gid: gid_t,
    pub pid: pid_t,
    pub stdin_fd: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub match_data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub ptr: *mut libc::c_void,
    pub used: uint32_t,
    pub size: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server_socket_array {
    pub ptr: *mut *mut server_socket,
    pub size: uint32_t,
    pub used: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server_socket {
    pub addr: sock_addr,
    pub fd: libc::c_int,
    pub is_ssl: uint8_t,
    pub srv_token_colon: uint8_t,
    pub sidx: libc::c_ushort,
    pub fdn: *mut fdnode,
    pub srv: *mut server,
    pub srv_token: *mut buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub ptr: *mut libc::c_char,
    pub used: uint32_t,
    pub size: uint32_t,
}
pub type fdnode = fdnode_st;
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
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
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
pub type in_port_t = uint16_t;
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
    pub __in6_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server_config {
    pub max_request_field_size: uint32_t,
    pub log_request_header_on_error: libc::c_uchar,
    pub http_header_strict: libc::c_uchar,
    pub http_host_strict: libc::c_uchar,
    pub http_host_normalize: libc::c_uchar,
    pub http_method_get_body: libc::c_uchar,
    pub high_precision_timestamps: libc::c_uchar,
    pub h2proto: libc::c_uchar,
    pub absolute_dir_redirect: libc::c_uchar,
    pub http_url_normalize: libc::c_ushort,
    pub max_worker: libc::c_ushort,
    pub max_fds: libc::c_ushort,
    pub max_conns: libc::c_ushort,
    pub port: libc::c_ushort,
    pub upload_temp_file_size: libc::c_uint,
    pub upload_tempdirs: *mut array,
    pub dont_daemonize: libc::c_uchar,
    pub preflight_check: libc::c_uchar,
    pub enable_cores: libc::c_uchar,
    pub compat_module_load: libc::c_uchar,
    pub config_deprecated: libc::c_uchar,
    pub config_unsupported: libc::c_uchar,
    pub systemd_socket_activation: libc::c_uchar,
    pub errorlog_use_syslog: libc::c_uchar,
    pub syslog_facility: *const buffer,
    pub bindhost: *const buffer,
    pub changeroot: *const buffer,
    pub username: *const buffer,
    pub groupname: *const buffer,
    pub network_backend: *const buffer,
    pub feature_flags: *const array,
    pub event_handler: *const libc::c_char,
    pub modules_dir: *const libc::c_char,
    pub pid_file: *mut buffer,
    pub modules: *mut array,
    pub config_touched: *mut array,
    pub empty_array: array,
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
pub type log_error_st = fdlog_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fdlog_st {
    pub mode: C2RustUnnamed_1,
    pub fd: libc::c_int,
    pub b: buffer,
    pub fn_0: *const libc::c_char,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const FDLOG_PIPE: C2RustUnnamed_1 = 3;
pub const FDLOG_SYSLOG: C2RustUnnamed_1 = 2;
pub const FDLOG_FD: C2RustUnnamed_1 = 1;
pub const FDLOG_FILE: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct connection {
    pub request: request_st,
    pub h2: *mut h2con,
    pub fd: libc::c_int,
    pub fdn: *mut fdnode,
    pub jqnext: *mut connection,
    pub is_readable: libc::c_schar,
    pub is_writable: libc::c_schar,
    pub is_ssl_sock: libc::c_char,
    pub traffic_limit_reached: libc::c_char,
    pub revents_err: uint16_t,
    pub proto_default_port: uint16_t,
    pub write_queue: *mut chunkqueue,
    pub read_queue: *mut chunkqueue,
    pub bytes_written: off_t,
    pub bytes_written_cur_second: off_t,
    pub bytes_read: off_t,
    pub network_write: Option::<
        unsafe extern "C" fn(*mut connection, *mut chunkqueue, off_t) -> libc::c_int,
    >,
    pub network_read: Option::<
        unsafe extern "C" fn(*mut connection, *mut chunkqueue, off_t) -> libc::c_int,
    >,
    pub reqbody_read: Option::<unsafe extern "C" fn(*mut request_st) -> handler_t>,
    pub srv: *mut server,
    pub plugin_slots: *mut libc::c_void,
    pub plugin_ctx: *mut *mut libc::c_void,
    pub config_data_base: *mut libc::c_void,
    pub dst_addr: sock_addr,
    pub dst_addr_buf: buffer,
    pub srv_socket: *const server_socket,
    pub read_idle_ts: unix_time64_t,
    pub close_timeout_ts: unix_time64_t,
    pub write_request_ts: unix_time64_t,
    pub connection_start: unix_time64_t,
    pub request_count: uint32_t,
    pub keep_alive_idle: libc::c_int,
    pub next: *mut connection,
    pub prev: *mut connection,
}
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
pub struct chunk {
    pub next: *mut chunk,
    pub type_0: C2RustUnnamed_4,
    pub mem: *mut buffer,
    pub offset: off_t,
    pub file: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub length: off_t,
    pub fd: libc::c_int,
    pub is_temp: libc::c_int,
    pub mmap: C2RustUnnamed_3,
    pub ref_0: *mut libc::c_void,
    pub refchg: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub start: *mut libc::c_char,
    pub length: size_t,
    pub offset: off_t,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const FILE_CHUNK: C2RustUnnamed_4 = 1;
pub const MEM_CHUNK: C2RustUnnamed_4 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_string {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
    pub ext: libc::c_int,
    pub value: buffer,
}
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
pub type C2RustUnnamed_5 = libc::c_uint;
pub const T_CONFIG_SCOPE_CONNECTION: C2RustUnnamed_5 = 2;
pub const T_CONFIG_SCOPE_SERVER: C2RustUnnamed_5 = 1;
pub const T_CONFIG_SCOPE_UNSET: C2RustUnnamed_5 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct encparms {
    pub gzip: C2RustUnnamed_9,
    pub brotli: C2RustUnnamed_8,
    pub zstd: C2RustUnnamed_7,
    pub bzip2: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub clevel: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub clevel: libc::c_int,
    pub strategy: libc::c_int,
    pub windowLog: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub quality: uint32_t,
    pub window: uint32_t,
    pub mode: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub clevel: libc::c_int,
    pub windowBits: libc::c_int,
    pub memLevel: libc::c_int,
    pub strategy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_config {
    pub mimetypes: *const array,
    pub cache_dir: *const buffer,
    pub max_compress_size: libc::c_uint,
    pub min_compress_size: libc::c_ushort,
    pub output_buffer_size: libc::c_ushort,
    pub work_block_size: libc::c_ushort,
    pub sync_flush: libc::c_ushort,
    pub compression_level: libc::c_short,
    pub allowed_encodings: *mut uint16_t,
    pub max_loadavg: libc::c_double,
    pub params: *const encparms,
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
    pub tmp_buf: buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct handler_ctx {
    pub u: C2RustUnnamed_10,
    pub bytes_in: off_t,
    pub bytes_out: off_t,
    pub output: *mut buffer,
    pub plugin_data: *mut plugin_data,
    pub r: *mut request_st,
    pub compression_type: libc::c_int,
    pub cache_fd: libc::c_int,
    pub cache_fn: *mut libc::c_char,
    pub in_queue: chunkqueue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub dummy: libc::c_int,
}
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
unsafe extern "C" fn chunkqueue_length(mut cq: *const chunkqueue) -> off_t {
    return (*cq).bytes_in - (*cq).bytes_out;
}
#[inline]
unsafe extern "C" fn chunkqueue_is_empty(mut cq: *const chunkqueue) -> libc::c_int {
    return (0 as *mut libc::c_void as *mut chunk == (*cq).first) as libc::c_int;
}
unsafe extern "C" fn handler_ctx_init() -> *mut handler_ctx {
    let mut hctx: *mut handler_ctx = 0 as *mut handler_ctx;
    hctx = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<handler_ctx>() as libc::c_ulong,
    ) as *mut handler_ctx;
    chunkqueue_init(&mut (*hctx).in_queue);
    (*hctx).cache_fd = -(1 as libc::c_int);
    return hctx;
}
unsafe extern "C" fn handler_ctx_free(mut hctx: *mut handler_ctx) {
    if !((*hctx).cache_fn).is_null() {
        unlink((*hctx).cache_fn);
        free((*hctx).cache_fn as *mut libc::c_void);
    }
    if -(1 as libc::c_int) != (*hctx).cache_fd {
        close((*hctx).cache_fd);
    }
    chunkqueue_reset(&mut (*hctx).in_queue);
    free(hctx as *mut libc::c_void);
}
#[cold]
unsafe extern "C" fn mod_deflate_init() -> *mut libc::c_void {
    let p: *mut plugin_data = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<plugin_data>() as libc::c_ulong,
    ) as *mut plugin_data;
    buffer_string_prepare_copy(&mut (*p).tmp_buf, 65536 as libc::c_int as size_t);
    return p as *mut libc::c_void;
}
#[cold]
unsafe extern "C" fn mod_deflate_free(mut p_d: *mut libc::c_void) {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    free((*p).tmp_buf.ptr as *mut libc::c_void);
    if ((*p).cvlist).is_null() {
        return;
    }
    let mut i: libc::c_int = ((*((*p).cvlist).offset(0 as libc::c_int as isize))
        .v
        .u2[1 as libc::c_int as usize] == 0) as libc::c_int;
    let mut used: libc::c_int = (*p).nconfig;
    while i < used {
        let mut cpv: *mut config_plugin_value_t = ((*p).cvlist)
            .offset(
                (*((*p).cvlist).offset(i as isize)).v.u2[0 as libc::c_int as usize]
                    as isize,
            );
        while -(1 as libc::c_int) != (*cpv).k_id {
            if !((*cpv).vtype as libc::c_uint
                != T_CONFIG_LOCAL as libc::c_int as libc::c_uint
                || ((*cpv).v.v).is_null())
            {
                match (*cpv).k_id {
                    1 | 14 => {
                        free((*cpv).v.v);
                    }
                    _ => {}
                }
            }
            cpv = cpv.offset(1);
        }
        i += 1;
    }
}
unsafe extern "C" fn mkdir_for_file(mut fn_0: *mut libc::c_char) -> libc::c_int {
    let mut p: *mut libc::c_char = fn_0;
    loop {
        p = strchr(p.offset(1 as libc::c_int as isize), '/' as i32);
        if p.is_null() {
            break;
        }
        if *p.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32 {
            return 0 as libc::c_int;
        }
        *p = '\u{0}' as i32 as libc::c_char;
        let mut rc: libc::c_int = mkdir(fn_0, 0o700 as libc::c_int as __mode_t);
        *p = '/' as i32 as libc::c_char;
        if 0 as libc::c_int != rc && *__errno_location() != 17 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mkdir_recursive(mut dir: *mut libc::c_char) -> libc::c_int {
    return if 0 as libc::c_int == mkdir_for_file(dir)
        && (0 as libc::c_int == mkdir(dir, 0o700 as libc::c_int as __mode_t)
            || *__errno_location() == 17 as libc::c_int)
    {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
unsafe extern "C" fn mod_deflate_cache_file_name(
    r: *mut request_st,
    mut cache_dir: *const buffer,
    etag: *const buffer,
) -> *mut buffer {
    let tb: *mut buffer = (*r).tmp_buf;
    buffer_copy_path_len2(
        tb,
        (*cache_dir).ptr,
        buffer_clen(cache_dir) as size_t,
        (*r).physical.path.ptr,
        buffer_clen(&mut (*r).physical.path) as size_t,
    );
    buffer_append_str2(
        tb,
        b"-\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ((*etag).ptr).offset(1 as libc::c_int as isize),
        (buffer_clen(etag)).wrapping_sub(2 as libc::c_int as libc::c_uint) as size_t,
    );
    return tb;
}
unsafe extern "C" fn mod_deflate_cache_file_open(
    hctx: *mut handler_ctx,
    fn_0: *const buffer,
) {
    let fnlen: uint32_t = buffer_clen(fn_0);
    (*hctx)
        .cache_fn = malloc(
        (fnlen.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
            .wrapping_add(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (8 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<intmax_t>() as libc::c_ulong,
                            )
                            .wrapping_mul(31 as libc::c_int as libc::c_ulong)
                            .wrapping_add(99 as libc::c_int as libc::c_ulong)
                            .wrapping_div(100 as libc::c_int as libc::c_ulong),
                    ),
            )
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if ((*hctx).cache_fn).is_null() {
        ck_assert_failed(
            b"src/mod_deflate.c\0" as *const u8 as *const libc::c_char,
            350 as libc::c_int as libc::c_uint,
            b"hctx->cache_fn\0" as *const u8 as *const libc::c_char,
        );
    }
    memcpy(
        (*hctx).cache_fn as *mut libc::c_void,
        (*fn_0).ptr as *const libc::c_void,
        fnlen as libc::c_ulong,
    );
    *((*hctx).cache_fn).offset(fnlen as isize) = '.' as i32 as libc::c_char;
    let ilen: size_t = li_itostrn(
        ((*hctx).cache_fn).offset(fnlen as isize).offset(1 as libc::c_int as isize),
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                (8 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<intmax_t>() as libc::c_ulong)
                    .wrapping_mul(31 as libc::c_int as libc::c_ulong)
                    .wrapping_add(99 as libc::c_int as libc::c_ulong)
                    .wrapping_div(100 as libc::c_int as libc::c_ulong),
            ),
        getpid() as intmax_t,
    );
    *((*hctx).cache_fn)
        .offset(
            (fnlen.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
                .wrapping_add(ilen) as isize,
        ) = '\u{0}' as i32 as libc::c_char;
    (*hctx)
        .cache_fd = fdevent_open_cloexec(
        (*hctx).cache_fn,
        1 as libc::c_int,
        0o2 as libc::c_int | 0o100 as libc::c_int,
        0o600 as libc::c_int as mode_t,
    );
    if -(1 as libc::c_int) == (*hctx).cache_fd {
        free((*hctx).cache_fn as *mut libc::c_void);
        (*hctx).cache_fn = 0 as *mut libc::c_char;
    }
}
unsafe extern "C" fn mod_deflate_cache_file_finish(
    r: *mut request_st,
    hctx: *mut handler_ctx,
    fn_0: *const buffer,
) -> libc::c_int {
    if 0 as libc::c_int != fdevent_rename((*hctx).cache_fn, (*fn_0).ptr) {
        return -(1 as libc::c_int);
    }
    free((*hctx).cache_fn as *mut libc::c_void);
    (*hctx).cache_fn = 0 as *mut libc::c_char;
    chunkqueue_reset(&mut (*r).write_queue);
    let mut rc: libc::c_int = http_chunk_append_file_fd(
        r,
        fn_0,
        (*hctx).cache_fd,
        (*hctx).bytes_out,
    );
    (*hctx).cache_fd = -(1 as libc::c_int);
    return rc;
}
unsafe extern "C" fn mod_deflate_merge_config_cpv(
    pconf: *mut plugin_config,
    cpv: *const config_plugin_value_t,
) {
    match (*cpv).k_id {
        0 => {
            (*pconf).mimetypes = (*cpv).v.a;
        }
        1 => {
            if (*cpv).vtype as libc::c_uint
                == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
            {
                (*pconf).allowed_encodings = (*cpv).v.v as *mut uint16_t;
            }
        }
        2 => {
            (*pconf).max_compress_size = (*cpv).v.u;
        }
        3 => {
            (*pconf).min_compress_size = (*cpv).v.shrt;
        }
        4 => {
            (*pconf).compression_level = (*cpv).v.shrt as libc::c_short;
        }
        5 => {
            (*pconf).output_buffer_size = (*cpv).v.shrt;
        }
        6 => {
            (*pconf).work_block_size = (*cpv).v.shrt;
        }
        7 => {
            (*pconf).max_loadavg = (*cpv).v.d;
        }
        8 => {
            (*pconf).cache_dir = (*cpv).v.b;
        }
        14 => {
            if (*cpv).vtype as libc::c_uint
                == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
            {
                (*pconf).params = (*cpv).v.v as *const encparms;
            }
        }
        _ => return,
    };
}
unsafe extern "C" fn mod_deflate_merge_config(
    pconf: *mut plugin_config,
    mut cpv: *const config_plugin_value_t,
) {
    loop {
        mod_deflate_merge_config_cpv(pconf, cpv);
        cpv = cpv.offset(1);
        if !((*cpv).k_id != -(1 as libc::c_int)) {
            break;
        }
    };
}
unsafe extern "C" fn mod_deflate_patch_config(r: *mut request_st, p: *mut plugin_data) {
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
            mod_deflate_merge_config(
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
unsafe extern "C" fn mod_deflate_parse_params(
    a: *const array,
    errh: *mut log_error_st,
) -> *mut encparms {
    let mut params: *mut encparms = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<encparms>() as libc::c_ulong,
    ) as *mut encparms;
    if params.is_null() {
        ck_assert_failed(
            b"src/mod_deflate.c\0" as *const u8 as *const libc::c_char,
            447 as libc::c_int as libc::c_uint,
            b"params\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*a).used {
        let du: *const data_unset = *((*a).data).offset(i as isize);
        log_error(
            errh,
            b"src/mod_deflate.c\0" as *const u8 as *const libc::c_char,
            617 as libc::c_int as libc::c_uint,
            b"unrecognized param: %s\0" as *const u8 as *const libc::c_char,
            (*du).key.ptr,
        );
        i = i.wrapping_add(1);
    }
    return params;
}
unsafe extern "C" fn mod_deflate_encodings_to_flags(
    mut encodings: *const array,
) -> *mut uint16_t {
    if (*encodings).used != 0 {
        let x: *mut uint16_t = calloc(
            ((*encodings).used).wrapping_add(1 as libc::c_int as libc::c_uint)
                as libc::c_ulong,
            ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
        ) as *mut uint16_t;
        if x.is_null() {
            ck_assert_failed(
                b"src/mod_deflate.c\0" as *const u8 as *const libc::c_char,
                627 as libc::c_int as libc::c_uint,
                b"x\0" as *const u8 as *const libc::c_char,
            );
        }
        let mut i: libc::c_int = 0 as libc::c_int;
        let mut j: uint32_t = 0 as libc::c_int as uint32_t;
        while j < (*encodings).used {
            j = j.wrapping_add(1);
        }
        *x.offset(i as isize) = 0 as libc::c_int as uint16_t;
        return x;
    } else {
        let x_0: *mut uint16_t = calloc(
            (4 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
            ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
        ) as *mut uint16_t;
        if x_0.is_null() {
            ck_assert_failed(
                b"src/mod_deflate.c\0" as *const u8 as *const libc::c_char,
                665 as libc::c_int as libc::c_uint,
                b"x\0" as *const u8 as *const libc::c_char,
            );
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        *x_0.offset(i_0 as isize) = 0 as libc::c_int as uint16_t;
        return x_0;
    };
}
static mut cpk: [config_plugin_keys_t; 16] = [config_plugin_keys_t {
    k: 0 as *const libc::c_char,
    klen: 0,
    ktype: 0,
    scope: 0,
}; 16];
#[cold]
unsafe extern "C" fn mod_deflate_set_defaults(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    if config_plugin_values_init(
        srv,
        p as *mut libc::c_void,
        cpk.as_ptr(),
        b"mod_deflate\0" as *const u8 as *const libc::c_char,
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
            let mut current_block_41: u64;
            match (*cpv).k_id {
                9 => {
                    log_error(
                        (*srv).errh,
                        b"src/mod_deflate.c\0" as *const u8 as *const libc::c_char,
                        750 as libc::c_int as libc::c_uint,
                        b"DEPRECATED: %s replaced with deflate.mimetypes\0" as *const u8
                            as *const libc::c_char,
                        cpk[(*cpv).k_id as usize].k,
                    );
                    (*cpv).k_id = 0 as libc::c_int;
                    current_block_41 = 4884931126302699652;
                }
                0 => {
                    current_block_41 = 4884931126302699652;
                }
                10 => {
                    log_error(
                        (*srv).errh,
                        b"src/mod_deflate.c\0" as *const u8 as *const libc::c_char,
                        768 as libc::c_int as libc::c_uint,
                        b"DEPRECATED: %s replaced with deflate.allowed-encodings\0"
                            as *const u8 as *const libc::c_char,
                        cpk[(*cpv).k_id as usize].k,
                    );
                    (*cpv).k_id = 1 as libc::c_int;
                    current_block_41 = 2153718138503504757;
                }
                1 => {
                    current_block_41 = 2153718138503504757;
                }
                12 => {
                    log_error(
                        (*srv).errh,
                        b"src/mod_deflate.c\0" as *const u8 as *const libc::c_char,
                        778 as libc::c_int as libc::c_uint,
                        b"DEPRECATED: %s replaced with deflate.max-compress-size\0"
                            as *const u8 as *const libc::c_char,
                        cpk[(*cpv).k_id as usize].k,
                    );
                    (*cpv).k_id = 2 as libc::c_int;
                    current_block_41 = 1239375570390974118;
                }
                2 => {
                    current_block_41 = 1239375570390974118;
                }
                3 => {
                    current_block_41 = 4180370549828840911;
                }
                4 => {
                    if (((*cpv).v.shrt as libc::c_int) < 1 as libc::c_int
                        || (*cpv).v.shrt as libc::c_int > 9 as libc::c_int)
                        && *(&mut (*cpv).v.shrt as *mut libc::c_ushort
                            as *mut libc::c_short) as libc::c_int != -(1 as libc::c_int)
                    {
                        log_error(
                            (*srv).errh,
                            b"src/mod_deflate.c\0" as *const u8 as *const libc::c_char,
                            789 as libc::c_int as libc::c_uint,
                            b"compression-level must be between 1 and 9: %hu\0"
                                as *const u8 as *const libc::c_char,
                            (*cpv).v.shrt as libc::c_int,
                        );
                        return HANDLER_ERROR;
                    }
                    current_block_41 = 13281731871476506071;
                }
                5 => {
                    current_block_41 = 1273590616204497650;
                }
                6 => {
                    current_block_41 = 1273590616204497650;
                }
                13 => {
                    log_error(
                        (*srv).errh,
                        b"src/mod_deflate.c\0" as *const u8 as *const libc::c_char,
                        799 as libc::c_int as libc::c_uint,
                        b"DEPRECATED: %s replaced with deflate.max-loadavg\0"
                            as *const u8 as *const libc::c_char,
                        cpk[(*cpv).k_id as usize].k,
                    );
                    (*cpv).k_id = 7 as libc::c_int;
                    current_block_41 = 15340494680459663889;
                }
                7 => {
                    current_block_41 = 15340494680459663889;
                }
                11 => {
                    log_error(
                        (*srv).errh,
                        b"src/mod_deflate.c\0" as *const u8 as *const libc::c_char,
                        810 as libc::c_int as libc::c_uint,
                        b"DEPRECATED: %s replaced with deflate.cache-dir\0" as *const u8
                            as *const libc::c_char,
                        cpk[(*cpv).k_id as usize].k,
                    );
                    (*cpv).k_id = 8 as libc::c_int;
                    current_block_41 = 2571479547849027551;
                }
                8 => {
                    current_block_41 = 2571479547849027551;
                }
                14 => {
                    (*cpv)
                        .v
                        .v = mod_deflate_parse_params((*cpv).v.a, (*srv).errh)
                        as *mut libc::c_void;
                    (*cpv).vtype = T_CONFIG_LOCAL;
                    current_block_41 = 13281731871476506071;
                }
                _ => {
                    current_block_41 = 13281731871476506071;
                }
            }
            match current_block_41 {
                2153718138503504757 => {
                    (*cpv)
                        .v
                        .v = mod_deflate_encodings_to_flags((*cpv).v.a)
                        as *mut libc::c_void;
                    (*cpv).vtype = T_CONFIG_LOCAL;
                    current_block_41 = 13281731871476506071;
                }
                4884931126302699652 => {
                    let mut m: uint32_t = 0 as libc::c_int as uint32_t;
                    while m < (*(*cpv).v.a).used {
                        let mut mimetype: *mut buffer = &mut (*(*((*(*cpv).v.a).data)
                            .offset(m as isize) as *mut data_string))
                            .value;
                        let mut len: size_t = buffer_clen(mimetype) as size_t;
                        if len > 2 as libc::c_int as libc::c_ulong
                            && *((*mimetype).ptr)
                                .offset(
                                    len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                ) as libc::c_int == '*' as i32
                        {
                            buffer_truncate(
                                mimetype,
                                len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    as uint32_t,
                            );
                        }
                        m = m.wrapping_add(1);
                    }
                    if 0 as libc::c_int as libc::c_uint == (*(*cpv).v.a).used {
                        (*cpv).v.a = 0 as *const array;
                    }
                    current_block_41 = 13281731871476506071;
                }
                1239375570390974118 => {
                    current_block_41 = 4180370549828840911;
                }
                1273590616204497650 => {
                    current_block_41 = 13281731871476506071;
                }
                15340494680459663889 => {
                    (*cpv)
                        .v
                        .d = if buffer_is_blank((*cpv).v.b) == 0 {
                        strtod((*(*cpv).v.b).ptr, 0 as *mut *mut libc::c_char)
                    } else {
                        0.0f64
                    };
                    current_block_41 = 13281731871476506071;
                }
                2571479547849027551 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        let mut b: *mut buffer = 0 as *mut buffer;
                        let ref mut fresh0 = *(&mut b as *mut *mut buffer
                            as *mut *const buffer);
                        *fresh0 = (*cpv).v.b;
                        let len_0: uint32_t = buffer_clen(b);
                        if len_0 > 0 as libc::c_int as libc::c_uint
                            && '/' as i32
                                == *((*b).ptr)
                                    .offset(
                                        len_0.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                            as isize,
                                    ) as libc::c_int
                        {
                            buffer_truncate(
                                b,
                                len_0.wrapping_sub(1 as libc::c_int as libc::c_uint),
                            );
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
                        if 0 as libc::c_int != stat((*b).ptr, &mut st)
                            && 0 as libc::c_int != mkdir_recursive((*b).ptr)
                        {
                            log_perror(
                                (*srv).errh,
                                b"src/mod_deflate.c\0" as *const u8 as *const libc::c_char,
                                824 as libc::c_int as libc::c_uint,
                                b"can't stat %s %s\0" as *const u8 as *const libc::c_char,
                                cpk[(*cpv).k_id as usize].k,
                                (*b).ptr,
                            );
                            return HANDLER_ERROR;
                        }
                    } else {
                        (*cpv).v.b = 0 as *const buffer;
                    }
                    current_block_41 = 13281731871476506071;
                }
                _ => {}
            }
            match current_block_41 {
                4180370549828840911 => {}
                _ => {}
            }
            cpv = cpv.offset(1);
        }
        i += 1;
    }
    (*p)
        .defaults
        .max_compress_size = (128 as libc::c_int * 1024 as libc::c_int) as libc::c_uint;
    (*p).defaults.min_compress_size = 256 as libc::c_int as libc::c_ushort;
    (*p).defaults.compression_level = -(1 as libc::c_int) as libc::c_short;
    (*p).defaults.output_buffer_size = 0 as libc::c_int as libc::c_ushort;
    (*p).defaults.work_block_size = 2048 as libc::c_int as libc::c_ushort;
    (*p).defaults.max_loadavg = 0.0f64;
    (*p).defaults.sync_flush = 0 as libc::c_int as libc::c_ushort;
    if (*p).nconfig > 0 as libc::c_int
        && (*(*p).cvlist).v.u2[1 as libc::c_int as usize] != 0
    {
        let mut cpv_0: *const config_plugin_value_t = ((*p).cvlist)
            .offset((*(*p).cvlist).v.u2[0 as libc::c_int as usize] as isize);
        if -(1 as libc::c_int) != (*cpv_0).k_id {
            mod_deflate_merge_config(&mut (*p).defaults, cpv_0);
        }
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_deflate_stream_init(mut hctx: *mut handler_ctx) -> libc::c_int {
    match (*hctx).compression_type {
        _ => {}
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mod_deflate_compress(
    hctx: *mut handler_ctx,
    start: *mut libc::c_uchar,
    mut st_size: off_t,
) -> libc::c_int {
    if 0 as libc::c_int as libc::c_long == st_size {
        return 0 as libc::c_int;
    }
    match (*hctx).compression_type {
        _ => {}
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mod_deflate_stream_flush(
    hctx: *mut handler_ctx,
    mut end: libc::c_int,
) -> libc::c_int {
    if 0 as libc::c_int as libc::c_long == (*hctx).bytes_in {
        return 0 as libc::c_int;
    }
    match (*hctx).compression_type {
        _ => {}
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mod_deflate_note_ratio(
    r: *mut request_st,
    bytes_out: off_t,
    bytes_in: off_t,
) {
    if 0 as libc::c_int as libc::c_long == bytes_in {
        return;
    }
    buffer_append_int(
        http_header_env_set_ptr(
            r,
            b"ratio\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        ),
        bytes_out * 100 as libc::c_int as libc::c_long / bytes_in,
    );
}
unsafe extern "C" fn mod_deflate_stream_end(mut hctx: *mut handler_ctx) -> libc::c_int {
    match (*hctx).compression_type {
        _ => {}
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn deflate_compress_cleanup(
    r: *mut request_st,
    hctx: *mut handler_ctx,
) -> libc::c_int {
    let mut rc: libc::c_int = mod_deflate_stream_end(hctx);
    if 0 as libc::c_int == rc && (*hctx).bytes_in < (*hctx).bytes_out {
        log_error(
            (*r).conf.errh,
            b"src/mod_deflate.c\0" as *const u8 as *const libc::c_char,
            1426 as libc::c_int as libc::c_uint,
            b"uri %s in=%lld smaller than out=%lld\0" as *const u8
                as *const libc::c_char,
            (*r).target.ptr,
            (*hctx).bytes_in as libc::c_longlong,
            (*hctx).bytes_out as libc::c_longlong,
        );
    }
    handler_ctx_free(hctx);
    return rc;
}
unsafe extern "C" fn mod_deflate_file_chunk(
    r: *mut request_st,
    hctx: *mut handler_ctx,
    c: *mut chunk,
    mut st_size: off_t,
) -> libc::c_int {
    let mut abs_offset: off_t = 0;
    let mut toSend: off_t = -(1 as libc::c_int) as off_t;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    start = 0 as *mut libc::c_char;
    if -(1 as libc::c_int) == (*c).file.fd {
        (*c)
            .file
            .fd = fdevent_open_cloexec(
            (*(*c).mem).ptr,
            (*r).conf.follow_symlink as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int as mode_t,
        );
        if -(1 as libc::c_int) == (*c).file.fd {
            log_perror(
                (*r).conf.errh,
                b"src/mod_deflate.c\0" as *const u8 as *const libc::c_char,
                1452 as libc::c_int as libc::c_uint,
                b"open failed %s\0" as *const u8 as *const libc::c_char,
                (*(*c).mem).ptr,
            );
            return -(1 as libc::c_int);
        }
    }
    abs_offset = (*c).offset;
    if -(1 as libc::c_int) as *mut libc::c_void
        == (*c).file.mmap.start as *mut libc::c_void
    {
        toSend = st_size;
        if toSend
            > (2 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int)
                as libc::c_long
        {
            toSend = (2 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int)
                as off_t;
        }
        start = malloc(toSend as size_t) as *mut libc::c_char;
        if start.is_null()
            || -(1 as libc::c_int) as libc::c_long
                == lseek((*c).file.fd, abs_offset, 0 as libc::c_int)
            || toSend != read((*c).file.fd, start as *mut libc::c_void, toSend as size_t)
        {
            log_perror(
                (*r).conf.errh,
                b"src/mod_deflate.c\0" as *const u8 as *const libc::c_char,
                1549 as libc::c_int as libc::c_uint,
                b"reading %s failed\0" as *const u8 as *const libc::c_char,
                (*(*c).mem).ptr,
            );
            free(start as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
        abs_offset = 0 as libc::c_int as off_t;
    }
    if mod_deflate_compress(
        hctx,
        (start as *mut libc::c_uchar)
            .offset((abs_offset - (*c).file.mmap.offset) as isize),
        toSend,
    ) < 0 as libc::c_int
    {
        log_error(
            (*r).conf.errh,
            b"src/mod_deflate.c\0" as *const u8 as *const libc::c_char,
            1573 as libc::c_int as libc::c_uint,
            b"compress failed.\0" as *const u8 as *const libc::c_char,
        );
        toSend = -(1 as libc::c_int) as off_t;
    }
    free(start as *mut libc::c_void);
    return toSend as libc::c_int;
}
unsafe extern "C" fn deflate_compress_response(
    r: *mut request_st,
    hctx: *mut handler_ctx,
) -> handler_t {
    let mut len: off_t = 0;
    let mut max: off_t = 0;
    let mut close_stream: libc::c_int = 0;
    let cq: *mut chunkqueue = &mut (*r).write_queue;
    len = chunkqueue_length(cq);
    chunkqueue_remove_finished_chunks(cq);
    chunkqueue_append_chunkqueue(&mut (*hctx).in_queue, cq);
    (*cq).bytes_in -= len;
    (*cq).bytes_out -= len;
    max = chunkqueue_length(&mut (*hctx).in_queue);
    while max != 0 {
        let mut c: *mut chunk = (*hctx).in_queue.first;
        match (*c).type_0 as libc::c_uint {
            0 => {
                len = buffer_clen((*c).mem) as libc::c_long - (*c).offset;
                if len > max {
                    len = max;
                }
                if mod_deflate_compress(
                    hctx,
                    ((*(*c).mem).ptr as *mut libc::c_uchar).offset((*c).offset as isize),
                    len,
                ) < 0 as libc::c_int
                {
                    log_error(
                        (*r).conf.errh,
                        b"src/mod_deflate.c\0" as *const u8 as *const libc::c_char,
                        1618 as libc::c_int as libc::c_uint,
                        b"compress failed.\0" as *const u8 as *const libc::c_char,
                    );
                    return HANDLER_ERROR;
                }
            }
            1 => {
                len = (*c).file.length - (*c).offset;
                if len > max {
                    len = max;
                }
                len = mod_deflate_file_chunk(r, hctx, c, len) as off_t;
                if len < 0 as libc::c_int as libc::c_long {
                    log_error(
                        (*r).conf.errh,
                        b"src/mod_deflate.c\0" as *const u8 as *const libc::c_char,
                        1626 as libc::c_int as libc::c_uint,
                        b"compress file chunk failed.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return HANDLER_ERROR;
                }
            }
            _ => {
                log_error(
                    (*r).conf.errh,
                    b"src/mod_deflate.c\0" as *const u8 as *const libc::c_char,
                    1631 as libc::c_int as libc::c_uint,
                    b"%d type not known\0" as *const u8 as *const libc::c_char,
                    (*c).type_0 as libc::c_uint,
                );
                return HANDLER_ERROR;
            }
        }
        max -= len;
        chunkqueue_mark_written(&mut (*hctx).in_queue, len);
    }
    close_stream = ((*r).resp_body_finished as libc::c_int != 0
        && chunkqueue_is_empty(&mut (*hctx).in_queue) != 0) as libc::c_int;
    if mod_deflate_stream_flush(hctx, close_stream) < 0 as libc::c_int {
        log_error(
            (*r).conf.errh,
            b"src/mod_deflate.c\0" as *const u8 as *const libc::c_char,
            1644 as libc::c_int as libc::c_uint,
            b"flush error\0" as *const u8 as *const libc::c_char,
        );
        return HANDLER_ERROR;
    }
    return (if close_stream != 0 {
        HANDLER_FINISHED as libc::c_int
    } else {
        HANDLER_GO_ON as libc::c_int
    }) as handler_t;
}
unsafe extern "C" fn mod_deflate_choose_encoding(
    mut value: *const libc::c_char,
    mut p: *mut plugin_data,
    mut label: *mut *const libc::c_char,
) -> libc::c_int {
    let mut accept_encoding: libc::c_int = 0 as libc::c_int;
    let mut x: *const uint16_t = (*p).conf.allowed_encodings;
    if x.is_null() {
        return 0 as libc::c_int;
    }
    while *x as libc::c_int != 0 && *x as libc::c_int & accept_encoding == 0 {
        x = x.offset(1);
    }
    accept_encoding &= *x as libc::c_int;
    if 0 as libc::c_int == accept_encoding {
        return 0 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn mod_deflate_handle_response_start(
    mut r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    let mut vbro: *const buffer = 0 as *const buffer;
    let mut vb: *mut buffer = 0 as *mut buffer;
    let mut hctx: *mut handler_ctx = 0 as *mut handler_ctx;
    let mut label: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: off_t = 0;
    let mut etaglen: uint32_t = 0;
    let mut compression_type: libc::c_int = 0;
    let mut rc: handler_t = HANDLER_GO_ON;
    let mut had_vary: libc::c_int = 0 as libc::c_int;
    if (*r).resp_body_finished == 0 {
        return HANDLER_GO_ON;
    }
    if (*r).http_method as libc::c_int == HTTP_METHOD_HEAD as libc::c_int {
        return HANDLER_GO_ON;
    }
    if (*r).resp_htags
        & (1 as libc::c_ulong) << HTTP_HEADER_TRANSFER_ENCODING as libc::c_int != 0
    {
        return HANDLER_GO_ON;
    }
    if (*r).resp_htags
        & (1 as libc::c_ulong) << HTTP_HEADER_CONTENT_ENCODING as libc::c_int != 0
    {
        return HANDLER_GO_ON;
    }
    match (*r).http_status {
        100 | 101 | 204 | 205 | 304 => return HANDLER_GO_ON,
        _ => {}
    }
    mod_deflate_patch_config(r, p);
    if ((*p).conf.mimetypes).is_null() {
        return HANDLER_GO_ON;
    }
    len = chunkqueue_length(&mut (*r).write_queue);
    if len <= (*p).conf.min_compress_size as off_t {
        return HANDLER_GO_ON;
    }
    if (*p).conf.max_compress_size != 0
        && len > ((*p).conf.max_compress_size as off_t) << 10 as libc::c_int
    {
        return HANDLER_GO_ON;
    }
    vbro = http_header_request_get(
        r,
        HTTP_HEADER_ACCEPT_ENCODING,
        b"Accept-Encoding\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if vbro.is_null() {
        return HANDLER_GO_ON;
    }
    compression_type = mod_deflate_choose_encoding((*vbro).ptr, p, &mut label);
    if compression_type == 0 {
        return HANDLER_GO_ON;
    }
    vbro = http_header_response_get(
        r,
        HTTP_HEADER_CONTENT_TYPE,
        b"Content-Type\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if !vbro.is_null() {
        if (array_match_value_prefix((*p).conf.mimetypes, vbro)).is_null() {
            return HANDLER_GO_ON;
        }
    } else {
        let mut mimetype: *mut data_string = *((*(*p).conf.mimetypes).data)
            .offset(0 as libc::c_int as isize) as *mut data_string;
        if buffer_is_blank(&mut (*mimetype).value) == 0 {
            return HANDLER_GO_ON;
        }
    }
    vb = http_header_response_get(
        r,
        HTTP_HEADER_VARY,
        b"Vary\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if !vb.is_null() {
        had_vary = 1 as libc::c_int;
        if http_header_str_contains_token(
            (*vb).ptr,
            buffer_clen(vb),
            b"Accept-Encoding\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        ) == 0
        {
            buffer_append_string_len(
                vb,
                b",Accept-Encoding\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
    } else {
        http_header_response_append(
            r,
            HTTP_HEADER_VARY,
            b"Vary\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            b"Accept-Encoding\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    vb = http_header_response_get(
        r,
        HTTP_HEADER_ETAG,
        b"ETag\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    etaglen = if !vb.is_null() {
        buffer_clen(vb)
    } else {
        0 as libc::c_int as libc::c_uint
    };
    if etaglen != 0
        && (*r).rqst_htags
            & (1 as libc::c_ulong) << HTTP_HEADER_IF_NONE_MATCH as libc::c_int != 0
    {
        let mut if_none_match: *const buffer = http_header_response_get(
            r,
            HTTP_HEADER_IF_NONE_MATCH,
            b"If-None-Match\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        if (*r).http_status < 300 as libc::c_int && !if_none_match.is_null()
            && 0 as libc::c_int
                == strncmp(
                    (*if_none_match).ptr,
                    (*vb).ptr,
                    etaglen.wrapping_sub(1 as libc::c_int as libc::c_uint)
                        as libc::c_ulong,
                )
            && *((*if_none_match).ptr)
                .offset(etaglen.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
                as libc::c_int == '-' as i32
            && 0 as libc::c_int
                == strncmp(
                    ((*if_none_match).ptr).offset(etaglen as isize),
                    label,
                    strlen(label),
                )
        {
            if (*r).http_method as libc::c_int <= HTTP_METHOD_HEAD as libc::c_int {
                *((*vb).ptr)
                    .offset(
                        etaglen.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                    ) = '-' as i32 as libc::c_char;
                buffer_append_string(vb, label);
                buffer_append_string_len(
                    vb,
                    b"\"\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
                (*r).http_status = 304 as libc::c_int;
            } else {
                (*r).http_status = 412 as libc::c_int;
            }
            http_response_body_clear(r, 0 as libc::c_int);
            (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
            (*r).handler_module = 0 as *const plugin;
            return HANDLER_GO_ON;
        }
    }
    if 0.0f64 < (*p).conf.max_loadavg
        && (*p).conf.max_loadavg < (*(*(*r).con).srv).loadavg[0 as libc::c_int as usize]
    {
        return HANDLER_GO_ON;
    }
    if etaglen != 0 {
        *((*vb).ptr)
            .offset(
                etaglen.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
            ) = '-' as i32 as libc::c_char;
        buffer_append_string(vb, label);
        buffer_append_string_len(
            vb,
            b"\"\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    http_header_response_set(
        r,
        HTTP_HEADER_CONTENT_ENCODING,
        b"Content-Encoding\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
        label,
        strlen(label) as uint32_t,
    );
    if HTTP_METHOD_HEAD as libc::c_int == (*r).http_method as libc::c_int {
        http_response_body_clear(r, 0 as libc::c_int);
        return HANDLER_GO_ON;
    }
    let mut tb: *mut buffer = 0 as *mut buffer;
    if !((*p).conf.cache_dir).is_null() && had_vary == 0
        && etaglen > 2 as libc::c_int as libc::c_uint
        && (*r).resp_body_finished as libc::c_int != 0
        && (*r).write_queue.first == (*r).write_queue.last
        && (*(*r).write_queue.first).type_0 as libc::c_uint
            == FILE_CHUNK as libc::c_int as libc::c_uint
        && (*(*r).write_queue.first).offset == 0 as libc::c_int as libc::c_long
        && (*(*r).write_queue.first).file.is_temp == 0
        && (*r).http_status != 206 as libc::c_int
    {
        tb = mod_deflate_cache_file_name(r, (*p).conf.cache_dir, vb);
        let mut sce: *mut stat_cache_entry = stat_cache_get_entry_open(
            tb,
            1 as libc::c_int,
        );
        if !sce.is_null() {
            chunkqueue_reset(&mut (*r).write_queue);
            if (*sce).fd < 0 as libc::c_int
                || 0 as libc::c_int != http_chunk_append_file_ref(r, sce)
            {
                return HANDLER_ERROR;
            }
            if (*r).resp_htags
                & (1 as libc::c_ulong) << HTTP_HEADER_CONTENT_LENGTH as libc::c_int != 0
            {
                http_header_response_unset(
                    r,
                    HTTP_HEADER_CONTENT_LENGTH,
                    b"Content-Length\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                );
            }
            mod_deflate_note_ratio(r, (*sce).st.st_size, len);
            return HANDLER_GO_ON;
        }
        sce = stat_cache_get_entry((*(*r).write_queue.first).mem);
        if sce.is_null() || (*sce).st.st_size != len {
            tb = 0 as *mut buffer;
        } else if 0 as libc::c_int != mkdir_for_file((*tb).ptr) {
            tb = 0 as *mut buffer;
        }
    }
    (*p)
        .conf
        .sync_flush = ((*r).conf.stream_response_body as libc::c_int
        & ((1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 1 as libc::c_int) != 0
        && 0 as libc::c_int == (*p).conf.output_buffer_size as libc::c_int)
        as libc::c_int as libc::c_ushort;
    hctx = handler_ctx_init();
    (*hctx).plugin_data = p;
    (*hctx).compression_type = compression_type;
    (*hctx).r = r;
    buffer_clear(&mut (*p).tmp_buf);
    (*hctx).output = &mut (*p).tmp_buf;
    if !tb.is_null() {
        mod_deflate_cache_file_open(hctx, tb);
    }
    if 0 as libc::c_int != mod_deflate_stream_init(hctx) {
        handler_ctx_free(hctx);
        log_error(
            (*r).conf.errh,
            b"src/mod_deflate.c\0" as *const u8 as *const libc::c_char,
            1972 as libc::c_int as libc::c_uint,
            b"Failed to initialize compression %s\0" as *const u8 as *const libc::c_char,
            label,
        );
        if etaglen != 0 {
            *((*vb).ptr)
                .offset(
                    etaglen.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                ) = '"' as i32 as libc::c_char;
            buffer_truncate(vb, etaglen);
        }
        http_header_response_unset(
            r,
            HTTP_HEADER_CONTENT_ENCODING,
            b"Content-Encoding\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        return HANDLER_GO_ON;
    }
    if (*r).resp_htags
        & (1 as libc::c_ulong) << HTTP_HEADER_CONTENT_LENGTH as libc::c_int != 0
    {
        http_header_response_unset(
            r,
            HTTP_HEADER_CONTENT_LENGTH,
            b"Content-Length\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    let ref mut fresh1 = *((*r).plugin_ctx).offset((*p).id as isize);
    *fresh1 = hctx as *mut libc::c_void;
    rc = deflate_compress_response(r, hctx);
    if HANDLER_GO_ON as libc::c_int as libc::c_uint == rc as libc::c_uint {
        return HANDLER_GO_ON;
    }
    if HANDLER_FINISHED as libc::c_int as libc::c_uint == rc as libc::c_uint {
        if -(1 as libc::c_int) == (*hctx).cache_fd
            || 0 as libc::c_int == mod_deflate_cache_file_finish(r, hctx, tb)
        {
            mod_deflate_note_ratio(r, (*hctx).bytes_out, (*hctx).bytes_in);
            rc = HANDLER_GO_ON;
        } else {
            rc = HANDLER_ERROR;
        }
    }
    let ref mut fresh2 = *((*r).plugin_ctx).offset((*p).id as isize);
    *fresh2 = 0 as *mut libc::c_void;
    if deflate_compress_cleanup(r, hctx) < 0 as libc::c_int {
        return HANDLER_ERROR;
    }
    return rc;
}
unsafe extern "C" fn mod_deflate_cleanup(
    r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    let mut hctx: *mut handler_ctx = *((*r).plugin_ctx).offset((*p).id as isize)
        as *mut handler_ctx;
    if !hctx.is_null() {
        let ref mut fresh3 = *((*r).plugin_ctx).offset((*p).id as isize);
        *fresh3 = 0 as *mut libc::c_void;
        deflate_compress_cleanup(r, hctx);
    }
    return HANDLER_GO_ON;
}
#[no_mangle]
pub unsafe extern "C" fn mod_deflate_plugin_init(mut p: *mut plugin) -> libc::c_int {
    (*p).version = 0x10440 as libc::c_int as size_t;
    (*p).name = b"deflate\0" as *const u8 as *const libc::c_char;
    (*p)
        .init = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
    >(Some(mod_deflate_init as unsafe extern "C" fn() -> *mut libc::c_void));
    (*p)
        .cleanup = Some(
        mod_deflate_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*p)
        .set_defaults = Some(
        mod_deflate_set_defaults
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_request_reset = Some(
        mod_deflate_cleanup
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_response_start = Some(
        mod_deflate_handle_response_start
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    return 0 as libc::c_int;
}
pub unsafe fn run_static_initializers() {
    cpk = [
        {
            let mut init = config_plugin_keys_t {
                k: b"deflate.mimetypes\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_VLIST as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"deflate.allowed-encodings\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_VLIST as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"deflate.max-compress-size\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_INT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"deflate.min-compress-size\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"deflate.compression-level\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"deflate.output-buffer-size\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"deflate.work-block-size\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"deflate.max-loadavg\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"deflate.cache-dir\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"compress.filetype\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_VLIST as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"compress.allowed-encodings\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_VLIST as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"compress.cache-dir\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"compress.max-filesize\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_INT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"compress.max-loadavg\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"deflate.params\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVANY as libc::c_int as uint8_t,
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
