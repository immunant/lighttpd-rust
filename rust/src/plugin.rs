use ::libc;
extern "C" {
    pub type fdnode_st;
    pub type stat_cache_entry;
    pub type pcre2_real_match_data_8;
    pub type h2con;
    pub type fdevents;
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
    fn array_free_data(a: *mut array);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn log_error(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        fmt: *const libc::c_char,
        _: ...
    );
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn mod_access_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_accesslog_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_ajp13_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_alias_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_auth_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_authn_file_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_cgi_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_deflate_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_dirlisting_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_evasive_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_evhost_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_expire_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_extforward_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_fastcgi_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_indexfile_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_proxy_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_redirect_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_rewrite_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_rrdtool_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_scgi_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_secdownload_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_setenv_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_simple_vhost_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_sockproxy_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_ssi_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_staticfile_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_status_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_uploadprogress_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_userdir_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_usertrack_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_vhostdb_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_webdav_plugin_init(p: *mut plugin) -> libc::c_int;
    fn mod_wstunnel_plugin_init(p: *mut plugin) -> libc::c_int;
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
pub struct plugin_data_base {
    pub id: libc::c_int,
    pub nconfig: libc::c_int,
    pub cvlist: *mut config_plugin_value_t,
    pub self_0: *mut plugin,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_load_functions {
    pub name: *const libc::c_char,
    pub plugin_init: Option::<unsafe extern "C" fn(*mut plugin) -> libc::c_int>,
}
pub const PLUGIN_FUNC_HANDLE_URI_CLEAN: C2RustUnnamed_5 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_fn_data {
    pub fn_0: Option::<unsafe extern "C" fn() -> handler_t>,
    pub data: *mut plugin_data_base,
}
pub const PLUGIN_FUNC_HANDLE_SUBREQUEST_START: C2RustUnnamed_5 = 3;
pub const PLUGIN_FUNC_HANDLE_RESPONSE_START: C2RustUnnamed_5 = 4;
pub const PLUGIN_FUNC_HANDLE_REQUEST_ENV: C2RustUnnamed_5 = 7;
pub const PLUGIN_FUNC_HANDLE_REQUEST_DONE: C2RustUnnamed_5 = 5;
pub const PLUGIN_FUNC_HANDLE_DOCROOT: C2RustUnnamed_5 = 1;
pub const PLUGIN_FUNC_HANDLE_PHYSICAL: C2RustUnnamed_5 = 2;
pub const PLUGIN_FUNC_HANDLE_REQUEST_RESET: C2RustUnnamed_5 = 6;
pub const PLUGIN_FUNC_HANDLE_CONNECTION_ACCEPT: C2RustUnnamed_5 = 8;
pub const PLUGIN_FUNC_HANDLE_CONNECTION_SHUT_WR: C2RustUnnamed_5 = 9;
pub const PLUGIN_FUNC_HANDLE_CONNECTION_CLOSE: C2RustUnnamed_5 = 10;
pub const PLUGIN_FUNC_HANDLE_TRIGGER: C2RustUnnamed_5 = 11;
pub const PLUGIN_FUNC_HANDLE_WAITPID: C2RustUnnamed_5 = 12;
pub const PLUGIN_FUNC_HANDLE_SIGHUP: C2RustUnnamed_5 = 13;
pub const PLUGIN_FUNC_WORKER_INIT: C2RustUnnamed_5 = 15;
pub const PLUGIN_FUNC_SET_DEFAULTS: C2RustUnnamed_5 = 14;
pub const PLUGIN_FUNC_SIZEOF: C2RustUnnamed_5 = 16;
pub type C2RustUnnamed_5 = libc::c_uint;
#[no_mangle]
pub static mut plugin_stats: array = array {
    data: 0 as *const *mut data_unset as *mut *mut data_unset,
    sorted: 0 as *const *mut data_unset as *mut *mut data_unset,
    used: 0,
    size: 0,
};
unsafe extern "C" fn plugin_init() -> *mut plugin {
    let mut p: *mut plugin = 0 as *mut plugin;
    p = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<plugin>() as libc::c_ulong,
    ) as *mut plugin;
    if p.is_null() {
        ck_assert_failed(
            b"src/plugin.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int as libc::c_uint,
            b"((void*)0) != p\0" as *const u8 as *const libc::c_char,
        );
    }
    return p;
}
unsafe extern "C" fn plugin_free(mut p: *mut plugin) {
    if p.is_null() {
        return;
    }
    free(p as *mut libc::c_void);
}
unsafe extern "C" fn plugins_register(mut srv: *mut server, mut p: *mut plugin) {
    let mut ps: *mut *mut plugin = 0 as *mut *mut plugin;
    if (*srv).plugins.used == (*srv).plugins.size {
        (*srv)
            .plugins
            .size = ((*srv).plugins.size as libc::c_uint)
            .wrapping_add(4 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
        (*srv)
            .plugins
            .ptr = realloc(
            (*srv).plugins.ptr,
            ((*srv).plugins.size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut plugin>() as libc::c_ulong),
        );
        if ((*srv).plugins.ptr).is_null() {
            ck_assert_failed(
                b"src/plugin.c\0" as *const u8 as *const libc::c_char,
                90 as libc::c_int as libc::c_uint,
                b"((void*)0) != srv->plugins.ptr\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    ps = (*srv).plugins.ptr as *mut *mut plugin;
    let fresh0 = (*srv).plugins.used;
    (*srv).plugins.used = ((*srv).plugins.used).wrapping_add(1);
    let ref mut fresh1 = *ps.offset(fresh0 as isize);
    *fresh1 = p;
}
static mut load_functions: [plugin_load_functions; 34] = [
    {
        let mut init = plugin_load_functions {
            name: b"mod_access\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_access_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_accesslog\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_accesslog_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_ajp13\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_ajp13_plugin_init as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_alias\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_alias_plugin_init as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_auth\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_auth_plugin_init as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_authn_file\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_authn_file_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_cgi\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_cgi_plugin_init as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_deflate\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_deflate_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_dirlisting\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_dirlisting_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_evasive\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_evasive_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_evhost\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_evhost_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_expire\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_expire_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_extforward\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_extforward_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_fastcgi\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_fastcgi_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_indexfile\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_indexfile_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_proxy\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_proxy_plugin_init as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_redirect\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_redirect_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_rewrite\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_rewrite_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_rrdtool\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_rrdtool_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_scgi\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_scgi_plugin_init as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_secdownload\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_secdownload_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_setenv\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_setenv_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_simple_vhost\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_simple_vhost_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_sockproxy\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_sockproxy_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_ssi\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_ssi_plugin_init as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_staticfile\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_staticfile_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_status\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_status_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_uploadprogress\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_uploadprogress_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_userdir\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_userdir_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_usertrack\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_usertrack_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_vhostdb\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_vhostdb_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_webdav\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_webdav_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: b"mod_wstunnel\0" as *const u8 as *const libc::c_char,
            plugin_init: Some(
                mod_wstunnel_plugin_init
                    as unsafe extern "C" fn(*mut plugin) -> libc::c_int,
            ),
        };
        init
    },
    {
        let mut init = plugin_load_functions {
            name: 0 as *const libc::c_char,
            plugin_init: None,
        };
        init
    },
];
#[no_mangle]
#[cold]
pub unsafe extern "C" fn plugins_load(mut srv: *mut server) -> libc::c_int {
    let mut current_block_9: u64;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*(*srv).srvconf.modules).used {
        let mut ds: *mut data_string = *((*(*srv).srvconf.modules).data)
            .offset(i as isize) as *mut data_string;
        let mut module: *mut libc::c_char = (*ds).value.ptr;
        let mut j: uint32_t = 0;
        j = 0 as libc::c_int as uint32_t;
        while !(load_functions[j as usize].name).is_null() {
            if 0 as libc::c_int == strcmp(load_functions[j as usize].name, module) {
                let p: *mut plugin = plugin_init();
                if (Some(
                    ((*load_functions.as_ptr().offset(j as isize)).plugin_init)
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(p) != 0
                {
                    log_error(
                        (*srv).errh,
                        b"src/plugin.c\0" as *const u8 as *const libc::c_char,
                        140 as libc::c_int as libc::c_uint,
                        b"%s plugin init failed\0" as *const u8 as *const libc::c_char,
                        module,
                    );
                    plugin_free(p);
                    return -(1 as libc::c_int);
                }
                plugins_register(srv, p);
                break;
            } else {
                j = j.wrapping_add(1);
            }
        }
        if (load_functions[j as usize].name).is_null() {
            log_error(
                (*srv).errh,
                b"src/plugin.c\0" as *const u8 as *const libc::c_char,
                149 as libc::c_int as libc::c_uint,
                b"%s plugin not found\0" as *const u8 as *const libc::c_char,
                module,
            );
            if (*srv).srvconf.compat_module_load != 0 {
                if buffer_eq_slen(
                    &mut (*ds).value,
                    b"mod_deflate\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                ) != 0
                {
                    current_block_9 = 9386390421034826751;
                } else {
                    current_block_9 = 6009453772311597924;
                }
            } else {
                current_block_9 = 6009453772311597924;
            }
            match current_block_9 {
                9386390421034826751 => {}
                _ => return -(1 as libc::c_int),
            }
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn plugins_call_fn_req_data(
    r: *mut request_st,
    e: libc::c_int,
) -> handler_t {
    let plugin_slots: *const libc::c_void = (*(*r).con).plugin_slots;
    let offset: uint32_t = *(plugin_slots as *const uint16_t).offset(e as isize)
        as uint32_t;
    if 0 as libc::c_int as libc::c_uint == offset {
        return HANDLER_GO_ON;
    }
    let mut plfd: *const plugin_fn_data = (plugin_slots as uintptr_t)
        .wrapping_add(offset as libc::c_ulong) as *const plugin_fn_data;
    let mut rc: handler_t = HANDLER_GO_ON;
    while ((*plfd).fn_0).is_some()
        && {
            rc = ::std::mem::transmute::<
                _,
                fn(_, _) -> handler_t,
            >(((*plfd).fn_0).expect("non-null function pointer"))(r, (*plfd).data);
            rc as libc::c_uint == HANDLER_GO_ON as libc::c_int as libc::c_uint
        }
    {
        plfd = plfd.offset(1);
    }
    return rc;
}
unsafe extern "C" fn plugins_call_fn_con_data(
    con: *mut connection,
    e: libc::c_int,
) -> handler_t {
    let plugin_slots: *const libc::c_void = (*con).plugin_slots;
    let offset: uint32_t = *(plugin_slots as *const uint16_t).offset(e as isize)
        as uint32_t;
    if 0 as libc::c_int as libc::c_uint == offset {
        return HANDLER_GO_ON;
    }
    let mut plfd: *const plugin_fn_data = (plugin_slots as uintptr_t)
        .wrapping_add(offset as libc::c_ulong) as *const plugin_fn_data;
    let mut rc: handler_t = HANDLER_GO_ON;
    while ((*plfd).fn_0).is_some()
        && {
            rc = ::std::mem::transmute::<
                _,
                fn(_, _) -> handler_t,
            >(((*plfd).fn_0).expect("non-null function pointer"))(con, (*plfd).data);
            rc as libc::c_uint == HANDLER_GO_ON as libc::c_int as libc::c_uint
        }
    {
        plfd = plfd.offset(1);
    }
    return rc;
}
unsafe extern "C" fn plugins_call_fn_srv_data(
    srv: *mut server,
    e: libc::c_int,
) -> handler_t {
    let offset: uint32_t = *((*srv).plugin_slots as *const uint16_t).offset(e as isize)
        as uint32_t;
    if 0 as libc::c_int as libc::c_uint == offset {
        return HANDLER_GO_ON;
    }
    let mut plfd: *const plugin_fn_data = ((*srv).plugin_slots as uintptr_t)
        .wrapping_add(offset as libc::c_ulong) as *const plugin_fn_data;
    let mut rc: handler_t = HANDLER_GO_ON;
    while ((*plfd).fn_0).is_some()
        && {
            rc = ::std::mem::transmute::<
                _,
                fn(_, _) -> handler_t,
            >(((*plfd).fn_0).expect("non-null function pointer"))(srv, (*plfd).data);
            rc as libc::c_uint == HANDLER_GO_ON as libc::c_int as libc::c_uint
        }
    {
        plfd = plfd.offset(1);
    }
    return rc;
}
unsafe extern "C" fn plugins_call_fn_srv_data_all(srv: *mut server, e: libc::c_int) {
    let offset: uint32_t = *((*srv).plugin_slots as *const uint16_t).offset(e as isize)
        as uint32_t;
    if 0 as libc::c_int as libc::c_uint == offset {
        return;
    }
    let mut plfd: *const plugin_fn_data = ((*srv).plugin_slots as uintptr_t)
        .wrapping_add(offset as libc::c_ulong) as *const plugin_fn_data;
    while ((*plfd).fn_0).is_some() {
        ::std::mem::transmute::<
            _,
            fn(_, _) -> handler_t,
        >(((*plfd).fn_0).expect("non-null function pointer"))(srv, (*plfd).data);
        plfd = plfd.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn plugins_call_handle_uri_clean(r: *mut request_st) -> handler_t {
    return plugins_call_fn_req_data(r, PLUGIN_FUNC_HANDLE_URI_CLEAN as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn plugins_call_handle_docroot(r: *mut request_st) -> handler_t {
    return plugins_call_fn_req_data(r, PLUGIN_FUNC_HANDLE_DOCROOT as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn plugins_call_handle_physical(r: *mut request_st) -> handler_t {
    return plugins_call_fn_req_data(r, PLUGIN_FUNC_HANDLE_PHYSICAL as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn plugins_call_handle_subrequest_start(
    r: *mut request_st,
) -> handler_t {
    return plugins_call_fn_req_data(
        r,
        PLUGIN_FUNC_HANDLE_SUBREQUEST_START as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn plugins_call_handle_response_start(
    r: *mut request_st,
) -> handler_t {
    return plugins_call_fn_req_data(r, PLUGIN_FUNC_HANDLE_RESPONSE_START as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn plugins_call_handle_request_done(
    r: *mut request_st,
) -> handler_t {
    return plugins_call_fn_req_data(r, PLUGIN_FUNC_HANDLE_REQUEST_DONE as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn plugins_call_handle_request_reset(
    r: *mut request_st,
) -> handler_t {
    return plugins_call_fn_req_data(r, PLUGIN_FUNC_HANDLE_REQUEST_RESET as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn plugins_call_handle_request_env(
    r: *mut request_st,
) -> handler_t {
    return plugins_call_fn_req_data(r, PLUGIN_FUNC_HANDLE_REQUEST_ENV as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn plugins_call_handle_connection_accept(
    mut con: *mut connection,
) -> handler_t {
    return plugins_call_fn_con_data(
        con,
        PLUGIN_FUNC_HANDLE_CONNECTION_ACCEPT as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn plugins_call_handle_connection_shut_wr(
    mut con: *mut connection,
) -> handler_t {
    return plugins_call_fn_con_data(
        con,
        PLUGIN_FUNC_HANDLE_CONNECTION_SHUT_WR as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn plugins_call_handle_connection_close(
    mut con: *mut connection,
) -> handler_t {
    return plugins_call_fn_con_data(
        con,
        PLUGIN_FUNC_HANDLE_CONNECTION_CLOSE as libc::c_int,
    );
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn plugins_call_set_defaults(mut srv: *mut server) -> handler_t {
    return plugins_call_fn_srv_data(srv, PLUGIN_FUNC_SET_DEFAULTS as libc::c_int);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn plugins_call_worker_init(mut srv: *mut server) -> handler_t {
    return plugins_call_fn_srv_data(srv, PLUGIN_FUNC_WORKER_INIT as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn plugins_call_handle_trigger(mut srv: *mut server) {
    plugins_call_fn_srv_data_all(srv, PLUGIN_FUNC_HANDLE_TRIGGER as libc::c_int);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn plugins_call_handle_sighup(mut srv: *mut server) {
    plugins_call_fn_srv_data_all(srv, PLUGIN_FUNC_HANDLE_SIGHUP as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn plugins_call_handle_waitpid(
    mut srv: *mut server,
    mut pid: pid_t,
    mut status: libc::c_int,
) -> handler_t {
    let offset: uint32_t = *((*srv).plugin_slots as *const uint16_t)
        .offset(PLUGIN_FUNC_HANDLE_WAITPID as libc::c_int as isize) as uint32_t;
    if 0 as libc::c_int as libc::c_uint == offset {
        return HANDLER_GO_ON;
    }
    let mut plfd: *const plugin_fn_data = ((*srv).plugin_slots as uintptr_t)
        .wrapping_add(offset as libc::c_ulong) as *const plugin_fn_data;
    let mut rc: handler_t = HANDLER_GO_ON;
    while ((*plfd).fn_0).is_some()
        && {
            rc = ::std::mem::transmute::<
                _,
                fn(_, _, _, _) -> handler_t,
            >(
                ((*plfd).fn_0).expect("non-null function pointer"),
            )(srv, (*plfd).data, pid, status);
            rc as libc::c_uint == HANDLER_GO_ON as libc::c_int as libc::c_uint
        }
    {
        plfd = plfd.offset(1);
    }
    return rc;
}
unsafe extern "C" fn plugins_call_cleanup(srv: *mut server) {
    let ps: *mut *mut plugin = (*srv).plugins.ptr as *mut *mut plugin;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*srv).plugins.used {
        let mut p: *mut plugin = *ps.offset(i as isize);
        if !p.is_null() {
            if !((*p).data).is_null() {
                let mut pd: *mut plugin_data_base = (*p).data as *mut plugin_data_base;
                if ((*p).cleanup).is_some() {
                    ((*p).cleanup).expect("non-null function pointer")((*p).data);
                }
                free((*pd).cvlist as *mut libc::c_void);
                free(pd as *mut libc::c_void);
                (*p).data = 0 as *mut libc::c_void;
            }
        }
        i = i.wrapping_add(1);
    }
}
#[cold]
unsafe extern "C" fn plugins_call_init_reverse(mut srv: *mut server, offset: uint32_t) {
    if 0 as libc::c_int as libc::c_uint == offset {
        return;
    }
    let mut a: *mut plugin_fn_data = ((*srv).plugin_slots as uintptr_t)
        .wrapping_add(offset as libc::c_ulong) as *mut plugin_fn_data;
    let mut b: *mut plugin_fn_data = a;
    while ((*b).fn_0).is_some() {
        b = b.offset(1);
    }
    loop {
        b = b.offset(-1);
        if !(a < b) {
            break;
        }
        let mut tmp: plugin_fn_data = *a;
        *a = *b;
        *b = tmp;
        a = a.offset(1);
    };
}
#[cold]
unsafe extern "C" fn plugins_call_init_slot(
    mut srv: *mut server,
    mut fn_0: Option::<unsafe extern "C" fn() -> handler_t>,
    mut data: *mut libc::c_void,
    offset: uint32_t,
) {
    if fn_0.is_some() {
        let mut plfd: *mut plugin_fn_data = ((*srv).plugin_slots as uintptr_t)
            .wrapping_add(offset as libc::c_ulong) as *mut plugin_fn_data;
        while ((*plfd).fn_0).is_some() {
            plfd = plfd.offset(1);
        }
        (*plfd).fn_0 = fn_0;
        (*plfd).data = data as *mut plugin_data_base;
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn plugins_call_init(mut srv: *mut server) -> handler_t {
    let ps: *mut *mut plugin = (*srv).plugins.ptr as *mut *mut plugin;
    let mut offsets: [uint16_t; 16] = [0; 16];
    memset(
        offsets.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
    );
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*srv).plugins.used {
        let mut p: *mut plugin = *ps.offset(i as isize);
        if ((*p).init).is_some() {
            (*p)
                .data = ::std::mem::transmute::<
                _,
                fn() -> *mut libc::c_void,
            >(((*p).init).expect("non-null function pointer"))();
            if ((*p).data).is_null() {
                log_error(
                    (*srv).errh,
                    b"src/plugin.c\0" as *const u8 as *const libc::c_char,
                    454 as libc::c_int as libc::c_uint,
                    b"plugin-init failed for module %s\0" as *const u8
                        as *const libc::c_char,
                    (*p).name,
                );
                return HANDLER_ERROR;
            }
            let ref mut fresh2 = (*((*p).data as *mut plugin_data_base)).self_0;
            *fresh2 = p;
            (*((*p).data as *mut plugin_data_base))
                .id = i.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
            if (*p).version != 0x10440 as libc::c_int as libc::c_ulong {
                log_error(
                    (*srv).errh,
                    b"src/plugin.c\0" as *const u8 as *const libc::c_char,
                    463 as libc::c_int as libc::c_uint,
                    b"plugin-version doesn't match lighttpd-version for %s\0"
                        as *const u8 as *const libc::c_char,
                    (*p).name,
                );
                return HANDLER_ERROR;
            }
        }
        if ((*p).priv_defaults).is_some()
            && HANDLER_ERROR as libc::c_int as libc::c_uint
                == ((*p).priv_defaults)
                    .expect("non-null function pointer")(srv, (*p).data) as libc::c_uint
        {
            return HANDLER_ERROR;
        }
        if ((*p).handle_uri_clean).is_some() {
            offsets[PLUGIN_FUNC_HANDLE_URI_CLEAN as libc::c_int
                as usize] = (offsets[PLUGIN_FUNC_HANDLE_URI_CLEAN as libc::c_int
                as usize])
                .wrapping_add(1);
        }
        if ((*p).handle_uri_raw).is_some() && ((*p).handle_uri_clean).is_none() {
            offsets[PLUGIN_FUNC_HANDLE_URI_CLEAN as libc::c_int
                as usize] = (offsets[PLUGIN_FUNC_HANDLE_URI_CLEAN as libc::c_int
                as usize])
                .wrapping_add(1);
        }
        if ((*p).handle_request_env).is_some() {
            offsets[PLUGIN_FUNC_HANDLE_REQUEST_ENV as libc::c_int
                as usize] = (offsets[PLUGIN_FUNC_HANDLE_REQUEST_ENV as libc::c_int
                as usize])
                .wrapping_add(1);
        }
        if ((*p).handle_request_done).is_some() {
            offsets[PLUGIN_FUNC_HANDLE_REQUEST_DONE as libc::c_int
                as usize] = (offsets[PLUGIN_FUNC_HANDLE_REQUEST_DONE as libc::c_int
                as usize])
                .wrapping_add(1);
        }
        if ((*p).handle_connection_accept).is_some() {
            offsets[PLUGIN_FUNC_HANDLE_CONNECTION_ACCEPT as libc::c_int
                as usize] = (offsets[PLUGIN_FUNC_HANDLE_CONNECTION_ACCEPT as libc::c_int
                as usize])
                .wrapping_add(1);
        }
        if ((*p).handle_connection_shut_wr).is_some() {
            offsets[PLUGIN_FUNC_HANDLE_CONNECTION_SHUT_WR as libc::c_int
                as usize] = (offsets[PLUGIN_FUNC_HANDLE_CONNECTION_SHUT_WR as libc::c_int
                as usize])
                .wrapping_add(1);
        }
        if ((*p).handle_connection_close).is_some() {
            offsets[PLUGIN_FUNC_HANDLE_CONNECTION_CLOSE as libc::c_int
                as usize] = (offsets[PLUGIN_FUNC_HANDLE_CONNECTION_CLOSE as libc::c_int
                as usize])
                .wrapping_add(1);
        }
        if ((*p).handle_trigger).is_some() {
            offsets[PLUGIN_FUNC_HANDLE_TRIGGER as libc::c_int
                as usize] = (offsets[PLUGIN_FUNC_HANDLE_TRIGGER as libc::c_int as usize])
                .wrapping_add(1);
        }
        if ((*p).handle_sighup).is_some() {
            offsets[PLUGIN_FUNC_HANDLE_SIGHUP as libc::c_int
                as usize] = (offsets[PLUGIN_FUNC_HANDLE_SIGHUP as libc::c_int as usize])
                .wrapping_add(1);
        }
        if ((*p).handle_waitpid).is_some() {
            offsets[PLUGIN_FUNC_HANDLE_WAITPID as libc::c_int
                as usize] = (offsets[PLUGIN_FUNC_HANDLE_WAITPID as libc::c_int as usize])
                .wrapping_add(1);
        }
        if ((*p).handle_subrequest_start).is_some() {
            offsets[PLUGIN_FUNC_HANDLE_SUBREQUEST_START as libc::c_int
                as usize] = (offsets[PLUGIN_FUNC_HANDLE_SUBREQUEST_START as libc::c_int
                as usize])
                .wrapping_add(1);
        }
        if ((*p).handle_response_start).is_some() {
            offsets[PLUGIN_FUNC_HANDLE_RESPONSE_START as libc::c_int
                as usize] = (offsets[PLUGIN_FUNC_HANDLE_RESPONSE_START as libc::c_int
                as usize])
                .wrapping_add(1);
        }
        if ((*p).handle_docroot).is_some() {
            offsets[PLUGIN_FUNC_HANDLE_DOCROOT as libc::c_int
                as usize] = (offsets[PLUGIN_FUNC_HANDLE_DOCROOT as libc::c_int as usize])
                .wrapping_add(1);
        }
        if ((*p).handle_physical).is_some() {
            offsets[PLUGIN_FUNC_HANDLE_PHYSICAL as libc::c_int
                as usize] = (offsets[PLUGIN_FUNC_HANDLE_PHYSICAL as libc::c_int
                as usize])
                .wrapping_add(1);
        }
        if ((*p).handle_request_reset).is_some() {
            offsets[PLUGIN_FUNC_HANDLE_REQUEST_RESET as libc::c_int
                as usize] = (offsets[PLUGIN_FUNC_HANDLE_REQUEST_RESET as libc::c_int
                as usize])
                .wrapping_add(1);
        }
        if ((*p).set_defaults).is_some() {
            offsets[PLUGIN_FUNC_SET_DEFAULTS as libc::c_int
                as usize] = (offsets[PLUGIN_FUNC_SET_DEFAULTS as libc::c_int as usize])
                .wrapping_add(1);
        }
        if ((*p).worker_init).is_some() {
            offsets[PLUGIN_FUNC_WORKER_INIT as libc::c_int
                as usize] = (offsets[PLUGIN_FUNC_WORKER_INIT as libc::c_int as usize])
                .wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    let mut nslots: uint32_t = (::std::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<plugin_fn_data>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<plugin_fn_data>() as libc::c_ulong)
        as uint32_t;
    let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
    while i_0 < PLUGIN_FUNC_SIZEOF as libc::c_int as libc::c_uint {
        if offsets[i_0 as usize] != 0 {
            let mut offset: uint32_t = nslots;
            nslots = (nslots as libc::c_uint)
                .wrapping_add(
                    (offsets[i_0 as usize] as libc::c_int + 1 as libc::c_int)
                        as libc::c_uint,
                ) as uint32_t as uint32_t;
            if !((offset as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<plugin_fn_data>() as libc::c_ulong)
                <= (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                    as libc::c_ulong)
            {
                ck_assert_failed(
                    b"src/plugin.c\0" as *const u8 as *const libc::c_char,
                    515 as libc::c_int as libc::c_uint,
                    b"offset * sizeof(plugin_fn_data) <= (32767 *2 +1)\0" as *const u8
                        as *const libc::c_char,
                );
            }
            offsets[i_0
                as usize] = (offset as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<plugin_fn_data>() as libc::c_ulong)
                as uint16_t;
        }
        i_0 = i_0.wrapping_add(1);
    }
    (*srv)
        .plugin_slots = calloc(
        nslots as libc::c_ulong,
        ::std::mem::size_of::<plugin_fn_data>() as libc::c_ulong,
    );
    if ((*srv).plugin_slots).is_null() {
        ck_assert_failed(
            b"src/plugin.c\0" as *const u8 as *const libc::c_char,
            522 as libc::c_int as libc::c_uint,
            b"((void*)0) != srv->plugin_slots\0" as *const u8 as *const libc::c_char,
        );
    }
    memcpy(
        (*srv).plugin_slots,
        offsets.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
    );
    let mut i_1: uint32_t = 0 as libc::c_int as uint32_t;
    while i_1 < (*srv).plugins.used {
        let p_0: *mut plugin = *ps.offset(i_1 as isize);
        plugins_call_init_slot(
            srv,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
                >,
                Option::<unsafe extern "C" fn() -> handler_t>,
            >((*p_0).handle_uri_raw),
            (*p_0).data,
            offsets[PLUGIN_FUNC_HANDLE_URI_CLEAN as libc::c_int as usize] as uint32_t,
        );
        i_1 = i_1.wrapping_add(1);
    }
    let mut i_2: uint32_t = 0 as libc::c_int as uint32_t;
    while i_2 < (*srv).plugins.used {
        let p_1: *mut plugin = *ps.offset(i_2 as isize);
        if ((*p_1).handle_uri_raw).is_none() {
            plugins_call_init_slot(
                srv,
                ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut request_st,
                            *mut libc::c_void,
                        ) -> handler_t,
                    >,
                    Option::<unsafe extern "C" fn() -> handler_t>,
                >((*p_1).handle_uri_clean),
                (*p_1).data,
                offsets[PLUGIN_FUNC_HANDLE_URI_CLEAN as libc::c_int as usize] as uint32_t,
            );
        }
        plugins_call_init_slot(
            srv,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
                >,
                Option::<unsafe extern "C" fn() -> handler_t>,
            >((*p_1).handle_request_env),
            (*p_1).data,
            offsets[PLUGIN_FUNC_HANDLE_REQUEST_ENV as libc::c_int as usize] as uint32_t,
        );
        plugins_call_init_slot(
            srv,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
                >,
                Option::<unsafe extern "C" fn() -> handler_t>,
            >((*p_1).handle_request_done),
            (*p_1).data,
            offsets[PLUGIN_FUNC_HANDLE_REQUEST_DONE as libc::c_int as usize] as uint32_t,
        );
        plugins_call_init_slot(
            srv,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut connection, *mut libc::c_void) -> handler_t,
                >,
                Option::<unsafe extern "C" fn() -> handler_t>,
            >((*p_1).handle_connection_accept),
            (*p_1).data,
            offsets[PLUGIN_FUNC_HANDLE_CONNECTION_ACCEPT as libc::c_int as usize]
                as uint32_t,
        );
        plugins_call_init_slot(
            srv,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut connection, *mut libc::c_void) -> handler_t,
                >,
                Option::<unsafe extern "C" fn() -> handler_t>,
            >((*p_1).handle_connection_shut_wr),
            (*p_1).data,
            offsets[PLUGIN_FUNC_HANDLE_CONNECTION_SHUT_WR as libc::c_int as usize]
                as uint32_t,
        );
        plugins_call_init_slot(
            srv,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut connection, *mut libc::c_void) -> handler_t,
                >,
                Option::<unsafe extern "C" fn() -> handler_t>,
            >((*p_1).handle_connection_close),
            (*p_1).data,
            offsets[PLUGIN_FUNC_HANDLE_CONNECTION_CLOSE as libc::c_int as usize]
                as uint32_t,
        );
        plugins_call_init_slot(
            srv,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
                >,
                Option::<unsafe extern "C" fn() -> handler_t>,
            >((*p_1).handle_trigger),
            (*p_1).data,
            offsets[PLUGIN_FUNC_HANDLE_TRIGGER as libc::c_int as usize] as uint32_t,
        );
        plugins_call_init_slot(
            srv,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
                >,
                Option::<unsafe extern "C" fn() -> handler_t>,
            >((*p_1).handle_sighup),
            (*p_1).data,
            offsets[PLUGIN_FUNC_HANDLE_SIGHUP as libc::c_int as usize] as uint32_t,
        );
        plugins_call_init_slot(
            srv,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut server,
                        *mut libc::c_void,
                        pid_t,
                        libc::c_int,
                    ) -> handler_t,
                >,
                Option::<unsafe extern "C" fn() -> handler_t>,
            >((*p_1).handle_waitpid),
            (*p_1).data,
            offsets[PLUGIN_FUNC_HANDLE_WAITPID as libc::c_int as usize] as uint32_t,
        );
        plugins_call_init_slot(
            srv,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
                >,
                Option::<unsafe extern "C" fn() -> handler_t>,
            >((*p_1).handle_subrequest_start),
            (*p_1).data,
            offsets[PLUGIN_FUNC_HANDLE_SUBREQUEST_START as libc::c_int as usize]
                as uint32_t,
        );
        plugins_call_init_slot(
            srv,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
                >,
                Option::<unsafe extern "C" fn() -> handler_t>,
            >((*p_1).handle_response_start),
            (*p_1).data,
            offsets[PLUGIN_FUNC_HANDLE_RESPONSE_START as libc::c_int as usize]
                as uint32_t,
        );
        plugins_call_init_slot(
            srv,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
                >,
                Option::<unsafe extern "C" fn() -> handler_t>,
            >((*p_1).handle_docroot),
            (*p_1).data,
            offsets[PLUGIN_FUNC_HANDLE_DOCROOT as libc::c_int as usize] as uint32_t,
        );
        plugins_call_init_slot(
            srv,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
                >,
                Option::<unsafe extern "C" fn() -> handler_t>,
            >((*p_1).handle_physical),
            (*p_1).data,
            offsets[PLUGIN_FUNC_HANDLE_PHYSICAL as libc::c_int as usize] as uint32_t,
        );
        plugins_call_init_slot(
            srv,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
                >,
                Option::<unsafe extern "C" fn() -> handler_t>,
            >((*p_1).handle_request_reset),
            (*p_1).data,
            offsets[PLUGIN_FUNC_HANDLE_REQUEST_RESET as libc::c_int as usize] as uint32_t,
        );
        plugins_call_init_slot(
            srv,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
                >,
                Option::<unsafe extern "C" fn() -> handler_t>,
            >((*p_1).set_defaults),
            (*p_1).data,
            offsets[PLUGIN_FUNC_SET_DEFAULTS as libc::c_int as usize] as uint32_t,
        );
        plugins_call_init_slot(
            srv,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
                >,
                Option::<unsafe extern "C" fn() -> handler_t>,
            >((*p_1).worker_init),
            (*p_1).data,
            offsets[PLUGIN_FUNC_WORKER_INIT as libc::c_int as usize] as uint32_t,
        );
        i_2 = i_2.wrapping_add(1);
    }
    plugins_call_init_reverse(
        srv,
        offsets[PLUGIN_FUNC_HANDLE_REQUEST_RESET as libc::c_int as usize] as uint32_t,
    );
    plugins_call_init_reverse(
        srv,
        offsets[PLUGIN_FUNC_HANDLE_CONNECTION_CLOSE as libc::c_int as usize] as uint32_t,
    );
    return HANDLER_GO_ON;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn plugins_free(mut srv: *mut server) {
    if !((*srv).plugin_slots).is_null() {
        plugins_call_cleanup(srv);
        free((*srv).plugin_slots);
        (*srv).plugin_slots = 0 as *mut libc::c_void;
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*srv).plugins.used {
        plugin_free(*((*srv).plugins.ptr as *mut *mut plugin).offset(i as isize));
        i = i.wrapping_add(1);
    }
    free((*srv).plugins.ptr);
    (*srv).plugins.ptr = 0 as *mut libc::c_void;
    (*srv).plugins.used = 0 as libc::c_int as uint32_t;
    (*srv).plugins.size = 0 as libc::c_int as uint32_t;
    array_free_data(&mut plugin_stats);
}
