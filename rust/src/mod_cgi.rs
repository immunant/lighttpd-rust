use ::libc;
extern "C" {
    pub type pcre2_real_match_data_8;
    pub type h2con;
    pub type fdevents;
    static mut log_con_jqueue: *mut connection;
    fn buffer_init() -> *mut buffer;
    fn buffer_free(b: *mut buffer);
    fn buffer_extend(b: *mut buffer, x: size_t) -> *mut libc::c_char;
    fn buffer_copy_string(b: *mut buffer, s: *const libc::c_char);
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_eq_icase_slen(
        b: *const buffer,
        s: *const libc::c_char,
        slen: size_t,
    ) -> libc::c_int;
    fn buffer_is_equal(a: *const buffer, b: *const buffer) -> libc::c_int;
    fn buffer_path_simplify(b: *mut buffer);
    fn buffer_to_upper(b: *mut buffer);
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn array_match_key_suffix(a: *const array, b: *const buffer) -> *mut data_unset;
    fn chunk_buffer_acquire() -> *mut buffer;
    fn chunk_buffer_release(b: *mut buffer);
    fn chunk_buffer_yield(b: *mut buffer);
    fn chunk_buffer_prepare_append(b: *mut buffer, sz: size_t) -> size_t;
    fn chunkqueue_set_tempdirs(
        cq: *mut chunkqueue,
        tempdirs: *const array,
        upload_temp_file_size: off_t,
    );
    fn chunkqueue_mark_written(cq: *mut chunkqueue, len: off_t);
    fn chunkqueue_remove_finished_chunks(cq: *mut chunkqueue);
    fn chunkqueue_open_file_chunk(
        cq: *mut chunkqueue,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn chunkqueue_write_chunk_to_pipe(
        fd: libc::c_int,
        cq: *mut chunkqueue,
        errh: *mut log_error_st,
    ) -> ssize_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn stat_cache_path_stat(name: *const buffer) -> *const stat_cache_st;
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
    fn http_response_reset(r: *mut request_st);
    fn http_response_backend_done(r: *mut request_st);
    fn http_response_backend_error(r: *mut request_st);
    fn http_response_upgrade_read_body_unknown(r: *mut request_st);
    fn http_cgi_headers(
        r: *mut request_st,
        opts: *mut http_cgi_opts,
        cb: http_cgi_header_append_cb,
        vdata: *mut libc::c_void,
    ) -> libc::c_int;
    fn http_chunk_append_buffer(r: *mut request_st, mem: *mut buffer) -> libc::c_int;
    fn http_header_request_unset(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    );
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
    fn config_feature_bool(
        srv: *const server,
        feature: *const libc::c_char,
        default_value: libc::c_int,
    ) -> libc::c_int;
    fn plugins_call_handle_request_reset(r: *mut request_st) -> handler_t;
    fn lseek(__fd: libc::c_int, __offset: __off64_t, __whence: libc::c_int) -> __off64_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
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
    fn fdevent_fcntl_set_nb(fd: libc::c_int) -> libc::c_int;
    fn fdevent_pipe_cloexec(
        fds: *mut libc::c_int,
        bufsz_hint: libc::c_uint,
    ) -> libc::c_int;
    fn fdevent_open_devnull() -> libc::c_int;
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
pub struct data_string {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
    pub ext: libc::c_int,
    pub value: buffer,
}
pub type stat_cache_st = stat;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const BACKEND_AJP13: C2RustUnnamed_5 = 4;
pub const BACKEND_SCGI: C2RustUnnamed_5 = 3;
pub const BACKEND_FASTCGI: C2RustUnnamed_5 = 2;
pub const BACKEND_CGI: C2RustUnnamed_5 = 1;
pub const BACKEND_PROXY: C2RustUnnamed_5 = 0;
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
pub type http_response_opts = http_response_opts_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_cgi_opts_t {
    pub authorizer: libc::c_int,
    pub break_scriptfilename_for_php: libc::c_int,
    pub docroot: *const buffer,
    pub strip_request_uri: *const buffer,
}
pub type http_cgi_opts = http_cgi_opts_t;
pub type http_cgi_header_append_cb = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_char,
        size_t,
        *const libc::c_char,
        size_t,
    ) -> libc::c_int,
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
pub struct env_accum {
    pub offsets: *mut uintptr_t,
    pub osize: size_t,
    pub oused: size_t,
    pub b: *mut buffer,
    pub boffsets: *mut buffer,
    pub ld_preload: *mut buffer,
    pub ld_library_path: *mut buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cgi_limits {
    pub read_timeout: unix_time64_t,
    pub write_timeout: unix_time64_t,
    pub signal_fin: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_config {
    pub cgi: *const array,
    pub limits: *const cgi_limits,
    pub execute_x_only: libc::c_ushort,
    pub local_redir: libc::c_ushort,
    pub xsendfile_allow: libc::c_ushort,
    pub upgrade: libc::c_ushort,
    pub xsendfile_docroot: *const array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cgi_pid_t {
    pub pid: pid_t,
    pub signal_sent: libc::c_int,
    pub hctx: *mut handler_ctx,
    pub next: *mut cgi_pid_t,
    pub prev: *mut cgi_pid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct handler_ctx {
    pub cgi_pid: *mut cgi_pid_t,
    pub fd: libc::c_int,
    pub fdtocgi: libc::c_int,
    pub rd_revents: libc::c_int,
    pub wr_revents: libc::c_int,
    pub fdn: *mut fdnode,
    pub fdntocgi: *mut fdnode,
    pub r: *mut request_st,
    pub con: *mut connection,
    pub ev: *mut fdevents,
    pub plugin_data: *mut plugin_data,
    pub response: *mut buffer,
    pub read_ts: unix_time64_t,
    pub write_ts: unix_time64_t,
    pub cgi_handler: *mut buffer,
    pub opts: http_response_opts,
    pub conf: plugin_config,
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
    pub tempfile_accum: libc::c_int,
    pub cgi_pid: *mut cgi_pid_t,
    pub env: env_accum,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub name: *const libc::c_char,
    pub sig: libc::c_int,
}
#[inline]
unsafe extern "C" fn connection_jq_append(con: *mut connection) {
    if ((*con).jqnext).is_null() {
        (*con).jqnext = log_con_jqueue;
        log_con_jqueue = con;
    }
}
#[inline]
unsafe extern "C" fn buffer_clear(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
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
unsafe extern "C" fn cgi_handler_ctx_init() -> *mut handler_ctx {
    let mut hctx: *mut handler_ctx = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<handler_ctx>() as libc::c_ulong,
    ) as *mut handler_ctx;
    if hctx.is_null() {
        ck_assert_failed(
            b"src/mod_cgi.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int as libc::c_uint,
            b"hctx\0" as *const u8 as *const libc::c_char,
        );
    }
    (*hctx).response = chunk_buffer_acquire();
    (*hctx).fd = -(1 as libc::c_int);
    (*hctx).fdtocgi = -(1 as libc::c_int);
    return hctx;
}
unsafe extern "C" fn cgi_handler_ctx_free(mut hctx: *mut handler_ctx) {
    chunk_buffer_release((*hctx).response);
    free(hctx as *mut libc::c_void);
}
#[cold]
unsafe extern "C" fn mod_cgi_init() -> *mut libc::c_void {
    let mut p: *mut plugin_data = 0 as *mut plugin_data;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    p = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<plugin_data>() as libc::c_ulong,
    ) as *mut plugin_data;
    if p.is_null() {
        ck_assert_failed(
            b"src/mod_cgi.c\0" as *const u8 as *const libc::c_char,
            123 as libc::c_int as libc::c_uint,
            b"p\0" as *const u8 as *const libc::c_char,
        );
    }
    s = getenv(b"LD_PRELOAD\0" as *const u8 as *const libc::c_char);
    if !s.is_null() {
        (*p).env.ld_preload = buffer_init();
        buffer_copy_string((*p).env.ld_preload, s);
    }
    s = getenv(b"LD_LIBRARY_PATH\0" as *const u8 as *const libc::c_char);
    if !s.is_null() {
        (*p).env.ld_library_path = buffer_init();
        buffer_copy_string((*p).env.ld_library_path, s);
    }
    return p as *mut libc::c_void;
}
#[cold]
unsafe extern "C" fn mod_cgi_free(mut p_d: *mut libc::c_void) {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    buffer_free((*p).env.ld_preload);
    buffer_free((*p).env.ld_library_path);
    let mut cgi_pid: *mut cgi_pid_t = (*p).cgi_pid;
    let mut next: *mut cgi_pid_t = 0 as *mut cgi_pid_t;
    while !cgi_pid.is_null() {
        next = (*cgi_pid).next;
        free(cgi_pid as *mut libc::c_void);
        cgi_pid = next;
    }
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
                    6 => {
                        free((*cpv).v.v);
                    }
                    _ => {}
                }
            }
            cpv = cpv.offset(1);
        }
        i += 1;
    }
}
unsafe extern "C" fn mod_cgi_merge_config_cpv(
    pconf: *mut plugin_config,
    cpv: *const config_plugin_value_t,
) {
    match (*cpv).k_id {
        0 => {
            (*pconf).cgi = (*cpv).v.a;
        }
        1 => {
            (*pconf).execute_x_only = (*cpv).v.u as libc::c_ushort;
        }
        2 => {
            (*pconf).xsendfile_allow = (*cpv).v.u as libc::c_ushort;
        }
        3 => {
            (*pconf).xsendfile_docroot = (*cpv).v.a;
        }
        4 => {
            (*pconf).local_redir = (*cpv).v.u as libc::c_ushort;
        }
        5 => {
            (*pconf).upgrade = (*cpv).v.u as libc::c_ushort;
        }
        6 => {
            if !((*cpv).vtype as libc::c_uint
                != T_CONFIG_LOCAL as libc::c_int as libc::c_uint)
            {
                (*pconf).limits = (*cpv).v.v as *const cgi_limits;
            }
        }
        _ => return,
    };
}
unsafe extern "C" fn mod_cgi_merge_config(
    pconf: *mut plugin_config,
    mut cpv: *const config_plugin_value_t,
) {
    loop {
        mod_cgi_merge_config_cpv(pconf, cpv);
        cpv = cpv.offset(1);
        if !((*cpv).k_id != -(1 as libc::c_int)) {
            break;
        }
    };
}
unsafe extern "C" fn mod_cgi_patch_config(r: *mut request_st, p: *mut plugin_data) {
    (*p).conf = (*p).defaults;
    let mut i: libc::c_int = 1 as libc::c_int;
    let mut used: libc::c_int = (*p).nconfig;
    while i < used {
        if config_check_cond(
            r,
            (*((*p).cvlist).offset(i as isize)).k_id as uint32_t as libc::c_int,
        ) != 0
        {
            mod_cgi_merge_config(
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
#[cold]
unsafe extern "C" fn mod_cgi_str_to_signal(
    mut s: *const libc::c_char,
    mut default_sig: libc::c_int,
) -> libc::c_int {
    static mut sigs: [C2RustUnnamed_7; 23] = [
        {
            let mut init = C2RustUnnamed_7 {
                name: b"HUP\0" as *const u8 as *const libc::c_char,
                sig: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                name: b"INT\0" as *const u8 as *const libc::c_char,
                sig: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                name: b"QUIT\0" as *const u8 as *const libc::c_char,
                sig: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                name: b"ILL\0" as *const u8 as *const libc::c_char,
                sig: 4 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                name: b"TRAP\0" as *const u8 as *const libc::c_char,
                sig: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                name: b"ABRT\0" as *const u8 as *const libc::c_char,
                sig: 6 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                name: b"BUS\0" as *const u8 as *const libc::c_char,
                sig: 7 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                name: b"FPE\0" as *const u8 as *const libc::c_char,
                sig: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                name: b"KILL\0" as *const u8 as *const libc::c_char,
                sig: 9 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                name: b"USR1\0" as *const u8 as *const libc::c_char,
                sig: 10 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                name: b"SEGV\0" as *const u8 as *const libc::c_char,
                sig: 11 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                name: b"USR2\0" as *const u8 as *const libc::c_char,
                sig: 12 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                name: b"PIPE\0" as *const u8 as *const libc::c_char,
                sig: 13 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                name: b"ALRM\0" as *const u8 as *const libc::c_char,
                sig: 14 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                name: b"TERM\0" as *const u8 as *const libc::c_char,
                sig: 15 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                name: b"CHLD\0" as *const u8 as *const libc::c_char,
                sig: 17 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                name: b"CONT\0" as *const u8 as *const libc::c_char,
                sig: 18 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                name: b"URG\0" as *const u8 as *const libc::c_char,
                sig: 23 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                name: b"XCPU\0" as *const u8 as *const libc::c_char,
                sig: 24 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                name: b"XFSZ\0" as *const u8 as *const libc::c_char,
                sig: 25 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                name: b"WINCH\0" as *const u8 as *const libc::c_char,
                sig: 28 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                name: b"POLL\0" as *const u8 as *const libc::c_char,
                sig: 29 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                name: b"IO\0" as *const u8 as *const libc::c_char,
                sig: 29 as libc::c_int,
            };
            init
        },
    ];
    if *s.offset(0 as libc::c_int as isize) as libc::c_int == 'S' as i32
        && *s.offset(1 as libc::c_int as isize) as libc::c_int == 'I' as i32
        && *s.offset(2 as libc::c_int as isize) as libc::c_int == 'G' as i32
    {
        s = s.offset(3 as libc::c_int as isize);
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[C2RustUnnamed_7; 23]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong)
    {
        if 0 as libc::c_int == strcmp(s, sigs[i as usize].name) {
            return sigs[i as usize].sig;
        }
        i = i.wrapping_add(1);
    }
    return default_sig;
}
unsafe extern "C" fn mod_cgi_parse_limits(
    a: *const array,
    errh: *mut log_error_st,
) -> *mut cgi_limits {
    let limits: *mut cgi_limits = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cgi_limits>() as libc::c_ulong,
    ) as *mut cgi_limits;
    if limits.is_null() {
        ck_assert_failed(
            b"src/mod_cgi.c\0" as *const u8 as *const libc::c_char,
            274 as libc::c_int as libc::c_uint,
            b"limits\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*a).used {
        let du: *const data_unset = *((*a).data).offset(i as isize);
        let mut v: int32_t = config_plugin_value_to_int32(du, -(1 as libc::c_int));
        if buffer_eq_icase_slen(
            &(*du).key,
            b"read-timeout\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            (*limits).read_timeout = v as unix_time64_t;
        } else if buffer_eq_icase_slen(
                &(*du).key,
                b"write-timeout\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            (*limits).write_timeout = v as unix_time64_t;
        } else if buffer_eq_icase_slen(
                &(*du).key,
                b"tcp-fin-propagate\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            if -(1 as libc::c_int) == v {
                v = 15 as libc::c_int;
                if (*du).type_0 as libc::c_uint
                    == TYPE_STRING as libc::c_int as libc::c_uint
                {
                    let vstr: *mut buffer = &mut (*(du as *mut data_string)).value;
                    buffer_to_upper(vstr);
                    v = mod_cgi_str_to_signal((*vstr).ptr, 15 as libc::c_int);
                }
            }
            (*limits).signal_fin = v;
        } else {
            log_error(
                errh,
                b"src/mod_cgi.c\0" as *const u8 as *const libc::c_char,
                298 as libc::c_int as libc::c_uint,
                b"unrecognized cgi.limits param: %s\0" as *const u8
                    as *const libc::c_char,
                (*du).key.ptr,
            );
        }
        i = i.wrapping_add(1);
    }
    return limits;
}
static mut cpk: [config_plugin_keys_t; 8] = [config_plugin_keys_t {
    k: 0 as *const libc::c_char,
    klen: 0,
    ktype: 0,
    scope: 0,
}; 8];
#[cold]
unsafe extern "C" fn mod_cgi_set_defaults(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    if config_plugin_values_init(
        srv,
        p as *mut libc::c_void,
        cpk.as_ptr(),
        b"mod_cgi\0" as *const u8 as *const libc::c_char,
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
            let mut current_block_13: u64;
            match (*cpv).k_id {
                0 => {
                    current_block_13 = 5634871135123216486;
                }
                1 => {
                    current_block_13 = 17551258386263640072;
                }
                2 => {
                    current_block_13 = 17551258386263640072;
                }
                3 => {
                    let mut j: uint32_t = 0 as libc::c_int as uint32_t;
                    while j < (*(*cpv).v.a).used {
                        let mut ds: *mut data_string = *((*(*cpv).v.a).data)
                            .offset(j as isize) as *mut data_string;
                        if *((*ds).value.ptr).offset(0 as libc::c_int as isize)
                            as libc::c_int != '/' as i32
                        {
                            log_error(
                                (*srv).errh,
                                b"src/mod_cgi.c\0" as *const u8 as *const libc::c_char,
                                350 as libc::c_int as libc::c_uint,
                                b"%s paths must begin with '/'; invalid: \"%s\"\0"
                                    as *const u8 as *const libc::c_char,
                                cpk[(*cpv).k_id as usize].k,
                                (*ds).value.ptr,
                            );
                            return HANDLER_ERROR;
                        }
                        buffer_path_simplify(&mut (*ds).value);
                        buffer_append_slash(&mut (*ds).value);
                        j = j.wrapping_add(1);
                    }
                    current_block_13 = 5634871135123216486;
                }
                4 => {
                    current_block_13 = 8925939473551295275;
                }
                5 => {
                    current_block_13 = 8925939473551295275;
                }
                6 => {
                    (*cpv)
                        .v
                        .v = mod_cgi_parse_limits((*cpv).v.a, (*srv).errh)
                        as *mut libc::c_void;
                    if ((*cpv).v.v).is_null() {
                        return HANDLER_ERROR;
                    }
                    (*cpv).vtype = T_CONFIG_LOCAL;
                    current_block_13 = 5634871135123216486;
                }
                _ => {
                    current_block_13 = 5634871135123216486;
                }
            }
            match current_block_13 {
                17551258386263640072 => {}
                8925939473551295275 => {}
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
            mod_cgi_merge_config(&mut (*p).defaults, cpv_0);
        }
    }
    (*p)
        .tempfile_accum = config_feature_bool(
        srv,
        b"cgi.tempfile-accum\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    return HANDLER_GO_ON;
}
unsafe extern "C" fn cgi_pid_add(
    mut p: *mut plugin_data,
    mut pid: pid_t,
    mut hctx: *mut handler_ctx,
) -> *mut cgi_pid_t {
    let mut cgi_pid: *mut cgi_pid_t = malloc(
        ::std::mem::size_of::<cgi_pid_t>() as libc::c_ulong,
    ) as *mut cgi_pid_t;
    if cgi_pid.is_null() {
        ck_assert_failed(
            b"src/mod_cgi.c\0" as *const u8 as *const libc::c_char,
            388 as libc::c_int as libc::c_uint,
            b"cgi_pid\0" as *const u8 as *const libc::c_char,
        );
    }
    (*cgi_pid).pid = pid;
    (*cgi_pid).signal_sent = 0 as libc::c_int;
    (*cgi_pid).hctx = hctx;
    (*cgi_pid).prev = 0 as *mut cgi_pid_t;
    (*cgi_pid).next = (*p).cgi_pid;
    (*p).cgi_pid = cgi_pid;
    return cgi_pid;
}
unsafe extern "C" fn cgi_pid_kill(mut cgi_pid: *mut cgi_pid_t, mut sig: libc::c_int) {
    (*cgi_pid).signal_sent = sig;
    kill((*cgi_pid).pid, sig);
}
unsafe extern "C" fn cgi_pid_del(mut p: *mut plugin_data, mut cgi_pid: *mut cgi_pid_t) {
    if !((*cgi_pid).prev).is_null() {
        (*(*cgi_pid).prev).next = (*cgi_pid).next;
    } else {
        (*p).cgi_pid = (*cgi_pid).next;
    }
    if !((*cgi_pid).next).is_null() {
        (*(*cgi_pid).next).prev = (*cgi_pid).prev;
    }
    free(cgi_pid as *mut libc::c_void);
}
unsafe extern "C" fn cgi_connection_close_fdtocgi(mut hctx: *mut handler_ctx) {
    if -(1 as libc::c_int) == (*hctx).fdtocgi {
        return;
    }
    let ev: *mut fdevents = (*hctx).ev;
    fdevent_fdnode_event_del(ev, (*hctx).fdntocgi);
    fdevent_sched_close(ev, (*hctx).fdtocgi, 0 as libc::c_int);
    (*hctx).fdntocgi = 0 as *mut fdnode;
    (*hctx).fdtocgi = -(1 as libc::c_int);
}
unsafe extern "C" fn cgi_connection_close(mut hctx: *mut handler_ctx) {
    if (*hctx).fd != -(1 as libc::c_int) {
        let ev: *mut fdevents = (*hctx).ev;
        fdevent_fdnode_event_del(ev, (*hctx).fdn);
        fdevent_sched_close(ev, (*hctx).fd, 0 as libc::c_int);
        (*hctx).fdn = 0 as *mut fdnode;
    }
    if (*hctx).fdtocgi != -(1 as libc::c_int) {
        cgi_connection_close_fdtocgi(hctx);
    }
    let p: *mut plugin_data = (*hctx).plugin_data;
    let r: *mut request_st = (*hctx).r;
    let ref mut fresh0 = *((*r).plugin_ctx).offset((*p).id as isize);
    *fresh0 = 0 as *mut libc::c_void;
    if !((*hctx).cgi_pid).is_null() {
        cgi_pid_kill((*hctx).cgi_pid, 15 as libc::c_int);
        (*(*hctx).cgi_pid).hctx = 0 as *mut handler_ctx;
    }
    cgi_handler_ctx_free(hctx);
    if (*r).handler_module == (*p).self_0 as *const plugin {
        http_response_backend_done(r);
    }
}
unsafe extern "C" fn cgi_connection_close_callback(
    r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut hctx: *mut handler_ctx = *((*r).plugin_ctx)
        .offset((*(p_d as *mut plugin_data)).id as isize) as *mut handler_ctx;
    if !hctx.is_null() {
        chunkqueue_set_tempdirs(
            &mut (*r).reqbody_queue,
            (*r).reqbody_queue.tempdirs,
            0 as libc::c_int as off_t,
        );
        cgi_connection_close(hctx);
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn cgi_handle_fdevent_send(
    mut ctx: *mut libc::c_void,
    mut revents: libc::c_int,
) -> handler_t {
    let mut hctx: *mut handler_ctx = ctx as *mut handler_ctx;
    (*hctx).wr_revents |= revents;
    connection_jq_append((*hctx).con);
    return HANDLER_FINISHED;
}
unsafe extern "C" fn cgi_process_wr_revents(
    hctx: *mut handler_ctx,
    r: *mut request_st,
    mut revents: libc::c_int,
) -> handler_t {
    if revents & 0x4 as libc::c_int != 0 {
        if 0 as libc::c_int != cgi_write_request(hctx, (*hctx).fdtocgi) {
            cgi_connection_close(hctx);
            return HANDLER_ERROR;
        }
    }
    if revents & 0x10 as libc::c_int != 0 {
        if (*r).reqbody_length != 0 {
            let mut cq: *mut chunkqueue = &mut (*r).reqbody_queue;
            chunkqueue_mark_written(cq, chunkqueue_length(cq));
            if (*cq).bytes_in != (*r).reqbody_length {
                (*r).keep_alive = 0 as libc::c_int as int8_t;
            }
        }
        cgi_connection_close_fdtocgi(hctx);
    } else if revents & 0x8 as libc::c_int != 0 {
        log_error(
            (*r).conf.errh,
            b"src/mod_cgi.c\0" as *const u8 as *const libc::c_char,
            508 as libc::c_int as libc::c_uint,
            b"cgi-FDEVENT_ERR\0" as *const u8 as *const libc::c_char,
        );
        cgi_connection_close(hctx);
        return HANDLER_ERROR;
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn cgi_response_headers(
    r: *mut request_st,
    mut opts: *mut http_response_opts_t,
) -> handler_t {
    let mut hctx: *mut handler_ctx = (*opts).pdata as *mut handler_ctx;
    if (*r).resp_htags & (1 as libc::c_ulong) << HTTP_HEADER_UPGRADE as libc::c_int != 0
    {
        if (*hctx).conf.upgrade as libc::c_int != 0
            && (*r).http_status == 101 as libc::c_int
        {
            http_response_upgrade_read_body_unknown(r);
        } else {
            (*r).resp_htags
                &= !((1 as libc::c_ulong) << HTTP_HEADER_UPGRADE as libc::c_int);
        }
    }
    if (*hctx).conf.upgrade as libc::c_int != 0
        && (*r).resp_htags & (1 as libc::c_ulong) << HTTP_HEADER_UPGRADE as libc::c_int
            == 0
    {
        let mut cq: *mut chunkqueue = &mut (*r).reqbody_queue;
        (*hctx).conf.upgrade = 0 as libc::c_int as libc::c_ushort;
        if (*cq).bytes_out == (*r).reqbody_length {
            cgi_connection_close_fdtocgi(hctx);
        }
    }
    return HANDLER_GO_ON;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn cgi_local_redir(
    r: *mut request_st,
    hctx: *mut handler_ctx,
) -> handler_t {
    buffer_clear((*hctx).response);
    chunk_buffer_yield((*hctx).response);
    http_response_reset(r);
    plugins_call_handle_request_reset(r);
    return HANDLER_COMEBACK;
}
unsafe extern "C" fn cgi_recv_response(
    r: *mut request_st,
    hctx: *mut handler_ctx,
) -> libc::c_int {
    let bytes_in: off_t = (*r).write_queue.bytes_in;
    match http_response_read(r, &mut (*hctx).opts, (*hctx).response, (*hctx).fdn)
        as libc::c_uint
    {
        4 => {
            http_response_backend_error(r);
        }
        1 => {}
        2 => return cgi_local_redir(r, hctx) as libc::c_int,
        _ => {
            if (*r).write_queue.bytes_in > bytes_in {
                (*hctx).read_ts = log_monotonic_secs;
            }
            return HANDLER_GO_ON as libc::c_int;
        }
    }
    cgi_connection_close(hctx);
    return HANDLER_FINISHED as libc::c_int;
}
unsafe extern "C" fn cgi_handle_fdevent(
    mut ctx: *mut libc::c_void,
    mut revents: libc::c_int,
) -> handler_t {
    let mut hctx: *mut handler_ctx = ctx as *mut handler_ctx;
    (*hctx).rd_revents |= revents;
    connection_jq_append((*hctx).con);
    return HANDLER_FINISHED;
}
unsafe extern "C" fn cgi_process_rd_revents(
    hctx: *mut handler_ctx,
    r: *mut request_st,
    mut revents: libc::c_int,
) -> handler_t {
    if revents & 0x1 as libc::c_int != 0 {
        let mut rc: handler_t = cgi_recv_response(r, hctx) as handler_t;
        if rc as libc::c_uint != HANDLER_GO_ON as libc::c_int as libc::c_uint {
            return rc;
        }
    }
    if revents & (0x10 as libc::c_int | 0x2000 as libc::c_int) != 0 {
        if (*r).resp_body_started != 0 {
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
                rc_0 = cgi_recv_response(r, hctx) as handler_t;
                if !(rc_0 as libc::c_uint
                    == HANDLER_GO_ON as libc::c_int as libc::c_uint)
                {
                    break;
                }
            }
            (*r).conf.stream_response_body = flags;
            return rc_0;
        } else {
            if buffer_is_blank((*hctx).response) == 0 {
                (*r).resp_body_started = 1 as libc::c_int as libc::c_char;
                if 0 as libc::c_int != http_chunk_append_buffer(r, (*hctx).response) {
                    cgi_connection_close(hctx);
                    return HANDLER_ERROR;
                }
                if 0 as libc::c_int == (*r).http_status {
                    (*r).http_status = 200 as libc::c_int;
                }
            }
        }
        cgi_connection_close(hctx);
    } else if revents & 0x8 as libc::c_int != 0 {
        cgi_connection_close(hctx);
        return HANDLER_ERROR;
    }
    return HANDLER_GO_ON;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn cgi_env_offset_resize(mut env: *mut env_accum) {
    chunk_buffer_prepare_append(
        (*env).boffsets,
        ((*(*env).boffsets).size).wrapping_mul(2 as libc::c_int as libc::c_uint)
            as size_t,
    );
    (*env).offsets = (*(*env).boffsets).ptr as *mut libc::c_void as *mut uintptr_t;
    (*env)
        .osize = ((*(*env).boffsets).size as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<uintptr_t>() as libc::c_ulong);
}
unsafe extern "C" fn cgi_env_add(
    mut venv: *mut libc::c_void,
    mut key: *const libc::c_char,
    mut key_len: size_t,
    mut val: *const libc::c_char,
    mut val_len: size_t,
) -> libc::c_int {
    let mut env: *mut env_accum = venv as *mut env_accum;
    if key.is_null() || val.is_null() && val_len != 0 {
        return -(1 as libc::c_int);
    }
    if ((*env).osize == (*env).oused) as libc::c_int as libc::c_long != 0 {
        cgi_env_offset_resize(env);
    }
    let fresh1 = (*env).oused;
    (*env).oused = ((*env).oused).wrapping_add(1);
    *((*env).offsets)
        .offset(
            fresh1 as isize,
        ) = ((*(*env).b).used).wrapping_sub(1 as libc::c_int as libc::c_uint)
        as uintptr_t;
    let dst: *mut libc::c_char = buffer_extend(
        (*env).b,
        key_len.wrapping_add(val_len).wrapping_add(2 as libc::c_int as libc::c_ulong),
    );
    memcpy(dst as *mut libc::c_void, key as *const libc::c_void, key_len);
    *dst.offset(key_len as isize) = '=' as i32 as libc::c_char;
    if val_len != 0 {
        memcpy(
            dst.offset(key_len as isize).offset(1 as libc::c_int as isize)
                as *mut libc::c_void,
            val as *const libc::c_void,
            val_len,
        );
    }
    *dst
        .offset(
            key_len.wrapping_add(1 as libc::c_int as libc::c_ulong).wrapping_add(val_len)
                as isize,
        ) = '\u{0}' as i32 as libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn cgi_write_request(
    mut hctx: *mut handler_ctx,
    mut fd: libc::c_int,
) -> libc::c_int {
    let r: *mut request_st = (*hctx).r;
    let mut cq: *mut chunkqueue = &mut (*r).reqbody_queue;
    let mut c: *mut chunk = 0 as *mut chunk;
    chunkqueue_remove_finished_chunks(cq);
    c = (*cq).first;
    while !c.is_null() {
        let mut wr: ssize_t = chunkqueue_write_chunk_to_pipe(fd, cq, (*r).conf.errh);
        if wr > 0 as libc::c_int as libc::c_long {
            (*hctx).write_ts = log_monotonic_secs;
            chunkqueue_mark_written(cq, wr);
            if !(c != (*cq).first || wr == 16384 as libc::c_int as libc::c_long) {
                break;
            }
            c = (*cq).first;
        } else {
            if wr < 0 as libc::c_int as libc::c_long {
                match *__errno_location() {
                    11 | 4 => {}
                    32 | 104 => {
                        log_error(
                            (*r).conf.errh,
                            b"src/mod_cgi.c\0" as *const u8 as *const libc::c_char,
                            691 as libc::c_int as libc::c_uint,
                            b"failed to send post data to cgi, connection closed by CGI\0"
                                as *const u8 as *const libc::c_char,
                        );
                        chunkqueue_mark_written(cq, chunkqueue_length(cq));
                    }
                    _ => {
                        log_perror(
                            (*r).conf.errh,
                            b"src/mod_cgi.c\0" as *const u8 as *const libc::c_char,
                            698 as libc::c_int as libc::c_uint,
                            b"write() failed\0" as *const u8 as *const libc::c_char,
                        );
                        return -(1 as libc::c_int);
                    }
                }
            }
            break;
        }
    }
    if (*cq).bytes_out == (*r).reqbody_length && (*hctx).conf.upgrade == 0 {
        if -(1 as libc::c_int) == (*hctx).fdtocgi {
            (*(*(*r).con).srv).cur_fds -= 1;
            if close(fd) != 0 {
                log_perror(
                    (*r).conf.errh,
                    b"src/mod_cgi.c\0" as *const u8 as *const libc::c_char,
                    712 as libc::c_int as libc::c_uint,
                    b"cgi stdin close %d failed\0" as *const u8 as *const libc::c_char,
                    fd,
                );
            }
        } else {
            cgi_connection_close_fdtocgi(hctx);
        }
    } else {
        let mut cqlen: off_t = chunkqueue_length(cq);
        if (*cq).bytes_in != (*r).reqbody_length
            && cqlen < (65536 as libc::c_int - 16384 as libc::c_int) as libc::c_long
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
        let ev: *mut fdevents = (*hctx).ev;
        if -(1 as libc::c_int) == (*hctx).fdtocgi {
            (*hctx).fdtocgi = fd;
            (*hctx)
                .fdntocgi = fdevent_register(
                ev,
                (*hctx).fdtocgi,
                Some(
                    cgi_handle_fdevent_send
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            libc::c_int,
                        ) -> handler_t,
                ),
                hctx as *mut libc::c_void,
            );
        }
        if 0 as libc::c_int as libc::c_long == cqlen {
            if (if !((*hctx).fdntocgi).is_null() {
                (*(*hctx).fdntocgi).events
            } else {
                0 as libc::c_int
            }) & 0x4 as libc::c_int != 0
            {
                fdevent_fdnode_event_set(ev, (*hctx).fdntocgi, 0 as libc::c_int);
            }
        } else {
            (*hctx).write_ts = log_monotonic_secs;
            fdevent_fdnode_event_set(ev, (*hctx).fdntocgi, 0x4 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn cgi_create_env(
    r: *mut request_st,
    p: *mut plugin_data,
    hctx: *mut handler_ctx,
    cgi_handler: *mut buffer,
) -> libc::c_int {
    let mut args: [*mut libc::c_char; 3] = [0 as *mut libc::c_char; 3];
    let mut to_cgi_fds: [libc::c_int; 2] = [0; 2];
    let mut from_cgi_fds: [libc::c_int; 2] = [0; 2];
    if buffer_is_blank(cgi_handler) == 0 {
        if (stat_cache_path_stat(cgi_handler)).is_null() {
            log_perror(
                (*r).conf.errh,
                b"src/mod_cgi.c\0" as *const u8 as *const libc::c_char,
                753 as libc::c_int as libc::c_uint,
                b"stat for cgi-handler %s\0" as *const u8 as *const libc::c_char,
                (*cgi_handler).ptr,
            );
            return -(1 as libc::c_int);
        }
    }
    to_cgi_fds[0 as libc::c_int as usize] = -(1 as libc::c_int);
    if 0 as libc::c_int as libc::c_long == (*r).reqbody_length {
        to_cgi_fds[0 as libc::c_int as usize] = fdevent_open_devnull();
        if -(1 as libc::c_int) == to_cgi_fds[0 as libc::c_int as usize] {
            log_perror(
                (*r).conf.errh,
                b"src/mod_cgi.c\0" as *const u8 as *const libc::c_char,
                766 as libc::c_int as libc::c_uint,
                b"open /dev/null\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else if (*r).conf.stream_request_body as libc::c_int
            & ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int) == 0
        {
        let cq: *mut chunkqueue = &mut (*r).reqbody_queue;
        let c: *mut chunk = (*cq).first;
        if !c.is_null() && c == (*cq).last
            && (*c).type_0 as libc::c_uint == FILE_CHUNK as libc::c_int as libc::c_uint
            && (*c).file.is_temp != 0
        {
            if -(1 as libc::c_int) == (*c).file.fd
                && 0 as libc::c_int != chunkqueue_open_file_chunk(cq, (*r).conf.errh)
            {
                return -(1 as libc::c_int);
            }
            if -(1 as libc::c_int) as libc::c_long
                == lseek((*c).file.fd, 0 as libc::c_int as __off64_t, 0 as libc::c_int)
            {
                log_perror(
                    (*r).conf.errh,
                    b"src/mod_cgi.c\0" as *const u8 as *const libc::c_char,
                    783 as libc::c_int as libc::c_uint,
                    b"lseek %s\0" as *const u8 as *const libc::c_char,
                    (*(*c).mem).ptr,
                );
                return -(1 as libc::c_int);
            }
            to_cgi_fds[0 as libc::c_int as usize] = (*c).file.fd;
            to_cgi_fds[1 as libc::c_int as usize] = -(1 as libc::c_int);
        }
    }
    let mut bufsz_hint: libc::c_uint = 16384 as libc::c_int as libc::c_uint;
    if -(1 as libc::c_int) == to_cgi_fds[0 as libc::c_int as usize]
        && fdevent_pipe_cloexec(to_cgi_fds.as_mut_ptr(), bufsz_hint) != 0
    {
        log_perror(
            (*r).conf.errh,
            b"src/mod_cgi.c\0" as *const u8 as *const libc::c_char,
            799 as libc::c_int as libc::c_uint,
            b"pipe failed\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if fdevent_pipe_cloexec(from_cgi_fds.as_mut_ptr(), bufsz_hint) != 0 {
        if 0 as libc::c_int as libc::c_long == (*r).reqbody_length {
            close(to_cgi_fds[0 as libc::c_int as usize]);
        } else if -(1 as libc::c_int) != to_cgi_fds[1 as libc::c_int as usize] {
            close(to_cgi_fds[0 as libc::c_int as usize]);
            close(to_cgi_fds[1 as libc::c_int as usize]);
        }
        log_perror(
            (*r).conf.errh,
            b"src/mod_cgi.c\0" as *const u8 as *const libc::c_char,
            810 as libc::c_int as libc::c_uint,
            b"pipe failed\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let env: *mut env_accum = &mut (*p).env;
    (*env).b = chunk_buffer_acquire();
    (*env).boffsets = chunk_buffer_acquire();
    buffer_truncate((*env).b, 0 as libc::c_int as uint32_t);
    let mut envp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut opts: http_cgi_opts = {
        let mut init = http_cgi_opts_t {
            authorizer: 0 as libc::c_int,
            break_scriptfilename_for_php: 0 as libc::c_int,
            docroot: 0 as *const buffer,
            strip_request_uri: 0 as *const buffer,
        };
        init
    };
    (*env).offsets = (*(*env).boffsets).ptr as *mut libc::c_void as *mut uintptr_t;
    (*env)
        .osize = ((*(*env).boffsets).size as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<uintptr_t>() as libc::c_ulong);
    (*env).oused = 0 as libc::c_int as size_t;
    http_cgi_headers(
        r,
        &mut opts,
        Some(
            cgi_env_add
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    size_t,
                    *const libc::c_char,
                    size_t,
                ) -> libc::c_int,
        ),
        env as *mut libc::c_void,
    );
    if !((*p).env.ld_preload).is_null() {
        cgi_env_add(
            env as *mut libc::c_void,
            b"LD_PRELOAD\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            (*(*p).env.ld_preload).ptr,
            buffer_clen((*p).env.ld_preload) as size_t,
        );
    }
    if !((*p).env.ld_library_path).is_null() {
        cgi_env_add(
            env as *mut libc::c_void,
            b"LD_LIBRARY_PATH\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            (*(*p).env.ld_library_path).ptr,
            buffer_clen((*p).env.ld_library_path) as size_t,
        );
    }
    if ((*env).osize == (*env).oused) as libc::c_int as libc::c_long != 0 {
        cgi_env_offset_resize(env);
    }
    envp = (*env).offsets as *mut *mut libc::c_char;
    let ref mut fresh2 = *envp.offset((*env).oused as isize);
    *fresh2 = 0 as *mut libc::c_char;
    let baseptr: uintptr_t = (*(*env).b).ptr as uintptr_t;
    i = 0 as libc::c_int as size_t;
    while i < (*env).oused {
        let ref mut fresh3 = *envp.offset(i as isize);
        *fresh3 = (*fresh3).offset(baseptr as isize);
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    if buffer_is_blank(cgi_handler) == 0 {
        let fresh4 = i;
        i = i.wrapping_add(1);
        args[fresh4 as usize] = (*cgi_handler).ptr;
    }
    let fresh5 = i;
    i = i.wrapping_add(1);
    args[fresh5 as usize] = (*r).physical.path.ptr;
    args[i as usize] = 0 as *mut libc::c_char;
    let mut dfd: libc::c_int = fdevent_open_dirname(
        (*r).physical.path.ptr,
        (*r).conf.follow_symlink as libc::c_int,
    );
    if -(1 as libc::c_int) == dfd {
        log_perror(
            (*r).conf.errh,
            b"src/mod_cgi.c\0" as *const u8 as *const libc::c_char,
            867 as libc::c_int as libc::c_uint,
            b"open dirname %s failed\0" as *const u8 as *const libc::c_char,
            (*r).physical.path.ptr,
        );
    }
    let mut serrh_fd: libc::c_int = if !((*r).conf.serrh).is_null() {
        (*(*r).conf.serrh).fd
    } else {
        -(1 as libc::c_int)
    };
    let mut pid: pid_t = if dfd >= 0 as libc::c_int {
        fdevent_fork_execve(
            args[0 as libc::c_int as usize],
            args.as_mut_ptr(),
            envp,
            to_cgi_fds[0 as libc::c_int as usize],
            from_cgi_fds[1 as libc::c_int as usize],
            serrh_fd,
            dfd,
        )
    } else {
        -(1 as libc::c_int)
    };
    chunk_buffer_release((*env).boffsets);
    chunk_buffer_release((*env).b);
    (*env).boffsets = 0 as *mut buffer;
    (*env).b = 0 as *mut buffer;
    if -(1 as libc::c_int) == pid {
        log_perror(
            (*r).conf.errh,
            b"src/mod_cgi.c\0" as *const u8 as *const libc::c_char,
            883 as libc::c_int as libc::c_uint,
            b"fork failed\0" as *const u8 as *const libc::c_char,
        );
        if -(1 as libc::c_int) != dfd {
            close(dfd);
        }
        close(from_cgi_fds[0 as libc::c_int as usize]);
        close(from_cgi_fds[1 as libc::c_int as usize]);
        if 0 as libc::c_int as libc::c_long == (*r).reqbody_length {
            close(to_cgi_fds[0 as libc::c_int as usize]);
        } else if -(1 as libc::c_int) != to_cgi_fds[1 as libc::c_int as usize] {
            close(to_cgi_fds[0 as libc::c_int as usize]);
            close(to_cgi_fds[1 as libc::c_int as usize]);
        }
        return -(1 as libc::c_int);
    } else {
        if -(1 as libc::c_int) != dfd {
            close(dfd);
        }
        close(from_cgi_fds[1 as libc::c_int as usize]);
        (*hctx).fd = from_cgi_fds[0 as libc::c_int as usize];
        (*hctx).cgi_pid = cgi_pid_add(p, pid, hctx);
        if 0 as libc::c_int as libc::c_long == (*r).reqbody_length {
            close(to_cgi_fds[0 as libc::c_int as usize]);
        } else if -(1 as libc::c_int) == to_cgi_fds[1 as libc::c_int as usize] {
            let cq_0: *mut chunkqueue = &mut (*r).reqbody_queue;
            chunkqueue_mark_written(cq_0, chunkqueue_length(cq_0));
        } else if 0 as libc::c_int
                == fdevent_fcntl_set_nb(to_cgi_fds[1 as libc::c_int as usize])
                && 0 as libc::c_int
                    == cgi_write_request(hctx, to_cgi_fds[1 as libc::c_int as usize])
            {
            close(to_cgi_fds[0 as libc::c_int as usize]);
            (*(*(*r).con).srv).cur_fds += 1;
        } else {
            close(to_cgi_fds[0 as libc::c_int as usize]);
            close(to_cgi_fds[1 as libc::c_int as usize]);
            close((*hctx).fd);
            (*hctx).fd = -(1 as libc::c_int);
            cgi_connection_close(hctx);
            return -(1 as libc::c_int);
        }
        (*(*(*r).con).srv).cur_fds += 1;
        let ev: *mut fdevents = (*hctx).ev;
        (*hctx)
            .fdn = fdevent_register(
            ev,
            (*hctx).fd,
            Some(
                cgi_handle_fdevent
                    as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> handler_t,
            ),
            hctx as *mut libc::c_void,
        );
        if -(1 as libc::c_int) == fdevent_fcntl_set_nb((*hctx).fd) {
            log_perror(
                (*r).conf.errh,
                b"src/mod_cgi.c\0" as *const u8 as *const libc::c_char,
                930 as libc::c_int as libc::c_uint,
                b"fcntl failed\0" as *const u8 as *const libc::c_char,
            );
            cgi_connection_close(hctx);
            return -(1 as libc::c_int);
        }
        (*hctx).read_ts = log_monotonic_secs;
        fdevent_fdnode_event_set(
            ev,
            (*hctx).fdn,
            0x1 as libc::c_int | 0x2000 as libc::c_int,
        );
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn cgi_is_handled(
    mut r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    let mut st: *const stat_cache_st = 0 as *const stat_cache_st;
    let mut ds: *mut data_string = 0 as *mut data_string;
    if !((*r).handler_module).is_null() {
        return HANDLER_GO_ON;
    }
    mod_cgi_patch_config(r, p);
    if ((*p).conf.cgi).is_null() {
        return HANDLER_GO_ON;
    }
    ds = array_match_key_suffix((*p).conf.cgi, &mut (*r).physical.path)
        as *mut data_string;
    if ds.is_null() {
        return HANDLER_GO_ON;
    }
    st = if !((*r).tmp_sce).is_null()
        && buffer_is_equal(&mut (*(*r).tmp_sce).name, &mut (*r).physical.path) != 0
    {
        &mut (*(*r).tmp_sce).st as *mut stat as *const stat
    } else {
        stat_cache_path_stat(&mut (*r).physical.path)
    };
    if st.is_null() {
        return HANDLER_GO_ON;
    }
    if !((*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint)
    {
        return HANDLER_GO_ON;
    }
    if (*p).conf.execute_x_only as libc::c_int == 1 as libc::c_int
        && (*st).st_mode
            & (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                as libc::c_uint == 0 as libc::c_int as libc::c_uint
    {
        return HANDLER_GO_ON;
    }
    if (*r).reqbody_length != 0 && (*p).tempfile_accum != 0
        && (*r).conf.stream_request_body as libc::c_int
            & ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int) == 0
    {
        (*r).reqbody_queue.upload_temp_file_size = 9223372036854775807 as libc::c_long;
    }
    let mut hctx: *mut handler_ctx = cgi_handler_ctx_init();
    (*hctx).ev = (*(*(*r).con).srv).ev;
    (*hctx).r = r;
    (*hctx).con = (*r).con;
    (*hctx).plugin_data = p;
    (*hctx).cgi_handler = &mut (*ds).value;
    memcpy(
        &mut (*hctx).conf as *mut plugin_config as *mut libc::c_void,
        &mut (*p).conf as *mut plugin_config as *const libc::c_void,
        ::std::mem::size_of::<plugin_config>() as libc::c_ulong,
    );
    if (*r).rqst_htags & (1 as libc::c_ulong) << HTTP_HEADER_UPGRADE as libc::c_int == 0
    {
        (*hctx).conf.upgrade = 0 as libc::c_int as libc::c_ushort;
    } else if (*hctx).conf.upgrade == 0
            || (*r).http_version as libc::c_int != HTTP_VERSION_1_1 as libc::c_int
        {
        (*hctx).conf.upgrade = 0 as libc::c_int as libc::c_ushort;
        http_header_request_unset(
            r,
            HTTP_HEADER_UPGRADE,
            b"Upgrade\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
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
    (*hctx).opts.fdfmt = 0o10000 as libc::c_int;
    (*hctx).opts.backend = BACKEND_CGI as libc::c_int;
    (*hctx).opts.authorizer = 0 as libc::c_int;
    (*hctx).opts.local_redir = (*hctx).conf.local_redir as uint8_t;
    (*hctx).opts.xsendfile_allow = (*hctx).conf.xsendfile_allow as uint8_t;
    (*hctx).opts.xsendfile_docroot = (*hctx).conf.xsendfile_docroot;
    (*hctx).opts.pdata = hctx as *mut libc::c_void;
    (*hctx)
        .opts
        .headers = Some(
        cgi_response_headers
            as unsafe extern "C" fn(
                *mut request_st,
                *mut http_response_opts_t,
            ) -> handler_t,
    );
    let ref mut fresh6 = *((*r).plugin_ctx).offset((*p).id as isize);
    *fresh6 = hctx as *mut libc::c_void;
    (*r).handler_module = (*p).self_0;
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_cgi_handle_subrequest(
    mut r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    let hctx: *mut handler_ctx = *((*r).plugin_ctx).offset((*p).id as isize)
        as *mut handler_ctx;
    if hctx.is_null() {
        return HANDLER_GO_ON;
    }
    if ((*r).conf.stream_request_body as libc::c_int
        & (1 as libc::c_int) << 13 as libc::c_int) as libc::c_long != 0
        && !((*hctx).conf.limits).is_null() && (*(*hctx).conf.limits).signal_fin != 0
    {
        if -(1 as libc::c_int) == (*hctx).fd {
            return HANDLER_ERROR;
        }
        if !((*hctx).cgi_pid).is_null() {
            cgi_pid_kill((*hctx).cgi_pid, (*(*hctx).conf.limits).signal_fin);
        }
    }
    let rd_revents: libc::c_int = (*hctx).rd_revents;
    let wr_revents: libc::c_int = (*hctx).wr_revents;
    if rd_revents != 0 {
        (*hctx).rd_revents = 0 as libc::c_int;
        let mut rc: handler_t = cgi_process_rd_revents(hctx, r, rd_revents);
        if rc as libc::c_uint != HANDLER_GO_ON as libc::c_int as libc::c_uint {
            return rc;
        }
    }
    if wr_revents != 0 {
        (*hctx).wr_revents = 0 as libc::c_int;
        let mut rc_0: handler_t = cgi_process_wr_revents(hctx, r, wr_revents);
        if rc_0 as libc::c_uint != HANDLER_GO_ON as libc::c_int as libc::c_uint {
            return rc_0;
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
            let mut rc_1: handler_t = cgi_recv_response(r, hctx) as handler_t;
            if rc_1 as libc::c_uint != HANDLER_GO_ON as libc::c_int as libc::c_uint {
                return rc_1;
            }
            (*hctx).read_ts = log_monotonic_secs;
            fdevent_fdnode_event_add((*hctx).ev, (*hctx).fdn, 0x1 as libc::c_int);
        }
    }
    let cq: *mut chunkqueue = &mut (*r).reqbody_queue;
    if (*cq).bytes_in != (*r).reqbody_length {
        if chunkqueue_length(cq)
            > (65536 as libc::c_int - 4096 as libc::c_int) as libc::c_long
            && (*r).conf.stream_request_body as libc::c_int
                & (1 as libc::c_int) << 1 as libc::c_int != 0
        {
            (*r)
                .conf
                .stream_request_body = ((*r).conf.stream_request_body as libc::c_int
                & !((1 as libc::c_int) << 15 as libc::c_int)) as libc::c_ushort;
            if -(1 as libc::c_int) != (*hctx).fd {
                return HANDLER_WAIT_FOR_EVENT;
            }
        } else {
            let mut rc_2: handler_t = ((*(*r).con).reqbody_read)
                .expect("non-null function pointer")(r);
            if chunkqueue_is_empty(cq) == 0 {
                if (if !((*hctx).fdntocgi).is_null() {
                    (*(*hctx).fdntocgi).events
                } else {
                    0 as libc::c_int
                }) & 0x4 as libc::c_int != 0
                {
                    return (if rc_2 as libc::c_uint
                        == HANDLER_GO_ON as libc::c_int as libc::c_uint
                    {
                        HANDLER_WAIT_FOR_EVENT as libc::c_int as libc::c_uint
                    } else {
                        rc_2 as libc::c_uint
                    }) as handler_t;
                }
            }
            if rc_2 as libc::c_uint != HANDLER_GO_ON as libc::c_int as libc::c_uint {
                return rc_2;
            }
            if -(1 as libc::c_int) as libc::c_long == (*r).reqbody_length {
                return (if (*r).conf.stream_request_body as libc::c_int
                    & (1 as libc::c_int) << 0 as libc::c_int != 0
                {
                    http_response_reqbody_read_error(r, 411 as libc::c_int)
                        as libc::c_uint
                } else {
                    HANDLER_WAIT_FOR_EVENT as libc::c_int as libc::c_uint
                }) as handler_t;
            }
        }
    }
    if -(1 as libc::c_int) == (*hctx).fd {
        if cgi_create_env(r, p, hctx, (*hctx).cgi_handler) != 0 {
            (*r).http_status = 500 as libc::c_int;
            (*r).handler_module = 0 as *const plugin;
            return HANDLER_FINISHED;
        }
    } else if chunkqueue_is_empty(cq) == 0 {
        if 0 as libc::c_int != cgi_write_request(hctx, (*hctx).fdtocgi) {
            cgi_connection_close(hctx);
            return HANDLER_ERROR;
        }
    }
    return HANDLER_WAIT_FOR_EVENT;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn cgi_trigger_hctx_timeout(
    hctx: *mut handler_ctx,
    msg: *const libc::c_char,
) {
    let r: *mut request_st = (*hctx).r;
    connection_jq_append((*r).con);
    log_error(
        (*r).conf.errh,
        b"src/mod_cgi.c\0" as *const u8 as *const libc::c_char,
        1114 as libc::c_int as libc::c_uint,
        b"%s timeout on CGI: %s (pid: %lld)\0" as *const u8 as *const libc::c_char,
        msg,
        (*r).physical.path.ptr,
        (*(*hctx).cgi_pid).pid as libc::c_longlong,
    );
    if *msg as libc::c_int == 'w' as i32 {
        let mut rc: handler_t = cgi_recv_response(r, hctx) as handler_t;
        if rc as libc::c_uint != HANDLER_GO_ON as libc::c_int as libc::c_uint {
            return;
        }
    }
    if 0 as libc::c_int == (*r).http_status {
        (*r).http_status = 504 as libc::c_int;
    }
    cgi_connection_close(hctx);
}
unsafe extern "C" fn cgi_trigger_cb(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mono: unix_time64_t = log_monotonic_secs;
    let p: *mut plugin_data = p_d as *mut plugin_data;
    let mut cgi_pid: *mut cgi_pid_t = (*p).cgi_pid;
    while !cgi_pid.is_null() {
        let hctx: *mut handler_ctx = (*cgi_pid).hctx;
        if !hctx.is_null() {
            let limits: *const cgi_limits = (*hctx).conf.limits;
            if !limits.is_null() {
                if (*limits).read_timeout != 0 && !((*hctx).fdn).is_null()
                    && (if !((*hctx).fdn).is_null() {
                        (*(*hctx).fdn).events
                    } else {
                        0 as libc::c_int
                    }) & 0x1 as libc::c_int != 0
                    && mono - (*hctx).read_ts > (*limits).read_timeout
                {
                    cgi_trigger_hctx_timeout(
                        hctx,
                        b"read\0" as *const u8 as *const libc::c_char,
                    );
                } else if (*limits).write_timeout != 0 && !((*hctx).fdntocgi).is_null()
                        && (if !((*hctx).fdntocgi).is_null() {
                            (*(*hctx).fdntocgi).events
                        } else {
                            0 as libc::c_int
                        }) & 0x4 as libc::c_int != 0
                        && mono - (*hctx).write_ts > (*limits).write_timeout
                    {
                    cgi_trigger_hctx_timeout(
                        hctx,
                        b"write\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        cgi_pid = (*cgi_pid).next;
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn cgi_waitpid_cb(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
    mut pid: pid_t,
    mut status: libc::c_int,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    let mut cgi_pid: *mut cgi_pid_t = (*p).cgi_pid;
    while !cgi_pid.is_null() {
        if pid != (*cgi_pid).pid {
            cgi_pid = (*cgi_pid).next;
        } else {
            let hctx: *mut handler_ctx = (*cgi_pid).hctx;
            if !hctx.is_null() {
                (*hctx).cgi_pid = 0 as *mut cgi_pid_t;
            }
            if !(status & 0x7f as libc::c_int == 0 as libc::c_int) {
                if ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
                    as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
                {
                    if status & 0x7f as libc::c_int != (*cgi_pid).signal_sent {
                        let mut errh: *mut log_error_st = if !hctx.is_null() {
                            (*(*hctx).r).conf.errh
                        } else {
                            (*srv).errh
                        };
                        log_error(
                            errh,
                            b"src/mod_cgi.c\0" as *const u8 as *const libc::c_char,
                            1176 as libc::c_int as libc::c_uint,
                            b"CGI pid %d died with signal %d\0" as *const u8
                                as *const libc::c_char,
                            pid,
                            status & 0x7f as libc::c_int,
                        );
                    }
                } else {
                    let mut errh_0: *mut log_error_st = if !hctx.is_null() {
                        (*(*hctx).r).conf.errh
                    } else {
                        (*srv).errh
                    };
                    log_error(
                        errh_0,
                        b"src/mod_cgi.c\0" as *const u8 as *const libc::c_char,
                        1182 as libc::c_int as libc::c_uint,
                        b"CGI pid %d ended unexpectedly\0" as *const u8
                            as *const libc::c_char,
                        pid,
                    );
                }
            }
            cgi_pid_del(p, cgi_pid);
            return HANDLER_FINISHED;
        }
    }
    return HANDLER_GO_ON;
}
#[no_mangle]
pub unsafe extern "C" fn mod_cgi_plugin_init(mut p: *mut plugin) -> libc::c_int {
    (*p).version = 0x10440 as libc::c_int as size_t;
    (*p).name = b"cgi\0" as *const u8 as *const libc::c_char;
    (*p)
        .handle_request_reset = Some(
        cgi_connection_close_callback
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_subrequest_start = Some(
        cgi_is_handled
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_subrequest = Some(
        mod_cgi_handle_subrequest
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_trigger = Some(
        cgi_trigger_cb
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_waitpid = Some(
        cgi_waitpid_cb
            as unsafe extern "C" fn(
                *mut server,
                *mut libc::c_void,
                pid_t,
                libc::c_int,
            ) -> handler_t,
    );
    (*p)
        .init = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
    >(Some(mod_cgi_init as unsafe extern "C" fn() -> *mut libc::c_void));
    (*p).cleanup = Some(mod_cgi_free as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*p)
        .set_defaults = Some(
        mod_cgi_set_defaults
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    return 0 as libc::c_int;
}
pub unsafe fn run_static_initializers() {
    cpk = [
        {
            let mut init = config_plugin_keys_t {
                k: b"cgi.assign\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVSTRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"cgi.execute-x-only\0" as *const u8 as *const libc::c_char,
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
                k: b"cgi.x-sendfile\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"cgi.x-sendfile-docroot\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_VLIST as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"cgi.local-redir\0" as *const u8 as *const libc::c_char,
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
                k: b"cgi.upgrade\0" as *const u8 as *const libc::c_char,
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
                k: b"cgi.limits\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVANY as libc::c_int as uint8_t,
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
