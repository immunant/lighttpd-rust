use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type stat_cache_entry;
    pub type pcre2_real_match_data_8;
    pub type lshpack_double_enc_head;
    pub type lshpack_enc_table_entry;
    pub type fdevents;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn lshpack_dec_decode(
        dec: *mut lshpack_dec,
        src: *mut *const libc::c_uchar,
        src_end: *const libc::c_uchar,
        output: *mut lsxpack_header,
    ) -> libc::c_int;
    fn lshpack_dec_cleanup(_: *mut lshpack_dec);
    fn lshpack_dec_init(_: *mut lshpack_dec);
    fn lshpack_enc_use_hist(_: *mut lshpack_enc, on: libc::c_int) -> libc::c_int;
    fn lshpack_enc_set_max_capacity(_: *mut lshpack_enc, _: libc::c_uint);
    fn lshpack_enc_encode(
        henc: *mut lshpack_enc,
        dst: *mut libc::c_uchar,
        dst_end: *mut libc::c_uchar,
        input: *mut lsxpack_header,
    ) -> *mut libc::c_uchar;
    fn lshpack_enc_cleanup(_: *mut lshpack_enc);
    fn lshpack_enc_init(_: *mut lshpack_enc) -> libc::c_int;
    fn buffer_string_prepare_copy(b: *mut buffer, size: size_t) -> *mut libc::c_char;
    fn buffer_string_prepare_append(b: *mut buffer, size: size_t) -> *mut libc::c_char;
    fn buffer_commit(b: *mut buffer, size: size_t);
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_str2(
        b: *mut buffer,
        s1: *const libc::c_char,
        len1: size_t,
        s2: *const libc::c_char,
        len2: size_t,
    );
    fn buffer_append_int(b: *mut buffer, val: intmax_t);
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn chunk_buffer_acquire() -> *mut buffer;
    fn chunk_buffer_release(b: *mut buffer);
    fn chunkqueue_append_mem(cq: *mut chunkqueue, mem: *const libc::c_char, len: size_t);
    fn chunkqueue_append_buffer_open_sz(cq: *mut chunkqueue, sz: size_t) -> *mut buffer;
    fn chunkqueue_append_buffer_commit(cq: *mut chunkqueue);
    fn chunkqueue_mark_written(cq: *mut chunkqueue, len: off_t);
    fn chunkqueue_remove_empty_chunks(cq: *mut chunkqueue);
    fn chunkqueue_steal(dest: *mut chunkqueue, src: *mut chunkqueue, len: off_t);
    fn chunkqueue_steal_with_tempfiles(
        dest: *mut chunkqueue,
        src: *mut chunkqueue,
        len: off_t,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn chunkqueue_compact_mem(cq: *mut chunkqueue, clen: size_t);
    fn chunkqueue_peek_data(
        cq: *mut chunkqueue,
        data: *mut *mut libc::c_char,
        dlen: *mut uint32_t,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn http_request_headers_process_h2(r: *mut request_st, scheme_port: libc::c_int);
    fn http_request_validate_pseudohdrs(
        r: *mut request_st,
        scheme: libc::c_int,
        http_parseopts: libc::c_uint,
    ) -> libc::c_int;
    fn http_request_parse_header(
        r: *mut request_st,
        hpctx: *mut http_header_parse_ctx,
    ) -> libc::c_int;
    fn http_date_time_to_str(
        s: *mut libc::c_char,
        sz: size_t,
        t: unix_time64_t,
    ) -> uint32_t;
    fn http_header_str_contains_token(
        s: *const libc::c_char,
        slen: uint32_t,
        m: *const libc::c_char,
        mlen: uint32_t,
    ) -> libc::c_int;
    fn http_header_remove_token(
        b: *mut buffer,
        m: *const libc::c_char,
        mlen: uint32_t,
    ) -> libc::c_int;
    fn http_header_response_unset(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    );
    fn http_header_request_get(
        r: *const request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn http_header_request_unset(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    );
    fn http_header_parse_hoff(
        n: *const libc::c_char,
        clen: uint32_t,
        hoff: *mut libc::c_ushort,
    ) -> uint32_t;
    static mut log_epoch_secs: unix_time64_t;
    static mut log_monotonic_secs: unix_time64_t;
    fn log_error(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        fmt: *const libc::c_char,
        _: ...
    );
    fn http_response_omit_header(
        r: *mut request_st,
        ds: *const data_string,
    ) -> libc::c_int;
    fn buffer_append_base64_decode(
        out: *mut buffer,
        in_0: *const libc::c_char,
        in_length: size_t,
        charset: base64_charset,
    ) -> *mut libc::c_uchar;
    fn plugins_call_handle_request_done(r: *mut request_st) -> handler_t;
    fn request_release(r: *mut request_st);
    fn request_acquire(con: *mut connection) -> *mut request_st;
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
pub type __syscall_slong_t = libc::c_long;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
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
pub type uintptr_t = libc::c_ulong;
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
pub type lsxpack_strlen_t = uint16_t;
pub type lsxpack_flag = libc::c_uint;
pub const LSXPACK_NEVER_INDEX: lsxpack_flag = 64;
pub const LSXPACK_VAL_MATCHED: lsxpack_flag = 32;
pub const LSXPACK_NAMEVAL_HASH: lsxpack_flag = 16;
pub const LSXPACK_NAME_HASH: lsxpack_flag = 8;
pub const LSXPACK_APP_IDX: lsxpack_flag = 4;
pub const LSXPACK_QPACK_IDX: lsxpack_flag = 2;
pub const LSXPACK_HPACK_VAL_MATCHED: lsxpack_flag = 1;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct lsxpack_header {
    pub buf: *mut libc::c_char,
    pub name_hash: uint32_t,
    pub nameval_hash: uint32_t,
    pub name_offset: lsxpack_strlen_t,
    pub name_len: lsxpack_strlen_t,
    pub val_offset: lsxpack_strlen_t,
    pub val_len: lsxpack_strlen_t,
    pub chain_next_idx: uint16_t,
    pub hpack_index: uint8_t,
    pub qpack_index: uint8_t,
    pub app_index: uint8_t,
    #[bitfield(name = "flags", ty = "lsxpack_flag", bits = "0..=7")]
    pub flags: [u8; 1],
    pub indexed_type: uint8_t,
    pub dec_overhead: uint8_t,
}
pub type lsxpack_header_t = lsxpack_header;
pub type lshpack_static_hdr_idx = libc::c_uint;
pub const LSHPACK_HDR_TOBE_INDEXED: lshpack_static_hdr_idx = 255;
pub const LSHPACK_HDR_WWW_AUTHENTICATE: lshpack_static_hdr_idx = 61;
pub const LSHPACK_HDR_VIA: lshpack_static_hdr_idx = 60;
pub const LSHPACK_HDR_VARY: lshpack_static_hdr_idx = 59;
pub const LSHPACK_HDR_USER_AGENT: lshpack_static_hdr_idx = 58;
pub const LSHPACK_HDR_TRANSFER_ENCODING: lshpack_static_hdr_idx = 57;
pub const LSHPACK_HDR_STRICT_TRANSPORT_SECURITY: lshpack_static_hdr_idx = 56;
pub const LSHPACK_HDR_SET_COOKIE: lshpack_static_hdr_idx = 55;
pub const LSHPACK_HDR_SERVER: lshpack_static_hdr_idx = 54;
pub const LSHPACK_HDR_RETRY_AFTER: lshpack_static_hdr_idx = 53;
pub const LSHPACK_HDR_REFRESH: lshpack_static_hdr_idx = 52;
pub const LSHPACK_HDR_REFERER: lshpack_static_hdr_idx = 51;
pub const LSHPACK_HDR_RANGE: lshpack_static_hdr_idx = 50;
pub const LSHPACK_HDR_PROXY_AUTHORIZATION: lshpack_static_hdr_idx = 49;
pub const LSHPACK_HDR_PROXY_AUTHENTICATE: lshpack_static_hdr_idx = 48;
pub const LSHPACK_HDR_MAX_FORWARDS: lshpack_static_hdr_idx = 47;
pub const LSHPACK_HDR_LOCATION: lshpack_static_hdr_idx = 46;
pub const LSHPACK_HDR_LINK: lshpack_static_hdr_idx = 45;
pub const LSHPACK_HDR_LAST_MODIFIED: lshpack_static_hdr_idx = 44;
pub const LSHPACK_HDR_IF_UNMODIFIED_SINCE: lshpack_static_hdr_idx = 43;
pub const LSHPACK_HDR_IF_RANGE: lshpack_static_hdr_idx = 42;
pub const LSHPACK_HDR_IF_NONE_MATCH: lshpack_static_hdr_idx = 41;
pub const LSHPACK_HDR_IF_MODIFIED_SINCE: lshpack_static_hdr_idx = 40;
pub const LSHPACK_HDR_IF_MATCH: lshpack_static_hdr_idx = 39;
pub const LSHPACK_HDR_HOST: lshpack_static_hdr_idx = 38;
pub const LSHPACK_HDR_FROM: lshpack_static_hdr_idx = 37;
pub const LSHPACK_HDR_EXPIRES: lshpack_static_hdr_idx = 36;
pub const LSHPACK_HDR_EXPECT: lshpack_static_hdr_idx = 35;
pub const LSHPACK_HDR_ETAG: lshpack_static_hdr_idx = 34;
pub const LSHPACK_HDR_DATE: lshpack_static_hdr_idx = 33;
pub const LSHPACK_HDR_COOKIE: lshpack_static_hdr_idx = 32;
pub const LSHPACK_HDR_CONTENT_TYPE: lshpack_static_hdr_idx = 31;
pub const LSHPACK_HDR_CONTENT_RANGE: lshpack_static_hdr_idx = 30;
pub const LSHPACK_HDR_CONTENT_LOCATION: lshpack_static_hdr_idx = 29;
pub const LSHPACK_HDR_CONTENT_LENGTH: lshpack_static_hdr_idx = 28;
pub const LSHPACK_HDR_CONTENT_LANGUAGE: lshpack_static_hdr_idx = 27;
pub const LSHPACK_HDR_CONTENT_ENCODING: lshpack_static_hdr_idx = 26;
pub const LSHPACK_HDR_CONTENT_DISPOSITION: lshpack_static_hdr_idx = 25;
pub const LSHPACK_HDR_CACHE_CONTROL: lshpack_static_hdr_idx = 24;
pub const LSHPACK_HDR_AUTHORIZATION: lshpack_static_hdr_idx = 23;
pub const LSHPACK_HDR_ALLOW: lshpack_static_hdr_idx = 22;
pub const LSHPACK_HDR_AGE: lshpack_static_hdr_idx = 21;
pub const LSHPACK_HDR_ACCESS_CONTROL_ALLOW_ORIGIN: lshpack_static_hdr_idx = 20;
pub const LSHPACK_HDR_ACCEPT: lshpack_static_hdr_idx = 19;
pub const LSHPACK_HDR_ACCEPT_RANGES: lshpack_static_hdr_idx = 18;
pub const LSHPACK_HDR_ACCEPT_LANGUAGE: lshpack_static_hdr_idx = 17;
pub const LSHPACK_HDR_ACCEPT_ENCODING: lshpack_static_hdr_idx = 16;
pub const LSHPACK_HDR_ACCEPT_CHARSET: lshpack_static_hdr_idx = 15;
pub const LSHPACK_HDR_STATUS_500: lshpack_static_hdr_idx = 14;
pub const LSHPACK_HDR_STATUS_404: lshpack_static_hdr_idx = 13;
pub const LSHPACK_HDR_STATUS_400: lshpack_static_hdr_idx = 12;
pub const LSHPACK_HDR_STATUS_304: lshpack_static_hdr_idx = 11;
pub const LSHPACK_HDR_STATUS_206: lshpack_static_hdr_idx = 10;
pub const LSHPACK_HDR_STATUS_204: lshpack_static_hdr_idx = 9;
pub const LSHPACK_HDR_STATUS_200: lshpack_static_hdr_idx = 8;
pub const LSHPACK_HDR_SCHEME_HTTPS: lshpack_static_hdr_idx = 7;
pub const LSHPACK_HDR_SCHEME_HTTP: lshpack_static_hdr_idx = 6;
pub const LSHPACK_HDR_PATH_INDEX_HTML: lshpack_static_hdr_idx = 5;
pub const LSHPACK_HDR_PATH: lshpack_static_hdr_idx = 4;
pub const LSHPACK_HDR_METHOD_POST: lshpack_static_hdr_idx = 3;
pub const LSHPACK_HDR_METHOD_GET: lshpack_static_hdr_idx = 2;
pub const LSHPACK_HDR_AUTHORITY: lshpack_static_hdr_idx = 1;
pub const LSHPACK_HDR_UNKNOWN: lshpack_static_hdr_idx = 0;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const H2_FTYPE_CONTINUATION: C2RustUnnamed_6 = 9;
pub const H2_FTYPE_WINDOW_UPDATE: C2RustUnnamed_6 = 8;
pub const H2_FTYPE_GOAWAY: C2RustUnnamed_6 = 7;
pub const H2_FTYPE_PING: C2RustUnnamed_6 = 6;
pub const H2_FTYPE_PUSH_PROMISE: C2RustUnnamed_6 = 5;
pub const H2_FTYPE_SETTINGS: C2RustUnnamed_6 = 4;
pub const H2_FTYPE_RST_STREAM: C2RustUnnamed_6 = 3;
pub const H2_FTYPE_PRIORITY: C2RustUnnamed_6 = 2;
pub const H2_FTYPE_HEADERS: C2RustUnnamed_6 = 1;
pub const H2_FTYPE_DATA: C2RustUnnamed_6 = 0;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const H2_SETTINGS_MAX_HEADER_LIST_SIZE: C2RustUnnamed_7 = 6;
pub const H2_SETTINGS_MAX_FRAME_SIZE: C2RustUnnamed_7 = 5;
pub const H2_SETTINGS_INITIAL_WINDOW_SIZE: C2RustUnnamed_7 = 4;
pub const H2_SETTINGS_MAX_CONCURRENT_STREAMS: C2RustUnnamed_7 = 3;
pub const H2_SETTINGS_ENABLE_PUSH: C2RustUnnamed_7 = 2;
pub const H2_SETTINGS_HEADER_TABLE_SIZE: C2RustUnnamed_7 = 1;
pub type C2RustUnnamed_8 = libc::c_uint;
pub const H2_FLAG_ACK: C2RustUnnamed_8 = 1;
pub const H2_FLAG_PRIORITY: C2RustUnnamed_8 = 32;
pub const H2_FLAG_PADDED: C2RustUnnamed_8 = 8;
pub const H2_FLAG_END_HEADERS: C2RustUnnamed_8 = 4;
pub const H2_FLAG_END_STREAM: C2RustUnnamed_8 = 1;
pub type request_h2error_t = libc::c_uint;
pub const H2_E_HTTP_1_1_REQUIRED: request_h2error_t = 13;
pub const H2_E_INADEQUATE_SECURITY: request_h2error_t = 12;
pub const H2_E_ENHANCE_YOUR_CALM: request_h2error_t = 11;
pub const H2_E_CONNECT_ERROR: request_h2error_t = 10;
pub const H2_E_COMPRESSION_ERROR: request_h2error_t = 9;
pub const H2_E_CANCEL: request_h2error_t = 8;
pub const H2_E_REFUSED_STREAM: request_h2error_t = 7;
pub const H2_E_FRAME_SIZE_ERROR: request_h2error_t = 6;
pub const H2_E_STREAM_CLOSED: request_h2error_t = 5;
pub const H2_E_SETTINGS_TIMEOUT: request_h2error_t = 4;
pub const H2_E_FLOW_CONTROL_ERROR: request_h2error_t = 3;
pub const H2_E_INTERNAL_ERROR: request_h2error_t = 2;
pub const H2_E_PROTOCOL_ERROR: request_h2error_t = 1;
pub const H2_E_NO_ERROR: request_h2error_t = 0;
pub type C2RustUnnamed_9 = libc::c_uint;
pub const H2_STATE_CLOSED: C2RustUnnamed_9 = 6;
pub const H2_STATE_HALF_CLOSED_REMOTE: C2RustUnnamed_9 = 5;
pub const H2_STATE_HALF_CLOSED_LOCAL: C2RustUnnamed_9 = 4;
pub const H2_STATE_OPEN: C2RustUnnamed_9 = 3;
pub const H2_STATE_RESERVED_REMOTE: C2RustUnnamed_9 = 2;
pub const H2_STATE_RESERVED_LOCAL: C2RustUnnamed_9 = 1;
pub const H2_STATE_IDLE: C2RustUnnamed_9 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub c: [uint8_t; 20],
    pub u: [uint32_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub c: [uint8_t; 16],
    pub u: [uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub c: [uint8_t; 16],
    pub u: [uint32_t; 4],
}
pub const HTTP_HEADER_CONTENT_LENGTH: http_header_e = 14;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_header_parse_ctx {
    pub k: *mut libc::c_char,
    pub v: *mut libc::c_char,
    pub klen: uint32_t,
    pub vlen: uint32_t,
    pub hlen: uint32_t,
    pub pseudo: uint8_t,
    pub scheme: uint8_t,
    pub trailers: uint8_t,
    pub id: int8_t,
    pub max_request_field_size: uint32_t,
    pub http_parseopts: libc::c_uint,
}
pub const HTTP_HEADER_WWW_AUTHENTICATE: http_header_e = 53;
pub const HTTP_HEADER_OTHER: http_header_e = 0;
pub const HTTP_HEADER_VARY: http_header_e = 52;
pub const HTTP_HEADER_USER_AGENT: http_header_e = 51;
pub const HTTP_HEADER_TRANSFER_ENCODING: http_header_e = 48;
pub const HTTP_HEADER_STRICT_TRANSPORT_SECURITY: http_header_e = 46;
pub const HTTP_HEADER_SET_COOKIE: http_header_e = 44;
pub const HTTP_HEADER_SERVER: http_header_e = 43;
pub const HTTP_HEADER_REFERER: http_header_e = 41;
pub const HTTP_HEADER_RANGE: http_header_e = 40;
pub const HTTP_HEADER_LOCATION: http_header_e = 36;
pub const HTTP_HEADER_LINK: http_header_e = 35;
pub const HTTP_HEADER_LAST_MODIFIED: http_header_e = 34;
pub const HTTP_HEADER_IF_UNMODIFIED_SINCE: http_header_e = 33;
pub const HTTP_HEADER_IF_RANGE: http_header_e = 32;
pub const HTTP_HEADER_IF_NONE_MATCH: http_header_e = 31;
pub const HTTP_HEADER_IF_MODIFIED_SINCE: http_header_e = 30;
pub const HTTP_HEADER_IF_MATCH: http_header_e = 29;
pub const HTTP_HEADER_HOST: http_header_e = 27;
pub const HTTP_HEADER_EXPIRES: http_header_e = 25;
pub const HTTP_HEADER_EXPECT: http_header_e = 23;
pub const HTTP_HEADER_ETAG: http_header_e = 22;
pub const HTTP_HEADER_DATE: http_header_e = 20;
pub const HTTP_HEADER_COOKIE: http_header_e = 19;
pub const HTTP_HEADER_CONTENT_TYPE: http_header_e = 18;
pub const HTTP_HEADER_CONTENT_RANGE: http_header_e = 16;
pub const HTTP_HEADER_CONTENT_LOCATION: http_header_e = 15;
pub const HTTP_HEADER_CONTENT_ENCODING: http_header_e = 13;
pub const HTTP_HEADER_CACHE_CONTROL: http_header_e = 11;
pub const HTTP_HEADER_AUTHORIZATION: http_header_e = 10;
pub const HTTP_HEADER_ALLOW: http_header_e = 7;
pub const HTTP_HEADER_AGE: http_header_e = 6;
pub const HTTP_HEADER_ACCESS_CONTROL_ALLOW_ORIGIN: http_header_e = 5;
pub const HTTP_HEADER_ACCEPT: http_header_e = 1;
pub const HTTP_HEADER_ACCEPT_RANGES: http_header_e = 4;
pub const HTTP_HEADER_ACCEPT_LANGUAGE: http_header_e = 3;
pub const HTTP_HEADER_ACCEPT_ENCODING: http_header_e = 2;
pub const HTTP_HEADER_H2_UNKNOWN: http_header_h2_e = -1;
pub const HTTP_HEADER_H2_SCHEME_HTTPS: http_header_h2_e = -8;
pub const HTTP_HEADER_H2_SCHEME_HTTP: http_header_h2_e = -7;
pub const HTTP_HEADER_H2_PATH_INDEX_HTML: http_header_h2_e = -6;
pub const HTTP_HEADER_H2_PATH: http_header_h2_e = -5;
pub const HTTP_HEADER_H2_METHOD_POST: http_header_h2_e = -4;
pub const HTTP_HEADER_H2_METHOD_GET: http_header_h2_e = -3;
pub const HTTP_HEADER_H2_AUTHORITY: http_header_h2_e = -2;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub c: [uint8_t; 12],
    pub u: [uint32_t; 3],
}
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
pub const HTTP_HEADER_UPGRADE_INSECURE_REQUESTS: http_header_e = 50;
pub const HTTP_HEADER_UPGRADE: http_header_e = 49;
pub const HTTP_HEADER_TE: http_header_e = 47;
pub const HTTP_HEADER_STATUS: http_header_e = 45;
pub const HTTP_HEADER_REFERRER_POLICY: http_header_e = 42;
pub const HTTP_HEADER_PRAGMA: http_header_e = 39;
pub const HTTP_HEADER_P3P: http_header_e = 38;
pub const HTTP_HEADER_ONION_LOCATION: http_header_e = 37;
pub const HTTP_HEADER_HTTP2_SETTINGS: http_header_e = 28;
pub const HTTP_HEADER_FORWARDED: http_header_e = 26;
pub const HTTP_HEADER_EXPECT_CT: http_header_e = 24;
pub const HTTP_HEADER_DNT: http_header_e = 21;
pub const HTTP_HEADER_CONTENT_SECURITY_POLICY: http_header_e = 17;
pub const HTTP_HEADER_CONNECTION: http_header_e = 12;
pub const HTTP_HEADER_ALT_USED: http_header_e = 9;
pub const HTTP_HEADER_ALT_SVC: http_header_e = 8;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub c: [uint8_t; 12],
    pub u: [uint32_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_15 {
    pub c: [uint8_t; 12],
    pub u: [uint32_t; 3],
}
pub type base64_charset = libc::c_uint;
pub const BASE64_URL: base64_charset = 1;
pub const BASE64_STANDARD: base64_charset = 0;
pub type http_header_h2_e = libc::c_int;
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
#[inline]
unsafe extern "C" fn buffer_clear(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn buffer_is_unset(mut b: *const buffer) -> libc::c_int {
    return (0 as libc::c_int as libc::c_uint == (*b).used) as libc::c_int;
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
static mut http_header_lc: [[libc::c_char; 32]; 59] = unsafe {
    [
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"accept\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"accept-encoding\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"accept-language\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"accept-ranges\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"access-control-allow-origin\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"age\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"allow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"alt-svc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"alt-used\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"authorization\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"cache-control\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"connection\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"content-encoding\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"content-length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"content-location\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"content-range\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"content-security-policy\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"content-type\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"cookie\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"date\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"dnt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"etag\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"expect\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"expect-ct\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"expires\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"forwarded\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"host\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"http2-settings\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"if-match\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"if-modified-since\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"if-none-match\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"if-range\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"if-unmodified-since\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"last-modified\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"link\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"location\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"onion-location\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"p3p\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"pragma\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"range\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"referer\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"referrer-policy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"server\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"set-cookie\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"status\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"strict-transport-security\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"te\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"transfer-encoding\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"upgrade\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"upgrade-insecure-requests\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"user-agent\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"vary\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"www-authenticate\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"x-content-type-options\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"x-forwarded-for\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"x-forwarded-proto\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"x-frame-options\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"x-xss-protection\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
    ]
};
static mut http_header_lshpack_idx: [uint8_t; 59] = [
    LSHPACK_HDR_UNKNOWN as libc::c_int as uint8_t,
    LSHPACK_HDR_ACCEPT as libc::c_int as uint8_t,
    LSHPACK_HDR_ACCEPT_ENCODING as libc::c_int as uint8_t,
    LSHPACK_HDR_ACCEPT_LANGUAGE as libc::c_int as uint8_t,
    LSHPACK_HDR_ACCEPT_RANGES as libc::c_int as uint8_t,
    LSHPACK_HDR_ACCESS_CONTROL_ALLOW_ORIGIN as libc::c_int as uint8_t,
    LSHPACK_HDR_AGE as libc::c_int as uint8_t,
    LSHPACK_HDR_ALLOW as libc::c_int as uint8_t,
    LSHPACK_HDR_UNKNOWN as libc::c_int as uint8_t,
    LSHPACK_HDR_UNKNOWN as libc::c_int as uint8_t,
    LSHPACK_HDR_AUTHORIZATION as libc::c_int as uint8_t,
    LSHPACK_HDR_CACHE_CONTROL as libc::c_int as uint8_t,
    LSHPACK_HDR_UNKNOWN as libc::c_int as uint8_t,
    LSHPACK_HDR_CONTENT_ENCODING as libc::c_int as uint8_t,
    LSHPACK_HDR_CONTENT_LENGTH as libc::c_int as uint8_t,
    LSHPACK_HDR_CONTENT_LOCATION as libc::c_int as uint8_t,
    LSHPACK_HDR_CONTENT_RANGE as libc::c_int as uint8_t,
    LSHPACK_HDR_UNKNOWN as libc::c_int as uint8_t,
    LSHPACK_HDR_CONTENT_TYPE as libc::c_int as uint8_t,
    LSHPACK_HDR_COOKIE as libc::c_int as uint8_t,
    LSHPACK_HDR_DATE as libc::c_int as uint8_t,
    LSHPACK_HDR_UNKNOWN as libc::c_int as uint8_t,
    LSHPACK_HDR_ETAG as libc::c_int as uint8_t,
    LSHPACK_HDR_EXPECT as libc::c_int as uint8_t,
    LSHPACK_HDR_UNKNOWN as libc::c_int as uint8_t,
    LSHPACK_HDR_EXPIRES as libc::c_int as uint8_t,
    LSHPACK_HDR_UNKNOWN as libc::c_int as uint8_t,
    LSHPACK_HDR_HOST as libc::c_int as uint8_t,
    LSHPACK_HDR_UNKNOWN as libc::c_int as uint8_t,
    LSHPACK_HDR_IF_MATCH as libc::c_int as uint8_t,
    LSHPACK_HDR_IF_MODIFIED_SINCE as libc::c_int as uint8_t,
    LSHPACK_HDR_IF_NONE_MATCH as libc::c_int as uint8_t,
    LSHPACK_HDR_IF_RANGE as libc::c_int as uint8_t,
    LSHPACK_HDR_IF_UNMODIFIED_SINCE as libc::c_int as uint8_t,
    LSHPACK_HDR_LAST_MODIFIED as libc::c_int as uint8_t,
    LSHPACK_HDR_LINK as libc::c_int as uint8_t,
    LSHPACK_HDR_LOCATION as libc::c_int as uint8_t,
    LSHPACK_HDR_UNKNOWN as libc::c_int as uint8_t,
    LSHPACK_HDR_UNKNOWN as libc::c_int as uint8_t,
    LSHPACK_HDR_UNKNOWN as libc::c_int as uint8_t,
    LSHPACK_HDR_RANGE as libc::c_int as uint8_t,
    LSHPACK_HDR_REFERER as libc::c_int as uint8_t,
    LSHPACK_HDR_UNKNOWN as libc::c_int as uint8_t,
    LSHPACK_HDR_SERVER as libc::c_int as uint8_t,
    LSHPACK_HDR_SET_COOKIE as libc::c_int as uint8_t,
    LSHPACK_HDR_UNKNOWN as libc::c_int as uint8_t,
    LSHPACK_HDR_STRICT_TRANSPORT_SECURITY as libc::c_int as uint8_t,
    LSHPACK_HDR_UNKNOWN as libc::c_int as uint8_t,
    LSHPACK_HDR_TRANSFER_ENCODING as libc::c_int as uint8_t,
    LSHPACK_HDR_UNKNOWN as libc::c_int as uint8_t,
    LSHPACK_HDR_UNKNOWN as libc::c_int as uint8_t,
    LSHPACK_HDR_USER_AGENT as libc::c_int as uint8_t,
    LSHPACK_HDR_VARY as libc::c_int as uint8_t,
    LSHPACK_HDR_WWW_AUTHENTICATE as libc::c_int as uint8_t,
    LSHPACK_HDR_UNKNOWN as libc::c_int as uint8_t,
    LSHPACK_HDR_UNKNOWN as libc::c_int as uint8_t,
    LSHPACK_HDR_UNKNOWN as libc::c_int as uint8_t,
    LSHPACK_HDR_UNKNOWN as libc::c_int as uint8_t,
    LSHPACK_HDR_UNKNOWN as libc::c_int as uint8_t,
];
static mut lshpack_idx_http_header: [int8_t; 62] = [
    HTTP_HEADER_H2_UNKNOWN as libc::c_int as int8_t,
    HTTP_HEADER_H2_AUTHORITY as libc::c_int as int8_t,
    HTTP_HEADER_H2_METHOD_GET as libc::c_int as int8_t,
    HTTP_HEADER_H2_METHOD_POST as libc::c_int as int8_t,
    HTTP_HEADER_H2_PATH as libc::c_int as int8_t,
    HTTP_HEADER_H2_PATH_INDEX_HTML as libc::c_int as int8_t,
    HTTP_HEADER_H2_SCHEME_HTTP as libc::c_int as int8_t,
    HTTP_HEADER_H2_SCHEME_HTTPS as libc::c_int as int8_t,
    HTTP_HEADER_H2_UNKNOWN as libc::c_int as int8_t,
    HTTP_HEADER_H2_UNKNOWN as libc::c_int as int8_t,
    HTTP_HEADER_H2_UNKNOWN as libc::c_int as int8_t,
    HTTP_HEADER_H2_UNKNOWN as libc::c_int as int8_t,
    HTTP_HEADER_H2_UNKNOWN as libc::c_int as int8_t,
    HTTP_HEADER_H2_UNKNOWN as libc::c_int as int8_t,
    HTTP_HEADER_H2_UNKNOWN as libc::c_int as int8_t,
    HTTP_HEADER_OTHER as libc::c_int as int8_t,
    HTTP_HEADER_ACCEPT_ENCODING as libc::c_int as int8_t,
    HTTP_HEADER_ACCEPT_LANGUAGE as libc::c_int as int8_t,
    HTTP_HEADER_ACCEPT_RANGES as libc::c_int as int8_t,
    HTTP_HEADER_ACCEPT as libc::c_int as int8_t,
    HTTP_HEADER_ACCESS_CONTROL_ALLOW_ORIGIN as libc::c_int as int8_t,
    HTTP_HEADER_AGE as libc::c_int as int8_t,
    HTTP_HEADER_ALLOW as libc::c_int as int8_t,
    HTTP_HEADER_AUTHORIZATION as libc::c_int as int8_t,
    HTTP_HEADER_CACHE_CONTROL as libc::c_int as int8_t,
    HTTP_HEADER_OTHER as libc::c_int as int8_t,
    HTTP_HEADER_CONTENT_ENCODING as libc::c_int as int8_t,
    HTTP_HEADER_OTHER as libc::c_int as int8_t,
    HTTP_HEADER_CONTENT_LENGTH as libc::c_int as int8_t,
    HTTP_HEADER_CONTENT_LOCATION as libc::c_int as int8_t,
    HTTP_HEADER_CONTENT_RANGE as libc::c_int as int8_t,
    HTTP_HEADER_CONTENT_TYPE as libc::c_int as int8_t,
    HTTP_HEADER_COOKIE as libc::c_int as int8_t,
    HTTP_HEADER_DATE as libc::c_int as int8_t,
    HTTP_HEADER_ETAG as libc::c_int as int8_t,
    HTTP_HEADER_EXPECT as libc::c_int as int8_t,
    HTTP_HEADER_EXPIRES as libc::c_int as int8_t,
    HTTP_HEADER_OTHER as libc::c_int as int8_t,
    HTTP_HEADER_HOST as libc::c_int as int8_t,
    HTTP_HEADER_IF_MATCH as libc::c_int as int8_t,
    HTTP_HEADER_IF_MODIFIED_SINCE as libc::c_int as int8_t,
    HTTP_HEADER_IF_NONE_MATCH as libc::c_int as int8_t,
    HTTP_HEADER_IF_RANGE as libc::c_int as int8_t,
    HTTP_HEADER_IF_UNMODIFIED_SINCE as libc::c_int as int8_t,
    HTTP_HEADER_LAST_MODIFIED as libc::c_int as int8_t,
    HTTP_HEADER_LINK as libc::c_int as int8_t,
    HTTP_HEADER_LOCATION as libc::c_int as int8_t,
    HTTP_HEADER_OTHER as libc::c_int as int8_t,
    HTTP_HEADER_OTHER as libc::c_int as int8_t,
    HTTP_HEADER_OTHER as libc::c_int as int8_t,
    HTTP_HEADER_RANGE as libc::c_int as int8_t,
    HTTP_HEADER_REFERER as libc::c_int as int8_t,
    HTTP_HEADER_OTHER as libc::c_int as int8_t,
    HTTP_HEADER_OTHER as libc::c_int as int8_t,
    HTTP_HEADER_SERVER as libc::c_int as int8_t,
    HTTP_HEADER_SET_COOKIE as libc::c_int as int8_t,
    HTTP_HEADER_STRICT_TRANSPORT_SECURITY as libc::c_int as int8_t,
    HTTP_HEADER_TRANSFER_ENCODING as libc::c_int as int8_t,
    HTTP_HEADER_USER_AGENT as libc::c_int as int8_t,
    HTTP_HEADER_VARY as libc::c_int as int8_t,
    HTTP_HEADER_OTHER as libc::c_int as int8_t,
    HTTP_HEADER_WWW_AUTHENTICATE as libc::c_int as int8_t,
];
#[inline]
unsafe extern "C" fn h2_u32(s: *const uint8_t) -> uint32_t {
    return (*s.offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*s.offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*s.offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | *s.offset(3 as libc::c_int as isize) as uint32_t;
}
#[inline]
unsafe extern "C" fn h2_u31(s: *const uint8_t) -> uint32_t {
    return h2_u32(s) & !(0x80000000 as libc::c_uint);
}
#[inline]
unsafe extern "C" fn h2_u24(s: *const uint8_t) -> uint32_t {
    return h2_u32(s) >> 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn h2_u16(s: *const uint8_t) -> uint16_t {
    return ((*s.offset(0 as libc::c_int as isize) as uint16_t as libc::c_int)
        << 8 as libc::c_int
        | *s.offset(1 as libc::c_int as isize) as uint16_t as libc::c_int) as uint16_t;
}
unsafe extern "C" fn h2_send_settings_ack(con: *mut connection) {
    static mut settings_ack: [uint8_t; 9] = [
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        H2_FTYPE_SETTINGS as libc::c_int as uint8_t,
        H2_FLAG_ACK as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    ];
    chunkqueue_append_mem(
        (*con).write_queue,
        settings_ack.as_ptr() as *const libc::c_char,
        ::std::mem::size_of::<[uint8_t; 9]>() as libc::c_ulong,
    );
}
#[cold]
unsafe extern "C" fn h2_send_rst_stream_id(
    mut h2id: uint32_t,
    con: *mut connection,
    e: request_h2error_t,
) {
    let mut rst_stream: C2RustUnnamed_11 = C2RustUnnamed_11 {
        c: [
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0x4 as libc::c_int as uint8_t,
            H2_FTYPE_RST_STREAM as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
        ],
    };
    rst_stream.u[2 as libc::c_int as usize] = __bswap_32(h2id);
    rst_stream.u[3 as libc::c_int as usize] = __bswap_32(e as __uint32_t);
    chunkqueue_append_mem(
        (*con).write_queue,
        ((rst_stream.c).as_mut_ptr() as *const libc::c_char)
            .offset(3 as libc::c_int as isize),
        (::std::mem::size_of::<C2RustUnnamed_11>() as libc::c_ulong)
            .wrapping_sub(3 as libc::c_int as libc::c_ulong),
    );
}
#[cold]
unsafe extern "C" fn h2_send_rst_stream_state(r: *mut request_st, h2c: *mut h2con) {
    if (*r).h2state != H2_STATE_HALF_CLOSED_REMOTE as libc::c_int as libc::c_uint
        && (*r).h2state != H2_STATE_CLOSED as libc::c_int as libc::c_uint
    {
        (*h2c).half_closed_ts = log_monotonic_secs;
    }
    (*r).state = CON_STATE_ERROR;
    (*r).h2state = H2_STATE_CLOSED as libc::c_int as uint32_t;
}
#[cold]
unsafe extern "C" fn h2_send_rst_stream(
    r: *mut request_st,
    con: *mut connection,
    e: request_h2error_t,
) {
    h2_send_rst_stream_state(r, (*con).h2);
    h2_send_rst_stream_id((*r).h2id, con, e);
}
#[cold]
unsafe extern "C" fn h2_send_goaway_rst_stream(con: *mut connection) {
    let h2c: *mut h2con = (*con).h2;
    let sent_goaway: libc::c_int = (*h2c).sent_goaway;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let mut rused: uint32_t = (*h2c).rused;
    while i < rused {
        let r: *mut request_st = (*h2c).r[i as usize];
        if !((*r).h2state == H2_STATE_CLOSED as libc::c_int as libc::c_uint) {
            h2_send_rst_stream_state(r, h2c);
            if sent_goaway != 0 {
                h2_send_rst_stream_id((*r).h2id, con, H2_E_PROTOCOL_ERROR);
            }
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn h2_send_goaway(con: *mut connection, e: request_h2error_t) {
    if e as libc::c_uint != H2_E_NO_ERROR as libc::c_int as libc::c_uint {
        h2_send_goaway_rst_stream(con);
    }
    let h2c: *mut h2con = (*con).h2;
    if (*h2c).sent_goaway != 0
        && ((*h2c).sent_goaway > 0 as libc::c_int
            || e as libc::c_uint == H2_E_NO_ERROR as libc::c_int as libc::c_uint)
    {
        return;
    }
    (*h2c)
        .sent_goaway = if e as libc::c_uint
        == H2_E_NO_ERROR as libc::c_int as libc::c_uint
    {
        -(1 as libc::c_int)
    } else {
        e as int32_t
    };
    let mut goaway: C2RustUnnamed_10 = C2RustUnnamed_10 {
        c: [
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0x8 as libc::c_int as uint8_t,
            H2_FTYPE_GOAWAY as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
        ],
    };
    goaway.u[3 as libc::c_int as usize] = __bswap_32((*h2c).h2_cid);
    goaway.u[4 as libc::c_int as usize] = __bswap_32(e as __uint32_t);
    chunkqueue_append_mem(
        (*con).write_queue,
        ((goaway.c).as_mut_ptr() as *const libc::c_char)
            .offset(3 as libc::c_int as isize),
        (::std::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong)
            .wrapping_sub(3 as libc::c_int as libc::c_ulong),
    );
}
#[cold]
unsafe extern "C" fn h2_send_goaway_e(con: *mut connection, e: request_h2error_t) {
    h2_send_goaway(con, e);
}
#[cold]
unsafe extern "C" fn h2_send_refused_stream(
    mut h2id: uint32_t,
    con: *mut connection,
) -> libc::c_int {
    let h2c: *mut h2con = (*con).h2;
    if (*h2c).sent_settings != 0 {
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        let mut rused: uint32_t = (*h2c).rused;
        while i < rused {
            let r: *const request_st = (*h2c).r[i as usize];
            if (*r).reqbody_length == (*r).reqbody_queue.bytes_in {
                return -(1 as libc::c_int);
            }
            i = i.wrapping_add(1);
        }
    }
    (*h2c).h2_cid = h2id;
    h2_send_rst_stream_id(h2id, con, H2_E_REFUSED_STREAM);
    return 1 as libc::c_int;
}
unsafe extern "C" fn h2_recv_goaway(
    con: *mut connection,
    s: *const uint8_t,
    mut len: uint32_t,
) -> libc::c_int {
    if 0 as libc::c_int as libc::c_uint != h2_u31(s.offset(5 as libc::c_int as isize)) {
        h2_send_goaway_e(con, H2_E_PROTOCOL_ERROR);
        return 0 as libc::c_int;
    }
    let e: uint32_t = h2_u32(s.offset(13 as libc::c_int as isize));
    h2_send_goaway(
        con,
        (if e == H2_E_NO_ERROR as libc::c_int as libc::c_uint {
            H2_E_NO_ERROR as libc::c_int
        } else {
            H2_E_PROTOCOL_ERROR as libc::c_int
        }) as request_h2error_t,
    );
    let h2c: *mut h2con = (*con).h2;
    if 0 as libc::c_int as libc::c_uint == (*h2c).rused {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn h2_recv_rst_stream(
    con: *mut connection,
    s: *const uint8_t,
    len: uint32_t,
) {
    if 4 as libc::c_int as libc::c_uint != len {
        h2_send_goaway_e(con, H2_E_FRAME_SIZE_ERROR);
        return;
    }
    let id: uint32_t = h2_u31(s.offset(5 as libc::c_int as isize));
    if 0 as libc::c_int as libc::c_uint == id {
        h2_send_goaway_e(con, H2_E_PROTOCOL_ERROR);
        return;
    }
    let h2c: *mut h2con = (*con).h2;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let mut rused: uint32_t = (*h2c).rused;
    while i < rused {
        let r: *mut request_st = (*h2c).r[i as usize];
        if (*r).h2id != id {
            i = i.wrapping_add(1);
        } else {
            if (*r).h2state == H2_STATE_IDLE as libc::c_int as libc::c_uint {
                h2_send_goaway_e(con, H2_E_PROTOCOL_ERROR);
                return;
            }
            (*r).state = CON_STATE_ERROR;
            (*r).h2state = H2_STATE_CLOSED as libc::c_int as uint32_t;
            return;
        }
    }
    if (*h2c).h2_cid < id {
        h2_send_goaway_e(con, H2_E_PROTOCOL_ERROR);
        return;
    }
}
unsafe extern "C" fn h2_recv_ping(con: *mut connection, s: *mut uint8_t, len: uint32_t) {
    if 8 as libc::c_int as libc::c_uint != len {
        h2_send_goaway_e(con, H2_E_FRAME_SIZE_ERROR);
        return;
    }
    let ref mut fresh0 = *s.offset(5 as libc::c_int as isize);
    *fresh0 = (*fresh0 as libc::c_int & !(0x80 as libc::c_int)) as uint8_t;
    if 0 as libc::c_int as libc::c_uint != h2_u31(s.offset(5 as libc::c_int as isize)) {
        h2_send_goaway_e(con, H2_E_PROTOCOL_ERROR);
        return;
    }
    if *s.offset(4 as libc::c_int as isize) as libc::c_int & H2_FLAG_ACK as libc::c_int
        != 0
    {
        return;
    }
    *s.offset(4 as libc::c_int as isize) = H2_FLAG_ACK as libc::c_int as uint8_t;
    chunkqueue_append_mem(
        (*con).write_queue,
        s as *const libc::c_char,
        17 as libc::c_int as size_t,
    );
}
unsafe extern "C" fn h2_recv_priority(
    con: *mut connection,
    s: *const uint8_t,
    len: uint32_t,
) {
    if 5 as libc::c_int as libc::c_uint != len {
        h2_send_goaway_e(con, H2_E_FRAME_SIZE_ERROR);
        return;
    }
    let id: uint32_t = h2_u31(s.offset(5 as libc::c_int as isize));
    if 0 as libc::c_int as libc::c_uint == id {
        h2_send_goaway_e(con, H2_E_PROTOCOL_ERROR);
        return;
    }
    let prio: uint32_t = h2_u31(s.offset(9 as libc::c_int as isize));
    let h2c: *mut h2con = (*con).h2;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let mut rused: uint32_t = (*h2c).rused;
    while i < rused {
        let r: *mut request_st = (*h2c).r[i as usize];
        if (*r).h2id != id {
            i = i.wrapping_add(1);
        } else {
            if prio == id {
                h2_send_rst_stream(r, con, H2_E_PROTOCOL_ERROR);
                return;
            }
            return;
        }
    }
    if prio == id {
        h2_send_rst_stream_id(id, con, H2_E_PROTOCOL_ERROR);
        return;
    }
}
unsafe extern "C" fn h2_recv_window_update(
    con: *mut connection,
    s: *const uint8_t,
    len: uint32_t,
) {
    if 4 as libc::c_int as libc::c_uint != len {
        h2_send_goaway_e(con, H2_E_FRAME_SIZE_ERROR);
        return;
    }
    let id: uint32_t = h2_u31(s.offset(5 as libc::c_int as isize));
    let v: int32_t = h2_u31(s.offset(9 as libc::c_int as isize)) as int32_t;
    let mut r: *mut request_st = 0 as *mut request_st;
    if 0 as libc::c_int as libc::c_uint == id {
        r = &mut (*con).request;
    } else {
        let h2c: *mut h2con = (*con).h2;
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        let mut rused: uint32_t = (*h2c).rused;
        while i < rused {
            let rr: *mut request_st = (*h2c).r[i as usize];
            if (*rr).h2id != id {
                i = i.wrapping_add(1);
            } else {
                r = rr;
                break;
            }
        }
        if r.is_null() {
            if (*h2c).h2_cid < id && 0 as libc::c_int == (*h2c).sent_goaway {
                h2_send_goaway_e(con, H2_E_PROTOCOL_ERROR);
            }
            return;
        }
        if (*r).h2state == H2_STATE_CLOSED as libc::c_int as libc::c_uint
            || (*r).h2state == H2_STATE_HALF_CLOSED_LOCAL as libc::c_int as libc::c_uint
        {
            return;
        }
    }
    if 0 as libc::c_int == v || (*r).h2_swin > 2147483647 as libc::c_int - v {
        let mut e: request_h2error_t = (if 0 as libc::c_int == v {
            H2_E_PROTOCOL_ERROR as libc::c_int
        } else {
            H2_E_FLOW_CONTROL_ERROR as libc::c_int
        }) as request_h2error_t;
        if 0 as libc::c_int as libc::c_uint == id {
            h2_send_goaway_e(con, e);
        } else {
            h2_send_rst_stream(r, con, e);
        }
        return;
    }
    (*r).h2_swin += v;
}
#[inline(never)]
unsafe extern "C" fn h2_send_window_update(
    con: *mut connection,
    mut h2id: uint32_t,
    len: uint32_t,
) {
    if 0 as libc::c_int as libc::c_uint == len {
        return;
    }
    let mut window_upd: C2RustUnnamed_12 = C2RustUnnamed_12 {
        c: [
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0x4 as libc::c_int as uint8_t,
            H2_FTYPE_WINDOW_UPDATE as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
        ],
    };
    window_upd.u[2 as libc::c_int as usize] = __bswap_32(h2id);
    window_upd.u[3 as libc::c_int as usize] = __bswap_32(len);
    chunkqueue_append_mem(
        (*con).write_queue,
        ((window_upd.c).as_mut_ptr() as *const libc::c_char)
            .offset(3 as libc::c_int as isize),
        (::std::mem::size_of::<C2RustUnnamed_12>() as libc::c_ulong)
            .wrapping_sub(3 as libc::c_int as libc::c_ulong),
    );
}
unsafe extern "C" fn h2_parse_frame_settings(
    con: *mut connection,
    mut s: *const uint8_t,
    mut len: uint32_t,
) {
    let h2c: *mut h2con = (*con).h2;
    while len >= 6 as libc::c_int as libc::c_uint {
        let mut v: uint32_t = h2_u32(s.offset(2 as libc::c_int as isize));
        match h2_u16(s) as libc::c_int {
            1 => {
                if v > 4096 as libc::c_int as libc::c_uint {
                    v = 4096 as libc::c_int as uint32_t;
                }
                if !(v == (*h2c).s_header_table_size) {
                    (*h2c).s_header_table_size = v;
                    lshpack_enc_set_max_capacity(&mut (*h2c).encoder, v);
                }
            }
            2 => {
                if v | 1 as libc::c_int as libc::c_uint
                    != 1 as libc::c_int as libc::c_uint
                {
                    h2_send_goaway_e(con, H2_E_PROTOCOL_ERROR);
                    return;
                }
                (*h2c).s_enable_push = v;
            }
            3 => {
                (*h2c).s_max_concurrent_streams = v;
            }
            4 => {
                if v > 2147483647 as libc::c_int as libc::c_uint {
                    h2_send_goaway_e(con, H2_E_FLOW_CONTROL_ERROR);
                    return;
                } else {
                    if (*h2c).rused != 0 {
                        let mut diff: int32_t = v
                            .wrapping_sub((*h2c).s_initial_window_size as uint32_t)
                            as int32_t;
                        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
                        let mut rused: uint32_t = (*h2c).rused;
                        while i < rused {
                            let r: *mut request_st = (*h2c).r[i as usize];
                            let swin: int32_t = (*r).h2_swin;
                            if !((*r).h2state
                                == H2_STATE_HALF_CLOSED_LOCAL as libc::c_int as libc::c_uint
                                || (*r).h2state
                                    == H2_STATE_CLOSED as libc::c_int as libc::c_uint)
                            {
                                if if diff >= 0 as libc::c_int {
                                    (swin > 2147483647 as libc::c_int - diff) as libc::c_int
                                } else {
                                    (swin
                                        < -(2147483647 as libc::c_int) - 1 as libc::c_int - diff)
                                        as libc::c_int
                                } != 0
                                {
                                    h2_send_rst_stream(r, con, H2_E_FLOW_CONTROL_ERROR);
                                } else {
                                    (*r).h2_swin += diff;
                                }
                            }
                            i = i.wrapping_add(1);
                        }
                    }
                }
                (*h2c).s_initial_window_size = v as int32_t;
            }
            5 => {
                if v < 16384 as libc::c_int as libc::c_uint
                    || v > 16777215 as libc::c_int as libc::c_uint
                {
                    h2_send_goaway_e(con, H2_E_PROTOCOL_ERROR);
                    return;
                }
                (*h2c).s_max_frame_size = v;
            }
            6 => {
                (*h2c).s_max_header_list_size = v;
            }
            _ => {}
        }
        len = (len as libc::c_uint).wrapping_sub(6 as libc::c_int as libc::c_uint)
            as uint32_t as uint32_t;
        s = s.offset(6 as libc::c_int as isize);
    }
    if len != 0 {
        h2_send_goaway_e(con, H2_E_FRAME_SIZE_ERROR);
        return;
    }
}
unsafe extern "C" fn h2_recv_settings(
    con: *mut connection,
    s: *const uint8_t,
    len: uint32_t,
) {
    if 0 as libc::c_int as libc::c_uint != h2_u31(s.offset(5 as libc::c_int as isize)) {
        h2_send_goaway_e(con, H2_E_PROTOCOL_ERROR);
        return;
    }
    let h2c: *mut h2con = (*con).h2;
    if *s.offset(4 as libc::c_int as isize) as libc::c_int & H2_FLAG_ACK as libc::c_int
        == 0
    {
        h2_parse_frame_settings(con, s.offset(9 as libc::c_int as isize), len);
        if (*h2c).sent_goaway <= 0 as libc::c_int {
            h2_send_settings_ack(con);
        }
    } else if 0 as libc::c_int as libc::c_uint != len {
        h2_send_goaway_e(con, H2_E_FRAME_SIZE_ERROR);
    } else if (*h2c).sent_settings != 0 {
        (*h2c).sent_settings = 0 as libc::c_int as unix_time64_t;
    } else {
        h2_send_goaway_e(con, H2_E_PROTOCOL_ERROR);
    };
}
unsafe extern "C" fn h2_recv_end_data(
    r: *mut request_st,
    con: *mut connection,
    alen: uint32_t,
) -> libc::c_int {
    let reqbody_queue: *mut chunkqueue = &mut (*r).reqbody_queue;
    (*r)
        .h2state = (if (*r).h2state == H2_STATE_OPEN as libc::c_int as libc::c_uint {
        H2_STATE_HALF_CLOSED_REMOTE as libc::c_int
    } else {
        H2_STATE_CLOSED as libc::c_int
    }) as uint32_t;
    if (*r).reqbody_length == -(1 as libc::c_int) as libc::c_long {
        (*r).reqbody_length = (*reqbody_queue).bytes_in + alen as off_t;
    } else if (*r).reqbody_length != (*reqbody_queue).bytes_in + alen as off_t {
        if 0 as libc::c_int as libc::c_long == (*reqbody_queue).bytes_out {
            h2_send_rst_stream(r, con, H2_E_PROTOCOL_ERROR);
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn h2_recv_data(
    con: *mut connection,
    s: *const uint8_t,
    len: uint32_t,
) -> libc::c_int {
    let h2c: *mut h2con = (*con).h2;
    let id: uint32_t = h2_u31(s.offset(5 as libc::c_int as isize));
    if 0 as libc::c_int as libc::c_uint == id || (*h2c).h2_cid < id {
        h2_send_goaway_e(con, H2_E_PROTOCOL_ERROR);
        return 0 as libc::c_int;
    }
    let mut alen: uint32_t = len;
    let mut pad: uint32_t = 0 as libc::c_int as uint32_t;
    if *s.offset(4 as libc::c_int as isize) as libc::c_int
        & H2_FLAG_PADDED as libc::c_int != 0
    {
        pad = *s.offset(9 as libc::c_int as isize) as uint32_t;
        if pad >= len {
            h2_send_goaway_e(con, H2_E_PROTOCOL_ERROR);
            return 0 as libc::c_int;
        }
        alen = (alen as libc::c_uint)
            .wrapping_sub((1 as libc::c_int as libc::c_uint).wrapping_add(pad))
            as uint32_t as uint32_t;
    }
    let h2r: *mut request_st = &mut (*con).request;
    if (*h2r).h2_rwin <= 0 as libc::c_int && 0 as libc::c_int as libc::c_uint != alen {
        return 0 as libc::c_int;
    }
    let mut r: *mut request_st = 0 as *mut request_st;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let mut rused: uint32_t = (*h2c).rused;
    while i < rused {
        let rr: *mut request_st = (*h2c).r[i as usize];
        if (*rr).h2id != id {
            i = i.wrapping_add(1);
        } else {
            r = rr;
            break;
        }
    }
    let cq: *mut chunkqueue = (*con).read_queue;
    if r.is_null() {
        chunkqueue_mark_written(
            cq,
            (9 as libc::c_int as libc::c_uint).wrapping_add(len) as off_t,
        );
        if (*h2c).half_closed_ts + 2 as libc::c_int as libc::c_long >= log_monotonic_secs
        {
            h2_send_window_update(con, 0 as libc::c_int as uint32_t, len);
            return 1 as libc::c_int;
        } else {
            if (*h2c).sent_goaway == 0 && 0 as libc::c_int as libc::c_uint != alen {
                h2_send_goaway_e(con, H2_E_NO_ERROR);
            }
            return 0 as libc::c_int;
        }
    }
    if (*r).h2state == H2_STATE_CLOSED as libc::c_int as libc::c_uint
        || (*r).h2state == H2_STATE_HALF_CLOSED_REMOTE as libc::c_int as libc::c_uint
    {
        h2_send_rst_stream_id(id, con, H2_E_STREAM_CLOSED);
        chunkqueue_mark_written(
            cq,
            (9 as libc::c_int as libc::c_uint).wrapping_add(len) as off_t,
        );
        h2_send_window_update(con, 0 as libc::c_int as uint32_t, len);
        return 1 as libc::c_int;
    }
    if (*r).h2_rwin <= 0 as libc::c_int && 0 as libc::c_int as libc::c_uint != alen {
        if (*r).conf.stream_request_body as libc::c_int
            & (1 as libc::c_int) << 1 as libc::c_int != 0
        {
            return 0 as libc::c_int;
        }
    }
    h2_send_window_update(con, 0 as libc::c_int as uint32_t, len);
    let dst: *mut chunkqueue = &mut (*r).reqbody_queue;
    if (*r).reqbody_length >= 0 as libc::c_int as libc::c_long
        && (*r).reqbody_length < (*dst).bytes_in + alen as libc::c_long
    {
        h2_send_rst_stream(r, con, H2_E_PROTOCOL_ERROR);
        chunkqueue_mark_written(
            cq,
            (9 as libc::c_int as libc::c_uint).wrapping_add(len) as off_t,
        );
        return 1 as libc::c_int;
    }
    let rq: *mut chunkqueue = &mut (*r).read_queue;
    (*rq).bytes_in += alen as off_t;
    (*rq).bytes_out += alen as off_t;
    let mut wupd: uint32_t = 0 as libc::c_int as uint32_t;
    if *s.offset(4 as libc::c_int as isize) as libc::c_int
        & H2_FLAG_END_STREAM as libc::c_int != 0
    {
        if h2_recv_end_data(r, con, alen) == 0 {
            chunkqueue_mark_written(
                cq,
                (9 as libc::c_int as libc::c_uint).wrapping_add(len) as off_t,
            );
            return 1 as libc::c_int;
        }
    } else if 0 as libc::c_int as libc::c_uint == (*r).conf.max_request_size {
        wupd = len;
    } else {
        let max_request_size: off_t = ((*r).conf.max_request_size as off_t)
            << 10 as libc::c_int;
        let mut n: off_t = max_request_size - (*dst).bytes_in - alen as off_t;
        let mut rwin: int32_t = (*r).h2_rwin - len as int32_t;
        if rwin < 0 as libc::c_int {
            rwin = 0 as libc::c_int;
        }
        if (n >= 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long != 0 {
            wupd = if n >= rwin as libc::c_long {
                n -= rwin as libc::c_long;
                if n > len as int32_t as libc::c_long {
                    len
                } else {
                    (n as uint32_t).wrapping_add(8 as libc::c_int as libc::c_uint)
                }
            } else {
                0 as libc::c_int as libc::c_uint
            };
        } else if -n > 65536 as libc::c_int as libc::c_long
                || 0 as libc::c_int == (*r).http_status
            {
            if 0 as libc::c_int == (*r).http_status {
                (*r).http_status = 413 as libc::c_int;
                (*r).handler_module = 0 as *const plugin;
                log_error(
                    (*r).conf.errh,
                    b"src/h2.c\0" as *const u8 as *const libc::c_char,
                    967 as libc::c_int as libc::c_uint,
                    b"request-size too long: %lld -> 413\0" as *const u8
                        as *const libc::c_char,
                    ((*dst).bytes_in + alen as off_t) as libc::c_longlong,
                );
            } else {
                h2_send_rst_stream_id(id, con, H2_E_STREAM_CLOSED);
            }
            chunkqueue_mark_written(
                cq,
                (9 as libc::c_int as libc::c_uint).wrapping_add(len) as off_t,
            );
            return 1 as libc::c_int;
        }
    }
    h2_send_window_update(con, (*r).h2id, wupd);
    chunkqueue_mark_written(
        cq,
        (9 as libc::c_int
            + (if *s.offset(4 as libc::c_int as isize) as libc::c_int
                & H2_FLAG_PADDED as libc::c_int != 0
            {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            })) as off_t,
    );
    let c: *const chunk = (*dst).last;
    if !c.is_null()
        && (*c).type_0 as libc::c_uint == FILE_CHUNK as libc::c_int as libc::c_uint
        && (*c).file.is_temp != 0
        || chunkqueue_length(dst) + alen as libc::c_long
            > 65536 as libc::c_int as libc::c_long
    {
        let errh: *mut log_error_st = (*r).conf.errh;
        if 0 as libc::c_int
            != chunkqueue_steal_with_tempfiles(dst, cq, alen as off_t, errh)
        {
            h2_send_rst_stream(r, con, H2_E_INTERNAL_ERROR);
            return 0 as libc::c_int;
        }
    } else {
        chunkqueue_steal(dst, cq, alen as off_t);
    }
    if pad != 0 {
        chunkqueue_mark_written(cq, pad as off_t);
    }
    return 1 as libc::c_int;
}
#[cold]
unsafe extern "C" fn h2_frame_cq_compact(
    cq: *mut chunkqueue,
    mut len: uint32_t,
) -> uint32_t {
    chunkqueue_compact_mem(cq, len as size_t);
    return (buffer_clen((*(*cq).first).mem))
        .wrapping_sub((*(*cq).first).offset as uint32_t);
}
#[cold]
unsafe extern "C" fn h2_recv_continuation(
    mut n: uint32_t,
    mut clen: uint32_t,
    cqlen: off_t,
    cq: *mut chunkqueue,
    con: *mut connection,
) -> uint32_t {
    let mut c: *mut chunk = (*cq).first;
    let mut s: *mut uint8_t = ((*(*c).mem).ptr).offset((*c).offset as isize)
        as *mut uint8_t;
    let mut m: uint32_t = n;
    let mut flags: uint32_t = 0;
    let h2c: *mut h2con = (*con).h2;
    let fsize: uint32_t = (*h2c).s_max_frame_size;
    let id: uint32_t = h2_u31(s.offset(5 as libc::c_int as isize));
    loop {
        if cqlen < n.wrapping_add(9 as libc::c_int as libc::c_uint) as libc::c_long {
            return n.wrapping_add(9 as libc::c_int as libc::c_uint);
        }
        if clen < n.wrapping_add(9 as libc::c_int as libc::c_uint) {
            clen = h2_frame_cq_compact(
                cq,
                n.wrapping_add(9 as libc::c_int as libc::c_uint),
            );
            c = (*cq).first;
            s = ((*(*c).mem).ptr).offset((*c).offset as isize) as *mut uint8_t;
        }
        if *s.offset(n.wrapping_add(3 as libc::c_int as libc::c_uint) as isize)
            as libc::c_int != H2_FTYPE_CONTINUATION as libc::c_int
        {
            h2_send_goaway_e(con, H2_E_PROTOCOL_ERROR);
            return 0 as libc::c_int as uint32_t;
        }
        flags = *s.offset(n.wrapping_add(4 as libc::c_int as libc::c_uint) as isize)
            as uint32_t;
        let flen: uint32_t = h2_u24(s.offset(n as isize));
        if id != h2_u32(s.offset(n as isize).offset(5 as libc::c_int as isize)) {
            h2_send_goaway_e(con, H2_E_PROTOCOL_ERROR);
            return 0 as libc::c_int as uint32_t;
        }
        if flen > fsize {
            h2_send_goaway_e(con, H2_E_FRAME_SIZE_ERROR);
            return 0 as libc::c_int as uint32_t;
        }
        n = (n as libc::c_uint)
            .wrapping_add((9 as libc::c_int as libc::c_uint).wrapping_add(flen))
            as uint32_t as uint32_t;
        if n >= 65536 as libc::c_int as libc::c_uint {
            h2_send_goaway_e(con, H2_E_FRAME_SIZE_ERROR);
            return 0 as libc::c_int as uint32_t;
        }
        if clen < n {
            clen = h2_frame_cq_compact(cq, n);
            if clen < n {
                return n;
            }
            c = (*cq).first;
            s = ((*(*c).mem).ptr).offset((*c).offset as isize) as *mut uint8_t;
        }
        if !(flags & H2_FLAG_END_HEADERS as libc::c_int as libc::c_uint == 0) {
            break;
        }
    }
    n = m;
    if *s.offset(4 as libc::c_int as isize) as libc::c_int
        & H2_FLAG_PADDED as libc::c_int != 0
    {
        let plen: uint32_t = *s.offset(9 as libc::c_int as isize) as uint32_t;
        let flen_0: uint32_t = h2_u24(s);
        if flen_0
            < (1 as libc::c_int as libc::c_uint)
                .wrapping_add(plen)
                .wrapping_add(
                    (if *s
                        .offset(
                            n.wrapping_add(4 as libc::c_int as libc::c_uint) as isize,
                        ) as libc::c_int & H2_FLAG_PRIORITY as libc::c_int != 0
                    {
                        5 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as libc::c_uint,
                )
        {
            h2_send_goaway_e(con, H2_E_PROTOCOL_ERROR);
            return 0 as libc::c_int as uint32_t;
        }
        *s.offset(9 as libc::c_int as isize) = 0 as libc::c_int as uint8_t;
        m = (m as libc::c_uint).wrapping_sub(plen) as uint32_t as uint32_t;
        (*cq).bytes_out += plen as libc::c_long;
    }
    loop {
        let flen_1: uint32_t = h2_u24(s.offset(n as isize));
        flags = *s.offset(n.wrapping_add(4 as libc::c_int as libc::c_uint) as isize)
            as uint32_t;
        memmove(
            s.offset(m as isize) as *mut libc::c_void,
            s.offset(n as isize).offset(9 as libc::c_int as isize)
                as *const libc::c_void,
            flen_1 as libc::c_ulong,
        );
        m = (m as libc::c_uint).wrapping_add(flen_1) as uint32_t as uint32_t;
        n = (n as libc::c_uint)
            .wrapping_add((9 as libc::c_int as libc::c_uint).wrapping_add(flen_1))
            as uint32_t as uint32_t;
        (*cq).bytes_out += 9 as libc::c_int as libc::c_long;
        if !(flags & H2_FLAG_END_HEADERS as libc::c_int as libc::c_uint == 0) {
            break;
        }
    }
    m = (m as libc::c_uint).wrapping_sub(9 as libc::c_int as libc::c_uint) as uint32_t
        as uint32_t;
    *s
        .offset(
            0 as libc::c_int as isize,
        ) = (m >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t;
    *s
        .offset(
            1 as libc::c_int as isize,
        ) = (m >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t;
    *s
        .offset(
            2 as libc::c_int as isize,
        ) = (m & 0xff as libc::c_int as libc::c_uint) as uint8_t;
    m = (m as libc::c_uint).wrapping_add(9 as libc::c_int as libc::c_uint) as uint32_t
        as uint32_t;
    if n < clen {
        memmove(
            s.offset(m as isize) as *mut libc::c_void,
            s.offset(n as isize) as *const libc::c_void,
            clen.wrapping_sub(n) as libc::c_ulong,
        );
        n = m.wrapping_add(clen.wrapping_sub(n));
    } else {
        n = m;
    }
    buffer_truncate((*c).mem, n.wrapping_add((*c).offset as uint32_t));
    return m;
}
#[cold]
unsafe extern "C" fn h2_recv_trailers_r(
    con: *mut connection,
    h2c: *mut h2con,
    id: uint32_t,
    flags: uint32_t,
) -> *mut request_st {
    let mut r: *mut request_st = 0 as *mut request_st;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let mut rused: uint32_t = (*h2c).rused;
    while i < rused {
        let rr: *mut request_st = (*h2c).r[i as usize];
        if (*rr).h2id != id {
            i = i.wrapping_add(1);
        } else {
            r = rr;
            break;
        }
    }
    if r.is_null() {
        h2_send_goaway_e(con, H2_E_PROTOCOL_ERROR);
        return 0 as *mut request_st;
    }
    if (*r).h2state != H2_STATE_OPEN as libc::c_int as libc::c_uint
        && (*r).h2state != H2_STATE_HALF_CLOSED_LOCAL as libc::c_int as libc::c_uint
    {
        h2_send_rst_stream(r, con, H2_E_STREAM_CLOSED);
        return 0 as *mut request_st;
    }
    if flags & H2_FLAG_END_STREAM as libc::c_int as libc::c_uint == 0 {
        h2_send_rst_stream(r, con, H2_E_PROTOCOL_ERROR);
        return 0 as *mut request_st;
    }
    return if h2_recv_end_data(r, con, 0 as libc::c_int as uint32_t) != 0 {
        r
    } else {
        0 as *mut request_st
    };
}
unsafe extern "C" fn h2_parse_headers_frame(
    r: *mut request_st,
    mut psrc: *const libc::c_uchar,
    plen: uint32_t,
    trailers: libc::c_int,
) {
    let h2c: *mut h2con = (*(*r).con).h2;
    let decoder: *mut lshpack_dec = &mut (*h2c).decoder;
    let endp: *const libc::c_uchar = psrc.offset(plen as isize);
    let mut hpctx: http_header_parse_ctx = http_header_parse_ctx {
        k: 0 as *mut libc::c_char,
        v: 0 as *mut libc::c_char,
        klen: 0,
        vlen: 0,
        hlen: 0,
        pseudo: 0,
        scheme: 0,
        trailers: 0,
        id: 0,
        max_request_field_size: 0,
        http_parseopts: 0,
    };
    hpctx.hlen = 0 as libc::c_int as uint32_t;
    hpctx.pseudo = 1 as libc::c_int as uint8_t;
    hpctx.scheme = 0 as libc::c_int as uint8_t;
    hpctx.trailers = trailers as uint8_t;
    hpctx.max_request_field_size = (*r).conf.max_request_field_size;
    hpctx.http_parseopts = (*r).conf.http_parseopts;
    let log_request_header: libc::c_int = (*r).conf.log_request_header as libc::c_int;
    let mut rc: libc::c_int = 0 as libc::c_int;
    let tb: *mut buffer = (*r).tmp_buf;
    if !((*tb).size >= 65536 as libc::c_int as libc::c_uint) {
        ck_assert_failed(
            b"src/h2.c\0" as *const u8 as *const libc::c_char,
            1234 as libc::c_int as libc::c_uint,
            b"tb->size >= 65536\0" as *const u8 as *const libc::c_char,
        );
    }
    let tbptr: *mut libc::c_char = (*tb).ptr;
    let tbsz: lsxpack_strlen_t = (if (*tb).size <= 65535 as libc::c_int as libc::c_uint {
        (*tb).size
    } else {
        65535 as libc::c_int as libc::c_uint
    }) as lsxpack_strlen_t;
    let mut lsx: lsxpack_header_t = lsxpack_header_t {
        buf: 0 as *mut libc::c_char,
        name_hash: 0,
        nameval_hash: 0,
        name_offset: 0,
        name_len: 0,
        val_offset: 0,
        val_len: 0,
        chain_next_idx: 0,
        hpack_index: 0,
        qpack_index: 0,
        app_index: 0,
        flags: [0; 1],
        indexed_type: 0,
        dec_overhead: 0,
    };
    while psrc < endp {
        memset(
            &mut lsx as *mut lsxpack_header_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<lsxpack_header_t>() as libc::c_ulong,
        );
        lsx.buf = tbptr;
        lsx.val_len = tbsz;
        rc = lshpack_dec_decode(decoder, &mut psrc, endp, &mut lsx);
        if 0 as libc::c_int == lsx.name_len as libc::c_int {
            rc = -(1 as libc::c_int);
        }
        if (rc == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
            hpctx.k = (lsx.buf).offset(lsx.name_offset as libc::c_int as isize);
            hpctx.v = (lsx.buf).offset(lsx.val_offset as libc::c_int as isize);
            hpctx.klen = lsx.name_len as uint32_t;
            hpctx.vlen = lsx.val_len as uint32_t;
            hpctx.id = lshpack_idx_http_header[lsx.hpack_index as usize];
            if log_request_header != 0 {
                log_error(
                    (*r).conf.errh,
                    b"src/h2.c\0" as *const u8 as *const libc::c_char,
                    1264 as libc::c_int as libc::c_uint,
                    b"fd:%d id:%u rqst: %.*s: %.*s\0" as *const u8
                        as *const libc::c_char,
                    (*(*r).con).fd,
                    (*r).h2id,
                    hpctx.klen as libc::c_int,
                    hpctx.k,
                    hpctx.vlen as libc::c_int,
                    hpctx.v,
                );
            }
            let http_status: libc::c_int = http_request_parse_header(r, &mut hpctx);
            if !((0 as libc::c_int != http_status) as libc::c_int as libc::c_long != 0) {
                continue;
            }
            if (*r).http_status == 0 as libc::c_int {
                (*r).http_status = http_status;
            }
            break;
        } else {
            let mut err: request_h2error_t = H2_E_COMPRESSION_ERROR;
            if rc != -(1 as libc::c_int) {
                err = H2_E_PROTOCOL_ERROR;
                h2_send_rst_stream(r, (*r).con, err);
            }
            if (*h2c).sent_goaway == 0 && trailers == 0 {
                (*h2c).h2_cid = (*r).h2id;
            }
            h2_send_goaway_e((*r).con, err);
            if trailers == 0 {
                h2_retire_stream(r, (*r).con);
                return;
            } else {
                (*r).state = CON_STATE_ERROR;
                (*r).h2state = H2_STATE_CLOSED as libc::c_int as uint32_t;
            }
            break;
        }
    }
    hpctx
        .hlen = (hpctx.hlen as libc::c_uint)
        .wrapping_add(2 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
    (*r)
        .rqst_header_len = ((*r).rqst_header_len as libc::c_uint)
        .wrapping_add(hpctx.hlen) as uint32_t as uint32_t;
    let rq: *mut chunkqueue = &mut (*r).read_queue;
    (*rq).bytes_in += hpctx.hlen as off_t;
    (*rq).bytes_out += hpctx.hlen as off_t;
    if 0 as libc::c_int == (*r).http_status && 0 as libc::c_int == rc && trailers == 0 {
        if hpctx.pseudo != 0 {
            (*r)
                .http_status = http_request_validate_pseudohdrs(
                r,
                hpctx.scheme as libc::c_int,
                hpctx.http_parseopts,
            );
        }
        if 0 as libc::c_int == (*r).http_status {
            http_request_headers_process_h2(
                r,
                (*(*r).con).proto_default_port as libc::c_int,
            );
        }
    }
}
unsafe extern "C" fn h2_recv_headers(
    con: *mut connection,
    s: *mut uint8_t,
    mut flen: uint32_t,
) -> libc::c_int {
    let mut r: *mut request_st = 0 as *mut request_st;
    let h2c: *mut h2con = (*con).h2;
    let id: uint32_t = h2_u31(s.offset(5 as libc::c_int as isize));
    if id & 1 as libc::c_int as libc::c_uint == 0 {
        h2_send_goaway_e(con, H2_E_PROTOCOL_ERROR);
        return 0 as libc::c_int;
    }
    let h2r: *mut request_st = &mut (*con).request;
    let mut trailers: libc::c_int = 0 as libc::c_int;
    if id > (*h2c).h2_cid {
        if (*h2c).rused as libc::c_ulong
            == (::std::mem::size_of::<[*mut request_st; 8]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<*mut request_st>() as libc::c_ulong)
        {
            return h2_send_refused_stream(id, con);
        }
        r = h2_init_stream(h2r, con);
        (*r).h2id = id;
        (*r)
            .h2state = (if *s.offset(4 as libc::c_int as isize) as libc::c_int
            & H2_FLAG_END_STREAM as libc::c_int != 0
        {
            H2_STATE_HALF_CLOSED_REMOTE as libc::c_int
        } else {
            H2_STATE_OPEN as libc::c_int
        }) as uint32_t;
        (*r).state = CON_STATE_REQUEST_END;
        (*r).start_hp.tv_sec = log_epoch_secs;
        if (*r).conf.high_precision_timestamps != 0 {
            clock_gettime(0 as libc::c_int, &mut (*r).start_hp);
        }
    } else {
        r = h2_recv_trailers_r(
            con,
            h2c,
            id,
            *s.offset(4 as libc::c_int as isize) as uint32_t,
        );
        if r.is_null() {
            return if (*h2c).sent_goaway > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                1 as libc::c_int
            };
        }
        trailers = 1 as libc::c_int;
    }
    let mut psrc: *const libc::c_uchar = s.offset(9 as libc::c_int as isize);
    let mut alen: uint32_t = flen;
    if *s.offset(4 as libc::c_int as isize) as libc::c_int
        & H2_FLAG_PADDED as libc::c_int != 0
    {
        psrc = psrc.offset(1);
        let pad: uint32_t = *s.offset(9 as libc::c_int as isize) as uint32_t;
        if flen
            < (1 as libc::c_int as libc::c_uint)
                .wrapping_add(pad)
                .wrapping_add(
                    (if *s.offset(4 as libc::c_int as isize) as libc::c_int
                        & H2_FLAG_PRIORITY as libc::c_int != 0
                    {
                        5 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as libc::c_uint,
                )
        {
            h2_send_goaway_e(con, H2_E_PROTOCOL_ERROR);
            if trailers == 0 {
                h2_retire_stream(r, con);
            } else {
                (*r).state = CON_STATE_ERROR;
                (*r).h2state = H2_STATE_CLOSED as libc::c_int as uint32_t;
            }
            return 0 as libc::c_int;
        }
        alen = (alen as libc::c_uint)
            .wrapping_sub((1 as libc::c_int as libc::c_uint).wrapping_add(pad))
            as uint32_t as uint32_t;
    }
    if *s.offset(4 as libc::c_int as isize) as libc::c_int
        & H2_FLAG_PRIORITY as libc::c_int != 0
    {
        let prio: uint32_t = h2_u31(psrc);
        if prio == id {
            h2_send_rst_stream(r, con, H2_E_PROTOCOL_ERROR);
            if trailers == 0 {
                h2_retire_stream(r, con);
            }
            return 1 as libc::c_int;
        }
        psrc = psrc.offset(5 as libc::c_int as isize);
        alen = (alen as libc::c_uint).wrapping_sub(5 as libc::c_int as libc::c_uint)
            as uint32_t as uint32_t;
    }
    h2_parse_headers_frame(r, psrc, alen, trailers);
    if (*h2c).sent_goaway == 0 {
        (*h2c).h2_cid = id;
        if (*r).rqst_htags
            & (1 as libc::c_ulong) << HTTP_HEADER_CONTENT_LENGTH as libc::c_int == 0
        {
            (*r)
                .reqbody_length = (if *s.offset(4 as libc::c_int as isize) as libc::c_int
                & H2_FLAG_END_STREAM as libc::c_int != 0
            {
                0 as libc::c_int
            } else {
                -(1 as libc::c_int)
            }) as off_t;
        }
    } else if (*h2c).h2_cid < id {
        (*r).http_status = 0 as libc::c_int;
        h2_retire_stream(r, con);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn h2_parse_frames(con: *mut connection) -> libc::c_int {
    let h2c: *mut h2con = (*con).h2;
    let cq: *mut chunkqueue = (*con).read_queue;
    let fsize: uint32_t = (*h2c).s_max_frame_size;
    let mut cqlen: off_t = chunkqueue_length(cq);
    while cqlen >= 9 as libc::c_int as libc::c_long {
        let mut c: *mut chunk = (*cq).first;
        let mut clen: uint32_t = (buffer_clen((*c).mem) as libc::c_long - (*c).offset)
            as uint32_t;
        if clen < 9 as libc::c_int as libc::c_uint {
            clen = h2_frame_cq_compact(cq, 9 as libc::c_int as uint32_t);
            c = (*cq).first;
        }
        let mut s: *mut uint8_t = ((*(*c).mem).ptr).offset((*c).offset as isize)
            as *mut uint8_t;
        let mut flen: uint32_t = h2_u24(s);
        if flen > fsize {
            h2_send_goaway_e(con, H2_E_FRAME_SIZE_ERROR);
            return 0 as libc::c_int;
        }
        if *s.offset(3 as libc::c_int as isize) as libc::c_int
            == H2_FTYPE_HEADERS as libc::c_int
        {
            if cqlen
                < (9 as libc::c_int as libc::c_uint).wrapping_add(flen) as libc::c_long
            {
                return 1 as libc::c_int;
            }
            if clen < (9 as libc::c_int as libc::c_uint).wrapping_add(flen) {
                clen = h2_frame_cq_compact(
                    cq,
                    (9 as libc::c_int as libc::c_uint).wrapping_add(flen),
                );
                c = (*cq).first;
                s = ((*(*c).mem).ptr).offset((*c).offset as isize) as *mut uint8_t;
            }
            if *s.offset(4 as libc::c_int as isize) as libc::c_int
                & H2_FLAG_END_HEADERS as libc::c_int == 0
            {
                clen = h2_recv_continuation(
                    (9 as libc::c_int as libc::c_uint).wrapping_add(flen),
                    clen,
                    cqlen,
                    cq,
                    con,
                );
                if 0 as libc::c_int as libc::c_uint == clen {
                    return 0 as libc::c_int;
                }
                if cqlen < clen as libc::c_long {
                    return 1 as libc::c_int;
                }
                c = (*cq).first;
                s = ((*(*c).mem).ptr).offset((*c).offset as isize) as *mut uint8_t;
                flen = h2_u24(s);
                cqlen = chunkqueue_length(cq);
            }
            let mut rc: libc::c_int = h2_recv_headers(con, s, flen);
            cqlen
                -= (9 as libc::c_int as libc::c_uint).wrapping_add(flen) as libc::c_long;
            if rc >= 0 as libc::c_int {
                chunkqueue_mark_written(
                    cq,
                    (9 as libc::c_int as libc::c_uint).wrapping_add(flen) as off_t,
                );
            }
            if rc <= 0 as libc::c_int {
                return 0 as libc::c_int;
            }
            (*con).read_idle_ts = log_monotonic_secs;
        } else if *s.offset(3 as libc::c_int as isize) as libc::c_int
                == H2_FTYPE_DATA as libc::c_int
            {
            if cqlen
                < (9 as libc::c_int as libc::c_uint).wrapping_add(flen) as libc::c_long
            {
                return 1 as libc::c_int;
            }
            (*con).read_idle_ts = log_monotonic_secs;
            if h2_recv_data(con, s, flen) == 0 {
                return 0 as libc::c_int;
            }
            cqlen
                -= (9 as libc::c_int as libc::c_uint).wrapping_add(flen) as libc::c_long;
        } else {
            if cqlen
                < (9 as libc::c_int as libc::c_uint).wrapping_add(flen) as libc::c_long
            {
                return 1 as libc::c_int;
            }
            if clen < (9 as libc::c_int as libc::c_uint).wrapping_add(flen) {
                clen = h2_frame_cq_compact(
                    cq,
                    (9 as libc::c_int as libc::c_uint).wrapping_add(flen),
                );
                c = (*cq).first;
                s = ((*(*c).mem).ptr).offset((*c).offset as isize) as *mut uint8_t;
            }
            match *s.offset(3 as libc::c_int as isize) as libc::c_int {
                8 => {
                    h2_recv_window_update(con, s, flen);
                }
                2 => {
                    h2_recv_priority(con, s, flen);
                }
                4 => {
                    h2_recv_settings(con, s, flen);
                }
                6 => {
                    h2_recv_ping(con, s, flen);
                }
                3 => {
                    h2_recv_rst_stream(con, s, flen);
                }
                7 => {
                    if h2_recv_goaway(con, s, flen) == 0 {
                        return 0 as libc::c_int;
                    }
                }
                5 | 9 => {
                    h2_send_goaway_e(con, H2_E_PROTOCOL_ERROR);
                    return 0 as libc::c_int;
                }
                _ => {}
            }
            cqlen
                -= (9 as libc::c_int as libc::c_uint).wrapping_add(flen) as libc::c_long;
            chunkqueue_mark_written(
                cq,
                (9 as libc::c_int as libc::c_uint).wrapping_add(flen) as off_t,
            );
        }
        if (*h2c).sent_goaway > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn h2_want_read(con: *mut connection) -> libc::c_int {
    let cq: *mut chunkqueue = (*con).read_queue;
    if chunkqueue_is_empty(cq) != 0 {
        return 1 as libc::c_int;
    }
    let cqlen: off_t = chunkqueue_length(cq);
    if cqlen < 9 as libc::c_int as libc::c_long {
        return 1 as libc::c_int;
    }
    let mut c: *mut chunk = (*cq).first;
    let mut clen: uint32_t = (buffer_clen((*c).mem) as libc::c_long - (*c).offset)
        as uint32_t;
    if clen < 9 as libc::c_int as libc::c_uint {
        clen = h2_frame_cq_compact(cq, 9 as libc::c_int as uint32_t);
        c = (*cq).first;
    }
    let mut s: *mut uint8_t = ((*(*c).mem).ptr).offset((*c).offset as isize)
        as *mut uint8_t;
    let mut flen: uint32_t = h2_u24(s);
    if clen < (9 as libc::c_int as libc::c_uint).wrapping_add(flen) {
        return 1 as libc::c_int;
    }
    if *s.offset(3 as libc::c_int as isize) as libc::c_int
        != H2_FTYPE_HEADERS as libc::c_int
        || *s.offset(4 as libc::c_int as isize) as libc::c_int
            & H2_FLAG_END_HEADERS as libc::c_int != 0
    {
        return 0 as libc::c_int;
    }
    let mut n: uint32_t = (9 as libc::c_int as libc::c_uint).wrapping_add(flen);
    while cqlen >= n.wrapping_add(9 as libc::c_int as libc::c_uint) as libc::c_long {
        if clen < n.wrapping_add(9 as libc::c_int as libc::c_uint) {
            clen = h2_frame_cq_compact(
                cq,
                n.wrapping_add(9 as libc::c_int as libc::c_uint),
            );
            c = (*cq).first;
            s = ((*(*c).mem).ptr).offset((*c).offset as isize) as *mut uint8_t;
        }
        flen = h2_u24(s.offset(n as isize));
        if cqlen
            < n.wrapping_add(9 as libc::c_int as libc::c_uint).wrapping_add(flen)
                as libc::c_long
        {
            return 1 as libc::c_int;
        }
        if *s.offset(4 as libc::c_int as isize) as libc::c_int
            & H2_FLAG_END_HEADERS as libc::c_int != 0
        {
            return 0 as libc::c_int;
        }
        n = (n as libc::c_uint)
            .wrapping_add((9 as libc::c_int as libc::c_uint).wrapping_add(flen))
            as uint32_t as uint32_t;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn h2_recv_client_connection_preface(
    con: *mut connection,
) -> libc::c_int {
    let cq: *mut chunkqueue = (*con).read_queue;
    if chunkqueue_length(cq) < 24 as libc::c_int as libc::c_long {
        let c: *mut chunk = (*cq).first;
        if !c.is_null()
            && buffer_clen((*c).mem) as libc::c_long - (*c).offset
                >= 4 as libc::c_int as libc::c_long
        {
            let s: *const libc::c_char = ((*(*c).mem).ptr).offset((*c).offset as isize);
            if *s.offset(0 as libc::c_int as isize) as libc::c_int != 'P' as i32
                || *s.offset(1 as libc::c_int as isize) as libc::c_int != 'R' as i32
                || *s.offset(2 as libc::c_int as isize) as libc::c_int != 'I' as i32
                || *s.offset(3 as libc::c_int as isize) as libc::c_int != ' ' as i32
            {
                h2_send_goaway_e(con, H2_E_PROTOCOL_ERROR);
                return 1 as libc::c_int;
            }
        }
        return 0 as libc::c_int;
    }
    static mut h2preface: [libc::c_char; 25] = unsafe {
        *::std::mem::transmute::<
            &[u8; 25],
            &[libc::c_char; 25],
        >(b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n\0")
    };
    let mut c_0: *mut chunk = (*cq).first;
    let clen: uint32_t = (buffer_clen((*c_0).mem) as libc::c_long - (*c_0).offset)
        as uint32_t;
    if clen < 24 as libc::c_int as libc::c_uint {
        h2_frame_cq_compact(cq, 24 as libc::c_int as uint32_t);
    }
    c_0 = (*cq).first;
    let s_0: *const uint8_t = ((*(*c_0).mem).ptr).offset((*c_0).offset as isize)
        as *mut uint8_t;
    if 0 as libc::c_int
        == memcmp(
            s_0 as *const libc::c_void,
            h2preface.as_ptr() as *const libc::c_void,
            24 as libc::c_int as libc::c_ulong,
        )
    {
        chunkqueue_mark_written(cq, 24 as libc::c_int as off_t);
    } else {
        h2_send_goaway_e(con, H2_E_PROTOCOL_ERROR);
    }
    return 1 as libc::c_int;
}
#[cold]
unsafe extern "C" fn h2_read_client_connection_preface(
    con: *mut connection,
    cq: *mut chunkqueue,
    mut max_bytes: off_t,
) -> libc::c_int {
    let hctx: *mut *mut libc::c_void = ((*con).plugin_ctx)
        .offset(0 as libc::c_int as isize);
    let network_read: Option::<
        unsafe extern "C" fn(*mut connection, *mut chunkqueue, off_t) -> libc::c_int,
    > = ::std::mem::transmute::<
        libc::intptr_t,
        Option::<
            unsafe extern "C" fn(*mut connection, *mut chunkqueue, off_t) -> libc::c_int,
        >,
    >(*hctx as uintptr_t as libc::intptr_t);
    if max_bytes < 24 as libc::c_int as libc::c_long {
        max_bytes = 24 as libc::c_int as off_t;
    }
    let mut rc: libc::c_int = network_read
        .expect("non-null function pointer")(con, cq, max_bytes);
    if ((*con).h2).is_null() {
        return rc;
    }
    if -(1 as libc::c_int) != rc && h2_recv_client_connection_preface(con) != 0 {
        (*con).network_read = network_read;
        *hctx = 0 as *mut libc::c_void;
        (*con).read_idle_ts = log_monotonic_secs;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn h2_init_con(
    h2r: *mut request_st,
    con: *mut connection,
    http2_settings: *const buffer,
) {
    let h2c: *mut h2con = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<h2con>() as libc::c_ulong,
    ) as *mut h2con;
    if h2c.is_null() {
        ck_assert_failed(
            b"src/h2.c\0" as *const u8 as *const libc::c_char,
            1740 as libc::c_int as libc::c_uint,
            b"h2c\0" as *const u8 as *const libc::c_char,
        );
    }
    (*con).h2 = h2c;
    (*con).read_idle_ts = log_monotonic_secs;
    (*con).keep_alive_idle = (*h2r).conf.max_keep_alive_idle as libc::c_int;
    (*h2r).h2_rwin = 65535 as libc::c_int;
    (*h2r).h2_swin = 65535 as libc::c_int;
    (*h2c).s_header_table_size = 4096 as libc::c_int as uint32_t;
    (*h2c).s_enable_push = 1 as libc::c_int as uint32_t;
    (*h2c).s_max_concurrent_streams = !(0 as libc::c_uint);
    (*h2c).s_initial_window_size = 65535 as libc::c_int;
    (*h2c).s_max_frame_size = 16384 as libc::c_int as uint32_t;
    (*h2c).s_max_header_list_size = !(0 as libc::c_uint);
    (*h2c).sent_settings = log_monotonic_secs;
    lshpack_dec_init(&mut (*h2c).decoder);
    lshpack_enc_init(&mut (*h2c).encoder);
    lshpack_enc_use_hist(&mut (*h2c).encoder, 1 as libc::c_int);
    if !http2_settings.is_null() {
        h2_parse_frame_settings(
            con,
            (*http2_settings).ptr as *mut uint8_t,
            buffer_clen(http2_settings),
        );
    }
    static mut h2settings: [uint8_t; 21] = [
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0xc as libc::c_int as uint8_t,
        H2_FTYPE_SETTINGS as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        H2_SETTINGS_MAX_CONCURRENT_STREAMS as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0x8 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        H2_SETTINGS_MAX_HEADER_LIST_SIZE as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0xff as libc::c_int as uint8_t,
        0xff as libc::c_int as uint8_t,
    ];
    chunkqueue_append_mem(
        (*con).write_queue,
        h2settings.as_ptr() as *const libc::c_char,
        ::std::mem::size_of::<[uint8_t; 21]>() as libc::c_ulong,
    );
    if h2_recv_client_connection_preface(con) == 0 {
        let ref mut fresh1 = *((*con).plugin_ctx).offset(0 as libc::c_int as isize);
        *fresh1 = ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut connection,
                    *mut chunkqueue,
                    off_t,
                ) -> libc::c_int,
            >,
            uintptr_t,
        >((*con).network_read) as *mut libc::c_void;
        (*con)
            .network_read = Some(
            h2_read_client_connection_preface
                as unsafe extern "C" fn(
                    *mut connection,
                    *mut chunkqueue,
                    off_t,
                ) -> libc::c_int,
        );
    }
    buffer_string_prepare_copy((*h2r).tmp_buf, 65535 as libc::c_int as size_t);
}
unsafe extern "C" fn h2_send_hpack(
    r: *mut request_st,
    con: *mut connection,
    mut data: *const libc::c_char,
    mut dlen: uint32_t,
    flags: uint32_t,
) {
    let mut headers: C2RustUnnamed_13 = C2RustUnnamed_13 {
        c: [
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            H2_FTYPE_HEADERS as libc::c_int as uint8_t,
            flags as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
        ],
    };
    headers.u[2 as libc::c_int as usize] = __bswap_32((*r).h2id);
    let b: *mut buffer = chunkqueue_append_buffer_open_sz(
        (*con).write_queue,
        dlen
            .wrapping_add(dlen >> 10 as libc::c_int)
            .wrapping_add(9 as libc::c_int as libc::c_uint) as size_t,
    );
    let mut ptr: *mut libc::c_char = (*b).ptr;
    let h2c: *mut h2con = (*con).h2;
    let fsize: uint32_t = (*h2c).s_max_frame_size;
    loop {
        let len: uint32_t = if dlen < fsize { dlen } else { fsize };
        headers
            .c[3 as libc::c_int
            as usize] = (len >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        headers
            .c[4 as libc::c_int
            as usize] = (len >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        headers
            .c[5 as libc::c_int
            as usize] = (len & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        if len == dlen {
            headers
                .c[7 as libc::c_int
                as usize] = (headers.c[7 as libc::c_int as usize] as libc::c_int
                | H2_FLAG_END_HEADERS as libc::c_int) as uint8_t;
        }
        memcpy(
            ptr as *mut libc::c_void,
            (headers.c).as_mut_ptr().offset(3 as libc::c_int as isize)
                as *const libc::c_void,
            (::std::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong)
                .wrapping_sub(3 as libc::c_int as libc::c_ulong),
        );
        memcpy(
            ptr
                .offset(
                    ::std::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong as isize,
                )
                .offset(-(3 as libc::c_int as isize)) as *mut libc::c_void,
            data as *const libc::c_void,
            len as libc::c_ulong,
        );
        ptr = ptr
            .offset(
                (len as libc::c_ulong)
                    .wrapping_add(
                        ::std::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong,
                    )
                    .wrapping_sub(3 as libc::c_int as libc::c_ulong) as isize,
            );
        data = data.offset(len as isize);
        dlen = (dlen as libc::c_uint).wrapping_sub(len) as uint32_t as uint32_t;
        headers
            .c[6 as libc::c_int
            as usize] = H2_FTYPE_CONTINUATION as libc::c_int as uint8_t;
        headers.c[7 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
        if !(dlen != 0) {
            break;
        }
    }
    buffer_truncate(b, ptr.offset_from((*b).ptr) as libc::c_long as uint32_t);
    chunkqueue_append_buffer_commit((*con).write_queue);
}
#[cold]
unsafe extern "C" fn h2_log_response_header_lsx(
    r: *mut request_st,
    lsx: *const lsxpack_header_t,
) {
    log_error(
        (*r).conf.errh,
        b"src/h2.c\0" as *const u8 as *const libc::c_char,
        1883 as libc::c_int as libc::c_uint,
        b"fd:%d id:%u resp: %.*s: %.*s\0" as *const u8 as *const libc::c_char,
        (*(*r).con).fd,
        (*r).h2id,
        (*lsx).name_len as libc::c_int,
        ((*lsx).buf).offset((*lsx).name_offset as libc::c_int as isize),
        (*lsx).val_len as libc::c_int,
        ((*lsx).buf).offset((*lsx).val_offset as libc::c_int as isize),
    );
}
#[cold]
unsafe extern "C" fn h2_log_response_header(
    r: *mut request_st,
    len: libc::c_int,
    hdr: *const libc::c_char,
) {
    log_error(
        (*r).conf.errh,
        b"src/h2.c\0" as *const u8 as *const libc::c_char,
        1894 as libc::c_int as libc::c_uint,
        b"fd:%d id:%u resp: %.*s\0" as *const u8 as *const libc::c_char,
        (*(*r).con).fd,
        (*r).h2id,
        len,
        hdr,
    );
}
#[no_mangle]
pub unsafe extern "C" fn h2_send_headers(r: *mut request_st, con: *mut connection) {
    (*con).keep_alive_idle = (*r).conf.max_keep_alive_idle as libc::c_int;
    if 304 as libc::c_int == (*r).http_status
        && (*r).resp_htags
            & (1 as libc::c_ulong) << HTTP_HEADER_CONTENT_ENCODING as libc::c_int != 0
    {
        http_header_response_unset(
            r,
            HTTP_HEADER_CONTENT_ENCODING,
            b"Content-Encoding\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    let tb: *mut buffer = (*r).tmp_buf;
    if !((*tb).size >= 65536 as libc::c_int as libc::c_uint) {
        ck_assert_failed(
            b"src/h2.c\0" as *const u8 as *const libc::c_char,
            1916 as libc::c_int as libc::c_uint,
            b"tb->size >= 65536\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut dst: *mut libc::c_uchar = (*tb).ptr as *mut libc::c_uchar;
    let dst_end: *mut libc::c_uchar = ((*tb).ptr as *mut libc::c_uchar)
        .offset((*tb).size as isize);
    let h2c: *mut h2con = (*con).h2;
    let encoder: *mut lshpack_enc = &mut (*h2c).encoder;
    let mut lsx: lsxpack_header_t = lsxpack_header_t {
        buf: 0 as *mut libc::c_char,
        name_hash: 0,
        nameval_hash: 0,
        name_offset: 0,
        name_len: 0,
        val_offset: 0,
        val_len: 0,
        chain_next_idx: 0,
        hpack_index: 0,
        qpack_index: 0,
        app_index: 0,
        flags: [0; 1],
        indexed_type: 0,
        dec_overhead: 0,
    };
    let mut alen: uint32_t = (7 as libc::c_int + 3 as libc::c_int + 4 as libc::c_int)
        as uint32_t;
    let log_response_header: libc::c_int = (*r).conf.log_response_header as libc::c_int;
    let resp_header_repeated: libc::c_int = (*r).resp_header_repeated as libc::c_int;
    let mut status: [libc::c_char; 12] = *::std::mem::transmute::<
        &[u8; 12],
        &mut [libc::c_char; 12],
    >(b":status: 200");
    memset(
        &mut lsx as *mut lsxpack_header_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<lsxpack_header_t>() as libc::c_ulong,
    );
    lsx.buf = status.as_mut_ptr();
    lsx.name_offset = 0 as libc::c_int as lsxpack_strlen_t;
    lsx.name_len = 7 as libc::c_int as lsxpack_strlen_t;
    lsx.val_offset = 9 as libc::c_int as lsxpack_strlen_t;
    lsx.val_len = 3 as libc::c_int as lsxpack_strlen_t;
    if (200 as libc::c_int == (*r).http_status) as libc::c_int as libc::c_long != 0 {
        lsx.hpack_index = LSHPACK_HDR_STATUS_200 as libc::c_int as uint8_t;
    } else {
        let mut x: libc::c_int = (*r).http_status;
        match x {
            204 => {
                lsx.hpack_index = LSHPACK_HDR_STATUS_204 as libc::c_int as uint8_t;
            }
            206 => {
                lsx.hpack_index = LSHPACK_HDR_STATUS_206 as libc::c_int as uint8_t;
            }
            304 => {
                lsx.hpack_index = LSHPACK_HDR_STATUS_304 as libc::c_int as uint8_t;
            }
            400 => {
                lsx.hpack_index = LSHPACK_HDR_STATUS_400 as libc::c_int as uint8_t;
            }
            404 => {
                lsx.hpack_index = LSHPACK_HDR_STATUS_404 as libc::c_int as uint8_t;
            }
            500 => {
                lsx.hpack_index = LSHPACK_HDR_STATUS_500 as libc::c_int as uint8_t;
            }
            _ => {}
        }
        let mut nx: libc::c_int = 0;
        nx = x / 10 as libc::c_int;
        status[11 as libc::c_int
            as usize] = (status[11 as libc::c_int as usize] as libc::c_int
            + (x - nx * 10 as libc::c_int)) as libc::c_char;
        x = nx;
        nx = x / 10 as libc::c_int;
        status[10 as libc::c_int
            as usize] = (status[10 as libc::c_int as usize] as libc::c_int
            + (x - nx * 10 as libc::c_int)) as libc::c_char;
        status[9 as libc::c_int as usize] = ('0' as i32 + nx) as libc::c_char;
    }
    dst = lshpack_enc_encode(encoder, dst, dst_end, &mut lsx);
    if dst == (*tb).ptr as *mut libc::c_uchar {
        h2_send_rst_stream(r, con, H2_E_INTERNAL_ERROR);
        return;
    }
    if log_response_header != 0 {
        h2_log_response_header(r, 12 as libc::c_int, status.as_mut_ptr());
    }
    let hdata: *const *mut data_string = (*r).resp_headers.data
        as *const *mut data_string;
    let mut current_block_67: u64;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let mut used: uint32_t = (*r).resp_headers.used;
    while i < used {
        let ds: *mut data_string = *hdata.offset(i as isize);
        let klen: uint32_t = buffer_clen(&mut (*ds).key);
        let vlen: uint32_t = buffer_clen(&mut (*ds).value);
        if !((0 as libc::c_int as libc::c_uint == klen) as libc::c_int as libc::c_long
            != 0)
        {
            if !((0 as libc::c_int as libc::c_uint == vlen) as libc::c_int
                as libc::c_long != 0)
            {
                alen = (alen as libc::c_uint)
                    .wrapping_add(
                        klen
                            .wrapping_add(vlen)
                            .wrapping_add(4 as libc::c_int as libc::c_uint),
                    ) as uint32_t as uint32_t;
                if alen > 65535 as libc::c_int as libc::c_uint {
                    h2_send_rst_stream(r, con, H2_E_INTERNAL_ERROR);
                    return;
                }
                let v: *mut libc::c_char = if (buffer_string_space(&mut (*ds).value)
                    >= klen) as libc::c_int as libc::c_long != 0
                {
                    ((*ds).value.ptr)
                        .offset(vlen as isize)
                        .offset(1 as libc::c_int as isize)
                } else {
                    (buffer_string_prepare_append(&mut (*ds).value, klen as size_t))
                        .offset(1 as libc::c_int as isize)
                };
                if ((*ds).ext != HTTP_HEADER_OTHER as libc::c_int) as libc::c_int
                    as libc::c_long != 0
                {
                    memcpy(
                        v as *mut libc::c_void,
                        (http_header_lc[(*ds).ext as usize]).as_ptr()
                            as *const libc::c_void,
                        klen as libc::c_ulong,
                    );
                    current_block_67 = 14001958660280927786;
                } else {
                    let k: *const libc::c_char = (*ds).key.ptr;
                    if *k.offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xdf as libc::c_int == 'X' as i32
                        && http_response_omit_header(r, ds) != 0
                    {
                        alen = (alen as libc::c_uint)
                            .wrapping_sub(
                                klen
                                    .wrapping_add(vlen)
                                    .wrapping_add(4 as libc::c_int as libc::c_uint),
                            ) as uint32_t as uint32_t;
                        current_block_67 = 1356832168064818221;
                    } else {
                        let mut j: uint32_t = 0 as libc::c_int as uint32_t;
                        while j < klen {
                            *v
                                .offset(
                                    j as isize,
                                ) = (if !((*k.offset(j as isize) as uint32_t)
                                .wrapping_sub('A' as i32 as libc::c_uint)
                                <= ('Z' as i32 - 'A' as i32) as libc::c_uint)
                            {
                                *k.offset(j as isize) as libc::c_int
                            } else {
                                *k.offset(j as isize) as libc::c_int | 0x20 as libc::c_int
                            }) as libc::c_char;
                            j = j.wrapping_add(1);
                        }
                        current_block_67 = 14001958660280927786;
                    }
                }
                match current_block_67 {
                    1356832168064818221 => {}
                    _ => {
                        let mut voff: uint32_t = 0 as libc::c_int as uint32_t;
                        let mut n: *const libc::c_char = 0 as *const libc::c_char;
                        lsx.buf = (*ds).value.ptr;
                        loop {
                            n = (if resp_header_repeated == 0 {
                                0 as *mut libc::c_void
                            } else {
                                memchr(
                                    (lsx.buf).offset(voff as isize) as *const libc::c_void,
                                    '\n' as i32,
                                    vlen.wrapping_sub(voff) as libc::c_ulong,
                                )
                            }) as *const libc::c_char;
                            memset(
                                &mut lsx as *mut lsxpack_header_t as *mut libc::c_void,
                                0 as libc::c_int,
                                ::std::mem::size_of::<lsxpack_header_t>() as libc::c_ulong,
                            );
                            lsx
                                .hpack_index = http_header_lshpack_idx[(*ds).ext as usize];
                            lsx.buf = (*ds).value.ptr;
                            lsx
                                .name_offset = vlen
                                .wrapping_add(1 as libc::c_int as libc::c_uint)
                                as lsxpack_strlen_t;
                            lsx.name_len = klen as lsxpack_strlen_t;
                            lsx.val_offset = voff as lsxpack_strlen_t;
                            if n.is_null() {
                                lsx.val_len = vlen.wrapping_sub(voff) as lsxpack_strlen_t;
                            } else {
                                voff = n
                                    .offset(1 as libc::c_int as isize)
                                    .offset_from(lsx.buf) as libc::c_long as uint32_t;
                                lsx
                                    .val_len = voff
                                    .wrapping_sub(2 as libc::c_int as libc::c_uint)
                                    .wrapping_sub(lsx.val_offset as libc::c_uint)
                                    as lsxpack_strlen_t;
                                voff = (voff as libc::c_uint)
                                    .wrapping_add(
                                        klen.wrapping_add(2 as libc::c_int as libc::c_uint),
                                    ) as uint32_t as uint32_t;
                            }
                            if log_response_header != 0 {
                                h2_log_response_header_lsx(r, &mut lsx);
                            }
                            let dst_in: *mut libc::c_uchar = dst;
                            dst = lshpack_enc_encode(encoder, dst, dst_end, &mut lsx);
                            if dst == dst_in {
                                h2_send_rst_stream(r, con, H2_E_INTERNAL_ERROR);
                                return;
                            }
                            if n.is_null() {
                                break;
                            }
                        }
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if (*r).resp_htags & (1 as libc::c_ulong) << HTTP_HEADER_DATE as libc::c_int == 0 {
        static mut tlast: unix_time64_t = 0 as libc::c_int as unix_time64_t;
        static mut tstr: [libc::c_char; 36] = unsafe {
            *::std::mem::transmute::<
                &[u8; 36],
                &mut [libc::c_char; 36],
            >(b"date: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0")
        };
        memset(
            &mut lsx as *mut lsxpack_header_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<lsxpack_header_t>() as libc::c_ulong,
        );
        lsx.buf = tstr.as_mut_ptr();
        lsx.name_offset = 0 as libc::c_int as lsxpack_strlen_t;
        lsx.name_len = 4 as libc::c_int as lsxpack_strlen_t;
        lsx.val_offset = 6 as libc::c_int as lsxpack_strlen_t;
        lsx.val_len = 29 as libc::c_int as lsxpack_strlen_t;
        lsx.hpack_index = LSHPACK_HDR_DATE as libc::c_int as uint8_t;
        let cur_ts: unix_time64_t = log_epoch_secs;
        if (tlast != cur_ts) as libc::c_int as libc::c_long != 0 {
            tlast = cur_ts;
            http_date_time_to_str(
                tstr.as_mut_ptr().offset(6 as libc::c_int as isize),
                (::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong)
                    .wrapping_sub(6 as libc::c_int as libc::c_ulong),
                tlast,
            );
        }
        alen = (alen as libc::c_uint)
            .wrapping_add((35 as libc::c_int + 2 as libc::c_int) as libc::c_uint)
            as uint32_t as uint32_t;
        if log_response_header != 0 {
            h2_log_response_header(r, 35 as libc::c_int, tstr.as_mut_ptr());
        }
        let dst_in_0: *mut libc::c_uchar = dst;
        dst = lshpack_enc_encode(encoder, dst, dst_end, &mut lsx);
        if dst == dst_in_0 {
            h2_send_rst_stream(r, con, H2_E_INTERNAL_ERROR);
            return;
        }
    }
    if (*r).resp_htags & (1 as libc::c_ulong) << HTTP_HEADER_SERVER as libc::c_int == 0
        && !((*r).conf.server_tag).is_null()
    {
        let b: *mut buffer = chunk_buffer_acquire();
        let vlen_0: uint32_t = buffer_clen((*r).conf.server_tag);
        buffer_append_str2(
            b,
            b"server: \0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            (*(*r).conf.server_tag).ptr,
            vlen_0 as size_t,
        );
        alen = (alen as libc::c_uint)
            .wrapping_add(
                (6 as libc::c_int as libc::c_uint)
                    .wrapping_add(vlen_0)
                    .wrapping_add(4 as libc::c_int as libc::c_uint),
            ) as uint32_t as uint32_t;
        if log_response_header != 0 {
            h2_log_response_header(
                r,
                (6 as libc::c_int as libc::c_uint)
                    .wrapping_add(vlen_0)
                    .wrapping_add(2 as libc::c_int as libc::c_uint) as libc::c_int,
                (*b).ptr,
            );
        }
        memset(
            &mut lsx as *mut lsxpack_header_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<lsxpack_header_t>() as libc::c_ulong,
        );
        lsx.buf = (*b).ptr;
        lsx.name_offset = 0 as libc::c_int as lsxpack_strlen_t;
        lsx.name_len = 6 as libc::c_int as lsxpack_strlen_t;
        lsx.val_offset = 8 as libc::c_int as lsxpack_strlen_t;
        lsx.val_len = vlen_0 as lsxpack_strlen_t;
        lsx.hpack_index = LSHPACK_HDR_SERVER as libc::c_int as uint8_t;
        let dst_in_1: *mut libc::c_uchar = dst;
        dst = lshpack_enc_encode(encoder, dst, dst_end, &mut lsx);
        chunk_buffer_release(b);
        if dst == dst_in_1 {
            h2_send_rst_stream(r, con, H2_E_INTERNAL_ERROR);
            return;
        }
    }
    alen = (alen as libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
        as uint32_t as uint32_t;
    (*r).resp_header_len = alen;
    let wq: *mut chunkqueue = &mut (*r).write_queue;
    (*wq).bytes_in += alen as off_t;
    (*wq).bytes_out += alen as off_t;
    let dlen: uint32_t = (dst as *mut libc::c_char).offset_from((*tb).ptr)
        as libc::c_long as uint32_t;
    let flags: uint32_t = (if (*r).resp_body_finished as libc::c_int != 0
        && chunkqueue_is_empty(&mut (*r).write_queue) != 0
    {
        H2_FLAG_END_STREAM as libc::c_int
    } else {
        0 as libc::c_int
    }) as uint32_t;
    h2_send_hpack(r, con, (*tb).ptr, dlen, flags);
}
#[cold]
#[inline(never)]
unsafe extern "C" fn h2_send_headers_block(
    r: *mut request_st,
    con: *mut connection,
    hdrs: *const libc::c_char,
    hlen: uint32_t,
    mut flags: uint32_t,
) {
    let mut hoff: [libc::c_ushort; 8192] = [0; 8192];
    hoff[0 as libc::c_int as usize] = 1 as libc::c_int as libc::c_ushort;
    hoff[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    let mut rc: uint32_t = http_header_parse_hoff(hdrs, hlen, hoff.as_mut_ptr());
    if 0 as libc::c_int as libc::c_uint == rc
        || rc
            > (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                as libc::c_uint
        || hoff[0 as libc::c_int as usize] as libc::c_ulong
            >= (::std::mem::size_of::<[libc::c_ushort; 8192]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        || 1 as libc::c_int == hoff[0 as libc::c_int as usize] as libc::c_int
    {
        log_error(
            (*r).conf.errh,
            b"src/h2.c\0" as *const u8 as *const libc::c_char,
            2133 as libc::c_int as libc::c_uint,
            b"oversized response-header\0" as *const u8 as *const libc::c_char,
        );
        hoff[0 as libc::c_int as usize] = 1 as libc::c_int as libc::c_ushort;
        hoff[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
        http_header_parse_hoff(
            b":status: 500\r\n\r\n\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            hoff.as_mut_ptr(),
        ) != 0;
    }
    let tb: *mut buffer = (*r).tmp_buf;
    if !((*tb).size >= 65536 as libc::c_int as libc::c_uint) {
        ck_assert_failed(
            b"src/h2.c\0" as *const u8 as *const libc::c_char,
            2144 as libc::c_int as libc::c_uint,
            b"tb->size >= 65536\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut dst: *mut libc::c_uchar = (*tb).ptr as *mut libc::c_uchar;
    let dst_end: *mut libc::c_uchar = ((*tb).ptr as *mut libc::c_uchar)
        .offset((*tb).size as isize);
    let h2c: *mut h2con = (*con).h2;
    let encoder: *mut lshpack_enc = &mut (*h2c).encoder;
    let mut lsx: lsxpack_header_t = lsxpack_header_t {
        buf: 0 as *mut libc::c_char,
        name_hash: 0,
        nameval_hash: 0,
        name_offset: 0,
        name_len: 0,
        val_offset: 0,
        val_len: 0,
        chain_next_idx: 0,
        hpack_index: 0,
        qpack_index: 0,
        app_index: 0,
        flags: [0; 1],
        indexed_type: 0,
        dec_overhead: 0,
    };
    let mut i: libc::c_int = 1 as libc::c_int;
    if *hdrs.offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32 {
        i = 2 as libc::c_int;
        memset(
            &mut lsx as *mut lsxpack_header_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<lsxpack_header_t>() as libc::c_ulong,
        );
        let ref mut fresh2 = *(&mut lsx.buf as *mut *mut libc::c_char
            as *mut *const libc::c_char);
        *fresh2 = hdrs;
        lsx.name_offset = 0 as libc::c_int as lsxpack_strlen_t;
        lsx
            .name_len = (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as lsxpack_strlen_t;
        lsx
            .val_offset = (lsx.name_len as libc::c_int + 2 as libc::c_int)
            as lsxpack_strlen_t;
        lsx.val_len = 3 as libc::c_int as lsxpack_strlen_t;
        dst = lshpack_enc_encode(encoder, dst, dst_end, &mut lsx);
        if dst == (*tb).ptr as *mut libc::c_uchar {
            h2_send_rst_stream(r, con, H2_E_INTERNAL_ERROR);
            return;
        }
    }
    while i < hoff[0 as libc::c_int as usize] as libc::c_int {
        let mut k: *const libc::c_char = hdrs
            .offset(
                (if i > 1 as libc::c_int {
                    hoff[i as usize] as libc::c_int
                } else {
                    0 as libc::c_int
                }) as isize,
            );
        let mut end: *const libc::c_char = hdrs
            .offset(hoff[(i + 1 as libc::c_int) as usize] as libc::c_int as isize);
        let mut v: *const libc::c_char = memchr(
            k as *const libc::c_void,
            ':' as i32,
            end.offset_from(k) as libc::c_long as libc::c_ulong,
        ) as *const libc::c_char;
        if !(v.is_null() || k == v) {
            let mut klen: uint32_t = v.offset_from(k) as libc::c_long as uint32_t;
            if !(0 as libc::c_int as libc::c_uint == klen) {
                loop {
                    v = v.offset(1);
                    if !(*v as libc::c_int == ' ' as i32
                        || *v as libc::c_int == '\t' as i32)
                    {
                        break;
                    }
                }
                if !(*end.offset(-(2 as libc::c_int) as isize) as libc::c_int
                    != '\r' as i32)
                {
                    end = end.offset(-(2 as libc::c_int as isize));
                    let mut vlen: uint32_t = end.offset_from(v) as libc::c_long
                        as uint32_t;
                    if !(0 as libc::c_int as libc::c_uint == vlen) {
                        memset(
                            &mut lsx as *mut lsxpack_header_t as *mut libc::c_void,
                            0 as libc::c_int,
                            ::std::mem::size_of::<lsxpack_header_t>() as libc::c_ulong,
                        );
                        let ref mut fresh3 = *(&mut lsx.buf as *mut *mut libc::c_char
                            as *mut *const libc::c_char);
                        *fresh3 = hdrs;
                        lsx
                            .name_offset = k.offset_from(hdrs) as libc::c_long
                            as lsxpack_strlen_t;
                        lsx.name_len = klen as lsxpack_strlen_t;
                        lsx
                            .val_offset = v.offset_from(hdrs) as libc::c_long
                            as lsxpack_strlen_t;
                        lsx.val_len = vlen as lsxpack_strlen_t;
                        let dst_in: *mut libc::c_uchar = dst;
                        dst = lshpack_enc_encode(encoder, dst, dst_end, &mut lsx);
                        if dst == dst_in {
                            h2_send_rst_stream(r, con, H2_E_INTERNAL_ERROR);
                            return;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    let mut dlen: uint32_t = (dst as *mut libc::c_char).offset_from((*tb).ptr)
        as libc::c_long as uint32_t;
    h2_send_hpack(r, con, (*tb).ptr, dlen, flags);
}
unsafe extern "C" fn h2_send_1xx_block(
    r: *mut request_st,
    con: *mut connection,
    hdrs: *const libc::c_char,
    hlen: uint32_t,
) {
    h2_send_headers_block(r, con, hdrs, hlen, 0 as libc::c_int as uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn h2_send_1xx(
    r: *mut request_st,
    con: *mut connection,
) -> libc::c_int {
    let b: *mut buffer = chunk_buffer_acquire();
    buffer_copy_string_len(
        b,
        b":status: \0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(b, (*r).http_status as intmax_t);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*r).resp_headers.used {
        let ds: *const data_string = *((*r).resp_headers.data).offset(i as isize)
            as *mut data_string;
        let klen: uint32_t = buffer_clen(&(*ds).key);
        let vlen: uint32_t = buffer_clen(&(*ds).value);
        if !(0 as libc::c_int as libc::c_uint == klen
            || 0 as libc::c_int as libc::c_uint == vlen)
        {
            buffer_append_str2(
                b,
                b"\r\n\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                (*ds).key.ptr,
                klen as size_t,
            );
            buffer_append_str2(
                b,
                b": \0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                (*ds).value.ptr,
                vlen as size_t,
            );
        }
        i = i.wrapping_add(1);
    }
    buffer_append_string_len(
        b,
        b"\r\n\r\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    h2_send_1xx_block(r, con, (*b).ptr, buffer_clen(b));
    chunk_buffer_release(b);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn h2_send_100_continue(r: *mut request_st, con: *mut connection) {
    h2_send_1xx_block(
        r,
        con,
        b":status: 100\r\n\r\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
}
#[cold]
#[inline(never)]
unsafe extern "C" fn h2_send_end_stream_trailers(
    r: *mut request_st,
    con: *mut connection,
    trailers: *const buffer,
) {
    let mut hoff: [libc::c_ushort; 8192] = [0; 8192];
    hoff[0 as libc::c_int as usize] = 1 as libc::c_int as libc::c_ushort;
    hoff[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    let mut rc: uint32_t = http_header_parse_hoff(
        (*trailers).ptr,
        buffer_clen(trailers),
        hoff.as_mut_ptr(),
    );
    if 0 as libc::c_int as libc::c_uint == rc
        || rc
            > (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                as libc::c_uint
        || hoff[0 as libc::c_int as usize] as libc::c_ulong
            >= (::std::mem::size_of::<[libc::c_ushort; 8192]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        || 1 as libc::c_int == hoff[0 as libc::c_int as usize] as libc::c_int
    {
        h2_send_end_stream_data(r, con);
        return;
    }
    let ptr: *mut libc::c_char = (*trailers).ptr;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < hoff[0 as libc::c_int as usize] as libc::c_int {
        let mut k: *mut libc::c_char = ptr
            .offset(
                (if i > 1 as libc::c_int {
                    hoff[i as usize] as libc::c_int
                } else {
                    0 as libc::c_int
                }) as isize,
            );
        if *k as libc::c_int == ':' as i32 {
            h2_send_end_stream_data(r, con);
            return;
        }
        let colon: *const libc::c_char = memchr(
            k as *const libc::c_void,
            ':' as i32,
            ptr
                .offset(hoff[(i + 1 as libc::c_int) as usize] as libc::c_int as isize)
                .offset_from(k) as libc::c_long as libc::c_ulong,
        ) as *const libc::c_char;
        if !colon.is_null() {
            loop {
                if (*k as uint32_t).wrapping_sub('A' as i32 as libc::c_uint)
                    <= ('Z' as i32 - 'A' as i32) as libc::c_uint
                {
                    *k = (*k as libc::c_int | 0x20 as libc::c_int) as libc::c_char;
                }
                k = k.offset(1);
                if !(k != colon as *mut libc::c_char) {
                    break;
                }
            }
        }
        i += 1;
    }
    h2_send_headers_block(
        r,
        con,
        (*trailers).ptr,
        buffer_clen(trailers),
        H2_FLAG_END_STREAM as libc::c_int as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn h2_send_cqdata(
    r: *mut request_st,
    con: *mut connection,
    cq: *mut chunkqueue,
    mut dlen: uint32_t,
) -> uint32_t {
    let mut dataframe: C2RustUnnamed_14 = C2RustUnnamed_14 {
        c: [
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            H2_FTYPE_DATA as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
        ],
    };
    dataframe.u[2 as libc::c_int as usize] = __bswap_32((*r).h2id);
    let h2r: *mut request_st = &mut (*con).request;
    if (*r).h2_swin < 0 as libc::c_int {
        return 0 as libc::c_int as uint32_t;
    }
    if (*h2r).h2_swin < 0 as libc::c_int {
        return 0 as libc::c_int as uint32_t;
    }
    if dlen as int32_t > (*r).h2_swin {
        dlen = (*r).h2_swin as uint32_t;
    }
    if dlen as int32_t > (*h2r).h2_swin {
        dlen = (*h2r).h2_swin as uint32_t;
    }
    let cqlen: uint32_t = chunkqueue_length(cq) as uint32_t;
    if dlen > cqlen {
        dlen = cqlen;
    }
    if 0 as libc::c_int as libc::c_uint == dlen {
        return 0 as libc::c_int as uint32_t;
    }
    let h2c: *mut h2con = (*con).h2;
    let fsize: uint32_t = (*h2c).s_max_frame_size;
    let mut sent: uint32_t = 0 as libc::c_int as uint32_t;
    let mut current_block_31: u64;
    loop {
        if (*(*cq).first).type_0 as libc::c_uint
            == FILE_CHUNK as libc::c_int as libc::c_uint
        {
            let len: uint32_t = if dlen < fsize {
                dlen
            } else {
                fsize.wrapping_sub(9 as libc::c_int as libc::c_uint)
            };
            let mut blen: uint32_t = len;
            let b: *mut buffer = chunkqueue_append_buffer_open_sz(
                (*con).write_queue,
                (9 as libc::c_int as libc::c_uint).wrapping_add(len) as size_t,
            );
            let mut data: *mut libc::c_char = ((*b).ptr)
                .offset(9 as libc::c_int as isize);
            if 0 as libc::c_int
                == chunkqueue_peek_data(cq, &mut data, &mut blen, (*r).conf.errh)
                && blen == len
            {
                dlen = (dlen as libc::c_uint).wrapping_sub(len) as uint32_t as uint32_t;
                sent = (sent as libc::c_uint).wrapping_add(len) as uint32_t as uint32_t;
                dataframe
                    .c[3 as libc::c_int
                    as usize] = (len >> 16 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as uint8_t;
                dataframe
                    .c[4 as libc::c_int
                    as usize] = (len >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as uint8_t;
                dataframe
                    .c[5 as libc::c_int
                    as usize] = (len & 0xff as libc::c_int as libc::c_uint) as uint8_t;
                memcpy(
                    (*b).ptr as *mut libc::c_void,
                    ((dataframe.c).as_mut_ptr() as *const libc::c_char)
                        .offset(3 as libc::c_int as isize) as *const libc::c_void,
                    (::std::mem::size_of::<C2RustUnnamed_14>() as libc::c_ulong)
                        .wrapping_sub(3 as libc::c_int as libc::c_ulong),
                );
                if ((*b).ptr).offset(9 as libc::c_int as isize) != data {
                    memcpy(
                        ((*b).ptr).offset(9 as libc::c_int as isize)
                            as *mut libc::c_void,
                        data as *const libc::c_void,
                        len as libc::c_ulong,
                    );
                }
                buffer_commit(
                    b,
                    (9 as libc::c_int as libc::c_uint).wrapping_add(len) as size_t,
                );
                chunkqueue_append_buffer_commit((*con).write_queue);
                chunkqueue_mark_written(cq, len as off_t);
                current_block_31 = 17860125682698302841;
            } else {
                chunkqueue_remove_empty_chunks(cq);
                current_block_31 = 11636175345244025579;
            }
        } else {
            current_block_31 = 11636175345244025579;
        }
        match current_block_31 {
            11636175345244025579 => {
                let len_0: uint32_t = if dlen < fsize { dlen } else { fsize };
                dlen = (dlen as libc::c_uint).wrapping_sub(len_0) as uint32_t
                    as uint32_t;
                sent = (sent as libc::c_uint).wrapping_add(len_0) as uint32_t
                    as uint32_t;
                dataframe
                    .c[3 as libc::c_int
                    as usize] = (len_0 >> 16 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as uint8_t;
                dataframe
                    .c[4 as libc::c_int
                    as usize] = (len_0 >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as uint8_t;
                dataframe
                    .c[5 as libc::c_int
                    as usize] = (len_0 & 0xff as libc::c_int as libc::c_uint) as uint8_t;
                chunkqueue_append_mem(
                    (*con).write_queue,
                    ((dataframe.c).as_mut_ptr() as *const libc::c_char)
                        .offset(3 as libc::c_int as isize),
                    (::std::mem::size_of::<C2RustUnnamed_14>() as libc::c_ulong)
                        .wrapping_sub(3 as libc::c_int as libc::c_ulong),
                );
                chunkqueue_steal((*con).write_queue, cq, len_0 as off_t);
            }
            _ => {}
        }
        if !(dlen != 0) {
            break;
        }
    }
    (*r).h2_swin -= sent as int32_t;
    (*h2r).h2_swin -= sent as int32_t;
    return sent;
}
unsafe extern "C" fn h2_send_end_stream_data(r: *mut request_st, con: *mut connection) {
    let mut dataframe: C2RustUnnamed_15 = C2RustUnnamed_15 {
        c: [
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            H2_FTYPE_DATA as libc::c_int as uint8_t,
            H2_FLAG_END_STREAM as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
        ],
    };
    dataframe.u[2 as libc::c_int as usize] = __bswap_32((*r).h2id);
    chunkqueue_append_mem(
        (*con).write_queue,
        ((dataframe.c).as_mut_ptr() as *const libc::c_char)
            .offset(3 as libc::c_int as isize),
        (::std::mem::size_of::<C2RustUnnamed_15>() as libc::c_ulong)
            .wrapping_sub(3 as libc::c_int as libc::c_ulong),
    );
    if (*r).h2state != H2_STATE_HALF_CLOSED_REMOTE as libc::c_int as libc::c_uint {
        let h2c: *mut h2con = (*con).h2;
        (*h2c).half_closed_ts = log_monotonic_secs;
        h2_send_rst_stream_id((*r).h2id, con, H2_E_NO_ERROR);
    }
    (*r).h2state = H2_STATE_CLOSED as libc::c_int as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn h2_send_end_stream(r: *mut request_st, con: *mut connection) {
    if (*r).h2state == H2_STATE_CLOSED as libc::c_int as libc::c_uint {
        return;
    }
    if (*r).state as libc::c_uint != CON_STATE_ERROR as libc::c_int as libc::c_uint
        && (*r).resp_body_finished as libc::c_int != 0
    {
        if !((*r).gw_dechunk).is_null() && (*(*r).gw_dechunk).done != 0
            && buffer_is_unset(&mut (*(*r).gw_dechunk).b) == 0
        {
            h2_send_end_stream_trailers(r, con, &mut (*(*r).gw_dechunk).b);
        } else {
            h2_send_end_stream_data(r, con);
        }
    } else {
        h2_send_rst_stream(r, con, H2_E_INTERNAL_ERROR);
    };
}
unsafe extern "C" fn h2_init_stream(
    h2r: *mut request_st,
    con: *mut connection,
) -> *mut request_st {
    let h2c: *mut h2con = (*con).h2;
    (*con).request_count = ((*con).request_count).wrapping_add(1);
    if !(((*h2c).rused as libc::c_ulong)
        < (::std::mem::size_of::<[*mut request_st; 8]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut request_st>() as libc::c_ulong))
    {
        ck_assert_failed(
            b"src/h2.c\0" as *const u8 as *const libc::c_char,
            2558 as libc::c_int as libc::c_uint,
            b"h2c->rused < sizeof(h2c->r)/sizeof(*h2c->r)\0" as *const u8
                as *const libc::c_char,
        );
    }
    let r: *mut request_st = request_acquire(con);
    let fresh4 = (*h2c).rused;
    (*h2c).rused = ((*h2c).rused).wrapping_add(1);
    (*h2c).r[fresh4 as usize] = r;
    (*r).h2_rwin = 65535 as libc::c_int;
    (*r).h2_swin = (*h2c).s_initial_window_size;
    (*r).http_version = HTTP_VERSION_2;
    let srv: *mut server = (*con).srv;
    let used: uint32_t = (*(*srv).config_context).used;
    (*r).conditional_is_valid = (*h2r).conditional_is_valid;
    memcpy(
        (*r).cond_cache as *mut libc::c_void,
        (*h2r).cond_cache as *const libc::c_void,
        (used as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cond_cache_t>() as libc::c_ulong),
    );
    if (*srv).config_captures != 0 {
        memcpy(
            (*r).cond_match as *mut libc::c_void,
            (*h2r).cond_match as *const libc::c_void,
            ((*srv).config_captures as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut cond_match_t>() as libc::c_ulong,
                ),
        );
    }
    (*r).server_name = (*h2r).server_name;
    memcpy(
        &mut (*r).conf as *mut request_config as *mut libc::c_void,
        &mut (*h2r).conf as *mut request_config as *const libc::c_void,
        ::std::mem::size_of::<request_config>() as libc::c_ulong,
    );
    return r;
}
unsafe extern "C" fn h2_release_stream(r: *mut request_st, con: *mut connection) {
    if (*r).http_status != 0 {
        plugins_call_handle_request_done(r);
    }
    request_release(r);
}
#[no_mangle]
pub unsafe extern "C" fn h2_retire_stream(mut r: *mut request_st, con: *mut connection) {
    if r.is_null() {
        return;
    }
    let h2c: *mut h2con = (*con).h2;
    let ar: *mut *mut request_st = ((*h2c).r).as_mut_ptr();
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let mut rused: uint32_t = (*h2c).rused;
    while i < rused && *ar.offset(i as isize) != r {
        i = i.wrapping_add(1);
    }
    if i != rused {
        rused = rused.wrapping_sub(1);
        if i != rused {
            memmove(
                ar.offset(i as isize) as *mut libc::c_void,
                ar.offset(i as isize).offset(1 as libc::c_int as isize)
                    as *const libc::c_void,
                (rused.wrapping_sub(i) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut request_st>() as libc::c_ulong,
                    ),
            );
        }
        (*h2c).rused = rused;
        (*h2c).r[(*h2c).rused as usize] = 0 as *mut request_st;
        h2_release_stream(r, con);
    }
}
#[no_mangle]
pub unsafe extern "C" fn h2_retire_con(h2r: *mut request_st, con: *mut connection) {
    let h2c: *mut h2con = (*con).h2;
    if h2c.is_null() {
        return;
    }
    if (*h2r).state as libc::c_uint != CON_STATE_ERROR as libc::c_int as libc::c_uint {
        h2_send_goaway(con, H2_E_NO_ERROR);
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        let mut rused: uint32_t = (*h2c).rused;
        while i < rused {
            let r: *mut request_st = (*h2c).r[i as usize];
            h2_send_rst_stream(r, con, H2_E_INTERNAL_ERROR);
            h2_release_stream(r, con);
            i = i.wrapping_add(1);
        }
        if chunkqueue_is_empty((*con).write_queue) == 0 {
            let cq: *mut chunkqueue = (*con).write_queue;
            let len: off_t = chunkqueue_length(cq);
            let mut written: off_t = (*cq).bytes_out;
            ((*con).network_write).expect("non-null function pointer")(con, cq, len);
            written = (*cq).bytes_out - written;
            (*con).bytes_written += written;
            (*con).bytes_written_cur_second += written;
            if !((*h2r).conf.global_bytes_per_second_cnt_ptr).is_null() {
                *(*h2r).conf.global_bytes_per_second_cnt_ptr += written;
            }
        }
    } else {
        let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
        let mut rused_0: uint32_t = (*h2c).rused;
        while i_0 < rused_0 {
            let r_0: *mut request_st = (*h2c).r[i_0 as usize];
            h2_release_stream(r_0, con);
            i_0 = i_0.wrapping_add(1);
        }
    }
    (*con).h2 = 0 as *mut h2con;
    lshpack_enc_cleanup(&mut (*h2c).encoder);
    lshpack_dec_cleanup(&mut (*h2c).decoder);
    free(h2c as *mut libc::c_void);
}
unsafe extern "C" fn h2_con_upgrade_h2c(
    h2r: *mut request_st,
    http2_settings: *const buffer,
) {
    static mut switch_proto: [libc::c_char; 72] = unsafe {
        *::std::mem::transmute::<
            &[u8; 72],
            &[libc::c_char; 72],
        >(
            b"HTTP/1.1 101 Switching Protocols\r\nConnection: Upgrade\r\nUpgrade: h2c\r\n\r\n\0",
        )
    };
    chunkqueue_append_mem(
        &mut (*h2r).write_queue,
        switch_proto.as_ptr(),
        (::std::mem::size_of::<[libc::c_char; 72]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    (*h2r)
        .resp_header_len = (::std::mem::size_of::<[libc::c_char; 72]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t;
    let con: *mut connection = (*h2r).con;
    h2_init_con(h2r, con, http2_settings);
    if (*(*con).h2).sent_goaway != 0 {
        return;
    }
    (*(*con).h2).h2_cid = 1 as libc::c_int as uint32_t;
    let r: *mut request_st = h2_init_stream(h2r, con);
    (*con).request_count = ((*con).request_count).wrapping_sub(1);
    (*r).state = CON_STATE_REQUEST_END;
    (*r).http_status = 0 as libc::c_int;
    (*r).http_method = (*h2r).http_method;
    (*r).h2state = H2_STATE_HALF_CLOSED_REMOTE as libc::c_int as uint32_t;
    (*r).h2id = 1 as libc::c_int as uint32_t;
    (*r).rqst_htags = (*h2r).rqst_htags;
    (*h2r).rqst_htags = 0 as libc::c_int as uint64_t;
    (*r).rqst_header_len = (*h2r).rqst_header_len;
    (*h2r).rqst_header_len = 0 as libc::c_int as uint32_t;
    (*r).rqst_headers = (*h2r).rqst_headers;
    memset(
        &mut (*h2r).rqst_headers as *mut array as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<array>() as libc::c_ulong,
    );
    (*r).uri = (*h2r).uri;
    memset(
        &mut (*h2r).rqst_headers as *mut array as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<array>() as libc::c_ulong,
    );
    memset(
        &mut (*h2r).uri as *mut request_uri as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<request_uri>() as libc::c_ulong,
    );
    (*r).http_host = (*h2r).http_host;
    (*h2r).http_host = 0 as *mut buffer;
    (*r).target = (*h2r).target;
    (*r).target_orig = (*h2r).target_orig;
    memset(
        &mut (*h2r).target as *mut buffer as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<buffer>() as libc::c_ulong,
    );
    memset(
        &mut (*h2r).target_orig as *mut buffer as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<buffer>() as libc::c_ulong,
    );
    (*r).keep_alive = (*h2r).keep_alive;
    (*r).tmp_buf = (*h2r).tmp_buf;
    (*r).start_hp = (*h2r).start_hp;
}
#[no_mangle]
#[cold]
#[inline(never)]
pub unsafe extern "C" fn h2_check_con_upgrade_h2c(r: *mut request_st) -> libc::c_int {
    let mut http_connection: *mut buffer = 0 as *mut buffer;
    let mut http2_settings: *mut buffer = 0 as *mut buffer;
    let mut upgrade: *mut buffer = http_header_request_get(
        r,
        HTTP_HEADER_UPGRADE,
        b"Upgrade\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if upgrade.is_null() {
        return 0 as libc::c_int;
    }
    http_connection = http_header_request_get(
        r,
        HTTP_HEADER_CONNECTION,
        b"Connection\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if http_connection.is_null() {
        http_header_request_unset(
            r,
            HTTP_HEADER_UPGRADE,
            b"Upgrade\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        return 0 as libc::c_int;
    }
    if (*r).http_version as libc::c_int != HTTP_VERSION_1_1 as libc::c_int {
        http_header_request_unset(
            r,
            HTTP_HEADER_UPGRADE,
            b"Upgrade\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        http_header_remove_token(
            http_connection,
            b"Upgrade\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        return 0 as libc::c_int;
    }
    if http_header_str_contains_token(
        (*upgrade).ptr,
        buffer_clen(upgrade),
        b"h2c\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    ) == 0
    {
        return 0 as libc::c_int;
    }
    http2_settings = http_header_request_get(
        r,
        HTTP_HEADER_HTTP2_SETTINGS,
        b"HTTP2-Settings\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if !http2_settings.is_null() {
        if 0 as libc::c_int as libc::c_long == (*r).reqbody_length {
            let b: *mut buffer = (*r).tmp_buf;
            buffer_clear(b);
            if (*r).conf.h2proto as libc::c_int > 1 as libc::c_int
                && (*(*r).con).is_ssl_sock == 0
                && http_header_str_contains_token(
                    (*http_connection).ptr,
                    buffer_clen(http_connection),
                    b"HTTP2-Settings\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                ) != 0
                && !(buffer_append_base64_decode(
                    b,
                    (*http2_settings).ptr,
                    buffer_clen(http2_settings) as size_t,
                    BASE64_URL,
                ))
                    .is_null()
            {
                h2_con_upgrade_h2c(r, b);
                (*r).http_version = HTTP_VERSION_2;
            }
        }
        http_header_request_unset(
            r,
            HTTP_HEADER_HTTP2_SETTINGS,
            b"HTTP2-Settings\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        http_header_remove_token(
            http_connection,
            b"HTTP2-Settings\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    http_header_request_unset(
        r,
        HTTP_HEADER_UPGRADE,
        b"Upgrade\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    http_header_remove_token(
        http_connection,
        b"Upgrade\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    return ((*r).http_version as libc::c_int == HTTP_VERSION_2 as libc::c_int)
        as libc::c_int;
}
