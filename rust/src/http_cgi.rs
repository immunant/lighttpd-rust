use ::libc;
extern "C" {
    pub type fdnode_st;
    pub type stat_cache_entry;
    pub type cond_match_t;
    pub type cond_cache_t;
    pub type plugin;
    pub type h2con;
    pub type fdevents;
    pub type sockaddr_x25;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn buffer_copy_path_len2(
        b: *mut buffer,
        s1: *const libc::c_char,
        len1: size_t,
        s2: *const libc::c_char,
        len2: size_t,
    );
    fn buffer_string_prepare_copy(b: *mut buffer, size: size_t) -> *mut libc::c_char;
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn li_itostrn(buf: *mut libc::c_char, buf_len: size_t, val: intmax_t) -> size_t;
    fn li_utostrn(buf: *mut libc::c_char, buf_len: size_t, val: uintmax_t) -> size_t;
    fn buffer_eq_icase_slen(
        b: *const buffer,
        s: *const libc::c_char,
        slen: size_t,
    ) -> libc::c_int;
    fn buffer_eq_slen(
        b: *const buffer,
        s: *const libc::c_char,
        slen: size_t,
    ) -> libc::c_int;
    fn buffer_is_equal(a: *const buffer, b: *const buffer) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn getsockname(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn sock_addr_get_port(saddr: *const sock_addr) -> libc::c_ushort;
    fn sock_addr_inet_ntop(
        saddr: *const sock_addr,
        buf: *mut libc::c_char,
        sz: socklen_t,
    ) -> *const libc::c_char;
    fn sock_addr_is_addr_wildcard(saddr: *const sock_addr) -> libc::c_int;
    fn get_http_version_name(i: libc::c_int) -> *const libc::c_char;
    fn http_method_buf(i: http_method_t) -> *const buffer;
    fn chunkqueue_reset(cq: *mut chunkqueue);
    fn log_error(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        fmt: *const libc::c_char,
        _: ...
    );
    fn http_header_response_get(
        r: *const request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
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
pub type uintmax_t = __uintmax_t;
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
pub type handler_t = libc::c_uint;
pub const HANDLER_ERROR: handler_t = 4;
pub const HANDLER_WAIT_FOR_EVENT: handler_t = 3;
pub const HANDLER_COMEBACK: handler_t = 2;
pub const HANDLER_FINISHED: handler_t = 1;
pub const HANDLER_GO_ON: handler_t = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_string {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
    pub ext: libc::c_int,
    pub value: buffer,
}
pub const HTTP_HEADER_CONTENT_TYPE: http_header_e = 18;
pub const HTTP_HEADER_OTHER: http_header_e = 0;
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
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
#[inline]
unsafe extern "C" fn buffer_truncate(mut b: *mut buffer, mut len: uint32_t) {
    *((*b).ptr).offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    (*b).used = len.wrapping_add(1 as libc::c_int as libc::c_uint);
}
#[inline]
unsafe extern "C" fn buffer_copy_buffer(mut b: *mut buffer, mut src: *const buffer) {
    buffer_copy_string_len(b, (*src).ptr, buffer_clen(src) as size_t);
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
unsafe extern "C" fn buffer_is_blank(mut b: *const buffer) -> libc::c_int {
    return ((*b).used < 2 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn buffer_is_unset(mut b: *const buffer) -> libc::c_int {
    return (0 as libc::c_int as libc::c_uint == (*b).used) as libc::c_int;
}
#[inline]
unsafe extern "C" fn light_isalpha(mut c: libc::c_int) -> libc::c_int {
    return ((c as uint32_t | 0x20 as libc::c_int as libc::c_uint)
        .wrapping_sub('a' as i32 as libc::c_uint)
        <= ('z' as i32 - 'a' as i32) as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn light_isdigit(mut c: libc::c_int) -> libc::c_int {
    return ((c as uint32_t).wrapping_sub('0' as i32 as libc::c_uint)
        <= ('9' as i32 - '0' as i32) as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn sock_addr_get_family(mut saddr: *const sock_addr) -> libc::c_int {
    return (*saddr).plain.sa_family as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn http_cgi_local_redir(r: *mut request_st) -> handler_t {
    let mut ulen: size_t = buffer_clen(&mut (*r).uri.path) as size_t;
    let mut vb: *const buffer = http_header_response_get(
        r,
        HTTP_HEADER_LOCATION,
        b"Location\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if !vb.is_null()
        && *((*vb).ptr).offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
        && (0 as libc::c_int != strncmp((*vb).ptr, (*r).uri.path.ptr, ulen)
            || *((*vb).ptr).offset(ulen as isize) as libc::c_int != '\u{0}' as i32
                && *((*vb).ptr).offset(ulen as isize) as libc::c_int != '/' as i32
                && *((*vb).ptr).offset(ulen as isize) as libc::c_int != '?' as i32)
        && 1 as libc::c_int as libc::c_uint == (*r).resp_headers.used
        && (*r).http_status >= 300 as libc::c_int
        && (*r).http_status < 400 as libc::c_int
    {
        (*r).loops_per_request += 1;
        if (*r).loops_per_request as libc::c_int > 5 as libc::c_int {
            log_error(
                (*r).conf.errh,
                b"src/http_cgi.c\0" as *const u8 as *const libc::c_char,
                58 as libc::c_int as libc::c_uint,
                b"too many internal loops while processing request: %s\0" as *const u8
                    as *const libc::c_char,
                (*r).target_orig.ptr,
            );
            (*r).http_status = 500 as libc::c_int;
            (*r).resp_body_started = 0 as libc::c_int as libc::c_char;
            (*r).handler_module = 0 as *const plugin;
            return HANDLER_FINISHED;
        }
        buffer_copy_buffer(&mut (*r).target, vb);
        if (*r).reqbody_length != 0 {
            if (*r).reqbody_length != (*r).reqbody_queue.bytes_in {
                (*r).keep_alive = 0 as libc::c_int as int8_t;
            }
            (*r).reqbody_length = 0 as libc::c_int as off_t;
            chunkqueue_reset(&mut (*r).reqbody_queue);
        }
        if (*r).http_status != 307 as libc::c_int
            && (*r).http_status != 308 as libc::c_int
        {
            (*r).http_method = HTTP_METHOD_GET;
        }
        return HANDLER_COMEBACK;
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn http_cgi_encode_varname(
    b: *mut buffer,
    s: *const libc::c_char,
    len: size_t,
    is_http_header: libc::c_int,
) {
    let p: *mut libc::c_char = buffer_string_prepare_copy(
        b,
        len.wrapping_add(5 as libc::c_int as libc::c_ulong),
    );
    let mut i: size_t = 0;
    let mut j: size_t = 0 as libc::c_int as size_t;
    if is_http_header != 0 {
        memcpy(
            p as *mut libc::c_void,
            b"HTTP_\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            5 as libc::c_int as libc::c_ulong,
        );
        j = 5 as libc::c_int as size_t;
    }
    i = 0 as libc::c_int as size_t;
    while i < len {
        let c: libc::c_uchar = *s.offset(i as isize) as libc::c_uchar;
        let fresh0 = j;
        j = j.wrapping_add(1);
        *p
            .offset(
                fresh0 as isize,
            ) = (if light_isalpha(c as libc::c_int) != 0 {
            c as libc::c_int & !(0x20 as libc::c_int)
        } else if light_isdigit(c as libc::c_int) != 0 {
            c as libc::c_int
        } else {
            '_' as i32
        }) as libc::c_char;
        i = i.wrapping_add(1);
    }
    buffer_truncate(b, j as uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn http_cgi_headers(
    r: *mut request_st,
    opts: *mut http_cgi_opts,
    mut cb: http_cgi_header_append_cb,
    mut vdata: *mut libc::c_void,
) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut len: uint32_t = 0;
    let tb: *mut buffer = (*r).tmp_buf;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: size_t = 0;
    let mut buf: [libc::c_char; 47] = [0; 47];
    if (*opts).authorizer == 0 {
        rc
            |= cb
                .expect(
                    "non-null function pointer",
                )(
                vdata,
                b"CONTENT_LENGTH\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                buf.as_mut_ptr(),
                li_itostrn(
                    buf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 47]>() as libc::c_ulong,
                    (*r).reqbody_length,
                ),
            );
    }
    n = buffer_clen(&mut (*r).uri.query) as size_t;
    rc
        |= cb
            .expect(
                "non-null function pointer",
            )(
            vdata,
            b"QUERY_STRING\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            if n != 0 {
                (*r).uri.query.ptr as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            n,
        );
    s = (*r).target_orig.ptr;
    n = buffer_clen(&mut (*r).target_orig) as size_t;
    len = if !((*opts).strip_request_uri).is_null() {
        buffer_clen((*opts).strip_request_uri)
    } else {
        0 as libc::c_int as libc::c_uint
    };
    if len != 0 {
        if n < len as libc::c_ulong
            || 0 as libc::c_int
                != memcmp(
                    s as *const libc::c_void,
                    (*(*opts).strip_request_uri).ptr as *const libc::c_void,
                    len as libc::c_ulong,
                ) || *s.offset(len as isize) as libc::c_int != '/' as i32
        {
            len = 0 as libc::c_int as uint32_t;
        }
    }
    rc
        |= cb
            .expect(
                "non-null function pointer",
            )(
            vdata,
            b"REQUEST_URI\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            s.offset(len as isize),
            n.wrapping_sub(len as libc::c_ulong),
        );
    if buffer_is_equal(&mut (*r).target, &mut (*r).target_orig) == 0 {
        rc
            |= cb
                .expect(
                    "non-null function pointer",
                )(
                vdata,
                b"REDIRECT_URI\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                (*r).target.ptr,
                buffer_clen(&mut (*r).target) as size_t,
            );
    }
    if 0 as libc::c_int == (*r).error_handler_saved_status {
        rc
            |= cb
                .expect(
                    "non-null function pointer",
                )(
                vdata,
                b"REDIRECT_STATUS\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                b"200\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
    }
    if (*opts).authorizer == 0 {
        rc
            |= cb
                .expect(
                    "non-null function pointer",
                )(
                vdata,
                b"SCRIPT_NAME\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                (*r).uri.path.ptr,
                buffer_clen(&mut (*r).uri.path) as size_t,
            );
        if buffer_is_blank(&mut (*r).pathinfo) == 0 {
            rc
                |= cb
                    .expect(
                        "non-null function pointer",
                    )(
                    vdata,
                    b"PATH_INFO\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    (*r).pathinfo.ptr,
                    buffer_clen(&mut (*r).pathinfo) as size_t,
                );
            let bd: *const buffer = if !((*opts).docroot).is_null() {
                (*opts).docroot
            } else {
                &mut (*r).physical.basedir as *mut buffer as *const buffer
            };
            buffer_copy_path_len2(
                tb,
                (*bd).ptr,
                buffer_clen(bd) as size_t,
                (*r).pathinfo.ptr,
                buffer_clen(&mut (*r).pathinfo) as size_t,
            );
            rc
                |= cb
                    .expect(
                        "non-null function pointer",
                    )(
                    vdata,
                    b"PATH_TRANSLATED\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    (*tb).ptr,
                    buffer_clen(tb) as size_t,
                );
        }
    }
    if !((*opts).docroot).is_null() {
        buffer_copy_path_len2(
            tb,
            (*(*opts).docroot).ptr,
            buffer_clen((*opts).docroot) as size_t,
            (*r).uri.path.ptr,
            buffer_clen(&mut (*r).uri.path) as size_t,
        );
        rc
            |= cb
                .expect(
                    "non-null function pointer",
                )(
                vdata,
                b"SCRIPT_FILENAME\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                (*tb).ptr,
                buffer_clen(tb) as size_t,
            );
        rc
            |= cb
                .expect(
                    "non-null function pointer",
                )(
                vdata,
                b"DOCUMENT_ROOT\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                (*(*opts).docroot).ptr,
                buffer_clen((*opts).docroot) as size_t,
            );
    } else {
        if (*opts).break_scriptfilename_for_php != 0 {
            buffer_copy_path_len2(
                tb,
                (*r).physical.path.ptr,
                buffer_clen(&mut (*r).physical.path) as size_t,
                (*r).pathinfo.ptr,
                buffer_clen(&mut (*r).pathinfo) as size_t,
            );
            rc
                |= cb
                    .expect(
                        "non-null function pointer",
                    )(
                    vdata,
                    b"SCRIPT_FILENAME\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    (*tb).ptr,
                    buffer_clen(tb) as size_t,
                );
        } else {
            rc
                |= cb
                    .expect(
                        "non-null function pointer",
                    )(
                    vdata,
                    b"SCRIPT_FILENAME\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    (*r).physical.path.ptr,
                    buffer_clen(&mut (*r).physical.path) as size_t,
                );
        }
        rc
            |= cb
                .expect(
                    "non-null function pointer",
                )(
                vdata,
                b"DOCUMENT_ROOT\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                (*r).physical.basedir.ptr,
                buffer_clen(&mut (*r).physical.basedir) as size_t,
            );
    }
    let m: *const buffer = http_method_buf((*r).http_method);
    rc
        |= cb
            .expect(
                "non-null function pointer",
            )(
            vdata,
            b"REQUEST_METHOD\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            (*m).ptr,
            buffer_clen(m) as size_t,
        );
    s = get_http_version_name((*r).http_version as libc::c_int);
    if s.is_null() {
        ck_assert_failed(
            b"src/http_cgi.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int as libc::c_uint,
            b"s\0" as *const u8 as *const libc::c_char,
        );
    }
    rc
        |= cb
            .expect(
                "non-null function pointer",
            )(
            vdata,
            b"SERVER_PROTOCOL\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            s,
            strlen(s),
        );
    if !((*r).conf.server_tag).is_null() {
        s = (*(*r).conf.server_tag).ptr;
        n = buffer_clen((*r).conf.server_tag) as size_t;
    } else {
        s = b"\0" as *const u8 as *const libc::c_char;
        n = 0 as libc::c_int as size_t;
    }
    rc
        |= cb
            .expect(
                "non-null function pointer",
            )(
            vdata,
            b"SERVER_SOFTWARE\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            s,
            n,
        );
    rc
        |= cb
            .expect(
                "non-null function pointer",
            )(
            vdata,
            b"GATEWAY_INTERFACE\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            b"CGI/1.1\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    rc
        |= cb
            .expect(
                "non-null function pointer",
            )(
            vdata,
            b"REQUEST_SCHEME\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            (*r).uri.scheme.ptr,
            buffer_clen(&mut (*r).uri.scheme) as size_t,
        );
    if buffer_eq_slen(
        &mut (*r).uri.scheme,
        b"https\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    ) != 0
    {
        rc
            |= cb
                .expect(
                    "non-null function pointer",
                )(
                vdata,
                b"HTTPS\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                b"on\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
    }
    let con: *const connection = (*r).con;
    let srv_sock: *const server_socket = (*con).srv_socket;
    let tlen: size_t = buffer_clen((*srv_sock).srv_token) as size_t;
    n = (*srv_sock).srv_token_colon as size_t;
    if n < tlen {
        s = ((*(*srv_sock).srv_token).ptr)
            .offset(n as isize)
            .offset(1 as libc::c_int as isize);
        n = tlen.wrapping_sub(n.wrapping_add(1 as libc::c_int as libc::c_ulong));
    } else {
        s = b"0\0" as *const u8 as *const libc::c_char;
        n = 1 as libc::c_int as size_t;
    }
    rc
        |= cb
            .expect(
                "non-null function pointer",
            )(
            vdata,
            b"SERVER_PORT\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            s,
            n,
        );
    n = 0 as libc::c_int as size_t;
    match sock_addr_get_family(&(*srv_sock).addr) {
        2 | 10 => {
            if sock_addr_is_addr_wildcard(&(*srv_sock).addr) != 0 {
                let mut addrbuf: sock_addr = sock_addr {
                    ipv6: sockaddr_in6 {
                        sin6_family: 0,
                        sin6_port: 0,
                        sin6_flowinfo: 0,
                        sin6_addr: in6_addr {
                            __in6_u: C2RustUnnamed_0 {
                                __u6_addr8: [0; 16],
                            },
                        },
                        sin6_scope_id: 0,
                    },
                };
                let mut addrlen: socklen_t = ::std::mem::size_of::<sock_addr>()
                    as libc::c_ulong as socklen_t;
                if 0 as libc::c_int
                    == getsockname(
                        (*con).fd,
                        __SOCKADDR_ARG {
                            __sockaddr__: &mut addrbuf as *mut sock_addr as *mut sockaddr,
                        },
                        &mut addrlen,
                    )
                {
                    s = sock_addr_inet_ntop(
                        &mut addrbuf,
                        buf.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 47]>() as libc::c_ulong
                            as socklen_t,
                    );
                    if !s.is_null() {
                        n = strlen(s);
                    } else {
                        s = b"\0" as *const u8 as *const libc::c_char;
                    }
                } else {
                    s = b"\0" as *const u8 as *const libc::c_char;
                }
            } else {
                s = (*(*srv_sock).srv_token).ptr;
                n = (*srv_sock).srv_token_colon as size_t;
            }
        }
        _ => {
            s = b"\0" as *const u8 as *const libc::c_char;
        }
    }
    rc
        |= cb
            .expect(
                "non-null function pointer",
            )(
            vdata,
            b"SERVER_ADDR\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            s,
            n,
        );
    n = buffer_clen((*r).server_name) as size_t;
    if n != 0 {
        s = (*(*r).server_name).ptr;
        if *s.offset(0 as libc::c_int as isize) as libc::c_int == '[' as i32 {
            let mut colon: *const libc::c_char = strstr(
                s,
                b"]:\0" as *const u8 as *const libc::c_char,
            );
            if !colon.is_null() {
                n = colon.offset(1 as libc::c_int as isize).offset_from(s)
                    as libc::c_long as size_t;
            }
        } else {
            let mut colon_0: *const libc::c_char = strchr(s, ':' as i32);
            if !colon_0.is_null() {
                n = colon_0.offset_from(s) as libc::c_long as size_t;
            }
        }
    }
    rc
        |= cb
            .expect(
                "non-null function pointer",
            )(
            vdata,
            b"SERVER_NAME\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            s,
            n,
        );
    rc
        |= cb
            .expect(
                "non-null function pointer",
            )(
            vdata,
            b"REMOTE_ADDR\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            (*con).dst_addr_buf.ptr,
            buffer_clen(&(*con).dst_addr_buf) as size_t,
        );
    rc
        |= cb
            .expect(
                "non-null function pointer",
            )(
            vdata,
            b"REMOTE_PORT\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            buf.as_mut_ptr(),
            li_utostrn(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 47]>() as libc::c_ulong,
                sock_addr_get_port(&(*con).dst_addr) as uintmax_t,
            ),
        );
    n = 0 as libc::c_int as size_t;
    while n < (*r).rqst_headers.used as libc::c_ulong {
        let mut ds: *mut data_string = *((*r).rqst_headers.data).offset(n as isize)
            as *mut data_string;
        if buffer_is_blank(&mut (*ds).value) == 0 && buffer_is_unset(&mut (*ds).key) == 0
        {
            if !((*ds).ext == HTTP_HEADER_OTHER as libc::c_int
                && buffer_eq_icase_slen(
                    &mut (*ds).key,
                    b"Proxy\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                ) != 0)
            {
                if (*ds).ext == HTTP_HEADER_CONTENT_TYPE as libc::c_int {
                    buffer_copy_string_len(
                        tb,
                        b"CONTENT_TYPE\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    );
                } else {
                    http_cgi_encode_varname(
                        tb,
                        (*ds).key.ptr,
                        buffer_clen(&mut (*ds).key) as size_t,
                        1 as libc::c_int,
                    );
                }
                rc
                    |= cb
                        .expect(
                            "non-null function pointer",
                        )(
                        vdata,
                        (*tb).ptr,
                        buffer_clen(tb) as size_t,
                        (*ds).value.ptr,
                        buffer_clen(&mut (*ds).value) as size_t,
                    );
            }
        }
        n = n.wrapping_add(1);
    }
    ((*(*con).srv).request_env).expect("non-null function pointer")(r);
    n = 0 as libc::c_int as size_t;
    while n < (*r).env.used as libc::c_ulong {
        let mut ds_0: *mut data_string = *((*r).env.data).offset(n as isize)
            as *mut data_string;
        if buffer_is_unset(&mut (*ds_0).value) == 0
            && buffer_is_unset(&mut (*ds_0).key) == 0
        {
            http_cgi_encode_varname(
                tb,
                (*ds_0).key.ptr,
                buffer_clen(&mut (*ds_0).key) as size_t,
                0 as libc::c_int,
            );
            rc
                |= cb
                    .expect(
                        "non-null function pointer",
                    )(
                    vdata,
                    (*tb).ptr,
                    buffer_clen(tb) as size_t,
                    (*ds_0).value.ptr,
                    buffer_clen(&mut (*ds_0).value) as size_t,
                );
        }
        n = n.wrapping_add(1);
    }
    return rc;
}
