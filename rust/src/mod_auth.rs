use ::libc;
extern "C" {
    pub type fdnode_st;
    pub type stat_cache_entry;
    pub type pcre2_real_match_data_8;
    pub type h2con;
    pub type fdevents;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn buffer_extend(b: *mut buffer, x: size_t) -> *mut libc::c_char;
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
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
    fn buffer_append_iovec(b: *mut buffer, iov: *const const_iovec, n: size_t);
    fn buffer_append_uint_hex_lc(b: *mut buffer, len: uintmax_t);
    fn li_tohex_lc(
        buf: *mut libc::c_char,
        buf_len: size_t,
        s: *const libc::c_char,
        s_len: size_t,
    );
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
    fn buffer_eq_slen(
        b: *const buffer,
        s: *const libc::c_char,
        slen: size_t,
    ) -> libc::c_int;
    fn buffer_urldecode_path(b: *mut buffer);
    fn buffer_is_valid_UTF8(b: *const buffer) -> libc::c_int;
    fn hex2int(c: libc::c_uchar) -> libc::c_char;
    fn li_hex2bin(
        bin: *mut libc::c_uchar,
        binlen: size_t,
        hexstr: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
    fn http_auth_backend_get(name: *const buffer) -> *const http_auth_backend_t;
    fn http_auth_scheme_get(name: *const buffer) -> *const http_auth_scheme_t;
    fn http_auth_match_rules(
        require: *const http_auth_require_t,
        user: *const libc::c_char,
        group: *const libc::c_char,
        host: *const libc::c_char,
    ) -> libc::c_int;
    fn http_auth_require_free(require: *mut http_auth_require_t);
    fn http_auth_require_init() -> *mut http_auth_require_t;
    fn http_auth_dumbdata_reset();
    fn ck_memeq_const_time(
        a: *const libc::c_void,
        alen: size_t,
        b: *const libc::c_void,
        blen: size_t,
    ) -> libc::c_int;
    fn array_get_buf_ptr(
        a: *mut array,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn array_match_key_prefix(a: *const array, b: *const buffer) -> *mut data_unset;
    fn http_auth_scheme_set(scheme: *const http_auth_scheme_t);
    fn ck_memclear_s(s: *mut libc::c_void, smax: rsize_t, n: rsize_t) -> errno_t;
    fn array_match_key_prefix_nc(a: *const array, b: *const buffer) -> *mut data_unset;
    fn ck_memeq_const_time_fixed_len(
        a: *const libc::c_void,
        b: *const libc::c_void,
        len: size_t,
    ) -> libc::c_int;
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn array_init(n: uint32_t) -> *mut array;
    fn array_free(a: *mut array);
    fn array_insert_unique(a: *mut array, entry: *mut data_unset);
    fn array_is_kvstring(a: *const array) -> libc::c_int;
    fn http_auth_setenv(
        r: *mut request_st,
        username: *const libc::c_char,
        ulen: size_t,
        auth_type: *const libc::c_char,
        alen: size_t,
    );
    fn MD5_Init(_: *mut MD5_CTX);
    fn MD5_Update(_: *mut MD5_CTX, _: *const libc::c_void, _: libc::c_uint);
    fn MD5_Final(_: *mut libc::c_uchar, _: *mut MD5_CTX);
    fn http_method_buf(i: http_method_t) -> *const buffer;
    fn http_header_response_set_ptr(
        r: *mut request_st,
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
    static mut log_epoch_secs: unix_time64_t;
    static mut log_monotonic_secs: unix_time64_t;
    fn log_error(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        fmt: *const libc::c_char,
        _: ...
    );
    fn splaytree_splay(t: *mut splay_tree, key: libc::c_int) -> *mut splay_tree;
    fn splaytree_delete(t: *mut splay_tree, key: libc::c_int) -> *mut splay_tree;
    fn splaytree_insert(
        t: *mut splay_tree,
        key: libc::c_int,
        data: *mut libc::c_void,
    ) -> *mut splay_tree;
    fn config_plugin_value_tobool(
        du: *const data_unset,
        default_value: libc::c_int,
    ) -> libc::c_int;
    fn config_plugin_value_to_int32(
        du: *const data_unset,
        default_value: int32_t,
    ) -> int32_t;
    fn config_plugin_values_init(
        srv: *mut server,
        p_d: *mut libc::c_void,
        cpk: *const config_plugin_keys_t,
        mname: *const libc::c_char,
    ) -> libc::c_int;
    fn config_check_cond(r: *mut request_st, context_ndx: libc::c_int) -> libc::c_int;
    fn li_base64_dec(
        result: *mut libc::c_uchar,
        out_length: size_t,
        in_0: *const libc::c_char,
        in_length: size_t,
        charset: base64_charset,
    ) -> size_t;
    fn li_rand_pseudo() -> libc::c_int;
    fn li_rand_pseudo_bytes(buf: *mut libc::c_uchar, num: libc::c_int);
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __uintmax_t = libc::c_ulong;
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
pub type rsize_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uint_fast32_t = libc::c_ulong;
pub type intptr_t = libc::c_long;
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
pub struct const_iovec {
    pub iov_base: *const libc::c_void,
    pub iov_len: size_t,
}
pub type errno_t = libc::c_int;
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
pub type http_auth_digest_type = libc::c_uint;
pub const HTTP_AUTH_DIGEST_SHA512_256: http_auth_digest_type = 8;
pub const HTTP_AUTH_DIGEST_SHA256: http_auth_digest_type = 4;
pub const HTTP_AUTH_DIGEST_MD5: http_auth_digest_type = 2;
pub const HTTP_AUTH_DIGEST_SESS: http_auth_digest_type = 1;
pub const HTTP_AUTH_DIGEST_NONE: http_auth_digest_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_auth_scheme_t {
    pub name: *const libc::c_char,
    pub checkfn: Option::<
        unsafe extern "C" fn(
            *mut request_st,
            *mut libc::c_void,
            *const http_auth_require_t,
            *const http_auth_backend_t,
        ) -> handler_t,
    >,
    pub p_d: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_auth_backend_t {
    pub name: *const libc::c_char,
    pub basic: Option::<
        unsafe extern "C" fn(
            *mut request_st,
            *mut libc::c_void,
            *const http_auth_require_t,
            *const buffer,
            *const libc::c_char,
        ) -> handler_t,
    >,
    pub digest: Option::<
        unsafe extern "C" fn(
            *mut request_st,
            *mut libc::c_void,
            *mut http_auth_info_t,
        ) -> handler_t,
    >,
    pub p_d: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_auth_info_t {
    pub dalgo: libc::c_int,
    pub dlen: libc::c_uint,
    pub username: *const libc::c_char,
    pub ulen: size_t,
    pub realm: *const libc::c_char,
    pub rlen: size_t,
    pub userhash: libc::c_int,
    pub digest: [libc::c_uchar; 32],
    pub userbuf: [libc::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_auth_require_t {
    pub scheme: *const http_auth_scheme_t,
    pub realm: *const buffer,
    pub nonce_secret: *const buffer,
    pub valid_user: uint8_t,
    pub userhash: uint8_t,
    pub algorithm: libc::c_int,
    pub user: array,
    pub group: array,
    pub host: array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5_CTX {
    pub state: [uint32_t; 4],
    pub count: [uint32_t; 2],
    pub buffer: [libc::c_uchar; 64],
}
pub type li_md_iov_fn = Option::<
    unsafe extern "C" fn(*mut libc::c_uchar, *const const_iovec, size_t) -> (),
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tree_node {
    pub left: *mut tree_node,
    pub right: *mut tree_node,
    pub key: libc::c_int,
    pub data: *mut libc::c_void,
}
pub type splay_tree = tree_node;
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
pub struct http_auth_cache {
    pub sptree: *mut splay_tree,
    pub max_age: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_config {
    pub auth_backend: *const http_auth_backend_t,
    pub auth_require: *const array,
    pub auth_cache: *mut http_auth_cache,
    pub auth_extern_authn: libc::c_uint,
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
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_auth_cache_entry {
    pub require: *const http_auth_require_t,
    pub ctime: unix_time64_t,
    pub dalgo: libc::c_int,
    pub dlen: uint32_t,
    pub ulen: uint32_t,
    pub klen: uint32_t,
    pub k: *mut libc::c_char,
    pub username: *mut libc::c_char,
    pub pwdigest: *mut libc::c_char,
}
pub type base64_charset = libc::c_uint;
pub const BASE64_URL: base64_charset = 1;
pub const BASE64_STANDARD: base64_charset = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_auth_digest_params_t {
    pub ptr: [*const libc::c_char; 11],
    pub len: [uint16_t; 11],
    pub send_nextnonce_ts: unix_time64_t,
    pub rdigest: [libc::c_uchar; 20],
}
pub const e_qop: http_auth_digest_params_e = 5;
pub const e_cnonce: http_auth_digest_params_e = 6;
pub const e_nc: http_auth_digest_params_e = 7;
pub const e_nonce: http_auth_digest_params_e = 2;
pub const e_uri: http_auth_digest_params_e = 3;
pub const e_response: http_auth_digest_params_e = 8;
pub const e_algorithm: http_auth_digest_params_e = 4;
pub const e_userstar: http_auth_digest_params_e = 9;
pub const e_userhash: http_auth_digest_params_e = 10;
pub const e_realm: http_auth_digest_params_e = 1;
pub const e_username: http_auth_digest_params_e = 0;
pub type http_auth_digest_params_e = libc::c_uint;
pub const http_auth_digest_params_sz: http_auth_digest_params_e = 11;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct digest_kv {
    pub key: *const libc::c_char,
    pub klen: uint32_t,
    pub id: http_auth_digest_params_e,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_auth {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
    pub require: *mut http_auth_require_t,
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
#[inline]
unsafe extern "C" fn buffer_clear(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
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
unsafe extern "C" fn ck_memzero(mut s: *mut libc::c_void, mut n: rsize_t) -> errno_t {
    return ck_memclear_s(s, n, n);
}
#[inline]
unsafe extern "C" fn MD5_iov(
    digest: *mut libc::c_uchar,
    iov: *const const_iovec,
    n: size_t,
) {
    let mut ctx: MD5_CTX = MD5_CTX {
        state: [0; 4],
        count: [0; 2],
        buffer: [0; 64],
    };
    MD5_Init(&mut ctx);
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n {
        if (*iov.offset(i as isize)).iov_len != 0 {
            MD5_Update(
                &mut ctx,
                (*iov.offset(i as isize)).iov_base,
                (*iov.offset(i as isize)).iov_len as libc::c_uint,
            );
        }
        i = i.wrapping_add(1);
    }
    MD5_Final(digest, &mut ctx);
}
#[inline]
unsafe extern "C" fn djbhash(
    mut str: *const libc::c_char,
    len: uint32_t,
    mut hash: uint32_t,
) -> uint32_t {
    let s: *const libc::c_uchar = str as *const libc::c_uchar;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < len {
        hash = (hash << 5 as libc::c_int).wrapping_add(hash)
            ^ *s.offset(i as isize) as libc::c_uint;
        i = i.wrapping_add(1);
    }
    return hash;
}
unsafe extern "C" fn http_auth_cache_entry_init(
    require: *const http_auth_require_t,
    dalgo: libc::c_int,
    mut k: *const libc::c_char,
    klen: uint32_t,
    mut username: *const libc::c_char,
    ulen: uint32_t,
    mut pw: *const libc::c_char,
    pwlen: uint32_t,
) -> *mut http_auth_cache_entry {
    let ae: *mut http_auth_cache_entry = malloc(
        (::std::mem::size_of::<http_auth_cache_entry>() as libc::c_ulong)
            .wrapping_add(ulen as libc::c_ulong)
            .wrapping_add(pwlen as libc::c_ulong)
            .wrapping_add(
                (if k == username { 0 as libc::c_int as libc::c_uint } else { klen })
                    as libc::c_ulong,
            ),
    ) as *mut http_auth_cache_entry;
    if ae.is_null() {
        ck_assert_failed(
            b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int as libc::c_uint,
            b"ae\0" as *const u8 as *const libc::c_char,
        );
    }
    (*ae).require = require;
    (*ae).ctime = log_monotonic_secs;
    (*ae).dalgo = dalgo;
    (*ae).ulen = ulen;
    (*ae).dlen = pwlen;
    (*ae).klen = klen;
    (*ae).username = ae.offset(1 as libc::c_int as isize) as *mut libc::c_char;
    (*ae).pwdigest = ((*ae).username).offset(ulen as isize);
    (*ae)
        .k = (if k == username {
        (*ae).username as *mut libc::c_void
    } else {
        memcpy(
            ((*ae).pwdigest).offset(pwlen as isize) as *mut libc::c_void,
            k as *const libc::c_void,
            klen as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    memcpy(
        (*ae).username as *mut libc::c_void,
        username as *const libc::c_void,
        ulen as libc::c_ulong,
    );
    memcpy(
        (*ae).pwdigest as *mut libc::c_void,
        pw as *const libc::c_void,
        pwlen as libc::c_ulong,
    );
    return ae;
}
unsafe extern "C" fn http_auth_cache_entry_free(mut data: *mut libc::c_void) {
    let ae: *mut http_auth_cache_entry = data as *mut http_auth_cache_entry;
    ck_memzero((*ae).pwdigest as *mut libc::c_void, (*ae).dlen as rsize_t);
    free(ae as *mut libc::c_void);
}
unsafe extern "C" fn http_auth_cache_free(mut ac: *mut http_auth_cache) {
    let mut sptree: *mut splay_tree = (*ac).sptree;
    while !sptree.is_null() {
        http_auth_cache_entry_free((*sptree).data);
        sptree = splaytree_delete(sptree, (*sptree).key);
    }
    free(ac as *mut libc::c_void);
}
unsafe extern "C" fn http_auth_cache_init(
    mut opts: *const array,
) -> *mut http_auth_cache {
    let mut ac: *mut http_auth_cache = malloc(
        ::std::mem::size_of::<http_auth_cache>() as libc::c_ulong,
    ) as *mut http_auth_cache;
    if ac.is_null() {
        ck_assert_failed(
            b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int as libc::c_uint,
            b"ac\0" as *const u8 as *const libc::c_char,
        );
    }
    (*ac).sptree = 0 as *mut splay_tree;
    (*ac).max_age = 600 as libc::c_int as time_t;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let mut used: uint32_t = (*opts).used;
    while i < used {
        let mut du: *mut data_unset = *((*opts).data).offset(i as isize);
        if buffer_eq_slen(
            &mut (*du).key,
            b"max-age\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            (*ac)
                .max_age = config_plugin_value_to_int32(du, (*ac).max_age as int32_t)
                as time_t;
        }
        i = i.wrapping_add(1);
    }
    return ac;
}
unsafe extern "C" fn http_auth_cache_hash(
    require: *const http_auth_require_t,
    mut username: *const libc::c_char,
    ulen: uint32_t,
) -> libc::c_int {
    let mut h: uint32_t = djbhash(
        require as intptr_t as *mut libc::c_char,
        ::std::mem::size_of::<intptr_t>() as libc::c_ulong as uint32_t,
        5381 as libc::c_int as uint32_t,
    );
    h = djbhash(username, ulen, h);
    return (h & !((1 as libc::c_int as uint32_t) << 31 as libc::c_int)) as int32_t;
}
unsafe extern "C" fn http_auth_cache_query(
    sptree: *mut *mut splay_tree,
    ndx: libc::c_int,
) -> *mut http_auth_cache_entry {
    *sptree = splaytree_splay(*sptree, ndx);
    return (if !(*sptree).is_null() && (**sptree).key == ndx {
        (**sptree).data
    } else {
        0 as *mut libc::c_void
    }) as *mut http_auth_cache_entry;
}
unsafe extern "C" fn http_auth_cache_insert(
    sptree: *mut *mut splay_tree,
    ndx: libc::c_int,
    data: *mut libc::c_void,
    mut data_free_fn: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    if (*sptree).is_null() || (**sptree).key != ndx {
        *sptree = splaytree_insert(*sptree, ndx, data);
    } else {
        data_free_fn.expect("non-null function pointer")((**sptree).data);
        (**sptree).data = data;
    };
}
unsafe extern "C" fn mod_auth_tag_old_entries(
    t: *mut splay_tree,
    keys: *mut libc::c_int,
    ndx: *mut libc::c_int,
    max_age: time_t,
    cur_ts: unix_time64_t,
) {
    if *ndx == 8192 as libc::c_int {
        return;
    }
    if !((*t).left).is_null() {
        mod_auth_tag_old_entries((*t).left, keys, ndx, max_age, cur_ts);
    }
    if !((*t).right).is_null() {
        mod_auth_tag_old_entries((*t).right, keys, ndx, max_age, cur_ts);
    }
    if *ndx == 8192 as libc::c_int {
        return;
    }
    let ae: *const http_auth_cache_entry = (*t).data as *const http_auth_cache_entry;
    if cur_ts - (*ae).ctime > max_age {
        let fresh0 = *ndx;
        *ndx = *ndx + 1;
        *keys.offset(fresh0 as isize) = (*t).key;
    }
}
#[inline(never)]
unsafe extern "C" fn mod_auth_periodic_cleanup(
    mut sptree_ptr: *mut *mut splay_tree,
    max_age: time_t,
    cur_ts: unix_time64_t,
) {
    let mut sptree: *mut splay_tree = *sptree_ptr;
    let mut max_ndx: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut keys: [libc::c_int; 8192] = [0; 8192];
    while !sptree.is_null() {
        max_ndx = 0 as libc::c_int;
        mod_auth_tag_old_entries(
            sptree,
            keys.as_mut_ptr(),
            &mut max_ndx,
            max_age,
            cur_ts,
        );
        i = 0 as libc::c_int;
        while i < max_ndx {
            let mut ndx: libc::c_int = keys[i as usize];
            sptree = splaytree_splay(sptree, ndx);
            if !sptree.is_null() && (*sptree).key == ndx {
                http_auth_cache_entry_free((*sptree).data);
                sptree = splaytree_delete(sptree, ndx);
            }
            i += 1;
        }
        if !(max_ndx as libc::c_ulong
            == (::std::mem::size_of::<[libc::c_int; 8192]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong))
        {
            break;
        }
    }
    *sptree_ptr = sptree;
}
unsafe extern "C" fn mod_auth_periodic(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *const plugin_data = p_d as *const plugin_data;
    let cur_ts: unix_time64_t = log_monotonic_secs;
    if cur_ts & 0x7 as libc::c_int as libc::c_long != 0 {
        return HANDLER_GO_ON;
    }
    if ((*p).cvlist).is_null() {
        return HANDLER_GO_ON;
    }
    let mut i: libc::c_int = ((*((*p).cvlist).offset(0 as libc::c_int as isize))
        .v
        .u2[1 as libc::c_int as usize] == 0) as libc::c_int;
    let mut used: libc::c_int = (*p).nconfig;
    while i < used {
        let mut cpv: *const config_plugin_value_t = ((*p).cvlist)
            .offset(
                (*((*p).cvlist).offset(i as isize)).v.u2[0 as libc::c_int as usize]
                    as isize,
            );
        while (*cpv).k_id != -(1 as libc::c_int) {
            if !((*cpv).k_id != 3 as libc::c_int) {
                if !((*cpv).vtype as libc::c_uint
                    != T_CONFIG_LOCAL as libc::c_int as libc::c_uint)
                {
                    let mut ac: *mut http_auth_cache = (*cpv).v.v
                        as *mut http_auth_cache;
                    mod_auth_periodic_cleanup(&mut (*ac).sptree, (*ac).max_age, cur_ts);
                }
            }
            cpv = cpv.offset(1);
        }
        i += 1;
    }
    return HANDLER_GO_ON;
}
#[cold]
unsafe extern "C" fn mod_auth_init() -> *mut libc::c_void {
    static mut http_auth_scheme_basic: http_auth_scheme_t = unsafe {
        {
            let mut init = http_auth_scheme_t {
                name: b"basic\0" as *const u8 as *const libc::c_char,
                checkfn: Some(
                    mod_auth_check_basic
                        as unsafe extern "C" fn(
                            *mut request_st,
                            *mut libc::c_void,
                            *const http_auth_require_t,
                            *const http_auth_backend_t,
                        ) -> handler_t,
                ),
                p_d: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        }
    };
    static mut http_auth_scheme_digest: http_auth_scheme_t = unsafe {
        {
            let mut init = http_auth_scheme_t {
                name: b"digest\0" as *const u8 as *const libc::c_char,
                checkfn: Some(
                    mod_auth_check_digest
                        as unsafe extern "C" fn(
                            *mut request_st,
                            *mut libc::c_void,
                            *const http_auth_require_t,
                            *const http_auth_backend_t,
                        ) -> handler_t,
                ),
                p_d: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        }
    };
    static mut http_auth_scheme_extern: http_auth_scheme_t = unsafe {
        {
            let mut init = http_auth_scheme_t {
                name: b"extern\0" as *const u8 as *const libc::c_char,
                checkfn: Some(
                    mod_auth_check_extern
                        as unsafe extern "C" fn(
                            *mut request_st,
                            *mut libc::c_void,
                            *const http_auth_require_t,
                            *const http_auth_backend_t,
                        ) -> handler_t,
                ),
                p_d: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        }
    };
    let mut p: *mut plugin_data = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<plugin_data>() as libc::c_ulong,
    ) as *mut plugin_data;
    if p.is_null() {
        ck_assert_failed(
            b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int as libc::c_uint,
            b"p\0" as *const u8 as *const libc::c_char,
        );
    }
    http_auth_scheme_basic.p_d = p as *mut libc::c_void;
    http_auth_scheme_set(&mut http_auth_scheme_basic);
    http_auth_scheme_digest.p_d = p as *mut libc::c_void;
    http_auth_scheme_set(&mut http_auth_scheme_digest);
    http_auth_scheme_set(&http_auth_scheme_extern);
    return p as *mut libc::c_void;
}
#[cold]
unsafe extern "C" fn mod_auth_free(mut p_d: *mut libc::c_void) {
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
                        array_free((*cpv).v.v as *mut array);
                    }
                    3 => {
                        http_auth_cache_free((*cpv).v.v as *mut http_auth_cache);
                    }
                    _ => {}
                }
            }
            cpv = cpv.offset(1);
        }
        i += 1;
    }
    http_auth_dumbdata_reset();
}
unsafe extern "C" fn data_auth_free(mut d: *mut data_unset) {
    let dauth: *mut data_auth = d as *mut data_auth;
    free((*dauth).key.ptr as *mut libc::c_void);
    http_auth_require_free((*dauth).require);
    free(dauth as *mut libc::c_void);
}
unsafe extern "C" fn data_auth_init() -> *mut data_auth {
    static mut fn_0: data_methods = unsafe {
        {
            let mut init = data_methods {
                copy: None,
                free: Some(
                    data_auth_free as unsafe extern "C" fn(*mut data_unset) -> (),
                ),
                insert_dup: None,
            };
            init
        }
    };
    let dauth: *mut data_auth = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<data_auth>() as libc::c_ulong,
    ) as *mut data_auth;
    if dauth.is_null() {
        ck_assert_failed(
            b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
            286 as libc::c_int as libc::c_uint,
            b"((void*)0) != dauth\0" as *const u8 as *const libc::c_char,
        );
    }
    (*dauth).type_0 = TYPE_OTHER;
    (*dauth).fn_0 = &fn_0;
    (*dauth).require = http_auth_require_init();
    return dauth;
}
unsafe extern "C" fn mod_auth_algorithm_parse(
    mut ai: *mut http_auth_info_t,
    mut s: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    if 0 as libc::c_int as libc::c_ulong == len {
        (*ai).dalgo = HTTP_AUTH_DIGEST_MD5 as libc::c_int;
        (*ai).dlen = 16 as libc::c_int as libc::c_uint;
        return 1 as libc::c_int;
    }
    if len > 5 as libc::c_int as libc::c_ulong
        && *s.offset(len.wrapping_sub(5 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '-' as i32
        && *s.offset(len.wrapping_sub(4 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int | 0x20 as libc::c_int == 's' as i32
        && *s.offset(len.wrapping_sub(3 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int | 0x20 as libc::c_int == 'e' as i32
        && *s.offset(len.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int | 0x20 as libc::c_int == 's' as i32
        && *s.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int | 0x20 as libc::c_int == 's' as i32
    {
        (*ai).dalgo = HTTP_AUTH_DIGEST_SESS as libc::c_int;
        len = (len as libc::c_ulong).wrapping_sub(5 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    } else {
        (*ai).dalgo = HTTP_AUTH_DIGEST_NONE as libc::c_int;
    }
    if 3 as libc::c_int as libc::c_ulong == len
        && 'm' as i32
            == *s.offset(0 as libc::c_int as isize) as libc::c_int | 0x20 as libc::c_int
        && 'd' as i32
            == *s.offset(1 as libc::c_int as isize) as libc::c_int | 0x20 as libc::c_int
        && '5' as i32 == *s.offset(2 as libc::c_int as isize) as libc::c_int
    {
        (*ai).dalgo |= HTTP_AUTH_DIGEST_MD5 as libc::c_int;
        (*ai).dlen = 16 as libc::c_int as libc::c_uint;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mod_auth_algorithms_parse(
    mut algorithm: *mut libc::c_int,
    mut algos: *mut buffer,
) -> libc::c_int {
    let mut s: *const libc::c_char = (*algos).ptr;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    while !s.is_null() {
        let mut ai: http_auth_info_t = http_auth_info_t {
            dalgo: 0,
            dlen: 0,
            username: 0 as *const libc::c_char,
            ulen: 0,
            realm: 0 as *const libc::c_char,
            rlen: 0,
            userhash: 0,
            digest: [0; 32],
            userbuf: [0; 256],
        };
        p = strchr(s, '|' as i32);
        if mod_auth_algorithm_parse(
            &mut ai,
            s,
            if !p.is_null() {
                p.offset_from(s) as libc::c_long as size_t
            } else {
                strlen(s)
            },
        ) == 0
        {
            return 0 as libc::c_int;
        }
        *algorithm |= ai.dalgo;
        s = if !p.is_null() {
            p.offset(1 as libc::c_int as isize)
        } else {
            0 as *const libc::c_char
        };
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn mod_auth_require_parse(
    require: *mut http_auth_require_t,
    mut b: *const buffer,
    mut errh: *mut log_error_st,
) -> libc::c_int {
    let mut str: *const libc::c_char = (*b).ptr;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if buffer_eq_slen(
        b,
        b"valid-user\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    ) != 0
    {
        (*require).valid_user = 1 as libc::c_int as uint8_t;
        return 1 as libc::c_int;
    }
    let mut current_block_24: u64;
    loop {
        let mut eq: *const libc::c_char = 0 as *const libc::c_char;
        let mut len: size_t = 0;
        p = strchr(str, '|' as i32);
        len = if !p.is_null() {
            p.offset_from(str) as libc::c_long as size_t
        } else {
            strlen(str)
        };
        eq = memchr(str as *const libc::c_void, '=' as i32, len) as *const libc::c_char;
        if eq.is_null() {
            log_error(
                errh,
                b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
                376 as libc::c_int as libc::c_uint,
                b"error parsing auth.require 'require' field: missing '=' (expecting \"valid-user\" or \"user=a|user=b|group=g|host=h\"). error value: %s error near: %s\0"
                    as *const u8 as *const libc::c_char,
                (*b).ptr,
                str,
            );
            return 0 as libc::c_int;
        }
        if *eq.offset(1 as libc::c_int as isize) as libc::c_int == '|' as i32
            || *eq.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        {
            log_error(
                errh,
                b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
                383 as libc::c_int as libc::c_uint,
                b"error parsing auth.require 'require' field: missing token after '=' (expecting \"valid-user\" or \"user=a|user=b|group=g|host=h\"). error value: %s error near: %s\0"
                    as *const u8 as *const libc::c_char,
                (*b).ptr,
                str,
            );
            return 0 as libc::c_int;
        }
        match eq.offset_from(str) as libc::c_long as libc::c_int {
            4 => {
                if 0 as libc::c_int
                    == memcmp(
                        str as *const libc::c_void,
                        b"user\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong,
                    )
                {
                    array_set_key_value(
                        &mut (*require).user,
                        str.offset(5 as libc::c_int as isize),
                        len.wrapping_sub(5 as libc::c_int as libc::c_ulong) as uint32_t,
                        b"\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    );
                    current_block_24 = 7502529970979898288;
                } else if 0 as libc::c_int
                        == memcmp(
                            str as *const libc::c_void,
                            b"host\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                as libc::c_ulong,
                        )
                    {
                    array_set_key_value(
                        &mut (*require).host,
                        str.offset(5 as libc::c_int as isize),
                        len.wrapping_sub(5 as libc::c_int as libc::c_ulong) as uint32_t,
                        b"\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    );
                    log_error(
                        errh,
                        b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
                        401 as libc::c_int as libc::c_uint,
                        b"warning parsing auth.require 'require' field: 'host' not implemented; field value: %s\0"
                            as *const u8 as *const libc::c_char,
                        (*b).ptr,
                    );
                    current_block_24 = 7502529970979898288;
                } else {
                    current_block_24 = 17281240262373992796;
                }
            }
            5 => {
                if 0 as libc::c_int
                    == memcmp(
                        str as *const libc::c_void,
                        b"group\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong,
                    )
                {
                    array_set_key_value(
                        &mut (*require).group,
                        str.offset(6 as libc::c_int as isize),
                        len.wrapping_sub(6 as libc::c_int as libc::c_ulong) as uint32_t,
                        b"\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    );
                    current_block_24 = 7502529970979898288;
                } else {
                    current_block_24 = 17281240262373992796;
                }
            }
            10 => {
                if 0 as libc::c_int
                    == memcmp(
                        str as *const libc::c_void,
                        b"valid-user\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong,
                    )
                {
                    log_error(
                        errh,
                        b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
                        421 as libc::c_int as libc::c_uint,
                        b"error parsing auth.require 'require' field: valid user can not be combined with other require rules (expecting \"valid-user\" or \"user=a|user=b|group=g|host=h\"). error value: %s\0"
                            as *const u8 as *const libc::c_char,
                        (*b).ptr,
                    );
                    return 0 as libc::c_int;
                }
                current_block_24 = 17281240262373992796;
            }
            _ => {
                current_block_24 = 17281240262373992796;
            }
        }
        match current_block_24 {
            7502529970979898288 => {
                if !(!p.is_null()
                    && {
                        str = p.offset(1 as libc::c_int as isize);
                        *str as libc::c_int != 0
                    })
                {
                    break;
                }
            }
            _ => {
                log_error(
                    errh,
                    b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
                    433 as libc::c_int as libc::c_uint,
                    b"error parsing auth.require 'require' field: invalid/unsupported token (expecting \"valid-user\" or \"user=a|user=b|group=g|host=h\"). error value: %s error near: %s\0"
                        as *const u8 as *const libc::c_char,
                    (*b).ptr,
                    str,
                );
                return 0 as libc::c_int;
            }
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn mod_auth_require_parse_array(
    mut value: *const array,
    auth_require: *mut array,
    mut errh: *mut log_error_st,
) -> handler_t {
    let mut n: uint32_t = 0 as libc::c_int as uint32_t;
    while n < (*value).used {
        let mut m: size_t = 0;
        let mut da_file: *mut data_array = *((*value).data).offset(n as isize)
            as *mut data_array;
        let mut method: *const buffer = 0 as *const buffer;
        let mut realm: *const buffer = 0 as *const buffer;
        let mut require: *const buffer = 0 as *const buffer;
        let mut nonce_secret: *const buffer = 0 as *const buffer;
        let mut userhash: *mut data_unset = 0 as *mut data_unset;
        let mut auth_scheme: *const http_auth_scheme_t = 0 as *const http_auth_scheme_t;
        let mut algos: *mut buffer = 0 as *mut buffer;
        let mut algorithm: libc::c_int = HTTP_AUTH_DIGEST_SESS as libc::c_int;
        if array_is_kvstring(&mut (*da_file).value) == 0 {
            log_error(
                errh,
                b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
                458 as libc::c_int as libc::c_uint,
                b"unexpected value for auth.require; expected auth.require = ( \"urlpath\" => ( \"option\" => \"value\" ) )\0"
                    as *const u8 as *const libc::c_char,
            );
            return HANDLER_ERROR;
        }
        m = 0 as libc::c_int as size_t;
        while m < (*da_file).value.used as libc::c_ulong {
            if (**((*da_file).value.data).offset(m as isize)).type_0 as libc::c_uint
                == TYPE_STRING as libc::c_int as libc::c_uint
            {
                let mut ds: *mut data_string = *((*da_file).value.data)
                    .offset(m as isize) as *mut data_string;
                if buffer_eq_slen(
                    &mut (*ds).key,
                    b"method\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                ) != 0
                {
                    method = &mut (*ds).value;
                } else if buffer_eq_slen(
                        &mut (*ds).key,
                        b"realm\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    ) != 0
                    {
                    realm = &mut (*ds).value;
                } else if buffer_eq_slen(
                        &mut (*ds).key,
                        b"require\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    ) != 0
                    {
                    require = &mut (*ds).value;
                } else if buffer_eq_slen(
                        &mut (*ds).key,
                        b"algorithm\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    ) != 0
                    {
                    algos = &mut (*ds).value;
                } else if buffer_eq_slen(
                        &mut (*ds).key,
                        b"nonce_secret\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    ) != 0
                        || buffer_eq_slen(
                            &mut (*ds).key,
                            b"nonce-secret\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        ) != 0
                    {
                    nonce_secret = &mut (*ds).value;
                } else if buffer_eq_slen(
                        &mut (*ds).key,
                        b"userhash\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    ) != 0
                    {
                    userhash = ds as *mut data_unset;
                } else {
                    log_error(
                        errh,
                        b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
                        482 as libc::c_int as libc::c_uint,
                        b"the field is unknown in: auth.require = ( \"...\" => ( ..., -> \"%s\" <- => \"...\" ) )\0"
                            as *const u8 as *const libc::c_char,
                        (**((*da_file).value.data).offset(m as isize)).key.ptr,
                    );
                    return HANDLER_ERROR;
                }
            } else {
                log_error(
                    errh,
                    b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
                    490 as libc::c_int as libc::c_uint,
                    b"a string was expected for: auth.require = ( \"...\" => ( ..., -> \"%s\" <- => \"...\" ) )\0"
                        as *const u8 as *const libc::c_char,
                    (**((*da_file).value.data).offset(m as isize)).key.ptr,
                );
                return HANDLER_ERROR;
            }
            m = m.wrapping_add(1);
        }
        if method.is_null() || buffer_is_blank(method) != 0 {
            log_error(
                errh,
                b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
                500 as libc::c_int as libc::c_uint,
                b"the method field is missing or blank in: auth.require = ( \"...\" => ( ..., \"method\" => \"...\" ) )\0"
                    as *const u8 as *const libc::c_char,
            );
            return HANDLER_ERROR;
        } else {
            auth_scheme = http_auth_scheme_get(method);
            if auth_scheme.is_null() {
                log_error(
                    errh,
                    b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
                    507 as libc::c_int as libc::c_uint,
                    b"unknown method %s (e.g. \"basic\", \"digest\" or \"extern\") in auth.require = ( \"...\" => ( ..., \"method\" => \"...\") )\0"
                        as *const u8 as *const libc::c_char,
                    (*method).ptr,
                );
                return HANDLER_ERROR;
            }
        }
        if realm.is_null() {
            log_error(
                errh,
                b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
                515 as libc::c_int as libc::c_uint,
                b"the realm field is missing in: auth.require = ( \"...\" => ( ..., \"realm\" => \"...\" ) )\0"
                    as *const u8 as *const libc::c_char,
            );
            return HANDLER_ERROR;
        }
        if require.is_null() || buffer_is_blank(require) != 0 {
            log_error(
                errh,
                b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
                522 as libc::c_int as libc::c_uint,
                b"the require field is missing or blank in: auth.require = ( \"...\" => ( ..., \"require\" => \"...\" ) )\0"
                    as *const u8 as *const libc::c_char,
            );
            return HANDLER_ERROR;
        }
        if algos.is_null() || buffer_is_blank(algos) != 0 {
            algorithm |= HTTP_AUTH_DIGEST_MD5 as libc::c_int;
        } else if mod_auth_algorithms_parse(&mut algorithm, algos) == 0 {
            log_error(
                errh,
                b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
                531 as libc::c_int as libc::c_uint,
                b"invalid algorithm in: auth.require = ( \"...\" => ( ..., \"algorithm\" => \"...\" ) )\0"
                    as *const u8 as *const libc::c_char,
            );
            return HANDLER_ERROR;
        }
        if !require.is_null() {
            let dauth: *mut data_auth = data_auth_init();
            buffer_copy_buffer(&mut (*dauth).key, &mut (*da_file).key);
            (*(*dauth).require).scheme = auth_scheme;
            (*(*dauth).require).algorithm = algorithm;
            (*(*dauth).require).realm = realm;
            (*(*dauth).require).nonce_secret = nonce_secret;
            (*(*dauth).require)
                .userhash = config_plugin_value_tobool(userhash, 0 as libc::c_int)
                as uint8_t;
            if mod_auth_require_parse((*dauth).require, require, errh) == 0 {
                ((*(*dauth).fn_0).free)
                    .expect("non-null function pointer")(dauth as *mut data_unset);
                return HANDLER_ERROR;
            }
            array_insert_unique(auth_require, dauth as *mut data_unset);
        }
        n = n.wrapping_add(1);
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_auth_merge_config_cpv(
    pconf: *mut plugin_config,
    cpv: *const config_plugin_value_t,
) {
    match (*cpv).k_id {
        0 => {
            if (*cpv).vtype as libc::c_uint
                == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
            {
                (*pconf).auth_backend = (*cpv).v.v as *const http_auth_backend_t;
            }
        }
        1 => {
            if (*cpv).vtype as libc::c_uint
                == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
            {
                (*pconf).auth_require = (*cpv).v.v as *const array;
            }
        }
        2 => {
            (*pconf).auth_extern_authn = (*cpv).v.u;
        }
        3 => {
            if (*cpv).vtype as libc::c_uint
                == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
            {
                (*pconf).auth_cache = (*cpv).v.v as *mut http_auth_cache;
            }
        }
        _ => return,
    };
}
unsafe extern "C" fn mod_auth_merge_config(
    pconf: *mut plugin_config,
    mut cpv: *const config_plugin_value_t,
) {
    loop {
        mod_auth_merge_config_cpv(pconf, cpv);
        cpv = cpv.offset(1);
        if !((*cpv).k_id != -(1 as libc::c_int)) {
            break;
        }
    };
}
unsafe extern "C" fn mod_auth_patch_config(r: *mut request_st, p: *mut plugin_data) {
    (*p).conf = (*p).defaults;
    let mut i: libc::c_int = 1 as libc::c_int;
    let mut used: libc::c_int = (*p).nconfig;
    while i < used {
        if config_check_cond(
            r,
            (*((*p).cvlist).offset(i as isize)).k_id as uint32_t as libc::c_int,
        ) != 0
        {
            mod_auth_merge_config(
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
unsafe extern "C" fn mod_auth_set_defaults(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    if config_plugin_values_init(
        srv,
        p as *mut libc::c_void,
        cpk.as_ptr(),
        b"mod_auth\0" as *const u8 as *const libc::c_char,
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
            match (*cpv).k_id {
                0 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        let auth_backend: *const http_auth_backend_t = http_auth_backend_get(
                            (*cpv).v.b,
                        );
                        if auth_backend.is_null() {
                            log_error(
                                (*srv).errh,
                                b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
                                627 as libc::c_int as libc::c_uint,
                                b"auth.backend not supported: %s\0" as *const u8
                                    as *const libc::c_char,
                                (*(*cpv).v.b).ptr,
                            );
                            return HANDLER_ERROR;
                        }
                        let ref mut fresh1 = *(&mut (*cpv).v.v as *mut *mut libc::c_void
                            as *mut *const http_auth_backend_t);
                        *fresh1 = auth_backend;
                        (*cpv).vtype = T_CONFIG_LOCAL;
                    }
                }
                1 => {
                    let a: *mut array = array_init(4 as libc::c_int as uint32_t);
                    if HANDLER_GO_ON as libc::c_int as libc::c_uint
                        != mod_auth_require_parse_array((*cpv).v.a, a, (*srv).errh)
                            as libc::c_uint
                    {
                        array_free(a);
                        return HANDLER_ERROR;
                    }
                    (*cpv).v.a = a;
                    (*cpv).vtype = T_CONFIG_LOCAL;
                }
                2 => {}
                3 => {
                    (*cpv).v.v = http_auth_cache_init((*cpv).v.a) as *mut libc::c_void;
                    (*cpv).vtype = T_CONFIG_LOCAL;
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
            mod_auth_merge_config(&mut (*p).defaults, cpv_0);
        }
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_auth_uri_handler(
    r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    let mut dauth: *mut data_auth = 0 as *mut data_auth;
    mod_auth_patch_config(r, p);
    if ((*p).conf.auth_require).is_null() {
        return HANDLER_GO_ON;
    }
    dauth = if (*r).conf.force_lowercase_filenames == 0 {
        array_match_key_prefix((*p).conf.auth_require, &mut (*r).uri.path)
            as *mut data_auth
    } else {
        array_match_key_prefix_nc((*p).conf.auth_require, &mut (*r).uri.path)
            as *mut data_auth
    };
    if dauth.is_null() {
        return HANDLER_GO_ON;
    }
    let scheme: *const http_auth_scheme_t = (*(*dauth).require).scheme;
    if (*p).conf.auth_extern_authn != 0 {
        let mut vb: *const buffer = http_header_env_get(
            r,
            b"REMOTE_USER\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        if !vb.is_null()
            && http_auth_match_rules(
                (*dauth).require,
                (*vb).ptr,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ) != 0
        {
            return HANDLER_GO_ON;
        }
    }
    return ((*scheme).checkfn)
        .expect(
            "non-null function pointer",
        )(r, (*scheme).p_d, (*dauth).require, (*p).conf.auth_backend);
}
#[no_mangle]
pub unsafe extern "C" fn mod_auth_plugin_init(mut p: *mut plugin) -> libc::c_int {
    (*p).version = 0x10440 as libc::c_int as size_t;
    (*p).name = b"auth\0" as *const u8 as *const libc::c_char;
    (*p)
        .init = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
    >(Some(mod_auth_init as unsafe extern "C" fn() -> *mut libc::c_void));
    (*p)
        .set_defaults = Some(
        mod_auth_set_defaults
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_trigger = Some(
        mod_auth_periodic
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_uri_clean = Some(
        mod_auth_uri_handler
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p).cleanup = Some(mod_auth_free as unsafe extern "C" fn(*mut libc::c_void) -> ());
    return 0 as libc::c_int;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn mod_auth_send_400_bad_request(r: *mut request_st) -> handler_t {
    (*r).http_status = 400 as libc::c_int;
    (*r).handler_module = 0 as *const plugin;
    return HANDLER_FINISHED;
}
#[inline(never)]
unsafe extern "C" fn mod_auth_send_401_unauthorized_basic(
    r: *mut request_st,
    realm: *const buffer,
) -> handler_t {
    (*r).http_status = 401 as libc::c_int;
    (*r).handler_module = 0 as *const plugin;
    buffer_append_str3(
        http_header_response_set_ptr(
            r,
            HTTP_HEADER_WWW_AUTHENTICATE,
            b"WWW-Authenticate\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        ),
        b"Basic realm=\"\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        (*realm).ptr,
        buffer_clen(realm) as size_t,
        b"\", charset=\"UTF-8\"\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    return HANDLER_FINISHED;
}
#[cold]
unsafe extern "C" fn mod_auth_basic_misconfigured(
    r: *mut request_st,
    backend: *const http_auth_backend_t,
) -> handler_t {
    if backend.is_null() {
        log_error(
            (*r).conf.errh,
            b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
            758 as libc::c_int as libc::c_uint,
            b"auth.backend not configured for %s\0" as *const u8 as *const libc::c_char,
            (*r).uri.path.ptr,
        );
    } else {
        log_error(
            (*r).conf.errh,
            b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
            761 as libc::c_int as libc::c_uint,
            b"auth.require \"method\" => \"basic\" invalid (try \"digest\"?) for %s\0"
                as *const u8 as *const libc::c_char,
            (*r).uri.path.ptr,
        );
    }
    (*r).http_status = 500 as libc::c_int;
    (*r).handler_module = 0 as *const plugin;
    return HANDLER_FINISHED;
}
unsafe extern "C" fn mod_auth_check_basic(
    r: *mut request_st,
    mut p_d: *mut libc::c_void,
    require: *const http_auth_require_t,
    backend: *const http_auth_backend_t,
) -> handler_t {
    if backend.is_null() || ((*backend).basic).is_none() {
        return mod_auth_basic_misconfigured(r, backend);
    }
    let vb: *const buffer = http_header_request_get(
        r,
        HTTP_HEADER_AUTHORIZATION,
        b"Authorization\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if vb.is_null()
        || buffer_eq_icase_ssn(
            (*vb).ptr,
            b"Basic \0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) == 0
    {
        return mod_auth_send_401_unauthorized_basic(r, (*require).realm);
    }
    let mut ulen: size_t = (buffer_clen(vb) as libc::c_ulong)
        .wrapping_sub(
            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    let mut pwlen: size_t = 0;
    let mut pw: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut user: [libc::c_char; 1024] = [0; 1024];
    if ulen > 1363 as libc::c_int as libc::c_ulong {
        return mod_auth_send_401_unauthorized_basic(r, (*require).realm);
    }
    ulen = li_base64_dec(
        user.as_mut_ptr() as *mut libc::c_uchar,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        ((*vb).ptr)
            .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize)
            .offset(-(1 as libc::c_int as isize)),
        ulen,
        BASE64_STANDARD,
    );
    if 0 as libc::c_int as libc::c_ulong == ulen {
        log_error(
            (*r).conf.errh,
            b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
            800 as libc::c_int as libc::c_uint,
            b"decoding base64-string failed %s\0" as *const u8 as *const libc::c_char,
            ((*vb).ptr)
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize,
                )
                .offset(-(1 as libc::c_int as isize)),
        );
        return mod_auth_send_400_bad_request(r);
    }
    user[ulen as usize] = '\u{0}' as i32 as libc::c_char;
    pw = memchr(user.as_mut_ptr() as *const libc::c_void, ':' as i32, ulen)
        as *mut libc::c_char;
    if pw.is_null() {
        log_error(
            (*r).conf.errh,
            b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
            807 as libc::c_int as libc::c_uint,
            b"missing ':' in %s\0" as *const u8 as *const libc::c_char,
            user.as_mut_ptr(),
        );
        return mod_auth_send_400_bad_request(r);
    }
    let fresh2 = pw;
    pw = pw.offset(1);
    *fresh2 = '\u{0}' as i32 as libc::c_char;
    pwlen = user.as_mut_ptr().offset(ulen as isize).offset_from(pw) as libc::c_long
        as size_t;
    ulen = pw.offset(-(1 as libc::c_int as isize)).offset_from(user.as_mut_ptr())
        as libc::c_long as size_t;
    let p: *mut plugin_data = p_d as *mut plugin_data;
    let mut sptree: *mut *mut splay_tree = if !((*p).conf.auth_cache).is_null() {
        &mut (*(*p).conf.auth_cache).sptree
    } else {
        0 as *mut *mut splay_tree
    };
    let mut ae: *mut http_auth_cache_entry = 0 as *mut http_auth_cache_entry;
    let mut rc: handler_t = HANDLER_ERROR;
    let mut ndx: libc::c_int = -(1 as libc::c_int);
    if !sptree.is_null() {
        ndx = http_auth_cache_hash(require, user.as_mut_ptr(), ulen as uint32_t);
        ae = http_auth_cache_query(sptree, ndx);
        if !ae.is_null() && (*ae).require == require
            && ulen == (*ae).ulen as libc::c_ulong
            && 0 as libc::c_int
                == memcmp(
                    user.as_mut_ptr() as *const libc::c_void,
                    (*ae).username as *const libc::c_void,
                    ulen,
                )
        {
            rc = (if ck_memeq_const_time(
                (*ae).pwdigest as *const libc::c_void,
                (*ae).dlen as size_t,
                pw as *const libc::c_void,
                pwlen,
            ) != 0
            {
                HANDLER_GO_ON as libc::c_int
            } else {
                HANDLER_ERROR as libc::c_int
            }) as handler_t;
        } else {
            ae = 0 as *mut http_auth_cache_entry;
        }
    }
    if ae.is_null() {
        let userb: buffer = {
            let mut init = buffer {
                ptr: user.as_mut_ptr(),
                used: ulen.wrapping_add(1 as libc::c_int as libc::c_ulong) as uint32_t,
                size: 0 as libc::c_int as uint32_t,
            };
            init
        };
        rc = ((*backend).basic)
            .expect("non-null function pointer")(r, (*backend).p_d, require, &userb, pw);
    }
    match rc as libc::c_uint {
        0 => {
            http_auth_setenv(
                r,
                user.as_mut_ptr(),
                ulen,
                b"Basic\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            if !sptree.is_null() && ae.is_null() {
                ae = http_auth_cache_entry_init(
                    require,
                    0 as libc::c_int,
                    user.as_mut_ptr(),
                    ulen as uint32_t,
                    user.as_mut_ptr(),
                    ulen as uint32_t,
                    pw,
                    pwlen as uint32_t,
                );
                http_auth_cache_insert(
                    sptree,
                    ndx,
                    ae as *mut libc::c_void,
                    Some(
                        http_auth_cache_entry_free
                            as unsafe extern "C" fn(*mut libc::c_void) -> (),
                    ),
                );
            }
        }
        3 | 1 => {}
        4 | _ => {
            log_error(
                (*r).conf.errh,
                b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
                852 as libc::c_int as libc::c_uint,
                b"password doesn't match for %s username: %s IP: %s\0" as *const u8
                    as *const libc::c_char,
                (*r).uri.path.ptr,
                user.as_mut_ptr(),
                (*(*r).con).dst_addr_buf.ptr,
            );
            (*r).keep_alive = -(1 as libc::c_int) as int8_t;
            rc = mod_auth_send_401_unauthorized_basic(r, (*require).realm);
        }
    }
    ck_memzero(pw as *mut libc::c_void, pwlen);
    return rc;
}
unsafe extern "C" fn mod_auth_digest_mutate(
    ai: *mut http_auth_info_t,
    dp: *const http_auth_digest_params_t,
    method: *const buffer,
) {
    if method.is_null() {
        ck_assert_failed(
            b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
            892 as libc::c_int as libc::c_uint,
            b"method\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut digest_iov: li_md_iov_fn = Some(
        MD5_iov
            as unsafe extern "C" fn(*mut libc::c_uchar, *const const_iovec, size_t) -> (),
    );
    let mut n: size_t = 0;
    let mut iov: [const_iovec; 11] = [const_iovec {
        iov_base: 0 as *const libc::c_void,
        iov_len: 0,
    }; 11];
    let mut a1: [libc::c_char; 48] = [0; 48];
    let mut a2: [libc::c_char; 48] = [0; 48];
    li_tohex_lc(
        a1.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong,
        ((*ai).digest).as_mut_ptr() as *const libc::c_char,
        (*ai).dlen as size_t,
    );
    if (*ai).dalgo & HTTP_AUTH_DIGEST_SESS as libc::c_int != 0 {
        iov[0 as libc::c_int as usize].iov_base = a1.as_mut_ptr() as *const libc::c_void;
        iov[0 as libc::c_int as usize]
            .iov_len = ((*ai).dlen).wrapping_mul(2 as libc::c_int as libc::c_uint)
            as size_t;
        iov[1 as libc::c_int as usize]
            .iov_base = b":\0" as *const u8 as *const libc::c_char
            as *const libc::c_void;
        iov[1 as libc::c_int as usize].iov_len = 1 as libc::c_int as size_t;
        iov[2 as libc::c_int as usize]
            .iov_base = (*dp).ptr[e_nonce as libc::c_int as usize]
            as *const libc::c_void;
        iov[2 as libc::c_int as usize]
            .iov_len = (*dp).len[e_nonce as libc::c_int as usize] as size_t;
        iov[3 as libc::c_int as usize]
            .iov_base = b":\0" as *const u8 as *const libc::c_char
            as *const libc::c_void;
        iov[3 as libc::c_int as usize].iov_len = 1 as libc::c_int as size_t;
        iov[4 as libc::c_int as usize]
            .iov_base = (*dp).ptr[e_cnonce as libc::c_int as usize]
            as *const libc::c_void;
        iov[4 as libc::c_int as usize]
            .iov_len = (*dp).len[e_cnonce as libc::c_int as usize] as size_t;
        digest_iov
            .expect(
                "non-null function pointer",
            )(((*ai).digest).as_mut_ptr(), iov.as_mut_ptr(), 5 as libc::c_int as size_t);
        li_tohex_lc(
            a1.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong,
            ((*ai).digest).as_mut_ptr() as *const libc::c_char,
            (*ai).dlen as size_t,
        );
    }
    iov[0 as libc::c_int as usize].iov_base = (*method).ptr as *const libc::c_void;
    iov[0 as libc::c_int as usize].iov_len = buffer_clen(method) as size_t;
    iov[1 as libc::c_int as usize]
        .iov_base = b":\0" as *const u8 as *const libc::c_char as *const libc::c_void;
    iov[1 as libc::c_int as usize].iov_len = 1 as libc::c_int as size_t;
    iov[2 as libc::c_int as usize]
        .iov_base = (*dp).ptr[e_uri as libc::c_int as usize] as *const libc::c_void;
    iov[2 as libc::c_int as usize]
        .iov_len = (*dp).len[e_uri as libc::c_int as usize] as size_t;
    n = 3 as libc::c_int as size_t;
    digest_iov
        .expect(
            "non-null function pointer",
        )(((*ai).digest).as_mut_ptr(), iov.as_mut_ptr(), n);
    li_tohex_lc(
        a2.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong,
        ((*ai).digest).as_mut_ptr() as *const libc::c_char,
        (*ai).dlen as size_t,
    );
    iov[0 as libc::c_int as usize].iov_base = a1.as_mut_ptr() as *const libc::c_void;
    iov[0 as libc::c_int as usize]
        .iov_len = ((*ai).dlen).wrapping_mul(2 as libc::c_int as libc::c_uint) as size_t;
    iov[1 as libc::c_int as usize]
        .iov_base = b":\0" as *const u8 as *const libc::c_char as *const libc::c_void;
    iov[1 as libc::c_int as usize].iov_len = 1 as libc::c_int as size_t;
    iov[2 as libc::c_int as usize]
        .iov_base = (*dp).ptr[e_nonce as libc::c_int as usize] as *const libc::c_void;
    iov[2 as libc::c_int as usize]
        .iov_len = (*dp).len[e_nonce as libc::c_int as usize] as size_t;
    iov[3 as libc::c_int as usize]
        .iov_base = b":\0" as *const u8 as *const libc::c_char as *const libc::c_void;
    iov[3 as libc::c_int as usize].iov_len = 1 as libc::c_int as size_t;
    n = 4 as libc::c_int as size_t;
    if (*dp).len[e_qop as libc::c_int as usize] != 0 {
        iov[4 as libc::c_int as usize]
            .iov_base = (*dp).ptr[e_nc as libc::c_int as usize] as *const libc::c_void;
        iov[4 as libc::c_int as usize]
            .iov_len = (*dp).len[e_nc as libc::c_int as usize] as size_t;
        iov[5 as libc::c_int as usize]
            .iov_base = b":\0" as *const u8 as *const libc::c_char
            as *const libc::c_void;
        iov[5 as libc::c_int as usize].iov_len = 1 as libc::c_int as size_t;
        iov[6 as libc::c_int as usize]
            .iov_base = (*dp).ptr[e_cnonce as libc::c_int as usize]
            as *const libc::c_void;
        iov[6 as libc::c_int as usize]
            .iov_len = (*dp).len[e_cnonce as libc::c_int as usize] as size_t;
        iov[7 as libc::c_int as usize]
            .iov_base = b":\0" as *const u8 as *const libc::c_char
            as *const libc::c_void;
        iov[7 as libc::c_int as usize].iov_len = 1 as libc::c_int as size_t;
        iov[8 as libc::c_int as usize]
            .iov_base = (*dp).ptr[e_qop as libc::c_int as usize] as *const libc::c_void;
        iov[8 as libc::c_int as usize]
            .iov_len = (*dp).len[e_qop as libc::c_int as usize] as size_t;
        iov[9 as libc::c_int as usize]
            .iov_base = b":\0" as *const u8 as *const libc::c_char
            as *const libc::c_void;
        iov[9 as libc::c_int as usize].iov_len = 1 as libc::c_int as size_t;
        n = 10 as libc::c_int as size_t;
    }
    iov[n as usize].iov_base = a2.as_mut_ptr() as *const libc::c_void;
    iov[n as usize]
        .iov_len = ((*ai).dlen).wrapping_mul(2 as libc::c_int as libc::c_uint) as size_t;
    digest_iov
        .expect(
            "non-null function pointer",
        )(
        ((*ai).digest).as_mut_ptr(),
        iov.as_mut_ptr(),
        n.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
}
unsafe extern "C" fn mod_auth_append_nonce(
    mut b: *mut buffer,
    mut cur_ts: unix_time64_t,
    mut require: *const http_auth_require_t,
    mut dalgo: libc::c_int,
    mut rndptr: *mut libc::c_int,
) {
    buffer_append_uint_hex_lc(b, cur_ts as uintmax_t);
    buffer_append_string_len(
        b,
        b":\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    let nonce_secret: *const buffer = (*require).nonce_secret;
    let mut rnd: libc::c_int = 0;
    if nonce_secret.is_null() {
        rnd = if !rndptr.is_null() { *rndptr } else { li_rand_pseudo() };
    } else {
        if !rndptr.is_null() {
            rnd = *rndptr;
        } else {
            li_rand_pseudo_bytes(
                &mut rnd as *mut libc::c_int as *mut libc::c_uchar,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
            );
        };
        buffer_append_uint_hex_lc(b, rnd as uintmax_t);
        buffer_append_string_len(
            b,
            b":\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    let mut n: size_t = 0;
    let mut iov: [const_iovec; 3] = [const_iovec {
        iov_base: 0 as *const libc::c_void,
        iov_len: 0,
    }; 3];
    iov[0 as libc::c_int as usize]
        .iov_base = &mut cur_ts as *mut unix_time64_t as *const libc::c_void;
    iov[0 as libc::c_int as usize]
        .iov_len = ::std::mem::size_of::<unix_time64_t>() as libc::c_ulong;
    iov[1 as libc::c_int as usize]
        .iov_base = &mut rnd as *mut libc::c_int as *const libc::c_void;
    iov[1 as libc::c_int as usize]
        .iov_len = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong;
    n = 2 as libc::c_int as size_t;
    if !nonce_secret.is_null() {
        iov[2 as libc::c_int as usize]
            .iov_base = (*nonce_secret).ptr as *const libc::c_void;
        iov[2 as libc::c_int as usize].iov_len = buffer_clen(nonce_secret) as size_t;
        n = 3 as libc::c_int as size_t;
    }
    let mut h: [libc::c_uchar; 20] = [0; 20];
    match dalgo {
        _ => {}
    }
    MD5_iov(h.as_mut_ptr(), iov.as_mut_ptr(), n);
    n = 16 as libc::c_int as size_t;
    li_tohex_lc(
        buffer_extend(b, n.wrapping_mul(2 as libc::c_int as libc::c_ulong)),
        n
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
        h.as_mut_ptr() as *const libc::c_char,
        n,
    );
}
unsafe extern "C" fn mod_auth_digest_www_authenticate(
    mut b: *mut buffer,
    mut cur_ts: unix_time64_t,
    mut require: *const http_auth_require_t,
    mut nonce_stale: libc::c_int,
) {
    let mut algos: libc::c_int = if nonce_stale != 0 {
        nonce_stale
    } else {
        (*require).algorithm
    };
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut algoid: [libc::c_int; 3] = [0; 3];
    let mut algolen: [libc::c_uint; 3] = [0; 3];
    let mut algoname: [*const libc::c_char; 3] = [0 as *const libc::c_char; 3];
    if algos & HTTP_AUTH_DIGEST_MD5 as libc::c_int != 0 {
        algoid[n as usize] = HTTP_AUTH_DIGEST_MD5 as libc::c_int;
        algoname[n as usize] = b"MD5\0" as *const u8 as *const libc::c_char;
        algolen[n
            as usize] = (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uint;
        n += 1;
    }
    buffer_clear(b);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let mut iov: [const_iovec; 6] = [
            {
                let mut init = const_iovec {
                    iov_base: b"\r\nWWW-Authenticate: \0" as *const u8
                        as *const libc::c_char as *const libc::c_void,
                    iov_len: (::std::mem::size_of::<[libc::c_char; 21]>()
                        as libc::c_ulong as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                };
                init
            },
            {
                let mut init = const_iovec {
                    iov_base: b"Digest realm=\"\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    iov_len: (::std::mem::size_of::<[libc::c_char; 15]>()
                        as libc::c_ulong as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                };
                init
            },
            {
                let mut init = const_iovec {
                    iov_base: (*(*require).realm).ptr as *const libc::c_void,
                    iov_len: buffer_clen((*require).realm) as size_t,
                };
                init
            },
            {
                let mut init = const_iovec {
                    iov_base: b"\", charset=\"UTF-8\", algorithm=\0" as *const u8
                        as *const libc::c_char as *const libc::c_void,
                    iov_len: (::std::mem::size_of::<[libc::c_char; 31]>()
                        as libc::c_ulong as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                };
                init
            },
            {
                let mut init = const_iovec {
                    iov_base: algoname[i as usize] as *const libc::c_void,
                    iov_len: algolen[i as usize] as size_t,
                };
                init
            },
            {
                let mut init = const_iovec {
                    iov_base: b", nonce=\"\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    iov_len: (::std::mem::size_of::<[libc::c_char; 10]>()
                        as libc::c_ulong as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                };
                init
            },
        ];
        buffer_append_iovec(
            b,
            iov.as_mut_ptr().offset((0 as libc::c_int == i) as libc::c_int as isize),
            (::std::mem::size_of::<[const_iovec; 6]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<const_iovec>() as libc::c_ulong)
                .wrapping_sub((0 as libc::c_int == i) as libc::c_int as libc::c_ulong),
        );
        mod_auth_append_nonce(
            b,
            cur_ts,
            require,
            algoid[i as usize],
            0 as *mut libc::c_int,
        );
        buffer_append_string_len(
            b,
            b"\", qop=\"auth\"\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        if (*require).userhash != 0 {
            buffer_append_string_len(
                b,
                b", userhash=true\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
        if nonce_stale != 0 {
            buffer_append_string_len(
                b,
                b", stale=true\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
        i += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn mod_auth_send_401_unauthorized_digest(
    r: *mut request_st,
    require: *const http_auth_require_t,
    mut nonce_stale: libc::c_int,
) -> handler_t {
    (*r).http_status = 401 as libc::c_int;
    (*r).handler_module = 0 as *const plugin;
    mod_auth_digest_www_authenticate(
        http_header_response_set_ptr(
            r,
            HTTP_HEADER_WWW_AUTHENTICATE,
            b"WWW-Authenticate\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        ),
        log_epoch_secs,
        require,
        nonce_stale,
    );
    return HANDLER_FINISHED;
}
unsafe extern "C" fn mod_auth_digest_authentication_info(
    mut b: *mut buffer,
    mut cur_ts: unix_time64_t,
    mut require: *const http_auth_require_t,
    mut dalgo: libc::c_int,
) {
    buffer_clear(b);
    buffer_append_string_len(
        b,
        b"nextnonce=\"\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    mod_auth_append_nonce(b, cur_ts, require, dalgo, 0 as *mut libc::c_int);
    buffer_append_string_len(
        b,
        b"\"\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
}
unsafe extern "C" fn mod_auth_digest_get(
    r: *mut request_st,
    mut p_d: *mut libc::c_void,
    require: *const http_auth_require_t,
    backend: *const http_auth_backend_t,
    ai: *mut http_auth_info_t,
) -> handler_t {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    let mut sptree: *mut *mut splay_tree = if !((*p).conf.auth_cache).is_null() {
        &mut (*(*p).conf.auth_cache).sptree
    } else {
        0 as *mut *mut splay_tree
    };
    let mut ae: *mut http_auth_cache_entry = 0 as *mut http_auth_cache_entry;
    let mut rc: handler_t = HANDLER_GO_ON;
    let mut ndx: libc::c_int = -(1 as libc::c_int);
    let mut user: *const libc::c_char = (*ai).username;
    let ulen: uint32_t = (*ai).ulen as uint32_t;
    let mut userbuf: [libc::c_char; 256] = [0; 256];
    if (*ai).userhash != 0
        && ulen as libc::c_ulong
            <= ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
    {
        let s: *const libc::c_char = (*ai).username;
        let mut i: uint_fast32_t = 0 as libc::c_int as uint_fast32_t;
        while i < ulen as libc::c_ulong {
            userbuf[i
                as usize] = (if !((*s.offset(i as isize) as uint32_t)
                .wrapping_sub('A' as i32 as libc::c_uint)
                <= ('Z' as i32 - 'A' as i32) as libc::c_uint)
            {
                *s.offset(i as isize) as libc::c_int
            } else {
                *s.offset(i as isize) as libc::c_int | 0x20 as libc::c_int
            }) as libc::c_char;
            i = i.wrapping_add(1);
        }
        user = userbuf.as_mut_ptr();
    }
    if !sptree.is_null() {
        ndx = http_auth_cache_hash(require, user, ulen);
        ae = http_auth_cache_query(sptree, ndx);
        if !ae.is_null() && (*ae).require == require && (*ae).dalgo == (*ai).dalgo
            && (*ae).dlen == (*ai).dlen && (*ae).klen == ulen
            && 0 as libc::c_int
                == memcmp(
                    (*ae).k as *const libc::c_void,
                    user as *const libc::c_void,
                    ulen as libc::c_ulong,
                ) && ((*ae).k == (*ae).username || (*ai).userhash != 0)
        {
            memcpy(
                ((*ai).digest).as_mut_ptr() as *mut libc::c_void,
                (*ae).pwdigest as *const libc::c_void,
                (*ai).dlen as libc::c_ulong,
            );
            if (*ae).k != (*ae).username {
                if ((*ae).ulen as libc::c_ulong
                    <= ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                    as libc::c_int as libc::c_long != 0
                {
                    (*ai).ulen = (*ae).ulen as size_t;
                    (*ai)
                        .username = memcpy(
                        ((*ai).userbuf).as_mut_ptr() as *mut libc::c_void,
                        (*ae).username as *const libc::c_void,
                        (*ae).ulen as libc::c_ulong,
                    ) as *const libc::c_char;
                }
            }
        } else {
            ae = 0 as *mut http_auth_cache_entry;
        }
    }
    if ae.is_null() {
        if (*ai).userhash != 0
            && ulen as libc::c_ulong
                <= ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
        {
            (*ai)
                .username = memcpy(
                ((*ai).userbuf).as_mut_ptr() as *mut libc::c_void,
                userbuf.as_mut_ptr() as *const libc::c_void,
                ulen as libc::c_ulong,
            ) as *const libc::c_char;
        }
        rc = ((*backend).digest)
            .expect("non-null function pointer")(r, (*backend).p_d, ai);
    }
    match rc as libc::c_uint {
        0 => {}
        3 => return HANDLER_WAIT_FOR_EVENT,
        1 => return HANDLER_FINISHED,
        4 | _ => {
            (*r).keep_alive = -(1 as libc::c_int) as int8_t;
            return mod_auth_send_401_unauthorized_digest(r, require, 0 as libc::c_int);
        }
    }
    if !sptree.is_null() && ae.is_null() {
        ae = http_auth_cache_entry_init(
            require,
            (*ai).dalgo,
            user,
            ulen,
            (*ai).username,
            (*ai).ulen as uint32_t,
            ((*ai).digest).as_mut_ptr() as *mut libc::c_char,
            (*ai).dlen,
        );
        http_auth_cache_insert(
            sptree,
            ndx,
            ae as *mut libc::c_void,
            Some(
                http_auth_cache_entry_free
                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
        );
    }
    return rc;
}
#[cold]
unsafe extern "C" fn mod_auth_digest_misconfigured(
    r: *mut request_st,
    backend: *const http_auth_backend_t,
) -> handler_t {
    if backend.is_null() {
        log_error(
            (*r).conf.errh,
            b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
            1201 as libc::c_int as libc::c_uint,
            b"auth.backend not configured for %s\0" as *const u8 as *const libc::c_char,
            (*r).uri.path.ptr,
        );
    } else {
        log_error(
            (*r).conf.errh,
            b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
            1204 as libc::c_int as libc::c_uint,
            b"auth.require \"method\" => \"digest\" invalid (try \"basic\"?) for %s\0"
                as *const u8 as *const libc::c_char,
            (*r).uri.path.ptr,
        );
    }
    (*r).http_status = 500 as libc::c_int;
    (*r).handler_module = 0 as *const plugin;
    return HANDLER_FINISHED;
}
static mut dkv: [digest_kv; 12] = [digest_kv {
    key: 0 as *const libc::c_char,
    klen: 0,
    id: e_username,
}; 12];
unsafe extern "C" fn mod_auth_digest_parse_authorization(
    dp: *mut http_auth_digest_params_t,
    mut c: *const libc::c_char,
) {
    let mut e: *const libc::c_char = 0 as *const libc::c_char;
    while *c != 0 {
        while *c as libc::c_int == ' ' as i32 || *c as libc::c_int == '\t' as i32
            || *c as libc::c_int == ',' as i32
        {
            c = c.offset(1);
        }
        if *c == 0 {
            break;
        }
        e = c;
        while *e as libc::c_int != '=' as i32 && *e as libc::c_int != ' ' as i32
            && *e as libc::c_int != '\t' as i32 && *e as libc::c_int != '\u{0}' as i32
        {
            e = e.offset(1);
        }
        let tlen: uint32_t = e.offset_from(c) as libc::c_long as uint32_t;
        let mut i: libc::c_int = 0 as libc::c_int;
        while !(dkv[i as usize].key).is_null() {
            if tlen != dkv[i as usize].klen
                || 0 as libc::c_int
                    != memcmp(
                        c as *const libc::c_void,
                        dkv[i as usize].key as *const libc::c_void,
                        tlen as libc::c_ulong,
                    )
            {
                i += 1;
            } else {
                c = c.offset(tlen as isize);
                if (*c as libc::c_int != '=' as i32) as libc::c_int as libc::c_long != 0
                {
                    while *c as libc::c_int == ' ' as i32
                        || *c as libc::c_int == '\t' as i32
                    {
                        c = c.offset(1);
                    }
                    if *c as libc::c_int != '=' as i32 {
                        return;
                    }
                }
                loop {
                    c = c.offset(1);
                    if !(*c as libc::c_int == ' ' as i32
                        || *c as libc::c_int == '\t' as i32)
                    {
                        break;
                    }
                }
                if *c as libc::c_int == '"' as i32 {
                    c = c.offset(1);
                    e = c;
                    while *e as libc::c_int != '"' as i32
                        && *e as libc::c_int != '\u{0}' as i32
                    {
                        if *e as libc::c_int == '\\' as i32
                            && {
                                e = e.offset(1);
                                *e as libc::c_int == '\u{0}' as i32
                            }
                        {
                            return;
                        }
                        e = e.offset(1);
                    }
                    if *e as libc::c_int != '"' as i32 {
                        return;
                    }
                } else {
                    e = c;
                    while *e as libc::c_int != ',' as i32
                        && *e as libc::c_int != ' ' as i32
                        && *e as libc::c_int != '\t' as i32
                        && *e as libc::c_int != '\u{0}' as i32
                    {
                        e = e.offset(1);
                    }
                }
                (*dp).ptr[dkv[i as usize].id as usize] = c;
                (*dp)
                    .len[dkv[i as usize].id
                    as usize] = e.offset_from(c) as libc::c_long as uint16_t;
                c = e;
                if *c as libc::c_int != ',' as i32 {
                    c = strchr(c, ',' as i32);
                    if c.is_null() {
                        return;
                    }
                }
                break;
            }
        }
        c = c.offset(1);
    }
}
unsafe extern "C" fn mod_auth_digest_validate_userstar(
    r: *mut request_st,
    dp: *mut http_auth_digest_params_t,
    ai: *mut http_auth_info_t,
) -> handler_t {
    if (*dp).len[e_userhash as libc::c_int as usize] as libc::c_int == 4 as libc::c_int {
        log_error(
            (*r).conf.errh,
            b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
            1290 as libc::c_int as libc::c_uint,
            b"digest: invalid \"username*\" with \"userhash\" = true\0" as *const u8
                as *const libc::c_char,
        );
        return mod_auth_send_400_bad_request(r);
    }
    let mut ptr: *const libc::c_char = (*dp).ptr[e_userstar as libc::c_int as usize];
    let mut len: uint32_t = (*dp).len[e_userstar as libc::c_int as usize] as uint32_t;
    if *ptr as libc::c_int | 0x20 as libc::c_int == 'u' as i32
        && len > 5 as libc::c_int as libc::c_uint
        && buffer_eq_icase_ssn(
            ptr,
            b"utf-8\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        ) != 0
    {
        ptr = ptr.offset(5 as libc::c_int as isize);
    } else if *ptr as libc::c_int | 0x20 as libc::c_int == 'i' as i32
            && len > 10 as libc::c_int as libc::c_uint
            && buffer_eq_icase_ssn(
                ptr,
                b"iso-8859-1\0" as *const u8 as *const libc::c_char,
                10 as libc::c_int as size_t,
            ) != 0
        {
        ptr = ptr.offset(10 as libc::c_int as isize);
    } else {
        ptr = b"\n\0" as *const u8 as *const libc::c_char;
    }
    let fresh3 = ptr;
    ptr = ptr.offset(1);
    if *fresh3 as libc::c_int != '\'' as i32
        || {
            ptr = memchr(
                ptr as *const libc::c_void,
                '\'' as i32,
                len
                    .wrapping_sub(
                        ptr.offset_from((*dp).ptr[e_userstar as libc::c_int as usize])
                            as libc::c_long as uint32_t,
                    ) as libc::c_ulong,
            ) as *const libc::c_char;
            ptr.is_null()
        }
    {
        log_error(
            (*r).conf.errh,
            b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
            1312 as libc::c_int as libc::c_uint,
            b"digest: invalid \"username*\" ext-value\0" as *const u8
                as *const libc::c_char,
        );
        return mod_auth_send_400_bad_request(r);
    }
    ptr = ptr.offset(1);
    let tb: *mut buffer = (*r).tmp_buf;
    buffer_copy_string_len(
        tb,
        ptr,
        len
            .wrapping_sub(
                ptr.offset_from((*dp).ptr[e_userstar as libc::c_int as usize])
                    as libc::c_long as uint32_t,
            ) as size_t,
    );
    buffer_urldecode_path(tb);
    if *((*dp).ptr[e_userstar as libc::c_int as usize]).offset(0 as libc::c_int as isize)
        as libc::c_int == 'u' as i32 && buffer_is_valid_UTF8(tb) == 0
    {
        log_error(
            (*r).conf.errh,
            b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
            1323 as libc::c_int as libc::c_uint,
            b"digest: invalid \"username*\" invalid UTF-8\0" as *const u8
                as *const libc::c_char,
        );
        return mod_auth_send_400_bad_request(r);
    }
    len = buffer_clen(tb);
    if len as libc::c_ulong
        > ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
    {
        log_error(
            (*r).conf.errh,
            b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
            1329 as libc::c_int as libc::c_uint,
            b"digest: invalid \"username*\" too long\0" as *const u8
                as *const libc::c_char,
        );
        return mod_auth_send_400_bad_request(r);
    }
    ptr = (*tb).ptr;
    while *ptr != 0 {
        if (*(ptr as *mut libc::c_uchar) as libc::c_int) < 0x20 as libc::c_int
            || *ptr as libc::c_int == 127 as libc::c_int
        {
            log_error(
                (*r).conf.errh,
                b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
                1337 as libc::c_int as libc::c_uint,
                b"digest: invalid \"username*\" contains ctrl chars\0" as *const u8
                    as *const libc::c_char,
            );
            return mod_auth_send_400_bad_request(r);
        }
        ptr = ptr.offset(1);
    }
    (*ai).ulen = len as size_t;
    (*ai)
        .username = memcpy(
        ((*ai).userbuf).as_mut_ptr() as *mut libc::c_void,
        (*tb).ptr as *const libc::c_void,
        len as libc::c_ulong,
    ) as *const libc::c_char;
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_auth_digest_validate_params(
    r: *mut request_st,
    require: *const http_auth_require_t,
    dp: *mut http_auth_digest_params_t,
    ai: *mut http_auth_info_t,
) -> handler_t {
    if (((*dp).ptr[e_qop as libc::c_int as usize]).is_null()
        || !((*dp).ptr[e_nc as libc::c_int as usize]).is_null()
            && !((*dp).ptr[e_cnonce as libc::c_int as usize]).is_null())
        && (0 as *mut libc::c_void as *const libc::c_char
            != (*dp).ptr[e_username as libc::c_int as usize]) as libc::c_int
            ^ (0 as *mut libc::c_void as *const libc::c_char
                != (*dp).ptr[e_userstar as libc::c_int as usize]) as libc::c_int != 0
        && !((*dp).ptr[e_realm as libc::c_int as usize]).is_null()
        && !((*dp).ptr[e_nonce as libc::c_int as usize]).is_null()
        && !((*dp).ptr[e_uri as libc::c_int as usize]).is_null()
        && !((*dp).ptr[e_response as libc::c_int as usize]).is_null()
    {
        (*ai).username = (*dp).ptr[e_username as libc::c_int as usize];
        (*ai).ulen = (*dp).len[e_username as libc::c_int as usize] as size_t;
        (*ai).realm = (*dp).ptr[e_realm as libc::c_int as usize];
        (*ai).rlen = (*dp).len[e_realm as libc::c_int as usize] as size_t;
        (*ai)
            .userhash = ((*dp).len[e_userhash as libc::c_int as usize] as libc::c_int
            == 4 as libc::c_int) as libc::c_int;
        if ((*ai).username).is_null() {
            if HANDLER_GO_ON as libc::c_int as libc::c_uint
                != mod_auth_digest_validate_userstar(r, dp, ai) as libc::c_uint
            {
                return HANDLER_FINISHED;
            }
        }
    } else {
        log_error(
            (*r).conf.errh,
            b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
            1370 as libc::c_int as libc::c_uint,
            b"digest: missing field\0" as *const u8 as *const libc::c_char,
        );
        return mod_auth_send_400_bad_request(r);
    }
    if buffer_eq_slen((*require).realm, (*ai).realm, (*ai).rlen) == 0 {
        log_error(
            (*r).conf.errh,
            b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
            1376 as libc::c_int as libc::c_uint,
            b"digest: realm mismatch\0" as *const u8 as *const libc::c_char,
        );
        return mod_auth_send_401_unauthorized_digest(r, require, 0 as libc::c_int);
    }
    if mod_auth_algorithm_parse(
        ai,
        (*dp).ptr[e_algorithm as libc::c_int as usize],
        (*dp).len[e_algorithm as libc::c_int as usize] as size_t,
    ) == 0
        || (*require).algorithm & (*ai).dalgo & !(HTTP_AUTH_DIGEST_SESS as libc::c_int)
            == 0
    {
        log_error(
            (*r).conf.errh,
            b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
            1383 as libc::c_int as libc::c_uint,
            b"digest: (%.*s): invalid\0" as *const u8 as *const libc::c_char,
            (*dp).len[e_algorithm as libc::c_int as usize] as libc::c_int,
            (*dp).ptr[e_algorithm as libc::c_int as usize],
        );
        return mod_auth_send_401_unauthorized_digest(r, require, 0 as libc::c_int);
    }
    if (*ai).dalgo & HTTP_AUTH_DIGEST_SESS as libc::c_int != 0
        && (((*dp).ptr[e_nonce as libc::c_int as usize]).is_null()
            || ((*dp).ptr[e_cnonce as libc::c_int as usize]).is_null())
    {
        log_error(
            (*r).conf.errh,
            b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
            1392 as libc::c_int as libc::c_uint,
            b"digest: (%.*s): missing field\0" as *const u8 as *const libc::c_char,
            (*dp).len[e_algorithm as libc::c_int as usize] as libc::c_int,
            (*dp).ptr[e_algorithm as libc::c_int as usize],
        );
        return mod_auth_send_400_bad_request(r);
    }
    if 0 as libc::c_int
        != li_hex2bin(
            ((*dp).rdigest).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong,
            (*dp).ptr[e_response as libc::c_int as usize],
            (*dp).len[e_response as libc::c_int as usize] as size_t,
        )
        || (*dp).len[e_response as libc::c_int as usize] as libc::c_uint
            != (*ai).dlen << 1 as libc::c_int
    {
        log_error(
            (*r).conf.errh,
            b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
            1401 as libc::c_int as libc::c_uint,
            b"digest: (%s): invalid format\0" as *const u8 as *const libc::c_char,
            (*dp).ptr[e_response as libc::c_int as usize],
        );
        return mod_auth_send_400_bad_request(r);
    }
    if !((*dp).ptr[e_qop as libc::c_int as usize]).is_null()
        && buffer_eq_icase_ss(
            (*dp).ptr[e_qop as libc::c_int as usize],
            (*dp).len[e_qop as libc::c_int as usize] as size_t,
            b"auth-int\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
    {
        log_error(
            (*r).conf.errh,
            b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
            1408 as libc::c_int as libc::c_uint,
            b"digest: qop=auth-int not supported\0" as *const u8 as *const libc::c_char,
        );
        return mod_auth_send_400_bad_request(r);
    }
    if buffer_eq_slen(
        &mut (*r).target_orig,
        (*dp).ptr[e_uri as libc::c_int as usize],
        (*dp).len[e_uri as libc::c_int as usize] as size_t,
    ) == 0
    {
        log_error(
            (*r).conf.errh,
            b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
            1422 as libc::c_int as libc::c_uint,
            b"digest: auth failed: uri mismatch (%s != %.*s), IP: %s\0" as *const u8
                as *const libc::c_char,
            (*r).target_orig.ptr,
            (*dp).len[e_uri as libc::c_int as usize] as libc::c_int,
            (*dp).ptr[e_uri as libc::c_int as usize],
            (*(*r).con).dst_addr_buf.ptr,
        );
        return mod_auth_send_400_bad_request(r);
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_auth_digest_validate_nonce(
    r: *mut request_st,
    require: *const http_auth_require_t,
    dp: *mut http_auth_digest_params_t,
    ai: *mut http_auth_info_t,
) -> handler_t {
    let mut ts: unix_time64_t = 0 as libc::c_int as unix_time64_t;
    let nonce: *const libc::c_uchar = (*dp).ptr[e_nonce as libc::c_int as usize]
        as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int
        && light_isxdigit(*nonce.offset(i as isize) as libc::c_int) != 0
    {
        ts = ((ts as uint64_t) << 4 as libc::c_int) as unix_time64_t
            | hex2int(*nonce.offset(i as isize)) as libc::c_long;
        i += 1;
    }
    let cur_ts: unix_time64_t = log_epoch_secs;
    if *nonce.offset(i as isize) as libc::c_int != ':' as i32 || ts > cur_ts
        || cur_ts - ts > 600 as libc::c_int as libc::c_long
    {
        return mod_auth_send_401_unauthorized_digest(r, require, (*ai).dalgo);
    }
    if cur_ts - ts > 540 as libc::c_int as libc::c_long {
        (*dp).send_nextnonce_ts = cur_ts;
    }
    if !((*require).nonce_secret).is_null() {
        let mut rnd: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut j: libc::c_int = i + 8 as libc::c_int;
        while i < j && light_isxdigit(*nonce.offset(i as isize) as libc::c_int) != 0 {
            rnd = (rnd << 4 as libc::c_int)
                .wrapping_add(hex2int(*nonce.offset(i as isize)) as libc::c_uint);
            i += 1;
        }
        if *nonce.offset(i as isize) as libc::c_int != ':' as i32 {
            log_error(
                (*r).conf.errh,
                b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
                1475 as libc::c_int as libc::c_uint,
                b"digest: nonce invalid\0" as *const u8 as *const libc::c_char,
            );
            return mod_auth_send_400_bad_request(r);
        }
        let tb: *mut buffer = (*r).tmp_buf;
        buffer_clear(tb);
        mod_auth_append_nonce(
            tb,
            cur_ts,
            require,
            (*ai).dalgo,
            &mut rnd as *mut libc::c_uint as *mut libc::c_int,
        );
        if buffer_eq_slen(
            tb,
            (*dp).ptr[e_nonce as libc::c_int as usize],
            (*dp).len[e_nonce as libc::c_int as usize] as size_t,
        ) == 0
        {
            log_error(
                (*r).conf.errh,
                b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
                1484 as libc::c_int as libc::c_uint,
                b"digest: nonce mismatch\0" as *const u8 as *const libc::c_char,
            );
            return mod_auth_send_401_unauthorized_digest(r, require, 0 as libc::c_int);
        }
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_auth_check_digest(
    r: *mut request_st,
    mut p_d: *mut libc::c_void,
    require: *const http_auth_require_t,
    backend: *const http_auth_backend_t,
) -> handler_t {
    if backend.is_null() || ((*backend).digest).is_none() {
        return mod_auth_digest_misconfigured(r, backend);
    }
    let vb: *const buffer = http_header_request_get(
        r,
        HTTP_HEADER_AUTHORIZATION,
        b"Authorization\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if vb.is_null()
        || buffer_eq_icase_ssn(
            (*vb).ptr,
            b"Digest \0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) == 0
    {
        return mod_auth_send_401_unauthorized_digest(r, require, 0 as libc::c_int);
    }
    let mut dp: http_auth_digest_params_t = http_auth_digest_params_t {
        ptr: [0 as *const libc::c_char; 11],
        len: [0; 11],
        send_nextnonce_ts: 0,
        rdigest: [0; 20],
    };
    let mut ai: http_auth_info_t = http_auth_info_t {
        dalgo: 0,
        dlen: 0,
        username: 0 as *const libc::c_char,
        ulen: 0,
        realm: 0 as *const libc::c_char,
        rlen: 0,
        userhash: 0,
        digest: [0; 32],
        userbuf: [0; 256],
    };
    let mut rc: handler_t = HANDLER_GO_ON;
    memset(
        &mut dp as *mut http_auth_digest_params_t as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<http_auth_digest_params_t>() as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong),
    );
    mod_auth_digest_parse_authorization(
        &mut dp,
        ((*vb).ptr)
            .offset(::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as isize)
            .offset(-(1 as libc::c_int as isize)),
    );
    rc = mod_auth_digest_validate_params(r, require, &mut dp, &mut ai);
    if (HANDLER_GO_ON as libc::c_int as libc::c_uint != rc as libc::c_uint)
        as libc::c_int as libc::c_long != 0
    {
        return rc;
    }
    rc = mod_auth_digest_validate_nonce(r, require, &mut dp, &mut ai);
    if (HANDLER_GO_ON as libc::c_int as libc::c_uint != rc as libc::c_uint)
        as libc::c_int as libc::c_long != 0
    {
        return rc;
    }
    rc = mod_auth_digest_get(r, p_d, require, backend, &mut ai);
    if (HANDLER_GO_ON as libc::c_int as libc::c_uint != rc as libc::c_uint)
        as libc::c_int as libc::c_long != 0
    {
        return rc;
    }
    mod_auth_digest_mutate(&mut ai, &mut dp, http_method_buf((*r).http_method));
    if ck_memeq_const_time_fixed_len(
        (dp.rdigest).as_mut_ptr() as *const libc::c_void,
        (ai.digest).as_mut_ptr() as *const libc::c_void,
        ai.dlen as size_t,
    ) == 0
    {
        log_error(
            (*r).conf.errh,
            b"src/mod_auth.c\0" as *const u8 as *const libc::c_char,
            1536 as libc::c_int as libc::c_uint,
            b"digest: auth failed for %.*s: wrong password, IP: %s\0" as *const u8
                as *const libc::c_char,
            ai.ulen as libc::c_int,
            ai.username,
            (*(*r).con).dst_addr_buf.ptr,
        );
        (*r).keep_alive = -(1 as libc::c_int) as int8_t;
        return mod_auth_send_401_unauthorized_digest(r, require, 0 as libc::c_int);
    }
    let tb: *mut buffer = (*r).tmp_buf;
    buffer_copy_string_len(tb, ai.username, ai.ulen);
    if http_auth_match_rules(
        require,
        (*tb).ptr,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
    ) == 0
    {
        return mod_auth_send_401_unauthorized_digest(r, require, 0 as libc::c_int);
    }
    if dp.send_nextnonce_ts != 0 {
        mod_auth_digest_authentication_info(
            http_header_response_set_ptr(
                r,
                HTTP_HEADER_OTHER,
                b"Authentication-Info\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            ),
            dp.send_nextnonce_ts,
            require,
            ai.dalgo,
        );
    }
    http_auth_setenv(
        r,
        ai.username,
        ai.ulen,
        b"Digest\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_auth_check_extern(
    r: *mut request_st,
    mut p_d: *mut libc::c_void,
    require: *const http_auth_require_t,
    backend: *const http_auth_backend_t,
) -> handler_t {
    let mut vb: *const buffer = http_header_env_get(
        r,
        b"REMOTE_USER\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if !vb.is_null()
        && http_auth_match_rules(
            require,
            (*vb).ptr,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
        ) != 0
    {
        return HANDLER_GO_ON
    } else {
        (*r).http_status = 401 as libc::c_int;
        (*r).handler_module = 0 as *const plugin;
        return HANDLER_FINISHED;
    };
}
pub unsafe fn run_static_initializers() {
    cpk = [
        {
            let mut init = config_plugin_keys_t {
                k: b"auth.backend\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"auth.require\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVARRAY as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"auth.extern-authn\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"auth.cache\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY as libc::c_int as uint8_t,
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
    dkv = [
        {
            let mut init = digest_kv {
                key: b"username\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                id: e_username,
            };
            init
        },
        {
            let mut init = digest_kv {
                key: b"realm\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                id: e_realm,
            };
            init
        },
        {
            let mut init = digest_kv {
                key: b"nonce\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                id: e_nonce,
            };
            init
        },
        {
            let mut init = digest_kv {
                key: b"uri\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                id: e_uri,
            };
            init
        },
        {
            let mut init = digest_kv {
                key: b"algorithm\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                id: e_algorithm,
            };
            init
        },
        {
            let mut init = digest_kv {
                key: b"qop\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                id: e_qop,
            };
            init
        },
        {
            let mut init = digest_kv {
                key: b"cnonce\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                id: e_cnonce,
            };
            init
        },
        {
            let mut init = digest_kv {
                key: b"nc\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                id: e_nc,
            };
            init
        },
        {
            let mut init = digest_kv {
                key: b"response\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                id: e_response,
            };
            init
        },
        {
            let mut init = digest_kv {
                key: b"username*\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                id: e_userstar,
            };
            init
        },
        {
            let mut init = digest_kv {
                key: b"userhash\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                id: e_userhash,
            };
            init
        },
        {
            let mut init = digest_kv {
                key: 0 as *const libc::c_char,
                klen: 0 as libc::c_int as uint32_t,
                id: http_auth_digest_params_sz,
            };
            init
        },
    ];
}
