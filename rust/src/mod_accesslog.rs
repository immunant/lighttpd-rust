use ::libc;
extern "C" {
    pub type stat_cache_entry;
    pub type pcre2_real_match_data_8;
    pub type h2con;
    pub type fdevents;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn buffer_string_prepare_append(b: *mut buffer, size: size_t) -> *mut libc::c_char;
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_int(b: *mut buffer, val: intmax_t);
    fn buffer_append_strftime(
        b: *mut buffer,
        format: *const libc::c_char,
        tm: *const tm,
    );
    fn buffer_eq_slen(
        b: *const buffer,
        s: *const libc::c_char,
        slen: size_t,
    ) -> libc::c_int;
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn fdlog_open(fn_0: *const libc::c_char) -> *mut fdlog_st;
    fn fdlog_files_flush(errh: *mut fdlog_st, memrel: libc::c_int);
    fn http_method_buf(i: http_method_t) -> *const buffer;
    fn http_version_append(b: *mut buffer, version: http_version_t);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn sock_addr_get_port(saddr: *const sock_addr) -> libc::c_ushort;
    static mut log_epoch_secs: unix_time64_t;
    static mut log_monotonic_secs: unix_time64_t;
    fn write_all(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
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
    fn http_header_hkey_get(s: *const libc::c_char, slen: size_t) -> http_header_e;
    fn http_header_response_get(
        r: *const request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn http_header_request_get(
        r: *const request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn http_header_env_get(
        r: *const request_st,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn config_plugin_values_init(
        srv: *mut server,
        p_d: *mut libc::c_void,
        cpk: *const config_plugin_keys_t,
        mname: *const libc::c_char,
    ) -> libc::c_int;
    fn config_check_cond(r: *mut request_st, context_ndx: libc::c_int) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn syslog(__pri: libc::c_int, __fmt: *const libc::c_char, _: ...);
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type clockid_t = __clockid_t;
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
pub struct format_mapping {
    pub key: libc::c_char,
    pub type_0: C2RustUnnamed_6,
}
pub type C2RustUnnamed_6 = libc::c_uint;
pub const FORMAT_NOTE: C2RustUnnamed_6 = 30;
pub const FORMAT_RESPONSE_HEADER: C2RustUnnamed_6 = 29;
pub const FORMAT_KEEPALIVE_COUNT: C2RustUnnamed_6 = 28;
pub const FORMAT_BYTES_OUT: C2RustUnnamed_6 = 27;
pub const FORMAT_BYTES_IN: C2RustUnnamed_6 = 26;
pub const FORMAT_CONNECTION_STATUS: C2RustUnnamed_6 = 25;
pub const FORMAT_HTTP_HOST: C2RustUnnamed_6 = 24;
pub const FORMAT_SERVER_NAME: C2RustUnnamed_6 = 23;
pub const FORMAT_URL: C2RustUnnamed_6 = 22;
pub const FORMAT_TIME_USED: C2RustUnnamed_6 = 21;
pub const FORMAT_QUERY_STRING: C2RustUnnamed_6 = 20;
pub const FORMAT_SERVER_PORT: C2RustUnnamed_6 = 19;
pub const FORMAT_REQUEST_METHOD: C2RustUnnamed_6 = 18;
pub const FORMAT_REQUEST_PROTOCOL: C2RustUnnamed_6 = 17;
pub const FORMAT_FILENAME: C2RustUnnamed_6 = 16;
pub const FORMAT_ENV: C2RustUnnamed_6 = 15;
pub const FORMAT_TIME_USED_US: C2RustUnnamed_6 = 14;
pub const FORMAT_COOKIE: C2RustUnnamed_6 = 13;
pub const FORMAT_LOCAL_ADDR: C2RustUnnamed_6 = 12;
pub const FORMAT_REMOTE_ADDR: C2RustUnnamed_6 = 11;
pub const FORMAT_HEADER: C2RustUnnamed_6 = 10;
pub const FORMAT_BYTES_OUT_NO_HEADER: C2RustUnnamed_6 = 9;
pub const FORMAT_STATUS: C2RustUnnamed_6 = 8;
pub const FORMAT_REQUEST_LINE: C2RustUnnamed_6 = 7;
pub const FORMAT_TIMESTAMP: C2RustUnnamed_6 = 6;
pub const FORMAT_REMOTE_USER: C2RustUnnamed_6 = 5;
pub const FORMAT_REMOTE_IDENT: C2RustUnnamed_6 = 4;
pub const FORMAT_REMOTE_HOST: C2RustUnnamed_6 = 3;
pub const FORMAT_PERCENT: C2RustUnnamed_6 = 2;
pub const FORMAT_UNSUPPORTED: C2RustUnnamed_6 = 1;
pub const FORMAT_UNSET: C2RustUnnamed_6 = 0;
pub type e_optflags_time = libc::c_uint;
pub const FORMAT_FLAG_TIME_NSEC_FRAC: e_optflags_time = 128;
pub const FORMAT_FLAG_TIME_USEC_FRAC: e_optflags_time = 64;
pub const FORMAT_FLAG_TIME_MSEC_FRAC: e_optflags_time = 32;
pub const FORMAT_FLAG_TIME_NSEC: e_optflags_time = 16;
pub const FORMAT_FLAG_TIME_USEC: e_optflags_time = 8;
pub const FORMAT_FLAG_TIME_MSEC: e_optflags_time = 4;
pub const FORMAT_FLAG_TIME_SEC: e_optflags_time = 2;
pub const FORMAT_FLAG_TIME_BEGIN: e_optflags_time = 1;
pub const FORMAT_FLAG_TIME_END: e_optflags_time = 0;
pub type e_optflags_port = libc::c_uint;
pub const FORMAT_FLAG_PORT_REMOTE: e_optflags_port = 2;
pub const FORMAT_FLAG_PORT_LOCAL: e_optflags_port = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct format_field {
    pub type_0: C2RustUnnamed_7,
    pub field: libc::c_int,
    pub opt: libc::c_int,
    pub string: buffer,
}
pub type C2RustUnnamed_7 = libc::c_uint;
pub const FIELD_FORMAT: C2RustUnnamed_7 = 2;
pub const FIELD_STRING: C2RustUnnamed_7 = 1;
pub const FIELD_UNSET: C2RustUnnamed_7 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct format_fields {
    pub last_generated_accesslog_ts: unix_time64_t,
    pub ts_accesslog_str: buffer,
    pub ptr: [format_field; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_config {
    pub fdlog: *mut fdlog_st,
    pub use_syslog: libc::c_char,
    pub syslog_level: libc::c_ushort,
    pub parsed_format: *mut format_fields,
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
    pub default_format: *mut format_fields,
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
unsafe extern "C" fn buffer_append_buffer(mut b: *mut buffer, mut src: *const buffer) {
    buffer_append_string_len(b, (*src).ptr, buffer_clen(src) as size_t);
}
#[inline]
unsafe extern "C" fn buffer_truncate(mut b: *mut buffer, mut len: uint32_t) {
    *((*b).ptr).offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    (*b).used = len.wrapping_add(1 as libc::c_int as libc::c_uint);
}
#[inline]
unsafe extern "C" fn http_method_append(b: *mut buffer, method: http_method_t) {
    let kv: *const buffer = http_method_buf(method);
    buffer_append_string_len(b, (*kv).ptr, buffer_clen(kv) as size_t);
}
static mut fmap: [format_mapping; 32] = [
    {
        let mut init = format_mapping {
            key: '%' as i32 as libc::c_char,
            type_0: FORMAT_PERCENT,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'h' as i32 as libc::c_char,
            type_0: FORMAT_REMOTE_HOST,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'l' as i32 as libc::c_char,
            type_0: FORMAT_REMOTE_IDENT,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'u' as i32 as libc::c_char,
            type_0: FORMAT_REMOTE_USER,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 't' as i32 as libc::c_char,
            type_0: FORMAT_TIMESTAMP,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'r' as i32 as libc::c_char,
            type_0: FORMAT_REQUEST_LINE,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 's' as i32 as libc::c_char,
            type_0: FORMAT_STATUS,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'b' as i32 as libc::c_char,
            type_0: FORMAT_BYTES_OUT_NO_HEADER,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'i' as i32 as libc::c_char,
            type_0: FORMAT_HEADER,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'a' as i32 as libc::c_char,
            type_0: FORMAT_REMOTE_ADDR,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'A' as i32 as libc::c_char,
            type_0: FORMAT_LOCAL_ADDR,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'B' as i32 as libc::c_char,
            type_0: FORMAT_BYTES_OUT_NO_HEADER,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'C' as i32 as libc::c_char,
            type_0: FORMAT_COOKIE,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'D' as i32 as libc::c_char,
            type_0: FORMAT_TIME_USED_US,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'e' as i32 as libc::c_char,
            type_0: FORMAT_ENV,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'f' as i32 as libc::c_char,
            type_0: FORMAT_FILENAME,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'H' as i32 as libc::c_char,
            type_0: FORMAT_REQUEST_PROTOCOL,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'k' as i32 as libc::c_char,
            type_0: FORMAT_KEEPALIVE_COUNT,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'm' as i32 as libc::c_char,
            type_0: FORMAT_REQUEST_METHOD,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'n' as i32 as libc::c_char,
            type_0: FORMAT_NOTE,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'p' as i32 as libc::c_char,
            type_0: FORMAT_SERVER_PORT,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'P' as i32 as libc::c_char,
            type_0: FORMAT_UNSUPPORTED,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'q' as i32 as libc::c_char,
            type_0: FORMAT_QUERY_STRING,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'T' as i32 as libc::c_char,
            type_0: FORMAT_TIME_USED,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'U' as i32 as libc::c_char,
            type_0: FORMAT_URL,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'v' as i32 as libc::c_char,
            type_0: FORMAT_SERVER_NAME,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'V' as i32 as libc::c_char,
            type_0: FORMAT_HTTP_HOST,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'X' as i32 as libc::c_char,
            type_0: FORMAT_CONNECTION_STATUS,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'I' as i32 as libc::c_char,
            type_0: FORMAT_BYTES_IN,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'O' as i32 as libc::c_char,
            type_0: FORMAT_BYTES_OUT,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: 'o' as i32 as libc::c_char,
            type_0: FORMAT_RESPONSE_HEADER,
        };
        init
    },
    {
        let mut init = format_mapping {
            key: '\u{0}' as i32 as libc::c_char,
            type_0: FORMAT_UNSET,
        };
        init
    },
];
#[cold]
unsafe extern "C" fn mod_accesslog_init() -> *mut libc::c_void {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<plugin_data>() as libc::c_ulong,
    );
}
unsafe extern "C" fn accesslog_append_escaped_str(
    dest: *mut buffer,
    str: *const libc::c_char,
    len: size_t,
) {
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    if 0 as libc::c_int as libc::c_ulong == len {
        return;
    }
    buffer_string_prepare_append(dest, len);
    let mut current_block_20: u64;
    start = str;
    ptr = start;
    end = str.offset(len as isize);
    while ptr < end {
        let c: libc::c_uchar = *(ptr as *const libc::c_uchar);
        if !(c as libc::c_int >= ' ' as i32 && c as libc::c_int <= '~' as i32
            && c as libc::c_int != '"' as i32 && c as libc::c_int != '\\' as i32)
        {
            if start < ptr {
                buffer_append_string_len(
                    dest,
                    start,
                    ptr.offset_from(start) as libc::c_long as size_t,
                );
            }
            start = ptr.offset(1 as libc::c_int as isize);
            let mut h2: *const libc::c_char = 0 as *const libc::c_char;
            match c as libc::c_int {
                34 => {
                    h2 = b"\\\"\0" as *const u8 as *const libc::c_char;
                    current_block_20 = 4488286894823169796;
                }
                92 => {
                    h2 = b"\\\\\0" as *const u8 as *const libc::c_char;
                    current_block_20 = 4488286894823169796;
                }
                8 => {
                    h2 = b"\\b\0" as *const u8 as *const libc::c_char;
                    current_block_20 = 4488286894823169796;
                }
                10 => {
                    h2 = b"\\n\0" as *const u8 as *const libc::c_char;
                    current_block_20 = 4488286894823169796;
                }
                13 => {
                    h2 = b"\\r\0" as *const u8 as *const libc::c_char;
                    current_block_20 = 4488286894823169796;
                }
                9 => {
                    h2 = b"\\t\0" as *const u8 as *const libc::c_char;
                    current_block_20 = 4488286894823169796;
                }
                11 => {
                    h2 = b"\\v\0" as *const u8 as *const libc::c_char;
                    current_block_20 = 4488286894823169796;
                }
                _ => {
                    let mut hh: [libc::c_char; 5] = [
                        '\\' as i32 as libc::c_char,
                        'x' as i32 as libc::c_char,
                        0 as libc::c_int as libc::c_char,
                        0 as libc::c_int as libc::c_char,
                        0 as libc::c_int as libc::c_char,
                    ];
                    let mut h: libc::c_char = (c as libc::c_int >> 4 as libc::c_int)
                        as libc::c_char;
                    hh[2 as libc::c_int
                        as usize] = (if h as libc::c_int > 9 as libc::c_int {
                        h as libc::c_int - 10 as libc::c_int + 'A' as i32
                    } else {
                        h as libc::c_int + '0' as i32
                    }) as libc::c_char;
                    h = (c as libc::c_int & 0xff as libc::c_int) as libc::c_char;
                    hh[3 as libc::c_int
                        as usize] = (if h as libc::c_int > 9 as libc::c_int {
                        h as libc::c_int - 10 as libc::c_int + 'A' as i32
                    } else {
                        h as libc::c_int + '0' as i32
                    }) as libc::c_char;
                    buffer_append_string_len(
                        dest,
                        hh.as_mut_ptr(),
                        4 as libc::c_int as size_t,
                    );
                    current_block_20 = 4644295000439058019;
                }
            }
            match current_block_20 {
                4644295000439058019 => {}
                _ => {
                    buffer_append_string_len(dest, h2, 2 as libc::c_int as size_t);
                }
            }
        }
        ptr = ptr.offset(1);
    }
    if start < end {
        buffer_append_string_len(
            dest,
            start,
            end.offset_from(start) as libc::c_long as size_t,
        );
    }
}
unsafe extern "C" fn accesslog_append_escaped(
    mut dest: *mut buffer,
    mut str: *const buffer,
) {
    accesslog_append_escaped_str(dest, (*str).ptr, buffer_clen(str) as size_t);
}
#[cold]
unsafe extern "C" fn accesslog_parse_format_err(
    mut errh: *mut log_error_st,
    mut file: *const libc::c_char,
    mut line: libc::c_uint,
    mut f: *mut format_field,
    mut msg: *const libc::c_char,
) -> *mut format_fields {
    log_error(errh, file, line, b"%s\0" as *const u8 as *const libc::c_char, msg);
    while (*f).type_0 as libc::c_uint != FIELD_UNSET as libc::c_int as libc::c_uint {
        free((*f).string.ptr as *mut libc::c_void);
        f = f.offset(1);
    }
    return 0 as *mut format_fields;
}
unsafe extern "C" fn accesslog_parse_format(
    format: *const libc::c_char,
    flen: size_t,
    errh: *mut log_error_st,
) -> *mut format_fields {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0 as libc::c_int as size_t;
    let mut start: size_t = 0 as libc::c_int as size_t;
    let mut used: uint32_t = 0 as libc::c_int as uint32_t;
    let sz: uint32_t = 127 as libc::c_int as uint32_t;
    let mut f: *mut format_field = 0 as *mut format_field;
    let mut fptr: [format_field; 128] = [format_field {
        type_0: FIELD_UNSET,
        field: 0,
        opt: 0,
        string: buffer {
            ptr: 0 as *mut libc::c_char,
            used: 0,
            size: 0,
        },
    }; 128];
    memset(
        fptr.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[format_field; 128]>() as libc::c_ulong,
    );
    if 0 as libc::c_int != FIELD_UNSET as libc::c_int {
        return 0 as *mut format_fields;
    }
    if 0 as libc::c_int as libc::c_ulong == flen {
        return 0 as *mut format_fields;
    }
    i = 0 as libc::c_int as size_t;
    while i < flen {
        if !(*format.offset(i as isize) as libc::c_int != '%' as i32) {
            if i > 0 as libc::c_int as libc::c_ulong && start != i {
                if used == sz {
                    return accesslog_parse_format_err(
                        errh,
                        b"src/mod_accesslog.c\0" as *const u8 as *const libc::c_char,
                        248 as libc::c_int as libc::c_uint,
                        fptr.as_mut_ptr(),
                        b"too many fields (>= 127) in accesslog.format\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                f = fptr.as_mut_ptr().offset(used as isize);
                (*f).type_0 = FIELD_STRING;
                memset(
                    &mut (*f).string as *mut buffer as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<buffer>() as libc::c_ulong,
                );
                buffer_copy_string_len(
                    &mut (*f).string,
                    format.offset(start as isize),
                    i.wrapping_sub(start),
                );
                used = used.wrapping_add(1);
            }
            if used == sz {
                return accesslog_parse_format_err(
                    errh,
                    b"src/mod_accesslog.c\0" as *const u8 as *const libc::c_char,
                    261 as libc::c_int as libc::c_uint,
                    fptr.as_mut_ptr(),
                    b"too many fields (>= 127) in accesslog.format\0" as *const u8
                        as *const libc::c_char,
                );
            }
            match *format
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
            {
                62 | 60 => {
                    if *format
                        .offset(
                            i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int == '\u{0}' as i32
                    {
                        return accesslog_parse_format_err(
                            errh,
                            b"src/mod_accesslog.c\0" as *const u8 as *const libc::c_char,
                            270 as libc::c_int as libc::c_uint,
                            fptr.as_mut_ptr(),
                            b"%< and %> have to be followed by a format-specifier\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    j = 0 as libc::c_int as size_t;
                    while fmap[j as usize].key as libc::c_int != '\u{0}' as i32 {
                        if fmap[j as usize].key as libc::c_int
                            != *format
                                .offset(
                                    i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                                ) as libc::c_int
                        {
                            j = j.wrapping_add(1);
                        } else {
                            f = fptr.as_mut_ptr().offset(used as isize);
                            (*f).type_0 = FIELD_FORMAT;
                            (*f).field = fmap[j as usize].type_0 as libc::c_int;
                            (*f).opt = 0 as libc::c_int;
                            (*f).string.ptr = 0 as *mut libc::c_char;
                            used = used.wrapping_add(1);
                            break;
                        }
                    }
                    if fmap[j as usize].key as libc::c_int == '\u{0}' as i32 {
                        return accesslog_parse_format_err(
                            errh,
                            b"src/mod_accesslog.c\0" as *const u8 as *const libc::c_char,
                            292 as libc::c_int as libc::c_uint,
                            fptr.as_mut_ptr(),
                            b"%< and %> have to be followed by a valid format-specifier\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    start = i.wrapping_add(3 as libc::c_int as libc::c_ulong);
                    i = start.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                }
                123 => {
                    k = i.wrapping_add(2 as libc::c_int as libc::c_ulong);
                    while k < flen {
                        if *format.offset(k as isize) as libc::c_int == '}' as i32 {
                            break;
                        }
                        k = k.wrapping_add(1);
                    }
                    if k == flen {
                        return accesslog_parse_format_err(
                            errh,
                            b"src/mod_accesslog.c\0" as *const u8 as *const libc::c_char,
                            308 as libc::c_int as libc::c_uint,
                            fptr.as_mut_ptr(),
                            b"%{ has to be terminated by a }\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    if *format
                        .offset(
                            k.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int == '\u{0}' as i32
                    {
                        return accesslog_parse_format_err(
                            errh,
                            b"src/mod_accesslog.c\0" as *const u8 as *const libc::c_char,
                            314 as libc::c_int as libc::c_uint,
                            fptr.as_mut_ptr(),
                            b"%{...} has to be followed by a format-specifier\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    if k == i.wrapping_add(2 as libc::c_int as libc::c_ulong) {
                        return accesslog_parse_format_err(
                            errh,
                            b"src/mod_accesslog.c\0" as *const u8 as *const libc::c_char,
                            319 as libc::c_int as libc::c_uint,
                            fptr.as_mut_ptr(),
                            b"%{...} has to contain a string\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    j = 0 as libc::c_int as size_t;
                    while fmap[j as usize].key as libc::c_int != '\u{0}' as i32 {
                        if fmap[j as usize].key as libc::c_int
                            != *format
                                .offset(
                                    k.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                ) as libc::c_int
                        {
                            j = j.wrapping_add(1);
                        } else {
                            f = fptr.as_mut_ptr().offset(used as isize);
                            (*f).type_0 = FIELD_FORMAT;
                            (*f).field = fmap[j as usize].type_0 as libc::c_int;
                            (*f).opt = 0 as libc::c_int;
                            memset(
                                &mut (*f).string as *mut buffer as *mut libc::c_void,
                                0 as libc::c_int,
                                ::std::mem::size_of::<buffer>() as libc::c_ulong,
                            );
                            buffer_copy_string_len(
                                &mut (*f).string,
                                format.offset(i as isize).offset(2 as libc::c_int as isize),
                                k
                                    .wrapping_sub(
                                        i.wrapping_add(2 as libc::c_int as libc::c_ulong),
                                    ),
                            );
                            used = used.wrapping_add(1);
                            break;
                        }
                    }
                    if fmap[j as usize].key as libc::c_int == '\u{0}' as i32 {
                        return accesslog_parse_format_err(
                            errh,
                            b"src/mod_accesslog.c\0" as *const u8 as *const libc::c_char,
                            341 as libc::c_int as libc::c_uint,
                            fptr.as_mut_ptr(),
                            b"%{...} has to be followed by a valid format-specifier\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    start = k.wrapping_add(2 as libc::c_int as libc::c_ulong);
                    i = start.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                }
                _ => {
                    if *format
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int == '\u{0}' as i32
                    {
                        return accesslog_parse_format_err(
                            errh,
                            b"src/mod_accesslog.c\0" as *const u8 as *const libc::c_char,
                            352 as libc::c_int as libc::c_uint,
                            fptr.as_mut_ptr(),
                            b"% has to be followed by a format-specifier\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    j = 0 as libc::c_int as size_t;
                    while fmap[j as usize].key as libc::c_int != '\u{0}' as i32 {
                        if fmap[j as usize].key as libc::c_int
                            != *format
                                .offset(
                                    i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                ) as libc::c_int
                        {
                            j = j.wrapping_add(1);
                        } else {
                            f = fptr.as_mut_ptr().offset(used as isize);
                            (*f).type_0 = FIELD_FORMAT;
                            (*f).field = fmap[j as usize].type_0 as libc::c_int;
                            (*f).string.ptr = 0 as *mut libc::c_char;
                            (*f).opt = 0 as libc::c_int;
                            used = used.wrapping_add(1);
                            break;
                        }
                    }
                    if fmap[j as usize].key as libc::c_int == '\u{0}' as i32 {
                        return accesslog_parse_format_err(
                            errh,
                            b"src/mod_accesslog.c\0" as *const u8 as *const libc::c_char,
                            373 as libc::c_int as libc::c_uint,
                            fptr.as_mut_ptr(),
                            b"% has to be followed by a valid format-specifier\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    start = i.wrapping_add(2 as libc::c_int as libc::c_ulong);
                    i = start.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if start < i {
        if used == sz {
            return accesslog_parse_format_err(
                errh,
                b"src/mod_accesslog.c\0" as *const u8 as *const libc::c_char,
                387 as libc::c_int as libc::c_uint,
                fptr.as_mut_ptr(),
                b"too many fields (>= 127) in accesslog.format\0" as *const u8
                    as *const libc::c_char,
            );
        }
        f = fptr.as_mut_ptr().offset(used as isize);
        (*f).type_0 = FIELD_STRING;
        memset(
            &mut (*f).string as *mut buffer as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<buffer>() as libc::c_ulong,
        );
        buffer_copy_string_len(
            &mut (*f).string,
            format.offset(start as isize),
            i.wrapping_sub(start),
        );
        used = used.wrapping_add(1);
    }
    let fields: *mut format_fields = malloc(
        (::std::mem::size_of::<format_fields>() as libc::c_ulong)
            .wrapping_add(
                (used.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<format_field>() as libc::c_ulong),
            ),
    ) as *mut format_fields;
    if fields.is_null() {
        ck_assert_failed(
            b"src/mod_accesslog.c\0" as *const u8 as *const libc::c_char,
            400 as libc::c_int as libc::c_uint,
            b"fields\0" as *const u8 as *const libc::c_char,
        );
    }
    memset(
        fields as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<format_fields>() as libc::c_ulong,
    );
    memcpy(
        ((*fields).ptr).as_mut_ptr() as *mut libc::c_void,
        fptr.as_mut_ptr() as *const libc::c_void,
        (used.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<format_field>() as libc::c_ulong),
    );
    return fields;
}
unsafe extern "C" fn mod_accesslog_free_format_fields(ff: *mut format_fields) {
    let mut f: *mut format_field = ((*ff).ptr).as_mut_ptr();
    while (*f).type_0 as libc::c_uint != FIELD_UNSET as libc::c_int as libc::c_uint {
        free((*f).string.ptr as *mut libc::c_void);
        f = f.offset(1);
    }
    free((*ff).ts_accesslog_str.ptr as *mut libc::c_void);
    free(ff as *mut libc::c_void);
}
#[cold]
unsafe extern "C" fn mod_accesslog_free(mut p_d: *mut libc::c_void) {
    let p: *mut plugin_data = p_d as *mut plugin_data;
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
                    1 => {
                        mod_accesslog_free_format_fields(
                            (*cpv).v.v as *mut format_fields,
                        );
                    }
                    0 | _ => {}
                }
            }
            cpv = cpv.offset(1);
        }
        i += 1;
    }
    if !((*p).default_format).is_null() {
        mod_accesslog_free_format_fields((*p).default_format);
    }
}
unsafe extern "C" fn mod_accesslog_merge_config_cpv(
    pconf: *mut plugin_config,
    cpv: *const config_plugin_value_t,
) {
    match (*cpv).k_id {
        0 => {
            if !((*cpv).vtype as libc::c_uint
                != T_CONFIG_LOCAL as libc::c_int as libc::c_uint)
            {
                (*pconf).fdlog = (*cpv).v.v as *mut fdlog_st;
            }
        }
        1 => {
            if !((*cpv).vtype as libc::c_uint
                != T_CONFIG_LOCAL as libc::c_int as libc::c_uint)
            {
                (*pconf).parsed_format = (*cpv).v.v as *mut format_fields;
            }
        }
        2 => {
            (*pconf).use_syslog = (*cpv).v.u as libc::c_int as libc::c_char;
        }
        3 => {
            (*pconf).syslog_level = (*cpv).v.shrt;
        }
        _ => return,
    };
}
unsafe extern "C" fn mod_accesslog_merge_config(
    pconf: *mut plugin_config,
    mut cpv: *const config_plugin_value_t,
) {
    loop {
        mod_accesslog_merge_config_cpv(pconf, cpv);
        cpv = cpv.offset(1);
        if !((*cpv).k_id != -(1 as libc::c_int)) {
            break;
        }
    };
}
unsafe extern "C" fn mod_accesslog_patch_config(
    r: *mut request_st,
    p: *mut plugin_data,
) {
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
            mod_accesslog_merge_config(
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
static mut cpk: [config_plugin_keys_t; 5] = [config_plugin_keys_t {
    k: 0 as *const libc::c_char,
    klen: 0,
    ktype: 0,
    scope: 0,
}; 5];
#[cold]
unsafe extern "C" fn mod_accesslog_set_defaults(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    if config_plugin_values_init(
        srv,
        p as *mut libc::c_void,
        cpk.as_ptr(),
        b"mod_accesslog\0" as *const u8 as *const libc::c_char,
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
        let mut use_syslog: libc::c_int = 0 as libc::c_int;
        let mut cpvfile: *mut config_plugin_value_t = 0 as *mut config_plugin_value_t;
        while -(1 as libc::c_int) != (*cpv).k_id {
            match (*cpv).k_id {
                0 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        cpvfile = cpv;
                    } else {
                        (*cpv).v.v = 0 as *mut libc::c_void;
                        (*cpv).vtype = T_CONFIG_LOCAL;
                    }
                }
                1 => {
                    if !(strchr((*(*cpv).v.b).ptr, '\\' as i32)).is_null() {
                        let mut b: *mut buffer = 0 as *mut buffer;
                        let ref mut fresh0 = *(&mut b as *mut *mut buffer
                            as *mut *const buffer);
                        *fresh0 = (*cpv).v.b;
                        let mut t: *mut libc::c_char = (*b).ptr;
                        let mut s: *mut libc::c_char = t;
                        while *s != 0 {
                            if *s.offset(0 as libc::c_int as isize) as libc::c_int
                                != '\\' as i32
                            {
                                let fresh1 = t;
                                t = t.offset(1);
                                *fresh1 = *s;
                            } else if !(*s.offset(1 as libc::c_int as isize)
                                    as libc::c_int == '\u{0}' as i32)
                                {
                                s = s.offset(1);
                                match *s as libc::c_int {
                                    97 => {
                                        let fresh2 = t;
                                        t = t.offset(1);
                                        *fresh2 = '\u{7}' as i32 as libc::c_char;
                                    }
                                    98 => {
                                        let fresh3 = t;
                                        t = t.offset(1);
                                        *fresh3 = '\u{8}' as i32 as libc::c_char;
                                    }
                                    102 => {
                                        let fresh4 = t;
                                        t = t.offset(1);
                                        *fresh4 = '\u{c}' as i32 as libc::c_char;
                                    }
                                    110 => {
                                        let fresh5 = t;
                                        t = t.offset(1);
                                        *fresh5 = '\n' as i32 as libc::c_char;
                                    }
                                    114 => {
                                        let fresh6 = t;
                                        t = t.offset(1);
                                        *fresh6 = '\r' as i32 as libc::c_char;
                                    }
                                    116 => {
                                        let fresh7 = t;
                                        t = t.offset(1);
                                        *fresh7 = '\t' as i32 as libc::c_char;
                                    }
                                    118 => {
                                        let fresh8 = t;
                                        t = t.offset(1);
                                        *fresh8 = '\u{b}' as i32 as libc::c_char;
                                    }
                                    _ => {
                                        let fresh9 = t;
                                        t = t.offset(1);
                                        *fresh9 = *s;
                                    }
                                }
                            }
                            s = s.offset(1);
                        }
                        buffer_truncate(
                            b,
                            t.offset_from((*b).ptr) as libc::c_long as size_t as uint32_t,
                        );
                    }
                    (*cpv)
                        .v
                        .v = mod_accesslog_process_format(
                        (*(*cpv).v.b).ptr,
                        buffer_clen((*cpv).v.b) as size_t,
                        srv,
                    ) as *mut libc::c_void;
                    if ((*cpv).v.v).is_null() {
                        return HANDLER_ERROR;
                    }
                    (*cpv).vtype = T_CONFIG_LOCAL;
                }
                2 => {
                    use_syslog = (*cpv).v.u as libc::c_int;
                }
                3 => {}
                _ => {}
            }
            cpv = cpv.offset(1);
        }
        if !((*srv).srvconf.preflight_check != 0) {
            if !(use_syslog != 0) {
                cpv = cpvfile;
                if !cpv.is_null() {
                    let fn_0: *const libc::c_char = (*(*cpv).v.b).ptr;
                    (*cpv).v.v = fdlog_open(fn_0) as *mut libc::c_void;
                    (*cpv).vtype = T_CONFIG_LOCAL;
                    if ((*cpv).v.v).is_null() {
                        log_perror(
                            (*srv).errh,
                            b"src/mod_accesslog.c\0" as *const u8 as *const libc::c_char,
                            565 as libc::c_int as libc::c_uint,
                            b"opening log '%s' failed\0" as *const u8
                                as *const libc::c_char,
                            fn_0,
                        );
                        return HANDLER_ERROR;
                    }
                }
            }
        }
        i += 1;
    }
    (*p).defaults.syslog_level = 6 as libc::c_int as libc::c_ushort;
    if (*p).nconfig > 0 as libc::c_int
        && (*(*p).cvlist).v.u2[1 as libc::c_int as usize] != 0
    {
        let mut cpv_0: *const config_plugin_value_t = ((*p).cvlist)
            .offset((*(*p).cvlist).v.u2[0 as libc::c_int as usize] as isize);
        if -(1 as libc::c_int) != (*cpv_0).k_id {
            mod_accesslog_merge_config(&mut (*p).defaults, cpv_0);
        }
    }
    if ((*p).defaults.parsed_format).is_null() {
        static mut fmt: [libc::c_char; 55] = unsafe {
            *::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"%h %V %u %t \"%r\" %>s %b \"%{Referer}i\" \"%{User-Agent}i\"\0")
        };
        (*p)
            .default_format = mod_accesslog_process_format(
            fmt.as_ptr(),
            (::std::mem::size_of::<[libc::c_char; 55]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            srv,
        );
        (*p).defaults.parsed_format = (*p).default_format;
        if ((*p).default_format).is_null() {
            return HANDLER_ERROR;
        }
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_accesslog_process_format(
    format: *const libc::c_char,
    flen: size_t,
    srv: *mut server,
) -> *mut format_fields {
    let parsed_format: *mut format_fields = accesslog_parse_format(
        format,
        flen,
        (*srv).errh,
    );
    if parsed_format.is_null() {
        log_error(
            (*srv).errh,
            b"src/mod_accesslog.c\0" as *const u8 as *const libc::c_char,
            597 as libc::c_int as libc::c_uint,
            b"parsing accesslog-definition failed: %s\0" as *const u8
                as *const libc::c_char,
            format,
        );
        return 0 as *mut format_fields;
    }
    let mut tcount: uint32_t = 0 as libc::c_int as uint32_t;
    let mut f: *mut format_field = ((*parsed_format).ptr).as_mut_ptr();
    while (*f).type_0 as libc::c_uint != FIELD_UNSET as libc::c_int as libc::c_uint {
        let fstr: *const buffer = &mut (*f).string;
        if !(FIELD_FORMAT as libc::c_int as libc::c_uint != (*f).type_0 as libc::c_uint)
        {
            if FORMAT_TIMESTAMP as libc::c_int == (*f).field {
                if buffer_is_blank(fstr) == 0 {
                    let mut ptr: *const libc::c_char = (*fstr).ptr;
                    if 0 as libc::c_int
                        == strncmp(
                            ptr,
                            b"begin:\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        )
                    {
                        (*f).opt |= FORMAT_FLAG_TIME_BEGIN as libc::c_int;
                        ptr = ptr
                            .offset(
                                (::std::mem::size_of::<[libc::c_char; 7]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            );
                    } else if 0 as libc::c_int
                            == strncmp(
                                ptr,
                                b"end:\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 5]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            )
                        {
                        (*f).opt |= FORMAT_FLAG_TIME_END as libc::c_int;
                        ptr = ptr
                            .offset(
                                (::std::mem::size_of::<[libc::c_char; 5]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            );
                    }
                    if 0 as libc::c_int
                        == strcmp(ptr, b"sec\0" as *const u8 as *const libc::c_char)
                    {
                        (*f).opt |= FORMAT_FLAG_TIME_SEC as libc::c_int;
                    } else if 0 as libc::c_int
                            == strcmp(ptr, b"msec\0" as *const u8 as *const libc::c_char)
                        {
                        (*f).opt |= FORMAT_FLAG_TIME_MSEC as libc::c_int;
                    } else if 0 as libc::c_int
                            == strcmp(ptr, b"usec\0" as *const u8 as *const libc::c_char)
                        {
                        (*f).opt |= FORMAT_FLAG_TIME_USEC as libc::c_int;
                    } else if 0 as libc::c_int
                            == strcmp(ptr, b"nsec\0" as *const u8 as *const libc::c_char)
                        {
                        (*f).opt |= FORMAT_FLAG_TIME_NSEC as libc::c_int;
                    } else if 0 as libc::c_int
                            == strcmp(
                                ptr,
                                b"msec_frac\0" as *const u8 as *const libc::c_char,
                            )
                        {
                        (*f).opt |= FORMAT_FLAG_TIME_MSEC_FRAC as libc::c_int;
                    } else if 0 as libc::c_int
                            == strcmp(
                                ptr,
                                b"usec_frac\0" as *const u8 as *const libc::c_char,
                            )
                        {
                        (*f).opt |= FORMAT_FLAG_TIME_USEC_FRAC as libc::c_int;
                    } else if 0 as libc::c_int
                            == strcmp(
                                ptr,
                                b"nsec_frac\0" as *const u8 as *const libc::c_char,
                            )
                        {
                        (*f).opt |= FORMAT_FLAG_TIME_NSEC_FRAC as libc::c_int;
                    } else if (strchr(ptr, '%' as i32)).is_null() {
                        log_error(
                            (*srv).errh,
                            b"src/mod_accesslog.c\0" as *const u8 as *const libc::c_char,
                            624 as libc::c_int as libc::c_uint,
                            b"constant string for time format (misspelled token? or missing '%%'): %s\0"
                                as *const u8 as *const libc::c_char,
                            format,
                        );
                        mod_accesslog_free_format_fields(parsed_format);
                        return 0 as *mut format_fields;
                    }
                }
                if (*f).opt
                    & !(FORMAT_FLAG_TIME_BEGIN as libc::c_int
                        | FORMAT_FLAG_TIME_END as libc::c_int
                        | FORMAT_FLAG_TIME_SEC as libc::c_int) == 0
                    && {
                        tcount = tcount.wrapping_add(1);
                        tcount > 1 as libc::c_int as libc::c_uint
                    }
                {
                    log_error(
                        (*srv).errh,
                        b"src/mod_accesslog.c\0" as *const u8 as *const libc::c_char,
                        634 as libc::c_int as libc::c_uint,
                        b"you may not use strftime timestamp format %%{}t twice in the same access log: %s\0"
                            as *const u8 as *const libc::c_char,
                        format,
                    );
                    mod_accesslog_free_format_fields(parsed_format);
                    return 0 as *mut format_fields;
                }
                if (*f).opt & FORMAT_FLAG_TIME_BEGIN as libc::c_int != 0 {
                    (*srv)
                        .srvconf
                        .high_precision_timestamps = 1 as libc::c_int as libc::c_uchar;
                }
            } else if FORMAT_TIME_USED_US as libc::c_int == (*f).field {
                (*f).opt |= FORMAT_FLAG_TIME_USEC as libc::c_int;
                (*srv)
                    .srvconf
                    .high_precision_timestamps = 1 as libc::c_int as libc::c_uchar;
            } else if FORMAT_TIME_USED as libc::c_int == (*f).field {
                if buffer_is_blank(fstr) != 0
                    || buffer_eq_slen(
                        fstr,
                        b"s\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    ) != 0
                    || buffer_eq_slen(
                        fstr,
                        b"sec\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    ) != 0
                {
                    (*f).opt |= FORMAT_FLAG_TIME_SEC as libc::c_int;
                } else if buffer_eq_slen(
                        fstr,
                        b"ms\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    ) != 0
                        || buffer_eq_slen(
                            fstr,
                            b"msec\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        ) != 0
                    {
                    (*f).opt |= FORMAT_FLAG_TIME_MSEC as libc::c_int;
                } else if buffer_eq_slen(
                        fstr,
                        b"us\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    ) != 0
                        || buffer_eq_slen(
                            fstr,
                            b"usec\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        ) != 0
                    {
                    (*f).opt |= FORMAT_FLAG_TIME_USEC as libc::c_int;
                } else if buffer_eq_slen(
                        fstr,
                        b"ns\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    ) != 0
                        || buffer_eq_slen(
                            fstr,
                            b"nsec\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        ) != 0
                    {
                    (*f).opt |= FORMAT_FLAG_TIME_NSEC as libc::c_int;
                } else {
                    log_error(
                        (*srv).errh,
                        b"src/mod_accesslog.c\0" as *const u8 as *const libc::c_char,
                        655 as libc::c_int as libc::c_uint,
                        b"invalid time unit in %%{UNIT}T: %s\0" as *const u8
                            as *const libc::c_char,
                        format,
                    );
                    mod_accesslog_free_format_fields(parsed_format);
                    return 0 as *mut format_fields;
                }
                if (*f).opt & !(FORMAT_FLAG_TIME_SEC as libc::c_int) != 0 {
                    (*srv)
                        .srvconf
                        .high_precision_timestamps = 1 as libc::c_int as libc::c_uchar;
                }
            } else if FORMAT_COOKIE as libc::c_int == (*f).field {
                if buffer_is_blank(fstr) != 0 {
                    (*f).type_0 = FIELD_STRING;
                }
            } else if FORMAT_SERVER_PORT as libc::c_int == (*f).field {
                if buffer_is_blank(fstr) != 0 {
                    (*f).opt |= FORMAT_FLAG_PORT_LOCAL as libc::c_int;
                } else if buffer_eq_slen(
                        fstr,
                        b"canonical\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    ) != 0
                    {
                    (*f).opt |= FORMAT_FLAG_PORT_LOCAL as libc::c_int;
                } else if buffer_eq_slen(
                        fstr,
                        b"local\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    ) != 0
                    {
                    (*f).opt |= FORMAT_FLAG_PORT_LOCAL as libc::c_int;
                } else if buffer_eq_slen(
                        fstr,
                        b"remote\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    ) != 0
                    {
                    (*f).opt |= FORMAT_FLAG_PORT_REMOTE as libc::c_int;
                } else {
                    log_error(
                        (*srv).errh,
                        b"src/mod_accesslog.c\0" as *const u8 as *const libc::c_char,
                        674 as libc::c_int as libc::c_uint,
                        b"invalid format %%{canonical,local,remote}p: %s\0" as *const u8
                            as *const libc::c_char,
                        format,
                    );
                    mod_accesslog_free_format_fields(parsed_format);
                    return 0 as *mut format_fields;
                }
            } else if FORMAT_HEADER as libc::c_int == (*f).field
                    || FORMAT_RESPONSE_HEADER as libc::c_int == (*f).field
                {
                (*f)
                    .opt = http_header_hkey_get((*fstr).ptr, buffer_clen(fstr) as size_t)
                    as libc::c_int;
            }
        }
        f = f.offset(1);
    }
    return parsed_format;
}
unsafe extern "C" fn log_access_periodic_flush(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    if 0 as libc::c_int as libc::c_long
        == log_monotonic_secs & 3 as libc::c_int as libc::c_long
    {
        fdlog_files_flush((*srv).errh, 0 as libc::c_int);
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn log_access_record(
    r: *const request_st,
    b: *mut buffer,
    parsed_format: *mut format_fields,
) -> libc::c_int {
    let con: *const connection = (*r).con;
    let mut vb: *const buffer = 0 as *const buffer;
    let mut ts: unix_timespec64_t = {
        let mut init = timespec {
            tv_sec: 0 as libc::c_int as __time_t,
            tv_nsec: 0 as libc::c_int as __syscall_slong_t,
        };
        init
    };
    let mut flush: libc::c_int = 0 as libc::c_int;
    let mut f: *const format_field = ((*parsed_format).ptr).as_mut_ptr();
    while (*f).type_0 as libc::c_uint != FIELD_UNSET as libc::c_int as libc::c_uint {
        match (*f).type_0 as libc::c_uint {
            1 => {
                buffer_append_buffer(b, &(*f).string);
            }
            2 => {
                let mut current_block_181: u64;
                match (*f).field {
                    6 => {
                        if (*f).opt
                            & !(FORMAT_FLAG_TIME_BEGIN as libc::c_int
                                | FORMAT_FLAG_TIME_END as libc::c_int) != 0
                        {
                            if (*f).opt & FORMAT_FLAG_TIME_SEC as libc::c_int != 0 {
                                let mut t: unix_time64_t = if (*f).opt
                                    & FORMAT_FLAG_TIME_BEGIN as libc::c_int == 0
                                {
                                    log_epoch_secs
                                } else {
                                    (*r).start_hp.tv_sec
                                };
                                buffer_append_int(b, t);
                            } else if (*f).opt
                                    & (FORMAT_FLAG_TIME_MSEC as libc::c_int
                                        | FORMAT_FLAG_TIME_USEC as libc::c_int
                                        | FORMAT_FLAG_TIME_NSEC as libc::c_int) != 0
                                {
                                let mut t_0: unix_time64_t = 0;
                                let mut ns: libc::c_long = 0;
                                if (*f).opt & FORMAT_FLAG_TIME_BEGIN as libc::c_int == 0 {
                                    if 0 as libc::c_int as libc::c_long == ts.tv_sec {
                                        clock_gettime(0 as libc::c_int, &mut ts);
                                    }
                                    t_0 = ts.tv_sec;
                                    ns = ts.tv_nsec;
                                } else {
                                    t_0 = (*r).start_hp.tv_sec;
                                    ns = (*r).start_hp.tv_nsec;
                                }
                                if (*f).opt & FORMAT_FLAG_TIME_MSEC as libc::c_int != 0 {
                                    t_0 *= 1000 as libc::c_int as libc::c_long;
                                    t_0
                                        += (ns + 999999 as libc::c_int as libc::c_long)
                                            / 1000000 as libc::c_int as libc::c_long;
                                } else if (*f).opt & FORMAT_FLAG_TIME_USEC as libc::c_int
                                        != 0
                                    {
                                    t_0 *= 1000000 as libc::c_int as libc::c_long;
                                    t_0
                                        += (ns + 999 as libc::c_int as libc::c_long)
                                            / 1000 as libc::c_int as libc::c_long;
                                } else {
                                    t_0 *= 1000000000 as libc::c_int as libc::c_long;
                                    t_0 += ns;
                                }
                                buffer_append_int(b, t_0);
                            } else {
                                let mut ns_0: libc::c_long = 0;
                                let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
                                if (*f).opt & FORMAT_FLAG_TIME_BEGIN as libc::c_int == 0 {
                                    if 0 as libc::c_int as libc::c_long == ts.tv_sec {
                                        clock_gettime(0 as libc::c_int, &mut ts);
                                    }
                                    ns_0 = ts.tv_nsec;
                                } else {
                                    ns_0 = (*r).start_hp.tv_nsec;
                                }
                                if (*f).opt & FORMAT_FLAG_TIME_MSEC_FRAC as libc::c_int != 0
                                {
                                    ns_0 += 999999 as libc::c_int as libc::c_long;
                                    ns_0 /= 1000000 as libc::c_int as libc::c_long;
                                    buffer_append_string_len(
                                        b,
                                        b"000\0" as *const u8 as *const libc::c_char,
                                        (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                                            as uint32_t)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                    );
                                } else if (*f).opt
                                        & FORMAT_FLAG_TIME_USEC_FRAC as libc::c_int != 0
                                    {
                                    ns_0 += 999 as libc::c_int as libc::c_long;
                                    ns_0 /= 1000 as libc::c_int as libc::c_long;
                                    buffer_append_string_len(
                                        b,
                                        b"000000\0" as *const u8 as *const libc::c_char,
                                        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                                            as uint32_t)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                    );
                                } else {
                                    buffer_append_string_len(
                                        b,
                                        b"000000000\0" as *const u8 as *const libc::c_char,
                                        (::std::mem::size_of::<[libc::c_char; 10]>()
                                            as libc::c_ulong as uint32_t)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                    );
                                }
                                ptr = ((*b).ptr).offset(buffer_clen(b) as isize);
                                let mut x: libc::c_long = 0;
                                while ns_0 > 0 as libc::c_int as libc::c_long {
                                    x = ns_0 / 10 as libc::c_int as libc::c_long;
                                    ptr = ptr.offset(-1);
                                    *ptr = (*ptr as libc::c_long
                                        + (ns_0 - x * 10 as libc::c_int as libc::c_long))
                                        as libc::c_char;
                                    ns_0 = x;
                                }
                            }
                        } else {
                            let ts_accesslog_str: *mut buffer = &mut (*parsed_format)
                                .ts_accesslog_str;
                            let mut t_1: unix_time64_t = 0;
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
                            if (*f).opt & FORMAT_FLAG_TIME_BEGIN as libc::c_int == 0 {
                                let cur_ts: unix_time64_t = log_epoch_secs;
                                if (*parsed_format).last_generated_accesslog_ts == cur_ts {
                                    buffer_append_buffer(b, ts_accesslog_str);
                                    current_block_181 = 15689130159044723804;
                                } else {
                                    (*parsed_format).last_generated_accesslog_ts = cur_ts;
                                    t_1 = (*parsed_format).last_generated_accesslog_ts;
                                    flush = 1 as libc::c_int;
                                    current_block_181 = 12758904613967585247;
                                }
                            } else {
                                t_1 = (*r).start_hp.tv_sec;
                                current_block_181 = 12758904613967585247;
                            }
                            match current_block_181 {
                                15689130159044723804 => {}
                                _ => {
                                    let mut fmt: *const libc::c_char = if buffer_is_blank(
                                        &(*f).string,
                                    ) != 0
                                    {
                                        0 as *mut libc::c_char
                                    } else {
                                        (*f).string.ptr
                                    };
                                    buffer_clear(ts_accesslog_str);
                                    buffer_append_strftime(
                                        ts_accesslog_str,
                                        if !fmt.is_null() {
                                            fmt
                                        } else {
                                            b"[%d/%b/%Y:%T %z]\0" as *const u8 as *const libc::c_char
                                        },
                                        localtime_r(&mut t_1, &mut tm),
                                    );
                                    buffer_append_buffer(b, ts_accesslog_str);
                                }
                            }
                        }
                    }
                    21 | 14 => {
                        if (*f).opt & FORMAT_FLAG_TIME_SEC as libc::c_int != 0 {
                            buffer_append_int(b, log_epoch_secs - (*r).start_hp.tv_sec);
                        } else {
                            let bs: *const unix_timespec64_t = &(*r).start_hp;
                            let mut tdiff: off_t = 0;
                            if 0 as libc::c_int as libc::c_long == ts.tv_sec {
                                clock_gettime(0 as libc::c_int, &mut ts);
                            }
                            tdiff = (ts.tv_sec - (*bs).tv_sec)
                                * 1000000000 as libc::c_int as libc::c_long
                                + (ts.tv_nsec - (*bs).tv_nsec);
                            if tdiff <= 0 as libc::c_int as libc::c_long {
                                tdiff = -(1 as libc::c_int) as off_t;
                            } else if (*f).opt & FORMAT_FLAG_TIME_MSEC as libc::c_int
                                    != 0
                                {
                                tdiff += 999999 as libc::c_int as libc::c_long;
                                tdiff /= 1000000 as libc::c_int as libc::c_long;
                            } else if (*f).opt & FORMAT_FLAG_TIME_USEC as libc::c_int
                                    != 0
                                {
                                tdiff += 999 as libc::c_int as libc::c_long;
                                tdiff /= 1000 as libc::c_int as libc::c_long;
                            }
                            buffer_append_int(b, tdiff);
                        }
                    }
                    11 | 3 => {
                        buffer_append_buffer(b, &(*con).dst_addr_buf);
                    }
                    4 => {
                        buffer_append_string_len(
                            b,
                            b"-\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        );
                    }
                    5 => {
                        vb = http_header_env_get(
                            r,
                            b"REMOTE_USER\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                        );
                        if !vb.is_null() {
                            accesslog_append_escaped(b, vb);
                        } else {
                            buffer_append_string_len(
                                b,
                                b"-\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                    as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                            );
                        }
                    }
                    7 => {
                        http_method_append(b, (*r).http_method);
                        buffer_append_string_len(
                            b,
                            b" \0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        );
                        accesslog_append_escaped(b, &(*r).target_orig);
                        buffer_append_string_len(
                            b,
                            b" \0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        );
                        http_version_append(b, (*r).http_version);
                    }
                    8 => {
                        buffer_append_int(b, (*r).http_status as intmax_t);
                    }
                    9 => {
                        let mut bytes: off_t = if (*r).http_version as libc::c_int
                            <= HTTP_VERSION_1_1 as libc::c_int
                        {
                            (*con).bytes_written - (*r).bytes_written_ckpt
                        } else {
                            (*r).write_queue.bytes_out
                        };
                        if bytes > 0 as libc::c_int as libc::c_long {
                            bytes -= (*r).resp_header_len as off_t;
                            buffer_append_int(
                                b,
                                if bytes > 0 as libc::c_int as libc::c_long {
                                    bytes
                                } else {
                                    0 as libc::c_int as libc::c_long
                                },
                            );
                        } else {
                            buffer_append_string_len(
                                b,
                                b"-\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                    as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                            );
                        }
                    }
                    10 => {
                        vb = http_header_request_get(
                            r,
                            (*f).opt as http_header_e,
                            (*f).string.ptr,
                            buffer_clen(&(*f).string),
                        );
                        if !vb.is_null() {
                            accesslog_append_escaped(b, vb);
                        } else {
                            buffer_append_string_len(
                                b,
                                b"-\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                    as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                            );
                        }
                    }
                    29 => {
                        vb = http_header_response_get(
                            r,
                            (*f).opt as http_header_e,
                            (*f).string.ptr,
                            buffer_clen(&(*f).string),
                        );
                        if !vb.is_null() {
                            accesslog_append_escaped(b, vb);
                        } else {
                            buffer_append_string_len(
                                b,
                                b"-\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                    as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                            );
                        }
                    }
                    15 | 30 => {
                        vb = http_header_env_get(
                            r,
                            (*f).string.ptr,
                            buffer_clen(&(*f).string),
                        );
                        if !vb.is_null() {
                            accesslog_append_escaped(b, vb);
                        } else {
                            buffer_append_string_len(
                                b,
                                b"-\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                    as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                            );
                        }
                    }
                    16 => {
                        if buffer_is_blank(&(*r).physical.path) == 0 {
                            buffer_append_buffer(b, &(*r).physical.path);
                        } else {
                            buffer_append_string_len(
                                b,
                                b"-\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                    as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                            );
                        }
                    }
                    27 => {
                        let mut bytes_0: off_t = if (*r).http_version as libc::c_int
                            <= HTTP_VERSION_1_1 as libc::c_int
                        {
                            (*con).bytes_written - (*r).bytes_written_ckpt
                        } else {
                            (*r).write_queue.bytes_out
                        };
                        if bytes_0 > 0 as libc::c_int as libc::c_long {
                            buffer_append_int(b, bytes_0);
                        } else {
                            buffer_append_string_len(
                                b,
                                b"-\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                    as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                            );
                        }
                    }
                    26 => {
                        let mut bytes_1: off_t = if (*r).http_version as libc::c_int
                            <= HTTP_VERSION_1_1 as libc::c_int
                        {
                            (*con).bytes_read - (*r).bytes_read_ckpt
                        } else {
                            (*r).read_queue.bytes_in + (*r).rqst_header_len as off_t
                        };
                        if bytes_1 > 0 as libc::c_int as libc::c_long {
                            buffer_append_int(b, bytes_1);
                        } else {
                            buffer_append_string_len(
                                b,
                                b"-\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                    as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                            );
                        }
                    }
                    23 => {
                        if buffer_is_blank((*r).server_name) == 0 {
                            buffer_append_buffer(b, (*r).server_name);
                        } else {
                            buffer_append_string_len(
                                b,
                                b"-\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                    as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                            );
                        }
                    }
                    24 => {
                        if buffer_is_blank(&(*r).uri.authority) == 0 {
                            accesslog_append_escaped(b, &(*r).uri.authority);
                        } else {
                            buffer_append_string_len(
                                b,
                                b"-\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                    as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                            );
                        }
                    }
                    17 => {
                        http_version_append(b, (*r).http_version);
                    }
                    18 => {
                        http_method_append(b, (*r).http_method);
                    }
                    2 => {
                        buffer_append_string_len(
                            b,
                            b"%\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        );
                    }
                    12 => {
                        let srv_sock: *const server_socket = (*con).srv_socket;
                        buffer_append_string_len(
                            b,
                            (*(*srv_sock).srv_token).ptr,
                            (*srv_sock).srv_token_colon as size_t,
                        );
                    }
                    19 => {
                        if (*f).opt & FORMAT_FLAG_PORT_REMOTE as libc::c_int != 0 {
                            buffer_append_int(
                                b,
                                sock_addr_get_port(&(*con).dst_addr) as intmax_t,
                            );
                        } else {
                            let srv_sock_0: *const server_socket = (*con).srv_socket;
                            let srv_token: *const buffer = (*srv_sock_0).srv_token;
                            let tlen: size_t = buffer_clen(srv_token) as size_t;
                            let mut colon: size_t = (*srv_sock_0).srv_token_colon
                                as size_t;
                            if colon < tlen {
                                buffer_append_string_len(
                                    b,
                                    ((*srv_token).ptr)
                                        .offset(colon as isize)
                                        .offset(1 as libc::c_int as isize),
                                    tlen
                                        .wrapping_sub(
                                            colon.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                        ),
                                );
                            }
                        }
                    }
                    20 => {
                        accesslog_append_escaped(b, &(*r).uri.query);
                    }
                    22 => {
                        let len: uint32_t = buffer_clen(&(*r).target);
                        let qmark: *const libc::c_char = memchr(
                            (*r).target.ptr as *const libc::c_void,
                            '?' as i32,
                            len as libc::c_ulong,
                        ) as *const libc::c_char;
                        accesslog_append_escaped_str(
                            b,
                            (*r).target.ptr,
                            (if !qmark.is_null() {
                                qmark.offset_from((*r).target.ptr) as libc::c_long
                                    as uint32_t
                            } else {
                                len
                            }) as size_t,
                        );
                    }
                    25 => {
                        if (*r).state as libc::c_uint
                            == CON_STATE_RESPONSE_END as libc::c_int as libc::c_uint
                        {
                            if (*r).keep_alive as libc::c_int <= 0 as libc::c_int {
                                buffer_append_string_len(
                                    b,
                                    b"-\0" as *const u8 as *const libc::c_char,
                                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                        as uint32_t)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                );
                            } else {
                                buffer_append_string_len(
                                    b,
                                    b"+\0" as *const u8 as *const libc::c_char,
                                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                        as uint32_t)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                );
                            }
                        } else {
                            buffer_append_string_len(
                                b,
                                b"X\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                    as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                            );
                        }
                    }
                    28 => {
                        if (*con).request_count > 1 as libc::c_int as libc::c_uint {
                            buffer_append_int(
                                b,
                                ((*con).request_count)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as intmax_t,
                            );
                        } else {
                            buffer_append_string_len(
                                b,
                                b"0\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                    as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                            );
                        }
                    }
                    13 => {
                        vb = http_header_request_get(
                            r,
                            HTTP_HEADER_COOKIE,
                            b"Cookie\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                        );
                        if !vb.is_null() {
                            let mut str: *mut libc::c_char = (*vb).ptr;
                            let mut len_0: size_t = buffer_clen(&(*f).string) as size_t;
                            loop {
                                while *str as libc::c_int == ' ' as i32
                                    || *str as libc::c_int == '\t' as i32
                                {
                                    str = str.offset(1);
                                }
                                if 0 as libc::c_int == strncmp(str, (*f).string.ptr, len_0)
                                    && *str.offset(len_0 as isize) as libc::c_int == '=' as i32
                                {
                                    let mut v: *mut libc::c_char = str
                                        .offset(len_0 as isize)
                                        .offset(1 as libc::c_int as isize);
                                    str = v;
                                    while *str as libc::c_int != '\u{0}' as i32
                                        && *str as libc::c_int != ';' as i32
                                    {
                                        str = str.offset(1);
                                    }
                                    if str == v {
                                        break;
                                    }
                                    loop {
                                        str = str.offset(-1);
                                        if !(str > v
                                            && (*str as libc::c_int == ' ' as i32
                                                || *str as libc::c_int == '\t' as i32))
                                        {
                                            break;
                                        }
                                    }
                                    accesslog_append_escaped_str(
                                        b,
                                        v,
                                        (str.offset_from(v) as libc::c_long
                                            + 1 as libc::c_int as libc::c_long) as size_t,
                                    );
                                    break;
                                } else {
                                    while *str as libc::c_int != ';' as i32
                                        && *str as libc::c_int != ' ' as i32
                                        && *str as libc::c_int != '\t' as i32
                                        && *str as libc::c_int != '\u{0}' as i32
                                    {
                                        str = str.offset(1);
                                    }
                                    while *str as libc::c_int == ' ' as i32
                                        || *str as libc::c_int == '\t' as i32
                                    {
                                        str = str.offset(1);
                                    }
                                    let fresh10 = str;
                                    str = str.offset(1);
                                    if !(*fresh10 as libc::c_int == ';' as i32) {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        f = f.offset(1);
    }
    return flush;
}
unsafe extern "C" fn log_access_write(
    mut r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    mod_accesslog_patch_config(r, p);
    let fdlog: *mut fdlog_st = (*p).conf.fdlog;
    if (*p).conf.use_syslog == 0 && fdlog.is_null() {
        return HANDLER_GO_ON;
    }
    let b: *mut buffer = if (*p).conf.use_syslog as libc::c_int != 0
        || (*fdlog).mode as libc::c_uint == FDLOG_PIPE as libc::c_int as libc::c_uint
    {
        buffer_clear((*r).tmp_buf);
        (*r).tmp_buf
    } else {
        &mut (*fdlog).b
    };
    let flush: libc::c_int = log_access_record(r, b, (*p).conf.parsed_format);
    if (*p).conf.use_syslog != 0 {
        if buffer_is_blank(b) == 0 {
            syslog(
                (*p).conf.syslog_level as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*b).ptr,
            );
        }
        return HANDLER_GO_ON;
    }
    buffer_append_string_len(
        b,
        b"\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    if flush != 0
        || (*fdlog).mode as libc::c_uint == FDLOG_PIPE as libc::c_int as libc::c_uint
        || buffer_clen(b) >= 8192 as libc::c_int as libc::c_uint
    {
        let wr: ssize_t = write_all(
            (*fdlog).fd,
            (*b).ptr as *const libc::c_void,
            buffer_clen(b) as size_t,
        );
        buffer_clear(b);
        if -(1 as libc::c_int) as libc::c_long == wr {
            log_perror(
                (*r).conf.errh,
                b"src/mod_accesslog.c\0" as *const u8 as *const libc::c_char,
                1045 as libc::c_int as libc::c_uint,
                b"error flushing log %s\0" as *const u8 as *const libc::c_char,
                (*fdlog).fn_0,
            );
        }
    }
    return HANDLER_GO_ON;
}
#[no_mangle]
pub unsafe extern "C" fn mod_accesslog_plugin_init(mut p: *mut plugin) -> libc::c_int {
    (*p).version = 0x10440 as libc::c_int as size_t;
    (*p).name = b"accesslog\0" as *const u8 as *const libc::c_char;
    (*p)
        .init = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
    >(Some(mod_accesslog_init as unsafe extern "C" fn() -> *mut libc::c_void));
    (*p)
        .set_defaults = Some(
        mod_accesslog_set_defaults
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .cleanup = Some(
        mod_accesslog_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*p)
        .handle_request_done = Some(
        log_access_write
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_trigger = Some(
        log_access_periodic_flush
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    return 0 as libc::c_int;
}
pub unsafe fn run_static_initializers() {
    cpk = [
        {
            let mut init = config_plugin_keys_t {
                k: b"accesslog.filename\0" as *const u8 as *const libc::c_char,
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
                k: b"accesslog.format\0" as *const u8 as *const libc::c_char,
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
                k: b"accesslog.use-syslog\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"accesslog.syslog-level\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong
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
