use ::libc;
extern "C" {
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
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
    fn buffer_append_int(b: *mut buffer, val: intmax_t);
    fn li_itostrn(buf: *mut libc::c_char, buf_len: size_t, val: intmax_t) -> size_t;
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
    fn gw_exts_clear_check_local(exts: *mut gw_exts);
    fn gw_free(p_d: *mut libc::c_void);
    fn gw_plugin_config_free(s: *mut gw_plugin_config);
    fn gw_init() -> *mut libc::c_void;
    fn chunk_buffer_acquire() -> *mut buffer;
    fn chunkqueue_init(cq: *mut chunkqueue) -> *mut chunkqueue;
    fn http_response_transfer_cqlen(
        r: *mut request_st,
        cq: *mut chunkqueue,
        len: size_t,
    ) -> libc::c_int;
    fn http_response_parse_headers(
        r: *mut request_st,
        opts: *mut http_response_opts,
        hdrs: *mut buffer,
    ) -> handler_t;
    fn config_plugin_values_init(
        srv: *mut server,
        p_d: *mut libc::c_void,
        cpk: *const config_plugin_keys_t,
        mname: *const libc::c_char,
    ) -> libc::c_int;
    fn config_check_cond(r: *mut request_st, context_ndx: libc::c_int) -> libc::c_int;
    static mut plugin_stats: array;
    fn chunkqueue_prepend_buffer_commit(cq: *mut chunkqueue);
    fn chunkqueue_prepend_buffer_open_sz(cq: *mut chunkqueue, sz: size_t) -> *mut buffer;
    fn chunkqueue_append_mem(cq: *mut chunkqueue, mem: *const libc::c_char, len: size_t);
    fn chunkqueue_append_mem_min(
        cq: *mut chunkqueue,
        mem: *const libc::c_char,
        len: size_t,
    );
    fn chunkqueue_append_buffer(cq: *mut chunkqueue, mem: *mut buffer);
    fn chunkqueue_peek_data(
        cq: *mut chunkqueue,
        data: *mut *mut libc::c_char,
        dlen: *mut uint32_t,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn chunkqueue_compact_mem(cq: *mut chunkqueue, clen: size_t);
    fn chunkqueue_mark_written(cq: *mut chunkqueue, len: off_t);
    fn chunkqueue_remove_finished_chunks(cq: *mut chunkqueue);
    fn chunkqueue_steal(dest: *mut chunkqueue, src: *mut chunkqueue, len: off_t);
    fn chunkqueue_reset(cq: *mut chunkqueue);
    fn get_http_version_name(i: libc::c_int) -> *const libc::c_char;
    fn sock_addr_get_port(saddr: *const sock_addr) -> libc::c_ushort;
    fn http_header_env_get(
        r: *const request_st,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
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
pub struct data_string {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
    pub ext: libc::c_int,
    pub value: buffer,
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
pub struct C2RustUnnamed_8 {
    pub h: *const libc::c_char,
    pub len: uint32_t,
}
pub type C2RustUnnamed_9 = libc::c_uint;
pub const AJP13_CPING: C2RustUnnamed_9 = 10;
pub const AJP13_CPONG_REPLY: C2RustUnnamed_9 = 9;
pub const AJP13_PING: C2RustUnnamed_9 = 8;
pub const AJP13_SHUTDOWN: C2RustUnnamed_9 = 7;
pub const AJP13_GET_BODY_CHUNK: C2RustUnnamed_9 = 6;
pub const AJP13_END_RESPONSE: C2RustUnnamed_9 = 5;
pub const AJP13_SEND_HEADERS: C2RustUnnamed_9 = 4;
pub const AJP13_SEND_BODY_CHUNK: C2RustUnnamed_9 = 3;
pub const AJP13_FORWARD_REQUEST: C2RustUnnamed_9 = 2;
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
unsafe extern "C" fn buffer_copy_buffer(mut b: *mut buffer, mut src: *const buffer) {
    buffer_copy_string_len(b, (*src).ptr, buffer_clen(src) as size_t);
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
unsafe extern "C" fn status_counter_inc(mut s: *const libc::c_char, mut len: size_t) {
    let ref mut fresh0 = *array_get_int_ptr(&mut plugin_stats, s, len as uint32_t);
    *fresh0 += 1;
}
unsafe extern "C" fn mod_ajp13_merge_config_cpv(
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
        _ => return,
    };
}
unsafe extern "C" fn mod_ajp13_merge_config(
    pconf: *mut plugin_config,
    mut cpv: *const config_plugin_value_t,
) {
    loop {
        mod_ajp13_merge_config_cpv(pconf, cpv);
        cpv = cpv.offset(1);
        if !((*cpv).k_id != -(1 as libc::c_int)) {
            break;
        }
    };
}
unsafe extern "C" fn mod_ajp13_patch_config(r: *mut request_st, p: *mut plugin_data) {
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
            mod_ajp13_merge_config(
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
unsafe extern "C" fn mod_ajp13_set_defaults(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    if config_plugin_values_init(
        srv,
        p as *mut libc::c_void,
        cpk.as_ptr(),
        b"mod_ajp13\0" as *const u8 as *const libc::c_char,
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
        let mut gw: *mut gw_plugin_config = 0 as *mut gw_plugin_config;
        while -(1 as libc::c_int) != (*cpv).k_id {
            match (*cpv).k_id {
                0 => {
                    gw = calloc(
                        1 as libc::c_int as libc::c_ulong,
                        ::std::mem::size_of::<gw_plugin_config>() as libc::c_ulong,
                    ) as *mut gw_plugin_config;
                    if gw.is_null() {
                        ck_assert_failed(
                            b"src/mod_ajp13.c\0" as *const u8 as *const libc::c_char,
                            118 as libc::c_int as libc::c_uint,
                            b"gw\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if gw_set_defaults_backend(
                        srv,
                        p,
                        (*cpv).v.a,
                        gw,
                        0 as libc::c_int,
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
                _ => {}
            }
            cpv = cpv.offset(1);
        }
        if !gw.is_null() && !((*gw).exts).is_null() {
            gw_exts_clear_check_local((*gw).exts);
        }
        i += 1;
    }
    if (*p).nconfig > 0 as libc::c_int
        && (*(*p).cvlist).v.u2[1 as libc::c_int as usize] != 0
    {
        let mut cpv_0: *const config_plugin_value_t = ((*p).cvlist)
            .offset((*(*p).cvlist).v.u2[0 as libc::c_int as usize] as isize);
        if -(1 as libc::c_int) != (*cpv_0).k_id {
            mod_ajp13_merge_config(&mut (*p).defaults, cpv_0);
        }
    }
    return HANDLER_GO_ON;
}
#[inline]
unsafe extern "C" fn ajp13_dec_uint16(x: *const uint8_t) -> uint32_t {
    return ((*x.offset(0 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
        | *x.offset(1 as libc::c_int as isize) as libc::c_int) as uint32_t;
}
#[inline]
unsafe extern "C" fn ajp13_enc_uint16_nc(x: *mut uint8_t, v: uint32_t) {
    *x
        .offset(
            0 as libc::c_int as isize,
        ) = (0xff as libc::c_int as libc::c_uint & v >> 8 as libc::c_int) as uint8_t;
    *x
        .offset(
            1 as libc::c_int as isize,
        ) = (0xff as libc::c_int as libc::c_uint & v) as uint8_t;
}
unsafe extern "C" fn ajp13_enc_uint16(
    x: *mut uint8_t,
    n: uint32_t,
    v: uint32_t,
) -> uint32_t {
    if n.wrapping_add(2 as libc::c_int as libc::c_uint)
        > 8192 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int as uint32_t;
    }
    ajp13_enc_uint16_nc(x.offset(n as isize), v);
    return n.wrapping_add(2 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn ajp13_enc_byte(
    x: *mut uint8_t,
    n: uint32_t,
    v: uint32_t,
) -> uint32_t {
    if n.wrapping_add(1 as libc::c_int as libc::c_uint)
        > 8192 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int as uint32_t;
    }
    *x.offset(n as isize) = v as uint8_t;
    return n.wrapping_add(1 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn ajp13_enc_string(
    x: *mut uint8_t,
    mut n: uint32_t,
    s: *const libc::c_char,
    len: uint32_t,
) -> uint32_t {
    if 0 as libc::c_int as libc::c_uint == len
        || len == 65535 as libc::c_int as libc::c_uint
    {
        return ajp13_enc_uint16(x, n, 0xffff as libc::c_int as uint32_t);
    }
    if n
        .wrapping_add(2 as libc::c_int as libc::c_uint)
        .wrapping_add(len)
        .wrapping_add(1 as libc::c_int as libc::c_uint)
        > 8192 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int as uint32_t;
    }
    ajp13_enc_uint16_nc(x.offset(n as isize), len);
    n = (n as libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint) as uint32_t
        as uint32_t;
    memcpy(
        x.offset(n as isize) as *mut libc::c_void,
        s as *const libc::c_void,
        len as libc::c_ulong,
    );
    n = (n as libc::c_uint).wrapping_add(len) as uint32_t as uint32_t;
    *x.offset(n as isize) = '\u{0}' as i32 as uint8_t;
    return n.wrapping_add(1 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn ajp13_stdin_append(hctx: *mut handler_ctx) -> handler_t {
    let req_cq: *mut chunkqueue = &mut (*(*hctx).r).reqbody_queue;
    let req_cqlen: off_t = chunkqueue_length(req_cq);
    let max_bytes: off_t = if ((*hctx).request_id as libc::c_long) < req_cqlen {
        (if (*hctx).request_id < 256 as libc::c_int * 1024 as libc::c_int {
            (*hctx).request_id
        } else {
            256 as libc::c_int * 1024 as libc::c_int
        }) as libc::c_long
    } else {
        req_cqlen
    };
    let mut sent: off_t = 0 as libc::c_int as off_t;
    let mut hdr: [uint8_t; 4] = [
        0x12 as libc::c_int as uint8_t,
        0x34 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    ];
    let mut dlen: off_t = 0;
    while sent < max_bytes {
        dlen = if max_bytes - sent
            > (8192 as libc::c_int - 4 as libc::c_int) as libc::c_long
        {
            (8192 as libc::c_int - 4 as libc::c_int) as libc::c_long
        } else {
            max_bytes - sent
        };
        if -(1 as libc::c_int) as libc::c_long != (*hctx).wb_reqlen {
            if (*hctx).wb_reqlen >= 0 as libc::c_int as libc::c_long {
                (*hctx)
                    .wb_reqlen = ((*hctx).wb_reqlen as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong)
                    as off_t as off_t;
            } else {
                (*hctx)
                    .wb_reqlen = ((*hctx).wb_reqlen as libc::c_ulong)
                    .wrapping_sub(::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong)
                    as off_t as off_t;
            }
        }
        ajp13_enc_uint16_nc(
            hdr.as_mut_ptr().offset(2 as libc::c_int as isize),
            dlen as uint32_t,
        );
        if chunkqueue_is_empty(&mut (*hctx).wb) != 0
            || (*(*hctx).wb.first).type_0 as libc::c_uint
                == MEM_CHUNK as libc::c_int as libc::c_uint
        {
            chunkqueue_append_mem(
                &mut (*hctx).wb,
                &mut hdr as *mut [uint8_t; 4] as *mut libc::c_char,
                ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong,
            );
        } else {
            chunkqueue_append_mem_min(
                &mut (*hctx).wb,
                &mut hdr as *mut [uint8_t; 4] as *mut libc::c_char,
                ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong,
            );
        };
        chunkqueue_steal(&mut (*hctx).wb, req_cq, dlen);
        sent += dlen;
    }
    (*hctx).request_id -= sent as libc::c_int;
    return HANDLER_GO_ON;
}
unsafe extern "C" fn ajp13_stdin_append_n(hctx: *mut handler_ctx, n: uint32_t) {
    if (*hctx).wb.bytes_in == (*hctx).wb_reqlen {
        let mut hdr: [uint8_t; 4] = [
            0x12 as libc::c_int as uint8_t,
            0x34 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
        ];
        (*hctx)
            .wb_reqlen = ((*hctx).wb_reqlen as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong)
            as off_t as off_t;
        chunkqueue_append_mem(
            &mut (*hctx).wb,
            hdr.as_mut_ptr() as *mut libc::c_char,
            ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong,
        );
    }
    if n <= (2147483647 as libc::c_int - (*hctx).request_id) as uint32_t {
        (*hctx).request_id += n as libc::c_int;
    } else {
        (*hctx).request_id = 2147483647 as libc::c_int;
    }
    ajp13_stdin_append(hctx);
}
unsafe extern "C" fn ajp13_method_byte(m: http_method_t) -> uint8_t {
    static mut ajp13_methods: [uint8_t; 38] = [
        2 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        0,
        1 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        26 as libc::c_int as uint8_t,
        0,
        18 as libc::c_int as uint8_t,
        19 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        24 as libc::c_int as uint8_t,
        0,
        13 as libc::c_int as uint8_t,
        25 as libc::c_int as uint8_t,
        27 as libc::c_int as uint8_t,
        0,
        10 as libc::c_int as uint8_t,
        0,
        22 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        0,
        0,
        8 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        0,
        16 as libc::c_int as uint8_t,
        21 as libc::c_int as uint8_t,
        0,
        20 as libc::c_int as uint8_t,
        0,
        14 as libc::c_int as uint8_t,
        23 as libc::c_int as uint8_t,
        0,
        17 as libc::c_int as uint8_t,
    ];
    return (if m as libc::c_int >= 0 as libc::c_int
        && (m as libc::c_int)
            < ::std::mem::size_of::<[uint8_t; 38]>() as libc::c_ulong as http_method_t
                as libc::c_int
    {
        ajp13_methods[m as usize] as libc::c_int
    } else {
        0 as libc::c_int
    }) as uint8_t;
}
unsafe extern "C" fn ajp13_enc_request_headers(
    x: *mut uint8_t,
    mut n: uint32_t,
    r: *const request_st,
) -> uint32_t {
    let rqst_headers: *const array = &(*r).rqst_headers;
    let add_content_length: libc::c_int = ((*r).rqst_htags
        & (1 as libc::c_ulong) << HTTP_HEADER_CONTENT_LENGTH as libc::c_int == 0)
        as libc::c_int;
    n = ajp13_enc_uint16(
        x,
        n,
        ((*rqst_headers).used).wrapping_add(add_content_length as libc::c_uint),
    );
    if 0 as libc::c_int as libc::c_uint == n {
        return n;
    }
    if add_content_length != 0 {
        n = ajp13_enc_uint16(x, n, 0xa008 as libc::c_int as uint32_t);
        if 0 as libc::c_int as libc::c_uint == n {
            return n;
        }
        let mut buf: [libc::c_char; 22] = [0; 22];
        n = ajp13_enc_string(
            x,
            n,
            buf.as_mut_ptr(),
            li_itostrn(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong,
                (*r).reqbody_length,
            ) as uint32_t,
        );
        if 0 as libc::c_int as libc::c_uint == n {
            return n;
        }
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let mut num: uint32_t = (*rqst_headers).used;
    while i < num {
        let ds: *const data_string = *((*rqst_headers).data).offset(i as isize)
            as *mut data_string;
        let mut code: uint8_t = 0 as libc::c_int as uint8_t;
        match (*ds).ext {
            1 => {
                code = 0x1 as libc::c_int as uint8_t;
            }
            2 => {
                code = 0x3 as libc::c_int as uint8_t;
            }
            3 => {
                code = 0x4 as libc::c_int as uint8_t;
            }
            10 => {
                code = 0x5 as libc::c_int as uint8_t;
            }
            12 => {
                code = 0x6 as libc::c_int as uint8_t;
            }
            18 => {
                code = 0x7 as libc::c_int as uint8_t;
            }
            14 => {
                code = 0x8 as libc::c_int as uint8_t;
            }
            19 => {
                code = 0x9 as libc::c_int as uint8_t;
            }
            27 => {
                code = 0xb as libc::c_int as uint8_t;
            }
            39 => {
                code = 0xc as libc::c_int as uint8_t;
            }
            41 => {
                code = 0xd as libc::c_int as uint8_t;
            }
            51 => {
                code = 0xe as libc::c_int as uint8_t;
            }
            0 => {
                if buffer_eq_icase_slen(
                    &(*ds).key,
                    b"Accept-Charset\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                ) != 0
                {
                    code = 0x2 as libc::c_int as uint8_t;
                } else if buffer_eq_icase_slen(
                        &(*ds).key,
                        b"Cookie2\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    ) != 0
                    {
                    code = 0xa as libc::c_int as uint8_t;
                }
            }
            _ => {}
        }
        n = if code as libc::c_int != 0 {
            ajp13_enc_uint16(
                x,
                n,
                (0xa000 as libc::c_int | code as libc::c_int) as uint32_t,
            )
        } else {
            ajp13_enc_string(x, n, (*ds).key.ptr, buffer_clen(&(*ds).key))
        };
        if 0 as libc::c_int as libc::c_uint == n {
            return n;
        }
        n = ajp13_enc_string(x, n, (*ds).value.ptr, buffer_clen(&(*ds).value));
        if 0 as libc::c_int as libc::c_uint == n {
            return n;
        }
        i = i.wrapping_add(1);
    }
    return n;
}
unsafe extern "C" fn ajp13_enc_attribute(
    x: *mut uint8_t,
    mut n: uint32_t,
    b: *const buffer,
    mut code: uint8_t,
) -> uint32_t {
    if b.is_null() {
        return n;
    }
    n = ajp13_enc_byte(x, n, code as uint32_t);
    if 0 as libc::c_int as libc::c_uint == n {
        return n;
    }
    return ajp13_enc_string(x, n, (*b).ptr, buffer_clen(b));
}
unsafe extern "C" fn ajp13_enc_attributes(
    x: *mut uint8_t,
    mut n: uint32_t,
    r: *mut request_st,
) -> uint32_t {
    let mut vb: *const buffer = 0 as *const buffer;
    vb = http_header_env_get(
        r,
        b"REMOTE_USER\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    n = ajp13_enc_attribute(x, n, vb, 0x3 as libc::c_int as uint8_t);
    if 0 as libc::c_int as libc::c_uint == n {
        return n;
    }
    vb = http_header_env_get(
        r,
        b"AUTH_TYPE\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    n = ajp13_enc_attribute(x, n, vb, 0x4 as libc::c_int as uint8_t);
    if 0 as libc::c_int as libc::c_uint == n {
        return n;
    }
    if buffer_is_blank(&mut (*r).uri.query) == 0 {
        n = ajp13_enc_attribute(
            x,
            n,
            &mut (*r).uri.query,
            0x5 as libc::c_int as uint8_t,
        );
        if 0 as libc::c_int as libc::c_uint == n {
            return n;
        }
    }
    if buffer_eq_slen(
        &mut (*r).uri.scheme,
        b"https\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    ) != 0
    {
        ((*(*(*r).con).srv).request_env).expect("non-null function pointer")(r);
        vb = http_header_env_get(
            r,
            b"SSL_CLIENT_CERT\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        n = ajp13_enc_attribute(x, n, vb, 0x7 as libc::c_int as uint8_t);
        if 0 as libc::c_int as libc::c_uint == n {
            return n;
        }
        vb = http_header_env_get(
            r,
            b"SSL_CIPHER\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        n = ajp13_enc_attribute(x, n, vb, 0x8 as libc::c_int as uint8_t);
        if 0 as libc::c_int as libc::c_uint == n {
            return n;
        }
        vb = http_header_env_get(
            r,
            b"SSL_CIPHER_USE_KEYSIZE\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        n = ajp13_enc_attribute(x, n, vb, 0xb as libc::c_int as uint8_t);
        if 0 as libc::c_int as libc::c_uint == n {
            return n;
        }
    }
    vb = http_header_env_get(
        r,
        b"AJP13_SECRET\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    n = ajp13_enc_attribute(x, n, vb, 0xc as libc::c_int as uint8_t);
    if 0 as libc::c_int as libc::c_uint == n {
        return n;
    }
    return n;
}
unsafe extern "C" fn ajp13_enc_server_name(
    x: *mut uint8_t,
    n: uint32_t,
    r: *const request_st,
) -> uint32_t {
    let mut len: uint32_t = buffer_clen((*r).server_name);
    if len != 0 {
        let ptr: *const libc::c_char = (*(*r).server_name).ptr;
        if *ptr.offset(0 as libc::c_int as isize) as libc::c_int == '[' as i32 {
            let mut colon: *const libc::c_char = strstr(
                ptr,
                b"]:\0" as *const u8 as *const libc::c_char,
            );
            if !colon.is_null() {
                len = colon.offset(1 as libc::c_int as isize).offset_from(ptr)
                    as libc::c_long as uint32_t;
            }
        } else {
            let mut colon_0: *const libc::c_char = strchr(ptr, ':' as i32);
            if !colon_0.is_null() {
                len = colon_0.offset_from(ptr) as libc::c_long as uint32_t;
            }
        }
        return ajp13_enc_string(x, n, ptr, len);
    } else {
        return ajp13_enc_string(
            x,
            n,
            0 as *const libc::c_char,
            0 as libc::c_int as uint32_t,
        )
    };
}
unsafe extern "C" fn ajp13_create_env(hctx: *mut handler_ctx) -> handler_t {
    let r: *mut request_st = (*hctx).r;
    let b: *mut buffer = chunkqueue_prepend_buffer_open_sz(
        &mut (*hctx).wb,
        8192 as libc::c_int as size_t,
    );
    let mut n: uint32_t = 6 as libc::c_int as uint32_t;
    let x: *mut uint8_t = (*b).ptr as *mut uint8_t;
    *x.offset(0 as libc::c_int as isize) = 0x12 as libc::c_int as uint8_t;
    *x.offset(1 as libc::c_int as isize) = 0x34 as libc::c_int as uint8_t;
    *x.offset(2 as libc::c_int as isize) = 0 as libc::c_int as uint8_t;
    *x.offset(3 as libc::c_int as isize) = 0 as libc::c_int as uint8_t;
    *x.offset(4 as libc::c_int as isize) = 0x2 as libc::c_int as uint8_t;
    let method_byte: uint8_t = ajp13_method_byte((*r).http_method);
    if !(0 as libc::c_int == method_byte as libc::c_int) {
        *x.offset(5 as libc::c_int as isize) = method_byte;
        let proto: *const libc::c_char = get_http_version_name(
            (*r).http_version as libc::c_int,
        );
        n = ajp13_enc_string(x, n, proto, strlen(proto) as uint32_t);
        if !(0 as libc::c_int as libc::c_uint == n) {
            n = ajp13_enc_string(
                x,
                n,
                (*r).uri.path.ptr,
                buffer_clen(&mut (*r).uri.path),
            );
            if !(0 as libc::c_int as libc::c_uint == n) {
                n = ajp13_enc_string(
                    x,
                    n,
                    (*(*r).con).dst_addr_buf.ptr,
                    buffer_clen(&mut (*(*r).con).dst_addr_buf),
                );
                if !(0 as libc::c_int as libc::c_uint == n) {
                    n = ajp13_enc_string(
                        x,
                        n,
                        0 as *const libc::c_char,
                        0 as libc::c_int as uint32_t,
                    );
                    if !(0 as libc::c_int as libc::c_uint == n) {
                        n = ajp13_enc_server_name(x, n, r);
                        if !(0 as libc::c_int as libc::c_uint == n) {
                            let mut port: libc::c_ushort = sock_addr_get_port(
                                &(*(*(*r).con).srv_socket).addr,
                            );
                            n = ajp13_enc_uint16(x, n, port as uint32_t);
                            if !(0 as libc::c_int as libc::c_uint == n) {
                                n = ajp13_enc_byte(
                                    x,
                                    n,
                                    buffer_eq_slen(
                                        &mut (*r).uri.scheme,
                                        b"https\0" as *const u8 as *const libc::c_char,
                                        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                                            as uint32_t)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                    ) as uint32_t,
                                );
                                if !(0 as libc::c_int as libc::c_uint == n) {
                                    n = ajp13_enc_request_headers(x, n, r);
                                    if !(0 as libc::c_int as libc::c_uint == n) {
                                        n = ajp13_enc_attributes(x, n, r);
                                        if !(0 as libc::c_int as libc::c_uint == n) {
                                            n = ajp13_enc_byte(x, n, 0xff as libc::c_int as uint32_t);
                                            if !(0 as libc::c_int as libc::c_uint == n) {
                                                ajp13_enc_uint16_nc(
                                                    x.offset(2 as libc::c_int as isize),
                                                    n.wrapping_sub(4 as libc::c_int as libc::c_uint),
                                                );
                                                buffer_extend(b, n as size_t);
                                                chunkqueue_prepend_buffer_commit(&mut (*hctx).wb);
                                                (*hctx).wb_reqlen = n as off_t;
                                                if (*r).reqbody_length != 0 {
                                                    if (*r).reqbody_length > 0 as libc::c_int as libc::c_long {
                                                        (*hctx).wb_reqlen += (*r).reqbody_length;
                                                    } else {
                                                        (*hctx).wb_reqlen = -(*hctx).wb_reqlen;
                                                    }
                                                }
                                                ajp13_stdin_append_n(
                                                    hctx,
                                                    (8192 as libc::c_int - 4 as libc::c_int) as uint32_t,
                                                );
                                                (*hctx).request_id = 0 as libc::c_int;
                                                status_counter_inc(
                                                    b"ajp13.requests\0" as *const u8 as *const libc::c_char,
                                                    (::std::mem::size_of::<[libc::c_char; 15]>()
                                                        as libc::c_ulong as uint32_t)
                                                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                                );
                                                return HANDLER_GO_ON;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    (*r).http_status = 400 as libc::c_int;
    (*r).handler_module = 0 as *const plugin;
    buffer_clear(b);
    chunkqueue_remove_finished_chunks(&mut (*hctx).wb);
    return HANDLER_FINISHED;
}
static mut hcode: [C2RustUnnamed_8; 11] = [C2RustUnnamed_8 {
    h: 0 as *const libc::c_char,
    len: 0,
}; 11];
unsafe extern "C" fn ajp13_expand_headers(
    b: *mut buffer,
    hctx: *mut handler_ctx,
    mut plen: uint32_t,
) {
    chunkqueue_compact_mem((*hctx).rb, plen as size_t);
    let c: *mut chunk = (*(*hctx).rb).first;
    let mut ptr: *mut uint8_t = ((*(*c).mem).ptr as *mut uint8_t)
        .offset((*c).offset as isize)
        .offset(5 as libc::c_int as isize);
    plen = (plen as libc::c_uint).wrapping_sub(5 as libc::c_int as libc::c_uint)
        as uint32_t as uint32_t;
    let mut len: uint32_t = 0;
    if !(plen < 2 as libc::c_int as libc::c_uint) {
        plen = (plen as libc::c_uint).wrapping_sub(2 as libc::c_int as libc::c_uint)
            as uint32_t as uint32_t;
        buffer_append_string_len(
            b,
            b"HTTP/1.1 \0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        buffer_append_int(b, ajp13_dec_uint16(ptr) as intmax_t);
        ptr = ptr.offset(2 as libc::c_int as isize);
        if !(plen < 2 as libc::c_int as libc::c_uint) {
            plen = (plen as libc::c_uint).wrapping_sub(2 as libc::c_int as libc::c_uint)
                as uint32_t as uint32_t;
            len = ajp13_dec_uint16(ptr);
            ptr = ptr.offset(2 as libc::c_int as isize);
            if !(plen < len.wrapping_add(1 as libc::c_int as libc::c_uint)) {
                plen = (plen as libc::c_uint)
                    .wrapping_sub(len.wrapping_add(1 as libc::c_int as libc::c_uint))
                    as uint32_t as uint32_t;
                buffer_append_string_len(
                    b,
                    b" \0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int as size_t,
                );
                if len != 0 {
                    buffer_append_string_len(b, ptr as *mut libc::c_char, len as size_t);
                }
                ptr = ptr
                    .offset(len.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
                if !(plen < 2 as libc::c_int as libc::c_uint) {
                    plen = (plen as libc::c_uint)
                        .wrapping_sub(2 as libc::c_int as libc::c_uint) as uint32_t
                        as uint32_t;
                    ptr = ptr.offset(2 as libc::c_int as isize);
                    let mut nhdrs: uint32_t = ajp13_dec_uint16(ptr);
                    while nhdrs != 0 {
                        if plen < 2 as libc::c_int as libc::c_uint {
                            break;
                        }
                        plen = (plen as libc::c_uint)
                            .wrapping_sub(2 as libc::c_int as libc::c_uint) as uint32_t
                            as uint32_t;
                        len = ajp13_dec_uint16(ptr);
                        ptr = ptr.offset(2 as libc::c_int as isize);
                        if len >= 0xa000 as libc::c_int as libc::c_uint {
                            if len == 0xa000 as libc::c_int as libc::c_uint
                                || len > 0xa00b as libc::c_int as libc::c_uint
                            {
                                break;
                            }
                            let idx: uint32_t = (len
                                & 0xf as libc::c_int as libc::c_uint)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint);
                            buffer_append_string_len(
                                b,
                                hcode[idx as usize].h,
                                hcode[idx as usize].len as size_t,
                            );
                        } else {
                            if plen < len.wrapping_add(1 as libc::c_int as libc::c_uint)
                            {
                                break;
                            }
                            plen = (plen as libc::c_uint)
                                .wrapping_sub(
                                    len.wrapping_add(1 as libc::c_int as libc::c_uint),
                                ) as uint32_t as uint32_t;
                            buffer_append_str3(
                                b,
                                b"\n\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                    as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                ptr as *mut libc::c_char,
                                len as size_t,
                                b": \0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                                    as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                            );
                            ptr = ptr
                                .offset(
                                    len.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                                );
                        }
                        if plen < 2 as libc::c_int as libc::c_uint {
                            break;
                        }
                        plen = (plen as libc::c_uint)
                            .wrapping_sub(2 as libc::c_int as libc::c_uint) as uint32_t
                            as uint32_t;
                        len = ajp13_dec_uint16(ptr);
                        ptr = ptr.offset(2 as libc::c_int as isize);
                        if plen < len.wrapping_add(1 as libc::c_int as libc::c_uint) {
                            break;
                        }
                        plen = (plen as libc::c_uint)
                            .wrapping_sub(
                                len.wrapping_add(1 as libc::c_int as libc::c_uint),
                            ) as uint32_t as uint32_t;
                        buffer_append_string_len(
                            b,
                            ptr as *mut libc::c_char,
                            len as size_t,
                        );
                        ptr = ptr
                            .offset(
                                len.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                            );
                        nhdrs = nhdrs.wrapping_sub(1);
                    }
                }
            }
        }
    }
    buffer_append_string_len(
        b,
        b"\n\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
}
#[cold]
unsafe extern "C" fn ajp13_recv_0(
    r: *const request_st,
    hctx: *const handler_ctx,
) -> handler_t {
    if -(1 as libc::c_int) == (*hctx).request_id {
        return HANDLER_FINISHED;
    }
    if (if !((*hctx).fdn).is_null() { (*(*hctx).fdn).events } else { 0 as libc::c_int })
        & 0x1 as libc::c_int == 0
        && (*r).conf.stream_response_body as libc::c_int
            & (1 as libc::c_int) << 15 as libc::c_int == 0
    {
        return HANDLER_GO_ON;
    }
    log_error(
        (*r).conf.errh,
        b"src/mod_ajp13.c\0" as *const u8 as *const libc::c_char,
        777 as libc::c_int as libc::c_uint,
        b"unexpected end-of-file (perhaps the ajp13 process died):pid: %d socket: %s\0"
            as *const u8 as *const libc::c_char,
        (*(*hctx).proc_0).pid,
        (*(*(*hctx).proc_0).connection_name).ptr,
    );
    return HANDLER_ERROR;
}
unsafe extern "C" fn ajp13_recv_parse_loop(
    r: *mut request_st,
    hctx: *mut handler_ctx,
) -> handler_t {
    let errh: *mut log_error_st = (*r).conf.errh;
    let mut fin: libc::c_int = 0 as libc::c_int;
    let mut current_block_58: u64;
    loop {
        let mut header: [uint8_t; 7] = [0; 7];
        let rblen: off_t = chunkqueue_length((*hctx).rb);
        if rblen < 5 as libc::c_int as libc::c_long {
            break;
        }
        let mut ptr: *mut libc::c_char = &mut header as *mut [uint8_t; 7]
            as *mut libc::c_char;
        let mut pklen: uint32_t = 5 as libc::c_int as uint32_t;
        if chunkqueue_peek_data((*hctx).rb, &mut ptr, &mut pklen, errh)
            < 0 as libc::c_int
        {
            break;
        }
        if pklen != 5 as libc::c_int as libc::c_uint {
            break;
        }
        if *ptr.offset(0 as libc::c_int as isize) as libc::c_int != 'A' as i32
            || *ptr.offset(1 as libc::c_int as isize) as libc::c_int != 'B' as i32
        {
            log_error(
                errh,
                b"src/mod_ajp13.c\0" as *const u8 as *const libc::c_char,
                803 as libc::c_int as libc::c_uint,
                b"invalid packet prefix sent from container:pid: %d socket: %s\0"
                    as *const u8 as *const libc::c_char,
                (*(*hctx).proc_0).pid,
                (*(*(*hctx).proc_0).connection_name).ptr,
            );
            return HANDLER_ERROR;
        }
        let mut plen: uint32_t = ajp13_dec_uint16(
            (ptr as *mut uint8_t).offset(2 as libc::c_int as isize),
        );
        if plen > (rblen as libc::c_uint).wrapping_sub(4 as libc::c_int as libc::c_uint)
        {
            break;
        }
        match *ptr.offset(4 as libc::c_int as isize) as libc::c_int {
            4 => {
                if 0 as libc::c_int == (*r).resp_body_started as libc::c_int {
                    let mut hdrs: *mut buffer = (*hctx).response;
                    if hdrs.is_null() {
                        hdrs = (*r).tmp_buf;
                        buffer_clear(hdrs);
                    }
                    ajp13_expand_headers(
                        hdrs,
                        hctx,
                        (4 as libc::c_int as libc::c_uint).wrapping_add(plen),
                    );
                    if HANDLER_GO_ON as libc::c_int as libc::c_uint
                        != http_response_parse_headers(r, &mut (*hctx).opts, hdrs)
                            as libc::c_uint
                    {
                        (*hctx).send_content_body = 0 as libc::c_int;
                        return HANDLER_FINISHED;
                    }
                    if 0 as libc::c_int == (*r).resp_body_started as libc::c_int {
                        if ((*hctx).response).is_null() {
                            (*hctx).response = chunk_buffer_acquire();
                            buffer_copy_buffer((*hctx).response, hdrs);
                        }
                    } else if (*hctx).gw_mode as libc::c_int == 2 as libc::c_int
                            && ((*r).http_status == 0 as libc::c_int
                                || (*r).http_status == 200 as libc::c_int)
                        {
                        (*hctx).send_content_body = 0 as libc::c_int;
                        (*hctx).opts.authorizer
                            |= ((*r).conf.stream_response_body as libc::c_int
                                & ((1 as libc::c_int) << 0 as libc::c_int
                                    | (1 as libc::c_int) << 1 as libc::c_int))
                                << 1 as libc::c_int;
                        (*r)
                            .conf
                            .stream_response_body = ((*r).conf.stream_response_body
                            as libc::c_int
                            & !((1 as libc::c_int) << 0 as libc::c_int
                                | (1 as libc::c_int) << 1 as libc::c_int))
                            as libc::c_ushort;
                    }
                } else {
                    log_error(
                        errh,
                        b"src/mod_ajp13.c\0" as *const u8 as *const libc::c_char,
                        863 as libc::c_int as libc::c_uint,
                        b"AJP13: headers received after body started\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                current_block_58 = 13853033528615664019;
            }
            3 => {
                if 0 as libc::c_int == (*r).resp_body_started as libc::c_int {
                    log_error(
                        errh,
                        b"src/mod_ajp13.c\0" as *const u8 as *const libc::c_char,
                        870 as libc::c_int as libc::c_uint,
                        b"AJP13: body received before headers\0" as *const u8
                            as *const libc::c_char,
                    );
                    return HANDLER_FINISHED;
                } else if (*hctx).send_content_body != 0 {
                    ptr = &mut header as *mut [uint8_t; 7] as *mut libc::c_char;
                    pklen = 7 as libc::c_int as uint32_t;
                    if chunkqueue_peek_data((*hctx).rb, &mut ptr, &mut pklen, errh)
                        < 0 as libc::c_int
                    {
                        return HANDLER_GO_ON;
                    }
                    if pklen != 7 as libc::c_int as libc::c_uint {
                        return HANDLER_GO_ON;
                    }
                    let mut len: uint32_t = ajp13_dec_uint16(
                        (ptr as *mut uint8_t).offset(5 as libc::c_int as isize),
                    );
                    if 0 as libc::c_int as libc::c_uint == len {
                        current_block_58 = 13853033528615664019;
                    } else {
                        if len > plen.wrapping_sub(3 as libc::c_int as libc::c_uint) {
                            log_error(
                                errh,
                                b"src/mod_ajp13.c\0" as *const u8 as *const libc::c_char,
                                884 as libc::c_int as libc::c_uint,
                                b"AJP13: body packet received with invalid length\0"
                                    as *const u8 as *const libc::c_char,
                            );
                            return HANDLER_FINISHED;
                        }
                        chunkqueue_mark_written((*hctx).rb, 7 as libc::c_int as off_t);
                        if 0 as libc::c_int
                            == http_response_transfer_cqlen(r, (*hctx).rb, len as size_t)
                        {
                            if len != plen.wrapping_sub(3 as libc::c_int as libc::c_uint)
                            {
                                chunkqueue_mark_written(
                                    (*hctx).rb,
                                    plen
                                        .wrapping_sub(3 as libc::c_int as libc::c_uint)
                                        .wrapping_sub(len) as off_t,
                                );
                            }
                        } else {
                            (*hctx).send_content_body = 0 as libc::c_int;
                            return HANDLER_FINISHED;
                        }
                        current_block_58 = 12237857397564741460;
                    }
                } else {
                    current_block_58 = 13853033528615664019;
                }
            }
            6 => {
                ptr = &mut header as *mut [uint8_t; 7] as *mut libc::c_char;
                pklen = 7 as libc::c_int as uint32_t;
                if chunkqueue_peek_data((*hctx).rb, &mut ptr, &mut pklen, errh)
                    < 0 as libc::c_int
                {
                    return HANDLER_GO_ON;
                }
                if pklen != 7 as libc::c_int as libc::c_uint {
                    return HANDLER_GO_ON;
                }
                ajp13_stdin_append_n(
                    hctx,
                    ajp13_dec_uint16(
                        (ptr as *mut uint8_t).offset(5 as libc::c_int as isize),
                    ),
                );
                current_block_58 = 13853033528615664019;
            }
            5 => {
                (*hctx).request_id = -(1 as libc::c_int);
                fin = 1 as libc::c_int;
                current_block_58 = 13853033528615664019;
            }
            9 => {
                current_block_58 = 13853033528615664019;
            }
            _ => {
                log_error(
                    errh,
                    b"src/mod_ajp13.c\0" as *const u8 as *const libc::c_char,
                    935 as libc::c_int as libc::c_uint,
                    b"AJP13: packet type not handled: %d\0" as *const u8
                        as *const libc::c_char,
                    *ptr.offset(4 as libc::c_int as isize) as libc::c_int,
                );
                current_block_58 = 13853033528615664019;
            }
        }
        match current_block_58 {
            13853033528615664019 => {
                chunkqueue_mark_written(
                    (*hctx).rb,
                    (4 as libc::c_int as libc::c_uint).wrapping_add(plen) as off_t,
                );
            }
            _ => {}
        }
        if !(0 as libc::c_int == fin) {
            break;
        }
    }
    return (if 0 as libc::c_int == fin {
        HANDLER_GO_ON as libc::c_int
    } else {
        HANDLER_FINISHED as libc::c_int
    }) as handler_t;
}
unsafe extern "C" fn ajp13_recv_parse(
    r: *mut request_st,
    opts: *mut http_response_opts_t,
    b: *mut buffer,
    mut n: size_t,
) -> handler_t {
    let hctx: *mut handler_ctx = (*opts).pdata as *mut handler_ctx;
    if 0 as libc::c_int as libc::c_ulong == n {
        return ajp13_recv_0(r, hctx);
    }
    chunkqueue_append_buffer((*hctx).rb, b);
    return ajp13_recv_parse_loop(r, hctx);
}
unsafe extern "C" fn ajp13_check_extension(
    r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    if !((*r).handler_module).is_null() {
        return HANDLER_GO_ON;
    }
    let p: *mut plugin_data = p_d as *mut plugin_data;
    mod_ajp13_patch_config(r, p);
    if ((*p).conf.exts).is_null() {
        return HANDLER_GO_ON;
    }
    let mut rc: handler_t = gw_check_extension(
        r,
        p,
        1 as libc::c_int,
        0 as libc::c_int as size_t,
    );
    if HANDLER_GO_ON as libc::c_int as libc::c_uint != rc as libc::c_uint {
        return rc;
    }
    if (*r).handler_module == (*p).self_0 as *const plugin {
        let mut hctx: *mut handler_ctx = *((*r).plugin_ctx).offset((*p).id as isize)
            as *mut handler_ctx;
        (*hctx).opts.backend = BACKEND_AJP13 as libc::c_int;
        (*hctx)
            .opts
            .parse = Some(
            ajp13_recv_parse
                as unsafe extern "C" fn(
                    *mut request_st,
                    *mut http_response_opts_t,
                    *mut buffer,
                    size_t,
                ) -> handler_t,
        );
        (*hctx).opts.pdata = hctx as *mut libc::c_void;
        (*hctx)
            .stdin_append = Some(
            ajp13_stdin_append as unsafe extern "C" fn(*mut handler_ctx) -> handler_t,
        );
        (*hctx)
            .create_env = Some(
            ajp13_create_env as unsafe extern "C" fn(*mut handler_ctx) -> handler_t,
        );
        if ((*hctx).rb).is_null() {
            (*hctx).rb = chunkqueue_init(0 as *mut chunkqueue);
        } else {
            chunkqueue_reset((*hctx).rb);
        }
    }
    return HANDLER_GO_ON;
}
#[no_mangle]
pub unsafe extern "C" fn mod_ajp13_plugin_init(mut p: *mut plugin) -> libc::c_int {
    (*p).version = 0x10440 as libc::c_int as size_t;
    (*p).name = b"ajp13\0" as *const u8 as *const libc::c_char;
    (*p)
        .init = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
    >(Some(gw_init as unsafe extern "C" fn() -> *mut libc::c_void));
    (*p).cleanup = Some(gw_free as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*p)
        .set_defaults = Some(
        mod_ajp13_set_defaults
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_request_reset = Some(
        gw_handle_request_reset
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_uri_clean = Some(
        ajp13_check_extension
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
                k: b"ajp13.server\0" as *const u8 as *const libc::c_char,
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
                k: b"ajp13.balance\0" as *const u8 as *const libc::c_char,
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
                k: b"ajp13.debug\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_INT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"ajp13.map-extensions\0" as *const u8 as *const libc::c_char,
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
                k: 0 as *const libc::c_char,
                klen: 0 as libc::c_int as uint8_t,
                ktype: T_CONFIG_UNSET as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_UNSET as libc::c_int as uint8_t,
            };
            init
        },
    ];
    hcode = [
        {
            let mut init = C2RustUnnamed_8 {
                h: b"\nContent-Type: \0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_8 {
                h: b"\nContent-Language: \0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_8 {
                h: b"\nContent-Length: \0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_8 {
                h: b"\nDate: \0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_8 {
                h: b"\nLast-Modified: \0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_8 {
                h: b"\nLocation: \0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_8 {
                h: b"\nSet-Cookie: \0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_8 {
                h: b"\nSet-Cookie2: \0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_8 {
                h: b"\nServlet-Engine: \0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_8 {
                h: b"\nStatus: \0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_8 {
                h: b"\nWWW-Authenticate: \0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
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
