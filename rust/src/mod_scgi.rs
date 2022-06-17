use ::libc;
extern "C" {
    pub type fdnode_st;
    pub type stat_cache_entry;
    pub type pcre2_real_match_data_8;
    pub type h2con;
    pub type fdevents;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn buffer_extend(b: *mut buffer, x: size_t) -> *mut libc::c_char;
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
    fn array_get_int_ptr(
        a: *mut array,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut libc::c_int;
    fn gw_handle_waitpid_cb(
        srv: *mut server,
        p_d: *mut libc::c_void,
        pid: pid_t,
        status: libc::c_int,
    ) -> handler_t;
    fn gw_handle_trigger(srv: *mut server, p_d: *mut libc::c_void) -> handler_t;
    fn gw_handle_subrequest(r: *mut request_st, p_d: *mut libc::c_void) -> handler_t;
    fn gw_handle_request_reset(r: *mut request_st, p_d: *mut libc::c_void) -> handler_t;
    fn gw_check_extension(
        r: *mut request_st,
        p: *mut gw_plugin_data,
        uri_path_handler: libc::c_int,
        hctx_sz: size_t,
    ) -> handler_t;
    fn gw_get_defaults_balance(srv: *mut server, b: *const buffer) -> libc::c_int;
    fn gw_set_defaults_backend(
        srv: *mut server,
        p: *mut gw_plugin_data,
        a: *const array,
        s: *mut gw_plugin_config,
        sh_exec: libc::c_int,
        cpkkey: *const libc::c_char,
    ) -> libc::c_int;
    fn gw_free(p_d: *mut libc::c_void);
    fn gw_plugin_config_free(s: *mut gw_plugin_config);
    fn gw_init() -> *mut libc::c_void;
    fn chunk_buffer_acquire() -> *mut buffer;
    fn chunkqueue_append_chunkqueue(cq: *mut chunkqueue, src: *mut chunkqueue);
    fn chunkqueue_prepend_buffer_open_sz(cq: *mut chunkqueue, sz: size_t) -> *mut buffer;
    fn chunkqueue_prepend_buffer_commit(cq: *mut chunkqueue);
    fn chunkqueue_mark_written(cq: *mut chunkqueue, len: off_t);
    fn chunkqueue_remove_finished_chunks(cq: *mut chunkqueue);
    fn config_plugin_values_init(
        srv: *mut server,
        p_d: *mut libc::c_void,
        cpk: *const config_plugin_keys_t,
        mname: *const libc::c_char,
    ) -> libc::c_int;
    static mut plugin_stats: array;
    fn config_check_cond(r: *mut request_st, context_ndx: libc::c_int) -> libc::c_int;
    fn http_cgi_headers(
        r: *mut request_st,
        opts: *mut http_cgi_opts,
        cb: http_cgi_header_append_cb,
        vdata: *mut libc::c_void,
    ) -> libc::c_int;
    fn log_error(
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
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
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
pub type socklen_t = __socklen_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
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
    pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
pub struct char_array {
    pub ptr: *mut *mut libc::c_char,
    pub size: uint32_t,
    pub used: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gw_proc {
    pub next: *mut gw_proc,
    pub state: C2RustUnnamed_0,
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PROC_STATE_KILLED: C2RustUnnamed_0 = 4;
pub const PROC_STATE_DIED: C2RustUnnamed_0 = 3;
pub const PROC_STATE_DIED_WAIT_FOR_PID: C2RustUnnamed_0 = 2;
pub const PROC_STATE_OVERLOADED: C2RustUnnamed_0 = 1;
pub const PROC_STATE_RUNNING: C2RustUnnamed_0 = 0;
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
    pub plugins: C2RustUnnamed_1,
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
pub struct C2RustUnnamed_1 {
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
    pub mode: C2RustUnnamed_2,
    pub fd: libc::c_int,
    pub b: buffer,
    pub fn_0: *const libc::c_char,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const FDLOG_PIPE: C2RustUnnamed_2 = 3;
pub const FDLOG_SYSLOG: C2RustUnnamed_2 = 2;
pub const FDLOG_FD: C2RustUnnamed_2 = 1;
pub const FDLOG_FILE: C2RustUnnamed_2 = 0;
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
    pub type_0: C2RustUnnamed_5,
    pub mem: *mut buffer,
    pub offset: off_t,
    pub file: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub length: off_t,
    pub fd: libc::c_int,
    pub is_temp: libc::c_int,
    pub mmap: C2RustUnnamed_4,
    pub ref_0: *mut libc::c_void,
    pub refchg: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub start: *mut libc::c_char,
    pub length: size_t,
    pub offset: off_t,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const FILE_CHUNK: C2RustUnnamed_5 = 1;
pub const MEM_CHUNK: C2RustUnnamed_5 = 0;
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
pub type C2RustUnnamed_6 = libc::c_uint;
pub const T_CONFIG_SCOPE_CONNECTION: C2RustUnnamed_6 = 2;
pub const T_CONFIG_SCOPE_SERVER: C2RustUnnamed_6 = 1;
pub const T_CONFIG_SCOPE_UNSET: C2RustUnnamed_6 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_plugin_keys_t {
    pub k: *const libc::c_char,
    pub klen: uint8_t,
    pub ktype: uint8_t,
    pub scope: uint8_t,
}
pub type C2RustUnnamed_7 = libc::c_uint;
pub const BACKEND_AJP13: C2RustUnnamed_7 = 4;
pub const BACKEND_SCGI: C2RustUnnamed_7 = 3;
pub const BACKEND_FASTCGI: C2RustUnnamed_7 = 2;
pub const BACKEND_CGI: C2RustUnnamed_7 = 1;
pub const BACKEND_PROXY: C2RustUnnamed_7 = 0;
pub type plugin_config = gw_plugin_config;
pub type plugin_data = gw_plugin_data;
pub type handler_ctx = gw_handler_ctx;
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
pub type C2RustUnnamed_8 = libc::c_uint;
pub const LI_PROTOCOL_UWSGI: C2RustUnnamed_8 = 1;
pub const LI_PROTOCOL_SCGI: C2RustUnnamed_8 = 0;
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
unsafe extern "C" fn status_counter_inc(mut s: *const libc::c_char, mut len: size_t) {
    let ref mut fresh0 = *array_get_int_ptr(&mut plugin_stats, s, len as uint32_t);
    *fresh0 += 1;
}
unsafe extern "C" fn mod_scgi_merge_config_cpv(
    pconf: *mut plugin_config,
    cpv: *const config_plugin_value_t,
) {
    match (*cpv).k_id {
        0 => {
            if (*cpv).vtype as libc::c_uint
                == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
            {
                let gw: *mut gw_plugin_config = (*cpv).v.v as *mut gw_plugin_config;
                (*pconf).exts = (*gw).exts;
                (*pconf).exts_auth = (*gw).exts_auth;
                (*pconf).exts_resp = (*gw).exts_resp;
            }
        }
        1 => {
            (*pconf).balance = (*cpv).v.u as libc::c_int;
        }
        2 => {
            (*pconf).debug = (*cpv).v.u as libc::c_int;
        }
        3 => {
            (*pconf).ext_mapping = (*cpv).v.a;
        }
        4 => {
            (*pconf).proto = (*cpv).v.u as libc::c_int;
        }
        _ => return,
    };
}
unsafe extern "C" fn mod_scgi_merge_config(
    pconf: *mut plugin_config,
    mut cpv: *const config_plugin_value_t,
) {
    loop {
        mod_scgi_merge_config_cpv(pconf, cpv);
        cpv = cpv.offset(1);
        if !((*cpv).k_id != -(1 as libc::c_int)) {
            break;
        }
    };
}
unsafe extern "C" fn mod_scgi_patch_config(r: *mut request_st, p: *mut plugin_data) {
    memcpy(
        &mut (*p).conf as *mut gw_plugin_config as *mut libc::c_void,
        &mut (*p).defaults as *mut gw_plugin_config as *const libc::c_void,
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
            mod_scgi_merge_config(
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
static mut cpk: [config_plugin_keys_t; 6] = [config_plugin_keys_t {
    k: 0 as *const libc::c_char,
    klen: 0,
    ktype: 0,
    scope: 0,
}; 6];
#[cold]
unsafe extern "C" fn mod_scgi_set_defaults(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    if config_plugin_values_init(
        srv,
        p as *mut libc::c_void,
        cpk.as_ptr(),
        b"mod_scgi\0" as *const u8 as *const libc::c_char,
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
                    let mut gw: *mut gw_plugin_config = calloc(
                        1 as libc::c_int as libc::c_ulong,
                        ::std::mem::size_of::<gw_plugin_config>() as libc::c_ulong,
                    ) as *mut gw_plugin_config;
                    if gw.is_null() {
                        ck_assert_failed(
                            b"src/mod_scgi.c\0" as *const u8 as *const libc::c_char,
                            98 as libc::c_int as libc::c_uint,
                            b"gw\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if gw_set_defaults_backend(
                        srv,
                        p,
                        (*cpv).v.a,
                        gw,
                        1 as libc::c_int,
                        cpk[(*cpv).k_id as usize].k,
                    ) == 0
                    {
                        gw_plugin_config_free(gw);
                        return HANDLER_ERROR;
                    }
                    (*cpv).v.v = gw as *mut libc::c_void;
                    (*cpv).vtype = T_CONFIG_LOCAL;
                }
                1 => {
                    (*cpv)
                        .v
                        .u = gw_get_defaults_balance(srv, (*cpv).v.b) as libc::c_uint;
                }
                2 => {}
                3 => {}
                4 => {
                    if buffer_eq_slen(
                        (*cpv).v.b,
                        b"scgi\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    ) != 0
                    {
                        (*cpv).v.u = LI_PROTOCOL_SCGI as libc::c_int as libc::c_uint;
                    } else if buffer_eq_slen(
                            (*cpv).v.b,
                            b"uwsgi\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        ) != 0
                        {
                        (*cpv).v.u = LI_PROTOCOL_UWSGI as libc::c_int as libc::c_uint;
                    } else {
                        log_error(
                            (*srv).errh,
                            b"src/mod_scgi.c\0" as *const u8 as *const libc::c_char,
                            120 as libc::c_int as libc::c_uint,
                            b"unexpected type for key: %sexpected \"scgi\" or \"uwsgi\"\0"
                                as *const u8 as *const libc::c_char,
                            cpk[(*cpv).k_id as usize].k,
                        );
                        return HANDLER_ERROR;
                    }
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
            mod_scgi_merge_config(&mut (*p).defaults, cpv_0);
        }
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn scgi_env_add_scgi(
    mut venv: *mut libc::c_void,
    mut key: *const libc::c_char,
    mut key_len: size_t,
    mut val: *const libc::c_char,
    mut val_len: size_t,
) -> libc::c_int {
    let mut env: *mut buffer = venv as *mut buffer;
    let mut len: size_t = 0;
    if key.is_null() || val.is_null() && val_len != 0 {
        return -(1 as libc::c_int);
    }
    len = key_len.wrapping_add(val_len).wrapping_add(2 as libc::c_int as libc::c_ulong);
    let mut dst: *mut libc::c_char = buffer_extend(env, len);
    memcpy(dst as *mut libc::c_void, key as *const libc::c_void, key_len);
    *dst.offset(key_len as isize) = '\u{0}' as i32 as libc::c_char;
    dst = dst.offset(key_len.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    memcpy(dst as *mut libc::c_void, val as *const libc::c_void, val_len);
    *dst.offset(val_len as isize) = '\u{0}' as i32 as libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn scgi_env_add_uwsgi(
    mut venv: *mut libc::c_void,
    mut key: *const libc::c_char,
    mut key_len: size_t,
    mut val: *const libc::c_char,
    mut val_len: size_t,
) -> libc::c_int {
    if key.is_null() || val.is_null() && val_len != 0 {
        return -(1 as libc::c_int);
    }
    if key_len
        > (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong
        || val_len
            > (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    let mut dst: *mut libc::c_char = buffer_extend(
        venv as *mut buffer,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_add(key_len)
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(val_len),
    );
    *dst
        .offset(
            0 as libc::c_int as isize,
        ) = (key_len & 0xff as libc::c_int as libc::c_ulong) as libc::c_char;
    *dst
        .offset(
            1 as libc::c_int as isize,
        ) = (key_len >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as libc::c_char;
    memcpy(
        dst.offset(2 as libc::c_int as isize) as *mut libc::c_void,
        key as *const libc::c_void,
        key_len,
    );
    dst = dst.offset((2 as libc::c_int as libc::c_ulong).wrapping_add(key_len) as isize);
    *dst
        .offset(
            0 as libc::c_int as isize,
        ) = (val_len & 0xff as libc::c_int as libc::c_ulong) as libc::c_char;
    *dst
        .offset(
            1 as libc::c_int as isize,
        ) = (val_len >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as libc::c_char;
    memcpy(
        dst.offset(2 as libc::c_int as isize) as *mut libc::c_void,
        val as *const libc::c_void,
        val_len,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn scgi_create_env(mut hctx: *mut handler_ctx) -> handler_t {
    let mut host: *mut gw_host = (*hctx).host;
    let r: *mut request_st = (*hctx).r;
    let mut opts: http_cgi_opts = {
        let mut init = http_cgi_opts_t {
            authorizer: 0 as libc::c_int,
            break_scriptfilename_for_php: 0 as libc::c_int,
            docroot: (*host).docroot,
            strip_request_uri: 0 as *const buffer,
        };
        init
    };
    let mut scgi_env_add: http_cgi_header_append_cb = if (*hctx).conf.proto
        == LI_PROTOCOL_SCGI as libc::c_int
    {
        Some(
            scgi_env_add_scgi
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    size_t,
                    *const libc::c_char,
                    size_t,
                ) -> libc::c_int,
        )
    } else {
        Some(
            scgi_env_add_uwsgi
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    size_t,
                    *const libc::c_char,
                    size_t,
                ) -> libc::c_int,
        )
    };
    let mut offset: size_t = 0;
    let mut rsz: size_t = ((*r).read_queue.bytes_out - (*hctx).wb.bytes_in) as size_t;
    if rsz >= 65536 as libc::c_int as libc::c_ulong {
        rsz = (*r).rqst_header_len as size_t;
    }
    let b: *mut buffer = chunkqueue_prepend_buffer_open_sz(&mut (*hctx).wb, rsz);
    buffer_copy_string_len(
        b,
        b"          \0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    if 0 as libc::c_int
        != http_cgi_headers(r, &mut opts, scgi_env_add, b as *mut libc::c_void)
    {
        (*r).http_status = 400 as libc::c_int;
        (*r).handler_module = 0 as *const plugin;
        buffer_clear(b);
        chunkqueue_remove_finished_chunks(&mut (*hctx).wb);
        return HANDLER_FINISHED;
    }
    if (*hctx).conf.proto == LI_PROTOCOL_SCGI as libc::c_int {
        let tb: *mut buffer = (*r).tmp_buf;
        let mut len: size_t = 0;
        scgi_env_add
            .expect(
                "non-null function pointer",
            )(
            b as *mut libc::c_void,
            b"SCGI\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            b"1\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        buffer_clear(tb);
        buffer_append_int(
            tb,
            (buffer_clen(b)).wrapping_sub(10 as libc::c_int as libc::c_uint) as intmax_t,
        );
        buffer_append_string_len(
            tb,
            b":\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        len = buffer_clen(tb) as size_t;
        offset = (10 as libc::c_int as libc::c_ulong).wrapping_sub(len);
        memcpy(
            ((*b).ptr).offset(offset as isize) as *mut libc::c_void,
            (*tb).ptr as *const libc::c_void,
            len,
        );
        buffer_append_string_len(
            b,
            b",\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    } else {
        let mut len_0: size_t = (buffer_clen(b))
            .wrapping_sub(10 as libc::c_int as libc::c_uint) as size_t;
        if len_0
            > (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                as libc::c_ulong
        {
            (*r).http_status = 431 as libc::c_int;
            (*r).handler_module = 0 as *const plugin;
            buffer_clear(b);
            chunkqueue_remove_finished_chunks(&mut (*hctx).wb);
            return HANDLER_FINISHED;
        }
        offset = (10 as libc::c_int - 4 as libc::c_int) as size_t;
        *((*b).ptr).offset(offset as isize) = 0 as libc::c_int as libc::c_char;
        *((*b).ptr)
            .offset(
                offset.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = (len_0 & 0xff as libc::c_int as libc::c_ulong) as libc::c_char;
        *((*b).ptr)
            .offset(
                offset.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
            ) = (len_0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_char;
        *((*b).ptr)
            .offset(
                offset.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
            ) = 0 as libc::c_int as libc::c_char;
    }
    (*hctx).wb_reqlen = (buffer_clen(b) as libc::c_ulong).wrapping_sub(offset) as off_t;
    chunkqueue_prepend_buffer_commit(&mut (*hctx).wb);
    chunkqueue_mark_written(&mut (*hctx).wb, offset as off_t);
    (*hctx).wb.bytes_in -= offset as off_t;
    (*hctx).wb.bytes_out -= offset as off_t;
    if (*r).reqbody_length != 0 {
        chunkqueue_append_chunkqueue(&mut (*hctx).wb, &mut (*r).reqbody_queue);
        if (*r).reqbody_length > 0 as libc::c_int as libc::c_long {
            (*hctx).wb_reqlen += (*r).reqbody_length;
        } else {
            (*hctx).wb_reqlen = -(*hctx).wb_reqlen;
        }
    }
    status_counter_inc(
        b"scgi.requests\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    return HANDLER_GO_ON;
}
unsafe extern "C" fn scgi_check_extension(
    r: *mut request_st,
    mut p_d: *mut libc::c_void,
    mut uri_path_handler: libc::c_int,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    let mut rc: handler_t = HANDLER_GO_ON;
    if !((*r).handler_module).is_null() {
        return HANDLER_GO_ON;
    }
    mod_scgi_patch_config(r, p);
    if ((*p).conf.exts).is_null() {
        return HANDLER_GO_ON;
    }
    rc = gw_check_extension(r, p, uri_path_handler, 0 as libc::c_int as size_t);
    if HANDLER_GO_ON as libc::c_int as libc::c_uint != rc as libc::c_uint {
        return rc;
    }
    if (*r).handler_module == (*p).self_0 as *const plugin {
        let mut hctx: *mut handler_ctx = *((*r).plugin_ctx).offset((*p).id as isize)
            as *mut handler_ctx;
        (*hctx).opts.backend = BACKEND_SCGI as libc::c_int;
        (*hctx)
            .create_env = Some(
            scgi_create_env as unsafe extern "C" fn(*mut handler_ctx) -> handler_t,
        );
        (*hctx).response = chunk_buffer_acquire();
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn scgi_check_extension_1(
    r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    return scgi_check_extension(r, p_d, 1 as libc::c_int);
}
unsafe extern "C" fn scgi_check_extension_2(
    r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    return scgi_check_extension(r, p_d, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn mod_scgi_plugin_init(mut p: *mut plugin) -> libc::c_int {
    (*p).version = 0x10440 as libc::c_int as size_t;
    (*p).name = b"scgi\0" as *const u8 as *const libc::c_char;
    (*p)
        .init = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
    >(Some(gw_init as unsafe extern "C" fn() -> *mut libc::c_void));
    (*p).cleanup = Some(gw_free as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*p)
        .set_defaults = Some(
        mod_scgi_set_defaults
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_request_reset = Some(
        gw_handle_request_reset
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_uri_clean = Some(
        scgi_check_extension_1
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_subrequest_start = Some(
        scgi_check_extension_2
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_subrequest = Some(
        gw_handle_subrequest
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_trigger = Some(
        gw_handle_trigger
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_waitpid = Some(
        gw_handle_waitpid_cb
            as unsafe extern "C" fn(
                *mut server,
                *mut libc::c_void,
                pid_t,
                libc::c_int,
            ) -> handler_t,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    cpk = [
        {
            let mut init = config_plugin_keys_t {
                k: b"scgi.server\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVARRAY as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"scgi.balance\0" as *const u8 as *const libc::c_char,
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
                k: b"scgi.debug\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_INT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"scgi.map-extensions\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVSTRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"scgi.protocol\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
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
