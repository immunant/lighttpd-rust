use ::libc;
extern "C" {
    pub type fdnode_st;
    pub type stat_cache_entry;
    pub type pcre2_real_match_data_8;
    pub type h2con;
    pub type fdevents;
    pub type pcre2_real_match_context_8;
    pub type pcre2_real_code_8;
    pub type pcre2_real_general_context_8;
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
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_eq_icase_slen(
        b: *const buffer,
        s: *const libc::c_char,
        slen: size_t,
    ) -> libc::c_int;
    fn buffer_is_equal(a: *const buffer, b: *const buffer) -> libc::c_int;
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn array_is_vlist(a: *const array) -> libc::c_int;
    fn array_is_kvany(a: *const array) -> libc::c_int;
    fn array_is_kvarray(a: *const array) -> libc::c_int;
    fn array_is_kvstring(a: *const array) -> libc::c_int;
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
    fn http_method_buf(i: http_method_t) -> *const buffer;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sock_addr_is_addr_eq(
        saddr1: *const sock_addr,
        saddr2: *const sock_addr,
    ) -> libc::c_int;
    fn sock_addr_is_addr_eq_bits(
        a: *const sock_addr,
        b: *const sock_addr,
        bits: libc::c_int,
    ) -> libc::c_int;
    fn log_error(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        fmt: *const libc::c_char,
        _: ...
    );
    fn http_header_request_get(
        r: *const request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn pcre2_match_8(
        _: *const pcre2_code_8,
        _: PCRE2_SPTR8,
        _: size_t,
        _: size_t,
        _: uint32_t,
        _: *mut pcre2_match_data_8,
        _: *mut pcre2_match_context_8,
    ) -> libc::c_int;
    fn pcre2_get_ovector_pointer_8(_: *mut pcre2_match_data_8) -> *mut size_t;
    fn pcre2_match_data_create_from_pattern_8(
        _: *const pcre2_code_8,
        _: *mut pcre2_general_context_8,
    ) -> *mut pcre2_match_data_8;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type uint_fast32_t = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;
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
pub type handler_t = libc::c_uint;
pub const HANDLER_ERROR: handler_t = 4;
pub const HANDLER_WAIT_FOR_EVENT: handler_t = 3;
pub const HANDLER_COMEBACK: handler_t = 2;
pub const HANDLER_FINISHED: handler_t = 1;
pub const HANDLER_GO_ON: handler_t = 0;
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
pub type config_cond_t = libc::c_uint;
pub const CONFIG_COND_ELSE: config_cond_t = 5;
pub const CONFIG_COND_NOMATCH: config_cond_t = 4;
pub const CONFIG_COND_NE: config_cond_t = 3;
pub const CONFIG_COND_MATCH: config_cond_t = 2;
pub const CONFIG_COND_EQ: config_cond_t = 1;
pub const CONFIG_COND_UNSET: config_cond_t = 0;
pub type comp_key_t = libc::c_uint;
pub const COMP_LAST_ELEMENT: comp_key_t = 13;
pub const COMP_HTTP_REQUEST_HEADER: comp_key_t = 12;
pub const COMP_HTTP_REQUEST_METHOD: comp_key_t = 11;
pub const COMP_HTTP_SCHEME: comp_key_t = 10;
pub const COMP_HTTP_QUERY_STRING: comp_key_t = 9;
pub const COMP_HTTP_REMOTE_IP: comp_key_t = 8;
pub const COMP_HTTP_COOKIE: comp_key_t = 7;
pub const COMP_HTTP_LANGUAGE: comp_key_t = 6;
pub const COMP_HTTP_USER_AGENT: comp_key_t = 5;
pub const COMP_HTTP_REFERER: comp_key_t = 4;
pub const COMP_HTTP_HOST: comp_key_t = 3;
pub const COMP_HTTP_URL: comp_key_t = 2;
pub const COMP_SERVER_SOCKET: comp_key_t = 1;
pub const COMP_UNSET: comp_key_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_cond_info {
    pub comp: comp_key_t,
    pub cond: config_cond_t,
    pub string: *const buffer,
    pub comp_key: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_config {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
    pub context_ndx: libc::c_int,
    pub comp: comp_key_t,
    pub cond: config_cond_t,
    pub parent: *mut data_config,
    pub prev: *mut data_config,
    pub next: *mut data_config,
    pub string: buffer,
    pub code: *mut libc::c_void,
    pub match_data: *mut pcre2_real_match_data_8,
    pub capture_idx: libc::c_int,
    pub ext: libc::c_int,
    pub comp_tag: buffer,
    pub comp_key: *const libc::c_char,
    pub children: vector_config_weak,
    pub value: *mut array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vector_config_weak {
    pub data: *mut *mut data_config,
    pub used: size_t,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub data: *const *const data_config,
    pub used: uint32_t,
}
pub const COND_RESULT_UNSET: cond_result_t = 0;
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
pub type C2RustUnnamed_6 = libc::c_uint;
pub const T_CONFIG_SCOPE_CONNECTION: C2RustUnnamed_6 = 2;
pub const T_CONFIG_SCOPE_SERVER: C2RustUnnamed_6 = 1;
pub const T_CONFIG_SCOPE_UNSET: C2RustUnnamed_6 = 0;
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
pub struct plugin_data_base {
    pub id: libc::c_int,
    pub nconfig: libc::c_int,
    pub cvlist: *mut config_plugin_value_t,
    pub self_0: *mut plugin,
}
pub type cond_result_t = libc::c_uint;
pub const COND_RESULT_TRUE: cond_result_t = 3;
pub const COND_RESULT_FALSE: cond_result_t = 2;
pub const COND_RESULT_SKIP: cond_result_t = 1;
pub type pcre2_match_context_8 = pcre2_real_match_context_8;
pub type pcre2_match_data_8 = pcre2_real_match_data_8;
pub type PCRE2_SPTR8 = *const PCRE2_UCHAR8;
pub type PCRE2_UCHAR8 = uint8_t;
pub type pcre2_code_8 = pcre2_real_code_8;
pub type pcre2_general_context_8 = pcre2_real_general_context_8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct const_char_buffer {
    pub ptr: *const libc::c_char,
    pub used: uint32_t,
    pub size: uint32_t,
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
unsafe extern "C" fn array_set_key_value(
    a: *mut array,
    k: *const libc::c_char,
    klen: uint32_t,
    v: *const libc::c_char,
    vlen: uint32_t,
) {
    buffer_copy_string_len(array_get_buf_ptr(a, k, klen), v, vlen as size_t);
}
static mut config_reference: C2RustUnnamed_5 = C2RustUnnamed_5 {
    data: 0 as *const *const data_config,
    used: 0,
};
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_get_config_cond_info(
    cfginfo: *mut config_cond_info,
    mut idx: uint32_t,
) {
    let dc: *const data_config = *(config_reference.data).offset(idx as isize)
        as *mut data_config;
    (*cfginfo).comp = (*dc).comp;
    (*cfginfo).cond = (*dc).cond;
    (*cfginfo).string = &(*dc).string;
    (*cfginfo).comp_key = (*dc).comp_key;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_capture(
    mut srv: *mut server,
    mut idx: libc::c_int,
) -> libc::c_int {
    let dc: *mut data_config = *(config_reference.data).offset(idx as isize)
        as *mut data_config;
    return if (*dc).capture_idx != 0 {
        (*dc).capture_idx
    } else {
        (*srv).config_captures += 1;
        (*dc).capture_idx = (*srv).config_captures;
        (*dc).capture_idx
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_feature_bool(
    mut srv: *const server,
    mut feature: *const libc::c_char,
    mut default_value: libc::c_int,
) -> libc::c_int {
    return if !((*srv).srvconf.feature_flags).is_null() {
        config_plugin_value_tobool(
            array_get_element_klen(
                (*srv).srvconf.feature_flags,
                feature,
                strlen(feature) as uint32_t,
            ),
            default_value,
        )
    } else {
        default_value
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_feature_int(
    mut srv: *const server,
    mut feature: *const libc::c_char,
    mut default_value: int32_t,
) -> int32_t {
    return if !((*srv).srvconf.feature_flags).is_null() {
        config_plugin_value_to_int32(
            array_get_element_klen(
                (*srv).srvconf.feature_flags,
                feature,
                strlen(feature) as uint32_t,
            ),
            default_value,
        )
    } else {
        default_value
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_plugin_value_tobool(
    mut du: *const data_unset,
    mut default_value: libc::c_int,
) -> libc::c_int {
    if du.is_null() {
        return default_value;
    }
    if (*du).type_0 as libc::c_uint == TYPE_STRING as libc::c_int as libc::c_uint {
        let mut b: *const buffer = &(*(du as *const data_string)).value;
        if buffer_eq_icase_slen(
            b,
            b"enable\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
            || buffer_eq_icase_slen(
                b,
                b"enabled\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            || buffer_eq_icase_slen(
                b,
                b"true\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            || buffer_eq_icase_slen(
                b,
                b"1\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
        {
            return 1 as libc::c_int
        } else if buffer_eq_icase_slen(
                b,
                b"disable\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
                || buffer_eq_icase_slen(
                    b,
                    b"disabled\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                ) != 0
                || buffer_eq_icase_slen(
                    b,
                    b"false\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                ) != 0
                || buffer_eq_icase_slen(
                    b,
                    b"0\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                ) != 0
            {
            return 0 as libc::c_int
        } else {
            return default_value
        }
    } else if (*du).type_0 as libc::c_uint == TYPE_INTEGER as libc::c_int as libc::c_uint
        {
        return (0 as libc::c_int != (*(du as *const data_integer)).value) as libc::c_int
    } else {
        return default_value
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_plugin_value_to_int32(
    mut du: *const data_unset,
    mut default_value: int32_t,
) -> int32_t {
    if du.is_null() {
        return default_value;
    }
    if (*du).type_0 as libc::c_uint == TYPE_STRING as libc::c_int as libc::c_uint {
        let b: *const buffer = &(*(du as *const data_string)).value;
        let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut v: libc::c_long = strtol((*b).ptr, &mut err, 10 as libc::c_int);
        return if *err as libc::c_int == '\u{0}' as i32 && err != (*b).ptr
            && (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_long <= v
            && v <= 2147483647 as libc::c_int as libc::c_long
        {
            v as int32_t
        } else {
            default_value
        };
    } else if (*du).type_0 as libc::c_uint == TYPE_INTEGER as libc::c_int as libc::c_uint
        {
        return (*(du as *const data_integer)).value
    } else {
        return default_value
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_plugin_values_init_block(
    srv: *mut server,
    ca: *const array,
    cpk: *const config_plugin_keys_t,
    mname: *const libc::c_char,
    mut cpv: *mut config_plugin_value_t,
) -> libc::c_int {
    let mut rc: libc::c_int = 1 as libc::c_int;
    let mut current_block_37: u64;
    let mut i: libc::c_int = 0 as libc::c_int;
    while (*cpk.offset(i as isize)).ktype as libc::c_int != T_CONFIG_UNSET as libc::c_int
    {
        let du: *const data_unset = array_get_element_klen(
            ca,
            (*cpk.offset(i as isize)).k,
            (*cpk.offset(i as isize)).klen as uint32_t,
        );
        if !du.is_null() {
            (*cpv).k_id = i;
            (*cpv).vtype = (*cpk.offset(i as isize)).ktype as config_values_type_t;
            match (*cpk.offset(i as isize)).ktype as libc::c_int {
                5 | 6 | 7 | 8 | 9 => {
                    current_block_37 = 1591380387575923842;
                    match current_block_37 {
                        1868291631715963762 => {
                            let mut v_1: libc::c_int = config_plugin_value_tobool(
                                du,
                                -(1 as libc::c_int),
                            );
                            if -(1 as libc::c_int) == v_1 {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    259 as libc::c_int as libc::c_uint,
                                    b"ERROR: unexpected type for key: %s (string) \"(enable|disable)\"\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            } else {
                                (*cpv).v.u = v_1 as libc::c_uint;
                                current_block_37 = 2472048668343472511;
                            }
                        }
                        9464725857721511177 => {
                            match (*du).type_0 as libc::c_uint {
                                2 => {
                                    (*cpv)
                                        .v
                                        .u = (*(du as *const data_integer)).value as libc::c_uint;
                                    current_block_37 = 2472048668343472511;
                                }
                                0 => {
                                    let v_0: *const libc::c_char = (*(du as *const data_string))
                                        .value
                                        .ptr;
                                    if !v_0.is_null() && *v_0 as libc::c_int != 0 {
                                        let mut e_0: *mut libc::c_char = 0 as *mut libc::c_char;
                                        let mut l_0: libc::c_long = strtol(
                                            v_0,
                                            &mut e_0,
                                            10 as libc::c_int,
                                        );
                                        if e_0 != v_0 as *mut libc::c_char && *e_0 == 0
                                            && l_0 >= 0 as libc::c_int as libc::c_long
                                        {
                                            (*cpv).v.shrt = l_0 as libc::c_uint as libc::c_ushort;
                                            current_block_37 = 2472048668343472511;
                                        } else {
                                            current_block_37 = 4888910987971495881;
                                        }
                                    } else {
                                        current_block_37 = 4888910987971495881;
                                    }
                                    match current_block_37 {
                                        2472048668343472511 => {}
                                        _ => {
                                            log_error(
                                                (*srv).errh,
                                                b"src/configfile-glue.c\0" as *const u8
                                                    as *const libc::c_char,
                                                242 as libc::c_int as libc::c_uint,
                                                b"got a string but expected an integer: %s %s\0"
                                                    as *const u8 as *const libc::c_char,
                                                (*cpk.offset(i as isize)).k,
                                                v_0,
                                            );
                                            rc = 0 as libc::c_int;
                                            current_block_37 = 16668937799742929182;
                                        }
                                    }
                                }
                                _ => {
                                    log_error(
                                        (*srv).errh,
                                        b"src/configfile-glue.c\0" as *const u8
                                            as *const libc::c_char,
                                        248 as libc::c_int as libc::c_uint,
                                        b"unexpected type for key: %s %d expected an integer, range 0 ... 4294967295\0"
                                            as *const u8 as *const libc::c_char,
                                        (*cpk.offset(i as isize)).k,
                                        (*du).type_0 as libc::c_uint,
                                    );
                                    rc = 0 as libc::c_int;
                                    current_block_37 = 16668937799742929182;
                                }
                            }
                        }
                        9441948780229972447 => {
                            match (*du).type_0 as libc::c_uint {
                                2 => {
                                    (*cpv)
                                        .v
                                        .shrt = (*(du as *const data_integer)).value
                                        as libc::c_ushort;
                                    current_block_37 = 2472048668343472511;
                                }
                                0 => {
                                    let v: *const libc::c_char = (*(du as *const data_string))
                                        .value
                                        .ptr;
                                    if !v.is_null() && *v as libc::c_int != 0 {
                                        let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
                                        let mut l: libc::c_long = strtol(
                                            v,
                                            &mut e,
                                            10 as libc::c_int,
                                        );
                                        if e != v as *mut libc::c_char && *e == 0
                                            && l >= 0 as libc::c_int as libc::c_long
                                            && l <= 65535 as libc::c_int as libc::c_long
                                        {
                                            (*cpv).v.shrt = l as libc::c_ushort;
                                            current_block_37 = 2472048668343472511;
                                        } else {
                                            current_block_37 = 10150597327160359210;
                                        }
                                    } else {
                                        current_block_37 = 10150597327160359210;
                                    }
                                    match current_block_37 {
                                        2472048668343472511 => {}
                                        _ => {
                                            log_error(
                                                (*srv).errh,
                                                b"src/configfile-glue.c\0" as *const u8
                                                    as *const libc::c_char,
                                                214 as libc::c_int as libc::c_uint,
                                                b"got a string but expected a short: %s %s\0" as *const u8
                                                    as *const libc::c_char,
                                                (*cpk.offset(i as isize)).k,
                                                v,
                                            );
                                            rc = 0 as libc::c_int;
                                            current_block_37 = 16668937799742929182;
                                        }
                                    }
                                }
                                _ => {
                                    log_error(
                                        (*srv).errh,
                                        b"src/configfile-glue.c\0" as *const u8
                                            as *const libc::c_char,
                                        220 as libc::c_int as libc::c_uint,
                                        b"unexpected type for key: %s %d expected a short integer, range 0 ... 65535\0"
                                            as *const u8 as *const libc::c_char,
                                        (*cpk.offset(i as isize)).k,
                                        (*du).type_0 as libc::c_uint,
                                    );
                                    rc = 0 as libc::c_int;
                                    current_block_37 = 16668937799742929182;
                                }
                            }
                        }
                        10889058540383362684 => {
                            if (*du).type_0 as libc::c_uint
                                == TYPE_STRING as libc::c_int as libc::c_uint
                            {
                                (*cpv).v.b = &(*(du as *const data_string)).value;
                                current_block_37 = 2472048668343472511;
                            } else {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    186 as libc::c_int as libc::c_uint,
                                    b"%s should have been a string like ... = \"...\"\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            }
                        }
                        1591380387575923842 => {
                            if (*du).type_0 as libc::c_uint
                                == TYPE_ARRAY as libc::c_int as libc::c_uint
                            {
                                (*cpv).v.a = &(*(du as *const data_array)).value;
                                match (*cpk.offset(i as isize)).ktype as libc::c_int {
                                    6 => {
                                        current_block_37 = 10144490576563699853;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    7 => {
                                        current_block_37 = 2253044032609351678;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    8 => {
                                        current_block_37 = 4577159841354843635;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    9 => {
                                        current_block_37 = 6585473732656942518;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        current_block_37 = 2472048668343472511;
                                    }
                                }
                            } else {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    129 as libc::c_int as libc::c_uint,
                                    b"%s should have been a list like %s = ( \"...\" )\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            }
                        }
                        4945542941617773199 => {
                            log_error(
                                (*srv).errh,
                                b"src/configfile-glue.c\0" as *const u8
                                    as *const libc::c_char,
                                272 as libc::c_int as libc::c_uint,
                                b"ERROR: found unsupported key: %s (%s)\0" as *const u8
                                    as *const libc::c_char,
                                (*cpk.offset(i as isize)).k,
                                mname,
                            );
                            (*srv)
                                .srvconf
                                .config_unsupported = 1 as libc::c_int as libc::c_uchar;
                            current_block_37 = 16668937799742929182;
                        }
                        7372943784920105432 => {
                            log_error(
                                (*srv).errh,
                                b"src/configfile-glue.c\0" as *const u8
                                    as *const libc::c_char,
                                277 as libc::c_int as libc::c_uint,
                                b"ERROR: found deprecated key: %s (%s)\0" as *const u8
                                    as *const libc::c_char,
                                (*cpk.offset(i as isize)).k,
                                mname,
                            );
                            (*srv)
                                .srvconf
                                .config_deprecated = 1 as libc::c_int as libc::c_uchar;
                            current_block_37 = 16668937799742929182;
                        }
                        _ => {}
                    }
                    match current_block_37 {
                        16668937799742929182 => {}
                        _ => {
                            cpv = cpv.offset(1);
                        }
                    }
                }
                1 => {
                    current_block_37 = 10889058540383362684;
                    match current_block_37 {
                        1868291631715963762 => {
                            let mut v_1: libc::c_int = config_plugin_value_tobool(
                                du,
                                -(1 as libc::c_int),
                            );
                            if -(1 as libc::c_int) == v_1 {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    259 as libc::c_int as libc::c_uint,
                                    b"ERROR: unexpected type for key: %s (string) \"(enable|disable)\"\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            } else {
                                (*cpv).v.u = v_1 as libc::c_uint;
                                current_block_37 = 2472048668343472511;
                            }
                        }
                        9464725857721511177 => {
                            match (*du).type_0 as libc::c_uint {
                                2 => {
                                    (*cpv)
                                        .v
                                        .u = (*(du as *const data_integer)).value as libc::c_uint;
                                    current_block_37 = 2472048668343472511;
                                }
                                0 => {
                                    let v_0: *const libc::c_char = (*(du as *const data_string))
                                        .value
                                        .ptr;
                                    if !v_0.is_null() && *v_0 as libc::c_int != 0 {
                                        let mut e_0: *mut libc::c_char = 0 as *mut libc::c_char;
                                        let mut l_0: libc::c_long = strtol(
                                            v_0,
                                            &mut e_0,
                                            10 as libc::c_int,
                                        );
                                        if e_0 != v_0 as *mut libc::c_char && *e_0 == 0
                                            && l_0 >= 0 as libc::c_int as libc::c_long
                                        {
                                            (*cpv).v.shrt = l_0 as libc::c_uint as libc::c_ushort;
                                            current_block_37 = 2472048668343472511;
                                        } else {
                                            current_block_37 = 4888910987971495881;
                                        }
                                    } else {
                                        current_block_37 = 4888910987971495881;
                                    }
                                    match current_block_37 {
                                        2472048668343472511 => {}
                                        _ => {
                                            log_error(
                                                (*srv).errh,
                                                b"src/configfile-glue.c\0" as *const u8
                                                    as *const libc::c_char,
                                                242 as libc::c_int as libc::c_uint,
                                                b"got a string but expected an integer: %s %s\0"
                                                    as *const u8 as *const libc::c_char,
                                                (*cpk.offset(i as isize)).k,
                                                v_0,
                                            );
                                            rc = 0 as libc::c_int;
                                            current_block_37 = 16668937799742929182;
                                        }
                                    }
                                }
                                _ => {
                                    log_error(
                                        (*srv).errh,
                                        b"src/configfile-glue.c\0" as *const u8
                                            as *const libc::c_char,
                                        248 as libc::c_int as libc::c_uint,
                                        b"unexpected type for key: %s %d expected an integer, range 0 ... 4294967295\0"
                                            as *const u8 as *const libc::c_char,
                                        (*cpk.offset(i as isize)).k,
                                        (*du).type_0 as libc::c_uint,
                                    );
                                    rc = 0 as libc::c_int;
                                    current_block_37 = 16668937799742929182;
                                }
                            }
                        }
                        9441948780229972447 => {
                            match (*du).type_0 as libc::c_uint {
                                2 => {
                                    (*cpv)
                                        .v
                                        .shrt = (*(du as *const data_integer)).value
                                        as libc::c_ushort;
                                    current_block_37 = 2472048668343472511;
                                }
                                0 => {
                                    let v: *const libc::c_char = (*(du as *const data_string))
                                        .value
                                        .ptr;
                                    if !v.is_null() && *v as libc::c_int != 0 {
                                        let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
                                        let mut l: libc::c_long = strtol(
                                            v,
                                            &mut e,
                                            10 as libc::c_int,
                                        );
                                        if e != v as *mut libc::c_char && *e == 0
                                            && l >= 0 as libc::c_int as libc::c_long
                                            && l <= 65535 as libc::c_int as libc::c_long
                                        {
                                            (*cpv).v.shrt = l as libc::c_ushort;
                                            current_block_37 = 2472048668343472511;
                                        } else {
                                            current_block_37 = 10150597327160359210;
                                        }
                                    } else {
                                        current_block_37 = 10150597327160359210;
                                    }
                                    match current_block_37 {
                                        2472048668343472511 => {}
                                        _ => {
                                            log_error(
                                                (*srv).errh,
                                                b"src/configfile-glue.c\0" as *const u8
                                                    as *const libc::c_char,
                                                214 as libc::c_int as libc::c_uint,
                                                b"got a string but expected a short: %s %s\0" as *const u8
                                                    as *const libc::c_char,
                                                (*cpk.offset(i as isize)).k,
                                                v,
                                            );
                                            rc = 0 as libc::c_int;
                                            current_block_37 = 16668937799742929182;
                                        }
                                    }
                                }
                                _ => {
                                    log_error(
                                        (*srv).errh,
                                        b"src/configfile-glue.c\0" as *const u8
                                            as *const libc::c_char,
                                        220 as libc::c_int as libc::c_uint,
                                        b"unexpected type for key: %s %d expected a short integer, range 0 ... 65535\0"
                                            as *const u8 as *const libc::c_char,
                                        (*cpk.offset(i as isize)).k,
                                        (*du).type_0 as libc::c_uint,
                                    );
                                    rc = 0 as libc::c_int;
                                    current_block_37 = 16668937799742929182;
                                }
                            }
                        }
                        10889058540383362684 => {
                            if (*du).type_0 as libc::c_uint
                                == TYPE_STRING as libc::c_int as libc::c_uint
                            {
                                (*cpv).v.b = &(*(du as *const data_string)).value;
                                current_block_37 = 2472048668343472511;
                            } else {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    186 as libc::c_int as libc::c_uint,
                                    b"%s should have been a string like ... = \"...\"\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            }
                        }
                        1591380387575923842 => {
                            if (*du).type_0 as libc::c_uint
                                == TYPE_ARRAY as libc::c_int as libc::c_uint
                            {
                                (*cpv).v.a = &(*(du as *const data_array)).value;
                                match (*cpk.offset(i as isize)).ktype as libc::c_int {
                                    6 => {
                                        current_block_37 = 10144490576563699853;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    7 => {
                                        current_block_37 = 2253044032609351678;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    8 => {
                                        current_block_37 = 4577159841354843635;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    9 => {
                                        current_block_37 = 6585473732656942518;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        current_block_37 = 2472048668343472511;
                                    }
                                }
                            } else {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    129 as libc::c_int as libc::c_uint,
                                    b"%s should have been a list like %s = ( \"...\" )\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            }
                        }
                        4945542941617773199 => {
                            log_error(
                                (*srv).errh,
                                b"src/configfile-glue.c\0" as *const u8
                                    as *const libc::c_char,
                                272 as libc::c_int as libc::c_uint,
                                b"ERROR: found unsupported key: %s (%s)\0" as *const u8
                                    as *const libc::c_char,
                                (*cpk.offset(i as isize)).k,
                                mname,
                            );
                            (*srv)
                                .srvconf
                                .config_unsupported = 1 as libc::c_int as libc::c_uchar;
                            current_block_37 = 16668937799742929182;
                        }
                        7372943784920105432 => {
                            log_error(
                                (*srv).errh,
                                b"src/configfile-glue.c\0" as *const u8
                                    as *const libc::c_char,
                                277 as libc::c_int as libc::c_uint,
                                b"ERROR: found deprecated key: %s (%s)\0" as *const u8
                                    as *const libc::c_char,
                                (*cpk.offset(i as isize)).k,
                                mname,
                            );
                            (*srv)
                                .srvconf
                                .config_deprecated = 1 as libc::c_int as libc::c_uchar;
                            current_block_37 = 16668937799742929182;
                        }
                        _ => {}
                    }
                    match current_block_37 {
                        16668937799742929182 => {}
                        _ => {
                            cpv = cpv.offset(1);
                        }
                    }
                }
                2 => {
                    current_block_37 = 9441948780229972447;
                    match current_block_37 {
                        1868291631715963762 => {
                            let mut v_1: libc::c_int = config_plugin_value_tobool(
                                du,
                                -(1 as libc::c_int),
                            );
                            if -(1 as libc::c_int) == v_1 {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    259 as libc::c_int as libc::c_uint,
                                    b"ERROR: unexpected type for key: %s (string) \"(enable|disable)\"\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            } else {
                                (*cpv).v.u = v_1 as libc::c_uint;
                                current_block_37 = 2472048668343472511;
                            }
                        }
                        9464725857721511177 => {
                            match (*du).type_0 as libc::c_uint {
                                2 => {
                                    (*cpv)
                                        .v
                                        .u = (*(du as *const data_integer)).value as libc::c_uint;
                                    current_block_37 = 2472048668343472511;
                                }
                                0 => {
                                    let v_0: *const libc::c_char = (*(du as *const data_string))
                                        .value
                                        .ptr;
                                    if !v_0.is_null() && *v_0 as libc::c_int != 0 {
                                        let mut e_0: *mut libc::c_char = 0 as *mut libc::c_char;
                                        let mut l_0: libc::c_long = strtol(
                                            v_0,
                                            &mut e_0,
                                            10 as libc::c_int,
                                        );
                                        if e_0 != v_0 as *mut libc::c_char && *e_0 == 0
                                            && l_0 >= 0 as libc::c_int as libc::c_long
                                        {
                                            (*cpv).v.shrt = l_0 as libc::c_uint as libc::c_ushort;
                                            current_block_37 = 2472048668343472511;
                                        } else {
                                            current_block_37 = 4888910987971495881;
                                        }
                                    } else {
                                        current_block_37 = 4888910987971495881;
                                    }
                                    match current_block_37 {
                                        2472048668343472511 => {}
                                        _ => {
                                            log_error(
                                                (*srv).errh,
                                                b"src/configfile-glue.c\0" as *const u8
                                                    as *const libc::c_char,
                                                242 as libc::c_int as libc::c_uint,
                                                b"got a string but expected an integer: %s %s\0"
                                                    as *const u8 as *const libc::c_char,
                                                (*cpk.offset(i as isize)).k,
                                                v_0,
                                            );
                                            rc = 0 as libc::c_int;
                                            current_block_37 = 16668937799742929182;
                                        }
                                    }
                                }
                                _ => {
                                    log_error(
                                        (*srv).errh,
                                        b"src/configfile-glue.c\0" as *const u8
                                            as *const libc::c_char,
                                        248 as libc::c_int as libc::c_uint,
                                        b"unexpected type for key: %s %d expected an integer, range 0 ... 4294967295\0"
                                            as *const u8 as *const libc::c_char,
                                        (*cpk.offset(i as isize)).k,
                                        (*du).type_0 as libc::c_uint,
                                    );
                                    rc = 0 as libc::c_int;
                                    current_block_37 = 16668937799742929182;
                                }
                            }
                        }
                        9441948780229972447 => {
                            match (*du).type_0 as libc::c_uint {
                                2 => {
                                    (*cpv)
                                        .v
                                        .shrt = (*(du as *const data_integer)).value
                                        as libc::c_ushort;
                                    current_block_37 = 2472048668343472511;
                                }
                                0 => {
                                    let v: *const libc::c_char = (*(du as *const data_string))
                                        .value
                                        .ptr;
                                    if !v.is_null() && *v as libc::c_int != 0 {
                                        let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
                                        let mut l: libc::c_long = strtol(
                                            v,
                                            &mut e,
                                            10 as libc::c_int,
                                        );
                                        if e != v as *mut libc::c_char && *e == 0
                                            && l >= 0 as libc::c_int as libc::c_long
                                            && l <= 65535 as libc::c_int as libc::c_long
                                        {
                                            (*cpv).v.shrt = l as libc::c_ushort;
                                            current_block_37 = 2472048668343472511;
                                        } else {
                                            current_block_37 = 10150597327160359210;
                                        }
                                    } else {
                                        current_block_37 = 10150597327160359210;
                                    }
                                    match current_block_37 {
                                        2472048668343472511 => {}
                                        _ => {
                                            log_error(
                                                (*srv).errh,
                                                b"src/configfile-glue.c\0" as *const u8
                                                    as *const libc::c_char,
                                                214 as libc::c_int as libc::c_uint,
                                                b"got a string but expected a short: %s %s\0" as *const u8
                                                    as *const libc::c_char,
                                                (*cpk.offset(i as isize)).k,
                                                v,
                                            );
                                            rc = 0 as libc::c_int;
                                            current_block_37 = 16668937799742929182;
                                        }
                                    }
                                }
                                _ => {
                                    log_error(
                                        (*srv).errh,
                                        b"src/configfile-glue.c\0" as *const u8
                                            as *const libc::c_char,
                                        220 as libc::c_int as libc::c_uint,
                                        b"unexpected type for key: %s %d expected a short integer, range 0 ... 65535\0"
                                            as *const u8 as *const libc::c_char,
                                        (*cpk.offset(i as isize)).k,
                                        (*du).type_0 as libc::c_uint,
                                    );
                                    rc = 0 as libc::c_int;
                                    current_block_37 = 16668937799742929182;
                                }
                            }
                        }
                        10889058540383362684 => {
                            if (*du).type_0 as libc::c_uint
                                == TYPE_STRING as libc::c_int as libc::c_uint
                            {
                                (*cpv).v.b = &(*(du as *const data_string)).value;
                                current_block_37 = 2472048668343472511;
                            } else {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    186 as libc::c_int as libc::c_uint,
                                    b"%s should have been a string like ... = \"...\"\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            }
                        }
                        1591380387575923842 => {
                            if (*du).type_0 as libc::c_uint
                                == TYPE_ARRAY as libc::c_int as libc::c_uint
                            {
                                (*cpv).v.a = &(*(du as *const data_array)).value;
                                match (*cpk.offset(i as isize)).ktype as libc::c_int {
                                    6 => {
                                        current_block_37 = 10144490576563699853;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    7 => {
                                        current_block_37 = 2253044032609351678;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    8 => {
                                        current_block_37 = 4577159841354843635;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    9 => {
                                        current_block_37 = 6585473732656942518;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        current_block_37 = 2472048668343472511;
                                    }
                                }
                            } else {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    129 as libc::c_int as libc::c_uint,
                                    b"%s should have been a list like %s = ( \"...\" )\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            }
                        }
                        4945542941617773199 => {
                            log_error(
                                (*srv).errh,
                                b"src/configfile-glue.c\0" as *const u8
                                    as *const libc::c_char,
                                272 as libc::c_int as libc::c_uint,
                                b"ERROR: found unsupported key: %s (%s)\0" as *const u8
                                    as *const libc::c_char,
                                (*cpk.offset(i as isize)).k,
                                mname,
                            );
                            (*srv)
                                .srvconf
                                .config_unsupported = 1 as libc::c_int as libc::c_uchar;
                            current_block_37 = 16668937799742929182;
                        }
                        7372943784920105432 => {
                            log_error(
                                (*srv).errh,
                                b"src/configfile-glue.c\0" as *const u8
                                    as *const libc::c_char,
                                277 as libc::c_int as libc::c_uint,
                                b"ERROR: found deprecated key: %s (%s)\0" as *const u8
                                    as *const libc::c_char,
                                (*cpk.offset(i as isize)).k,
                                mname,
                            );
                            (*srv)
                                .srvconf
                                .config_deprecated = 1 as libc::c_int as libc::c_uchar;
                            current_block_37 = 16668937799742929182;
                        }
                        _ => {}
                    }
                    match current_block_37 {
                        16668937799742929182 => {}
                        _ => {
                            cpv = cpv.offset(1);
                        }
                    }
                }
                3 => {
                    current_block_37 = 9464725857721511177;
                    match current_block_37 {
                        1868291631715963762 => {
                            let mut v_1: libc::c_int = config_plugin_value_tobool(
                                du,
                                -(1 as libc::c_int),
                            );
                            if -(1 as libc::c_int) == v_1 {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    259 as libc::c_int as libc::c_uint,
                                    b"ERROR: unexpected type for key: %s (string) \"(enable|disable)\"\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            } else {
                                (*cpv).v.u = v_1 as libc::c_uint;
                                current_block_37 = 2472048668343472511;
                            }
                        }
                        9464725857721511177 => {
                            match (*du).type_0 as libc::c_uint {
                                2 => {
                                    (*cpv)
                                        .v
                                        .u = (*(du as *const data_integer)).value as libc::c_uint;
                                    current_block_37 = 2472048668343472511;
                                }
                                0 => {
                                    let v_0: *const libc::c_char = (*(du as *const data_string))
                                        .value
                                        .ptr;
                                    if !v_0.is_null() && *v_0 as libc::c_int != 0 {
                                        let mut e_0: *mut libc::c_char = 0 as *mut libc::c_char;
                                        let mut l_0: libc::c_long = strtol(
                                            v_0,
                                            &mut e_0,
                                            10 as libc::c_int,
                                        );
                                        if e_0 != v_0 as *mut libc::c_char && *e_0 == 0
                                            && l_0 >= 0 as libc::c_int as libc::c_long
                                        {
                                            (*cpv).v.shrt = l_0 as libc::c_uint as libc::c_ushort;
                                            current_block_37 = 2472048668343472511;
                                        } else {
                                            current_block_37 = 4888910987971495881;
                                        }
                                    } else {
                                        current_block_37 = 4888910987971495881;
                                    }
                                    match current_block_37 {
                                        2472048668343472511 => {}
                                        _ => {
                                            log_error(
                                                (*srv).errh,
                                                b"src/configfile-glue.c\0" as *const u8
                                                    as *const libc::c_char,
                                                242 as libc::c_int as libc::c_uint,
                                                b"got a string but expected an integer: %s %s\0"
                                                    as *const u8 as *const libc::c_char,
                                                (*cpk.offset(i as isize)).k,
                                                v_0,
                                            );
                                            rc = 0 as libc::c_int;
                                            current_block_37 = 16668937799742929182;
                                        }
                                    }
                                }
                                _ => {
                                    log_error(
                                        (*srv).errh,
                                        b"src/configfile-glue.c\0" as *const u8
                                            as *const libc::c_char,
                                        248 as libc::c_int as libc::c_uint,
                                        b"unexpected type for key: %s %d expected an integer, range 0 ... 4294967295\0"
                                            as *const u8 as *const libc::c_char,
                                        (*cpk.offset(i as isize)).k,
                                        (*du).type_0 as libc::c_uint,
                                    );
                                    rc = 0 as libc::c_int;
                                    current_block_37 = 16668937799742929182;
                                }
                            }
                        }
                        9441948780229972447 => {
                            match (*du).type_0 as libc::c_uint {
                                2 => {
                                    (*cpv)
                                        .v
                                        .shrt = (*(du as *const data_integer)).value
                                        as libc::c_ushort;
                                    current_block_37 = 2472048668343472511;
                                }
                                0 => {
                                    let v: *const libc::c_char = (*(du as *const data_string))
                                        .value
                                        .ptr;
                                    if !v.is_null() && *v as libc::c_int != 0 {
                                        let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
                                        let mut l: libc::c_long = strtol(
                                            v,
                                            &mut e,
                                            10 as libc::c_int,
                                        );
                                        if e != v as *mut libc::c_char && *e == 0
                                            && l >= 0 as libc::c_int as libc::c_long
                                            && l <= 65535 as libc::c_int as libc::c_long
                                        {
                                            (*cpv).v.shrt = l as libc::c_ushort;
                                            current_block_37 = 2472048668343472511;
                                        } else {
                                            current_block_37 = 10150597327160359210;
                                        }
                                    } else {
                                        current_block_37 = 10150597327160359210;
                                    }
                                    match current_block_37 {
                                        2472048668343472511 => {}
                                        _ => {
                                            log_error(
                                                (*srv).errh,
                                                b"src/configfile-glue.c\0" as *const u8
                                                    as *const libc::c_char,
                                                214 as libc::c_int as libc::c_uint,
                                                b"got a string but expected a short: %s %s\0" as *const u8
                                                    as *const libc::c_char,
                                                (*cpk.offset(i as isize)).k,
                                                v,
                                            );
                                            rc = 0 as libc::c_int;
                                            current_block_37 = 16668937799742929182;
                                        }
                                    }
                                }
                                _ => {
                                    log_error(
                                        (*srv).errh,
                                        b"src/configfile-glue.c\0" as *const u8
                                            as *const libc::c_char,
                                        220 as libc::c_int as libc::c_uint,
                                        b"unexpected type for key: %s %d expected a short integer, range 0 ... 65535\0"
                                            as *const u8 as *const libc::c_char,
                                        (*cpk.offset(i as isize)).k,
                                        (*du).type_0 as libc::c_uint,
                                    );
                                    rc = 0 as libc::c_int;
                                    current_block_37 = 16668937799742929182;
                                }
                            }
                        }
                        10889058540383362684 => {
                            if (*du).type_0 as libc::c_uint
                                == TYPE_STRING as libc::c_int as libc::c_uint
                            {
                                (*cpv).v.b = &(*(du as *const data_string)).value;
                                current_block_37 = 2472048668343472511;
                            } else {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    186 as libc::c_int as libc::c_uint,
                                    b"%s should have been a string like ... = \"...\"\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            }
                        }
                        1591380387575923842 => {
                            if (*du).type_0 as libc::c_uint
                                == TYPE_ARRAY as libc::c_int as libc::c_uint
                            {
                                (*cpv).v.a = &(*(du as *const data_array)).value;
                                match (*cpk.offset(i as isize)).ktype as libc::c_int {
                                    6 => {
                                        current_block_37 = 10144490576563699853;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    7 => {
                                        current_block_37 = 2253044032609351678;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    8 => {
                                        current_block_37 = 4577159841354843635;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    9 => {
                                        current_block_37 = 6585473732656942518;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        current_block_37 = 2472048668343472511;
                                    }
                                }
                            } else {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    129 as libc::c_int as libc::c_uint,
                                    b"%s should have been a list like %s = ( \"...\" )\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            }
                        }
                        4945542941617773199 => {
                            log_error(
                                (*srv).errh,
                                b"src/configfile-glue.c\0" as *const u8
                                    as *const libc::c_char,
                                272 as libc::c_int as libc::c_uint,
                                b"ERROR: found unsupported key: %s (%s)\0" as *const u8
                                    as *const libc::c_char,
                                (*cpk.offset(i as isize)).k,
                                mname,
                            );
                            (*srv)
                                .srvconf
                                .config_unsupported = 1 as libc::c_int as libc::c_uchar;
                            current_block_37 = 16668937799742929182;
                        }
                        7372943784920105432 => {
                            log_error(
                                (*srv).errh,
                                b"src/configfile-glue.c\0" as *const u8
                                    as *const libc::c_char,
                                277 as libc::c_int as libc::c_uint,
                                b"ERROR: found deprecated key: %s (%s)\0" as *const u8
                                    as *const libc::c_char,
                                (*cpk.offset(i as isize)).k,
                                mname,
                            );
                            (*srv)
                                .srvconf
                                .config_deprecated = 1 as libc::c_int as libc::c_uchar;
                            current_block_37 = 16668937799742929182;
                        }
                        _ => {}
                    }
                    match current_block_37 {
                        16668937799742929182 => {}
                        _ => {
                            cpv = cpv.offset(1);
                        }
                    }
                }
                4 => {
                    current_block_37 = 1868291631715963762;
                    match current_block_37 {
                        1868291631715963762 => {
                            let mut v_1: libc::c_int = config_plugin_value_tobool(
                                du,
                                -(1 as libc::c_int),
                            );
                            if -(1 as libc::c_int) == v_1 {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    259 as libc::c_int as libc::c_uint,
                                    b"ERROR: unexpected type for key: %s (string) \"(enable|disable)\"\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            } else {
                                (*cpv).v.u = v_1 as libc::c_uint;
                                current_block_37 = 2472048668343472511;
                            }
                        }
                        9464725857721511177 => {
                            match (*du).type_0 as libc::c_uint {
                                2 => {
                                    (*cpv)
                                        .v
                                        .u = (*(du as *const data_integer)).value as libc::c_uint;
                                    current_block_37 = 2472048668343472511;
                                }
                                0 => {
                                    let v_0: *const libc::c_char = (*(du as *const data_string))
                                        .value
                                        .ptr;
                                    if !v_0.is_null() && *v_0 as libc::c_int != 0 {
                                        let mut e_0: *mut libc::c_char = 0 as *mut libc::c_char;
                                        let mut l_0: libc::c_long = strtol(
                                            v_0,
                                            &mut e_0,
                                            10 as libc::c_int,
                                        );
                                        if e_0 != v_0 as *mut libc::c_char && *e_0 == 0
                                            && l_0 >= 0 as libc::c_int as libc::c_long
                                        {
                                            (*cpv).v.shrt = l_0 as libc::c_uint as libc::c_ushort;
                                            current_block_37 = 2472048668343472511;
                                        } else {
                                            current_block_37 = 4888910987971495881;
                                        }
                                    } else {
                                        current_block_37 = 4888910987971495881;
                                    }
                                    match current_block_37 {
                                        2472048668343472511 => {}
                                        _ => {
                                            log_error(
                                                (*srv).errh,
                                                b"src/configfile-glue.c\0" as *const u8
                                                    as *const libc::c_char,
                                                242 as libc::c_int as libc::c_uint,
                                                b"got a string but expected an integer: %s %s\0"
                                                    as *const u8 as *const libc::c_char,
                                                (*cpk.offset(i as isize)).k,
                                                v_0,
                                            );
                                            rc = 0 as libc::c_int;
                                            current_block_37 = 16668937799742929182;
                                        }
                                    }
                                }
                                _ => {
                                    log_error(
                                        (*srv).errh,
                                        b"src/configfile-glue.c\0" as *const u8
                                            as *const libc::c_char,
                                        248 as libc::c_int as libc::c_uint,
                                        b"unexpected type for key: %s %d expected an integer, range 0 ... 4294967295\0"
                                            as *const u8 as *const libc::c_char,
                                        (*cpk.offset(i as isize)).k,
                                        (*du).type_0 as libc::c_uint,
                                    );
                                    rc = 0 as libc::c_int;
                                    current_block_37 = 16668937799742929182;
                                }
                            }
                        }
                        9441948780229972447 => {
                            match (*du).type_0 as libc::c_uint {
                                2 => {
                                    (*cpv)
                                        .v
                                        .shrt = (*(du as *const data_integer)).value
                                        as libc::c_ushort;
                                    current_block_37 = 2472048668343472511;
                                }
                                0 => {
                                    let v: *const libc::c_char = (*(du as *const data_string))
                                        .value
                                        .ptr;
                                    if !v.is_null() && *v as libc::c_int != 0 {
                                        let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
                                        let mut l: libc::c_long = strtol(
                                            v,
                                            &mut e,
                                            10 as libc::c_int,
                                        );
                                        if e != v as *mut libc::c_char && *e == 0
                                            && l >= 0 as libc::c_int as libc::c_long
                                            && l <= 65535 as libc::c_int as libc::c_long
                                        {
                                            (*cpv).v.shrt = l as libc::c_ushort;
                                            current_block_37 = 2472048668343472511;
                                        } else {
                                            current_block_37 = 10150597327160359210;
                                        }
                                    } else {
                                        current_block_37 = 10150597327160359210;
                                    }
                                    match current_block_37 {
                                        2472048668343472511 => {}
                                        _ => {
                                            log_error(
                                                (*srv).errh,
                                                b"src/configfile-glue.c\0" as *const u8
                                                    as *const libc::c_char,
                                                214 as libc::c_int as libc::c_uint,
                                                b"got a string but expected a short: %s %s\0" as *const u8
                                                    as *const libc::c_char,
                                                (*cpk.offset(i as isize)).k,
                                                v,
                                            );
                                            rc = 0 as libc::c_int;
                                            current_block_37 = 16668937799742929182;
                                        }
                                    }
                                }
                                _ => {
                                    log_error(
                                        (*srv).errh,
                                        b"src/configfile-glue.c\0" as *const u8
                                            as *const libc::c_char,
                                        220 as libc::c_int as libc::c_uint,
                                        b"unexpected type for key: %s %d expected a short integer, range 0 ... 65535\0"
                                            as *const u8 as *const libc::c_char,
                                        (*cpk.offset(i as isize)).k,
                                        (*du).type_0 as libc::c_uint,
                                    );
                                    rc = 0 as libc::c_int;
                                    current_block_37 = 16668937799742929182;
                                }
                            }
                        }
                        10889058540383362684 => {
                            if (*du).type_0 as libc::c_uint
                                == TYPE_STRING as libc::c_int as libc::c_uint
                            {
                                (*cpv).v.b = &(*(du as *const data_string)).value;
                                current_block_37 = 2472048668343472511;
                            } else {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    186 as libc::c_int as libc::c_uint,
                                    b"%s should have been a string like ... = \"...\"\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            }
                        }
                        1591380387575923842 => {
                            if (*du).type_0 as libc::c_uint
                                == TYPE_ARRAY as libc::c_int as libc::c_uint
                            {
                                (*cpv).v.a = &(*(du as *const data_array)).value;
                                match (*cpk.offset(i as isize)).ktype as libc::c_int {
                                    6 => {
                                        current_block_37 = 10144490576563699853;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    7 => {
                                        current_block_37 = 2253044032609351678;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    8 => {
                                        current_block_37 = 4577159841354843635;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    9 => {
                                        current_block_37 = 6585473732656942518;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        current_block_37 = 2472048668343472511;
                                    }
                                }
                            } else {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    129 as libc::c_int as libc::c_uint,
                                    b"%s should have been a list like %s = ( \"...\" )\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            }
                        }
                        4945542941617773199 => {
                            log_error(
                                (*srv).errh,
                                b"src/configfile-glue.c\0" as *const u8
                                    as *const libc::c_char,
                                272 as libc::c_int as libc::c_uint,
                                b"ERROR: found unsupported key: %s (%s)\0" as *const u8
                                    as *const libc::c_char,
                                (*cpk.offset(i as isize)).k,
                                mname,
                            );
                            (*srv)
                                .srvconf
                                .config_unsupported = 1 as libc::c_int as libc::c_uchar;
                            current_block_37 = 16668937799742929182;
                        }
                        7372943784920105432 => {
                            log_error(
                                (*srv).errh,
                                b"src/configfile-glue.c\0" as *const u8
                                    as *const libc::c_char,
                                277 as libc::c_int as libc::c_uint,
                                b"ERROR: found deprecated key: %s (%s)\0" as *const u8
                                    as *const libc::c_char,
                                (*cpk.offset(i as isize)).k,
                                mname,
                            );
                            (*srv)
                                .srvconf
                                .config_deprecated = 1 as libc::c_int as libc::c_uchar;
                            current_block_37 = 16668937799742929182;
                        }
                        _ => {}
                    }
                    match current_block_37 {
                        16668937799742929182 => {}
                        _ => {
                            cpv = cpv.offset(1);
                        }
                    }
                }
                10 | 0 => {}
                12 => {
                    current_block_37 = 4945542941617773199;
                    match current_block_37 {
                        1868291631715963762 => {
                            let mut v_1: libc::c_int = config_plugin_value_tobool(
                                du,
                                -(1 as libc::c_int),
                            );
                            if -(1 as libc::c_int) == v_1 {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    259 as libc::c_int as libc::c_uint,
                                    b"ERROR: unexpected type for key: %s (string) \"(enable|disable)\"\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            } else {
                                (*cpv).v.u = v_1 as libc::c_uint;
                                current_block_37 = 2472048668343472511;
                            }
                        }
                        9464725857721511177 => {
                            match (*du).type_0 as libc::c_uint {
                                2 => {
                                    (*cpv)
                                        .v
                                        .u = (*(du as *const data_integer)).value as libc::c_uint;
                                    current_block_37 = 2472048668343472511;
                                }
                                0 => {
                                    let v_0: *const libc::c_char = (*(du as *const data_string))
                                        .value
                                        .ptr;
                                    if !v_0.is_null() && *v_0 as libc::c_int != 0 {
                                        let mut e_0: *mut libc::c_char = 0 as *mut libc::c_char;
                                        let mut l_0: libc::c_long = strtol(
                                            v_0,
                                            &mut e_0,
                                            10 as libc::c_int,
                                        );
                                        if e_0 != v_0 as *mut libc::c_char && *e_0 == 0
                                            && l_0 >= 0 as libc::c_int as libc::c_long
                                        {
                                            (*cpv).v.shrt = l_0 as libc::c_uint as libc::c_ushort;
                                            current_block_37 = 2472048668343472511;
                                        } else {
                                            current_block_37 = 4888910987971495881;
                                        }
                                    } else {
                                        current_block_37 = 4888910987971495881;
                                    }
                                    match current_block_37 {
                                        2472048668343472511 => {}
                                        _ => {
                                            log_error(
                                                (*srv).errh,
                                                b"src/configfile-glue.c\0" as *const u8
                                                    as *const libc::c_char,
                                                242 as libc::c_int as libc::c_uint,
                                                b"got a string but expected an integer: %s %s\0"
                                                    as *const u8 as *const libc::c_char,
                                                (*cpk.offset(i as isize)).k,
                                                v_0,
                                            );
                                            rc = 0 as libc::c_int;
                                            current_block_37 = 16668937799742929182;
                                        }
                                    }
                                }
                                _ => {
                                    log_error(
                                        (*srv).errh,
                                        b"src/configfile-glue.c\0" as *const u8
                                            as *const libc::c_char,
                                        248 as libc::c_int as libc::c_uint,
                                        b"unexpected type for key: %s %d expected an integer, range 0 ... 4294967295\0"
                                            as *const u8 as *const libc::c_char,
                                        (*cpk.offset(i as isize)).k,
                                        (*du).type_0 as libc::c_uint,
                                    );
                                    rc = 0 as libc::c_int;
                                    current_block_37 = 16668937799742929182;
                                }
                            }
                        }
                        9441948780229972447 => {
                            match (*du).type_0 as libc::c_uint {
                                2 => {
                                    (*cpv)
                                        .v
                                        .shrt = (*(du as *const data_integer)).value
                                        as libc::c_ushort;
                                    current_block_37 = 2472048668343472511;
                                }
                                0 => {
                                    let v: *const libc::c_char = (*(du as *const data_string))
                                        .value
                                        .ptr;
                                    if !v.is_null() && *v as libc::c_int != 0 {
                                        let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
                                        let mut l: libc::c_long = strtol(
                                            v,
                                            &mut e,
                                            10 as libc::c_int,
                                        );
                                        if e != v as *mut libc::c_char && *e == 0
                                            && l >= 0 as libc::c_int as libc::c_long
                                            && l <= 65535 as libc::c_int as libc::c_long
                                        {
                                            (*cpv).v.shrt = l as libc::c_ushort;
                                            current_block_37 = 2472048668343472511;
                                        } else {
                                            current_block_37 = 10150597327160359210;
                                        }
                                    } else {
                                        current_block_37 = 10150597327160359210;
                                    }
                                    match current_block_37 {
                                        2472048668343472511 => {}
                                        _ => {
                                            log_error(
                                                (*srv).errh,
                                                b"src/configfile-glue.c\0" as *const u8
                                                    as *const libc::c_char,
                                                214 as libc::c_int as libc::c_uint,
                                                b"got a string but expected a short: %s %s\0" as *const u8
                                                    as *const libc::c_char,
                                                (*cpk.offset(i as isize)).k,
                                                v,
                                            );
                                            rc = 0 as libc::c_int;
                                            current_block_37 = 16668937799742929182;
                                        }
                                    }
                                }
                                _ => {
                                    log_error(
                                        (*srv).errh,
                                        b"src/configfile-glue.c\0" as *const u8
                                            as *const libc::c_char,
                                        220 as libc::c_int as libc::c_uint,
                                        b"unexpected type for key: %s %d expected a short integer, range 0 ... 65535\0"
                                            as *const u8 as *const libc::c_char,
                                        (*cpk.offset(i as isize)).k,
                                        (*du).type_0 as libc::c_uint,
                                    );
                                    rc = 0 as libc::c_int;
                                    current_block_37 = 16668937799742929182;
                                }
                            }
                        }
                        10889058540383362684 => {
                            if (*du).type_0 as libc::c_uint
                                == TYPE_STRING as libc::c_int as libc::c_uint
                            {
                                (*cpv).v.b = &(*(du as *const data_string)).value;
                                current_block_37 = 2472048668343472511;
                            } else {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    186 as libc::c_int as libc::c_uint,
                                    b"%s should have been a string like ... = \"...\"\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            }
                        }
                        1591380387575923842 => {
                            if (*du).type_0 as libc::c_uint
                                == TYPE_ARRAY as libc::c_int as libc::c_uint
                            {
                                (*cpv).v.a = &(*(du as *const data_array)).value;
                                match (*cpk.offset(i as isize)).ktype as libc::c_int {
                                    6 => {
                                        current_block_37 = 10144490576563699853;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    7 => {
                                        current_block_37 = 2253044032609351678;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    8 => {
                                        current_block_37 = 4577159841354843635;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    9 => {
                                        current_block_37 = 6585473732656942518;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        current_block_37 = 2472048668343472511;
                                    }
                                }
                            } else {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    129 as libc::c_int as libc::c_uint,
                                    b"%s should have been a list like %s = ( \"...\" )\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            }
                        }
                        4945542941617773199 => {
                            log_error(
                                (*srv).errh,
                                b"src/configfile-glue.c\0" as *const u8
                                    as *const libc::c_char,
                                272 as libc::c_int as libc::c_uint,
                                b"ERROR: found unsupported key: %s (%s)\0" as *const u8
                                    as *const libc::c_char,
                                (*cpk.offset(i as isize)).k,
                                mname,
                            );
                            (*srv)
                                .srvconf
                                .config_unsupported = 1 as libc::c_int as libc::c_uchar;
                            current_block_37 = 16668937799742929182;
                        }
                        7372943784920105432 => {
                            log_error(
                                (*srv).errh,
                                b"src/configfile-glue.c\0" as *const u8
                                    as *const libc::c_char,
                                277 as libc::c_int as libc::c_uint,
                                b"ERROR: found deprecated key: %s (%s)\0" as *const u8
                                    as *const libc::c_char,
                                (*cpk.offset(i as isize)).k,
                                mname,
                            );
                            (*srv)
                                .srvconf
                                .config_deprecated = 1 as libc::c_int as libc::c_uchar;
                            current_block_37 = 16668937799742929182;
                        }
                        _ => {}
                    }
                    match current_block_37 {
                        16668937799742929182 => {}
                        _ => {
                            cpv = cpv.offset(1);
                        }
                    }
                }
                11 => {
                    current_block_37 = 7372943784920105432;
                    match current_block_37 {
                        1868291631715963762 => {
                            let mut v_1: libc::c_int = config_plugin_value_tobool(
                                du,
                                -(1 as libc::c_int),
                            );
                            if -(1 as libc::c_int) == v_1 {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    259 as libc::c_int as libc::c_uint,
                                    b"ERROR: unexpected type for key: %s (string) \"(enable|disable)\"\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            } else {
                                (*cpv).v.u = v_1 as libc::c_uint;
                                current_block_37 = 2472048668343472511;
                            }
                        }
                        9464725857721511177 => {
                            match (*du).type_0 as libc::c_uint {
                                2 => {
                                    (*cpv)
                                        .v
                                        .u = (*(du as *const data_integer)).value as libc::c_uint;
                                    current_block_37 = 2472048668343472511;
                                }
                                0 => {
                                    let v_0: *const libc::c_char = (*(du as *const data_string))
                                        .value
                                        .ptr;
                                    if !v_0.is_null() && *v_0 as libc::c_int != 0 {
                                        let mut e_0: *mut libc::c_char = 0 as *mut libc::c_char;
                                        let mut l_0: libc::c_long = strtol(
                                            v_0,
                                            &mut e_0,
                                            10 as libc::c_int,
                                        );
                                        if e_0 != v_0 as *mut libc::c_char && *e_0 == 0
                                            && l_0 >= 0 as libc::c_int as libc::c_long
                                        {
                                            (*cpv).v.shrt = l_0 as libc::c_uint as libc::c_ushort;
                                            current_block_37 = 2472048668343472511;
                                        } else {
                                            current_block_37 = 4888910987971495881;
                                        }
                                    } else {
                                        current_block_37 = 4888910987971495881;
                                    }
                                    match current_block_37 {
                                        2472048668343472511 => {}
                                        _ => {
                                            log_error(
                                                (*srv).errh,
                                                b"src/configfile-glue.c\0" as *const u8
                                                    as *const libc::c_char,
                                                242 as libc::c_int as libc::c_uint,
                                                b"got a string but expected an integer: %s %s\0"
                                                    as *const u8 as *const libc::c_char,
                                                (*cpk.offset(i as isize)).k,
                                                v_0,
                                            );
                                            rc = 0 as libc::c_int;
                                            current_block_37 = 16668937799742929182;
                                        }
                                    }
                                }
                                _ => {
                                    log_error(
                                        (*srv).errh,
                                        b"src/configfile-glue.c\0" as *const u8
                                            as *const libc::c_char,
                                        248 as libc::c_int as libc::c_uint,
                                        b"unexpected type for key: %s %d expected an integer, range 0 ... 4294967295\0"
                                            as *const u8 as *const libc::c_char,
                                        (*cpk.offset(i as isize)).k,
                                        (*du).type_0 as libc::c_uint,
                                    );
                                    rc = 0 as libc::c_int;
                                    current_block_37 = 16668937799742929182;
                                }
                            }
                        }
                        9441948780229972447 => {
                            match (*du).type_0 as libc::c_uint {
                                2 => {
                                    (*cpv)
                                        .v
                                        .shrt = (*(du as *const data_integer)).value
                                        as libc::c_ushort;
                                    current_block_37 = 2472048668343472511;
                                }
                                0 => {
                                    let v: *const libc::c_char = (*(du as *const data_string))
                                        .value
                                        .ptr;
                                    if !v.is_null() && *v as libc::c_int != 0 {
                                        let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
                                        let mut l: libc::c_long = strtol(
                                            v,
                                            &mut e,
                                            10 as libc::c_int,
                                        );
                                        if e != v as *mut libc::c_char && *e == 0
                                            && l >= 0 as libc::c_int as libc::c_long
                                            && l <= 65535 as libc::c_int as libc::c_long
                                        {
                                            (*cpv).v.shrt = l as libc::c_ushort;
                                            current_block_37 = 2472048668343472511;
                                        } else {
                                            current_block_37 = 10150597327160359210;
                                        }
                                    } else {
                                        current_block_37 = 10150597327160359210;
                                    }
                                    match current_block_37 {
                                        2472048668343472511 => {}
                                        _ => {
                                            log_error(
                                                (*srv).errh,
                                                b"src/configfile-glue.c\0" as *const u8
                                                    as *const libc::c_char,
                                                214 as libc::c_int as libc::c_uint,
                                                b"got a string but expected a short: %s %s\0" as *const u8
                                                    as *const libc::c_char,
                                                (*cpk.offset(i as isize)).k,
                                                v,
                                            );
                                            rc = 0 as libc::c_int;
                                            current_block_37 = 16668937799742929182;
                                        }
                                    }
                                }
                                _ => {
                                    log_error(
                                        (*srv).errh,
                                        b"src/configfile-glue.c\0" as *const u8
                                            as *const libc::c_char,
                                        220 as libc::c_int as libc::c_uint,
                                        b"unexpected type for key: %s %d expected a short integer, range 0 ... 65535\0"
                                            as *const u8 as *const libc::c_char,
                                        (*cpk.offset(i as isize)).k,
                                        (*du).type_0 as libc::c_uint,
                                    );
                                    rc = 0 as libc::c_int;
                                    current_block_37 = 16668937799742929182;
                                }
                            }
                        }
                        10889058540383362684 => {
                            if (*du).type_0 as libc::c_uint
                                == TYPE_STRING as libc::c_int as libc::c_uint
                            {
                                (*cpv).v.b = &(*(du as *const data_string)).value;
                                current_block_37 = 2472048668343472511;
                            } else {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    186 as libc::c_int as libc::c_uint,
                                    b"%s should have been a string like ... = \"...\"\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            }
                        }
                        1591380387575923842 => {
                            if (*du).type_0 as libc::c_uint
                                == TYPE_ARRAY as libc::c_int as libc::c_uint
                            {
                                (*cpv).v.a = &(*(du as *const data_array)).value;
                                match (*cpk.offset(i as isize)).ktype as libc::c_int {
                                    6 => {
                                        current_block_37 = 10144490576563699853;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    7 => {
                                        current_block_37 = 2253044032609351678;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    8 => {
                                        current_block_37 = 4577159841354843635;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    9 => {
                                        current_block_37 = 6585473732656942518;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        current_block_37 = 2472048668343472511;
                                    }
                                }
                            } else {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    129 as libc::c_int as libc::c_uint,
                                    b"%s should have been a list like %s = ( \"...\" )\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            }
                        }
                        4945542941617773199 => {
                            log_error(
                                (*srv).errh,
                                b"src/configfile-glue.c\0" as *const u8
                                    as *const libc::c_char,
                                272 as libc::c_int as libc::c_uint,
                                b"ERROR: found unsupported key: %s (%s)\0" as *const u8
                                    as *const libc::c_char,
                                (*cpk.offset(i as isize)).k,
                                mname,
                            );
                            (*srv)
                                .srvconf
                                .config_unsupported = 1 as libc::c_int as libc::c_uchar;
                            current_block_37 = 16668937799742929182;
                        }
                        7372943784920105432 => {
                            log_error(
                                (*srv).errh,
                                b"src/configfile-glue.c\0" as *const u8
                                    as *const libc::c_char,
                                277 as libc::c_int as libc::c_uint,
                                b"ERROR: found deprecated key: %s (%s)\0" as *const u8
                                    as *const libc::c_char,
                                (*cpk.offset(i as isize)).k,
                                mname,
                            );
                            (*srv)
                                .srvconf
                                .config_deprecated = 1 as libc::c_int as libc::c_uchar;
                            current_block_37 = 16668937799742929182;
                        }
                        _ => {}
                    }
                    match current_block_37 {
                        16668937799742929182 => {}
                        _ => {
                            cpv = cpv.offset(1);
                        }
                    }
                }
                _ => {
                    current_block_37 = 2472048668343472511;
                    match current_block_37 {
                        1868291631715963762 => {
                            let mut v_1: libc::c_int = config_plugin_value_tobool(
                                du,
                                -(1 as libc::c_int),
                            );
                            if -(1 as libc::c_int) == v_1 {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    259 as libc::c_int as libc::c_uint,
                                    b"ERROR: unexpected type for key: %s (string) \"(enable|disable)\"\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            } else {
                                (*cpv).v.u = v_1 as libc::c_uint;
                                current_block_37 = 2472048668343472511;
                            }
                        }
                        9464725857721511177 => {
                            match (*du).type_0 as libc::c_uint {
                                2 => {
                                    (*cpv)
                                        .v
                                        .u = (*(du as *const data_integer)).value as libc::c_uint;
                                    current_block_37 = 2472048668343472511;
                                }
                                0 => {
                                    let v_0: *const libc::c_char = (*(du as *const data_string))
                                        .value
                                        .ptr;
                                    if !v_0.is_null() && *v_0 as libc::c_int != 0 {
                                        let mut e_0: *mut libc::c_char = 0 as *mut libc::c_char;
                                        let mut l_0: libc::c_long = strtol(
                                            v_0,
                                            &mut e_0,
                                            10 as libc::c_int,
                                        );
                                        if e_0 != v_0 as *mut libc::c_char && *e_0 == 0
                                            && l_0 >= 0 as libc::c_int as libc::c_long
                                        {
                                            (*cpv).v.shrt = l_0 as libc::c_uint as libc::c_ushort;
                                            current_block_37 = 2472048668343472511;
                                        } else {
                                            current_block_37 = 4888910987971495881;
                                        }
                                    } else {
                                        current_block_37 = 4888910987971495881;
                                    }
                                    match current_block_37 {
                                        2472048668343472511 => {}
                                        _ => {
                                            log_error(
                                                (*srv).errh,
                                                b"src/configfile-glue.c\0" as *const u8
                                                    as *const libc::c_char,
                                                242 as libc::c_int as libc::c_uint,
                                                b"got a string but expected an integer: %s %s\0"
                                                    as *const u8 as *const libc::c_char,
                                                (*cpk.offset(i as isize)).k,
                                                v_0,
                                            );
                                            rc = 0 as libc::c_int;
                                            current_block_37 = 16668937799742929182;
                                        }
                                    }
                                }
                                _ => {
                                    log_error(
                                        (*srv).errh,
                                        b"src/configfile-glue.c\0" as *const u8
                                            as *const libc::c_char,
                                        248 as libc::c_int as libc::c_uint,
                                        b"unexpected type for key: %s %d expected an integer, range 0 ... 4294967295\0"
                                            as *const u8 as *const libc::c_char,
                                        (*cpk.offset(i as isize)).k,
                                        (*du).type_0 as libc::c_uint,
                                    );
                                    rc = 0 as libc::c_int;
                                    current_block_37 = 16668937799742929182;
                                }
                            }
                        }
                        9441948780229972447 => {
                            match (*du).type_0 as libc::c_uint {
                                2 => {
                                    (*cpv)
                                        .v
                                        .shrt = (*(du as *const data_integer)).value
                                        as libc::c_ushort;
                                    current_block_37 = 2472048668343472511;
                                }
                                0 => {
                                    let v: *const libc::c_char = (*(du as *const data_string))
                                        .value
                                        .ptr;
                                    if !v.is_null() && *v as libc::c_int != 0 {
                                        let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
                                        let mut l: libc::c_long = strtol(
                                            v,
                                            &mut e,
                                            10 as libc::c_int,
                                        );
                                        if e != v as *mut libc::c_char && *e == 0
                                            && l >= 0 as libc::c_int as libc::c_long
                                            && l <= 65535 as libc::c_int as libc::c_long
                                        {
                                            (*cpv).v.shrt = l as libc::c_ushort;
                                            current_block_37 = 2472048668343472511;
                                        } else {
                                            current_block_37 = 10150597327160359210;
                                        }
                                    } else {
                                        current_block_37 = 10150597327160359210;
                                    }
                                    match current_block_37 {
                                        2472048668343472511 => {}
                                        _ => {
                                            log_error(
                                                (*srv).errh,
                                                b"src/configfile-glue.c\0" as *const u8
                                                    as *const libc::c_char,
                                                214 as libc::c_int as libc::c_uint,
                                                b"got a string but expected a short: %s %s\0" as *const u8
                                                    as *const libc::c_char,
                                                (*cpk.offset(i as isize)).k,
                                                v,
                                            );
                                            rc = 0 as libc::c_int;
                                            current_block_37 = 16668937799742929182;
                                        }
                                    }
                                }
                                _ => {
                                    log_error(
                                        (*srv).errh,
                                        b"src/configfile-glue.c\0" as *const u8
                                            as *const libc::c_char,
                                        220 as libc::c_int as libc::c_uint,
                                        b"unexpected type for key: %s %d expected a short integer, range 0 ... 65535\0"
                                            as *const u8 as *const libc::c_char,
                                        (*cpk.offset(i as isize)).k,
                                        (*du).type_0 as libc::c_uint,
                                    );
                                    rc = 0 as libc::c_int;
                                    current_block_37 = 16668937799742929182;
                                }
                            }
                        }
                        10889058540383362684 => {
                            if (*du).type_0 as libc::c_uint
                                == TYPE_STRING as libc::c_int as libc::c_uint
                            {
                                (*cpv).v.b = &(*(du as *const data_string)).value;
                                current_block_37 = 2472048668343472511;
                            } else {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    186 as libc::c_int as libc::c_uint,
                                    b"%s should have been a string like ... = \"...\"\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            }
                        }
                        1591380387575923842 => {
                            if (*du).type_0 as libc::c_uint
                                == TYPE_ARRAY as libc::c_int as libc::c_uint
                            {
                                (*cpv).v.a = &(*(du as *const data_array)).value;
                                match (*cpk.offset(i as isize)).ktype as libc::c_int {
                                    6 => {
                                        current_block_37 = 10144490576563699853;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    7 => {
                                        current_block_37 = 2253044032609351678;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    8 => {
                                        current_block_37 = 4577159841354843635;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    9 => {
                                        current_block_37 = 6585473732656942518;
                                        match current_block_37 {
                                            6585473732656942518 => {
                                                if array_is_vlist((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        168 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of string values like %s = ( \"...\", \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            2253044032609351678 => {
                                                if array_is_kvarray((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        148 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => list like %s = ( \"...\" => ( \"...\" => \"...\" ) )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            10144490576563699853 => {
                                                if array_is_kvany((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        138 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                            _ => {
                                                if array_is_kvstring((*cpv).v.a) == 0 {
                                                    log_error(
                                                        (*srv).errh,
                                                        b"src/configfile-glue.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        158 as libc::c_int as libc::c_uint,
                                                        b"%s should have been a list of key => string values like %s = ( \"...\" => \"...\", \"...\" => \"...\" )\0"
                                                            as *const u8 as *const libc::c_char,
                                                        (*cpk.offset(i as isize)).k,
                                                        (*cpk.offset(i as isize)).k,
                                                    );
                                                    rc = 0 as libc::c_int;
                                                    current_block_37 = 16668937799742929182;
                                                } else {
                                                    current_block_37 = 2472048668343472511;
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        current_block_37 = 2472048668343472511;
                                    }
                                }
                            } else {
                                log_error(
                                    (*srv).errh,
                                    b"src/configfile-glue.c\0" as *const u8
                                        as *const libc::c_char,
                                    129 as libc::c_int as libc::c_uint,
                                    b"%s should have been a list like %s = ( \"...\" )\0"
                                        as *const u8 as *const libc::c_char,
                                    (*cpk.offset(i as isize)).k,
                                    (*cpk.offset(i as isize)).k,
                                );
                                rc = 0 as libc::c_int;
                                current_block_37 = 16668937799742929182;
                            }
                        }
                        4945542941617773199 => {
                            log_error(
                                (*srv).errh,
                                b"src/configfile-glue.c\0" as *const u8
                                    as *const libc::c_char,
                                272 as libc::c_int as libc::c_uint,
                                b"ERROR: found unsupported key: %s (%s)\0" as *const u8
                                    as *const libc::c_char,
                                (*cpk.offset(i as isize)).k,
                                mname,
                            );
                            (*srv)
                                .srvconf
                                .config_unsupported = 1 as libc::c_int as libc::c_uchar;
                            current_block_37 = 16668937799742929182;
                        }
                        7372943784920105432 => {
                            log_error(
                                (*srv).errh,
                                b"src/configfile-glue.c\0" as *const u8
                                    as *const libc::c_char,
                                277 as libc::c_int as libc::c_uint,
                                b"ERROR: found deprecated key: %s (%s)\0" as *const u8
                                    as *const libc::c_char,
                                (*cpk.offset(i as isize)).k,
                                mname,
                            );
                            (*srv)
                                .srvconf
                                .config_deprecated = 1 as libc::c_int as libc::c_uchar;
                            current_block_37 = 16668937799742929182;
                        }
                        _ => {}
                    }
                    match current_block_37 {
                        16668937799742929182 => {}
                        _ => {
                            cpv = cpv.offset(1);
                        }
                    }
                }
            }
        }
        i += 1;
    }
    (*cpv).k_id = -(1 as libc::c_int);
    return rc;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_plugin_values_init(
    srv: *mut server,
    mut p_d: *mut libc::c_void,
    cpk: *const config_plugin_keys_t,
    mname: *const libc::c_char,
) -> libc::c_int {
    let p: *mut plugin_data_base = p_d as *mut plugin_data_base;
    let touched: *mut array = (*srv).srvconf.config_touched;
    let mut matches: [libc::c_uchar; 4096] = [0; 4096];
    let mut contexts: [libc::c_ushort; 4096] = [0; 4096];
    let mut n: uint32_t = 0 as libc::c_int as uint32_t;
    let mut rc: libc::c_int = 1 as libc::c_int;
    if !(::std::mem::size_of::<[libc::c_uchar; 4096]>() as libc::c_ulong
        >= (*(*srv).config_context).used as libc::c_ulong)
    {
        ck_assert_failed(
            b"src/configfile-glue.c\0" as *const u8 as *const libc::c_char,
            298 as libc::c_int as libc::c_uint,
            b"sizeof(matches) >= srv->config_context->used\0" as *const u8
                as *const libc::c_char,
        );
    }
    config_reference.data = (*(*srv).config_context).data as *const *const data_config;
    config_reference.used = (*(*srv).config_context).used;
    let mut u: uint32_t = 0 as libc::c_int as uint32_t;
    while u < (*(*srv).config_context).used {
        let mut ca: *const array = (*(*((*(*srv).config_context).data).offset(u as isize)
            as *const data_config))
            .value;
        matches[n as usize] = 0 as libc::c_int as libc::c_uchar;
        let mut i: libc::c_int = 0 as libc::c_int;
        while (*cpk.offset(i as isize)).ktype as libc::c_int
            != T_CONFIG_UNSET as libc::c_int
        {
            let du: *const data_unset = array_get_element_klen(
                ca,
                (*cpk.offset(i as isize)).k,
                (*cpk.offset(i as isize)).klen as uint32_t,
            );
            if !du.is_null() {
                matches[n as usize] = (matches[n as usize]).wrapping_add(1);
                array_set_key_value(
                    touched,
                    (*cpk.offset(i as isize)).k,
                    (*cpk.offset(i as isize)).klen as uint32_t,
                    b"\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                );
                if (*cpk.offset(i as isize)).scope as libc::c_int
                    == T_CONFIG_SCOPE_SERVER as libc::c_int
                    && 0 as libc::c_int as libc::c_uint != u
                {
                    log_error(
                        (*srv).errh,
                        b"src/configfile-glue.c\0" as *const u8 as *const libc::c_char,
                        323 as libc::c_int as libc::c_uint,
                        b"DEPRECATED: do not set server options in conditionals, variable: %s\0"
                            as *const u8 as *const libc::c_char,
                        (*cpk.offset(i as isize)).k,
                    );
                }
            }
            i += 1;
        }
        if matches[n as usize] != 0 {
            let fresh0 = n;
            n = n.wrapping_add(1);
            contexts[fresh0 as usize] = u as libc::c_ushort;
        }
        u = u.wrapping_add(1);
    }
    let mut elts: uint32_t = 0 as libc::c_int as uint32_t;
    let mut u_0: uint32_t = 0 as libc::c_int as uint32_t;
    while u_0 < n {
        elts = (elts as libc::c_uint).wrapping_add(matches[u_0 as usize] as libc::c_uint)
            as uint32_t as uint32_t;
        u_0 = u_0.wrapping_add(1);
    }
    (*p).nconfig = n as libc::c_int;
    (*p)
        .cvlist = calloc(
        (1 as libc::c_int as libc::c_uint)
            .wrapping_add(n)
            .wrapping_add(n)
            .wrapping_add(elts) as libc::c_ulong,
        ::std::mem::size_of::<config_plugin_value_t>() as libc::c_ulong,
    ) as *mut config_plugin_value_t;
    if ((*p).cvlist).is_null() {
        ck_assert_failed(
            b"src/configfile-glue.c\0" as *const u8 as *const libc::c_char,
            338 as libc::c_int as libc::c_uint,
            b"p->cvlist\0" as *const u8 as *const libc::c_char,
        );
    }
    elts = (1 as libc::c_int as libc::c_uint).wrapping_add(n);
    let shft: uint32_t = (0 as libc::c_int as libc::c_uint != n
        && 0 as libc::c_int != contexts[0 as libc::c_int as usize] as libc::c_int)
        as libc::c_int as uint32_t;
    if shft != 0 {
        (*p).nconfig += 1;
    }
    let mut u_1: uint32_t = 0 as libc::c_int as uint32_t;
    while u_1 < n {
        let cpv: *mut config_plugin_value_t = ((*p).cvlist)
            .offset(shft as isize)
            .offset(u_1 as isize);
        (*cpv).k_id = contexts[u_1 as usize] as libc::c_int;
        (*cpv).v.u2[0 as libc::c_int as usize] = elts;
        (*cpv).v.u2[1 as libc::c_int as usize] = matches[u_1 as usize] as uint32_t;
        elts = (elts as libc::c_uint)
            .wrapping_add(
                (matches[u_1 as usize] as libc::c_int + 1 as libc::c_int) as libc::c_uint,
            ) as uint32_t as uint32_t;
        u_1 = u_1.wrapping_add(1);
    }
    let mut u_2: uint32_t = 0 as libc::c_int as uint32_t;
    while u_2 < n {
        let mut ca_0: *const array = (*(*((*(*srv).config_context).data)
            .offset(contexts[u_2 as usize] as isize) as *const data_config))
            .value;
        let mut cpv_0: *mut config_plugin_value_t = ((*p).cvlist)
            .offset(
                (*((*p).cvlist).offset(shft.wrapping_add(u_2) as isize))
                    .v
                    .u2[0 as libc::c_int as usize] as isize,
            );
        if config_plugin_values_init_block(srv, ca_0, cpk, mname, cpv_0) == 0 {
            rc = 0 as libc::c_int;
        }
        u_2 = u_2.wrapping_add(1);
    }
    return rc;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn config_cond_result_trace(
    r: *mut request_st,
    dc: *const data_config,
    cached: libc::c_int,
) {
    let cache: *mut cond_cache_t = &mut *((*r).cond_cache)
        .offset((*dc).context_ndx as isize) as *mut cond_cache_t;
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    match (*cache).result as libc::c_int {
        0 => {
            msg = b"unset\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            msg = b"skipped\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            msg = b"false\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            msg = b"true\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            msg = b"invalid cond_result_t\0" as *const u8 as *const libc::c_char;
        }
    }
    log_error(
        (*r).conf.errh,
        b"src/configfile-glue.c\0" as *const u8 as *const libc::c_char,
        375 as libc::c_int as libc::c_uint,
        b"%d (%s) result: %s (cond: %s)\0" as *const u8 as *const libc::c_char,
        (*dc).context_ndx,
        &*(b"uncached\0" as *const u8 as *const libc::c_char)
            .offset(
                (if cached != 0 { 2 as libc::c_int } else { 0 as libc::c_int }) as isize,
            ) as *const libc::c_char,
        msg,
        (*dc).key.ptr,
    );
}
unsafe extern "C" fn config_check_cond_nocache_calc(
    r: *mut request_st,
    dc: *const data_config,
    debug_cond: libc::c_int,
    cache: *mut cond_cache_t,
) -> cond_result_t {
    (*cache).result = config_check_cond_nocache(r, dc, debug_cond, cache) as int8_t;
    if debug_cond != 0 {
        config_cond_result_trace(r, dc, 0 as libc::c_int);
    }
    return (*cache).result as cond_result_t;
}
unsafe extern "C" fn config_check_cond_cached(
    r: *mut request_st,
    dc: *const data_config,
    debug_cond: libc::c_int,
) -> cond_result_t {
    let cache: *mut cond_cache_t = &mut *((*r).cond_cache)
        .offset((*dc).context_ndx as isize) as *mut cond_cache_t;
    if COND_RESULT_UNSET as libc::c_int != (*cache).result as libc::c_int {
        if debug_cond != 0 {
            config_cond_result_trace(r, dc, 1 as libc::c_int);
        }
        return (*cache).result as cond_result_t;
    }
    return config_check_cond_nocache_calc(r, dc, debug_cond, cache);
}
unsafe extern "C" fn config_check_cond_nocache(
    r: *mut request_st,
    dc: *const data_config,
    debug_cond: libc::c_int,
    cache: *mut cond_cache_t,
) -> cond_result_t {
    if !((*dc).parent).is_null() && (*(*dc).parent).context_ndx != 0 {
        if debug_cond != 0 {
            log_error(
                (*r).conf.errh,
                b"src/configfile-glue.c\0" as *const u8 as *const libc::c_char,
                409 as libc::c_int as libc::c_uint,
                b"go parent %s\0" as *const u8 as *const libc::c_char,
                (*(*dc).parent).key.ptr,
            );
        }
        match config_check_cond_cached(r, (*dc).parent, debug_cond) as libc::c_uint {
            0 => return COND_RESULT_UNSET,
            1 | 2 => return COND_RESULT_SKIP,
            3 | _ => {}
        }
    }
    if !((*dc).prev).is_null() {
        if debug_cond != 0 {
            log_error(
                (*r).conf.errh,
                b"src/configfile-glue.c\0" as *const u8 as *const libc::c_char,
                432 as libc::c_int as libc::c_uint,
                b"go prev %s\0" as *const u8 as *const libc::c_char,
                (*(*dc).prev).key.ptr,
            );
        }
        match config_check_cond_cached(r, (*dc).prev, debug_cond) as libc::c_uint {
            0 => return COND_RESULT_UNSET,
            1 | 3 => return COND_RESULT_SKIP,
            2 | _ => {}
        }
    }
    if (*r).conditional_is_valid
        & ((1 as libc::c_int) << (*dc).comp as libc::c_uint) as libc::c_uint == 0
    {
        if debug_cond != 0 {
            log_error(
                (*r).conf.errh,
                b"src/configfile-glue.c\0" as *const u8 as *const libc::c_char,
                452 as libc::c_int as libc::c_uint,
                b"%d %s not available yet\0" as *const u8 as *const libc::c_char,
                (*dc).comp as libc::c_uint,
                (*dc).key.ptr,
            );
        }
        return COND_RESULT_UNSET;
    }
    match (*cache).local_result as libc::c_int {
        3 | 2 => return (*cache).local_result as cond_result_t,
        _ => {}
    }
    if CONFIG_COND_ELSE as libc::c_int as libc::c_uint == (*dc).cond as libc::c_uint {
        (*cache).local_result = COND_RESULT_TRUE as libc::c_int as int8_t;
        return (*cache).local_result as cond_result_t;
    }
    return config_check_cond_nocache_eval(r, dc, debug_cond, cache);
}
unsafe extern "C" fn config_check_cond_nocache_eval(
    r: *mut request_st,
    dc: *const data_config,
    debug_cond: libc::c_int,
    cache: *mut cond_cache_t,
) -> cond_result_t {
    static mut empty_string: const_char_buffer = {
        let mut init = const_char_buffer {
            ptr: b"\0" as *const u8 as *const libc::c_char,
            used: 1 as libc::c_int as uint32_t,
            size: 0 as libc::c_int as uint32_t,
        };
        init
    };
    let mut l: *const buffer = 0 as *const buffer;
    match (*dc).comp as libc::c_uint {
        3 => {
            l = &mut (*r).uri.authority;
        }
        8 => {
            l = &mut (*(*r).con).dst_addr_buf;
        }
        10 => {
            l = &mut (*r).uri.scheme;
        }
        2 => {
            l = &mut (*r).uri.path;
        }
        9 => {
            l = &mut (*r).uri.query;
        }
        1 => {
            l = (*(*(*r).con).srv_socket).srv_token;
        }
        12 => {
            l = http_header_request_get(
                r,
                (*dc).ext as http_header_e,
                (*dc).comp_tag.ptr,
                buffer_clen(&(*dc).comp_tag),
            );
            if l.is_null() {
                l = &mut empty_string as *mut const_char_buffer as *mut buffer;
            }
        }
        11 => {
            l = http_method_buf((*r).http_method);
        }
        _ => {
            (*cache).local_result = COND_RESULT_FALSE as libc::c_int as int8_t;
            return (*cache).local_result as cond_result_t;
        }
    }
    if buffer_is_blank(l) as libc::c_long != 0 {
        l = &mut empty_string as *mut const_char_buffer as *mut buffer;
    }
    if debug_cond != 0 {
        log_error(
            (*r).conf.errh,
            b"src/configfile-glue.c\0" as *const u8 as *const libc::c_char,
            519 as libc::c_int as libc::c_uint,
            b"%s compare to %s\0" as *const u8 as *const libc::c_char,
            (*dc).comp_key,
            (*l).ptr,
        );
    }
    let mut match_0: libc::c_int = 0;
    let mut current_block_22: u64;
    match (*dc).cond as libc::c_uint {
        3 | 1 => {
            match_0 = ((*dc).cond as libc::c_uint
                == CONFIG_COND_EQ as libc::c_int as libc::c_uint) as libc::c_int;
            if (*dc).comp as libc::c_uint
                == COMP_HTTP_HOST as libc::c_int as libc::c_uint
                && *((*dc).string.ptr).offset(0 as libc::c_int as isize) as libc::c_int
                    != '/' as i32
            {
                let mut llen: uint_fast32_t = buffer_clen(l) as uint_fast32_t;
                let mut dlen: uint_fast32_t = buffer_clen(&(*dc).string)
                    as uint_fast32_t;
                if llen != 0 && llen != dlen {
                    match_0
                        ^= ((if llen > dlen {
                            (*((*l).ptr).offset(dlen as isize) as libc::c_int
                                == ':' as i32
                                && llen.wrapping_sub(dlen)
                                    <= 6 as libc::c_int as libc::c_ulong) as libc::c_int
                        } else {
                            dlen = llen;
                            (*((*dc).string.ptr).offset(dlen as isize) as libc::c_int
                                == ':' as i32) as libc::c_int
                        }) != 0
                            && 0 as libc::c_int
                                == memcmp(
                                    (*l).ptr as *const libc::c_void,
                                    (*dc).string.ptr as *const libc::c_void,
                                    dlen,
                                )) as libc::c_int;
                    current_block_22 = 11743904203796629665;
                } else {
                    current_block_22 = 1118134448028020070;
                }
            } else if (*dc).comp as libc::c_uint
                    == COMP_HTTP_REMOTE_IP as libc::c_int as libc::c_uint
                    && *((*dc).string.ptr).offset(0 as libc::c_int as isize)
                        as libc::c_int != '/' as i32
                {
                let addr: *const sock_addr = (((*dc).string.ptr as uintptr_t)
                    .wrapping_add((*dc).string.used as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(7 as libc::c_int as libc::c_ulong)
                    & !(7 as libc::c_int) as libc::c_ulong) as *mut sock_addr;
                let mut bits: libc::c_int = *((*dc).string.ptr as *mut libc::c_uchar)
                    .offset((*dc).string.used as isize) as libc::c_int;
                match_0
                    ^= if bits != 0 {
                        sock_addr_is_addr_eq_bits(addr, &mut (*(*r).con).dst_addr, bits)
                    } else {
                        sock_addr_is_addr_eq(addr, &mut (*(*r).con).dst_addr)
                    };
                current_block_22 = 11743904203796629665;
            } else {
                current_block_22 = 1118134448028020070;
            }
            match current_block_22 {
                11743904203796629665 => {}
                _ => {
                    match_0 ^= buffer_is_equal(l, &(*dc).string);
                }
            }
        }
        4 | 2 => {
            match_0 = ((*dc).cond as libc::c_uint
                == CONFIG_COND_MATCH as libc::c_int as libc::c_uint) as libc::c_int;
            match_0 ^= (config_pcre_match(r, dc, l) > 0 as libc::c_int) as libc::c_int;
        }
        _ => {
            match_0 = 1 as libc::c_int;
        }
    }
    (*cache)
        .local_result = (if match_0 != 0 {
        COND_RESULT_FALSE as libc::c_int
    } else {
        COND_RESULT_TRUE as libc::c_int
    }) as int8_t;
    return (*cache).local_result as cond_result_t;
}
#[inline(never)]
unsafe extern "C" fn config_check_cond_calc(
    r: *mut request_st,
    context_ndx: libc::c_int,
    cache: *mut cond_cache_t,
) -> cond_result_t {
    let dc: *const data_config = *(config_reference.data).offset(context_ndx as isize);
    let debug_cond: libc::c_int = (*r).conf.log_condition_handling as libc::c_int;
    if debug_cond != 0 {
        log_error(
            (*r).conf.errh,
            b"src/configfile-glue.c\0" as *const u8 as *const libc::c_char,
            576 as libc::c_int as libc::c_uint,
            b"=== start of condition block ===\0" as *const u8 as *const libc::c_char,
        );
    }
    return config_check_cond_nocache_calc(r, dc, debug_cond, cache);
}
#[no_mangle]
pub unsafe extern "C" fn config_check_cond(
    r: *mut request_st,
    context_ndx: libc::c_int,
) -> libc::c_int {
    let cache: *mut cond_cache_t = &mut *((*r).cond_cache).offset(context_ndx as isize)
        as *mut cond_cache_t;
    return (COND_RESULT_TRUE as libc::c_int as libc::c_uint
        == (if COND_RESULT_UNSET as libc::c_int != (*cache).result as libc::c_int {
            (*cache).result as cond_result_t as libc::c_uint
        } else {
            config_check_cond_calc(r, context_ndx, cache) as libc::c_uint
        })) as libc::c_int;
}
unsafe extern "C" fn config_cond_clear_node(
    cond_cache: *mut cond_cache_t,
    dc: *const data_config,
) {
    if (*cond_cache.offset((*dc).context_ndx as isize)).result as libc::c_int
        != COND_RESULT_UNSET as libc::c_int
    {
        (*cond_cache.offset((*dc).context_ndx as isize))
            .result = COND_RESULT_UNSET as libc::c_int as int8_t;
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while (i as libc::c_ulong) < (*dc).children.used {
            let mut dc_child: *const data_config = *((*dc).children.data)
                .offset(i as isize);
            if ((*dc_child).prev).is_null() {
                config_cond_clear_node(cond_cache, dc_child);
            }
            i = i.wrapping_add(1);
        }
        if !((*dc).next).is_null() {
            config_cond_clear_node(cond_cache, (*dc).next);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn config_cond_cache_reset_item(
    r: *mut request_st,
    mut item: comp_key_t,
) {
    let cond_cache: *mut cond_cache_t = (*r).cond_cache;
    let data: *const *const data_config = config_reference.data;
    let used: uint32_t = config_reference.used;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < used {
        let dc: *const data_config = *data.offset(i as isize);
        if item as libc::c_uint == (*dc).comp as libc::c_uint {
            (*cond_cache.offset(i as isize))
                .local_result = COND_RESULT_UNSET as libc::c_int as int8_t;
            config_cond_clear_node(cond_cache, dc);
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn config_cond_cache_reset(r: *mut request_st) {
    let used: uint32_t = config_reference.used;
    if used > 1 as libc::c_int as libc::c_uint {
        memset(
            (*r).cond_cache as *mut libc::c_void,
            0 as libc::c_int,
            (used as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cond_cache_t>() as libc::c_ulong),
        );
    }
}
unsafe extern "C" fn config_pcre_match(
    r: *mut request_st,
    dc: *const data_config,
    b: *const buffer,
) -> libc::c_int {
    if (0 as libc::c_int == (*dc).capture_idx) as libc::c_int as libc::c_long != 0 {
        return pcre2_match_8(
            (*dc).code as *const pcre2_code_8,
            (*b).ptr as PCRE2_SPTR8,
            buffer_clen(b) as size_t,
            0 as libc::c_int as size_t,
            0 as libc::c_int as uint32_t,
            (*dc).match_data,
            0 as *mut pcre2_match_context_8,
        );
    }
    let capture_offset: libc::c_int = (*dc).capture_idx - 1 as libc::c_int;
    let ref mut fresh1 = *((*r).cond_match).offset(capture_offset as isize);
    *fresh1 = ((*r).cond_match_data).offset(capture_offset as isize);
    let cond_match: *mut cond_match_t = *fresh1;
    let mut match_data: *mut pcre2_match_data_8 = (*cond_match).match_data;
    if (0 as *mut libc::c_void as *mut pcre2_match_data_8 == match_data) as libc::c_int
        as libc::c_long != 0
    {
        (*cond_match)
            .match_data = pcre2_match_data_create_from_pattern_8(
            (*dc).code as *const pcre2_code_8,
            0 as *mut pcre2_general_context_8,
        );
        match_data = (*cond_match).match_data;
        if match_data.is_null() {
            ck_assert_failed(
                b"src/configfile-glue.c\0" as *const u8 as *const libc::c_char,
                674 as libc::c_int as libc::c_uint,
                b"match_data\0" as *const u8 as *const libc::c_char,
            );
        }
        (*cond_match)
            .matches = pcre2_get_ovector_pointer_8(match_data) as *mut libc::c_void;
    }
    (*cond_match).comp_value = b;
    (*cond_match)
        .captures = pcre2_match_8(
        (*dc).code as *const pcre2_code_8,
        (*b).ptr as PCRE2_SPTR8,
        buffer_clen(b) as size_t,
        0 as libc::c_int as size_t,
        0 as libc::c_int as uint32_t,
        match_data,
        0 as *mut pcre2_match_context_8,
    );
    return (*cond_match).captures;
}
