use ::libc;
extern "C" {
    pub type fdnode_st;
    pub type stat_cache_entry;
    pub type pcre2_real_match_data_8;
    pub type h2con;
    pub type fdevents;
    fn buffer_move(b: *mut buffer, src: *mut buffer);
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_copy_string_len_lc(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_eq_icase_ssn(
        a: *const libc::c_char,
        b: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
    fn buffer_eq_icase_ss(
        a: *const libc::c_char,
        alen: size_t,
        b: *const libc::c_char,
        blen: size_t,
    ) -> libc::c_int;
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
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn array_init(n: uint32_t) -> *mut array;
    fn array_free_data(a: *mut array);
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
    fn array_insert_value(a: *mut array, v: *const libc::c_char, vlen: uint32_t);
    fn http_request_host_policy(
        b: *mut buffer,
        http_parseopts: libc::c_uint,
        scheme_port: libc::c_int,
    ) -> libc::c_int;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn sock_addr_from_str_numeric(
        saddr: *mut sock_addr,
        str: *const libc::c_char,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn sock_addr_inet_ntop_copy_buffer(
        b: *mut buffer,
        saddr: *const sock_addr,
    ) -> libc::c_int;
    fn sock_addr_inet_pton(
        saddr: *mut sock_addr,
        str: *const libc::c_char,
        family: libc::c_int,
        port: libc::c_ushort,
    ) -> libc::c_int;
    fn sock_addr_assign(
        saddr: *mut sock_addr,
        family: libc::c_int,
        nport: libc::c_ushort,
        naddr: *const libc::c_void,
    ) -> libc::c_int;
    fn sock_addr_is_addr_eq_bits(
        a: *const sock_addr,
        b: *const sock_addr,
        bits: libc::c_int,
    ) -> libc::c_int;
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
    fn http_header_request_get(
        r: *const request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn http_header_request_set_ptr(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn http_header_env_get(
        r: *const request_st,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn http_header_env_set(
        r: *mut request_st,
        k: *const libc::c_char,
        klen: uint32_t,
        v: *const libc::c_char,
        vlen: uint32_t,
    );
    fn config_cond_cache_reset_item(r: *mut request_st, item: comp_key_t);
    fn config_plugin_value_tobool(
        du: *const data_unset,
        default_value: libc::c_int,
    ) -> libc::c_int;
    fn config_plugin_values_init(
        srv: *mut server,
        p_d: *mut libc::c_void,
        cpk: *const config_plugin_keys_t,
        mname: *const libc::c_char,
    ) -> libc::c_int;
    fn config_check_cond(r: *mut request_st, context_ndx: libc::c_int) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
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
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed_5 = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed_5 = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed_5 = 67108864;
pub const MSG_BATCH: C2RustUnnamed_5 = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed_5 = 65536;
pub const MSG_MORE: C2RustUnnamed_5 = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed_5 = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed_5 = 8192;
pub const MSG_RST: C2RustUnnamed_5 = 4096;
pub const MSG_CONFIRM: C2RustUnnamed_5 = 2048;
pub const MSG_SYN: C2RustUnnamed_5 = 1024;
pub const MSG_FIN: C2RustUnnamed_5 = 512;
pub const MSG_WAITALL: C2RustUnnamed_5 = 256;
pub const MSG_EOR: C2RustUnnamed_5 = 128;
pub const MSG_DONTWAIT: C2RustUnnamed_5 = 64;
pub const MSG_TRUNC: C2RustUnnamed_5 = 32;
pub const MSG_PROXY: C2RustUnnamed_5 = 16;
pub const MSG_CTRUNC: C2RustUnnamed_5 = 8;
pub const MSG_TRYHARD: C2RustUnnamed_5 = 4;
pub const MSG_DONTROUTE: C2RustUnnamed_5 = 4;
pub const MSG_PEEK: C2RustUnnamed_5 = 2;
pub const MSG_OOB: C2RustUnnamed_5 = 1;
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
pub type proxy_forwarded_t = libc::c_uint;
pub const PROXY_FORWARDED_REMOTE_USER: proxy_forwarded_t = 16;
pub const PROXY_FORWARDED_BY: proxy_forwarded_t = 8;
pub const PROXY_FORWARDED_HOST: proxy_forwarded_t = 4;
pub const PROXY_FORWARDED_PROTO: proxy_forwarded_t = 2;
pub const PROXY_FORWARDED_FOR: proxy_forwarded_t = 1;
pub const PROXY_FORWARDED_NONE: proxy_forwarded_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sock_addr_mask {
    pub addr: sock_addr,
    pub bits: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct forwarder_cfg {
    pub forwarder: *const array,
    pub forward_all: libc::c_int,
    pub addrs_used: uint32_t,
    pub addrs: [sock_addr_mask; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_config {
    pub forwarder: *const array,
    pub forward_all: libc::c_int,
    pub forward_masks_used: uint32_t,
    pub forward_masks: *const sock_addr_mask,
    pub headers: *const array,
    pub opts: libc::c_uint,
    pub hap_PROXY: libc::c_char,
    pub hap_PROXY_ssl_client_verify: libc::c_char,
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
    pub default_headers: *mut array,
    pub tokens: array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct handler_ctx {
    pub saved_remote_addr: sock_addr,
    pub saved_remote_addr_buf: buffer,
    pub saved_network_read: Option::<
        unsafe extern "C" fn(*mut connection, *mut chunkqueue, off_t) -> libc::c_int,
    >,
    pub env: *mut array,
    pub ssl_client_verify: libc::c_int,
    pub request_count: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union hap_PROXY_hdr {
    pub v1: C2RustUnnamed_12,
    pub v2: C2RustUnnamed_7,
    pub ext: [uint64_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub sig: [uint8_t; 12],
    pub ver_cmd: uint8_t,
    pub fam: uint8_t,
    pub len: uint16_t,
    pub addr: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub ip4: C2RustUnnamed_11,
    pub ip6: C2RustUnnamed_10,
    pub unx: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub src_addr: [uint8_t; 108],
    pub dst_addr: [uint8_t; 108],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub src_addr: [uint8_t; 16],
    pub dst_addr: [uint8_t; 16],
    pub src_port: uint16_t,
    pub dst_port: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub src_addr: uint32_t,
    pub dst_addr: uint32_t,
    pub src_port: uint16_t,
    pub dst_port: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub line: [libc::c_char; 108],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pp2_tlv {
    pub type_0: uint8_t,
    pub length_hi: uint8_t,
    pub length_lo: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pp2_tlv_ssl {
    pub client: uint8_t,
    pub verify: uint32_t,
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
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
unsafe extern "C" fn light_isxdigit(mut c: libc::c_int) -> libc::c_int {
    return (light_isdigit(c) != 0
        || (c as uint32_t | 0x20 as libc::c_int as libc::c_uint)
            .wrapping_sub('a' as i32 as libc::c_uint)
            <= ('f' as i32 - 'a' as i32) as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn buffer_is_unset(mut b: *const buffer) -> libc::c_int {
    return (0 as libc::c_int as libc::c_uint == (*b).used) as libc::c_int;
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
unsafe extern "C" fn sock_addr_get_family(mut saddr: *const sock_addr) -> libc::c_int {
    return (*saddr).plain.sa_family as libc::c_int;
}
static mut mod_extforward_plugin_data_singleton: *mut plugin_data = 0
    as *const plugin_data as *mut plugin_data;
static mut extforward_check_proxy: libc::c_int = 0;
unsafe extern "C" fn handler_ctx_init() -> *mut handler_ctx {
    let mut hctx: *mut handler_ctx = 0 as *mut handler_ctx;
    hctx = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<handler_ctx>() as libc::c_ulong,
    ) as *mut handler_ctx;
    if hctx.is_null() {
        ck_assert_failed(
            b"src/mod_extforward.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int as libc::c_uint,
            b"hctx\0" as *const u8 as *const libc::c_char,
        );
    }
    return hctx;
}
unsafe extern "C" fn handler_ctx_free(mut hctx: *mut handler_ctx) {
    free((*hctx).saved_remote_addr_buf.ptr as *mut libc::c_void);
    free(hctx as *mut libc::c_void);
}
#[cold]
unsafe extern "C" fn mod_extforward_init() -> *mut libc::c_void {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<plugin_data>() as libc::c_ulong,
    );
}
#[cold]
unsafe extern "C" fn mod_extforward_free(mut p_d: *mut libc::c_void) {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    array_free((*p).default_headers);
    array_free_data(&mut (*p).tokens);
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
unsafe extern "C" fn mod_extforward_merge_config_cpv(
    pconf: *mut plugin_config,
    cpv: *const config_plugin_value_t,
) {
    match (*cpv).k_id {
        0 => {
            if (*cpv).vtype as libc::c_uint
                == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
            {
                let fwd: *const forwarder_cfg = (*cpv).v.v as *const forwarder_cfg;
                (*pconf).forwarder = (*fwd).forwarder;
                (*pconf).forward_all = (*fwd).forward_all;
                (*pconf).forward_masks_used = (*fwd).addrs_used;
                (*pconf).forward_masks = ((*fwd).addrs).as_ptr();
            }
        }
        1 => {
            (*pconf).headers = (*cpv).v.a;
        }
        2 => {
            if (*cpv).vtype as libc::c_uint
                == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
            {
                (*pconf).opts = (*cpv).v.u;
            }
        }
        3 => {
            (*pconf).hap_PROXY = (*cpv).v.u as libc::c_char;
        }
        4 => {
            (*pconf).hap_PROXY_ssl_client_verify = (*cpv).v.u as libc::c_char;
        }
        _ => return,
    };
}
unsafe extern "C" fn mod_extforward_merge_config(
    pconf: *mut plugin_config,
    mut cpv: *const config_plugin_value_t,
) {
    loop {
        mod_extforward_merge_config_cpv(pconf, cpv);
        cpv = cpv.offset(1);
        if !((*cpv).k_id != -(1 as libc::c_int)) {
            break;
        }
    };
}
unsafe extern "C" fn mod_extforward_patch_config(
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
            mod_extforward_merge_config(
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
unsafe extern "C" fn mod_extforward_parse_forwarder(
    mut srv: *mut server,
    mut forwarder: *const array,
) -> *mut libc::c_void {
    let allds: *const data_string = array_get_element_klen(
        forwarder,
        b"all\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    ) as *const data_string;
    let forward_all: libc::c_int = if allds.is_null() {
        0 as libc::c_int
    } else if buffer_eq_icase_slen(
            &(*allds).value,
            b"trust\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    let mut nmasks: uint32_t = 0 as libc::c_int as uint32_t;
    let mut j: uint32_t = 0 as libc::c_int as uint32_t;
    while j < (*forwarder).used {
        let ds: *mut data_string = *((*forwarder).data).offset(j as isize)
            as *mut data_string;
        let nm_slash: *mut libc::c_char = strchr((*ds).key.ptr, '/' as i32);
        if !nm_slash.is_null() {
            nmasks = nmasks.wrapping_add(1);
        }
        if buffer_eq_icase_slen(
            &mut (*ds).value,
            b"trust\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) == 0
        {
            if buffer_eq_icase_slen(
                &mut (*ds).value,
                b"untrusted\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) == 0
            {
                log_error(
                    (*srv).errh,
                    b"src/mod_extforward.c\0" as *const u8 as *const libc::c_char,
                    213 as libc::c_int as libc::c_uint,
                    b"ERROR: expect \"trust\", not \"%s\" => \"%s\"; treating as untrusted\0"
                        as *const u8 as *const libc::c_char,
                    (*ds).key.ptr,
                    (*ds).value.ptr,
                );
            }
            if !nm_slash.is_null() {
                nmasks = nmasks.wrapping_sub(1);
                log_error(
                    (*srv).errh,
                    b"src/mod_extforward.c\0" as *const u8 as *const libc::c_char,
                    220 as libc::c_int as libc::c_uint,
                    b"ERROR: untrusted CIDR masks are ignored (\"%s\" => \"%s\")\0"
                        as *const u8 as *const libc::c_char,
                    (*ds).key.ptr,
                    (*ds).value.ptr,
                );
            }
            buffer_clear(&mut (*ds).value);
        }
        j = j.wrapping_add(1);
    }
    let fwd: *mut forwarder_cfg = malloc(
        (::std::mem::size_of::<forwarder_cfg>() as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<sock_addr_mask>() as libc::c_ulong)
                    .wrapping_mul(nmasks as libc::c_ulong),
            ),
    ) as *mut forwarder_cfg;
    if fwd.is_null() {
        ck_assert_failed(
            b"src/mod_extforward.c\0" as *const u8 as *const libc::c_char,
            231 as libc::c_int as libc::c_uint,
            b"fwd\0" as *const u8 as *const libc::c_char,
        );
    }
    memset(
        fwd as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<forwarder_cfg>() as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<sock_addr_mask>() as libc::c_ulong)
                    .wrapping_mul(nmasks as libc::c_ulong),
            ),
    );
    (*fwd).forwarder = forwarder;
    (*fwd).forward_all = forward_all;
    (*fwd).addrs_used = 0 as libc::c_int as uint32_t;
    let mut j_0: uint32_t = 0 as libc::c_int as uint32_t;
    while j_0 < (*forwarder).used {
        let ds_0: *mut data_string = *((*forwarder).data).offset(j_0 as isize)
            as *mut data_string;
        let nm_slash_0: *mut libc::c_char = strchr((*ds_0).key.ptr, '/' as i32);
        if !nm_slash_0.is_null() {
            if !(buffer_is_blank(&mut (*ds_0).value) != 0) {
                let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
                let nm_bits: libc::c_int = strtol(
                    nm_slash_0.offset(1 as libc::c_int as isize),
                    &mut err,
                    10 as libc::c_int,
                ) as libc::c_int;
                let mut rc: libc::c_int = 0;
                if *err as libc::c_int != 0 || nm_bits <= 0 as libc::c_int
                    || light_isdigit(
                        *nm_slash_0.offset(1 as libc::c_int as isize) as libc::c_int,
                    ) == 0
                {
                    log_error(
                        (*srv).errh,
                        b"src/mod_extforward.c\0" as *const u8 as *const libc::c_char,
                        247 as libc::c_int as libc::c_uint,
                        b"ERROR: invalid netmask: %s %s\0" as *const u8
                            as *const libc::c_char,
                        (*ds_0).key.ptr,
                        err,
                    );
                    free(fwd as *mut libc::c_void);
                    return 0 as *mut libc::c_void;
                }
                let fresh0 = (*fwd).addrs_used;
                (*fwd).addrs_used = ((*fwd).addrs_used).wrapping_add(1);
                let sm: *mut sock_addr_mask = ((*fwd).addrs)
                    .as_mut_ptr()
                    .offset(fresh0 as isize);
                (*sm).bits = nm_bits;
                *nm_slash_0 = '\u{0}' as i32 as libc::c_char;
                rc = sock_addr_from_str_numeric(
                    &mut (*sm).addr,
                    (*ds_0).key.ptr,
                    (*srv).errh,
                );
                *nm_slash_0 = '/' as i32 as libc::c_char;
                if 1 as libc::c_int != rc {
                    free(fwd as *mut libc::c_void);
                    return 0 as *mut libc::c_void;
                }
                buffer_clear(&mut (*ds_0).value);
            }
        }
        j_0 = j_0.wrapping_add(1);
    }
    return fwd as *mut libc::c_void;
}
unsafe extern "C" fn mod_extforward_parse_opts(
    mut srv: *mut server,
    mut opts_params: *const array,
) -> libc::c_uint {
    let mut opts: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut j: uint32_t = 0 as libc::c_int as uint32_t;
    let mut used: uint32_t = (*opts_params).used;
    while j < used {
        let mut param: proxy_forwarded_t = PROXY_FORWARDED_NONE;
        let mut du: *mut data_unset = *((*opts_params).data).offset(j as isize);
        if buffer_eq_slen(
            &mut (*du).key,
            b"host\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            param = PROXY_FORWARDED_HOST;
        } else if buffer_eq_slen(
                &mut (*du).key,
                b"remote_user\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            param = PROXY_FORWARDED_REMOTE_USER;
        } else {
            log_error(
                (*srv).errh,
                b"src/mod_extforward.c\0" as *const u8 as *const libc::c_char,
                290 as libc::c_int as libc::c_uint,
                b"extforward.params keys must be one of: host, remote_user, but not: %s\0"
                    as *const u8 as *const libc::c_char,
                (*du).key.ptr,
            );
            return (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint);
        }
        let mut val: libc::c_int = config_plugin_value_tobool(du, 2 as libc::c_int);
        if 2 as libc::c_int == val {
            log_error(
                (*srv).errh,
                b"src/mod_extforward.c\0" as *const u8 as *const libc::c_char,
                298 as libc::c_int as libc::c_uint,
                b"extforward.params values must be one of: 0, 1, enable, disable; error for key: %s\0"
                    as *const u8 as *const libc::c_char,
                (*du).key.ptr,
            );
            return (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint);
        }
        if val != 0 {
            opts |= param as libc::c_uint;
        }
        j = j.wrapping_add(1);
    }
    return opts;
}
static mut cpk: [config_plugin_keys_t; 6] = [config_plugin_keys_t {
    k: 0 as *const libc::c_char,
    klen: 0,
    ktype: 0,
    scope: 0,
}; 6];
#[cold]
unsafe extern "C" fn mod_extforward_set_defaults(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    if config_plugin_values_init(
        srv,
        p as *mut libc::c_void,
        cpk.as_ptr(),
        b"mod_extforward\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return HANDLER_ERROR;
    }
    let mut hap_PROXY: libc::c_int = 0 as libc::c_int;
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
                    (*cpv).v.v = mod_extforward_parse_forwarder(srv, (*cpv).v.a);
                    if ((*cpv).v.v).is_null() {
                        log_error(
                            (*srv).errh,
                            b"src/mod_extforward.c\0" as *const u8
                                as *const libc::c_char,
                            346 as libc::c_int as libc::c_uint,
                            b"unexpected value for %s\0" as *const u8
                                as *const libc::c_char,
                            cpk[(*cpv).k_id as usize].k,
                        );
                        return HANDLER_ERROR;
                    }
                    (*cpv).vtype = T_CONFIG_LOCAL;
                }
                1 => {
                    if (*(*cpv).v.a).used != 0 {
                        let mut a: *mut array = 0 as *mut array;
                        let ref mut fresh1 = *(&mut a as *mut *mut array
                            as *mut *const array);
                        *fresh1 = (*cpv).v.a;
                        let mut j: uint32_t = 0 as libc::c_int as uint32_t;
                        while j < (*a).used {
                            let ds: *mut data_string = *((*a).data).offset(j as isize)
                                as *mut data_string;
                            (*ds)
                                .ext = http_header_hkey_get(
                                (*ds).value.ptr,
                                buffer_clen(&mut (*ds).value) as size_t,
                            ) as libc::c_int;
                            j = j.wrapping_add(1);
                        }
                    }
                }
                2 => {
                    (*cpv).v.u = mod_extforward_parse_opts(srv, (*cpv).v.a);
                    if (2147483647 as libc::c_int as libc::c_uint)
                        .wrapping_mul(2 as libc::c_uint)
                        .wrapping_add(1 as libc::c_uint) == (*cpv).v.u
                    {
                        return HANDLER_ERROR;
                    }
                }
                3 => {
                    if (*cpv).v.u != 0 {
                        hap_PROXY = 1 as libc::c_int;
                    }
                }
                4 => {}
                _ => {}
            }
            cpv = cpv.offset(1);
        }
        i += 1;
    }
    mod_extforward_plugin_data_singleton = p;
    (*p).defaults.opts = PROXY_FORWARDED_NONE as libc::c_int as libc::c_uint;
    if (*p).nconfig > 0 as libc::c_int
        && (*(*p).cvlist).v.u2[1 as libc::c_int as usize] != 0
    {
        let mut cpv_0: *const config_plugin_value_t = ((*p).cvlist)
            .offset((*(*p).cvlist).v.u2[0 as libc::c_int as usize] as isize);
        if -(1 as libc::c_int) != (*cpv_0).k_id {
            mod_extforward_merge_config(&mut (*p).defaults, cpv_0);
        }
    }
    if (*p).defaults.hap_PROXY == 0
        && (((*p).defaults.headers).is_null()
            || 0 as libc::c_int as libc::c_uint == (*(*p).defaults.headers).used)
    {
        (*p).default_headers = array_init(2 as libc::c_int as uint32_t);
        (*p).defaults.headers = (*p).default_headers;
        array_insert_value(
            (*p).default_headers,
            b"X-Forwarded-For\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        array_insert_value(
            (*p).default_headers,
            b"Forwarded-For\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
        while i_0 < (*(*p).default_headers).used {
            let ds_0: *mut data_string = *((*(*p).default_headers).data)
                .offset(i_0 as isize) as *mut data_string;
            (*ds_0)
                .ext = http_header_hkey_get(
                (*ds_0).value.ptr,
                buffer_clen(&mut (*ds_0).value) as size_t,
            ) as libc::c_int;
            i_0 = i_0.wrapping_add(1);
        }
    }
    if hap_PROXY != 0 {
        let mut i_1: uint32_t = 0;
        i_1 = 0 as libc::c_int as uint32_t;
        while i_1 < (*(*srv).srvconf.modules).used {
            let mut ds_1: *mut data_string = *((*(*srv).srvconf.modules).data)
                .offset(i_1 as isize) as *mut data_string;
            if buffer_eq_slen(
                &mut (*ds_1).value,
                b"mod_extforward\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
                break;
            }
            i_1 = i_1.wrapping_add(1);
        }
        while i_1 < (*(*srv).srvconf.modules).used {
            let mut ds_2: *mut data_string = *((*(*srv).srvconf.modules).data)
                .offset(i_1 as isize) as *mut data_string;
            if buffer_eq_slen(
                &mut (*ds_2).value,
                b"mod_openssl\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
                || buffer_eq_slen(
                    &mut (*ds_2).value,
                    b"mod_mbedtls\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                ) != 0
                || buffer_eq_slen(
                    &mut (*ds_2).value,
                    b"mod_wolfssl\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                ) != 0
                || buffer_eq_slen(
                    &mut (*ds_2).value,
                    b"mod_nss\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                ) != 0
                || buffer_eq_slen(
                    &mut (*ds_2).value,
                    b"mod_gnutls\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                ) != 0
            {
                log_error(
                    (*srv).errh,
                    b"src/mod_extforward.c\0" as *const u8 as *const libc::c_char,
                    423 as libc::c_int as libc::c_uint,
                    b"mod_extforward must be loaded after %s in server.modules when extforward.hap-PROXY = \"enable\"\0"
                        as *const u8 as *const libc::c_char,
                    (*ds_2).value.ptr,
                );
                break;
            } else {
                i_1 = i_1.wrapping_add(1);
            }
        }
    }
    let mut i_2: uint32_t = 0 as libc::c_int as uint32_t;
    while i_2 < (*(*srv).srvconf.modules).used {
        let mut ds_3: *mut data_string = *((*(*srv).srvconf.modules).data)
            .offset(i_2 as isize) as *mut data_string;
        if buffer_eq_slen(
            &mut (*ds_3).value,
            b"mod_proxy\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            extforward_check_proxy = 1 as libc::c_int;
            break;
        } else {
            i_2 = i_2.wrapping_add(1);
        }
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn extract_forward_array(
    result: *mut array,
    mut pbuffer: *const buffer,
) {
    let mut base: *const libc::c_char = 0 as *const libc::c_char;
    let mut curr: *const libc::c_char = 0 as *const libc::c_char;
    let mut in_str: libc::c_int = 0 as libc::c_int;
    base = (*pbuffer).ptr;
    curr = (*pbuffer).ptr;
    while *curr != 0 {
        let mut hex_or_colon: libc::c_int = (light_isxdigit(*curr as libc::c_int) != 0
            || *curr as libc::c_int == ':' as i32) as libc::c_int;
        if in_str != 0 {
            if hex_or_colon == 0 && *curr as libc::c_int != '.' as i32 {
                array_insert_value(
                    result,
                    base,
                    curr.offset_from(base) as libc::c_long as uint32_t,
                );
                in_str = 0 as libc::c_int;
            }
        } else if hex_or_colon != 0 {
            base = curr;
            in_str = 1 as libc::c_int;
        }
        curr = curr.offset(1);
    }
    if in_str != 0 {
        array_insert_value(
            result,
            base,
            curr.offset_from(base) as libc::c_long as uint32_t,
        );
    }
}
unsafe extern "C" fn is_proxy_trusted(
    mut p: *mut plugin_data,
    ip: *const libc::c_char,
    mut iplen: size_t,
) -> libc::c_int {
    let mut ds: *const data_string = array_get_element_klen(
        (*p).conf.forwarder,
        ip,
        iplen as uint32_t,
    ) as *const data_string;
    if !ds.is_null() {
        return (buffer_is_blank(&(*ds).value) == 0) as libc::c_int;
    }
    if (*p).conf.forward_masks_used != 0 {
        let addrs: *const sock_addr_mask = (*p).conf.forward_masks;
        let aused: uint32_t = (*p).conf.forward_masks_used;
        let mut addr: sock_addr = sock_addr {
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
        let mut addrstr: [libc::c_char; 64] = [0; 64];
        if 0 as libc::c_int as libc::c_ulong == iplen
            || iplen >= ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
        {
            return 0 as libc::c_int;
        }
        memcpy(
            addrstr.as_mut_ptr() as *mut libc::c_void,
            ip as *const libc::c_void,
            iplen,
        );
        addrstr[iplen as usize] = '\u{0}' as i32 as libc::c_char;
        if 1 as libc::c_int
            != sock_addr_inet_pton(
                &mut addr,
                addrstr.as_mut_ptr(),
                2 as libc::c_int,
                0 as libc::c_int as libc::c_ushort,
            )
            && 1 as libc::c_int
                != sock_addr_inet_pton(
                    &mut addr,
                    addrstr.as_mut_ptr(),
                    10 as libc::c_int,
                    0 as libc::c_int as libc::c_ushort,
                )
        {
            return 0 as libc::c_int;
        }
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while i < aused {
            if sock_addr_is_addr_eq_bits(
                &mut addr,
                &(*addrs.offset(i as isize)).addr,
                (*addrs.offset(i as isize)).bits,
            ) != 0
            {
                return 1 as libc::c_int;
            }
            i = i.wrapping_add(1);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn is_connection_trusted(
    con: *mut connection,
    mut p: *mut plugin_data,
) -> libc::c_int {
    if (*p).conf.forward_all != 0 {
        return (1 as libc::c_int == (*p).conf.forward_all) as libc::c_int;
    }
    return is_proxy_trusted(
        p,
        (*con).dst_addr_buf.ptr,
        buffer_clen(&mut (*con).dst_addr_buf) as size_t,
    );
}
unsafe extern "C" fn last_not_in_array(
    mut a: *mut array,
    mut p: *mut plugin_data,
) -> *const buffer {
    let mut i: libc::c_int = 0;
    i = ((*a).used).wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut ds: *mut data_string = *((*a).data).offset(i as isize)
            as *mut data_string;
        if is_proxy_trusted(p, (*ds).value.ptr, buffer_clen(&mut (*ds).value) as size_t)
            == 0
        {
            return &mut (*ds).value;
        }
        i -= 1;
    }
    return 0 as *const buffer;
}
unsafe extern "C" fn mod_extforward_set_addr(
    r: *mut request_st,
    mut p: *mut plugin_data,
    mut addr: *const libc::c_char,
    mut addrlen: size_t,
) -> libc::c_int {
    let con: *mut connection = (*r).con;
    let mut sock: sock_addr = sock_addr {
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
    let mut hctx: *mut handler_ctx = *((*con).plugin_ctx).offset((*p).id as isize)
        as *mut handler_ctx;
    if !hctx.is_null() && buffer_is_unset(&mut (*hctx).saved_remote_addr_buf) == 0
        && (*r).http_version as libc::c_int > HTTP_VERSION_1_1 as libc::c_int
    {
        if extforward_check_proxy != 0 {
            http_header_env_set(
                r,
                b"_L_EXTFORWARD_ACTUAL_FOR\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                (*hctx).saved_remote_addr_buf.ptr,
                buffer_clen(&mut (*hctx).saved_remote_addr_buf),
            );
        }
        return 1 as libc::c_int;
    }
    if (*r).conf.log_request_handling != 0 {
        log_error(
            (*r).conf.errh,
            b"src/mod_extforward.c\0" as *const u8 as *const libc::c_char,
            546 as libc::c_int as libc::c_uint,
            b"using address: %s\0" as *const u8 as *const libc::c_char,
            addr,
        );
    }
    sock.plain.sa_family = 0 as libc::c_int as sa_family_t;
    if 1 as libc::c_int != sock_addr_from_str_numeric(&mut sock, addr, (*r).conf.errh) {
        return 0 as libc::c_int;
    }
    if sock.plain.sa_family as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if !hctx.is_null() {
        if buffer_is_unset(&mut (*hctx).saved_remote_addr_buf) == 0 {
            if (*r).conf.log_request_handling != 0 {
                log_error(
                    (*r).conf.errh,
                    b"src/mod_extforward.c\0" as *const u8 as *const libc::c_char,
                    557 as libc::c_int as libc::c_uint,
                    b"-- mod_extforward_uri_handler already patched this connection, resetting state\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            (*con).dst_addr = (*hctx).saved_remote_addr;
            buffer_move(&mut (*con).dst_addr_buf, &mut (*hctx).saved_remote_addr_buf);
        }
    } else {
        hctx = handler_ctx_init();
        let ref mut fresh2 = *((*con).plugin_ctx).offset((*p).id as isize);
        *fresh2 = hctx as *mut libc::c_void;
    }
    if extforward_check_proxy != 0 {
        http_header_env_set(
            r,
            b"_L_EXTFORWARD_ACTUAL_FOR\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            (*con).dst_addr_buf.ptr,
            buffer_clen(&mut (*con).dst_addr_buf),
        );
    }
    (*hctx).request_count = (*con).request_count;
    (*hctx).saved_remote_addr = (*con).dst_addr;
    buffer_move(&mut (*hctx).saved_remote_addr_buf, &mut (*con).dst_addr_buf);
    (*con).dst_addr = sock;
    buffer_copy_string_len(&mut (*con).dst_addr_buf, addr, addrlen);
    config_cond_cache_reset_item(r, COMP_HTTP_REMOTE_IP);
    return 1 as libc::c_int;
}
unsafe extern "C" fn mod_extforward_set_proto(
    r: *mut request_st,
    proto: *const libc::c_char,
    mut protolen: size_t,
) {
    if 0 as libc::c_int as libc::c_ulong != protolen
        && buffer_eq_icase_slen(&mut (*r).uri.scheme, proto, protolen) == 0
    {
        if extforward_check_proxy != 0 {
            http_header_env_set(
                r,
                b"_L_EXTFORWARD_ACTUAL_PROTO\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                (*r).uri.scheme.ptr,
                buffer_clen(&mut (*r).uri.scheme),
            );
        }
        if buffer_eq_icase_ss(
            proto,
            protolen,
            b"https\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            (*(*r).con).proto_default_port = 443 as libc::c_int as uint16_t;
            buffer_copy_string_len(
                &mut (*r).uri.scheme,
                b"https\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            config_cond_cache_reset_item(r, COMP_HTTP_SCHEME);
        } else if buffer_eq_icase_ss(
                proto,
                protolen,
                b"http\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            (*(*r).con).proto_default_port = 80 as libc::c_int as uint16_t;
            buffer_copy_string_len(
                &mut (*r).uri.scheme,
                b"http\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            config_cond_cache_reset_item(r, COMP_HTTP_SCHEME);
        }
    }
}
unsafe extern "C" fn mod_extforward_X_Forwarded_For(
    r: *mut request_st,
    p: *mut plugin_data,
    x_forwarded_for: *const buffer,
) -> handler_t {
    let forward_array: *mut array = &mut (*p).tokens;
    extract_forward_array(forward_array, x_forwarded_for);
    let mut real_remote_addr: *const buffer = last_not_in_array(forward_array, p);
    if !real_remote_addr.is_null() {
        let mut x_forwarded_proto: *const buffer = http_header_request_get(
            r,
            HTTP_HEADER_X_FORWARDED_PROTO,
            b"X-Forwarded-Proto\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        if mod_extforward_set_addr(
            r,
            p,
            (*real_remote_addr).ptr,
            buffer_clen(real_remote_addr) as size_t,
        ) != 0 && !x_forwarded_proto.is_null()
        {
            mod_extforward_set_proto(
                r,
                (*x_forwarded_proto).ptr,
                buffer_clen(x_forwarded_proto) as size_t,
            );
        }
    }
    array_reset_data_strings(forward_array);
    return HANDLER_GO_ON;
}
unsafe extern "C" fn find_end_quoted_string(
    s: *const libc::c_char,
    mut i: libc::c_int,
) -> libc::c_int {
    loop {
        i += 1;
        if !(*s.offset(i as isize) as libc::c_int != '"' as i32
            && *s.offset(i as isize) as libc::c_int != '\u{0}' as i32
            && (*s.offset(i as isize) as libc::c_int != '\\' as i32
                || {
                    i += 1;
                    *s.offset(i as isize) as libc::c_int != '\u{0}' as i32
                }))
        {
            break;
        }
    }
    return i;
}
unsafe extern "C" fn find_next_semicolon_or_comma_or_eq(
    s: *const libc::c_char,
    mut i: libc::c_int,
) -> libc::c_int {
    while *s.offset(i as isize) as libc::c_int != '=' as i32
        && *s.offset(i as isize) as libc::c_int != ';' as i32
        && *s.offset(i as isize) as libc::c_int != ',' as i32
        && *s.offset(i as isize) as libc::c_int != '\u{0}' as i32
    {
        if *s.offset(i as isize) as libc::c_int == '"' as i32 {
            i = find_end_quoted_string(s, i);
            if *s.offset(i as isize) as libc::c_int == '\u{0}' as i32 {
                return -(1 as libc::c_int);
            }
        }
        i += 1;
    }
    return i;
}
unsafe extern "C" fn find_next_semicolon_or_comma(
    s: *const libc::c_char,
    mut i: libc::c_int,
) -> libc::c_int {
    while *s.offset(i as isize) as libc::c_int != ';' as i32
        && *s.offset(i as isize) as libc::c_int != ',' as i32
        && *s.offset(i as isize) as libc::c_int != '\u{0}' as i32
    {
        if *s.offset(i as isize) as libc::c_int == '"' as i32 {
            i = find_end_quoted_string(s, i);
            if *s.offset(i as isize) as libc::c_int == '\u{0}' as i32 {
                return -(1 as libc::c_int);
            }
        }
        i += 1;
    }
    return i;
}
unsafe extern "C" fn buffer_backslash_unescape(b: *mut buffer) -> libc::c_int {
    let mut j: size_t = 0 as libc::c_int as size_t;
    let mut len: size_t = buffer_clen(b) as size_t;
    let mut p: *mut libc::c_char = memchr(
        (*b).ptr as *const libc::c_void,
        '\\' as i32,
        len,
    ) as *mut libc::c_char;
    if p.is_null() {
        return 1 as libc::c_int;
    }
    len = (len as libc::c_ulong)
        .wrapping_sub(p.offset_from((*b).ptr) as libc::c_long as size_t) as size_t
        as size_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < len {
        if *p.offset(i as isize) as libc::c_int == '\\' as i32 {
            i = i.wrapping_add(1);
            if i == len {
                return 0 as libc::c_int;
            }
        }
        let fresh3 = j;
        j = j.wrapping_add(1);
        *p.offset(fresh3 as isize) = *p.offset(i as isize);
        i = i.wrapping_add(1);
    }
    buffer_truncate(
        b,
        p.offset(j as isize).offset_from((*b).ptr) as libc::c_long as size_t as uint32_t,
    );
    return 1 as libc::c_int;
}
#[cold]
unsafe extern "C" fn mod_extforward_bad_request(
    r: *mut request_st,
    line: libc::c_uint,
    msg: *const libc::c_char,
) -> handler_t {
    (*r).http_status = 400 as libc::c_int;
    (*r).handler_module = 0 as *const plugin;
    log_error(
        (*r).conf.errh,
        b"src/mod_extforward.c\0" as *const u8 as *const libc::c_char,
        line,
        b"%s\0" as *const u8 as *const libc::c_char,
        msg,
    );
    return HANDLER_FINISHED;
}
unsafe extern "C" fn mod_extforward_Forwarded(
    r: *mut request_st,
    p: *mut plugin_data,
    forwarded: *const buffer,
) -> handler_t {
    let s: *mut libc::c_char = (*forwarded).ptr;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = -(1 as libc::c_int);
    let mut v: libc::c_int = 0;
    let mut vlen: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut klen: libc::c_int = 0;
    let mut used: libc::c_int = buffer_clen(forwarded) as libc::c_int;
    let mut ofor: libc::c_int = -(1 as libc::c_int);
    let mut oproto: libc::c_int = 0;
    let mut ohost: libc::c_int = 0;
    let mut oby: libc::c_int = 0;
    let mut oremote_user: libc::c_int = 0;
    let mut offsets: [libc::c_int; 256] = [0; 256];
    while i < used {
        while *s.offset(i as isize) as libc::c_int == ' ' as i32
            || *s.offset(i as isize) as libc::c_int == '\t' as i32
        {
            i += 1;
        }
        if *s.offset(i as isize) as libc::c_int == ';' as i32 {
            i += 1;
        } else if *s.offset(i as isize) as libc::c_int == ',' as i32 {
            if j
                >= (::std::mem::size_of::<[libc::c_int; 256]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    as libc::c_int - 1 as libc::c_int
            {
                break;
            }
            j += 1;
            offsets[j as usize] = -(1 as libc::c_int);
            i += 1;
        } else {
            if *s.offset(i as isize) as libc::c_int == '\u{0}' as i32 {
                break;
            }
            k = i;
            i = find_next_semicolon_or_comma_or_eq(s, i);
            if i < 0 as libc::c_int {
                return mod_extforward_bad_request(
                    r,
                    729 as libc::c_int as libc::c_uint,
                    b"invalid quoted-string in Forwarded header\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if *s.offset(i as isize) as libc::c_int != '=' as i32 {
                continue;
            }
            klen = i - k;
            i += 1;
            v = i;
            i = find_next_semicolon_or_comma(s, i);
            if i < 0 as libc::c_int {
                return mod_extforward_bad_request(
                    r,
                    738 as libc::c_int as libc::c_uint,
                    b"invalid quoted-string in Forwarded header\0" as *const u8
                        as *const libc::c_char,
                );
            }
            vlen = i - v;
            if 0 as libc::c_int == klen {
                continue;
            }
            if j
                >= (::std::mem::size_of::<[libc::c_int; 256]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    as libc::c_int - 4 as libc::c_int
            {
                break;
            }
            offsets[(j + 1 as libc::c_int) as usize] = k;
            offsets[(j + 2 as libc::c_int) as usize] = klen;
            offsets[(j + 3 as libc::c_int) as usize] = v;
            offsets[(j + 4 as libc::c_int) as usize] = vlen;
            j += 4 as libc::c_int;
        }
    }
    if j
        >= (::std::mem::size_of::<[libc::c_int; 256]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int - 4 as libc::c_int
    {
        return mod_extforward_bad_request(
            r,
            758 as libc::c_int as libc::c_uint,
            b"Too many params in Forwarded header\0" as *const u8 as *const libc::c_char,
        );
    }
    if -(1 as libc::c_int) == j {
        return HANDLER_GO_ON;
    }
    used = j + 1 as libc::c_int;
    offsets[used as usize] = -(1 as libc::c_int);
    while j >= 4 as libc::c_int {
        if -(1 as libc::c_int) == offsets[j as usize] {
            j -= 1;
        } else {
            loop {
                j -= 3 as libc::c_int;
                if !((3 as libc::c_int != offsets[(j + 1 as libc::c_int) as usize]
                    || buffer_eq_icase_ssn(
                        s.offset(offsets[j as usize] as isize),
                        b"for\0" as *const u8 as *const libc::c_char,
                        3 as libc::c_int as size_t,
                    ) == 0)
                    && {
                        let fresh4 = j;
                        j = j - 1;
                        0 as libc::c_int != fresh4
                    } && -(1 as libc::c_int) != offsets[j as usize])
                {
                    break;
                }
            }
            if j < 0 as libc::c_int {
                break;
            }
            if -(1 as libc::c_int) == offsets[j as usize] {
                j -= 1;
            } else {
                v = offsets[(j + 2 as libc::c_int) as usize];
                vlen = v + offsets[(j + 3 as libc::c_int) as usize];
                while vlen > v
                    && (*s.offset((vlen - 1 as libc::c_int) as isize) as libc::c_int
                        == ' ' as i32
                        || *s.offset((vlen - 1 as libc::c_int) as isize) as libc::c_int
                            == '\t' as i32)
                {
                    vlen -= 1;
                }
                if vlen > v + 1 as libc::c_int
                    && *s.offset(v as isize) as libc::c_int == '"' as i32
                    && *s.offset((vlen - 1 as libc::c_int) as isize) as libc::c_int
                        == '"' as i32
                {
                    v += 1;
                    offsets[(j + 2 as libc::c_int) as usize] = v;
                    vlen -= 1;
                    if *s.offset(v as isize) as libc::c_int == '[' as i32 {
                        v += 1;
                        loop {
                            vlen -= 1;
                            if !(vlen > v
                                && *s.offset(vlen as isize) as libc::c_int != ']' as i32)
                            {
                                break;
                            }
                        }
                        if v == vlen {
                            return mod_extforward_bad_request(
                                r,
                                790 as libc::c_int as libc::c_uint,
                                b"Invalid IPv6 addr in Forwarded header\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                    } else if *s.offset(v as isize) as libc::c_int != '_' as i32
                            && *s.offset(v as isize) as libc::c_int != '/' as i32
                            && *s.offset(v as isize) as libc::c_int != 'u' as i32
                        {
                        klen = vlen;
                        vlen = v;
                        while vlen < klen
                            && *s.offset(vlen as isize) as libc::c_int != ':' as i32
                        {
                            vlen += 1;
                        }
                    }
                    offsets[(j + 2 as libc::c_int) as usize] = v;
                }
                offsets[(j + 3 as libc::c_int) as usize] = vlen - v;
                if v != vlen {
                    let mut trusted: libc::c_int = is_proxy_trusted(
                        p,
                        s.offset(v as isize),
                        (vlen - v) as size_t,
                    );
                    if *s.offset(v as isize) as libc::c_int != '_' as i32
                        && *s.offset(v as isize) as libc::c_int != '/' as i32
                        && (7 as libc::c_int != vlen - v
                            || 0 as libc::c_int
                                != memcmp(
                                    s.offset(v as isize) as *const libc::c_void,
                                    b"unknown\0" as *const u8 as *const libc::c_char
                                        as *const libc::c_void,
                                    7 as libc::c_int as libc::c_ulong,
                                ))
                    {
                        ofor = j;
                    }
                    if trusted == 0 {
                        break;
                    }
                }
                loop {
                    j -= 1;
                    if !(j > 0 as libc::c_int
                        && -(1 as libc::c_int) != offsets[j as usize])
                    {
                        break;
                    }
                }
                if j <= 0 as libc::c_int {
                    break;
                }
                j -= 1;
            }
        }
    }
    if -(1 as libc::c_int) != ofor {
        let mut ipend: *mut libc::c_char = s
            .offset(offsets[(ofor + 2 as libc::c_int) as usize] as isize)
            .offset(offsets[(ofor + 3 as libc::c_int) as usize] as isize);
        let mut c: libc::c_char = *ipend;
        let mut rc: libc::c_int = 0;
        *ipend = '\u{0}' as i32 as libc::c_char;
        rc = mod_extforward_set_addr(
            r,
            p,
            s.offset(offsets[(ofor + 2 as libc::c_int) as usize] as isize),
            offsets[(ofor + 3 as libc::c_int) as usize] as size_t,
        );
        *ipend = c;
        if rc == 0 {
            return HANDLER_GO_ON;
        }
    } else {
        return HANDLER_GO_ON
    }
    oremote_user = -(1 as libc::c_int);
    oby = oremote_user;
    ohost = oby;
    oproto = ohost;
    j = ofor;
    if j > 0 as libc::c_int {
        loop {
            j -= 1;
            if !(j > 0 as libc::c_int && -(1 as libc::c_int) != offsets[j as usize]) {
                break;
            }
        }
    }
    if -(1 as libc::c_int) == offsets[j as usize] {
        j += 1;
    }
    if j == ofor {
        j += 4 as libc::c_int;
    }
    while -(1 as libc::c_int) != offsets[j as usize] {
        match offsets[(j + 1 as libc::c_int) as usize] {
            4 => {
                if buffer_eq_icase_ssn(
                    s.offset(offsets[j as usize] as isize),
                    b"host\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as size_t,
                ) != 0
                {
                    ohost = j;
                }
            }
            5 => {
                if buffer_eq_icase_ssn(
                    s.offset(offsets[j as usize] as isize),
                    b"proto\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as size_t,
                ) != 0
                {
                    oproto = j;
                }
            }
            11 => {
                if buffer_eq_icase_ssn(
                    s.offset(offsets[j as usize] as isize),
                    b"remote_user\0" as *const u8 as *const libc::c_char,
                    11 as libc::c_int as size_t,
                ) != 0
                {
                    oremote_user = j;
                }
            }
            _ => {}
        }
        j += 4 as libc::c_int;
    }
    i = j + 1 as libc::c_int;
    if -(1 as libc::c_int) != oproto {
        v = offsets[(oproto + 2 as libc::c_int) as usize];
        vlen = v + offsets[(oproto + 3 as libc::c_int) as usize];
        while vlen > v
            && (*s.offset((vlen - 1 as libc::c_int) as isize) as libc::c_int
                == ' ' as i32
                || *s.offset((vlen - 1 as libc::c_int) as isize) as libc::c_int
                    == '\t' as i32)
        {
            vlen -= 1;
        }
        if vlen > v + 1 as libc::c_int
            && *s.offset(v as isize) as libc::c_int == '"' as i32
            && *s.offset((vlen - 1 as libc::c_int) as isize) as libc::c_int == '"' as i32
        {
            v += 1;
            vlen -= 1;
        }
        mod_extforward_set_proto(r, s.offset(v as isize), (vlen - v) as size_t);
    }
    if (*p).conf.opts & PROXY_FORWARDED_HOST as libc::c_int as libc::c_uint != 0 {
        j = i;
        while j < used && -(1 as libc::c_int) == ohost {
            if -(1 as libc::c_int) == offsets[j as usize] {
                j += 1;
            } else {
                if 4 as libc::c_int == offsets[(j + 1 as libc::c_int) as usize]
                    && buffer_eq_icase_ssn(
                        s.offset(offsets[j as usize] as isize),
                        b"host\0" as *const u8 as *const libc::c_char,
                        4 as libc::c_int as size_t,
                    ) != 0
                {
                    ohost = j;
                }
                j += 4 as libc::c_int;
            }
        }
        if -(1 as libc::c_int) != ohost {
            if !((*r).http_host).is_null() && buffer_is_blank((*r).http_host) == 0 {
                if extforward_check_proxy != 0 {
                    http_header_env_set(
                        r,
                        b"_L_EXTFORWARD_ACTUAL_HOST\0" as *const u8
                            as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                        (*(*r).http_host).ptr,
                        buffer_clen((*r).http_host),
                    );
                }
            } else {
                (*r)
                    .http_host = http_header_request_set_ptr(
                    r,
                    HTTP_HEADER_HOST,
                    b"Host\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                );
            }
            v = offsets[(ohost + 2 as libc::c_int) as usize];
            vlen = v + offsets[(ohost + 3 as libc::c_int) as usize];
            while vlen > v
                && (*s.offset((vlen - 1 as libc::c_int) as isize) as libc::c_int
                    == ' ' as i32
                    || *s.offset((vlen - 1 as libc::c_int) as isize) as libc::c_int
                        == '\t' as i32)
            {
                vlen -= 1;
            }
            if vlen > v + 1 as libc::c_int
                && *s.offset(v as isize) as libc::c_int == '"' as i32
                && *s.offset((vlen - 1 as libc::c_int) as isize) as libc::c_int
                    == '"' as i32
            {
                v += 1;
                vlen -= 1;
                buffer_copy_string_len_lc(
                    (*r).http_host,
                    s.offset(v as isize),
                    (vlen - v) as size_t,
                );
                if buffer_backslash_unescape((*r).http_host) == 0 {
                    return mod_extforward_bad_request(
                        r,
                        934 as libc::c_int as libc::c_uint,
                        b"invalid host= value in Forwarded header\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else {
                buffer_copy_string_len_lc(
                    (*r).http_host,
                    s.offset(v as isize),
                    (vlen - v) as size_t,
                );
            }
            if 0 as libc::c_int
                != http_request_host_policy(
                    (*r).http_host,
                    (*r).conf.http_parseopts,
                    (*(*r).con).proto_default_port as libc::c_int,
                )
            {
                return mod_extforward_bad_request(
                    r,
                    946 as libc::c_int as libc::c_uint,
                    b"invalid host= value in Forwarded header\0" as *const u8
                        as *const libc::c_char,
                );
            }
            config_cond_cache_reset_item(r, COMP_HTTP_HOST);
        }
    }
    if (*p).conf.opts & PROXY_FORWARDED_REMOTE_USER as libc::c_int as libc::c_uint != 0 {
        j = i;
        while j < used {
            if -(1 as libc::c_int) == offsets[j as usize] {
                j += 1;
            } else {
                if 11 as libc::c_int == offsets[(j + 1 as libc::c_int) as usize]
                    && buffer_eq_icase_ssn(
                        s.offset(offsets[j as usize] as isize),
                        b"remote_user\0" as *const u8 as *const libc::c_char,
                        11 as libc::c_int as size_t,
                    ) != 0
                {
                    oremote_user = j;
                }
                j += 4 as libc::c_int;
            }
        }
        if -(1 as libc::c_int) != oremote_user {
            v = offsets[(oremote_user + 2 as libc::c_int) as usize];
            vlen = v + offsets[(oremote_user + 3 as libc::c_int) as usize];
            while vlen > v
                && (*s.offset((vlen - 1 as libc::c_int) as isize) as libc::c_int
                    == ' ' as i32
                    || *s.offset((vlen - 1 as libc::c_int) as isize) as libc::c_int
                        == '\t' as i32)
            {
                vlen -= 1;
            }
            if vlen > v + 1 as libc::c_int
                && *s.offset(v as isize) as libc::c_int == '"' as i32
                && *s.offset((vlen - 1 as libc::c_int) as isize) as libc::c_int
                    == '"' as i32
            {
                let mut euser: *mut buffer = 0 as *mut buffer;
                v += 1;
                vlen -= 1;
                http_header_env_set(
                    r,
                    b"REMOTE_USER\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    s.offset(v as isize),
                    (vlen - v) as uint32_t,
                );
                euser = http_header_env_get(
                    r,
                    b"REMOTE_USER\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                );
                if euser.is_null() {
                    ck_assert_failed(
                        b"src/mod_extforward.c\0" as *const u8 as *const libc::c_char,
                        976 as libc::c_int as libc::c_uint,
                        b"((void*)0) != euser\0" as *const u8 as *const libc::c_char,
                    );
                }
                if buffer_backslash_unescape(euser) == 0 {
                    return mod_extforward_bad_request(
                        r,
                        978 as libc::c_int as libc::c_uint,
                        b"invalid remote_user= value in Forwarded header\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else {
                http_header_env_set(
                    r,
                    b"REMOTE_USER\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    s.offset(v as isize),
                    (vlen - v) as uint32_t,
                );
            }
        }
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_extforward_uri_handler(
    mut r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    mod_extforward_patch_config(r, p);
    if ((*p).conf.forwarder).is_null() {
        return HANDLER_GO_ON;
    }
    if (*p).conf.hap_PROXY_ssl_client_verify != 0 {
        let mut ds: *const data_string = 0 as *const data_string;
        let mut hctx: *mut handler_ctx = *((*(*r).con).plugin_ctx)
            .offset((*p).id as isize) as *mut handler_ctx;
        if !hctx.is_null() && (*hctx).ssl_client_verify != 0 && !((*hctx).env).is_null()
            && {
                ds = array_get_element_klen(
                    (*hctx).env,
                    b"SSL_CLIENT_S_DN_CN\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                ) as *const data_string;
                !ds.is_null()
            }
        {
            http_header_env_set(
                r,
                b"SSL_CLIENT_VERIFY\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                b"SUCCESS\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            );
            http_header_env_set(
                r,
                b"REMOTE_USER\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                (*ds).value.ptr,
                buffer_clen(&(*ds).value),
            );
            http_header_env_set(
                r,
                b"AUTH_TYPE\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                b"SSL_CLIENT_VERIFY\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            );
        } else {
            http_header_env_set(
                r,
                b"SSL_CLIENT_VERIFY\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                b"NONE\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            );
        }
    }
    if ((*p).conf.headers).is_null() {
        return HANDLER_GO_ON;
    }
    let mut hctx_0: *mut handler_ctx = *((*(*r).con).plugin_ctx).offset((*p).id as isize)
        as *mut handler_ctx;
    if !hctx_0.is_null() && buffer_is_unset(&mut (*hctx_0).saved_remote_addr_buf) == 0
        && (*hctx_0).request_count == (*(*r).con).request_count
    {
        return HANDLER_GO_ON;
    }
    let mut forwarded: *const buffer = 0 as *const buffer;
    let mut is_forwarded_header: libc::c_int = 0 as libc::c_int;
    let mut k: uint32_t = 0 as libc::c_int as uint32_t;
    while k < (*(*p).conf.headers).used {
        let ds_0: *const data_string = *((*(*p).conf.headers).data).offset(k as isize)
            as *mut data_string;
        let hdr: *const buffer = &(*ds_0).value;
        forwarded = http_header_request_get(
            r,
            (*ds_0).ext as http_header_e,
            (*hdr).ptr,
            buffer_clen(hdr),
        );
        if !forwarded.is_null() {
            is_forwarded_header = ((*ds_0).ext == HTTP_HEADER_FORWARDED as libc::c_int)
                as libc::c_int;
            break;
        } else {
            k = k.wrapping_add(1);
        }
    }
    if !forwarded.is_null() && is_connection_trusted((*r).con, p) != 0 {
        return (if is_forwarded_header != 0 {
            mod_extforward_Forwarded(r, p, forwarded) as libc::c_uint
        } else {
            mod_extforward_X_Forwarded_For(r, p, forwarded) as libc::c_uint
        }) as handler_t
    } else {
        if (*r).conf.log_request_handling != 0 {
            log_error(
                (*r).conf.errh,
                b"src/mod_extforward.c\0" as *const u8 as *const libc::c_char,
                1095 as libc::c_int as libc::c_uint,
                b"no forward header found or remote address %s is NOT a trusted proxy, skipping\0"
                    as *const u8 as *const libc::c_char,
                (*(*r).con).dst_addr_buf.ptr,
            );
        }
        return HANDLER_GO_ON;
    };
}
unsafe extern "C" fn mod_extforward_handle_request_env(
    mut r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    let mut hctx: *mut handler_ctx = *((*(*r).con).plugin_ctx).offset((*p).id as isize)
        as *mut handler_ctx;
    if hctx.is_null() || ((*hctx).env).is_null() {
        return HANDLER_GO_ON;
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*(*hctx).env).used {
        let mut ds: *mut data_string = *((*(*hctx).env).data).offset(i as isize)
            as *mut data_string;
        http_header_env_set(
            r,
            (*ds).key.ptr,
            buffer_clen(&mut (*ds).key),
            (*ds).value.ptr,
            buffer_clen(&mut (*ds).value),
        );
        i = i.wrapping_add(1);
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_extforward_restore(
    mut r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    if (*r).http_version as libc::c_int > HTTP_VERSION_1_1 as libc::c_int {
        return HANDLER_GO_ON;
    }
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    let con: *mut connection = (*r).con;
    let mut hctx: *mut handler_ctx = *((*con).plugin_ctx).offset((*p).id as isize)
        as *mut handler_ctx;
    if hctx.is_null() {
        return HANDLER_GO_ON;
    }
    if buffer_is_unset(&mut (*hctx).saved_remote_addr_buf) == 0 {
        (*con).dst_addr = (*hctx).saved_remote_addr;
        buffer_move(&mut (*con).dst_addr_buf, &mut (*hctx).saved_remote_addr_buf);
        config_cond_cache_reset_item(r, COMP_HTTP_REMOTE_IP);
    }
    if ((*hctx).env).is_null() {
        handler_ctx_free(hctx);
        let ref mut fresh5 = *((*con).plugin_ctx).offset((*p).id as isize);
        *fresh5 = 0 as *mut libc::c_void;
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_extforward_handle_con_close(
    mut con: *mut connection,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    let mut hctx: *mut handler_ctx = *((*con).plugin_ctx).offset((*p).id as isize)
        as *mut handler_ctx;
    if !hctx.is_null() {
        if ((*hctx).saved_network_read).is_some() {
            (*con).network_read = (*hctx).saved_network_read;
        }
        if buffer_is_unset(&mut (*hctx).saved_remote_addr_buf) == 0 {
            (*con).dst_addr = (*hctx).saved_remote_addr;
            buffer_move(&mut (*con).dst_addr_buf, &mut (*hctx).saved_remote_addr_buf);
        }
        if !((*hctx).env).is_null() {
            array_free((*hctx).env);
        }
        handler_ctx_free(hctx);
        let ref mut fresh6 = *((*con).plugin_ctx).offset((*p).id as isize);
        *fresh6 = 0 as *mut libc::c_void;
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_extforward_handle_con_accept(
    mut con: *mut connection,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let r: *mut request_st = &mut (*con).request;
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    mod_extforward_patch_config(r, p);
    if (*p).conf.hap_PROXY == 0 {
        return HANDLER_GO_ON;
    }
    if ((*p).conf.forwarder).is_null() {
        return HANDLER_GO_ON;
    }
    if is_connection_trusted(con, p) != 0 {
        let mut hctx: *mut handler_ctx = handler_ctx_init();
        let ref mut fresh7 = *((*con).plugin_ctx).offset((*p).id as isize);
        *fresh7 = hctx as *mut libc::c_void;
        (*hctx).saved_network_read = (*con).network_read;
        (*con)
            .network_read = Some(
            mod_extforward_network_read
                as unsafe extern "C" fn(
                    *mut connection,
                    *mut chunkqueue,
                    off_t,
                ) -> libc::c_int,
        );
    } else if (*r).conf.log_request_handling != 0 {
        log_error(
            (*r).conf.errh,
            b"src/mod_extforward.c\0" as *const u8 as *const libc::c_char,
            1188 as libc::c_int as libc::c_uint,
            b"remote address %s is NOT a trusted proxy, skipping\0" as *const u8
                as *const libc::c_char,
            (*con).dst_addr_buf.ptr,
        );
    }
    return HANDLER_GO_ON;
}
#[no_mangle]
pub unsafe extern "C" fn mod_extforward_plugin_init(mut p: *mut plugin) -> libc::c_int {
    (*p).version = 0x10440 as libc::c_int as size_t;
    (*p).name = b"extforward\0" as *const u8 as *const libc::c_char;
    (*p)
        .init = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
    >(Some(mod_extforward_init as unsafe extern "C" fn() -> *mut libc::c_void));
    (*p)
        .handle_connection_accept = Some(
        mod_extforward_handle_con_accept
            as unsafe extern "C" fn(*mut connection, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_uri_raw = Some(
        mod_extforward_uri_handler
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_request_env = Some(
        mod_extforward_handle_request_env
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_request_reset = Some(
        mod_extforward_restore
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_connection_close = Some(
        mod_extforward_handle_con_close
            as unsafe extern "C" fn(*mut connection, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .set_defaults = Some(
        mod_extforward_set_defaults
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .cleanup = Some(
        mod_extforward_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn hap_PROXY_recv(
    fd: libc::c_int,
    hdr: *mut hap_PROXY_hdr,
    family: libc::c_int,
    so_type: libc::c_int,
) -> libc::c_int {
    static mut v2sig: [libc::c_char; 12] = unsafe {
        *::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"\r\n\r\n\0\r\nQUIT\n")
    };
    let mut ret: ssize_t = 0;
    let mut sz: size_t = 0;
    let mut ver: libc::c_int = 0;
    loop {
        ret = recv(
            fd,
            hdr as *mut libc::c_void,
            ::std::mem::size_of::<hap_PROXY_hdr>() as libc::c_ulong,
            MSG_PEEK as libc::c_int | MSG_DONTWAIT as libc::c_int
                | MSG_NOSIGNAL as libc::c_int,
        );
        if !(-(1 as libc::c_int) as libc::c_long == ret
            && *__errno_location() == 4 as libc::c_int)
        {
            break;
        }
    }
    if -(1 as libc::c_int) as libc::c_long == ret {
        return if *__errno_location() == 11 as libc::c_int {
            0 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
    }
    if ret >= 16 as libc::c_int as libc::c_long
        && 0 as libc::c_int
            == memcmp(
                &mut (*hdr).v2 as *mut C2RustUnnamed_7 as *const libc::c_void,
                v2sig.as_ptr() as *const libc::c_void,
                12 as libc::c_int as libc::c_ulong,
            )
        && (*hdr).v2.ver_cmd as libc::c_int & 0xf0 as libc::c_int == 0x20 as libc::c_int
    {
        ver = 2 as libc::c_int;
        sz = (16 as libc::c_int as libc::c_ulong)
            .wrapping_add(__bswap_16((*hdr).v2.len) as size_t);
        if (ret as size_t) < sz {
            return -(2 as libc::c_int);
        }
        match (*hdr).v2.ver_cmd as libc::c_int & 0xf as libc::c_int {
            1 | 0 => {}
            _ => return -(2 as libc::c_int),
        }
    } else if ret >= 8 as libc::c_int as libc::c_long
            && 0 as libc::c_int
                == memcmp(
                    ((*hdr).v1.line).as_mut_ptr() as *const libc::c_void,
                    b"PROXY\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    5 as libc::c_int as libc::c_ulong,
                )
        {
        let mut end: *const libc::c_char = memchr(
            ((*hdr).v1.line).as_mut_ptr() as *const libc::c_void,
            '\r' as i32,
            (ret - 1 as libc::c_int as libc::c_long) as libc::c_ulong,
        ) as *const libc::c_char;
        if end.is_null()
            || *end.offset(1 as libc::c_int as isize) as libc::c_int != '\n' as i32
        {
            return -(2 as libc::c_int);
        }
        ver = 1 as libc::c_int;
        sz = end
            .offset(2 as libc::c_int as isize)
            .offset_from(((*hdr).v1.line).as_mut_ptr()) as libc::c_long as size_t;
    } else {
        return -(2 as libc::c_int)
    }
    let mut current_block_24: u64;
    loop {
        if (family == 2 as libc::c_int || family == 10 as libc::c_int)
            && so_type == SOCK_STREAM as libc::c_int
        {
            ret = recv(
                fd,
                hdr as *mut libc::c_void,
                sz,
                MSG_TRUNC as libc::c_int | MSG_DONTWAIT as libc::c_int
                    | MSG_NOSIGNAL as libc::c_int,
            );
            if ret >= 0 as libc::c_int as libc::c_long
                || *__errno_location() != 22 as libc::c_int
            {
                current_block_24 = 15768484401365413375;
            } else {
                current_block_24 = 16924917904204750491;
            }
        } else {
            current_block_24 = 16924917904204750491;
        }
        match current_block_24 {
            16924917904204750491 => {
                ret = recv(
                    fd,
                    hdr as *mut libc::c_void,
                    sz,
                    MSG_DONTWAIT as libc::c_int | MSG_NOSIGNAL as libc::c_int,
                );
            }
            _ => {}
        }
        if !(-(1 as libc::c_int) as libc::c_long == ret
            && *__errno_location() == 4 as libc::c_int)
        {
            break;
        }
    }
    if ret < 0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int);
    }
    if ret != sz as ssize_t {
        *__errno_location() = 5 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if 1 as libc::c_int == ver {
        (*hdr)
            .v1
            .line[sz.wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as usize] = '\u{0}' as i32 as libc::c_char;
    }
    return ver;
}
unsafe extern "C" fn mod_extforward_str_to_port(s: *const libc::c_char) -> libc::c_int {
    let mut port: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        if light_isdigit(*s.offset(i as isize) as libc::c_int) == 0 {
            return -(1 as libc::c_int);
        }
        port += *s.offset(i as isize) as libc::c_int - '0' as i32;
        if *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '\u{0}' as i32 {
            return port;
        }
        i += 1;
        port *= 10 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mod_extforward_hap_PROXY_v1(
    con: *mut connection,
    hdr: *mut hap_PROXY_hdr,
) -> libc::c_int {
    let mut s: *mut libc::c_char = ((*hdr).v1.line)
        .as_mut_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut src_addr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dst_addr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut src_port: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dst_port: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut family: libc::c_int = 0;
    let mut src_lport: libc::c_int = 0;
    let mut dst_lport: libc::c_int = 0;
    if *s as libc::c_int != ' ' as i32 {
        return -(1 as libc::c_int);
    }
    s = s.offset(1);
    if *s.offset(0 as libc::c_int as isize) as libc::c_int == 'T' as i32
        && *s.offset(1 as libc::c_int as isize) as libc::c_int == 'C' as i32
        && *s.offset(2 as libc::c_int as isize) as libc::c_int == 'P' as i32
        && *s.offset(4 as libc::c_int as isize) as libc::c_int == ' ' as i32
    {
        if *s.offset(3 as libc::c_int as isize) as libc::c_int == '4' as i32 {
            family = 2 as libc::c_int;
        } else if *s.offset(3 as libc::c_int as isize) as libc::c_int == '6' as i32 {
            family = 10 as libc::c_int;
        } else {
            return -(1 as libc::c_int)
        }
        s = s.offset(5 as libc::c_int as isize);
    } else if 0 as libc::c_int
            == memcmp(
                s as *const libc::c_void,
                b"UNKNOWN\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            )
            && (*s.offset(7 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
                || *s.offset(7 as libc::c_int as isize) as libc::c_int == ' ' as i32)
        {
        return 0 as libc::c_int
    } else {
        return -(1 as libc::c_int)
    }
    src_addr = s;
    dst_addr = strchr(src_addr, ' ' as i32);
    if dst_addr.is_null() {
        return -(1 as libc::c_int);
    }
    let fresh8 = dst_addr;
    dst_addr = dst_addr.offset(1);
    *fresh8 = '\u{0}' as i32 as libc::c_char;
    src_port = strchr(dst_addr, ' ' as i32);
    if src_port.is_null() {
        return -(1 as libc::c_int);
    }
    let fresh9 = src_port;
    src_port = src_port.offset(1);
    *fresh9 = '\u{0}' as i32 as libc::c_char;
    dst_port = strchr(src_port, ' ' as i32);
    if dst_port.is_null() {
        return -(1 as libc::c_int);
    }
    let fresh10 = dst_port;
    dst_port = dst_port.offset(1);
    *fresh10 = '\u{0}' as i32 as libc::c_char;
    src_lport = mod_extforward_str_to_port(src_port);
    if src_lport <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    dst_lport = mod_extforward_str_to_port(dst_port);
    if dst_lport <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if 1 as libc::c_int
        != sock_addr_inet_pton(
            &mut (*con).dst_addr,
            src_addr,
            family,
            src_lport as libc::c_ushort,
        )
    {
        return -(1 as libc::c_int);
    }
    sock_addr_inet_ntop_copy_buffer(&mut (*con).dst_addr_buf, &mut (*con).dst_addr);
    return 0 as libc::c_int;
}
unsafe extern "C" fn mod_extforward_hap_PROXY_v2(
    con: *mut connection,
    hdr: *mut hap_PROXY_hdr,
) -> libc::c_int {
    let mut tlv: *mut pp2_tlv = 0 as *mut pp2_tlv;
    let mut sz: uint32_t = __bswap_16((*hdr).v2.len) as uint32_t;
    let mut len: uint32_t = 0 as libc::c_int as uint32_t;
    match (*hdr).v2.ver_cmd as libc::c_int & 0xf as libc::c_int {
        1 => {}
        0 => return 0 as libc::c_int,
        _ => return -(1 as libc::c_int),
    }
    match (*hdr).v2.fam as libc::c_int {
        17 => {
            sock_addr_assign(
                &mut (*con).dst_addr,
                2 as libc::c_int,
                (*hdr).v2.addr.ip4.src_port,
                &mut (*hdr).v2.addr.ip4.src_addr as *mut uint32_t as *const libc::c_void,
            );
            sock_addr_inet_ntop_copy_buffer(
                &mut (*con).dst_addr_buf,
                &mut (*con).dst_addr,
            );
            len = ::std::mem::size_of::<C2RustUnnamed_11>() as libc::c_ulong as uint32_t;
        }
        33 => {
            sock_addr_assign(
                &mut (*con).dst_addr,
                10 as libc::c_int,
                (*hdr).v2.addr.ip6.src_port,
                &mut (*hdr).v2.addr.ip6.src_addr as *mut [uint8_t; 16]
                    as *const libc::c_void,
            );
            sock_addr_inet_ntop_copy_buffer(
                &mut (*con).dst_addr_buf,
                &mut (*con).dst_addr,
            );
            len = ::std::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong as uint32_t;
        }
        49 => {
            let mut src_addr: *mut libc::c_char = ((*hdr).v2.addr.unx.src_addr)
                .as_mut_ptr() as *mut libc::c_char;
            let mut z: *mut libc::c_char = memchr(
                src_addr as *const libc::c_void,
                '\u{0}' as i32,
                108 as libc::c_int as libc::c_ulong,
            ) as *mut libc::c_char;
            if z.is_null() {
                return -(1 as libc::c_int);
            }
            len = (z.offset_from(src_addr) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as uint32_t;
            sock_addr_assign(
                &mut (*con).dst_addr,
                1 as libc::c_int,
                0 as libc::c_int as libc::c_ushort,
                src_addr as *const libc::c_void,
            );
            buffer_copy_string_len(&mut (*con).dst_addr_buf, src_addr, len as size_t);
            len = ::std::mem::size_of::<C2RustUnnamed_9>() as libc::c_ulong as uint32_t;
        }
        _ => return 0 as libc::c_int,
    }
    if (3 as libc::c_int as libc::c_uint).wrapping_add(len) > sz {
        return 0 as libc::c_int;
    }
    let hctx: *mut handler_ctx = *((*con).plugin_ctx)
        .offset((*mod_extforward_plugin_data_singleton).id as isize) as *mut handler_ctx;
    tlv = (hdr as *mut libc::c_char).offset(16 as libc::c_int as isize) as *mut pp2_tlv;
    sz = (sz as libc::c_uint).wrapping_sub(len) as uint32_t as uint32_t;
    len = (len as libc::c_uint).wrapping_sub(3 as libc::c_int as libc::c_uint)
        as uint32_t as uint32_t;
    while sz >= 3 as libc::c_int as libc::c_uint {
        tlv = (tlv as *mut libc::c_char)
            .offset(3 as libc::c_int as isize)
            .offset(len as isize) as *mut pp2_tlv;
        len = ((*tlv).length_hi as uint32_t) << 8 as libc::c_int
            | (*tlv).length_lo as libc::c_uint;
        if (3 as libc::c_int as libc::c_uint).wrapping_add(len) > sz {
            break;
        }
        let mut k: *const libc::c_char = 0 as *const libc::c_char;
        let mut klen: uint32_t = 0;
        match (*tlv).type_0 as libc::c_int {
            32 => {
                if !(len < 5 as libc::c_int as libc::c_uint) {
                    static mut zero: uint32_t = 0 as libc::c_int as uint32_t;
                    let mut tlv_ssl: *mut pp2_tlv_ssl = (tlv as *mut libc::c_char)
                        .offset(3 as libc::c_int as isize) as *mut libc::c_void
                        as *mut pp2_tlv_ssl;
                    let mut subtlv: *mut pp2_tlv = tlv;
                    if (*tlv_ssl).client as libc::c_int & 0x1 as libc::c_int != 0 {
                        (*con).proto_default_port = 443 as libc::c_int as uint16_t;
                    }
                    if (*tlv_ssl).client as libc::c_int
                        & (0x2 as libc::c_int | 0x4 as libc::c_int) != 0
                        && 0 as libc::c_int
                            == memcmp(
                                &mut (*tlv_ssl).verify as *mut uint32_t
                                    as *const libc::c_void,
                                &zero as *const uint32_t as *const libc::c_void,
                                4 as libc::c_int as libc::c_ulong,
                            )
                    {
                        (*hctx).ssl_client_verify = 1 as libc::c_int;
                    }
                    if !(len < (5 as libc::c_int + 3 as libc::c_int) as libc::c_uint) {
                        if ((*hctx).env).is_null() {
                            (*hctx).env = array_init(8 as libc::c_int as uint32_t);
                        }
                        let mut current_block_44: u64;
                        let mut subsz: uint32_t = len
                            .wrapping_sub(5 as libc::c_int as libc::c_uint);
                        let mut n: uint32_t = 5 as libc::c_int as uint32_t;
                        while subsz >= 3 as libc::c_int as libc::c_uint {
                            subtlv = (subtlv as *mut libc::c_char)
                                .offset(3 as libc::c_int as isize)
                                .offset(n as isize) as *mut pp2_tlv;
                            n = ((*subtlv).length_hi as uint32_t) << 8 as libc::c_int
                                | (*subtlv).length_lo as libc::c_uint;
                            if (3 as libc::c_int as libc::c_uint).wrapping_add(n) > subsz
                            {
                                break;
                            }
                            match (*subtlv).type_0 as libc::c_int {
                                33 => {
                                    k = b"SSL_PROTOCOL\0" as *const u8 as *const libc::c_char;
                                    klen = (::std::mem::size_of::<[libc::c_char; 13]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as uint32_t;
                                    current_block_44 = 17075014677070940716;
                                }
                                34 => {
                                    k = b"SSL_CLIENT_S_DN_CN\0" as *const u8
                                        as *const libc::c_char;
                                    klen = (::std::mem::size_of::<[libc::c_char; 19]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as uint32_t;
                                    current_block_44 = 17075014677070940716;
                                }
                                35 => {
                                    k = b"SSL_CIPHER\0" as *const u8 as *const libc::c_char;
                                    klen = (::std::mem::size_of::<[libc::c_char; 11]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as uint32_t;
                                    current_block_44 = 17075014677070940716;
                                }
                                36 => {
                                    k = b"SSL_SERVER_A_SIG\0" as *const u8
                                        as *const libc::c_char;
                                    klen = (::std::mem::size_of::<[libc::c_char; 17]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as uint32_t;
                                    current_block_44 = 17075014677070940716;
                                }
                                37 => {
                                    k = b"SSL_SERVER_A_KEY\0" as *const u8
                                        as *const libc::c_char;
                                    klen = (::std::mem::size_of::<[libc::c_char; 17]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as uint32_t;
                                    current_block_44 = 17075014677070940716;
                                }
                                _ => {
                                    current_block_44 = 15004371738079956865;
                                }
                            }
                            match current_block_44 {
                                17075014677070940716 => {
                                    array_set_key_value(
                                        (*hctx).env,
                                        k,
                                        klen,
                                        (subtlv as *mut libc::c_char)
                                            .offset(3 as libc::c_int as isize),
                                        n,
                                    );
                                }
                                _ => {}
                            }
                            subsz = (subsz as libc::c_uint)
                                .wrapping_sub(
                                    (3 as libc::c_int as libc::c_uint).wrapping_add(n),
                                ) as uint32_t as uint32_t;
                        }
                    }
                }
            }
            5 => {
                k = b"PP2_UNIQUE_ID\0" as *const u8 as *const libc::c_char;
                klen = (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t;
                if ((*hctx).env).is_null() {
                    (*hctx).env = array_init(8 as libc::c_int as uint32_t);
                }
                array_set_key_value(
                    (*hctx).env,
                    k,
                    klen,
                    (tlv as *mut libc::c_char).offset(3 as libc::c_int as isize),
                    len,
                );
            }
            _ => {}
        }
        sz = (sz as libc::c_uint)
            .wrapping_sub((3 as libc::c_int as libc::c_uint).wrapping_add(len))
            as uint32_t as uint32_t;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mod_extforward_network_read(
    mut con: *mut connection,
    mut cq: *mut chunkqueue,
    mut max_bytes: off_t,
) -> libc::c_int {
    let mut hdr: hap_PROXY_hdr = hap_PROXY_hdr {
        v1: C2RustUnnamed_12 { line: [0; 108] },
    };
    let mut errh: *mut log_error_st = 0 as *mut log_error_st;
    let family: libc::c_int = sock_addr_get_family(&mut (*con).dst_addr);
    let mut rc: libc::c_int = hap_PROXY_recv(
        (*con).fd,
        &mut hdr,
        family,
        SOCK_STREAM as libc::c_int,
    );
    let mut current_block_9: u64;
    match rc {
        2 => {
            rc = mod_extforward_hap_PROXY_v2(con, &mut hdr);
            current_block_9 = 17860125682698302841;
        }
        1 => {
            rc = mod_extforward_hap_PROXY_v1(con, &mut hdr);
            current_block_9 = 17860125682698302841;
        }
        0 => return 0 as libc::c_int,
        -1 => {
            errh = (*(*con).srv).errh;
            log_perror(
                errh,
                b"src/mod_extforward.c\0" as *const u8 as *const libc::c_char,
                1686 as libc::c_int as libc::c_uint,
                b"hap-PROXY recv()\0" as *const u8 as *const libc::c_char,
            );
            rc = -(1 as libc::c_int);
            current_block_9 = 17860125682698302841;
        }
        -2 => {
            errh = (*(*con).srv).errh;
            log_error(
                errh,
                b"src/mod_extforward.c\0" as *const u8 as *const libc::c_char,
                1689 as libc::c_int as libc::c_uint,
                b"hap-PROXY proto received invalid/unsupported request\0" as *const u8
                    as *const libc::c_char,
            );
            current_block_9 = 5464269622659670818;
        }
        _ => {
            current_block_9 = 5464269622659670818;
        }
    }
    match current_block_9 {
        5464269622659670818 => {
            rc = -(1 as libc::c_int);
        }
        _ => {}
    }
    let mut hctx: *mut handler_ctx = *((*con).plugin_ctx)
        .offset((*mod_extforward_plugin_data_singleton).id as isize) as *mut handler_ctx;
    (*con).network_read = (*hctx).saved_network_read;
    (*hctx).saved_network_read = None;
    return if 0 as libc::c_int == rc {
        ((*con).network_read).expect("non-null function pointer")(con, cq, max_bytes)
    } else {
        rc
    };
}
unsafe extern "C" fn run_static_initializers() {
    cpk = [
        {
            let mut init = config_plugin_keys_t {
                k: b"extforward.forwarder\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVSTRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"extforward.headers\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_VLIST as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"extforward.params\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVANY as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"extforward.hap-PROXY\0" as *const u8 as *const libc::c_char,
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
                k: b"extforward.hap-PROXY-ssl-client-verify\0" as *const u8
                    as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 39]>() as libc::c_ulong
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
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
