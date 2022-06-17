use ::libc;
extern "C" {
    pub type sockaddr_x25;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    pub type stat_cache_entry;
    pub type pcre2_real_match_data_8;
    pub type h2con;
    pub type fdevents;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
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
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
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
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn buffer_init() -> *mut buffer;
    fn buffer_free(b: *mut buffer);
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
    fn buffer_append_int(b: *mut buffer, val: intmax_t);
    fn li_utostrn(buf: *mut libc::c_char, buf_len: size_t, val: uintmax_t) -> size_t;
    fn buffer_eq_slen(
        b: *const buffer,
        s: *const libc::c_char,
        slen: size_t,
    ) -> libc::c_int;
    fn buffer_is_equal(a: *const buffer, b: *const buffer) -> libc::c_int;
    fn buffer_path_simplify(b: *mut buffer);
    fn buffer_copy_path_len2(
        b: *mut buffer,
        s1: *const libc::c_char,
        len1: size_t,
        s2: *const libc::c_char,
        len2: size_t,
    );
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn array_is_kvany(a: *const array) -> libc::c_int;
    fn array_get_data_unset(
        a: *const array,
        key: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut data_unset;
    fn array_get_int_ptr(
        a: *mut array,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut libc::c_int;
    fn array_match_key_suffix(a: *const array, b: *const buffer) -> *mut data_unset;
    fn http_response_backend_error(r: *mut request_st);
    fn http_response_backend_done(r: *mut request_st);
    fn chunk_buffer_acquire() -> *mut buffer;
    fn chunk_buffer_release(b: *mut buffer);
    fn chunkqueue_init(cq: *mut chunkqueue) -> *mut chunkqueue;
    fn chunkqueue_append_chunkqueue(cq: *mut chunkqueue, src: *mut chunkqueue);
    fn chunkqueue_free(cq: *mut chunkqueue);
    fn chunkqueue_reset(cq: *mut chunkqueue);
    fn config_plugin_values_init_block(
        srv: *mut server,
        ca: *const array,
        cpk: *const config_plugin_keys_t,
        mname: *const libc::c_char,
        cpv: *mut config_plugin_value_t,
    ) -> libc::c_int;
    fn config_feature_bool(
        srv: *const server,
        feature: *const libc::c_char,
        default_value: libc::c_int,
    ) -> libc::c_int;
    static mut plugin_stats: array;
    fn http_response_reset(r: *mut request_st);
    fn http_response_read(
        r: *mut request_st,
        opts: *mut http_response_opts,
        b: *mut buffer,
        fdn: *mut fdnode,
    ) -> handler_t;
    fn http_response_reqbody_read_error(
        r: *mut request_st,
        http_status: libc::c_int,
    ) -> handler_t;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn sock_addr_inet_ntop_copy_buffer(
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
    static mut log_con_jqueue: *mut connection;
    fn fdevent_fdnode_event_del(ev: *mut fdevents, fdn: *mut fdnode);
    fn fdevent_fdnode_event_set(
        ev: *mut fdevents,
        fdn: *mut fdnode,
        events: libc::c_int,
    );
    fn fdevent_fdnode_event_add(ev: *mut fdevents, fdn: *mut fdnode, event: libc::c_int);
    fn fdevent_fdnode_event_clr(ev: *mut fdevents, fdn: *mut fdnode, event: libc::c_int);
    fn fdevent_register(
        ev: *mut fdevents,
        fd: libc::c_int,
        handler: fdevent_handler,
        ctx: *mut libc::c_void,
    ) -> *mut fdnode;
    fn fdevent_sched_close(ev: *mut fdevents, fd: libc::c_int, issock: libc::c_int);
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
    fn fdevent_environ() -> *mut *mut libc::c_char;
    fn fdevent_open_dirname(
        path: *mut libc::c_char,
        symlinks: libc::c_int,
    ) -> libc::c_int;
    fn fdevent_fork_execve(
        name: *const libc::c_char,
        argv: *mut *mut libc::c_char,
        envp: *mut *mut libc::c_char,
        fdin: libc::c_int,
        fdout: libc::c_int,
        fderr: libc::c_int,
        dfd: libc::c_int,
    ) -> pid_t;
    fn fdevent_waitpid(
        pid: pid_t,
        status: *mut libc::c_int,
        nb: libc::c_int,
    ) -> libc::c_int;
    fn fdevent_connect_status(fd: libc::c_int) -> libc::c_int;
    fn fdevent_set_tcp_nodelay(fd: libc::c_int, opt: libc::c_int) -> libc::c_int;
    fn fdevent_set_so_reuseaddr(fd: libc::c_int, opt: libc::c_int) -> libc::c_int;
    static mut log_monotonic_secs: unix_time64_t;
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
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
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
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
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
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type unix_time64_t = time_t;
pub type unix_timespec64_t = timespec;
pub type socklen_t = __socklen_t;
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
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type C2RustUnnamed = libc::c_uint;
pub const SHUT_RDWR: C2RustUnnamed = 2;
pub const SHUT_WR: C2RustUnnamed = 1;
pub const SHUT_RD: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
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
pub type in_port_t = uint16_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub ptr: *mut libc::c_char,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct char_array {
    pub ptr: *mut *mut libc::c_char,
    pub size: uint32_t,
    pub used: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gw_proc {
    pub next: *mut gw_proc,
    pub state: C2RustUnnamed_1,
    pub load: uint32_t,
    pub last_used: unix_time64_t,
    pub stats_load: *mut libc::c_int,
    pub stats_connected: *mut libc::c_int,
    pub pid: pid_t,
    pub is_local: libc::c_int,
    pub id: uint32_t,
    pub saddrlen: socklen_t,
    pub saddr: *mut sockaddr,
    pub disabled_until: unix_time64_t,
    pub prev: *mut gw_proc,
    pub connection_name: *mut buffer,
    pub unixsocket: *mut buffer,
    pub port: libc::c_ushort,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const PROC_STATE_KILLED: C2RustUnnamed_1 = 4;
pub const PROC_STATE_DIED: C2RustUnnamed_1 = 3;
pub const PROC_STATE_DIED_WAIT_FOR_PID: C2RustUnnamed_1 = 2;
pub const PROC_STATE_OVERLOADED: C2RustUnnamed_1 = 1;
pub const PROC_STATE_RUNNING: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gw_handler_ctx {
    pub proc_0: *mut gw_proc,
    pub host: *mut gw_host,
    pub ext: *mut gw_extension,
    pub ext_auth: *mut gw_extension,
    pub gw_mode: libc::c_ushort,
    pub state: gw_connection_state_t,
    pub rb: *mut chunkqueue,
    pub wb_reqlen: off_t,
    pub wb: chunkqueue,
    pub response: *mut buffer,
    pub ev: *mut fdevents,
    pub fdn: *mut fdnode,
    pub fd: libc::c_int,
    pub revents: libc::c_int,
    pub pid: pid_t,
    pub reconnects: libc::c_int,
    pub request_id: libc::c_int,
    pub send_content_body: libc::c_int,
    pub opts: http_response_opts,
    pub conf: gw_plugin_config,
    pub r: *mut request_st,
    pub con: *mut connection,
    pub plugin_data: *mut gw_plugin_data,
    pub read_ts: unix_time64_t,
    pub write_ts: unix_time64_t,
    pub stdin_append: Option::<unsafe extern "C" fn(*mut gw_handler_ctx) -> handler_t>,
    pub create_env: Option::<unsafe extern "C" fn(*mut gw_handler_ctx) -> handler_t>,
    pub prev: *mut gw_handler_ctx,
    pub next: *mut gw_handler_ctx,
    pub backend_error: Option::<unsafe extern "C" fn(*mut gw_handler_ctx) -> ()>,
    pub handler_ctx_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
pub type handler_t = libc::c_uint;
pub const HANDLER_ERROR: handler_t = 4;
pub const HANDLER_WAIT_FOR_EVENT: handler_t = 3;
pub const HANDLER_COMEBACK: handler_t = 2;
pub const HANDLER_FINISHED: handler_t = 1;
pub const HANDLER_GO_ON: handler_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gw_plugin_data {
    pub id: libc::c_int,
    pub nconfig: libc::c_int,
    pub cvlist: *mut config_plugin_value_t,
    pub self_0: *mut plugin,
    pub srv_pid: pid_t,
    pub conf: gw_plugin_config,
    pub defaults: gw_plugin_config,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gw_plugin_config {
    pub exts: *mut gw_exts,
    pub exts_auth: *mut gw_exts,
    pub exts_resp: *mut gw_exts,
    pub ext_mapping: *const array,
    pub balance: libc::c_int,
    pub proto: libc::c_int,
    pub debug: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gw_exts {
    pub exts: *mut gw_extension,
    pub used: uint32_t,
    pub size: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gw_extension {
    pub key: buffer,
    pub note_is_sent: libc::c_int,
    pub last_used_ndx: libc::c_int,
    pub hosts: *mut *mut gw_host,
    pub used: uint32_t,
    pub size: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gw_host {
    pub first: *mut gw_proc,
    pub active_procs: uint32_t,
    pub gw_hash: uint32_t,
    pub load: int32_t,
    pub stats_load: *mut libc::c_int,
    pub stats_global_active: *mut libc::c_int,
    pub port: libc::c_ushort,
    pub family: libc::c_ushort,
    pub host: *const buffer,
    pub id: *const buffer,
    pub unused_procs: *mut gw_proc,
    pub min_procs: libc::c_ushort,
    pub max_procs: libc::c_ushort,
    pub num_procs: uint32_t,
    pub max_load_per_proc: libc::c_ushort,
    pub idle_timeout: libc::c_ushort,
    pub disable_time: libc::c_ushort,
    pub read_timeout: libc::c_ushort,
    pub write_timeout: libc::c_ushort,
    pub connect_timeout: libc::c_ushort,
    pub hctxs: *mut gw_handler_ctx,
    pub unixsocket: *const buffer,
    pub bin_path: *const buffer,
    pub bin_env: *const array,
    pub bin_env_copy: *const array,
    pub docroot: *const buffer,
    pub break_scriptfilename_for_php: libc::c_ushort,
    pub check_local: libc::c_ushort,
    pub fix_root_path_name: libc::c_ushort,
    pub xsendfile_allow: libc::c_ushort,
    pub xsendfile_docroot: *const array,
    pub max_id: uint32_t,
    pub strip_request_uri: *const buffer,
    pub tcp_fin_propagate: libc::c_ushort,
    pub kill_signal: libc::c_ushort,
    pub listen_backlog: libc::c_int,
    pub refcount: libc::c_int,
    pub args: char_array,
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
    pub plugins: C2RustUnnamed_2,
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
pub struct C2RustUnnamed_2 {
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
pub type log_error_st = fdlog_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fdlog_st {
    pub mode: C2RustUnnamed_3,
    pub fd: libc::c_int,
    pub b: buffer,
    pub fn_0: *const libc::c_char,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const FDLOG_PIPE: C2RustUnnamed_3 = 3;
pub const FDLOG_SYSLOG: C2RustUnnamed_3 = 2;
pub const FDLOG_FD: C2RustUnnamed_3 = 1;
pub const FDLOG_FILE: C2RustUnnamed_3 = 0;
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
    pub type_0: C2RustUnnamed_6,
    pub mem: *mut buffer,
    pub offset: off_t,
    pub file: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub length: off_t,
    pub fd: libc::c_int,
    pub is_temp: libc::c_int,
    pub mmap: C2RustUnnamed_5,
    pub ref_0: *mut libc::c_void,
    pub refchg: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub start: *mut libc::c_char,
    pub length: size_t,
    pub offset: off_t,
}
pub type C2RustUnnamed_6 = libc::c_uint;
pub const FILE_CHUNK: C2RustUnnamed_6 = 1;
pub const MEM_CHUNK: C2RustUnnamed_6 = 0;
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
pub type http_response_opts = http_response_opts_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_response_opts_t {
    pub max_per_read: uint32_t,
    pub fdfmt: libc::c_int,
    pub backend: libc::c_int,
    pub authorizer: libc::c_int,
    pub simple_accum: uint8_t,
    pub local_redir: uint8_t,
    pub xsendfile_allow: uint8_t,
    pub xsendfile_docroot: *const array,
    pub pdata: *mut libc::c_void,
    pub parse: Option::<
        unsafe extern "C" fn(
            *mut request_st,
            *mut http_response_opts_t,
            *mut buffer,
            size_t,
        ) -> handler_t,
    >,
    pub headers: Option::<
        unsafe extern "C" fn(*mut request_st, *mut http_response_opts_t) -> handler_t,
    >,
}
pub type gw_connection_state_t = libc::c_uint;
pub const GW_STATE_READ: gw_connection_state_t = 4;
pub const GW_STATE_WRITE: gw_connection_state_t = 3;
pub const GW_STATE_PREPARE_WRITE: gw_connection_state_t = 2;
pub const GW_STATE_CONNECT_DELAYED: gw_connection_state_t = 1;
pub const GW_STATE_INIT: gw_connection_state_t = 0;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const T_CONFIG_SCOPE_CONNECTION: C2RustUnnamed_7 = 2;
pub const T_CONFIG_SCOPE_SERVER: C2RustUnnamed_7 = 1;
pub const T_CONFIG_SCOPE_UNSET: C2RustUnnamed_7 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_plugin_keys_t {
    pub k: *const libc::c_char,
    pub klen: uint8_t,
    pub ktype: uint8_t,
    pub scope: uint8_t,
}
pub type C2RustUnnamed_8 = libc::c_uint;
pub const BACKEND_AJP13: C2RustUnnamed_8 = 4;
pub const BACKEND_SCGI: C2RustUnnamed_8 = 3;
pub const BACKEND_FASTCGI: C2RustUnnamed_8 = 2;
pub const BACKEND_CGI: C2RustUnnamed_8 = 1;
pub const BACKEND_PROXY: C2RustUnnamed_8 = 0;
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
pub const GW_BALANCE_LEAST_CONNECTION: C2RustUnnamed_9 = 0;
pub const GW_BALANCE_STICKY: C2RustUnnamed_9 = 3;
pub const GW_BALANCE_HASH: C2RustUnnamed_9 = 2;
pub const GW_BALANCE_RR: C2RustUnnamed_9 = 1;
pub const HTTP_HEADER_UPGRADE: http_header_e = 49;
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
pub type C2RustUnnamed_9 = libc::c_uint;
#[inline]
unsafe extern "C" fn buffer_clear(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn buffer_has_slash_suffix(b: *const buffer) -> libc::c_int {
    return ((*b).used > 1 as libc::c_int as libc::c_uint
        && *((*b).ptr)
            .offset(((*b).used).wrapping_sub(2 as libc::c_int as libc::c_uint) as isize)
            as libc::c_int == '/' as i32) as libc::c_int;
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
unsafe extern "C" fn buffer_truncate(mut b: *mut buffer, mut len: uint32_t) {
    *((*b).ptr).offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    (*b).used = len.wrapping_add(1 as libc::c_int as libc::c_uint);
}
#[inline]
unsafe extern "C" fn buffer_append_slash(mut b: *mut buffer) {
    let len: uint32_t = buffer_clen(b);
    if len > 0 as libc::c_int as libc::c_uint
        && '/' as i32
            != *((*b).ptr)
                .offset(len.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
                as libc::c_int
    {
        buffer_append_string_len(
            b,
            b"/\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
}
#[inline]
unsafe extern "C" fn chunkqueue_length(mut cq: *const chunkqueue) -> off_t {
    return (*cq).bytes_in - (*cq).bytes_out;
}
#[inline]
unsafe extern "C" fn chunkqueue_is_empty(mut cq: *const chunkqueue) -> libc::c_int {
    return (0 as *mut libc::c_void as *mut chunk == (*cq).first) as libc::c_int;
}
#[inline]
unsafe extern "C" fn sock_addr_get_family(mut saddr: *const sock_addr) -> libc::c_int {
    return (*saddr).plain.sa_family as libc::c_int;
}
#[inline]
unsafe extern "C" fn connection_jq_append(con: *mut connection) {
    if ((*con).jqnext).is_null() {
        (*con).jqnext = log_con_jqueue;
        log_con_jqueue = con;
    }
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
#[inline]
unsafe extern "C" fn status_counter_get_counter(
    mut s: *const libc::c_char,
    mut len: size_t,
) -> *mut libc::c_int {
    return array_get_int_ptr(&mut plugin_stats, s, len as uint32_t);
}
#[inline(never)]
unsafe extern "C" fn gw_status_get_counter(
    mut host: *mut gw_host,
    mut proc_0: *mut gw_proc,
    mut tag: *const libc::c_char,
    mut tlen: size_t,
) -> *mut libc::c_int {
    let mut label: [libc::c_char; 288] = [0; 288];
    let mut llen: size_t = (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut len: size_t = 0;
    memcpy(
        label.as_mut_ptr() as *mut libc::c_void,
        b"gw.backend.\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        llen,
    );
    len = buffer_clen((*host).id) as size_t;
    if len != 0 {
        if !(len
            < (::std::mem::size_of::<[libc::c_char; 288]>() as libc::c_ulong)
                .wrapping_sub(llen))
        {
            ck_assert_failed(
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                56 as libc::c_int as libc::c_uint,
                b"len < sizeof(label) - llen\0" as *const u8 as *const libc::c_char,
            );
        }
        memcpy(
            label.as_mut_ptr().offset(llen as isize) as *mut libc::c_void,
            (*(*host).id).ptr as *const libc::c_void,
            len,
        );
        llen = (llen as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    }
    if !proc_0.is_null() {
        if !(llen
            < (::std::mem::size_of::<[libc::c_char; 288]>() as libc::c_ulong)
                .wrapping_sub(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            (8 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<intmax_t>() as libc::c_ulong,
                                )
                                .wrapping_mul(31 as libc::c_int as libc::c_ulong)
                                .wrapping_add(99 as libc::c_int as libc::c_ulong)
                                .wrapping_div(100 as libc::c_int as libc::c_ulong),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ))
        {
            ck_assert_failed(
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                62 as libc::c_int as libc::c_uint,
                b"llen < sizeof(label) - ((2 + (8 * sizeof(intmax_t) * 31 + 99) / 100) + 1)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        let fresh0 = llen;
        llen = llen.wrapping_add(1);
        label[fresh0 as usize] = '.' as i32 as libc::c_char;
        len = li_utostrn(
            label.as_mut_ptr().offset(llen as isize),
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (8 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<intmax_t>() as libc::c_ulong)
                        .wrapping_mul(31 as libc::c_int as libc::c_ulong)
                        .wrapping_add(99 as libc::c_int as libc::c_ulong)
                        .wrapping_div(100 as libc::c_int as libc::c_ulong),
                ),
            (*proc_0).id as uintmax_t,
        );
        llen = (llen as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    }
    if !(tlen
        < (::std::mem::size_of::<[libc::c_char; 288]>() as libc::c_ulong)
            .wrapping_sub(llen))
    {
        ck_assert_failed(
            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int as libc::c_uint,
            b"tlen < sizeof(label) - llen\0" as *const u8 as *const libc::c_char,
        );
    }
    memcpy(
        label.as_mut_ptr().offset(llen as isize) as *mut libc::c_void,
        tag as *const libc::c_void,
        tlen,
    );
    llen = (llen as libc::c_ulong).wrapping_add(tlen) as size_t as size_t;
    label[llen as usize] = '\u{0}' as i32 as libc::c_char;
    return status_counter_get_counter(label.as_mut_ptr(), llen);
}
unsafe extern "C" fn gw_proc_tag_inc(
    mut host: *mut gw_host,
    mut proc_0: *mut gw_proc,
    mut tag: *const libc::c_char,
    mut len: size_t,
) {
    let ref mut fresh1 = *gw_status_get_counter(host, proc_0, tag, len);
    *fresh1 += 1;
}
unsafe extern "C" fn gw_proc_connected_inc(
    mut host: *mut gw_host,
    mut proc_0: *mut gw_proc,
) {
    *(*proc_0).stats_connected += 1;
}
unsafe extern "C" fn gw_proc_load_inc(mut host: *mut gw_host, mut proc_0: *mut gw_proc) {
    (*proc_0).load = ((*proc_0).load).wrapping_add(1);
    *(*proc_0).stats_load = (*proc_0).load as libc::c_int;
    *(*host).stats_global_active += 1;
}
unsafe extern "C" fn gw_proc_load_dec(mut host: *mut gw_host, mut proc_0: *mut gw_proc) {
    (*proc_0).load = ((*proc_0).load).wrapping_sub(1);
    *(*proc_0).stats_load = (*proc_0).load as libc::c_int;
    *(*host).stats_global_active -= 1;
}
unsafe extern "C" fn gw_host_assign(mut host: *mut gw_host) {
    (*host).load += 1;
    *(*host).stats_load = (*host).load;
}
unsafe extern "C" fn gw_host_reset(mut host: *mut gw_host) {
    (*host).load -= 1;
    *(*host).stats_load = (*host).load;
}
unsafe extern "C" fn gw_status_init_proc(
    mut host: *mut gw_host,
    mut proc_0: *mut gw_proc,
) {
    *gw_status_get_counter(
        host,
        proc_0,
        b".disabled\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    ) = 0 as libc::c_int;
    *gw_status_get_counter(
        host,
        proc_0,
        b".died\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    ) = 0 as libc::c_int;
    *gw_status_get_counter(
        host,
        proc_0,
        b".overloaded\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    ) = 0 as libc::c_int;
    (*proc_0)
        .stats_connected = gw_status_get_counter(
        host,
        proc_0,
        b".connected\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    *(*proc_0).stats_connected = 0 as libc::c_int;
    (*proc_0)
        .stats_load = gw_status_get_counter(
        host,
        proc_0,
        b".load\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    *(*proc_0).stats_load = 0 as libc::c_int;
}
unsafe extern "C" fn gw_status_init_host(mut host: *mut gw_host) {
    (*host)
        .stats_load = gw_status_get_counter(
        host,
        0 as *mut gw_proc,
        b".load\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    *(*host).stats_load = 0 as libc::c_int;
    (*host)
        .stats_global_active = status_counter_get_counter(
        b"gw.active-requests\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
}
#[cold]
unsafe extern "C" fn gw_proc_set_state(
    mut host: *mut gw_host,
    mut proc_0: *mut gw_proc,
    mut state: libc::c_int,
) {
    if (*proc_0).state as libc::c_int == state {
        return;
    }
    if (*proc_0).state as libc::c_uint
        == PROC_STATE_RUNNING as libc::c_int as libc::c_uint
    {
        (*host).active_procs = ((*host).active_procs).wrapping_sub(1);
    } else if state == PROC_STATE_RUNNING as libc::c_int {
        (*host).active_procs = ((*host).active_procs).wrapping_add(1);
    }
    (*proc_0).state = state as C2RustUnnamed_1;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn gw_proc_init_portpath(
    mut host: *mut gw_host,
    mut proc_0: *mut gw_proc,
) {
    if ((*host).unixsocket).is_null() {
        (*proc_0)
            .port = ((*host).port as libc::c_uint).wrapping_add((*proc_0).id)
            as libc::c_ushort;
        return;
    }
    if ((*proc_0).unixsocket).is_null() {
        (*proc_0).unixsocket = buffer_init();
    }
    if ((*host).bin_path).is_null() {
        buffer_copy_buffer((*proc_0).unixsocket, (*host).unixsocket);
    } else {
        buffer_clear((*proc_0).unixsocket);
        buffer_append_str2(
            (*proc_0).unixsocket,
            (*(*host).unixsocket).ptr,
            buffer_clen((*host).unixsocket) as size_t,
            b"-\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        buffer_append_int((*proc_0).unixsocket, (*proc_0).id as intmax_t);
    };
}
#[cold]
#[inline(never)]
unsafe extern "C" fn gw_proc_init(mut host: *mut gw_host) -> *mut gw_proc {
    let mut proc_0: *mut gw_proc = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gw_proc>() as libc::c_ulong,
    ) as *mut gw_proc;
    if proc_0.is_null() {
        ck_assert_failed(
            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
            164 as libc::c_int as libc::c_uint,
            b"proc\0" as *const u8 as *const libc::c_char,
        );
    }
    (*proc_0).connection_name = buffer_init();
    (*proc_0).prev = 0 as *mut gw_proc;
    (*proc_0).next = 0 as *mut gw_proc;
    (*proc_0).state = PROC_STATE_DIED;
    let fresh2 = (*host).max_id;
    (*host).max_id = ((*host).max_id).wrapping_add(1);
    (*proc_0).id = fresh2;
    gw_status_init_proc(host, proc_0);
    gw_proc_init_portpath(host, proc_0);
    return proc_0;
}
unsafe extern "C" fn gw_proc_free(mut proc_0: *mut gw_proc) {
    if proc_0.is_null() {
        return;
    }
    gw_proc_free((*proc_0).next);
    buffer_free((*proc_0).unixsocket);
    buffer_free((*proc_0).connection_name);
    free((*proc_0).saddr as *mut libc::c_void);
    free(proc_0 as *mut libc::c_void);
}
unsafe extern "C" fn gw_host_init() -> *mut gw_host {
    let mut f: *mut gw_host = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gw_host>() as libc::c_ulong,
    ) as *mut gw_host;
    if f.is_null() {
        ck_assert_failed(
            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
            196 as libc::c_int as libc::c_uint,
            b"f\0" as *const u8 as *const libc::c_char,
        );
    }
    return f;
}
unsafe extern "C" fn gw_host_free(mut h: *mut gw_host) {
    if h.is_null() {
        return;
    }
    if (*h).refcount != 0 {
        (*h).refcount -= 1;
        return;
    }
    gw_proc_free((*h).first);
    gw_proc_free((*h).unused_procs);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*h).args.used {
        free(*((*h).args.ptr).offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1);
    }
    free((*h).args.ptr as *mut libc::c_void);
    free(h as *mut libc::c_void);
}
unsafe extern "C" fn gw_extensions_init() -> *mut gw_exts {
    let mut f: *mut gw_exts = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gw_exts>() as libc::c_ulong,
    ) as *mut gw_exts;
    if f.is_null() {
        ck_assert_failed(
            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
            219 as libc::c_int as libc::c_uint,
            b"f\0" as *const u8 as *const libc::c_char,
        );
    }
    return f;
}
unsafe extern "C" fn gw_extensions_free(mut f: *mut gw_exts) {
    if f.is_null() {
        return;
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*f).used {
        let mut fe: *mut gw_extension = ((*f).exts).offset(i as isize);
        let mut j: uint32_t = 0 as libc::c_int as uint32_t;
        while j < (*fe).used {
            gw_host_free(*((*fe).hosts).offset(j as isize));
            j = j.wrapping_add(1);
        }
        free((*fe).hosts as *mut libc::c_void);
        i = i.wrapping_add(1);
    }
    free((*f).exts as *mut libc::c_void);
    free(f as *mut libc::c_void);
}
unsafe extern "C" fn gw_extension_insert(
    mut ext: *mut gw_exts,
    mut key: *const buffer,
    mut fh: *mut gw_host,
) -> libc::c_int {
    let mut fe: *mut gw_extension = 0 as *mut gw_extension;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*ext).used {
        if buffer_is_equal(key, &(*((*ext).exts).offset(i as isize)).key) != 0 {
            fe = ((*ext).exts).offset(i as isize);
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    if fe.is_null() {
        if (*ext).used == (*ext).size {
            (*ext)
                .size = ((*ext).size as libc::c_uint)
                .wrapping_add(8 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
            (*ext)
                .exts = realloc(
                (*ext).exts as *mut libc::c_void,
                ((*ext).size as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<gw_extension>() as libc::c_ulong),
            ) as *mut gw_extension;
            if ((*ext).exts).is_null() {
                ck_assert_failed(
                    b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                    249 as libc::c_int as libc::c_uint,
                    b"ext->exts\0" as *const u8 as *const libc::c_char,
                );
            }
            memset(
                ((*ext).exts).offset((*ext).used as isize) as *mut libc::c_void,
                0 as libc::c_int,
                (8 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<gw_extension>() as libc::c_ulong),
            );
        }
        let fresh3 = (*ext).used;
        (*ext).used = ((*ext).used).wrapping_add(1);
        fe = ((*ext).exts).offset(fresh3 as isize);
        (*fe).last_used_ndx = -(1 as libc::c_int);
        let mut b: *mut buffer = 0 as *mut buffer;
        let ref mut fresh4 = *(&mut b as *mut *mut buffer as *mut *const buffer);
        *fresh4 = &(*fe).key;
        memcpy(
            b as *mut libc::c_void,
            key as *const libc::c_void,
            ::std::mem::size_of::<buffer>() as libc::c_ulong,
        );
    }
    if (*fe).size == (*fe).used {
        (*fe)
            .size = ((*fe).size as libc::c_uint)
            .wrapping_add(4 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
        (*fe)
            .hosts = realloc(
            (*fe).hosts as *mut libc::c_void,
            ((*fe).size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut gw_host>() as libc::c_ulong),
        ) as *mut *mut gw_host;
        if ((*fe).hosts).is_null() {
            ck_assert_failed(
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                262 as libc::c_int as libc::c_uint,
                b"fe->hosts\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    let fresh5 = (*fe).used;
    (*fe).used = ((*fe).used).wrapping_add(1);
    let ref mut fresh6 = *((*fe).hosts).offset(fresh5 as isize);
    *fresh6 = fh;
    return 0 as libc::c_int;
}
unsafe extern "C" fn gw_proc_connect_success(
    mut host: *mut gw_host,
    mut proc_0: *mut gw_proc,
    mut debug: libc::c_int,
    r: *mut request_st,
) {
    gw_proc_connected_inc(host, proc_0);
    (*proc_0).last_used = log_monotonic_secs;
    if debug != 0 {
        log_error(
            (*r).conf.errh,
            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
            274 as libc::c_int as libc::c_uint,
            b"got proc: pid: %d socket: %s load: %d\0" as *const u8
                as *const libc::c_char,
            (*proc_0).pid,
            (*(*proc_0).connection_name).ptr,
            (*proc_0).load,
        );
    }
}
#[cold]
unsafe extern "C" fn gw_proc_connect_error(
    r: *mut request_st,
    mut host: *mut gw_host,
    mut proc_0: *mut gw_proc,
    mut pid: pid_t,
    mut errnum: libc::c_int,
    mut debug: libc::c_int,
) {
    let cur_ts: unix_time64_t = log_monotonic_secs;
    let errh: *mut log_error_st = (*r).conf.errh;
    *__errno_location() = errnum;
    log_perror(
        errh,
        b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
        285 as libc::c_int as libc::c_uint,
        b"establishing connection failed: socket: %s\0" as *const u8
            as *const libc::c_char,
        (*(*proc_0).connection_name).ptr,
    );
    if (*proc_0).is_local == 0 {
        (*proc_0).disabled_until = cur_ts + (*host).disable_time as libc::c_long;
        gw_proc_set_state(host, proc_0, PROC_STATE_OVERLOADED as libc::c_int);
    } else if (*proc_0).pid == pid
            && (*proc_0).state as libc::c_uint
                == PROC_STATE_RUNNING as libc::c_int as libc::c_uint
        {
        log_error(
            errh,
            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
            301 as libc::c_int as libc::c_uint,
            b"backend error; we'll disable for %dsecs and send the request to another backend instead:load: %d\0"
                as *const u8 as *const libc::c_char,
            (*host).disable_time as libc::c_int,
            (*host).load,
        );
        if 11 as libc::c_int == errnum {
            log_error(
                errh,
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                308 as libc::c_int as libc::c_uint,
                b"If this happened on Linux: You have run out of local ports. Check the manual, section Performance how to handle this.\0"
                    as *const u8 as *const libc::c_char,
            );
            if debug != 0 {
                log_error(
                    errh,
                    b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                    313 as libc::c_int as libc::c_uint,
                    b"This means that you have more incoming requests than your FastCGI backend can handle in parallel.  It might help to spawn more FastCGI backends or PHP children; if not, decrease server.max-connections.  The load for this FastCGI backend %s is %d\0"
                        as *const u8 as *const libc::c_char,
                    (*(*proc_0).connection_name).ptr,
                    (*proc_0).load,
                );
            }
            (*proc_0).disabled_until = cur_ts + (*host).disable_time as libc::c_long;
            gw_proc_set_state(host, proc_0, PROC_STATE_OVERLOADED as libc::c_int);
        } else {
            (*proc_0).disabled_until = cur_ts + (*host).disable_time as libc::c_long;
            gw_proc_set_state(host, proc_0, PROC_STATE_OVERLOADED as libc::c_int);
        }
    }
    if 11 as libc::c_int == errnum {
        gw_proc_tag_inc(
            host,
            proc_0,
            b".overloaded\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    } else {
        gw_proc_tag_inc(
            host,
            proc_0,
            b".died\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    };
}
unsafe extern "C" fn gw_proc_release(
    mut host: *mut gw_host,
    mut proc_0: *mut gw_proc,
    mut debug: libc::c_int,
    mut errh: *mut log_error_st,
) {
    gw_proc_load_dec(host, proc_0);
    if debug != 0 {
        log_error(
            errh,
            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
            349 as libc::c_int as libc::c_uint,
            b"released proc: pid: %d socket: %s load: %u\0" as *const u8
                as *const libc::c_char,
            (*proc_0).pid,
            (*(*proc_0).connection_name).ptr,
            (*proc_0).load,
        );
    }
}
#[cold]
unsafe extern "C" fn gw_proc_check_enable(
    host: *mut gw_host,
    proc_0: *mut gw_proc,
    errh: *mut log_error_st,
) {
    if log_monotonic_secs <= (*proc_0).disabled_until {
        return;
    }
    if (*proc_0).state as libc::c_uint
        != PROC_STATE_OVERLOADED as libc::c_int as libc::c_uint
    {
        return;
    }
    gw_proc_set_state(host, proc_0, PROC_STATE_RUNNING as libc::c_int);
    log_error(
        errh,
        b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
        362 as libc::c_int as libc::c_uint,
        b"gw-server re-enabled: %s %s %hu %s\0" as *const u8 as *const libc::c_char,
        (*(*proc_0).connection_name).ptr,
        if !((*host).host).is_null() {
            (*(*host).host).ptr as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        (*host).port as libc::c_int,
        if !((*host).unixsocket).is_null() {
            (*(*host).unixsocket).ptr as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
}
#[cold]
unsafe extern "C" fn gw_proc_waitpid_log(
    host: *const gw_host,
    proc_0: *const gw_proc,
    errh: *mut log_error_st,
    status: libc::c_int,
) {
    if status & 0x7f as libc::c_int == 0 as libc::c_int {
        if (*proc_0).state as libc::c_uint
            != PROC_STATE_KILLED as libc::c_int as libc::c_uint
        {
            log_error(
                errh,
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                373 as libc::c_int as libc::c_uint,
                b"child exited: %d %s\0" as *const u8 as *const libc::c_char,
                (status & 0xff00 as libc::c_int) >> 8 as libc::c_int,
                (*(*proc_0).connection_name).ptr,
            );
        }
    } else if ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
            as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
        {
        if status & 0x7f as libc::c_int != 15 as libc::c_int
            && status & 0x7f as libc::c_int != 2 as libc::c_int
            && status & 0x7f as libc::c_int != (*host).kill_signal as libc::c_int
        {
            log_error(
                errh,
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                380 as libc::c_int as libc::c_uint,
                b"child signalled: %d\0" as *const u8 as *const libc::c_char,
                status & 0x7f as libc::c_int,
            );
        }
    } else {
        log_error(
            errh,
            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int as libc::c_uint,
            b"child died somehow: %d\0" as *const u8 as *const libc::c_char,
            status,
        );
    };
}
unsafe extern "C" fn gw_proc_waitpid(
    mut host: *mut gw_host,
    mut proc_0: *mut gw_proc,
    mut errh: *mut log_error_st,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    if (*proc_0).is_local == 0 {
        return 0 as libc::c_int;
    }
    if (*proc_0).pid <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    rc = fdevent_waitpid((*proc_0).pid, &mut status, 1 as libc::c_int);
    if 0 as libc::c_int == rc {
        return 0 as libc::c_int;
    }
    if -(1 as libc::c_int) == rc {
        log_perror(
            errh,
            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
            402 as libc::c_int as libc::c_uint,
            b"pid %d %d not found\0" as *const u8 as *const libc::c_char,
            (*proc_0).pid,
            (*proc_0).state as libc::c_uint,
        );
    } else {
        gw_proc_waitpid_log(host, proc_0, errh, status);
    }
    (*proc_0).pid = 0 as libc::c_int;
    if (*proc_0).state as libc::c_uint
        != PROC_STATE_KILLED as libc::c_int as libc::c_uint
    {
        (*proc_0).disabled_until = log_monotonic_secs;
    }
    gw_proc_set_state(host, proc_0, PROC_STATE_DIED as libc::c_int);
    return 1 as libc::c_int;
}
#[cold]
unsafe extern "C" fn gw_proc_sockaddr_init(
    host: *mut gw_host,
    proc_0: *mut gw_proc,
    errh: *mut log_error_st,
) -> libc::c_int {
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
    let mut addrlen: socklen_t = 0;
    if !((*proc_0).unixsocket).is_null() {
        if 1 as libc::c_int
            != sock_addr_from_str_hints(
                &mut addr,
                &mut addrlen,
                (*(*proc_0).unixsocket).ptr,
                1 as libc::c_int,
                0 as libc::c_int as libc::c_ushort,
                errh,
            )
        {
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int);
        }
        buffer_clear((*proc_0).connection_name);
        buffer_append_str2(
            (*proc_0).connection_name,
            b"unix:\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            (*(*proc_0).unixsocket).ptr,
            buffer_clen((*proc_0).unixsocket) as size_t,
        );
    } else {
        if 1 as libc::c_int
            != sock_addr_from_str_hints(
                &mut addr,
                &mut addrlen,
                (*(*host).host).ptr,
                0 as libc::c_int,
                (*proc_0).port,
                errh,
            )
        {
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int);
        } else {
            if (*(*host).host).size != 0 {
                let mut h: *mut buffer = 0 as *mut buffer;
                let ref mut fresh7 = *(&mut h as *mut *mut buffer as *mut *const buffer);
                *fresh7 = (*host).host;
                sock_addr_inet_ntop_copy_buffer(h, &mut addr);
                (*host).family = sock_addr_get_family(&mut addr) as libc::c_ushort;
            }
        }
        buffer_clear((*proc_0).connection_name);
        buffer_append_str3(
            (*proc_0).connection_name,
            b"tcp:\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            (*(*host).host).ptr,
            buffer_clen((*host).host) as size_t,
            b":\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        buffer_append_int((*proc_0).connection_name, (*proc_0).port as intmax_t);
    }
    if !((*proc_0).saddr).is_null() && (*proc_0).saddrlen < addrlen {
        free((*proc_0).saddr as *mut libc::c_void);
        (*proc_0).saddr = 0 as *mut sockaddr;
    }
    if ((*proc_0).saddr).is_null() {
        (*proc_0).saddr = malloc(addrlen as libc::c_ulong) as *mut sockaddr;
        if ((*proc_0).saddr).is_null() {
            ck_assert_failed(
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                465 as libc::c_int as libc::c_uint,
                b"proc->saddr\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    (*proc_0).saddrlen = addrlen;
    memcpy(
        (*proc_0).saddr as *mut libc::c_void,
        &mut addr as *mut sock_addr as *const libc::c_void,
        addrlen as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn env_add(
    mut env: *mut char_array,
    mut key: *const libc::c_char,
    mut key_len: size_t,
    mut val: *const libc::c_char,
    mut val_len: size_t,
) -> libc::c_int {
    let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
    if key.is_null() || val.is_null() {
        return -(1 as libc::c_int);
    }
    dst = malloc(
        key_len.wrapping_add(val_len).wrapping_add(3 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if dst.is_null() {
        ck_assert_failed(
            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
            478 as libc::c_int as libc::c_uint,
            b"dst\0" as *const u8 as *const libc::c_char,
        );
    }
    memcpy(dst as *mut libc::c_void, key as *const libc::c_void, key_len);
    *dst.offset(key_len as isize) = '=' as i32 as libc::c_char;
    memcpy(
        dst.offset(key_len as isize).offset(1 as libc::c_int as isize)
            as *mut libc::c_void,
        val as *const libc::c_void,
        val_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*env).used {
        if 0 as libc::c_int
            == strncmp(
                dst,
                *((*env).ptr).offset(i as isize),
                key_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            )
        {
            free(*((*env).ptr).offset(i as isize) as *mut libc::c_void);
            let ref mut fresh8 = *((*env).ptr).offset(i as isize);
            *fresh8 = dst;
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    if (*env).size <= ((*env).used).wrapping_add(1 as libc::c_int as libc::c_uint) {
        (*env)
            .size = ((*env).size as libc::c_uint)
            .wrapping_add(16 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
        (*env)
            .ptr = realloc(
            (*env).ptr as *mut libc::c_void,
            ((*env).size as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
        if ((*env).ptr).is_null() {
            ck_assert_failed(
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                497 as libc::c_int as libc::c_uint,
                b"env->ptr\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    let fresh9 = (*env).used;
    (*env).used = ((*env).used).wrapping_add(1);
    let ref mut fresh10 = *((*env).ptr).offset(fresh9 as isize);
    *fresh10 = dst;
    return 0 as libc::c_int;
}
#[cold]
unsafe extern "C" fn gw_spawn_connection(
    host: *mut gw_host,
    proc_0: *mut gw_proc,
    errh: *mut log_error_st,
    mut debug: libc::c_int,
) -> libc::c_int {
    let mut gw_fd: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    if debug != 0 {
        log_error(
            errh,
            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
            514 as libc::c_int as libc::c_uint,
            b"new proc, socket: %hu %s\0" as *const u8 as *const libc::c_char,
            (*proc_0).port as libc::c_int,
            if !((*proc_0).unixsocket).is_null() {
                (*(*proc_0).unixsocket).ptr as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    }
    gw_fd = fdevent_socket_cloexec(
        (*(*proc_0).saddr).sa_family as libc::c_int,
        SOCK_STREAM as libc::c_int,
        0 as libc::c_int,
    );
    if -(1 as libc::c_int) == gw_fd {
        log_perror(
            errh,
            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
            521 as libc::c_int as libc::c_uint,
            b"socket()\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    loop {
        status = connect(
            gw_fd,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: (*proc_0).saddr,
            },
            (*proc_0).saddrlen,
        );
        if !(-(1 as libc::c_int) == status && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    if -(1 as libc::c_int) == status && *__errno_location() != 2 as libc::c_int
        && !((*proc_0).unixsocket).is_null()
    {
        log_perror(
            errh,
            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
            530 as libc::c_int as libc::c_uint,
            b"connect %s\0" as *const u8 as *const libc::c_char,
            (*(*proc_0).unixsocket).ptr,
        );
        unlink((*(*proc_0).unixsocket).ptr);
    }
    close(gw_fd);
    if -(1 as libc::c_int) == status {
        let mut env: char_array = char_array {
            ptr: 0 as *mut *mut libc::c_char,
            size: 0,
            used: 0,
        };
        let mut i: uint32_t = 0;
        gw_fd = fdevent_socket_cloexec(
            (*(*proc_0).saddr).sa_family as libc::c_int,
            SOCK_STREAM as libc::c_int,
            0 as libc::c_int,
        );
        if -(1 as libc::c_int) == gw_fd {
            log_perror(
                errh,
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                545 as libc::c_int as libc::c_uint,
                b"socket()\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if fdevent_set_so_reuseaddr(gw_fd, 1 as libc::c_int) < 0 as libc::c_int {
            log_perror(
                errh,
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                550 as libc::c_int as libc::c_uint,
                b"socketsockopt()\0" as *const u8 as *const libc::c_char,
            );
            close(gw_fd);
            return -(1 as libc::c_int);
        }
        if -(1 as libc::c_int)
            == bind(
                gw_fd,
                __CONST_SOCKADDR_ARG {
                    __sockaddr__: (*proc_0).saddr,
                },
                (*proc_0).saddrlen,
            )
        {
            log_perror(
                errh,
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                557 as libc::c_int as libc::c_uint,
                b"bind failed for: %s\0" as *const u8 as *const libc::c_char,
                (*(*proc_0).connection_name).ptr,
            );
            close(gw_fd);
            return -(1 as libc::c_int);
        }
        if -(1 as libc::c_int) == listen(gw_fd, (*host).listen_backlog) {
            log_perror(
                errh,
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                564 as libc::c_int as libc::c_uint,
                b"listen()\0" as *const u8 as *const libc::c_char,
            );
            close(gw_fd);
            return -(1 as libc::c_int);
        }
        env.ptr = 0 as *mut *mut libc::c_char;
        env.size = 0 as libc::c_int as uint32_t;
        env.used = 0 as libc::c_int as uint32_t;
        if !((*host).bin_env_copy).is_null() && (*(*host).bin_env_copy).used != 0 {
            i = 0 as libc::c_int as uint32_t;
            while i < (*(*host).bin_env_copy).used {
                let mut ds: *mut data_string = *((*(*host).bin_env_copy).data)
                    .offset(i as isize) as *mut data_string;
                let mut ge: *mut libc::c_char = 0 as *mut libc::c_char;
                ge = getenv((*ds).value.ptr);
                if !ge.is_null() {
                    env_add(
                        &mut env,
                        (*ds).value.ptr,
                        buffer_clen(&mut (*ds).value) as size_t,
                        ge,
                        strlen(ge),
                    );
                }
                i = i.wrapping_add(1);
            }
        } else {
            let e: *mut *mut libc::c_char = fdevent_environ();
            i = 0 as libc::c_int as uint32_t;
            while !(*e.offset(i as isize)).is_null() {
                let mut eq: *mut libc::c_char = 0 as *mut libc::c_char;
                eq = strchr(*e.offset(i as isize), '=' as i32);
                if !eq.is_null() {
                    env_add(
                        &mut env,
                        *e.offset(i as isize),
                        eq.offset_from(*e.offset(i as isize)) as libc::c_long as size_t,
                        eq.offset(1 as libc::c_int as isize),
                        strlen(eq.offset(1 as libc::c_int as isize)),
                    );
                }
                i = i.wrapping_add(1);
            }
        }
        if !((*host).bin_env).is_null() {
            i = 0 as libc::c_int as uint32_t;
            while i < (*(*host).bin_env).used {
                let mut ds_0: *mut data_string = *((*(*host).bin_env).data)
                    .offset(i as isize) as *mut data_string;
                env_add(
                    &mut env,
                    (*ds_0).key.ptr,
                    buffer_clen(&mut (*ds_0).key) as size_t,
                    (*ds_0).value.ptr,
                    buffer_clen(&mut (*ds_0).value) as size_t,
                );
                i = i.wrapping_add(1);
            }
        }
        i = 0 as libc::c_int as uint32_t;
        while i < env.used {
            if 0 as libc::c_int
                == strncmp(
                    *(env.ptr).offset(i as isize),
                    b"PHP_FCGI_CHILDREN=\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
            {
                break;
            }
            i = i.wrapping_add(1);
        }
        if i == env.used {
            env_add(
                &mut env,
                b"PHP_FCGI_CHILDREN\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                b"1\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
        let ref mut fresh11 = *(env.ptr).offset(env.used as isize);
        *fresh11 = 0 as *mut libc::c_char;
        let mut dfd: libc::c_int = fdevent_open_dirname(
            *((*host).args.ptr).offset(0 as libc::c_int as isize),
            1 as libc::c_int,
        );
        if -(1 as libc::c_int) == dfd {
            log_perror(
                errh,
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                624 as libc::c_int as libc::c_uint,
                b"open dirname failed: %s\0" as *const u8 as *const libc::c_char,
                *((*host).args.ptr).offset(0 as libc::c_int as isize),
            );
        }
        (*proc_0)
            .pid = if dfd >= 0 as libc::c_int {
            fdevent_fork_execve(
                *((*host).args.ptr).offset(0 as libc::c_int as isize),
                (*host).args.ptr,
                env.ptr,
                gw_fd,
                -(1 as libc::c_int),
                -(1 as libc::c_int),
                dfd,
            )
        } else {
            -(1 as libc::c_int)
        };
        i = 0 as libc::c_int as uint32_t;
        while i < env.used {
            free(*(env.ptr).offset(i as isize) as *mut libc::c_void);
            i = i.wrapping_add(1);
        }
        free(env.ptr as *mut libc::c_void);
        if -(1 as libc::c_int) != dfd {
            close(dfd);
        }
        close(gw_fd);
        if -(1 as libc::c_int) == (*proc_0).pid {
            log_error(
                errh,
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                640 as libc::c_int as libc::c_uint,
                b"gw-backend failed to start: %s\0" as *const u8 as *const libc::c_char,
                (*(*host).bin_path).ptr,
            );
            (*proc_0).pid = 0 as libc::c_int;
            (*proc_0).disabled_until = log_monotonic_secs;
            return -(1 as libc::c_int);
        }
        (*proc_0).last_used = log_monotonic_secs;
        (*proc_0).is_local = 1 as libc::c_int;
        let mut tv: timeval = {
            let mut init = timeval {
                tv_sec: 0 as libc::c_int as __time_t,
                tv_usec: 1000 as libc::c_int as __suseconds_t,
            };
            init
        };
        select(
            0 as libc::c_int,
            0 as *mut fd_set,
            0 as *mut fd_set,
            0 as *mut fd_set,
            &mut tv,
        );
        if 0 as libc::c_int != gw_proc_waitpid(host, proc_0, errh) {
            log_error(
                errh,
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                656 as libc::c_int as libc::c_uint,
                b"gw-backend failed to start: %s\0" as *const u8 as *const libc::c_char,
                (*(*host).bin_path).ptr,
            );
            log_error(
                errh,
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                658 as libc::c_int as libc::c_uint,
                b"If you're trying to run your app as a FastCGI backend, make sure you're using the FastCGI-enabled version.  If this is PHP on Gentoo, add 'fastcgi' to the USE flags.  If this is PHP, try removing the bytecode caches for now and try again.\0"
                    as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else {
        (*proc_0).is_local = 0 as libc::c_int;
        (*proc_0).pid = 0 as libc::c_int;
        if debug != 0 {
            log_error(
                errh,
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                670 as libc::c_int as libc::c_uint,
                b"(debug) socket is already used; won't spawn: %s\0" as *const u8
                    as *const libc::c_char,
                (*(*proc_0).connection_name).ptr,
            );
        }
    }
    gw_proc_set_state(host, proc_0, PROC_STATE_RUNNING as libc::c_int);
    return 0 as libc::c_int;
}
#[cold]
unsafe extern "C" fn gw_proc_spawn(
    host: *mut gw_host,
    errh: *mut log_error_st,
    debug: libc::c_int,
) {
    let mut proc_0: *mut gw_proc = 0 as *mut gw_proc;
    proc_0 = (*host).unused_procs;
    while !proc_0.is_null() {
        if (*proc_0).pid > 0 as libc::c_int {
            proc_0 = (*proc_0).next;
        } else {
            if (*proc_0).disabled_until >= log_monotonic_secs {
                return;
            }
            break;
        }
    }
    if !proc_0.is_null() {
        if proc_0 == (*host).unused_procs {
            (*host).unused_procs = (*proc_0).next;
        } else {
            (*(*proc_0).prev).next = (*proc_0).next;
        }
        if !((*proc_0).next).is_null() {
            (*(*proc_0).next).prev = (*proc_0).prev;
            (*proc_0).next = 0 as *mut gw_proc;
        }
        (*proc_0).prev = 0 as *mut gw_proc;
        gw_proc_init_portpath(host, proc_0);
    } else {
        proc_0 = gw_proc_init(host);
    }
    if 0 as libc::c_int != gw_proc_sockaddr_init(host, proc_0, errh) {
        log_error(
            errh,
            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
            710 as libc::c_int as libc::c_uint,
            b"ERROR: spawning backend failed.\0" as *const u8 as *const libc::c_char,
        );
        if (*proc_0).id
            == ((*host).max_id).wrapping_sub(1 as libc::c_int as libc::c_uint)
        {
            (*host).max_id = ((*host).max_id).wrapping_sub(1);
        }
        gw_proc_free(proc_0);
    } else if gw_spawn_connection(host, proc_0, errh, debug) != 0 {
        log_error(
            errh,
            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
            715 as libc::c_int as libc::c_uint,
            b"ERROR: spawning backend failed.\0" as *const u8 as *const libc::c_char,
        );
        (*proc_0).next = (*host).unused_procs;
        if !((*host).unused_procs).is_null() {
            (*(*host).unused_procs).prev = proc_0;
        }
        (*host).unused_procs = proc_0;
    } else {
        (*proc_0).next = (*host).first;
        if !((*host).first).is_null() {
            (*(*host).first).prev = proc_0;
        }
        (*host).first = proc_0;
        (*host).num_procs = ((*host).num_procs).wrapping_add(1);
    };
}
#[cold]
unsafe extern "C" fn gw_proc_kill(mut host: *mut gw_host, mut proc_0: *mut gw_proc) {
    if !((*proc_0).next).is_null() {
        (*(*proc_0).next).prev = (*proc_0).prev;
    }
    if !((*proc_0).prev).is_null() {
        (*(*proc_0).prev).next = (*proc_0).next;
    } else {
        (*host).first = (*proc_0).next;
    }
    (*host).num_procs = ((*host).num_procs).wrapping_sub(1);
    (*proc_0).prev = 0 as *mut gw_proc;
    (*proc_0).next = (*host).unused_procs;
    (*proc_0).disabled_until = 0 as libc::c_int as unix_time64_t;
    if !((*host).unused_procs).is_null() {
        (*(*host).unused_procs).prev = proc_0;
    }
    (*host).unused_procs = proc_0;
    kill((*proc_0).pid, (*host).kill_signal as libc::c_int);
    gw_proc_set_state(host, proc_0, PROC_STATE_KILLED as libc::c_int);
}
unsafe extern "C" fn unixsocket_is_dup(
    mut p: *mut gw_plugin_data,
    mut unixsocket: *const buffer,
) -> *mut gw_host {
    if ((*p).cvlist).is_null() {
        return 0 as *mut gw_host;
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
        let mut conf: *mut gw_plugin_config = 0 as *mut gw_plugin_config;
        while -(1 as libc::c_int) != (*cpv).k_id {
            match (*cpv).k_id {
                0 => {
                    if (*cpv).vtype as libc::c_uint
                        == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
                    {
                        conf = (*cpv).v.v as *mut gw_plugin_config;
                    }
                }
                _ => {}
            }
            cpv = cpv.offset(1);
        }
        if !(conf.is_null() || ((*conf).exts).is_null()) {
            let mut exts: *mut gw_exts = (*conf).exts;
            let mut j: uint32_t = 0 as libc::c_int as uint32_t;
            while j < (*exts).used {
                let mut ex: *mut gw_extension = ((*exts).exts).offset(j as isize);
                let mut n: uint32_t = 0 as libc::c_int as uint32_t;
                while n < (*ex).used {
                    let mut host: *mut gw_host = *((*ex).hosts).offset(n as isize);
                    if !((*host).unixsocket).is_null()
                        && buffer_is_equal((*host).unixsocket, unixsocket) != 0
                        && !((*host).bin_path).is_null()
                    {
                        return host;
                    }
                    n = n.wrapping_add(1);
                }
                j = j.wrapping_add(1);
            }
        }
        i += 1;
    }
    return 0 as *mut gw_host;
}
unsafe extern "C" fn parse_binpath(
    mut env: *mut char_array,
    mut b: *const buffer,
) -> libc::c_int {
    let mut start: *mut libc::c_char = (*b).ptr;
    let mut c: libc::c_char = 0;
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut used: size_t = buffer_clen(b) as size_t;
    while i < used {
        match *((*b).ptr).offset(i as isize) as libc::c_int {
            32 | 9 => {
                if (*env).size == (*env).used {
                    (*env)
                        .size = ((*env).size as libc::c_uint)
                        .wrapping_add(16 as libc::c_int as libc::c_uint) as uint32_t
                        as uint32_t;
                    (*env)
                        .ptr = realloc(
                        (*env).ptr as *mut libc::c_void,
                        ((*env).size as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                            ),
                    ) as *mut *mut libc::c_char;
                    if ((*env).ptr).is_null() {
                        ck_assert_failed(
                            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                            798 as libc::c_int as libc::c_uint,
                            b"env->ptr\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
                c = *((*b).ptr).offset(i as isize);
                *((*b).ptr).offset(i as isize) = '\u{0}' as i32 as libc::c_char;
                let fresh12 = (*env).used;
                (*env).used = ((*env).used).wrapping_add(1);
                let ref mut fresh13 = *((*env).ptr).offset(fresh12 as isize);
                *fresh13 = strdup(start);
                *((*b).ptr).offset(i as isize) = c;
                start = ((*b).ptr).offset(i as isize).offset(1 as libc::c_int as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
    if (*env).size == (*env).used {
        (*env)
            .size = ((*env).size as libc::c_uint)
            .wrapping_add(16 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
        (*env)
            .ptr = realloc(
            (*env).ptr as *mut libc::c_void,
            ((*env).size as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
    }
    let fresh14 = (*env).used;
    (*env).used = ((*env).used).wrapping_add(1);
    let ref mut fresh15 = *((*env).ptr).offset(fresh14 as isize);
    *fresh15 = strdup(start);
    if (*env).size == (*env).used {
        (*env)
            .size = ((*env).size as libc::c_uint)
            .wrapping_add(16 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
        (*env)
            .ptr = realloc(
            (*env).ptr as *mut libc::c_void,
            ((*env).size as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
    }
    let fresh16 = (*env).used;
    (*env).used = ((*env).used).wrapping_add(1);
    let ref mut fresh17 = *((*env).ptr).offset(fresh16 as isize);
    *fresh17 = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
#[inline(never)]
unsafe extern "C" fn gw_hash(
    mut str: *const libc::c_char,
    len: uint32_t,
    mut hash: uint32_t,
) -> uint32_t {
    return djbhash(str, len, hash);
}
unsafe extern "C" fn gw_host_get(
    r: *mut request_st,
    mut extension: *mut gw_extension,
    mut balance: libc::c_int,
    mut debug: libc::c_int,
) -> *mut gw_host {
    let mut ndx: libc::c_int = -(1 as libc::c_int);
    let ext_used: libc::c_int = (*extension).used as libc::c_int;
    if ext_used <= 1 as libc::c_int {
        if 1 as libc::c_int == ext_used
            && (**((*extension).hosts).offset(0 as libc::c_int as isize)).active_procs
                > 0 as libc::c_int as libc::c_uint
        {
            ndx = 0 as libc::c_int;
        }
    } else {
        match balance {
            2 => {
                let base_hash: uint32_t = gw_hash(
                    (*r).uri.authority.ptr,
                    buffer_clen(&mut (*r).uri.authority),
                    gw_hash(
                        (*r).uri.path.ptr,
                        buffer_clen(&mut (*r).uri.path),
                        5381 as libc::c_int as uint32_t,
                    ),
                );
                let mut last_max: uint32_t = 4294967295 as libc::c_uint;
                let mut k: libc::c_int = 0 as libc::c_int;
                while k < ext_used {
                    let host: *const gw_host = *((*extension).hosts).offset(k as isize);
                    if !(0 as libc::c_int as libc::c_uint == (*host).active_procs) {
                        let cur_max: uint32_t = base_hash ^ (*host).gw_hash;
                        if last_max < cur_max || last_max == 4294967295 as libc::c_uint {
                            last_max = cur_max;
                            ndx = k;
                        }
                    }
                    k += 1;
                }
            }
            0 => {
                let mut k_0: libc::c_int = 0 as libc::c_int;
                let mut max_usage: libc::c_int = 2147483647 as libc::c_int;
                while k_0 < ext_used {
                    let host_0: *const gw_host = *((*extension).hosts)
                        .offset(k_0 as isize);
                    if !(0 as libc::c_int as libc::c_uint == (*host_0).active_procs) {
                        if (*host_0).load < max_usage {
                            max_usage = (*host_0).load;
                            ndx = k_0;
                        }
                    }
                    k_0 += 1;
                }
            }
            1 => {
                let mut host_1: *const gw_host = *((*extension).hosts)
                    .offset(0 as libc::c_int as isize);
                let mut k_1: libc::c_int = (*extension).last_used_ndx;
                ndx = k_1 + 1 as libc::c_int;
                if ndx < 0 as libc::c_int {
                    ndx = 0 as libc::c_int;
                }
                while ndx < ext_used
                    && {
                        host_1 = *((*extension).hosts).offset(ndx as isize);
                        0 as libc::c_int as libc::c_uint == (*host_1).active_procs
                    }
                {
                    ndx += 1;
                }
                if ndx >= ext_used {
                    ndx = 0 as libc::c_int;
                    while ndx <= k_1 {
                        host_1 = *((*extension).hosts).offset(ndx as isize);
                        if 0 as libc::c_int as libc::c_uint != (*host_1).active_procs {
                            break;
                        }
                        ndx += 1;
                    }
                    if 0 as libc::c_int as libc::c_uint == (*host_1).active_procs {
                        ndx = -(1 as libc::c_int);
                    }
                }
                (*extension).last_used_ndx = ndx;
            }
            3 => {
                let dst_addr_buf: *const buffer = &mut (*(*r).con).dst_addr_buf;
                let base_hash_0: uint32_t = gw_hash(
                    (*dst_addr_buf).ptr,
                    buffer_clen(dst_addr_buf),
                    5381 as libc::c_int as uint32_t,
                );
                let mut last_max_0: uint32_t = 4294967295 as libc::c_uint;
                let mut k_2: libc::c_int = 0 as libc::c_int;
                while k_2 < ext_used {
                    let host_2: *const gw_host = *((*extension).hosts)
                        .offset(k_2 as isize);
                    if !(0 as libc::c_int as libc::c_uint == (*host_2).active_procs) {
                        let cur_max_0: uint32_t = base_hash_0 ^ (*host_2).gw_hash
                            ^ (*host_2).port as libc::c_uint;
                        if last_max_0 < cur_max_0
                            || last_max_0 == 4294967295 as libc::c_uint
                        {
                            last_max_0 = cur_max_0;
                            ndx = k_2;
                        }
                    }
                    k_2 += 1;
                }
            }
            _ => {}
        }
    }
    if (-(1 as libc::c_int) != ndx) as libc::c_int as libc::c_long != 0 {
        if debug != 0 {
            let host_3: *mut gw_host = *((*extension).hosts).offset(ndx as isize);
            log_error(
                (*r).conf.errh,
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                969 as libc::c_int as libc::c_uint,
                b"gw - found a host %s %hu\0" as *const u8 as *const libc::c_char,
                if !((*host_3).host).is_null() {
                    (*(*host_3).host).ptr as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                (*host_3).port as libc::c_int,
            );
            return host_3;
        }
        return *((*extension).hosts).offset(ndx as isize);
    } else {
        if 0 as libc::c_int == (*(*(*r).con).srv).srvconf.max_worker as libc::c_int {
            let mut k_3: libc::c_int = 0 as libc::c_int;
            while k_3 < ext_used {
                let host_4: *mut gw_host = *((*extension).hosts).offset(k_3 as isize);
                if 0 as libc::c_int == (*host_4).min_procs as libc::c_int
                    && 0 as libc::c_int as libc::c_uint == (*host_4).num_procs
                    && !((*host_4).bin_path).is_null()
                {
                    gw_proc_spawn(host_4, (*(*(*r).con).srv).errh, debug);
                    if (*host_4).num_procs != 0 {
                        return host_4;
                    }
                }
                k_3 += 1;
            }
        }
    }
    (*r).http_status = 503 as libc::c_int;
    (*r).handler_module = 0 as *const plugin;
    if (*extension).note_is_sent == 0 {
        (*extension).note_is_sent = 1 as libc::c_int;
        log_error(
            (*r).conf.errh,
            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
            995 as libc::c_int as libc::c_uint,
            b"all handlers for %s?%.*s on %s are down.\0" as *const u8
                as *const libc::c_char,
            (*r).uri.path.ptr,
            buffer_clen(&mut (*r).uri.query) as libc::c_int,
            (*r).uri.query.ptr,
            (*extension).key.ptr,
        );
    }
    return 0 as *mut gw_host;
}
unsafe extern "C" fn gw_establish_connection(
    r: *mut request_st,
    mut host: *mut gw_host,
    mut proc_0: *mut gw_proc,
    mut pid: pid_t,
    mut gw_fd: libc::c_int,
    mut debug: libc::c_int,
) -> libc::c_int {
    if -(1 as libc::c_int)
        == connect(
            gw_fd,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: (*proc_0).saddr,
            },
            (*proc_0).saddrlen,
        )
    {
        let mut errnum: libc::c_int = *__errno_location();
        if errnum == 115 as libc::c_int || errnum == 114 as libc::c_int
            || errnum == 4 as libc::c_int
            || errnum == 11 as libc::c_int && !((*host).unixsocket).is_null()
        {
            if debug > 2 as libc::c_int {
                log_error(
                    (*r).conf.errh,
                    b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                    1010 as libc::c_int as libc::c_uint,
                    b"connect delayed; will continue later: %s\0" as *const u8
                        as *const libc::c_char,
                    (*(*proc_0).connection_name).ptr,
                );
            }
            return 1 as libc::c_int;
        } else {
            gw_proc_connect_error(r, host, proc_0, pid, errnum, debug);
            return -(1 as libc::c_int);
        }
    }
    if debug > 1 as libc::c_int {
        log_error(
            (*r).conf.errh,
            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
            1023 as libc::c_int as libc::c_uint,
            b"connect succeeded: %d\0" as *const u8 as *const libc::c_char,
            gw_fd,
        );
    }
    return 0 as libc::c_int;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn gw_restart_dead_proc(
    host: *mut gw_host,
    errh: *mut log_error_st,
    debug: libc::c_int,
    trigger: libc::c_int,
    proc_0: *mut gw_proc,
) {
    let mut current_block_15: u64;
    match (*proc_0).state as libc::c_uint {
        1 => {
            gw_proc_check_enable(host, proc_0, errh);
            current_block_15 = 2719512138335094285;
        }
        4 => {
            if trigger != 0
                && {
                    (*proc_0).disabled_until += 1;
                    (*proc_0).disabled_until > 4 as libc::c_int as libc::c_long
                }
            {
                let mut sig: libc::c_int = if (*proc_0).disabled_until
                    <= 8 as libc::c_int as libc::c_long
                {
                    (*host).kill_signal as libc::c_int
                } else if (*proc_0).disabled_until <= 16 as libc::c_int as libc::c_long {
                    15 as libc::c_int
                } else {
                    9 as libc::c_int
                };
                kill((*proc_0).pid, sig);
            }
            current_block_15 = 2719512138335094285;
        }
        2 => {
            if 0 as libc::c_int == gw_proc_waitpid(host, proc_0, errh) {
                gw_proc_check_enable(host, proc_0, errh);
            }
            if (*proc_0).state as libc::c_uint
                != PROC_STATE_DIED as libc::c_int as libc::c_uint
            {
                current_block_15 = 2719512138335094285;
            } else {
                current_block_15 = 5587327783247221013;
            }
        }
        3 => {
            current_block_15 = 5587327783247221013;
        }
        0 | _ => {
            current_block_15 = 2719512138335094285;
        }
    }
    match current_block_15 {
        5587327783247221013 => {
            if !((*host).bin_path).is_null() {
                if !((*proc_0).load != 0 as libc::c_int as libc::c_uint) {
                    if !((*proc_0).disabled_until >= log_monotonic_secs) {
                        if debug != 0 {
                            log_error(
                                errh,
                                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                                1075 as libc::c_int as libc::c_uint,
                                b"--- gw spawning\n\tsocket %s\n\tcurrent: 1 / %u\0"
                                    as *const u8 as *const libc::c_char,
                                (*(*proc_0).connection_name).ptr,
                                (*host).max_procs as libc::c_int,
                            );
                        }
                        if gw_spawn_connection(host, proc_0, errh, debug) != 0 {
                            log_error(
                                errh,
                                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                                1083 as libc::c_int as libc::c_uint,
                                b"ERROR: spawning gw failed.\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                    }
                }
            } else {
                gw_proc_check_enable(host, proc_0, errh);
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn gw_restart_dead_procs(
    host: *mut gw_host,
    errh: *mut log_error_st,
    debug: libc::c_int,
    trigger: libc::c_int,
) {
    let mut proc_0: *mut gw_proc = (*host).first;
    while !proc_0.is_null() {
        if debug > 2 as libc::c_int {
            log_error(
                errh,
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                1096 as libc::c_int as libc::c_uint,
                b"proc: %s %d %d %d %d\0" as *const u8 as *const libc::c_char,
                (*(*proc_0).connection_name).ptr,
                (*proc_0).state as libc::c_uint,
                (*proc_0).is_local,
                (*proc_0).load,
                (*proc_0).pid,
            );
        }
        if (*proc_0).state as libc::c_uint
            != PROC_STATE_RUNNING as libc::c_int as libc::c_uint
        {
            gw_restart_dead_proc(host, errh, debug, trigger, proc_0);
        }
        proc_0 = (*proc_0).next;
    }
}
unsafe extern "C" fn handler_ctx_init(mut sz: size_t) -> *mut gw_handler_ctx {
    let mut hctx: *mut gw_handler_ctx = calloc(
        1 as libc::c_int as libc::c_ulong,
        if 0 as libc::c_int as libc::c_ulong == sz {
            ::std::mem::size_of::<gw_handler_ctx>() as libc::c_ulong
        } else {
            sz
        },
    ) as *mut gw_handler_ctx;
    if hctx.is_null() {
        ck_assert_failed(
            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
            1120 as libc::c_int as libc::c_uint,
            b"hctx\0" as *const u8 as *const libc::c_char,
        );
    }
    (*hctx).request_id = 0 as libc::c_int;
    (*hctx).gw_mode = 1 as libc::c_int as libc::c_ushort;
    (*hctx).state = GW_STATE_INIT;
    (*hctx).proc_0 = 0 as *mut gw_proc;
    (*hctx).fd = -(1 as libc::c_int);
    (*hctx).reconnects = 0 as libc::c_int;
    (*hctx).send_content_body = 1 as libc::c_int;
    chunkqueue_init(&mut (*hctx).wb);
    (*hctx).wb_reqlen = 0 as libc::c_int as off_t;
    return hctx;
}
unsafe extern "C" fn handler_ctx_free(mut hctx: *mut gw_handler_ctx) {
    if ((*hctx).handler_ctx_free).is_some() {
        ((*hctx).handler_ctx_free)
            .expect("non-null function pointer")(hctx as *mut libc::c_void);
    }
    chunk_buffer_release((*hctx).response);
    if !((*hctx).rb).is_null() {
        chunkqueue_free((*hctx).rb);
    }
    chunkqueue_reset(&mut (*hctx).wb);
    free(hctx as *mut libc::c_void);
}
unsafe extern "C" fn handler_ctx_clear(mut hctx: *mut gw_handler_ctx) {
    (*hctx).proc_0 = 0 as *mut gw_proc;
    (*hctx).host = 0 as *mut gw_host;
    (*hctx).ext = 0 as *mut gw_extension;
    (*hctx).gw_mode = 1 as libc::c_int as libc::c_ushort;
    (*hctx).state = GW_STATE_INIT;
    if !((*hctx).rb).is_null() {
        chunkqueue_reset((*hctx).rb);
    }
    chunkqueue_reset(&mut (*hctx).wb);
    (*hctx).wb_reqlen = 0 as libc::c_int as off_t;
    if !((*hctx).response).is_null() {
        buffer_clear((*hctx).response);
    }
    (*hctx).fd = -(1 as libc::c_int);
    (*hctx).reconnects = 0 as libc::c_int;
    (*hctx).request_id = 0 as libc::c_int;
    (*hctx).send_content_body = 1 as libc::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn gw_init() -> *mut libc::c_void {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gw_plugin_data>() as libc::c_ulong,
    );
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn gw_plugin_config_free(mut s: *mut gw_plugin_config) {
    let mut exts: *mut gw_exts = (*s).exts;
    if !exts.is_null() {
        let mut j: uint32_t = 0 as libc::c_int as uint32_t;
        while j < (*exts).used {
            let mut ex: *mut gw_extension = ((*exts).exts).offset(j as isize);
            let mut n: uint32_t = 0 as libc::c_int as uint32_t;
            while n < (*ex).used {
                let mut proc_0: *mut gw_proc = 0 as *mut gw_proc;
                let mut host: *mut gw_host = *((*ex).hosts).offset(n as isize);
                proc_0 = (*host).first;
                while !proc_0.is_null() {
                    if (*proc_0).pid > 0 as libc::c_int {
                        kill((*proc_0).pid, (*host).kill_signal as libc::c_int);
                    }
                    if (*proc_0).is_local != 0 && !((*proc_0).unixsocket).is_null() {
                        unlink((*(*proc_0).unixsocket).ptr);
                    }
                    proc_0 = (*proc_0).next;
                }
                proc_0 = (*host).unused_procs;
                while !proc_0.is_null() {
                    if (*proc_0).pid > 0 as libc::c_int {
                        kill((*proc_0).pid, (*host).kill_signal as libc::c_int);
                    }
                    if (*proc_0).is_local != 0 && !((*proc_0).unixsocket).is_null() {
                        unlink((*(*proc_0).unixsocket).ptr);
                    }
                    proc_0 = (*proc_0).next;
                }
                n = n.wrapping_add(1);
            }
            j = j.wrapping_add(1);
        }
        gw_extensions_free((*s).exts);
        gw_extensions_free((*s).exts_auth);
        gw_extensions_free((*s).exts_resp);
    }
    free(s as *mut libc::c_void);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn gw_free(mut p_d: *mut libc::c_void) {
    let p: *mut gw_plugin_data = p_d as *mut gw_plugin_data;
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
                        gw_plugin_config_free((*cpv).v.v as *mut gw_plugin_config);
                    }
                }
                _ => {}
            }
            cpv = cpv.offset(1);
        }
        i += 1;
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn gw_exts_clear_check_local(mut exts: *mut gw_exts) {
    let mut j: uint32_t = 0 as libc::c_int as uint32_t;
    while j < (*exts).used {
        let mut ex: *mut gw_extension = ((*exts).exts).offset(j as isize);
        let mut n: uint32_t = 0 as libc::c_int as uint32_t;
        while n < (*ex).used {
            (**((*ex).hosts).offset(n as isize))
                .check_local = 0 as libc::c_int as libc::c_ushort;
            n = n.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
}
static mut cpk: [config_plugin_keys_t; 27] = [config_plugin_keys_t {
    k: 0 as *const libc::c_char,
    klen: 0,
    ktype: 0,
    scope: 0,
}; 27];
static mut lhost: buffer = buffer {
    ptr: 0 as *const libc::c_char as *mut libc::c_char,
    used: 0,
    size: 0,
};
#[no_mangle]
#[cold]
pub unsafe extern "C" fn gw_set_defaults_backend(
    mut srv: *mut server,
    mut p: *mut gw_plugin_data,
    mut a: *const array,
    mut s: *mut gw_plugin_config,
    mut sh_exec: libc::c_int,
    mut cpkkey: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut host: *mut gw_host = 0 as *mut gw_host;
    let mut graceful_restart_bg: libc::c_int = config_feature_bool(
        srv,
        b"server.graceful-restart-bg\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    (*p).srv_pid = (*srv).pid;
    (*s).exts = gw_extensions_init();
    (*s).exts_auth = gw_extensions_init();
    (*s).exts_resp = gw_extensions_init();
    let mut j: uint32_t = 0 as libc::c_int as uint32_t;
    's_28: loop {
        if !(j < (*a).used) {
            current_block = 9675306770437543583;
            break;
        }
        let mut da_ext: *mut data_array = *((*a).data).offset(j as isize)
            as *mut data_array;
        let mut n: uint32_t = 0 as libc::c_int as uint32_t;
        while n < (*da_ext).value.used {
            let da_host: *mut data_array = *((*da_ext).value.data).offset(n as isize)
                as *mut data_array;
            if (*da_host).type_0 as libc::c_uint
                != TYPE_ARRAY as libc::c_int as libc::c_uint
                || array_is_kvany(&mut (*da_host).value) == 0
            {
                log_error(
                    (*srv).errh,
                    b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                    1378 as libc::c_int as libc::c_uint,
                    b"unexpected value for gw.server near [%s](string); expected ( \"ext\" => ( \"backend-label\" => ( \"key\" => \"value\" )))\0"
                        as *const u8 as *const libc::c_char,
                    if !((*da_host).key.ptr).is_null() {
                        (*da_host).key.ptr as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                );
                current_block = 18182134051239909875;
                break 's_28;
            } else {
                let mut cvlist: [config_plugin_value_t; 28] = [config_plugin_value_t {
                    k_id: 0,
                    vtype: T_CONFIG_UNSET,
                    v: v_u { v: 0 as *mut libc::c_void },
                }; 28];
                memset(
                    cvlist.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<[config_plugin_value_t; 28]>() as libc::c_ulong,
                );
                let mut ca: *mut array = &mut (*da_host).value;
                if config_plugin_values_init_block(
                    srv,
                    ca,
                    cpk.as_ptr(),
                    cpkkey,
                    cvlist.as_mut_ptr(),
                ) == 0
                {
                    current_block = 18182134051239909875;
                    break 's_28;
                }
                let mut host_mode: libc::c_ushort = 1 as libc::c_int as libc::c_ushort;
                host = gw_host_init();
                (*host).id = &mut (*da_host).key;
                (*host).check_local = 1 as libc::c_int as libc::c_ushort;
                (*host).min_procs = 4 as libc::c_int as libc::c_ushort;
                (*host).max_procs = 4 as libc::c_int as libc::c_ushort;
                (*host).max_load_per_proc = 1 as libc::c_int as libc::c_ushort;
                (*host).idle_timeout = 60 as libc::c_int as libc::c_ushort;
                (*host).connect_timeout = 8 as libc::c_int as libc::c_ushort;
                (*host).disable_time = 1 as libc::c_int as libc::c_ushort;
                (*host)
                    .break_scriptfilename_for_php = 0 as libc::c_int as libc::c_ushort;
                (*host).kill_signal = 15 as libc::c_int as libc::c_ushort;
                (*host).fix_root_path_name = 0 as libc::c_int as libc::c_ushort;
                (*host).listen_backlog = 1024 as libc::c_int;
                (*host).xsendfile_allow = 0 as libc::c_int as libc::c_ushort;
                (*host).refcount = 0 as libc::c_int;
                let mut cpv: *mut config_plugin_value_t = cvlist.as_mut_ptr();
                while -(1 as libc::c_int) != (*cpv).k_id {
                    match (*cpv).k_id {
                        0 => {
                            if buffer_is_blank((*cpv).v.b) == 0 {
                                (*host).host = (*cpv).v.b;
                            }
                        }
                        1 => {
                            (*host).port = (*cpv).v.shrt;
                        }
                        2 => {
                            if buffer_is_blank((*cpv).v.b) == 0 {
                                (*host).unixsocket = (*cpv).v.b;
                            }
                        }
                        3 => {
                            (*host).listen_backlog = (*cpv).v.u as libc::c_int;
                        }
                        4 => {
                            if buffer_is_blank((*cpv).v.b) == 0 {
                                (*host).bin_path = (*cpv).v.b;
                            }
                        }
                        5 => {
                            (*host).kill_signal = (*cpv).v.shrt;
                        }
                        6 => {
                            (*host)
                                .check_local = (0 as libc::c_int as libc::c_uint
                                != (*cpv).v.u) as libc::c_int as libc::c_ushort;
                        }
                        7 => {
                            if buffer_is_blank((*cpv).v.b) == 0 {
                                let mut b: *const buffer = (*cpv).v.b;
                                if buffer_eq_slen(
                                    b,
                                    b"responder\0" as *const u8 as *const libc::c_char,
                                    (::std::mem::size_of::<[libc::c_char; 10]>()
                                        as libc::c_ulong as uint32_t)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                ) != 0
                                {
                                    host_mode = 1 as libc::c_int as libc::c_ushort;
                                } else if buffer_eq_slen(
                                        b,
                                        b"authorizer\0" as *const u8 as *const libc::c_char,
                                        (::std::mem::size_of::<[libc::c_char; 11]>()
                                            as libc::c_ulong as uint32_t)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                    ) != 0
                                    {
                                    host_mode = 2 as libc::c_int as libc::c_ushort;
                                } else {
                                    log_error(
                                        (*srv).errh,
                                        b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                                        1446 as libc::c_int as libc::c_uint,
                                        b"WARNING: unknown gw mode: %s (ignored, mode set to responder)\0"
                                            as *const u8 as *const libc::c_char,
                                        (*b).ptr,
                                    );
                                }
                            }
                        }
                        8 => {
                            if buffer_is_blank((*cpv).v.b) == 0 {
                                (*host).docroot = (*cpv).v.b;
                            }
                        }
                        9 => {
                            (*host).min_procs = (*cpv).v.shrt;
                        }
                        10 => {
                            (*host).max_procs = (*cpv).v.shrt;
                        }
                        11 => {
                            (*host).max_load_per_proc = (*cpv).v.shrt;
                        }
                        12 => {
                            (*host).idle_timeout = (*cpv).v.shrt;
                        }
                        13 => {
                            (*host).disable_time = (*cpv).v.shrt;
                        }
                        14 => {
                            (*host).bin_env = (*cpv).v.a;
                        }
                        15 => {
                            (*host).bin_env_copy = (*cpv).v.a;
                        }
                        16 => {
                            (*host)
                                .break_scriptfilename_for_php = (0 as libc::c_int
                                as libc::c_uint != (*cpv).v.u) as libc::c_int
                                as libc::c_ushort;
                        }
                        17 => {
                            (*host).strip_request_uri = (*cpv).v.b;
                            if buffer_has_slash_suffix((*host).strip_request_uri) != 0 {
                                let mut b_0: *mut buffer = 0 as *mut buffer;
                                let ref mut fresh18 = *(&mut b_0 as *mut *mut buffer
                                    as *mut *const buffer);
                                *fresh18 = (*host).strip_request_uri;
                                buffer_truncate(
                                    b_0,
                                    (buffer_clen(b_0))
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                );
                            }
                        }
                        18 => {
                            (*host)
                                .fix_root_path_name = (0 as libc::c_int as libc::c_uint
                                != (*cpv).v.u) as libc::c_int as libc::c_ushort;
                        }
                        19 => {
                            (*host)
                                .xsendfile_allow = (0 as libc::c_int as libc::c_uint
                                != (*cpv).v.u) as libc::c_int as libc::c_ushort;
                        }
                        20 => {
                            (*host)
                                .xsendfile_allow = (0 as libc::c_int as libc::c_uint
                                != (*cpv).v.u) as libc::c_int as libc::c_ushort;
                        }
                        21 => {
                            (*host).xsendfile_docroot = (*cpv).v.a;
                            if (*(*cpv).v.a).used != 0 {
                                let mut k: uint32_t = 0 as libc::c_int as uint32_t;
                                while k < (*(*cpv).v.a).used {
                                    let mut ds: *mut data_string = *((*(*cpv).v.a).data)
                                        .offset(k as isize) as *mut data_string;
                                    if (*ds).type_0 as libc::c_uint
                                        != TYPE_STRING as libc::c_int as libc::c_uint
                                    {
                                        log_error(
                                            (*srv).errh,
                                            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                                            1502 as libc::c_int as libc::c_uint,
                                            b"unexpected type for x-sendfile-docroot; expected: \"x-sendfile-docroot\" => ( \"/allowed/path\", ... )\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                        current_block = 18182134051239909875;
                                        break 's_28;
                                    } else if *((*ds).value.ptr)
                                            .offset(0 as libc::c_int as isize) as libc::c_int
                                            != '/' as i32
                                        {
                                        log_error(
                                            (*srv).errh,
                                            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                                            1509 as libc::c_int as libc::c_uint,
                                            b"x-sendfile-docroot paths must begin with '/'; invalid: \"%s\"\0"
                                                as *const u8 as *const libc::c_char,
                                            (*ds).value.ptr,
                                        );
                                        current_block = 18182134051239909875;
                                        break 's_28;
                                    } else {
                                        buffer_path_simplify(&mut (*ds).value);
                                        buffer_append_slash(&mut (*ds).value);
                                        k = k.wrapping_add(1);
                                    }
                                }
                            }
                        }
                        22 => {
                            (*host)
                                .tcp_fin_propagate = (0 as libc::c_int as libc::c_uint
                                != (*cpv).v.u) as libc::c_int as libc::c_ushort;
                        }
                        23 => {
                            (*host).connect_timeout = (*cpv).v.u as libc::c_ushort;
                        }
                        24 => {
                            (*host).write_timeout = (*cpv).v.u as libc::c_ushort;
                        }
                        25 => {
                            (*host).read_timeout = (*cpv).v.u as libc::c_ushort;
                        }
                        _ => {}
                    }
                    cpv = cpv.offset(1);
                }
                let mut m: uint32_t = 0 as libc::c_int as uint32_t;
                while m < (*da_host).value.used {
                    if !(strchr(
                        (**((*da_host).value.data).offset(m as isize)).key.ptr,
                        '_' as i32,
                    ))
                        .is_null()
                    {
                        log_error(
                            (*srv).errh,
                            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                            1538 as libc::c_int as libc::c_uint,
                            b"incorrect directive contains underscore ('_') instead of dash ('-'): %s\0"
                                as *const u8 as *const libc::c_char,
                            (**((*da_host).value.data).offset(m as isize)).key.ptr,
                        );
                    }
                    m = m.wrapping_add(1);
                }
                if (!((*host).host).is_null() || (*host).port as libc::c_int != 0)
                    && !((*host).unixsocket).is_null()
                {
                    log_error(
                        (*srv).errh,
                        b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                        1545 as libc::c_int as libc::c_uint,
                        b"either host/port or socket have to be set in: %s = (%s => (%s ( ...\0"
                            as *const u8 as *const libc::c_char,
                        cpkkey,
                        (*da_ext).key.ptr,
                        (*da_host).key.ptr,
                    );
                    current_block = 18182134051239909875;
                    break 's_28;
                } else {
                    if !((*host).host).is_null()
                        && *(*(*host).host).ptr as libc::c_int == '/' as i32
                        && ((*host).unixsocket).is_null()
                    {
                        (*host).unixsocket = (*host).host;
                    }
                    if !((*host).unixsocket).is_null() {
                        let mut un: sockaddr_un = sockaddr_un {
                            sun_family: 0,
                            sun_path: [0; 108],
                        };
                        if (buffer_clen((*host).unixsocket))
                            .wrapping_add(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong
                            > (::std::mem::size_of::<[libc::c_char; 108]>()
                                as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                        {
                            log_error(
                                (*srv).errh,
                                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                                1562 as libc::c_int as libc::c_uint,
                                b"unixsocket is too long in: %s = (%s => (%s ( ...\0"
                                    as *const u8 as *const libc::c_char,
                                cpkkey,
                                (*da_ext).key.ptr,
                                (*da_host).key.ptr,
                            );
                            current_block = 18182134051239909875;
                            break 's_28;
                        } else {
                            if !((*host).bin_path).is_null() {
                                let mut duplicate: *mut gw_host = unixsocket_is_dup(
                                    p,
                                    (*host).unixsocket,
                                );
                                if !duplicate.is_null() {
                                    if buffer_is_equal((*host).bin_path, (*duplicate).bin_path)
                                        == 0
                                    {
                                        log_error(
                                            (*srv).errh,
                                            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                                            1573 as libc::c_int as libc::c_uint,
                                            b"duplicate unixsocket path: %s\0" as *const u8
                                                as *const libc::c_char,
                                            (*(*host).unixsocket).ptr,
                                        );
                                        current_block = 18182134051239909875;
                                        break 's_28;
                                    } else {
                                        gw_host_free(host);
                                        host = duplicate;
                                        (*host).refcount += 1;
                                    }
                                }
                            }
                            (*host).family = 1 as libc::c_int as libc::c_ushort;
                        }
                    } else if ((*host).host).is_null() && ((*host).bin_path).is_null() {
                        log_error(
                            (*srv).errh,
                            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                            1589 as libc::c_int as libc::c_uint,
                            b"host or bin-path have to be set in: %s = (%s => (%s ( ...\0"
                                as *const u8 as *const libc::c_char,
                            cpkkey,
                            (*da_ext).key.ptr,
                            (*da_host).key.ptr,
                        );
                        current_block = 18182134051239909875;
                        break 's_28;
                    } else {
                        if 0 as libc::c_int == (*host).port as libc::c_int {
                            (*host).port = 80 as libc::c_int as libc::c_ushort;
                        }
                        if ((*host).host).is_null() {
                            (*host).host = &lhost;
                        }
                        (*host)
                            .family = (if !(strchr((*(*host).host).ptr, ':' as i32))
                            .is_null()
                        {
                            10 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) as libc::c_ushort;
                    }
                    if (*host).refcount == 0 {
                        gw_status_init_host(host);
                    }
                    if !((*host).refcount != 0) {
                        if !((*host).bin_path).is_null() {
                            let mut st: stat = stat {
                                st_dev: 0,
                                st_ino: 0,
                                st_nlink: 0,
                                st_mode: 0,
                                st_uid: 0,
                                st_gid: 0,
                                __pad0: 0,
                                st_rdev: 0,
                                st_size: 0,
                                st_blksize: 0,
                                st_blocks: 0,
                                st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
                                st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
                                st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
                                __glibc_reserved: [0; 3],
                            };
                            parse_binpath(&mut (*host).args, (*host).bin_path);
                            if 0 as libc::c_int
                                != stat(
                                    *((*host).args.ptr).offset(0 as libc::c_int as isize),
                                    &mut st,
                                )
                                || !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                                    == 0o100000 as libc::c_int as libc::c_uint)
                                || st.st_mode
                                    & (0o100 as libc::c_int
                                        | 0o100 as libc::c_int >> 3 as libc::c_int
                                        | 0o100 as libc::c_int >> 3 as libc::c_int
                                            >> 3 as libc::c_int) as libc::c_uint == 0
                            {
                                log_error(
                                    (*srv).errh,
                                    b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                                    1619 as libc::c_int as libc::c_uint,
                                    b"invalid \"bin-path\" => \"%s\" (check that file exists, is regular file, and is executable by lighttpd)\0"
                                        as *const u8 as *const libc::c_char,
                                    (*(*host).bin_path).ptr,
                                );
                            }
                            if sh_exec != 0 {
                                let mut m_0: uint32_t = 0 as libc::c_int as uint32_t;
                                while m_0 < (*host).args.used {
                                    free(
                                        *((*host).args.ptr).offset(m_0 as isize)
                                            as *mut libc::c_void,
                                    );
                                    m_0 = m_0.wrapping_add(1);
                                }
                                free((*host).args.ptr as *mut libc::c_void);
                                (*host)
                                    .args
                                    .ptr = calloc(
                                    4 as libc::c_int as libc::c_ulong,
                                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                                ) as *mut *mut libc::c_char;
                                if ((*host).args.ptr).is_null() {
                                    ck_assert_failed(
                                        b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                                        1634 as libc::c_int as libc::c_uint,
                                        b"host->args.ptr\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                (*host).args.used = 3 as libc::c_int as uint32_t;
                                (*host).args.size = 4 as libc::c_int as uint32_t;
                                let ref mut fresh19 = *((*host).args.ptr)
                                    .offset(0 as libc::c_int as isize);
                                *fresh19 = malloc(
                                    ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                                ) as *mut libc::c_char;
                                if (*((*host).args.ptr).offset(0 as libc::c_int as isize))
                                    .is_null()
                                {
                                    ck_assert_failed(
                                        b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                                        1638 as libc::c_int as libc::c_uint,
                                        b"host->args.ptr[0]\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                memcpy(
                                    *((*host).args.ptr).offset(0 as libc::c_int as isize)
                                        as *mut libc::c_void,
                                    b"/bin/sh\0" as *const u8 as *const libc::c_char
                                        as *const libc::c_void,
                                    ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                                );
                                let ref mut fresh20 = *((*host).args.ptr)
                                    .offset(1 as libc::c_int as isize);
                                *fresh20 = malloc(
                                    ::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong,
                                ) as *mut libc::c_char;
                                if (*((*host).args.ptr).offset(1 as libc::c_int as isize))
                                    .is_null()
                                {
                                    ck_assert_failed(
                                        b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                                        1641 as libc::c_int as libc::c_uint,
                                        b"host->args.ptr[1]\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                memcpy(
                                    *((*host).args.ptr).offset(1 as libc::c_int as isize)
                                        as *mut libc::c_void,
                                    b"-c\0" as *const u8 as *const libc::c_char
                                        as *const libc::c_void,
                                    ::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong,
                                );
                                let ref mut fresh21 = *((*host).args.ptr)
                                    .offset(2 as libc::c_int as isize);
                                *fresh21 = malloc(
                                    (::std::mem::size_of::<[libc::c_char; 6]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(
                                            buffer_clen((*host).bin_path) as libc::c_ulong,
                                        )
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as *mut libc::c_char;
                                if (*((*host).args.ptr).offset(2 as libc::c_int as isize))
                                    .is_null()
                                {
                                    ck_assert_failed(
                                        b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                                        1645 as libc::c_int as libc::c_uint,
                                        b"host->args.ptr[2]\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                memcpy(
                                    *((*host).args.ptr).offset(2 as libc::c_int as isize)
                                        as *mut libc::c_void,
                                    b"exec \0" as *const u8 as *const libc::c_char
                                        as *const libc::c_void,
                                    (::std::mem::size_of::<[libc::c_char; 6]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                );
                                memcpy(
                                    (*((*host).args.ptr).offset(2 as libc::c_int as isize))
                                        .offset(
                                            ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                                                as isize,
                                        )
                                        .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                                    (*(*host).bin_path).ptr as *const libc::c_void,
                                    (buffer_clen((*host).bin_path))
                                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                                        as libc::c_ulong,
                                );
                                let ref mut fresh22 = *((*host).args.ptr)
                                    .offset(3 as libc::c_int as isize);
                                *fresh22 = 0 as *mut libc::c_char;
                            }
                            if (*host).min_procs as libc::c_int
                                > (*host).max_procs as libc::c_int
                            {
                                (*host).min_procs = (*host).max_procs;
                            }
                            if (*host).min_procs as libc::c_int
                                != (*host).max_procs as libc::c_int
                                && 0 as libc::c_int
                                    != (*srv).srvconf.max_worker as libc::c_int
                            {
                                (*host).min_procs = (*host).max_procs;
                                log_error(
                                    (*srv).errh,
                                    b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                                    1657 as libc::c_int as libc::c_uint,
                                    b"adaptive backend spawning disabled (server.max_worker is non-zero)\0"
                                        as *const u8 as *const libc::c_char,
                                );
                            }
                            if ((*host).max_load_per_proc as libc::c_int)
                                < 1 as libc::c_int
                            {
                                (*host)
                                    .max_load_per_proc = 0 as libc::c_int as libc::c_ushort;
                            }
                            if (*s).debug != 0 {
                                log_error(
                                    (*srv).errh,
                                    b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                                    1665 as libc::c_int as libc::c_uint,
                                    b"--- gw spawning local\n\tproc: %s\n\tport: %hu\n\tsocket %s\n\tmin-procs: %d\n\tmax-procs: %d\0"
                                        as *const u8 as *const libc::c_char,
                                    (*(*host).bin_path).ptr,
                                    (*host).port as libc::c_int,
                                    if !((*host).unixsocket).is_null() {
                                        (*(*host).unixsocket).ptr as *const libc::c_char
                                    } else {
                                        b"\0" as *const u8 as *const libc::c_char
                                    },
                                    (*host).min_procs as libc::c_int,
                                    (*host).max_procs as libc::c_int,
                                );
                            }
                            let mut pno: uint32_t = 0 as libc::c_int as uint32_t;
                            while pno < (*host).min_procs as libc::c_uint {
                                let proc_0: *mut gw_proc = gw_proc_init(host);
                                if (*s).debug != 0 {
                                    log_error(
                                        (*srv).errh,
                                        b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                                        1683 as libc::c_int as libc::c_uint,
                                        b"--- gw spawning\n\tport: %hu\n\tsocket %s\n\tcurrent: %u / %u\0"
                                            as *const u8 as *const libc::c_char,
                                        (*host).port as libc::c_int,
                                        if !((*host).unixsocket).is_null() {
                                            (*(*host).unixsocket).ptr as *const libc::c_char
                                        } else {
                                            b"\0" as *const u8 as *const libc::c_char
                                        },
                                        pno,
                                        (*host).max_procs as libc::c_int,
                                    );
                                }
                                if 0 as libc::c_int
                                    != gw_proc_sockaddr_init(host, proc_0, (*srv).errh)
                                {
                                    gw_proc_free(proc_0);
                                    current_block = 18182134051239909875;
                                    break 's_28;
                                } else if (*srv).srvconf.preflight_check == 0
                                        && gw_spawn_connection(
                                            host,
                                            proc_0,
                                            (*srv).errh,
                                            (*s).debug,
                                        ) != 0
                                    {
                                    log_error(
                                        (*srv).errh,
                                        b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                                        1700 as libc::c_int as libc::c_uint,
                                        b"[ERROR]: spawning gw failed.\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    gw_proc_free(proc_0);
                                    current_block = 18182134051239909875;
                                    break 's_28;
                                } else {
                                    (*proc_0).next = (*host).first;
                                    if !((*host).first).is_null() {
                                        (*(*host).first).prev = proc_0;
                                    }
                                    (*host).first = proc_0;
                                    (*host).num_procs = ((*host).num_procs).wrapping_add(1);
                                    pno = pno.wrapping_add(1);
                                }
                            }
                            if graceful_restart_bg != 0 {
                                graceful_restart_bg = 0 as libc::c_int;
                                log_error(
                                    (*srv).errh,
                                    b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                                    1715 as libc::c_int as libc::c_uint,
                                    b"server.graceful-restart-bg disabled (incompatible with %s.server \"bin-path\")\0"
                                        as *const u8 as *const libc::c_char,
                                    (*(*p).self_0).name,
                                );
                                let du: *mut data_unset = array_get_data_unset(
                                    (*srv).srvconf.feature_flags,
                                    b"server.graceful-restart-bg\0" as *const u8
                                        as *const libc::c_char,
                                    (::std::mem::size_of::<[libc::c_char; 27]>()
                                        as libc::c_ulong as uint32_t)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                );
                                if (*du).type_0 as libc::c_uint
                                    == TYPE_STRING as libc::c_int as libc::c_uint
                                {
                                    buffer_copy_string_len(
                                        &mut (*(du as *mut data_string)).value,
                                        b"false\0" as *const u8 as *const libc::c_char,
                                        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                                            as uint32_t)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                    );
                                } else {
                                    (*(du as *mut data_integer)).value = 0 as libc::c_int;
                                }
                            }
                        } else {
                            let proc_1: *mut gw_proc = gw_proc_init(host);
                            (*host).first = proc_1;
                            (*host).num_procs = ((*host).num_procs).wrapping_add(1);
                            (*host).min_procs = 1 as libc::c_int as libc::c_ushort;
                            (*host).max_procs = 1 as libc::c_int as libc::c_ushort;
                            if 0 as libc::c_int
                                != gw_proc_sockaddr_init(host, proc_1, (*srv).errh)
                            {
                                current_block = 18182134051239909875;
                                break 's_28;
                            }
                            gw_proc_set_state(
                                host,
                                proc_1,
                                PROC_STATE_RUNNING as libc::c_int,
                            );
                        }
                    }
                    let h: *const buffer = if !((*host).host).is_null() {
                        (*host).host
                    } else {
                        (*host).unixsocket
                    };
                    (*host)
                        .gw_hash = gw_hash(
                        (*h).ptr,
                        buffer_clen(h),
                        5381 as libc::c_int as uint32_t,
                    );
                    gw_extension_insert((*s).exts, &mut (*da_ext).key, host);
                    if host_mode as libc::c_int == 2 as libc::c_int {
                        (*host).refcount += 1;
                        gw_extension_insert((*s).exts_auth, &mut (*da_ext).key, host);
                    } else if host_mode as libc::c_int == 1 as libc::c_int {
                        (*host).refcount += 1;
                        gw_extension_insert((*s).exts_resp, &mut (*da_ext).key, host);
                    }
                    host = 0 as *mut gw_host;
                    n = n.wrapping_add(1);
                }
            }
        }
        j = j.wrapping_add(1);
    }
    match current_block {
        9675306770437543583 => return 1 as libc::c_int,
        _ => {
            if !host.is_null() {
                gw_host_free(host);
            }
            return 0 as libc::c_int;
        }
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn gw_get_defaults_balance(
    mut srv: *mut server,
    mut b: *const buffer,
) -> libc::c_int {
    if b.is_null() || buffer_is_blank(b) != 0 {
        return GW_BALANCE_LEAST_CONNECTION as libc::c_int;
    }
    if buffer_eq_slen(
        b,
        b"fair\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    ) != 0
    {
        return GW_BALANCE_LEAST_CONNECTION as libc::c_int;
    }
    if buffer_eq_slen(
        b,
        b"least-connection\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    ) != 0
    {
        return GW_BALANCE_LEAST_CONNECTION as libc::c_int;
    }
    if buffer_eq_slen(
        b,
        b"round-robin\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    ) != 0
    {
        return GW_BALANCE_RR as libc::c_int;
    }
    if buffer_eq_slen(
        b,
        b"hash\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    ) != 0
    {
        return GW_BALANCE_HASH as libc::c_int;
    }
    if buffer_eq_slen(
        b,
        b"sticky\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    ) != 0
    {
        return GW_BALANCE_STICKY as libc::c_int;
    }
    log_error(
        (*srv).errh,
        b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
        1789 as libc::c_int as libc::c_uint,
        b"xxxxx.balance has to be one of: least-connection, round-robin, hash, sticky, but not: %s\0"
            as *const u8 as *const libc::c_char,
        (*b).ptr,
    );
    return GW_BALANCE_LEAST_CONNECTION as libc::c_int;
}
unsafe extern "C" fn gw_set_state(
    mut hctx: *mut gw_handler_ctx,
    mut state: gw_connection_state_t,
) {
    (*hctx).state = state;
}
#[no_mangle]
pub unsafe extern "C" fn gw_set_transparent(mut hctx: *mut gw_handler_ctx) {
    if 1 as libc::c_int != (*(*hctx).host).family as libc::c_int {
        -(1 as libc::c_int) == fdevent_set_tcp_nodelay((*hctx).fd, 1 as libc::c_int);
    }
    (*hctx).wb_reqlen = -(1 as libc::c_int) as off_t;
    gw_set_state(hctx, GW_STATE_WRITE);
}
unsafe extern "C" fn gw_host_hctx_enq(hctx: *mut gw_handler_ctx) {
    let host: *mut gw_host = (*hctx).host;
    (*hctx).prev = 0 as *mut gw_handler_ctx;
    (*hctx).next = (*host).hctxs;
    if !((*hctx).next).is_null() {
        (*(*hctx).next).prev = hctx;
    }
    (*host).hctxs = hctx;
}
unsafe extern "C" fn gw_host_hctx_deq(hctx: *mut gw_handler_ctx) {
    if !((*hctx).prev).is_null() {
        (*(*hctx).prev).next = (*hctx).next;
    } else {
        (*(*hctx).host).hctxs = (*hctx).next;
    }
    if !((*hctx).next).is_null() {
        (*(*hctx).next).prev = (*hctx).prev;
    }
    (*hctx).next = 0 as *mut gw_handler_ctx;
    (*hctx).prev = 0 as *mut gw_handler_ctx;
}
unsafe extern "C" fn gw_backend_close(hctx: *mut gw_handler_ctx, r: *mut request_st) {
    if (*hctx).fd >= 0 as libc::c_int {
        fdevent_fdnode_event_del((*hctx).ev, (*hctx).fdn);
        fdevent_sched_close((*hctx).ev, (*hctx).fd, 1 as libc::c_int);
        (*hctx).fdn = 0 as *mut fdnode;
        (*hctx).fd = -(1 as libc::c_int);
        gw_host_hctx_deq(hctx);
    }
    if !((*hctx).host).is_null() {
        if !((*hctx).proc_0).is_null() {
            gw_proc_release(
                (*hctx).host,
                (*hctx).proc_0,
                (*hctx).conf.debug,
                (*r).conf.errh,
            );
            (*hctx).proc_0 = 0 as *mut gw_proc;
        }
        gw_host_reset((*hctx).host);
        (*hctx).host = 0 as *mut gw_host;
    }
}
unsafe extern "C" fn gw_connection_close(hctx: *mut gw_handler_ctx, r: *mut request_st) {
    let mut p: *mut gw_plugin_data = (*hctx).plugin_data;
    gw_backend_close(hctx, r);
    handler_ctx_free(hctx);
    let ref mut fresh23 = *((*r).plugin_ctx).offset((*p).id as isize);
    *fresh23 = 0 as *mut libc::c_void;
    if (*r).handler_module == (*p).self_0 as *const plugin {
        http_response_backend_done(r);
    }
}
unsafe extern "C" fn gw_reconnect(
    hctx: *mut gw_handler_ctx,
    r: *mut request_st,
) -> handler_t {
    gw_backend_close(hctx, r);
    (*hctx).host = gw_host_get(r, (*hctx).ext, (*hctx).conf.balance, (*hctx).conf.debug);
    if ((*hctx).host).is_null() {
        return HANDLER_FINISHED;
    }
    gw_host_assign((*hctx).host);
    (*hctx).request_id = 0 as libc::c_int;
    (*hctx).opts.xsendfile_allow = (*(*hctx).host).xsendfile_allow as uint8_t;
    (*hctx).opts.xsendfile_docroot = (*(*hctx).host).xsendfile_docroot;
    gw_set_state(hctx, GW_STATE_INIT);
    return HANDLER_COMEBACK;
}
#[no_mangle]
pub unsafe extern "C" fn gw_handle_request_reset(
    r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut p: *mut gw_plugin_data = p_d as *mut gw_plugin_data;
    let mut hctx: *mut gw_handler_ctx = *((*r).plugin_ctx).offset((*p).id as isize)
        as *mut gw_handler_ctx;
    if !hctx.is_null() {
        gw_connection_close(hctx, r);
    }
    return HANDLER_GO_ON;
}
#[cold]
unsafe extern "C" fn gw_conditional_tcp_fin(
    hctx: *mut gw_handler_ctx,
    r: *mut request_st,
) {
    if chunkqueue_is_empty(&mut (*hctx).wb) == 0 {
        return;
    }
    if (*(*hctx).host).tcp_fin_propagate == 0 {
        return;
    }
    if (*hctx).gw_mode as libc::c_int == 2 as libc::c_int {
        return;
    }
    if (*r).conf.stream_request_body as libc::c_int
        & (1 as libc::c_int) << 14 as libc::c_int != 0
    {
        return;
    }
    (*r)
        .conf
        .stream_request_body = ((*r).conf.stream_request_body as libc::c_int
        | (1 as libc::c_int) << 14 as libc::c_int) as libc::c_ushort;
    (*r)
        .conf
        .stream_request_body = ((*r).conf.stream_request_body as libc::c_int
        & !((1 as libc::c_int) << 15 as libc::c_int)) as libc::c_ushort;
    (*(*r).con).is_readable = 0 as libc::c_int as libc::c_schar;
    shutdown((*hctx).fd, SHUT_WR as libc::c_int);
    fdevent_fdnode_event_clr((*hctx).ev, (*hctx).fdn, 0x4 as libc::c_int);
}
unsafe extern "C" fn gw_write_request(
    hctx: *mut gw_handler_ctx,
    r: *mut request_st,
) -> handler_t {
    let mut current_block_94: u64;
    match (*hctx).state as libc::c_uint {
        0 => {
            (*hctx).proc_0 = 0 as *mut gw_proc;
            let mut proc_0: *mut gw_proc = (*(*hctx).host).first;
            while !proc_0.is_null() {
                if (*proc_0).state as libc::c_uint
                    == PROC_STATE_RUNNING as libc::c_int as libc::c_uint
                {
                    (*hctx).proc_0 = proc_0;
                    break;
                } else {
                    proc_0 = (*proc_0).next;
                }
            }
            if ((*hctx).proc_0).is_null() {
                return HANDLER_ERROR;
            }
            let mut proc_1: *mut gw_proc = (*(*hctx).proc_0).next;
            while !proc_1.is_null() {
                if !((*proc_1).state as libc::c_uint
                    != PROC_STATE_RUNNING as libc::c_int as libc::c_uint)
                {
                    if (*proc_1).load < (*(*hctx).proc_0).load {
                        (*hctx).proc_0 = proc_1;
                    }
                }
                proc_1 = (*proc_1).next;
            }
            gw_proc_load_inc((*hctx).host, (*hctx).proc_0);
            (*hctx)
                .fd = fdevent_socket_nb_cloexec(
                (*(*hctx).host).family as libc::c_int,
                SOCK_STREAM as libc::c_int,
                0 as libc::c_int,
            );
            if -(1 as libc::c_int) == (*hctx).fd {
                log_perror(
                    (*r).conf.errh,
                    b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                    1944 as libc::c_int as libc::c_uint,
                    b"socket() failed (cur_fds:%d) (max_fds:%d)\0" as *const u8
                        as *const libc::c_char,
                    (*(*(*r).con).srv).cur_fds,
                    (*(*(*r).con).srv).max_fds,
                );
                return HANDLER_ERROR;
            }
            (*(*(*r).con).srv).cur_fds += 1;
            (*hctx)
                .fdn = fdevent_register(
                (*hctx).ev,
                (*hctx).fd,
                Some(
                    gw_handle_fdevent
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            libc::c_int,
                        ) -> handler_t,
                ),
                hctx as *mut libc::c_void,
            );
            if (*(*hctx).proc_0).is_local != 0 {
                (*hctx).pid = (*(*hctx).proc_0).pid;
            }
            (*hctx).write_ts = log_monotonic_secs;
            gw_host_hctx_enq(hctx);
            match gw_establish_connection(
                r,
                (*hctx).host,
                (*hctx).proc_0,
                (*hctx).pid,
                (*hctx).fd,
                (*hctx).conf.debug,
            ) {
                1 => {
                    fdevent_fdnode_event_set(
                        (*hctx).ev,
                        (*hctx).fdn,
                        0x4 as libc::c_int,
                    );
                    gw_set_state(hctx, GW_STATE_CONNECT_DELAYED);
                    return HANDLER_WAIT_FOR_EVENT;
                }
                -1 => return HANDLER_ERROR,
                0 => {
                    (*hctx).reconnects = 0 as libc::c_int;
                }
                _ => {}
            }
            current_block_94 = 14183312310117581265;
        }
        1 => {
            current_block_94 = 14183312310117581265;
        }
        2 => {
            current_block_94 = 14254167867906520637;
        }
        3 => {
            current_block_94 = 15942550528347855625;
        }
        4 => return HANDLER_WAIT_FOR_EVENT,
        _ => {
            log_error(
                (*r).conf.errh,
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                2091 as libc::c_int as libc::c_uint,
                b"(debug) unknown state\0" as *const u8 as *const libc::c_char,
            );
            return HANDLER_ERROR;
        }
    }
    match current_block_94 {
        14183312310117581265 => {
            if (*hctx).state as libc::c_uint
                == GW_STATE_CONNECT_DELAYED as libc::c_int as libc::c_uint
            {
                let mut socket_error: libc::c_int = fdevent_connect_status((*hctx).fd);
                if socket_error != 0 as libc::c_int {
                    gw_proc_connect_error(
                        r,
                        (*hctx).host,
                        (*hctx).proc_0,
                        (*hctx).pid,
                        socket_error,
                        (*hctx).conf.debug,
                    );
                    return HANDLER_ERROR;
                }
                (*hctx).write_ts = log_monotonic_secs;
            }
            gw_proc_connect_success((*hctx).host, (*hctx).proc_0, (*hctx).conf.debug, r);
            gw_set_state(hctx, GW_STATE_PREPARE_WRITE);
            current_block_94 = 14254167867906520637;
        }
        _ => {}
    }
    match current_block_94 {
        14254167867906520637 => {
            let mut rc: handler_t = ((*hctx).create_env)
                .expect("non-null function pointer")(hctx);
            if HANDLER_GO_ON as libc::c_int as libc::c_uint != rc as libc::c_uint {
                if HANDLER_FINISHED as libc::c_int as libc::c_uint != rc as libc::c_uint
                    && HANDLER_ERROR as libc::c_int as libc::c_uint != rc as libc::c_uint
                {
                    fdevent_fdnode_event_clr(
                        (*hctx).ev,
                        (*hctx).fdn,
                        0x4 as libc::c_int,
                    );
                }
                return rc;
            }
            if 1 as libc::c_int != (*(*hctx).host).family as libc::c_int {
                if (*r).reqbody_length < 0 as libc::c_int as libc::c_long {
                    -(1 as libc::c_int)
                        == fdevent_set_tcp_nodelay((*hctx).fd, 1 as libc::c_int);
                }
            }
            (*hctx).read_ts = log_monotonic_secs;
            fdevent_fdnode_event_add(
                (*hctx).ev,
                (*hctx).fdn,
                0x1 as libc::c_int | 0x2000 as libc::c_int,
            );
            gw_set_state(hctx, GW_STATE_WRITE);
        }
        _ => {}
    }
    if chunkqueue_is_empty(&mut (*hctx).wb) == 0 {
        let errh: *mut log_error_st = (*r).conf.errh;
        let mut bytes_out: off_t = (*hctx).wb.bytes_out;
        if ((*(*(*r).con).srv).network_backend_write)
            .expect(
                "non-null function pointer",
            )(
            (*hctx).fd,
            &mut (*hctx).wb,
            (256 as libc::c_int * 1024 as libc::c_int) as off_t,
            errh,
        ) < 0 as libc::c_int
        {
            match *__errno_location() {
                32 | 107 | 104 => {
                    log_error(
                        errh,
                        b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                        2035 as libc::c_int as libc::c_uint,
                        b"connection was dropped after accept() (perhaps the gw process died), write-offset: %lld socket: %s\0"
                            as *const u8 as *const libc::c_char,
                        (*hctx).wb.bytes_out as libc::c_longlong,
                        (*(*(*hctx).proc_0).connection_name).ptr,
                    );
                    return HANDLER_ERROR;
                }
                _ => {
                    log_perror(
                        errh,
                        b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                        2043 as libc::c_int as libc::c_uint,
                        b"write failed\0" as *const u8 as *const libc::c_char,
                    );
                    return HANDLER_ERROR;
                }
            }
        } else {
            if (*hctx).wb.bytes_out > bytes_out {
                (*(*hctx).proc_0).last_used = log_monotonic_secs;
                (*hctx).write_ts = (*(*hctx).proc_0).last_used;
                if ((*hctx).stdin_append).is_some()
                    && chunkqueue_length(&mut (*hctx).wb)
                        < (65536 as libc::c_int - 16384 as libc::c_int) as libc::c_long
                    && chunkqueue_is_empty(&mut (*r).reqbody_queue) == 0
                {
                    let mut rc_0: handler_t = ((*hctx).stdin_append)
                        .expect("non-null function pointer")(hctx);
                    if HANDLER_GO_ON as libc::c_int as libc::c_uint
                        != rc_0 as libc::c_uint
                    {
                        return rc_0;
                    }
                }
            }
        }
    }
    if (*hctx).wb.bytes_out == (*hctx).wb_reqlen {
        fdevent_fdnode_event_clr((*hctx).ev, (*hctx).fdn, 0x4 as libc::c_int);
        gw_set_state(hctx, GW_STATE_READ);
    } else {
        let mut wblen: off_t = chunkqueue_length(&mut (*hctx).wb);
        if ((*hctx).wb.bytes_in < (*hctx).wb_reqlen
            || (*hctx).wb_reqlen < 0 as libc::c_int as libc::c_long)
            && wblen < (65536 as libc::c_int - 16384 as libc::c_int) as libc::c_long
        {
            if (*r).conf.stream_request_body as libc::c_int
                & (1 as libc::c_int) << 15 as libc::c_int == 0
            {
                (*r)
                    .conf
                    .stream_request_body = ((*r).conf.stream_request_body as libc::c_int
                    | (1 as libc::c_int) << 15 as libc::c_int) as libc::c_ushort;
                (*(*r).con).is_readable = 1 as libc::c_int as libc::c_schar;
            }
        }
        if 0 as libc::c_int as libc::c_long == wblen {
            fdevent_fdnode_event_clr((*hctx).ev, (*hctx).fdn, 0x4 as libc::c_int);
        } else if (if !((*hctx).fdn).is_null() {
                (*(*hctx).fdn).events
            } else {
                0 as libc::c_int
            }) & 0x4 as libc::c_int == 0
            {
            (*hctx).write_ts = log_monotonic_secs;
            fdevent_fdnode_event_add((*hctx).ev, (*hctx).fdn, 0x4 as libc::c_int);
        }
    }
    if (*r).conf.stream_request_body as libc::c_int
        & (1 as libc::c_int) << 13 as libc::c_int != 0
    {
        gw_conditional_tcp_fin(hctx, r);
    }
    return HANDLER_WAIT_FOR_EVENT;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn gw_backend_error(
    hctx: *mut gw_handler_ctx,
    r: *mut request_st,
) -> handler_t {
    if ((*hctx).backend_error).is_some() {
        ((*hctx).backend_error).expect("non-null function pointer")(hctx);
    }
    http_response_backend_error(r);
    gw_connection_close(hctx, r);
    return HANDLER_FINISHED;
}
#[cold]
unsafe extern "C" fn gw_write_error(
    hctx: *mut gw_handler_ctx,
    r: *mut request_st,
) -> handler_t {
    if (*hctx).state as libc::c_uint == GW_STATE_INIT as libc::c_int as libc::c_uint
        || (*hctx).state as libc::c_uint
            == GW_STATE_CONNECT_DELAYED as libc::c_int as libc::c_uint
    {
        if !((*hctx).proc_0).is_null() && (*(*hctx).proc_0).is_local != 0 {
            let srv: *mut server = (*(*r).con).srv;
            if 0 as libc::c_int == (*srv).srvconf.max_worker as libc::c_int {
                gw_restart_dead_procs(
                    (*hctx).host,
                    (*srv).errh,
                    (*hctx).conf.debug,
                    0 as libc::c_int,
                );
            }
        }
        let fresh24 = (*hctx).reconnects;
        (*hctx).reconnects = (*hctx).reconnects + 1;
        if fresh24 < 5 as libc::c_int {
            return gw_reconnect(hctx, r);
        }
    } else {
        let mut rc: handler_t = gw_recv_response(hctx, r);
        if rc as libc::c_uint != HANDLER_GO_ON as libc::c_int as libc::c_uint {
            return rc;
        }
    }
    if (*r).resp_body_started == 0 && (*r).http_status < 500 as libc::c_int
        && (*r).http_status != 400 as libc::c_int
    {
        (*r).http_status = 503 as libc::c_int;
    }
    return gw_backend_error(hctx, r);
}
unsafe extern "C" fn gw_send_request(
    hctx: *mut gw_handler_ctx,
    r: *mut request_st,
) -> handler_t {
    let mut rc: handler_t = gw_write_request(hctx, r);
    return (if HANDLER_ERROR as libc::c_int as libc::c_uint != rc as libc::c_uint {
        rc as libc::c_uint
    } else {
        gw_write_error(hctx, r) as libc::c_uint
    }) as handler_t;
}
#[no_mangle]
pub unsafe extern "C" fn gw_handle_subrequest(
    r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut p: *mut gw_plugin_data = p_d as *mut gw_plugin_data;
    let mut hctx: *mut gw_handler_ctx = *((*r).plugin_ctx).offset((*p).id as isize)
        as *mut gw_handler_ctx;
    if hctx.is_null() {
        return HANDLER_GO_ON;
    }
    let revents: libc::c_int = (*hctx).revents;
    if revents != 0 {
        (*hctx).revents = 0 as libc::c_int;
        let mut rc: handler_t = gw_process_fdevent(hctx, r, revents);
        if rc as libc::c_uint != HANDLER_GO_ON as libc::c_int as libc::c_uint
            && rc as libc::c_uint
                != HANDLER_WAIT_FOR_EVENT as libc::c_int as libc::c_uint
        {
            return rc;
        }
    }
    if (*r).conf.stream_response_body as libc::c_int
        & (1 as libc::c_int) << 1 as libc::c_int != 0
        && (*r).resp_body_started as libc::c_int != 0
    {
        if chunkqueue_length(&mut (*r).write_queue)
            > (65536 as libc::c_int - 4096 as libc::c_int) as libc::c_long
        {
            fdevent_fdnode_event_clr((*hctx).ev, (*hctx).fdn, 0x1 as libc::c_int);
        } else if (if !((*hctx).fdn).is_null() {
                (*(*hctx).fdn).events
            } else {
                0 as libc::c_int
            }) & 0x1 as libc::c_int == 0
            {
            let mut rc_0: handler_t = HANDLER_GO_ON;
            rc_0 = gw_recv_response(hctx, r);
            if rc_0 as libc::c_uint != HANDLER_GO_ON as libc::c_int as libc::c_uint {
                return rc_0;
            }
            (*hctx).read_ts = log_monotonic_secs;
            fdevent_fdnode_event_add((*hctx).ev, (*hctx).fdn, 0x1 as libc::c_int);
        }
    }
    if (*hctx).gw_mode as libc::c_int != 2 as libc::c_int
        && (if 0 as libc::c_int as libc::c_long == (*hctx).wb.bytes_in {
            ((*r).state as libc::c_uint
                == CON_STATE_READ_POST as libc::c_int as libc::c_uint
                || -(1 as libc::c_int) as libc::c_long == (*hctx).wb_reqlen)
                as libc::c_int
        } else {
            ((*hctx).wb.bytes_in < (*hctx).wb_reqlen
                || (*hctx).wb_reqlen < 0 as libc::c_int as libc::c_long) as libc::c_int
        }) != 0
    {
        if chunkqueue_length(&mut (*hctx).wb)
            > (65536 as libc::c_int - 4096 as libc::c_int) as libc::c_long
        {
            if (*r).conf.stream_request_body as libc::c_int
                & (1 as libc::c_int) << 1 as libc::c_int != 0
            {
                (*r)
                    .conf
                    .stream_request_body = ((*r).conf.stream_request_body as libc::c_int
                    & !((1 as libc::c_int) << 15 as libc::c_int)) as libc::c_ushort;
            }
            if 0 as libc::c_int as libc::c_long != (*hctx).wb.bytes_in {
                return HANDLER_WAIT_FOR_EVENT;
            }
        } else {
            let mut rc_1: handler_t = ((*(*r).con).reqbody_read)
                .expect("non-null function pointer")(r);
            if -(1 as libc::c_int) as libc::c_long == (*r).reqbody_length
                && (*hctx).opts.backend != BACKEND_PROXY as libc::c_int
            {
                return (if (*r).conf.stream_request_body as libc::c_int
                    & (1 as libc::c_int) << 0 as libc::c_int != 0
                {
                    http_response_reqbody_read_error(r, 411 as libc::c_int)
                        as libc::c_uint
                } else {
                    HANDLER_WAIT_FOR_EVENT as libc::c_int as libc::c_uint
                }) as handler_t;
            }
            if (*hctx).wb_reqlen < -(1 as libc::c_int) as libc::c_long
                && (*r).reqbody_length >= 0 as libc::c_int as libc::c_long
            {
                (*hctx).wb_reqlen = -(*hctx).wb_reqlen;
                if ((*hctx).stdin_append).is_some() {
                    let mut rca: handler_t = ((*hctx).stdin_append)
                        .expect("non-null function pointer")(hctx);
                    if HANDLER_GO_ON as libc::c_int as libc::c_uint
                        != rca as libc::c_uint
                    {
                        return rca;
                    }
                }
            }
            if (0 as libc::c_int as libc::c_long != (*hctx).wb.bytes_in
                || -(1 as libc::c_int) as libc::c_long == (*hctx).wb_reqlen)
                && chunkqueue_is_empty(&mut (*r).reqbody_queue) == 0
            {
                if ((*hctx).stdin_append).is_some() {
                    if chunkqueue_length(&mut (*hctx).wb)
                        < (65536 as libc::c_int - 16384 as libc::c_int) as libc::c_long
                    {
                        let mut rca_0: handler_t = ((*hctx).stdin_append)
                            .expect("non-null function pointer")(hctx);
                        if HANDLER_GO_ON as libc::c_int as libc::c_uint
                            != rca_0 as libc::c_uint
                        {
                            return rca_0;
                        }
                    }
                } else {
                    chunkqueue_append_chunkqueue(
                        &mut (*hctx).wb,
                        &mut (*r).reqbody_queue,
                    );
                }
                if (if !((*hctx).fdn).is_null() {
                    (*(*hctx).fdn).events
                } else {
                    0 as libc::c_int
                }) & 0x4 as libc::c_int != 0
                {
                    return (if rc_1 as libc::c_uint
                        == HANDLER_GO_ON as libc::c_int as libc::c_uint
                    {
                        HANDLER_WAIT_FOR_EVENT as libc::c_int as libc::c_uint
                    } else {
                        rc_1 as libc::c_uint
                    }) as handler_t;
                }
            }
            if rc_1 as libc::c_uint != HANDLER_GO_ON as libc::c_int as libc::c_uint {
                return rc_1;
            }
        }
    }
    let mut rc_2: handler_t = (if (0 as libc::c_int as libc::c_long
        == (*hctx).wb.bytes_in || chunkqueue_is_empty(&mut (*hctx).wb) == 0)
        && (*hctx).state as libc::c_uint
            != GW_STATE_CONNECT_DELAYED as libc::c_int as libc::c_uint
    {
        gw_send_request(hctx, r) as libc::c_uint
    } else {
        HANDLER_WAIT_FOR_EVENT as libc::c_int as libc::c_uint
    }) as handler_t;
    if HANDLER_WAIT_FOR_EVENT as libc::c_int as libc::c_uint != rc_2 as libc::c_uint {
        return rc_2;
    }
    if (*r).conf.stream_request_body as libc::c_int
        & (1 as libc::c_int) << 13 as libc::c_int != 0
    {
        gw_conditional_tcp_fin(hctx, r);
    }
    return HANDLER_WAIT_FOR_EVENT;
}
unsafe extern "C" fn gw_authorizer_ok(
    hctx: *mut gw_handler_ctx,
    r: *mut request_st,
) -> handler_t {
    let mut physpath: *mut libc::c_char = 0 as *mut libc::c_char;
    let host: *mut gw_host = (*hctx).host;
    if !((*host).docroot).is_null() {
        buffer_copy_buffer(&mut (*r).physical.doc_root, (*host).docroot);
        buffer_copy_buffer(&mut (*r).physical.basedir, (*host).docroot);
        buffer_copy_path_len2(
            &mut (*r).physical.path,
            (*(*host).docroot).ptr,
            buffer_clen((*host).docroot) as size_t,
            (*r).uri.path.ptr,
            buffer_clen(&mut (*r).uri.path) as size_t,
        );
        physpath = (*r).physical.path.ptr;
    }
    (*r)
        .conf
        .stream_response_body = ((*r).conf.stream_response_body as libc::c_int
        | (*hctx).opts.authorizer >> 1 as libc::c_int) as libc::c_ushort;
    gw_backend_close(hctx, r);
    handler_ctx_clear(hctx);
    (*r).loops_per_request += 1;
    if (*r).loops_per_request as libc::c_int > 5 as libc::c_int {
        log_error(
            (*r).conf.errh,
            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
            2292 as libc::c_int as libc::c_uint,
            b"too many loops while processing request: %s\0" as *const u8
                as *const libc::c_char,
            (*r).target_orig.ptr,
        );
        (*r).http_status = 500 as libc::c_int;
        (*r).handler_module = 0 as *const plugin;
        return HANDLER_FINISHED;
    }
    if !physpath.is_null() {
        (*r).physical.path.ptr = 0 as *mut libc::c_char;
    }
    http_response_reset(r);
    if !physpath.is_null() {
        (*r).physical.path.ptr = physpath;
    }
    (*r).handler_module = 0 as *const plugin;
    return HANDLER_COMEBACK;
}
unsafe extern "C" fn gw_recv_response(
    hctx: *mut gw_handler_ctx,
    r: *mut request_st,
) -> handler_t {
    let mut b: *mut buffer = if (*hctx).opts.backend == BACKEND_FASTCGI as libc::c_int
        || (*hctx).opts.backend == BACKEND_AJP13 as libc::c_int
    {
        chunk_buffer_acquire()
    } else {
        (*hctx).response
    };
    let bytes_in: off_t = (*r).write_queue.bytes_in;
    let mut rc: handler_t = http_response_read(r, &mut (*hctx).opts, b, (*hctx).fdn);
    if b != (*hctx).response {
        chunk_buffer_release(b);
    }
    let proc_0: *mut gw_proc = (*hctx).proc_0;
    match rc as libc::c_uint {
        1 => {
            (*proc_0).last_used = log_monotonic_secs;
            if (*hctx).gw_mode as libc::c_int == 2 as libc::c_int
                && (200 as libc::c_int == (*r).http_status
                    || 0 as libc::c_int == (*r).http_status)
            {
                return gw_authorizer_ok(hctx, r);
            }
            gw_connection_close(hctx, r);
            return HANDLER_FINISHED;
        }
        2 | 4 => return gw_recv_response_error(hctx, r, proc_0),
        _ => {
            if (*r).write_queue.bytes_in > bytes_in {
                (*proc_0).last_used = log_monotonic_secs;
                (*hctx).read_ts = (*proc_0).last_used;
            }
            return HANDLER_GO_ON;
        }
    };
}
#[cold]
unsafe extern "C" fn gw_recv_response_error(
    hctx: *mut gw_handler_ctx,
    r: *mut request_st,
    proc_0: *mut gw_proc,
) -> handler_t {
    if (*proc_0).is_local != 0 && 1 as libc::c_int as libc::c_uint == (*proc_0).load
        && (*proc_0).pid == (*hctx).pid
        && (*proc_0).state as libc::c_uint
            != PROC_STATE_DIED as libc::c_int as libc::c_uint
        && 0 as libc::c_int == (*(*(*r).con).srv).srvconf.max_worker as libc::c_int
    {
        let host: *mut gw_host = (*hctx).host;
        let errh: *mut log_error_st = (*(*(*r).con).srv).errh;
        if (*proc_0).disabled_until < log_monotonic_secs
            && 0 as libc::c_int != gw_proc_waitpid(host, proc_0, errh)
        {
            if (*hctx).conf.debug != 0 {
                log_error(
                    errh,
                    b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                    2373 as libc::c_int as libc::c_uint,
                    b"--- gw spawning\n\tsocket %s\n\tcurrent: 1/%d\0" as *const u8
                        as *const libc::c_char,
                    (*(*proc_0).connection_name).ptr,
                    (*host).num_procs,
                );
            }
            if gw_spawn_connection(host, proc_0, errh, (*hctx).conf.debug) != 0 {
                log_error(
                    errh,
                    b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                    2379 as libc::c_int as libc::c_uint,
                    b"respawning failed, will retry later\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
    }
    if (*r).resp_body_started as libc::c_int == 0 as libc::c_int {
        if (*hctx).wb.bytes_out == 0 as libc::c_int as libc::c_long
            && {
                let fresh25 = (*hctx).reconnects;
                (*hctx).reconnects = (*hctx).reconnects + 1;
                fresh25 < 5 as libc::c_int
            }
        {
            log_error(
                (*r).conf.errh,
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                2389 as libc::c_int as libc::c_uint,
                b"response not received, request not sent on socket: %s for %s?%.*s, reconnecting\0"
                    as *const u8 as *const libc::c_char,
                (*(*proc_0).connection_name).ptr,
                (*r).uri.path.ptr,
                buffer_clen(&mut (*r).uri.query) as libc::c_int,
                (*r).uri.query.ptr,
            );
            return gw_reconnect(hctx, r);
        }
        log_error(
            (*r).conf.errh,
            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
            2398 as libc::c_int as libc::c_uint,
            b"response not received, request sent: %lld on socket: %s for %s?%.*s, closing connection\0"
                as *const u8 as *const libc::c_char,
            (*hctx).wb.bytes_out as libc::c_longlong,
            (*(*proc_0).connection_name).ptr,
            (*r).uri.path.ptr,
            buffer_clen(&mut (*r).uri.query) as libc::c_int,
            (*r).uri.query.ptr,
        );
    } else if (*r).resp_htags
            & (1 as libc::c_ulong) << HTTP_HEADER_UPGRADE as libc::c_int == 0
        {
        log_error(
            (*r).conf.errh,
            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
            2404 as libc::c_int as libc::c_uint,
            b"response already sent out, but backend returned error on socket: %s for %s?%.*s, terminating connection\0"
                as *const u8 as *const libc::c_char,
            (*(*proc_0).connection_name).ptr,
            (*r).uri.path.ptr,
            buffer_clen(&mut (*r).uri.query) as libc::c_int,
            (*r).uri.query.ptr,
        );
    }
    return gw_backend_error(hctx, r);
}
unsafe extern "C" fn gw_handle_fdevent(
    mut ctx: *mut libc::c_void,
    mut revents: libc::c_int,
) -> handler_t {
    let mut hctx: *mut gw_handler_ctx = ctx as *mut gw_handler_ctx;
    (*hctx).revents |= revents;
    connection_jq_append((*hctx).con);
    return HANDLER_FINISHED;
}
unsafe extern "C" fn gw_process_fdevent(
    hctx: *mut gw_handler_ctx,
    r: *mut request_st,
    mut revents: libc::c_int,
) -> handler_t {
    if revents & 0x1 as libc::c_int != 0 {
        let mut rc: handler_t = gw_recv_response(hctx, r);
        if rc as libc::c_uint != HANDLER_GO_ON as libc::c_int as libc::c_uint {
            return rc;
        }
    }
    if revents & 0x4 as libc::c_int != 0 {
        return gw_send_request(hctx, r);
    }
    if revents & (0x10 as libc::c_int | 0x2000 as libc::c_int) != 0 {
        if (*hctx).state as libc::c_uint
            == GW_STATE_CONNECT_DELAYED as libc::c_int as libc::c_uint
        {
            return gw_send_request(hctx, r)
        } else if (*r).resp_body_started != 0 {
            let mut rc_0: handler_t = HANDLER_GO_ON;
            let flags: libc::c_ushort = (*r).conf.stream_response_body;
            (*r)
                .conf
                .stream_response_body = ((*r).conf.stream_response_body as libc::c_int
                & !((1 as libc::c_int) << 1 as libc::c_int)) as libc::c_ushort;
            (*r)
                .conf
                .stream_response_body = ((*r).conf.stream_response_body as libc::c_int
                | (1 as libc::c_int) << 15 as libc::c_int) as libc::c_ushort;
            loop {
                rc_0 = gw_recv_response(hctx, r);
                if !(rc_0 as libc::c_uint
                    == HANDLER_GO_ON as libc::c_int as libc::c_uint)
                {
                    break;
                }
            }
            (*r).conf.stream_response_body = flags;
            return rc_0;
        } else {
            let mut proc_0: *mut gw_proc = (*hctx).proc_0;
            log_error(
                (*r).conf.errh,
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                2453 as libc::c_int as libc::c_uint,
                b"error: unexpected close of gw connection for %s?%.*s (no gw process on socket: %s ?) %d\0"
                    as *const u8 as *const libc::c_char,
                (*r).uri.path.ptr,
                buffer_clen(&mut (*r).uri.query) as libc::c_int,
                (*r).uri.query.ptr,
                (*(*proc_0).connection_name).ptr,
                (*hctx).state as libc::c_uint,
            );
            gw_connection_close(hctx, r);
            return HANDLER_FINISHED;
        }
    } else {
        if revents & 0x8 as libc::c_int != 0 {
            log_error(
                (*r).conf.errh,
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                2463 as libc::c_int as libc::c_uint,
                b"gw: got a FDEVENT_ERR. Don't know why.\0" as *const u8
                    as *const libc::c_char,
            );
            return gw_backend_error(hctx, r);
        }
    }
    return HANDLER_GO_ON;
}
#[no_mangle]
pub unsafe extern "C" fn gw_check_extension(
    r: *mut request_st,
    p: *mut gw_plugin_data,
    mut uri_path_handler: libc::c_int,
    mut hctx_sz: size_t,
) -> handler_t {
    let mut fn_0: *mut buffer = if uri_path_handler != 0 {
        &mut (*r).uri.path
    } else {
        &mut (*r).physical.path
    };
    let s_len: size_t = buffer_clen(fn_0) as size_t;
    let mut extension: *mut gw_extension = 0 as *mut gw_extension;
    let mut host: *mut gw_host = 0 as *mut gw_host;
    let mut hctx: *mut gw_handler_ctx = 0 as *mut gw_handler_ctx;
    let mut gw_mode: libc::c_ushort = 0;
    if 0 as libc::c_int as libc::c_ulong == s_len {
        return HANDLER_GO_ON;
    }
    hctx = *((*r).plugin_ctx).offset((*p).id as isize) as *mut gw_handler_ctx;
    gw_mode = (if hctx.is_null() || ((*hctx).ext_auth).is_null() {
        0 as libc::c_int
    } else {
        2 as libc::c_int
    }) as libc::c_ushort;
    loop {
        let mut exts: *mut gw_exts = 0 as *mut gw_exts;
        if 0 as libc::c_int == gw_mode as libc::c_int {
            gw_mode = 2 as libc::c_int as libc::c_ushort;
            exts = (*p).conf.exts_auth;
        } else {
            gw_mode = 1 as libc::c_int as libc::c_ushort;
            exts = (*p).conf.exts_resp;
        }
        if !(0 as libc::c_int as libc::c_uint == (*exts).used) {
            if !((*p).conf.ext_mapping).is_null() {
                let mut ds: *mut data_string = array_match_key_suffix(
                    (*p).conf.ext_mapping,
                    fn_0,
                ) as *mut data_string;
                if !ds.is_null() {
                    let mut k: uint32_t = 0;
                    k = 0 as libc::c_int as uint32_t;
                    while k < (*exts).used {
                        extension = ((*exts).exts).offset(k as isize);
                        if buffer_is_equal(&mut (*ds).value, &(*extension).key) != 0 {
                            break;
                        }
                        k = k.wrapping_add(1);
                    }
                    if k == (*exts).used {
                        extension = 0 as *mut gw_extension;
                    }
                }
            }
            if extension.is_null() {
                let mut uri_path_len: size_t = buffer_clen(&mut (*r).uri.path) as size_t;
                let mut k_0: uint32_t = 0 as libc::c_int as uint32_t;
                while k_0 < (*exts).used {
                    let mut ext: *mut gw_extension = ((*exts).exts).offset(k_0 as isize);
                    let mut ct_len: size_t = buffer_clen(&(*ext).key) as size_t;
                    if *((*ext).key.ptr).offset(0 as libc::c_int as isize) as libc::c_int
                        == '/' as i32
                    {
                        if ct_len <= uri_path_len
                            && 0 as libc::c_int
                                == memcmp(
                                    (*r).uri.path.ptr as *const libc::c_void,
                                    (*ext).key.ptr as *const libc::c_void,
                                    ct_len,
                                )
                        {
                            extension = ext;
                            break;
                        }
                    } else if ct_len <= s_len
                            && 0 as libc::c_int
                                == memcmp(
                                    ((*fn_0).ptr)
                                        .offset(s_len as isize)
                                        .offset(-(ct_len as isize)) as *const libc::c_void,
                                    (*ext).key.ptr as *const libc::c_void,
                                    ct_len,
                                )
                        {
                        extension = ext;
                        break;
                    }
                    k_0 = k_0.wrapping_add(1);
                }
            }
        }
        if !(extension.is_null() && gw_mode as libc::c_int != 1 as libc::c_int) {
            break;
        }
    }
    if extension.is_null() {
        return HANDLER_GO_ON;
    }
    host = gw_host_get(r, extension, (*p).conf.balance, (*p).conf.debug);
    if host.is_null() {
        return HANDLER_FINISHED;
    }
    (*extension).note_is_sent = 0 as libc::c_int;
    if uri_path_handler != 0 {
        if (*host).check_local != 0 {
            return HANDLER_GO_ON;
        }
        if *((*extension).key.ptr).offset(0 as libc::c_int as isize) as libc::c_int
            == '/' as i32 && gw_mode as libc::c_int != 2 as libc::c_int
        {
            let mut elen: uint32_t = buffer_clen(&(*extension).key);
            let mut pathinfo: *const libc::c_char = 0 as *const libc::c_char;
            if 1 as libc::c_int as libc::c_uint == elen
                && (*host).fix_root_path_name as libc::c_int != 0
            {
                buffer_copy_buffer(&mut (*r).pathinfo, &mut (*r).uri.path);
                buffer_truncate(&mut (*r).uri.path, 0 as libc::c_int as uint32_t);
            } else if s_len > elen as libc::c_ulong
                    && {
                        pathinfo = strchr(
                            ((*r).uri.path.ptr).offset(elen as isize),
                            '/' as i32,
                        );
                        !pathinfo.is_null()
                    }
                {
                let plen: uint32_t = ((*r).uri.path.ptr)
                    .offset(s_len as isize)
                    .offset_from(pathinfo) as libc::c_long as uint32_t;
                buffer_copy_string_len(&mut (*r).pathinfo, pathinfo, plen as size_t);
                buffer_truncate(
                    &mut (*r).uri.path,
                    s_len.wrapping_sub(plen as libc::c_ulong) as uint32_t,
                );
            }
        }
    }
    if hctx.is_null() {
        hctx = handler_ctx_init(hctx_sz);
    }
    (*hctx).ev = (*(*(*r).con).srv).ev;
    (*hctx).r = r;
    (*hctx).con = (*r).con;
    (*hctx).plugin_data = p;
    (*hctx).host = host;
    (*hctx).proc_0 = 0 as *mut gw_proc;
    (*hctx).ext = extension;
    gw_host_assign(host);
    (*hctx).gw_mode = gw_mode;
    if gw_mode as libc::c_int == 2 as libc::c_int {
        (*hctx).ext_auth = (*hctx).ext;
    }
    (*hctx).conf.balance = (*p).conf.balance;
    (*hctx).conf.proto = (*p).conf.proto;
    (*hctx).conf.debug = (*p).conf.debug;
    (*hctx)
        .opts
        .max_per_read = (if (*r).conf.stream_response_body as libc::c_int
        & ((1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 1 as libc::c_int) == 0
    {
        262144 as libc::c_int
    } else if (*r).conf.stream_response_body as libc::c_int
            & (1 as libc::c_int) << 1 as libc::c_int != 0
        {
        16384 as libc::c_int
    } else {
        65536 as libc::c_int
    }) as uint32_t;
    (*hctx).opts.fdfmt = 0o140000 as libc::c_int;
    (*hctx)
        .opts
        .authorizer = (gw_mode as libc::c_int == 2 as libc::c_int) as libc::c_int;
    (*hctx).opts.local_redir = 0 as libc::c_int as uint8_t;
    (*hctx).opts.xsendfile_allow = (*host).xsendfile_allow as uint8_t;
    (*hctx).opts.xsendfile_docroot = (*host).xsendfile_docroot;
    let ref mut fresh26 = *((*r).plugin_ctx).offset((*p).id as isize);
    *fresh26 = hctx as *mut libc::c_void;
    (*r).handler_module = (*p).self_0;
    if (*r).conf.log_request_handling != 0 {
        log_error(
            (*r).conf.errh,
            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
            2682 as libc::c_int as libc::c_uint,
            b"handling the request using %s\0" as *const u8 as *const libc::c_char,
            (*(*p).self_0).name,
        );
    }
    return HANDLER_GO_ON;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn gw_handle_trigger_hctx_timeout(
    hctx: *mut gw_handler_ctx,
    msg: *const libc::c_char,
) {
    let r: *mut request_st = (*hctx).r;
    connection_jq_append((*r).con);
    if *msg as libc::c_int == 'c' as i32 {
        gw_proc_connect_error(
            r,
            (*hctx).host,
            (*hctx).proc_0,
            (*hctx).pid,
            110 as libc::c_int,
            (*hctx).conf.debug,
        );
        let fresh27 = (*hctx).reconnects;
        (*hctx).reconnects = (*hctx).reconnects + 1;
        if fresh27 < 1 as libc::c_int {
            gw_reconnect(hctx, r);
            return;
        }
        (*r).http_status = 503 as libc::c_int;
    } else {
        log_error(
            (*r).conf.errh,
            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
            2716 as libc::c_int as libc::c_uint,
            b"%s timeout on socket: %s (fd: %d)\0" as *const u8 as *const libc::c_char,
            msg,
            (*(*(*hctx).proc_0).connection_name).ptr,
            (*hctx).fd,
        );
        if *msg as libc::c_int == 'w' as i32 {
            gw_write_error(hctx, r);
            if (*r).http_status == 503 as libc::c_int {
                (*r).http_status = 504 as libc::c_int;
            }
            return;
        }
    }
    gw_backend_error(hctx, r);
    if (*r).http_status == 500 as libc::c_int && (*r).resp_body_started == 0
        && ((*r).handler_module).is_null()
    {
        (*r).http_status = 504 as libc::c_int;
    }
}
#[inline(never)]
unsafe extern "C" fn gw_handle_trigger_host_timeouts(host: *mut gw_host) {
    if ((*host).hctxs).is_null() {
        return;
    }
    let rsecs: unix_time64_t = (*host).read_timeout as unix_time64_t;
    let wsecs: unix_time64_t = (*host).write_timeout as unix_time64_t;
    let csecs: unix_time64_t = (*host).connect_timeout as unix_time64_t;
    if rsecs == 0 && wsecs == 0 && csecs == 0 {
        return;
    }
    let mono: unix_time64_t = log_monotonic_secs;
    let mut hctx: *mut gw_handler_ctx = (*host).hctxs;
    let mut next: *mut gw_handler_ctx = 0 as *mut gw_handler_ctx;
    while !hctx.is_null() {
        next = (*hctx).next;
        if (*hctx).state as libc::c_uint
            == GW_STATE_CONNECT_DELAYED as libc::c_int as libc::c_uint
        {
            if mono - (*hctx).write_ts > csecs && csecs != 0 {
                gw_handle_trigger_hctx_timeout(
                    hctx,
                    b"connect\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            let events: libc::c_int = if !((*hctx).fdn).is_null() {
                (*(*hctx).fdn).events
            } else {
                0 as libc::c_int
            };
            if events & 0x1 as libc::c_int != 0 && mono - (*hctx).read_ts > rsecs
                && rsecs != 0
            {
                gw_handle_trigger_hctx_timeout(
                    hctx,
                    b"read\0" as *const u8 as *const libc::c_char,
                );
            } else if events & 0x4 as libc::c_int != 0 && mono - (*hctx).write_ts > wsecs
                    && wsecs != 0
                {
                gw_handle_trigger_hctx_timeout(
                    hctx,
                    b"write\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        hctx = next;
    }
}
unsafe extern "C" fn gw_handle_trigger_host(
    host: *mut gw_host,
    errh: *mut log_error_st,
    debug: libc::c_int,
) {
    gw_handle_trigger_host_timeouts(host);
    let mut proc_0: *mut gw_proc = 0 as *mut gw_proc;
    let mut idle_timestamp: unix_time64_t = 0;
    let mut overload: libc::c_int = 1 as libc::c_int;
    gw_restart_dead_procs(host, errh, debug, 1 as libc::c_int);
    if (*host).min_procs as libc::c_int == (*host).max_procs as libc::c_int {
        return;
    }
    if ((*host).bin_path).is_null() {
        return;
    }
    proc_0 = (*host).first;
    while !proc_0.is_null() {
        if (*proc_0).load <= (*host).max_load_per_proc as libc::c_uint {
            overload = 0 as libc::c_int;
            break;
        } else {
            proc_0 = (*proc_0).next;
        }
    }
    if overload != 0 && (*host).num_procs != 0
        && (*host).num_procs < (*host).max_procs as libc::c_uint
    {
        if debug != 0 {
            log_error(
                errh,
                b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                2798 as libc::c_int as libc::c_uint,
                b"overload detected, spawning a new child\0" as *const u8
                    as *const libc::c_char,
            );
        }
        gw_proc_spawn(host, errh, debug);
    }
    idle_timestamp = log_monotonic_secs - (*host).idle_timeout as libc::c_long;
    proc_0 = (*host).first;
    while !proc_0.is_null() {
        if (*host).num_procs <= (*host).min_procs as libc::c_uint {
            break;
        }
        if !(0 as libc::c_int as libc::c_uint != (*proc_0).load) {
            if !((*proc_0).pid <= 0 as libc::c_int) {
                if !((*proc_0).last_used >= idle_timestamp) {
                    if debug != 0 {
                        log_error(
                            errh,
                            b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                            2814 as libc::c_int as libc::c_uint,
                            b"idle-timeout reached, terminating child: socket: %s pid %d\0"
                                as *const u8 as *const libc::c_char,
                            if !((*proc_0).unixsocket).is_null() {
                                (*(*proc_0).unixsocket).ptr as *const libc::c_char
                            } else {
                                b"\0" as *const u8 as *const libc::c_char
                            },
                            (*proc_0).pid,
                        );
                    }
                    gw_proc_kill(host, proc_0);
                    break;
                }
            }
        }
        proc_0 = (*proc_0).next;
    }
}
unsafe extern "C" fn gw_handle_trigger_exts(
    exts: *mut gw_exts,
    errh: *mut log_error_st,
    debug: libc::c_int,
) {
    let mut j: uint32_t = 0 as libc::c_int as uint32_t;
    while j < (*exts).used {
        let mut ex: *mut gw_extension = ((*exts).exts).offset(j as isize);
        let mut n: uint32_t = 0 as libc::c_int as uint32_t;
        while n < (*ex).used {
            gw_handle_trigger_host(*((*ex).hosts).offset(n as isize), errh, debug);
            n = n.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn gw_handle_trigger_exts_wkr(
    mut exts: *mut gw_exts,
    mut errh: *mut log_error_st,
) {
    let mut j: uint32_t = 0 as libc::c_int as uint32_t;
    while j < (*exts).used {
        let ex: *mut gw_extension = ((*exts).exts).offset(j as isize);
        let mut n: uint32_t = 0 as libc::c_int as uint32_t;
        while n < (*ex).used {
            let host: *mut gw_host = *((*ex).hosts).offset(n as isize);
            gw_handle_trigger_host_timeouts(host);
            let mut proc_0: *mut gw_proc = (*host).first;
            while !proc_0.is_null() {
                if (*proc_0).state as libc::c_uint
                    == PROC_STATE_OVERLOADED as libc::c_int as libc::c_uint
                {
                    gw_proc_check_enable(host, proc_0, errh);
                }
                proc_0 = (*proc_0).next;
            }
            n = n.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn gw_handle_trigger(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *mut gw_plugin_data = p_d as *mut gw_plugin_data;
    let mut wkr: libc::c_int = (0 as libc::c_int
        != (*srv).srvconf.max_worker as libc::c_int && (*p).srv_pid != (*srv).pid)
        as libc::c_int;
    let errh: *mut log_error_st = (*srv).errh;
    let mut global_debug: libc::c_int = 0 as libc::c_int;
    if ((*p).cvlist).is_null() {
        return HANDLER_GO_ON;
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
        let mut conf: *mut gw_plugin_config = 0 as *mut gw_plugin_config;
        let mut debug: libc::c_int = global_debug;
        while -(1 as libc::c_int) != (*cpv).k_id {
            match (*cpv).k_id {
                0 => {
                    if (*cpv).vtype as libc::c_uint
                        == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
                    {
                        conf = (*cpv).v.v as *mut gw_plugin_config;
                    }
                }
                2 => {
                    debug = (*cpv).v.u as libc::c_int;
                    if 0 as libc::c_int == i {
                        global_debug = (*cpv).v.u as libc::c_int;
                    }
                }
                _ => {}
            }
            cpv = cpv.offset(1);
        }
        if !(conf.is_null() || ((*conf).exts).is_null()) {
            if wkr != 0 {
                gw_handle_trigger_exts_wkr((*conf).exts, errh);
            } else {
                gw_handle_trigger_exts((*conf).exts, errh, debug);
            };
        }
        i += 1;
    }
    return HANDLER_GO_ON;
}
#[no_mangle]
pub unsafe extern "C" fn gw_handle_waitpid_cb(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
    mut pid: pid_t,
    mut status: libc::c_int,
) -> handler_t {
    let p: *mut gw_plugin_data = p_d as *mut gw_plugin_data;
    if 0 as libc::c_int != (*srv).srvconf.max_worker as libc::c_int
        && (*p).srv_pid != (*srv).pid
    {
        return HANDLER_GO_ON;
    }
    let errh: *mut log_error_st = (*srv).errh;
    let mut global_debug: libc::c_int = 0 as libc::c_int;
    if ((*p).cvlist).is_null() {
        return HANDLER_GO_ON;
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
        let mut conf: *mut gw_plugin_config = 0 as *mut gw_plugin_config;
        let mut debug: libc::c_int = global_debug;
        while -(1 as libc::c_int) != (*cpv).k_id {
            match (*cpv).k_id {
                0 => {
                    if (*cpv).vtype as libc::c_uint
                        == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
                    {
                        conf = (*cpv).v.v as *mut gw_plugin_config;
                    }
                }
                2 => {
                    debug = (*cpv).v.u as libc::c_int;
                    if 0 as libc::c_int == i {
                        global_debug = (*cpv).v.u as libc::c_int;
                    }
                }
                _ => {}
            }
            cpv = cpv.offset(1);
        }
        if !(conf.is_null() || ((*conf).exts).is_null()) {
            let cur_ts: unix_time64_t = log_monotonic_secs;
            let mut exts: *mut gw_exts = (*conf).exts;
            let mut j: uint32_t = 0 as libc::c_int as uint32_t;
            while j < (*exts).used {
                let mut ex: *mut gw_extension = ((*exts).exts).offset(j as isize);
                let mut n: uint32_t = 0 as libc::c_int as uint32_t;
                while n < (*ex).used {
                    let mut host: *mut gw_host = *((*ex).hosts).offset(n as isize);
                    let mut proc_0: *mut gw_proc = 0 as *mut gw_proc;
                    proc_0 = (*host).first;
                    while !proc_0.is_null() {
                        if (*proc_0).is_local == 0 || (*proc_0).pid != pid {
                            proc_0 = (*proc_0).next;
                        } else {
                            gw_proc_waitpid_log(host, proc_0, errh, status);
                            gw_proc_set_state(
                                host,
                                proc_0,
                                PROC_STATE_DIED as libc::c_int,
                            );
                            (*proc_0).pid = 0 as libc::c_int;
                            if (*proc_0).disabled_until < cur_ts {
                                if (*proc_0).state as libc::c_uint
                                    != PROC_STATE_KILLED as libc::c_int as libc::c_uint
                                {
                                    (*proc_0).disabled_until = cur_ts;
                                }
                                if gw_spawn_connection(host, proc_0, errh, debug) != 0 {
                                    log_error(
                                        errh,
                                        b"src/gw_backend.c\0" as *const u8 as *const libc::c_char,
                                        2947 as libc::c_int as libc::c_uint,
                                        b"ERROR: spawning gw failed.\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                            }
                            return HANDLER_FINISHED;
                        }
                    }
                    proc_0 = (*host).unused_procs;
                    while !proc_0.is_null() {
                        if (*proc_0).is_local == 0 || (*proc_0).pid != pid {
                            proc_0 = (*proc_0).next;
                        } else {
                            gw_proc_waitpid_log(host, proc_0, errh, status);
                            if (*proc_0).state as libc::c_uint
                                != PROC_STATE_KILLED as libc::c_int as libc::c_uint
                            {
                                (*proc_0).disabled_until = cur_ts;
                            }
                            gw_proc_set_state(
                                host,
                                proc_0,
                                PROC_STATE_DIED as libc::c_int,
                            );
                            (*proc_0).pid = 0 as libc::c_int;
                            return HANDLER_FINISHED;
                        }
                    }
                    n = n.wrapping_add(1);
                }
                j = j.wrapping_add(1);
            }
        }
        i += 1;
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn run_static_initializers() {
    cpk = [
        {
            let mut init = config_plugin_keys_t {
                k: b"host\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"port\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"socket\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"listen-backlog\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_INT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"bin-path\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"kill-signal\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"check-local\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"mode\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"docroot\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"min-procs\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"max-procs\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"max-load-per-proc\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"idle-timeout\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"disable-time\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"bin-environment\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVSTRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"bin-copy-environment\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_VLIST as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"broken-scriptfilename\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"strip-request-uri\0" as *const u8 as *const libc::c_char,
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
                k: b"fix-root-scriptname\0" as *const u8 as *const libc::c_char,
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
                k: b"allow-x-send-file\0" as *const u8 as *const libc::c_char,
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
                k: b"x-sendfile\0" as *const u8 as *const libc::c_char,
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
                k: b"x-sendfile-docroot\0" as *const u8 as *const libc::c_char,
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
                k: b"tcp-fin-propagate\0" as *const u8 as *const libc::c_char,
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
                k: b"connect-timeout\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_INT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"write-timeout\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_INT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"read-timeout\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_INT as libc::c_int as uint8_t,
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
    lhost = {
        let mut init = buffer {
            ptr: b"127.0.0.1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            used: (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong
                as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint),
            size: 0 as libc::c_int as uint32_t,
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
