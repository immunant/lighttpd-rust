use ::libc;
extern "C" {
    pub type pcre2_real_match_data_8;
    pub type lshpack_double_enc_head;
    pub type lshpack_enc_table_entry;
    pub type fdevents;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_str2(
        b: *mut buffer,
        s1: *const libc::c_char,
        len1: size_t,
        s2: *const libc::c_char,
        len2: size_t,
    );
    fn buffer_eq_icase_slen(
        b: *const buffer,
        s: *const libc::c_char,
        slen: size_t,
    ) -> libc::c_int;
    fn hex2int(c: libc::c_uchar) -> libc::c_char;
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn chunkqueue_init(cq: *mut chunkqueue) -> *mut chunkqueue;
    fn chunkqueue_append_mem(cq: *mut chunkqueue, mem: *const libc::c_char, len: size_t);
    fn chunkqueue_append_chunkqueue(cq: *mut chunkqueue, src: *mut chunkqueue);
    fn chunkqueue_append_buffer_open(cq: *mut chunkqueue) -> *mut buffer;
    fn chunkqueue_append_buffer_commit(cq: *mut chunkqueue);
    fn chunkqueue_get_memory(cq: *mut chunkqueue, len: *mut size_t) -> *mut libc::c_char;
    fn chunkqueue_use_memory(cq: *mut chunkqueue, ckpt: *mut chunk, len: size_t);
    fn chunkqueue_mark_written(cq: *mut chunkqueue, len: off_t);
    fn chunkqueue_remove_finished_chunks(cq: *mut chunkqueue);
    fn chunkqueue_steal(dest: *mut chunkqueue, src: *mut chunkqueue, len: off_t);
    fn chunkqueue_steal_with_tempfiles(
        dest: *mut chunkqueue,
        src: *mut chunkqueue,
        len: off_t,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn chunkqueue_compact_mem_offset(cq: *mut chunkqueue);
    fn chunkqueue_compact_mem(cq: *mut chunkqueue, clen: size_t);
    fn chunkqueue_free(cq: *mut chunkqueue);
    fn chunkqueue_reset(cq: *mut chunkqueue);
    fn http_status_append(b: *mut buffer, status: libc::c_int);
    fn http_request_headers_process(
        r: *mut request_st,
        hdrs: *mut libc::c_char,
        hoff: *const libc::c_ushort,
        scheme_port: libc::c_int,
    );
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    static mut log_con_jqueue: *mut connection;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    static mut log_epoch_secs: unix_time64_t;
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
    fn log_error_multiline(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        multiline: *const libc::c_char,
        len: size_t,
        fmt: *const libc::c_char,
        _: ...
    );
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
    fn fdevent_socket_read_discard(
        fd: libc::c_int,
        buf: *mut libc::c_char,
        sz: size_t,
        family: libc::c_int,
        so_type: libc::c_int,
    ) -> ssize_t;
    fn fdevent_ioctl_fionread(
        fd: libc::c_int,
        fdfmt: libc::c_int,
        toread: *mut libc::c_int,
    ) -> libc::c_int;
    fn fdevent_is_tcp_half_closed(fd: libc::c_int) -> libc::c_int;
    fn h2_init_con(
        h2r: *mut request_st,
        con: *mut connection,
        http2_settings: *const buffer,
    );
    fn h2_send_goaway(con: *mut connection, e: request_h2error_t);
    fn h2_want_read(con: *mut connection) -> libc::c_int;
    fn h2_parse_frames(con: *mut connection) -> libc::c_int;
    fn h2_send_100_continue(r: *mut request_st, con: *mut connection);
    fn h2_send_headers(r: *mut request_st, con: *mut connection);
    fn h2_send_cqdata(
        r: *mut request_st,
        con: *mut connection,
        cq: *mut chunkqueue,
        dlen: uint32_t,
    ) -> uint32_t;
    fn h2_send_end_stream(r: *mut request_st, con: *mut connection);
    fn h2_retire_stream(r: *mut request_st, con: *mut connection);
    fn h2_retire_con(h2r: *mut request_st, con: *mut connection);
    fn h2_check_con_upgrade_h2c(r: *mut request_st) -> libc::c_int;
    fn http_header_request_get(
        r: *const request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn http_header_request_unset(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    );
    fn http_header_parse_hoff(
        n: *const libc::c_char,
        clen: uint32_t,
        hoff: *mut libc::c_ushort,
    ) -> uint32_t;
    fn request_init_data(r: *mut request_st, con: *mut connection, srv: *mut server);
    fn request_reset(r: *mut request_st);
    fn request_reset_ex(r: *mut request_st);
    fn request_free_data(r: *mut request_st);
    fn http_response_reqbody_read_error(
        r: *mut request_st,
        http_status: libc::c_int,
    ) -> handler_t;
    fn http_response_write_header(r: *mut request_st);
    fn http_response_handler(r: *mut request_st) -> handler_t;
    fn plugins_call_handle_request_done(r: *mut request_st) -> handler_t;
    fn plugins_call_handle_connection_accept(con: *mut connection) -> handler_t;
    fn plugins_call_handle_connection_shut_wr(con: *mut connection) -> handler_t;
    fn plugins_call_handle_connection_close(con: *mut connection) -> handler_t;
    fn config_cond_cache_reset(r: *mut request_st);
    fn sock_addr_cache_inet_ntop_copy_buffer(
        b: *mut buffer,
        saddr: *const sock_addr,
    ) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn __errno_location() -> *mut libc::c_int;
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
pub type __clockid_t = libc::c_int;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type clockid_t = __clockid_t;
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
pub type C2RustUnnamed_6 = libc::c_uint;
pub const SHUT_RDWR: C2RustUnnamed_6 = 2;
pub const SHUT_WR: C2RustUnnamed_6 = 1;
pub const SHUT_RD: C2RustUnnamed_6 = 0;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_7 = 263;
pub const IPPROTO_MPTCP: C2RustUnnamed_7 = 262;
pub const IPPROTO_RAW: C2RustUnnamed_7 = 255;
pub const IPPROTO_ETHERNET: C2RustUnnamed_7 = 143;
pub const IPPROTO_MPLS: C2RustUnnamed_7 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_7 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_7 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_7 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_7 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_7 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_7 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_7 = 92;
pub const IPPROTO_AH: C2RustUnnamed_7 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_7 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_7 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_7 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_7 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_7 = 33;
pub const IPPROTO_TP: C2RustUnnamed_7 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_7 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_7 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_7 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_7 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_7 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_7 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_7 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_7 = 1;
pub const IPPROTO_IP: C2RustUnnamed_7 = 0;
pub type request_h2error_t = libc::c_uint;
pub const H2_E_HTTP_1_1_REQUIRED: request_h2error_t = 13;
pub const H2_E_INADEQUATE_SECURITY: request_h2error_t = 12;
pub const H2_E_ENHANCE_YOUR_CALM: request_h2error_t = 11;
pub const H2_E_CONNECT_ERROR: request_h2error_t = 10;
pub const H2_E_COMPRESSION_ERROR: request_h2error_t = 9;
pub const H2_E_CANCEL: request_h2error_t = 8;
pub const H2_E_REFUSED_STREAM: request_h2error_t = 7;
pub const H2_E_FRAME_SIZE_ERROR: request_h2error_t = 6;
pub const H2_E_STREAM_CLOSED: request_h2error_t = 5;
pub const H2_E_SETTINGS_TIMEOUT: request_h2error_t = 4;
pub const H2_E_FLOW_CONTROL_ERROR: request_h2error_t = 3;
pub const H2_E_INTERNAL_ERROR: request_h2error_t = 2;
pub const H2_E_PROTOCOL_ERROR: request_h2error_t = 1;
pub const H2_E_NO_ERROR: request_h2error_t = 0;
pub const COMP_HTTP_REMOTE_IP: C2RustUnnamed_9 = 8;
pub const COMP_SERVER_SOCKET: C2RustUnnamed_9 = 1;
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
pub const H2_STATE_HALF_CLOSED_REMOTE: C2RustUnnamed_8 = 5;
pub type C2RustUnnamed_8 = libc::c_uint;
pub const H2_STATE_CLOSED: C2RustUnnamed_8 = 6;
pub const H2_STATE_HALF_CLOSED_LOCAL: C2RustUnnamed_8 = 4;
pub const H2_STATE_OPEN: C2RustUnnamed_8 = 3;
pub const H2_STATE_RESERVED_REMOTE: C2RustUnnamed_8 = 2;
pub const H2_STATE_RESERVED_LOCAL: C2RustUnnamed_8 = 1;
pub const H2_STATE_IDLE: C2RustUnnamed_8 = 0;
pub type C2RustUnnamed_9 = libc::c_uint;
pub const COMP_LAST_ELEMENT: C2RustUnnamed_9 = 13;
pub const COMP_HTTP_REQUEST_HEADER: C2RustUnnamed_9 = 12;
pub const COMP_HTTP_REQUEST_METHOD: C2RustUnnamed_9 = 11;
pub const COMP_HTTP_SCHEME: C2RustUnnamed_9 = 10;
pub const COMP_HTTP_QUERY_STRING: C2RustUnnamed_9 = 9;
pub const COMP_HTTP_COOKIE: C2RustUnnamed_9 = 7;
pub const COMP_HTTP_LANGUAGE: C2RustUnnamed_9 = 6;
pub const COMP_HTTP_USER_AGENT: C2RustUnnamed_9 = 5;
pub const COMP_HTTP_REFERER: C2RustUnnamed_9 = 4;
pub const COMP_HTTP_HOST: C2RustUnnamed_9 = 3;
pub const COMP_HTTP_URL: C2RustUnnamed_9 = 2;
pub const COMP_UNSET: C2RustUnnamed_9 = 0;
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
unsafe extern "C" fn chunkqueue_is_empty(mut cq: *const chunkqueue) -> libc::c_int {
    return (0 as *mut libc::c_void as *mut chunk == (*cq).first) as libc::c_int;
}
#[inline]
unsafe extern "C" fn connection_jq_append(con: *mut connection) {
    if ((*con).jqnext).is_null() {
        (*con).jqnext = log_con_jqueue;
        log_con_jqueue = con;
    }
}
#[inline]
unsafe extern "C" fn sock_addr_get_family(mut saddr: *const sock_addr) -> libc::c_int {
    return (*saddr).plain.sa_family as libc::c_int;
}
#[cold]
unsafe extern "C" fn connection_set_state_error(
    r: *mut request_st,
    state: request_state_t,
) {
    (*r).state = state;
}
unsafe extern "C" fn connections_get_new_connection(
    mut srv: *mut server,
) -> *mut connection {
    let mut con: *mut connection = 0 as *mut connection;
    (*srv).lim_conns = ((*srv).lim_conns).wrapping_sub(1);
    if !((*srv).conns_pool).is_null() {
        con = (*srv).conns_pool;
        (*srv).conns_pool = (*con).next;
    } else {
        con = connection_init(srv);
        connection_reset(con);
    }
    (*con).next = (*srv).conns;
    if !((*con).next).is_null() {
        (*(*con).next).prev = con;
    }
    (*srv).conns = con;
    return (*srv).conns;
}
unsafe extern "C" fn connection_del(mut srv: *mut server, mut con: *mut connection) {
    if !((*con).next).is_null() {
        (*(*con).next).prev = (*con).prev;
    }
    if !((*con).prev).is_null() {
        (*(*con).prev).next = (*con).next;
    } else {
        (*srv).conns = (*con).next;
    }
    (*con).prev = 0 as *mut connection;
    (*con).next = (*srv).conns_pool;
    (*srv).conns_pool = con;
    (*srv).lim_conns = ((*srv).lim_conns).wrapping_add(1);
}
unsafe extern "C" fn connection_close(mut con: *mut connection) {
    if (*con).fd < 0 as libc::c_int {
        (*con).fd = -(*con).fd;
    }
    plugins_call_handle_connection_close(con);
    let srv: *mut server = (*con).srv;
    let r: *mut request_st = &mut (*con).request;
    request_reset_ex(r);
    (*r).state = CON_STATE_CONNECT;
    chunkqueue_reset((*con).read_queue);
    (*con).request_count = 0 as libc::c_int as uint32_t;
    (*con).is_ssl_sock = 0 as libc::c_int as libc::c_char;
    (*con).revents_err = 0 as libc::c_int as uint16_t;
    fdevent_fdnode_event_del((*srv).ev, (*con).fdn);
    fdevent_unregister((*srv).ev, (*con).fd);
    (*con).fdn = 0 as *mut fdnode;
    if 0 as libc::c_int == close((*con).fd) {
        (*srv).cur_fds -= 1;
    } else {
        log_perror(
            (*r).conf.errh,
            b"src/connections.c\0" as *const u8 as *const libc::c_char,
            101 as libc::c_int as libc::c_uint,
            b"(warning) close: %d\0" as *const u8 as *const libc::c_char,
            (*con).fd,
        );
    }
    if (*r).conf.log_state_handling != 0 {
        log_error(
            (*r).conf.errh,
            b"src/connections.c\0" as *const u8 as *const libc::c_char,
            105 as libc::c_int as libc::c_uint,
            b"connection closed for fd %d\0" as *const u8 as *const libc::c_char,
            (*con).fd,
        );
    }
    (*con).fd = -(1 as libc::c_int);
    connection_del(srv, con);
}
unsafe extern "C" fn connection_read_for_eos_plain(con: *mut connection) {
    let mut len: ssize_t = 0;
    let type_0: libc::c_int = sock_addr_get_family(&mut (*con).dst_addr);
    let mut buf: [libc::c_char; 16384] = [0; 16384];
    loop {
        len = fdevent_socket_read_discard(
            (*con).fd,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong,
            type_0,
            SOCK_STREAM as libc::c_int,
        );
        if !(len > 0 as libc::c_int as libc::c_long
            || len < 0 as libc::c_int as libc::c_long
                && *__errno_location() == 4 as libc::c_int)
        {
            break;
        }
    }
    if len < 0 as libc::c_int as libc::c_long && *__errno_location() == 11 as libc::c_int
    {
        return;
    }
    (*con)
        .close_timeout_ts = log_monotonic_secs
        - (5 as libc::c_int + 1 as libc::c_int) as libc::c_long;
}
unsafe extern "C" fn connection_read_for_eos_ssl(con: *mut connection) {
    if ((*con).network_read)
        .expect(
            "non-null function pointer",
        )(con, (*con).read_queue, (256 as libc::c_int * 1024 as libc::c_int) as off_t)
        < 0 as libc::c_int
    {
        (*con)
            .close_timeout_ts = log_monotonic_secs
            - (5 as libc::c_int + 1 as libc::c_int) as libc::c_long;
    }
    chunkqueue_reset((*con).read_queue);
}
unsafe extern "C" fn connection_read_for_eos(con: *mut connection) {
    if (*con).is_ssl_sock == 0 {
        connection_read_for_eos_plain(con);
    } else {
        connection_read_for_eos_ssl(con);
    };
}
unsafe extern "C" fn connection_handle_close_state(mut con: *mut connection) {
    connection_read_for_eos(con);
    if log_monotonic_secs - (*con).close_timeout_ts > 5 as libc::c_int as libc::c_long {
        connection_close(con);
    }
}
unsafe extern "C" fn connection_handle_shutdown(mut con: *mut connection) {
    plugins_call_handle_connection_shut_wr(con);
    connection_reset(con);
    (*(*con).srv).con_closed += 1;
    if (*con).fd >= 0 as libc::c_int
        && ((*con).is_ssl_sock as libc::c_int != 0
            || 0 as libc::c_int == shutdown((*con).fd, SHUT_WR as libc::c_int))
    {
        (*con).close_timeout_ts = log_monotonic_secs;
        let r: *mut request_st = &mut (*con).request;
        (*r).state = CON_STATE_CLOSE;
        if (*r).conf.log_state_handling != 0 {
            log_error(
                (*r).conf.errh,
                b"src/connections.c\0" as *const u8 as *const libc::c_char,
                170 as libc::c_int as libc::c_uint,
                b"shutdown for fd %d\0" as *const u8 as *const libc::c_char,
                (*con).fd,
            );
        }
    } else {
        connection_close(con);
    };
}
unsafe extern "C" fn connection_handle_response_end_state(
    r: *mut request_st,
    con: *mut connection,
) {
    if (*r).http_version as libc::c_int > HTTP_VERSION_1_1 as libc::c_int {
        h2_retire_con(r, con);
        (*r).keep_alive = 0 as libc::c_int as int8_t;
        (*r).http_status = 100 as libc::c_int;
    }
    if (*r).http_status != 0 {
        plugins_call_handle_request_done(r);
    }
    if (*r).state as libc::c_uint != CON_STATE_ERROR as libc::c_int as libc::c_uint {
        (*(*con).srv).con_written += 1;
    }
    if (*r).reqbody_length != (*r).reqbody_queue.bytes_in
        || (*r).state as libc::c_uint == CON_STATE_ERROR as libc::c_int as libc::c_uint
    {
        (*r).keep_alive = 0 as libc::c_int as int8_t;
        if &mut (*r).write_queue as *mut chunkqueue != (*con).write_queue {
            chunkqueue_free((*con).write_queue);
            (*con).write_queue = &mut (*r).write_queue;
        }
    }
    if (*r).keep_alive as libc::c_int > 0 as libc::c_int {
        request_reset(r);
        (*con).is_readable = 1 as libc::c_int as libc::c_schar;
        (*r).bytes_read_ckpt = (*con).bytes_read;
        (*r).bytes_written_ckpt = (*con).bytes_written;
        (*r).state = CON_STATE_REQUEST_START;
    } else {
        connection_handle_shutdown(con);
    };
}
unsafe extern "C" fn connection_write_throttled(
    con: *const connection,
    mut max_bytes: off_t,
) -> off_t {
    let rconf: *const request_config = &(*con).request.conf;
    if 0 as libc::c_int as libc::c_uint == (*rconf).global_bytes_per_second
        && 0 as libc::c_int as libc::c_uint == (*rconf).bytes_per_second
    {
        return max_bytes;
    }
    if (*rconf).global_bytes_per_second != 0 {
        let mut limit: off_t = (*rconf).global_bytes_per_second as off_t
            - *(*rconf).global_bytes_per_second_cnt_ptr;
        if max_bytes > limit {
            max_bytes = limit;
        }
    }
    if (*rconf).bytes_per_second != 0 {
        let mut limit_0: off_t = (*rconf).bytes_per_second as off_t
            - (*con).bytes_written_cur_second;
        if max_bytes > limit_0 {
            max_bytes = limit_0;
        }
    }
    return if max_bytes > 0 as libc::c_int as libc::c_long {
        max_bytes
    } else {
        0 as libc::c_int as libc::c_long
    };
}
unsafe extern "C" fn connection_write_throttle(
    con: *mut connection,
    mut max_bytes: off_t,
) -> off_t {
    max_bytes = connection_write_throttled(con, max_bytes);
    if 0 as libc::c_int as libc::c_long == max_bytes {
        (*con).traffic_limit_reached = 1 as libc::c_int as libc::c_char;
    }
    return max_bytes;
}
unsafe extern "C" fn connection_write_chunkqueue(
    con: *mut connection,
    cq: *mut chunkqueue,
    mut max_bytes: off_t,
) -> libc::c_int {
    (*con).write_request_ts = log_monotonic_secs;
    max_bytes = connection_write_throttle(con, max_bytes);
    if 0 as libc::c_int as libc::c_long == max_bytes {
        return 1 as libc::c_int;
    }
    let mut written: off_t = (*cq).bytes_out;
    let mut ret: libc::c_int = 0;
    let mut corked: libc::c_int = 0 as libc::c_int;
    if !((*(*cq).first).next).is_null()
        && (*(*cq).first).type_0 as libc::c_uint
            == MEM_CHUNK as libc::c_int as libc::c_uint
    {
        let mut c: *const chunk = (*cq).first;
        loop {
            c = (*c).next;
            if !(!c.is_null()
                && (*c).type_0 as libc::c_uint
                    == MEM_CHUNK as libc::c_int as libc::c_uint)
            {
                break;
            }
        }
        if !c.is_null()
            || max_bytes > 16384 as libc::c_int as libc::c_long
                && (*con).is_ssl_sock as libc::c_int != 0
        {
            let sa_family: libc::c_int = sock_addr_get_family(
                &(*(*con).srv_socket).addr,
            );
            if sa_family == 2 as libc::c_int || sa_family == 10 as libc::c_int {
                corked = 1 as libc::c_int;
                setsockopt(
                    (*con).fd,
                    IPPROTO_TCP as libc::c_int,
                    3 as libc::c_int,
                    &mut corked as *mut libc::c_int as *const libc::c_void,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
                );
            }
        }
    }
    ret = ((*con).network_write).expect("non-null function pointer")(con, cq, max_bytes);
    if ret >= 0 as libc::c_int {
        ret = if chunkqueue_is_empty(cq) != 0 {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        };
    }
    if corked != 0 {
        corked = 0 as libc::c_int;
        setsockopt(
            (*con).fd,
            IPPROTO_TCP as libc::c_int,
            3 as libc::c_int,
            &mut corked as *mut libc::c_int as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
        );
    }
    written = (*cq).bytes_out - written;
    (*con).bytes_written += written;
    (*con).bytes_written_cur_second += written;
    let r: *mut request_st = &mut (*con).request;
    if !((*r).conf.global_bytes_per_second_cnt_ptr).is_null() {
        *(*r).conf.global_bytes_per_second_cnt_ptr += written;
    }
    return ret;
}
unsafe extern "C" fn connection_write_1xx_info(
    r: *mut request_st,
    con: *mut connection,
) -> libc::c_int {
    let cq: *mut chunkqueue = (*con).write_queue;
    let mut written: off_t = (*cq).bytes_out;
    let mut rc: libc::c_int = ((*con).network_write)
        .expect(
            "non-null function pointer",
        )(con, cq, (256 as libc::c_int * 1024 as libc::c_int) as off_t);
    written = (*cq).bytes_out - written;
    (*con).bytes_written += written;
    (*con).bytes_written_cur_second += written;
    if !((*r).conf.global_bytes_per_second_cnt_ptr).is_null() {
        *(*r).conf.global_bytes_per_second_cnt_ptr += written;
    }
    if rc < 0 as libc::c_int {
        connection_set_state_error(r, CON_STATE_ERROR);
        return 0 as libc::c_int;
    }
    if chunkqueue_is_empty(cq) == 0 {
        (*con).is_writable = 0 as libc::c_int as libc::c_schar;
        if cq == &mut (*r).write_queue as *mut chunkqueue {
            (*con).write_queue = chunkqueue_init(0 as *mut chunkqueue);
            chunkqueue_append_chunkqueue((*con).write_queue, cq);
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn connection_send_1xx(
    r: *mut request_st,
    con: *mut connection,
) -> libc::c_int {
    let cq: *mut chunkqueue = (*con).write_queue;
    let b: *mut buffer = chunkqueue_append_buffer_open(cq);
    buffer_copy_string_len(
        b,
        b"HTTP/1.1 \0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    http_status_append(b, (*r).http_status);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*r).resp_headers.used {
        let ds: *const data_string = *((*r).resp_headers.data).offset(i as isize)
            as *mut data_string;
        let klen: uint32_t = buffer_clen(&(*ds).key);
        let vlen: uint32_t = buffer_clen(&(*ds).value);
        if !(0 as libc::c_int as libc::c_uint == klen
            || 0 as libc::c_int as libc::c_uint == vlen)
        {
            buffer_append_str2(
                b,
                b"\r\n\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                (*ds).key.ptr,
                klen as size_t,
            );
            buffer_append_str2(
                b,
                b": \0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                (*ds).value.ptr,
                vlen as size_t,
            );
        }
        i = i.wrapping_add(1);
    }
    buffer_append_string_len(
        b,
        b"\r\n\r\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    chunkqueue_append_buffer_commit(cq);
    if (*con).traffic_limit_reached != 0 {
        return 1 as libc::c_int;
    }
    return connection_write_1xx_info(r, con);
}
unsafe extern "C" fn connection_write_100_continue(
    r: *mut request_st,
    con: *mut connection,
) -> libc::c_int {
    static mut http_100_continue: [libc::c_char; 26] = unsafe {
        *::std::mem::transmute::<
            &[u8; 26],
            &[libc::c_char; 26],
        >(b"HTTP/1.1 100 Continue\r\n\r\n\0")
    };
    if (*con).traffic_limit_reached != 0 {
        return 1 as libc::c_int;
    }
    let cq: *mut chunkqueue = (*con).write_queue;
    chunkqueue_append_mem(
        cq,
        http_100_continue.as_ptr(),
        (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    return connection_write_1xx_info(r, con);
}
unsafe extern "C" fn connection_handle_write(
    r: *mut request_st,
    con: *mut connection,
) -> libc::c_int {
    if (*con).is_writable as libc::c_int <= 0 as libc::c_int {
        return CON_STATE_WRITE as libc::c_int;
    }
    let mut rc: libc::c_int = connection_write_chunkqueue(
        con,
        (*con).write_queue,
        (256 as libc::c_int * 1024 as libc::c_int) as off_t,
    );
    let mut current_block_11: u64;
    match rc {
        0 => {
            if (*r).resp_body_finished != 0 {
                (*r).state = CON_STATE_RESPONSE_END;
                return CON_STATE_RESPONSE_END as libc::c_int;
            }
            current_block_11 = 10599921512955367680;
        }
        -1 => {
            log_error(
                (*r).conf.errh,
                b"src/connections.c\0" as *const u8 as *const libc::c_char,
                437 as libc::c_int as libc::c_uint,
                b"connection closed: write failed on fd %d\0" as *const u8
                    as *const libc::c_char,
                (*con).fd,
            );
            current_block_11 = 7817030710690106619;
        }
        -2 => {
            current_block_11 = 7817030710690106619;
        }
        1 => {
            if (*(*con).write_queue).bytes_out != 0 {
                (*con).is_writable = 0 as libc::c_int as libc::c_schar;
            }
            current_block_11 = 10599921512955367680;
        }
        _ => {
            current_block_11 = 10599921512955367680;
        }
    }
    match current_block_11 {
        10599921512955367680 => {}
        _ => {
            connection_set_state_error(r, CON_STATE_ERROR);
            return CON_STATE_ERROR as libc::c_int;
        }
    }
    return CON_STATE_WRITE as libc::c_int;
}
unsafe extern "C" fn connection_handle_write_state(
    r: *mut request_st,
    con: *mut connection,
) -> libc::c_int {
    loop {
        if chunkqueue_is_empty(&mut (*r).write_queue) == 0 {
            if (*r).http_version as libc::c_int <= HTTP_VERSION_1_1 as libc::c_int {
                let mut rc: libc::c_int = connection_handle_write(r, con);
                if rc != CON_STATE_WRITE as libc::c_int {
                    return rc;
                }
            }
        } else if (*r).resp_body_finished != 0 {
            (*r).state = CON_STATE_RESPONSE_END;
            return CON_STATE_RESPONSE_END as libc::c_int;
        }
        if !((*r).handler_module).is_null() && (*r).resp_body_finished == 0 {
            let p: *const plugin = (*r).handler_module;
            let mut rc_0: libc::c_int = ((*p).handle_subrequest)
                .expect("non-null function pointer")(r, (*p).data) as libc::c_int;
            's_95: {
                match rc_0 {
                    3 | 1 | 0 => {
                        break 's_95;
                    }
                    4 => {}
                    2 | _ => {
                        log_error(
                            (*r).conf.errh,
                            b"src/connections.c\0" as *const u8 as *const libc::c_char,
                            479 as libc::c_int as libc::c_uint,
                            b"unexpected subrequest handler ret-value: %d %d\0"
                                as *const u8 as *const libc::c_char,
                            (*con).fd,
                            rc_0,
                        );
                    }
                }
                connection_set_state_error(r, CON_STATE_ERROR);
                return CON_STATE_ERROR as libc::c_int;
            }
        }
        if !((*r).http_version as libc::c_int <= HTTP_VERSION_1_1 as libc::c_int
            && (if chunkqueue_is_empty(&mut (*r).write_queue) == 0 {
                ((*con).is_writable as libc::c_int > 0 as libc::c_int
                    && 0 as libc::c_int == (*con).traffic_limit_reached as libc::c_int)
                    as libc::c_int
            } else {
                (*r).resp_body_finished as libc::c_int
            }) != 0)
        {
            break;
        }
    }
    return CON_STATE_WRITE as libc::c_int;
}
#[cold]
unsafe extern "C" fn connection_init(mut srv: *mut server) -> *mut connection {
    let con: *mut connection = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<connection>() as libc::c_ulong,
    ) as *mut connection;
    if con.is_null() {
        ck_assert_failed(
            b"src/connections.c\0" as *const u8 as *const libc::c_char,
            500 as libc::c_int as libc::c_uint,
            b"((void*)0) != con\0" as *const u8 as *const libc::c_char,
        );
    }
    (*con).srv = srv;
    (*con).plugin_slots = (*srv).plugin_slots;
    (*con).config_data_base = (*srv).config_data_base;
    let r: *mut request_st = &mut (*con).request;
    request_init_data(r, con, srv);
    (*con).write_queue = &mut (*r).write_queue;
    (*con).read_queue = &mut (*r).read_queue;
    (*con)
        .plugin_ctx = calloc(
        1 as libc::c_int as libc::c_ulong,
        (((*srv).plugins.used).wrapping_add(1 as libc::c_int as libc::c_uint)
            as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
    ) as *mut *mut libc::c_void;
    if ((*con).plugin_ctx).is_null() {
        ck_assert_failed(
            b"src/connections.c\0" as *const u8 as *const libc::c_char,
            513 as libc::c_int as libc::c_uint,
            b"((void*)0) != con->plugin_ctx\0" as *const u8 as *const libc::c_char,
        );
    }
    return con;
}
unsafe extern "C" fn connection_free(con: *mut connection) {
    let r: *mut request_st = &mut (*con).request;
    connection_reset(con);
    if (*con).write_queue != &mut (*r).write_queue as *mut chunkqueue {
        chunkqueue_free((*con).write_queue);
    }
    if (*con).read_queue != &mut (*r).read_queue as *mut chunkqueue {
        chunkqueue_free((*con).read_queue);
    }
    request_free_data(r);
    free((*con).plugin_ctx as *mut libc::c_void);
    free((*con).dst_addr_buf.ptr as *mut libc::c_void);
    free(con as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn connections_pool_clear(srv: *mut server) {
    let mut con: *mut connection = 0 as *mut connection;
    loop {
        con = (*srv).conns_pool;
        if con.is_null() {
            break;
        }
        (*srv).conns_pool = (*con).next;
        connection_free(con);
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn connections_free(mut srv: *mut server) {
    connections_pool_clear(srv);
    let mut con: *mut connection = 0 as *mut connection;
    loop {
        con = (*srv).conns;
        if con.is_null() {
            break;
        }
        (*srv).conns = (*con).next;
        connection_free(con);
    };
}
unsafe extern "C" fn connection_reset(mut con: *mut connection) {
    let r: *mut request_st = &mut (*con).request;
    request_reset(r);
    (*r).bytes_read_ckpt = 0 as libc::c_int as off_t;
    (*r).bytes_written_ckpt = 0 as libc::c_int as off_t;
    (*con).is_readable = 1 as libc::c_int as libc::c_schar;
    (*con).bytes_written = 0 as libc::c_int as off_t;
    (*con).bytes_written_cur_second = 0 as libc::c_int as off_t;
    (*con).bytes_read = 0 as libc::c_int as off_t;
}
#[cold]
unsafe extern "C" fn connection_discard_blank_line(
    cq: *mut chunkqueue,
    mut header_len: uint32_t,
) -> *mut chunk {
    chunkqueue_mark_written(cq, header_len as off_t);
    return (*cq).first;
}
unsafe extern "C" fn connection_read_header_more(
    mut con: *mut connection,
    mut cq: *mut chunkqueue,
    mut c: *mut chunk,
    olen: size_t,
) -> *mut chunk {
    if (c.is_null() || ((*c).next).is_null())
        && (*con).is_readable as libc::c_int > 0 as libc::c_int
    {
        (*con).read_idle_ts = log_monotonic_secs;
        if 0 as libc::c_int
            != ((*con).network_read)
                .expect(
                    "non-null function pointer",
                )(con, cq, (256 as libc::c_int * 1024 as libc::c_int) as off_t)
        {
            let r: *mut request_st = &mut (*con).request;
            connection_set_state_error(r, CON_STATE_ERROR);
        }
        let r_0: *mut request_st = &mut (*con).request;
        if (*r_0).http_version as libc::c_int == HTTP_VERSION_2 as libc::c_int {
            return 0 as *mut chunk;
        }
    }
    if (*cq).first != (*cq).last && 0 as libc::c_int as libc::c_ulong != olen {
        let clen: size_t = chunkqueue_length(cq) as size_t;
        let mut block: size_t = olen
            .wrapping_add((16384 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
            & !(16384 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
        block = (block as libc::c_ulong)
            .wrapping_add(
                (if block.wrapping_sub(olen) > 1024 as libc::c_int as libc::c_ulong {
                    0 as libc::c_int
                } else {
                    16384 as libc::c_int
                }) as libc::c_ulong,
            ) as size_t as size_t;
        chunkqueue_compact_mem(cq, if block > clen { clen } else { block });
    }
    c = (*cq).first;
    return if !c.is_null()
        && ((*c).offset as size_t).wrapping_add(olen)
            < buffer_clen((*c).mem) as libc::c_ulong
    {
        c
    } else {
        0 as *mut chunk
    };
}
#[cold]
unsafe extern "C" fn connection_transition_h2(
    h2r: *mut request_st,
    con: *mut connection,
) {
    buffer_copy_string_len(
        &mut (*h2r).target,
        b"*\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_copy_string_len(
        &mut (*h2r).target_orig,
        b"*\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_copy_string_len(
        &mut (*h2r).uri.path,
        b"*\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    (*h2r).http_method = HTTP_METHOD_PRI;
    (*h2r).reqbody_length = -(1 as libc::c_int) as off_t;
    (*h2r)
        .conf
        .stream_request_body = ((*h2r).conf.stream_request_body as libc::c_int
        | (1 as libc::c_int) << 15 as libc::c_int) as libc::c_ushort;
    if (*h2r).state as libc::c_uint != CON_STATE_ERROR as libc::c_int as libc::c_uint {
        (*h2r).state = CON_STATE_WRITE;
    }
    if ((*con).h2).is_null() {
        h2_init_con(h2r, con, 0 as *const buffer);
    }
}
#[inline(never)]
unsafe extern "C" fn connection_handle_read_state(con: *mut connection) -> libc::c_int {
    let cq: *mut chunkqueue = (*con).read_queue;
    let mut c: *mut chunk = (*cq).first;
    let mut clen: uint32_t = 0 as libc::c_int as uint32_t;
    let mut header_len: uint32_t = 0 as libc::c_int as uint32_t;
    let r: *mut request_st = &mut (*con).request;
    let mut keepalive_request_start: uint8_t = 0 as libc::c_int as uint8_t;
    let mut pipelined_request_start: uint8_t = 0 as libc::c_int as uint8_t;
    let mut discard_blank: uint8_t = 0 as libc::c_int as uint8_t;
    let mut hoff: [libc::c_ushort; 8192] = [0; 8192];
    if (*con).request_count > 1 as libc::c_int as libc::c_uint {
        discard_blank = 1 as libc::c_int as uint8_t;
        if (*con).bytes_read == (*r).bytes_read_ckpt {
            keepalive_request_start = 1 as libc::c_int as uint8_t;
            if !c.is_null() {
                pipelined_request_start = 1 as libc::c_int as uint8_t;
                (*con).is_readable = 1 as libc::c_int as libc::c_schar;
            }
        }
    }
    let mut current_block_30: u64;
    loop {
        if !c.is_null() {
            clen = (buffer_clen((*c).mem) as libc::c_long - (*c).offset) as uint32_t;
            if !(0 as libc::c_int as libc::c_uint == clen) {
                if ((*c).offset
                    > (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                        as libc::c_long) as libc::c_int as libc::c_long != 0
                {
                    chunkqueue_compact_mem_offset(cq);
                }
                hoff[0 as libc::c_int as usize] = 1 as libc::c_int as libc::c_ushort;
                hoff[1 as libc::c_int as usize] = (*c).offset as libc::c_ushort;
                header_len = http_header_parse_hoff(
                    ((*(*c).mem).ptr).offset((*c).offset as isize),
                    clen,
                    hoff.as_mut_ptr(),
                );
                let max_request_field_size: uint32_t = (*r).conf.max_request_field_size;
                if (if header_len != 0 { header_len } else { clen })
                    > max_request_field_size
                    || hoff[0 as libc::c_int as usize] as libc::c_ulong
                        >= (::std::mem::size_of::<[libc::c_ushort; 8192]>()
                            as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong,
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                {
                    log_error(
                        (*r).conf.errh,
                        b"src/connections.c\0" as *const u8 as *const libc::c_char,
                        689 as libc::c_int as libc::c_uint,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        b"oversized request-header -> sending Status 431\0" as *const u8
                            as *const libc::c_char,
                    );
                    (*r).http_status = 431 as libc::c_int;
                    (*r).keep_alive = 0 as libc::c_int as int8_t;
                    return 1 as libc::c_int;
                }
                if (0 as libc::c_int as libc::c_uint != header_len) as libc::c_int
                    as libc::c_long != 0
                {
                    if (hoff[0 as libc::c_int as usize] as libc::c_int
                        > 1 as libc::c_int) as libc::c_int as libc::c_long != 0
                    {
                        break;
                    }
                    if discard_blank != 0 {
                        if header_len == clen {
                            current_block_30 = 17860125682698302841;
                        } else {
                            let ch: libc::c_int = *((*(*c).mem).ptr)
                                .offset(((*c).offset + header_len as libc::c_long) as isize)
                                as libc::c_int;
                            if ch != '\r' as i32 && ch != '\n' as i32 {
                                discard_blank = 0 as libc::c_int as uint8_t;
                                clen = 0 as libc::c_int as uint32_t;
                                c = connection_discard_blank_line(cq, header_len);
                                current_block_30 = 17860125682698302841;
                            } else {
                                current_block_30 = 3123434771885419771;
                            }
                        }
                    } else {
                        current_block_30 = 3123434771885419771;
                    }
                } else {
                    current_block_30 = 3123434771885419771;
                }
                match current_block_30 {
                    17860125682698302841 => {}
                    _ => {
                        if (*((*(*c).mem).ptr as *mut libc::c_uchar)
                            .offset((*c).offset as isize) as libc::c_int)
                            < 32 as libc::c_int
                        {
                            log_error(
                                (*r).conf.errh,
                                b"src/connections.c\0" as *const u8 as *const libc::c_char,
                                717 as libc::c_int as libc::c_uint,
                                b"%s (%s)\0" as *const u8 as *const libc::c_char,
                                if *((*(*c).mem).ptr).offset((*c).offset as isize)
                                    as libc::c_int == 0x16 as libc::c_int
                                {
                                    b"unexpected TLS ClientHello on clear port\0" as *const u8
                                        as *const libc::c_char
                                } else {
                                    b"invalid request-line -> sending Status 400\0" as *const u8
                                        as *const libc::c_char
                                },
                                (*con).dst_addr_buf.ptr,
                            );
                            (*r).http_status = 400 as libc::c_int;
                            (*r).keep_alive = 0 as libc::c_int as int8_t;
                            return 1 as libc::c_int;
                        }
                    }
                }
            }
        }
        c = connection_read_header_more(con, cq, c, clen as size_t);
        if c.is_null() {
            break;
        }
    }
    if keepalive_request_start != 0 {
        if (*con).bytes_read > (*r).bytes_read_ckpt {
            (*r).start_hp.tv_sec = log_epoch_secs;
            if (*r).conf.high_precision_timestamps != 0 {
                clock_gettime(0 as libc::c_int, &mut (*r).start_hp);
            }
        }
        if pipelined_request_start as libc::c_int != 0 && !c.is_null() {
            (*con).read_idle_ts = log_monotonic_secs;
        }
    }
    if c.is_null() {
        return 0 as libc::c_int;
    }
    let hdrs: *mut libc::c_char = ((*(*c).mem).ptr)
        .offset(hoff[1 as libc::c_int as usize] as libc::c_int as isize);
    if (*con).request_count > 1 as libc::c_int as libc::c_uint {
        request_reset_ex(r);
    } else if (*con).is_ssl_sock == 0 && (*r).conf.h2proto as libc::c_int != 0
            && hoff[0 as libc::c_int as usize] as libc::c_int == 2 as libc::c_int
            && hoff[2 as libc::c_int as usize] as libc::c_int == 16 as libc::c_int
            && *hdrs.offset(0 as libc::c_int as isize) as libc::c_int == 'P' as i32
            && *hdrs.offset(1 as libc::c_int as isize) as libc::c_int == 'R' as i32
            && *hdrs.offset(2 as libc::c_int as isize) as libc::c_int == 'I' as i32
            && *hdrs.offset(3 as libc::c_int as isize) as libc::c_int == ' ' as i32
        {
        (*r).http_version = HTTP_VERSION_2;
        return 0 as libc::c_int;
    }
    (*r).rqst_header_len = header_len;
    if (*r).conf.log_request_header != 0 {
        log_error_multiline(
            (*r).conf.errh,
            b"src/connections.c\0" as *const u8 as *const libc::c_char,
            770 as libc::c_int as libc::c_uint,
            hdrs,
            header_len as size_t,
            b"fd:%d rqst: \0" as *const u8 as *const libc::c_char,
            (*con).fd,
        );
    }
    http_request_headers_process(
        r,
        hdrs,
        hoff.as_mut_ptr(),
        (*con).proto_default_port as libc::c_int,
    );
    chunkqueue_mark_written(cq, (*r).rqst_header_len as off_t);
    if (*r).rqst_htags & (1 as libc::c_ulong) << HTTP_HEADER_UPGRADE as libc::c_int != 0
        && 0 as libc::c_int == (*r).http_status && h2_check_con_upgrade_h2c(r) != 0
    {
        (*r)
            .conditional_is_valid = ((1 as libc::c_int)
            << COMP_SERVER_SOCKET as libc::c_int
            | (1 as libc::c_int) << COMP_HTTP_REMOTE_IP as libc::c_int) as uint32_t;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn connection_handle_fdevent(
    context: *mut libc::c_void,
    revents: libc::c_int,
) -> handler_t {
    let mut con: *mut connection = context as *mut connection;
    let is_ssl_sock: libc::c_int = (*con).is_ssl_sock as libc::c_int;
    connection_jq_append(con);
    if revents & !(0x1 as libc::c_int | 0x4 as libc::c_int) != 0 {
        (*con)
            .revents_err = ((*con).revents_err as libc::c_int
            | revents & !(0x1 as libc::c_int | 0x4 as libc::c_int)) as uint16_t;
    }
    if revents & (0x1 as libc::c_int | 0x4 as libc::c_int) != 0 {
        if is_ssl_sock != 0 {
            (*con).is_writable = 1 as libc::c_int as libc::c_schar;
            (*con).is_readable = (*con).is_writable;
        } else {
            if revents & 0x1 as libc::c_int != 0 {
                (*con).is_readable = 1 as libc::c_int as libc::c_schar;
            }
            if revents & 0x4 as libc::c_int != 0 {
                (*con).is_writable = 1 as libc::c_int as libc::c_schar;
            }
        }
    }
    return HANDLER_FINISHED;
}
#[cold]
unsafe extern "C" fn connection_read_cq_err(mut con: *mut connection) -> libc::c_int {
    let r: *mut request_st = &mut (*con).request;
    match *__errno_location() {
        11 => return 0 as libc::c_int,
        4 => {
            (*con).is_readable = 1 as libc::c_int as libc::c_schar;
            return 0 as libc::c_int;
        }
        104 => {}
        _ => {
            log_perror(
                (*r).conf.errh,
                b"src/connections.c\0" as *const u8 as *const libc::c_char,
                845 as libc::c_int as libc::c_uint,
                b"connection closed - read failed\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    connection_set_state_error(r, CON_STATE_ERROR);
    return -(1 as libc::c_int);
}
unsafe extern "C" fn connection_read_cq(
    mut con: *mut connection,
    mut cq: *mut chunkqueue,
    mut max_bytes: off_t,
) -> libc::c_int {
    let mut len: ssize_t = 0;
    let mut mem_len: size_t = 0 as libc::c_int as size_t;
    loop {
        let mut ckpt: *mut chunk = (*cq).last;
        let mem: *mut libc::c_char = chunkqueue_get_memory(cq, &mut mem_len);
        if mem_len > max_bytes as size_t {
            mem_len = max_bytes as size_t;
        }
        len = read((*con).fd, mem as *mut libc::c_void, mem_len);
        chunkqueue_use_memory(
            cq,
            ckpt,
            (if len > 0 as libc::c_int as libc::c_long {
                len
            } else {
                0 as libc::c_int as libc::c_long
            }) as size_t,
        );
        if len != mem_len as ssize_t {
            (*con).is_readable = 0 as libc::c_int as libc::c_schar;
            if len > 0 as libc::c_int as libc::c_long {
                (*con).bytes_read += len;
                return 0 as libc::c_int;
            } else if 0 as libc::c_int as libc::c_long == len {
                return -(2 as libc::c_int)
            } else {
                return connection_read_cq_err(con)
            }
        }
        (*con).bytes_read += len;
        max_bytes -= len;
        let mut frd: libc::c_int = 0;
        mem_len = if 0 as libc::c_int
            == fdevent_ioctl_fionread((*con).fd, 0o140000 as libc::c_int, &mut frd)
        {
            if (frd as libc::c_long) < max_bytes {
                frd as size_t
            } else {
                max_bytes as size_t
            }
        } else {
            0 as libc::c_int as libc::c_ulong
        };
        if !(max_bytes != 0) {
            break;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn connection_write_cq(
    mut con: *mut connection,
    mut cq: *mut chunkqueue,
    mut max_bytes: off_t,
) -> libc::c_int {
    let r: *mut request_st = &mut (*con).request;
    return ((*(*con).srv).network_backend_write)
        .expect("non-null function pointer")((*con).fd, cq, max_bytes, (*r).conf.errh);
}
#[no_mangle]
pub unsafe extern "C" fn connection_accepted(
    mut srv: *mut server,
    mut srv_socket: *const server_socket,
    mut cnt_addr: *mut sock_addr,
    mut cnt: libc::c_int,
) -> *mut connection {
    let mut con: *mut connection = 0 as *mut connection;
    (*srv).cur_fds += 1;
    (*srv).con_opened += 1;
    con = connections_get_new_connection(srv);
    (*con).fd = cnt;
    (*con)
        .fdn = fdevent_register(
        (*srv).ev,
        (*con).fd,
        Some(
            connection_handle_fdevent
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> handler_t,
        ),
        con as *mut libc::c_void,
    );
    (*con)
        .network_read = Some(
        connection_read_cq
            as unsafe extern "C" fn(
                *mut connection,
                *mut chunkqueue,
                off_t,
            ) -> libc::c_int,
    );
    (*con)
        .network_write = Some(
        connection_write_cq
            as unsafe extern "C" fn(
                *mut connection,
                *mut chunkqueue,
                off_t,
            ) -> libc::c_int,
    );
    (*con)
        .reqbody_read = Some(
        connection_handle_read_post_state
            as unsafe extern "C" fn(*mut request_st) -> handler_t,
    );
    let r: *mut request_st = &mut (*con).request;
    (*r).state = CON_STATE_REQUEST_START;
    (*con).connection_start = log_monotonic_secs;
    (*con).dst_addr = *cnt_addr;
    sock_addr_cache_inet_ntop_copy_buffer(
        &mut (*con).dst_addr_buf,
        &mut (*con).dst_addr,
    );
    (*con).srv_socket = srv_socket;
    (*con).is_ssl_sock = (*srv_socket).is_ssl as libc::c_char;
    (*con).proto_default_port = 80 as libc::c_int as uint16_t;
    config_cond_cache_reset(r);
    (*r)
        .conditional_is_valid = ((1 as libc::c_int) << COMP_SERVER_SOCKET as libc::c_int
        | (1 as libc::c_int) << COMP_HTTP_REMOTE_IP as libc::c_int) as uint32_t;
    if HANDLER_GO_ON as libc::c_int as libc::c_uint
        != plugins_call_handle_connection_accept(con) as libc::c_uint
    {
        connection_reset(con);
        connection_close(con);
        return 0 as *mut connection;
    }
    if (*r).http_status < 0 as libc::c_int {
        (*r).state = CON_STATE_WRITE;
    }
    return con;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn connection_get_state(
    mut state: request_state_t,
) -> *const libc::c_char {
    match state as libc::c_uint {
        0 => return b"connect\0" as *const u8 as *const libc::c_char,
        2 => return b"read\0" as *const u8 as *const libc::c_char,
        4 => return b"readpost\0" as *const u8 as *const libc::c_char,
        7 => return b"write\0" as *const u8 as *const libc::c_char,
        10 => return b"close\0" as *const u8 as *const libc::c_char,
        9 => return b"error\0" as *const u8 as *const libc::c_char,
        5 => return b"handle-req\0" as *const u8 as *const libc::c_char,
        1 => return b"req-start\0" as *const u8 as *const libc::c_char,
        3 => return b"req-end\0" as *const u8 as *const libc::c_char,
        6 => return b"resp-start\0" as *const u8 as *const libc::c_char,
        8 => return b"resp-end\0" as *const u8 as *const libc::c_char,
        _ => return b"(unknown)\0" as *const u8 as *const libc::c_char,
    };
}
unsafe extern "C" fn connection_state_machine_loop(
    r: *mut request_st,
    con: *mut connection,
) {
    let mut ostate: request_state_t = CON_STATE_CONNECT;
    let mut current_block_34: u64;
    loop {
        if (*r).conf.log_state_handling != 0 {
            log_error(
                (*r).conf.errh,
                b"src/connections.c\0" as *const u8 as *const libc::c_char,
                987 as libc::c_int as libc::c_uint,
                b"state for fd:%d id:%d %s\0" as *const u8 as *const libc::c_char,
                (*con).fd,
                (*r).h2id,
                connection_get_state((*r).state),
            );
        }
        ostate = (*r).state;
        match ostate as libc::c_uint {
            1 => {
                (*r).start_hp.tv_sec = log_epoch_secs;
                (*con).read_idle_ts = log_monotonic_secs;
                if (*r).conf.high_precision_timestamps != 0 {
                    clock_gettime(0 as libc::c_int, &mut (*r).start_hp);
                }
                (*con).request_count = ((*con).request_count).wrapping_add(1);
                (*r).loops_per_request = 0 as libc::c_int as libc::c_char;
                (*r).state = CON_STATE_READ;
                current_block_34 = 7494613362390292015;
            }
            2 => {
                current_block_34 = 7494613362390292015;
            }
            3 => {
                current_block_34 = 16530171831619307892;
            }
            4 | 5 => {
                current_block_34 = 16550236284264147277;
            }
            7 => {
                current_block_34 = 5870691524712373942;
            }
            8 | 9 => {
                current_block_34 = 1804405408969159832;
            }
            10 => {
                connection_handle_close_state(con);
                current_block_34 = 11174649648027449784;
            }
            0 => {
                current_block_34 = 11174649648027449784;
            }
            _ => {
                log_error(
                    (*r).conf.errh,
                    b"src/connections.c\0" as *const u8 as *const libc::c_char,
                    1064 as libc::c_int as libc::c_uint,
                    b"unknown state: %d %d\0" as *const u8 as *const libc::c_char,
                    (*con).fd,
                    (*r).state as libc::c_uint,
                );
                current_block_34 = 11174649648027449784;
            }
        }
        match current_block_34 {
            7494613362390292015 => {
                if connection_handle_read_state(con) == 0 {
                    if (*r).http_version as libc::c_int == HTTP_VERSION_2 as libc::c_int
                    {
                        connection_transition_h2(r, con);
                        connection_state_machine_h2(r, con);
                        return;
                    }
                    current_block_34 = 11174649648027449784;
                } else {
                    current_block_34 = 16530171831619307892;
                }
            }
            _ => {}
        }
        match current_block_34 {
            16530171831619307892 => {
                (*r)
                    .state = (if 0 as libc::c_int as libc::c_long == (*r).reqbody_length
                {
                    CON_STATE_HANDLE_REQUEST as libc::c_int
                } else {
                    CON_STATE_READ_POST as libc::c_int
                }) as request_state_t;
                current_block_34 = 16550236284264147277;
            }
            _ => {}
        }
        match current_block_34 {
            16550236284264147277 => {
                match http_response_handler(r) as libc::c_uint {
                    0 | 1 => {
                        if (*r).http_version as libc::c_int
                            > HTTP_VERSION_1_1 as libc::c_int
                        {
                            h2_send_headers(r, con);
                        } else {
                            http_response_write_header(r);
                        }
                        (*r).state = CON_STATE_WRITE;
                        current_block_34 = 5870691524712373942;
                    }
                    3 => return,
                    _ => {
                        connection_set_state_error(r, CON_STATE_ERROR);
                        current_block_34 = 11174649648027449784;
                    }
                }
            }
            _ => {}
        }
        match current_block_34 {
            5870691524712373942 => {
                if connection_handle_write_state(r, con)
                    == CON_STATE_WRITE as libc::c_int
                {
                    return;
                }
                current_block_34 = 1804405408969159832;
            }
            _ => {}
        }
        match current_block_34 {
            1804405408969159832 => {
                if (*r).http_version as libc::c_int > HTTP_VERSION_1_1 as libc::c_int
                    && r != &mut (*con).request as *mut request_st
                {
                    return;
                }
                connection_handle_response_end_state(r, con);
                ostate = CON_STATE_RESPONSE_END;
            }
            _ => {}
        }
        if !(ostate as libc::c_uint != (*r).state as libc::c_uint) {
            break;
        }
    };
}
#[cold]
unsafe extern "C" fn connection_revents_err(r: *mut request_st, con: *mut connection) {
    let revents: libc::c_int = (*con).revents_err as libc::c_int;
    (*con).revents_err = 0 as libc::c_int as uint16_t;
    if (*r).state as libc::c_uint == CON_STATE_CLOSE as libc::c_int as libc::c_uint {
        (*con)
            .close_timeout_ts = log_monotonic_secs
            - (5 as libc::c_int + 1 as libc::c_int) as libc::c_long;
    } else if revents & 0x10 as libc::c_int != 0 {
        connection_set_state_error(r, CON_STATE_ERROR);
    } else if revents & 0x2000 as libc::c_int != 0 {
        let mut events: libc::c_int = if !((*con).fdn).is_null() {
            (*(*con).fdn).events
        } else {
            0 as libc::c_int
        };
        events &= !(0x1 as libc::c_int | 0x2000 as libc::c_int);
        (*r)
            .conf
            .stream_request_body = ((*r).conf.stream_request_body as libc::c_int
            & !((1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 15 as libc::c_int)) as libc::c_ushort;
        (*r)
            .conf
            .stream_request_body = ((*r).conf.stream_request_body as libc::c_int
            | (1 as libc::c_int) << 12 as libc::c_int) as libc::c_ushort;
        (*con).is_readable = 1 as libc::c_int as libc::c_schar;
        if chunkqueue_is_empty((*con).read_queue) != 0 {
            (*r).keep_alive = 0 as libc::c_int as int8_t;
        }
        if (*r).reqbody_length < -(1 as libc::c_int) as libc::c_long {
            (*r).reqbody_length = (*r).reqbody_queue.bytes_in;
        }
        if sock_addr_get_family(&mut (*con).dst_addr) == 1 as libc::c_int {
            fdevent_fdnode_event_set((*(*con).srv).ev, (*con).fdn, events);
        } else if fdevent_is_tcp_half_closed((*con).fd) != 0 {
            (*r)
                .conf
                .stream_request_body = ((*r).conf.stream_request_body as libc::c_int
                | (1 as libc::c_int) << 13 as libc::c_int) as libc::c_ushort;
            fdevent_fdnode_event_set((*(*con).srv).ev, (*con).fdn, events);
        } else {
            connection_set_state_error(r, CON_STATE_ERROR);
        }
    } else if revents & 0x8 as libc::c_int != 0 {
        connection_set_state_error(r, CON_STATE_ERROR);
    } else {
        log_error(
            (*r).conf.errh,
            b"src/connections.c\0" as *const u8 as *const libc::c_char,
            1123 as libc::c_int as libc::c_uint,
            b"connection closed: poll() -> ??? %d\0" as *const u8 as *const libc::c_char,
            revents,
        );
    };
}
unsafe extern "C" fn connection_set_fdevent_interest(
    r: *mut request_st,
    con: *mut connection,
) {
    if (*con).fd < 0 as libc::c_int {
        return;
    }
    if (*con).revents_err as libc::c_int != 0
        && (*r).state as libc::c_uint != CON_STATE_ERROR as libc::c_int as libc::c_uint
    {
        connection_revents_err(r, con);
        connection_state_machine(con);
        return;
    }
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut current_block_18: u64;
    match (*r).state as libc::c_uint {
        2 => {
            n = 0x1 as libc::c_int;
            if (*r).conf.stream_request_body as libc::c_int
                & (1 as libc::c_int) << 12 as libc::c_int == 0
            {
                n |= 0x2000 as libc::c_int;
            }
            current_block_18 = 224731115979188411;
        }
        7 => {
            if chunkqueue_is_empty((*con).write_queue) == 0
                && 0 as libc::c_int == (*con).is_writable as libc::c_int
                && 0 as libc::c_int == (*con).traffic_limit_reached as libc::c_int
            {
                n |= 0x4 as libc::c_int;
            }
            current_block_18 = 11767084564122738674;
        }
        4 => {
            current_block_18 = 11767084564122738674;
        }
        10 => {
            n = 0x1 as libc::c_int;
            current_block_18 = 224731115979188411;
        }
        0 => return,
        _ => {
            current_block_18 = 224731115979188411;
        }
    }
    match current_block_18 {
        11767084564122738674 => {
            if (*r).conf.stream_request_body as libc::c_int
                & (1 as libc::c_int) << 15 as libc::c_int != 0
            {
                n |= 0x1 as libc::c_int;
            }
            if (*r).conf.stream_request_body as libc::c_int
                & (1 as libc::c_int) << 12 as libc::c_int == 0
            {
                n |= 0x2000 as libc::c_int;
            }
        }
        _ => {}
    }
    let events: libc::c_int = if !((*con).fdn).is_null() {
        (*(*con).fdn).events
    } else {
        0 as libc::c_int
    };
    if ((*con).is_readable as libc::c_int) < 0 as libc::c_int {
        (*con).is_readable = 0 as libc::c_int as libc::c_schar;
        n |= 0x1 as libc::c_int;
    }
    if ((*con).is_writable as libc::c_int) < 0 as libc::c_int {
        (*con).is_writable = 0 as libc::c_int as libc::c_schar;
        n |= 0x4 as libc::c_int;
    }
    if events & 0x2000 as libc::c_int != 0 {
        n |= 0x2000 as libc::c_int;
    }
    if n == events {
        return;
    }
    if n & 0x1 as libc::c_int != 0 && events & 0x1 as libc::c_int == 0 {
        (*con).read_idle_ts = log_monotonic_secs;
    }
    if n & 0x4 as libc::c_int != 0 && events & 0x4 as libc::c_int == 0 {
        (*con).write_request_ts = log_monotonic_secs;
    }
    fdevent_fdnode_event_set((*(*con).srv).ev, (*con).fdn, n);
}
#[cold]
unsafe extern "C" fn connection_request_end_h2(
    h2r: *mut request_st,
    con: *mut connection,
) {
    if (*h2r).keep_alive as libc::c_int >= 0 as libc::c_int {
        (*h2r).keep_alive = -(1 as libc::c_int) as int8_t;
        h2_send_goaway(con, H2_E_NO_ERROR);
    } else {
        h2_send_goaway(con, H2_E_ENHANCE_YOUR_CALM);
    };
}
unsafe extern "C" fn connection_state_machine_h2(
    h2r: *mut request_st,
    con: *mut connection,
) {
    let h2c: *mut h2con = (*con).h2;
    if (*h2c).sent_goaway <= 0 as libc::c_int
        && (chunkqueue_is_empty((*con).read_queue) != 0 || h2_parse_frames(con) != 0)
        && (*con).is_readable as libc::c_int > 0 as libc::c_int
    {
        let cq: *mut chunkqueue = (*con).read_queue;
        let mark: off_t = (*cq).bytes_in;
        if 0 as libc::c_int
            == ((*con).network_read)
                .expect(
                    "non-null function pointer",
                )(con, cq, (256 as libc::c_int * 1024 as libc::c_int) as off_t)
        {
            if mark < (*cq).bytes_in {
                h2_parse_frames(con);
            }
        } else {
            (*h2c).sent_goaway = H2_E_CONNECT_ERROR as libc::c_int;
            connection_set_state_error(h2r, CON_STATE_ERROR);
        }
    }
    let mut resched: libc::c_int = 0 as libc::c_int;
    if (*h2c).sent_goaway <= 0 as libc::c_int && (*h2c).rused != 0 {
        let mut max_bytes: off_t = if (*con).is_writable as libc::c_int
            > 0 as libc::c_int
        {
            connection_write_throttle(
                con,
                (256 as libc::c_int * 1024 as libc::c_int) as off_t,
            )
        } else {
            0 as libc::c_int as libc::c_long
        };
        let cqlen: off_t = chunkqueue_length((*con).write_queue);
        if cqlen > 8192 as libc::c_int as libc::c_long
            && max_bytes > 65536 as libc::c_int as libc::c_long
        {
            max_bytes = 65536 as libc::c_int as off_t;
        }
        max_bytes -= cqlen;
        if max_bytes < 0 as libc::c_int as libc::c_long {
            max_bytes = 0 as libc::c_int as off_t;
        }
        let mut current_block_33: u64;
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while i < (*h2c).rused {
            let r: *mut request_st = (*h2c).r[i as usize];
            connection_state_machine_loop(r, con);
            if !(((*r).state as libc::c_uint)
                < CON_STATE_WRITE as libc::c_int as libc::c_uint)
            {
                if (*r).state as libc::c_uint
                    == CON_STATE_WRITE as libc::c_int as libc::c_uint
                {
                    if (chunkqueue_is_empty(&mut (*r).write_queue) == 0) as libc::c_int
                        as libc::c_long != 0 && max_bytes != 0
                        && ((*r).resp_body_finished as libc::c_int != 0
                            || (*r).conf.stream_response_body as libc::c_int
                                & ((1 as libc::c_int) << 0 as libc::c_int
                                    | (1 as libc::c_int) << 1 as libc::c_int) != 0)
                    {
                        let mut dlen: uint32_t = if max_bytes
                            > 32768 as libc::c_int as libc::c_long
                        {
                            32768 as libc::c_int as libc::c_uint
                        } else {
                            max_bytes as uint32_t
                        };
                        dlen = h2_send_cqdata(r, con, &mut (*r).write_queue, dlen);
                        if dlen != 0 {
                            max_bytes -= dlen as off_t;
                            if chunkqueue_is_empty(&mut (*r).write_queue) == 0 {
                                resched |= 1 as libc::c_int;
                            }
                        }
                    }
                    if chunkqueue_is_empty(&mut (*r).write_queue) == 0
                        || (*r).resp_body_finished == 0
                    {
                        current_block_33 = 6009453772311597924;
                    } else {
                        (*r).state = CON_STATE_RESPONSE_END;
                        if (*r).conf.log_state_handling as libc::c_long != 0 {
                            connection_state_machine_loop(r, con);
                        }
                        current_block_33 = 9520865839495247062;
                    }
                } else {
                    current_block_33 = 9520865839495247062;
                }
                match current_block_33 {
                    6009453772311597924 => {}
                    _ => {
                        if (*h2c).rused as libc::c_ulong
                            == (::std::mem::size_of::<[*mut request_st; 8]>()
                                as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<*mut request_st>() as libc::c_ulong,
                                ) && chunkqueue_is_empty((*con).read_queue) == 0
                        {
                            resched |= 2 as libc::c_int;
                        }
                        h2_send_end_stream(r, con);
                        let alive: libc::c_int = (*r).keep_alive as libc::c_int;
                        h2_retire_stream(r, con);
                        i = i.wrapping_sub(1);
                        if alive < 0 as libc::c_int {
                            connection_request_end_h2(h2r, con);
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
        if 0 as libc::c_int as libc::c_long == max_bytes {
            resched |= 1 as libc::c_int;
        }
    }
    if (*h2c).sent_goaway > 0 as libc::c_int && (*h2c).rused != 0 {
        let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
        while i_0 < (*h2c).rused {
            let r_0: *mut request_st = (*h2c).r[i_0 as usize];
            h2_retire_stream(r_0, con);
            i_0 = i_0.wrapping_sub(1);
            i_0 = i_0.wrapping_add(1);
        }
    }
    if (*h2r).state as libc::c_uint == CON_STATE_WRITE as libc::c_int as libc::c_uint {
        if chunkqueue_is_empty((*con).write_queue) == 0 {
            connection_handle_write(h2r, con);
        }
        if chunkqueue_is_empty((*con).write_queue) != 0
            && 0 as libc::c_int as libc::c_uint == (*h2c).rused
            && (*h2c).sent_goaway != 0
        {
            (*h2r).state = CON_STATE_RESPONSE_END;
        }
    }
    if (*h2r).state as libc::c_uint == CON_STATE_WRITE as libc::c_int as libc::c_uint {
        if resched & 1 as libc::c_int != 0
            && (*con).is_writable as libc::c_int > 0 as libc::c_int
            && (*con).traffic_limit_reached == 0 || resched & 2 as libc::c_int != 0
        {
            connection_jq_append(con);
        }
        if h2_want_read(con) != 0 {
            (*h2r)
                .conf
                .stream_request_body = ((*h2r).conf.stream_request_body as libc::c_int
                | (1 as libc::c_int) << 15 as libc::c_int) as libc::c_ushort;
        } else {
            (*h2r)
                .conf
                .stream_request_body = ((*h2r).conf.stream_request_body as libc::c_int
                & !((1 as libc::c_int) << 15 as libc::c_int)) as libc::c_ushort;
        }
    } else {
        connection_state_machine_loop(h2r, con);
    }
    connection_set_fdevent_interest(h2r, con);
}
unsafe extern "C" fn connection_state_machine_h1(
    r: *mut request_st,
    con: *mut connection,
) {
    let log_state_handling: libc::c_int = (*r).conf.log_state_handling as libc::c_int;
    if log_state_handling != 0 {
        log_error(
            (*r).conf.errh,
            b"src/connections.c\0" as *const u8 as *const libc::c_char,
            1368 as libc::c_int as libc::c_uint,
            b"state at enter %d %s\0" as *const u8 as *const libc::c_char,
            (*con).fd,
            connection_get_state((*r).state),
        );
    }
    connection_state_machine_loop(r, con);
    if log_state_handling != 0 {
        log_error(
            (*r).conf.errh,
            b"src/connections.c\0" as *const u8 as *const libc::c_char,
            1375 as libc::c_int as libc::c_uint,
            b"state at exit: %d %s\0" as *const u8 as *const libc::c_char,
            (*con).fd,
            connection_get_state((*r).state),
        );
    }
    connection_set_fdevent_interest(r, con);
}
#[no_mangle]
pub unsafe extern "C" fn connection_state_machine(con: *mut connection) {
    let r: *mut request_st = &mut (*con).request;
    if (*r).http_version as libc::c_int == HTTP_VERSION_2 as libc::c_int {
        connection_state_machine_h2(r, con);
    } else {
        connection_state_machine_h1(r, con);
    };
}
unsafe extern "C" fn connection_check_timeout(
    con: *mut connection,
    cur_ts: unix_time64_t,
) {
    let waitevents: libc::c_int = if !((*con).fdn).is_null() {
        (*(*con).fdn).events
    } else {
        0 as libc::c_int
    };
    let mut changed: libc::c_int = 0 as libc::c_int;
    let mut t_diff: libc::c_int = 0;
    let r: *mut request_st = &mut (*con).request;
    if (*r).state as libc::c_uint == CON_STATE_CLOSE as libc::c_int as libc::c_uint {
        if cur_ts - (*con).close_timeout_ts > 5 as libc::c_int as libc::c_long {
            changed = 1 as libc::c_int;
        }
    } else if !((*con).h2).is_null()
            && (*r).state as libc::c_uint
                == CON_STATE_WRITE as libc::c_int as libc::c_uint
        {
        let h2c: *mut h2con = (*con).h2;
        if (*h2c).rused != 0 {
            let mut i: uint32_t = 0 as libc::c_int as uint32_t;
            while i < (*h2c).rused {
                let rr: *mut request_st = (*h2c).r[i as usize];
                if (*rr).state as libc::c_uint
                    == CON_STATE_ERROR as libc::c_int as libc::c_uint
                {
                    changed = 1 as libc::c_int;
                } else {
                    if (*rr).reqbody_length != (*rr).reqbody_queue.bytes_in {
                        if cur_ts - (*con).read_idle_ts
                            > (*rr).conf.max_read_idle as libc::c_long
                        {
                            if (*rr).conf.log_request_handling != 0 {
                                log_error(
                                    (*rr).conf.errh,
                                    b"src/connections.c\0" as *const u8 as *const libc::c_char,
                                    1423 as libc::c_int as libc::c_uint,
                                    b"request aborted - read timeout: %d\0" as *const u8
                                        as *const libc::c_char,
                                    (*con).fd,
                                );
                            }
                            connection_set_state_error(r, CON_STATE_ERROR);
                            changed = 1 as libc::c_int;
                        }
                    }
                    if (*rr).state as libc::c_uint
                        != CON_STATE_READ_POST as libc::c_int as libc::c_uint
                        && (*con).write_request_ts != 0 as libc::c_int as libc::c_long
                    {
                        if cur_ts - (*con).write_request_ts
                            > (*r).conf.max_write_idle as libc::c_long
                        {
                            if (*r).conf.log_timeouts != 0 {
                                log_error(
                                    (*r).conf.errh,
                                    b"src/connections.c\0" as *const u8 as *const libc::c_char,
                                    1441 as libc::c_int as libc::c_uint,
                                    b"NOTE: a request from %s for %.*s timed out after writing %lld bytes. We waited %d seconds. If this is a problem, increase server.max-write-idle\0"
                                        as *const u8 as *const libc::c_char,
                                    (*con).dst_addr_buf.ptr,
                                    buffer_clen(&mut (*r).target) as libc::c_int,
                                    (*r).target.ptr,
                                    (*r).write_queue.bytes_out as libc::c_longlong,
                                    (*r).conf.max_write_idle as libc::c_int,
                                );
                            }
                            connection_set_state_error(r, CON_STATE_ERROR);
                            changed = 1 as libc::c_int;
                        }
                    }
                }
                i = i.wrapping_add(1);
            }
        } else if cur_ts - (*con).read_idle_ts > (*con).keep_alive_idle as libc::c_long {
            if (*r).conf.log_request_handling != 0 {
                log_error(
                    (*r).conf.errh,
                    b"src/connections.c\0" as *const u8 as *const libc::c_char,
                    1461 as libc::c_int as libc::c_uint,
                    b"connection closed - keep-alive timeout: %d\0" as *const u8
                        as *const libc::c_char,
                    (*con).fd,
                );
            }
            (*r).state = CON_STATE_RESPONSE_END;
            changed = 1 as libc::c_int;
        }
        if changed != 0 {
            (*con).is_readable = 0 as libc::c_int as libc::c_schar;
        }
    } else if waitevents & 0x1 as libc::c_int != 0 {
        if (*con).request_count == 1 as libc::c_int as libc::c_uint
            || (*r).state as libc::c_uint
                != CON_STATE_READ as libc::c_int as libc::c_uint
        {
            if cur_ts - (*con).read_idle_ts > (*r).conf.max_read_idle as libc::c_long {
                if (*r).conf.log_request_handling != 0 {
                    log_error(
                        (*r).conf.errh,
                        b"src/connections.c\0" as *const u8 as *const libc::c_char,
                        1479 as libc::c_int as libc::c_uint,
                        b"connection closed - read timeout: %d\0" as *const u8
                            as *const libc::c_char,
                        (*con).fd,
                    );
                }
                connection_set_state_error(r, CON_STATE_ERROR);
                changed = 1 as libc::c_int;
            }
        } else if cur_ts - (*con).read_idle_ts > (*con).keep_alive_idle as libc::c_long {
            if (*r).conf.log_request_handling != 0 {
                log_error(
                    (*r).conf.errh,
                    b"src/connections.c\0" as *const u8 as *const libc::c_char,
                    1490 as libc::c_int as libc::c_uint,
                    b"connection closed - keep-alive timeout: %d\0" as *const u8
                        as *const libc::c_char,
                    (*con).fd,
                );
            }
            connection_set_state_error(r, CON_STATE_ERROR);
            changed = 1 as libc::c_int;
        }
    }
    if (*r).http_version as libc::c_int <= HTTP_VERSION_1_1 as libc::c_int
        && (*r).state as libc::c_uint == CON_STATE_WRITE as libc::c_int as libc::c_uint
        && (*con).write_request_ts != 0 as libc::c_int as libc::c_long
    {
        if cur_ts - (*con).write_request_ts > (*r).conf.max_write_idle as libc::c_long {
            if (*r).conf.log_timeouts != 0 {
                log_error(
                    (*r).conf.errh,
                    b"src/connections.c\0" as *const u8 as *const libc::c_char,
                    1526 as libc::c_int as libc::c_uint,
                    b"NOTE: a request from %s for %.*s timed out after writing %lld bytes. We waited %d seconds. If this is a problem, increase server.max-write-idle\0"
                        as *const u8 as *const libc::c_char,
                    (*con).dst_addr_buf.ptr,
                    buffer_clen(&mut (*r).target) as libc::c_int,
                    (*r).target.ptr,
                    (*con).bytes_written as libc::c_longlong,
                    (*r).conf.max_write_idle as libc::c_int,
                );
            }
            connection_set_state_error(r, CON_STATE_ERROR);
            changed = 1 as libc::c_int;
        }
    }
    t_diff = (cur_ts - (*con).connection_start) as libc::c_int;
    if 0 as libc::c_int == t_diff {
        t_diff = 1 as libc::c_int;
    }
    if (*con).traffic_limit_reached as libc::c_int != 0
        && ((*r).conf.bytes_per_second == 0 as libc::c_int as libc::c_uint
            || (*con).bytes_written
                < (*r).conf.bytes_per_second as off_t * t_diff as libc::c_long)
    {
        (*con).traffic_limit_reached = 0 as libc::c_int as libc::c_char;
        changed = 1 as libc::c_int;
    }
    (*con).bytes_written_cur_second = 0 as libc::c_int as off_t;
    if changed != 0 {
        connection_state_machine(con);
    }
}
#[no_mangle]
pub unsafe extern "C" fn connection_periodic_maint(
    srv: *mut server,
    cur_ts: unix_time64_t,
) {
    let mut con: *mut connection = (*srv).conns;
    let mut tc: *mut connection = 0 as *mut connection;
    while !con.is_null() {
        tc = (*con).next;
        connection_check_timeout(con, cur_ts);
        con = tc;
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn connection_graceful_shutdown_maint(mut srv: *mut server) {
    let graceful_expire: libc::c_int = ((*srv).graceful_expire_ts != 0
        && (*srv).graceful_expire_ts < log_monotonic_secs) as libc::c_int;
    let mut con: *mut connection = (*srv).conns;
    let mut tc: *mut connection = 0 as *mut connection;
    while !con.is_null() {
        tc = (*con).next;
        let mut changed: libc::c_int = 0 as libc::c_int;
        let r: *mut request_st = &mut (*con).request;
        if (*r).state as libc::c_uint == CON_STATE_CLOSE as libc::c_int as libc::c_uint {
            if 5 as libc::c_int > 1 as libc::c_int {
                (*con).close_timeout_ts
                    -= (5 as libc::c_int - 1 as libc::c_int) as libc::c_long;
            }
            if log_monotonic_secs - (*con).close_timeout_ts
                > 5 as libc::c_int as libc::c_long
            {
                changed = 1 as libc::c_int;
            }
        } else if !((*con).h2).is_null()
                && (*r).state as libc::c_uint
                    == CON_STATE_WRITE as libc::c_int as libc::c_uint
            {
            h2_send_goaway(con, H2_E_NO_ERROR);
            if 0 as libc::c_int as libc::c_uint == (*(*con).h2).rused
                && chunkqueue_is_empty((*con).write_queue) != 0
            {
                (*r).state = CON_STATE_RESPONSE_END;
                changed = 1 as libc::c_int;
            }
        } else if (*r).state as libc::c_uint
                == CON_STATE_READ as libc::c_int as libc::c_uint
                && (*con).request_count > 1 as libc::c_int as libc::c_uint
                && chunkqueue_is_empty((*con).read_queue) != 0
            {
            connection_set_state_error(r, CON_STATE_ERROR);
            changed = 1 as libc::c_int;
        }
        if graceful_expire != 0 {
            connection_set_state_error(r, CON_STATE_ERROR);
            changed = 1 as libc::c_int;
        }
        (*r).keep_alive = 0 as libc::c_int as int8_t;
        (*r).conf.bytes_per_second = 0 as libc::c_int as libc::c_uint;
        (*r).conf.global_bytes_per_second = 0 as libc::c_int as libc::c_uint;
        if (*con).traffic_limit_reached != 0 {
            (*con).traffic_limit_reached = 0 as libc::c_int as libc::c_char;
            changed = 1 as libc::c_int;
        }
        if changed != 0 {
            connection_state_machine(con);
        }
        con = tc;
    }
}
unsafe extern "C" fn connection_handle_read_post_cq_compact(
    cq: *mut chunkqueue,
) -> libc::c_int {
    let mut c: *mut chunk = (*cq).first;
    if c.is_null() {
        return 0 as libc::c_int;
    }
    let mlen: uint32_t = (buffer_clen((*c).mem) as libc::c_ulong)
        .wrapping_sub((*c).offset as size_t) as uint32_t;
    loop {
        c = (*c).next;
        if c.is_null() {
            break;
        }
        let blen: uint32_t = (buffer_clen((*c).mem) as libc::c_ulong)
            .wrapping_sub((*c).offset as size_t) as uint32_t;
        if 0 as libc::c_int as libc::c_uint == blen {
            continue;
        }
        chunkqueue_compact_mem(cq, mlen.wrapping_add(blen) as size_t);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn connection_handle_read_post_chunked_crlf(
    cq: *mut chunkqueue,
) -> libc::c_int {
    let mut c: *mut chunk = 0 as *mut chunk;
    let mut b: *mut buffer = 0 as *mut buffer;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    if chunkqueue_is_empty(cq) != 0 {
        return 0 as libc::c_int;
    }
    c = (*cq).first;
    b = (*c).mem;
    p = ((*b).ptr).offset((*c).offset as isize);
    if *p.offset(0 as libc::c_int as isize) as libc::c_int != '\r' as i32 {
        return -(1 as libc::c_int);
    }
    if *p.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
        return 1 as libc::c_int;
    }
    len = (buffer_clen(b) as libc::c_ulong).wrapping_sub((*c).offset as size_t);
    if 1 as libc::c_int as libc::c_ulong != len {
        return -(1 as libc::c_int);
    }
    loop {
        c = (*c).next;
        if c.is_null() {
            break;
        }
        b = (*c).mem;
        len = (buffer_clen(b) as libc::c_ulong).wrapping_sub((*c).offset as size_t);
        if 0 as libc::c_int as libc::c_ulong == len {
            continue;
        }
        p = ((*b).ptr).offset((*c).offset as isize);
        return if *p.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn connection_handle_read_post_chunked(
    r: *mut request_st,
    cq: *mut chunkqueue,
    dst_cq: *mut chunkqueue,
) -> handler_t {
    let max_request_size: off_t = ((*r).conf.max_request_size as off_t)
        << 10 as libc::c_int;
    let mut te_chunked: off_t = (*r).te_chunked;
    loop {
        let mut len: off_t = chunkqueue_length(cq);
        while 0 as libc::c_int as libc::c_long == te_chunked {
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut c: *mut chunk = (*cq).first;
            if c.is_null() {
                break;
            }
            if !((*c).type_0 as libc::c_uint == MEM_CHUNK as libc::c_int as libc::c_uint)
            {
                ck_assert_failed(
                    b"src/connections.c\0" as *const u8 as *const libc::c_char,
                    1685 as libc::c_int as libc::c_uint,
                    b"c->type == MEM_CHUNK\0" as *const u8 as *const libc::c_char,
                );
            }
            p = strchr(((*(*c).mem).ptr).offset((*c).offset as isize), '\n' as i32);
            if !p.is_null() {
                let mut hsz: off_t = p
                    .offset(1 as libc::c_int as isize)
                    .offset_from(((*(*c).mem).ptr).offset((*c).offset as isize))
                    as libc::c_long;
                let mut s: *mut libc::c_uchar = ((*(*c).mem).ptr as *mut libc::c_uchar)
                    .offset((*c).offset as isize);
                let mut u: libc::c_uchar = 0;
                loop {
                    u = hex2int(*s) as libc::c_uchar;
                    if !(u as libc::c_int != 0xff as libc::c_int) {
                        break;
                    }
                    if te_chunked
                        > ((1 as libc::c_ulonglong)
                            << (8 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<off_t>() as libc::c_ulong,
                                )
                                .wrapping_sub(5 as libc::c_int as libc::c_ulong)) as off_t
                            - 1 as libc::c_int as libc::c_long
                            - 2 as libc::c_int as libc::c_long
                    {
                        log_error(
                            (*r).conf.errh,
                            b"src/connections.c\0" as *const u8 as *const libc::c_char,
                            1692 as libc::c_int as libc::c_uint,
                            b"chunked data size too large -> 400\0" as *const u8
                                as *const libc::c_char,
                        );
                        return http_response_reqbody_read_error(r, 400 as libc::c_int);
                    }
                    te_chunked <<= 4 as libc::c_int;
                    te_chunked |= u as libc::c_long;
                    s = s.offset(1);
                }
                if s
                    == ((*(*c).mem).ptr as *mut libc::c_uchar)
                        .offset((*c).offset as isize)
                {
                    log_error(
                        (*r).conf.errh,
                        b"src/connections.c\0" as *const u8 as *const libc::c_char,
                        1701 as libc::c_int as libc::c_uint,
                        b"chunked header invalid chars -> 400\0" as *const u8
                            as *const libc::c_char,
                    );
                    return http_response_reqbody_read_error(r, 400 as libc::c_int);
                }
                while *s as libc::c_int == ' ' as i32 || *s as libc::c_int == '\t' as i32
                {
                    s = s.offset(1);
                }
                if *s as libc::c_int != '\r' as i32 && *s as libc::c_int != ';' as i32 {
                    log_error(
                        (*r).conf.errh,
                        b"src/connections.c\0" as *const u8 as *const libc::c_char,
                        1708 as libc::c_int as libc::c_uint,
                        b"chunked header invalid chars -> 400\0" as *const u8
                            as *const libc::c_char,
                    );
                    return http_response_reqbody_read_error(r, 400 as libc::c_int);
                }
                if hsz >= 1024 as libc::c_int as libc::c_long {
                    log_error(
                        (*r).conf.errh,
                        b"src/connections.c\0" as *const u8 as *const libc::c_char,
                        1717 as libc::c_int as libc::c_uint,
                        b"chunked header line too long -> 400\0" as *const u8
                            as *const libc::c_char,
                    );
                    return http_response_reqbody_read_error(r, 400 as libc::c_int);
                }
                if 0 as libc::c_int as libc::c_long == te_chunked {
                    if *p.offset(0 as libc::c_int as isize) as libc::c_int == '\r' as i32
                        && *p.offset(1 as libc::c_int as isize) as libc::c_int
                            == '\n' as i32
                    {
                        hsz += 2 as libc::c_int as libc::c_long;
                    } else {
                        hsz -= 2 as libc::c_int as libc::c_long;
                        loop {
                            c = (*cq).first;
                            p = strstr(
                                ((*(*c).mem).ptr)
                                    .offset((*c).offset as isize)
                                    .offset(hsz as isize),
                                b"\r\n\r\n\0" as *const u8 as *const libc::c_char,
                            );
                            if !(p.is_null()
                                && connection_handle_read_post_cq_compact(cq) != 0)
                            {
                                break;
                            }
                        }
                        if p.is_null() {
                            if buffer_clen((*c).mem) as off_t - (*c).offset
                                < (*r).conf.max_request_field_size as off_t
                            {
                                break;
                            }
                            (*r).keep_alive = 0 as libc::c_int as int8_t;
                            p = ((*(*c).mem).ptr)
                                .offset(buffer_clen((*c).mem) as isize)
                                .offset(-(4 as libc::c_int as isize));
                        }
                        hsz = p
                            .offset(4 as libc::c_int as isize)
                            .offset_from(((*(*c).mem).ptr).offset((*c).offset as isize))
                            as libc::c_long;
                    }
                    chunkqueue_mark_written(cq, hsz as size_t as off_t);
                    (*r).reqbody_length = (*dst_cq).bytes_in;
                    break;
                } else {
                    chunkqueue_mark_written(cq, hsz as size_t as off_t);
                    len = chunkqueue_length(cq);
                    if 0 as libc::c_int as libc::c_long != max_request_size
                        && (max_request_size < te_chunked
                            || max_request_size - te_chunked < (*dst_cq).bytes_in)
                    {
                        log_error(
                            (*r).conf.errh,
                            b"src/connections.c\0" as *const u8 as *const libc::c_char,
                            1775 as libc::c_int as libc::c_uint,
                            b"request-size too long: %lld -> 413\0" as *const u8
                                as *const libc::c_char,
                            ((*dst_cq).bytes_in + te_chunked) as libc::c_longlong,
                        );
                        return http_response_reqbody_read_error(r, 413 as libc::c_int);
                    }
                    te_chunked += 2 as libc::c_int as libc::c_long;
                    break;
                }
            } else if buffer_clen((*c).mem) as off_t - (*c).offset
                    >= 1024 as libc::c_int as libc::c_long
                {
                log_error(
                    (*r).conf.errh,
                    b"src/connections.c\0" as *const u8 as *const libc::c_char,
                    1790 as libc::c_int as libc::c_uint,
                    b"chunked header line too long -> 400\0" as *const u8
                        as *const libc::c_char,
                );
                return http_response_reqbody_read_error(r, 400 as libc::c_int);
            } else if connection_handle_read_post_cq_compact(cq) == 0 {
                break;
            }
        }
        if 0 as libc::c_int as libc::c_long == te_chunked {
            break;
        }
        if te_chunked > 2 as libc::c_int as libc::c_long {
            if len > te_chunked - 2 as libc::c_int as libc::c_long {
                len = te_chunked - 2 as libc::c_int as libc::c_long;
            }
            if (*dst_cq).bytes_in + te_chunked
                <= (64 as libc::c_int * 1024 as libc::c_int) as libc::c_long
            {
                chunkqueue_steal(dst_cq, cq, len);
            } else if 0 as libc::c_int
                    != chunkqueue_steal_with_tempfiles(dst_cq, cq, len, (*r).conf.errh)
                {
                return http_response_reqbody_read_error(r, 500 as libc::c_int)
            }
            te_chunked -= len;
            len = chunkqueue_length(cq);
        }
        if len < te_chunked {
            break;
        }
        if 2 as libc::c_int as libc::c_long == te_chunked {
            if -(1 as libc::c_int) == connection_handle_read_post_chunked_crlf(cq) {
                log_error(
                    (*r).conf.errh,
                    b"src/connections.c\0" as *const u8 as *const libc::c_char,
                    1820 as libc::c_int as libc::c_uint,
                    b"chunked data missing end CRLF -> 400\0" as *const u8
                        as *const libc::c_char,
                );
                return http_response_reqbody_read_error(r, 400 as libc::c_int);
            }
            chunkqueue_mark_written(cq, 2 as libc::c_int as off_t);
            te_chunked -= 2 as libc::c_int as libc::c_long;
        }
        if !(chunkqueue_is_empty(cq) == 0) {
            break;
        }
    }
    (*r).te_chunked = te_chunked;
    return HANDLER_GO_ON;
}
unsafe extern "C" fn connection_handle_read_body_unknown(
    r: *mut request_st,
    cq: *mut chunkqueue,
    dst_cq: *mut chunkqueue,
) -> handler_t {
    let max_request_size: off_t = ((*r).conf.max_request_size as off_t)
        << 10 as libc::c_int;
    chunkqueue_append_chunkqueue(dst_cq, cq);
    if 0 as libc::c_int as libc::c_long != max_request_size
        && (*dst_cq).bytes_in > max_request_size
    {
        log_error(
            (*r).conf.errh,
            b"src/connections.c\0" as *const u8 as *const libc::c_char,
            1843 as libc::c_int as libc::c_uint,
            b"request-size too long: %lld -> 413\0" as *const u8 as *const libc::c_char,
            (*dst_cq).bytes_in as libc::c_longlong,
        );
        return http_response_reqbody_read_error(r, 413 as libc::c_int);
    }
    return HANDLER_GO_ON;
}
#[cold]
unsafe extern "C" fn connection_check_expect_100(
    r: *mut request_st,
    con: *mut connection,
) -> libc::c_int {
    if (*con).is_writable as libc::c_int <= 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    let vb: *const buffer = http_header_request_get(
        r,
        HTTP_HEADER_EXPECT,
        b"Expect\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if vb.is_null() {
        return 1 as libc::c_int;
    }
    let mut rc: libc::c_int = buffer_eq_icase_slen(
        vb,
        b"100-continue\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    http_header_request_unset(
        r,
        HTTP_HEADER_EXPECT,
        b"Expect\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if rc == 0 || 0 as libc::c_int as libc::c_long != (*r).reqbody_queue.bytes_in
        || chunkqueue_is_empty(&mut (*r).read_queue) == 0
        || chunkqueue_is_empty(&mut (*r).write_queue) == 0
    {
        return 1 as libc::c_int;
    }
    if (*r).http_version as libc::c_int > HTTP_VERSION_1_1 as libc::c_int {
        h2_send_100_continue(r, con);
    } else if (*r).http_version as libc::c_int == HTTP_VERSION_1_1 as libc::c_int {
        return connection_write_100_continue(r, con)
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn connection_handle_read_post_state(r: *mut request_st) -> handler_t {
    let con: *mut connection = (*r).con;
    let cq: *mut chunkqueue = &mut (*r).read_queue;
    let dst_cq: *mut chunkqueue = &mut (*r).reqbody_queue;
    let mut is_closed: libc::c_int = 0 as libc::c_int;
    if (*r).http_version as libc::c_int > HTTP_VERSION_1_1 as libc::c_int {
        if (*r).h2state >= H2_STATE_HALF_CLOSED_REMOTE as libc::c_int as libc::c_uint {
            is_closed = 1 as libc::c_int;
        }
    } else if (*con).is_readable as libc::c_int > 0 as libc::c_int {
        (*con).read_idle_ts = log_monotonic_secs;
        let max_per_read: off_t = (if (*r).conf.stream_request_body as libc::c_int
            & ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int) == 0
        {
            256 as libc::c_int * 1024 as libc::c_int
        } else if (*r).conf.stream_request_body as libc::c_int
                & (1 as libc::c_int) << 1 as libc::c_int != 0
            {
            16384 as libc::c_int
        } else {
            65536 as libc::c_int
        }) as off_t;
        match ((*con).network_read)
            .expect("non-null function pointer")(con, cq, max_per_read)
        {
            -1 => {
                connection_set_state_error(r, CON_STATE_ERROR);
                return HANDLER_ERROR;
            }
            -2 => {
                is_closed = 1 as libc::c_int;
            }
            _ => {}
        }
        chunkqueue_remove_finished_chunks(cq);
    }
    if (*r).rqst_htags & (1 as libc::c_ulong) << HTTP_HEADER_EXPECT as libc::c_int != 0
        && connection_check_expect_100(r, con) == 0
    {
        return HANDLER_ERROR;
    }
    if !((*r).http_version as libc::c_int > HTTP_VERSION_1_1 as libc::c_int) {
        if (*r).reqbody_length < 0 as libc::c_int as libc::c_long {
            let mut rc: handler_t = (if -(1 as libc::c_int) as libc::c_long
                == (*r).reqbody_length
            {
                connection_handle_read_post_chunked(r, cq, dst_cq) as libc::c_uint
            } else {
                connection_handle_read_body_unknown(r, cq, dst_cq) as libc::c_uint
            }) as handler_t;
            if HANDLER_GO_ON as libc::c_int as libc::c_uint != rc as libc::c_uint {
                return rc;
            }
            chunkqueue_remove_finished_chunks(cq);
        } else {
            let mut len: off_t = (*r).reqbody_length - (*dst_cq).bytes_in;
            if (*r).reqbody_length
                <= (64 as libc::c_int * 1024 as libc::c_int) as libc::c_long
            {
                chunkqueue_steal(dst_cq, cq, len);
            } else if 0 as libc::c_int
                    != chunkqueue_steal_with_tempfiles(dst_cq, cq, len, (*r).conf.errh)
                {
                return http_response_reqbody_read_error(r, 500 as libc::c_int)
            }
            chunkqueue_remove_finished_chunks(cq);
        }
    }
    if (*dst_cq).bytes_in == (*r).reqbody_length {
        (*r)
            .conf
            .stream_request_body = ((*r).conf.stream_request_body as libc::c_int
            & !((1 as libc::c_int) << 15 as libc::c_int)) as libc::c_ushort;
        if (*r).state as libc::c_uint
            == CON_STATE_READ_POST as libc::c_int as libc::c_uint
        {
            (*r).state = CON_STATE_HANDLE_REQUEST;
        }
        return HANDLER_GO_ON;
    } else if is_closed != 0 {
        return HANDLER_ERROR
    } else {
        (*r)
            .conf
            .stream_request_body = ((*r).conf.stream_request_body as libc::c_int
            | (1 as libc::c_int) << 15 as libc::c_int) as libc::c_ushort;
        return (if (*r).conf.stream_request_body as libc::c_int
            & (1 as libc::c_int) << 0 as libc::c_int != 0
        {
            HANDLER_GO_ON as libc::c_int
        } else {
            HANDLER_WAIT_FOR_EVENT as libc::c_int
        }) as handler_t;
    };
}
