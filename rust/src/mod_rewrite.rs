use ::libc;
extern "C" {
    pub type fdnode_st;
    pub type pcre2_real_match_data_8;
    pub type h2con;
    pub type fdevents;
    pub type pcre_keyvalue;
    fn buffer_free_ptr(b: *mut buffer);
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn sock_addr_get_port(saddr: *const sock_addr) -> libc::c_ushort;
    fn pcre_keyvalue_buffer_init() -> *mut pcre_keyvalue_buffer;
    fn pcre_keyvalue_buffer_append(
        errh: *mut log_error_st,
        kvb: *mut pcre_keyvalue_buffer,
        key: *const buffer,
        value: *const buffer,
        pcre_jit: libc::c_int,
    ) -> libc::c_int;
    fn pcre_keyvalue_buffer_free(kvb: *mut pcre_keyvalue_buffer);
    fn pcre_keyvalue_buffer_process(
        kvb: *const pcre_keyvalue_buffer,
        ctx: *mut pcre_keyvalue_ctx,
        input: *const buffer,
        result: *mut buffer,
    ) -> handler_t;
    fn pcre_keyvalue_burl_normalize_key(k: *mut buffer, t: *mut buffer);
    fn pcre_keyvalue_burl_normalize_value(v: *mut buffer, t: *mut buffer);
    fn log_error(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        fmt: *const libc::c_char,
        _: ...
    );
    fn config_get_config_cond_info(cfginfo: *mut config_cond_info, idx: uint32_t);
    fn config_capture(srv: *mut server, idx: libc::c_int) -> libc::c_int;
    fn config_plugin_values_init(
        srv: *mut server,
        p_d: *mut libc::c_void,
        cpk: *const config_plugin_keys_t,
        mname: *const libc::c_char,
    ) -> libc::c_int;
    fn config_check_cond(r: *mut request_st, context_ndx: libc::c_int) -> libc::c_int;
    fn config_feature_bool(
        srv: *const server,
        feature: *const libc::c_char,
        default_value: libc::c_int,
    ) -> libc::c_int;
    fn stat_cache_path_stat(name: *const buffer) -> *const stat_cache_st;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
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
pub struct burl_parts_t {
    pub scheme: *const buffer,
    pub authority: *const buffer,
    pub port: libc::c_ushort,
    pub path: *const buffer,
    pub query: *const buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pcre_keyvalue_ctx {
    pub cache: *mut cond_match_t,
    pub burl: *mut burl_parts_t,
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub ovec: *mut libc::c_void,
    pub subject: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pcre_keyvalue_buffer {
    pub kv: *mut pcre_keyvalue,
    pub used: uint32_t,
    pub x0: libc::c_int,
    pub x1: libc::c_int,
    pub cfgidx: libc::c_int,
}
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
pub type stat_cache_st = stat;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_config {
    pub rewrite: *mut pcre_keyvalue_buffer,
    pub rewrite_NF: *mut pcre_keyvalue_buffer,
}
pub type C2RustUnnamed_6 = libc::c_uint;
pub const REWRITE_STATE_FINISHED: C2RustUnnamed_6 = 2048;
pub const REWRITE_STATE_REWRITTEN: C2RustUnnamed_6 = 1024;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_data {
    pub id: libc::c_int,
    pub nconfig: libc::c_int,
    pub cvlist: *mut config_plugin_value_t,
    pub self_0: *mut plugin,
    pub defaults: plugin_config,
    pub conf: plugin_config,
}
#[inline]
unsafe extern "C" fn buffer_reset(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
    if (*b).size > 4096 as libc::c_int as libc::c_uint {
        buffer_free_ptr(b);
    }
}
#[inline]
unsafe extern "C" fn light_isdigit(mut c: libc::c_int) -> libc::c_int {
    return ((c as uint32_t).wrapping_sub('0' as i32 as libc::c_uint)
        <= ('9' as i32 - '0' as i32) as libc::c_uint) as libc::c_int;
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
#[cold]
unsafe extern "C" fn mod_rewrite_init() -> *mut libc::c_void {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<plugin_data>() as libc::c_ulong,
    );
}
#[cold]
unsafe extern "C" fn mod_rewrite_free(mut p_d: *mut libc::c_void) {
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
        let mut kvb: *mut pcre_keyvalue_buffer = 0 as *mut pcre_keyvalue_buffer;
        let mut kvb_NF: *mut pcre_keyvalue_buffer = 0 as *mut pcre_keyvalue_buffer;
        while -(1 as libc::c_int) != (*cpv).k_id {
            let mut current_block_5: u64;
            match (*cpv).k_id {
                1 => {
                    current_block_5 = 18268539312520417794;
                }
                2 => {
                    current_block_5 = 18268539312520417794;
                }
                0 | 3 => {
                    current_block_5 = 9959828968968616484;
                }
                4 | 5 => {
                    if (*cpv).vtype as libc::c_uint
                        == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
                    {
                        kvb_NF = (*cpv).v.v as *mut pcre_keyvalue_buffer;
                    }
                    current_block_5 = 10048703153582371463;
                }
                _ => {
                    current_block_5 = 10048703153582371463;
                }
            }
            match current_block_5 {
                18268539312520417794 => {
                    current_block_5 = 9959828968968616484;
                }
                _ => {}
            }
            match current_block_5 {
                9959828968968616484 => {
                    if (*cpv).vtype as libc::c_uint
                        == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
                    {
                        kvb = (*cpv).v.v as *mut pcre_keyvalue_buffer;
                    }
                }
                _ => {}
            }
            cpv = cpv.offset(1);
        }
        if !kvb.is_null() {
            pcre_keyvalue_buffer_free(kvb);
        }
        if !kvb_NF.is_null() {
            pcre_keyvalue_buffer_free(kvb_NF);
        }
        i += 1;
    }
}
unsafe extern "C" fn mod_rewrite_merge_config_cpv(
    pconf: *mut plugin_config,
    cpv: *const config_plugin_value_t,
) {
    let mut current_block_2: u64;
    match (*cpv).k_id {
        0 => {
            current_block_2 = 5851633680733726149;
        }
        1 => {
            current_block_2 = 5851633680733726149;
        }
        2 => {
            current_block_2 = 6181402094927065493;
        }
        3 => {
            current_block_2 = 3604972210128459352;
        }
        4 | 5 => {
            (*pconf).rewrite_NF = (*cpv).v.v as *mut pcre_keyvalue_buffer;
            current_block_2 = 2473556513754201174;
        }
        _ => return,
    }
    match current_block_2 {
        5851633680733726149 => {
            current_block_2 = 6181402094927065493;
        }
        _ => {}
    }
    match current_block_2 {
        6181402094927065493 => {
            current_block_2 = 3604972210128459352;
        }
        _ => {}
    }
    match current_block_2 {
        3604972210128459352 => {
            (*pconf).rewrite = (*cpv).v.v as *mut pcre_keyvalue_buffer;
        }
        _ => {}
    };
}
unsafe extern "C" fn mod_rewrite_merge_config(
    pconf: *mut plugin_config,
    mut cpv: *const config_plugin_value_t,
) {
    loop {
        mod_rewrite_merge_config_cpv(pconf, cpv);
        cpv = cpv.offset(1);
        if !((*cpv).k_id != -(1 as libc::c_int)) {
            break;
        }
    };
}
unsafe extern "C" fn mod_rewrite_patch_config(r: *mut request_st, p: *mut plugin_data) {
    (*p).conf = (*p).defaults;
    let mut i: libc::c_int = 1 as libc::c_int;
    let mut used: libc::c_int = (*p).nconfig;
    while i < used {
        if config_check_cond(
            r,
            (*((*p).cvlist).offset(i as isize)).k_id as uint32_t as libc::c_int,
        ) != 0
        {
            mod_rewrite_merge_config(
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
unsafe extern "C" fn mod_rewrite_parse_list(
    mut srv: *mut server,
    mut a: *const array,
    mut kvb: *mut pcre_keyvalue_buffer,
    condidx: libc::c_int,
) -> *mut pcre_keyvalue_buffer {
    let pcre_jit: libc::c_int = config_feature_bool(
        srv,
        b"server.pcre_jit\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    let mut allocated: libc::c_int = 0 as libc::c_int;
    if kvb.is_null() {
        allocated = 1 as libc::c_int;
        kvb = pcre_keyvalue_buffer_init();
        (*kvb).cfgidx = condidx;
    }
    let tb: *mut buffer = (*srv).tmp_buf;
    let mut percent: libc::c_int = 0 as libc::c_int;
    let mut j: uint32_t = 0 as libc::c_int as uint32_t;
    while j < (*a).used {
        let mut ds: *mut data_string = *((*a).data).offset(j as isize)
            as *mut data_string;
        if (*srv).srvconf.http_url_normalize != 0 {
            pcre_keyvalue_burl_normalize_key(&mut (*ds).key, tb);
            pcre_keyvalue_burl_normalize_value(&mut (*ds).value, tb);
        }
        let mut s: *const libc::c_char = (*ds).value.ptr;
        loop {
            s = strchr(s, '%' as i32);
            if s.is_null() {
                break;
            }
            if *s.offset(1 as libc::c_int as isize) as libc::c_int == '%' as i32 {
                s = s.offset(1);
            } else if light_isdigit(*s.offset(1 as libc::c_int as isize) as libc::c_int)
                    != 0
                    || *s.offset(1 as libc::c_int as isize) as libc::c_int == '{' as i32
                {
                percent |= 1 as libc::c_int;
                break;
            }
            s = s.offset(1);
        }
        if pcre_keyvalue_buffer_append(
            (*srv).errh,
            kvb,
            &mut (*ds).key,
            &mut (*ds).value,
            pcre_jit,
        ) == 0
        {
            log_error(
                (*srv).errh,
                b"src/mod_rewrite.c\0" as *const u8 as *const libc::c_char,
                123 as libc::c_int as libc::c_uint,
                b"pcre-compile failed for %s\0" as *const u8 as *const libc::c_char,
                (*ds).key.ptr,
            );
            if allocated != 0 {
                pcre_keyvalue_buffer_free(kvb);
            }
            return 0 as *mut pcre_keyvalue_buffer;
        }
        j = j.wrapping_add(1);
    }
    if percent != 0 && 0 as libc::c_int == (*kvb).x0 {
        (*kvb).x0 = config_capture(srv, condidx);
    }
    return kvb;
}
static mut cpk: [config_plugin_keys_t; 7] = [config_plugin_keys_t {
    k: 0 as *const libc::c_char,
    klen: 0,
    ktype: 0,
    scope: 0,
}; 7];
#[cold]
unsafe extern "C" fn mod_rewrite_set_defaults(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    if config_plugin_values_init(
        srv,
        p as *mut libc::c_void,
        cpk.as_ptr(),
        b"mod_rewrite\0" as *const u8 as *const libc::c_char,
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
        let mut rewrite_once: *mut config_plugin_value_t = 0
            as *mut config_plugin_value_t;
        let mut rewrite_repeat: *mut config_plugin_value_t = 0
            as *mut config_plugin_value_t;
        let mut rewrite_NF: *mut config_plugin_value_t = 0 as *mut config_plugin_value_t;
        let mut rewrite_repeat_NF: *mut config_plugin_value_t = 0
            as *mut config_plugin_value_t;
        let mut rewrite: *mut config_plugin_value_t = 0 as *mut config_plugin_value_t;
        let mut rewrite_final: *mut config_plugin_value_t = 0
            as *mut config_plugin_value_t;
        while -(1 as libc::c_int) != (*cpv).k_id {
            match (*cpv).k_id {
                0 => {
                    rewrite_once = cpv;
                }
                1 => {
                    rewrite_final = cpv;
                }
                2 => {
                    rewrite = cpv;
                }
                3 => {
                    rewrite_repeat = cpv;
                }
                4 => {
                    rewrite_NF = cpv;
                }
                5 => {
                    rewrite_repeat_NF = cpv;
                }
                _ => {}
            }
            cpv = cpv.offset(1);
        }
        let condidx: libc::c_int = (*((*p).cvlist).offset(i as isize)).k_id;
        let mut kvb: *mut pcre_keyvalue_buffer = 0 as *mut pcre_keyvalue_buffer;
        let mut kvb_NF: *mut pcre_keyvalue_buffer = 0 as *mut pcre_keyvalue_buffer;
        cpv = rewrite_once;
        if !cpv.is_null() {
            (*cpv)
                .v
                .v = mod_rewrite_parse_list(srv, (*cpv).v.a, kvb, condidx)
                as *mut libc::c_void;
            if ((*cpv).v.v).is_null() {
                return HANDLER_ERROR;
            }
            (*cpv).vtype = T_CONFIG_LOCAL;
            kvb = (*cpv).v.v as *mut pcre_keyvalue_buffer;
        }
        cpv = rewrite_final;
        if !cpv.is_null() {
            (*cpv)
                .v
                .v = mod_rewrite_parse_list(srv, (*cpv).v.a, kvb, condidx)
                as *mut libc::c_void;
            if ((*cpv).v.v).is_null() {
                return HANDLER_ERROR;
            }
            (*cpv).vtype = T_CONFIG_LOCAL;
            kvb = (*cpv).v.v as *mut pcre_keyvalue_buffer;
        }
        cpv = rewrite;
        if !cpv.is_null() {
            (*cpv)
                .v
                .v = mod_rewrite_parse_list(srv, (*cpv).v.a, kvb, condidx)
                as *mut libc::c_void;
            if ((*cpv).v.v).is_null() {
                return HANDLER_ERROR;
            }
            (*cpv).vtype = T_CONFIG_LOCAL;
            kvb = (*cpv).v.v as *mut pcre_keyvalue_buffer;
        }
        if !kvb.is_null() {
            (*kvb).x1 = (*kvb).used as libc::c_int;
        }
        cpv = rewrite_repeat;
        if !cpv.is_null() {
            (*cpv)
                .v
                .v = mod_rewrite_parse_list(srv, (*cpv).v.a, kvb, condidx)
                as *mut libc::c_void;
            if ((*cpv).v.v).is_null() {
                return HANDLER_ERROR;
            }
            (*cpv).vtype = T_CONFIG_LOCAL;
        }
        cpv = rewrite_NF;
        if !cpv.is_null() {
            (*cpv)
                .v
                .v = mod_rewrite_parse_list(srv, (*cpv).v.a, kvb_NF, condidx)
                as *mut libc::c_void;
            if ((*cpv).v.v).is_null() {
                return HANDLER_ERROR;
            }
            (*cpv).vtype = T_CONFIG_LOCAL;
            kvb_NF = (*cpv).v.v as *mut pcre_keyvalue_buffer;
        }
        if !kvb_NF.is_null() {
            (*kvb_NF).x1 = (*kvb_NF).used as libc::c_int;
        }
        cpv = rewrite_repeat_NF;
        if !cpv.is_null() {
            (*cpv)
                .v
                .v = mod_rewrite_parse_list(srv, (*cpv).v.a, kvb_NF, condidx)
                as *mut libc::c_void;
            if ((*cpv).v.v).is_null() {
                return HANDLER_ERROR;
            }
            (*cpv).vtype = T_CONFIG_LOCAL;
        }
        i += 1;
    }
    if (*p).nconfig > 0 as libc::c_int
        && (*(*p).cvlist).v.u2[1 as libc::c_int as usize] != 0
    {
        let mut cpv_0: *const config_plugin_value_t = ((*p).cvlist)
            .offset((*(*p).cvlist).v.u2[0 as libc::c_int as usize] as isize);
        if -(1 as libc::c_int) != (*cpv_0).k_id {
            mod_rewrite_merge_config(&mut (*p).defaults, cpv_0);
        }
    }
    return HANDLER_GO_ON;
}
#[cold]
unsafe extern "C" fn process_rewrite_rules_loop_error(
    r: *mut request_st,
    cfgidx: libc::c_int,
) -> handler_t {
    if 0 as libc::c_int != cfgidx {
        let mut cfginfo: config_cond_info = config_cond_info {
            comp: COMP_UNSET,
            cond: CONFIG_COND_UNSET,
            string: 0 as *const buffer,
            comp_key: 0 as *const libc::c_char,
        };
        config_get_config_cond_info(&mut cfginfo, cfgidx as uint32_t);
        log_error(
            (*r).conf.errh,
            b"src/mod_rewrite.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int as libc::c_uint,
            b"ENDLESS LOOP IN rewrite-rule DETECTED ... aborting request, perhaps you want to use url.rewrite-once instead of url.rewrite-repeat (%s)\0"
                as *const u8 as *const libc::c_char,
            cfginfo.comp_key,
        );
    } else {
        log_error(
            (*r).conf.errh,
            b"src/mod_rewrite.c\0" as *const u8 as *const libc::c_char,
            268 as libc::c_int as libc::c_uint,
            b"ENDLESS LOOP IN rewrite-rule DETECTED ... aborting request\0" as *const u8
                as *const libc::c_char,
        );
    }
    return HANDLER_ERROR;
}
unsafe extern "C" fn mod_rewrite_con_reset(
    mut r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let ref mut fresh0 = *((*r).plugin_ctx)
        .offset((*(p_d as *mut plugin_data)).id as isize);
    *fresh0 = 0 as *mut libc::c_void;
    return HANDLER_GO_ON;
}
unsafe extern "C" fn process_rewrite_rules(
    r: *mut request_st,
    mut p: *mut plugin_data,
    mut kvb: *const pcre_keyvalue_buffer,
) -> handler_t {
    let mut burl: burl_parts_t = burl_parts_t {
        scheme: 0 as *const buffer,
        authority: 0 as *const buffer,
        port: 0,
        path: 0 as *const buffer,
        query: 0 as *const buffer,
    };
    let mut ctx: pcre_keyvalue_ctx = pcre_keyvalue_ctx {
        cache: 0 as *mut cond_match_t,
        burl: 0 as *mut burl_parts_t,
        m: 0,
        n: 0,
        ovec: 0 as *mut libc::c_void,
        subject: 0 as *const libc::c_char,
    };
    let mut rc: handler_t = HANDLER_GO_ON;
    if !(*((*r).plugin_ctx).offset((*p).id as isize)).is_null() {
        let hctx: *mut uintptr_t = ((*r).plugin_ctx).offset((*p).id as isize)
            as *mut uintptr_t;
        *hctx = (*hctx).wrapping_add(1);
        if *hctx & 0x1ff as libc::c_int as libc::c_ulong
            > 100 as libc::c_int as libc::c_ulong
        {
            return process_rewrite_rules_loop_error(r, (*kvb).cfgidx);
        }
        if *hctx & REWRITE_STATE_FINISHED as libc::c_int as libc::c_ulong != 0 {
            return HANDLER_GO_ON;
        }
    }
    ctx.cache = 0 as *mut cond_match_t;
    if (*kvb).x0 != 0 {
        ctx.cache = *((*r).cond_match).offset(((*kvb).x0 - 1 as libc::c_int) as isize);
    }
    ctx.burl = &mut burl;
    burl.scheme = &mut (*r).uri.scheme;
    burl.authority = &mut (*r).uri.authority;
    burl.port = sock_addr_get_port(&(*(*(*r).con).srv_socket).addr);
    burl.path = &mut (*r).target;
    burl.query = &mut (*r).uri.query;
    if buffer_is_blank(burl.authority) != 0 {
        burl.authority = (*r).server_name;
    }
    let tb: *mut buffer = (*r).tmp_buf;
    rc = pcre_keyvalue_buffer_process(kvb, &mut ctx, &mut (*r).target, tb);
    if HANDLER_FINISHED as libc::c_int as libc::c_uint == rc as libc::c_uint
        && buffer_is_blank(tb) == 0
        && *((*tb).ptr).offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        buffer_copy_buffer(&mut (*r).target, tb);
        let hctx_0: *mut uintptr_t = ((*r).plugin_ctx).offset((*p).id as isize)
            as *mut uintptr_t;
        *hctx_0 |= REWRITE_STATE_REWRITTEN as libc::c_int as libc::c_ulong;
        if ctx.m < (*kvb).x1 {
            *hctx_0 |= REWRITE_STATE_FINISHED as libc::c_int as libc::c_ulong;
        }
        buffer_reset(&mut (*r).physical.path);
        rc = HANDLER_COMEBACK;
    } else if HANDLER_FINISHED as libc::c_int as libc::c_uint == rc as libc::c_uint {
        rc = HANDLER_ERROR;
        log_error(
            (*r).conf.errh,
            b"src/mod_rewrite.c\0" as *const u8 as *const libc::c_char,
            318 as libc::c_int as libc::c_uint,
            b"mod_rewrite invalid result (not beginning with '/') while processing uri: %s\0"
                as *const u8 as *const libc::c_char,
            (*r).target.ptr,
        );
    } else if HANDLER_ERROR as libc::c_int as libc::c_uint == rc as libc::c_uint {
        log_error(
            (*r).conf.errh,
            b"src/mod_rewrite.c\0" as *const u8 as *const libc::c_char,
            323 as libc::c_int as libc::c_uint,
            b"pcre_exec() error while processing uri: %s\0" as *const u8
                as *const libc::c_char,
            (*r).target.ptr,
        );
    }
    return rc;
}
unsafe extern "C" fn mod_rewrite_physical(
    mut r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    if !((*r).handler_module).is_null() {
        return HANDLER_GO_ON;
    }
    mod_rewrite_patch_config(r, p);
    if ((*p).conf.rewrite_NF).is_null() || (*(*p).conf.rewrite_NF).used == 0 {
        return HANDLER_GO_ON;
    }
    let st: *const stat_cache_st = stat_cache_path_stat(&mut (*r).physical.path);
    if !st.is_null()
        && (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint
    {
        return HANDLER_GO_ON;
    }
    return process_rewrite_rules(r, p, (*p).conf.rewrite_NF);
}
unsafe extern "C" fn mod_rewrite_uri_handler(
    mut r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    mod_rewrite_patch_config(r, p);
    if ((*p).conf.rewrite).is_null() || (*(*p).conf.rewrite).used == 0 {
        return HANDLER_GO_ON;
    }
    return process_rewrite_rules(r, p, (*p).conf.rewrite);
}
#[no_mangle]
pub unsafe extern "C" fn mod_rewrite_plugin_init(mut p: *mut plugin) -> libc::c_int {
    (*p).version = 0x10440 as libc::c_int as size_t;
    (*p).name = b"rewrite\0" as *const u8 as *const libc::c_char;
    (*p)
        .init = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
    >(Some(mod_rewrite_init as unsafe extern "C" fn() -> *mut libc::c_void));
    (*p)
        .handle_uri_raw = Some(
        mod_rewrite_uri_handler
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_physical = Some(
        mod_rewrite_physical
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .cleanup = Some(
        mod_rewrite_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*p)
        .handle_request_reset = Some(
        mod_rewrite_con_reset
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .set_defaults = Some(
        mod_rewrite_set_defaults
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    return 0 as libc::c_int;
}
pub unsafe fn run_static_initializers() {
    cpk = [
        {
            let mut init = config_plugin_keys_t {
                k: b"url.rewrite-once\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVSTRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"url.rewrite-final\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVSTRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"url.rewrite\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVSTRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"url.rewrite-repeat\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVSTRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"url.rewrite-if-not-file\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVSTRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"url.rewrite-repeat-if-not-file\0" as *const u8
                    as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVSTRING as libc::c_int as uint8_t,
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
