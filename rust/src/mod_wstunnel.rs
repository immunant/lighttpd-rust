use ::libc;
extern "C" {
    pub type stat_cache_entry;
    pub type pcre2_real_match_data_8;
    pub type h2con;
    pub type fdevents;
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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_str2(
        b: *mut buffer,
        s1: *const libc::c_char,
        len1: size_t,
        s2: *const libc::c_char,
        len2: size_t,
    );
    fn buffer_eq_icase_ssn(
        a: *const libc::c_char,
        b: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
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
    fn gw_set_transparent(hctx: *mut gw_handler_ctx);
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
    fn chunk_buffer_acquire() -> *mut buffer;
    fn chunk_buffer_release(b: *mut buffer);
    fn chunkqueue_append_mem(cq: *mut chunkqueue, mem: *const libc::c_char, len: size_t);
    fn chunkqueue_append_buffer(cq: *mut chunkqueue, mem: *mut buffer);
    fn chunkqueue_append_chunkqueue(cq: *mut chunkqueue, src: *mut chunkqueue);
    fn chunkqueue_get_memory(cq: *mut chunkqueue, len: *mut size_t) -> *mut libc::c_char;
    fn chunkqueue_mark_written(cq: *mut chunkqueue, len: off_t);
    fn chunkqueue_read_data(
        cq: *mut chunkqueue,
        data: *mut libc::c_char,
        dlen: uint32_t,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn config_plugin_values_init(
        srv: *mut server,
        p_d: *mut libc::c_void,
        cpk: *const config_plugin_keys_t,
        mname: *const libc::c_char,
    ) -> libc::c_int;
    fn config_check_cond(r: *mut request_st, context_ndx: libc::c_int) -> libc::c_int;
    fn http_response_upgrade_read_body_unknown(r: *mut request_st);
    static mut log_con_jqueue: *mut connection;
    fn http_header_str_contains_token(
        s: *const libc::c_char,
        slen: uint32_t,
        m: *const libc::c_char,
        mlen: uint32_t,
    ) -> libc::c_int;
    fn http_header_response_set_ptr(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn http_header_response_set(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
        v: *const libc::c_char,
        vlen: uint32_t,
    );
    fn http_header_request_get(
        r: *const request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    static mut log_monotonic_secs: unix_time64_t;
    fn log_error(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        fmt: *const libc::c_char,
        _: ...
    );
    fn MD5_Init(_: *mut MD5_CTX);
    fn MD5_Update(_: *mut MD5_CTX, _: *const libc::c_void, _: libc::c_uint);
    fn SHA1_Final(digest: *mut sha1_byte, context: *mut SHA_CTX);
    fn SHA1_Update(context: *mut SHA_CTX, data: *const sha1_byte, len: libc::c_uint);
    fn SHA1_Init(context: *mut SHA_CTX);
    fn MD5_Final(_: *mut libc::c_uchar, _: *mut MD5_CTX);
    fn buffer_append_base64_decode(
        out: *mut buffer,
        in_0: *const libc::c_char,
        in_length: size_t,
        charset: base64_charset,
    ) -> *mut libc::c_uchar;
    fn li_base64_enc(
        out: *mut libc::c_char,
        out_length: size_t,
        in_0: *const libc::c_uchar,
        in_length: size_t,
        charset: base64_charset,
        pad: libc::c_int,
    ) -> size_t;
    fn buffer_append_base64_enc(
        out: *mut buffer,
        in_0: *const libc::c_uchar,
        in_length: size_t,
        charset: base64_charset,
        pad: libc::c_int,
    ) -> *mut libc::c_char;
    fn http_chunk_append_mem(
        r: *mut request_st,
        mem: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
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
pub struct plugin_config {
    pub gw: gw_plugin_config,
    pub origins: *const array,
    pub frame_type: libc::c_uint,
    pub ping_interval: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_data {
    pub id: libc::c_int,
    pub nconfig: libc::c_int,
    pub cvlist: *mut config_plugin_value_t,
    pub self_0: *mut plugin,
    pub srv_pid: pid_t,
    pub conf: plugin_config,
    pub defaults: plugin_config,
}
pub type mod_wstunnel_frame_state_t = libc::c_uint;
pub const MOD_WEBSOCKET_FRAME_STATE_READ_PAYLOAD: mod_wstunnel_frame_state_t = 4;
pub const MOD_WEBSOCKET_FRAME_STATE_READ_MASK: mod_wstunnel_frame_state_t = 3;
pub const MOD_WEBSOCKET_FRAME_STATE_READ_EX_LENGTH: mod_wstunnel_frame_state_t = 2;
pub const MOD_WEBSOCKET_FRAME_STATE_READ_LENGTH: mod_wstunnel_frame_state_t = 1;
pub const MOD_WEBSOCKET_FRAME_STATE_INIT: mod_wstunnel_frame_state_t = 0;
pub type mod_wstunnel_frame_type_t = libc::c_uint;
pub const MOD_WEBSOCKET_FRAME_TYPE_PONG: mod_wstunnel_frame_type_t = 4;
pub const MOD_WEBSOCKET_FRAME_TYPE_PING: mod_wstunnel_frame_type_t = 3;
pub const MOD_WEBSOCKET_FRAME_TYPE_CLOSE: mod_wstunnel_frame_type_t = 2;
pub const MOD_WEBSOCKET_FRAME_TYPE_BIN: mod_wstunnel_frame_type_t = 1;
pub const MOD_WEBSOCKET_FRAME_TYPE_TEXT: mod_wstunnel_frame_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mod_wstunnel_frame_control_t {
    pub siz: uint64_t,
    pub siz_cnt: libc::c_int,
    pub mask_cnt: libc::c_int,
    pub mask: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mod_wstunnel_frame_t {
    pub state: mod_wstunnel_frame_state_t,
    pub ctl: mod_wstunnel_frame_control_t,
    pub type_0: mod_wstunnel_frame_type_t,
    pub type_before: mod_wstunnel_frame_type_t,
    pub type_backend: mod_wstunnel_frame_type_t,
    pub payload: *mut buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct handler_ctx {
    pub gw: gw_handler_ctx,
    pub frame: mod_wstunnel_frame_t,
    pub hybivers: libc::c_int,
    pub ping_ts: unix_time64_t,
    pub subproto: libc::c_int,
    pub errh: *mut log_error_st,
    pub conf: plugin_config,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5_CTX {
    pub state: [uint32_t; 4],
    pub count: [uint32_t; 2],
    pub buffer: [libc::c_uchar; 64],
}
pub type base64_charset = libc::c_uint;
pub const BASE64_URL: base64_charset = 1;
pub const BASE64_STANDARD: base64_charset = 0;
pub type SHA_CTX = _SHA_CTX;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SHA_CTX {
    pub state: [sha1_quadbyte; 5],
    pub count: [sha1_quadbyte; 2],
    pub buffer: [sha1_byte; 64],
}
pub type sha1_byte = libc::c_uchar;
pub type sha1_quadbyte = uint32_t;
#[inline]
unsafe extern "C" fn buffer_clear(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn light_isdigit(mut c: libc::c_int) -> libc::c_int {
    return ((c as uint32_t).wrapping_sub('0' as i32 as libc::c_uint)
        <= ('9' as i32 - '0' as i32) as libc::c_uint) as libc::c_int;
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
unsafe extern "C" fn chunkqueue_length(mut cq: *const chunkqueue) -> off_t {
    return (*cq).bytes_in - (*cq).bytes_out;
}
#[inline]
unsafe extern "C" fn connection_jq_append(con: *mut connection) {
    if ((*con).jqnext).is_null() {
        (*con).jqnext = log_con_jqueue;
        log_con_jqueue = con;
    }
}
#[cold]
unsafe extern "C" fn mod_wstunnel_init() -> *mut libc::c_void {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<plugin_data>() as libc::c_ulong,
    );
}
unsafe extern "C" fn mod_wstunnel_merge_config_cpv(
    pconf: *mut plugin_config,
    cpv: *const config_plugin_value_t,
) {
    match (*cpv).k_id {
        0 => {
            if (*cpv).vtype as libc::c_uint
                == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
            {
                let gw: *mut gw_plugin_config = (*cpv).v.v as *mut gw_plugin_config;
                (*pconf).gw.exts = (*gw).exts;
                (*pconf).gw.exts_auth = (*gw).exts_auth;
                (*pconf).gw.exts_resp = (*gw).exts_resp;
            }
        }
        1 => {
            (*pconf).gw.balance = (*cpv).v.u as libc::c_int;
        }
        2 => {
            (*pconf).gw.debug = (*cpv).v.u as libc::c_int;
        }
        3 => {
            (*pconf).gw.ext_mapping = (*cpv).v.a;
        }
        4 => {
            (*pconf).frame_type = (*cpv).v.u;
        }
        5 => {
            (*pconf).origins = (*cpv).v.a;
        }
        6 => {
            (*pconf).ping_interval = (*cpv).v.shrt;
        }
        _ => return,
    };
}
unsafe extern "C" fn mod_wstunnel_merge_config(
    pconf: *mut plugin_config,
    mut cpv: *const config_plugin_value_t,
) {
    loop {
        mod_wstunnel_merge_config_cpv(pconf, cpv);
        cpv = cpv.offset(1);
        if !((*cpv).k_id != -(1 as libc::c_int)) {
            break;
        }
    };
}
unsafe extern "C" fn mod_wstunnel_patch_config(r: *mut request_st, p: *mut plugin_data) {
    memcpy(
        &mut (*p).conf as *mut plugin_config as *mut libc::c_void,
        &mut (*p).defaults as *mut plugin_config as *const libc::c_void,
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
            mod_wstunnel_merge_config(
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
static mut cpk: [config_plugin_keys_t; 8] = [config_plugin_keys_t {
    k: 0 as *const libc::c_char,
    klen: 0,
    ktype: 0,
    scope: 0,
}; 8];
#[cold]
unsafe extern "C" fn mod_wstunnel_set_defaults(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    if config_plugin_values_init(
        srv,
        p as *mut libc::c_void,
        cpk.as_ptr(),
        b"mod_wstunnel\0" as *const u8 as *const libc::c_char,
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
            let mut current_block_25: u64;
            match (*cpv).k_id {
                0 => {
                    gw = calloc(
                        1 as libc::c_int as libc::c_ulong,
                        ::std::mem::size_of::<gw_plugin_config>() as libc::c_ulong,
                    ) as *mut gw_plugin_config;
                    if gw.is_null() {
                        ck_assert_failed(
                            b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                            281 as libc::c_int as libc::c_uint,
                            b"gw\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if gw_set_defaults_backend(
                        srv,
                        p as *mut gw_plugin_data,
                        (*cpv).v.a,
                        gw,
                        0 as libc::c_int,
                        cpk[(*cpv).k_id as usize].k,
                    ) == 0
                    {
                        gw_plugin_config_free(gw);
                        return HANDLER_ERROR;
                    }
                    if !((*gw).exts_auth).is_null() && (*(*gw).exts_auth).used != 0 {
                        log_error(
                            (*srv).errh,
                            b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                            291 as libc::c_int as libc::c_uint,
                            b"%s must not define any hosts with attribute \"mode\" = \"authorizer\"\0"
                                as *const u8 as *const libc::c_char,
                            cpk[(*cpv).k_id as usize].k,
                        );
                        gw_plugin_config_free(gw);
                        return HANDLER_ERROR;
                    }
                    (*cpv).v.v = gw as *mut libc::c_void;
                    (*cpv).vtype = T_CONFIG_LOCAL;
                    current_block_25 = 2873832966593178012;
                }
                1 => {
                    (*cpv)
                        .v
                        .u = gw_get_defaults_balance(srv, (*cpv).v.b) as libc::c_uint;
                    current_block_25 = 2873832966593178012;
                }
                2 => {
                    current_block_25 = 12457477072398871281;
                }
                3 => {
                    current_block_25 = 12457477072398871281;
                }
                4 => {
                    (*cpv)
                        .v
                        .u = buffer_eq_icase_slen(
                        (*cpv).v.b,
                        b"binary\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    ) as libc::c_uint;
                    current_block_25 = 2873832966593178012;
                }
                5 => {
                    let mut j: uint32_t = 0 as libc::c_int as uint32_t;
                    while j < (*(*cpv).v.a).used {
                        let mut origin: *mut buffer = &mut (*(*((*(*cpv).v.a).data)
                            .offset(j as isize) as *mut data_string))
                            .value;
                        if buffer_is_blank(origin) != 0 {
                            log_error(
                                (*srv).errh,
                                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                315 as libc::c_int as libc::c_uint,
                                b"unexpected empty string in %s\0" as *const u8
                                    as *const libc::c_char,
                                cpk[(*cpv).k_id as usize].k,
                            );
                            return HANDLER_ERROR;
                        }
                        j = j.wrapping_add(1);
                    }
                    current_block_25 = 2873832966593178012;
                }
                6 => {
                    current_block_25 = 2873832966593178012;
                }
                _ => {
                    current_block_25 = 2873832966593178012;
                }
            }
            match current_block_25 {
                12457477072398871281 => {}
                _ => {}
            }
            cpv = cpv.offset(1);
        }
        if !gw.is_null() && !((*gw).exts).is_null() {
            gw_exts_clear_check_local((*gw).exts);
        }
        i += 1;
    }
    (*p).defaults.ping_interval = 0 as libc::c_int as libc::c_ushort;
    if (*p).nconfig > 0 as libc::c_int
        && (*(*p).cvlist).v.u2[1 as libc::c_int as usize] != 0
    {
        let mut cpv_0: *const config_plugin_value_t = ((*p).cvlist)
            .offset((*(*p).cvlist).v.u2[0 as libc::c_int as usize] as isize);
        if -(1 as libc::c_int) != (*cpv_0).k_id {
            mod_wstunnel_merge_config(&mut (*p).defaults, cpv_0);
        }
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn wstunnel_create_env(mut gwhctx: *mut gw_handler_ctx) -> handler_t {
    let mut hctx: *mut handler_ctx = gwhctx as *mut handler_ctx;
    let r: *mut request_st = (*hctx).gw.r;
    let mut rc: handler_t = HANDLER_GO_ON;
    if 0 as libc::c_int as libc::c_long == (*r).reqbody_length {
        http_response_upgrade_read_body_unknown(r);
        chunkqueue_append_chunkqueue(&mut (*r).reqbody_queue, &mut (*r).read_queue);
    }
    rc = mod_wstunnel_handshake_create_response(hctx);
    if rc as libc::c_uint != HANDLER_GO_ON as libc::c_int as libc::c_uint {
        return rc;
    }
    (*r).http_status = 101 as libc::c_int;
    (*r).resp_body_started = 1 as libc::c_int as libc::c_char;
    (*hctx).ping_ts = log_monotonic_secs;
    gw_set_transparent(&mut (*hctx).gw);
    return HANDLER_GO_ON;
}
unsafe extern "C" fn wstunnel_stdin_append(
    mut gwhctx: *mut gw_handler_ctx,
) -> handler_t {
    let mut hctx: *mut handler_ctx = gwhctx as *mut handler_ctx;
    if 0 as libc::c_int == mod_wstunnel_frame_recv(hctx) {
        return HANDLER_GO_ON
    } else {
        let r: *mut request_st = (*hctx).gw.r;
        if (*hctx).gw.conf.debug >= 3 as libc::c_int {
            log_error(
                (*hctx).errh,
                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                380 as libc::c_int as libc::c_uint,
                b"disconnected from client (fd=%d)\0" as *const u8
                    as *const libc::c_char,
                (*(*r).con).fd,
            );
        }
        if (*hctx).gw.conf.debug >= 4 as libc::c_int {
            log_error(
                (*hctx).errh,
                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                381 as libc::c_int as libc::c_uint,
                b"send close response to client (fd=%d)\0" as *const u8
                    as *const libc::c_char,
                (*(*r).con).fd,
            );
        }
        mod_wstunnel_frame_send(
            hctx,
            MOD_WEBSOCKET_FRAME_TYPE_CLOSE,
            b"1000\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        gw_handle_request_reset(r, (*hctx).gw.plugin_data as *mut libc::c_void);
        return HANDLER_FINISHED;
    };
}
unsafe extern "C" fn wstunnel_recv_parse(
    r: *mut request_st,
    opts: *mut http_response_opts,
    b: *mut buffer,
    mut n: size_t,
) -> handler_t {
    let mut hctx: *mut handler_ctx = (*opts).pdata as *mut handler_ctx;
    if (*hctx).gw.conf.debug >= 4 as libc::c_int {
        log_error(
            (*hctx).errh,
            b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
            390 as libc::c_int as libc::c_uint,
            b"recv data from backend (fd=%d), size=%zx\0" as *const u8
                as *const libc::c_char,
            (*hctx).gw.fd,
            n,
        );
    }
    if 0 as libc::c_int as libc::c_ulong == n {
        return HANDLER_FINISHED;
    }
    if mod_wstunnel_frame_send(hctx, (*hctx).frame.type_backend, (*b).ptr, n)
        < 0 as libc::c_int
    {
        if (*hctx).gw.conf.debug >= 1 as libc::c_int {
            log_error(
                (*hctx).errh,
                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                393 as libc::c_int as libc::c_uint,
                b"%s\0" as *const u8 as *const libc::c_char,
                b"fail to send data to client\0" as *const u8 as *const libc::c_char,
            );
        }
        return HANDLER_ERROR;
    }
    buffer_clear(b);
    return HANDLER_GO_ON;
}
unsafe extern "C" fn wstunnel_is_allowed_origin(
    r: *mut request_st,
    hctx: *mut handler_ctx,
) -> libc::c_int {
    let allowed_origins: *const array = (*hctx).conf.origins;
    let mut origin: *const buffer = 0 as *const buffer;
    let mut olen: size_t = 0;
    if allowed_origins.is_null()
        || 0 as libc::c_int as libc::c_uint == (*allowed_origins).used
    {
        if (*hctx).gw.conf.debug >= 3 as libc::c_int {
            log_error(
                (*hctx).errh,
                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                410 as libc::c_int as libc::c_uint,
                b"%s\0" as *const u8 as *const libc::c_char,
                b"allowed origins not specified\0" as *const u8 as *const libc::c_char,
            );
        }
        return 1 as libc::c_int;
    }
    origin = http_header_request_get(
        r,
        HTTP_HEADER_OTHER,
        b"Origin\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if origin.is_null() {
        origin = http_header_request_get(
            r,
            HTTP_HEADER_OTHER,
            b"Sec-WebSocket-Origin\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    olen = (if !origin.is_null() {
        buffer_clen(origin)
    } else {
        0 as libc::c_int as libc::c_uint
    }) as size_t;
    if 0 as libc::c_int as libc::c_ulong == olen {
        if (*hctx).gw.conf.debug >= 1 as libc::c_int {
            log_error(
                (*hctx).errh,
                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                423 as libc::c_int as libc::c_uint,
                b"%s\0" as *const u8 as *const libc::c_char,
                b"Origin header is invalid\0" as *const u8 as *const libc::c_char,
            );
        }
        (*r).http_status = 400 as libc::c_int;
        return 0 as libc::c_int;
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < (*allowed_origins).used as libc::c_ulong {
        let mut b: *mut buffer = &mut (*(*((*allowed_origins).data).offset(i as isize)
            as *mut data_string))
            .value;
        let mut blen: size_t = buffer_clen(b) as size_t;
        if (if olen > blen {
            (*((*origin).ptr)
                .offset(
                    olen
                        .wrapping_sub(blen)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int == '.' as i32) as libc::c_int
        } else {
            (olen == blen) as libc::c_int
        }) != 0
            && 0 as libc::c_int
                == memcmp(
                    ((*origin).ptr).offset(olen as isize).offset(-(blen as isize))
                        as *const libc::c_void,
                    (*b).ptr as *const libc::c_void,
                    blen,
                )
        {
            if (*hctx).gw.conf.debug >= 3 as libc::c_int {
                log_error(
                    (*hctx).errh,
                    b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                    433 as libc::c_int as libc::c_uint,
                    b"%s matches allowed origin: %s\0" as *const u8
                        as *const libc::c_char,
                    (*origin).ptr,
                    (*b).ptr,
                );
            }
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    if (*hctx).gw.conf.debug >= 3 as libc::c_int {
        log_error(
            (*hctx).errh,
            b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
            437 as libc::c_int as libc::c_uint,
            b"%s does not match any allowed origins\0" as *const u8
                as *const libc::c_char,
            (*origin).ptr,
        );
    }
    (*r).http_status = 403 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn wstunnel_check_request(
    r: *mut request_st,
    hctx: *mut handler_ctx,
) -> libc::c_int {
    let vers: *const buffer = http_header_request_get(
        r,
        HTTP_HEADER_OTHER,
        b"Sec-WebSocket-Version\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    let hybivers: libc::c_long = if !vers.is_null() {
        if light_isdigit(*(*vers).ptr as libc::c_int) != 0 {
            strtol((*vers).ptr, 0 as *mut *mut libc::c_char, 10 as libc::c_int)
        } else {
            -(1 as libc::c_int) as libc::c_long
        }
    } else {
        0 as libc::c_int as libc::c_long
    };
    if hybivers < 0 as libc::c_int as libc::c_long
        || hybivers > 2147483647 as libc::c_int as libc::c_long
    {
        if (*hctx).gw.conf.debug >= 1 as libc::c_int {
            log_error(
                (*hctx).errh,
                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                449 as libc::c_int as libc::c_uint,
                b"%s\0" as *const u8 as *const libc::c_char,
                b"invalid Sec-WebSocket-Version\0" as *const u8 as *const libc::c_char,
            );
        }
        (*r).http_status = 400 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if ((*r).http_host).is_null() || buffer_is_blank((*r).http_host) != 0 {
        if (*hctx).gw.conf.debug >= 1 as libc::c_int {
            log_error(
                (*hctx).errh,
                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                456 as libc::c_int as libc::c_uint,
                b"%s\0" as *const u8 as *const libc::c_char,
                b"Host header does not exist\0" as *const u8 as *const libc::c_char,
            );
        }
        (*r).http_status = 400 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if wstunnel_is_allowed_origin(r, hctx) == 0 {
        return -(1 as libc::c_int);
    }
    return hybivers as libc::c_int;
}
unsafe extern "C" fn wstunnel_backend_error(mut gwhctx: *mut gw_handler_ctx) {
    let mut hctx: *mut handler_ctx = gwhctx as *mut handler_ctx;
    if (*hctx).gw.state as libc::c_uint == GW_STATE_WRITE as libc::c_int as libc::c_uint
        || (*hctx).gw.state as libc::c_uint
            == GW_STATE_READ as libc::c_int as libc::c_uint
    {
        mod_wstunnel_frame_send(
            hctx,
            MOD_WEBSOCKET_FRAME_TYPE_CLOSE,
            b"1001\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
}
unsafe extern "C" fn wstunnel_handler_ctx_free(mut gwhctx: *mut libc::c_void) {
    let mut hctx: *mut handler_ctx = gwhctx as *mut handler_ctx;
    chunk_buffer_release((*hctx).frame.payload);
}
unsafe extern "C" fn wstunnel_handler_setup(
    r: *mut request_st,
    p: *mut plugin_data,
) -> handler_t {
    let mut hctx: *mut handler_ctx = *((*r).plugin_ctx).offset((*p).id as isize)
        as *mut handler_ctx;
    let mut hybivers: libc::c_int = 0;
    (*hctx).errh = (*r).conf.errh;
    (*hctx).conf = (*p).conf;
    hybivers = wstunnel_check_request(r, hctx);
    if hybivers < 0 as libc::c_int {
        return HANDLER_FINISHED;
    }
    (*hctx).hybivers = hybivers;
    if 0 as libc::c_int == hybivers {
        if (*hctx).gw.conf.debug >= 3 as libc::c_int {
            log_error(
                (*hctx).errh,
                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                489 as libc::c_int as libc::c_uint,
                b"WebSocket Version = %s\0" as *const u8 as *const libc::c_char,
                b"hybi-00\0" as *const u8 as *const libc::c_char,
            );
        }
    } else if (*hctx).gw.conf.debug >= 3 as libc::c_int {
        log_error(
            (*hctx).errh,
            b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
            492 as libc::c_int as libc::c_uint,
            b"WebSocket Version = %d\0" as *const u8 as *const libc::c_char,
            hybivers,
        );
    }
    (*hctx).gw.opts.backend = BACKEND_PROXY as libc::c_int;
    (*hctx).gw.opts.pdata = hctx as *mut libc::c_void;
    (*hctx)
        .gw
        .opts
        .parse = Some(
        wstunnel_recv_parse
            as unsafe extern "C" fn(
                *mut request_st,
                *mut http_response_opts,
                *mut buffer,
                size_t,
            ) -> handler_t,
    );
    (*hctx)
        .gw
        .stdin_append = Some(
        wstunnel_stdin_append as unsafe extern "C" fn(*mut gw_handler_ctx) -> handler_t,
    );
    (*hctx)
        .gw
        .create_env = Some(
        wstunnel_create_env as unsafe extern "C" fn(*mut gw_handler_ctx) -> handler_t,
    );
    (*hctx)
        .gw
        .handler_ctx_free = Some(
        wstunnel_handler_ctx_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*hctx)
        .gw
        .backend_error = Some(
        wstunnel_backend_error as unsafe extern "C" fn(*mut gw_handler_ctx) -> (),
    );
    (*hctx).gw.response = chunk_buffer_acquire();
    (*hctx).frame.state = MOD_WEBSOCKET_FRAME_STATE_INIT;
    (*hctx).frame.ctl.siz = 0 as libc::c_int as uint64_t;
    (*hctx).frame.payload = chunk_buffer_acquire();
    let mut binary: libc::c_uint = (*hctx).conf.frame_type;
    if binary == 0 {
        let mut vb: *const buffer = http_header_request_get(
            r,
            HTTP_HEADER_OTHER,
            b"Sec-WebSocket-Protocol\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        if !vb.is_null() {
            let mut s: *const libc::c_char = (*vb).ptr;
            while *s != 0 {
                while *s as libc::c_int == ' ' as i32 || *s as libc::c_int == '\t' as i32
                    || *s as libc::c_int == '\r' as i32
                    || *s as libc::c_int == '\n' as i32
                {
                    s = s.offset(1);
                }
                if buffer_eq_icase_ssn(
                    s,
                    b"binary\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                ) != 0
                {
                    s = s
                        .offset(
                            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    while *s as libc::c_int == ' ' as i32
                        || *s as libc::c_int == '\t' as i32
                        || *s as libc::c_int == '\r' as i32
                        || *s as libc::c_int == '\n' as i32
                    {
                        s = s.offset(1);
                    }
                    if *s as libc::c_int == ',' as i32
                        || *s as libc::c_int == '\u{0}' as i32
                    {
                        (*hctx).subproto = 1 as libc::c_int;
                        binary = 1 as libc::c_int as libc::c_uint;
                        break;
                    }
                } else if buffer_eq_icase_ssn(
                        s,
                        b"base64\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    ) != 0
                    {
                    s = s
                        .offset(
                            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    while *s as libc::c_int == ' ' as i32
                        || *s as libc::c_int == '\t' as i32
                        || *s as libc::c_int == '\r' as i32
                        || *s as libc::c_int == '\n' as i32
                    {
                        s = s.offset(1);
                    }
                    if *s as libc::c_int == ',' as i32
                        || *s as libc::c_int == '\u{0}' as i32
                    {
                        (*hctx).subproto = -(1 as libc::c_int);
                        break;
                    }
                }
                s = strchr(s, ',' as i32);
                if s.is_null() {
                    break;
                }
                s = s.offset(1);
            }
        }
    }
    if binary != 0 {
        if (*hctx).gw.conf.debug >= 3 as libc::c_int {
            log_error(
                (*hctx).errh,
                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                539 as libc::c_int as libc::c_uint,
                b"%s\0" as *const u8 as *const libc::c_char,
                b"will recv binary data from backend\0" as *const u8
                    as *const libc::c_char,
            );
        }
        (*hctx).frame.type_0 = MOD_WEBSOCKET_FRAME_TYPE_BIN;
        (*hctx).frame.type_before = MOD_WEBSOCKET_FRAME_TYPE_BIN;
        (*hctx).frame.type_backend = MOD_WEBSOCKET_FRAME_TYPE_BIN;
    } else {
        if (*hctx).gw.conf.debug >= 3 as libc::c_int {
            log_error(
                (*hctx).errh,
                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                545 as libc::c_int as libc::c_uint,
                b"%s\0" as *const u8 as *const libc::c_char,
                b"will recv text data from backend\0" as *const u8 as *const libc::c_char,
            );
        }
        (*hctx).frame.type_0 = MOD_WEBSOCKET_FRAME_TYPE_TEXT;
        (*hctx).frame.type_before = MOD_WEBSOCKET_FRAME_TYPE_TEXT;
        (*hctx).frame.type_backend = MOD_WEBSOCKET_FRAME_TYPE_TEXT;
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_wstunnel_check_extension(
    r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    let mut vb: *const buffer = 0 as *const buffer;
    let mut rc: handler_t = HANDLER_GO_ON;
    if !((*r).handler_module).is_null() {
        return HANDLER_GO_ON;
    }
    if (*r).http_method as libc::c_int != HTTP_METHOD_GET as libc::c_int {
        return HANDLER_GO_ON;
    }
    if (*r).http_version as libc::c_int != HTTP_VERSION_1_1 as libc::c_int {
        return HANDLER_GO_ON;
    }
    vb = http_header_request_get(
        r,
        HTTP_HEADER_UPGRADE,
        b"Upgrade\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if vb.is_null()
        || http_header_str_contains_token(
            (*vb).ptr,
            buffer_clen(vb),
            b"websocket\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        ) == 0
    {
        return HANDLER_GO_ON;
    }
    vb = http_header_request_get(
        r,
        HTTP_HEADER_CONNECTION,
        b"Connection\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if vb.is_null()
        || http_header_str_contains_token(
            (*vb).ptr,
            buffer_clen(vb),
            b"upgrade\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        ) == 0
    {
        return HANDLER_GO_ON;
    }
    mod_wstunnel_patch_config(r, p);
    if ((*p).conf.gw.exts).is_null() {
        return HANDLER_GO_ON;
    }
    rc = gw_check_extension(
        r,
        p as *mut gw_plugin_data,
        1 as libc::c_int,
        ::std::mem::size_of::<handler_ctx>() as libc::c_ulong,
    );
    return (if HANDLER_GO_ON as libc::c_int as libc::c_uint == rc as libc::c_uint
        && (*r).handler_module == (*p).self_0 as *const plugin
    {
        wstunnel_handler_setup(r, p) as libc::c_uint
    } else {
        rc as libc::c_uint
    }) as handler_t;
}
unsafe extern "C" fn mod_wstunnel_handle_trigger(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *const plugin_data = p_d as *const plugin_data;
    let cur_ts: unix_time64_t = log_monotonic_secs + 1 as libc::c_int as libc::c_long;
    gw_handle_trigger(srv, p_d);
    let mut con: *mut connection = (*srv).conns;
    while !con.is_null() {
        let r: *mut request_st = &mut (*con).request;
        let mut hctx: *mut handler_ctx = *((*r).plugin_ctx).offset((*p).id as isize)
            as *mut handler_ctx;
        if !(hctx.is_null() || (*r).handler_module != (*p).self_0 as *const plugin) {
            if !((*hctx).gw.state as libc::c_uint
                != GW_STATE_WRITE as libc::c_int as libc::c_uint
                && (*hctx).gw.state as libc::c_uint
                    != GW_STATE_READ as libc::c_int as libc::c_uint)
            {
                if cur_ts - (*con).read_idle_ts > (*r).conf.max_read_idle as libc::c_long
                {
                    if (*hctx).gw.conf.debug >= 3 as libc::c_int {
                        log_error(
                            (*hctx).errh,
                            b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                            604 as libc::c_int as libc::c_uint,
                            b"timeout client (fd=%d)\0" as *const u8
                                as *const libc::c_char,
                            (*con).fd,
                        );
                    }
                    mod_wstunnel_frame_send(
                        hctx,
                        MOD_WEBSOCKET_FRAME_TYPE_CLOSE,
                        0 as *const libc::c_char,
                        0 as libc::c_int as size_t,
                    );
                    gw_handle_request_reset(r, p_d);
                    connection_jq_append(con);
                    (*con).read_idle_ts = cur_ts;
                } else if 0 as libc::c_int != (*hctx).hybivers
                        && (*hctx).conf.ping_interval as libc::c_int > 0 as libc::c_int
                        && (*hctx).conf.ping_interval as int32_t as libc::c_long
                            + (*hctx).ping_ts < cur_ts
                    {
                    (*hctx).ping_ts = cur_ts;
                    mod_wstunnel_frame_send(
                        hctx,
                        MOD_WEBSOCKET_FRAME_TYPE_PING,
                        b"ping\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    );
                    connection_jq_append(con);
                }
            }
        }
        con = (*con).next;
    }
    return HANDLER_GO_ON;
}
#[no_mangle]
pub unsafe extern "C" fn mod_wstunnel_plugin_init(mut p: *mut plugin) -> libc::c_int {
    (*p).version = 0x10440 as libc::c_int as size_t;
    (*p).name = b"wstunnel\0" as *const u8 as *const libc::c_char;
    (*p)
        .init = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
    >(Some(mod_wstunnel_init as unsafe extern "C" fn() -> *mut libc::c_void));
    (*p).cleanup = Some(gw_free as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*p)
        .set_defaults = Some(
        mod_wstunnel_set_defaults
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_request_reset = Some(
        gw_handle_request_reset
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_uri_clean = Some(
        mod_wstunnel_check_extension
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_subrequest = Some(
        gw_handle_subrequest
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_trigger = Some(
        mod_wstunnel_handle_trigger
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
#[inline]
unsafe extern "C" fn MD5_once(
    digest: *mut libc::c_uchar,
    data: *const libc::c_void,
    n: size_t,
) {
    let mut ctx: MD5_CTX = MD5_CTX {
        state: [0; 4],
        count: [0; 2],
        buffer: [0; 64],
    };
    MD5_Init(&mut ctx);
    MD5_Update(&mut ctx, data, n as libc::c_uint);
    MD5_Final(digest, &mut ctx);
}
unsafe extern "C" fn get_key3(
    r: *mut request_st,
    mut buf: *mut libc::c_char,
    mut bytes: uint32_t,
) -> libc::c_int {
    let mut cq: *mut chunkqueue = &mut (*r).reqbody_queue;
    return chunkqueue_read_data(cq, buf, bytes, (*r).conf.errh);
}
unsafe extern "C" fn get_key_number(
    mut ret: *mut uint32_t,
    mut b: *const buffer,
) -> libc::c_int {
    let s: *const libc::c_char = (*b).ptr;
    let mut j: size_t = 0 as libc::c_int as size_t;
    let mut n: libc::c_ulong = 0;
    let mut sp: uint32_t = 0 as libc::c_int as uint32_t;
    let mut tmp: [libc::c_char; 11] = [0; 11];
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut used: size_t = buffer_clen(b) as size_t;
    while i < used {
        if light_isdigit(*s.offset(i as isize) as libc::c_int) != 0 {
            tmp[j as usize] = *s.offset(i as isize);
            j = j.wrapping_add(1);
            if j >= ::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong {
                return -(1 as libc::c_int);
            }
        } else if *s.offset(i as isize) as libc::c_int == ' ' as i32 {
            sp = sp.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    tmp[j as usize] = '\u{0}' as i32 as libc::c_char;
    n = strtoul(tmp.as_mut_ptr(), 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    if n > 4294967295 as libc::c_uint as libc::c_ulong
        || 0 as libc::c_int as libc::c_uint == sp
        || light_isdigit(*tmp.as_mut_ptr() as libc::c_int) == 0
    {
        return -(1 as libc::c_int);
    }
    *ret = (n as uint32_t).wrapping_div(sp);
    return 0 as libc::c_int;
}
unsafe extern "C" fn create_MD5_sum(r: *mut request_st) -> libc::c_int {
    let mut buf: [uint32_t; 4] = [0; 4];
    let mut key1: *const buffer = http_header_request_get(
        r,
        HTTP_HEADER_OTHER,
        b"Sec-WebSocket-Key1\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    let mut key2: *const buffer = http_header_request_get(
        r,
        HTTP_HEADER_OTHER,
        b"Sec-WebSocket-Key2\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if key1.is_null()
        || get_key_number(buf.as_mut_ptr().offset(0 as libc::c_int as isize), key1)
            < 0 as libc::c_int || key2.is_null()
        || get_key_number(buf.as_mut_ptr().offset(1 as libc::c_int as isize), key2)
            < 0 as libc::c_int
        || get_key3(
            r,
            buf.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut libc::c_char,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                as uint32_t,
        ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    MD5_once(
        buf.as_mut_ptr() as *mut libc::c_uchar,
        buf.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint32_t; 4]>() as libc::c_ulong,
    );
    chunkqueue_append_mem(
        &mut (*r).write_queue,
        buf.as_mut_ptr() as *mut libc::c_char,
        ::std::mem::size_of::<[uint32_t; 4]>() as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn create_response_ietf_00(mut hctx: *mut handler_ctx) -> libc::c_int {
    let r: *mut request_st = (*hctx).gw.r;
    let mut origin: *const buffer = http_header_request_get(
        r,
        HTTP_HEADER_OTHER,
        b"Origin\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if origin.is_null() {
        origin = http_header_request_get(
            r,
            HTTP_HEADER_OTHER,
            b"Sec-WebSocket-Origin\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    if origin.is_null() {
        if (*hctx).gw.conf.debug >= 1 as libc::c_int {
            log_error(
                (*hctx).errh,
                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                724 as libc::c_int as libc::c_uint,
                b"%s\0" as *const u8 as *const libc::c_char,
                b"Origin header is invalid\0" as *const u8 as *const libc::c_char,
            );
        }
        return -(1 as libc::c_int);
    }
    if ((*r).http_host).is_null() || buffer_is_blank((*r).http_host) != 0 {
        if (*hctx).gw.conf.debug >= 1 as libc::c_int {
            log_error(
                (*hctx).errh,
                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                728 as libc::c_int as libc::c_uint,
                b"%s\0" as *const u8 as *const libc::c_char,
                b"Host header does not exist\0" as *const u8 as *const libc::c_char,
            );
        }
        return -(1 as libc::c_int);
    }
    if create_MD5_sum(r) < 0 as libc::c_int {
        if (*hctx).gw.conf.debug >= 1 as libc::c_int {
            log_error(
                (*hctx).errh,
                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                734 as libc::c_int as libc::c_uint,
                b"%s\0" as *const u8 as *const libc::c_char,
                b"Sec-WebSocket-Key is invalid\0" as *const u8 as *const libc::c_char,
            );
        }
        return -(1 as libc::c_int);
    }
    http_header_response_set(
        r,
        HTTP_HEADER_UPGRADE,
        b"Upgrade\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
        b"websocket\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    let value: *mut buffer = http_header_response_set_ptr(
        r,
        HTTP_HEADER_OTHER,
        b"Sec-WebSocket-Location\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if buffer_eq_slen(
        &mut (*r).uri.scheme,
        b"https\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    ) != 0
    {
        buffer_copy_string_len(
            value,
            b"wss://\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    } else {
        buffer_copy_string_len(
            value,
            b"ws://\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    buffer_append_str2(
        value,
        (*(*r).http_host).ptr,
        buffer_clen((*r).http_host) as size_t,
        (*r).uri.path.ptr,
        buffer_clen(&mut (*r).uri.path) as size_t,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn create_response_rfc_6455(
    mut hctx: *mut handler_ctx,
) -> libc::c_int {
    let r: *mut request_st = (*hctx).gw.r;
    let mut sha: SHA_CTX = SHA_CTX {
        state: [0; 5],
        count: [0; 2],
        buffer: [0; 64],
    };
    let mut sha_digest: [libc::c_uchar; 20] = [0; 20];
    let mut value_wskey: *const buffer = http_header_request_get(
        r,
        HTTP_HEADER_OTHER,
        b"Sec-WebSocket-Key\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if value_wskey.is_null() {
        if (*hctx).gw.conf.debug >= 1 as libc::c_int {
            log_error(
                (*hctx).errh,
                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                783 as libc::c_int as libc::c_uint,
                b"%s\0" as *const u8 as *const libc::c_char,
                b"Sec-WebSocket-Key is invalid\0" as *const u8 as *const libc::c_char,
            );
        }
        return -(1 as libc::c_int);
    }
    SHA1_Init(&mut sha);
    SHA1_Update(
        &mut sha,
        (*value_wskey).ptr as *const libc::c_uchar,
        buffer_clen(value_wskey),
    );
    SHA1_Update(
        &mut sha,
        b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11\0" as *const u8 as *const libc::c_char
            as *const libc::c_uchar,
        (::std::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    SHA1_Final(sha_digest.as_mut_ptr(), &mut sha);
    http_header_response_set(
        r,
        HTTP_HEADER_UPGRADE,
        b"Upgrade\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
        b"websocket\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    let value: *mut buffer = http_header_response_set_ptr(
        r,
        HTTP_HEADER_OTHER,
        b"Sec-WebSocket-Accept\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    buffer_append_base64_enc(
        value,
        sha_digest.as_mut_ptr(),
        20 as libc::c_int as size_t,
        BASE64_STANDARD,
        1 as libc::c_int,
    );
    if (*hctx).frame.type_0 as libc::c_uint
        == MOD_WEBSOCKET_FRAME_TYPE_BIN as libc::c_int as libc::c_uint
    {
        http_header_response_set(
            r,
            HTTP_HEADER_OTHER,
            b"Sec-WebSocket-Protocol\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            b"binary\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    } else if -(1 as libc::c_int) == (*hctx).subproto {
        http_header_response_set(
            r,
            HTTP_HEADER_OTHER,
            b"Sec-WebSocket-Protocol\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            b"base64\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mod_wstunnel_handshake_create_response(
    mut hctx: *mut handler_ctx,
) -> handler_t {
    let r: *mut request_st = (*hctx).gw.r;
    if (*hctx).hybivers >= 8 as libc::c_int {
        if (*hctx).gw.conf.debug >= 4 as libc::c_int {
            log_error(
                (*hctx).errh,
                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                827 as libc::c_int as libc::c_uint,
                b"%s\0" as *const u8 as *const libc::c_char,
                b"send handshake response\0" as *const u8 as *const libc::c_char,
            );
        }
        if 0 as libc::c_int != create_response_rfc_6455(hctx) {
            (*r).http_status = 400 as libc::c_int;
            return HANDLER_ERROR;
        }
        return HANDLER_GO_ON;
    }
    if (*hctx).hybivers == 0 as libc::c_int {
        let mut cq: *mut chunkqueue = &mut (*r).reqbody_queue;
        if chunkqueue_length(cq) < 8 as libc::c_int as libc::c_long {
            return HANDLER_WAIT_FOR_EVENT;
        }
        if (*hctx).gw.conf.debug >= 4 as libc::c_int {
            log_error(
                (*hctx).errh,
                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                846 as libc::c_int as libc::c_uint,
                b"%s\0" as *const u8 as *const libc::c_char,
                b"send handshake response\0" as *const u8 as *const libc::c_char,
            );
        }
        if 0 as libc::c_int != create_response_ietf_00(hctx) {
            (*r).http_status = 400 as libc::c_int;
            return HANDLER_ERROR;
        }
        return HANDLER_GO_ON;
    }
    if (*hctx).gw.conf.debug >= 1 as libc::c_int {
        log_error(
            (*hctx).errh,
            b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
            855 as libc::c_int as libc::c_uint,
            b"%s\0" as *const u8 as *const libc::c_char,
            b"not supported WebSocket Version\0" as *const u8 as *const libc::c_char,
        );
    }
    (*r).http_status = 503 as libc::c_int;
    return HANDLER_ERROR;
}
unsafe extern "C" fn send_ietf_00(
    mut hctx: *mut handler_ctx,
    mut type_0: mod_wstunnel_frame_type_t,
    mut payload: *const libc::c_char,
    mut siz: size_t,
) -> libc::c_int {
    static mut head: libc::c_char = 0 as libc::c_int as libc::c_char;
    static mut tail: libc::c_char = !(0 as libc::c_int) as libc::c_char;
    let r: *mut request_st = (*hctx).gw.r;
    let mut mem: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    match type_0 as libc::c_uint {
        0 => {
            if 0 as libc::c_int as libc::c_ulong == siz {
                return 0 as libc::c_int;
            }
            http_chunk_append_mem(r, &head, 1 as libc::c_int as size_t);
            http_chunk_append_mem(r, payload, siz);
            http_chunk_append_mem(r, &tail, 1 as libc::c_int as size_t);
            len = siz.wrapping_add(2 as libc::c_int as libc::c_ulong);
        }
        1 => {
            if 0 as libc::c_int as libc::c_ulong == siz {
                return 0 as libc::c_int;
            }
            http_chunk_append_mem(r, &head, 1 as libc::c_int as size_t);
            len = (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(siz.wrapping_div(3 as libc::c_int as libc::c_ulong))
                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            mem = malloc(len) as *mut libc::c_char;
            if mem.is_null() {
                ck_assert_failed(
                    b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                    896 as libc::c_int as libc::c_uint,
                    b"mem\0" as *const u8 as *const libc::c_char,
                );
            }
            len = li_base64_enc(
                mem,
                len,
                payload as *mut libc::c_uchar,
                siz,
                BASE64_STANDARD,
                1 as libc::c_int,
            );
            http_chunk_append_mem(r, mem, len);
            free(mem as *mut libc::c_void);
            http_chunk_append_mem(r, &tail, 1 as libc::c_int as size_t);
            len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        }
        2 => {
            http_chunk_append_mem(r, &tail, 1 as libc::c_int as size_t);
            http_chunk_append_mem(r, &head, 1 as libc::c_int as size_t);
            len = 2 as libc::c_int as size_t;
        }
        _ => {
            if (*hctx).gw.conf.debug >= 1 as libc::c_int {
                log_error(
                    (*hctx).errh,
                    b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                    909 as libc::c_int as libc::c_uint,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    b"invalid frame type\0" as *const u8 as *const libc::c_char,
                );
            }
            return -(1 as libc::c_int);
        }
    }
    if (*hctx).gw.conf.debug >= 4 as libc::c_int {
        log_error(
            (*hctx).errh,
            b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
            913 as libc::c_int as libc::c_uint,
            b"send data to client (fd=%d), frame size=%zx\0" as *const u8
                as *const libc::c_char,
            (*(*r).con).fd,
            len,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn recv_ietf_00(mut hctx: *mut handler_ctx) -> libc::c_int {
    let r: *mut request_st = (*hctx).gw.r;
    let mut cq: *mut chunkqueue = &mut (*r).reqbody_queue;
    let mut payload: *mut buffer = (*hctx).frame.payload;
    let mut mem: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*hctx).gw.conf.debug >= 4 as libc::c_int {
        log_error(
            (*hctx).errh,
            b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
            923 as libc::c_int as libc::c_uint,
            b"recv data from client (fd=%d), size=%llx\0" as *const u8
                as *const libc::c_char,
            (*(*r).con).fd,
            chunkqueue_length(cq) as libc::c_longlong,
        );
    }
    let mut c: *mut chunk = (*cq).first;
    while !c.is_null() {
        let mut frame: *mut libc::c_char = ((*(*c).mem).ptr)
            .offset((*c).offset as isize);
        let mut flen: size_t = (buffer_clen((*c).mem) as libc::c_long - (*c).offset)
            as size_t;
        if !((*c).type_0 as libc::c_uint == MEM_CHUNK as libc::c_int as libc::c_uint) {
            ck_assert_failed(
                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                929 as libc::c_int as libc::c_uint,
                b"c->type == MEM_CHUNK\0" as *const u8 as *const libc::c_char,
            );
        }
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < flen {
            match (*hctx).frame.state as libc::c_uint {
                0 => {
                    (*hctx).frame.ctl.siz = 0 as libc::c_int as uint64_t;
                    if *frame.offset(i as isize) as libc::c_int == 0 as libc::c_int {
                        (*hctx).frame.state = MOD_WEBSOCKET_FRAME_STATE_READ_PAYLOAD;
                        i = i.wrapping_add(1);
                    } else if *(frame as *mut libc::c_uchar).offset(i as isize)
                            as libc::c_int == 0xff as libc::c_int
                        {
                        if (*hctx).gw.conf.debug >= 4 as libc::c_int {
                            log_error(
                                (*hctx).errh,
                                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                939 as libc::c_int as libc::c_uint,
                                b"%s\0" as *const u8 as *const libc::c_char,
                                b"recv close frame\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        return -(1 as libc::c_int);
                    } else {
                        if (*hctx).gw.conf.debug >= 4 as libc::c_int {
                            log_error(
                                (*hctx).errh,
                                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                943 as libc::c_int as libc::c_uint,
                                b"%s\0" as *const u8 as *const libc::c_char,
                                b"recv invalid frame\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        return -(1 as libc::c_int);
                    }
                }
                4 => {
                    mem = memchr(
                        frame.offset(i as isize) as *const libc::c_void,
                        0xff as libc::c_int,
                        flen.wrapping_sub(i),
                    ) as *mut libc::c_char;
                    if mem.is_null() {
                        if (*hctx).gw.conf.debug >= 4 as libc::c_int {
                            log_error(
                                (*hctx).errh,
                                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                950 as libc::c_int as libc::c_uint,
                                b"got continuous payload, size=%zx\0" as *const u8
                                    as *const libc::c_char,
                                flen.wrapping_sub(i),
                            );
                        }
                        (*hctx)
                            .frame
                            .ctl
                            .siz = ((*hctx).frame.ctl.siz as libc::c_ulong)
                            .wrapping_add(flen.wrapping_sub(i)) as uint64_t as uint64_t;
                        if (*hctx).frame.ctl.siz
                            > 0xfffff as libc::c_int as libc::c_ulong
                        {
                            if (*hctx).gw.conf.debug >= 2 as libc::c_int {
                                log_error(
                                    (*hctx).errh,
                                    b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                    954 as libc::c_int as libc::c_uint,
                                    b"frame size has been exceeded: %x\0" as *const u8
                                        as *const libc::c_char,
                                    0xfffff as libc::c_int,
                                );
                            }
                            return -(1 as libc::c_int);
                        }
                        buffer_append_string_len(
                            payload,
                            frame.offset(i as isize),
                            flen.wrapping_sub(i),
                        );
                        i = (i as libc::c_ulong).wrapping_add(flen.wrapping_sub(i))
                            as size_t as size_t;
                    } else {
                        if (*hctx).gw.conf.debug >= 4 as libc::c_int {
                            log_error(
                                (*hctx).errh,
                                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                962 as libc::c_int as libc::c_uint,
                                b"got final payload, size=%zx\0" as *const u8
                                    as *const libc::c_char,
                                mem.offset_from(frame.offset(i as isize)) as libc::c_long,
                            );
                        }
                        (*hctx)
                            .frame
                            .ctl
                            .siz = ((*hctx).frame.ctl.siz as libc::c_ulong)
                            .wrapping_add(
                                mem.offset_from(frame.offset(i as isize)) as libc::c_long
                                    as libc::c_ulong,
                            ) as uint64_t as uint64_t;
                        if (*hctx).frame.ctl.siz
                            > 0xfffff as libc::c_int as libc::c_ulong
                        {
                            if (*hctx).gw.conf.debug >= 2 as libc::c_int {
                                log_error(
                                    (*hctx).errh,
                                    b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                    966 as libc::c_int as libc::c_uint,
                                    b"frame size has been exceeded: %x\0" as *const u8
                                        as *const libc::c_char,
                                    0xfffff as libc::c_int,
                                );
                            }
                            return -(1 as libc::c_int);
                        }
                        buffer_append_string_len(
                            payload,
                            frame.offset(i as isize),
                            mem.offset_from(frame.offset(i as isize)) as libc::c_long
                                as size_t,
                        );
                        i = (i as libc::c_ulong)
                            .wrapping_add(
                                mem.offset_from(frame.offset(i as isize)) as libc::c_long
                                    as libc::c_ulong,
                            ) as size_t as size_t;
                        (*hctx).frame.state = MOD_WEBSOCKET_FRAME_STATE_INIT;
                    }
                    i = i.wrapping_add(1);
                    if (*hctx).frame.type_0 as libc::c_uint
                        == MOD_WEBSOCKET_FRAME_TYPE_TEXT as libc::c_int as libc::c_uint
                        && buffer_is_unset(payload) == 0
                    {
                        (*hctx).frame.ctl.siz = 0 as libc::c_int as uint64_t;
                        chunkqueue_append_buffer(&mut (*hctx).gw.wb, payload);
                    } else if (*hctx).frame.state as libc::c_uint
                            == MOD_WEBSOCKET_FRAME_STATE_INIT as libc::c_int
                                as libc::c_uint && buffer_is_unset(payload) == 0
                        {
                        let mut b: *mut buffer = 0 as *mut buffer;
                        let mut len: size_t = buffer_clen(payload) as size_t;
                        len = len
                            .wrapping_add(3 as libc::c_int as libc::c_ulong)
                            .wrapping_div(4 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(3 as libc::c_int as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong);
                        chunkqueue_get_memory(&mut (*hctx).gw.wb, &mut len);
                        b = (*(*hctx).gw.wb.last).mem;
                        len = buffer_clen(b) as size_t;
                        if (*hctx).gw.conf.debug >= 4 as libc::c_int {
                            log_error(
                                (*hctx).errh,
                                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                990 as libc::c_int as libc::c_uint,
                                b"try to base64 decode: %s\0" as *const u8
                                    as *const libc::c_char,
                                (*payload).ptr,
                            );
                        }
                        if (buffer_append_base64_decode(
                            b,
                            (*payload).ptr,
                            buffer_clen(payload) as size_t,
                            BASE64_STANDARD,
                        ))
                            .is_null()
                        {
                            if (*hctx).gw.conf.debug >= 1 as libc::c_int {
                                log_error(
                                    (*hctx).errh,
                                    b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                    994 as libc::c_int as libc::c_uint,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    b"fail to base64-decode\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            return -(1 as libc::c_int);
                        }
                        buffer_clear(payload);
                        (*hctx)
                            .gw
                            .wb
                            .bytes_in = ((*hctx).gw.wb.bytes_in as libc::c_ulong)
                            .wrapping_add(
                                (buffer_clen(b) as libc::c_ulong).wrapping_sub(len),
                            ) as off_t as off_t;
                    }
                }
                _ => {
                    if (*hctx).gw.conf.debug >= 1 as libc::c_int {
                        log_error(
                            (*hctx).errh,
                            b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                            1004 as libc::c_int as libc::c_uint,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            b"BUG: unknown state\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    return -(1 as libc::c_int);
                }
            }
        }
        c = (*c).next;
    }
    chunkqueue_mark_written(cq, chunkqueue_length(cq));
    return 0 as libc::c_int;
}
unsafe extern "C" fn send_rfc_6455(
    mut hctx: *mut handler_ctx,
    mut type_0: mod_wstunnel_frame_type_t,
    mut payload: *const libc::c_char,
    mut siz: size_t,
) -> libc::c_int {
    let mut mem: [libc::c_char; 10] = [0; 10];
    let mut len: size_t = 0;
    if payload.is_null()
        && (type_0 as libc::c_uint
            == MOD_WEBSOCKET_FRAME_TYPE_TEXT as libc::c_int as libc::c_uint
            || type_0 as libc::c_uint
                == MOD_WEBSOCKET_FRAME_TYPE_BIN as libc::c_int as libc::c_uint)
    {
        return -(1 as libc::c_int);
    }
    match type_0 as libc::c_uint {
        0 => {
            mem[0 as libc::c_int
                as usize] = (0x80 as libc::c_int | 0x1 as libc::c_int) as libc::c_char;
            if (*hctx).gw.conf.debug >= 4 as libc::c_int {
                log_error(
                    (*hctx).errh,
                    b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                    1045 as libc::c_int as libc::c_uint,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    b"type = text\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        1 => {
            mem[0 as libc::c_int
                as usize] = (0x80 as libc::c_int | 0x2 as libc::c_int) as libc::c_char;
            if (*hctx).gw.conf.debug >= 4 as libc::c_int {
                log_error(
                    (*hctx).errh,
                    b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                    1049 as libc::c_int as libc::c_uint,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    b"type = binary\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        3 => {
            mem[0 as libc::c_int
                as usize] = (0x80 as libc::c_int | 0x9 as libc::c_int) as libc::c_char;
            if (*hctx).gw.conf.debug >= 4 as libc::c_int {
                log_error(
                    (*hctx).errh,
                    b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                    1053 as libc::c_int as libc::c_uint,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    b"type = ping\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        4 => {
            mem[0 as libc::c_int
                as usize] = (0x80 as libc::c_int | 0xa as libc::c_int) as libc::c_char;
            if (*hctx).gw.conf.debug >= 4 as libc::c_int {
                log_error(
                    (*hctx).errh,
                    b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                    1057 as libc::c_int as libc::c_uint,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    b"type = pong\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        2 | _ => {
            mem[0 as libc::c_int
                as usize] = (0x80 as libc::c_int | 0x8 as libc::c_int) as libc::c_char;
            if (*hctx).gw.conf.debug >= 4 as libc::c_int {
                log_error(
                    (*hctx).errh,
                    b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                    1062 as libc::c_int as libc::c_uint,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    b"type = close\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    if (*hctx).gw.conf.debug >= 4 as libc::c_int {
        log_error(
            (*hctx).errh,
            b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
            1066 as libc::c_int as libc::c_uint,
            b"payload size=%zx\0" as *const u8 as *const libc::c_char,
            siz,
        );
    }
    if siz < 0x7e as libc::c_int as libc::c_ulong {
        mem[1 as libc::c_int as usize] = siz as libc::c_char;
        len = 2 as libc::c_int as size_t;
    } else if siz <= 65535 as libc::c_int as libc::c_ulong {
        mem[1 as libc::c_int as usize] = 0x7e as libc::c_int as libc::c_char;
        mem[2 as libc::c_int
            as usize] = (siz >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_char;
        mem[3 as libc::c_int
            as usize] = (siz & 0xff as libc::c_int as libc::c_ulong) as libc::c_char;
        len = (1 as libc::c_int + 2 as libc::c_int + 1 as libc::c_int) as size_t;
    } else {
        mem[1 as libc::c_int as usize] = 0x7f as libc::c_int as libc::c_char;
        mem[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        mem[3 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        mem[4 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        mem[5 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        mem[6 as libc::c_int
            as usize] = (siz >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_char;
        mem[7 as libc::c_int
            as usize] = (siz >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_char;
        mem[8 as libc::c_int
            as usize] = (siz >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_char;
        mem[9 as libc::c_int
            as usize] = (siz & 0xff as libc::c_int as libc::c_ulong) as libc::c_char;
        len = (1 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int) as size_t;
    }
    let r: *mut request_st = (*hctx).gw.r;
    http_chunk_append_mem(r, mem.as_mut_ptr(), len);
    if siz != 0 {
        http_chunk_append_mem(r, payload, siz);
    }
    if (*hctx).gw.conf.debug >= 4 as libc::c_int {
        log_error(
            (*hctx).errh,
            b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
            1093 as libc::c_int as libc::c_uint,
            b"send data to client (fd=%d), frame size=%zx\0" as *const u8
                as *const libc::c_char,
            (*(*r).con).fd,
            len.wrapping_add(siz),
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn unmask_payload(mut hctx: *mut handler_ctx) {
    let b: *mut buffer = (*hctx).frame.payload;
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut used: size_t = buffer_clen(b) as size_t;
    while i < used {
        let ref mut fresh0 = *((*b).ptr).offset(i as isize);
        *fresh0 = (*fresh0 as libc::c_int
            ^ (*hctx).frame.ctl.mask[(*hctx).frame.ctl.mask_cnt as usize] as libc::c_int)
            as libc::c_char;
        (*hctx)
            .frame
            .ctl
            .mask_cnt = ((*hctx).frame.ctl.mask_cnt + 1 as libc::c_int)
            % 4 as libc::c_int;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn recv_rfc_6455(mut hctx: *mut handler_ctx) -> libc::c_int {
    let r: *mut request_st = (*hctx).gw.r;
    let mut cq: *mut chunkqueue = &mut (*r).reqbody_queue;
    let mut payload: *mut buffer = (*hctx).frame.payload;
    if (*hctx).gw.conf.debug >= 4 as libc::c_int {
        log_error(
            (*hctx).errh,
            b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
            1110 as libc::c_int as libc::c_uint,
            b"recv data from client (fd=%d), size=%llx\0" as *const u8
                as *const libc::c_char,
            (*(*r).con).fd,
            chunkqueue_length(cq) as libc::c_longlong,
        );
    }
    let mut c: *mut chunk = (*cq).first;
    while !c.is_null() {
        let mut frame: *mut libc::c_char = ((*(*c).mem).ptr)
            .offset((*c).offset as isize);
        let mut flen: size_t = (buffer_clen((*c).mem) as libc::c_long - (*c).offset)
            as size_t;
        if !((*c).type_0 as libc::c_uint == MEM_CHUNK as libc::c_int as libc::c_uint) {
            ck_assert_failed(
                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                1116 as libc::c_int as libc::c_uint,
                b"c->type == MEM_CHUNK\0" as *const u8 as *const libc::c_char,
            );
        }
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < flen {
            match (*hctx).frame.state as libc::c_uint {
                0 => {
                    match *frame.offset(i as isize) as libc::c_int & 0xf as libc::c_int {
                        0 => {
                            if (*hctx).gw.conf.debug >= 4 as libc::c_int {
                                log_error(
                                    (*hctx).errh,
                                    b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                    1122 as libc::c_int as libc::c_uint,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    b"type = continue\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            (*hctx).frame.type_0 = (*hctx).frame.type_before;
                        }
                        1 => {
                            if (*hctx).gw.conf.debug >= 4 as libc::c_int {
                                log_error(
                                    (*hctx).errh,
                                    b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                    1126 as libc::c_int as libc::c_uint,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    b"type = text\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            (*hctx).frame.type_0 = MOD_WEBSOCKET_FRAME_TYPE_TEXT;
                            (*hctx).frame.type_before = (*hctx).frame.type_0;
                        }
                        2 => {
                            if (*hctx).gw.conf.debug >= 4 as libc::c_int {
                                log_error(
                                    (*hctx).errh,
                                    b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                    1131 as libc::c_int as libc::c_uint,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    b"type = binary\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            (*hctx).frame.type_0 = MOD_WEBSOCKET_FRAME_TYPE_BIN;
                            (*hctx).frame.type_before = (*hctx).frame.type_0;
                        }
                        9 => {
                            if (*hctx).gw.conf.debug >= 4 as libc::c_int {
                                log_error(
                                    (*hctx).errh,
                                    b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                    1136 as libc::c_int as libc::c_uint,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    b"type = ping\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            (*hctx).frame.type_0 = MOD_WEBSOCKET_FRAME_TYPE_PING;
                        }
                        10 => {
                            if (*hctx).gw.conf.debug >= 4 as libc::c_int {
                                log_error(
                                    (*hctx).errh,
                                    b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                    1140 as libc::c_int as libc::c_uint,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    b"type = pong\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            (*hctx).frame.type_0 = MOD_WEBSOCKET_FRAME_TYPE_PONG;
                        }
                        8 => {
                            if (*hctx).gw.conf.debug >= 4 as libc::c_int {
                                log_error(
                                    (*hctx).errh,
                                    b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                    1144 as libc::c_int as libc::c_uint,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    b"type = close\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            (*hctx).frame.type_0 = MOD_WEBSOCKET_FRAME_TYPE_CLOSE;
                            return -(1 as libc::c_int);
                        }
                        _ => {
                            if (*hctx).gw.conf.debug >= 1 as libc::c_int {
                                log_error(
                                    (*hctx).errh,
                                    b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                    1149 as libc::c_int as libc::c_uint,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    b"type is invalid\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            return -(1 as libc::c_int);
                        }
                    }
                    i = i.wrapping_add(1);
                    (*hctx).frame.state = MOD_WEBSOCKET_FRAME_STATE_READ_LENGTH;
                }
                1 => {
                    if *frame.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                        != 0x80 as libc::c_int
                    {
                        if (*hctx).gw.conf.debug >= 1 as libc::c_int {
                            log_error(
                                (*hctx).errh,
                                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                1158 as libc::c_int as libc::c_uint,
                                b"%s\0" as *const u8 as *const libc::c_char,
                                b"payload was not masked\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        return -(1 as libc::c_int);
                    }
                    (*hctx).frame.ctl.mask_cnt = 0 as libc::c_int;
                    (*hctx)
                        .frame
                        .ctl
                        .siz = (*frame.offset(i as isize) as libc::c_int
                        & 0x7f as libc::c_int) as uint64_t;
                    if (*hctx).frame.ctl.siz == 0 as libc::c_int as libc::c_ulong {
                        if (*hctx).gw.conf.debug >= 4 as libc::c_int {
                            log_error(
                                (*hctx).errh,
                                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                1165 as libc::c_int as libc::c_uint,
                                b"specified payload size=%llx\0" as *const u8
                                    as *const libc::c_char,
                                (*hctx).frame.ctl.siz as libc::c_ulonglong,
                            );
                        }
                        (*hctx).frame.state = MOD_WEBSOCKET_FRAME_STATE_READ_MASK;
                    } else if (*hctx).frame.ctl.siz
                            == 0x7e as libc::c_int as libc::c_ulong
                        {
                        (*hctx).frame.ctl.siz = 0 as libc::c_int as uint64_t;
                        (*hctx).frame.ctl.siz_cnt = 2 as libc::c_int;
                        (*hctx).frame.state = MOD_WEBSOCKET_FRAME_STATE_READ_EX_LENGTH;
                    } else if (*hctx).frame.ctl.siz
                            == 0x7f as libc::c_int as libc::c_ulong
                        {
                        (*hctx).frame.ctl.siz = 0 as libc::c_int as uint64_t;
                        (*hctx).frame.ctl.siz_cnt = 8 as libc::c_int;
                        (*hctx).frame.state = MOD_WEBSOCKET_FRAME_STATE_READ_EX_LENGTH;
                    } else {
                        if (*hctx).gw.conf.debug >= 4 as libc::c_int {
                            log_error(
                                (*hctx).errh,
                                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                1182 as libc::c_int as libc::c_uint,
                                b"specified payload size=%llx\0" as *const u8
                                    as *const libc::c_char,
                                (*hctx).frame.ctl.siz as libc::c_ulonglong,
                            );
                        }
                        (*hctx).frame.state = MOD_WEBSOCKET_FRAME_STATE_READ_MASK;
                    }
                    i = i.wrapping_add(1);
                }
                2 => {
                    (*hctx)
                        .frame
                        .ctl
                        .siz = ((*hctx).frame.ctl.siz << 8 as libc::c_int)
                        .wrapping_add(
                            (*frame.offset(i as isize) as libc::c_int
                                & 0xff as libc::c_int) as libc::c_ulong,
                        );
                    (*hctx).frame.ctl.siz_cnt -= 1;
                    if (*hctx).frame.ctl.siz_cnt <= 0 as libc::c_int {
                        if (*hctx).frame.type_0 as libc::c_uint
                            == MOD_WEBSOCKET_FRAME_TYPE_PING as libc::c_int
                                as libc::c_uint
                            && (*hctx).frame.ctl.siz
                                > 0xfffff as libc::c_int as libc::c_ulong
                        {
                            if (*hctx).gw.conf.debug >= 2 as libc::c_int {
                                log_error(
                                    (*hctx).errh,
                                    b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                    1195 as libc::c_int as libc::c_uint,
                                    b"frame size has been exceeded: %x\0" as *const u8
                                        as *const libc::c_char,
                                    0xfffff as libc::c_int,
                                );
                            }
                            return -(1 as libc::c_int);
                        }
                        if (*hctx).gw.conf.debug >= 4 as libc::c_int {
                            log_error(
                                (*hctx).errh,
                                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                1199 as libc::c_int as libc::c_uint,
                                b"specified payload size=%llx\0" as *const u8
                                    as *const libc::c_char,
                                (*hctx).frame.ctl.siz as libc::c_ulonglong,
                            );
                        }
                        (*hctx).frame.state = MOD_WEBSOCKET_FRAME_STATE_READ_MASK;
                    }
                    i = i.wrapping_add(1);
                }
                3 => {
                    (*hctx)
                        .frame
                        .ctl
                        .mask[(*hctx).frame.ctl.mask_cnt
                        as usize] = *frame.offset(i as isize) as libc::c_uchar;
                    (*hctx).frame.ctl.mask_cnt += 1;
                    if (*hctx).frame.ctl.mask_cnt >= 4 as libc::c_int {
                        (*hctx).frame.ctl.mask_cnt = 0 as libc::c_int;
                        if (*hctx).frame.type_0 as libc::c_uint
                            == MOD_WEBSOCKET_FRAME_TYPE_PING as libc::c_int
                                as libc::c_uint
                            && (*hctx).frame.ctl.siz == 0 as libc::c_int as libc::c_ulong
                        {
                            mod_wstunnel_frame_send(
                                hctx,
                                MOD_WEBSOCKET_FRAME_TYPE_PONG,
                                0 as *const libc::c_char,
                                0 as libc::c_int as size_t,
                            );
                        }
                        if (*hctx).frame.ctl.siz == 0 as libc::c_int as libc::c_ulong {
                            (*hctx).frame.state = MOD_WEBSOCKET_FRAME_STATE_INIT;
                        } else {
                            (*hctx).frame.state = MOD_WEBSOCKET_FRAME_STATE_READ_PAYLOAD;
                        }
                    }
                    i = i.wrapping_add(1);
                }
                4 => {
                    if (*hctx).frame.ctl.siz <= flen.wrapping_sub(i) {
                        if (*hctx).gw.conf.debug >= 4 as libc::c_int {
                            log_error(
                                (*hctx).errh,
                                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                1229 as libc::c_int as libc::c_uint,
                                b"read payload, size=%llx\0" as *const u8
                                    as *const libc::c_char,
                                (*hctx).frame.ctl.siz as libc::c_ulonglong,
                            );
                        }
                        buffer_append_string_len(
                            payload,
                            frame.offset(i as isize),
                            (*hctx).frame.ctl.siz & 18446744073709551615 as libc::c_ulong,
                        );
                        i = (i as libc::c_ulong)
                            .wrapping_add(
                                (*hctx).frame.ctl.siz
                                    & 18446744073709551615 as libc::c_ulong,
                            ) as size_t as size_t;
                        (*hctx).frame.ctl.siz = 0 as libc::c_int as uint64_t;
                        (*hctx).frame.state = MOD_WEBSOCKET_FRAME_STATE_INIT;
                        if (*hctx).gw.conf.debug >= 4 as libc::c_int {
                            log_error(
                                (*hctx).errh,
                                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                1235 as libc::c_int as libc::c_uint,
                                b"rest of frame size=%zx\0" as *const u8
                                    as *const libc::c_char,
                                flen.wrapping_sub(i),
                            );
                        }
                    } else {
                        if (*hctx).gw.conf.debug >= 4 as libc::c_int {
                            log_error(
                                (*hctx).errh,
                                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                1239 as libc::c_int as libc::c_uint,
                                b"read payload, size=%zx\0" as *const u8
                                    as *const libc::c_char,
                                flen.wrapping_sub(i),
                            );
                        }
                        buffer_append_string_len(
                            payload,
                            frame.offset(i as isize),
                            flen.wrapping_sub(i),
                        );
                        (*hctx)
                            .frame
                            .ctl
                            .siz = ((*hctx).frame.ctl.siz as libc::c_ulong)
                            .wrapping_sub(flen.wrapping_sub(i)) as uint64_t as uint64_t;
                        i = (i as libc::c_ulong).wrapping_add(flen.wrapping_sub(i))
                            as size_t as size_t;
                        if (*hctx).gw.conf.debug >= 4 as libc::c_int {
                            log_error(
                                (*hctx).errh,
                                b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                1244 as libc::c_int as libc::c_uint,
                                b"rest of payload size=%llx\0" as *const u8
                                    as *const libc::c_char,
                                (*hctx).frame.ctl.siz as libc::c_ulonglong,
                            );
                        }
                    }
                    match (*hctx).frame.type_0 as libc::c_uint {
                        0 | 1 => {
                            unmask_payload(hctx);
                            chunkqueue_append_buffer(&mut (*hctx).gw.wb, payload);
                        }
                        3 => {
                            if (*hctx).frame.ctl.siz == 0 as libc::c_int as libc::c_ulong
                            {
                                unmask_payload(hctx);
                                mod_wstunnel_frame_send(
                                    hctx,
                                    MOD_WEBSOCKET_FRAME_TYPE_PONG,
                                    (*payload).ptr,
                                    buffer_clen(payload) as size_t,
                                );
                                buffer_clear(payload);
                            }
                        }
                        4 => {
                            buffer_clear(payload);
                        }
                        2 | _ => {
                            if (*hctx).gw.conf.debug >= 1 as libc::c_int {
                                log_error(
                                    (*hctx).errh,
                                    b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                                    1269 as libc::c_int as libc::c_uint,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    b"BUG: invalid frame type\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            return -(1 as libc::c_int);
                        }
                    }
                }
                _ => {
                    if (*hctx).gw.conf.debug >= 1 as libc::c_int {
                        log_error(
                            (*hctx).errh,
                            b"src/mod_wstunnel.c\0" as *const u8 as *const libc::c_char,
                            1274 as libc::c_int as libc::c_uint,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            b"BUG: invalid state\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    return -(1 as libc::c_int);
                }
            }
        }
        c = (*c).next;
    }
    chunkqueue_mark_written(cq, chunkqueue_length(cq));
    return 0 as libc::c_int;
}
unsafe extern "C" fn mod_wstunnel_frame_send(
    mut hctx: *mut handler_ctx,
    mut type_0: mod_wstunnel_frame_type_t,
    mut payload: *const libc::c_char,
    mut siz: size_t,
) -> libc::c_int {
    if (*hctx).hybivers >= 8 as libc::c_int {
        return send_rfc_6455(hctx, type_0, payload, siz);
    }
    if 0 as libc::c_int == (*hctx).hybivers {
        return send_ietf_00(hctx, type_0, payload, siz);
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mod_wstunnel_frame_recv(mut hctx: *mut handler_ctx) -> libc::c_int {
    if (*hctx).hybivers >= 8 as libc::c_int {
        return recv_rfc_6455(hctx);
    }
    if 0 as libc::c_int == (*hctx).hybivers {
        return recv_ietf_00(hctx);
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn run_static_initializers() {
    cpk = [
        {
            let mut init = config_plugin_keys_t {
                k: b"wstunnel.server\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVARRAY as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"wstunnel.balance\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"wstunnel.debug\0" as *const u8 as *const libc::c_char,
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
                k: b"wstunnel.map-extensions\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVSTRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"wstunnel.frame-type\0" as *const u8 as *const libc::c_char,
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
                k: b"wstunnel.origins\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_VLIST as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"wstunnel.ping-interval\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
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
