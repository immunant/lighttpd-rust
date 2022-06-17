use ::libc;
extern "C" {
    pub type fdnode_st;
    pub type stat_cache_entry;
    pub type cond_match_t;
    pub type cond_cache_t;
    pub type plugin;
    pub type h2con;
    pub type fdevents;
    fn chunkqueue_mark_written(cq: *mut chunkqueue, len: off_t);
    fn chunkqueue_open_file_chunk(
        cq: *mut chunkqueue,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn chunkqueue_remove_finished_chunks(cq: *mut chunkqueue);
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    fn __errno_location() -> *mut libc::c_int;
    fn pread(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __nbytes: size_t,
        __offset: __off64_t,
    ) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn writev(__fd: libc::c_int, __iovec: *const iovec, __count: libc::c_int) -> ssize_t;
    fn sendfile(
        __out_fd: libc::c_int,
        __in_fd: libc::c_int,
        __offset: *mut __off64_t,
        __count: size_t,
    ) -> ssize_t;
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
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
pub const NETWORK_BACKEND_WRITE: network_backend_t = 1;
pub const NETWORK_BACKEND_WRITEV: network_backend_t = 2;
pub const NETWORK_BACKEND_SENDFILE: network_backend_t = 3;
pub type network_backend_t = libc::c_uint;
pub const NETWORK_BACKEND_UNSET: network_backend_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nb_map {
    pub nb: network_backend_t,
    pub name: *const libc::c_char,
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
unsafe extern "C" fn network_write_error(
    mut fd: libc::c_int,
    mut errh: *mut log_error_st,
) -> libc::c_int {
    match *__errno_location() {
        11 | 4 => return -(3 as libc::c_int),
        32 | 104 => return -(2 as libc::c_int),
        _ => {
            log_perror(
                errh,
                b"src/network_write.c\0" as *const u8 as *const libc::c_char,
                96 as libc::c_int as libc::c_uint,
                b"write failed: %d\0" as *const u8 as *const libc::c_char,
                fd,
            );
            return -(1 as libc::c_int);
        }
    };
}
#[cold]
unsafe extern "C" fn network_remove_finished_chunks(
    cq: *mut chunkqueue,
    len: off_t,
) -> libc::c_int {
    if !(len >= 0 as libc::c_int as libc::c_long) {
        ck_assert_failed(
            b"src/network_write.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int as libc::c_uint,
            b"len >= 0\0" as *const u8 as *const libc::c_char,
        );
    }
    chunkqueue_remove_finished_chunks(cq);
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn network_write_data_len(
    mut fd: libc::c_int,
    mut data: *const libc::c_char,
    mut len: off_t,
) -> ssize_t {
    return write(fd, data as *const libc::c_void, len as size_t);
}
unsafe extern "C" fn network_write_accounting(
    fd: libc::c_int,
    cq: *mut chunkqueue,
    p_max_bytes: *mut off_t,
    errh: *mut log_error_st,
    wr: ssize_t,
    toSend: off_t,
) -> libc::c_int {
    if wr >= 0 as libc::c_int as libc::c_long {
        *p_max_bytes -= wr;
        let rc: libc::c_int = if wr == toSend
            && *p_max_bytes > 0 as libc::c_int as libc::c_long
        {
            0 as libc::c_int
        } else {
            -(3 as libc::c_int)
        };
        chunkqueue_mark_written(cq, wr);
        return rc;
    } else {
        return network_write_error(fd, errh)
    };
}
unsafe extern "C" fn network_write_file_chunk_no_mmap(
    fd: libc::c_int,
    cq: *mut chunkqueue,
    p_max_bytes: *mut off_t,
    errh: *mut log_error_st,
) -> libc::c_int {
    let c: *mut chunk = (*cq).first;
    let mut offset: off_t = 0;
    let mut toSend: off_t = 0;
    let mut buf: [libc::c_char; 16384] = [0; 16384];
    offset = (*c).offset;
    toSend = (*c).file.length - (*c).offset;
    if toSend > *p_max_bytes {
        toSend = *p_max_bytes;
    }
    if toSend <= 0 as libc::c_int as libc::c_long {
        return network_remove_finished_chunks(cq, toSend);
    }
    if (*c).file.fd < 0 as libc::c_int
        && 0 as libc::c_int != chunkqueue_open_file_chunk(cq, errh)
    {
        return -(1 as libc::c_int);
    }
    if toSend > ::std::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong as off_t
    {
        toSend = ::std::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong
            as off_t;
    }
    toSend = pread(
        (*c).file.fd,
        buf.as_mut_ptr() as *mut libc::c_void,
        toSend as size_t,
        offset,
    );
    if toSend <= 0 as libc::c_int as libc::c_long {
        log_perror(
            errh,
            b"src/network_write.c\0" as *const u8 as *const libc::c_char,
            176 as libc::c_int as libc::c_uint,
            b"read\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut wr: ssize_t = network_write_data_len(fd, buf.as_mut_ptr(), toSend);
    return network_write_accounting(fd, cq, p_max_bytes, errh, wr, toSend);
}
unsafe extern "C" fn network_writev_mem_chunks(
    fd: libc::c_int,
    cq: *mut chunkqueue,
    p_max_bytes: *mut off_t,
    errh: *mut log_error_st,
) -> libc::c_int {
    let mut num_chunks: size_t = 0 as libc::c_int as size_t;
    let mut toSend: off_t = 0 as libc::c_int as off_t;
    let mut chunks: [iovec; 32] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 32];
    let mut c: *const chunk = (*cq).first;
    while !c.is_null()
        && MEM_CHUNK as libc::c_int as libc::c_uint == (*c).type_0 as libc::c_uint
    {
        let c_len: off_t = buffer_clen((*c).mem) as off_t - (*c).offset;
        if c_len > 0 as libc::c_int as libc::c_long {
            toSend += c_len;
            chunks[num_chunks as usize]
                .iov_base = ((*(*c).mem).ptr).offset((*c).offset as isize)
                as *mut libc::c_void;
            chunks[num_chunks as usize].iov_len = c_len as size_t;
            num_chunks = num_chunks.wrapping_add(1);
            if num_chunks == 32 as libc::c_int as libc::c_ulong || toSend >= *p_max_bytes
            {
                break;
            }
        } else if c_len < 0 as libc::c_int as libc::c_long {
            return network_remove_finished_chunks(cq, c_len)
        }
        c = (*c).next;
    }
    if 0 as libc::c_int as libc::c_ulong == num_chunks {
        return network_remove_finished_chunks(cq, 0 as libc::c_int as off_t);
    }
    let mut wr: ssize_t = writev(fd, chunks.as_mut_ptr(), num_chunks as libc::c_int);
    return network_write_accounting(fd, cq, p_max_bytes, errh, wr, toSend);
}
unsafe extern "C" fn network_write_file_chunk_sendfile(
    fd: libc::c_int,
    cq: *mut chunkqueue,
    p_max_bytes: *mut off_t,
    errh: *mut log_error_st,
) -> libc::c_int {
    let c: *mut chunk = (*cq).first;
    let mut wr: ssize_t = 0;
    let mut offset: off_t = 0;
    let mut toSend: off_t = 0;
    let mut written: off_t = 0 as libc::c_int as off_t;
    offset = (*c).offset;
    toSend = (*c).file.length - (*c).offset;
    if toSend > *p_max_bytes {
        toSend = *p_max_bytes;
    }
    if toSend <= 0 as libc::c_int as libc::c_long {
        return network_remove_finished_chunks(cq, toSend);
    }
    if (*c).file.fd < 0 as libc::c_int
        && 0 as libc::c_int != chunkqueue_open_file_chunk(cq, errh)
    {
        return -(1 as libc::c_int);
    }
    wr = sendfile(fd, (*c).file.fd, &mut offset, toSend as size_t);
    if wr > 0 as libc::c_int as libc::c_long {
        written = wr;
    }
    if -(1 as libc::c_int) as libc::c_long == wr {
        match *__errno_location() {
            11 | 4 => {}
            32 | 104 | 107 => return -(2 as libc::c_int),
            22 | 38 | 95 | 94 | 97 => {
                return network_write_file_chunk_no_mmap(fd, cq, p_max_bytes, errh);
            }
            _ => {
                log_perror(
                    errh,
                    b"src/network_write.c\0" as *const u8 as *const libc::c_char,
                    479 as libc::c_int as libc::c_uint,
                    b"sendfile(): fd: %d\0" as *const u8 as *const libc::c_char,
                    fd,
                );
                return -(1 as libc::c_int);
            }
        }
    }
    if written > 0 as libc::c_int as libc::c_long {
        chunkqueue_mark_written(cq, written);
        *p_max_bytes -= written;
        if (*p_max_bytes <= 0 as libc::c_int as libc::c_long) as libc::c_int
            as libc::c_long != 0
        {
            return -(3 as libc::c_int);
        }
    } else if 0 as libc::c_int as libc::c_long == wr {
        log_error(
            errh,
            b"src/network_write.c\0" as *const u8 as *const libc::c_char,
            490 as libc::c_int as libc::c_uint,
            b"sendfile(): fd: %d file truncated\0" as *const u8 as *const libc::c_char,
            fd,
        );
        return -(1 as libc::c_int);
    }
    return if wr >= 0 as libc::c_int as libc::c_long && written == toSend {
        0 as libc::c_int
    } else {
        -(3 as libc::c_int)
    };
}
unsafe extern "C" fn network_write_chunkqueue_writev(
    fd: libc::c_int,
    cq: *mut chunkqueue,
    mut max_bytes: off_t,
    errh: *mut log_error_st,
) -> libc::c_int {
    while !((*cq).first).is_null() {
        let mut rc: libc::c_int = -(1 as libc::c_int);
        match (*(*cq).first).type_0 as libc::c_uint {
            0 => {
                rc = network_writev_mem_chunks(fd, cq, &mut max_bytes, errh);
            }
            1 => {
                rc = network_write_file_chunk_no_mmap(fd, cq, &mut max_bytes, errh);
            }
            _ => {}
        }
        if (0 as libc::c_int != rc) as libc::c_int as libc::c_long != 0 {
            return if -(3 as libc::c_int) == rc { 0 as libc::c_int } else { rc };
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn network_write_chunkqueue_sendfile(
    fd: libc::c_int,
    cq: *mut chunkqueue,
    mut max_bytes: off_t,
    errh: *mut log_error_st,
) -> libc::c_int {
    while !((*cq).first).is_null() {
        let mut rc: libc::c_int = -(1 as libc::c_int);
        match (*(*cq).first).type_0 as libc::c_uint {
            0 => {
                rc = network_writev_mem_chunks(fd, cq, &mut max_bytes, errh);
            }
            1 => {
                rc = network_write_file_chunk_sendfile(fd, cq, &mut max_bytes, errh);
            }
            _ => {}
        }
        if (0 as libc::c_int != rc) as libc::c_int as libc::c_long != 0 {
            return if -(3 as libc::c_int) == rc { 0 as libc::c_int } else { rc };
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn network_write_init(mut srv: *mut server) -> libc::c_int {
    let mut backend: network_backend_t = NETWORK_BACKEND_UNSET;
    let mut network_backends: [nb_map; 7] = [
        {
            let mut init = nb_map {
                nb: NETWORK_BACKEND_SENDFILE,
                name: b"sendfile\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = nb_map {
                nb: NETWORK_BACKEND_SENDFILE,
                name: b"linux-sendfile\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = nb_map {
                nb: NETWORK_BACKEND_SENDFILE,
                name: b"freebsd-sendfile\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = nb_map {
                nb: NETWORK_BACKEND_SENDFILE,
                name: b"solaris-sendfilev\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = nb_map {
                nb: NETWORK_BACKEND_WRITEV,
                name: b"writev\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = nb_map {
                nb: NETWORK_BACKEND_WRITE,
                name: b"write\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = nb_map {
                nb: NETWORK_BACKEND_UNSET,
                name: 0 as *const libc::c_char,
            };
            init
        },
    ];
    backend = network_backends[0 as libc::c_int as usize].nb;
    if !((*srv).srvconf.network_backend).is_null() {
        let mut name: *const libc::c_char = 0 as *const libc::c_char;
        let mut confname: *const libc::c_char = (*(*srv).srvconf.network_backend).ptr;
        let mut i: size_t = 0 as libc::c_int as size_t;
        loop {
            name = network_backends[i as usize].name;
            if name.is_null() {
                break;
            }
            if 0 as libc::c_int == strcmp(confname, name) {
                backend = network_backends[i as usize].nb;
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        if name.is_null() {
            log_error(
                (*srv).errh,
                b"src/network_write.c\0" as *const u8 as *const libc::c_char,
                604 as libc::c_int as libc::c_uint,
                b"server.network-backend has an unknown value: %s\0" as *const u8
                    as *const libc::c_char,
                confname,
            );
            return -(1 as libc::c_int);
        }
    }
    match backend as libc::c_uint {
        3 => {
            (*srv)
                .network_backend_write = Some(
                network_write_chunkqueue_sendfile
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut chunkqueue,
                        off_t,
                        *mut log_error_st,
                    ) -> libc::c_int,
            );
        }
        2 | 1 => {
            (*srv)
                .network_backend_write = Some(
                network_write_chunkqueue_writev
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut chunkqueue,
                        off_t,
                        *mut log_error_st,
                    ) -> libc::c_int,
            );
        }
        _ => return -(1 as libc::c_int),
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn network_write_show_handlers() -> *const libc::c_char {
    return b"\nNetwork handler:\n\n\t+ linux-sendfile\n\t- freebsd-sendfile\n\t- darwin-sendfile\n\t- solaris-sendfilev\n\t+ writev\n\t+ write\n\t- mmap support\n\0"
        as *const u8 as *const libc::c_char;
}
