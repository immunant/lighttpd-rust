use ::libc;
extern "C" {
    pub type stat_cache_entry;
    pub type pcre2_real_match_data_8;
    pub type h2con;
    pub type fdevents;
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_str3(
        b: *mut buffer,
        s1: *const libc::c_char,
        len1: size_t,
        s2: *const libc::c_char,
        len2: size_t,
        s3: *const libc::c_char,
        len3: size_t,
    );
    fn buffer_append_int(b: *mut buffer, val: intmax_t);
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn fdevent_pipe_cloexec(
        fds: *mut libc::c_int,
        bufsz_hint: libc::c_uint,
    ) -> libc::c_int;
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
    fn config_plugin_values_init(
        srv: *mut server,
        p_d: *mut libc::c_void,
        cpk: *const config_plugin_keys_t,
        mname: *const libc::c_char,
    ) -> libc::c_int;
    fn config_check_cond(r: *mut request_st, context_ndx: libc::c_int) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn __errno_location() -> *mut libc::c_int;
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
pub struct rrd_config {
    pub path_rrd: *const buffer,
    pub requests: off_t,
    pub bytes_written: off_t,
    pub bytes_read: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_config {
    pub rrd: *mut rrd_config,
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
    pub read_fd: libc::c_int,
    pub write_fd: libc::c_int,
    pub rrdtool_pid: pid_t,
    pub srv_pid: pid_t,
    pub rrdtool_running: libc::c_int,
    pub path_rrdtool_bin: *const buffer,
    pub srv: *mut server,
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
#[cold]
unsafe extern "C" fn mod_rrd_init() -> *mut libc::c_void {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<plugin_data>() as libc::c_ulong,
    );
}
unsafe extern "C" fn mod_rrd_free_config(p: *mut plugin_data) {
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
            match (*cpv).k_id {
                0 => {
                    if (*cpv).vtype as libc::c_uint
                        == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
                    {
                        free((*cpv).v.v);
                    }
                }
                _ => {}
            }
            cpv = cpv.offset(1);
        }
        i += 1;
    }
}
#[cold]
unsafe extern "C" fn mod_rrd_free(mut p_d: *mut libc::c_void) {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    if ((*p).srv).is_null() {
        return;
    }
    mod_rrd_free_config(p);
    if (*p).read_fd >= 0 as libc::c_int {
        close((*p).read_fd);
    }
    if (*p).write_fd >= 0 as libc::c_int {
        close((*p).write_fd);
    }
    if (*p).rrdtool_pid > 0 as libc::c_int && (*p).srv_pid == (*(*p).srv).pid {
        fdevent_waitpid((*p).rrdtool_pid, 0 as *mut libc::c_int, 0 as libc::c_int);
    }
}
unsafe extern "C" fn mod_rrd_create_pipe(
    mut srv: *mut server,
    mut p: *mut plugin_data,
) -> libc::c_int {
    let mut args: [*mut libc::c_char; 3] = [0 as *mut libc::c_char; 3];
    let mut to_rrdtool_fds: [libc::c_int; 2] = [0; 2];
    let mut from_rrdtool_fds: [libc::c_int; 2] = [0; 2];
    if fdevent_pipe_cloexec(
        to_rrdtool_fds.as_mut_ptr(),
        4096 as libc::c_int as libc::c_uint,
    ) != 0
    {
        log_perror(
            (*srv).errh,
            b"src/mod_rrdtool.c\0" as *const u8 as *const libc::c_char,
            90 as libc::c_int as libc::c_uint,
            b"pipe()\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fdevent_pipe_cloexec(
        from_rrdtool_fds.as_mut_ptr(),
        4096 as libc::c_int as libc::c_uint,
    ) != 0
    {
        log_perror(
            (*srv).errh,
            b"src/mod_rrdtool.c\0" as *const u8 as *const libc::c_char,
            94 as libc::c_int as libc::c_uint,
            b"pipe()\0" as *const u8 as *const libc::c_char,
        );
        close(to_rrdtool_fds[0 as libc::c_int as usize]);
        close(to_rrdtool_fds[1 as libc::c_int as usize]);
        return 0 as libc::c_int;
    }
    let path_rrdtool_bin: *const libc::c_char = if !((*p).path_rrdtool_bin).is_null() {
        (*(*p).path_rrdtool_bin).ptr as *const libc::c_char
    } else {
        b"/usr/bin/rrdtool\0" as *const u8 as *const libc::c_char
    };
    let ref mut fresh0 = *(&mut *args.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut *mut libc::c_char as *mut *const libc::c_char);
    *fresh0 = path_rrdtool_bin;
    let ref mut fresh1 = *(&mut *args.as_mut_ptr().offset(1 as libc::c_int as isize)
        as *mut *mut libc::c_char as *mut *const libc::c_char);
    *fresh1 = b"-\0" as *const u8 as *const libc::c_char;
    args[2 as libc::c_int as usize] = 0 as *mut libc::c_char;
    (*p)
        .rrdtool_pid = fdevent_fork_execve(
        args[0 as libc::c_int as usize],
        args.as_mut_ptr(),
        0 as *mut *mut libc::c_char,
        to_rrdtool_fds[0 as libc::c_int as usize],
        from_rrdtool_fds[1 as libc::c_int as usize],
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    );
    if -(1 as libc::c_int) != (*p).rrdtool_pid {
        close(from_rrdtool_fds[1 as libc::c_int as usize]);
        close(to_rrdtool_fds[0 as libc::c_int as usize]);
        if (*p).read_fd >= 0 as libc::c_int {
            close((*p).read_fd);
        }
        if (*p).write_fd >= 0 as libc::c_int {
            close((*p).write_fd);
        }
        (*p).write_fd = to_rrdtool_fds[1 as libc::c_int as usize];
        (*p).read_fd = from_rrdtool_fds[0 as libc::c_int as usize];
        (*p).srv_pid = (*srv).pid;
        return 1 as libc::c_int;
    } else {
        log_perror(
            (*srv).errh,
            b"src/mod_rrdtool.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int as libc::c_uint,
            b"fork/exec(%s)\0" as *const u8 as *const libc::c_char,
            path_rrdtool_bin,
        );
        close(to_rrdtool_fds[0 as libc::c_int as usize]);
        close(to_rrdtool_fds[1 as libc::c_int as usize]);
        close(from_rrdtool_fds[0 as libc::c_int as usize]);
        close(from_rrdtool_fds[1 as libc::c_int as usize]);
        return 0 as libc::c_int;
    };
}
#[inline(never)]
unsafe extern "C" fn mod_rrd_exec(
    mut srv: *mut server,
    mut p: *mut plugin_data,
) -> libc::c_int {
    (*p).rrdtool_running = mod_rrd_create_pipe(srv, p);
    return (*p).rrdtool_running;
}
unsafe extern "C" fn mod_rrd_merge_config_cpv(
    pconf: *mut plugin_config,
    cpv: *const config_plugin_value_t,
) {
    match (*cpv).k_id {
        0 => {
            if (*cpv).vtype as libc::c_uint
                == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
            {
                (*pconf).rrd = (*cpv).v.v as *mut rrd_config;
            }
        }
        1 => {}
        _ => return,
    };
}
unsafe extern "C" fn mod_rrd_merge_config(
    pconf: *mut plugin_config,
    mut cpv: *const config_plugin_value_t,
) {
    loop {
        mod_rrd_merge_config_cpv(pconf, cpv);
        cpv = cpv.offset(1);
        if !((*cpv).k_id != -(1 as libc::c_int)) {
            break;
        }
    };
}
unsafe extern "C" fn mod_rrd_patch_config(r: *mut request_st, p: *mut plugin_data) {
    (*p).conf = (*p).defaults;
    let mut i: libc::c_int = 1 as libc::c_int;
    let mut used: libc::c_int = (*p).nconfig;
    while i < used {
        if config_check_cond(
            r,
            (*((*p).cvlist).offset(i as isize)).k_id as uint32_t as libc::c_int,
        ) != 0
        {
            mod_rrd_merge_config(
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
static mut cpk: [config_plugin_keys_t; 3] = [config_plugin_keys_t {
    k: 0 as *const libc::c_char,
    klen: 0,
    ktype: 0,
    scope: 0,
}; 3];
#[cold]
unsafe extern "C" fn mod_rrd_set_defaults(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    (*p).srv = srv;
    if config_plugin_values_init(
        srv,
        p as *mut libc::c_void,
        cpk.as_ptr(),
        b"mod_rrdtool\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return HANDLER_ERROR;
    }
    let mut activate: libc::c_int = 0 as libc::c_int;
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
            match (*cpv).k_id {
                0 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        let mut rrd: *mut rrd_config = calloc(
                            1 as libc::c_int as libc::c_ulong,
                            ::std::mem::size_of::<rrd_config>() as libc::c_ulong,
                        ) as *mut rrd_config;
                        if rrd.is_null() {
                            ck_assert_failed(
                                b"src/mod_rrdtool.c\0" as *const u8 as *const libc::c_char,
                                189 as libc::c_int as libc::c_uint,
                                b"rrd\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        (*rrd).path_rrd = (*cpv).v.b;
                        (*cpv).v.v = rrd as *mut libc::c_void;
                        (*cpv).vtype = T_CONFIG_LOCAL;
                        activate = 1 as libc::c_int;
                    }
                }
                1 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        (*p).path_rrdtool_bin = (*cpv).v.b;
                    }
                }
                _ => {}
            }
            cpv = cpv.offset(1);
        }
        i += 1;
    }
    if (*p).nconfig > 0 as libc::c_int
        && (*(*p).cvlist).v.u2[1 as libc::c_int as usize] != 0
    {
        let mut cpv_0: *const config_plugin_value_t = ((*p).cvlist)
            .offset((*(*p).cvlist).v.u2[0 as libc::c_int as usize] as isize);
        if -(1 as libc::c_int) != (*cpv_0).k_id {
            mod_rrd_merge_config(&mut (*p).defaults, cpv_0);
        }
    }
    (*p).rrdtool_running = 0 as libc::c_int;
    (*p).read_fd = -(1 as libc::c_int);
    (*p).write_fd = -(1 as libc::c_int);
    return (if activate == 0 || mod_rrd_exec(srv, p) != 0 {
        HANDLER_GO_ON as libc::c_int
    } else {
        HANDLER_ERROR as libc::c_int
    }) as handler_t;
}
unsafe extern "C" fn safe_write(
    mut fd: libc::c_int,
    mut buf: *const libc::c_void,
    mut count: size_t,
) -> ssize_t {
    let mut res: ssize_t = 0;
    let mut sum: ssize_t = 0 as libc::c_int as ssize_t;
    loop {
        res = write(fd, buf, count);
        if res >= 0 as libc::c_int as libc::c_long {
            sum += res;
            if res == 0 as libc::c_int as libc::c_long || res as size_t == count {
                return sum;
            }
            count = (count as libc::c_ulong).wrapping_sub(res as libc::c_ulong) as size_t
                as size_t;
            buf = (buf as *const libc::c_char).offset(res as isize)
                as *const libc::c_void;
        } else {
            match *__errno_location() {
                4 => {
                    continue;
                }
                _ => {}
            }
            return -(1 as libc::c_int) as ssize_t;
        }
    };
}
unsafe extern "C" fn safe_read(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut sz: size_t,
) -> ssize_t {
    let mut res: ssize_t = 0;
    loop {
        res = read(
            fd,
            buf as *mut libc::c_void,
            sz.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        if !(-(1 as libc::c_int) as libc::c_long == res
            && *__errno_location() == 4 as libc::c_int)
        {
            break;
        }
    }
    if res >= 0 as libc::c_int as libc::c_long {
        *buf.offset(res as isize) = '\u{0}' as i32 as libc::c_char;
    }
    return res;
}
unsafe extern "C" fn mod_rrdtool_create_rrd(
    mut srv: *mut server,
    mut p: *mut plugin_data,
    mut s: *mut rrd_config,
    mut resp: *mut libc::c_char,
    mut respsz: size_t,
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
    if 0 as libc::c_int == stat((*(*s).path_rrd).ptr, &mut st) {
        if !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint)
        {
            log_error(
                (*srv).errh,
                b"src/mod_rrdtool.c\0" as *const u8 as *const libc::c_char,
                264 as libc::c_int as libc::c_uint,
                b"not a regular file: %s\0" as *const u8 as *const libc::c_char,
                (*(*s).path_rrd).ptr,
            );
            return HANDLER_ERROR as libc::c_int;
        }
        if st.st_size > 0 as libc::c_int as libc::c_long {
            return HANDLER_GO_ON as libc::c_int;
        }
    }
    let cmd: *mut buffer = (*srv).tmp_buf;
    buffer_clear(cmd);
    buffer_append_str3(
        cmd,
        b"create \0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        (*(*s).path_rrd).ptr,
        buffer_clen((*s).path_rrd) as size_t,
        b" --step 60 DS:InOctets:ABSOLUTE:600:U:U DS:OutOctets:ABSOLUTE:600:U:U DS:Requests:ABSOLUTE:600:U:U RRA:AVERAGE:0.5:1:600 RRA:AVERAGE:0.5:6:700 RRA:AVERAGE:0.5:24:775 RRA:AVERAGE:0.5:288:797 RRA:MAX:0.5:1:600 RRA:MAX:0.5:6:700 RRA:MAX:0.5:24:775 RRA:MAX:0.5:288:797 RRA:MIN:0.5:1:600 RRA:MIN:0.5:6:700 RRA:MIN:0.5:24:775 RRA:MIN:0.5:288:797\n\0"
            as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 341]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    if -(1 as libc::c_int) as libc::c_long
        == safe_write(
            (*p).write_fd,
            (*cmd).ptr as *const libc::c_void,
            buffer_clen(cmd) as size_t,
        )
    {
        log_perror(
            (*srv).errh,
            b"src/mod_rrdtool.c\0" as *const u8 as *const libc::c_char,
            300 as libc::c_int as libc::c_uint,
            b"rrdtool-write: failed\0" as *const u8 as *const libc::c_char,
        );
        return HANDLER_ERROR as libc::c_int;
    }
    if -(1 as libc::c_int) as libc::c_long == safe_read((*p).read_fd, resp, respsz) {
        log_perror(
            (*srv).errh,
            b"src/mod_rrdtool.c\0" as *const u8 as *const libc::c_char,
            305 as libc::c_int as libc::c_uint,
            b"rrdtool-read: failed\0" as *const u8 as *const libc::c_char,
        );
        return HANDLER_ERROR as libc::c_int;
    }
    if *resp.offset(0 as libc::c_int as isize) as libc::c_int != 'O' as i32
        || *resp.offset(1 as libc::c_int as isize) as libc::c_int != 'K' as i32
    {
        log_error(
            (*srv).errh,
            b"src/mod_rrdtool.c\0" as *const u8 as *const libc::c_char,
            310 as libc::c_int as libc::c_uint,
            b"rrdtool-response: %s %s\0" as *const u8 as *const libc::c_char,
            (*cmd).ptr,
            resp,
        );
        return HANDLER_ERROR as libc::c_int;
    }
    return HANDLER_GO_ON as libc::c_int;
}
#[cold]
unsafe extern "C" fn mod_rrd_fatal_error(mut p: *mut plugin_data) -> libc::c_int {
    (*p).rrdtool_running = 0 as libc::c_int;
    return 0 as libc::c_int;
}
#[inline(never)]
unsafe extern "C" fn mod_rrd_write_data(
    mut srv: *mut server,
    mut p: *mut plugin_data,
    mut s: *mut rrd_config,
) -> libc::c_int {
    let mut resp: [libc::c_char; 4096] = [0; 4096];
    if HANDLER_GO_ON as libc::c_int
        != mod_rrdtool_create_rrd(
            srv,
            p,
            s,
            resp.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        )
    {
        return 0 as libc::c_int;
    }
    let cmd: *mut buffer = (*srv).tmp_buf;
    buffer_clear(cmd);
    buffer_append_str3(
        cmd,
        b"update \0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        (*(*s).path_rrd).ptr,
        buffer_clen((*s).path_rrd) as size_t,
        b" N:\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(cmd, (*s).bytes_read);
    buffer_append_string_len(
        cmd,
        b":\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(cmd, (*s).bytes_written);
    buffer_append_string_len(
        cmd,
        b":\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(cmd, (*s).requests);
    buffer_append_string_len(
        cmd,
        b"\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    if -(1 as libc::c_int) as libc::c_long
        == safe_write(
            (*p).write_fd,
            (*cmd).ptr as *const libc::c_void,
            buffer_clen(cmd) as size_t,
        )
    {
        log_error(
            (*srv).errh,
            b"src/mod_rrdtool.c\0" as *const u8 as *const libc::c_char,
            345 as libc::c_int as libc::c_uint,
            b"rrdtool-write: failed\0" as *const u8 as *const libc::c_char,
        );
        return mod_rrd_fatal_error(p);
    }
    if -(1 as libc::c_int) as libc::c_long
        == safe_read(
            (*p).read_fd,
            resp.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        )
    {
        log_error(
            (*srv).errh,
            b"src/mod_rrdtool.c\0" as *const u8 as *const libc::c_char,
            350 as libc::c_int as libc::c_uint,
            b"rrdtool-read: failed\0" as *const u8 as *const libc::c_char,
        );
        return mod_rrd_fatal_error(p);
    }
    if resp[0 as libc::c_int as usize] as libc::c_int == 'O' as i32
        && resp[1 as libc::c_int as usize] as libc::c_int == 'K' as i32
    {
        (*s).requests = 0 as libc::c_int as off_t;
        (*s).bytes_written = 0 as libc::c_int as off_t;
        (*s).bytes_read = 0 as libc::c_int as off_t;
    } else if !(!(strstr(
            resp.as_mut_ptr(),
            b"(minimum one second step)\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
            && log_epoch_secs - (*srv).startup_ts < 3 as libc::c_int as libc::c_long)
        {
        log_error(
            (*srv).errh,
            b"src/mod_rrdtool.c\0" as *const u8 as *const libc::c_char,
            363 as libc::c_int as libc::c_uint,
            b"rrdtool-response: %s %s\0" as *const u8 as *const libc::c_char,
            (*cmd).ptr,
            resp.as_mut_ptr(),
        );
        return mod_rrd_fatal_error(p);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn mod_rrd_write_data_loop(
    mut srv: *mut server,
    mut p: *mut plugin_data,
) {
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
            match (*cpv).k_id {
                0 => {
                    if !((*cpv).vtype as libc::c_uint
                        != T_CONFIG_LOCAL as libc::c_int as libc::c_uint)
                    {
                        mod_rrd_write_data(srv, p, (*cpv).v.v as *mut rrd_config);
                        if (*p).rrdtool_running == 0 {
                            return;
                        }
                    }
                }
                _ => {}
            }
            cpv = cpv.offset(1);
        }
        i += 1;
    }
}
unsafe extern "C" fn mod_rrd_trigger(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    if 0 as libc::c_int == (*p).rrdtool_pid {
        return HANDLER_GO_ON;
    }
    if log_epoch_secs % 60 as libc::c_int as libc::c_long
        != 0 as libc::c_int as libc::c_long
    {
        return HANDLER_GO_ON;
    }
    if (*p).rrdtool_running == 0 {
        if (*srv).pid != (*p).srv_pid {
            return HANDLER_GO_ON;
        }
        if mod_rrd_exec(srv, p) == 0 {
            return HANDLER_GO_ON;
        }
    }
    mod_rrd_write_data_loop(srv, p);
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_rrd_waitpid_cb(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
    mut pid: pid_t,
    mut status: libc::c_int,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    if pid != (*p).rrdtool_pid {
        return HANDLER_GO_ON;
    }
    if (*srv).pid != (*p).srv_pid {
        return HANDLER_GO_ON;
    }
    (*p).rrdtool_running = 0 as libc::c_int;
    (*p).rrdtool_pid = -(1 as libc::c_int);
    return HANDLER_FINISHED;
}
unsafe extern "C" fn mod_rrd_account(
    mut r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    if 0 as libc::c_int == (*p).rrdtool_pid {
        return HANDLER_GO_ON;
    }
    mod_rrd_patch_config(r, p);
    let rrd: *mut rrd_config = (*p).conf.rrd;
    if rrd.is_null() {
        return HANDLER_GO_ON;
    }
    (*rrd).requests += 1;
    if (*r).http_version as libc::c_int <= HTTP_VERSION_1_1 as libc::c_int {
        (*rrd).bytes_written += (*(*r).con).bytes_written - (*r).bytes_written_ckpt;
        (*rrd).bytes_read += (*(*r).con).bytes_read - (*r).bytes_read_ckpt;
    } else {
        (*rrd).bytes_written += (*r).write_queue.bytes_out;
        (*rrd).bytes_read += (*r).read_queue.bytes_in;
    }
    return HANDLER_GO_ON;
}
#[no_mangle]
pub unsafe extern "C" fn mod_rrdtool_plugin_init(mut p: *mut plugin) -> libc::c_int {
    (*p).version = 0x10440 as libc::c_int as size_t;
    (*p).name = b"rrd\0" as *const u8 as *const libc::c_char;
    (*p)
        .init = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
    >(Some(mod_rrd_init as unsafe extern "C" fn() -> *mut libc::c_void));
    (*p).cleanup = Some(mod_rrd_free as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*p)
        .set_defaults = Some(
        mod_rrd_set_defaults
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_trigger = Some(
        mod_rrd_trigger
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_waitpid = Some(
        mod_rrd_waitpid_cb
            as unsafe extern "C" fn(
                *mut server,
                *mut libc::c_void,
                pid_t,
                libc::c_int,
            ) -> handler_t,
    );
    (*p)
        .handle_request_done = Some(
        mod_rrd_account
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    return 0 as libc::c_int;
}
pub unsafe fn run_static_initializers() {
    cpk = [
        {
            let mut init = config_plugin_keys_t {
                k: b"rrdtool.db-name\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"rrdtool.binary\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
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
