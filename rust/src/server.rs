#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
use ::rust::*;
extern "C" {
    pub type pcre2_real_match_data_8;
    pub type lshpack_double_enc_head;
    pub type lshpack_enc_table_entry;
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
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fdlog_free(fdlog: *mut fdlog_st);
    fn fdlog_flushall(errh: *mut fdlog_st);
    fn fdlog_files_cycle(errh: *mut fdlog_st);
    fn fdlog_pipes_waitpid_cb(pid: pid_t) -> libc::c_int;
    fn fdlog_pipes_restart(ts: unix_time64_t);
    fn fdlog_pipes_abandon_pids();
    fn time(__timer: *mut time_t) -> time_t;
    fn tzset();
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn buffer_init() -> *mut buffer;
    fn buffer_free(b: *mut buffer);
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_int(b: *mut buffer, val: intmax_t);
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
    fn array_get_data_unset(
        a: *const array,
        key: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut data_unset;
    fn chunkqueue_chunk_pool_clear();
    fn chunkqueue_chunk_pool_free();
    fn chunkqueue_set_tempdirs_default_reset();
    fn chunkqueue_internal_pipes(init: libc::c_int);
    static mut log_con_jqueue: *mut connection;
    fn getsockname(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn getpeername(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> libc::c_int;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sock_addr_is_addr_wildcard(saddr: *const sock_addr) -> libc::c_int;
    fn sock_addr_is_port_eq(
        saddr1: *const sock_addr,
        saddr2: *const sock_addr,
    ) -> libc::c_int;
    fn sock_addr_is_addr_eq(
        saddr1: *const sock_addr,
        saddr2: *const sock_addr,
    ) -> libc::c_int;
    fn sock_addr_inet_pton(
        saddr: *mut sock_addr,
        str: *const libc::c_char,
        family: libc::c_int,
        port: libc::c_ushort,
    ) -> libc::c_int;
    fn network_accept_tcp_nagle_disable(fd: libc::c_int);
    fn network_init(srv: *mut server, stdin_fd: libc::c_int) -> libc::c_int;
    fn network_close(srv: *mut server) -> libc::c_int;
    fn network_register_fdevents(srv: *mut server) -> libc::c_int;
    fn network_unregister_sock(srv: *mut server, srv_socket: *mut server_socket);
    fn network_socket_activation_to_env(srv: *mut server);
    static mut log_epoch_secs: unix_time64_t;
    static mut log_monotonic_secs: unix_time64_t;
    fn write_all(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
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
    fn log_set_global_errh(
        errh: *mut log_error_st,
        ts_high_precision: libc::c_int,
    ) -> *mut log_error_st;
    fn li_rand_reseed();
    fn li_rand_cleanup();
    fn h2_send_1xx(r: *mut request_st, con: *mut connection) -> libc::c_int;
    fn fdevent_show_event_handlers() -> *const libc::c_char;
    fn fdevent_init(
        event_handler: *const libc::c_char,
        max_fds: *mut libc::c_int,
        cur_fds: *mut libc::c_int,
        errh: *mut log_error_st,
    ) -> *mut fdevents;
    fn fdevent_free(ev: *mut fdevents);
    fn fdevent_fdnode_event_del(ev: *mut fdevents, fdn: *mut fdnode);
    fn fdevent_fdnode_event_set(
        ev: *mut fdevents,
        fdn: *mut fdnode,
        events: libc::c_int,
    );
    fn fdevent_poll(ev: *mut fdevents, timeout_ms: libc::c_int) -> libc::c_int;
    fn fdevent_register(
        ev: *mut fdevents,
        fd: libc::c_int,
        handler: fdevent_handler,
        ctx: *mut libc::c_void,
    ) -> *mut fdnode;
    fn fdevent_unregister(ev: *mut fdevents, fd: libc::c_int);
    fn fdevent_fcntl_set_nb_cloexec(fd: libc::c_int) -> libc::c_int;
    fn fdevent_fcntl_set_nb_cloexec_sock(fd: libc::c_int) -> libc::c_int;
    fn fdevent_open_cloexec(
        pathname: *const libc::c_char,
        symlinks: libc::c_int,
        flags: libc::c_int,
        mode: mode_t,
    ) -> libc::c_int;
    fn fdevent_pipe_cloexec(
        fds: *mut libc::c_int,
        bufsz_hint: libc::c_uint,
    ) -> libc::c_int;
    fn fdevent_open_devnull() -> libc::c_int;
    fn fdevent_set_stdin_stdout_stderr(
        fdin: libc::c_int,
        fdout: libc::c_int,
        fderr: libc::c_int,
    ) -> libc::c_int;
    fn fdevent_waitpid(
        pid: pid_t,
        status: *mut libc::c_int,
        nb: libc::c_int,
    ) -> libc::c_int;
    fn fdevent_waitpid_intr(pid: pid_t, status: *mut libc::c_int) -> libc::c_int;
    fn connections_pool_clear(srv: *mut server);
    fn connections_free(srv: *mut server);
    fn connection_graceful_shutdown_maint(srv: *mut server);
    fn connection_periodic_maint(srv: *mut server, cur_ts: unix_time64_t);
    fn connection_send_1xx(r: *mut request_st, con: *mut connection) -> libc::c_int;
    fn connection_accepted(
        srv: *mut server,
        srv_socket: *const server_socket,
        cnt_addr: *mut sock_addr,
        cnt: libc::c_int,
    ) -> *mut connection;
    fn connection_state_machine(con: *mut connection);
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn stat_cache_init(ev: *mut fdevents, errh: *mut log_error_st) -> libc::c_int;
    fn stat_cache_free();
    fn stat_cache_trigger_cleanup();
    fn plugins_load(srv: *mut server) -> libc::c_int;
    fn plugins_free(srv: *mut server);
    fn plugins_call_handle_request_env(r: *mut request_st) -> handler_t;
    fn plugins_call_handle_trigger(srv: *mut server);
    fn plugins_call_worker_init(srv: *mut server) -> handler_t;
    fn plugins_call_set_defaults(srv: *mut server) -> handler_t;
    fn plugins_call_init(srv: *mut server) -> handler_t;
    fn plugins_call_handle_sighup(srv: *mut server);
    fn plugins_call_handle_waitpid(
        srv: *mut server,
        pid: pid_t,
        status: libc::c_int,
    ) -> handler_t;
    fn config_reset_config_bytes_sec(p: *mut libc::c_void);
    fn config_log_error_close(srv: *mut server);
    fn config_log_error_open(srv: *mut server) -> libc::c_int;
    fn config_free(srv: *mut server);
    fn config_finalize(
        srv: *mut server,
        default_server_tag_0: *const buffer,
    ) -> libc::c_int;
    fn config_set_defaults(srv: *mut server) -> libc::c_int;
    fn config_read(srv: *mut server, fn_0: *const libc::c_char) -> libc::c_int;
    fn config_print(srv: *mut server);
    fn config_init(srv: *mut server);
    fn config_feature_int(
        srv: *const server,
        feature: *const libc::c_char,
        default_value: int32_t,
    ) -> int32_t;
    fn config_feature_bool(
        srv: *const server,
        feature: *const libc::c_char,
        default_value: libc::c_int,
    ) -> libc::c_int;
    fn network_write_show_handlers() -> *const libc::c_char;
    fn request_pool_free();
    fn http_response_send_1xx_cb_set(fn_0: http_response_send_1xx_cb, vers: libc::c_int);
    fn strftime_cache_reset();
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn execv(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn getpid() -> __pid_t;
    fn setsid() -> __pid_t;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getegid() -> __gid_t;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn setgid(__gid: __gid_t) -> libc::c_int;
    fn fork() -> __pid_t;
    static mut optind: libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off64_t) -> libc::c_int;
    fn chroot(__path: *const libc::c_char) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
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
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
    fn getloadavg(__loadavg: *mut libc::c_double, __nelem: libc::c_int) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn getgrgid(__gid: __gid_t) -> *mut group;
    fn getgrnam(__name: *const libc::c_char) -> *mut group;
    fn setgroups(__n: size_t, __groups: *const __gid_t) -> libc::c_int;
    fn initgroups(__user: *const libc::c_char, __group: __gid_t) -> libc::c_int;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn getrlimit(__resource: __rlimit_resource_t, __rlimits: *mut rlimit) -> libc::c_int;
    fn setrlimit(
        __resource: __rlimit_resource_t,
        __rlimits: *const rlimit,
    ) -> libc::c_int;
    fn prctl(__option: libc::c_int, _: ...) -> libc::c_int;
    fn mallopt(__param: libc::c_int, __val: libc::c_int) -> libc::c_int;
    fn malloc_trim(__pad: size_t) -> libc::c_int;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __rlim64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type __sig_atomic_t = libc::c_int;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
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
pub struct data_string {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
    pub ext: libc::c_int,
    pub value: buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_integer {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
    pub value: libc::c_int,
}
pub type socklen_t = __socklen_t;
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
pub type http_response_send_1xx_cb = Option::<
    unsafe extern "C" fn(*mut request_st, *mut connection) -> libc::c_int,
>;
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_15,
    pub _timer: C2RustUnnamed_14,
    pub _rt: C2RustUnnamed_13,
    pub _sigchld: C2RustUnnamed_12,
    pub _sigfault: C2RustUnnamed_9,
    pub _sigpoll: C2RustUnnamed_8,
    pub _sigsys: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub _addr_bnd: C2RustUnnamed_11,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_16,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
pub type __rlimit_resource = libc::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type rlim_t = __rlim64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = __rlimit_resource;
#[inline]
unsafe extern "C" fn buffer_clear(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
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
unsafe extern "C" fn chunkqueue_is_empty(mut cq: *const chunkqueue) -> libc::c_int {
    return (0 as *mut libc::c_void as *mut chunk == (*cq).first) as libc::c_int;
}
#[inline]
unsafe extern "C" fn sock_addr_get_family(mut saddr: *const sock_addr) -> libc::c_int {
    return (*saddr).plain.sa_family as libc::c_int;
}
static mut default_server_tag: buffer = buffer {
    ptr: 0 as *const libc::c_char as *mut libc::c_char,
    used: 0,
    size: 0,
};
static mut malloc_trim_fn: Option::<unsafe extern "C" fn(size_t) -> libc::c_int> = None;
static mut malloc_top_pad: size_t = 0;
static mut oneshot_fd: libc::c_int = 0 as libc::c_int;
static mut oneshot_fdout: libc::c_int = -(1 as libc::c_int);
static mut oneshot_fdn: *mut fdnode = 0 as *const fdnode as *mut fdnode;
static mut oneshot_read_cq: Option::<
    unsafe extern "C" fn(*mut connection, *mut chunkqueue, off_t) -> libc::c_int,
> = None;
static mut pid_fd: libc::c_int = -(2 as libc::c_int);
static mut graceful_sockets: server_socket_array = server_socket_array {
    ptr: 0 as *const *mut server_socket as *mut *mut server_socket,
    size: 0,
    used: 0,
};
static mut inherited_sockets: server_socket_array = server_socket_array {
    ptr: 0 as *const *mut server_socket as *mut *mut server_socket,
    size: 0,
    used: 0,
};
static mut graceful_restart: sig_atomic_t = 0 as libc::c_int;
static mut graceful_shutdown: sig_atomic_t = 0 as libc::c_int;
static mut srv_shutdown: sig_atomic_t = 0 as libc::c_int;
static mut handle_sig_child: sig_atomic_t = 0 as libc::c_int;
static mut handle_sig_alarm: sig_atomic_t = 1 as libc::c_int;
static mut handle_sig_hup: sig_atomic_t = 0 as libc::c_int;
static mut idle_limit: libc::c_int = 0 as libc::c_int;
static mut last_sigterm_info: siginfo_t = siginfo_t {
    si_signo: 0,
    si_errno: 0,
    si_code: 0,
    __pad0: 0,
    _sifields: C2RustUnnamed_6 { _pad: [0; 28] },
};
static mut last_sighup_info: siginfo_t = siginfo_t {
    si_signo: 0,
    si_errno: 0,
    si_code: 0,
    __pad0: 0,
    _sifields: C2RustUnnamed_6 { _pad: [0; 28] },
};
unsafe extern "C" fn sigaction_handler(
    mut sig: libc::c_int,
    mut si: *mut siginfo_t,
    mut context: *mut libc::c_void,
) {
    static mut empty_siginfo: siginfo_t = siginfo_t {
        si_signo: 0,
        si_errno: 0,
        si_code: 0,
        __pad0: 0,
        _sifields: C2RustUnnamed_6 { _pad: [0; 28] },
    };
    if si.is_null() {
        let ref mut fresh0 = *(&mut si as *mut *mut siginfo_t as *mut *const siginfo_t);
        *fresh0 = &empty_siginfo;
    }
    match sig {
        15 => {
            ::std::ptr::write_volatile(
                &mut srv_shutdown as *mut sig_atomic_t,
                1 as libc::c_int,
            );
            ::std::ptr::write_volatile(&mut last_sigterm_info as *mut siginfo_t, *si);
        }
        10 => {
            if graceful_shutdown == 0 {
                ::std::ptr::write_volatile(
                    &mut graceful_restart as *mut sig_atomic_t,
                    1 as libc::c_int,
                );
                ::std::ptr::write_volatile(
                    &mut graceful_shutdown as *mut sig_atomic_t,
                    1 as libc::c_int,
                );
                ::std::ptr::write_volatile(
                    &mut last_sigterm_info as *mut siginfo_t,
                    *si,
                );
            }
        }
        2 => {
            if graceful_shutdown != 0 {
                if 2 as libc::c_int == graceful_restart {
                    ::std::ptr::write_volatile(
                        &mut graceful_restart as *mut sig_atomic_t,
                        1 as libc::c_int,
                    );
                } else {
                    ::std::ptr::write_volatile(
                        &mut srv_shutdown as *mut sig_atomic_t,
                        1 as libc::c_int,
                    );
                }
            } else {
                ::std::ptr::write_volatile(
                    &mut graceful_shutdown as *mut sig_atomic_t,
                    1 as libc::c_int,
                );
            }
            ::std::ptr::write_volatile(&mut last_sigterm_info as *mut siginfo_t, *si);
        }
        14 => {
            ::std::ptr::write_volatile(
                &mut handle_sig_alarm as *mut sig_atomic_t,
                1 as libc::c_int,
            );
        }
        1 => {
            ::std::ptr::write_volatile(
                &mut handle_sig_hup as *mut sig_atomic_t,
                1 as libc::c_int,
            );
            ::std::ptr::write_volatile(&mut last_sighup_info as *mut siginfo_t, *si);
        }
        17 => {
            ::std::ptr::write_volatile(
                &mut handle_sig_child as *mut sig_atomic_t,
                1 as libc::c_int,
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn daemonize() -> libc::c_int {
    let mut pipefd: [libc::c_int; 2] = [0; 2];
    let mut pid: pid_t = 0;
    signal(
        22 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    signal(
        21 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    signal(
        20 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    if fdevent_pipe_cloexec(pipefd.as_mut_ptr(), 64 as libc::c_int as libc::c_uint)
        < 0 as libc::c_int
    {
        exit(-(1 as libc::c_int));
    }
    pid = fork();
    if 0 as libc::c_int > pid {
        exit(-(1 as libc::c_int));
    }
    if (0 as libc::c_int) < pid {
        let mut buf: libc::c_char = 0;
        let mut bytes: ssize_t = 0;
        close(pipefd[1 as libc::c_int as usize]);
        loop {
            bytes = read(
                pipefd[0 as libc::c_int as usize],
                &mut buf as *mut libc::c_char as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
            );
            if !(bytes < 0 as libc::c_int as libc::c_long
                && 4 as libc::c_int == *__errno_location())
            {
                break;
            }
        }
        close(pipefd[0 as libc::c_int as usize]);
        if bytes <= 0 as libc::c_int as libc::c_long {
            fputs(
                b"daemonized server failed to start; check error log for details\n\0"
                    as *const u8 as *const libc::c_char,
                stderr,
            );
            exit(-(1 as libc::c_int));
        }
        exit(0 as libc::c_int);
    }
    close(pipefd[0 as libc::c_int as usize]);
    if -(1 as libc::c_int) == setsid() {
        exit(0 as libc::c_int);
    }
    signal(
        1 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    if 0 as libc::c_int != fork() {
        exit(0 as libc::c_int);
    }
    if 0 as libc::c_int != chdir(b"/\0" as *const u8 as *const libc::c_char) {
        exit(0 as libc::c_int);
    }
    return pipefd[1 as libc::c_int as usize];
}
static mut clockid_mono_coarse: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn server_monotonic_secs() -> unix_time64_t {
    let mut ts: unix_timespec64_t = unix_timespec64_t {
        tv_sec: 0,
        tv_nsec: 0,
    };
    return if 0 as libc::c_int == clock_gettime(clockid_mono_coarse, &mut ts) {
        ts.tv_sec
    } else {
        log_monotonic_secs
    };
}
unsafe extern "C" fn server_epoch_secs(
    srv: *mut server,
    mut mono_ts_delta: unix_time64_t,
) -> unix_time64_t {
    let cur_ts: unix_time64_t = log_epoch_secs;
    let new_ts: unix_time64_t = time(0 as *mut time_t);
    let new_ts_adj: unix_time64_t = new_ts - mono_ts_delta;
    if new_ts_adj < cur_ts || new_ts_adj - cur_ts > 300 as libc::c_int as libc::c_long {
        log_error(
            (*srv).errh,
            b"src/server.c\0" as *const u8 as *const libc::c_char,
            263 as libc::c_int as libc::c_uint,
            b"warning: clock jumped %lld secs\0" as *const u8 as *const libc::c_char,
            (new_ts_adj - cur_ts) as libc::c_longlong,
        );
        let mut delta: libc::c_int = config_feature_int(
            srv,
            b"server.clock-jump-restart\0" as *const u8 as *const libc::c_char,
            1800 as libc::c_int,
        );
        if delta != 0
            && (if new_ts_adj > cur_ts {
                new_ts_adj - cur_ts
            } else {
                cur_ts - new_ts_adj
            }) > delta as libc::c_long
        {
            log_error(
                (*srv).errh,
                b"src/server.c\0" as *const u8 as *const libc::c_char,
                271 as libc::c_int as libc::c_uint,
                b"attempting graceful restart in < ~5 seconds, else hard restart\0"
                    as *const u8 as *const libc::c_char,
            );
            (*srv)
                .graceful_expire_ts = log_monotonic_secs
                + 5 as libc::c_int as libc::c_long;
            raise(10 as libc::c_int);
        }
    }
    return new_ts;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn server_init() -> *mut server {
    let mut srv: *mut server = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<server>() as libc::c_ulong,
    ) as *mut server;
    if srv.is_null() {
        ck_assert_failed(
            b"src/server.c\0" as *const u8 as *const libc::c_char,
            285 as libc::c_int as libc::c_uint,
            b"srv\0" as *const u8 as *const libc::c_char,
        );
    }
    (*srv).tmp_buf = buffer_init();
    strftime_cache_reset();
    li_rand_reseed();
    log_epoch_secs = time(0 as *mut time_t);
    (*srv).startup_ts = log_epoch_secs;
    let mut ts: unix_timespec64_t = unix_timespec64_t {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if 0 as libc::c_int == clock_gettime(6 as libc::c_int, &mut ts) {
        clockid_mono_coarse = 6 as libc::c_int;
    } else if 0 as libc::c_int == clock_gettime(4 as libc::c_int, &mut ts) {
        clockid_mono_coarse = 4 as libc::c_int;
    } else {
        clockid_mono_coarse = 1 as libc::c_int;
    }
    log_monotonic_secs = server_monotonic_secs();
    (*srv).errh = log_set_global_errh(0 as *mut log_error_st, 0 as libc::c_int);
    config_init(srv);
    (*srv)
        .request_env = Some(
        plugins_call_handle_request_env
            as unsafe extern "C" fn(*mut request_st) -> handler_t,
    );
    (*srv).loadavg[0 as libc::c_int as usize] = 0.0f64;
    (*srv).loadavg[1 as libc::c_int as usize] = 0.0f64;
    (*srv).loadavg[2 as libc::c_int as usize] = 0.0f64;
    (*srv).stdin_fd = -(1 as libc::c_int);
    log_con_jqueue = &mut log_con_jqueue as *mut *mut connection as uintptr_t
        as *mut connection;
    return srv;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn server_free(mut srv: *mut server) {
    if oneshot_fd > 0 as libc::c_int {
        if !oneshot_fdn.is_null() {
            fdevent_fdnode_event_del((*srv).ev, oneshot_fdn);
            fdevent_unregister((*srv).ev, oneshot_fd);
            oneshot_fdn = 0 as *mut fdnode;
        }
        close(oneshot_fd);
    }
    if oneshot_fdout >= 0 as libc::c_int {
        close(oneshot_fdout);
    }
    if (*srv).stdin_fd >= 0 as libc::c_int {
        close((*srv).stdin_fd);
    }
    buffer_free((*srv).tmp_buf);
    fdevent_free((*srv).ev);
    config_free(srv);
    stat_cache_free();
    li_rand_cleanup();
    chunkqueue_chunk_pool_free();
    if (*srv).errh != log_set_global_errh(0 as *mut log_error_st, 0 as libc::c_int) {
        fdlog_free((*srv).errh);
    }
    free(srv as *mut libc::c_void);
}
#[cold]
#[inline(never)]
unsafe extern "C" fn remove_pid_file(mut srv: *mut server) {
    if pid_fd <= -(2 as libc::c_int) {
        return;
    }
    if !((*srv).srvconf.pid_file).is_null() && 0 as libc::c_int <= pid_fd {
        if 0 as libc::c_int != ftruncate(pid_fd, 0 as libc::c_int as __off64_t) {
            log_perror(
                (*srv).errh,
                b"src/server.c\0" as *const u8 as *const libc::c_char,
                372 as libc::c_int as libc::c_uint,
                b"ftruncate failed for: %s\0" as *const u8 as *const libc::c_char,
                (*(*srv).srvconf.pid_file).ptr,
            );
        }
    }
    if 0 as libc::c_int <= pid_fd {
        close(pid_fd);
        ::std::ptr::write_volatile(&mut pid_fd as *mut libc::c_int, -(1 as libc::c_int));
    }
    if !((*srv).srvconf.pid_file).is_null() && ((*srv).srvconf.changeroot).is_null() {
        if 0 as libc::c_int != unlink((*(*srv).srvconf.pid_file).ptr) {
            if *__errno_location() != 13 as libc::c_int
                && *__errno_location() != 1 as libc::c_int
            {
                log_perror(
                    (*srv).errh,
                    b"src/server.c\0" as *const u8 as *const libc::c_char,
                    383 as libc::c_int as libc::c_uint,
                    b"unlink failed for: %s\0" as *const u8 as *const libc::c_char,
                    (*(*srv).srvconf.pid_file).ptr,
                );
            }
        }
    }
}
#[cold]
unsafe extern "C" fn server_oneshot_getsock(
    mut srv: *mut server,
    mut cnt_addr: *mut sock_addr,
) -> *mut server_socket {
    let mut srv_socket: *mut server_socket = 0 as *mut server_socket;
    let mut srv_socket_wild: *mut server_socket = 0 as *mut server_socket;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*srv).srv_sockets.used {
        srv_socket = *((*srv).srv_sockets.ptr).offset(i as isize);
        if !(sock_addr_is_port_eq(&mut (*srv_socket).addr, cnt_addr) == 0) {
            if sock_addr_is_addr_eq(&mut (*srv_socket).addr, cnt_addr) != 0 {
                return srv_socket;
            }
            if srv_socket_wild.is_null() {
                if sock_addr_is_addr_wildcard(&mut (*srv_socket).addr) != 0 {
                    srv_socket_wild = srv_socket;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if !srv_socket_wild.is_null() {
        return srv_socket_wild
    } else if (*srv).srv_sockets.used != 0 {
        return *((*srv).srv_sockets.ptr).offset(0 as libc::c_int as isize)
    } else {
        log_error(
            (*srv).errh,
            b"src/server.c\0" as *const u8 as *const libc::c_char,
            410 as libc::c_int as libc::c_uint,
            b"no sockets configured\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut server_socket;
    };
}
unsafe extern "C" fn server_oneshot_read_cq(
    mut con: *mut connection,
    mut cq: *mut chunkqueue,
    mut max_bytes: off_t,
) -> libc::c_int {
    let mut fd: libc::c_int = (*con).fd;
    (*con).fd = (*oneshot_fdn).fd;
    let mut rc: libc::c_int = oneshot_read_cq
        .expect("non-null function pointer")(con, cq, max_bytes);
    (*con).fd = fd;
    let events: libc::c_int = if !oneshot_fdn.is_null() {
        (*oneshot_fdn).events
    } else {
        0 as libc::c_int
    };
    let mut n: libc::c_int = if (*con).is_readable as libc::c_int > 0 as libc::c_int {
        0 as libc::c_int
    } else {
        0x1 as libc::c_int
    };
    if events & 0x2000 as libc::c_int != 0 {
        n |= 0x2000 as libc::c_int;
    }
    fdevent_fdnode_event_set((*(*con).srv).ev, oneshot_fdn, n);
    return rc;
}
unsafe extern "C" fn server_oneshot_handle_fdevent(
    mut context: *mut libc::c_void,
    mut revents: libc::c_int,
) -> handler_t {
    let mut con: *mut connection = context as *mut connection;
    let mut rdhup: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = if !oneshot_fdn.is_null() {
        (*oneshot_fdn).events
    } else {
        0 as libc::c_int
    };
    if revents & 0x1 as libc::c_int != 0 {
        n &= !(0x1 as libc::c_int);
    }
    let r: *mut request_st = &mut (*con).request;
    if (*r).state as libc::c_uint != CON_STATE_ERROR as libc::c_int as libc::c_uint
        && revents & (0x10 as libc::c_int | 0x2000 as libc::c_int) != 0
    {
        revents &= !(0x10 as libc::c_int | 0x2000 as libc::c_int);
        n &= !(0x1 as libc::c_int | 0x2000 as libc::c_int);
        rdhup = 1 as libc::c_int;
    }
    fdevent_fdnode_event_set((*(*con).srv).ev, oneshot_fdn, n);
    let fdn: *mut fdnode = (*con).fdn;
    let mut rc: handler_t = (if !fdn.is_null() && ((*fdn).handler).is_some() {
        (Some(((*fdn).handler).expect("non-null function pointer")))
            .expect("non-null function pointer")(con as *mut libc::c_void, revents)
            as libc::c_uint
    } else {
        HANDLER_FINISHED as libc::c_int as libc::c_uint
    }) as handler_t;
    if rdhup != 0 {
        (*r)
            .conf
            .stream_request_body = ((*r).conf.stream_request_body as libc::c_int
            & !((1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 15 as libc::c_int)) as libc::c_ushort;
        (*r)
            .conf
            .stream_request_body = ((*r).conf.stream_request_body as libc::c_int
            | (1 as libc::c_int) << 12 as libc::c_int) as libc::c_ushort;
        (*r)
            .conf
            .stream_request_body = ((*r).conf.stream_request_body as libc::c_int
            | (1 as libc::c_int) << 13 as libc::c_int) as libc::c_ushort;
        (*con).is_readable = 1 as libc::c_int as libc::c_schar;
        if chunkqueue_is_empty((*con).read_queue) != 0 {
            (*r).keep_alive = 0 as libc::c_int as int8_t;
        }
        if (*r).reqbody_length < -(1 as libc::c_int) as libc::c_long {
            (*r).reqbody_length = (*r).reqbody_queue.bytes_in;
        }
    }
    return rc;
}
#[cold]
unsafe extern "C" fn server_oneshot_init_pipe(
    mut srv: *mut server,
    mut fdin: libc::c_int,
    mut fdout: libc::c_int,
) -> libc::c_int {
    let mut con: *mut connection = 0 as *mut connection;
    let mut srv_socket: *const server_socket = 0 as *const server_socket;
    let mut cnt_addr: sock_addr = sock_addr {
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
    let ncat: *const libc::c_char = getenv(
        b"NCAT_LOCAL_ADDR\0" as *const u8 as *const libc::c_char,
    );
    let ncat_local_addr: *const libc::c_char = if !ncat.is_null() {
        ncat
    } else {
        b"127.0.0.1\0" as *const u8 as *const libc::c_char
    };
    let ncat_local_port: *const libc::c_char = if !ncat.is_null() {
        getenv(b"NCAT_LOCAL_PORT\0" as *const u8 as *const libc::c_char)
            as *const libc::c_char
    } else {
        b"80\0" as *const u8 as *const libc::c_char
    };
    let ncat_remote_addr: *const libc::c_char = if !ncat.is_null() {
        getenv(b"NCAT_REMOTE_ADDR\0" as *const u8 as *const libc::c_char)
            as *const libc::c_char
    } else {
        b"127.0.0.1\0" as *const u8 as *const libc::c_char
    };
    let ncat_remote_port: *const libc::c_char = if !ncat.is_null() {
        getenv(b"NCAT_REMOTE_PORT\0" as *const u8 as *const libc::c_char)
            as *const libc::c_char
    } else {
        b"48080\0" as *const u8 as *const libc::c_char
    };
    if ncat_local_addr.is_null() || ncat_local_port.is_null() {
        return 0 as libc::c_int;
    }
    if ncat_remote_addr.is_null() || ncat_remote_port.is_null() {
        return 0 as libc::c_int;
    }
    let family: libc::c_int = if !ncat.is_null()
        && !(strchr(ncat_local_addr, ':' as i32)).is_null()
    {
        10 as libc::c_int
    } else {
        2 as libc::c_int
    };
    let mut port: libc::c_ushort = 0;
    port = strtol(ncat_local_port, 0 as *mut *mut libc::c_char, 10 as libc::c_int)
        as libc::c_ushort;
    if 1 as libc::c_int
        != sock_addr_inet_pton(&mut cnt_addr, ncat_local_addr, family, port)
    {
        log_error(
            (*srv).errh,
            b"src/server.c\0" as *const u8 as *const libc::c_char,
            508 as libc::c_int as libc::c_uint,
            b"invalid local addr\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    srv_socket = server_oneshot_getsock(srv, &mut cnt_addr);
    if srv_socket.is_null() {
        return 0 as libc::c_int;
    }
    port = strtol(ncat_remote_port, 0 as *mut *mut libc::c_char, 10 as libc::c_int)
        as libc::c_ushort;
    if 1 as libc::c_int
        != sock_addr_inet_pton(&mut cnt_addr, ncat_remote_addr, family, port)
    {
        log_error(
            (*srv).errh,
            b"src/server.c\0" as *const u8 as *const libc::c_char,
            517 as libc::c_int as libc::c_uint,
            b"invalid remote addr\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if -(1 as libc::c_int) == fdevent_fcntl_set_nb_cloexec(fdin) {
        log_perror(
            (*srv).errh,
            b"src/server.c\0" as *const u8 as *const libc::c_char,
            523 as libc::c_int as libc::c_uint,
            b"fcntl()\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if -(1 as libc::c_int) == fdevent_fcntl_set_nb_cloexec(fdout) {
        log_perror(
            (*srv).errh,
            b"src/server.c\0" as *const u8 as *const libc::c_char,
            527 as libc::c_int as libc::c_uint,
            b"fcntl()\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    con = connection_accepted(srv, srv_socket, &mut cnt_addr, fdout);
    if con.is_null() {
        return 0 as libc::c_int;
    }
    oneshot_read_cq = (*con).network_read;
    (*con)
        .network_read = Some(
        server_oneshot_read_cq
            as unsafe extern "C" fn(
                *mut connection,
                *mut chunkqueue,
                off_t,
            ) -> libc::c_int,
    );
    oneshot_fdn = fdevent_register(
        (*srv).ev,
        fdin,
        Some(
            server_oneshot_handle_fdevent
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> handler_t,
        ),
        con as *mut libc::c_void,
    );
    fdevent_fdnode_event_set((*srv).ev, oneshot_fdn, 0x2000 as libc::c_int);
    connection_state_machine(con);
    return 1 as libc::c_int;
}
#[cold]
unsafe extern "C" fn server_oneshot_init(
    mut srv: *mut server,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut con: *mut connection = 0 as *mut connection;
    let mut srv_socket: *const server_socket = 0 as *const server_socket;
    let mut cnt_addr: sock_addr = sock_addr {
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
    let mut cnt_len: socklen_t = 0;
    cnt_len = ::std::mem::size_of::<sock_addr>() as libc::c_ulong as socklen_t;
    if 0 as libc::c_int
        != getsockname(
            fd,
            __SOCKADDR_ARG {
                __sockaddr__: &mut cnt_addr as *mut sock_addr as *mut sockaddr,
            },
            &mut cnt_len,
        )
    {
        log_perror(
            (*srv).errh,
            b"src/server.c\0" as *const u8 as *const libc::c_char,
            565 as libc::c_int as libc::c_uint,
            b"getsockname()\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    srv_socket = server_oneshot_getsock(srv, &mut cnt_addr);
    if srv_socket.is_null() {
        return 0 as libc::c_int;
    }
    cnt_len = ::std::mem::size_of::<sock_addr>() as libc::c_ulong as socklen_t;
    if 0 as libc::c_int
        != getpeername(
            fd,
            __SOCKADDR_ARG {
                __sockaddr__: &mut cnt_addr as *mut sock_addr as *mut sockaddr,
            },
            &mut cnt_len,
        )
    {
        log_perror(
            (*srv).errh,
            b"src/server.c\0" as *const u8 as *const libc::c_char,
            577 as libc::c_int as libc::c_uint,
            b"getpeername()\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if -(1 as libc::c_int) == fdevent_fcntl_set_nb_cloexec(fd) {
        log_perror(
            (*srv).errh,
            b"src/server.c\0" as *const u8 as *const libc::c_char,
            583 as libc::c_int as libc::c_uint,
            b"fcntl()\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if sock_addr_get_family(&mut cnt_addr) != 1 as libc::c_int {
        network_accept_tcp_nagle_disable(fd);
    }
    con = connection_accepted(srv, srv_socket, &mut cnt_addr, fd);
    if con.is_null() {
        return 0 as libc::c_int;
    }
    connection_state_machine(con);
    return 1 as libc::c_int;
}
#[cold]
unsafe extern "C" fn show_version() {
    let mut b: *mut libc::c_char = b"lighttpd/1.4.64 - a light and fast webserver\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    write_all(1 as libc::c_int, b as *const libc::c_void, strlen(b));
}
#[cold]
unsafe extern "C" fn show_features() {
    static mut features: [libc::c_char; 407] = unsafe {
        *::std::mem::transmute::<
            &[u8; 407],
            &[libc::c_char; 407],
        >(
            b"\nFeatures:\n\n\t+ IPv6 support\n\t- zlib support\n\t- zstd support\n\t- bzip2 support\n\t- brotli support\n\t+ crypt support\n\t- OpenSSL support\n\t- mbedTLS support\n\t- NSS crypto support\n\t- GnuTLS support\n\t- WolfSSL support\n\t- Nettle support\n\t+ PCRE support\n\t- MySQL support\n\t- PgSQL support\n\t- DBI support\n\t- Kerberos support\n\t- LDAP support\n\t- PAM support\n\t- FAM support\n\t- LUA support\n\t- xml support\n\t- SQLite support\n\0",
        )
    };
    show_version();
    printf(
        b"%s%s%s%s\n\0" as *const u8 as *const libc::c_char,
        fdevent_show_event_handlers(),
        network_write_show_handlers(),
        features.as_ptr(),
        if ::std::mem::size_of::<time_t>() as libc::c_ulong
            > 4 as libc::c_int as libc::c_ulong
            || ::std::mem::size_of::<time_t>() as libc::c_ulong
                == 4 as libc::c_int as libc::c_ulong
                && -(1 as libc::c_int) as time_t > 1 as libc::c_int as time_t
        {
            b"\t+ Y2038 support\n\0" as *const u8 as *const libc::c_char
        } else {
            b"\t- Y2038 support (unsafe 32-bit signed time_t)\n\0" as *const u8
                as *const libc::c_char
        },
    );
}
#[cold]
unsafe extern "C" fn show_help() {
    let mut b: *mut libc::c_char = b"lighttpd/1.4.64 - a light and fast webserver\nusage:\n -f <name>  filename of the config-file\n -m <name>  module directory (default: sconsbuild/install/lib)\n -i <secs>  graceful shutdown after <secs> of inactivity\n -1         process single (one) request on stdin socket, then exit\n -p         print the parsed config-file in internal form, and exit\n -t         test config-file syntax, then exit\n -tt        test config-file syntax, load and init modules, then exit\n -D         don't go to background (default: go to background)\n -v         show version\n -V         show compile-time features\n -h         show this help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    write_all(1 as libc::c_int, b as *const libc::c_void, strlen(b));
}
#[cold]
#[inline(never)]
unsafe extern "C" fn server_sockets_save(mut srv: *mut server) {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*srv).srv_sockets.used {
        let ref mut fresh1 = (**((*srv).srv_sockets.ptr).offset(i as isize)).srv;
        *fresh1 = 0 as *mut server;
        i = i.wrapping_add(1);
    }
    let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
    while i_0 < (*srv).srv_sockets_inherited.used {
        let ref mut fresh2 = (**((*srv).srv_sockets_inherited.ptr).offset(i_0 as isize))
            .srv;
        *fresh2 = 0 as *mut server;
        i_0 = i_0.wrapping_add(1);
    }
    memcpy(
        &mut graceful_sockets as *mut server_socket_array as *mut libc::c_void,
        &mut (*srv).srv_sockets as *mut server_socket_array as *const libc::c_void,
        ::std::mem::size_of::<server_socket_array>() as libc::c_ulong,
    );
    memset(
        &mut (*srv).srv_sockets as *mut server_socket_array as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<server_socket_array>() as libc::c_ulong,
    );
    memcpy(
        &mut inherited_sockets as *mut server_socket_array as *mut libc::c_void,
        &mut (*srv).srv_sockets_inherited as *mut server_socket_array
            as *const libc::c_void,
        ::std::mem::size_of::<server_socket_array>() as libc::c_ulong,
    );
    memset(
        &mut (*srv).srv_sockets_inherited as *mut server_socket_array
            as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<server_socket_array>() as libc::c_ulong,
    );
}
#[cold]
#[inline(never)]
unsafe extern "C" fn server_sockets_restore(mut srv: *mut server) {
    memcpy(
        &mut (*srv).srv_sockets as *mut server_socket_array as *mut libc::c_void,
        &mut graceful_sockets as *mut server_socket_array as *const libc::c_void,
        ::std::mem::size_of::<server_socket_array>() as libc::c_ulong,
    );
    memset(
        &mut graceful_sockets as *mut server_socket_array as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<server_socket_array>() as libc::c_ulong,
    );
    memcpy(
        &mut (*srv).srv_sockets_inherited as *mut server_socket_array
            as *mut libc::c_void,
        &mut inherited_sockets as *mut server_socket_array as *const libc::c_void,
        ::std::mem::size_of::<server_socket_array>() as libc::c_ulong,
    );
    memset(
        &mut inherited_sockets as *mut server_socket_array as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<server_socket_array>() as libc::c_ulong,
    );
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*srv).srv_sockets.used {
        let ref mut fresh3 = (**((*srv).srv_sockets.ptr).offset(i as isize)).srv;
        *fresh3 = srv;
        i = i.wrapping_add(1);
    }
    let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
    while i_0 < (*srv).srv_sockets_inherited.used {
        let ref mut fresh4 = (**((*srv).srv_sockets_inherited.ptr).offset(i_0 as isize))
            .srv;
        *fresh4 = srv;
        i_0 = i_0.wrapping_add(1);
    }
}
#[cold]
unsafe extern "C" fn server_sockets_set_nb_cloexec(mut srv: *mut server) -> libc::c_int {
    if (*srv).sockets_disabled != 0 {
        return 0 as libc::c_int;
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*srv).srv_sockets.used {
        let mut srv_socket: *mut server_socket = *((*srv).srv_sockets.ptr)
            .offset(i as isize);
        if -(1 as libc::c_int) == fdevent_fcntl_set_nb_cloexec_sock((*srv_socket).fd) {
            log_perror(
                (*srv).errh,
                b"src/server.c\0" as *const u8 as *const libc::c_char,
                796 as libc::c_int as libc::c_uint,
                b"fcntl()\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
#[cold]
unsafe extern "C" fn server_sockets_set_event(
    mut srv: *mut server,
    mut event: libc::c_int,
) {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*srv).srv_sockets.used {
        let mut srv_socket: *mut server_socket = *((*srv).srv_sockets.ptr)
            .offset(i as isize);
        fdevent_fdnode_event_set((*srv).ev, (*srv_socket).fdn, event);
        i = i.wrapping_add(1);
    }
}
#[cold]
unsafe extern "C" fn server_sockets_unregister(mut srv: *mut server) {
    if 2 as libc::c_int == (*srv).sockets_disabled {
        return;
    }
    (*srv).sockets_disabled = 2 as libc::c_int;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*srv).srv_sockets.used {
        network_unregister_sock(srv, *((*srv).srv_sockets.ptr).offset(i as isize));
        i = i.wrapping_add(1);
    }
}
#[cold]
unsafe extern "C" fn server_sockets_close(mut srv: *mut server) {
    if 3 as libc::c_int == (*srv).sockets_disabled {
        return;
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*srv).srv_sockets.used {
        let mut srv_socket: *mut server_socket = *((*srv).srv_sockets.ptr)
            .offset(i as isize);
        if !(-(1 as libc::c_int) == (*srv_socket).fd) {
            if 2 as libc::c_int != (*srv).sockets_disabled {
                network_unregister_sock(srv, srv_socket);
            }
            close((*srv_socket).fd);
            (*srv_socket).fd = -(1 as libc::c_int);
        }
        i = i.wrapping_add(1);
    }
    (*srv).sockets_disabled = 3 as libc::c_int;
}
#[cold]
unsafe extern "C" fn server_graceful_signal_prev_generation() {
    let prev_gen: *const libc::c_char = getenv(
        b"LIGHTTPD_PREV_GEN\0" as *const u8 as *const libc::c_char,
    );
    if prev_gen.is_null() {
        return;
    }
    let mut pid: pid_t = strtol(prev_gen, 0 as *mut *mut libc::c_char, 10 as libc::c_int)
        as pid_t;
    unsetenv(b"LIGHTTPD_PREV_GEN\0" as *const u8 as *const libc::c_char);
    if pid <= 0 as libc::c_int {
        return;
    }
    if pid == fdevent_waitpid(pid, 0 as *mut libc::c_int, 1 as libc::c_int) {
        return;
    }
    kill(pid, 2 as libc::c_int);
}
#[cold]
unsafe extern "C" fn server_graceful_state_bg(mut srv: *mut server) -> libc::c_int {
    if srv_shutdown != 0 {
        return 0 as libc::c_int;
    }
    if config_feature_bool(
        srv,
        b"server.graceful-restart-bg\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    let du: *mut data_unset = array_get_data_unset(
        (*srv).srvconf.feature_flags,
        b"server.graceful-restart-bg\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if (*du).type_0 as libc::c_uint == TYPE_STRING as libc::c_int as libc::c_uint {
        buffer_copy_string_len(
            &mut (*(du as *mut data_string)).value,
            b"false\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    } else {
        (*(du as *mut data_integer)).value = 0 as libc::c_int;
    }
    let argv: *mut *mut libc::c_char = (*srv).argv;
    if if 0 as libc::c_int == (*srv).srvconf.dont_daemonize as libc::c_int {
        (*(*argv.offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int != '/' as i32) as libc::c_int
    } else {
        (0 as *mut libc::c_void as *mut libc::c_char
            == strchr(*argv.offset(0 as libc::c_int as isize), '/' as i32))
            as libc::c_int
    } != 0
    {
        return 0 as libc::c_int;
    }
    plugins_call_handle_sighup(srv);
    fdlog_files_cycle((*srv).errh);
    let mut pid: pid_t = fork();
    if pid != 0 {
        if pid < 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        network_socket_activation_to_env(srv);
        server_graceful_signal_prev_generation();
        if 0 as libc::c_int == (*srv).srvconf.max_worker as libc::c_int {
            let tb: *mut buffer = (*srv).tmp_buf;
            buffer_clear(tb);
            buffer_append_int(tb, pid as intmax_t);
            setenv(
                b"LIGHTTPD_PREV_GEN\0" as *const u8 as *const libc::c_char,
                (*tb).ptr,
                1 as libc::c_int,
            );
        }
        execv(*argv.offset(0 as libc::c_int as isize), argv as *const *mut libc::c_char);
        _exit(1 as libc::c_int);
    }
    if 0 as libc::c_int != (*srv).srvconf.max_worker as libc::c_int {
        server_sockets_close(srv);
    }
    if 0 as libc::c_int <= pid_fd {
        close(pid_fd);
        ::std::ptr::write_volatile(&mut pid_fd as *mut libc::c_int, -(1 as libc::c_int));
    }
    (*srv).srvconf.pid_file = 0 as *mut buffer;
    log_error(
        (*srv).errh,
        b"src/server.c\0" as *const u8 as *const libc::c_char,
        968 as libc::c_int as libc::c_uint,
        b"[note] pid %lld continuing to handle %u connection(s) in progress\0"
            as *const u8 as *const libc::c_char,
        getpid() as libc::c_longlong,
        ((*srv).srvconf.max_conns as libc::c_uint).wrapping_sub((*srv).lim_conns),
    );
    if 0 as libc::c_int == (*srv).srvconf.max_worker as libc::c_int {
        (*srv).graceful_expire_ts = 0 as libc::c_int as unix_time64_t;
        ::std::ptr::write_volatile(
            &mut graceful_shutdown as *mut sig_atomic_t,
            0 as libc::c_int,
        );
    }
    ::std::ptr::write_volatile(
        &mut graceful_restart as *mut sig_atomic_t,
        0 as libc::c_int,
    );
    return 1 as libc::c_int;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn server_graceful_shutdown_maint(mut srv: *mut server) {
    if oneshot_fd != 0 {
        if (*srv).graceful_expire_ts == 0 {
            return;
        }
        if (*srv).graceful_expire_ts >= log_monotonic_secs {
            return;
        }
    }
    connection_graceful_shutdown_maint(srv);
}
#[cold]
#[inline(never)]
unsafe extern "C" fn server_graceful_state(mut srv: *mut server) {
    if srv_shutdown == 0 {
        if 0 as libc::c_int as libc::c_long == (*srv).graceful_expire_ts {
            (*srv)
                .graceful_expire_ts = config_feature_int(
                srv,
                b"server.graceful-shutdown-timeout\0" as *const u8
                    as *const libc::c_char,
                8 as libc::c_int,
            ) as unix_time64_t;
            if (*srv).graceful_expire_ts != 0 {
                (*srv).graceful_expire_ts += log_monotonic_secs;
            }
        }
        server_graceful_shutdown_maint(srv);
    }
    if 2 as libc::c_int == (*srv).sockets_disabled
        || 3 as libc::c_int == (*srv).sockets_disabled
    {
        if oneshot_fd != 0 {
            ::std::ptr::write_volatile(
                &mut graceful_restart as *mut sig_atomic_t,
                0 as libc::c_int,
            );
        }
        return;
    }
    log_error(
        (*srv).errh,
        b"src/server.c\0" as *const u8 as *const libc::c_char,
        1011 as libc::c_int as libc::c_uint,
        b"[note] graceful shutdown started\0" as *const u8 as *const libc::c_char,
    );
    if !((*srv).srvconf.changeroot).is_null() || oneshot_fd != 0
        || 2 as libc::c_int == graceful_shutdown
    {
        ::std::ptr::write_volatile(
            &mut graceful_restart as *mut sig_atomic_t,
            0 as libc::c_int,
        );
    }
    if graceful_restart != 0 {
        if server_graceful_state_bg(srv) == 0 {
            server_sockets_unregister(srv);
        }
        if pid_fd > 0 as libc::c_int {
            ::std::ptr::write_volatile(&mut pid_fd as *mut libc::c_int, -pid_fd);
        }
    } else {
        server_sockets_close(srv);
        remove_pid_file(srv);
        (*srv).srvconf.pid_file = 0 as *mut buffer;
    };
}
#[cold]
#[inline(never)]
unsafe extern "C" fn server_sockets_enable(mut srv: *mut server) {
    server_sockets_set_event(srv, 0x1 as libc::c_int);
    (*srv).sockets_disabled = 0 as libc::c_int;
    log_error(
        (*srv).errh,
        b"src/server.c\0" as *const u8 as *const libc::c_char,
        1035 as libc::c_int as libc::c_uint,
        b"[note] sockets enabled again\0" as *const u8 as *const libc::c_char,
    );
}
#[cold]
#[inline(never)]
unsafe extern "C" fn server_sockets_disable(mut srv: *mut server) {
    server_sockets_set_event(srv, 0 as libc::c_int);
    (*srv).sockets_disabled = 1 as libc::c_int;
    log_error(
        (*srv).errh,
        b"src/server.c\0" as *const u8 as *const libc::c_char,
        1043 as libc::c_int as libc::c_uint,
        if 0 as libc::c_int as libc::c_uint == (*srv).lim_conns {
            b"[note] sockets disabled, connection limit reached\0" as *const u8
                as *const libc::c_char
        } else {
            b"[note] sockets disabled, out-of-fds\0" as *const u8 as *const libc::c_char
        },
    );
}
#[cold]
unsafe extern "C" fn server_overload_check(mut srv: *mut server) {
    if (*srv).cur_fds < (*srv).max_fds_lowat
        && 0 as libc::c_int as libc::c_uint != (*srv).lim_conns
    {
        server_sockets_enable(srv);
    }
}
unsafe extern "C" fn server_load_check(mut srv: *mut server) {
    if (*srv).cur_fds > (*srv).max_fds_hiwat
        || 0 as libc::c_int as libc::c_uint == (*srv).lim_conns
    {
        server_sockets_disable(srv);
    }
}
#[cold]
#[inline(never)]
unsafe extern "C" fn server_main_setup(
    srv: *mut server,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut print_config: libc::c_int = 0 as libc::c_int;
    let mut test_config: libc::c_int = 0 as libc::c_int;
    let mut i_am_root: libc::c_int = 0 as libc::c_int;
    let mut o: libc::c_int = 0;
    let mut num_childs: libc::c_int = 0 as libc::c_int;
    let mut i: uint32_t = 0;
    let mut act: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_16 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut parent_pipe_fd: libc::c_int = -(1 as libc::c_int);
    i_am_root = (0 as libc::c_int as libc::c_uint == getuid()) as libc::c_int;
    oneshot_fd = 0 as libc::c_int;
    oneshot_fdout = -(1 as libc::c_int);
    ::std::ptr::write_volatile(&mut srv_shutdown as *mut sig_atomic_t, 0 as libc::c_int);
    ::std::ptr::write_volatile(
        &mut graceful_shutdown as *mut sig_atomic_t,
        0 as libc::c_int,
    );
    ::std::ptr::write_volatile(
        &mut handle_sig_alarm as *mut sig_atomic_t,
        1 as libc::c_int,
    );
    ::std::ptr::write_volatile(
        &mut handle_sig_hup as *mut sig_atomic_t,
        0 as libc::c_int,
    );
    idle_limit = 0 as libc::c_int;
    chunkqueue_set_tempdirs_default_reset();
    (*srv).argv = argv;
    loop {
        o = getopt(argc, argv, b"f:m:i:hvVD1pt\0" as *const u8 as *const libc::c_char);
        if !(-(1 as libc::c_int) != o) {
            break;
        }
        match o {
            102 => {
                if !((*srv).config_data_base).is_null() {
                    log_error(
                        (*srv).errh,
                        b"src/server.c\0" as *const u8 as *const libc::c_char,
                        1104 as libc::c_int as libc::c_uint,
                        b"Can only read one config file. Use the include command to use multiple config files.\0"
                            as *const u8 as *const libc::c_char,
                    );
                    return -(1 as libc::c_int);
                }
                if config_read(srv, optarg) != 0 {
                    return -(1 as libc::c_int);
                }
            }
            109 => {
                (*srv).srvconf.modules_dir = optarg;
            }
            105 => {
                let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut timeout: libc::c_long = strtol(
                    optarg,
                    &mut endptr,
                    0 as libc::c_int,
                );
                if *optarg == 0 || *endptr as libc::c_int != 0
                    || timeout < 0 as libc::c_int as libc::c_long
                {
                    log_error(
                        (*srv).errh,
                        b"src/server.c\0" as *const u8 as *const libc::c_char,
                        1119 as libc::c_int as libc::c_uint,
                        b"Invalid idle timeout value: %s\0" as *const u8
                            as *const libc::c_char,
                        optarg,
                    );
                    return -(1 as libc::c_int);
                }
                idle_limit = timeout as libc::c_int;
            }
            112 => {
                print_config = 1 as libc::c_int;
            }
            116 => {
                test_config += 1;
            }
            49 => {
                if 0 as libc::c_int == oneshot_fd {
                    oneshot_fd = dup(0 as libc::c_int);
                }
            }
            68 => {
                (*srv).srvconf.dont_daemonize = 1 as libc::c_int as libc::c_uchar;
            }
            118 => {
                show_version();
                return 0 as libc::c_int;
            }
            86 => {
                show_features();
                return 0 as libc::c_int;
            }
            104 => {
                show_help();
                return 0 as libc::c_int;
            }
            _ => {
                show_help();
                return -(1 as libc::c_int);
            }
        }
    }
    if ((*srv).config_data_base).is_null() {
        log_error(
            (*srv).errh,
            b"src/server.c\0" as *const u8 as *const libc::c_char,
            1154 as libc::c_int as libc::c_uint,
            b"No configuration available. Try using -f option.\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if print_config != 0 {
        config_print(srv);
        puts((*(*srv).tmp_buf).ptr);
    }
    if test_config != 0 {
        (*srv).srvconf.pid_file = 0 as *mut buffer;
        if 1 as libc::c_int == test_config {
            printf(b"Syntax OK\n\0" as *const u8 as *const libc::c_char);
        } else {
            test_config = 0 as libc::c_int;
            (*srv).srvconf.preflight_check = 1 as libc::c_int as libc::c_uchar;
            (*srv).srvconf.dont_daemonize = 1 as libc::c_int as libc::c_uchar;
        }
    }
    if test_config != 0 || print_config != 0 {
        return 0 as libc::c_int;
    }
    if oneshot_fd != 0 {
        if oneshot_fd <= 2 as libc::c_int {
            log_error(
                (*srv).errh,
                b"src/server.c\0" as *const u8 as *const libc::c_char,
                1181 as libc::c_int as libc::c_uint,
                b"Invalid fds at startup with lighttpd -1\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        ::std::ptr::write_volatile(
            &mut graceful_shutdown as *mut sig_atomic_t,
            1 as libc::c_int,
        );
        (*srv).sockets_disabled = 2 as libc::c_int;
        (*srv).srvconf.dont_daemonize = 1 as libc::c_int as libc::c_uchar;
        (*srv).srvconf.pid_file = 0 as *mut buffer;
        if (*srv).srvconf.max_worker != 0 {
            (*srv).srvconf.max_worker = 0 as libc::c_int as libc::c_ushort;
            log_error(
                (*srv).errh,
                b"src/server.c\0" as *const u8 as *const libc::c_char,
                1191 as libc::c_int as libc::c_uint,
                b"server one-shot command line option disables server.max-worker config file option.\0"
                    as *const u8 as *const libc::c_char,
            );
        }
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
            st_atim: unix_timespec64_t {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: unix_timespec64_t {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: unix_timespec64_t {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        if 0 as libc::c_int != fstat(oneshot_fd, &mut st) {
            log_perror(
                (*srv).errh,
                b"src/server.c\0" as *const u8 as *const libc::c_char,
                1197 as libc::c_int as libc::c_uint,
                b"fstat()\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o10000 as libc::c_int as libc::c_uint
        {
            oneshot_fdout = dup(1 as libc::c_int);
            if oneshot_fdout <= 2 as libc::c_int {
                log_perror(
                    (*srv).errh,
                    b"src/server.c\0" as *const u8 as *const libc::c_char,
                    1204 as libc::c_int as libc::c_uint,
                    b"dup()\0" as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
        } else if !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o140000 as libc::c_int as libc::c_uint)
            {
            log_error(
                (*srv).errh,
                b"src/server.c\0" as *const u8 as *const libc::c_char,
                1211 as libc::c_int as libc::c_uint,
                b"lighttpd -1 stdin is not a socket\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    if !((*srv).srvconf.bindhost).is_null()
        && buffer_eq_slen(
            (*srv).srvconf.bindhost,
            b"/dev/stdin\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
    {
        if -(1 as libc::c_int) == (*srv).stdin_fd {
            (*srv).stdin_fd = dup(0 as libc::c_int);
        }
        if (*srv).stdin_fd <= 2 as libc::c_int {
            log_error(
                (*srv).errh,
                b"src/server.c\0" as *const u8 as *const libc::c_char,
                1221 as libc::c_int as libc::c_uint,
                b"Invalid fds at startup\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    let mut st_0: stat = stat {
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
        st_atim: unix_timespec64_t {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: unix_timespec64_t {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: unix_timespec64_t {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut devnull: libc::c_int = 0;
    let mut errfd: libc::c_int = 0;
    loop {
        devnull = fdevent_open_devnull();
        if !(-(1 as libc::c_int) != devnull && devnull <= 2 as libc::c_int) {
            break;
        }
    }
    if -(1 as libc::c_int) == devnull {
        log_perror(
            (*srv).errh,
            b"src/server.c\0" as *const u8 as *const libc::c_char,
            1240 as libc::c_int as libc::c_uint,
            b"opening /dev/null failed\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    errfd = if 0 as libc::c_int == fstat(2 as libc::c_int, &mut st_0) {
        -(1 as libc::c_int)
    } else {
        devnull
    };
    if 0 as libc::c_int != fdevent_set_stdin_stdout_stderr(devnull, devnull, errfd) {
        log_perror(
            (*srv).errh,
            b"src/server.c\0" as *const u8 as *const libc::c_char,
            1246 as libc::c_int as libc::c_uint,
            b"setting default fds failed\0" as *const u8 as *const libc::c_char,
        );
        if -(1 as libc::c_int) != errfd {
            close(errfd);
        }
        if devnull != errfd {
            close(devnull);
        }
        return -(1 as libc::c_int);
    }
    if -(1 as libc::c_int) != errfd {
        close(errfd);
    }
    if devnull != errfd {
        close(devnull);
    }
    http_response_send_1xx_cb_set(None, HTTP_VERSION_2 as libc::c_int);
    if config_feature_bool(
        srv,
        b"server.h2-discard-backend-1xx\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) == 0
    {
        http_response_send_1xx_cb_set(
            Some(
                h2_send_1xx
                    as unsafe extern "C" fn(
                        *mut request_st,
                        *mut connection,
                    ) -> libc::c_int,
            ),
            HTTP_VERSION_2 as libc::c_int,
        );
    }
    http_response_send_1xx_cb_set(None, HTTP_VERSION_1_1 as libc::c_int);
    if config_feature_bool(
        srv,
        b"server.h1-discard-backend-1xx\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) == 0
    {
        http_response_send_1xx_cb_set(
            Some(
                connection_send_1xx
                    as unsafe extern "C" fn(
                        *mut request_st,
                        *mut connection,
                    ) -> libc::c_int,
            ),
            HTTP_VERSION_1_1 as libc::c_int,
        );
    }
    if 0 as libc::c_int != config_set_defaults(srv) {
        log_error(
            (*srv).errh,
            b"src/server.c\0" as *const u8 as *const libc::c_char,
            1269 as libc::c_int as libc::c_uint,
            b"setting default values failed\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if plugins_load(srv) != 0 {
        log_error(
            (*srv).errh,
            b"src/server.c\0" as *const u8 as *const libc::c_char,
            1275 as libc::c_int as libc::c_uint,
            b"loading plugins finally failed\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if HANDLER_GO_ON as libc::c_int as libc::c_uint
        != plugins_call_init(srv) as libc::c_uint
    {
        log_error(
            (*srv).errh,
            b"src/server.c\0" as *const u8 as *const libc::c_char,
            1281 as libc::c_int as libc::c_uint,
            b"Initialization of plugins failed. Going down.\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int as uint32_t;
    let mut pname: *const libc::c_char = 0 as *const libc::c_char;
    while i < (*srv).plugins.used {
        let mut p: *mut plugin = *((*srv).plugins.ptr as *mut *mut plugin)
            .offset(i as isize);
        if !pname.is_null()
            && 0 as libc::c_int
                == strcmp((*p).name, b"indexfile\0" as *const u8 as *const libc::c_char)
        {
            log_error(
                (*srv).errh,
                b"src/server.c\0" as *const u8 as *const libc::c_char,
                1291 as libc::c_int as libc::c_uint,
                b"Warning: mod_indexfile should be listed in server.modules prior to mod_%s\0"
                    as *const u8 as *const libc::c_char,
                pname,
            );
            break;
        } else {
            if ((*p).handle_subrequest_start).is_some()
                && ((*p).handle_subrequest).is_some()
            {
                if pname.is_null() {
                    pname = (*p).name;
                }
            }
            i = i.wrapping_add(1);
        }
    }
    if -(2 as libc::c_int) == pid_fd {
        ::std::ptr::write_volatile(&mut pid_fd as *mut libc::c_int, -(1 as libc::c_int));
    }
    if -(1 as libc::c_int) == pid_fd && !((*srv).srvconf.pid_file).is_null() {
        let mut pidfile: *const libc::c_char = (*(*srv).srvconf.pid_file).ptr;
        ::std::ptr::write_volatile(
            &mut pid_fd as *mut libc::c_int,
            fdevent_open_cloexec(
                pidfile,
                0 as libc::c_int,
                0o1 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int
                    | 0o1000 as libc::c_int,
                (0o400 as libc::c_int | 0o200 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                    as mode_t,
            ),
        );
        if -(1 as libc::c_int)
            == ::std::ptr::read_volatile::<libc::c_int>(&pid_fd as *const libc::c_int)
        {
            let mut st_1: stat = stat {
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
                st_atim: unix_timespec64_t {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_mtim: unix_timespec64_t {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_ctim: unix_timespec64_t {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                __glibc_reserved: [0; 3],
            };
            if *__errno_location() != 17 as libc::c_int {
                log_perror(
                    (*srv).errh,
                    b"src/server.c\0" as *const u8 as *const libc::c_char,
                    1307 as libc::c_int as libc::c_uint,
                    b"opening pid-file failed: %s\0" as *const u8 as *const libc::c_char,
                    pidfile,
                );
                return -(1 as libc::c_int);
            }
            if 0 as libc::c_int != stat(pidfile, &mut st_1) {
                log_perror(
                    (*srv).errh,
                    b"src/server.c\0" as *const u8 as *const libc::c_char,
                    1313 as libc::c_int as libc::c_uint,
                    b"stating existing pid-file failed: %s\0" as *const u8
                        as *const libc::c_char,
                    pidfile,
                );
            }
            if !(st_1.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint)
            {
                log_error(
                    (*srv).errh,
                    b"src/server.c\0" as *const u8 as *const libc::c_char,
                    1318 as libc::c_int as libc::c_uint,
                    b"pid-file exists and isn't regular file: %s\0" as *const u8
                        as *const libc::c_char,
                    pidfile,
                );
                return -(1 as libc::c_int);
            }
            ::std::ptr::write_volatile(
                &mut pid_fd as *mut libc::c_int,
                fdevent_open_cloexec(
                    pidfile,
                    0 as libc::c_int,
                    0o1 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
                    (0o400 as libc::c_int | 0o200 as libc::c_int
                        | 0o400 as libc::c_int >> 3 as libc::c_int
                        | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                        as mode_t,
                ),
            );
            if -(1 as libc::c_int)
                == ::std::ptr::read_volatile::<
                    libc::c_int,
                >(&pid_fd as *const libc::c_int)
            {
                log_perror(
                    (*srv).errh,
                    b"src/server.c\0" as *const u8 as *const libc::c_char,
                    1324 as libc::c_int as libc::c_uint,
                    b"opening pid-file failed: %s\0" as *const u8 as *const libc::c_char,
                    pidfile,
                );
                return -(1 as libc::c_int);
            }
        }
    }
    let mut rlim: rlimit = {
        let mut init = rlimit {
            rlim_cur: 4096 as libc::c_int as rlim_t,
            rlim_max: 4096 as libc::c_int as rlim_t,
        };
        init
    };
    let mut use_rlimit: libc::c_int = 1 as libc::c_int;
    if 0 as libc::c_int != getrlimit(RLIMIT_NOFILE, &mut rlim) {
        log_perror(
            (*srv).errh,
            b"src/server.c\0" as *const u8 as *const libc::c_char,
            1340 as libc::c_int as libc::c_uint,
            b"getrlimit()\0" as *const u8 as *const libc::c_char,
        );
        use_rlimit = 0 as libc::c_int;
    }
    if use_rlimit != 0 && (*srv).srvconf.max_fds as libc::c_int != 0
        && (i_am_root != 0 || (*srv).srvconf.max_fds as libc::c_ulong <= rlim.rlim_max)
    {
        let mut rlim_cur: rlim_t = rlim.rlim_cur;
        rlim.rlim_cur = (*srv).srvconf.max_fds as rlim_t;
        if i_am_root != 0 {
            rlim.rlim_max = (*srv).srvconf.max_fds as rlim_t;
        }
        if 0 as libc::c_int != setrlimit(RLIMIT_NOFILE, &mut rlim) {
            log_perror(
                (*srv).errh,
                b"src/server.c\0" as *const u8 as *const libc::c_char,
                1356 as libc::c_int as libc::c_uint,
                b"setrlimit()\0" as *const u8 as *const libc::c_char,
            );
            log_error(
                (*srv).errh,
                b"src/server.c\0" as *const u8 as *const libc::c_char,
                1357 as libc::c_int as libc::c_uint,
                b"setrlimit() may need root to run once: setsebool -P httpd_setrlimit on\0"
                    as *const u8 as *const libc::c_char,
            );
            use_rlimit = 0 as libc::c_int;
            if (*srv).srvconf.max_fds as libc::c_ulong > rlim_cur {
                (*srv).srvconf.max_fds = rlim_cur as libc::c_ushort;
            }
        }
    }
    if 0 as libc::c_int == (*srv).srvconf.max_fds as libc::c_int {
        (*srv)
            .srvconf
            .max_fds = (if rlim.rlim_cur <= 4096 as libc::c_int as libc::c_ulong {
            rlim.rlim_cur as libc::c_ushort as libc::c_int
        } else {
            4096 as libc::c_int
        }) as libc::c_ushort;
    }
    if use_rlimit != 0 && (*srv).srvconf.enable_cores as libc::c_int != 0
        && getrlimit(RLIMIT_CORE, &mut rlim) == 0 as libc::c_int
    {
        rlim.rlim_cur = rlim.rlim_max;
        setrlimit(RLIMIT_CORE, &mut rlim);
    }
    if 0 as libc::c_int != network_init(srv, (*srv).stdin_fd) {
        return -(1 as libc::c_int);
    }
    (*srv).stdin_fd = -(1 as libc::c_int);
    if i_am_root != 0 {
        let mut grp: *mut group = 0 as *mut group;
        let mut pwd: *mut passwd = 0 as *mut passwd;
        if !((*srv).srvconf.groupname).is_null() {
            grp = getgrnam((*(*srv).srvconf.groupname).ptr);
            if grp.is_null() {
                log_error(
                    (*srv).errh,
                    b"src/server.c\0" as *const u8 as *const libc::c_char,
                    1392 as libc::c_int as libc::c_uint,
                    b"can't find groupname %s\0" as *const u8 as *const libc::c_char,
                    (*(*srv).srvconf.groupname).ptr,
                );
                return -(1 as libc::c_int);
            }
        }
        if !((*srv).srvconf.username).is_null() {
            pwd = getpwnam((*(*srv).srvconf.username).ptr);
            if pwd.is_null() {
                log_error(
                    (*srv).errh,
                    b"src/server.c\0" as *const u8 as *const libc::c_char,
                    1400 as libc::c_int as libc::c_uint,
                    b"can't find username %s\0" as *const u8 as *const libc::c_char,
                    (*(*srv).srvconf.username).ptr,
                );
                return -(1 as libc::c_int);
            }
            if (*pwd).pw_uid == 0 as libc::c_int as libc::c_uint {
                log_error(
                    (*srv).errh,
                    b"src/server.c\0" as *const u8 as *const libc::c_char,
                    1406 as libc::c_int as libc::c_uint,
                    b"I will not set uid to 0\n\0" as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            if grp.is_null()
                && {
                    grp = getgrgid((*pwd).pw_gid);
                    grp.is_null()
                }
            {
                log_error(
                    (*srv).errh,
                    b"src/server.c\0" as *const u8 as *const libc::c_char,
                    1412 as libc::c_int as libc::c_uint,
                    b"can't find group id %d\0" as *const u8 as *const libc::c_char,
                    (*pwd).pw_gid as libc::c_int,
                );
                return -(1 as libc::c_int);
            }
        }
        if !grp.is_null() {
            if (*grp).gr_gid == 0 as libc::c_int as libc::c_uint {
                log_error(
                    (*srv).errh,
                    b"src/server.c\0" as *const u8 as *const libc::c_char,
                    1420 as libc::c_int as libc::c_uint,
                    b"I will not set gid to 0\n\0" as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
        }
        if !grp.is_null() {
            if -(1 as libc::c_int) == setgid((*grp).gr_gid) {
                log_perror(
                    (*srv).errh,
                    b"src/server.c\0" as *const u8 as *const libc::c_char,
                    1432 as libc::c_int as libc::c_uint,
                    b"setgid()\0" as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            if -(1 as libc::c_int)
                == setgroups(0 as libc::c_int as size_t, 0 as *const __gid_t)
            {
                log_perror(
                    (*srv).errh,
                    b"src/server.c\0" as *const u8 as *const libc::c_char,
                    1436 as libc::c_int as libc::c_uint,
                    b"setgroups()\0" as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            if !((*srv).srvconf.username).is_null() {
                initgroups((*(*srv).srvconf.username).ptr, (*grp).gr_gid);
            }
        }
        if !((*srv).srvconf.changeroot).is_null() {
            tzset();
            if -(1 as libc::c_int) == chroot((*(*srv).srvconf.changeroot).ptr) {
                log_perror(
                    (*srv).errh,
                    b"src/server.c\0" as *const u8 as *const libc::c_char,
                    1449 as libc::c_int as libc::c_uint,
                    b"chroot()\0" as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            if -(1 as libc::c_int) == chdir(b"/\0" as *const u8 as *const libc::c_char) {
                log_perror(
                    (*srv).errh,
                    b"src/server.c\0" as *const u8 as *const libc::c_char,
                    1453 as libc::c_int as libc::c_uint,
                    b"chdir()\0" as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
        }
        if !pwd.is_null() {
            if -(1 as libc::c_int) == setuid((*pwd).pw_uid) {
                log_perror(
                    (*srv).errh,
                    b"src/server.c\0" as *const u8 as *const libc::c_char,
                    1462 as libc::c_int as libc::c_uint,
                    b"setuid()\0" as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
        }
        if (*srv).srvconf.enable_cores != 0 {
            prctl(
                4 as libc::c_int,
                1 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
        }
    }
    if 0 as libc::c_int == (*srv).srvconf.dont_daemonize as libc::c_int
        && 0 as libc::c_int == graceful_restart
    {
        parent_pipe_fd = daemonize();
    }
    ::std::ptr::write_volatile(
        &mut graceful_restart as *mut sig_atomic_t,
        0 as libc::c_int,
    );
    if 0 as libc::c_int == oneshot_fd {
        ::std::ptr::write_volatile(
            &mut graceful_shutdown as *mut sig_atomic_t,
            0 as libc::c_int,
        );
    }
    memset(
        &mut act as *mut sigaction as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sigaction>() as libc::c_ulong,
    );
    act
        .__sigaction_handler
        .sa_handler = ::std::mem::transmute::<
        libc::intptr_t,
        __sighandler_t,
    >(1 as libc::c_int as libc::intptr_t);
    sigaction(13 as libc::c_int, &mut act, 0 as *mut sigaction);
    ::std::ptr::write_volatile(
        &mut last_sighup_info._sifields._kill.si_uid as *mut __uid_t,
        0 as libc::c_int as __uid_t,
    );
    ::std::ptr::write_volatile(
        &mut last_sighup_info._sifields._kill.si_pid as *mut __pid_t,
        0 as libc::c_int,
    );
    ::std::ptr::write_volatile(
        &mut last_sigterm_info._sifields._kill.si_uid as *mut __uid_t,
        0 as libc::c_int as __uid_t,
    );
    ::std::ptr::write_volatile(
        &mut last_sigterm_info._sifields._kill.si_pid as *mut __pid_t,
        0 as libc::c_int,
    );
    act
        .__sigaction_handler
        .sa_sigaction = Some(
        sigaction_handler
            as unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    );
    sigemptyset(&mut act.sa_mask);
    act.sa_flags = 4 as libc::c_int;
    sigaction(2 as libc::c_int, &mut act, 0 as *mut sigaction);
    sigaction(15 as libc::c_int, &mut act, 0 as *mut sigaction);
    sigaction(1 as libc::c_int, &mut act, 0 as *mut sigaction);
    sigaction(14 as libc::c_int, &mut act, 0 as *mut sigaction);
    sigaction(10 as libc::c_int, &mut act, 0 as *mut sigaction);
    act.sa_flags |= 0x10000000 as libc::c_int | 1 as libc::c_int;
    sigaction(17 as libc::c_int, &mut act, 0 as *mut sigaction);
    (*srv).gid = getgid();
    (*srv).uid = getuid();
    (*srv).pid = getpid();
    if pid_fd > 2 as libc::c_int {
        let tb: *mut buffer = (*srv).tmp_buf;
        buffer_clear(tb);
        buffer_append_int(tb, (*srv).pid as intmax_t);
        buffer_append_string_len(
            tb,
            b"\n\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        if -(1 as libc::c_int) as libc::c_long
            == write_all(
                pid_fd,
                (*tb).ptr as *const libc::c_void,
                buffer_clen(tb) as size_t,
            )
        {
            log_perror(
                (*srv).errh,
                b"src/server.c\0" as *const u8 as *const libc::c_char,
                1551 as libc::c_int as libc::c_uint,
                b"Couldn't write pid file\0" as *const u8 as *const libc::c_char,
            );
            close(pid_fd);
            ::std::ptr::write_volatile(
                &mut pid_fd as *mut libc::c_int,
                -(1 as libc::c_int),
            );
            return -(1 as libc::c_int);
        }
    } else if pid_fd < -(2 as libc::c_int) {
        ::std::ptr::write_volatile(&mut pid_fd as *mut libc::c_int, -pid_fd);
    }
    if (*srv).srvconf.preflight_check == 0 {
        if -(1 as libc::c_int) == config_log_error_open(srv) {
            log_error(
                (*srv).errh,
                b"src/server.c\0" as *const u8 as *const libc::c_char,
                1564 as libc::c_int as libc::c_uint,
                b"Opening errorlog failed. Going down.\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if oneshot_fd == 0 {
            log_error(
                (*srv).errh,
                b"src/server.c\0" as *const u8 as *const libc::c_char,
                1568 as libc::c_int as libc::c_uint,
                b"server started (lighttpd/1.4.64)\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if HANDLER_GO_ON as libc::c_int as libc::c_uint
        != plugins_call_set_defaults(srv) as libc::c_uint
    {
        log_error(
            (*srv).errh,
            b"src/server.c\0" as *const u8 as *const libc::c_char,
            1572 as libc::c_int as libc::c_uint,
            b"Configuration of plugins failed. Going down.\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if config_finalize(srv, &default_server_tag) == 0 {
        return -(1 as libc::c_int);
    }
    if (*srv).srvconf.preflight_check != 0 {
        return 0 as libc::c_int;
    }
    if 0 as libc::c_int == (*srv).srvconf.dont_daemonize as libc::c_int
        && -(1 as libc::c_int) != parent_pipe_fd
    {
        if 0 as libc::c_int as libc::c_long
            > write(
                parent_pipe_fd,
                b"\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            )
        {
            return -(1 as libc::c_int);
        }
        close(parent_pipe_fd);
    }
    if idle_limit != 0 && (*srv).srvconf.max_worker as libc::c_int != 0 {
        (*srv).srvconf.max_worker = 0 as libc::c_int as libc::c_ushort;
        log_error(
            (*srv).errh,
            b"src/server.c\0" as *const u8 as *const libc::c_char,
            1598 as libc::c_int as libc::c_uint,
            b"server idle time limit command line option disables server.max-worker config file option.\0"
                as *const u8 as *const libc::c_char,
        );
    }
    num_childs = (*srv).srvconf.max_worker as libc::c_int;
    if num_childs > 0 as libc::c_int {
        let vla = num_childs as usize;
        let mut pids: Vec::<pid_t> = ::std::vec::from_elem(0, vla);
        let mut pid: pid_t = 0;
        let npids: libc::c_int = num_childs;
        let mut child: libc::c_int = 0 as libc::c_int;
        let mut timer: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut n: libc::c_int = 0 as libc::c_int;
        while n < npids {
            *pids.as_mut_ptr().offset(n as isize) = -(1 as libc::c_int);
            n += 1;
        }
        server_graceful_signal_prev_generation();
        while child == 0 && srv_shutdown == 0 && graceful_shutdown == 0 {
            if num_childs > 0 as libc::c_int {
                pid = fork();
                match pid {
                    -1 => return -(1 as libc::c_int),
                    0 => {
                        child = 1 as libc::c_int;
                        alarm(0 as libc::c_int as libc::c_uint);
                    }
                    _ => {
                        num_childs -= 1;
                        let mut n_0: libc::c_int = 0 as libc::c_int;
                        while n_0 < npids {
                            if -(1 as libc::c_int)
                                == *pids.as_mut_ptr().offset(n_0 as isize)
                            {
                                *pids.as_mut_ptr().offset(n_0 as isize) = pid;
                                break;
                            } else {
                                n_0 += 1;
                            }
                        }
                    }
                }
            } else {
                let mut status: libc::c_int = 0;
                let mut mono_ts: unix_time64_t = 0;
                pid = fdevent_waitpid_intr(-(1 as libc::c_int), &mut status);
                if -(1 as libc::c_int) != pid {
                    mono_ts = log_monotonic_secs;
                    log_monotonic_secs = server_monotonic_secs();
                    log_epoch_secs = server_epoch_secs(
                        srv,
                        log_monotonic_secs - mono_ts,
                    );
                    if plugins_call_handle_waitpid(srv, pid, status) as libc::c_uint
                        != HANDLER_GO_ON as libc::c_int as libc::c_uint
                    {
                        if timer == 0 {
                            timer = 5 as libc::c_int as libc::c_uint;
                            alarm(timer);
                        }
                    } else {
                        match fdlog_pipes_waitpid_cb(pid) {
                            -1 => {
                                if timer == 0 {
                                    timer = 5 as libc::c_int as libc::c_uint;
                                    alarm(timer);
                                }
                            }
                            1 => {}
                            _ => {
                                let mut n_1: libc::c_int = 0 as libc::c_int;
                                while n_1 < npids {
                                    if pid == *pids.as_mut_ptr().offset(n_1 as isize) {
                                        *pids
                                            .as_mut_ptr()
                                            .offset(n_1 as isize) = -(1 as libc::c_int);
                                        num_childs += 1;
                                        break;
                                    } else {
                                        n_1 += 1;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    match *__errno_location() {
                        4 => {
                            mono_ts = log_monotonic_secs;
                            log_monotonic_secs = server_monotonic_secs();
                            log_epoch_secs = server_epoch_secs(
                                srv,
                                log_monotonic_secs - mono_ts,
                            );
                            if handle_sig_hup != 0 {
                                ::std::ptr::write_volatile(
                                    &mut handle_sig_hup as *mut sig_atomic_t,
                                    0 as libc::c_int,
                                );
                                fdlog_files_cycle((*srv).errh);
                                let mut n_2: libc::c_int = 0 as libc::c_int;
                                while n_2 < npids {
                                    if *pids.as_mut_ptr().offset(n_2 as isize)
                                        > 0 as libc::c_int
                                    {
                                        kill(
                                            *pids.as_mut_ptr().offset(n_2 as isize),
                                            1 as libc::c_int,
                                        );
                                    }
                                    n_2 += 1;
                                }
                            }
                            if handle_sig_alarm != 0 {
                                ::std::ptr::write_volatile(
                                    &mut handle_sig_alarm as *mut sig_atomic_t,
                                    0 as libc::c_int,
                                );
                                timer = 0 as libc::c_int as libc::c_uint;
                                plugins_call_handle_trigger(srv);
                                fdlog_pipes_restart(log_monotonic_secs);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        if child == 0 {
            if graceful_shutdown != 0 || graceful_restart != 0 {
                if graceful_restart != 0 {
                    ::std::ptr::write_volatile(
                        &mut graceful_restart as *mut sig_atomic_t,
                        2 as libc::c_int,
                    );
                }
                kill(0 as libc::c_int, 2 as libc::c_int);
                server_graceful_state(srv);
            } else if srv_shutdown != 0 {
                kill(0 as libc::c_int, 15 as libc::c_int);
            }
            return 0 as libc::c_int;
        }
        let mut actignore: sigaction = sigaction {
            __sigaction_handler: C2RustUnnamed_16 {
                sa_handler: None,
            },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        memset(
            &mut actignore as *mut sigaction as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<sigaction>() as libc::c_ulong,
        );
        actignore
            .__sigaction_handler
            .sa_handler = ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t);
        sigaction(10 as libc::c_int, &mut actignore, 0 as *mut sigaction);
        if 0 as libc::c_int <= pid_fd {
            close(pid_fd);
            ::std::ptr::write_volatile(
                &mut pid_fd as *mut libc::c_int,
                -(1 as libc::c_int),
            );
        }
        (*srv).srvconf.pid_file = 0 as *mut buffer;
        fdlog_pipes_abandon_pids();
        (*srv).pid = getpid();
        li_rand_reseed();
    }
    (*srv).max_fds = (*srv).srvconf.max_fds as libc::c_int;
    if (*srv).max_fds < 32 as libc::c_int {
        (*srv).max_fds = 32 as libc::c_int;
    }
    (*srv)
        .ev = fdevent_init(
        (*srv).srvconf.event_handler,
        &mut (*srv).max_fds,
        &mut (*srv).cur_fds,
        (*srv).errh,
    );
    if ((*srv).ev).is_null() {
        log_error(
            (*srv).errh,
            b"src/server.c\0" as *const u8 as *const libc::c_char,
            1743 as libc::c_int as libc::c_uint,
            b"fdevent_init failed\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*srv).max_fds_lowat = (*srv).max_fds * 8 as libc::c_int / 10 as libc::c_int;
    (*srv).max_fds_hiwat = (*srv).max_fds * 9 as libc::c_int / 10 as libc::c_int;
    if (*srv).srvconf.max_conns as libc::c_int > (*srv).max_fds / 2 as libc::c_int {
        log_error(
            (*srv).errh,
            b"src/server.c\0" as *const u8 as *const libc::c_char,
            1753 as libc::c_int as libc::c_uint,
            b"can't have more connections than fds/2: %hu %d\0" as *const u8
                as *const libc::c_char,
            (*srv).srvconf.max_conns as libc::c_int,
            (*srv).max_fds,
        );
        (*srv).srvconf.max_conns = ((*srv).max_fds / 2 as libc::c_int) as libc::c_ushort;
        (*srv).lim_conns = (*srv).srvconf.max_conns as uint32_t;
    } else if (*srv).srvconf.max_conns != 0 {
        (*srv).lim_conns = (*srv).srvconf.max_conns as uint32_t;
    } else {
        (*srv).srvconf.max_conns = ((*srv).max_fds / 3 as libc::c_int) as libc::c_ushort;
        (*srv).lim_conns = (*srv).srvconf.max_conns as uint32_t;
    }
    sigaction(17 as libc::c_int, &mut act, 0 as *mut sigaction);
    if 0 as libc::c_int != network_register_fdevents(srv) {
        return -(1 as libc::c_int);
    }
    chunkqueue_internal_pipes(
        config_feature_bool(
            srv,
            b"chunkqueue.splice\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        ),
    );
    if stat_cache_init((*srv).ev, (*srv).errh) == 0 {
        log_error(
            (*srv).errh,
            b"src/server.c\0" as *const u8 as *const libc::c_char,
            1785 as libc::c_int as libc::c_uint,
            b"stat-cache could not be setup, dying.\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut fd: libc::c_int = fdevent_open_devnull();
    if fd >= 0 as libc::c_int {
        (*srv).cur_fds = fd;
        close(fd);
    }
    if 0 as libc::c_int != server_sockets_set_nb_cloexec(srv) {
        return -(1 as libc::c_int);
    }
    if HANDLER_GO_ON as libc::c_int as libc::c_uint
        != plugins_call_worker_init(srv) as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    if oneshot_fdout > 0 as libc::c_int {
        if server_oneshot_init_pipe(srv, oneshot_fd, oneshot_fdout) != 0 {
            oneshot_fd = -(1 as libc::c_int);
            oneshot_fdout = -(1 as libc::c_int);
        }
    } else if oneshot_fd != 0 && server_oneshot_init(srv, oneshot_fd) != 0 {
        oneshot_fd = -(1 as libc::c_int);
    }
    if 0 as libc::c_int == (*srv).srvconf.max_worker as libc::c_int {
        server_graceful_signal_prev_generation();
    }
    return 1 as libc::c_int;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn server_handle_sighup(srv: *mut server) {
    plugins_call_handle_sighup(srv);
    fdlog_files_cycle((*srv).errh);
    log_error(
        (*srv).errh,
        b"src/server.c\0" as *const u8 as *const libc::c_char,
        1847 as libc::c_int as libc::c_uint,
        b"logfiles cycled UID = %d PID = %d\0" as *const u8 as *const libc::c_char,
        last_sighup_info._sifields._kill.si_uid as libc::c_int,
        last_sighup_info._sifields._kill.si_pid,
    );
}
#[inline(never)]
unsafe extern "C" fn server_handle_sigalrm(
    srv: *mut server,
    mut mono_ts: unix_time64_t,
    mut last_active_ts: unix_time64_t,
) {
    plugins_call_handle_trigger(srv);
    log_monotonic_secs = mono_ts;
    log_epoch_secs = server_epoch_secs(srv, 0 as libc::c_int as unix_time64_t);
    if idle_limit != 0 && (idle_limit as libc::c_long) < mono_ts - last_active_ts
        && graceful_shutdown == 0
    {
        log_error(
            (*srv).errh,
            b"src/server.c\0" as *const u8 as *const libc::c_char,
            1867 as libc::c_int as libc::c_uint,
            b"[note] idle timeout %ds exceeded, initiating graceful shutdown\0"
                as *const u8 as *const libc::c_char,
            idle_limit,
        );
        ::std::ptr::write_volatile(
            &mut graceful_shutdown as *mut sig_atomic_t,
            2 as libc::c_int,
        );
        if graceful_restart != 0 {
            ::std::ptr::write_volatile(
                &mut graceful_restart as *mut sig_atomic_t,
                0 as libc::c_int,
            );
            if pid_fd < -(2 as libc::c_int) {
                ::std::ptr::write_volatile(&mut pid_fd as *mut libc::c_int, -pid_fd);
            }
            server_sockets_close(srv);
        }
    }
    if ((*srv).loadts + 30 as libc::c_int as libc::c_long) < mono_ts {
        if -(1 as libc::c_int)
            != getloadavg(((*srv).loadavg).as_mut_ptr(), 3 as libc::c_int)
        {
            (*srv).loadts = mono_ts;
        }
    }
    if 0 as libc::c_int as libc::c_long == mono_ts & 0x3f as libc::c_int as libc::c_long
    {
        fdlog_flushall((*srv).errh);
        chunkqueue_chunk_pool_clear();
        request_pool_free();
        connections_pool_clear(srv);
        if malloc_trim_fn.is_some() {
            malloc_trim_fn.expect("non-null function pointer")(malloc_top_pad);
        }
        if 0 as libc::c_int == (*srv).srvconf.max_worker as libc::c_int {
            fdlog_pipes_restart(mono_ts);
        }
    }
    stat_cache_trigger_cleanup();
    config_reset_config_bytes_sec((*srv).config_data_base);
    if graceful_shutdown != 0 && srv_shutdown == 0 {
        server_graceful_shutdown_maint(srv);
    }
    connection_periodic_maint(srv, mono_ts);
}
#[inline(never)]
unsafe extern "C" fn server_handle_sigchld(srv: *mut server) {
    let mut pid: pid_t = 0;
    loop {
        let mut status: libc::c_int = 0;
        pid = fdevent_waitpid(-(1 as libc::c_int), &mut status, 1 as libc::c_int);
        if pid > 0 as libc::c_int {
            if !(plugins_call_handle_waitpid(srv, pid, status) as libc::c_uint
                != HANDLER_GO_ON as libc::c_int as libc::c_uint)
            {
                if 0 as libc::c_int == (*srv).srvconf.max_worker as libc::c_int {
                    fdlog_pipes_waitpid_cb(pid) != 0;
                }
            }
        }
        if !(pid > 0 as libc::c_int
            || -(1 as libc::c_int) == pid && *__errno_location() == 4 as libc::c_int)
        {
            break;
        }
    };
}
unsafe extern "C" fn server_run_con_queue(
    joblist: *mut connection,
    sent: *const connection,
) {
    let mut con: *mut connection = joblist;
    let mut jqnext: *mut connection = 0 as *mut connection;
    while con != sent as *mut connection {
        jqnext = (*con).jqnext;
        (*con).jqnext = 0 as *mut connection;
        connection_state_machine(con);
        con = jqnext;
    }
}
static mut sentinel: *mut connection = 0 as *const connection as *mut connection;
#[inline(never)]
unsafe extern "C" fn server_main_loop(srv: *mut server) {
    let mut last_active_ts: unix_time64_t = server_monotonic_secs();
    log_epoch_secs = server_epoch_secs(srv, 0 as libc::c_int as unix_time64_t);
    while srv_shutdown == 0 {
        if handle_sig_hup != 0 {
            ::std::ptr::write_volatile(
                &mut handle_sig_hup as *mut sig_atomic_t,
                0 as libc::c_int,
            );
            server_handle_sighup(srv);
        }
        let mut mono_ts: unix_time64_t = server_monotonic_secs();
        if mono_ts != log_monotonic_secs {
            server_handle_sigalrm(srv, mono_ts, last_active_ts);
        }
        if handle_sig_child != 0 {
            ::std::ptr::write_volatile(
                &mut handle_sig_child as *mut sig_atomic_t,
                0 as libc::c_int,
            );
            server_handle_sigchld(srv);
        }
        if graceful_shutdown != 0 {
            server_graceful_state(srv);
            if ((*srv).conns).is_null() && graceful_shutdown != 0 {
                ::std::ptr::write_volatile(
                    &mut srv_shutdown as *mut sig_atomic_t,
                    1 as libc::c_int,
                );
                break;
            }
        } else if (*srv).sockets_disabled != 0 {
            server_overload_check(srv);
        } else {
            server_load_check(srv);
        }
        let joblist: *mut connection = log_con_jqueue;
        log_con_jqueue = sentinel;
        server_run_con_queue(joblist, sentinel);
        if fdevent_poll(
            (*srv).ev,
            (if log_con_jqueue != sentinel {
                0 as libc::c_int
            } else {
                1000 as libc::c_int
            }),
        ) > 0 as libc::c_int
        {
            last_active_ts = log_monotonic_secs;
        }
    }
}
#[cold]
#[inline(never)]
unsafe extern "C" fn main_init_once() -> libc::c_int {
    if 0 as libc::c_int as libc::c_uint != getuid()
        && (geteuid() != getuid() || getegid() != getgid())
    {
        fprintf(
            stderr,
            b"Are you nuts ? Don't apply a SUID bit to this binary\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    mallopt(-(8 as libc::c_int), 2 as libc::c_int);
    malloc_top_pad = 524288 as libc::c_int as size_t;
    let top_pad_str: *const libc::c_char = getenv(
        b"MALLOC_TOP_PAD_\0" as *const u8 as *const libc::c_char,
    );
    if !top_pad_str.is_null() {
        let mut top_pad: libc::c_ulong = strtoul(
            top_pad_str,
            0 as *mut *mut libc::c_char,
            10 as libc::c_int,
        );
        if top_pad
            != (9223372036854775807 as libc::c_long as libc::c_ulong)
                .wrapping_mul(2 as libc::c_ulong)
                .wrapping_add(1 as libc::c_ulong)
        {
            malloc_top_pad = top_pad;
        }
    }
    malloc_trim_fn = Some(malloc_trim as unsafe extern "C" fn(size_t) -> libc::c_int);
    setlocale(2 as libc::c_int, b"C\0" as *const u8 as *const libc::c_char);
    tzset();
    return 1 as libc::c_int;
}
#[cold]
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    if main_init_once() == 0 {
        return -(1 as libc::c_int);
    }
    let mut rc: libc::c_int = 0;
    loop {
        let srv: *mut server = server_init();
        if graceful_restart != 0 {
            server_sockets_restore(srv);
            optind = 1 as libc::c_int;
        }
        rc = server_main_setup(srv, argc, argv);
        if rc > 0 as libc::c_int {
            server_main_loop(srv);
            if graceful_shutdown != 0 || graceful_restart != 0 {
                server_graceful_state(srv);
            }
            if ((*srv).conns).is_null() {
                rc = 0 as libc::c_int;
            }
            if 2 as libc::c_int == graceful_shutdown {
                log_error(
                    (*srv).errh,
                    b"src/server.c\0" as *const u8 as *const libc::c_char,
                    2073 as libc::c_int as libc::c_uint,
                    b"server stopped after idle timeout\0" as *const u8
                        as *const libc::c_char,
                );
            } else if oneshot_fd == 0 {
                log_error(
                    (*srv).errh,
                    b"src/server.c\0" as *const u8 as *const libc::c_char,
                    2077 as libc::c_int as libc::c_uint,
                    b"server stopped by UID = %d PID = %d\0" as *const u8
                        as *const libc::c_char,
                    last_sigterm_info._sifields._kill.si_uid as libc::c_int,
                    last_sigterm_info._sifields._kill.si_pid,
                );
            }
        }
        chunkqueue_internal_pipes(0 as libc::c_int);
        remove_pid_file(srv);
        config_log_error_close(srv);
        if graceful_restart != 0 {
            server_sockets_save(srv);
        } else {
            network_close(srv);
        }
        request_pool_free();
        connections_free(srv);
        plugins_free(srv);
        server_free(srv);
        if rc < 0 as libc::c_int || graceful_restart == 0 {
            break;
        }
        while fdevent_waitpid(
            -(1 as libc::c_int),
            0 as *mut libc::c_int,
            0 as libc::c_int,
        ) > 0 as libc::c_int
        {}
        if !(graceful_restart != 0) {
            break;
        }
    }
    return rc;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
unsafe extern "C" fn run_static_initializers() {
    default_server_tag = {
        let mut init = buffer {
            ptr: b"lighttpd/1.4.64\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            used: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint),
            size: 0 as libc::c_int as uint32_t,
        };
        init
    };
    sentinel = &log_con_jqueue as *const *mut connection as *mut *mut connection
        as uintptr_t as *mut connection;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
