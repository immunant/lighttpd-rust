use ::libc;
extern "C" {
    pub type pcre2_real_match_data_8;
    pub type h2con;
    pub type fdevents;
    pub type pcre2_real_general_context_8;
    pub type dirent;
    fn array_init(n: uint32_t) -> *mut array;
    fn array_copy_array(dst: *mut array, src: *const array);
    fn array_free(a: *mut array);
    fn array_insert_unique(a: *mut array, entry: *mut data_unset);
    fn array_get_element_klen(
        a: *const array,
        key: *const libc::c_char,
        klen: uint32_t,
    ) -> *const data_unset;
    fn array_get_int_ptr(
        a: *mut array,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut libc::c_int;
    fn buffer_init() -> *mut buffer;
    fn buffer_free(b: *mut buffer);
    fn buffer_string_prepare_append(b: *mut buffer, size: size_t) -> *mut libc::c_char;
    fn buffer_extend(b: *mut buffer, x: size_t) -> *mut libc::c_char;
    fn buffer_commit(b: *mut buffer, size: size_t);
    fn buffer_copy_string(b: *mut buffer, s: *const libc::c_char);
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_copy_string_len_lc(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_string(b: *mut buffer, s: *const libc::c_char);
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
    fn buffer_append_int(b: *mut buffer, val: intmax_t);
    fn buffer_eq_slen(
        b: *const buffer,
        s: *const libc::c_char,
        slen: size_t,
    ) -> libc::c_int;
    fn buffer_is_equal(a: *const buffer, b: *const buffer) -> libc::c_int;
    fn buffer_to_upper(b: *mut buffer);
    fn buffer_append_path_len(b: *mut buffer, a: *const libc::c_char, alen: size_t);
    fn buffer_copy_path_len2(
        b: *mut buffer,
        s1: *const libc::c_char,
        len1: size_t,
        s2: *const libc::c_char,
        len2: size_t,
    );
    fn ck_memclear_s(s: *mut libc::c_void, smax: rsize_t, n: rsize_t) -> errno_t;
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn array_get_buf_ptr(
        a: *mut array,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn array_insert_value(a: *mut array, v: *const libc::c_char, vlen: uint32_t);
    fn fdlog_open(fn_0: *const libc::c_char) -> *mut fdlog_st;
    fn fdlog_closeall(errh: *mut fdlog_st);
    fn fdlog_pipe_serrh(fd: libc::c_int);
    fn chunk_buffer_acquire() -> *mut buffer;
    fn chunk_buffer_release(b: *mut buffer);
    fn chunk_buffer_prepare_append(b: *mut buffer, sz: size_t) -> size_t;
    fn chunkqueue_set_chunk_size(sz: size_t);
    fn chunkqueue_set_tempdirs_default(
        tempdirs: *const array,
        upload_temp_file_size: off_t,
    );
    fn http_request_host_normalize(
        b: *mut buffer,
        scheme_port: libc::c_int,
    ) -> libc::c_int;
    fn sock_addr_inet_pton(
        saddr: *mut sock_addr,
        str: *const libc::c_char,
        family: libc::c_int,
        port: libc::c_ushort,
    ) -> libc::c_int;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fdevent_config(
        event_handler_name: *mut *const libc::c_char,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn fdevent_setfd_cloexec(fd: libc::c_int);
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
    fn fdevent_load_file(
        fn_0: *const libc::c_char,
        lim: *mut off_t,
        errh: *mut log_error_st,
        malloc_fn: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
        free_fn: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> *mut libc::c_char;
    fn pcre_keyvalue_burl_normalize_key(k: *mut buffer, t: *mut buffer);
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
    fn config_plugin_values_init(
        srv: *mut server,
        p_d: *mut libc::c_void,
        cpk: *const config_plugin_keys_t,
        mname: *const libc::c_char,
    ) -> libc::c_int;
    fn data_config_init() -> *mut data_config;
    fn configparserAlloc(
        mallocProc: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    ) -> *mut libc::c_void;
    fn config_feature_bool(
        srv: *const server,
        feature: *const libc::c_char,
        default_value: libc::c_int,
    ) -> libc::c_int;
    fn data_config_pcre_compile(
        dc: *mut data_config,
        pcre_jit: libc::c_int,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn configparser(
        yyp: *mut libc::c_void,
        yymajor: libc::c_int,
        yyminor: *mut buffer,
        ctx: *mut config_t,
    );
    fn configparserFree(
        p: *mut libc::c_void,
        freeProc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
    fn config_plugin_value_tobool(
        du: *const data_unset,
        default_value: libc::c_int,
    ) -> libc::c_int;
    fn config_check_cond(r: *mut request_st, context_ndx: libc::c_int) -> libc::c_int;
    fn vector_free(data: *mut libc::c_void);
    fn request_config_set_defaults(config_defaults: *const request_config);
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn stat_cache_choose_engine(
        stat_cache_string: *const buffer,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn stat_cache_xattrname(name: *const libc::c_char);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn glob(
        __pattern: *const libc::c_char,
        __flags: libc::c_int,
        __errfunc: Option::<
            unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int,
        >,
        __pglob: *mut glob_t,
    ) -> libc::c_int;
    fn globfree(__pglob: *mut glob_t);
    fn closelog();
    fn openlog(
        __ident: *const libc::c_char,
        __option: libc::c_int,
        __facility: libc::c_int,
    );
    fn pcre2_match_data_free_8(_: *mut pcre2_match_data_8);
    fn pcre2_match_data_create_8(
        _: uint32_t,
        _: *mut pcre2_general_context_8,
    ) -> *mut pcre2_match_data_8;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
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
pub type rsize_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_integer {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
    pub value: libc::c_int,
}
pub type burl_opts_e = libc::c_uint;
pub const HTTP_PARSEOPT_METHOD_GET_BODY: burl_opts_e = 32768;
pub const HTTP_PARSEOPT_URL_NORMALIZE_QUERY_20_PLUS: burl_opts_e = 4096;
pub const HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REJECT: burl_opts_e = 2048;
pub const HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REMOVE: burl_opts_e = 1024;
pub const HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_REJECT: burl_opts_e = 512;
pub const HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_DECODE: burl_opts_e = 256;
pub const HTTP_PARSEOPT_URL_NORMALIZE_PATH_BACKSLASH_TRANS: burl_opts_e = 128;
pub const HTTP_PARSEOPT_URL_NORMALIZE_CTRLS_REJECT: burl_opts_e = 64;
pub const HTTP_PARSEOPT_URL_NORMALIZE_REQUIRED: burl_opts_e = 32;
pub const HTTP_PARSEOPT_URL_NORMALIZE_UNRESERVED: burl_opts_e = 16;
pub const HTTP_PARSEOPT_URL_NORMALIZE: burl_opts_e = 8;
pub const HTTP_PARSEOPT_HOST_NORMALIZE: burl_opts_e = 4;
pub const HTTP_PARSEOPT_HOST_STRICT: burl_opts_e = 2;
pub const HTTP_PARSEOPT_HEADER_STRICT: burl_opts_e = 1;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const ETAG_USE_SIZE: C2RustUnnamed_5 = 4;
pub const ETAG_USE_MTIME: C2RustUnnamed_5 = 2;
pub const ETAG_USE_INODE: C2RustUnnamed_5 = 1;
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
pub struct data_config {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
    pub context_ndx: libc::c_int,
    pub comp: comp_key_t,
    pub cond: config_cond_t,
    pub parent: *mut data_config,
    pub prev: *mut data_config,
    pub next: *mut data_config,
    pub string: buffer,
    pub code: *mut libc::c_void,
    pub match_data: *mut pcre2_real_match_data_8,
    pub capture_idx: libc::c_int,
    pub ext: libc::c_int,
    pub comp_tag: buffer,
    pub comp_key: *const libc::c_char,
    pub children: vector_config_weak,
    pub value: *mut array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vector_config_weak {
    pub data: *mut *mut data_config,
    pub used: size_t,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_data_base {
    pub id: libc::c_int,
    pub nconfig: libc::c_int,
    pub cvlist: *mut config_plugin_value_t,
    pub self_0: *mut plugin,
    pub defaults: request_config,
}
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
pub struct config_plugin_keys_t {
    pub k: *const libc::c_char,
    pub klen: uint8_t,
    pub ktype: uint8_t,
    pub scope: uint8_t,
}
pub const T_CONFIG_SCOPE_UNSET: C2RustUnnamed_6 = 0;
pub const T_CONFIG_SCOPE_CONNECTION: C2RustUnnamed_6 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_data_base {
    pub id: libc::c_int,
    pub nconfig: libc::c_int,
    pub cvlist: *mut config_plugin_value_t,
    pub self_0: *mut plugin,
}
pub const T_CONFIG_SCOPE_SERVER: C2RustUnnamed_6 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_t {
    pub srv: *mut server,
    pub ok: libc::c_int,
    pub all_configs: *mut array,
    pub configs_stack: vector_config_weak,
    pub current: *mut data_config,
    pub basedir: *mut buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tokenizer_t {
    pub source: *const libc::c_char,
    pub input: *const libc::c_char,
    pub offset: size_t,
    pub size: size_t,
    pub line_pos: libc::c_int,
    pub line: libc::c_int,
    pub in_key: libc::c_int,
    pub in_brace: libc::c_int,
    pub in_cond: libc::c_int,
    pub simulate_eol: libc::c_int,
}
pub const _ISalnum: C2RustUnnamed_7 = 8;
pub const _ISdigit: C2RustUnnamed_7 = 2048;
pub const _ISalpha: C2RustUnnamed_7 = 1024;
pub type pcre2_match_data_8 = pcre2_real_match_data_8;
pub type pcre2_general_context_8 = pcre2_real_general_context_8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct facility_name_st {
    pub name: *const libc::c_char,
    pub val: libc::c_int,
}
pub type C2RustUnnamed_6 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut libc::c_char,
    pub gl_offs: __size_t,
    pub gl_flags: libc::c_int,
    pub gl_closedir: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub gl_readdir: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut dirent>,
    pub gl_opendir: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void,
    >,
    pub gl_lstat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
    pub gl_stat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
}
pub type __size_t = libc::c_ulong;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const _ISpunct: C2RustUnnamed_7 = 4;
pub const _IScntrl: C2RustUnnamed_7 = 2;
pub const _ISblank: C2RustUnnamed_7 = 1;
pub const _ISgraph: C2RustUnnamed_7 = 32768;
pub const _ISprint: C2RustUnnamed_7 = 16384;
pub const _ISspace: C2RustUnnamed_7 = 8192;
pub const _ISxdigit: C2RustUnnamed_7 = 4096;
pub const _ISlower: C2RustUnnamed_7 = 512;
pub const _ISupper: C2RustUnnamed_7 = 256;
#[inline]
unsafe extern "C" fn buffer_clear(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
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
unsafe extern "C" fn buffer_copy_buffer(mut b: *mut buffer, mut src: *const buffer) {
    buffer_copy_string_len(b, (*src).ptr, buffer_clen(src) as size_t);
}
#[inline]
unsafe extern "C" fn buffer_append_buffer(mut b: *mut buffer, mut src: *const buffer) {
    buffer_append_string_len(b, (*src).ptr, buffer_clen(src) as size_t);
}
#[inline]
unsafe extern "C" fn buffer_truncate(mut b: *mut buffer, mut len: uint32_t) {
    *((*b).ptr).offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    (*b).used = len.wrapping_add(1 as libc::c_int as libc::c_uint);
}
#[inline]
unsafe extern "C" fn ck_memzero(mut s: *mut libc::c_void, mut n: rsize_t) -> errno_t {
    return ck_memclear_s(s, n, n);
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
unsafe extern "C" fn vector_config_weak_clear(mut v: *mut vector_config_weak) {
    if (::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*mut data_config) -> ()>,
    >(0 as *mut libc::c_void))
        .is_some()
    {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < (*v).used {
            (::std::mem::transmute::<
                *mut libc::c_void,
                Option::<unsafe extern "C" fn(*mut data_config) -> ()>,
            >(0 as *mut libc::c_void))
                .expect("non-null function pointer")(*((*v).data).offset(i as isize));
            i = i.wrapping_add(1);
        }
    }
    vector_free((*v).data as *mut libc::c_void);
    vector_config_weak_init(v);
}
#[inline]
unsafe extern "C" fn vector_config_weak_init(mut v: *mut vector_config_weak) {
    (*v).data = 0 as *mut *mut data_config;
    (*v).size = 0 as libc::c_int as size_t;
    (*v).used = (*v).size;
}
unsafe extern "C" fn config_free_config(p_d: *mut libc::c_void) {
    let p: *mut plugin_data_base = p_d as *mut plugin_data_base;
    if p.is_null() {
        return;
    }
    if ((*p).cvlist).is_null() {
        free(p as *mut libc::c_void);
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
                18 => {
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
    free((*p).cvlist as *mut libc::c_void);
    free(p as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn config_reset_config_bytes_sec(p_d: *mut libc::c_void) {
    let p: *mut plugin_data_base = p_d as *mut plugin_data_base;
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
                18 => {
                    if (*cpv).vtype as libc::c_uint
                        == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
                    {
                        *((*cpv).v.v as *mut off_t)
                            .offset(
                                0 as libc::c_int as isize,
                            ) = 0 as libc::c_int as off_t;
                    }
                }
                _ => {}
            }
            cpv = cpv.offset(1);
        }
        i += 1;
    }
}
unsafe extern "C" fn config_merge_config_cpv(
    pconf: *mut request_config,
    cpv: *const config_plugin_value_t,
) {
    match (*cpv).k_id {
        0 => {
            (*pconf).document_root = (*cpv).v.b;
        }
        1 => {
            (*pconf).server_name = (*cpv).v.b;
        }
        2 => {
            (*pconf).server_tag = (*cpv).v.b;
        }
        3 => {
            (*pconf).max_request_size = (*cpv).v.u;
        }
        4 => {
            (*pconf).max_keep_alive_requests = (*cpv).v.shrt;
        }
        5 => {
            (*pconf).max_keep_alive_idle = (*cpv).v.shrt;
        }
        6 => {
            (*pconf).max_read_idle = (*cpv).v.shrt;
        }
        7 => {
            (*pconf).max_write_idle = (*cpv).v.shrt;
        }
        8 => {
            (*pconf).errorfile_prefix = (*cpv).v.b;
        }
        9 => {
            (*pconf).error_handler = (*cpv).v.b;
        }
        10 => {
            (*pconf).error_handler_404 = (*cpv).v.b;
        }
        11 => {
            (*pconf)
                .error_intercept = (0 as libc::c_int as libc::c_uint != (*cpv).v.u)
                as libc::c_int as libc::c_uchar;
        }
        12 => {
            (*pconf)
                .force_lowercase_filenames = (0 as libc::c_int as libc::c_uint
                != (*cpv).v.u) as libc::c_int as libc::c_uchar;
        }
        13 => {
            (*pconf)
                .follow_symlink = (0 as libc::c_int as libc::c_uint != (*cpv).v.u)
                as libc::c_int as libc::c_uchar;
        }
        14 => {
            (*pconf)
                .allow_http11 = (0 as libc::c_int as libc::c_uint != (*cpv).v.u)
                as libc::c_int as libc::c_uchar;
        }
        15 => {
            (*pconf)
                .range_requests = (0 as libc::c_int as libc::c_uint != (*cpv).v.u)
                as libc::c_int as libc::c_uchar;
        }
        16 => {
            (*pconf).stream_request_body = (*cpv).v.shrt;
        }
        17 => {
            (*pconf).stream_response_body = (*cpv).v.shrt;
        }
        18 => {
            (*pconf)
                .global_bytes_per_second = *((*cpv).v.v as *mut off_t)
                .offset(1 as libc::c_int as isize) as libc::c_uint;
            (*pconf).global_bytes_per_second_cnt_ptr = (*cpv).v.v as *mut off_t;
        }
        19 => {
            (*pconf)
                .bytes_per_second = ((*cpv).v.shrt as libc::c_uint) << 10 as libc::c_int;
        }
        20 => {
            (*pconf).mimetypes = (*cpv).v.a;
        }
        21 => {
            (*pconf)
                .use_xattr = (0 as libc::c_int as libc::c_uint != (*cpv).v.u)
                as libc::c_int as libc::c_uchar;
        }
        22 => {
            if (*cpv).v.u != 0 {
                (*pconf)
                    .etag_flags = ((*pconf).etag_flags as libc::c_int
                    | ETAG_USE_INODE as libc::c_int) as libc::c_uchar;
            } else {
                (*pconf)
                    .etag_flags = ((*pconf).etag_flags as libc::c_int
                    & !(ETAG_USE_INODE as libc::c_int)) as libc::c_uchar;
            };
        }
        23 => {
            if (*cpv).v.u != 0 {
                (*pconf)
                    .etag_flags = ((*pconf).etag_flags as libc::c_int
                    | ETAG_USE_MTIME as libc::c_int) as libc::c_uchar;
            } else {
                (*pconf)
                    .etag_flags = ((*pconf).etag_flags as libc::c_int
                    & !(ETAG_USE_MTIME as libc::c_int)) as libc::c_uchar;
            };
        }
        24 => {
            if (*cpv).v.u != 0 {
                (*pconf)
                    .etag_flags = ((*pconf).etag_flags as libc::c_int
                    | ETAG_USE_SIZE as libc::c_int) as libc::c_uchar;
            } else {
                (*pconf)
                    .etag_flags = ((*pconf).etag_flags as libc::c_int
                    & !(ETAG_USE_SIZE as libc::c_int)) as libc::c_uchar;
            };
        }
        25 => {
            (*pconf)
                .log_condition_handling = (0 as libc::c_int as libc::c_uint
                != (*cpv).v.u) as libc::c_int as libc::c_uchar;
        }
        26 => {
            (*pconf)
                .log_file_not_found = (0 as libc::c_int as libc::c_uint != (*cpv).v.u)
                as libc::c_int as libc::c_uchar;
        }
        27 => {
            (*pconf)
                .log_request_handling = (0 as libc::c_int as libc::c_uint != (*cpv).v.u)
                as libc::c_int as libc::c_uchar;
        }
        28 => {
            (*pconf)
                .log_request_header = (0 as libc::c_int as libc::c_uint != (*cpv).v.u)
                as libc::c_int as libc::c_uchar;
        }
        29 => {
            (*pconf)
                .log_response_header = (0 as libc::c_int as libc::c_uint != (*cpv).v.u)
                as libc::c_int as libc::c_uchar;
        }
        30 => {
            (*pconf)
                .log_timeouts = (0 as libc::c_int as libc::c_uint != (*cpv).v.u)
                as libc::c_int as libc::c_uchar;
        }
        31 => {
            (*pconf)
                .log_state_handling = (0 as libc::c_int as libc::c_uint != (*cpv).v.u)
                as libc::c_int as libc::c_uchar;
        }
        32 => {
            if (*cpv).vtype as libc::c_uint
                == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
            {
                (*pconf).errh = (*cpv).v.v as *mut fdlog_st;
            }
        }
        33 => {
            if (*cpv).vtype as libc::c_uint
                == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
            {
                (*pconf).serrh = (*cpv).v.v as *mut fdlog_st;
            }
        }
        _ => return,
    };
}
unsafe extern "C" fn config_merge_config(
    pconf: *mut request_config,
    mut cpv: *const config_plugin_value_t,
) {
    loop {
        config_merge_config_cpv(pconf, cpv);
        cpv = cpv.offset(1);
        if !((*cpv).k_id != -(1 as libc::c_int)) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn config_patch_config(r: *mut request_st) {
    let p: *mut config_data_base = (*(*r).con).config_data_base as *mut config_data_base;
    let mut i: libc::c_int = 1 as libc::c_int;
    let mut used: libc::c_int = (*p).nconfig;
    while i < used {
        if config_check_cond(
            r,
            (*((*p).cvlist).offset(i as isize)).k_id as uint32_t as libc::c_int,
        ) != 0
        {
            config_merge_config(
                &mut (*r).conf,
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
unsafe extern "C" fn config_burl_normalize_cond(srv: *mut server) {
    let tb: *mut buffer = (*srv).tmp_buf;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*(*srv).config_context).used {
        let config: *mut data_config = *((*(*srv).config_context).data)
            .offset(i as isize) as *mut data_config;
        if !(COMP_HTTP_QUERY_STRING as libc::c_int as libc::c_uint
            != (*config).comp as libc::c_uint)
        {
            match (*config).cond as libc::c_uint {
                3 | 1 => {
                    pcre_keyvalue_burl_normalize_key(&mut (*config).string, tb);
                }
                4 | 2 => {
                    pcre_keyvalue_burl_normalize_key(&mut (*config).string, tb);
                }
                _ => {}
            }
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn config_pcre_keyvalue(srv: *mut server) -> libc::c_int {
    let pcre_jit: libc::c_int = config_feature_bool(
        srv,
        b"server.pcre_jit\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*(*srv).config_context).used {
        let dc: *mut data_config = *((*(*srv).config_context).data).offset(i as isize)
            as *mut data_config;
        if !((*dc).cond as libc::c_uint
            != CONFIG_COND_NOMATCH as libc::c_int as libc::c_uint
            && (*dc).cond as libc::c_uint
                != CONFIG_COND_MATCH as libc::c_int as libc::c_uint)
        {
            if data_config_pcre_compile(dc, pcre_jit, (*srv).errh) == 0 {
                return 0 as libc::c_int;
            }
        }
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn config_check_module_duplicates(mut srv: *mut server) {
    let mut dups: libc::c_int = 0 as libc::c_int;
    let data: *mut *mut data_string = (*(*srv).srvconf.modules).data
        as *mut *mut data_string;
    let used: uint32_t = (*(*srv).srvconf.modules).used;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < used {
        let m: *const buffer = &mut (**data.offset(i as isize)).value;
        let mut j: uint32_t = i.wrapping_add(1 as libc::c_int as libc::c_uint);
        while j < used {
            if buffer_is_equal(m, &mut (**data.offset(j as isize)).value) != 0 {
                dups += 1;
                break;
            } else {
                j = j.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }
    if dups == 0 {
        return;
    }
    let modules: *mut array = array_init(used.wrapping_sub(dups as libc::c_uint));
    let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
    while i_0 < used {
        let m_0: *const buffer = &mut (**data.offset(i_0 as isize)).value;
        let mut j_0: uint32_t = 0;
        j_0 = 0 as libc::c_int as uint32_t;
        while j_0 < (*modules).used {
            let mut n: *mut buffer = &mut (*(*((*modules).data).offset(j_0 as isize)
                as *mut data_string))
                .value;
            if buffer_is_equal(m_0, n) != 0 {
                break;
            }
            j_0 = j_0.wrapping_add(1);
        }
        if j_0 == (*modules).used {
            array_insert_value(modules, (*m_0).ptr, buffer_clen(m_0));
        }
        i_0 = i_0.wrapping_add(1);
    }
    array_free((*srv).srvconf.modules);
    (*srv).srvconf.modules = modules;
}
unsafe extern "C" fn config_has_opt_and_value(
    srv: *const server,
    opt: *const libc::c_char,
    olen: uint32_t,
    v: *const libc::c_char,
    vlen: uint32_t,
) -> *const libc::c_char {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*(*srv).config_context).used {
        let config: *const data_config = *((*(*srv).config_context).data)
            .offset(i as isize) as *const data_config;
        let ds: *const data_string = array_get_element_klen((*config).value, opt, olen)
            as *mut data_string;
        if !ds.is_null()
            && (*ds).type_0 as libc::c_uint == TYPE_STRING as libc::c_int as libc::c_uint
            && buffer_eq_slen(&(*ds).value, v, vlen as size_t) != 0
        {
            return v;
        }
        i = i.wrapping_add(1);
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn config_compat_module_prepend(
    mut srv: *mut server,
    mut module: *const libc::c_char,
    mut len: uint32_t,
) {
    let mut modules: *mut array = array_init(
        ((*(*srv).srvconf.modules).used).wrapping_add(4 as libc::c_int as libc::c_uint),
    );
    array_insert_value(modules, module, len);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*(*srv).srvconf.modules).used {
        let mut ds: *mut data_string = *((*(*srv).srvconf.modules).data)
            .offset(i as isize) as *mut data_string;
        array_insert_value(modules, (*ds).value.ptr, buffer_clen(&mut (*ds).value));
        i = i.wrapping_add(1);
    }
    array_free((*srv).srvconf.modules);
    (*srv).srvconf.modules = modules;
}
unsafe extern "C" fn config_warn_authn_module(
    mut srv: *mut server,
    mut module: *const libc::c_char,
    mut len: uint32_t,
    mut v: *const libc::c_char,
) {
    let tb: *mut buffer = (*srv).tmp_buf;
    buffer_copy_string_len(
        tb,
        b"mod_authn_\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_string_len(tb, module, len as size_t);
    array_insert_value((*srv).srvconf.modules, (*tb).ptr, buffer_clen(tb));
    log_error(
        (*srv).errh,
        b"src/configfile.c\0" as *const u8 as *const libc::c_char,
        353 as libc::c_int as libc::c_uint,
        b"Warning: please add \"mod_authn_%s\" to server.modules list in lighttpd.conf.  A future release of lighttpd 1.4.x will not automatically load mod_authn_%s and lighttpd will fail to start up since your lighttpd.conf uses auth.backend = \"%s\".\0"
            as *const u8 as *const libc::c_char,
        module,
        module,
        v,
    );
}
unsafe extern "C" fn config_compat_module_load(mut srv: *mut server) {
    let mut prepend_mod_indexfile: libc::c_int = 1 as libc::c_int;
    let mut append_mod_dirlisting: libc::c_int = 1 as libc::c_int;
    let mut append_mod_staticfile: libc::c_int = 1 as libc::c_int;
    let mut append_mod_authn_file: libc::c_int = 1 as libc::c_int;
    let mut append_mod_authn_ldap: libc::c_int = 1 as libc::c_int;
    let mut append_mod_openssl: libc::c_int = 1 as libc::c_int;
    let mut contains_mod_auth: libc::c_int = 0 as libc::c_int;
    let mut prepend_mod_auth: libc::c_int = 0 as libc::c_int;
    let mut prepend_mod_vhostdb: libc::c_int = 0 as libc::c_int;
    let mut dyn_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*(*srv).srvconf.modules).used {
        let mut m: *mut buffer = &mut (*(*((*(*srv).srvconf.modules).data)
            .offset(i as isize) as *mut data_string))
            .value;
        if buffer_eq_slen(
            m,
            b"mod_indexfile\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            prepend_mod_indexfile = 0 as libc::c_int;
        } else if buffer_eq_slen(
                m,
                b"mod_staticfile\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            append_mod_staticfile = 0 as libc::c_int;
        } else if buffer_eq_slen(
                m,
                b"mod_dirlisting\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            append_mod_dirlisting = 0 as libc::c_int;
        } else if buffer_eq_slen(
                m,
                b"mod_gnutls\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            append_mod_openssl = 0 as libc::c_int;
        } else if buffer_eq_slen(
                m,
                b"mod_mbedtls\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            append_mod_openssl = 0 as libc::c_int;
        } else if buffer_eq_slen(
                m,
                b"mod_nss\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            append_mod_openssl = 0 as libc::c_int;
        } else if buffer_eq_slen(
                m,
                b"mod_openssl\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            append_mod_openssl = 0 as libc::c_int;
        } else if buffer_eq_slen(
                m,
                b"mod_wolfssl\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            append_mod_openssl = 0 as libc::c_int;
        } else if 0 as libc::c_int
                == strncmp(
                    (*m).ptr,
                    b"mod_auth\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
            {
            if buffer_eq_slen(
                m,
                b"mod_auth\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
                if contains_mod_auth == 0 {
                    contains_mod_auth = 1 as libc::c_int;
                    if !dyn_name.is_null() {
                        log_error(
                            (*srv).errh,
                            b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                            397 as libc::c_int as libc::c_uint,
                            b"Warning: mod_auth should be listed in server.modules before dynamic backends such as %s\0"
                                as *const u8 as *const libc::c_char,
                            dyn_name,
                        );
                    }
                }
            } else if contains_mod_auth == 0 {
                prepend_mod_auth = 1 as libc::c_int;
            }
            if buffer_eq_slen(
                m,
                b"mod_authn_file\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
                append_mod_authn_file = 0 as libc::c_int;
            } else if buffer_eq_slen(
                    m,
                    b"mod_authn_ldap\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                ) != 0
                {
                append_mod_authn_ldap = 0 as libc::c_int;
            }
        } else if 0 as libc::c_int
                == strncmp(
                    (*m).ptr,
                    b"mod_vhostdb\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
            {
            if buffer_eq_slen(
                m,
                b"mod_vhostdb\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
                prepend_mod_vhostdb |= 2 as libc::c_int;
            } else if prepend_mod_vhostdb & 2 as libc::c_int == 0 {
                prepend_mod_vhostdb |= 1 as libc::c_int;
            }
        } else if 0 as libc::c_int
                == strncmp(
                    (*m).ptr,
                    b"mod_ajp13\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
                || 0 as libc::c_int
                    == strncmp(
                        (*m).ptr,
                        b"mod_cgi\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    )
                || 0 as libc::c_int
                    == strncmp(
                        (*m).ptr,
                        b"mod_fastcgi\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    )
                || 0 as libc::c_int
                    == strncmp(
                        (*m).ptr,
                        b"mod_proxy\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    )
                || 0 as libc::c_int
                    == strncmp(
                        (*m).ptr,
                        b"mod_scgi\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    )
                || 0 as libc::c_int
                    == strncmp(
                        (*m).ptr,
                        b"mod_sockproxy\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    )
                || 0 as libc::c_int
                    == strncmp(
                        (*m).ptr,
                        b"mod_wstunnel\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    )
            {
            if dyn_name.is_null() {
                dyn_name = (*m).ptr;
            }
        }
        i = i.wrapping_add(1);
    }
    if prepend_mod_indexfile != 0 {
        config_compat_module_prepend(
            srv,
            b"mod_indexfile\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    if append_mod_dirlisting != 0 {
        array_insert_value(
            (*srv).srvconf.modules,
            b"mod_dirlisting\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    if append_mod_staticfile != 0 {
        array_insert_value(
            (*srv).srvconf.modules,
            b"mod_staticfile\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    append_mod_openssl != 0;
    if contains_mod_auth != 0 {
        if append_mod_authn_file != 0 {
            let mut v: *const libc::c_char = 0 as *const libc::c_char;
            v = config_has_opt_and_value(
                srv,
                b"auth.backend\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                b"htdigest\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            );
            if !v.is_null()
                || {
                    v = config_has_opt_and_value(
                        srv,
                        b"auth.backend\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                        b"htpasswd\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    );
                    !v.is_null()
                }
                || {
                    v = config_has_opt_and_value(
                        srv,
                        b"auth.backend\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                        b"plain\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    );
                    !v.is_null()
                }
            {
                config_warn_authn_module(
                    srv,
                    b"file\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    v,
                );
            }
        }
        append_mod_authn_ldap != 0;
    }
    if prepend_mod_auth != 0 {
        config_compat_module_prepend(
            srv,
            b"mod_auth\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    if prepend_mod_vhostdb & 1 as libc::c_int != 0 {
        config_compat_module_prepend(
            srv,
            b"mod_vhostdb\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
}
unsafe extern "C" fn config_deprecate_module_compress(mut srv: *mut server) {
    let mut mod_compress_idx: libc::c_int = -(1 as libc::c_int);
    let mut mod_deflate_idx: libc::c_int = -(1 as libc::c_int);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*(*srv).srvconf.modules).used {
        let mut m: *mut buffer = &mut (*(*((*(*srv).srvconf.modules).data)
            .offset(i as isize) as *mut data_string))
            .value;
        if buffer_eq_slen(
            m,
            b"mod_compress\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            mod_compress_idx = i as libc::c_int;
        } else if buffer_eq_slen(
                m,
                b"mod_deflate\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            mod_deflate_idx = i as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    if mod_compress_idx < 0 as libc::c_int {
        return;
    }
    let mut has_compress_directive: libc::c_int = 0 as libc::c_int;
    let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
    while i_0 < (*(*srv).config_context).used {
        let mut config: *const data_config = *((*(*srv).config_context).data)
            .offset(i_0 as isize) as *const data_config;
        let mut j: uint32_t = 0 as libc::c_int as uint32_t;
        while j < (*(*config).value).used {
            let mut k: *mut buffer = &mut (**((*(*config).value).data)
                .offset(j as isize))
                .key;
            if 0 as libc::c_int
                == strncmp(
                    (*k).ptr,
                    b"compress.\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
            {
                has_compress_directive = 1 as libc::c_int;
                break;
            } else {
                j = j.wrapping_add(1);
            }
        }
        if has_compress_directive != 0 {
            log_error(
                (*srv).errh,
                b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                514 as libc::c_int as libc::c_uint,
                b"Warning: \"mod_compress\" is DEPRECATED and has been replaced with \"mod_deflate\".  A future release of lighttpd 1.4.x will not contain mod_compress and lighttpd may fail to start up\0"
                    as *const u8 as *const libc::c_char,
            );
            break;
        } else {
            i_0 = i_0.wrapping_add(1);
        }
    }
    if mod_deflate_idx >= 0 as libc::c_int || has_compress_directive == 0 {
        let mut a: *mut array = array_init(
            ((*(*srv).srvconf.modules).used)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        let mut i_1: uint32_t = 0 as libc::c_int as uint32_t;
        while i_1 < (*(*srv).srvconf.modules).used {
            let mut m_0: *mut buffer = &mut (*(*((*(*srv).srvconf.modules).data)
                .offset(i_1 as isize) as *mut data_string))
                .value;
            if !(buffer_eq_slen(
                m_0,
                b"mod_compress\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0)
            {
                array_insert_value(a, (*m_0).ptr, buffer_clen(m_0));
            }
            i_1 = i_1.wrapping_add(1);
        }
        array_free((*srv).srvconf.modules);
        (*srv).srvconf.modules = a;
    } else {
        let mut m_1: *mut buffer = &mut (*(*((*(*srv).srvconf.modules).data)
            .offset(mod_compress_idx as isize) as *mut data_string))
            .value;
        buffer_copy_string_len(
            m_1,
            b"mod_deflate\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    };
}
unsafe extern "C" fn config_http_parseopts(
    mut srv: *mut server,
    mut a: *const array,
) -> libc::c_int {
    let mut opts: libc::c_ushort = (*srv).srvconf.http_url_normalize;
    let mut decode_2f: libc::c_ushort = 1 as libc::c_int as libc::c_ushort;
    let mut rc: libc::c_int = 1 as libc::c_int;
    let mut current_block_25: u64;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < (*a).used as libc::c_ulong {
        let ds: *const data_string = *((*a).data).offset(i as isize)
            as *const data_string;
        let mut k: *const buffer = &(*ds).key;
        let mut opt: libc::c_ushort = 0;
        let mut val: libc::c_int = config_plugin_value_tobool(
            ds as *mut data_unset,
            2 as libc::c_int,
        );
        if 2 as libc::c_int == val {
            log_error(
                (*srv).errh,
                b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                551 as libc::c_int as libc::c_uint,
                b"unrecognized value for server.http-parseopts: %s => %s (expect \"[enable|disable]\")\0"
                    as *const u8 as *const libc::c_char,
                (*k).ptr,
                (*ds).value.ptr,
            );
            rc = 0 as libc::c_int;
        }
        if buffer_eq_slen(
            k,
            b"url-normalize\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            opt = HTTP_PARSEOPT_URL_NORMALIZE as libc::c_int as libc::c_ushort;
            current_block_25 = 7659304154607701039;
        } else if buffer_eq_slen(
                k,
                b"url-normalize-unreserved\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            opt = HTTP_PARSEOPT_URL_NORMALIZE_UNRESERVED as libc::c_int
                as libc::c_ushort;
            current_block_25 = 7659304154607701039;
        } else if buffer_eq_slen(
                k,
                b"url-normalize-required\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            opt = HTTP_PARSEOPT_URL_NORMALIZE_REQUIRED as libc::c_int as libc::c_ushort;
            current_block_25 = 7659304154607701039;
        } else if buffer_eq_slen(
                k,
                b"url-ctrls-reject\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            opt = HTTP_PARSEOPT_URL_NORMALIZE_CTRLS_REJECT as libc::c_int
                as libc::c_ushort;
            current_block_25 = 7659304154607701039;
        } else if buffer_eq_slen(
                k,
                b"url-path-backslash-trans\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            opt = HTTP_PARSEOPT_URL_NORMALIZE_PATH_BACKSLASH_TRANS as libc::c_int
                as libc::c_ushort;
            current_block_25 = 7659304154607701039;
        } else if buffer_eq_slen(
                k,
                b"url-path-2f-decode\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            opt = HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_DECODE as libc::c_int
                as libc::c_ushort;
            current_block_25 = 7659304154607701039;
        } else if buffer_eq_slen(
                k,
                b"url-path-2f-reject\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            opt = HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_REJECT as libc::c_int
                as libc::c_ushort;
            current_block_25 = 7659304154607701039;
        } else if buffer_eq_slen(
                k,
                b"url-path-dotseg-remove\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            opt = HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REMOVE as libc::c_int
                as libc::c_ushort;
            current_block_25 = 7659304154607701039;
        } else if buffer_eq_slen(
                k,
                b"url-path-dotseg-reject\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            opt = HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REJECT as libc::c_int
                as libc::c_ushort;
            current_block_25 = 7659304154607701039;
        } else if buffer_eq_slen(
                k,
                b"url-query-20-plus\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            opt = HTTP_PARSEOPT_URL_NORMALIZE_QUERY_20_PLUS as libc::c_int
                as libc::c_ushort;
            current_block_25 = 7659304154607701039;
        } else {
            if buffer_eq_slen(
                k,
                b"header-strict\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
                (*srv).srvconf.http_header_strict = val as libc::c_uchar;
            } else if buffer_eq_slen(
                    k,
                    b"host-strict\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                ) != 0
                {
                (*srv).srvconf.http_host_strict = val as libc::c_uchar;
            } else if buffer_eq_slen(
                    k,
                    b"host-normalize\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                ) != 0
                {
                (*srv).srvconf.http_host_normalize = val as libc::c_uchar;
            } else if buffer_eq_slen(
                    k,
                    b"method-get-body\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                ) != 0
                {
                (*srv).srvconf.http_method_get_body = val as libc::c_uchar;
            } else {
                log_error(
                    (*srv).errh,
                    b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                    593 as libc::c_int as libc::c_uint,
                    b"unrecognized key for server.http-parseopts: %s\0" as *const u8
                        as *const libc::c_char,
                    (*k).ptr,
                );
                rc = 0 as libc::c_int;
            }
            current_block_25 = 14916268686031723178;
        }
        match current_block_25 {
            7659304154607701039 => {
                if val != 0 {
                    opts = (opts as libc::c_int | opt as libc::c_int) as libc::c_ushort;
                } else {
                    opts = (opts as libc::c_int & !(opt as libc::c_int))
                        as libc::c_ushort;
                    if opt as libc::c_int == HTTP_PARSEOPT_URL_NORMALIZE as libc::c_int {
                        opts = 0 as libc::c_int as libc::c_ushort;
                        break;
                    } else if opt as libc::c_int
                            == HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_DECODE as libc::c_int
                        {
                        decode_2f = 0 as libc::c_int as libc::c_ushort;
                    }
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
    if opts as libc::c_int != 0 as libc::c_int {
        opts = (opts as libc::c_int | HTTP_PARSEOPT_URL_NORMALIZE as libc::c_int)
            as libc::c_ushort;
        if opts as libc::c_int
            & (HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_DECODE as libc::c_int
                | HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_REJECT as libc::c_int)
            == HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_DECODE as libc::c_int
                | HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_REJECT as libc::c_int
        {
            log_error(
                (*srv).errh,
                b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                617 as libc::c_int as libc::c_uint,
                b"conflicting options in server.http-parseopts:url-path-2f-decode, url-path-2f-reject\0"
                    as *const u8 as *const libc::c_char,
            );
            rc = 0 as libc::c_int;
        }
        if opts as libc::c_int
            & (HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REMOVE as libc::c_int
                | HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REJECT as libc::c_int)
            == HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REMOVE as libc::c_int
                | HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REJECT as libc::c_int
        {
            log_error(
                (*srv).errh,
                b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                626 as libc::c_int as libc::c_uint,
                b"conflicting options in server.http-parseopts:url-path-dotseg-remove, url-path-dotseg-reject\0"
                    as *const u8 as *const libc::c_char,
            );
            rc = 0 as libc::c_int;
        }
        if opts as libc::c_int
            & (HTTP_PARSEOPT_URL_NORMALIZE_UNRESERVED as libc::c_int
                | HTTP_PARSEOPT_URL_NORMALIZE_REQUIRED as libc::c_int) == 0
        {
            opts = (opts as libc::c_int
                | HTTP_PARSEOPT_URL_NORMALIZE_UNRESERVED as libc::c_int)
                as libc::c_ushort;
            if decode_2f as libc::c_int != 0
                && opts as libc::c_int
                    & HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_REJECT as libc::c_int == 0
            {
                opts = (opts as libc::c_int
                    | HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_DECODE as libc::c_int)
                    as libc::c_ushort;
            }
        }
    }
    (*srv).srvconf.http_url_normalize = opts;
    return rc;
}
static mut cpk: [config_plugin_keys_t; 34] = [config_plugin_keys_t {
    k: 0 as *const libc::c_char,
    klen: 0,
    ktype: 0,
    scope: 0,
}; 34];
unsafe extern "C" fn config_insert_srvconf(mut srv: *mut server) -> libc::c_int {
    (*srv).srvconf.h2proto = 2 as libc::c_int as libc::c_uchar;
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut srvplug: plugin_data_base = plugin_data_base {
        id: 0,
        nconfig: 0,
        cvlist: 0 as *mut config_plugin_value_t,
        self_0: 0 as *mut plugin,
    };
    memset(
        &mut srvplug as *mut plugin_data_base as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<plugin_data_base>() as libc::c_ulong,
    );
    let p: *mut plugin_data_base = &mut srvplug;
    if config_plugin_values_init(
        srv,
        p as *mut libc::c_void,
        cpk.as_ptr(),
        b"global\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return HANDLER_ERROR as libc::c_int;
    }
    let mut ssl_enabled: libc::c_int = 0 as libc::c_int;
    if (*((*p).cvlist).offset(0 as libc::c_int as isize)).v.u2[1 as libc::c_int as usize]
        != 0
    {
        let mut cpv: *mut config_plugin_value_t = ((*p).cvlist)
            .offset(
                (*((*p).cvlist).offset(0 as libc::c_int as isize))
                    .v
                    .u2[0 as libc::c_int as usize] as isize,
            );
        while -(1 as libc::c_int) != (*cpv).k_id {
            match (*cpv).k_id {
                0 => {
                    array_copy_array((*srv).srvconf.modules, (*cpv).v.a);
                }
                1 => {
                    (*srv)
                        .srvconf
                        .compat_module_load = (*cpv).v.u as libc::c_ushort
                        as libc::c_uchar;
                }
                2 => {
                    (*srv)
                        .srvconf
                        .systemd_socket_activation = (*cpv).v.u as libc::c_ushort
                        as libc::c_uchar;
                }
                3 => {
                    (*srv).srvconf.port = (*cpv).v.shrt;
                }
                4 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        (*srv).srvconf.bindhost = (*cpv).v.b;
                    }
                }
                5 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        (*srv).srvconf.network_backend = (*cpv).v.b;
                    }
                }
                6 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        (*srv).srvconf.changeroot = (*cpv).v.b;
                    }
                }
                7 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        (*srv).srvconf.username = (*cpv).v.b;
                    }
                }
                8 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        (*srv).srvconf.groupname = (*cpv).v.b;
                    }
                }
                9 => {}
                10 => {}
                11 => {
                    (*srv)
                        .srvconf
                        .errorlog_use_syslog = (*cpv).v.u as libc::c_ushort
                        as libc::c_uchar;
                }
                12 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        (*srv).srvconf.syslog_facility = (*cpv).v.b;
                    }
                }
                13 => {
                    (*srv)
                        .srvconf
                        .enable_cores = (*cpv).v.u as libc::c_ushort as libc::c_uchar;
                }
                14 => {
                    (*srv).srvconf.event_handler = (*(*cpv).v.b).ptr;
                }
                15 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        let ref mut fresh0 = *(&mut (*srv).srvconf.pid_file
                            as *mut *mut buffer as *mut *const buffer);
                        *fresh0 = (*cpv).v.b;
                    }
                }
                16 => {
                    (*srv).srvconf.max_worker = (*cpv).v.u as libc::c_ushort;
                }
                17 => {
                    (*srv).srvconf.max_fds = (*cpv).v.u as libc::c_ushort;
                }
                18 => {
                    (*srv).srvconf.max_conns = (*cpv).v.u as libc::c_ushort;
                }
                19 => {
                    (*srv).srvconf.max_request_field_size = (*cpv).v.u;
                }
                20 => {
                    chunkqueue_set_chunk_size((*cpv).v.u as size_t);
                }
                21 => {
                    (*srv).srvconf.upload_temp_file_size = (*cpv).v.u;
                }
                22 => {
                    array_copy_array((*srv).srvconf.upload_tempdirs, (*cpv).v.a);
                }
                23 => {
                    if config_http_parseopts(srv, (*cpv).v.a) == 0 {
                        rc = HANDLER_ERROR as libc::c_int;
                    }
                }
                24 => {
                    (*srv)
                        .srvconf
                        .http_header_strict = (0 as libc::c_int as libc::c_uint
                        != (*cpv).v.u) as libc::c_int as libc::c_uchar;
                }
                25 => {
                    (*srv)
                        .srvconf
                        .http_host_strict = (0 as libc::c_int as libc::c_uint
                        != (*cpv).v.u) as libc::c_int as libc::c_uchar;
                }
                26 => {
                    (*srv)
                        .srvconf
                        .http_host_normalize = (0 as libc::c_int as libc::c_uint
                        != (*cpv).v.u) as libc::c_int as libc::c_uchar;
                }
                27 => {}
                28 => {
                    if 0 as libc::c_int
                        != stat_cache_choose_engine((*cpv).v.b, (*srv).errh)
                    {
                        rc = HANDLER_ERROR as libc::c_int;
                    }
                }
                29 => {
                    stat_cache_xattrname((*(*cpv).v.b).ptr);
                }
                30 => {
                    ssl_enabled = (0 as libc::c_int as libc::c_uint != (*cpv).v.u)
                        as libc::c_int;
                    if ssl_enabled != 0 {
                        log_error(
                            (*srv).errh,
                            b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                            871 as libc::c_int as libc::c_uint,
                            b"ssl support is missing; recompile with e.g. --with-openssl\0"
                                as *const u8 as *const libc::c_char,
                        );
                        rc = HANDLER_ERROR as libc::c_int;
                    }
                }
                31 => {
                    (*srv)
                        .srvconf
                        .log_request_header_on_error = (0 as libc::c_int as libc::c_uint
                        != (*cpv).v.u) as libc::c_int as libc::c_uchar;
                }
                32 => {
                    (*srv).srvconf.feature_flags = (*cpv).v.a;
                    (*srv)
                        .srvconf
                        .h2proto = config_plugin_value_tobool(
                        array_get_element_klen(
                            (*cpv).v.a,
                            b"server.h2proto\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                        ),
                        1 as libc::c_int,
                    ) as libc::c_uchar;
                    if (*srv).srvconf.h2proto != 0 {
                        (*srv)
                            .srvconf
                            .h2proto = ((*srv).srvconf.h2proto as libc::c_int
                            + config_plugin_value_tobool(
                                array_get_element_klen(
                                    (*cpv).v.a,
                                    b"server.h2c\0" as *const u8 as *const libc::c_char,
                                    (::std::mem::size_of::<[libc::c_char; 11]>()
                                        as libc::c_ulong as uint32_t)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                ),
                                1 as libc::c_int,
                            )) as libc::c_uchar;
                    }
                    (*srv)
                        .srvconf
                        .absolute_dir_redirect = config_plugin_value_tobool(
                        array_get_element_klen(
                            (*cpv).v.a,
                            b"server.absolute-dir-redirect\0" as *const u8
                                as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                        ),
                        0 as libc::c_int,
                    ) as libc::c_uchar;
                }
                _ => {}
            }
            cpv = cpv.offset(1);
        }
    }
    if 0 as libc::c_int == (*srv).srvconf.port as libc::c_int {
        (*srv)
            .srvconf
            .port = (if ssl_enabled != 0 {
            443 as libc::c_int
        } else {
            80 as libc::c_int
        }) as libc::c_ushort;
    }
    config_check_module_duplicates(srv);
    if (*srv).srvconf.compat_module_load != 0 {
        config_compat_module_load(srv);
    }
    config_deprecate_module_compress(srv);
    if (*srv).srvconf.http_url_normalize != 0 {
        config_burl_normalize_cond(srv);
    }
    if config_pcre_keyvalue(srv) == 0 {
        rc = HANDLER_ERROR as libc::c_int;
    }
    free(srvplug.cvlist as *mut libc::c_void);
    return rc;
}
static mut cpk_0: [config_plugin_keys_t; 35] = [config_plugin_keys_t {
    k: 0 as *const libc::c_char,
    klen: 0,
    ktype: 0,
    scope: 0,
}; 35];
unsafe extern "C" fn config_insert(mut srv: *mut server) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    let p: *mut config_data_base = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<config_data_base>() as libc::c_ulong,
    ) as *mut config_data_base;
    if p.is_null() {
        ck_assert_failed(
            b"src/configfile.c\0" as *const u8 as *const libc::c_char,
            1035 as libc::c_int as libc::c_uint,
            b"p\0" as *const u8 as *const libc::c_char,
        );
    }
    (*srv).config_data_base = p as *mut libc::c_void;
    if config_plugin_values_init(
        srv,
        p as *mut libc::c_void,
        cpk_0.as_ptr(),
        b"base\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return HANDLER_ERROR as libc::c_int;
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
            let mut current_block_37: u64;
            match (*cpv).k_id {
                0 => {
                    current_block_37 = 17836213544692497527;
                }
                1 => {
                    if buffer_is_blank((*cpv).v.b) != 0 {
                        (*cpv).v.b = 0 as *const buffer;
                    }
                    current_block_37 = 17836213544692497527;
                }
                2 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        let mut b: *mut buffer = 0 as *mut buffer;
                        let ref mut fresh1 = *(&mut b as *mut *mut buffer
                            as *mut *const buffer);
                        *fresh1 = (*cpv).v.b;
                        let mut t: *mut libc::c_char = strchr((*b).ptr, '\n' as i32);
                        while !t.is_null() {
                            if !(*t.offset(1 as libc::c_int as isize) as libc::c_int
                                == ' ' as i32
                                || *t.offset(1 as libc::c_int as isize) as libc::c_int
                                    == '\t' as i32)
                            {
                                let mut off: off_t = t.offset_from((*b).ptr)
                                    as libc::c_long;
                                let mut len: size_t = buffer_clen(b) as size_t;
                                buffer_string_prepare_append(b, 1 as libc::c_int as size_t);
                                t = ((*b).ptr).offset(off as isize);
                                memmove(
                                    t.offset(2 as libc::c_int as isize) as *mut libc::c_void,
                                    t.offset(1 as libc::c_int as isize) as *const libc::c_void,
                                    len
                                        .wrapping_sub(off as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                );
                                *t
                                    .offset(
                                        1 as libc::c_int as isize,
                                    ) = ' ' as i32 as libc::c_char;
                                buffer_commit(b, 1 as libc::c_int as size_t);
                            }
                            t = strchr(t.offset(2 as libc::c_int as isize), '\n' as i32);
                        }
                        let mut t_0: *mut libc::c_char = (*b).ptr;
                        while *t_0 as libc::c_int == ' ' as i32
                            || *t_0 as libc::c_int == '\t' as i32
                            || *t_0 as libc::c_int == '\r' as i32
                            || *t_0 as libc::c_int == '\n' as i32
                        {
                            t_0 = t_0.offset(1);
                        }
                        if *t_0 as libc::c_int == '\u{0}' as i32 {
                            buffer_truncate(b, 0 as libc::c_int as uint32_t);
                        }
                    } else if 0 as libc::c_int != i {
                        (*cpv).v.b = 0 as *const buffer;
                    }
                    current_block_37 = 17836213544692497527;
                }
                3 => {
                    current_block_37 = 757891641966779454;
                }
                4 => {
                    current_block_37 = 757891641966779454;
                }
                5 => {
                    current_block_37 = 8334645929148943372;
                }
                6 => {
                    current_block_37 = 16449761503251207207;
                }
                7 => {
                    current_block_37 = 17600203686288049616;
                }
                8 => {
                    current_block_37 = 11898363343877628413;
                }
                9 | 10 => {
                    current_block_37 = 11898363343877628413;
                }
                11 => {
                    current_block_37 = 7305379076187230449;
                }
                12 => {
                    current_block_37 = 7305379076187230449;
                }
                13 => {
                    current_block_37 = 17836213544692497527;
                }
                14 => {
                    current_block_37 = 12347726321192918635;
                }
                15 => {
                    current_block_37 = 12347726321192918635;
                }
                16 => {
                    if (*cpv).v.shrt as libc::c_int
                        & (1 as libc::c_int) << 1 as libc::c_int != 0
                    {
                        (*cpv)
                            .v
                            .shrt = ((*cpv).v.shrt as libc::c_int
                            | (1 as libc::c_int) << 0 as libc::c_int) as libc::c_ushort;
                    }
                    current_block_37 = 17836213544692497527;
                }
                17 => {
                    if (*cpv).v.shrt as libc::c_int
                        & (1 as libc::c_int) << 1 as libc::c_int != 0
                    {
                        (*cpv)
                            .v
                            .shrt = ((*cpv).v.shrt as libc::c_int
                            | (1 as libc::c_int) << 0 as libc::c_int) as libc::c_ushort;
                    }
                    current_block_37 = 17836213544692497527;
                }
                18 => {
                    let cnt: *mut off_t = malloc(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<off_t>() as libc::c_ulong,
                            ),
                    ) as *mut off_t;
                    if cnt.is_null() {
                        ck_assert_failed(
                            b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                            1114 as libc::c_int as libc::c_uint,
                            b"cnt\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    *cnt.offset(0 as libc::c_int as isize) = 0 as libc::c_int as off_t;
                    *cnt
                        .offset(
                            1 as libc::c_int as isize,
                        ) = ((*cpv).v.shrt as off_t) << 10 as libc::c_int;
                    (*cpv).v.v = cnt as *mut libc::c_void;
                    (*cpv).vtype = T_CONFIG_LOCAL;
                    current_block_37 = 17836213544692497527;
                }
                19 => {
                    current_block_37 = 18096013668015154698;
                }
                20 => {
                    current_block_37 = 18096013668015154698;
                }
                21 => {
                    current_block_37 = 17839086213769877305;
                }
                22 => {
                    current_block_37 = 1350687994066923870;
                }
                23 => {
                    current_block_37 = 14439643916017301289;
                }
                24 => {
                    current_block_37 = 13234347239804218320;
                }
                25 => {
                    current_block_37 = 9783001691843402906;
                }
                26 => {
                    current_block_37 = 17010805083111289497;
                }
                27 => {
                    current_block_37 = 11600600136415632188;
                }
                28 => {
                    current_block_37 = 3800083325811240045;
                }
                29 => {
                    current_block_37 = 11439929792828129558;
                }
                30 => {
                    current_block_37 = 1105880378241905275;
                }
                31 => {
                    current_block_37 = 7455813144181628641;
                }
                32 => {
                    current_block_37 = 13562573508807214898;
                }
                33 => {
                    current_block_37 = 11133198759790587203;
                }
                _ => {
                    current_block_37 = 17836213544692497527;
                }
            }
            match current_block_37 {
                757891641966779454 => {
                    current_block_37 = 8334645929148943372;
                }
                11898363343877628413 => {
                    if buffer_is_blank((*cpv).v.b) != 0 {
                        (*cpv).v.b = 0 as *const buffer;
                    }
                    current_block_37 = 17836213544692497527;
                }
                7305379076187230449 => {
                    current_block_37 = 17836213544692497527;
                }
                12347726321192918635 => {
                    current_block_37 = 17836213544692497527;
                }
                18096013668015154698 => {
                    current_block_37 = 17839086213769877305;
                }
                _ => {}
            }
            match current_block_37 {
                8334645929148943372 => {
                    current_block_37 = 16449761503251207207;
                }
                17839086213769877305 => {
                    current_block_37 = 1350687994066923870;
                }
                _ => {}
            }
            match current_block_37 {
                16449761503251207207 => {
                    current_block_37 = 17600203686288049616;
                }
                1350687994066923870 => {
                    current_block_37 = 14439643916017301289;
                }
                _ => {}
            }
            match current_block_37 {
                17600203686288049616 => {
                    current_block_37 = 17836213544692497527;
                }
                14439643916017301289 => {
                    current_block_37 = 13234347239804218320;
                }
                _ => {}
            }
            match current_block_37 {
                13234347239804218320 => {
                    current_block_37 = 9783001691843402906;
                }
                _ => {}
            }
            match current_block_37 {
                9783001691843402906 => {
                    current_block_37 = 17010805083111289497;
                }
                _ => {}
            }
            match current_block_37 {
                17010805083111289497 => {
                    current_block_37 = 11600600136415632188;
                }
                _ => {}
            }
            match current_block_37 {
                11600600136415632188 => {
                    current_block_37 = 3800083325811240045;
                }
                _ => {}
            }
            match current_block_37 {
                3800083325811240045 => {
                    current_block_37 = 11439929792828129558;
                }
                _ => {}
            }
            match current_block_37 {
                11439929792828129558 => {
                    current_block_37 = 1105880378241905275;
                }
                _ => {}
            }
            match current_block_37 {
                1105880378241905275 => {
                    current_block_37 = 7455813144181628641;
                }
                _ => {}
            }
            match current_block_37 {
                7455813144181628641 => {
                    current_block_37 = 13562573508807214898;
                }
                _ => {}
            }
            match current_block_37 {
                13562573508807214898 => {
                    current_block_37 = 11133198759790587203;
                }
                _ => {}
            }
            match current_block_37 {
                11133198759790587203 => {}
                _ => {}
            }
            cpv = cpv.offset(1);
        }
        i += 1;
    }
    (*p).defaults.errh = (*srv).errh;
    (*p).defaults.max_keep_alive_requests = 100 as libc::c_int as libc::c_ushort;
    (*p).defaults.max_keep_alive_idle = 5 as libc::c_int as libc::c_ushort;
    (*p).defaults.max_read_idle = 60 as libc::c_int as libc::c_ushort;
    (*p).defaults.max_write_idle = 360 as libc::c_int as libc::c_ushort;
    (*p).defaults.follow_symlink = 1 as libc::c_int as libc::c_uchar;
    (*p).defaults.allow_http11 = 1 as libc::c_int as libc::c_uchar;
    (*p)
        .defaults
        .etag_flags = (ETAG_USE_INODE as libc::c_int | ETAG_USE_MTIME as libc::c_int
        | ETAG_USE_SIZE as libc::c_int) as libc::c_uchar;
    (*p).defaults.range_requests = 1 as libc::c_int as libc::c_uchar;
    (*p).defaults.force_lowercase_filenames = 2 as libc::c_int as libc::c_uchar;
    (*p)
        .defaults
        .http_parseopts = ((if (*srv).srvconf.http_header_strict as libc::c_int != 0 {
        HTTP_PARSEOPT_HEADER_STRICT as libc::c_int
    } else {
        0 as libc::c_int
    })
        | (if (*srv).srvconf.http_host_strict as libc::c_int != 0 {
            HTTP_PARSEOPT_HOST_STRICT as libc::c_int
                | HTTP_PARSEOPT_HOST_NORMALIZE as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if (*srv).srvconf.http_host_normalize as libc::c_int != 0 {
            HTTP_PARSEOPT_HOST_NORMALIZE as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if (*srv).srvconf.http_method_get_body as libc::c_int != 0 {
            HTTP_PARSEOPT_METHOD_GET_BODY as libc::c_int
        } else {
            0 as libc::c_int
        })) as libc::c_uint;
    (*p).defaults.http_parseopts |= (*srv).srvconf.http_url_normalize as libc::c_uint;
    (*p).defaults.mimetypes = &mut (*srv).srvconf.empty_array;
    (*p).defaults.h2proto = (*srv).srvconf.h2proto;
    if (*p).nconfig > 0 as libc::c_int
        && (*(*p).cvlist).v.u2[1 as libc::c_int as usize] != 0
    {
        let mut cpv_0: *const config_plugin_value_t = ((*p).cvlist)
            .offset((*(*p).cvlist).v.u2[0 as libc::c_int as usize] as isize);
        if -(1 as libc::c_int) != (*cpv_0).k_id {
            config_merge_config(&mut (*p).defaults, cpv_0);
        }
    }
    (*p).defaults.max_request_field_size = (*srv).srvconf.max_request_field_size;
    (*p)
        .defaults
        .log_request_header_on_error = (*srv).srvconf.log_request_header_on_error;
    if (*p).defaults.log_request_handling as libc::c_int != 0
        || (*p).defaults.log_request_header as libc::c_int != 0
    {
        (*p).defaults.log_request_header_on_error = 1 as libc::c_int as libc::c_uchar;
    }
    request_config_set_defaults(&mut (*p).defaults);
    return rc;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_finalize(
    mut srv: *mut server,
    mut default_server_tag: *const buffer,
) -> libc::c_int {
    let p: *mut config_data_base = (*srv).config_data_base as *mut config_data_base;
    (*p).defaults.high_precision_timestamps = (*srv).srvconf.high_precision_timestamps;
    if ((*p).defaults.server_tag).is_null() {
        (*p).defaults.server_tag = default_server_tag;
    } else if buffer_is_blank((*p).defaults.server_tag) != 0 {
        (*p).defaults.server_tag = 0 as *const buffer;
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*(*srv).config_context).used {
        let mut config: *mut array = (*(*((*(*srv).config_context).data)
            .offset(i as isize) as *mut data_config))
            .value;
        let mut j: uint32_t = 0 as libc::c_int as uint32_t;
        while !config.is_null() && j < (*config).used {
            let k: *const buffer = &mut (**((*config).data).offset(j as isize)).key;
            if !(strncmp(
                (*k).ptr,
                b"var.\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int)
            {
                if (array_get_element_klen(
                    (*srv).srvconf.config_touched,
                    (*k).ptr,
                    buffer_clen(k),
                ))
                    .is_null()
                {
                    log_error(
                        (*srv).errh,
                        b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                        1213 as libc::c_int as libc::c_uint,
                        b"WARNING: unknown config-key: %s (ignored)\0" as *const u8
                            as *const libc::c_char,
                        (*k).ptr,
                    );
                }
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    array_free((*srv).srvconf.config_touched);
    (*srv).srvconf.config_touched = 0 as *mut array;
    if (*srv).srvconf.config_unsupported as libc::c_int != 0
        || (*srv).srvconf.config_deprecated as libc::c_int != 0
    {
        if (*srv).srvconf.config_unsupported != 0 {
            log_error(
                (*srv).errh,
                b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                1223 as libc::c_int as libc::c_uint,
                b"Configuration contains unsupported keys. Going down.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if (*srv).srvconf.config_deprecated != 0 {
            log_error(
                (*srv).errh,
                b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                1226 as libc::c_int as libc::c_uint,
                b"Configuration contains deprecated keys. Going down.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    let mut i_0: uint32_t = 1 as libc::c_int as uint32_t;
    while i_0 < (*(*srv).config_context).used {
        let dc: *mut data_config = *((*(*srv).config_context).data).offset(i_0 as isize)
            as *mut data_config;
        if ((*dc).cond as libc::c_uint
            == CONFIG_COND_MATCH as libc::c_int as libc::c_uint
            || (*dc).cond as libc::c_uint
                == CONFIG_COND_NOMATCH as libc::c_int as libc::c_uint)
            && 0 as libc::c_int == (*dc).capture_idx
        {
            if (0 as *mut libc::c_void == (*srv).match_data) as libc::c_int
                as libc::c_long != 0
            {
                let mut ovec_max: uint32_t = 10 as libc::c_int as uint32_t;
                (*srv)
                    .match_data = pcre2_match_data_create_8(
                    ovec_max,
                    0 as *mut pcre2_general_context_8,
                ) as *mut libc::c_void;
                if ((*srv).match_data).is_null() {
                    ck_assert_failed(
                        b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                        1261 as libc::c_int as libc::c_uint,
                        b"srv->match_data\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            (*dc).match_data = (*srv).match_data as *mut pcre2_real_match_data_8;
        }
        i_0 = i_0.wrapping_add(1);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn config_print_indent(mut b: *mut buffer, mut depth: libc::c_int) {
    depth <<= 2 as libc::c_int;
    memset(
        buffer_extend(b, depth as size_t) as *mut libc::c_void,
        ' ' as i32,
        depth as libc::c_ulong,
    );
}
unsafe extern "C" fn config_print_array_max_klen(a: *const array) -> uint32_t {
    let mut maxlen: uint32_t = 0 as libc::c_int as uint32_t;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*a).used {
        let mut len: uint32_t = buffer_clen(&mut (**((*a).data).offset(i as isize)).key);
        if maxlen < len {
            maxlen = len;
        }
        i = i.wrapping_add(1);
    }
    return maxlen;
}
unsafe extern "C" fn config_print_array(
    a: *const array,
    b: *mut buffer,
    mut depth: libc::c_int,
) {
    if (*a).used <= 5 as libc::c_int as libc::c_uint
        && ((*a).used == 0
            || buffer_is_unset(
                &mut (**((*a).data).offset(0 as libc::c_int as isize)).key,
            ) != 0)
    {
        let mut oneline: libc::c_int = 1 as libc::c_int;
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while i < (*a).used {
            let mut du: *mut data_unset = *((*a).data).offset(i as isize);
            if (*du).type_0 as libc::c_uint != TYPE_STRING as libc::c_int as libc::c_uint
                && (*du).type_0 as libc::c_uint
                    != TYPE_INTEGER as libc::c_int as libc::c_uint
            {
                oneline = 0 as libc::c_int;
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        if oneline != 0 {
            buffer_append_string(b, b"(\0" as *const u8 as *const libc::c_char);
            let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
            while i_0 < (*a).used {
                if i_0 != 0 as libc::c_int as libc::c_uint {
                    buffer_append_string(b, b", \0" as *const u8 as *const libc::c_char);
                }
                config_print_by_type(
                    *((*a).data).offset(i_0 as isize),
                    b,
                    depth + 1 as libc::c_int,
                );
                i_0 = i_0.wrapping_add(1);
            }
            buffer_append_string(b, b")\0" as *const u8 as *const libc::c_char);
            return;
        }
    }
    let maxlen: uint32_t = config_print_array_max_klen(a);
    buffer_append_string(b, b"(\n\0" as *const u8 as *const libc::c_char);
    let mut i_1: uint32_t = 0 as libc::c_int as uint32_t;
    while i_1 < (*a).used {
        config_print_indent(b, depth + 1 as libc::c_int);
        let mut du_0: *mut data_unset = *((*a).data).offset(i_1 as isize);
        if buffer_is_unset(&mut (*du_0).key) == 0 {
            buffer_append_str3(
                b,
                b"\"\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                (*du_0).key.ptr,
                buffer_clen(&mut (*du_0).key) as size_t,
                b"\"\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            let mut indent: libc::c_int = maxlen
                .wrapping_sub(buffer_clen(&mut (*du_0).key)) as libc::c_int;
            if indent > 0 as libc::c_int {
                memset(
                    buffer_extend(b, indent as size_t) as *mut libc::c_void,
                    ' ' as i32,
                    indent as libc::c_ulong,
                );
            }
            buffer_append_string(b, b" => \0" as *const u8 as *const libc::c_char);
        }
        config_print_by_type(du_0, b, depth + 1 as libc::c_int);
        buffer_append_string(b, b",\n\0" as *const u8 as *const libc::c_char);
        i_1 = i_1.wrapping_add(1);
    }
    config_print_indent(b, depth);
    buffer_append_string(b, b")\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn config_print_config(
    mut d: *const data_unset,
    b: *mut buffer,
    mut depth: libc::c_int,
) {
    let mut dc: *mut data_config = d as *mut data_config;
    let mut a: *mut array = (*dc).value;
    if 0 as libc::c_int == (*dc).context_ndx {
        buffer_append_string(b, b"config {\n\0" as *const u8 as *const libc::c_char);
    } else {
        if (*dc).cond as libc::c_uint != CONFIG_COND_ELSE as libc::c_int as libc::c_uint
        {
            buffer_append_string(b, (*dc).comp_key);
            buffer_append_string(b, b" \0" as *const u8 as *const libc::c_char);
        }
        buffer_append_string(b, b"{\n\0" as *const u8 as *const libc::c_char);
        config_print_indent(b, depth + 1 as libc::c_int);
        buffer_append_string(b, b"# block \0" as *const u8 as *const libc::c_char);
        buffer_append_int(b, (*dc).context_ndx as intmax_t);
        buffer_append_string(b, b"\n\0" as *const u8 as *const libc::c_char);
    }
    depth += 1;
    let maxlen: uint32_t = config_print_array_max_klen(a);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*a).used {
        config_print_indent(b, depth);
        let mut du: *mut data_unset = *((*a).data).offset(i as isize);
        buffer_append_buffer(b, &mut (*du).key);
        let mut indent: libc::c_int = maxlen.wrapping_sub(buffer_clen(&mut (*du).key))
            as libc::c_int;
        if indent > 0 as libc::c_int {
            memset(
                buffer_extend(b, indent as size_t) as *mut libc::c_void,
                ' ' as i32,
                indent as libc::c_ulong,
            );
        }
        buffer_append_string(b, b" = \0" as *const u8 as *const libc::c_char);
        config_print_by_type(du, b, depth);
        buffer_append_string(b, b"\n\0" as *const u8 as *const libc::c_char);
        i = i.wrapping_add(1);
    }
    buffer_append_string(b, b"\n\0" as *const u8 as *const libc::c_char);
    let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
    while (i_0 as libc::c_ulong) < (*dc).children.used {
        let mut dcc: *mut data_config = *((*dc).children.data).offset(i_0 as isize);
        if ((*dcc).prev).is_null() {
            buffer_append_string(b, b"\n\0" as *const u8 as *const libc::c_char);
            config_print_indent(b, depth);
            config_print_by_type(dcc as *mut data_unset, b, depth);
            buffer_append_string(b, b"\n\0" as *const u8 as *const libc::c_char);
        }
        i_0 = i_0.wrapping_add(1);
    }
    depth -= 1;
    config_print_indent(b, depth);
    buffer_append_string(b, b"}\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != (*dc).context_ndx {
        buffer_append_string(b, b" # end of \0" as *const u8 as *const libc::c_char);
        buffer_append_string(
            b,
            if (*dc).cond as libc::c_uint
                != CONFIG_COND_ELSE as libc::c_int as libc::c_uint
            {
                (*dc).comp_key
            } else {
                b"else\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if !((*dc).next).is_null() {
        buffer_append_string(b, b"\n\0" as *const u8 as *const libc::c_char);
        config_print_indent(b, depth);
        buffer_append_string(b, b"else \0" as *const u8 as *const libc::c_char);
        config_print_by_type((*dc).next as *mut data_unset, b, depth);
    }
}
unsafe extern "C" fn config_print_string(mut du: *const data_unset, b: *mut buffer) {
    let vb: *const buffer = &mut (*(du as *mut data_string)).value;
    let mut dst: *mut libc::c_char = buffer_string_prepare_append(
        b,
        (buffer_clen(vb)).wrapping_mul(2 as libc::c_int as libc::c_uint) as size_t,
    );
    let mut n: uint32_t = 0 as libc::c_int as uint32_t;
    let fresh2 = n;
    n = n.wrapping_add(1);
    *dst.offset(fresh2 as isize) = '"' as i32 as libc::c_char;
    if !((*vb).ptr).is_null() {
        let mut p: *const libc::c_char = (*vb).ptr;
        while *p != 0 {
            if *p as libc::c_int == '"' as i32 {
                let fresh3 = n;
                n = n.wrapping_add(1);
                *dst.offset(fresh3 as isize) = '\\' as i32 as libc::c_char;
            }
            let fresh4 = n;
            n = n.wrapping_add(1);
            *dst.offset(fresh4 as isize) = *p;
            p = p.offset(1);
        }
    }
    let fresh5 = n;
    n = n.wrapping_add(1);
    *dst.offset(fresh5 as isize) = '"' as i32 as libc::c_char;
    buffer_commit(b, n as size_t);
}
#[cold]
unsafe extern "C" fn config_print_by_type(
    du: *const data_unset,
    b: *mut buffer,
    mut depth: libc::c_int,
) {
    match (*du).type_0 as libc::c_uint {
        0 => {
            config_print_string(du, b);
        }
        2 => {
            buffer_append_int(b, (*(du as *mut data_integer)).value as intmax_t);
        }
        1 => {
            config_print_array(&mut (*(du as *mut data_array)).value, b, depth);
        }
        3 => {
            config_print_config(du, b, depth);
        }
        _ => {}
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_print(mut srv: *mut server) {
    buffer_clear((*srv).tmp_buf);
    let mut dc: *mut data_unset = *((*(*srv).config_context).data)
        .offset(0 as libc::c_int as isize);
    config_print_by_type(dc, (*srv).tmp_buf, 0 as libc::c_int);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_free(mut srv: *mut server) {
    config_free_config((*srv).config_data_base);
    array_free((*srv).config_context);
    array_free((*srv).srvconf.config_touched);
    array_free((*srv).srvconf.modules);
    array_free((*srv).srvconf.upload_tempdirs);
    if ((*srv).match_data).is_null() {
        pcre2_match_data_free_8((*srv).match_data as *mut pcre2_match_data_8);
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_init(mut srv: *mut server) {
    (*srv).config_context = array_init(16 as libc::c_int as uint32_t);
    (*srv).srvconf.config_touched = array_init(128 as libc::c_int as uint32_t);
    (*srv).srvconf.port = 0 as libc::c_int as libc::c_ushort;
    (*srv).srvconf.dont_daemonize = 0 as libc::c_int as libc::c_uchar;
    (*srv).srvconf.preflight_check = 0 as libc::c_int as libc::c_uchar;
    (*srv).srvconf.compat_module_load = 1 as libc::c_int as libc::c_uchar;
    (*srv).srvconf.systemd_socket_activation = 0 as libc::c_int as libc::c_uchar;
    (*srv).srvconf.high_precision_timestamps = 0 as libc::c_int as libc::c_uchar;
    (*srv).srvconf.max_request_field_size = 8192 as libc::c_int as uint32_t;
    (*srv).srvconf.http_header_strict = 1 as libc::c_int as libc::c_uchar;
    (*srv).srvconf.http_host_strict = 1 as libc::c_int as libc::c_uchar;
    (*srv).srvconf.http_host_normalize = 0 as libc::c_int as libc::c_uchar;
    (*srv)
        .srvconf
        .http_url_normalize = (HTTP_PARSEOPT_URL_NORMALIZE as libc::c_int
        | HTTP_PARSEOPT_URL_NORMALIZE_UNRESERVED as libc::c_int
        | HTTP_PARSEOPT_URL_NORMALIZE_CTRLS_REJECT as libc::c_int
        | HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_DECODE as libc::c_int
        | HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REMOVE as libc::c_int)
        as libc::c_ushort;
    (*srv).srvconf.modules = array_init(16 as libc::c_int as uint32_t);
    (*srv)
        .srvconf
        .modules_dir = b"sconsbuild/install/lib\0" as *const u8 as *const libc::c_char;
    (*srv).srvconf.upload_tempdirs = array_init(2 as libc::c_int as uint32_t);
}
unsafe extern "C" fn config_log_error_open_syslog(
    mut srv: *mut server,
    mut errh: *mut log_error_st,
    mut syslog_facility: *const buffer,
) {
    (*errh).mode = FDLOG_SYSLOG;
    (*errh).fd = -(1 as libc::c_int);
    let mut facility: libc::c_int = -(1 as libc::c_int);
    if !syslog_facility.is_null() {
        static mut facility_names: [facility_name_st; 21] = [
            {
                let mut init = facility_name_st {
                    name: b"auth\0" as *const u8 as *const libc::c_char,
                    val: (4 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"authpriv\0" as *const u8 as *const libc::c_char,
                    val: (10 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"cron\0" as *const u8 as *const libc::c_char,
                    val: (9 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"daemon\0" as *const u8 as *const libc::c_char,
                    val: (3 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"ftp\0" as *const u8 as *const libc::c_char,
                    val: (11 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"kern\0" as *const u8 as *const libc::c_char,
                    val: (0 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"lpr\0" as *const u8 as *const libc::c_char,
                    val: (6 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"mail\0" as *const u8 as *const libc::c_char,
                    val: (2 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"news\0" as *const u8 as *const libc::c_char,
                    val: (7 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"security\0" as *const u8 as *const libc::c_char,
                    val: (4 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"syslog\0" as *const u8 as *const libc::c_char,
                    val: (5 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"user\0" as *const u8 as *const libc::c_char,
                    val: (1 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"uucp\0" as *const u8 as *const libc::c_char,
                    val: (8 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"local0\0" as *const u8 as *const libc::c_char,
                    val: (16 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"local1\0" as *const u8 as *const libc::c_char,
                    val: (17 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"local2\0" as *const u8 as *const libc::c_char,
                    val: (18 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"local3\0" as *const u8 as *const libc::c_char,
                    val: (19 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"local4\0" as *const u8 as *const libc::c_char,
                    val: (20 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"local5\0" as *const u8 as *const libc::c_char,
                    val: (21 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"local6\0" as *const u8 as *const libc::c_char,
                    val: (22 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"local7\0" as *const u8 as *const libc::c_char,
                    val: (23 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
        ];
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<[facility_name_st; 21]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<facility_name_st>() as libc::c_ulong)
        {
            let mut f: *const facility_name_st = facility_names
                .as_ptr()
                .offset(i as isize);
            if 0 as libc::c_int == strcmp((*syslog_facility).ptr, (*f).name) {
                facility = (*f).val;
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        if -(1 as libc::c_int) == facility {
            log_error(
                (*srv).errh,
                b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                1559 as libc::c_int as libc::c_uint,
                b"unrecognized server.syslog-facility: \"%s\"; defaulting to \"daemon\" facility\0"
                    as *const u8 as *const libc::c_char,
                (*syslog_facility).ptr,
            );
        }
    }
    openlog(
        b"lighttpd\0" as *const u8 as *const libc::c_char,
        0x2 as libc::c_int | 0x1 as libc::c_int,
        if -(1 as libc::c_int) == facility {
            (3 as libc::c_int) << 3 as libc::c_int
        } else {
            facility
        },
    );
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_log_error_open(mut srv: *mut server) -> libc::c_int {
    let p: *mut config_data_base = (*srv).config_data_base as *mut config_data_base;
    let mut serrh: *mut log_error_st = 0 as *mut log_error_st;
    let mut i: libc::c_int = ((*((*p).cvlist).offset(0 as libc::c_int as isize))
        .v
        .u2[1 as libc::c_int as usize] == 0) as libc::c_int;
    while i < (*p).nconfig {
        let mut cpv: *mut config_plugin_value_t = ((*p).cvlist)
            .offset(
                (*((*p).cvlist).offset(i as isize)).v.u2[0 as libc::c_int as usize]
                    as isize,
            );
        let mut current_block_15: u64;
        while -(1 as libc::c_int) != (*cpv).k_id {
            let mut fn_0: *const libc::c_char = 0 as *const libc::c_char;
            let mut errh: *mut log_error_st = 0 as *mut log_error_st;
            match (*cpv).k_id {
                32 => {
                    if 0 as libc::c_int == i {
                        if (*srv).srvconf.errorlog_use_syslog != 0 {
                            current_block_15 = 715039052867723359;
                        } else {
                            errh = (*srv).errh;
                            current_block_15 = 3498060765610854713;
                        }
                    } else {
                        current_block_15 = 3498060765610854713;
                    }
                }
                33 => {
                    current_block_15 = 3498060765610854713;
                }
                _ => {
                    current_block_15 = 9606288038608642794;
                }
            }
            match current_block_15 {
                3498060765610854713 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        fn_0 = (*(*cpv).v.b).ptr;
                    }
                    current_block_15 = 9606288038608642794;
                }
                _ => {}
            }
            match current_block_15 {
                9606288038608642794 => {
                    if !fn_0.is_null() {
                        let fdlog: *mut fdlog_st = fdlog_open(fn_0);
                        if fdlog.is_null() {
                            log_perror(
                                (*srv).errh,
                                b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                                1608 as libc::c_int as libc::c_uint,
                                b"opening errorlog '%s' failed\0" as *const u8
                                    as *const libc::c_char,
                                fn_0,
                            );
                            return -(1 as libc::c_int);
                        }
                        if !errh.is_null() {
                            (*srv).errh = fdlog;
                            (*p).defaults.errh = (*srv).errh;
                            log_set_global_errh((*srv).errh, 0 as libc::c_int);
                        }
                        errh = fdlog;
                        (*cpv).v.v = errh as *mut libc::c_void;
                        (*cpv).vtype = T_CONFIG_LOCAL;
                        if 0 as libc::c_int == i && errh != (*srv).errh {
                            serrh = errh;
                        }
                    }
                }
                _ => {}
            }
            cpv = cpv.offset(1);
        }
        i += 1;
    }
    if config_feature_bool(
        srv,
        b"server.errorlog-high-precision\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) != 0
    {
        log_set_global_errh((*srv).errh, 1 as libc::c_int);
    }
    if (*srv).srvconf.errorlog_use_syslog != 0 {
        config_log_error_open_syslog(srv, (*srv).errh, (*srv).srvconf.syslog_facility);
    } else if (*(*srv).errh).mode as libc::c_uint
            == FDLOG_FD as libc::c_int as libc::c_uint
            && (*srv).srvconf.dont_daemonize == 0
        {
        (*(*srv).errh).fd = -(1 as libc::c_int);
    }
    let mut errfd: libc::c_int = 0;
    if !serrh.is_null() {
        if (*(*srv).errh).mode as libc::c_uint == FDLOG_FD as libc::c_int as libc::c_uint
        {
            (*(*srv).errh).fd = dup(2 as libc::c_int);
            fdevent_setfd_cloexec((*(*srv).errh).fd);
        }
        errfd = (*serrh).fd;
        if *(*serrh).fn_0 as libc::c_int == '|' as i32 {
            fdlog_pipe_serrh(errfd);
        }
    } else if (*srv).srvconf.dont_daemonize == 0 {
        errfd = fdevent_open_devnull();
        if -(1 as libc::c_int) == errfd {
            log_perror(
                (*srv).errh,
                b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                1659 as libc::c_int as libc::c_uint,
                b"opening /dev/null failed\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else {
        errfd = -(1 as libc::c_int);
    }
    if 0 as libc::c_int
        != fdevent_set_stdin_stdout_stderr(
            -(1 as libc::c_int),
            -(1 as libc::c_int),
            errfd,
        )
    {
        log_perror(
            (*srv).errh,
            b"src/configfile.c\0" as *const u8 as *const libc::c_char,
            1669 as libc::c_int as libc::c_uint,
            b"setting stderr failed\0" as *const u8 as *const libc::c_char,
        );
        if -(1 as libc::c_int) != errfd && serrh.is_null() {
            close(errfd);
        }
        return -(1 as libc::c_int);
    }
    if -(1 as libc::c_int) != errfd && serrh.is_null() {
        close(errfd);
    }
    if !serrh.is_null() {
        close(errfd);
        (*serrh).fd = 2 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_log_error_close(mut srv: *mut server) {
    let p: *mut config_data_base = (*srv).config_data_base as *mut config_data_base;
    if p.is_null() {
        return;
    }
    (*p).defaults.serrh = 0 as *mut fdlog_st;
    fdlog_closeall((*srv).errh);
    if (*(*srv).errh).mode as libc::c_uint == FDLOG_SYSLOG as libc::c_int as libc::c_uint
    {
        (*(*srv).errh).mode = FDLOG_FD;
        (*(*srv).errh).fd = 2 as libc::c_int;
        closelog();
    }
}
unsafe extern "C" fn config_skip_newline(mut t: *mut tokenizer_t) -> libc::c_int {
    let mut skipped: libc::c_int = 1 as libc::c_int;
    if !(*((*t).input).offset((*t).offset as isize) as libc::c_int == '\r' as i32
        || *((*t).input).offset((*t).offset as isize) as libc::c_int == '\n' as i32)
    {
        ck_assert_failed(
            b"src/configfile.c\0" as *const u8 as *const libc::c_char,
            1724 as libc::c_int as libc::c_uint,
            b"t->input[t->offset] == '\\r' || t->input[t->offset] == '\\n'\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if *((*t).input).offset((*t).offset as isize) as libc::c_int == '\r' as i32
        && *((*t).input)
            .offset(
                ((*t).offset).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) as libc::c_int == '\n' as i32
    {
        skipped += 1;
        (*t).offset = ((*t).offset).wrapping_add(1);
    }
    (*t).offset = ((*t).offset).wrapping_add(1);
    return skipped;
}
unsafe extern "C" fn config_skip_comment(mut t: *mut tokenizer_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if !(*((*t).input).offset((*t).offset as isize) as libc::c_int == '#' as i32) {
        ck_assert_failed(
            b"src/configfile.c\0" as *const u8 as *const libc::c_char,
            1735 as libc::c_int as libc::c_uint,
            b"t->input[t->offset] == '#'\0" as *const u8 as *const libc::c_char,
        );
    }
    i = 1 as libc::c_int;
    while *((*t).input).offset(((*t).offset).wrapping_add(i as libc::c_ulong) as isize)
        as libc::c_int != 0
        && (*((*t).input).offset(((*t).offset).wrapping_add(i as libc::c_ulong) as isize)
            as libc::c_int != '\n' as i32
            && *((*t).input)
                .offset(((*t).offset).wrapping_add(i as libc::c_ulong) as isize)
                as libc::c_int != '\r' as i32)
    {
        i += 1;
    }
    (*t)
        .offset = ((*t).offset as libc::c_ulong).wrapping_add(i as libc::c_ulong)
        as size_t as size_t;
    return i;
}
#[cold]
unsafe extern "C" fn config_tokenizer_err(
    mut srv: *mut server,
    mut file: *const libc::c_char,
    mut line: libc::c_uint,
    mut t: *mut tokenizer_t,
    mut msg: *const libc::c_char,
) -> libc::c_int {
    log_error(
        (*srv).errh,
        file,
        line,
        b"source: %s line: %d pos: %d %s\0" as *const u8 as *const libc::c_char,
        (*t).source,
        (*t).line,
        (*t).line_pos,
        msg,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn config_tokenizer(
    mut srv: *mut server,
    mut t: *mut tokenizer_t,
    mut token_id: *mut libc::c_int,
    mut token: *mut buffer,
) -> libc::c_int {
    let mut tid: libc::c_int = 0 as libc::c_int;
    let mut i: size_t = 0;
    if (*t).simulate_eol != 0 {
        (*t).simulate_eol = 0 as libc::c_int;
        (*t).in_key = 1 as libc::c_int;
        tid = 1 as libc::c_int;
        buffer_copy_string_len(
            token,
            b"(EOL)\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    while tid == 0 as libc::c_int && (*t).offset < (*t).size
        && *((*t).input).offset((*t).offset as isize) as libc::c_int != 0
    {
        let mut c: libc::c_char = *((*t).input).offset((*t).offset as isize);
        let mut start: *const libc::c_char = 0 as *const libc::c_char;
        match c as libc::c_int {
            61 => {
                if (*t).in_brace != 0 {
                    if *((*t).input)
                        .offset(
                            ((*t).offset).wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) as libc::c_int == '>' as i32
                    {
                        (*t)
                            .offset = ((*t).offset as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        buffer_copy_string_len(
                            token,
                            b"=>\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        );
                        tid = 12 as libc::c_int;
                    } else {
                        return config_tokenizer_err(
                            srv,
                            b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                            1775 as libc::c_int as libc::c_uint,
                            t,
                            b"use => for assignments in arrays\0" as *const u8
                                as *const libc::c_char,
                        )
                    }
                } else if (*t).in_cond != 0 {
                    if *((*t).input)
                        .offset(
                            ((*t).offset).wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) as libc::c_int == '=' as i32
                    {
                        (*t)
                            .offset = ((*t).offset as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        buffer_copy_string_len(
                            token,
                            b"==\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        );
                        tid = 21 as libc::c_int;
                    } else if *((*t).input)
                            .offset(
                                ((*t).offset)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int == '~' as i32
                        {
                        (*t)
                            .offset = ((*t).offset as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        buffer_copy_string_len(
                            token,
                            b"=~\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        );
                        tid = 22 as libc::c_int;
                    } else {
                        return config_tokenizer_err(
                            srv,
                            b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                            1792 as libc::c_int as libc::c_uint,
                            t,
                            b"only =~ and == are allowed in the condition\0" as *const u8
                                as *const libc::c_char,
                        )
                    }
                    (*t).in_key = 1 as libc::c_int;
                    (*t).in_cond = 0 as libc::c_int;
                } else if (*t).in_key != 0 {
                    tid = 2 as libc::c_int;
                    buffer_copy_string_len(
                        token,
                        ((*t).input).offset((*t).offset as isize),
                        1 as libc::c_int as size_t,
                    );
                    (*t).offset = ((*t).offset).wrapping_add(1);
                    (*t).line_pos += 1;
                } else {
                    return config_tokenizer_err(
                        srv,
                        b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                        1805 as libc::c_int as libc::c_uint,
                        t,
                        b"unexpected equal-sign: =\0" as *const u8 as *const libc::c_char,
                    )
                }
            }
            33 => {
                if (*t).in_cond != 0 {
                    if *((*t).input)
                        .offset(
                            ((*t).offset).wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) as libc::c_int == '=' as i32
                    {
                        (*t)
                            .offset = ((*t).offset as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        buffer_copy_string_len(
                            token,
                            b"!=\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        );
                        tid = 23 as libc::c_int;
                    } else if *((*t).input)
                            .offset(
                                ((*t).offset)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int == '~' as i32
                        {
                        (*t)
                            .offset = ((*t).offset as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        buffer_copy_string_len(
                            token,
                            b"!~\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        );
                        tid = 24 as libc::c_int;
                    } else {
                        return config_tokenizer_err(
                            srv,
                            b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                            1825 as libc::c_int as libc::c_uint,
                            t,
                            b"only !~ and != are allowed in the condition\0" as *const u8
                                as *const libc::c_char,
                        )
                    }
                    (*t).in_key = 1 as libc::c_int;
                    (*t).in_cond = 0 as libc::c_int;
                } else {
                    return config_tokenizer_err(
                        srv,
                        b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                        1831 as libc::c_int as libc::c_uint,
                        t,
                        b"unexpected exclamation-marks: !\0" as *const u8
                            as *const libc::c_char,
                    )
                }
            }
            9 | 32 => {
                (*t).offset = ((*t).offset).wrapping_add(1);
                (*t).line_pos += 1;
            }
            10 | 13 => {
                if (*t).in_brace == 0 as libc::c_int {
                    let mut done: libc::c_int = 0 as libc::c_int;
                    while done == 0 && (*t).offset < (*t).size {
                        match *((*t).input).offset((*t).offset as isize) as libc::c_int {
                            13 | 10 => {
                                config_skip_newline(t);
                                (*t).line_pos = 1 as libc::c_int;
                                (*t).line += 1;
                            }
                            35 => {
                                (*t).line_pos += config_skip_comment(t);
                            }
                            9 | 32 => {
                                (*t).offset = ((*t).offset).wrapping_add(1);
                                (*t).line_pos += 1;
                            }
                            _ => {
                                done = 1 as libc::c_int;
                            }
                        }
                    }
                    (*t).in_key = 1 as libc::c_int;
                    tid = 1 as libc::c_int;
                    buffer_copy_string_len(
                        token,
                        b"(EOL)\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    );
                } else {
                    config_skip_newline(t);
                    (*t).line_pos = 1 as libc::c_int;
                    (*t).line += 1;
                }
            }
            44 => {
                if (*t).in_brace > 0 as libc::c_int {
                    tid = 11 as libc::c_int;
                    buffer_copy_string_len(
                        token,
                        b"(COMMA)\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    );
                }
                (*t).offset = ((*t).offset).wrapping_add(1);
                (*t).line_pos += 1;
            }
            34 => {
                start = ((*t).input)
                    .offset((*t).offset as isize)
                    .offset(1 as libc::c_int as isize);
                buffer_copy_string_len(
                    token,
                    b"\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
                i = 1 as libc::c_int as size_t;
                while *((*t).input).offset(((*t).offset).wrapping_add(i) as isize) != 0 {
                    if *((*t).input).offset(((*t).offset).wrapping_add(i) as isize)
                        as libc::c_int == '\\' as i32
                        && *((*t).input)
                            .offset(
                                ((*t).offset)
                                    .wrapping_add(i)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int == '"' as i32
                    {
                        buffer_append_string_len(
                            token,
                            start,
                            ((*t).input)
                                .offset((*t).offset as isize)
                                .offset(i as isize)
                                .offset_from(start) as libc::c_long as size_t,
                        );
                        start = ((*t).input)
                            .offset((*t).offset as isize)
                            .offset(i as isize)
                            .offset(1 as libc::c_int as isize);
                        i = i.wrapping_add(1);
                    } else if *((*t).input)
                            .offset(((*t).offset).wrapping_add(i) as isize)
                            as libc::c_int == '"' as i32
                        {
                        tid = 7 as libc::c_int;
                        buffer_append_string_len(
                            token,
                            start,
                            ((*t).input)
                                .offset((*t).offset as isize)
                                .offset(i as isize)
                                .offset_from(start) as libc::c_long as size_t,
                        );
                        break;
                    }
                    i = i.wrapping_add(1);
                }
                if *((*t).input).offset(((*t).offset).wrapping_add(i) as isize)
                    as libc::c_int == '\u{0}' as i32
                {
                    return config_tokenizer_err(
                        srv,
                        b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                        1916 as libc::c_int as libc::c_uint,
                        t,
                        b"missing closing quote\0" as *const u8 as *const libc::c_char,
                    );
                }
                (*t)
                    .offset = ((*t).offset as libc::c_ulong)
                    .wrapping_add(i.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as size_t as size_t;
                (*t)
                    .line_pos = ((*t).line_pos as libc::c_ulong)
                    .wrapping_add(i.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as libc::c_int as libc::c_int;
            }
            40 => {
                (*t).offset = ((*t).offset).wrapping_add(1);
                (*t).in_brace += 1;
                tid = 9 as libc::c_int;
                buffer_copy_string_len(
                    token,
                    b"(\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
            }
            41 => {
                (*t).offset = ((*t).offset).wrapping_add(1);
                (*t).in_brace -= 1;
                tid = 10 as libc::c_int;
                buffer_copy_string_len(
                    token,
                    b")\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
            }
            36 => {
                (*t).offset = ((*t).offset).wrapping_add(1);
                tid = 17 as libc::c_int;
                (*t).in_cond = 1 as libc::c_int;
                (*t).in_key = 0 as libc::c_int;
                buffer_copy_string_len(
                    token,
                    b"$\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
            }
            43 => {
                if *((*t).input)
                    .offset(
                        ((*t).offset).wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as libc::c_int == '=' as i32
                {
                    (*t)
                        .offset = ((*t).offset as libc::c_ulong)
                        .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                    buffer_copy_string_len(
                        token,
                        b"+=\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    );
                    tid = 4 as libc::c_int;
                } else {
                    (*t).offset = ((*t).offset).wrapping_add(1);
                    tid = 6 as libc::c_int;
                    buffer_copy_string_len(
                        token,
                        b"+\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    );
                }
            }
            58 => {
                if *((*t).input)
                    .offset(
                        ((*t).offset).wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as libc::c_int == '=' as i32
                {
                    (*t)
                        .offset = ((*t).offset as libc::c_ulong)
                        .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                    tid = 3 as libc::c_int;
                    buffer_copy_string_len(
                        token,
                        b":=\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    );
                } else {
                    return config_tokenizer_err(
                        srv,
                        b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                        1969 as libc::c_int as libc::c_uint,
                        t,
                        b"unexpected character ':'\0" as *const u8 as *const libc::c_char,
                    )
                }
            }
            123 => {
                (*t).offset = ((*t).offset).wrapping_add(1);
                tid = 14 as libc::c_int;
                buffer_copy_string_len(
                    token,
                    b"{\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
            }
            125 => {
                (*t).offset = ((*t).offset).wrapping_add(1);
                tid = 15 as libc::c_int;
                buffer_copy_string_len(
                    token,
                    b"}\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
                while (*t).offset < (*t).size {
                    c = *((*t).input).offset((*t).offset as isize);
                    if c as libc::c_int == '\r' as i32 || c as libc::c_int == '\n' as i32
                    {
                        break;
                    }
                    if c as libc::c_int == '#' as i32 {
                        (*t).line_pos += config_skip_comment(t);
                        break;
                    } else if c as libc::c_int != ' ' as i32
                            && c as libc::c_int != '\t' as i32
                        {
                        (*t).simulate_eol = 1 as libc::c_int;
                        break;
                    } else {
                        (*t).offset = ((*t).offset).wrapping_add(1);
                        (*t).line_pos += 1;
                    }
                }
            }
            91 => {
                (*t).offset = ((*t).offset).wrapping_add(1);
                tid = 19 as libc::c_int;
                buffer_copy_string_len(
                    token,
                    b"[\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
            }
            93 => {
                (*t).offset = ((*t).offset).wrapping_add(1);
                tid = 20 as libc::c_int;
                buffer_copy_string_len(
                    token,
                    b"]\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
            }
            35 => {
                (*t).line_pos += config_skip_comment(t);
            }
            _ => {
                if (*t).in_cond != 0 {
                    i = 0 as libc::c_int as size_t;
                    while *((*t).input).offset(((*t).offset).wrapping_add(i) as isize)
                        as libc::c_int != 0
                        && (*(*__ctype_b_loc())
                            .offset(
                                *((*t).input).offset(((*t).offset).wrapping_add(i) as isize)
                                    as libc::c_uchar as libc::c_int as isize,
                            ) as libc::c_int
                            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                            || *((*t).input)
                                .offset(((*t).offset).wrapping_add(i) as isize)
                                as libc::c_int == '_' as i32)
                    {
                        i = i.wrapping_add(1);
                    }
                    if i != 0
                        && *((*t).input).offset(((*t).offset).wrapping_add(i) as isize)
                            as libc::c_int != 0
                    {
                        tid = 18 as libc::c_int;
                        buffer_copy_string_len(
                            token,
                            ((*t).input).offset((*t).offset as isize),
                            i,
                        );
                        (*t)
                            .offset = ((*t).offset as libc::c_ulong).wrapping_add(i)
                            as size_t as size_t;
                        (*t)
                            .line_pos = ((*t).line_pos as libc::c_ulong).wrapping_add(i)
                            as libc::c_int as libc::c_int;
                    } else {
                        return config_tokenizer_err(
                            srv,
                            b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                            2041 as libc::c_int as libc::c_uint,
                            t,
                            b"invalid character in condition\0" as *const u8
                                as *const libc::c_char,
                        )
                    }
                } else if *(*__ctype_b_loc())
                        .offset(c as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                    i = 0 as libc::c_int as size_t;
                    while *((*t).input).offset(((*t).offset).wrapping_add(i) as isize)
                        as libc::c_int != 0
                        && *(*__ctype_b_loc())
                            .offset(
                                *((*t).input).offset(((*t).offset).wrapping_add(i) as isize)
                                    as libc::c_uchar as libc::c_int as isize,
                            ) as libc::c_int
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                    {
                        i = i.wrapping_add(1);
                    }
                    if i != 0 {
                        tid = 8 as libc::c_int;
                        buffer_copy_string_len(
                            token,
                            ((*t).input).offset((*t).offset as isize),
                            i,
                        );
                        (*t)
                            .offset = ((*t).offset as libc::c_ulong).wrapping_add(i)
                            as size_t as size_t;
                        (*t)
                            .line_pos = ((*t).line_pos as libc::c_ulong).wrapping_add(i)
                            as libc::c_int as libc::c_int;
                    }
                } else {
                    i = 0 as libc::c_int as size_t;
                    while *((*t).input).offset(((*t).offset).wrapping_add(i) as isize)
                        as libc::c_int != 0
                        && (*(*__ctype_b_loc())
                            .offset(
                                *((*t).input).offset(((*t).offset).wrapping_add(i) as isize)
                                    as libc::c_uchar as libc::c_int as isize,
                            ) as libc::c_int
                            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                            || *((*t).input)
                                .offset(((*t).offset).wrapping_add(i) as isize)
                                as libc::c_int == '.' as i32
                            || *((*t).input)
                                .offset(((*t).offset).wrapping_add(i) as isize)
                                as libc::c_int == '_' as i32
                            || *((*t).input)
                                .offset(((*t).offset).wrapping_add(i) as isize)
                                as libc::c_int == '-' as i32)
                    {
                        i = i.wrapping_add(1);
                    }
                    if i != 0
                        && *((*t).input).offset(((*t).offset).wrapping_add(i) as isize)
                            as libc::c_int != 0
                    {
                        buffer_copy_string_len(
                            token,
                            ((*t).input).offset((*t).offset as isize),
                            i,
                        );
                        if strcmp(
                            (*token).ptr,
                            b"include\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            tid = 25 as libc::c_int;
                        } else if strcmp(
                                (*token).ptr,
                                b"include_shell\0" as *const u8 as *const libc::c_char,
                            ) == 0 as libc::c_int
                            {
                            tid = 26 as libc::c_int;
                        } else if strcmp(
                                (*token).ptr,
                                b"global\0" as *const u8 as *const libc::c_char,
                            ) == 0 as libc::c_int
                            {
                            tid = 13 as libc::c_int;
                        } else if strcmp(
                                (*token).ptr,
                                b"else\0" as *const u8 as *const libc::c_char,
                            ) == 0 as libc::c_int
                            {
                            tid = 16 as libc::c_int;
                        } else {
                            tid = 5 as libc::c_int;
                        }
                        (*t)
                            .offset = ((*t).offset as libc::c_ulong).wrapping_add(i)
                            as size_t as size_t;
                        (*t)
                            .line_pos = ((*t).line_pos as libc::c_ulong).wrapping_add(i)
                            as libc::c_int as libc::c_int;
                    } else if 0 as libc::c_int as libc::c_ulong == i
                            && *((*t).input as *mut uint8_t)
                                .offset(
                                    ((*t).offset)
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
                                ) as libc::c_int == 0xc2 as libc::c_int
                            && *((*t).input as *mut uint8_t)
                                .offset(
                                    ((*t).offset)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                ) as libc::c_int == 0xa0 as libc::c_int
                        {
                        (*t)
                            .offset = ((*t).offset as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        (*t).line_pos += 2 as libc::c_int;
                    } else {
                        return config_tokenizer_err(
                            srv,
                            b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                            2091 as libc::c_int as libc::c_uint,
                            t,
                            b"invalid character in variable name\0" as *const u8
                                as *const libc::c_char,
                        )
                    }
                }
            }
        }
    }
    if tid != 0 {
        *token_id = tid;
        return 1 as libc::c_int;
    } else {
        if (*t).offset < (*t).size {
            log_error(
                (*srv).errh,
                b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                2103 as libc::c_int as libc::c_uint,
                b"%d, %s\0" as *const u8 as *const libc::c_char,
                tid,
                (*token).ptr,
            );
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn config_parse(
    mut srv: *mut server,
    mut context: *mut config_t,
    mut source: *const libc::c_char,
    mut input: *const libc::c_char,
    mut isize: size_t,
) -> libc::c_int {
    let mut pParser: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut token: *mut buffer = 0 as *mut buffer;
    let mut lasttoken: *mut buffer = 0 as *mut buffer;
    let mut token_id: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0;
    let mut t: tokenizer_t = tokenizer_t {
        source: 0 as *const libc::c_char,
        input: 0 as *const libc::c_char,
        offset: 0,
        size: 0,
        line_pos: 0,
        line: 0,
        in_key: 0,
        in_brace: 0,
        in_cond: 0,
        simulate_eol: 0,
    };
    t.source = source;
    t.input = input;
    t.size = isize;
    t.offset = 0 as libc::c_int as size_t;
    t.line = 1 as libc::c_int;
    t.line_pos = 1 as libc::c_int;
    t.in_key = 1 as libc::c_int;
    t.in_brace = 0 as libc::c_int;
    t.in_cond = 0 as libc::c_int;
    t.simulate_eol = 0 as libc::c_int;
    pParser = configparserAlloc(
        Some(malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void),
    );
    if pParser.is_null() {
        ck_assert_failed(
            b"src/configfile.c\0" as *const u8 as *const libc::c_char,
            2128 as libc::c_int as libc::c_uint,
            b"pParser\0" as *const u8 as *const libc::c_char,
        );
    }
    lasttoken = buffer_init();
    token = buffer_init();
    loop {
        ret = config_tokenizer(srv, &mut t, &mut token_id, token);
        if !(1 as libc::c_int == ret && (*context).ok != 0) {
            break;
        }
        buffer_copy_buffer(lasttoken, token);
        configparser(pParser, token_id, token, context);
        token = buffer_init();
    }
    buffer_free(token);
    if ret != -(1 as libc::c_int) && (*context).ok != 0 {
        token = buffer_init();
        buffer_copy_string(token, b"(EOL)\0" as *const u8 as *const libc::c_char);
        configparser(pParser, 1 as libc::c_int, token, context);
        if (*context).ok != 0 {
            configparser(pParser, 0 as libc::c_int, 0 as *mut buffer, context);
        }
    }
    configparserFree(
        pParser,
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    if ret == -(1 as libc::c_int) {
        log_error(
            (*srv).errh,
            b"src/configfile.c\0" as *const u8 as *const libc::c_char,
            2150 as libc::c_int as libc::c_uint,
            b"configfile parser failed at: %s\0" as *const u8 as *const libc::c_char,
            (*lasttoken).ptr,
        );
    } else if (*context).ok == 0 as libc::c_int {
        log_error(
            (*srv).errh,
            b"src/configfile.c\0" as *const u8 as *const libc::c_char,
            2153 as libc::c_int as libc::c_uint,
            b"source: %s line: %d pos: %d parser failed somehow near here: %s\0"
                as *const u8 as *const libc::c_char,
            t.source,
            t.line,
            t.line_pos,
            (*lasttoken).ptr,
        );
        ret = -(1 as libc::c_int);
    }
    buffer_free(lasttoken);
    return if ret == -(1 as libc::c_int) {
        -(1 as libc::c_int)
    } else {
        0 as libc::c_int
    };
}
#[cold]
unsafe extern "C" fn config_parse_stdin(
    mut srv: *mut server,
    mut context: *mut config_t,
) -> libc::c_int {
    let b: *mut buffer = chunk_buffer_acquire();
    let mut dlen: size_t = 0;
    let mut n: ssize_t = -(1 as libc::c_int) as ssize_t;
    loop {
        if n > 0 as libc::c_int as libc::c_long {
            buffer_commit(b, n as size_t);
        }
        let mut avail: size_t = buffer_string_space(b) as size_t;
        dlen = buffer_clen(b) as size_t;
        if (avail < 1024 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_long
            != 0
        {
            if dlen
                >= (32 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int)
                    as libc::c_ulong
            {
                log_error(
                    (*srv).errh,
                    b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                    2176 as libc::c_int as libc::c_uint,
                    b"config read from stdin is way too large\0" as *const u8
                        as *const libc::c_char,
                );
                break;
            } else {
                avail = chunk_buffer_prepare_append(
                    b,
                    ((*b).size as libc::c_ulong).wrapping_add(avail),
                );
            }
        }
        n = read(
            0 as libc::c_int,
            ((*b).ptr).offset(dlen as isize) as *mut libc::c_void,
            avail,
        );
        if !(n > 0 as libc::c_int as libc::c_long
            || n == -(1 as libc::c_int) as libc::c_long
                && *__errno_location() == 4 as libc::c_int)
        {
            break;
        }
    }
    let mut rc: libc::c_int = -(1 as libc::c_int);
    if 0 as libc::c_int as libc::c_long == n {
        rc = if dlen != 0 {
            config_parse(
                srv,
                context,
                b"-\0" as *const u8 as *const libc::c_char,
                (*b).ptr,
                dlen,
            )
        } else {
            0 as libc::c_int
        };
    } else {
        log_perror(
            (*srv).errh,
            b"src/configfile.c\0" as *const u8 as *const libc::c_char,
            2188 as libc::c_int as libc::c_uint,
            b"config read from stdin\0" as *const u8 as *const libc::c_char,
        );
    }
    if dlen != 0 {
        ck_memzero((*b).ptr as *mut libc::c_void, dlen);
    }
    chunk_buffer_release(b);
    return rc;
}
unsafe extern "C" fn config_parse_file_stream(
    mut srv: *mut server,
    mut context: *mut config_t,
    mut fn_0: *const libc::c_char,
) -> libc::c_int {
    let mut dlen: off_t = (32 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int)
        as off_t;
    let mut data: *mut libc::c_char = fdevent_load_file(
        fn_0,
        &mut dlen,
        0 as *mut log_error_st,
        Some(malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    if data.is_null() {
        log_perror(
            (*srv).errh,
            b"src/configfile.c\0" as *const u8 as *const libc::c_char,
            2200 as libc::c_int as libc::c_uint,
            b"opening configfile %s failed\0" as *const u8 as *const libc::c_char,
            fn_0,
        );
        return -(1 as libc::c_int);
    }
    let mut rc: libc::c_int = 0 as libc::c_int;
    if dlen != 0 {
        rc = config_parse(srv, context, fn_0, data, dlen as size_t);
        ck_memzero(data as *mut libc::c_void, dlen as size_t);
    }
    free(data as *mut libc::c_void);
    return rc;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_parse_file(
    mut srv: *mut server,
    mut context: *mut config_t,
    mut fn_0: *const libc::c_char,
) -> libc::c_int {
    let filename: *mut buffer = buffer_init();
    let fnlen: size_t = strlen(fn_0);
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut flags: libc::c_int = (1 as libc::c_int) << 10 as libc::c_int;
    let mut gl: glob_t = glob_t {
        gl_pathc: 0,
        gl_pathv: 0 as *mut *mut libc::c_char,
        gl_offs: 0,
        gl_flags: 0,
        gl_closedir: None,
        gl_readdir: None,
        gl_opendir: None,
        gl_lstat: None,
        gl_stat: None,
    };
    if *fn_0.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
        || *fn_0.offset(0 as libc::c_int as isize) as libc::c_int == '\\' as i32
        || *fn_0.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && (*fn_0.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
                || *fn_0.offset(1 as libc::c_int as isize) as libc::c_int == '\\' as i32)
        || *fn_0.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && *fn_0.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
            && (*fn_0.offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32
                || *fn_0.offset(2 as libc::c_int as isize) as libc::c_int == '\\' as i32)
    {
        buffer_copy_string_len(filename, fn_0, fnlen);
    } else {
        buffer_copy_path_len2(
            filename,
            (*(*context).basedir).ptr,
            buffer_clen((*context).basedir) as size_t,
            fn_0,
            fnlen,
        );
    }
    match glob((*filename).ptr, flags, None, &mut gl) {
        0 => {
            let mut i: size_t = 0 as libc::c_int as size_t;
            while i < gl.gl_pathc {
                ret = config_parse_file_stream(
                    srv,
                    context,
                    *(gl.gl_pathv).offset(i as isize),
                );
                if 0 as libc::c_int != ret {
                    break;
                }
                i = i.wrapping_add(1);
            }
            globfree(&mut gl);
        }
        3 => {
            if *((*filename).ptr)
                .offset(
                    strcspn(
                        (*filename).ptr,
                        b"*?[]{}\0" as *const u8 as *const libc::c_char,
                    ) as isize,
                ) as libc::c_int != '\u{0}' as i32
            {
                ret = 0 as libc::c_int;
            } else {
                log_error(
                    (*srv).errh,
                    b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                    2247 as libc::c_int as libc::c_uint,
                    b"include file not found: %s\0" as *const u8 as *const libc::c_char,
                    (*filename).ptr,
                );
            }
        }
        2 | 1 => {
            log_perror(
                (*srv).errh,
                b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                2252 as libc::c_int as libc::c_uint,
                b"glob() %s failed\0" as *const u8 as *const libc::c_char,
                (*filename).ptr,
            );
        }
        _ => {}
    }
    buffer_free(filename);
    return ret;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_parse_cmd(
    mut srv: *mut server,
    mut context: *mut config_t,
    mut cmd: *const libc::c_char,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut fds: [libc::c_int; 2] = [0; 2];
    let mut oldpwd: [libc::c_char; 4096] = [0; 4096];
    if (getcwd(
        oldpwd.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    ))
        .is_null()
    {
        log_perror(
            (*srv).errh,
            b"src/configfile.c\0" as *const u8 as *const libc::c_char,
            2282 as libc::c_int as libc::c_uint,
            b"getcwd()\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if buffer_is_blank((*context).basedir) == 0 {
        if 0 as libc::c_int != chdir((*(*context).basedir).ptr) {
            log_perror(
                (*srv).errh,
                b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                2288 as libc::c_int as libc::c_uint,
                b"cannot change directory to %s\0" as *const u8 as *const libc::c_char,
                (*(*context).basedir).ptr,
            );
            return -(1 as libc::c_int);
        }
    }
    if fdevent_pipe_cloexec(fds.as_mut_ptr(), 65536 as libc::c_int as libc::c_uint) != 0
    {
        log_perror(
            (*srv).errh,
            b"src/configfile.c\0" as *const u8 as *const libc::c_char,
            2295 as libc::c_int as libc::c_uint,
            b"pipe()\0" as *const u8 as *const libc::c_char,
        );
        ret = -(1 as libc::c_int);
    } else {
        let mut shell: *mut libc::c_char = getenv(
            b"SHELL\0" as *const u8 as *const libc::c_char,
        );
        let mut args: [*mut libc::c_char; 4] = [0 as *mut libc::c_char; 4];
        let mut pid: pid_t = 0;
        let ref mut fresh6 = *(&mut *args.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut *mut libc::c_char as *mut *const libc::c_char);
        *fresh6 = if !shell.is_null() {
            shell as *const libc::c_char
        } else {
            b"/bin/sh\0" as *const u8 as *const libc::c_char
        };
        let ref mut fresh7 = *(&mut *args.as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut *mut libc::c_char as *mut *const libc::c_char);
        *fresh7 = b"-c\0" as *const u8 as *const libc::c_char;
        let ref mut fresh8 = *(&mut *args.as_mut_ptr().offset(2 as libc::c_int as isize)
            as *mut *mut libc::c_char as *mut *const libc::c_char);
        *fresh8 = cmd;
        args[3 as libc::c_int as usize] = 0 as *mut libc::c_char;
        pid = fdevent_fork_execve(
            args[0 as libc::c_int as usize],
            args.as_mut_ptr(),
            0 as *mut *mut libc::c_char,
            -(1 as libc::c_int),
            fds[1 as libc::c_int as usize],
            -(1 as libc::c_int),
            -(1 as libc::c_int),
        );
        if -(1 as libc::c_int) == pid {
            log_perror(
                (*srv).errh,
                b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                2309 as libc::c_int as libc::c_uint,
                b"fork/exec(%s)\0" as *const u8 as *const libc::c_char,
                cmd,
            );
            ret = -(1 as libc::c_int);
        } else {
            let mut rd: ssize_t = 0;
            let mut wstatus: libc::c_int = 0 as libc::c_int;
            let mut out: *mut buffer = buffer_init();
            close(fds[1 as libc::c_int as usize]);
            fds[1 as libc::c_int as usize] = -(1 as libc::c_int);
            loop {
                rd = read(
                    fds[0 as libc::c_int as usize],
                    buffer_string_prepare_append(out, 1023 as libc::c_int as size_t)
                        as *mut libc::c_void,
                    1023 as libc::c_int as size_t,
                );
                if rd >= 0 as libc::c_int as libc::c_long {
                    buffer_commit(out, rd as size_t);
                }
                if !(rd > 0 as libc::c_int as libc::c_long
                    || -(1 as libc::c_int) as libc::c_long == rd
                        && *__errno_location() == 4 as libc::c_int)
                {
                    break;
                }
            }
            if 0 as libc::c_int as libc::c_long != rd {
                log_perror(
                    (*srv).errh,
                    b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                    2323 as libc::c_int as libc::c_uint,
                    b"read \"%s\"\0" as *const u8 as *const libc::c_char,
                    cmd,
                );
                ret = -(1 as libc::c_int);
            }
            close(fds[0 as libc::c_int as usize]);
            fds[0 as libc::c_int as usize] = -(1 as libc::c_int);
            if pid != fdevent_waitpid(pid, &mut wstatus, 0 as libc::c_int) {
                log_perror(
                    (*srv).errh,
                    b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                    2329 as libc::c_int as libc::c_uint,
                    b"waitpid \"%s\"\0" as *const u8 as *const libc::c_char,
                    cmd,
                );
                ret = -(1 as libc::c_int);
            }
            if 0 as libc::c_int != wstatus {
                log_error(
                    (*srv).errh,
                    b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                    2333 as libc::c_int as libc::c_uint,
                    b"command \"%s\" exited non-zero: %d\0" as *const u8
                        as *const libc::c_char,
                    cmd,
                    (wstatus & 0xff00 as libc::c_int) >> 8 as libc::c_int,
                );
                ret = -(1 as libc::c_int);
            }
            if -(1 as libc::c_int) != ret {
                ret = config_parse(
                    srv,
                    context,
                    cmd,
                    (*out).ptr,
                    buffer_clen(out) as size_t,
                );
            }
            buffer_free(out);
        }
        if -(1 as libc::c_int) != fds[0 as libc::c_int as usize] {
            close(fds[0 as libc::c_int as usize]);
        }
        if -(1 as libc::c_int) != fds[1 as libc::c_int as usize] {
            close(fds[1 as libc::c_int as usize]);
        }
    }
    if 0 as libc::c_int != chdir(oldpwd.as_mut_ptr()) {
        log_perror(
            (*srv).errh,
            b"src/configfile.c\0" as *const u8 as *const libc::c_char,
            2349 as libc::c_int as libc::c_uint,
            b"cannot change directory to %s\0" as *const u8 as *const libc::c_char,
            oldpwd.as_mut_ptr(),
        );
        ret = -(1 as libc::c_int);
    }
    return ret;
}
unsafe extern "C" fn config_remoteip_normalize_ipv6(
    b: *mut buffer,
    tb: *mut buffer,
) -> libc::c_int {
    buffer_clear(tb);
    if *((*b).ptr).offset(0 as libc::c_int as isize) as libc::c_int != '[' as i32 {
        buffer_append_str3(
            tb,
            b"[\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            (*b).ptr,
            buffer_clen(b) as size_t,
            b"]\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    } else {
        buffer_append_buffer(tb, b);
    }
    let mut rc: libc::c_int = http_request_host_normalize(tb, 0 as libc::c_int);
    if 0 as libc::c_int == rc {
        let mut blen: size_t = buffer_clen(tb) as size_t;
        if blen > 1 as libc::c_int as libc::c_ulong {
            buffer_copy_string_len(
                b,
                ((*tb).ptr).offset(1 as libc::c_int as isize),
                blen.wrapping_sub(2 as libc::c_int as libc::c_ulong),
            );
        }
    }
    return rc;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_remoteip_normalize(
    b: *mut buffer,
    tb: *mut buffer,
) -> libc::c_int {
    if *((*b).ptr).offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
        return 1 as libc::c_int;
    }
    let slash: *const libc::c_char = strchr((*b).ptr, '/' as i32);
    let colon: *const libc::c_char = strchr((*b).ptr, ':' as i32);
    let mut nm_bits: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    if !slash.is_null() {
        let mut nptr: *mut libc::c_char = 0 as *mut libc::c_char;
        nm_bits = strtoul(
            slash.offset(1 as libc::c_int as isize),
            &mut nptr,
            10 as libc::c_int,
        );
        if *nptr as libc::c_int != 0 || 0 as libc::c_int as libc::c_ulong == nm_bits
            || nm_bits
                > (if !colon.is_null() { 128 as libc::c_int } else { 32 as libc::c_int })
                    as libc::c_ulong
        {
            return -(1 as libc::c_int);
        }
        buffer_truncate(
            b,
            slash.offset_from((*b).ptr) as libc::c_long as size_t as uint32_t,
        );
    }
    let mut family: libc::c_int = if !colon.is_null() {
        10 as libc::c_int
    } else {
        2 as libc::c_int
    };
    let mut rc: libc::c_int = if family == 2 as libc::c_int {
        http_request_host_normalize(b, 0 as libc::c_int)
    } else {
        config_remoteip_normalize_ipv6(b, tb)
    };
    let mut len: uint32_t = buffer_clen(b);
    if nm_bits != 0 {
        buffer_append_string_len(
            b,
            b"/\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        buffer_append_int(b, nm_bits as libc::c_int as intmax_t);
    }
    if 0 as libc::c_int != rc {
        return -(1 as libc::c_int);
    }
    let mut after: *mut libc::c_char = buffer_string_prepare_append(
        b,
        (1 as libc::c_int + 7 as libc::c_int + 28 as libc::c_int) as size_t,
    );
    after = after.offset(1);
    *(after as *mut libc::c_uchar) = nm_bits as libc::c_uchar;
    let addr: *mut sock_addr = ((after as uintptr_t)
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(7 as libc::c_int as libc::c_ulong)
        & !(7 as libc::c_int) as libc::c_ulong) as *mut sock_addr;
    if nm_bits != 0 {
        *((*b).ptr).offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    }
    rc = sock_addr_inet_pton(addr, (*b).ptr, family, 0 as libc::c_int as libc::c_ushort);
    if nm_bits != 0 {
        *((*b).ptr).offset(len as isize) = '/' as i32 as libc::c_char;
    }
    return (1 as libc::c_int == rc) as libc::c_int;
}
unsafe extern "C" fn context_init(mut srv: *mut server, mut context: *mut config_t) {
    (*context).srv = srv;
    (*context).ok = 1 as libc::c_int;
    vector_config_weak_init(&mut (*context).configs_stack);
    (*context).basedir = buffer_init();
}
unsafe extern "C" fn context_free(mut context: *mut config_t) {
    vector_config_weak_clear(&mut (*context).configs_stack);
    buffer_free((*context).basedir);
}
#[inline(never)]
unsafe extern "C" fn config_vars_init(mut a: *mut array) {
    let mut dcwd: [libc::c_char; 4096] = [0; 4096];
    if !(getcwd(
        dcwd.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    ))
        .is_null()
    {
        array_set_key_value(
            a,
            b"var.CWD\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            dcwd.as_mut_ptr(),
            strlen(dcwd.as_mut_ptr()) as uint32_t,
        );
    }
    *array_get_int_ptr(
        a,
        b"var.PID\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    ) = getpid();
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_read(
    mut srv: *mut server,
    mut fn_0: *const libc::c_char,
) -> libc::c_int {
    let mut context: config_t = config_t {
        srv: 0 as *mut server,
        ok: 0,
        all_configs: 0 as *mut array,
        configs_stack: vector_config_weak {
            data: 0 as *mut *mut data_config,
            used: 0,
            size: 0,
        },
        current: 0 as *mut data_config,
        basedir: 0 as *mut buffer,
    };
    let mut dc: *mut data_config = 0 as *mut data_config;
    let mut ret: libc::c_int = 0;
    let mut pos: *mut libc::c_char = 0 as *mut libc::c_char;
    context_init(srv, &mut context);
    context.all_configs = (*srv).config_context;
    pos = strrchr(fn_0, '/' as i32);
    if !pos.is_null() {
        buffer_copy_string_len(
            context.basedir,
            fn_0,
            (pos.offset_from(fn_0) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as size_t,
        );
    }
    dc = data_config_init();
    buffer_copy_string_len(
        &mut (*dc).key,
        b"global\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    config_vars_init((*dc).value);
    if !((*context.all_configs).used == 0 as libc::c_int as libc::c_uint) {
        ck_assert_failed(
            b"src/configfile.c\0" as *const u8 as *const libc::c_char,
            2467 as libc::c_int as libc::c_uint,
            b"context.all_configs->used == 0\0" as *const u8 as *const libc::c_char,
        );
    }
    (*dc).context_ndx = (*context.all_configs).used as libc::c_int;
    array_insert_unique(context.all_configs, dc as *mut data_unset);
    context.current = dc;
    ret = if 0 as libc::c_int != strcmp(fn_0, b"-\0" as *const u8 as *const libc::c_char)
    {
        config_parse_file_stream(srv, &mut context, fn_0)
    } else {
        config_parse_stdin(srv, &mut context)
    };
    if 0 as libc::c_int == ret && context.ok != 0
        && 0 as libc::c_int as libc::c_ulong != context.configs_stack.used
    {
        ck_assert_failed(
            b"src/configfile.c\0" as *const u8 as *const libc::c_char,
            2477 as libc::c_int as libc::c_uint,
            b"!(0 == ret && context.ok && 0 != context.configs_stack.used)\0"
                as *const u8 as *const libc::c_char,
        );
    }
    context_free(&mut context);
    if 0 as libc::c_int != ret {
        return ret;
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*(*srv).config_context).used {
        dc = *((*(*srv).config_context).data).offset(i as isize) as *mut data_config;
        if !((*dc).context_ndx == i as libc::c_int) {
            let mut j: uint32_t = i;
            while j < (*(*srv).config_context).used {
                dc = *((*(*srv).config_context).data).offset(j as isize)
                    as *mut data_config;
                if (*dc).context_ndx == i as libc::c_int {
                    let ref mut fresh9 = *((*(*srv).config_context).data)
                        .offset(j as isize);
                    *fresh9 = *((*(*srv).config_context).data).offset(i as isize);
                    let ref mut fresh10 = *((*(*srv).config_context).data)
                        .offset(i as isize);
                    *fresh10 = dc as *mut data_unset;
                    break;
                } else {
                    j = j.wrapping_add(1);
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if 0 as libc::c_int != config_insert_srvconf(srv) {
        return -(1 as libc::c_int);
    }
    if 0 as libc::c_int != config_insert(srv) {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_set_defaults(mut srv: *mut server) -> libc::c_int {
    let mut i: size_t = 0;
    let mut s: *mut request_config = &mut (*((*srv).config_data_base
        as *mut config_data_base))
        .defaults;
    let mut st1: stat = stat {
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
    let mut st2: stat = stat {
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
    if fdevent_config(&mut (*srv).srvconf.event_handler, (*srv).errh) <= 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if !((*srv).srvconf.changeroot).is_null() {
        if -(1 as libc::c_int) == stat((*(*srv).srvconf.changeroot).ptr, &mut st1) {
            log_error(
                (*srv).errh,
                b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                2526 as libc::c_int as libc::c_uint,
                b"server.chroot doesn't exist: %s\0" as *const u8 as *const libc::c_char,
                (*(*srv).srvconf.changeroot).ptr,
            );
            return -(1 as libc::c_int);
        }
        if !(st1.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint)
        {
            log_error(
                (*srv).errh,
                b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                2532 as libc::c_int as libc::c_uint,
                b"server.chroot isn't a directory: %s\0" as *const u8
                    as *const libc::c_char,
                (*(*srv).srvconf.changeroot).ptr,
            );
            return -(1 as libc::c_int);
        }
    }
    if (*(*srv).srvconf.upload_tempdirs).used == 0 {
        let mut tmpdir: *const libc::c_char = getenv(
            b"TMPDIR\0" as *const u8 as *const libc::c_char,
        );
        if tmpdir.is_null() {
            tmpdir = b"/var/tmp\0" as *const u8 as *const libc::c_char;
        }
        array_insert_value(
            (*srv).srvconf.upload_tempdirs,
            tmpdir,
            strlen(tmpdir) as uint32_t,
        );
    }
    if (*(*srv).srvconf.upload_tempdirs).used != 0 {
        let tb: *mut buffer = (*srv).tmp_buf;
        buffer_clear(tb);
        if !((*srv).srvconf.changeroot).is_null() {
            buffer_copy_buffer(tb, (*srv).srvconf.changeroot);
        }
        let len: size_t = buffer_clen(tb) as size_t;
        i = 0 as libc::c_int as size_t;
        while i < (*(*srv).srvconf.upload_tempdirs).used as libc::c_ulong {
            let ds: *const data_string = *((*(*srv).srvconf.upload_tempdirs).data)
                .offset(i as isize) as *mut data_string;
            if len != 0 {
                buffer_truncate(tb, len as uint32_t);
                buffer_append_path_len(
                    tb,
                    (*ds).value.ptr,
                    buffer_clen(&(*ds).value) as size_t,
                );
            } else {
                buffer_copy_buffer(tb, &(*ds).value);
            }
            if -(1 as libc::c_int) == stat((*tb).ptr, &mut st1) {
                log_error(
                    (*srv).errh,
                    b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                    2562 as libc::c_int as libc::c_uint,
                    b"server.upload-dirs doesn't exist: %s\0" as *const u8
                        as *const libc::c_char,
                    (*tb).ptr,
                );
            } else if !(st1.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint)
                {
                log_error(
                    (*srv).errh,
                    b"src/configfile.c\0" as *const u8 as *const libc::c_char,
                    2565 as libc::c_int as libc::c_uint,
                    b"server.upload-dirs isn't a directory: %s\0" as *const u8
                        as *const libc::c_char,
                    (*tb).ptr,
                );
            }
            i = i.wrapping_add(1);
        }
    }
    chunkqueue_set_tempdirs_default(
        (*srv).srvconf.upload_tempdirs,
        (*srv).srvconf.upload_temp_file_size as off_t,
    );
    if ((*s).document_root).is_null() || buffer_is_blank((*s).document_root) != 0 {
        log_error(
            (*srv).errh,
            b"src/configfile.c\0" as *const u8 as *const libc::c_char,
            2576 as libc::c_int as libc::c_uint,
            b"server.document-root is not set\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if 2 as libc::c_int == (*s).force_lowercase_filenames as libc::c_int {
        (*s).force_lowercase_filenames = 0 as libc::c_int as libc::c_uchar;
        let tb_0: *mut buffer = (*srv).tmp_buf;
        buffer_copy_string_len_lc(
            tb_0,
            (*(*s).document_root).ptr,
            buffer_clen((*s).document_root) as size_t,
        );
        if 0 as libc::c_int == stat((*tb_0).ptr, &mut st1) {
            let mut is_lower: libc::c_int = 0 as libc::c_int;
            is_lower = buffer_is_equal(tb_0, (*s).document_root);
            buffer_copy_buffer(tb_0, (*s).document_root);
            buffer_to_upper(tb_0);
            if is_lower != 0 && buffer_is_equal(tb_0, (*s).document_root) != 0 {
                (*s).force_lowercase_filenames = 0 as libc::c_int as libc::c_uchar;
            } else if 0 as libc::c_int == stat((*tb_0).ptr, &mut st2) {
                if st1.st_ino == st2.st_ino {
                    (*s).force_lowercase_filenames = 1 as libc::c_int as libc::c_uchar;
                }
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    cpk = [
        {
            let mut init = config_plugin_keys_t {
                k: b"server.modules\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_VLIST as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.compat-module-load\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.systemd-socket-activation\0" as *const u8
                    as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.port\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.bind\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.network-backend\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.chroot\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.username\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.groupname\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.errorlog-placeholder-moved-to-config-insert\0" as *const u8
                    as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 51]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.breakagelog-placeholder-moved-to-config-insert\0"
                    as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 54]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.errorlog-use-syslog\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.syslog-facility\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.core-files\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.event-handler\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.pid-file\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.max-worker\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.max-fds\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.max-connections\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.max-request-field-size\0" as *const u8
                    as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_INT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.chunkqueue-chunk-sz\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_INT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.upload-temp-file-size\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_INT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.upload-dirs\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_VLIST as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.http-parseopts\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVSTRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.http-parseopt-header-strict\0" as *const u8
                    as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 35]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.http-parseopt-host-strict\0" as *const u8
                    as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.http-parseopt-host-normalize\0" as *const u8
                    as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.reject-expect-100-with-417\0" as *const u8
                    as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 34]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.stat-cache-engine\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"mimetype.xattr-name\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
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
                k: b"debug.log-request-header-on-error\0" as *const u8
                    as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 34]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.feature-flags\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVANY as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
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
    cpk_0 = [
        {
            let mut init = config_plugin_keys_t {
                k: b"server.document-root\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.name\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.tag\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.max-request-size\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_INT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.max-keep-alive-requests\0" as *const u8
                    as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.max-keep-alive-idle\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.max-read-idle\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.max-write-idle\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.errorfile-prefix\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.error-handler\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.error-handler-404\0" as *const u8 as *const libc::c_char,
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
                k: b"server.error-intercept\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.force-lowercase-filenames\0" as *const u8
                    as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.follow-symlink\0" as *const u8 as *const libc::c_char,
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
                k: b"server.protocol-http11\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.range-requests\0" as *const u8 as *const libc::c_char,
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
                k: b"server.stream-request-body\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.stream-response-body\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.kbytes-per-second\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"connection.kbytes-per-second\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"mimetype.assign\0" as *const u8 as *const libc::c_char,
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
                k: b"mimetype.use-xattr\0" as *const u8 as *const libc::c_char,
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
                k: b"etag.use-inode\0" as *const u8 as *const libc::c_char,
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
                k: b"etag.use-mtime\0" as *const u8 as *const libc::c_char,
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
                k: b"etag.use-size\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"debug.log-condition-handling\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"debug.log-file-not-found\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"debug.log-request-handling\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"debug.log-request-header\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"debug.log-response-header\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"debug.log-timeouts\0" as *const u8 as *const libc::c_char,
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
                k: b"debug.log-state-handling\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.errorlog\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.breakagelog\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
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
