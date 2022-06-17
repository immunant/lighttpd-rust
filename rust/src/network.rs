use ::libc;
extern "C" {
    pub type stat_cache_entry;
    pub type pcre2_real_match_data_8;
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
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn getsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *mut libc::c_void,
        __optlen: *mut socklen_t,
    ) -> libc::c_int;
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn buffer_init() -> *mut buffer;
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn buffer_free(b: *mut buffer);
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn bind(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn sock_addr_stringify_append_buffer(
        b: *mut buffer,
        saddr: *const sock_addr,
    ) -> libc::c_int;
    fn sock_addr_from_str_hints(
        saddr: *mut sock_addr,
        len: *mut socklen_t,
        str: *const libc::c_char,
        family: libc::c_int,
        port: libc::c_ushort,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn getsockname(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn buffer_is_equal(a: *const buffer, b: *const buffer) -> libc::c_int;
    fn buffer_append_int(b: *mut buffer, val: intmax_t);
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn fdevent_reset(ev: *mut fdevents) -> libc::c_int;
    fn fdevent_fdnode_event_del(ev: *mut fdevents, fdn: *mut fdnode);
    fn fdevent_fdnode_event_set(
        ev: *mut fdevents,
        fdn: *mut fdnode,
        events: libc::c_int,
    );
    fn fdevent_register(
        ev: *mut fdevents,
        fd: libc::c_int,
        handler: fdevent_handler,
        ctx: *mut libc::c_void,
    ) -> *mut fdnode;
    fn fdevent_unregister(ev: *mut fdevents, fd: libc::c_int);
    fn fdevent_clrfd_cloexec(fd: libc::c_int);
    fn fdevent_fcntl_set_nb(fd: libc::c_int) -> libc::c_int;
    fn fdevent_fcntl_set_nb_cloexec(fd: libc::c_int) -> libc::c_int;
    fn fdevent_socket_cloexec(
        domain: libc::c_int,
        type_0: libc::c_int,
        protocol: libc::c_int,
    ) -> libc::c_int;
    fn fdevent_socket_nb_cloexec(
        domain: libc::c_int,
        type_0: libc::c_int,
        protocol: libc::c_int,
    ) -> libc::c_int;
    fn fdevent_accept_listenfd(
        listenfd: libc::c_int,
        addr: *mut sockaddr,
        addrlen: *mut size_t,
    ) -> libc::c_int;
    fn fdevent_set_tcp_nodelay(fd: libc::c_int, opt: libc::c_int) -> libc::c_int;
    fn fdevent_set_so_reuseaddr(fd: libc::c_int, opt: libc::c_int) -> libc::c_int;
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
    fn connection_accepted(
        srv: *mut server,
        srv_socket: *const server_socket,
        cnt_addr: *mut sock_addr,
        cnt: libc::c_int,
    ) -> *mut connection;
    fn connection_state_machine(con: *mut connection);
    fn config_plugin_values_init(
        srv: *mut server,
        p_d: *mut libc::c_void,
        cpk: *const config_plugin_keys_t,
        mname: *const libc::c_char,
    ) -> libc::c_int;
    fn config_get_config_cond_info(cfginfo: *mut config_cond_info, idx: uint32_t);
    fn network_write_init(srv: *mut server) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn getppid() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
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
pub type __mode_t = libc::c_uint;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
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
pub type socklen_t = __socklen_t;
pub const IPPROTO_TCP: C2RustUnnamed_5 = 6;
pub type config_plugin_value_t = config_plugin_value;
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
pub struct network_plugin_data {
    pub id: libc::c_int,
    pub nconfig: libc::c_int,
    pub cvlist: *mut config_plugin_value_t,
    pub self_0: *mut plugin,
    pub defaults: network_socket_config,
    pub conf: network_socket_config,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct network_socket_config {
    pub listen_backlog: libc::c_int,
    pub ssl_enabled: libc::c_uchar,
    pub use_ipv6: libc::c_uchar,
    pub set_v6only: libc::c_uchar,
    pub defer_accept: libc::c_uchar,
    pub v4mapped: int8_t,
    pub socket_perms: *const buffer,
    pub bsd_accept_filter: *const buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
pub const IPPROTO_IPV6: C2RustUnnamed_5 = 41;
pub const SOCK_STREAM: __socket_type = 1;
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
pub const CONFIG_COND_NE: config_cond_t = 3;
pub type config_cond_t = libc::c_uint;
pub const CONFIG_COND_ELSE: config_cond_t = 5;
pub const CONFIG_COND_NOMATCH: config_cond_t = 4;
pub const CONFIG_COND_MATCH: config_cond_t = 2;
pub const CONFIG_COND_EQ: config_cond_t = 1;
pub const CONFIG_COND_UNSET: config_cond_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_cond_info {
    pub comp: comp_key_t,
    pub cond: config_cond_t,
    pub string: *const buffer,
    pub comp_key: *const libc::c_char,
}
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
pub struct config_plugin_keys_t {
    pub k: *const libc::c_char,
    pub klen: uint8_t,
    pub ktype: uint8_t,
    pub scope: uint8_t,
}
pub const T_CONFIG_SCOPE_UNSET: C2RustUnnamed_6 = 0;
pub const T_CONFIG_SCOPE_CONNECTION: C2RustUnnamed_6 = 2;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_5 = 263;
pub const IPPROTO_MPTCP: C2RustUnnamed_5 = 262;
pub const IPPROTO_RAW: C2RustUnnamed_5 = 255;
pub const IPPROTO_ETHERNET: C2RustUnnamed_5 = 143;
pub const IPPROTO_MPLS: C2RustUnnamed_5 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_5 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_5 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_5 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_5 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_5 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_5 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_5 = 92;
pub const IPPROTO_AH: C2RustUnnamed_5 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_5 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_5 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_5 = 46;
pub const IPPROTO_DCCP: C2RustUnnamed_5 = 33;
pub const IPPROTO_TP: C2RustUnnamed_5 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_5 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_5 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_5 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_5 = 8;
pub const IPPROTO_IPIP: C2RustUnnamed_5 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_5 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_5 = 1;
pub const IPPROTO_IP: C2RustUnnamed_5 = 0;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const T_CONFIG_SCOPE_SERVER: C2RustUnnamed_6 = 1;
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
unsafe extern "C" fn sock_addr_get_family(mut saddr: *const sock_addr) -> libc::c_int {
    return (*saddr).plain.sa_family as libc::c_int;
}
#[inline]
unsafe extern "C" fn buffer_is_blank(mut b: *const buffer) -> libc::c_int {
    return ((*b).used < 2 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn network_accept_tcp_nagle_disable(fd: libc::c_int) {
    static mut noinherit_tcpnodelay: libc::c_int = -(1 as libc::c_int);
    let mut opt: libc::c_int = 0;
    if noinherit_tcpnodelay == 0 {
        return;
    }
    if noinherit_tcpnodelay < 0 as libc::c_int {
        let mut optlen: socklen_t = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            as socklen_t;
        if 0 as libc::c_int
            == getsockopt(
                fd,
                IPPROTO_TCP as libc::c_int,
                1 as libc::c_int,
                &mut opt as *mut libc::c_int as *mut libc::c_void,
                &mut optlen,
            )
        {
            noinherit_tcpnodelay = (opt == 0) as libc::c_int;
            if opt != 0 {
                return;
            }
        }
    }
    opt = 1 as libc::c_int;
    setsockopt(
        fd,
        IPPROTO_TCP as libc::c_int,
        1 as libc::c_int,
        &mut opt as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
}
unsafe extern "C" fn network_server_handle_fdevent(
    mut context: *mut libc::c_void,
    mut revents: libc::c_int,
) -> handler_t {
    let srv_socket: *const server_socket = context as *mut server_socket;
    let srv: *mut server = (*srv_socket).srv;
    if 0 as libc::c_int == revents & 0x1 as libc::c_int {
        log_error(
            (*srv).errh,
            b"src/network.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int as libc::c_uint,
            b"strange event for server socket %d %d\0" as *const u8
                as *const libc::c_char,
            (*srv_socket).fd,
            revents,
        );
        return HANDLER_ERROR;
    }
    let mut loops: libc::c_int = (*srv).lim_conns as libc::c_int;
    if loops > 100 as libc::c_int {
        loops = 100 as libc::c_int;
    } else if loops <= 0 as libc::c_int {
        return HANDLER_GO_ON
    }
    let nagle_disable: libc::c_int = (sock_addr_get_family(&(*srv_socket).addr)
        != 1 as libc::c_int) as libc::c_int;
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
    let mut addrlen: size_t = 0;
    loop {
        addrlen = ::std::mem::size_of::<sock_addr>() as libc::c_ulong;
        let mut fd: libc::c_int = fdevent_accept_listenfd(
            (*srv_socket).fd,
            &mut addr as *mut sock_addr as *mut sockaddr,
            &mut addrlen,
        );
        if -(1 as libc::c_int) == fd {
            break;
        }
        if nagle_disable != 0 {
            network_accept_tcp_nagle_disable(fd);
        }
        let mut con: *mut connection = connection_accepted(
            srv,
            srv_socket,
            &mut addr,
            fd,
        );
        if con.is_null() as libc::c_int as libc::c_long != 0 {
            return HANDLER_GO_ON;
        }
        connection_state_machine(con);
        loops -= 1;
        if !(loops != 0) {
            break;
        }
    }
    if loops != 0 {
        match *__errno_location() {
            11 | 4 | 103 | 24 => {}
            _ => {
                log_perror(
                    (*srv).errh,
                    b"src/network.c\0" as *const u8 as *const libc::c_char,
                    94 as libc::c_int as libc::c_uint,
                    b"accept()\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn network_host_normalize_addr_str(
    mut host: *mut buffer,
    mut addr: *mut sock_addr,
) {
    buffer_clear(host);
    sock_addr_stringify_append_buffer(host, addr);
}
unsafe extern "C" fn network_host_parse_addr(
    mut srv: *mut server,
    mut addr: *mut sock_addr,
    mut addr_len: *mut socklen_t,
    mut host: *mut buffer,
    mut use_ipv6: libc::c_int,
) -> libc::c_int {
    let mut h: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut colon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut chost: *const libc::c_char = 0 as *const libc::c_char;
    let mut family: sa_family_t = (if use_ipv6 != 0 {
        10 as libc::c_int
    } else {
        2 as libc::c_int
    }) as sa_family_t;
    let mut port: libc::c_uint = (*srv).srvconf.port as libc::c_uint;
    if buffer_is_blank(host) != 0 {
        log_error(
            (*srv).errh,
            b"src/network.c\0" as *const u8 as *const libc::c_char,
            113 as libc::c_int as libc::c_uint,
            b"value of $SERVER[\"socket\"] must not be empty\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    h = (*host).ptr;
    if *h.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
        return if 1 as libc::c_int
            == sock_addr_from_str_hints(
                addr,
                addr_len,
                h,
                1 as libc::c_int,
                0 as libc::c_int as libc::c_ushort,
                (*srv).errh,
            )
        {
            0 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
    }
    let tb: *mut buffer = (*srv).tmp_buf;
    buffer_copy_buffer(tb, host);
    h = (*tb).ptr;
    if *h.offset(0 as libc::c_int as isize) as libc::c_int == '[' as i32 {
        family = 10 as libc::c_int as sa_family_t;
        h = strchr(h, ']' as i32);
        if !h.is_null() {
            let fresh0 = h;
            h = h.offset(1);
            *fresh0 = '\u{0}' as i32 as libc::c_char;
            if *h as libc::c_int == ':' as i32 {
                colon = h;
            }
        }
        h = ((*tb).ptr).offset(1 as libc::c_int as isize);
    } else {
        colon = strrchr(h, ':' as i32);
    }
    if !colon.is_null() {
        let fresh1 = colon;
        colon = colon.offset(1);
        *fresh1 = '\u{0}' as i32 as libc::c_char;
        port = strtol(colon, 0 as *mut *mut libc::c_char, 10 as libc::c_int)
            as libc::c_uint;
        if port == 0 as libc::c_int as libc::c_uint
            || port > 65535 as libc::c_int as libc::c_uint
        {
            log_error(
                (*srv).errh,
                b"src/network.c\0" as *const u8 as *const libc::c_char,
                148 as libc::c_int as libc::c_uint,
                b"port not set or out of range: %u\0" as *const u8
                    as *const libc::c_char,
                port,
            );
            return -(1 as libc::c_int);
        }
    }
    if *h.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32
        && *h.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
    {
        family = 2 as libc::c_int as sa_family_t;
        h = h.offset(1);
    }
    chost = if *h as libc::c_int != 0 {
        h as *const libc::c_char
    } else if family as libc::c_int == 2 as libc::c_int {
        b"0.0.0.0\0" as *const u8 as *const libc::c_char
    } else {
        b"::\0" as *const u8 as *const libc::c_char
    };
    if 1 as libc::c_int
        != sock_addr_from_str_hints(
            addr,
            addr_len,
            chost,
            family as libc::c_int,
            port as libc::c_ushort,
            (*srv).errh,
        )
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn network_srv_sockets_append(
    mut srv: *mut server,
    mut srv_socket: *mut server_socket,
) {
    if (*srv).srv_sockets.used == (*srv).srv_sockets.size {
        (*srv)
            .srv_sockets
            .size = ((*srv).srv_sockets.size as libc::c_uint)
            .wrapping_add(4 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
        (*srv)
            .srv_sockets
            .ptr = realloc(
            (*srv).srv_sockets.ptr as *mut libc::c_void,
            ((*srv).srv_sockets.size as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut server_socket>() as libc::c_ulong,
                ),
        ) as *mut *mut server_socket;
        if ((*srv).srv_sockets.ptr).is_null() {
            ck_assert_failed(
                b"src/network.c\0" as *const u8 as *const libc::c_char,
                169 as libc::c_int as libc::c_uint,
                b"((void*)0) != srv->srv_sockets.ptr\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    let fresh2 = (*srv).srv_sockets.used;
    (*srv).srv_sockets.used = ((*srv).srv_sockets.used).wrapping_add(1);
    let ref mut fresh3 = *((*srv).srv_sockets.ptr).offset(fresh2 as isize);
    *fresh3 = srv_socket;
}
unsafe extern "C" fn network_merge_config_cpv(
    pconf: *mut network_socket_config,
    cpv: *const config_plugin_value_t,
) {
    match (*cpv).k_id {
        0 => {
            (*pconf)
                .ssl_enabled = (0 as libc::c_int as libc::c_uint != (*cpv).v.u)
                as libc::c_int as libc::c_uchar;
        }
        1 => {
            (*pconf).listen_backlog = (*cpv).v.u as libc::c_int;
        }
        2 => {
            (*pconf).socket_perms = (*cpv).v.b;
        }
        3 => {
            (*pconf).bsd_accept_filter = (*cpv).v.b;
        }
        4 => {
            (*pconf)
                .defer_accept = (0 as libc::c_int as libc::c_uint != (*cpv).v.u)
                as libc::c_int as libc::c_uchar;
        }
        5 => {
            (*pconf)
                .use_ipv6 = (0 as libc::c_int as libc::c_uint != (*cpv).v.u)
                as libc::c_int as libc::c_uchar;
        }
        6 => {
            (*pconf)
                .set_v6only = (0 as libc::c_int as libc::c_uint != (*cpv).v.u)
                as libc::c_int as libc::c_uchar;
        }
        7 => {
            (*pconf)
                .v4mapped = (0 as libc::c_int as libc::c_uint != (*cpv).v.u)
                as libc::c_int as int8_t;
        }
        _ => return,
    };
}
unsafe extern "C" fn network_merge_config(
    pconf: *mut network_socket_config,
    mut cpv: *const config_plugin_value_t,
) {
    loop {
        network_merge_config_cpv(pconf, cpv);
        cpv = cpv.offset(1);
        if !((*cpv).k_id != -(1 as libc::c_int)) {
            break;
        }
    };
}
unsafe extern "C" fn network_srv_token_colon(b: *const buffer) -> uint8_t {
    let mut colon: *const libc::c_char = 0 as *const libc::c_char;
    let p: *const libc::c_char = (*b).ptr;
    if *p as libc::c_int == '[' as i32 {
        colon = strstr(p, b"]:\0" as *const u8 as *const libc::c_char);
        if !colon.is_null() {
            colon = colon.offset(1);
        }
    } else if *p as libc::c_int != '/' as i32 {
        colon = strchr(p, ':' as i32);
    }
    return (if !colon.is_null() {
        colon.offset_from(p) as libc::c_long as uint8_t as libc::c_int
    } else {
        buffer_clen(b) as uint8_t as libc::c_int
    }) as uint8_t;
}
unsafe extern "C" fn network_server_init(
    mut srv: *mut server,
    mut s: *mut network_socket_config,
    mut host_token: *mut buffer,
    mut sidx: size_t,
    mut stdin_fd: libc::c_int,
) -> libc::c_int {
    let mut srv_socket: *mut server_socket = 0 as *mut server_socket;
    let mut host: *const libc::c_char = 0 as *const libc::c_char;
    let mut addr_len: socklen_t = ::std::mem::size_of::<sock_addr>() as libc::c_ulong
        as socklen_t;
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
    let mut family: libc::c_int = 0 as libc::c_int;
    let mut set_v6only: libc::c_int = 0 as libc::c_int;
    if buffer_is_blank(host_token) != 0 {
        log_error(
            (*srv).errh,
            b"src/network.c\0" as *const u8 as *const libc::c_char,
            253 as libc::c_int as libc::c_uint,
            b"value of $SERVER[\"socket\"] must not be empty\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*srv).srv_sockets.used {
        if buffer_is_equal(
            (**((*srv).srv_sockets.ptr).offset(i as isize)).srv_token,
            host_token,
        ) != 0
        {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    host = (*host_token).ptr;
    if (*s).use_ipv6 as libc::c_int != 0
        && (*host as libc::c_int == '\u{0}' as i32 || *host as libc::c_int == ':' as i32)
        || *host.offset(0 as libc::c_int as isize) as libc::c_int == '[' as i32
            && *host.offset(1 as libc::c_int as isize) as libc::c_int == ']' as i32
    {
        log_error(
            (*srv).errh,
            b"src/network.c\0" as *const u8 as *const libc::c_char,
            269 as libc::c_int as libc::c_uint,
            b"warning: please use server.use-ipv6 only for hostnames, not without server.bind / empty address; your config will break if the kernel default for IPV6_V6ONLY changes\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if *host as libc::c_int == '[' as i32 {
        (*s).use_ipv6 = 1 as libc::c_int as libc::c_uchar;
    }
    memset(
        &mut addr as *mut sock_addr as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sock_addr>() as libc::c_ulong,
    );
    if -(1 as libc::c_int) != stdin_fd {
        if -(1 as libc::c_int)
            == getsockname(
                stdin_fd,
                __SOCKADDR_ARG {
                    __sockaddr__: &mut addr as *mut sock_addr as *mut sockaddr,
                },
                &mut addr_len,
            )
        {
            log_perror(
                (*srv).errh,
                b"src/network.c\0" as *const u8 as *const libc::c_char,
                279 as libc::c_int as libc::c_uint,
                b"getsockname()\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else if 0 as libc::c_int
            != network_host_parse_addr(
                srv,
                &mut addr,
                &mut addr_len,
                host_token,
                (*s).use_ipv6 as libc::c_int,
            )
        {
        return -(1 as libc::c_int)
    }
    family = sock_addr_get_family(&mut addr);
    if *host as libc::c_int != '\u{0}' as i32 && 10 as libc::c_int == family {
        if (*s).set_v6only != 0 {
            set_v6only = 1 as libc::c_int;
        } else {
            log_error(
                (*srv).errh,
                b"src/network.c\0" as *const u8 as *const libc::c_char,
                293 as libc::c_int as libc::c_uint,
                b"warning: server.set-v6only will be removed soon, update your config to have different sockets for ipv4 and ipv6\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
    if 10 as libc::c_int == family && -(1 as libc::c_int) != (*s).v4mapped as libc::c_int
    {
        set_v6only = if (*s).v4mapped as libc::c_int != 0 {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        };
    }
    network_host_normalize_addr_str(host_token, &mut addr);
    host = (*host_token).ptr;
    if (*srv).srvconf.preflight_check != 0 {
        return 0 as libc::c_int;
    }
    let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
    while i_0 < (*srv).srv_sockets.used {
        if 0 as libc::c_int
            == memcmp(
                &mut (**((*srv).srv_sockets.ptr).offset(i_0 as isize)).addr
                    as *mut sock_addr as *const libc::c_void,
                &mut addr as *mut sock_addr as *const libc::c_void,
                ::std::mem::size_of::<sock_addr>() as libc::c_ulong,
            )
        {
            return 0 as libc::c_int;
        }
        i_0 = i_0.wrapping_add(1);
    }
    srv_socket = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<server_socket>() as libc::c_ulong,
    ) as *mut server_socket;
    if srv_socket.is_null() {
        ck_assert_failed(
            b"src/network.c\0" as *const u8 as *const libc::c_char,
            318 as libc::c_int as libc::c_uint,
            b"((void*)0) != srv_socket\0" as *const u8 as *const libc::c_char,
        );
    }
    memcpy(
        &mut (*srv_socket).addr as *mut sock_addr as *mut libc::c_void,
        &mut addr as *mut sock_addr as *const libc::c_void,
        addr_len as libc::c_ulong,
    );
    (*srv_socket).fd = -(1 as libc::c_int);
    (*srv_socket).sidx = sidx as libc::c_ushort;
    (*srv_socket).is_ssl = (*s).ssl_enabled;
    (*srv_socket).srv = srv;
    (*srv_socket).srv_token = buffer_init();
    buffer_copy_buffer((*srv_socket).srv_token, host_token);
    (*srv_socket).srv_token_colon = network_srv_token_colon((*srv_socket).srv_token);
    network_srv_sockets_append(srv, srv_socket);
    if (*srv).sockets_disabled != 0 {
        return 0 as libc::c_int;
    }
    if (*srv).srvconf.systemd_socket_activation != 0 {
        let mut i_1: uint32_t = 0 as libc::c_int as uint32_t;
        while i_1 < (*srv).srv_sockets_inherited.used {
            if 0 as libc::c_int
                != memcmp(
                    &mut (**((*srv).srv_sockets_inherited.ptr).offset(i_1 as isize)).addr
                        as *mut sock_addr as *const libc::c_void,
                    &mut (*srv_socket).addr as *mut sock_addr as *const libc::c_void,
                    addr_len as libc::c_ulong,
                )
            {
                i_1 = i_1.wrapping_add(1);
            } else {
                if !(0 as libc::c_uint) as libc::c_ushort as libc::c_int
                    == (**((*srv).srv_sockets_inherited.ptr).offset(i_1 as isize)).sidx
                        as libc::c_int
                {
                    (**((*srv).srv_sockets_inherited.ptr).offset(i_1 as isize))
                        .sidx = sidx as libc::c_ushort;
                }
                stdin_fd = (**((*srv).srv_sockets_inherited.ptr).offset(i_1 as isize))
                    .fd;
                break;
            }
        }
    }
    if -(1 as libc::c_int) != stdin_fd {
        (*srv_socket).fd = stdin_fd;
        if -(1 as libc::c_int) == fdevent_fcntl_set_nb_cloexec(stdin_fd) {
            log_perror(
                (*srv).errh,
                b"src/network.c\0" as *const u8 as *const libc::c_char,
                348 as libc::c_int as libc::c_uint,
                b"fcntl\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else if 1 as libc::c_int == family {
        if host.is_null() {
            ck_assert_failed(
                b"src/network.c\0" as *const u8 as *const libc::c_char,
                355 as libc::c_int as libc::c_uint,
                b"host\0" as *const u8 as *const libc::c_char,
            );
        }
        (*srv_socket)
            .fd = fdevent_socket_cloexec(
            1 as libc::c_int,
            SOCK_STREAM as libc::c_int,
            0 as libc::c_int,
        );
        if -(1 as libc::c_int) == (*srv_socket).fd {
            log_perror(
                (*srv).errh,
                b"src/network.c\0" as *const u8 as *const libc::c_char,
                357 as libc::c_int as libc::c_uint,
                b"socket\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if 0 as libc::c_int
            == connect(
                (*srv_socket).fd,
                __CONST_SOCKADDR_ARG {
                    __sockaddr__: &mut (*srv_socket).addr as *mut sock_addr
                        as *mut sockaddr,
                },
                addr_len,
            )
        {
            log_error(
                (*srv).errh,
                b"src/network.c\0" as *const u8 as *const libc::c_char,
                361 as libc::c_int as libc::c_uint,
                b"server socket is still in use: %s\0" as *const u8
                    as *const libc::c_char,
                host,
            );
            return -(1 as libc::c_int);
        }
        match *__errno_location() {
            111 => {
                unlink(host);
            }
            2 => {}
            _ => {
                log_perror(
                    (*srv).errh,
                    b"src/network.c\0" as *const u8 as *const libc::c_char,
                    374 as libc::c_int as libc::c_uint,
                    b"testing socket failed: %s\0" as *const u8 as *const libc::c_char,
                    host,
                );
                return -(1 as libc::c_int);
            }
        }
        if -(1 as libc::c_int) == fdevent_fcntl_set_nb((*srv_socket).fd) {
            log_perror(
                (*srv).errh,
                b"src/network.c\0" as *const u8 as *const libc::c_char,
                380 as libc::c_int as libc::c_uint,
                b"fcntl\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else {
        (*srv_socket)
            .fd = fdevent_socket_nb_cloexec(
            family,
            SOCK_STREAM as libc::c_int,
            IPPROTO_TCP as libc::c_int,
        );
        if -(1 as libc::c_int) == (*srv_socket).fd {
            log_perror(
                (*srv).errh,
                b"src/network.c\0" as *const u8 as *const libc::c_char,
                387 as libc::c_int as libc::c_uint,
                b"socket\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if set_v6only != 0 {
            let mut val: libc::c_int = (set_v6only > 0 as libc::c_int) as libc::c_int;
            if -(1 as libc::c_int)
                == setsockopt(
                    (*srv_socket).fd,
                    IPPROTO_IPV6 as libc::c_int,
                    26 as libc::c_int,
                    &mut val as *mut libc::c_int as *const libc::c_void,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
                )
            {
                log_perror(
                    (*srv).errh,
                    b"src/network.c\0" as *const u8 as *const libc::c_char,
                    395 as libc::c_int as libc::c_uint,
                    b"setsockopt(IPV6_V6ONLY)\0" as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
        }
    }
    (*srv).cur_fds = (*srv_socket).fd;
    if fdevent_set_so_reuseaddr((*srv_socket).fd, 1 as libc::c_int) < 0 as libc::c_int {
        log_perror(
            (*srv).errh,
            b"src/network.c\0" as *const u8 as *const libc::c_char,
            406 as libc::c_int as libc::c_uint,
            b"setsockopt(SO_REUSEADDR)\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if family != 1 as libc::c_int {
        if fdevent_set_tcp_nodelay((*srv_socket).fd, 1 as libc::c_int) < 0 as libc::c_int
        {
            log_perror(
                (*srv).errh,
                b"src/network.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_int as libc::c_uint,
                b"setsockopt(TCP_NODELAY)\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    if !(-(1 as libc::c_int) != stdin_fd) {
        if 0 as libc::c_int
            != bind(
                (*srv_socket).fd,
                __CONST_SOCKADDR_ARG {
                    __sockaddr__: &mut (*srv_socket).addr as *mut sock_addr
                        as *mut sockaddr,
                },
                addr_len,
            )
        {
            log_perror(
                (*srv).errh,
                b"src/network.c\0" as *const u8 as *const libc::c_char,
                419 as libc::c_int as libc::c_uint,
                b"can't bind to socket: %s\0" as *const u8 as *const libc::c_char,
                host,
            );
            return -(1 as libc::c_int);
        }
    }
    if !(-(1 as libc::c_int) != stdin_fd) {
        if 1 as libc::c_int == family && !((*s).socket_perms).is_null() {
            let mut m: mode_t = 0 as libc::c_int as mode_t;
            let mut str: *mut libc::c_char = (*(*s).socket_perms).ptr;
            while *str != 0 {
                m <<= 3 as libc::c_int;
                m |= (*str as libc::c_int - '0' as i32) as libc::c_uint;
                str = str.offset(1);
            }
            if 0 as libc::c_int as libc::c_uint != m
                && -(1 as libc::c_int) == chmod(host, m)
            {
                log_perror(
                    (*srv).errh,
                    b"src/network.c\0" as *const u8 as *const libc::c_char,
                    432 as libc::c_int as libc::c_uint,
                    b"chmod(\"%s\", %s)\0" as *const u8 as *const libc::c_char,
                    host,
                    (*(*s).socket_perms).ptr,
                );
                return -(1 as libc::c_int);
            }
        }
    }
    if !(-(1 as libc::c_int) != stdin_fd) {
        if -(1 as libc::c_int) == listen((*srv_socket).fd, (*s).listen_backlog) {
            log_perror(
                (*srv).errh,
                b"src/network.c\0" as *const u8 as *const libc::c_char,
                440 as libc::c_int as libc::c_uint,
                b"listen\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    if !((*s).ssl_enabled != 0) {
        if (*s).defer_accept != 0 {
            let mut v: libc::c_int = (*s).defer_accept as libc::c_int;
            if -(1 as libc::c_int)
                == setsockopt(
                    (*srv_socket).fd,
                    IPPROTO_TCP as libc::c_int,
                    9 as libc::c_int,
                    &mut v as *mut libc::c_int as *const libc::c_void,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
                )
            {
                log_perror(
                    (*srv).errh,
                    b"src/network.c\0" as *const u8 as *const libc::c_char,
                    450 as libc::c_int as libc::c_uint,
                    b"can't set TCP_DEFER_ACCEPT\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn network_close(mut srv: *mut server) -> libc::c_int {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*srv).srv_sockets.used {
        let mut srv_socket: *mut server_socket = *((*srv).srv_sockets.ptr)
            .offset(i as isize);
        if (*srv_socket).fd != -(1 as libc::c_int) {
            network_unregister_sock(srv, srv_socket);
            close((*srv_socket).fd);
        }
        buffer_free((*srv_socket).srv_token);
        free(srv_socket as *mut libc::c_void);
        i = i.wrapping_add(1);
    }
    free((*srv).srv_sockets.ptr as *mut libc::c_void);
    (*srv).srv_sockets.ptr = 0 as *mut *mut server_socket;
    (*srv).srv_sockets.used = 0 as libc::c_int as uint32_t;
    (*srv).srv_sockets.size = 0 as libc::c_int as uint32_t;
    let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
    while i_0 < (*srv).srv_sockets_inherited.used {
        let mut srv_socket_0: *mut server_socket = *((*srv).srv_sockets_inherited.ptr)
            .offset(i_0 as isize);
        if (*srv_socket_0).fd != -(1 as libc::c_int)
            && (*srv_socket_0).sidx as libc::c_int
                != !(0 as libc::c_uint) as libc::c_ushort as libc::c_int
        {
            close((*srv_socket_0).fd);
        }
        buffer_free((*srv_socket_0).srv_token);
        free(srv_socket_0 as *mut libc::c_void);
        i_0 = i_0.wrapping_add(1);
    }
    free((*srv).srv_sockets_inherited.ptr as *mut libc::c_void);
    (*srv).srv_sockets_inherited.ptr = 0 as *mut *mut server_socket;
    (*srv).srv_sockets_inherited.used = 0 as libc::c_int as uint32_t;
    (*srv).srv_sockets_inherited.size = 0 as libc::c_int as uint32_t;
    return 0 as libc::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn network_socket_activation_to_env(srv: *mut server) {
    let mut fd: libc::c_int = 3 as libc::c_int;
    let mut n: uint32_t = 0 as libc::c_int as uint32_t;
    let mut i: uint32_t = 0;
    while n < (*srv).srv_sockets.used {
        let mut srv_socket: *mut server_socket = *((*srv).srv_sockets.ptr)
            .offset(n as isize);
        if !((*srv_socket).fd < fd) {
            if (*srv_socket).fd == fd {
                fdevent_clrfd_cloexec(fd);
                fd += 1;
            } else {
                i = n.wrapping_add(1 as libc::c_int as libc::c_uint);
                while i < (*srv).srv_sockets.used {
                    if fd == (**((*srv).srv_sockets.ptr).offset(i as isize)).fd {
                        break;
                    }
                    i = i.wrapping_add(1);
                }
                if i < (*srv).srv_sockets.used {
                    fdevent_clrfd_cloexec(fd);
                    fd += 1;
                    n = n.wrapping_sub(1);
                } else if !(fd != dup2((*srv_socket).fd, fd)) {
                    fd += 1;
                }
            }
        }
        n = n.wrapping_add(1);
    }
    fd -= 3 as libc::c_int;
    if 0 as libc::c_int == fd {
        return;
    }
    let tb: *mut buffer = (*srv).tmp_buf;
    buffer_clear(tb);
    buffer_append_int(tb, fd as intmax_t);
    setenv(
        b"LISTEN_FDS\0" as *const u8 as *const libc::c_char,
        (*tb).ptr,
        1 as libc::c_int,
    );
    buffer_clear(tb);
    buffer_append_int(tb, (*srv).pid as intmax_t);
    setenv(
        b"LISTEN_PID\0" as *const u8 as *const libc::c_char,
        (*tb).ptr,
        1 as libc::c_int,
    );
}
unsafe extern "C" fn network_socket_activation_nfds(
    mut srv: *mut server,
    mut s: *mut network_socket_config,
    mut nfds: libc::c_int,
) -> libc::c_int {
    let mut host: *mut buffer = buffer_init();
    let mut addr_len: socklen_t = 0;
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
    let mut rc: libc::c_int = 0 as libc::c_int;
    nfds += 3 as libc::c_int;
    let mut fd: libc::c_int = 3 as libc::c_int;
    while fd < nfds {
        addr_len = ::std::mem::size_of::<sock_addr>() as libc::c_ulong as socklen_t;
        rc = getsockname(
            fd,
            __SOCKADDR_ARG {
                __sockaddr__: &mut addr as *mut sock_addr as *mut sockaddr,
            },
            &mut addr_len,
        );
        if -(1 as libc::c_int) == rc {
            log_perror(
                (*srv).errh,
                b"src/network.c\0" as *const u8 as *const libc::c_char,
                564 as libc::c_int as libc::c_uint,
                b"socket activation getsockname()\0" as *const u8 as *const libc::c_char,
            );
            break;
        } else {
            network_host_normalize_addr_str(host, &mut addr);
            rc = network_server_init(srv, s, host, 0 as libc::c_int as size_t, fd);
            if 0 as libc::c_int != rc {
                break;
            }
            (**((*srv).srv_sockets.ptr)
                .offset(
                    ((*srv).srv_sockets.used)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                ))
                .sidx = !(0 as libc::c_uint) as libc::c_ushort;
            fd += 1;
        }
    }
    buffer_free(host);
    memcpy(
        &mut (*srv).srv_sockets_inherited as *mut server_socket_array
            as *mut libc::c_void,
        &mut (*srv).srv_sockets as *mut server_socket_array as *const libc::c_void,
        ::std::mem::size_of::<server_socket_array>() as libc::c_ulong,
    );
    memset(
        &mut (*srv).srv_sockets as *mut server_socket_array as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<server_socket_array>() as libc::c_ulong,
    );
    return rc;
}
unsafe extern "C" fn network_socket_activation_from_env(
    mut srv: *mut server,
    mut s: *mut network_socket_config,
) -> libc::c_int {
    let mut listen_pid: *mut libc::c_char = getenv(
        b"LISTEN_PID\0" as *const u8 as *const libc::c_char,
    );
    let mut listen_fds: *mut libc::c_char = getenv(
        b"LISTEN_FDS\0" as *const u8 as *const libc::c_char,
    );
    let mut lpid: pid_t = if !listen_pid.is_null() {
        strtoul(listen_pid, 0 as *mut *mut libc::c_char, 10 as libc::c_int) as pid_t
    } else {
        0 as libc::c_int
    };
    let mut nfds: libc::c_int = if !listen_fds.is_null() {
        atoi(listen_fds)
    } else {
        0 as libc::c_int
    };
    let mut rc: libc::c_int = if nfds > 0 as libc::c_int && nfds < 5000 as libc::c_int
        && (lpid == getpid()
            || 0 as libc::c_int
                == strncmp(
                    listen_pid,
                    b"parent:\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as libc::c_ulong,
                )
                && getppid()
                    == strtoul(
                        listen_pid.offset(7 as libc::c_int as isize),
                        0 as *mut *mut libc::c_char,
                        10 as libc::c_int,
                    ) as pid_t)
    {
        network_socket_activation_nfds(srv, s, nfds)
    } else {
        0 as libc::c_int
    };
    unsetenv(b"LISTEN_PID\0" as *const u8 as *const libc::c_char);
    unsetenv(b"LISTEN_FDS\0" as *const u8 as *const libc::c_char);
    unsetenv(b"LISTEN_FDNAMES\0" as *const u8 as *const libc::c_char);
    return rc;
}
static mut cpk: [config_plugin_keys_t; 9] = [config_plugin_keys_t {
    k: 0 as *const libc::c_char,
    klen: 0,
    ktype: 0,
    scope: 0,
}; 9];
#[no_mangle]
#[cold]
pub unsafe extern "C" fn network_init(
    mut srv: *mut server,
    mut stdin_fd: libc::c_int,
) -> libc::c_int {
    if 0 as libc::c_int != network_write_init(srv) {
        return -(1 as libc::c_int);
    }
    let mut np: network_plugin_data = network_plugin_data {
        id: 0,
        nconfig: 0,
        cvlist: 0 as *mut config_plugin_value_t,
        self_0: 0 as *mut plugin,
        defaults: network_socket_config {
            listen_backlog: 0,
            ssl_enabled: 0,
            use_ipv6: 0,
            set_v6only: 0,
            defer_accept: 0,
            v4mapped: 0,
            socket_perms: 0 as *const buffer,
            bsd_accept_filter: 0 as *const buffer,
        },
        conf: network_socket_config {
            listen_backlog: 0,
            ssl_enabled: 0,
            use_ipv6: 0,
            set_v6only: 0,
            defer_accept: 0,
            v4mapped: 0,
            socket_perms: 0 as *const buffer,
            bsd_accept_filter: 0 as *const buffer,
        },
    };
    memset(
        &mut np as *mut network_plugin_data as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<network_plugin_data>() as libc::c_ulong,
    );
    let mut p: *mut network_plugin_data = &mut np;
    if config_plugin_values_init(
        srv,
        p as *mut libc::c_void,
        cpk.as_ptr(),
        b"network\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return HANDLER_ERROR as libc::c_int;
    }
    (*p).defaults.listen_backlog = 1024 as libc::c_int;
    (*p).defaults.defer_accept = 0 as libc::c_int as libc::c_uchar;
    (*p).defaults.use_ipv6 = 0 as libc::c_int as libc::c_uchar;
    (*p).defaults.set_v6only = 1 as libc::c_int as libc::c_uchar;
    (*p).defaults.v4mapped = -(1 as libc::c_int) as int8_t;
    if (*p).nconfig > 0 as libc::c_int
        && (*(*p).cvlist).v.u2[1 as libc::c_int as usize] != 0
    {
        let mut cpv: *const config_plugin_value_t = ((*p).cvlist)
            .offset((*(*p).cvlist).v.u2[0 as libc::c_int as usize] as isize);
        if -(1 as libc::c_int) != (*cpv).k_id {
            network_merge_config(&mut (*p).defaults, cpv);
        }
    }
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut current_block_53: u64;
    if (*srv).srvconf.systemd_socket_activation != 0 {
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while i < (*srv).srv_sockets_inherited.used {
            (**((*srv).srv_sockets_inherited.ptr).offset(i as isize))
                .sidx = !(0 as libc::c_uint) as libc::c_ushort;
            i = i.wrapping_add(1);
        }
        rc = network_socket_activation_from_env(srv, &mut (*p).defaults);
        if 0 as libc::c_int != rc {
            current_block_53 = 9705665520141849625;
        } else {
            if 0 as libc::c_int as libc::c_uint == (*srv).srv_sockets_inherited.used {
                (*srv)
                    .srvconf
                    .systemd_socket_activation = 0 as libc::c_int as libc::c_uchar;
            }
            current_block_53 = 18386322304582297246;
        }
    } else {
        current_block_53 = 18386322304582297246;
    }
    match current_block_53 {
        18386322304582297246 => {
            if -(1 as libc::c_int) != stdin_fd {
                let mut b: *mut buffer = buffer_init();
                buffer_copy_buffer(b, (*srv).srvconf.bindhost);
                rc = if 0 as libc::c_int as libc::c_uint == (*srv).srv_sockets.used {
                    network_server_init(
                        srv,
                        &mut (*p).defaults,
                        b,
                        0 as libc::c_int as size_t,
                        stdin_fd,
                    )
                } else {
                    close(stdin_fd)
                };
                buffer_free(b);
                if 0 as libc::c_int != rc {
                    current_block_53 = 9705665520141849625;
                } else {
                    current_block_53 = 15897653523371991391;
                }
            } else {
                current_block_53 = 15897653523371991391;
            }
            match current_block_53 {
                9705665520141849625 => {}
                _ => {
                    let mut i_0: uint32_t = 1 as libc::c_int as uint32_t;
                    while i_0 < (*(*srv).config_context).used {
                        let mut cfginfo: config_cond_info = config_cond_info {
                            comp: COMP_UNSET,
                            cond: CONFIG_COND_UNSET,
                            string: 0 as *const buffer,
                            comp_key: 0 as *const libc::c_char,
                        };
                        config_get_config_cond_info(&mut cfginfo, i_0);
                        if !(COMP_SERVER_SOCKET as libc::c_int as libc::c_uint
                            != cfginfo.comp as libc::c_uint)
                        {
                            let mut host_token: *mut buffer = 0 as *mut buffer;
                            let ref mut fresh4 = *(&mut host_token as *mut *mut buffer
                                as *mut *const buffer);
                            *fresh4 = cfginfo.string;
                            memcpy(
                                &mut (*p).conf as *mut network_socket_config
                                    as *mut libc::c_void,
                                &mut (*p).defaults as *mut network_socket_config
                                    as *const libc::c_void,
                                ::std::mem::size_of::<network_socket_config>()
                                    as libc::c_ulong,
                            );
                            let mut j: libc::c_int = ((*((*p).cvlist)
                                .offset(0 as libc::c_int as isize))
                                .v
                                .u2[1 as libc::c_int as usize] == 0) as libc::c_int;
                            while j < (*p).nconfig {
                                if i_0 as libc::c_int
                                    != (*((*p).cvlist).offset(j as isize)).k_id
                                {
                                    j += 1;
                                } else {
                                    let mut cpv_0: *const config_plugin_value_t = ((*p).cvlist)
                                        .offset(
                                            (*((*p).cvlist).offset(j as isize))
                                                .v
                                                .u2[0 as libc::c_int as usize] as isize,
                                        );
                                    network_merge_config(&mut (*p).conf, cpv_0);
                                    break;
                                }
                            }
                            if cfginfo.cond as libc::c_uint
                                == CONFIG_COND_EQ as libc::c_int as libc::c_uint
                            {
                                rc = network_server_init(
                                    srv,
                                    &mut (*p).conf,
                                    host_token,
                                    i_0 as size_t,
                                    -(1 as libc::c_int),
                                );
                                if 0 as libc::c_int != rc {
                                    break;
                                }
                            } else if cfginfo.cond as libc::c_uint
                                    == CONFIG_COND_NE as libc::c_int as libc::c_uint
                                {
                                let mut addr_len: socklen_t = ::std::mem::size_of::<
                                    sock_addr,
                                >() as libc::c_ulong as socklen_t;
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
                                rc = network_host_parse_addr(
                                    srv,
                                    &mut addr,
                                    &mut addr_len,
                                    host_token,
                                    (*p).conf.use_ipv6 as libc::c_int,
                                );
                                if 0 as libc::c_int != rc {
                                    break;
                                }
                                network_host_normalize_addr_str(host_token, &mut addr);
                            }
                        }
                        i_0 = i_0.wrapping_add(1);
                    }
                    if !(0 as libc::c_int != rc) {
                        if ((*srv).srvconf.systemd_socket_activation == 0
                            || !((*srv).srvconf.bindhost).is_null())
                            && -(1 as libc::c_int) == stdin_fd
                        {
                            let mut b_0: *mut buffer = buffer_init();
                            if !((*srv).srvconf.bindhost).is_null() {
                                buffer_copy_buffer(b_0, (*srv).srvconf.bindhost);
                            }
                            if ((*b_0).ptr).is_null()
                                || *((*b_0).ptr).offset(0 as libc::c_int as isize)
                                    as libc::c_int != '/' as i32
                            {
                                buffer_append_string_len(
                                    b_0,
                                    b":\0" as *const u8 as *const libc::c_char,
                                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                        as uint32_t)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                );
                                buffer_append_int(b_0, (*srv).srvconf.port as intmax_t);
                            }
                            rc = network_server_init(
                                srv,
                                &mut (*p).defaults,
                                b_0,
                                0 as libc::c_int as size_t,
                                -(1 as libc::c_int),
                            );
                            buffer_free(b_0);
                            if 0 as libc::c_int != rc {
                                current_block_53 = 9705665520141849625;
                            } else {
                                current_block_53 = 981995395831942902;
                            }
                        } else {
                            current_block_53 = 981995395831942902;
                        }
                        match current_block_53 {
                            9705665520141849625 => {}
                            _ => {
                                if (*srv).srvconf.systemd_socket_activation != 0 {
                                    let mut srv_socket: *mut server_socket = 0
                                        as *mut server_socket;
                                    let mut i_1: uint32_t = 0 as libc::c_int as uint32_t;
                                    while i_1 < (*srv).srv_sockets_inherited.used {
                                        if !(!(0 as libc::c_uint) as libc::c_ushort as libc::c_int
                                            != (**((*srv).srv_sockets_inherited.ptr)
                                                .offset(i_1 as isize))
                                                .sidx as libc::c_int)
                                        {
                                            (**((*srv).srv_sockets_inherited.ptr).offset(i_1 as isize))
                                                .sidx = 0 as libc::c_int as libc::c_ushort;
                                            srv_socket = calloc(
                                                1 as libc::c_int as libc::c_ulong,
                                                ::std::mem::size_of::<server_socket>() as libc::c_ulong,
                                            ) as *mut server_socket;
                                            if srv_socket.is_null() {
                                                ck_assert_failed(
                                                    b"src/network.c\0" as *const u8 as *const libc::c_char,
                                                    764 as libc::c_int as libc::c_uint,
                                                    b"((void*)0) != srv_socket\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            }
                                            memcpy(
                                                srv_socket as *mut libc::c_void,
                                                *((*srv).srv_sockets_inherited.ptr).offset(i_1 as isize)
                                                    as *const libc::c_void,
                                                ::std::mem::size_of::<server_socket>() as libc::c_ulong,
                                            );
                                            let srv_token: *const buffer = (*srv_socket).srv_token;
                                            (*srv_socket).srv_token = buffer_init();
                                            buffer_copy_buffer((*srv_socket).srv_token, srv_token);
                                            (*srv_socket)
                                                .srv_token_colon = network_srv_token_colon(
                                                (*srv_socket).srv_token,
                                            );
                                            network_srv_sockets_append(srv, srv_socket);
                                        }
                                        i_1 = i_1.wrapping_add(1);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    free((*p).cvlist as *mut libc::c_void);
    return rc;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn network_unregister_sock(
    mut srv: *mut server,
    mut srv_socket: *mut server_socket,
) {
    let mut fdn: *mut fdnode = (*srv_socket).fdn;
    if fdn.is_null() {
        return;
    }
    fdevent_fdnode_event_del((*srv).ev, fdn);
    fdevent_unregister((*srv).ev, (*fdn).fd);
    (*srv_socket).fdn = 0 as *mut fdnode;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn network_register_fdevents(mut srv: *mut server) -> libc::c_int {
    if -(1 as libc::c_int) == fdevent_reset((*srv).ev) {
        return -(1 as libc::c_int);
    }
    if (*srv).sockets_disabled != 0 {
        return 0 as libc::c_int;
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*srv).srv_sockets.used {
        let mut srv_socket: *mut server_socket = *((*srv).srv_sockets.ptr)
            .offset(i as isize);
        (*srv_socket)
            .fdn = fdevent_register(
            (*srv).ev,
            (*srv_socket).fd,
            Some(
                network_server_handle_fdevent
                    as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> handler_t,
            ),
            srv_socket as *mut libc::c_void,
        );
        fdevent_fdnode_event_set((*srv).ev, (*srv_socket).fdn, 0x1 as libc::c_int);
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    cpk = [
        {
            let mut init = config_plugin_keys_t {
                k: b"ssl.engine\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.listen-backlog\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_INT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.socket-perms\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.bsd-accept-filter\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.defer-accept\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.use-ipv6\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.set-v6only\0" as *const u8 as *const libc::c_char,
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
                k: b"server.v4mapped\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
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
