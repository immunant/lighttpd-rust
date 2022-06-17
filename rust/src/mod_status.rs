use ::libc;
extern "C" {
    pub type stat_cache_entry;
    pub type pcre2_real_match_data_8;
    pub type lshpack_double_enc_head;
    pub type lshpack_enc_table_entry;
    pub type fdevents;
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
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn buffer_string_prepare_append(b: *mut buffer, size: size_t) -> *mut libc::c_char;
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_str2(
        b: *mut buffer,
        s1: *const libc::c_char,
        len1: size_t,
        s2: *const libc::c_char,
        len2: size_t,
    );
    fn buffer_append_str3(
        b: *mut buffer,
        s1: *const libc::c_char,
        len1: size_t,
        s2: *const libc::c_char,
        len2: size_t,
        s3: *const libc::c_char,
        len3: size_t,
    );
    fn buffer_append_iovec(b: *mut buffer, iov: *const const_iovec, n: size_t);
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
    fn buffer_is_equal(a: *const buffer, b: *const buffer) -> libc::c_int;
    fn buffer_append_string_encoded(
        b: *mut buffer,
        s: *const libc::c_char,
        s_len: size_t,
        encoding: buffer_encoding_t,
    );
    fn chunkqueue_append_buffer_open(cq: *mut chunkqueue) -> *mut buffer;
    fn chunkqueue_append_buffer_commit(cq: *mut chunkqueue);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn http_chunk_append_mem(
        r: *mut request_st,
        mem: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
    fn http_header_response_set(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
        v: *const libc::c_char,
        vlen: uint32_t,
    );
    static mut log_epoch_secs: unix_time64_t;
    fn config_plugin_values_init(
        srv: *mut server,
        p_d: *mut libc::c_void,
        cpk: *const config_plugin_keys_t,
        mname: *const libc::c_char,
    ) -> libc::c_int;
    fn config_check_cond(r: *mut request_st, context_ndx: libc::c_int) -> libc::c_int;
    static mut plugin_stats: array;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct h2con {
    pub r: [*mut request_st; 8],
    pub rused: uint32_t,
    pub h2_cid: uint32_t,
    pub h2_sid: uint32_t,
    pub sent_goaway: int32_t,
    pub sent_settings: unix_time64_t,
    pub s_header_table_size: uint32_t,
    pub s_enable_push: uint32_t,
    pub s_max_concurrent_streams: uint32_t,
    pub s_initial_window_size: int32_t,
    pub s_max_frame_size: uint32_t,
    pub s_max_header_list_size: uint32_t,
    pub decoder: lshpack_dec,
    pub encoder: lshpack_enc,
    pub half_closed_ts: unix_time64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lshpack_enc {
    pub hpe_cur_capacity: libc::c_uint,
    pub hpe_max_capacity: libc::c_uint,
    pub hpe_next_id: libc::c_uint,
    pub hpe_nelem: libc::c_uint,
    pub hpe_nbits: libc::c_uint,
    pub hpe_all_entries: lshpack_enc_head,
    pub hpe_buckets: *mut lshpack_double_enc_head,
    pub hpe_hist_buf: *mut uint32_t,
    pub hpe_hist_size: libc::c_uint,
    pub hpe_hist_idx: libc::c_uint,
    pub hpe_hist_wrapped: libc::c_int,
    pub hpe_flags: C2RustUnnamed_5,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const LSHPACK_ENC_USE_HIST: C2RustUnnamed_5 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lshpack_enc_head {
    pub stqh_first: *mut lshpack_enc_table_entry,
    pub stqh_last: *mut *mut lshpack_enc_table_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lshpack_dec {
    pub hpd_dyn_table: lshpack_arr,
    pub hpd_max_capacity: libc::c_uint,
    pub hpd_cur_max_capacity: libc::c_uint,
    pub hpd_cur_capacity: libc::c_uint,
    pub hpd_state: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lshpack_arr {
    pub nalloc: libc::c_uint,
    pub nelem: libc::c_uint,
    pub off: libc::c_uint,
    pub els: *mut uintptr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct const_iovec {
    pub iov_base: *const libc::c_void,
    pub iov_len: size_t,
}
pub type buffer_encoding_t = libc::c_uint;
pub const ENCODING_MINIMAL_XML: buffer_encoding_t = 3;
pub const ENCODING_HTML: buffer_encoding_t = 2;
pub const ENCODING_REL_URI_PART: buffer_encoding_t = 1;
pub const ENCODING_REL_URI: buffer_encoding_t = 0;
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
pub struct plugin_config {
    pub config_url: *const buffer,
    pub status_url: *const buffer,
    pub statistics_url: *const buffer,
    pub sort: libc::c_int,
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
    pub traffic_out: libc::c_double,
    pub requests: libc::c_double,
    pub mod_5s_traffic_out: [libc::c_double; 5],
    pub mod_5s_requests: [libc::c_double; 5],
    pub mod_5s_ndx: size_t,
    pub rel_traffic_out: libc::c_double,
    pub rel_requests: libc::c_double,
    pub abs_traffic_out: libc::c_double,
    pub abs_requests: libc::c_double,
    pub bytes_written: libc::c_double,
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
unsafe extern "C" fn light_isalnum(mut c: libc::c_int) -> libc::c_int {
    return (light_isdigit(c) != 0 || light_isalpha(c) != 0) as libc::c_int;
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
unsafe extern "C" fn chunkqueue_length(mut cq: *const chunkqueue) -> off_t {
    return (*cq).bytes_in - (*cq).bytes_out;
}
#[cold]
unsafe extern "C" fn mod_status_init() -> *mut libc::c_void {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<plugin_data>() as libc::c_ulong,
    );
}
unsafe extern "C" fn mod_status_merge_config_cpv(
    pconf: *mut plugin_config,
    cpv: *const config_plugin_value_t,
) {
    match (*cpv).k_id {
        0 => {
            (*pconf).status_url = (*cpv).v.b;
        }
        1 => {
            (*pconf).config_url = (*cpv).v.b;
        }
        2 => {
            (*pconf).statistics_url = (*cpv).v.b;
        }
        3 => {
            (*pconf).sort = (*cpv).v.u as libc::c_int;
        }
        _ => return,
    };
}
unsafe extern "C" fn mod_status_merge_config(
    pconf: *mut plugin_config,
    mut cpv: *const config_plugin_value_t,
) {
    loop {
        mod_status_merge_config_cpv(pconf, cpv);
        cpv = cpv.offset(1);
        if !((*cpv).k_id != -(1 as libc::c_int)) {
            break;
        }
    };
}
unsafe extern "C" fn mod_status_patch_config(r: *mut request_st, p: *mut plugin_data) {
    (*p).conf = (*p).defaults;
    let mut i: libc::c_int = 1 as libc::c_int;
    let mut used: libc::c_int = (*p).nconfig;
    while i < used {
        if config_check_cond(
            r,
            (*((*p).cvlist).offset(i as isize)).k_id as uint32_t as libc::c_int,
        ) != 0
        {
            mod_status_merge_config(
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
unsafe extern "C" fn mod_status_set_defaults(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    if config_plugin_values_init(
        srv,
        p as *mut libc::c_void,
        cpk.as_ptr(),
        b"mod_status\0" as *const u8 as *const libc::c_char,
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
                1 => {
                    current_block_3 = 4807769481574701640;
                }
                0 | 2 => {
                    current_block_3 = 4807769481574701640;
                }
                3 => {
                    current_block_3 = 5399440093318478209;
                }
                _ => {
                    current_block_3 = 5399440093318478209;
                }
            }
            match current_block_3 {
                4807769481574701640 => {
                    if buffer_is_blank((*cpv).v.b) != 0 {
                        (*cpv).v.b = 0 as *const buffer;
                    }
                }
                _ => {}
            }
            cpv = cpv.offset(1);
        }
        i += 1;
    }
    (*p).defaults.sort = 1 as libc::c_int;
    if (*p).nconfig > 0 as libc::c_int
        && (*(*p).cvlist).v.u2[1 as libc::c_int as usize] != 0
    {
        let mut cpv_0: *const config_plugin_value_t = ((*p).cvlist)
            .offset((*(*p).cvlist).v.u2[0 as libc::c_int as usize] as isize);
        if -(1 as libc::c_int) != (*cpv_0).k_id {
            mod_status_merge_config(&mut (*p).defaults, cpv_0);
        }
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_status_append_state(
    b: *mut buffer,
    mut state: request_state_t,
) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: size_t = 0;
    match state as libc::c_uint {
        0 => {
            s = b"connect\0" as *const u8 as *const libc::c_char;
            n = (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
        2 => {
            s = b"read\0" as *const u8 as *const libc::c_char;
            n = (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
        4 => {
            s = b"readpost\0" as *const u8 as *const libc::c_char;
            n = (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
        7 => {
            s = b"write\0" as *const u8 as *const libc::c_char;
            n = (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
        10 => {
            s = b"close\0" as *const u8 as *const libc::c_char;
            n = (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
        9 => {
            s = b"error\0" as *const u8 as *const libc::c_char;
            n = (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
        5 => {
            s = b"handle-req\0" as *const u8 as *const libc::c_char;
            n = (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
        1 => {
            s = b"req-start\0" as *const u8 as *const libc::c_char;
            n = (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
        3 => {
            s = b"req-end\0" as *const u8 as *const libc::c_char;
            n = (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
        6 => {
            s = b"resp-start\0" as *const u8 as *const libc::c_char;
            n = (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
        8 => {
            s = b"resp-end\0" as *const u8 as *const libc::c_char;
            n = (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
        _ => {
            s = b"(unknown)\0" as *const u8 as *const libc::c_char;
            n = (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
    }
    buffer_append_string_len(b, s, n);
}
unsafe extern "C" fn mod_status_get_short_state(
    mut state: request_state_t,
) -> *const libc::c_char {
    match state as libc::c_uint {
        0 => return b".\0" as *const u8 as *const libc::c_char,
        2 => return b"r\0" as *const u8 as *const libc::c_char,
        4 => return b"R\0" as *const u8 as *const libc::c_char,
        7 => return b"W\0" as *const u8 as *const libc::c_char,
        10 => return b"C\0" as *const u8 as *const libc::c_char,
        9 => return b"E\0" as *const u8 as *const libc::c_char,
        5 => return b"h\0" as *const u8 as *const libc::c_char,
        1 => return b"q\0" as *const u8 as *const libc::c_char,
        3 => return b"Q\0" as *const u8 as *const libc::c_char,
        6 => return b"s\0" as *const u8 as *const libc::c_char,
        8 => return b"S\0" as *const u8 as *const libc::c_char,
        _ => return b"x\0" as *const u8 as *const libc::c_char,
    };
}
unsafe extern "C" fn mod_status_header_append_sort(
    mut b: *mut buffer,
    mut p: *mut plugin_data,
    mut k: *const libc::c_char,
    mut klen: size_t,
) {
    if (*p).conf.sort != 0 {
        buffer_append_str3(
            b,
            b"<th class=\"status\"><a href=\"#\" class=\"sortheader\" onclick=\"resort(this);return false;\">\0"
                as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 88]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            k,
            klen,
            b"<span class=\"sortarrow\">:</span></a></th>\n\0" as *const u8
                as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    } else {
        buffer_append_str3(
            b,
            b"<th class=\"status\">\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            k,
            klen,
            b"</th>\n\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    };
}
unsafe extern "C" fn mod_status_get_multiplier(
    mut b: *mut buffer,
    mut avg: libc::c_double,
    mut size: libc::c_int,
) {
    let mut unit: [libc::c_char; 3] = *::std::mem::transmute::<
        &[u8; 3],
        &mut [libc::c_char; 3],
    >(b"  \0");
    if avg > size as libc::c_double {
        avg /= size as libc::c_double;
        unit[1 as libc::c_int as usize] = 'k' as i32 as libc::c_char;
    }
    if avg > size as libc::c_double {
        avg /= size as libc::c_double;
        unit[1 as libc::c_int as usize] = 'M' as i32 as libc::c_char;
    }
    if avg > size as libc::c_double {
        avg /= size as libc::c_double;
        unit[1 as libc::c_int as usize] = 'G' as i32 as libc::c_char;
    }
    if avg > size as libc::c_double {
        avg /= size as libc::c_double;
        unit[1 as libc::c_int as usize] = 'T' as i32 as libc::c_char;
    }
    if avg > size as libc::c_double {
        avg /= size as libc::c_double;
        unit[1 as libc::c_int as usize] = 'P' as i32 as libc::c_char;
    }
    if avg > size as libc::c_double {
        avg /= size as libc::c_double;
        unit[1 as libc::c_int as usize] = 'E' as i32 as libc::c_char;
    }
    if avg > size as libc::c_double {
        avg /= size as libc::c_double;
        unit[1 as libc::c_int as usize] = 'Z' as i32 as libc::c_char;
    }
    if avg > size as libc::c_double {
        avg /= size as libc::c_double;
        unit[1 as libc::c_int as usize] = 'Y' as i32 as libc::c_char;
    }
    if size == 1000 as libc::c_int {
        buffer_append_int(b, avg as intmax_t);
    } else {
        let mut buf: [libc::c_char; 33] = [0; 33];
        buffer_append_string_len(
            b,
            buf.as_mut_ptr(),
            snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong,
                b"%.2f\0" as *const u8 as *const libc::c_char,
                avg,
            ) as size_t,
        );
    }
    buffer_append_string_len(b, unit.as_mut_ptr(), 2 as libc::c_int as size_t);
}
unsafe extern "C" fn mod_status_html_rtable_r(
    b: *mut buffer,
    r: *const request_st,
    con: *const connection,
    cur_ts: unix_time64_t,
) {
    buffer_append_str3(
        b,
        b"<tr><td class=\"string\">\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        (*con).dst_addr_buf.ptr,
        buffer_clen(&(*con).dst_addr_buf) as size_t,
        b"</td><td class=\"int\">\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    if (*r).reqbody_length != 0 {
        buffer_append_int(b, (*r).reqbody_queue.bytes_in);
        buffer_append_string_len(
            b,
            b"/\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        buffer_append_int(b, (*r).reqbody_length);
    } else {
        buffer_append_string_len(
            b,
            b"0/0\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    buffer_append_string_len(
        b,
        b"</td><td class=\"int\">\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(b, (*r).write_queue.bytes_out);
    buffer_append_string_len(
        b,
        b"/\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(
        b,
        (*r).write_queue.bytes_out + chunkqueue_length(&(*r).write_queue),
    );
    buffer_append_string_len(
        b,
        b"</td><td class=\"string\">\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    if CON_STATE_READ as libc::c_int as libc::c_uint == (*r).state as libc::c_uint
        && buffer_is_blank(&(*r).target_orig) == 0
    {
        buffer_append_string_len(
            b,
            b"keep-alive\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    } else {
        mod_status_append_state(b, (*r).state);
    }
    buffer_append_string_len(
        b,
        b"</td><td class=\"int\">\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(b, cur_ts - (*r).start_hp.tv_sec);
    buffer_append_string_len(
        b,
        b"</td><td class=\"string\">\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    if buffer_is_blank((*r).server_name) != 0 {
        buffer_append_string_encoded(
            b,
            (*r).uri.authority.ptr,
            buffer_clen(&(*r).uri.authority) as size_t,
            ENCODING_HTML,
        );
    } else {
        buffer_append_string_encoded(
            b,
            (*(*r).server_name).ptr,
            buffer_clen((*r).server_name) as size_t,
            ENCODING_HTML,
        );
    }
    buffer_append_string_len(
        b,
        b"</td><td class=\"string\">\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    if buffer_is_blank(&(*r).uri.path) == 0 {
        buffer_append_string_encoded(
            b,
            (*r).uri.path.ptr,
            buffer_clen(&(*r).uri.path) as size_t,
            ENCODING_HTML,
        );
    }
    if buffer_is_blank(&(*r).uri.query) == 0 {
        buffer_append_string_len(
            b,
            b"?\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        buffer_append_string_encoded(
            b,
            (*r).uri.query.ptr,
            buffer_clen(&(*r).uri.query) as size_t,
            ENCODING_HTML,
        );
    }
    if buffer_is_blank(&(*r).target_orig) == 0 {
        buffer_append_string_len(
            b,
            b" (\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        buffer_append_string_encoded(
            b,
            (*r).target_orig.ptr,
            buffer_clen(&(*r).target_orig) as size_t,
            ENCODING_HTML,
        );
        buffer_append_string_len(
            b,
            b")\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    buffer_append_string_len(
        b,
        b"</td><td class=\"string\">\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_string_encoded(
        b,
        (*r).physical.path.ptr,
        buffer_clen(&(*r).physical.path) as size_t,
        ENCODING_HTML,
    );
    buffer_append_string_len(
        b,
        b"</td></tr>\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
}
unsafe extern "C" fn mod_status_html_rtable(
    rq: *mut request_st,
    srv: *const server,
    cur_ts: unix_time64_t,
) {
    let b: *mut buffer = (*rq).tmp_buf;
    buffer_clear(b);
    let mut con: *const connection = (*srv).conns;
    while !con.is_null() {
        let r: *const request_st = &(*con).request;
        if (*r).http_status <= HTTP_VERSION_1_1 as libc::c_int {
            if buffer_string_space(b) < 4096 as libc::c_int as libc::c_uint {
                http_chunk_append_mem(rq, (*b).ptr, buffer_clen(b) as size_t);
                buffer_clear(b);
            }
            mod_status_html_rtable_r(b, r, con, cur_ts);
        } else {
            let h2c: *mut h2con = (*con).h2;
            let mut j: uint32_t = 0 as libc::c_int as uint32_t;
            let mut rused: uint32_t = (*h2c).rused;
            while j < rused {
                if buffer_string_space(b) < 4096 as libc::c_int as libc::c_uint {
                    http_chunk_append_mem(rq, (*b).ptr, buffer_clen(b) as size_t);
                    buffer_clear(b);
                }
                mod_status_html_rtable_r(b, (*h2c).r[j as usize], con, cur_ts);
                j = j.wrapping_add(1);
            }
        }
        con = (*con).next;
    }
    http_chunk_append_mem(rq, (*b).ptr, buffer_clen(b) as size_t);
}
unsafe extern "C" fn mod_status_handle_server_status_html(
    mut srv: *mut server,
    r: *mut request_st,
    mut p: *mut plugin_data,
) -> handler_t {
    let b: *mut buffer = chunkqueue_append_buffer_open(&mut (*r).write_queue);
    buffer_string_prepare_append(b, (8192 as libc::c_int - 1 as libc::c_int) as size_t);
    let mut avg: libc::c_double = 0.;
    let mut j: uint32_t = 0;
    let mut ts: unix_time64_t = 0;
    let cur_ts: unix_time64_t = log_epoch_secs;
    let mut days: libc::c_int = 0;
    let mut hours: libc::c_int = 0;
    let mut mins: libc::c_int = 0;
    let mut seconds: libc::c_int = 0;
    let mut cstates: [libc::c_int; 13] = [0; 13];
    memset(
        cstates.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_int; 13]>() as libc::c_ulong,
    );
    buffer_copy_string_len(
        b,
        b"<?xml version=\"1.0\" encoding=\"iso-8859-1\"?>\n<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\"\n         \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">\n<html xmlns=\"http://www.w3.org/1999/xhtml\" xml:lang=\"en\" lang=\"en\">\n <head>\n  <title>Status</title>\n  <style type=\"text/css\">\n    table.status { border: black solid thin; }\n    td { white-space: nowrap; }\n    td.int { background-color: #f0f0f0; text-align: right }\n    td.string { background-color: #f0f0f0; text-align: left }\n    th.status { background-color: black; color: white; font-weight: bold; }\n    a.sortheader { background-color: black; color: white; font-weight: bold; text-decoration: none; display: block; }\n    span.sortarrow { color: white; text-decoration: none; }\n  </style>\n\0"
            as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 768]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    if buffer_is_blank(&mut (*r).uri.query) == 0
        && 0 as libc::c_int
            == memcmp(
                (*r).uri.query.ptr as *const libc::c_void,
                b"refresh=\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
            )
    {
        let refresh: libc::c_long = strtol(
            ((*r).uri.query.ptr)
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as isize,
                )
                .offset(-(1 as libc::c_int as isize)),
            0 as *mut *mut libc::c_char,
            10 as libc::c_int,
        );
        if refresh > 0 as libc::c_int as libc::c_long {
            buffer_append_string_len(
                b,
                b"<meta http-equiv=\"refresh\" content=\"\0" as *const u8
                    as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            buffer_append_int(
                b,
                if refresh < 604800 as libc::c_int as libc::c_long {
                    refresh
                } else {
                    604800 as libc::c_int as libc::c_long
                },
            );
            buffer_append_string_len(
                b,
                b"\">\n\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
    }
    if (*p).conf.sort != 0 {
        buffer_append_string_len(
            b,
            b"<script type=\"text/javascript\">\n// <!--\nvar sort_column;\nvar prev_span = null;\nfunction get_inner_text(el) {\n if((typeof el == 'string')||(typeof el == 'undefined'))\n  return el;\n if(el.innerText)\n  return el.innerText;\n else {\n  var str = \"\";\n  var cs = el.childNodes;\n  var l = cs.length;\n  for (i=0;i<l;i++) {\n   if (cs[i].nodeType==1) str += get_inner_text(cs[i]);\n   else if (cs[i].nodeType==3) str += cs[i].nodeValue;\n  }\n }\n return str;\n}\nfunction sortfn(a,b) {\n var at = get_inner_text(a.cells[sort_column]);\n var bt = get_inner_text(b.cells[sort_column]);\n if (a.cells[sort_column].className == 'int') {\n  return parseInt(at)-parseInt(bt);\n } else {\n  aa = at.toLowerCase();\n  bb = bt.toLowerCase();\n  if (aa==bb) return 0;\n  else if (aa<bb) return -1;\n  else return 1;\n }\n}\nfunction resort(lnk) {\n var span = lnk.childNodes[1];\n var table = lnk.parentNode.parentNode.parentNode.parentNode;\n var rows = new Array();\n for (j=1;j<table.rows.length;j++)\n  rows[j-1] = table.rows[j];\n sort_column = lnk.parentNode.cellIndex;\n rows.sort(sortfn);\n if (prev_span != null) prev_span.innerHTML = '';\n if (span.getAttribute('sortdir')=='down') {\n  span.innerHTML = '&uarr;';\n  span.setAttribute('sortdir','up');\n  rows.reverse();\n } else {\n  span.innerHTML = '&darr;';\n  span.setAttribute('sortdir','down');\n }\n for (i=0;i<rows.length;i++)\n  table.tBodies[0].appendChild(rows[i]);\n prev_span = span;\n}\n// -->\n</script>\n\0"
                as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 1419]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    buffer_append_string_len(
        b,
        b" </head>\n<body>\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_string_len(
        b,
        b"<h1>Server-Status\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    if !((*r).conf.server_tag).is_null() {
        buffer_append_str3(
            b,
            b" (\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            (*(*r).conf.server_tag).ptr,
            buffer_clen((*r).conf.server_tag) as size_t,
            b")\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    buffer_append_string_len(
        b,
        b"</h1><table summary=\"status\" class=\"status\"><tr><td>Hostname</td><td class=\"string\">\0"
            as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 85]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_string_encoded(
        b,
        (*r).uri.authority.ptr,
        buffer_clen(&mut (*r).uri.authority) as size_t,
        ENCODING_HTML,
    );
    if buffer_is_blank((*r).server_name) == 0
        && (*r).server_name != &mut (*r).uri.authority as *mut buffer as *const buffer
    {
        buffer_append_string_len(
            b,
            b" (\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        buffer_append_string_encoded(
            b,
            (*(*r).server_name).ptr,
            buffer_clen((*r).server_name) as size_t,
            ENCODING_HTML,
        );
        buffer_append_string_len(
            b,
            b")\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    buffer_append_string_len(
        b,
        b"</td></tr>\n<tr><td>Uptime</td><td class=\"string\">\0" as *const u8
            as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    ts = cur_ts - (*srv).startup_ts;
    days = (ts
        / (60 as libc::c_int * 60 as libc::c_int * 24 as libc::c_int) as libc::c_long)
        as libc::c_int;
    ts %= (60 as libc::c_int * 60 as libc::c_int * 24 as libc::c_int) as libc::c_long;
    hours = (ts / (60 as libc::c_int * 60 as libc::c_int) as libc::c_long)
        as libc::c_int;
    ts %= (60 as libc::c_int * 60 as libc::c_int) as libc::c_long;
    mins = (ts / 60 as libc::c_int as libc::c_long) as libc::c_int;
    ts %= 60 as libc::c_int as libc::c_long;
    seconds = ts as libc::c_int;
    if days != 0 {
        buffer_append_int(b, days as intmax_t);
        buffer_append_string_len(
            b,
            b" days \0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    if hours != 0 {
        buffer_append_int(b, hours as intmax_t);
        buffer_append_string_len(
            b,
            b" hours \0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    if mins != 0 {
        buffer_append_int(b, mins as intmax_t);
        buffer_append_string_len(
            b,
            b" min \0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    buffer_append_int(b, seconds as intmax_t);
    buffer_append_string_len(
        b,
        b" s</td></tr>\n<tr><td>Started at</td><td class=\"string\">\0" as *const u8
            as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 56]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    ts = (*srv).startup_ts;
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
    buffer_append_strftime(
        b,
        b"%F %T\0" as *const u8 as *const libc::c_char,
        localtime_r(&mut ts, &mut tm),
    );
    buffer_append_string_len(
        b,
        b"</td></tr>\n<tr><th colspan=\"2\">absolute (since start)</th></tr>\n<tr><td>Requests</td><td class=\"string\">\0"
            as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 105]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    avg = (*p).abs_requests;
    mod_status_get_multiplier(b, avg, 1000 as libc::c_int);
    buffer_append_string_len(
        b,
        b"req</td></tr>\n<tr><td>Traffic</td><td class=\"string\">\0" as *const u8
            as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 54]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    avg = (*p).abs_traffic_out;
    mod_status_get_multiplier(b, avg, 1024 as libc::c_int);
    buffer_append_string_len(
        b,
        b"byte</td></tr>\n<tr><th colspan=\"2\">average (since start)</th></tr>\n<tr><td>Requests</td><td class=\"string\">\0"
            as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    avg = (*p).abs_requests / (cur_ts - (*srv).startup_ts) as libc::c_double;
    mod_status_get_multiplier(b, avg, 1000 as libc::c_int);
    buffer_append_string_len(
        b,
        b"req/s</td></tr>\n<tr><td>Traffic</td><td class=\"string\">\0" as *const u8
            as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 56]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    avg = (*p).abs_traffic_out / (cur_ts - (*srv).startup_ts) as libc::c_double;
    mod_status_get_multiplier(b, avg, 1024 as libc::c_int);
    buffer_append_string_len(
        b,
        b"byte/s</td></tr>\n<tr><th colspan=\"2\">average (5s sliding average)</th></tr>\n\0"
            as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 77]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    j = 0 as libc::c_int as uint32_t;
    avg = 0 as libc::c_int as libc::c_double;
    while j < 5 as libc::c_int as libc::c_uint {
        avg += (*p).mod_5s_requests[j as usize];
        j = j.wrapping_add(1);
    }
    avg /= 5 as libc::c_int as libc::c_double;
    buffer_append_string_len(
        b,
        b"<tr><td>Requests</td><td class=\"string\">\0" as *const u8
            as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 41]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    mod_status_get_multiplier(b, avg, 1000 as libc::c_int);
    buffer_append_string_len(
        b,
        b"req/s</td></tr>\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    j = 0 as libc::c_int as uint32_t;
    avg = 0 as libc::c_int as libc::c_double;
    while j < 5 as libc::c_int as libc::c_uint {
        avg += (*p).mod_5s_traffic_out[j as usize];
        j = j.wrapping_add(1);
    }
    avg /= 5 as libc::c_int as libc::c_double;
    buffer_append_string_len(
        b,
        b"<tr><td>Traffic</td><td class=\"string\">\0" as *const u8
            as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    mod_status_get_multiplier(b, avg, 1024 as libc::c_int);
    buffer_append_string_len(
        b,
        b"byte/s</td></tr>\n</table>\n<hr />\n<pre>\n<b>\0" as *const u8
            as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(
        b,
        ((*srv).srvconf.max_conns as libc::c_uint).wrapping_sub((*srv).lim_conns)
            as intmax_t,
    );
    buffer_append_string_len(
        b,
        b" connections</b>\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    let mut per_line: libc::c_int = 50 as libc::c_int;
    let mut c: *const connection = (*srv).conns;
    while !c.is_null() {
        let cr: *const request_st = &(*c).request;
        let mut state: *const libc::c_char = 0 as *const libc::c_char;
        if !((*c).h2).is_null() && 0 as libc::c_int as libc::c_uint == (*(*c).h2).rused
            || CON_STATE_READ as libc::c_int as libc::c_uint
                == (*cr).state as libc::c_uint
                && buffer_is_blank(&(*cr).target_orig) == 0
        {
            state = b"k\0" as *const u8 as *const libc::c_char;
            cstates[(CON_STATE_CLOSE as libc::c_int + 2 as libc::c_int) as usize] += 1;
        } else {
            state = mod_status_get_short_state((*cr).state);
            cstates[(if (*cr).state as libc::c_uint
                <= CON_STATE_CLOSE as libc::c_int as libc::c_uint
            {
                (*cr).state as libc::c_uint
            } else {
                (CON_STATE_CLOSE as libc::c_int + 1 as libc::c_int) as libc::c_uint
            }) as usize] += 1;
        }
        buffer_append_string_len(b, state, 1 as libc::c_int as size_t);
        per_line -= 1;
        if 0 as libc::c_int == per_line {
            per_line = 50 as libc::c_int;
            buffer_append_string_len(
                b,
                b"\n\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
        c = (*c).next;
    }
    buffer_append_string_len(
        b,
        b"\n\n<table>\n<tr><td style=\"text-align:right\">\0" as *const u8
            as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 44]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(
        b,
        cstates[(CON_STATE_CLOSE as libc::c_int + 2 as libc::c_int) as usize] as intmax_t,
    );
    buffer_append_string_len(
        b,
        b"<td>&nbsp;&nbsp;k = keep-alive</td></tr>\n\0" as *const u8
            as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 42]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    j = 0 as libc::c_int as uint32_t;
    while j < (CON_STATE_CLOSE as libc::c_int + 2 as libc::c_int) as libc::c_uint {
        if !(0 as libc::c_int == cstates[j as usize]
            && j == (CON_STATE_CLOSE as libc::c_int + 1 as libc::c_int) as libc::c_uint)
        {
            buffer_append_string_len(
                b,
                b"<tr><td style=\"text-align:right\">\0" as *const u8
                    as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 34]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            buffer_append_int(b, cstates[j as usize] as intmax_t);
            buffer_append_str3(
                b,
                b"</td><td>&nbsp;&nbsp;\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                mod_status_get_short_state(j as request_state_t),
                1 as libc::c_int as size_t,
                b" = \0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            mod_status_append_state(b, j as request_state_t);
            buffer_append_string_len(
                b,
                b"</td></tr>\n\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
        j = j.wrapping_add(1);
    }
    buffer_append_string_len(
        b,
        b"</table>\n</pre><hr />\n<h2>Connections</h2>\n<table summary=\"status\" class=\"status\">\n<tr>\0"
            as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 88]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    mod_status_header_append_sort(
        b,
        p,
        b"Client IP\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    mod_status_header_append_sort(
        b,
        p,
        b"Read\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    mod_status_header_append_sort(
        b,
        p,
        b"Written\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    mod_status_header_append_sort(
        b,
        p,
        b"State\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    mod_status_header_append_sort(
        b,
        p,
        b"Time\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    mod_status_header_append_sort(
        b,
        p,
        b"Host\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    mod_status_header_append_sort(
        b,
        p,
        b"URI\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    mod_status_header_append_sort(
        b,
        p,
        b"File\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_string_len(
        b,
        b"</tr>\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    chunkqueue_append_buffer_commit(&mut (*r).write_queue);
    mod_status_html_rtable(r, srv, cur_ts);
    http_chunk_append_mem(
        r,
        b"</table>\n</body>\n</html>\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
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
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_status_handle_server_status_text(
    mut srv: *mut server,
    r: *mut request_st,
    mut p: *mut plugin_data,
) -> handler_t {
    let mut b: *mut buffer = chunkqueue_append_buffer_open(&mut (*r).write_queue);
    buffer_append_string_len(
        b,
        b"Total Accesses: \0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(b, (*p).abs_requests as intmax_t);
    buffer_append_string_len(
        b,
        b"\nTotal kBytes: \0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(
        b,
        ((*p).abs_traffic_out / 1024 as libc::c_int as libc::c_double) as intmax_t,
    );
    buffer_append_string_len(
        b,
        b"\nUptime: \0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(b, log_epoch_secs - (*srv).startup_ts);
    buffer_append_string_len(
        b,
        b"\nBusyServers: \0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(
        b,
        ((*srv).srvconf.max_conns as libc::c_uint).wrapping_sub((*srv).lim_conns)
            as intmax_t,
    );
    buffer_append_string_len(
        b,
        b"\nIdleServers: \0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(b, (*srv).lim_conns as intmax_t);
    buffer_append_string_len(
        b,
        b"\nScoreboard: \0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    let mut c: *const connection = (*srv).conns;
    while !c.is_null() {
        let cr: *const request_st = &(*c).request;
        let mut state: *const libc::c_char = if !((*c).h2).is_null()
            && 0 as libc::c_int as libc::c_uint == (*(*c).h2).rused
            || CON_STATE_READ as libc::c_int as libc::c_uint
                == (*cr).state as libc::c_uint
                && buffer_is_blank(&(*cr).target_orig) == 0
        {
            b"k\0" as *const u8 as *const libc::c_char
        } else {
            mod_status_get_short_state((*cr).state)
        };
        buffer_append_string_len(b, state, 1 as libc::c_int as size_t);
        c = (*c).next;
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*srv).lim_conns {
        buffer_append_string_len(
            b,
            b"_\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        i = i.wrapping_add(1);
    }
    buffer_append_string_len(
        b,
        b"\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    chunkqueue_append_buffer_commit(&mut (*r).write_queue);
    http_header_response_set(
        r,
        HTTP_HEADER_CONTENT_TYPE,
        b"Content-Type\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
        b"text/plain\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_status_handle_server_status_json(
    mut srv: *mut server,
    r: *mut request_st,
    mut p: *mut plugin_data,
) -> handler_t {
    let mut b: *mut buffer = chunkqueue_append_buffer_open(&mut (*r).write_queue);
    let mut avg: libc::c_double = 0.;
    let mut j: uint32_t = 0;
    let mut jsonp: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if buffer_clen(&mut (*r).uri.query) as libc::c_ulong
        >= (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && 0 as libc::c_int
            == memcmp(
                (*r).uri.query.ptr as *const libc::c_void,
                b"jsonp=\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
            )
    {
        let mut f: *const libc::c_char = ((*r).uri.query.ptr)
            .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize)
            .offset(-(1 as libc::c_int as isize));
        let mut len: libc::c_int = 0 as libc::c_int;
        while light_isalnum(*f.offset(len as isize) as libc::c_int) != 0
            || *f.offset(len as isize) as libc::c_int == '_' as i32
        {
            len += 1;
        }
        if 0 as libc::c_int != len
            && light_isalpha(*f.offset(0 as libc::c_int as isize) as libc::c_int) != 0
            && *f.offset(len as isize) as libc::c_int == '\u{0}' as i32
        {
            buffer_append_str2(
                b,
                f,
                len as size_t,
                b"(\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            jsonp = 1 as libc::c_int as libc::c_uint;
        }
    }
    buffer_append_string_len(
        b,
        b"{\n\t\"RequestsTotal\": \0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(b, (*p).abs_requests as intmax_t);
    buffer_append_string_len(
        b,
        b",\n\t\"TrafficTotal\": \0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(
        b,
        ((*p).abs_traffic_out / 1024 as libc::c_int as libc::c_double) as intmax_t,
    );
    buffer_append_string_len(
        b,
        b",\n\t\"Uptime\": \0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(b, log_epoch_secs - (*srv).startup_ts);
    buffer_append_string_len(
        b,
        b",\n\t\"BusyServers\": \0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(
        b,
        ((*srv).srvconf.max_conns as libc::c_uint).wrapping_sub((*srv).lim_conns)
            as intmax_t,
    );
    buffer_append_string_len(
        b,
        b",\n\t\"IdleServers\": \0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(b, (*srv).lim_conns as intmax_t);
    buffer_append_string_len(
        b,
        b",\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    j = 0 as libc::c_int as uint32_t;
    avg = 0 as libc::c_int as libc::c_double;
    while j < 5 as libc::c_int as libc::c_uint {
        avg += (*p).mod_5s_requests[j as usize];
        j = j.wrapping_add(1);
    }
    avg /= 5 as libc::c_int as libc::c_double;
    buffer_append_string_len(
        b,
        b"\t\"RequestAverage5s\":\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(b, avg as intmax_t);
    buffer_append_string_len(
        b,
        b",\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    j = 0 as libc::c_int as uint32_t;
    avg = 0 as libc::c_int as libc::c_double;
    while j < 5 as libc::c_int as libc::c_uint {
        avg += (*p).mod_5s_traffic_out[j as usize];
        j = j.wrapping_add(1);
    }
    avg /= 5 as libc::c_int as libc::c_double;
    buffer_append_string_len(
        b,
        b"\t\"TrafficAverage5s\":\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(b, (avg / 1024 as libc::c_int as libc::c_double) as intmax_t);
    buffer_append_string_len(
        b,
        b"\n}\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    if jsonp != 0 {
        buffer_append_string_len(
            b,
            b");\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    chunkqueue_append_buffer_commit(&mut (*r).write_queue);
    http_header_response_set(
        r,
        HTTP_HEADER_CONTENT_TYPE,
        b"Content-Type\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
        b"application/javascript\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_status_handle_server_statistics(
    r: *mut request_st,
) -> handler_t {
    let mut b: *mut buffer = 0 as *mut buffer;
    let mut i: size_t = 0;
    let mut st: *mut array = &mut plugin_stats;
    if 0 as libc::c_int as libc::c_uint == (*st).used {
        (*r).http_status = 204 as libc::c_int;
        (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
        return HANDLER_FINISHED;
    }
    b = chunkqueue_append_buffer_open(&mut (*r).write_queue);
    i = 0 as libc::c_int as size_t;
    while i < (*st).used as libc::c_ulong {
        buffer_append_str2(
            b,
            (**((*st).sorted).offset(i as isize)).key.ptr,
            buffer_clen(&mut (**((*st).sorted).offset(i as isize)).key) as size_t,
            b": \0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        buffer_append_int(
            b,
            (*(*((*st).sorted).offset(i as isize) as *mut data_integer)).value
                as intmax_t,
        );
        buffer_append_string_len(
            b,
            b"\n\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        i = i.wrapping_add(1);
    }
    chunkqueue_append_buffer_commit(&mut (*r).write_queue);
    http_header_response_set(
        r,
        HTTP_HEADER_CONTENT_TYPE,
        b"Content-Type\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
        b"text/plain\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    (*r).http_status = 200 as libc::c_int;
    (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
    return HANDLER_FINISHED;
}
unsafe extern "C" fn mod_status_handle_server_status(
    r: *mut request_st,
    p: *mut plugin_data,
) -> handler_t {
    let srv: *mut server = (*(*r).con).srv;
    if buffer_eq_slen(
        &mut (*r).uri.query,
        b"auto\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    ) != 0
    {
        mod_status_handle_server_status_text(srv, r, p);
    } else if buffer_clen(&mut (*r).uri.query) as libc::c_ulong
            >= (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && 0 as libc::c_int
                == memcmp(
                    (*r).uri.query.ptr as *const libc::c_void,
                    b"json\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
                )
        {
        mod_status_handle_server_status_json(srv, r, p);
    } else {
        mod_status_handle_server_status_html(srv, r, p);
    }
    (*r).http_status = 200 as libc::c_int;
    (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
    return HANDLER_FINISHED;
}
unsafe extern "C" fn mod_status_row_append(
    mut b: *mut buffer,
    mut k: *const libc::c_char,
    mut klen: size_t,
    mut v: *const libc::c_char,
    mut vlen: size_t,
) {
    let mut iov: [const_iovec; 5] = [
        {
            let mut init = const_iovec {
                iov_base: b"   <tr>\n    <td><b>\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                iov_len: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: k as *const libc::c_void,
                iov_len: klen,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: b"</b></td>\n    <td>\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                iov_len: (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: v as *const libc::c_void,
                iov_len: vlen,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: b"</td>\n   </tr>\n\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                iov_len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            };
            init
        },
    ];
    buffer_append_iovec(
        b,
        iov.as_mut_ptr(),
        (::std::mem::size_of::<[const_iovec; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<const_iovec>() as libc::c_ulong),
    );
}
unsafe extern "C" fn mod_status_header_append(
    mut b: *mut buffer,
    mut k: *const libc::c_char,
    mut klen: size_t,
) {
    buffer_append_str3(
        b,
        b"   <tr>\n    <th colspan=\"2\">\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        k,
        klen,
        b"</th>\n   </tr>\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
}
unsafe extern "C" fn mod_status_handle_server_config(r: *mut request_st) -> handler_t {
    let srv: *mut server = (*(*r).con).srv;
    let tb: *mut buffer = (*r).tmp_buf;
    buffer_clear(tb);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*srv).plugins.used {
        let mut name: *const libc::c_char = (**((*srv).plugins.ptr as *mut *mut plugin)
            .offset(i as isize))
            .name;
        if i != 0 as libc::c_int as libc::c_uint {
            buffer_append_string_len(
                tb,
                b"<br />\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
        buffer_append_string_len(tb, name, strlen(name));
        i = i.wrapping_add(1);
    }
    let mut b: *mut buffer = chunkqueue_append_buffer_open(&mut (*r).write_queue);
    buffer_append_string_len(
        b,
        b"<?xml version=\"1.0\" encoding=\"iso-8859-1\"?>\n<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\"\n         \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">\n<html xmlns=\"http://www.w3.org/1999/xhtml\" xml:lang=\"en\" lang=\"en\">\n <head>\n  <title>Status</title>\n </head>\n <body>\n\0"
            as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 293]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    if !((*r).conf.server_tag).is_null() {
        buffer_append_str3(
            b,
            b"  <h1>\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            (*(*r).conf.server_tag).ptr,
            buffer_clen((*r).conf.server_tag) as size_t,
            b"  </h1>\n\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    buffer_append_string_len(
        b,
        b"  <table summary=\"status\" border=\"1\">\n\0" as *const u8
            as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 39]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    mod_status_header_append(
        b,
        b"Server-Features\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    mod_status_row_append(
        b,
        b"RegEx Conditionals\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        b"enabled\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    mod_status_header_append(
        b,
        b"Network Engine\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    mod_status_row_append(
        b,
        b"fd-Event-Handler\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        (*srv).srvconf.event_handler,
        strlen((*srv).srvconf.event_handler),
    );
    mod_status_header_append(
        b,
        b"Config-File-Settings\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    mod_status_row_append(
        b,
        b"Loaded Modules\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        (*tb).ptr,
        buffer_clen(tb) as size_t,
    );
    buffer_append_string_len(
        b,
        b"  </table>\n </body>\n</html>\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    chunkqueue_append_buffer_commit(&mut (*r).write_queue);
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
    (*r).http_status = 200 as libc::c_int;
    (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
    return HANDLER_FINISHED;
}
unsafe extern "C" fn mod_status_handler(
    r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    if !((*r).handler_module).is_null() {
        return HANDLER_GO_ON;
    }
    mod_status_patch_config(r, p);
    if !((*p).conf.status_url).is_null()
        && buffer_is_equal((*p).conf.status_url, &mut (*r).uri.path) != 0
    {
        return mod_status_handle_server_status(r, p)
    } else {
        if !((*p).conf.config_url).is_null()
            && buffer_is_equal((*p).conf.config_url, &mut (*r).uri.path) != 0
        {
            return mod_status_handle_server_config(r)
        } else {
            if !((*p).conf.statistics_url).is_null()
                && buffer_is_equal((*p).conf.statistics_url, &mut (*r).uri.path) != 0
            {
                return mod_status_handle_server_statistics(r);
            }
        }
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_status_trigger(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    let mut c: *const connection = (*srv).conns;
    while !c.is_null() {
        (*p).bytes_written += (*c).bytes_written_cur_second as libc::c_double;
        c = (*c).next;
    }
    (*p).mod_5s_traffic_out[(*p).mod_5s_ndx as usize] = (*p).bytes_written;
    (*p).mod_5s_requests[(*p).mod_5s_ndx as usize] = (*p).requests;
    (*p)
        .mod_5s_ndx = ((*p).mod_5s_ndx)
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_rem(5 as libc::c_int as libc::c_ulong);
    (*p).abs_traffic_out += (*p).bytes_written;
    (*p).rel_traffic_out += (*p).bytes_written;
    (*p).bytes_written = 0 as libc::c_int as libc::c_double;
    (*p).traffic_out = 0 as libc::c_int as libc::c_double;
    (*p).requests = 0 as libc::c_int as libc::c_double;
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_status_account(
    mut r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    (*p).requests += 1.;
    (*p).rel_requests += 1.;
    (*p).abs_requests += 1.;
    (*p).bytes_written += (*(*r).con).bytes_written_cur_second as libc::c_double;
    return HANDLER_GO_ON;
}
#[no_mangle]
pub unsafe extern "C" fn mod_status_plugin_init(mut p: *mut plugin) -> libc::c_int {
    (*p).version = 0x10440 as libc::c_int as size_t;
    (*p).name = b"status\0" as *const u8 as *const libc::c_char;
    (*p)
        .init = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
    >(Some(mod_status_init as unsafe extern "C" fn() -> *mut libc::c_void));
    (*p)
        .set_defaults = Some(
        mod_status_set_defaults
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_uri_clean = Some(
        mod_status_handler
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_trigger = Some(
        mod_status_trigger
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_request_done = Some(
        mod_status_account
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    return 0 as libc::c_int;
}
pub unsafe fn run_static_initializers() {
    cpk = [
        {
            let mut init = config_plugin_keys_t {
                k: b"status.status-url\0" as *const u8 as *const libc::c_char,
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
                k: b"status.config-url\0" as *const u8 as *const libc::c_char,
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
                k: b"status.statistics-url\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"status.enable-sort\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
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
