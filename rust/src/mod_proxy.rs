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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn getsockname(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn buffer_string_prepare_append(b: *mut buffer, size: size_t) -> *mut libc::c_char;
    fn buffer_extend(b: *mut buffer, x: size_t) -> *mut libc::c_char;
    fn buffer_commit(b: *mut buffer, size: size_t);
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
    fn buffer_append_uint_hex_lc(b: *mut buffer, len: uintmax_t);
    fn buffer_append_int(b: *mut buffer, val: intmax_t);
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
    fn buffer_substr_replace(
        b: *mut buffer,
        offset: size_t,
        len: size_t,
        replace: *const buffer,
    );
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn array_is_kvstring(a: *const array) -> libc::c_int;
    fn array_get_int_ptr(
        a: *mut array,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut libc::c_int;
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
    fn http_response_upgrade_read_body_unknown(r: *mut request_st);
    fn http_response_reqbody_read_error(
        r: *mut request_st,
        http_status: libc::c_int,
    ) -> handler_t;
    fn config_plugin_value_tobool(
        du: *const data_unset,
        default_value: libc::c_int,
    ) -> libc::c_int;
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
    static mut plugin_stats: array;
    fn chunkqueue_append_mem(cq: *mut chunkqueue, mem: *const libc::c_char, len: size_t);
    fn chunkqueue_append_mem_min(
        cq: *mut chunkqueue,
        mem: *const libc::c_char,
        len: size_t,
    );
    fn chunkqueue_append_chunkqueue(cq: *mut chunkqueue, src: *mut chunkqueue);
    fn chunkqueue_prepend_buffer_open_sz(cq: *mut chunkqueue, sz: size_t) -> *mut buffer;
    fn chunkqueue_prepend_buffer_commit(cq: *mut chunkqueue);
    fn chunkqueue_steal(dest: *mut chunkqueue, src: *mut chunkqueue, len: off_t);
    fn http_method_buf(i: http_method_t) -> *const buffer;
    fn sock_addr_stringify_append_buffer(
        b: *mut buffer,
        saddr: *const sock_addr,
    ) -> libc::c_int;
    fn http_header_response_get(
        r: *const request_st,
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
    fn http_header_request_set_ptr(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn http_header_request_set(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
        v: *const libc::c_char,
        vlen: uint32_t,
    );
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
pub type __uintmax_t = libc::c_ulong;
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
pub type uintmax_t = __uintmax_t;
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
pub struct data_array {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
    pub value: array,
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
pub struct http_header_remap_opts {
    pub urlpaths: *const array,
    pub hosts_request: *const array,
    pub hosts_response: *const array,
    pub force_http10: libc::c_int,
    pub https_remap: libc::c_int,
    pub upgrade: libc::c_int,
    pub connect_method: libc::c_int,
    pub http_host: *const buffer,
    pub forwarded_host: *const buffer,
    pub forwarded_urlpath: *const data_string,
}
pub type proxy_forwarded_t = libc::c_uint;
pub const PROXY_FORWARDED_REMOTE_USER: proxy_forwarded_t = 16;
pub const PROXY_FORWARDED_BY: proxy_forwarded_t = 8;
pub const PROXY_FORWARDED_HOST: proxy_forwarded_t = 4;
pub const PROXY_FORWARDED_PROTO: proxy_forwarded_t = 2;
pub const PROXY_FORWARDED_FOR: proxy_forwarded_t = 1;
pub const PROXY_FORWARDED_NONE: proxy_forwarded_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_config {
    pub gw: gw_plugin_config,
    pub replace_http_host: libc::c_uint,
    pub forwarded: libc::c_uint,
    pub header: http_header_remap_opts,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct handler_ctx {
    pub gw: gw_handler_ctx,
    pub conf: plugin_config,
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
unsafe extern "C" fn buffer_append_buffer(mut b: *mut buffer, mut src: *const buffer) {
    buffer_append_string_len(b, (*src).ptr, buffer_clen(src) as size_t);
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
unsafe extern "C" fn status_counter_inc(mut s: *const libc::c_char, mut len: size_t) {
    let ref mut fresh0 = *array_get_int_ptr(&mut plugin_stats, s, len as uint32_t);
    *fresh0 += 1;
}
static mut proxy_check_extforward: libc::c_int = 0;
#[cold]
unsafe extern "C" fn mod_proxy_init() -> *mut libc::c_void {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<plugin_data>() as libc::c_ulong,
    );
}
unsafe extern "C" fn mod_proxy_free_config(p: *mut plugin_data) {
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
                5 => {
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
}
#[cold]
unsafe extern "C" fn mod_proxy_free(mut p_d: *mut libc::c_void) {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    mod_proxy_free_config(p);
    gw_free(p as *mut libc::c_void);
}
unsafe extern "C" fn mod_proxy_merge_config_cpv(
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
            (*pconf).forwarded = (*cpv).v.u;
        }
        5 => {
            (*pconf).header = *((*cpv).v.v as *mut http_header_remap_opts);
        }
        6 => {
            (*pconf).replace_http_host = (*cpv).v.u;
        }
        _ => return,
    };
}
unsafe extern "C" fn mod_proxy_merge_config(
    pconf: *mut plugin_config,
    mut cpv: *const config_plugin_value_t,
) {
    loop {
        mod_proxy_merge_config_cpv(pconf, cpv);
        cpv = cpv.offset(1);
        if !((*cpv).k_id != -(1 as libc::c_int)) {
            break;
        }
    };
}
unsafe extern "C" fn mod_proxy_patch_config(r: *mut request_st, p: *mut plugin_data) {
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
            mod_proxy_merge_config(
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
unsafe extern "C" fn mod_proxy_parse_forwarded(
    mut srv: *mut server,
    mut a: *const array,
) -> libc::c_uint {
    let mut forwarded: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut j: uint32_t = 0 as libc::c_int as uint32_t;
    let mut used: uint32_t = (*a).used;
    while j < used {
        let mut param: proxy_forwarded_t = PROXY_FORWARDED_NONE;
        let mut du: *mut data_unset = *((*a).data).offset(j as isize);
        if buffer_eq_slen(
            &mut (*du).key,
            b"by\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            param = PROXY_FORWARDED_BY;
        } else if buffer_eq_slen(
                &mut (*du).key,
                b"for\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            param = PROXY_FORWARDED_FOR;
        } else if buffer_eq_slen(
                &mut (*du).key,
                b"host\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            param = PROXY_FORWARDED_HOST;
        } else if buffer_eq_slen(
                &mut (*du).key,
                b"proto\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            param = PROXY_FORWARDED_PROTO;
        } else if buffer_eq_slen(
                &mut (*du).key,
                b"remote_user\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            param = PROXY_FORWARDED_REMOTE_USER;
        } else {
            log_error(
                (*srv).errh,
                b"src/mod_proxy.c\0" as *const u8 as *const libc::c_char,
                174 as libc::c_int as libc::c_uint,
                b"proxy.forwarded keys must be one of: by, for, host, proto, remote_user, but not: %s\0"
                    as *const u8 as *const libc::c_char,
                (*du).key.ptr,
            );
            return (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint);
        }
        let mut val: libc::c_int = config_plugin_value_tobool(du, 2 as libc::c_int);
        if 2 as libc::c_int == val {
            log_error(
                (*srv).errh,
                b"src/mod_proxy.c\0" as *const u8 as *const libc::c_char,
                181 as libc::c_int as libc::c_uint,
                b"proxy.forwarded values must be one of: 0, 1, enable, disable; error for key: %s\0"
                    as *const u8 as *const libc::c_char,
                (*du).key.ptr,
            );
            return (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint);
        }
        if val != 0 {
            forwarded |= param as libc::c_uint;
        }
        j = j.wrapping_add(1);
    }
    return forwarded;
}
unsafe extern "C" fn mod_proxy_parse_header_opts(
    mut srv: *mut server,
    mut a: *const array,
) -> *mut http_header_remap_opts {
    let mut header: http_header_remap_opts = http_header_remap_opts {
        urlpaths: 0 as *const array,
        hosts_request: 0 as *const array,
        hosts_response: 0 as *const array,
        force_http10: 0,
        https_remap: 0,
        upgrade: 0,
        connect_method: 0,
        http_host: 0 as *const buffer,
        forwarded_host: 0 as *const buffer,
        forwarded_urlpath: 0 as *const data_string,
    };
    memset(
        &mut header as *mut http_header_remap_opts as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<http_header_remap_opts>() as libc::c_ulong,
    );
    let mut j: uint32_t = 0 as libc::c_int as uint32_t;
    let mut used: uint32_t = (*a).used;
    while j < used {
        let mut da: *mut data_array = *((*a).data).offset(j as isize) as *mut data_array;
        if buffer_eq_slen(
            &mut (*da).key,
            b"https-remap\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            let mut val: libc::c_int = config_plugin_value_tobool(
                da as *mut data_unset,
                2 as libc::c_int,
            );
            if 2 as libc::c_int == val {
                log_error(
                    (*srv).errh,
                    b"src/mod_proxy.c\0" as *const u8 as *const libc::c_char,
                    202 as libc::c_int as libc::c_uint,
                    b"unexpected value for proxy.header; expected \"https-remap\" => \"enable\" or \"disable\"\0"
                        as *const u8 as *const libc::c_char,
                );
                return 0 as *mut http_header_remap_opts;
            }
            header.https_remap = val;
        } else if buffer_eq_slen(
                &mut (*da).key,
                b"force-http10\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            let mut val_0: libc::c_int = config_plugin_value_tobool(
                da as *mut data_unset,
                2 as libc::c_int,
            );
            if 2 as libc::c_int == val_0 {
                log_error(
                    (*srv).errh,
                    b"src/mod_proxy.c\0" as *const u8 as *const libc::c_char,
                    213 as libc::c_int as libc::c_uint,
                    b"unexpected value for proxy.header; expected \"force-http10\" => \"enable\" or \"disable\"\0"
                        as *const u8 as *const libc::c_char,
                );
                return 0 as *mut http_header_remap_opts;
            }
            header.force_http10 = val_0;
        } else if buffer_eq_slen(
                &mut (*da).key,
                b"upgrade\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            let mut val_1: libc::c_int = config_plugin_value_tobool(
                da as *mut data_unset,
                2 as libc::c_int,
            );
            if 2 as libc::c_int == val_1 {
                log_error(
                    (*srv).errh,
                    b"src/mod_proxy.c\0" as *const u8 as *const libc::c_char,
                    224 as libc::c_int as libc::c_uint,
                    b"unexpected value for proxy.header; expected \"upgrade\" => \"enable\" or \"disable\"\0"
                        as *const u8 as *const libc::c_char,
                );
                return 0 as *mut http_header_remap_opts;
            }
            header.upgrade = val_1;
        } else if buffer_eq_slen(
                &mut (*da).key,
                b"connect\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
            let mut val_2: libc::c_int = config_plugin_value_tobool(
                da as *mut data_unset,
                2 as libc::c_int,
            );
            if 2 as libc::c_int == val_2 {
                log_error(
                    (*srv).errh,
                    b"src/mod_proxy.c\0" as *const u8 as *const libc::c_char,
                    235 as libc::c_int as libc::c_uint,
                    b"unexpected value for proxy.header; expected \"connect\" => \"enable\" or \"disable\"\0"
                        as *const u8 as *const libc::c_char,
                );
                return 0 as *mut http_header_remap_opts;
            }
            header.connect_method = val_2;
        } else {
            if (*da).type_0 as libc::c_uint != TYPE_ARRAY as libc::c_int as libc::c_uint
                || array_is_kvstring(&mut (*da).value) == 0
            {
                log_error(
                    (*srv).errh,
                    b"src/mod_proxy.c\0" as *const u8 as *const libc::c_char,
                    244 as libc::c_int as libc::c_uint,
                    b"unexpected value for proxy.header; expected ( \"param\" => ( \"key\" => \"value\" ) ) near key %s\0"
                        as *const u8 as *const libc::c_char,
                    (*da).key.ptr,
                );
                return 0 as *mut http_header_remap_opts;
            }
            if buffer_eq_slen(
                &mut (*da).key,
                b"map-urlpath\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
                header.urlpaths = &mut (*da).value;
            } else if buffer_eq_slen(
                    &mut (*da).key,
                    b"map-host-request\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                ) != 0
                {
                header.hosts_request = &mut (*da).value;
            } else if buffer_eq_slen(
                    &mut (*da).key,
                    b"map-host-response\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                ) != 0
                {
                header.hosts_response = &mut (*da).value;
            } else {
                log_error(
                    (*srv).errh,
                    b"src/mod_proxy.c\0" as *const u8 as *const libc::c_char,
                    260 as libc::c_int as libc::c_uint,
                    b"unexpected key for proxy.header; expected ( \"param\" => ( \"key\" => \"value\" ) ) near key %s\0"
                        as *const u8 as *const libc::c_char,
                    (*da).key.ptr,
                );
                return 0 as *mut http_header_remap_opts;
            }
        }
        j = j.wrapping_add(1);
    }
    let mut opts: *mut http_header_remap_opts = malloc(
        ::std::mem::size_of::<http_header_remap_opts>() as libc::c_ulong,
    ) as *mut http_header_remap_opts;
    if opts.is_null() {
        ck_assert_failed(
            b"src/mod_proxy.c\0" as *const u8 as *const libc::c_char,
            269 as libc::c_int as libc::c_uint,
            b"opts\0" as *const u8 as *const libc::c_char,
        );
    }
    memcpy(
        opts as *mut libc::c_void,
        &mut header as *mut http_header_remap_opts as *const libc::c_void,
        ::std::mem::size_of::<http_header_remap_opts>() as libc::c_ulong,
    );
    return opts;
}
static mut cpk: [config_plugin_keys_t; 8] = [config_plugin_keys_t {
    k: 0 as *const libc::c_char,
    klen: 0,
    ktype: 0,
    scope: 0,
}; 8];
#[cold]
unsafe extern "C" fn mod_proxy_set_defaults(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    if config_plugin_values_init(
        srv,
        p as *mut libc::c_void,
        cpk.as_ptr(),
        b"mod_proxy\0" as *const u8 as *const libc::c_char,
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
            let mut current_block_26: u64;
            match (*cpv).k_id {
                0 => {
                    gw = calloc(
                        1 as libc::c_int as libc::c_ulong,
                        ::std::mem::size_of::<gw_plugin_config>() as libc::c_ulong,
                    ) as *mut gw_plugin_config;
                    if gw.is_null() {
                        ck_assert_failed(
                            b"src/mod_proxy.c\0" as *const u8 as *const libc::c_char,
                            317 as libc::c_int as libc::c_uint,
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
                            b"src/mod_proxy.c\0" as *const u8 as *const libc::c_char,
                            327 as libc::c_int as libc::c_uint,
                            b"%s must not define any hosts with attribute \"mode\" = \"authorizer\"\0"
                                as *const u8 as *const libc::c_char,
                            cpk[(*cpv).k_id as usize].k,
                        );
                        gw_plugin_config_free(gw);
                        return HANDLER_ERROR;
                    }
                    (*cpv).v.v = gw as *mut libc::c_void;
                    (*cpv).vtype = T_CONFIG_LOCAL;
                    current_block_26 = 14434620278749266018;
                }
                1 => {
                    (*cpv)
                        .v
                        .u = gw_get_defaults_balance(srv, (*cpv).v.b) as libc::c_uint;
                    current_block_26 = 14434620278749266018;
                }
                2 => {
                    current_block_26 = 6392508821572509226;
                }
                3 => {
                    current_block_26 = 6392508821572509226;
                }
                4 => {
                    (*cpv).v.u = mod_proxy_parse_forwarded(srv, (*cpv).v.a);
                    if (2147483647 as libc::c_int as libc::c_uint)
                        .wrapping_mul(2 as libc::c_uint)
                        .wrapping_add(1 as libc::c_uint) == (*cpv).v.u
                    {
                        return HANDLER_ERROR;
                    }
                    (*cpv).vtype = T_CONFIG_LOCAL;
                    current_block_26 = 14434620278749266018;
                }
                5 => {
                    (*cpv)
                        .v
                        .v = mod_proxy_parse_header_opts(srv, (*cpv).v.a)
                        as *mut libc::c_void;
                    if ((*cpv).v.v).is_null() {
                        return HANDLER_ERROR;
                    }
                    (*cpv).vtype = T_CONFIG_LOCAL;
                    current_block_26 = 14434620278749266018;
                }
                6 => {
                    current_block_26 = 14434620278749266018;
                }
                _ => {
                    current_block_26 = 14434620278749266018;
                }
            }
            match current_block_26 {
                6392508821572509226 => {}
                _ => {}
            }
            cpv = cpv.offset(1);
        }
        if !gw.is_null() && !((*gw).exts).is_null() {
            gw_exts_clear_check_local((*gw).exts);
        }
        i += 1;
    }
    (*p)
        .defaults
        .header
        .force_http10 = config_feature_bool(
        srv,
        b"proxy.force-http10\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    if (*p).nconfig > 0 as libc::c_int
        && (*(*p).cvlist).v.u2[1 as libc::c_int as usize] != 0
    {
        let mut cpv_0: *const config_plugin_value_t = ((*p).cvlist)
            .offset((*(*p).cvlist).v.u2[0 as libc::c_int as usize] as isize);
        if -(1 as libc::c_int) != (*cpv_0).k_id {
            mod_proxy_merge_config(&mut (*p).defaults, cpv_0);
        }
    }
    let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
    while i_0 < (*(*srv).srvconf.modules).used {
        let mut m: *mut buffer = &mut (*(*((*(*srv).srvconf.modules).data)
            .offset(i_0 as isize) as *mut data_string))
            .value;
        if buffer_eq_slen(
            m,
            b"mod_extforward\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            proxy_check_extforward = 1 as libc::c_int;
            break;
        } else {
            i_0 = i_0.wrapping_add(1);
        }
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn http_header_remap_host_match(
    mut b: *mut buffer,
    mut off: size_t,
    mut remap_hdrs: *mut http_header_remap_opts,
    mut is_req: libc::c_int,
    mut alen: size_t,
) -> *const buffer {
    let mut hosts: *const array = if is_req != 0 {
        (*remap_hdrs).hosts_request
    } else {
        (*remap_hdrs).hosts_response
    };
    if !hosts.is_null() {
        let s: *const libc::c_char = ((*b).ptr).offset(off as isize);
        let mut current_block_9: u64;
        let mut i: size_t = 0 as libc::c_int as size_t;
        let mut used: size_t = (*hosts).used as size_t;
        while i < used {
            let ds: *const data_string = *((*hosts).data).offset(i as isize)
                as *mut data_string;
            let mut k: *const buffer = &(*ds).key;
            let mut mlen: size_t = buffer_clen(k) as size_t;
            if 1 as libc::c_int as libc::c_ulong == mlen
                && *((*k).ptr).offset(0 as libc::c_int as isize) as libc::c_int
                    == '-' as i32
            {
                k = if is_req != 0 {
                    (*remap_hdrs).http_host
                } else {
                    (*remap_hdrs).forwarded_host
                };
                if k.is_null() {
                    current_block_9 = 14155750587950065367;
                } else {
                    mlen = buffer_clen(k) as size_t;
                    current_block_9 = 3640593987805443782;
                }
            } else {
                current_block_9 = 3640593987805443782;
            }
            match current_block_9 {
                3640593987805443782 => {
                    if buffer_eq_icase_ss(s, alen, (*k).ptr, mlen) != 0 {
                        if buffer_eq_slen(
                            &(*ds).value,
                            b"-\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        ) != 0
                        {
                            return (*remap_hdrs).http_host
                        } else {
                            if buffer_is_blank(&(*ds).value) == 0 {
                                if is_req != 0 && ((*remap_hdrs).forwarded_host).is_null() {
                                    (*remap_hdrs).forwarded_host = &(*ds).value;
                                }
                                return &(*ds).value;
                            }
                        }
                        break;
                    }
                }
                _ => {}
            }
            i = i.wrapping_add(1);
        }
    }
    return 0 as *const buffer;
}
unsafe extern "C" fn http_header_remap_host(
    mut b: *mut buffer,
    mut off: size_t,
    mut remap_hdrs: *mut http_header_remap_opts,
    mut is_req: libc::c_int,
    mut alen: size_t,
) -> size_t {
    let m: *const buffer = http_header_remap_host_match(
        b,
        off,
        remap_hdrs,
        is_req,
        alen,
    );
    if m.is_null() {
        return alen;
    }
    buffer_substr_replace(b, off, alen, m);
    return buffer_clen(m) as size_t;
}
unsafe extern "C" fn http_header_remap_urlpath(
    mut b: *mut buffer,
    mut off: size_t,
    mut remap_hdrs: *mut http_header_remap_opts,
    mut is_req: libc::c_int,
) -> size_t {
    let mut urlpaths: *const array = (*remap_hdrs).urlpaths;
    if !urlpaths.is_null() {
        let s: *const libc::c_char = ((*b).ptr).offset(off as isize);
        let plen: size_t = (buffer_clen(b) as libc::c_ulong).wrapping_sub(off);
        if is_req != 0 {
            let mut i: size_t = 0 as libc::c_int as size_t;
            let mut used: size_t = (*urlpaths).used as size_t;
            while i < used {
                let ds: *const data_string = *((*urlpaths).data).offset(i as isize)
                    as *mut data_string;
                let mlen: size_t = buffer_clen(&(*ds).key) as size_t;
                if mlen <= plen
                    && 0 as libc::c_int
                        == memcmp(
                            s as *const libc::c_void,
                            (*ds).key.ptr as *const libc::c_void,
                            mlen,
                        )
                {
                    if ((*remap_hdrs).forwarded_urlpath).is_null() {
                        (*remap_hdrs).forwarded_urlpath = ds;
                    }
                    buffer_substr_replace(b, off, mlen, &(*ds).value);
                    return buffer_clen(&(*ds).value) as size_t;
                }
                i = i.wrapping_add(1);
            }
        } else {
            if !((*remap_hdrs).forwarded_urlpath).is_null() {
                let ds_0: *const data_string = (*remap_hdrs).forwarded_urlpath;
                let mlen_0: size_t = buffer_clen(&(*ds_0).value) as size_t;
                if mlen_0 <= plen
                    && 0 as libc::c_int
                        == memcmp(
                            s as *const libc::c_void,
                            (*ds_0).value.ptr as *const libc::c_void,
                            mlen_0,
                        )
                {
                    buffer_substr_replace(b, off, mlen_0, &(*ds_0).key);
                    return buffer_clen(&(*ds_0).key) as size_t;
                }
            }
            let mut i_0: size_t = 0 as libc::c_int as size_t;
            let mut used_0: size_t = (*urlpaths).used as size_t;
            while i_0 < used_0 {
                let ds_1: *const data_string = *((*urlpaths).data).offset(i_0 as isize)
                    as *mut data_string;
                let mlen_1: size_t = buffer_clen(&(*ds_1).value) as size_t;
                if mlen_1 <= plen
                    && 0 as libc::c_int
                        == memcmp(
                            s as *const libc::c_void,
                            (*ds_1).value.ptr as *const libc::c_void,
                            mlen_1,
                        )
                {
                    buffer_substr_replace(b, off, mlen_1, &(*ds_1).key);
                    return buffer_clen(&(*ds_1).key) as size_t;
                }
                i_0 = i_0.wrapping_add(1);
            }
        }
    }
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn http_header_remap_uri(
    mut b: *mut buffer,
    mut off: size_t,
    mut remap_hdrs: *mut http_header_remap_opts,
    mut is_req: libc::c_int,
) {
    if *((*b).ptr).offset(off as isize) as libc::c_int != '/' as i32 {
        let mut s: *mut libc::c_char = ((*b).ptr).offset(off as isize);
        let mut alen: size_t = 0;
        let mut slen: size_t = 0;
        let mut m: *const buffer = 0 as *const buffer;
        s = strchr(s, ':' as i32);
        if s.is_null()
            || *s.offset(1 as libc::c_int as isize) as libc::c_int != '/' as i32
            || *s.offset(2 as libc::c_int as isize) as libc::c_int != '/' as i32
        {
            return;
        }
        slen = s.offset_from(((*b).ptr).offset(off as isize)) as libc::c_long as size_t;
        s = s.offset(3 as libc::c_int as isize);
        off = s.offset_from((*b).ptr) as libc::c_long as size_t;
        s = strchr(s, '/' as i32);
        if !s.is_null() {
            alen = (s.offset_from((*b).ptr) as libc::c_long as size_t).wrapping_sub(off);
            if 0 as libc::c_int as libc::c_ulong == alen {
                return;
            }
        } else {
            alen = (buffer_clen(b) as libc::c_ulong).wrapping_sub(off);
            if 0 as libc::c_int as libc::c_ulong == alen {
                return;
            }
            buffer_append_string_len(
                b,
                b"/\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
        m = http_header_remap_host_match(b, off, remap_hdrs, is_req, alen);
        if !m.is_null() {
            if (*remap_hdrs).https_remap != 0
                && (if is_req != 0 {
                    (5 as libc::c_int as libc::c_ulong == slen
                        && 0 as libc::c_int
                            == memcmp(
                                ((*b).ptr)
                                    .offset(off as isize)
                                    .offset(-(slen as isize))
                                    .offset(-(3 as libc::c_int as isize))
                                    as *const libc::c_void,
                                b"https\0" as *const u8 as *const libc::c_char
                                    as *const libc::c_void,
                                5 as libc::c_int as libc::c_ulong,
                            )) as libc::c_int
                } else {
                    (4 as libc::c_int as libc::c_ulong == slen
                        && 0 as libc::c_int
                            == memcmp(
                                ((*b).ptr)
                                    .offset(off as isize)
                                    .offset(-(slen as isize))
                                    .offset(-(3 as libc::c_int as isize))
                                    as *const libc::c_void,
                                b"http\0" as *const u8 as *const libc::c_char
                                    as *const libc::c_void,
                                4 as libc::c_int as libc::c_ulong,
                            )) as libc::c_int
                }) != 0
            {
                if is_req != 0 {
                    memcpy(
                        ((*b).ptr)
                            .offset(off as isize)
                            .offset(-(slen as isize))
                            .offset(-(3 as libc::c_int as isize))
                            .offset(4 as libc::c_int as isize) as *mut libc::c_void,
                        b"://\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        3 as libc::c_int as libc::c_ulong,
                    );
                    off = off.wrapping_sub(1);
                    alen = alen.wrapping_add(1);
                } else {
                    memcpy(
                        ((*b).ptr)
                            .offset(off as isize)
                            .offset(-(slen as isize))
                            .offset(-(3 as libc::c_int as isize))
                            .offset(4 as libc::c_int as isize) as *mut libc::c_void,
                        b"s://\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        4 as libc::c_int as libc::c_ulong,
                    );
                    off = off.wrapping_add(1);
                    alen = alen.wrapping_sub(1);
                }
            }
            buffer_substr_replace(b, off, alen, m);
            alen = buffer_clen(m) as size_t;
        }
        off = (off as libc::c_ulong).wrapping_add(alen) as size_t as size_t;
    }
    http_header_remap_urlpath(b, off, remap_hdrs, is_req);
}
unsafe extern "C" fn http_header_remap_setcookie(
    mut b: *mut buffer,
    mut off: size_t,
    mut remap_hdrs: *mut http_header_remap_opts,
) {
    let mut current_block_31: u64;
    let mut s: *mut libc::c_char = ((*b).ptr).offset(off as isize);
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    while *s != 0 {
        let mut len: size_t = 0;
        while *s as libc::c_int != ';' as i32 && *s as libc::c_int != '\n' as i32
            && *s as libc::c_int != '\u{0}' as i32
        {
            s = s.offset(1);
        }
        if *s as libc::c_int == '\n' as i32 {
            s = s
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as isize,
                );
        }
        if '\u{0}' as i32 == *s as libc::c_int {
            return;
        }
        loop {
            s = s.offset(1);
            if !(*s as libc::c_int == ' ' as i32 || *s as libc::c_int == '\t' as i32) {
                break;
            }
        }
        if '\u{0}' as i32 == *s as libc::c_int {
            return;
        }
        e = s.offset(1 as libc::c_int as isize);
        if !('=' as i32 == *s as libc::c_int) {
            while *e as libc::c_int != '=' as i32 && *e as libc::c_int != '\u{0}' as i32
            {
                e = e.offset(1);
            }
            if '\u{0}' as i32 == *e as libc::c_int {
                return;
            }
            e = e.offset(1);
            match (e.offset_from(s) as libc::c_long - 1 as libc::c_int as libc::c_long)
                as libc::c_int
            {
                4 => {
                    current_block_31 = 3812213244340499285;
                    match current_block_31 {
                        3059221110377644841 => {
                            if buffer_eq_icase_ssn(
                                s,
                                b"domain\0" as *const u8 as *const libc::c_char,
                                6 as libc::c_int as size_t,
                            ) != 0
                            {
                                let mut alen: size_t = 0 as libc::c_int as size_t;
                                if *e as libc::c_int == '"' as i32 {
                                    e = e.offset(1);
                                }
                                if *e as libc::c_int == '.' as i32 {
                                    e = e.offset(1);
                                }
                                if !(*e as libc::c_int == ';' as i32) {
                                    off = e.offset_from((*b).ptr) as libc::c_long as size_t;
                                    let mut c: libc::c_char = 0;
                                    loop {
                                        c = *e.offset(alen as isize);
                                        if !(c as libc::c_int != ';' as i32
                                            && c as libc::c_int != ' ' as i32
                                            && c as libc::c_int != '\t' as i32
                                            && c as libc::c_int != '\r' as i32
                                            && c as libc::c_int != '\u{0}' as i32)
                                        {
                                            break;
                                        }
                                        alen = alen.wrapping_add(1);
                                    }
                                    len = http_header_remap_host(
                                        b,
                                        off,
                                        remap_hdrs,
                                        0 as libc::c_int,
                                        alen,
                                    );
                                    e = ((*b).ptr).offset(off as isize).offset(len as isize);
                                }
                            }
                        }
                        _ => {
                            if buffer_eq_icase_ssn(
                                s,
                                b"path\0" as *const u8 as *const libc::c_char,
                                4 as libc::c_int as size_t,
                            ) != 0
                            {
                                if *e as libc::c_int == '"' as i32 {
                                    e = e.offset(1);
                                }
                                if !(*e as libc::c_int != '/' as i32) {
                                    off = e.offset_from((*b).ptr) as libc::c_long as size_t;
                                    len = http_header_remap_urlpath(
                                        b,
                                        off,
                                        remap_hdrs,
                                        0 as libc::c_int,
                                    );
                                    e = ((*b).ptr).offset(off as isize).offset(len as isize);
                                }
                            }
                        }
                    }
                }
                6 => {
                    current_block_31 = 3059221110377644841;
                    match current_block_31 {
                        3059221110377644841 => {
                            if buffer_eq_icase_ssn(
                                s,
                                b"domain\0" as *const u8 as *const libc::c_char,
                                6 as libc::c_int as size_t,
                            ) != 0
                            {
                                let mut alen: size_t = 0 as libc::c_int as size_t;
                                if *e as libc::c_int == '"' as i32 {
                                    e = e.offset(1);
                                }
                                if *e as libc::c_int == '.' as i32 {
                                    e = e.offset(1);
                                }
                                if !(*e as libc::c_int == ';' as i32) {
                                    off = e.offset_from((*b).ptr) as libc::c_long as size_t;
                                    let mut c: libc::c_char = 0;
                                    loop {
                                        c = *e.offset(alen as isize);
                                        if !(c as libc::c_int != ';' as i32
                                            && c as libc::c_int != ' ' as i32
                                            && c as libc::c_int != '\t' as i32
                                            && c as libc::c_int != '\r' as i32
                                            && c as libc::c_int != '\u{0}' as i32)
                                        {
                                            break;
                                        }
                                        alen = alen.wrapping_add(1);
                                    }
                                    len = http_header_remap_host(
                                        b,
                                        off,
                                        remap_hdrs,
                                        0 as libc::c_int,
                                        alen,
                                    );
                                    e = ((*b).ptr).offset(off as isize).offset(len as isize);
                                }
                            }
                        }
                        _ => {
                            if buffer_eq_icase_ssn(
                                s,
                                b"path\0" as *const u8 as *const libc::c_char,
                                4 as libc::c_int as size_t,
                            ) != 0
                            {
                                if *e as libc::c_int == '"' as i32 {
                                    e = e.offset(1);
                                }
                                if !(*e as libc::c_int != '/' as i32) {
                                    off = e.offset_from((*b).ptr) as libc::c_long as size_t;
                                    len = http_header_remap_urlpath(
                                        b,
                                        off,
                                        remap_hdrs,
                                        0 as libc::c_int,
                                    );
                                    e = ((*b).ptr).offset(off as isize).offset(len as isize);
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        s = e;
    }
}
unsafe extern "C" fn buffer_append_string_backslash_escaped(
    mut b: *mut buffer,
    mut s: *const libc::c_char,
    mut len: size_t,
) {
    let mut j: size_t = 0 as libc::c_int as size_t;
    let p: *mut libc::c_char = buffer_string_prepare_append(
        b,
        len
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(4 as libc::c_int as libc::c_ulong),
    );
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < len {
        let mut c: libc::c_int = *s.offset(i as isize) as libc::c_int;
        if c == '"' as i32 || c == '\\' as i32 || c == 0x7f as libc::c_int
            || c < 0x20 as libc::c_int && c != '\t' as i32
        {
            let fresh1 = j;
            j = j.wrapping_add(1);
            *p.offset(fresh1 as isize) = '\\' as i32 as libc::c_char;
        }
        let fresh2 = j;
        j = j.wrapping_add(1);
        *p.offset(fresh2 as isize) = c as libc::c_char;
        i = i.wrapping_add(1);
    }
    buffer_commit(b, j);
}
unsafe extern "C" fn proxy_set_Forwarded(
    con: *mut connection,
    r: *mut request_st,
    flags: libc::c_uint,
) {
    let mut b: *mut buffer = 0 as *mut buffer;
    let mut efor: *mut buffer = 0 as *mut buffer;
    let mut eproto: *mut buffer = 0 as *mut buffer;
    let mut ehost: *mut buffer = 0 as *mut buffer;
    let mut semicolon: libc::c_int = 0 as libc::c_int;
    if proxy_check_extforward != 0 {
        efor = http_header_env_get(
            r,
            b"_L_EXTFORWARD_ACTUAL_FOR\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        eproto = http_header_env_get(
            r,
            b"_L_EXTFORWARD_ACTUAL_PROTO\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        ehost = http_header_env_get(
            r,
            b"_L_EXTFORWARD_ACTUAL_HOST\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    if flags != 0 {
        b = http_header_request_get(
            r,
            HTTP_HEADER_FORWARDED,
            b"Forwarded\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    if flags != 0 && b.is_null() {
        let mut xff: *const buffer = http_header_request_get(
            r,
            HTTP_HEADER_X_FORWARDED_FOR,
            b"X-Forwarded-For\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        b = http_header_request_set_ptr(
            r,
            HTTP_HEADER_FORWARDED,
            b"Forwarded\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        if !xff.is_null() {
            let mut s: *mut libc::c_char = (*xff).ptr;
            let mut used: size_t = buffer_clen(xff) as size_t;
            let mut i: size_t = 0 as libc::c_int as size_t;
            let mut j: size_t = 0;
            let mut ipv6: size_t = 0;
            while i < used {
                while *s.offset(i as isize) as libc::c_int == ' ' as i32
                    || *s.offset(i as isize) as libc::c_int == '\t' as i32
                    || *s.offset(i as isize) as libc::c_int == ',' as i32
                {
                    i = i.wrapping_add(1);
                }
                if *s.offset(i as isize) as libc::c_int == '\u{0}' as i32 {
                    break;
                }
                j = i;
                loop {
                    i = i.wrapping_add(1);
                    if !(*s.offset(i as isize) as libc::c_int != ' ' as i32
                        && *s.offset(i as isize) as libc::c_int != '\t' as i32
                        && *s.offset(i as isize) as libc::c_int != ',' as i32
                        && *s.offset(i as isize) as libc::c_int != '\u{0}' as i32)
                    {
                        break;
                    }
                }
                ipv6 = (0 as *mut libc::c_void
                    != memchr(
                        s.offset(j as isize) as *const libc::c_void,
                        ':' as i32,
                        i.wrapping_sub(j),
                    )) as libc::c_int as size_t;
                if ipv6 != 0 {
                    buffer_append_string_len(
                        b,
                        b"for=\"[\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    );
                } else {
                    buffer_append_string_len(
                        b,
                        b"for=\"\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    );
                };
                buffer_append_string_backslash_escaped(
                    b,
                    s.offset(j as isize),
                    i.wrapping_sub(j),
                );
                if ipv6 != 0 {
                    buffer_append_string_len(
                        b,
                        b"]\", \0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    );
                } else {
                    buffer_append_string_len(
                        b,
                        b"\", \0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    );
                };
                i = i.wrapping_add(1);
            }
        }
    } else if flags != 0 {
        buffer_append_string_len(
            b,
            b", \0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    if flags & PROXY_FORWARDED_FOR as libc::c_int as libc::c_uint != 0 {
        let mut family: libc::c_int = sock_addr_get_family(&mut (*con).dst_addr);
        buffer_append_string_len(
            b,
            b"for=\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        if !efor.is_null() {
            let mut ipv6_0: libc::c_int = (0 as *mut libc::c_void as *mut libc::c_char
                != strchr((*efor).ptr, ':' as i32)) as libc::c_int;
            if ipv6_0 != 0 {
                buffer_append_string_len(
                    b,
                    b"\"[\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
            } else {
                buffer_append_string_len(
                    b,
                    b"\"\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
            };
            buffer_append_string_backslash_escaped(
                b,
                (*efor).ptr,
                buffer_clen(efor) as size_t,
            );
            if ipv6_0 != 0 {
                buffer_append_string_len(
                    b,
                    b"]\"\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
            } else {
                buffer_append_string_len(
                    b,
                    b"\"\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
            };
        } else if family == 2 as libc::c_int {
            buffer_append_buffer(b, &mut (*con).dst_addr_buf);
        } else if family == 10 as libc::c_int {
            buffer_append_str3(
                b,
                b"\"[\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                (*con).dst_addr_buf.ptr,
                buffer_clen(&mut (*con).dst_addr_buf) as size_t,
                b"]\"\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        } else {
            buffer_append_string_len(
                b,
                b"\"\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            buffer_append_string_backslash_escaped(
                b,
                (*con).dst_addr_buf.ptr,
                buffer_clen(&mut (*con).dst_addr_buf) as size_t,
            );
            buffer_append_string_len(
                b,
                b"\"\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
        semicolon = 1 as libc::c_int;
    }
    if flags & PROXY_FORWARDED_BY as libc::c_int as libc::c_uint != 0 {
        let mut family_0: libc::c_int = sock_addr_get_family(&(*(*con).srv_socket).addr);
        if semicolon != 0 {
            buffer_append_string_len(
                b,
                b";\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
        buffer_append_string_len(
            b,
            b"by=\"\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        if family_0 == 1 as libc::c_int {
            buffer_append_string_backslash_escaped(
                b,
                (*(*(*con).srv_socket).srv_token).ptr,
                buffer_clen((*(*con).srv_socket).srv_token) as size_t,
            );
        } else {
            let mut addr: sock_addr = sock_addr {
                ipv6: sockaddr_in6 {
                    sin6_family: 0,
                    sin6_port: 0,
                    sin6_flowinfo: 0,
                    sin6_addr: in6_addr {
                        __in6_u: C2RustUnnamed {
                            __u6_addr8: [0; 16],
                        },
                    },
                    sin6_scope_id: 0,
                },
            };
            let mut addrlen: socklen_t = ::std::mem::size_of::<sock_addr>()
                as libc::c_ulong as socklen_t;
            if 0 as libc::c_int
                == getsockname(
                    (*con).fd,
                    __SOCKADDR_ARG {
                        __sockaddr__: &mut addr as *mut sock_addr as *mut sockaddr,
                    },
                    &mut addrlen,
                )
            {
                sock_addr_stringify_append_buffer(b, &mut addr);
            }
        }
        buffer_append_string_len(
            b,
            b"\"\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        semicolon = 1 as libc::c_int;
    }
    if flags & PROXY_FORWARDED_PROTO as libc::c_int as libc::c_uint != 0 {
        if semicolon != 0 {
            buffer_append_string_len(
                b,
                b";\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
        if !eproto.is_null() {
            buffer_append_str2(
                b,
                b"proto=\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                (*eproto).ptr,
                buffer_clen(eproto) as size_t,
            );
        } else if (*(*con).srv_socket).is_ssl != 0 {
            buffer_append_string_len(
                b,
                b"proto=https\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        } else {
            buffer_append_string_len(
                b,
                b"proto=http\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
        semicolon = 1 as libc::c_int;
    }
    if flags & PROXY_FORWARDED_HOST as libc::c_int as libc::c_uint != 0 {
        if !ehost.is_null() {
            if semicolon != 0 {
                buffer_append_string_len(
                    b,
                    b";\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
            }
            buffer_append_string_len(
                b,
                b"host=\"\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            buffer_append_string_backslash_escaped(
                b,
                (*ehost).ptr,
                buffer_clen(ehost) as size_t,
            );
            buffer_append_string_len(
                b,
                b"\"\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            semicolon = 1 as libc::c_int;
        } else if !((*r).http_host).is_null() && buffer_is_blank((*r).http_host) == 0 {
            if semicolon != 0 {
                buffer_append_string_len(
                    b,
                    b";\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
            }
            buffer_append_string_len(
                b,
                b"host=\"\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            buffer_append_string_backslash_escaped(
                b,
                (*(*r).http_host).ptr,
                buffer_clen((*r).http_host) as size_t,
            );
            buffer_append_string_len(
                b,
                b"\"\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            semicolon = 1 as libc::c_int;
        }
    }
    if flags & PROXY_FORWARDED_REMOTE_USER as libc::c_int as libc::c_uint != 0 {
        let mut remote_user: *const buffer = http_header_env_get(
            r,
            b"REMOTE_USER\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        if !remote_user.is_null() {
            if semicolon != 0 {
                buffer_append_string_len(
                    b,
                    b";\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
            }
            buffer_append_string_len(
                b,
                b"remote_user=\"\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            buffer_append_string_backslash_escaped(
                b,
                (*remote_user).ptr,
                buffer_clen(remote_user) as size_t,
            );
            buffer_append_string_len(
                b,
                b"\"\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
    }
    b = if !efor.is_null() { efor } else { &mut (*con).dst_addr_buf };
    http_header_request_set(
        r,
        HTTP_HEADER_X_FORWARDED_FOR,
        b"X-Forwarded-For\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
        (*b).ptr,
        buffer_clen(b),
    );
    b = if !ehost.is_null() { ehost } else { (*r).http_host };
    if !b.is_null() && buffer_is_blank(b) == 0 {
        http_header_request_set(
            r,
            HTTP_HEADER_OTHER,
            b"X-Host\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            (*b).ptr,
            buffer_clen(b),
        );
        http_header_request_set(
            r,
            HTTP_HEADER_OTHER,
            b"X-Forwarded-Host\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            (*b).ptr,
            buffer_clen(b),
        );
    }
    b = if !eproto.is_null() { eproto } else { &mut (*r).uri.scheme };
    http_header_request_set(
        r,
        HTTP_HEADER_X_FORWARDED_PROTO,
        b"X-Forwarded-Proto\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
        (*b).ptr,
        buffer_clen(b),
    );
}
unsafe extern "C" fn proxy_stdin_append(mut hctx: *mut gw_handler_ctx) -> handler_t {
    let req_cq: *mut chunkqueue = &mut (*(*hctx).r).reqbody_queue;
    let req_cqlen: off_t = chunkqueue_length(req_cq);
    if req_cqlen != 0 {
        let tb: *mut buffer = (*(*hctx).r).tmp_buf;
        buffer_clear(tb);
        buffer_append_uint_hex_lc(tb, req_cqlen as uintmax_t);
        buffer_append_string_len(
            tb,
            b"\r\n\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        let len: off_t = buffer_clen(tb) as off_t + 2 as libc::c_int as libc::c_long
            + req_cqlen;
        if -(1 as libc::c_int) as libc::c_long != (*hctx).wb_reqlen {
            (*hctx).wb_reqlen
                += if (*hctx).wb_reqlen >= 0 as libc::c_int as libc::c_long {
                    len
                } else {
                    -len
                };
        }
        if chunkqueue_is_empty(&mut (*hctx).wb) != 0
            || (*(*hctx).wb.first).type_0 as libc::c_uint
                == MEM_CHUNK as libc::c_int as libc::c_uint
        {
            chunkqueue_append_mem(&mut (*hctx).wb, (*tb).ptr, buffer_clen(tb) as size_t);
        } else {
            chunkqueue_append_mem_min(
                &mut (*hctx).wb,
                (*tb).ptr,
                buffer_clen(tb) as size_t,
            );
        };
        chunkqueue_steal(&mut (*hctx).wb, req_cq, req_cqlen);
        chunkqueue_append_mem_min(
            &mut (*hctx).wb,
            b"\r\n\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    if (*hctx).wb.bytes_in == (*hctx).wb_reqlen {
        chunkqueue_append_mem(
            &mut (*hctx).wb,
            b"0\r\n\r\n\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        (*hctx).wb_reqlen
            += ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as libc::c_int
                as libc::c_long;
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn proxy_create_env(mut gwhctx: *mut gw_handler_ctx) -> handler_t {
    let mut hctx: *mut handler_ctx = gwhctx as *mut handler_ctx;
    let r: *mut request_st = (*hctx).gw.r;
    let remap_headers: libc::c_int = (!((*hctx).conf.header.urlpaths).is_null()
        || !((*hctx).conf.header.hosts_request).is_null()) as libc::c_int;
    let mut rsz: size_t = ((*r).read_queue.bytes_out - (*hctx).gw.wb.bytes_in) as size_t;
    if rsz >= 65536 as libc::c_int as libc::c_ulong {
        rsz = (*r).rqst_header_len as size_t;
    }
    let b: *mut buffer = chunkqueue_prepend_buffer_open_sz(&mut (*hctx).gw.wb, rsz);
    let m: *const buffer = http_method_buf((*r).http_method);
    buffer_append_str3(
        b,
        (*m).ptr,
        buffer_clen(m) as size_t,
        b" \0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        (*r).target.ptr,
        buffer_clen(&mut (*r).target) as size_t,
    );
    if remap_headers != 0 {
        http_header_remap_uri(
            b,
            (buffer_clen(b)).wrapping_sub(buffer_clen(&mut (*r).target)) as size_t,
            &mut (*hctx).conf.header,
            1 as libc::c_int,
        );
    }
    buffer_append_string_len(
        b,
        if (*hctx).conf.header.force_http10 == 0 {
            b" HTTP/1.1\0" as *const u8 as *const libc::c_char
        } else {
            b" HTTP/1.0\0" as *const u8 as *const libc::c_char
        },
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    if (*hctx).conf.replace_http_host != 0 && buffer_is_blank((*(*hctx).gw.host).id) == 0
    {
        if (*hctx).gw.conf.debug > 1 as libc::c_int {
            log_error(
                (*r).conf.errh,
                b"src/mod_proxy.c\0" as *const u8 as *const libc::c_char,
                864 as libc::c_int as libc::c_uint,
                b"proxy - using \"%s\" as HTTP Host\0" as *const u8
                    as *const libc::c_char,
                (*(*(*hctx).gw.host).id).ptr,
            );
        }
        buffer_append_str2(
            b,
            b"\r\nHost: \0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            (*(*(*hctx).gw.host).id).ptr,
            buffer_clen((*(*hctx).gw.host).id) as size_t,
        );
    } else if !((*r).http_host).is_null() && buffer_is_unset((*r).http_host) == 0 {
        buffer_append_str2(
            b,
            b"\r\nHost: \0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            (*(*r).http_host).ptr,
            buffer_clen((*r).http_host) as size_t,
        );
        if remap_headers != 0 {
            let mut alen: size_t = buffer_clen((*r).http_host) as size_t;
            http_header_remap_host(
                b,
                (buffer_clen(b) as libc::c_ulong).wrapping_sub(alen),
                &mut (*hctx).conf.header,
                1 as libc::c_int,
                alen,
            );
        }
    } else {
        *((*b).ptr)
            .offset(
                ((*b).used).wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
            ) = '0' as i32 as libc::c_char;
    }
    if (*r).reqbody_length > 0 as libc::c_int as libc::c_long
        || 0 as libc::c_int as libc::c_long == (*r).reqbody_length
            && !((*r).http_method as libc::c_int <= HTTP_METHOD_HEAD as libc::c_int)
    {
        let mut vb: *const buffer = http_header_request_get(
            r,
            HTTP_HEADER_CONTENT_LENGTH,
            b"Content-Length\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        if vb.is_null() {
            buffer_append_int(
                http_header_request_set_ptr(
                    r,
                    HTTP_HEADER_CONTENT_LENGTH,
                    b"Content-Length\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                ),
                (*r).reqbody_length,
            );
        }
    } else if -(1 as libc::c_int) as libc::c_long == (*r).reqbody_length
            && (*r).conf.stream_request_body as libc::c_int
                & ((1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int) != 0
        {
        if (*hctx).conf.header.force_http10 as libc::c_long != 0 {
            return http_response_reqbody_read_error(r, 411 as libc::c_int);
        }
        (*hctx)
            .gw
            .stdin_append = Some(
            proxy_stdin_append as unsafe extern "C" fn(*mut gw_handler_ctx) -> handler_t,
        );
        buffer_append_string_len(
            b,
            b"\r\nTransfer-Encoding: chunked\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    proxy_set_Forwarded((*r).con, r, (*hctx).conf.forwarded);
    let mut connhdr: *const buffer = 0 as *const buffer;
    let mut te: *const buffer = 0 as *const buffer;
    let mut upgrade: *const buffer = 0 as *const buffer;
    let mut current_block_43: u64;
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut used: size_t = (*r).rqst_headers.used as size_t;
    while i < used {
        let ds: *const data_string = *((*r).rqst_headers.data).offset(i as isize)
            as *mut data_string;
        match (*ds).ext {
            0 => {
                if ('p' as i32
                    == *((*ds).key.ptr).offset(0 as libc::c_int as isize) as libc::c_int
                        | 0x20 as libc::c_int) as libc::c_int as libc::c_long != 0
                {
                    if buffer_eq_icase_slen(
                        &(*ds).key,
                        b"Proxy-Connection\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    ) != 0
                    {
                        current_block_43 = 1836292691772056875;
                    } else if buffer_eq_icase_slen(
                            &(*ds).key,
                            b"Proxy\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        ) != 0
                        {
                        current_block_43 = 1836292691772056875;
                    } else {
                        current_block_43 = 10380409671385728102;
                    }
                } else {
                    current_block_43 = 10380409671385728102;
                }
            }
            47 => {
                if (*hctx).conf.header.force_http10 != 0
                    || (*r).http_version as libc::c_int
                        == HTTP_VERSION_1_0 as libc::c_int
                {
                    current_block_43 = 1836292691772056875;
                } else if buffer_eq_icase_slen(
                        &(*ds).value,
                        b"trailers\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    ) == 0
                    {
                    current_block_43 = 1836292691772056875;
                } else {
                    te = &(*ds).value;
                    current_block_43 = 10380409671385728102;
                }
            }
            49 => {
                if (*hctx).conf.header.force_http10 != 0
                    || (*r).http_version as libc::c_int
                        != HTTP_VERSION_1_1 as libc::c_int
                {
                    current_block_43 = 1836292691772056875;
                } else if (*hctx).conf.header.upgrade == 0 {
                    current_block_43 = 1836292691772056875;
                } else {
                    if buffer_is_blank(&(*ds).value) == 0 {
                        upgrade = &(*ds).value;
                    }
                    current_block_43 = 10380409671385728102;
                }
            }
            12 => {
                connhdr = &(*ds).value;
                current_block_43 = 1836292691772056875;
            }
            27 | 44 => {
                current_block_43 = 1836292691772056875;
            }
            _ => {
                current_block_43 = 10380409671385728102;
            }
        }
        match current_block_43 {
            10380409671385728102 => {
                let klen: uint32_t = buffer_clen(&(*ds).key);
                let vlen: uint32_t = buffer_clen(&(*ds).value);
                if !(0 as libc::c_int as libc::c_uint == klen
                    || 0 as libc::c_int as libc::c_uint == vlen)
                {
                    let mut s: *mut libc::c_char = buffer_extend(
                        b,
                        klen
                            .wrapping_add(vlen)
                            .wrapping_add(4 as libc::c_int as libc::c_uint) as size_t,
                    );
                    *s.offset(0 as libc::c_int as isize) = '\r' as i32 as libc::c_char;
                    *s.offset(1 as libc::c_int as isize) = '\n' as i32 as libc::c_char;
                    memcpy(
                        s.offset(2 as libc::c_int as isize) as *mut libc::c_void,
                        (*ds).key.ptr as *const libc::c_void,
                        klen as libc::c_ulong,
                    );
                    s = s
                        .offset(
                            (2 as libc::c_int as libc::c_uint).wrapping_add(klen)
                                as isize,
                        );
                    *s.offset(0 as libc::c_int as isize) = ':' as i32 as libc::c_char;
                    *s.offset(1 as libc::c_int as isize) = ' ' as i32 as libc::c_char;
                    memcpy(
                        s.offset(2 as libc::c_int as isize) as *mut libc::c_void,
                        (*ds).value.ptr as *const libc::c_void,
                        vlen as libc::c_ulong,
                    );
                    if !(remap_headers == 0) {
                        match klen {
                            11 => {
                                current_block_43 = 12847541577353527391;
                                match current_block_43 {
                                    7889058985558890564 => {
                                        if (*ds).ext == HTTP_HEADER_CONTENT_LOCATION as libc::c_int
                                        {
                                            current_block_43 = 777662472977924419;
                                        } else {
                                            current_block_43 = 1836292691772056875;
                                        }
                                    }
                                    _ => {
                                        if (*ds).ext == HTTP_HEADER_OTHER as libc::c_int
                                            && buffer_eq_icase_slen(
                                                &(*ds).key,
                                                b"Destination\0" as *const u8 as *const libc::c_char,
                                                (::std::mem::size_of::<[libc::c_char; 12]>()
                                                    as libc::c_ulong as uint32_t)
                                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                            ) != 0
                                        {
                                            current_block_43 = 777662472977924419;
                                        } else {
                                            current_block_43 = 1836292691772056875;
                                        }
                                    }
                                }
                                match current_block_43 {
                                    1836292691772056875 => {}
                                    _ => {
                                        http_header_remap_uri(
                                            b,
                                            (buffer_clen(b)).wrapping_sub(vlen) as size_t,
                                            &mut (*hctx).conf.header,
                                            1 as libc::c_int,
                                        );
                                    }
                                }
                            }
                            16 => {
                                current_block_43 = 7889058985558890564;
                                match current_block_43 {
                                    7889058985558890564 => {
                                        if (*ds).ext == HTTP_HEADER_CONTENT_LOCATION as libc::c_int
                                        {
                                            current_block_43 = 777662472977924419;
                                        } else {
                                            current_block_43 = 1836292691772056875;
                                        }
                                    }
                                    _ => {
                                        if (*ds).ext == HTTP_HEADER_OTHER as libc::c_int
                                            && buffer_eq_icase_slen(
                                                &(*ds).key,
                                                b"Destination\0" as *const u8 as *const libc::c_char,
                                                (::std::mem::size_of::<[libc::c_char; 12]>()
                                                    as libc::c_ulong as uint32_t)
                                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                            ) != 0
                                        {
                                            current_block_43 = 777662472977924419;
                                        } else {
                                            current_block_43 = 1836292691772056875;
                                        }
                                    }
                                }
                                match current_block_43 {
                                    1836292691772056875 => {}
                                    _ => {
                                        http_header_remap_uri(
                                            b,
                                            (buffer_clen(b)).wrapping_sub(vlen) as size_t,
                                            &mut (*hctx).conf.header,
                                            1 as libc::c_int,
                                        );
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
    if !connhdr.is_null() && (*hctx).conf.header.force_http10 == 0
        && (*r).http_version as libc::c_int >= HTTP_VERSION_1_1 as libc::c_int
        && buffer_eq_icase_slen(
            connhdr,
            b"close\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) == 0
    {
        buffer_append_string_len(
            b,
            b"\r\nConnection: close\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        if !te.is_null() {
            buffer_append_string_len(
                b,
                b", te\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
        if !upgrade.is_null() {
            buffer_append_string_len(
                b,
                b", upgrade\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
        buffer_append_string_len(
            b,
            b"\r\n\r\n\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    } else {
        buffer_append_string_len(
            b,
            b"\r\nConnection: close\r\n\r\n\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    (*hctx).gw.wb_reqlen = buffer_clen(b) as off_t;
    chunkqueue_prepend_buffer_commit(&mut (*hctx).gw.wb);
    if (*r).reqbody_length != 0 {
        if (*r).reqbody_length > 0 as libc::c_int as libc::c_long {
            (*hctx).gw.wb_reqlen += (*r).reqbody_length;
        } else {
            (*hctx).gw.wb_reqlen = -(*hctx).gw.wb_reqlen;
        }
        if (*hctx).gw.stdin_append
            == Some(
                proxy_stdin_append
                    as unsafe extern "C" fn(*mut gw_handler_ctx) -> handler_t,
            )
        {
            proxy_stdin_append(&mut (*hctx).gw);
        } else {
            chunkqueue_append_chunkqueue(&mut (*hctx).gw.wb, &mut (*r).reqbody_queue);
        }
    }
    status_counter_inc(
        b"proxy.requests\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    return HANDLER_GO_ON;
}
unsafe extern "C" fn proxy_create_env_connect(
    mut gwhctx: *mut gw_handler_ctx,
) -> handler_t {
    let mut hctx: *mut handler_ctx = gwhctx as *mut handler_ctx;
    let r: *mut request_st = (*hctx).gw.r;
    (*r).http_status = 200 as libc::c_int;
    (*r).resp_body_started = 1 as libc::c_int as libc::c_char;
    gw_set_transparent(&mut (*hctx).gw);
    http_response_upgrade_read_body_unknown(r);
    status_counter_inc(
        b"proxy.requests\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    return HANDLER_GO_ON;
}
unsafe extern "C" fn proxy_response_headers(
    r: *mut request_st,
    mut opts: *mut http_response_opts_t,
) -> handler_t {
    let mut hctx: *mut handler_ctx = (*opts).pdata as *mut handler_ctx;
    let remap_hdrs: *mut http_header_remap_opts = &mut (*hctx).conf.header;
    if (*r).resp_htags & (1 as libc::c_ulong) << HTTP_HEADER_UPGRADE as libc::c_int != 0
    {
        if (*remap_hdrs).upgrade != 0 && (*r).http_status == 101 as libc::c_int {
            gw_set_transparent(&mut (*hctx).gw);
            http_response_upgrade_read_body_unknown(r);
        } else {
            (*r).resp_htags
                &= !((1 as libc::c_ulong) << HTTP_HEADER_UPGRADE as libc::c_int);
        }
    }
    if ((*remap_hdrs).urlpaths).is_null() && ((*remap_hdrs).hosts_response).is_null() {
        return HANDLER_GO_ON;
    }
    let mut vb: *mut buffer = 0 as *mut buffer;
    if (*r).resp_htags & (1 as libc::c_ulong) << HTTP_HEADER_LOCATION as libc::c_int != 0
    {
        vb = http_header_response_get(
            r,
            HTTP_HEADER_LOCATION,
            b"Location\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        if !vb.is_null() {
            http_header_remap_uri(
                vb,
                0 as libc::c_int as size_t,
                remap_hdrs,
                0 as libc::c_int,
            );
        }
    }
    if (*r).resp_htags
        & (1 as libc::c_ulong) << HTTP_HEADER_CONTENT_LOCATION as libc::c_int != 0
    {
        vb = http_header_response_get(
            r,
            HTTP_HEADER_CONTENT_LOCATION,
            b"Content-Location\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        if !vb.is_null() {
            http_header_remap_uri(
                vb,
                0 as libc::c_int as size_t,
                remap_hdrs,
                0 as libc::c_int,
            );
        }
    }
    if (*r).resp_htags & (1 as libc::c_ulong) << HTTP_HEADER_SET_COOKIE as libc::c_int
        != 0
    {
        vb = http_header_response_get(
            r,
            HTTP_HEADER_SET_COOKIE,
            b"Set-Cookie\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        if !vb.is_null() {
            http_header_remap_setcookie(vb, 0 as libc::c_int as size_t, remap_hdrs);
        }
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_proxy_check_extension(
    r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    let mut rc: handler_t = HANDLER_GO_ON;
    if !((*r).handler_module).is_null() {
        return HANDLER_GO_ON;
    }
    mod_proxy_patch_config(r, p);
    if ((*p).conf.gw.exts).is_null() {
        return HANDLER_GO_ON;
    }
    rc = gw_check_extension(
        r,
        p as *mut gw_plugin_data,
        1 as libc::c_int,
        ::std::mem::size_of::<handler_ctx>() as libc::c_ulong,
    );
    if HANDLER_GO_ON as libc::c_int as libc::c_uint != rc as libc::c_uint {
        return rc;
    }
    if (*r).handler_module == (*p).self_0 as *const plugin {
        let mut hctx: *mut handler_ctx = *((*r).plugin_ctx).offset((*p).id as isize)
            as *mut handler_ctx;
        (*hctx)
            .gw
            .create_env = Some(
            proxy_create_env as unsafe extern "C" fn(*mut gw_handler_ctx) -> handler_t,
        );
        (*hctx).gw.response = chunk_buffer_acquire();
        (*hctx).gw.opts.backend = BACKEND_PROXY as libc::c_int;
        (*hctx).gw.opts.pdata = hctx as *mut libc::c_void;
        (*hctx)
            .gw
            .opts
            .headers = Some(
            proxy_response_headers
                as unsafe extern "C" fn(
                    *mut request_st,
                    *mut http_response_opts_t,
                ) -> handler_t,
        );
        (*hctx).conf = (*p).conf;
        (*hctx).conf.header.http_host = (*r).http_host;
        (*hctx).conf.header.upgrade
            &= ((*r).http_version as libc::c_int == HTTP_VERSION_1_1 as libc::c_int)
                as libc::c_int;
        if (*hctx).conf.header.https_remap != 0 {
            (*hctx)
                .conf
                .header
                .https_remap = buffer_eq_slen(
                &mut (*r).uri.scheme,
                b"https\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
        if (*r).http_method as libc::c_int == HTTP_METHOD_CONNECT as libc::c_int {
            if (*hctx).conf.header.connect_method != 0 {
                (*hctx)
                    .gw
                    .create_env = Some(
                    proxy_create_env_connect
                        as unsafe extern "C" fn(*mut gw_handler_ctx) -> handler_t,
                );
            } else {
                (*r).http_status = 405 as libc::c_int;
                (*r).handler_module = 0 as *const plugin;
                return HANDLER_FINISHED;
            }
        }
    }
    return HANDLER_GO_ON;
}
#[no_mangle]
pub unsafe extern "C" fn mod_proxy_plugin_init(mut p: *mut plugin) -> libc::c_int {
    (*p).version = 0x10440 as libc::c_int as size_t;
    (*p).name = b"proxy\0" as *const u8 as *const libc::c_char;
    (*p)
        .init = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
    >(Some(mod_proxy_init as unsafe extern "C" fn() -> *mut libc::c_void));
    (*p).cleanup = Some(mod_proxy_free as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*p)
        .set_defaults = Some(
        mod_proxy_set_defaults
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_request_reset = Some(
        gw_handle_request_reset
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_uri_clean = Some(
        mod_proxy_check_extension
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
                k: b"proxy.server\0" as *const u8 as *const libc::c_char,
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
                k: b"proxy.balance\0" as *const u8 as *const libc::c_char,
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
                k: b"proxy.debug\0" as *const u8 as *const libc::c_char,
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
                k: b"proxy.map-extensions\0" as *const u8 as *const libc::c_char,
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
                k: b"proxy.forwarded\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVANY as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"proxy.header\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVANY as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"proxy.replace-http-host\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong
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
