use ::libc;
extern "C" {
    pub type cond_match_t;
    pub type cond_cache_t;
    pub type plugin;
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
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn buffer_commit(b: *mut buffer, size: size_t);
    fn buffer_free_ptr(b: *mut buffer);
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_str2(
        b: *mut buffer,
        s1: *const libc::c_char,
        len1: size_t,
        s2: *const libc::c_char,
        len2: size_t,
    );
    fn buffer_append_int(b: *mut buffer, val: intmax_t);
    fn buffer_eq_icase_ssn(
        a: *const libc::c_char,
        b: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
    fn buffer_eq_slen(
        b: *const buffer,
        s: *const libc::c_char,
        slen: size_t,
    ) -> libc::c_int;
    fn buffer_is_equal(a: *const buffer, b: *const buffer) -> libc::c_int;
    fn buffer_append_string_encoded(
        b: *mut buffer,
        s: *const libc::c_char,
        s_len: size_t,
        encoding: buffer_encoding_t,
    );
    fn buffer_urldecode_path(b: *mut buffer);
    fn buffer_is_valid_UTF8(b: *const buffer) -> libc::c_int;
    fn buffer_path_simplify(b: *mut buffer);
    fn buffer_to_lower(b: *mut buffer);
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn array_reset_data_strings(a: *mut array);
    fn array_match_value_prefix(a: *const array, b: *const buffer) -> *const buffer;
    fn array_match_value_prefix_nc(a: *const array, b: *const buffer) -> *const buffer;
    fn chunk_buffer_yield(b: *mut buffer);
    fn chunk_buffer_prepare_append(b: *mut buffer, sz: size_t) -> size_t;
    fn chunkqueue_append_splice_pipe_tempfile(
        cq: *mut chunkqueue,
        fd: libc::c_int,
        len: libc::c_uint,
        errh: *mut log_error_st,
    ) -> ssize_t;
    fn chunkqueue_append_splice_sock_tempfile(
        cq: *mut chunkqueue,
        fd: libc::c_int,
        len: libc::c_uint,
        errh: *mut log_error_st,
    ) -> ssize_t;
    fn chunkqueue_mark_written(cq: *mut chunkqueue, len: off_t);
    fn chunkqueue_reset(cq: *mut chunkqueue);
    fn li_restricted_strtoint64(
        v: *const libc::c_char,
        vlen: uint32_t,
        err: *mut *const libc::c_char,
    ) -> int64_t;
    fn getsockname(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> libc::c_int;
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
    fn sock_addr_get_port(saddr: *const sock_addr) -> libc::c_ushort;
    fn sock_addr_nameinfo_append_buffer(
        b: *mut buffer,
        saddr: *const sock_addr,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn fdevent_fdnode_event_clr(ev: *mut fdevents, fdn: *mut fdnode, event: libc::c_int);
    fn fdevent_ioctl_fionread(
        fd: libc::c_int,
        fdfmt: libc::c_int,
        toread: *mut libc::c_int,
    ) -> libc::c_int;
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
    fn http_chunk_append_mem(
        r: *mut request_st,
        mem: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
    fn http_chunk_append_buffer(r: *mut request_st, mem: *mut buffer) -> libc::c_int;
    fn http_chunk_decode_append_mem(
        r: *mut request_st,
        mem: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
    fn http_chunk_decode_append_buffer(
        r: *mut request_st,
        mem: *mut buffer,
    ) -> libc::c_int;
    fn http_chunk_transfer_cqlen(
        r: *mut request_st,
        src: *mut chunkqueue,
        len: size_t,
    ) -> libc::c_int;
    fn http_chunk_append_file_ref(
        r: *mut request_st,
        sce: *mut stat_cache_entry,
    ) -> libc::c_int;
    fn http_chunk_append_file_ref_range(
        r: *mut request_st,
        sce: *mut stat_cache_entry,
        offset: off_t,
        len: off_t,
    );
    fn http_chunk_close(r: *mut request_st);
    fn http_cgi_local_redir(r: *mut request_st) -> handler_t;
    fn http_date_time_to_str(
        s: *mut libc::c_char,
        sz: size_t,
        t: unix_time64_t,
    ) -> uint32_t;
    fn http_date_if_modified_since(
        ifmod: *const libc::c_char,
        ifmodlen: uint32_t,
        lmtime: unix_time64_t,
    ) -> libc::c_int;
    fn http_etag_matches(
        etag: *const buffer,
        matches: *const libc::c_char,
        weak_ok: libc::c_int,
    ) -> libc::c_int;
    fn http_header_hkey_get(s: *const libc::c_char, slen: size_t) -> http_header_e;
    fn http_header_str_to_code(s: *const libc::c_char) -> libc::c_int;
    fn http_header_str_contains_token(
        s: *const libc::c_char,
        slen: uint32_t,
        m: *const libc::c_char,
        mlen: uint32_t,
    ) -> libc::c_int;
    fn http_header_response_get(
        r: *const request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn http_header_response_set_ptr(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn http_header_response_unset(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    );
    fn http_header_response_set(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
        v: *const libc::c_char,
        vlen: uint32_t,
    );
    fn http_header_response_insert(
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
    fn http_header_env_append(
        r: *mut request_st,
        k: *const libc::c_char,
        klen: uint32_t,
        v: *const libc::c_char,
        vlen: uint32_t,
    );
    fn http_header_parse_hoff(
        n: *const libc::c_char,
        clen: uint32_t,
        hoff: *mut libc::c_ushort,
    ) -> uint32_t;
    fn stat_cache_etag_get(
        sce: *mut stat_cache_entry,
        flags: libc::c_int,
    ) -> *const buffer;
    fn stat_cache_content_type_get_by_ext(
        sce: *mut stat_cache_entry,
        mimetypes: *const array,
    ) -> *const buffer;
    fn stat_cache_path_contains_symlink(
        name: *const buffer,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn stat_cache_get_entry_open(
        name: *const buffer,
        symlinks: libc::c_int,
    ) -> *mut stat_cache_entry;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
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
pub type __time_t = libc::c_long;
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
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
pub type buffer_encoding_t = libc::c_uint;
pub const ENCODING_MINIMAL_XML: buffer_encoding_t = 3;
pub const ENCODING_HTML: buffer_encoding_t = 2;
pub const ENCODING_REL_URI_PART: buffer_encoding_t = 1;
pub const ENCODING_REL_URI: buffer_encoding_t = 0;
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
pub type http_response_send_1xx_cb = Option::<
    unsafe extern "C" fn(*mut request_st, *mut connection) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtime_cache_type {
    pub mtime: unix_time64_t,
    pub str_0: buffer,
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
#[inline]
unsafe extern "C" fn buffer_clear(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn buffer_reset(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
    if (*b).size > 4096 as libc::c_int as libc::c_uint {
        buffer_free_ptr(b);
    }
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
unsafe extern "C" fn chunkqueue_length(mut cq: *const chunkqueue) -> off_t {
    return (*cq).bytes_in - (*cq).bytes_out;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn http_response_buffer_append_authority(
    r: *mut request_st,
    o: *mut buffer,
) -> libc::c_int {
    if buffer_is_blank(&mut (*r).uri.authority) == 0 {
        buffer_append_buffer(o, &mut (*r).uri.authority);
    } else {
        let mut our_addr: sock_addr = sock_addr {
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
        let mut our_addr_len: socklen_t = 0;
        our_addr.plain.sa_family = 0 as libc::c_int as sa_family_t;
        our_addr_len = ::std::mem::size_of::<sock_addr>() as libc::c_ulong as socklen_t;
        if -(1 as libc::c_int)
            == getsockname(
                (*(*r).con).fd,
                __SOCKADDR_ARG {
                    __sockaddr__: &mut our_addr as *mut sock_addr as *mut sockaddr,
                },
                &mut our_addr_len,
            )
            || our_addr_len
                > ::std::mem::size_of::<sock_addr>() as libc::c_ulong as socklen_t
        {
            (*r).http_status = 500 as libc::c_int;
            log_perror(
                (*r).conf.errh,
                b"src/http-header-glue.c\0" as *const u8 as *const libc::c_char,
                49 as libc::c_int as libc::c_uint,
                b"can't get sockname\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if our_addr.plain.sa_family as libc::c_int == 2 as libc::c_int
            && our_addr.ipv4.sin_addr.s_addr
                == __bswap_32(0x7f000001 as libc::c_int as in_addr_t)
        {
            static mut lhost: [libc::c_char; 32] = [0; 32];
            static mut lhost_len: size_t = 0 as libc::c_int as size_t;
            if 0 as libc::c_int as libc::c_ulong != lhost_len {
                buffer_append_string_len(o, lhost.as_mut_ptr(), lhost_len);
            } else {
                let mut olen: size_t = buffer_clen(o) as size_t;
                if 0 as libc::c_int
                    == sock_addr_nameinfo_append_buffer(o, &mut our_addr, (*r).conf.errh)
                {
                    lhost_len = (buffer_clen(o) as libc::c_ulong).wrapping_sub(olen);
                    if lhost_len
                        < ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong
                    {
                        memcpy(
                            lhost.as_mut_ptr() as *mut libc::c_void,
                            ((*o).ptr).offset(olen as isize) as *const libc::c_void,
                            lhost_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        );
                    } else {
                        lhost_len = 0 as libc::c_int as size_t;
                    }
                } else {
                    lhost_len = (::std::mem::size_of::<[libc::c_char; 10]>()
                        as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    memcpy(
                        lhost.as_mut_ptr() as *mut libc::c_void,
                        b"localhost\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        lhost_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    );
                    buffer_append_string_len(o, lhost.as_mut_ptr(), lhost_len);
                }
            }
        } else if buffer_is_blank((*r).server_name) == 0 {
            buffer_append_buffer(o, (*r).server_name);
        } else if 0 as libc::c_int
                != sock_addr_nameinfo_append_buffer(o, &mut our_addr, (*r).conf.errh)
            {
            (*r).http_status = 500 as libc::c_int;
            return -(1 as libc::c_int);
        }
        let mut listen_port: libc::c_ushort = sock_addr_get_port(&mut our_addr);
        let mut default_port: libc::c_ushort = 80 as libc::c_int as libc::c_ushort;
        if buffer_eq_slen(
            &mut (*r).uri.scheme,
            b"https\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            default_port = 443 as libc::c_int as libc::c_ushort;
        }
        if 0 as libc::c_int == listen_port as libc::c_int {
            listen_port = (*(*(*r).con).srv).srvconf.port;
        }
        if default_port as libc::c_int != listen_port as libc::c_int {
            buffer_append_string_len(
                o,
                b":\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            buffer_append_int(o, listen_port as intmax_t);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn http_response_redirect_to_directory(
    r: *mut request_st,
    mut status: libc::c_int,
) -> libc::c_int {
    let mut o: *mut buffer = (*r).tmp_buf;
    buffer_clear(o);
    if (*(*(*r).con).srv).srvconf.absolute_dir_redirect != 0 {
        buffer_append_str2(
            o,
            (*r).uri.scheme.ptr,
            buffer_clen(&mut (*r).uri.scheme) as size_t,
            b"://\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        if 0 as libc::c_int != http_response_buffer_append_authority(r, o) {
            return -(1 as libc::c_int);
        }
    }
    let mut vb: *mut buffer = 0 as *mut buffer;
    if status >= 300 as libc::c_int {
        (*r).http_status = status;
        (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
        vb = http_header_response_set_ptr(
            r,
            HTTP_HEADER_LOCATION,
            b"Location\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    } else {
        vb = http_header_response_set_ptr(
            r,
            HTTP_HEADER_CONTENT_LOCATION,
            b"Content-Location\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    buffer_copy_buffer(vb, o);
    buffer_append_string_encoded(
        vb,
        (*r).uri.path.ptr,
        buffer_clen(&mut (*r).uri.path) as size_t,
        ENCODING_REL_URI,
    );
    buffer_append_string_len(
        vb,
        b"/\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    if buffer_is_blank(&mut (*r).uri.query) == 0 {
        buffer_append_str2(
            vb,
            b"?\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            (*r).uri.query.ptr,
            buffer_clen(&mut (*r).uri.query) as size_t,
        );
    }
    return 0 as libc::c_int;
}
static mut mtime_cache: [mtime_cache_type; 16] = [mtime_cache_type {
    mtime: 0,
    str_0: buffer {
        ptr: 0 as *const libc::c_char as *mut libc::c_char,
        used: 0,
        size: 0,
    },
}; 16];
static mut mtime_cache_str: [[libc::c_char; 30]; 16] = [[0; 30]; 16];
#[no_mangle]
#[cold]
pub unsafe extern "C" fn strftime_cache_reset() {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        mtime_cache[i as usize].mtime = -(1 as libc::c_int) as unix_time64_t;
        mtime_cache[i as usize].str_0.ptr = (mtime_cache_str[i as usize]).as_mut_ptr();
        mtime_cache[i as usize]
            .str_0
            .used = ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
            as uint32_t;
        mtime_cache[i as usize]
            .str_0
            .size = ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
            as uint32_t;
        i += 1;
    }
}
unsafe extern "C" fn strftime_cache_get(last_mod: unix_time64_t) -> *const buffer {
    static mut mtime_cache_idx: libc::c_int = 0;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < 16 as libc::c_int {
        if mtime_cache[j as usize].mtime == last_mod {
            return &mut (*mtime_cache.as_mut_ptr().offset(j as isize)).str_0;
        }
        j += 1;
    }
    mtime_cache_idx += 1;
    if mtime_cache_idx == 16 as libc::c_int {
        mtime_cache_idx = 0 as libc::c_int;
    }
    let i: libc::c_int = mtime_cache_idx;
    mtime_cache[i as usize].mtime = last_mod;
    http_date_time_to_str(
        mtime_cache[i as usize].str_0.ptr,
        ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
        mtime_cache[i as usize].mtime,
    );
    return &mut (*mtime_cache.as_mut_ptr().offset(i as isize)).str_0;
}
#[no_mangle]
pub unsafe extern "C" fn http_response_set_last_modified(
    r: *mut request_st,
    lmtime: unix_time64_t,
) -> *const buffer {
    let vb: *mut buffer = http_header_response_set_ptr(
        r,
        HTTP_HEADER_LAST_MODIFIED,
        b"Last-Modified\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    buffer_copy_buffer(vb, strftime_cache_get(lmtime));
    return vb;
}
unsafe extern "C" fn http_response_maybe_cachable(r: *const request_st) -> libc::c_int {
    return ((*r).rqst_htags
        & ((1 as libc::c_ulong) << HTTP_HEADER_IF_NONE_MATCH as libc::c_int
            | (1 as libc::c_ulong) << HTTP_HEADER_IF_MODIFIED_SINCE as libc::c_int))
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn http_response_handle_cachable(
    r: *mut request_st,
    mut lmod: *const buffer,
    lmtime: unix_time64_t,
) -> libc::c_int {
    if http_response_maybe_cachable(r) == 0 {
        return HANDLER_GO_ON as libc::c_int;
    }
    let mut vb: *const buffer = 0 as *const buffer;
    let mut etag: *const buffer = 0 as *const buffer;
    vb = http_header_request_get(
        r,
        HTTP_HEADER_IF_NONE_MATCH,
        b"If-None-Match\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if !vb.is_null()
        && {
            etag = http_header_response_get(
                r,
                HTTP_HEADER_ETAG,
                b"ETag\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            );
            !etag.is_null()
        }
    {
        let mut range_request: libc::c_int = (0 as libc::c_int as libc::c_ulong
            != (*r).rqst_htags
                & (1 as libc::c_ulong) << HTTP_HEADER_RANGE as libc::c_int)
            as libc::c_int;
        if http_etag_matches(etag, (*vb).ptr, (range_request == 0) as libc::c_int) != 0 {
            if (*r).http_method as libc::c_int <= HTTP_METHOD_HEAD as libc::c_int {
                (*r).http_status = 304 as libc::c_int;
                return HANDLER_FINISHED as libc::c_int;
            } else {
                (*r).http_status = 412 as libc::c_int;
                (*r).handler_module = 0 as *const plugin;
                return HANDLER_FINISHED as libc::c_int;
            }
        }
    } else if (*r).http_method as libc::c_int <= HTTP_METHOD_HEAD as libc::c_int
            && {
                vb = http_header_request_get(
                    r,
                    HTTP_HEADER_IF_MODIFIED_SINCE,
                    b"If-Modified-Since\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                );
                !vb.is_null()
            }
            && (!lmod.is_null()
                || {
                    lmod = http_header_response_get(
                        r,
                        HTTP_HEADER_LAST_MODIFIED,
                        b"Last-Modified\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    );
                    !lmod.is_null()
                })
        {
        if buffer_is_equal(lmod, vb) != 0
            || http_date_if_modified_since((*vb).ptr, buffer_clen(vb), lmtime) == 0
        {
            (*r).http_status = 304 as libc::c_int;
            return HANDLER_FINISHED as libc::c_int;
        }
    }
    return HANDLER_GO_ON as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn http_response_body_clear(
    r: *mut request_st,
    mut preserve_length: libc::c_int,
) {
    (*r).resp_send_chunked = 0 as libc::c_int as libc::c_char;
    (*r).resp_body_scratchpad = -(1 as libc::c_int) as off_t;
    if (*r).resp_htags
        & (1 as libc::c_ulong) << HTTP_HEADER_TRANSFER_ENCODING as libc::c_int != 0
    {
        http_header_response_unset(
            r,
            HTTP_HEADER_TRANSFER_ENCODING,
            b"Transfer-Encoding\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    if preserve_length == 0 {
        if (*r).resp_htags
            & (1 as libc::c_ulong) << HTTP_HEADER_CONTENT_LENGTH as libc::c_int != 0
        {
            http_header_response_unset(
                r,
                HTTP_HEADER_CONTENT_LENGTH,
                b"Content-Length\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            );
        }
        (*r).resp_decode_chunked = 0 as libc::c_int as libc::c_char;
        if !((*r).gw_dechunk).is_null() {
            free((*(*r).gw_dechunk).b.ptr as *mut libc::c_void);
            free((*r).gw_dechunk as *mut libc::c_void);
            (*r).gw_dechunk = 0 as *mut response_dechunk;
        }
    }
    chunkqueue_reset(&mut (*r).write_queue);
}
unsafe extern "C" fn http_response_header_clear(r: *mut request_st) {
    (*r).http_status = 0 as libc::c_int;
    (*r).resp_htags = 0 as libc::c_int as uint64_t;
    (*r).resp_header_len = 0 as libc::c_int as uint32_t;
    (*r).resp_header_repeated = 0 as libc::c_int as libc::c_char;
    array_reset_data_strings(&mut (*r).resp_headers);
    (*r).resp_send_chunked = 0 as libc::c_int as libc::c_char;
    (*r).resp_decode_chunked = 0 as libc::c_int as libc::c_char;
    (*r).resp_body_scratchpad = -(1 as libc::c_int) as off_t;
    if !((*r).gw_dechunk).is_null() {
        free((*(*r).gw_dechunk).b.ptr as *mut libc::c_void);
        free((*r).gw_dechunk as *mut libc::c_void);
        (*r).gw_dechunk = 0 as *mut response_dechunk;
    }
}
#[no_mangle]
pub unsafe extern "C" fn http_response_reset(r: *mut request_st) {
    (*r).http_status = 0 as libc::c_int;
    (*(*r).con).is_writable = 1 as libc::c_int as libc::c_schar;
    (*r).resp_body_finished = 0 as libc::c_int as libc::c_char;
    (*r).resp_body_started = 0 as libc::c_int as libc::c_char;
    (*r).handler_module = 0 as *const plugin;
    if !((*r).physical.path.ptr).is_null() {
        buffer_clear(&mut (*r).physical.doc_root);
        buffer_clear(&mut (*r).physical.basedir);
        buffer_reset(&mut (*r).physical.path);
        buffer_reset(&mut (*r).physical.rel_path);
    }
    (*r).resp_htags = 0 as libc::c_int as uint64_t;
    (*r).resp_header_len = 0 as libc::c_int as uint32_t;
    (*r).resp_header_repeated = 0 as libc::c_int as libc::c_char;
    array_reset_data_strings(&mut (*r).resp_headers);
    http_response_body_clear(r, 0 as libc::c_int);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn http_response_reqbody_read_error(
    r: *mut request_st,
    mut http_status: libc::c_int,
) -> handler_t {
    (*r).keep_alive = 0 as libc::c_int as int8_t;
    if 0 as libc::c_int as libc::c_uint != (*r).resp_header_len {
        return HANDLER_ERROR;
    }
    http_response_body_clear(r, 0 as libc::c_int);
    (*r).http_status = http_status;
    (*r).handler_module = 0 as *const plugin;
    return HANDLER_FINISHED;
}
static mut octet_stream: buffer = buffer {
    ptr: 0 as *const libc::c_char as *mut libc::c_char,
    used: 0,
    size: 0,
};
#[no_mangle]
pub unsafe extern "C" fn http_response_send_file(
    r: *mut request_st,
    path: *const buffer,
    mut sce: *mut stat_cache_entry,
) {
    if (0 as *mut libc::c_void as *mut stat_cache_entry == sce) as libc::c_int
        as libc::c_long != 0
        || ((*sce).fd < 0 as libc::c_int) as libc::c_int as libc::c_long != 0
            && 0 as libc::c_int as libc::c_long != (*sce).st.st_size
    {
        sce = stat_cache_get_entry_open(path, (*r).conf.follow_symlink as libc::c_int);
        if sce.is_null() {
            (*r)
                .http_status = if *__errno_location() == 2 as libc::c_int {
                404 as libc::c_int
            } else {
                403 as libc::c_int
            };
            log_error(
                (*r).conf.errh,
                b"src/http-header-glue.c\0" as *const u8 as *const libc::c_char,
                326 as libc::c_int as libc::c_uint,
                b"not a regular file: %s -> %s\0" as *const u8 as *const libc::c_char,
                (*r).uri.path.ptr,
                (*path).ptr,
            );
            return;
        }
        if (*sce).fd < 0 as libc::c_int
            && (0 as libc::c_int as libc::c_long != (*sce).st.st_size) as libc::c_int
                as libc::c_long != 0
        {
            (*r)
                .http_status = if *__errno_location() == 2 as libc::c_int {
                404 as libc::c_int
            } else {
                403 as libc::c_int
            };
            if (*r).conf.log_request_handling != 0 {
                log_perror(
                    (*r).conf.errh,
                    b"src/http-header-glue.c\0" as *const u8 as *const libc::c_char,
                    333 as libc::c_int as libc::c_uint,
                    b"file open failed: %s\0" as *const u8 as *const libc::c_char,
                    (*path).ptr,
                );
            }
            return;
        }
    }
    if ((*r).conf.follow_symlink == 0) as libc::c_int as libc::c_long != 0
        && 0 as libc::c_int != stat_cache_path_contains_symlink(path, (*r).conf.errh)
    {
        (*r).http_status = 403 as libc::c_int;
        if (*r).conf.log_request_handling != 0 {
            log_error(
                (*r).conf.errh,
                b"src/http-header-glue.c\0" as *const u8 as *const libc::c_char,
                344 as libc::c_int as libc::c_uint,
                b"-- access denied due symlink restriction\0" as *const u8
                    as *const libc::c_char,
            );
            log_error(
                (*r).conf.errh,
                b"src/http-header-glue.c\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int as libc::c_uint,
                b"Path         : %s\0" as *const u8 as *const libc::c_char,
                (*path).ptr,
            );
        }
        return;
    }
    if !((*sce).st.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_long != 0
    {
        (*r).http_status = 403 as libc::c_int;
        if (*r).conf.log_file_not_found != 0 {
            log_error(
                (*r).conf.errh,
                b"src/http-header-glue.c\0" as *const u8 as *const libc::c_char,
                356 as libc::c_int as libc::c_uint,
                b"not a regular file: %s -> %s\0" as *const u8 as *const libc::c_char,
                (*r).uri.path.ptr,
                (*path).ptr,
            );
        }
        return;
    }
    let mut content_type: *const buffer = 0 as *const buffer;
    if ((*r).resp_htags & (1 as libc::c_ulong) << HTTP_HEADER_CONTENT_TYPE as libc::c_int
        == 0) as libc::c_int as libc::c_long != 0
    {
        content_type = stat_cache_content_type_get_by_ext(sce, (*r).conf.mimetypes);
        if content_type.is_null() as libc::c_int as libc::c_long != 0
            || buffer_is_blank(content_type) as libc::c_long != 0
        {
            content_type = &octet_stream;
        }
        http_header_response_set(
            r,
            HTTP_HEADER_CONTENT_TYPE,
            b"Content-Type\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            (*content_type).ptr,
            buffer_clen(content_type),
        );
    }
    let mut allow_caching: libc::c_int = (content_type != &octet_stream as *const buffer
        && (0 as libc::c_int == (*r).http_status
            || 200 as libc::c_int == (*r).http_status)) as libc::c_int;
    if allow_caching as libc::c_long != 0 {
        if ((*r).resp_htags & (1 as libc::c_ulong) << HTTP_HEADER_ETAG as libc::c_int
            == 0) as libc::c_int as libc::c_long != 0
            && 0 as libc::c_int != (*r).conf.etag_flags as libc::c_int
        {
            let mut etag: *const buffer = stat_cache_etag_get(
                sce,
                (*r).conf.etag_flags as libc::c_int,
            );
            if !etag.is_null() && buffer_is_blank(etag) == 0 {
                http_header_response_set(
                    r,
                    HTTP_HEADER_ETAG,
                    b"ETag\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    (*etag).ptr,
                    buffer_clen(etag),
                );
            }
        }
        let lmod: *const buffer = if ((*r).resp_htags
            & (1 as libc::c_ulong) << HTTP_HEADER_LAST_MODIFIED as libc::c_int == 0)
            as libc::c_int as libc::c_long != 0
        {
            http_response_set_last_modified(r, (*sce).st.st_mtim.tv_sec)
        } else {
            0 as *const buffer
        };
        if http_response_maybe_cachable(r) != 0
            && HANDLER_FINISHED as libc::c_int
                == http_response_handle_cachable(r, lmod, (*sce).st.st_mtim.tv_sec)
        {
            return;
        }
    }
    if 0 as libc::c_int as libc::c_long == (*sce).st.st_size
        || 0 as libc::c_int == http_chunk_append_file_ref(r, sce)
    {
        (*r).http_status = 200 as libc::c_int;
        (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
        buffer_append_int(
            http_header_response_set_ptr(
                r,
                HTTP_HEADER_CONTENT_LENGTH,
                b"Content-Length\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            ),
            (*sce).st.st_size,
        );
    } else {
        (*r).http_status = 500 as libc::c_int;
    };
}
unsafe extern "C" fn http_response_xsendfile(
    r: *mut request_st,
    path: *mut buffer,
    xdocroot: *const array,
) {
    let status: libc::c_int = (*r).http_status;
    let mut valid: libc::c_int = 1 as libc::c_int;
    if (*r).resp_htags
        & (1 as libc::c_ulong) << HTTP_HEADER_CONTENT_LENGTH as libc::c_int != 0
    {
        http_header_response_unset(
            r,
            HTTP_HEADER_CONTENT_LENGTH,
            b"Content-Length\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    buffer_urldecode_path(path);
    if buffer_is_valid_UTF8(path) == 0 {
        log_error(
            (*r).conf.errh,
            b"src/http-header-glue.c\0" as *const u8 as *const libc::c_char,
            440 as libc::c_int as libc::c_uint,
            b"X-Sendfile invalid UTF-8 after url-decode: %s\0" as *const u8
                as *const libc::c_char,
            (*path).ptr,
        );
        if (*r).http_status < 400 as libc::c_int {
            (*r).http_status = 502 as libc::c_int;
            (*r).handler_module = 0 as *const plugin;
        }
        return;
    }
    buffer_path_simplify(path);
    if (*r).conf.force_lowercase_filenames != 0 {
        buffer_to_lower(path);
    }
    if buffer_is_blank(path) != 0 {
        (*r).http_status = 502 as libc::c_int;
        valid = 0 as libc::c_int;
    }
    if !xdocroot.is_null() && (*xdocroot).used != 0 {
        let xval: *const buffer = if (*r).conf.force_lowercase_filenames == 0 {
            array_match_value_prefix(xdocroot, path)
        } else {
            array_match_value_prefix_nc(xdocroot, path)
        };
        if xval.is_null() {
            log_error(
                (*r).conf.errh,
                b"src/http-header-glue.c\0" as *const u8 as *const libc::c_char,
                466 as libc::c_int as libc::c_uint,
                b"X-Sendfile (%s) not under configured x-sendfile-docroot(s)\0"
                    as *const u8 as *const libc::c_char,
                (*path).ptr,
            );
            (*r).http_status = 403 as libc::c_int;
            valid = 0 as libc::c_int;
        }
    }
    if valid != 0 {
        http_response_send_file(r, path, 0 as *mut stat_cache_entry);
    }
    if (*r).http_status >= 400 as libc::c_int && status < 300 as libc::c_int {
        (*r).handler_module = 0 as *const plugin;
    } else if 0 as libc::c_int != status && 200 as libc::c_int != status {
        (*r).http_status = status;
    }
}
unsafe extern "C" fn http_response_xsendfile2(
    r: *mut request_st,
    value: *const buffer,
    xdocroot: *const array,
) {
    let mut current_block: u64;
    let mut pos: *const libc::c_char = (*value).ptr;
    let b: *mut buffer = (*r).tmp_buf;
    let status: libc::c_int = (*r).http_status;
    if (*r).resp_htags
        & (1 as libc::c_ulong) << HTTP_HEADER_CONTENT_LENGTH as libc::c_int != 0
    {
        http_header_response_unset(
            r,
            HTTP_HEADER_CONTENT_LENGTH,
            b"Content-Length\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    while *pos != 0 {
        let mut filename: *const libc::c_char = 0 as *const libc::c_char;
        let mut range: *const libc::c_char = 0 as *const libc::c_char;
        let mut sce: *mut stat_cache_entry = 0 as *mut stat_cache_entry;
        let mut begin_range: off_t = 0;
        let mut end_range: off_t = 0;
        let mut range_len: off_t = 0;
        while ' ' as i32 == *pos as libc::c_int {
            pos = pos.offset(1);
        }
        if *pos == 0 {
            break;
        }
        filename = pos;
        range = strchr(pos, ' ' as i32);
        if range.is_null() {
            log_error(
                (*r).conf.errh,
                b"src/http-header-glue.c\0" as *const u8 as *const libc::c_char,
                505 as libc::c_int as libc::c_uint,
                b"Couldn't find range after filename: %s\0" as *const u8
                    as *const libc::c_char,
                filename,
            );
            (*r).http_status = 502 as libc::c_int;
            break;
        } else {
            buffer_copy_string_len(
                b,
                filename,
                range.offset_from(filename) as libc::c_long as size_t,
            );
            range = range.offset(1);
            pos = range;
            while *pos as libc::c_int != 0 && *pos as libc::c_int != ' ' as i32
                && *pos as libc::c_int != ',' as i32
            {
                pos = pos.offset(1);
            }
            buffer_urldecode_path(b);
            if buffer_is_valid_UTF8(b) == 0 {
                log_error(
                    (*r).conf.errh,
                    b"src/http-header-glue.c\0" as *const u8 as *const libc::c_char,
                    517 as libc::c_int as libc::c_uint,
                    b"X-Sendfile2 invalid UTF-8 after url-decode: %s\0" as *const u8
                        as *const libc::c_char,
                    (*b).ptr,
                );
                (*r).http_status = 502 as libc::c_int;
                break;
            } else {
                buffer_path_simplify(b);
                if (*r).conf.force_lowercase_filenames != 0 {
                    buffer_to_lower(b);
                }
                if buffer_is_blank(b) != 0 {
                    (*r).http_status = 502 as libc::c_int;
                    break;
                } else {
                    if !xdocroot.is_null() && (*xdocroot).used != 0 {
                        let xval: *const buffer = if (*r).conf.force_lowercase_filenames
                            == 0
                        {
                            array_match_value_prefix(xdocroot, b)
                        } else {
                            array_match_value_prefix_nc(xdocroot, b)
                        };
                        if xval.is_null() {
                            log_error(
                                (*r).conf.errh,
                                b"src/http-header-glue.c\0" as *const u8
                                    as *const libc::c_char,
                                535 as libc::c_int as libc::c_uint,
                                b"X-Sendfile2 (%s) not under configured x-sendfile-docroot(s)\0"
                                    as *const u8 as *const libc::c_char,
                                (*b).ptr,
                            );
                            (*r).http_status = 403 as libc::c_int;
                            break;
                        }
                    }
                    sce = stat_cache_get_entry_open(
                        b,
                        (*r).conf.follow_symlink as libc::c_int,
                    );
                    if sce.is_null() {
                        log_error(
                            (*r).conf.errh,
                            b"src/http-header-glue.c\0" as *const u8
                                as *const libc::c_char,
                            545 as libc::c_int as libc::c_uint,
                            b"send-file error: couldn't get stat_cache entry for X-Sendfile2: %s\0"
                                as *const u8 as *const libc::c_char,
                            (*b).ptr,
                        );
                        (*r).http_status = 404 as libc::c_int;
                        break;
                    } else if !((*sce).st.st_mode
                            & 0o170000 as libc::c_int as libc::c_uint
                            == 0o100000 as libc::c_int as libc::c_uint)
                        {
                        log_error(
                            (*r).conf.errh,
                            b"src/http-header-glue.c\0" as *const u8
                                as *const libc::c_char,
                            551 as libc::c_int as libc::c_uint,
                            b"send-file error: wrong filetype for X-Sendfile2: %s\0"
                                as *const u8 as *const libc::c_char,
                            (*b).ptr,
                        );
                        (*r).http_status = 502 as libc::c_int;
                        break;
                    } else {
                        end_range = (*sce).st.st_size - 1 as libc::c_int as libc::c_long;
                        let mut rpos: *mut libc::c_char = 0 as *mut libc::c_char;
                        *__errno_location() = 0 as libc::c_int;
                        begin_range = strtoll(range, &mut rpos, 10 as libc::c_int)
                            as off_t;
                        if !(*__errno_location() != 0 as libc::c_int
                            || begin_range < 0 as libc::c_int as libc::c_long
                            || rpos == range as *mut libc::c_char)
                        {
                            let fresh0 = rpos;
                            rpos = rpos.offset(1);
                            if !('-' as i32 != *fresh0 as libc::c_int) {
                                if rpos != pos as *mut libc::c_char {
                                    range = rpos;
                                    end_range = strtoll(range, &mut rpos, 10 as libc::c_int)
                                        as off_t;
                                    if *__errno_location() != 0 as libc::c_int
                                        || end_range < 0 as libc::c_int as libc::c_long
                                        || rpos == range as *mut libc::c_char
                                    {
                                        current_block = 11675972470164985936;
                                    } else {
                                        current_block = 7420279277351916581;
                                    }
                                } else {
                                    current_block = 7420279277351916581;
                                }
                                match current_block {
                                    11675972470164985936 => {}
                                    _ => {
                                        if !(rpos != pos as *mut libc::c_char) {
                                            while *pos as libc::c_int == ' ' as i32 {
                                                pos = pos.offset(1);
                                            }
                                            if *pos as libc::c_int != '\u{0}' as i32
                                                && *pos as libc::c_int != ',' as i32
                                            {
                                                (*r).http_status = 502 as libc::c_int;
                                                break;
                                            } else {
                                                range_len = end_range - begin_range
                                                    + 1 as libc::c_int as libc::c_long;
                                                if range_len < 0 as libc::c_int as libc::c_long {
                                                    (*r).http_status = 502 as libc::c_int;
                                                    break;
                                                } else {
                                                    if range_len != 0 as libc::c_int as libc::c_long {
                                                        http_chunk_append_file_ref_range(
                                                            r,
                                                            sce,
                                                            begin_range,
                                                            range_len,
                                                        );
                                                    }
                                                    if *pos as libc::c_int == ',' as i32 {
                                                        pos = pos.offset(1);
                                                    }
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        log_error(
                            (*r).conf.errh,
                            b"src/http-header-glue.c\0" as *const u8
                                as *const libc::c_char,
                            578 as libc::c_int as libc::c_uint,
                            b"Couldn't decode range after filename: %s\0" as *const u8
                                as *const libc::c_char,
                            filename,
                        );
                        (*r).http_status = 502 as libc::c_int;
                        break;
                    }
                }
            }
        }
    }
    if (*r).http_status >= 400 as libc::c_int && status < 300 as libc::c_int {
        (*r).handler_module = 0 as *const plugin;
    } else if 0 as libc::c_int != status && 200 as libc::c_int != status {
        (*r).http_status = status;
    }
}
#[no_mangle]
pub unsafe extern "C" fn http_response_backend_error(r: *mut request_st) {
    if (*r).resp_body_started != 0 {
        (*r).handler_module = 0 as *const plugin;
        (*r).keep_alive = 0 as libc::c_int as int8_t;
        (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn http_response_backend_done(r: *mut request_st) {
    let mut current_block_8: u64;
    match (*r).state as libc::c_uint {
        5 | 4 => {
            if (*r).resp_body_started == 0 {
                if (*r).http_status < 500 as libc::c_int
                    && (*r).http_status != 400 as libc::c_int
                {
                    (*r).http_status = 500 as libc::c_int;
                }
                (*r).handler_module = 0 as *const plugin;
                current_block_8 = 1856101646708284338;
            } else {
                current_block_8 = 13244265311439482134;
            }
        }
        7 => {
            current_block_8 = 13244265311439482134;
        }
        _ => {
            current_block_8 = 1856101646708284338;
        }
    }
    match current_block_8 {
        13244265311439482134 => {
            if (*r).resp_body_finished == 0 {
                if (*r).http_version as libc::c_int == HTTP_VERSION_1_1 as libc::c_int {
                    http_chunk_close(r);
                }
                (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn http_response_upgrade_read_body_unknown(r: *mut request_st) {
    if (*r).conf.stream_request_body as libc::c_int
        & (1 as libc::c_int) << 0 as libc::c_int == 0
    {
        (*r)
            .conf
            .stream_request_body = ((*r).conf.stream_request_body as libc::c_int
            | ((1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 0 as libc::c_int)) as libc::c_ushort;
    }
    if (*r).conf.stream_response_body as libc::c_int
        & (1 as libc::c_int) << 0 as libc::c_int == 0
    {
        (*r)
            .conf
            .stream_response_body = ((*r).conf.stream_response_body as libc::c_int
            | ((1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 0 as libc::c_int)) as libc::c_ushort;
    }
    (*r)
        .conf
        .stream_request_body = ((*r).conf.stream_request_body as libc::c_int
        | (1 as libc::c_int) << 15 as libc::c_int) as libc::c_ushort;
    (*r).reqbody_length = -(2 as libc::c_int) as off_t;
    (*r).resp_body_scratchpad = -(1 as libc::c_int) as off_t;
    (*r).keep_alive = 0 as libc::c_int as int8_t;
}
unsafe extern "C" fn http_response_append_buffer_simple_accum(
    r: *const request_st,
    len: off_t,
) -> libc::c_int {
    return (len < 32768 as libc::c_int as libc::c_long
        && !((*r).write_queue.last).is_null()
        && (*(*r).write_queue.last).file.is_temp != 0) as libc::c_int;
}
unsafe extern "C" fn http_response_append_buffer(
    r: *mut request_st,
    mem: *mut buffer,
    simple_accum: libc::c_int,
) -> libc::c_int {
    if (*r).resp_decode_chunked != 0 {
        return http_chunk_decode_append_buffer(r, mem);
    }
    if (*r).resp_body_scratchpad > 0 as libc::c_int as libc::c_long {
        let mut len: off_t = buffer_clen(mem) as off_t;
        (*r).resp_body_scratchpad -= len;
        if (*r).resp_body_scratchpad > 0 as libc::c_int as libc::c_long {
            if simple_accum != 0 && http_response_append_buffer_simple_accum(r, len) != 0
            {
                (*r).resp_body_scratchpad += len;
                return 0 as libc::c_int;
            }
        } else {
            (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
            if ((*r).resp_body_scratchpad < 0 as libc::c_int as libc::c_long)
                as libc::c_int as libc::c_long != 0
            {
                len += (*r).resp_body_scratchpad;
                (*r).resp_body_scratchpad = 0 as libc::c_int as off_t;
                buffer_truncate(mem, len as uint32_t);
            }
        }
    } else if 0 as libc::c_int as libc::c_long == (*r).resp_body_scratchpad {
        buffer_clear(mem);
        return 0 as libc::c_int;
    } else {
        if simple_accum != 0
            && http_response_append_buffer_simple_accum(r, buffer_clen(mem) as off_t)
                != 0
        {
            return 0 as libc::c_int;
        }
    }
    return http_chunk_append_buffer(r, mem);
}
unsafe extern "C" fn http_response_append_splice(
    r: *mut request_st,
    opts: *mut http_response_opts,
    b: *mut buffer,
    fd: libc::c_int,
    mut toread: libc::c_uint,
) -> libc::c_int {
    if (*r).resp_body_scratchpad >= toread as libc::c_long
        && (toread > 32768 as libc::c_int as libc::c_uint
            || toread >= 8192 as libc::c_int as libc::c_uint
                && !((*r).write_queue.last).is_null()
                && (*(*r).write_queue.last).file.is_temp != 0)
    {
        if buffer_is_blank(b) == 0 {
            let mut rc: libc::c_int = http_response_append_buffer(
                r,
                b,
                0 as libc::c_int,
            );
            chunk_buffer_yield(b);
            if (0 as libc::c_int != rc) as libc::c_int as libc::c_long != 0 {
                return -(1 as libc::c_int);
            }
        }
        let mut n: ssize_t = if (*opts).fdfmt == 0o140000 as libc::c_int {
            chunkqueue_append_splice_sock_tempfile(
                &mut (*r).write_queue,
                fd,
                toread,
                (*r).conf.errh,
            )
        } else {
            chunkqueue_append_splice_pipe_tempfile(
                &mut (*r).write_queue,
                fd,
                toread,
                (*r).conf.errh,
            )
        };
        if (n >= 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long != 0 {
            (*r).resp_body_scratchpad -= n;
            if 0 as libc::c_int as libc::c_long == (*r).resp_body_scratchpad {
                (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
            }
            return 1 as libc::c_int;
        } else {
            if n != -(22 as libc::c_int) as libc::c_long {
                return -(1 as libc::c_int);
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn http_response_append_mem(
    r: *mut request_st,
    mem: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    if (*r).resp_decode_chunked != 0 {
        return http_chunk_decode_append_mem(r, mem, len);
    }
    if (*r).resp_body_scratchpad > 0 as libc::c_int as libc::c_long {
        (*r).resp_body_scratchpad -= len as off_t;
        if (*r).resp_body_scratchpad <= 0 as libc::c_int as libc::c_long {
            (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
            if ((*r).resp_body_scratchpad < 0 as libc::c_int as libc::c_long)
                as libc::c_int as libc::c_long != 0
            {
                len = ((*r).resp_body_scratchpad + len as off_t) as size_t;
                (*r).resp_body_scratchpad = 0 as libc::c_int as off_t;
            }
        }
    } else if 0 as libc::c_int as libc::c_long == (*r).resp_body_scratchpad {
        return 0 as libc::c_int
    }
    return http_chunk_append_mem(r, mem, len);
}
#[no_mangle]
pub unsafe extern "C" fn http_response_transfer_cqlen(
    r: *mut request_st,
    cq: *mut chunkqueue,
    mut len: size_t,
) -> libc::c_int {
    if 0 as libc::c_int as libc::c_ulong == len {
        return 0 as libc::c_int;
    }
    if ((*r).resp_decode_chunked == 0) as libc::c_int as libc::c_long != 0 {
        let olen: size_t = len;
        if (*r).resp_body_scratchpad >= 0 as libc::c_int as libc::c_long {
            (*r).resp_body_scratchpad -= len as off_t;
            if ((*r).resp_body_scratchpad < 0 as libc::c_int as libc::c_long)
                as libc::c_int as libc::c_long != 0
            {
                len = ((*r).resp_body_scratchpad + len as off_t) as size_t;
                (*r).resp_body_scratchpad = 0 as libc::c_int as off_t;
            }
        }
        let mut rc: libc::c_int = http_chunk_transfer_cqlen(r, cq, len);
        if (0 as libc::c_int != rc) as libc::c_int as libc::c_long != 0 {
            return -(1 as libc::c_int);
        }
        if (olen != len) as libc::c_int as libc::c_long != 0 {
            chunkqueue_mark_written(cq, olen.wrapping_sub(len) as off_t);
        }
    } else {
        let mut remain: uint32_t = len as uint32_t;
        let mut wr: uint32_t = 0;
        let mut c: *const chunk = (*cq).first;
        while !c.is_null() && remain != 0 {
            wr = (buffer_clen((*c).mem) as libc::c_long - (*c).offset) as uint32_t;
            if wr > remain {
                wr = remain;
            }
            if 0 as libc::c_int
                != http_chunk_decode_append_mem(
                    r,
                    ((*(*c).mem).ptr).offset((*c).offset as isize),
                    wr as size_t,
                )
            {
                return -(1 as libc::c_int);
            }
            c = (*c).next;
            remain = (remain as libc::c_uint).wrapping_sub(wr) as uint32_t as uint32_t;
        }
        chunkqueue_mark_written(cq, len as off_t);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn http_response_process_headers(
    r: *mut request_st,
    opts: *mut http_response_opts,
    s: *mut libc::c_char,
    mut hoff: *const libc::c_ushort,
    is_nph: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 1 as libc::c_int;
    if is_nph != 0 {
        if *s.offset(12 as libc::c_int as isize) as libc::c_int == '\r' as i32
            || *s.offset(12 as libc::c_int as isize) as libc::c_int == '\n' as i32
        {
            *s.offset(12 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
        }
        if (*s.offset(5 as libc::c_int as isize) as libc::c_int == '1' as i32
            || (*opts).backend != BACKEND_PROXY as libc::c_int)
            && *s.offset(6 as libc::c_int as isize) as libc::c_int == '.' as i32
            && (*s.offset(7 as libc::c_int as isize) as libc::c_int == '1' as i32
                || *s.offset(7 as libc::c_int as isize) as libc::c_int == '0' as i32)
            && *s.offset(8 as libc::c_int as isize) as libc::c_int == ' ' as i32
        {
            let mut status: libc::c_int = http_header_str_to_code(
                s.offset(9 as libc::c_int as isize),
            );
            if status >= 100 as libc::c_int && status < 1000 as libc::c_int {
                (*r).http_status = status;
                (*opts).local_redir = 0 as libc::c_int as uint8_t;
                i = 2 as libc::c_int;
            }
        }
        if 0 as libc::c_int == (*r).http_status {
            log_error(
                (*r).conf.errh,
                b"src/http-header-glue.c\0" as *const u8 as *const libc::c_char,
                874 as libc::c_int as libc::c_uint,
                b"invalid HTTP status line: %s\0" as *const u8 as *const libc::c_char,
                s,
            );
            (*r).http_status = 502 as libc::c_int;
            (*r).handler_module = 0 as *const plugin;
            return 0 as libc::c_int;
        }
    } else if ((*opts).backend == BACKEND_PROXY as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
        (*r).http_status = 502 as libc::c_int;
        (*r).handler_module = 0 as *const plugin;
        return 0 as libc::c_int;
    }
    let mut current_block_64: u64;
    while i < *hoff.offset(0 as libc::c_int as isize) as libc::c_int {
        let mut k: *const libc::c_char = s
            .offset(*hoff.offset(i as isize) as libc::c_int as isize);
        let mut value: *const libc::c_char = 0 as *const libc::c_char;
        let mut end: *mut libc::c_char = s
            .offset(
                *hoff.offset((i + 1 as libc::c_int) as isize) as libc::c_int as isize,
            )
            .offset(-(1 as libc::c_int as isize));
        value = memchr(
            k as *const libc::c_void,
            ':' as i32,
            end.offset_from(k) as libc::c_long as libc::c_ulong,
        ) as *const libc::c_char;
        if !value.is_null() {
            let klen: uint32_t = value.offset_from(k) as libc::c_long as uint32_t;
            if !(0 as libc::c_int as libc::c_uint == klen) {
                let id: http_header_e = http_header_hkey_get(k, klen as size_t);
                loop {
                    value = value.offset(1);
                    if !(*value as libc::c_int == ' ' as i32
                        || *value as libc::c_int == '\t' as i32)
                    {
                        break;
                    }
                }
                if end > value as *mut libc::c_char
                    && *end.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == '\r' as i32
                {
                    end = end.offset(-1);
                }
                if (*opts).authorizer != 0
                    && (0 as libc::c_int == (*r).http_status
                        || 200 as libc::c_int == (*r).http_status)
                {
                    if id as libc::c_uint
                        == HTTP_HEADER_STATUS as libc::c_int as libc::c_uint
                    {
                        *end
                            .offset(
                                0 as libc::c_int as isize,
                            ) = '\u{0}' as i32 as libc::c_char;
                        let mut status_0: libc::c_int = http_header_str_to_code(value);
                        if status_0 >= 100 as libc::c_int
                            && status_0 < 1000 as libc::c_int
                        {
                            (*r).http_status = status_0;
                            (*opts).local_redir = 0 as libc::c_int as uint8_t;
                        } else {
                            (*r).http_status = 502 as libc::c_int;
                            break;
                        }
                    } else if id as libc::c_uint
                            == HTTP_HEADER_OTHER as libc::c_int as libc::c_uint
                            && klen > 9 as libc::c_int as libc::c_uint
                            && *k.offset(0 as libc::c_int as isize) as libc::c_int
                                & 0xdf as libc::c_int == 'V' as i32
                            && buffer_eq_icase_ssn(
                                k,
                                b"Variable-\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 10]>()
                                    as libc::c_ulong as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                            ) != 0
                        {
                        http_header_env_append(
                            r,
                            k.offset(9 as libc::c_int as isize),
                            klen.wrapping_sub(9 as libc::c_int as libc::c_uint),
                            value,
                            end.offset_from(value) as libc::c_long as uint32_t,
                        );
                    }
                } else {
                    match id as libc::c_uint {
                        45 => {
                            current_block_64 = 12217996289273463446;
                            match current_block_64 {
                                674703269050930462 => {
                                    if *value as libc::c_int == '+' as i32 {
                                        value = value.offset(1);
                                    }
                                    if (*r).resp_decode_chunked == 0
                                        && (*r).resp_htags
                                            & (1 as libc::c_ulong)
                                                << HTTP_HEADER_CONTENT_LENGTH as libc::c_int == 0
                                    {
                                        let mut err: *const libc::c_char = end;
                                        while err > value
                                            && (*err.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                == ' ' as i32
                                                || *err.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                    == '\t' as i32)
                                        {
                                            err = err.offset(-1);
                                        }
                                        if err <= value {
                                            current_block_64 = 224731115979188411;
                                        } else {
                                            let mut vlen: uint32_t = err.offset_from(value)
                                                as libc::c_long as uint32_t;
                                            (*r)
                                                .resp_body_scratchpad = li_restricted_strtoint64(
                                                value,
                                                vlen,
                                                &mut err,
                                            );
                                            if err != value.offset(vlen as isize) {
                                                (*r).resp_body_scratchpad = -(1 as libc::c_int) as off_t;
                                            }
                                            current_block_64 = 14541395414537699361;
                                        }
                                    } else {
                                        current_block_64 = 224731115979188411;
                                    }
                                }
                                9949038703110489483 => {
                                    if (*opts).backend == BACKEND_PROXY as libc::c_int {
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        if http_header_str_contains_token(
                                            value,
                                            end.offset_from(value) as libc::c_long as uint32_t,
                                            b"close\0" as *const u8 as *const libc::c_char,
                                            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                                                as uint32_t)
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                        ) != 0
                                        {
                                            (*r).keep_alive = 0 as libc::c_int as int8_t;
                                        }
                                        if (*r).http_version as libc::c_int
                                            >= HTTP_VERSION_2 as libc::c_int
                                        {
                                            current_block_64 = 224731115979188411;
                                        } else {
                                            current_block_64 = 14541395414537699361;
                                        }
                                    }
                                }
                                10618843670980330763 => {
                                    if (*opts).backend != BACKEND_PROXY as libc::c_int
                                        && (*opts).backend != BACKEND_CGI as libc::c_int
                                    {
                                        current_block_64 = 224731115979188411;
                                    } else if (*r).http_version as libc::c_int
                                            >= HTTP_VERSION_2 as libc::c_int
                                        {
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        current_block_64 = 14541395414537699361;
                                    }
                                }
                                12217996289273463446 => {
                                    if (*opts).backend != BACKEND_PROXY as libc::c_int {
                                        *end
                                            .offset(
                                                0 as libc::c_int as isize,
                                            ) = '\u{0}' as i32 as libc::c_char;
                                        let mut status_1: libc::c_int = http_header_str_to_code(
                                            value,
                                        );
                                        if status_1 >= 100 as libc::c_int
                                            && status_1 < 1000 as libc::c_int
                                        {
                                            (*r).http_status = status_1;
                                            (*opts).local_redir = 0 as libc::c_int as uint8_t;
                                        } else {
                                            (*r).http_status = 502 as libc::c_int;
                                            (*r).handler_module = 0 as *const plugin;
                                        }
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        current_block_64 = 14541395414537699361;
                                    }
                                }
                                17160945220383050795 => {
                                    if (*r).resp_htags
                                        & (1 as libc::c_ulong)
                                            << HTTP_HEADER_CONTENT_LENGTH as libc::c_int != 0
                                    {
                                        (*r).resp_body_scratchpad = -(1 as libc::c_int) as off_t;
                                        http_header_response_unset(
                                            r,
                                            HTTP_HEADER_CONTENT_LENGTH,
                                            b"Content-Length\0" as *const u8 as *const libc::c_char,
                                            (::std::mem::size_of::<[libc::c_char; 15]>()
                                                as libc::c_ulong as uint32_t)
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                        );
                                    }
                                    (*r).resp_decode_chunked = 1 as libc::c_int as libc::c_char;
                                    (*r)
                                        .gw_dechunk = calloc(
                                        1 as libc::c_int as libc::c_ulong,
                                        ::std::mem::size_of::<response_dechunk>() as libc::c_ulong,
                                    ) as *mut response_dechunk;
                                    if ((*r).gw_dechunk).is_null() {
                                        ck_assert_failed(
                                            b"src/http-header-glue.c\0" as *const u8
                                                as *const libc::c_char,
                                            999 as libc::c_int as libc::c_uint,
                                            b"r->gw_dechunk\0" as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    current_block_64 = 224731115979188411;
                                }
                                3809271318960479663 => {
                                    if *k
                                        .offset(
                                            klen.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                        ) as libc::c_int == ' ' as i32
                                        || *k
                                            .offset(
                                                klen.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                            ) as libc::c_int == '\t' as i32
                                    {
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        current_block_64 = 14541395414537699361;
                                    }
                                }
                                _ => {}
                            }
                            match current_block_64 {
                                224731115979188411 => {}
                                _ => {
                                    if end.offset_from(value) as libc::c_long != 0 {
                                        http_header_response_insert(
                                            r,
                                            id,
                                            k,
                                            klen,
                                            value,
                                            end.offset_from(value) as libc::c_long as uint32_t,
                                        );
                                    }
                                }
                            }
                        }
                        49 => {
                            current_block_64 = 10618843670980330763;
                            match current_block_64 {
                                674703269050930462 => {
                                    if *value as libc::c_int == '+' as i32 {
                                        value = value.offset(1);
                                    }
                                    if (*r).resp_decode_chunked == 0
                                        && (*r).resp_htags
                                            & (1 as libc::c_ulong)
                                                << HTTP_HEADER_CONTENT_LENGTH as libc::c_int == 0
                                    {
                                        let mut err: *const libc::c_char = end;
                                        while err > value
                                            && (*err.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                == ' ' as i32
                                                || *err.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                    == '\t' as i32)
                                        {
                                            err = err.offset(-1);
                                        }
                                        if err <= value {
                                            current_block_64 = 224731115979188411;
                                        } else {
                                            let mut vlen: uint32_t = err.offset_from(value)
                                                as libc::c_long as uint32_t;
                                            (*r)
                                                .resp_body_scratchpad = li_restricted_strtoint64(
                                                value,
                                                vlen,
                                                &mut err,
                                            );
                                            if err != value.offset(vlen as isize) {
                                                (*r).resp_body_scratchpad = -(1 as libc::c_int) as off_t;
                                            }
                                            current_block_64 = 14541395414537699361;
                                        }
                                    } else {
                                        current_block_64 = 224731115979188411;
                                    }
                                }
                                9949038703110489483 => {
                                    if (*opts).backend == BACKEND_PROXY as libc::c_int {
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        if http_header_str_contains_token(
                                            value,
                                            end.offset_from(value) as libc::c_long as uint32_t,
                                            b"close\0" as *const u8 as *const libc::c_char,
                                            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                                                as uint32_t)
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                        ) != 0
                                        {
                                            (*r).keep_alive = 0 as libc::c_int as int8_t;
                                        }
                                        if (*r).http_version as libc::c_int
                                            >= HTTP_VERSION_2 as libc::c_int
                                        {
                                            current_block_64 = 224731115979188411;
                                        } else {
                                            current_block_64 = 14541395414537699361;
                                        }
                                    }
                                }
                                10618843670980330763 => {
                                    if (*opts).backend != BACKEND_PROXY as libc::c_int
                                        && (*opts).backend != BACKEND_CGI as libc::c_int
                                    {
                                        current_block_64 = 224731115979188411;
                                    } else if (*r).http_version as libc::c_int
                                            >= HTTP_VERSION_2 as libc::c_int
                                        {
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        current_block_64 = 14541395414537699361;
                                    }
                                }
                                12217996289273463446 => {
                                    if (*opts).backend != BACKEND_PROXY as libc::c_int {
                                        *end
                                            .offset(
                                                0 as libc::c_int as isize,
                                            ) = '\u{0}' as i32 as libc::c_char;
                                        let mut status_1: libc::c_int = http_header_str_to_code(
                                            value,
                                        );
                                        if status_1 >= 100 as libc::c_int
                                            && status_1 < 1000 as libc::c_int
                                        {
                                            (*r).http_status = status_1;
                                            (*opts).local_redir = 0 as libc::c_int as uint8_t;
                                        } else {
                                            (*r).http_status = 502 as libc::c_int;
                                            (*r).handler_module = 0 as *const plugin;
                                        }
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        current_block_64 = 14541395414537699361;
                                    }
                                }
                                17160945220383050795 => {
                                    if (*r).resp_htags
                                        & (1 as libc::c_ulong)
                                            << HTTP_HEADER_CONTENT_LENGTH as libc::c_int != 0
                                    {
                                        (*r).resp_body_scratchpad = -(1 as libc::c_int) as off_t;
                                        http_header_response_unset(
                                            r,
                                            HTTP_HEADER_CONTENT_LENGTH,
                                            b"Content-Length\0" as *const u8 as *const libc::c_char,
                                            (::std::mem::size_of::<[libc::c_char; 15]>()
                                                as libc::c_ulong as uint32_t)
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                        );
                                    }
                                    (*r).resp_decode_chunked = 1 as libc::c_int as libc::c_char;
                                    (*r)
                                        .gw_dechunk = calloc(
                                        1 as libc::c_int as libc::c_ulong,
                                        ::std::mem::size_of::<response_dechunk>() as libc::c_ulong,
                                    ) as *mut response_dechunk;
                                    if ((*r).gw_dechunk).is_null() {
                                        ck_assert_failed(
                                            b"src/http-header-glue.c\0" as *const u8
                                                as *const libc::c_char,
                                            999 as libc::c_int as libc::c_uint,
                                            b"r->gw_dechunk\0" as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    current_block_64 = 224731115979188411;
                                }
                                3809271318960479663 => {
                                    if *k
                                        .offset(
                                            klen.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                        ) as libc::c_int == ' ' as i32
                                        || *k
                                            .offset(
                                                klen.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                            ) as libc::c_int == '\t' as i32
                                    {
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        current_block_64 = 14541395414537699361;
                                    }
                                }
                                _ => {}
                            }
                            match current_block_64 {
                                224731115979188411 => {}
                                _ => {
                                    if end.offset_from(value) as libc::c_long != 0 {
                                        http_header_response_insert(
                                            r,
                                            id,
                                            k,
                                            klen,
                                            value,
                                            end.offset_from(value) as libc::c_long as uint32_t,
                                        );
                                    }
                                }
                            }
                        }
                        12 => {
                            current_block_64 = 9949038703110489483;
                            match current_block_64 {
                                674703269050930462 => {
                                    if *value as libc::c_int == '+' as i32 {
                                        value = value.offset(1);
                                    }
                                    if (*r).resp_decode_chunked == 0
                                        && (*r).resp_htags
                                            & (1 as libc::c_ulong)
                                                << HTTP_HEADER_CONTENT_LENGTH as libc::c_int == 0
                                    {
                                        let mut err: *const libc::c_char = end;
                                        while err > value
                                            && (*err.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                == ' ' as i32
                                                || *err.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                    == '\t' as i32)
                                        {
                                            err = err.offset(-1);
                                        }
                                        if err <= value {
                                            current_block_64 = 224731115979188411;
                                        } else {
                                            let mut vlen: uint32_t = err.offset_from(value)
                                                as libc::c_long as uint32_t;
                                            (*r)
                                                .resp_body_scratchpad = li_restricted_strtoint64(
                                                value,
                                                vlen,
                                                &mut err,
                                            );
                                            if err != value.offset(vlen as isize) {
                                                (*r).resp_body_scratchpad = -(1 as libc::c_int) as off_t;
                                            }
                                            current_block_64 = 14541395414537699361;
                                        }
                                    } else {
                                        current_block_64 = 224731115979188411;
                                    }
                                }
                                9949038703110489483 => {
                                    if (*opts).backend == BACKEND_PROXY as libc::c_int {
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        if http_header_str_contains_token(
                                            value,
                                            end.offset_from(value) as libc::c_long as uint32_t,
                                            b"close\0" as *const u8 as *const libc::c_char,
                                            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                                                as uint32_t)
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                        ) != 0
                                        {
                                            (*r).keep_alive = 0 as libc::c_int as int8_t;
                                        }
                                        if (*r).http_version as libc::c_int
                                            >= HTTP_VERSION_2 as libc::c_int
                                        {
                                            current_block_64 = 224731115979188411;
                                        } else {
                                            current_block_64 = 14541395414537699361;
                                        }
                                    }
                                }
                                10618843670980330763 => {
                                    if (*opts).backend != BACKEND_PROXY as libc::c_int
                                        && (*opts).backend != BACKEND_CGI as libc::c_int
                                    {
                                        current_block_64 = 224731115979188411;
                                    } else if (*r).http_version as libc::c_int
                                            >= HTTP_VERSION_2 as libc::c_int
                                        {
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        current_block_64 = 14541395414537699361;
                                    }
                                }
                                12217996289273463446 => {
                                    if (*opts).backend != BACKEND_PROXY as libc::c_int {
                                        *end
                                            .offset(
                                                0 as libc::c_int as isize,
                                            ) = '\u{0}' as i32 as libc::c_char;
                                        let mut status_1: libc::c_int = http_header_str_to_code(
                                            value,
                                        );
                                        if status_1 >= 100 as libc::c_int
                                            && status_1 < 1000 as libc::c_int
                                        {
                                            (*r).http_status = status_1;
                                            (*opts).local_redir = 0 as libc::c_int as uint8_t;
                                        } else {
                                            (*r).http_status = 502 as libc::c_int;
                                            (*r).handler_module = 0 as *const plugin;
                                        }
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        current_block_64 = 14541395414537699361;
                                    }
                                }
                                17160945220383050795 => {
                                    if (*r).resp_htags
                                        & (1 as libc::c_ulong)
                                            << HTTP_HEADER_CONTENT_LENGTH as libc::c_int != 0
                                    {
                                        (*r).resp_body_scratchpad = -(1 as libc::c_int) as off_t;
                                        http_header_response_unset(
                                            r,
                                            HTTP_HEADER_CONTENT_LENGTH,
                                            b"Content-Length\0" as *const u8 as *const libc::c_char,
                                            (::std::mem::size_of::<[libc::c_char; 15]>()
                                                as libc::c_ulong as uint32_t)
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                        );
                                    }
                                    (*r).resp_decode_chunked = 1 as libc::c_int as libc::c_char;
                                    (*r)
                                        .gw_dechunk = calloc(
                                        1 as libc::c_int as libc::c_ulong,
                                        ::std::mem::size_of::<response_dechunk>() as libc::c_ulong,
                                    ) as *mut response_dechunk;
                                    if ((*r).gw_dechunk).is_null() {
                                        ck_assert_failed(
                                            b"src/http-header-glue.c\0" as *const u8
                                                as *const libc::c_char,
                                            999 as libc::c_int as libc::c_uint,
                                            b"r->gw_dechunk\0" as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    current_block_64 = 224731115979188411;
                                }
                                3809271318960479663 => {
                                    if *k
                                        .offset(
                                            klen.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                        ) as libc::c_int == ' ' as i32
                                        || *k
                                            .offset(
                                                klen.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                            ) as libc::c_int == '\t' as i32
                                    {
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        current_block_64 = 14541395414537699361;
                                    }
                                }
                                _ => {}
                            }
                            match current_block_64 {
                                224731115979188411 => {}
                                _ => {
                                    if end.offset_from(value) as libc::c_long != 0 {
                                        http_header_response_insert(
                                            r,
                                            id,
                                            k,
                                            klen,
                                            value,
                                            end.offset_from(value) as libc::c_long as uint32_t,
                                        );
                                    }
                                }
                            }
                        }
                        14 => {
                            current_block_64 = 674703269050930462;
                            match current_block_64 {
                                674703269050930462 => {
                                    if *value as libc::c_int == '+' as i32 {
                                        value = value.offset(1);
                                    }
                                    if (*r).resp_decode_chunked == 0
                                        && (*r).resp_htags
                                            & (1 as libc::c_ulong)
                                                << HTTP_HEADER_CONTENT_LENGTH as libc::c_int == 0
                                    {
                                        let mut err: *const libc::c_char = end;
                                        while err > value
                                            && (*err.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                == ' ' as i32
                                                || *err.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                    == '\t' as i32)
                                        {
                                            err = err.offset(-1);
                                        }
                                        if err <= value {
                                            current_block_64 = 224731115979188411;
                                        } else {
                                            let mut vlen: uint32_t = err.offset_from(value)
                                                as libc::c_long as uint32_t;
                                            (*r)
                                                .resp_body_scratchpad = li_restricted_strtoint64(
                                                value,
                                                vlen,
                                                &mut err,
                                            );
                                            if err != value.offset(vlen as isize) {
                                                (*r).resp_body_scratchpad = -(1 as libc::c_int) as off_t;
                                            }
                                            current_block_64 = 14541395414537699361;
                                        }
                                    } else {
                                        current_block_64 = 224731115979188411;
                                    }
                                }
                                9949038703110489483 => {
                                    if (*opts).backend == BACKEND_PROXY as libc::c_int {
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        if http_header_str_contains_token(
                                            value,
                                            end.offset_from(value) as libc::c_long as uint32_t,
                                            b"close\0" as *const u8 as *const libc::c_char,
                                            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                                                as uint32_t)
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                        ) != 0
                                        {
                                            (*r).keep_alive = 0 as libc::c_int as int8_t;
                                        }
                                        if (*r).http_version as libc::c_int
                                            >= HTTP_VERSION_2 as libc::c_int
                                        {
                                            current_block_64 = 224731115979188411;
                                        } else {
                                            current_block_64 = 14541395414537699361;
                                        }
                                    }
                                }
                                10618843670980330763 => {
                                    if (*opts).backend != BACKEND_PROXY as libc::c_int
                                        && (*opts).backend != BACKEND_CGI as libc::c_int
                                    {
                                        current_block_64 = 224731115979188411;
                                    } else if (*r).http_version as libc::c_int
                                            >= HTTP_VERSION_2 as libc::c_int
                                        {
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        current_block_64 = 14541395414537699361;
                                    }
                                }
                                12217996289273463446 => {
                                    if (*opts).backend != BACKEND_PROXY as libc::c_int {
                                        *end
                                            .offset(
                                                0 as libc::c_int as isize,
                                            ) = '\u{0}' as i32 as libc::c_char;
                                        let mut status_1: libc::c_int = http_header_str_to_code(
                                            value,
                                        );
                                        if status_1 >= 100 as libc::c_int
                                            && status_1 < 1000 as libc::c_int
                                        {
                                            (*r).http_status = status_1;
                                            (*opts).local_redir = 0 as libc::c_int as uint8_t;
                                        } else {
                                            (*r).http_status = 502 as libc::c_int;
                                            (*r).handler_module = 0 as *const plugin;
                                        }
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        current_block_64 = 14541395414537699361;
                                    }
                                }
                                17160945220383050795 => {
                                    if (*r).resp_htags
                                        & (1 as libc::c_ulong)
                                            << HTTP_HEADER_CONTENT_LENGTH as libc::c_int != 0
                                    {
                                        (*r).resp_body_scratchpad = -(1 as libc::c_int) as off_t;
                                        http_header_response_unset(
                                            r,
                                            HTTP_HEADER_CONTENT_LENGTH,
                                            b"Content-Length\0" as *const u8 as *const libc::c_char,
                                            (::std::mem::size_of::<[libc::c_char; 15]>()
                                                as libc::c_ulong as uint32_t)
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                        );
                                    }
                                    (*r).resp_decode_chunked = 1 as libc::c_int as libc::c_char;
                                    (*r)
                                        .gw_dechunk = calloc(
                                        1 as libc::c_int as libc::c_ulong,
                                        ::std::mem::size_of::<response_dechunk>() as libc::c_ulong,
                                    ) as *mut response_dechunk;
                                    if ((*r).gw_dechunk).is_null() {
                                        ck_assert_failed(
                                            b"src/http-header-glue.c\0" as *const u8
                                                as *const libc::c_char,
                                            999 as libc::c_int as libc::c_uint,
                                            b"r->gw_dechunk\0" as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    current_block_64 = 224731115979188411;
                                }
                                3809271318960479663 => {
                                    if *k
                                        .offset(
                                            klen.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                        ) as libc::c_int == ' ' as i32
                                        || *k
                                            .offset(
                                                klen.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                            ) as libc::c_int == '\t' as i32
                                    {
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        current_block_64 = 14541395414537699361;
                                    }
                                }
                                _ => {}
                            }
                            match current_block_64 {
                                224731115979188411 => {}
                                _ => {
                                    if end.offset_from(value) as libc::c_long != 0 {
                                        http_header_response_insert(
                                            r,
                                            id,
                                            k,
                                            klen,
                                            value,
                                            end.offset_from(value) as libc::c_long as uint32_t,
                                        );
                                    }
                                }
                            }
                        }
                        48 => {
                            current_block_64 = 17160945220383050795;
                            match current_block_64 {
                                674703269050930462 => {
                                    if *value as libc::c_int == '+' as i32 {
                                        value = value.offset(1);
                                    }
                                    if (*r).resp_decode_chunked == 0
                                        && (*r).resp_htags
                                            & (1 as libc::c_ulong)
                                                << HTTP_HEADER_CONTENT_LENGTH as libc::c_int == 0
                                    {
                                        let mut err: *const libc::c_char = end;
                                        while err > value
                                            && (*err.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                == ' ' as i32
                                                || *err.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                    == '\t' as i32)
                                        {
                                            err = err.offset(-1);
                                        }
                                        if err <= value {
                                            current_block_64 = 224731115979188411;
                                        } else {
                                            let mut vlen: uint32_t = err.offset_from(value)
                                                as libc::c_long as uint32_t;
                                            (*r)
                                                .resp_body_scratchpad = li_restricted_strtoint64(
                                                value,
                                                vlen,
                                                &mut err,
                                            );
                                            if err != value.offset(vlen as isize) {
                                                (*r).resp_body_scratchpad = -(1 as libc::c_int) as off_t;
                                            }
                                            current_block_64 = 14541395414537699361;
                                        }
                                    } else {
                                        current_block_64 = 224731115979188411;
                                    }
                                }
                                9949038703110489483 => {
                                    if (*opts).backend == BACKEND_PROXY as libc::c_int {
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        if http_header_str_contains_token(
                                            value,
                                            end.offset_from(value) as libc::c_long as uint32_t,
                                            b"close\0" as *const u8 as *const libc::c_char,
                                            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                                                as uint32_t)
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                        ) != 0
                                        {
                                            (*r).keep_alive = 0 as libc::c_int as int8_t;
                                        }
                                        if (*r).http_version as libc::c_int
                                            >= HTTP_VERSION_2 as libc::c_int
                                        {
                                            current_block_64 = 224731115979188411;
                                        } else {
                                            current_block_64 = 14541395414537699361;
                                        }
                                    }
                                }
                                10618843670980330763 => {
                                    if (*opts).backend != BACKEND_PROXY as libc::c_int
                                        && (*opts).backend != BACKEND_CGI as libc::c_int
                                    {
                                        current_block_64 = 224731115979188411;
                                    } else if (*r).http_version as libc::c_int
                                            >= HTTP_VERSION_2 as libc::c_int
                                        {
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        current_block_64 = 14541395414537699361;
                                    }
                                }
                                12217996289273463446 => {
                                    if (*opts).backend != BACKEND_PROXY as libc::c_int {
                                        *end
                                            .offset(
                                                0 as libc::c_int as isize,
                                            ) = '\u{0}' as i32 as libc::c_char;
                                        let mut status_1: libc::c_int = http_header_str_to_code(
                                            value,
                                        );
                                        if status_1 >= 100 as libc::c_int
                                            && status_1 < 1000 as libc::c_int
                                        {
                                            (*r).http_status = status_1;
                                            (*opts).local_redir = 0 as libc::c_int as uint8_t;
                                        } else {
                                            (*r).http_status = 502 as libc::c_int;
                                            (*r).handler_module = 0 as *const plugin;
                                        }
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        current_block_64 = 14541395414537699361;
                                    }
                                }
                                17160945220383050795 => {
                                    if (*r).resp_htags
                                        & (1 as libc::c_ulong)
                                            << HTTP_HEADER_CONTENT_LENGTH as libc::c_int != 0
                                    {
                                        (*r).resp_body_scratchpad = -(1 as libc::c_int) as off_t;
                                        http_header_response_unset(
                                            r,
                                            HTTP_HEADER_CONTENT_LENGTH,
                                            b"Content-Length\0" as *const u8 as *const libc::c_char,
                                            (::std::mem::size_of::<[libc::c_char; 15]>()
                                                as libc::c_ulong as uint32_t)
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                        );
                                    }
                                    (*r).resp_decode_chunked = 1 as libc::c_int as libc::c_char;
                                    (*r)
                                        .gw_dechunk = calloc(
                                        1 as libc::c_int as libc::c_ulong,
                                        ::std::mem::size_of::<response_dechunk>() as libc::c_ulong,
                                    ) as *mut response_dechunk;
                                    if ((*r).gw_dechunk).is_null() {
                                        ck_assert_failed(
                                            b"src/http-header-glue.c\0" as *const u8
                                                as *const libc::c_char,
                                            999 as libc::c_int as libc::c_uint,
                                            b"r->gw_dechunk\0" as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    current_block_64 = 224731115979188411;
                                }
                                3809271318960479663 => {
                                    if *k
                                        .offset(
                                            klen.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                        ) as libc::c_int == ' ' as i32
                                        || *k
                                            .offset(
                                                klen.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                            ) as libc::c_int == '\t' as i32
                                    {
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        current_block_64 = 14541395414537699361;
                                    }
                                }
                                _ => {}
                            }
                            match current_block_64 {
                                224731115979188411 => {}
                                _ => {
                                    if end.offset_from(value) as libc::c_long != 0 {
                                        http_header_response_insert(
                                            r,
                                            id,
                                            k,
                                            klen,
                                            value,
                                            end.offset_from(value) as libc::c_long as uint32_t,
                                        );
                                    }
                                }
                            }
                        }
                        28 => {}
                        0 => {
                            current_block_64 = 3809271318960479663;
                            match current_block_64 {
                                674703269050930462 => {
                                    if *value as libc::c_int == '+' as i32 {
                                        value = value.offset(1);
                                    }
                                    if (*r).resp_decode_chunked == 0
                                        && (*r).resp_htags
                                            & (1 as libc::c_ulong)
                                                << HTTP_HEADER_CONTENT_LENGTH as libc::c_int == 0
                                    {
                                        let mut err: *const libc::c_char = end;
                                        while err > value
                                            && (*err.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                == ' ' as i32
                                                || *err.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                    == '\t' as i32)
                                        {
                                            err = err.offset(-1);
                                        }
                                        if err <= value {
                                            current_block_64 = 224731115979188411;
                                        } else {
                                            let mut vlen: uint32_t = err.offset_from(value)
                                                as libc::c_long as uint32_t;
                                            (*r)
                                                .resp_body_scratchpad = li_restricted_strtoint64(
                                                value,
                                                vlen,
                                                &mut err,
                                            );
                                            if err != value.offset(vlen as isize) {
                                                (*r).resp_body_scratchpad = -(1 as libc::c_int) as off_t;
                                            }
                                            current_block_64 = 14541395414537699361;
                                        }
                                    } else {
                                        current_block_64 = 224731115979188411;
                                    }
                                }
                                9949038703110489483 => {
                                    if (*opts).backend == BACKEND_PROXY as libc::c_int {
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        if http_header_str_contains_token(
                                            value,
                                            end.offset_from(value) as libc::c_long as uint32_t,
                                            b"close\0" as *const u8 as *const libc::c_char,
                                            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                                                as uint32_t)
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                        ) != 0
                                        {
                                            (*r).keep_alive = 0 as libc::c_int as int8_t;
                                        }
                                        if (*r).http_version as libc::c_int
                                            >= HTTP_VERSION_2 as libc::c_int
                                        {
                                            current_block_64 = 224731115979188411;
                                        } else {
                                            current_block_64 = 14541395414537699361;
                                        }
                                    }
                                }
                                10618843670980330763 => {
                                    if (*opts).backend != BACKEND_PROXY as libc::c_int
                                        && (*opts).backend != BACKEND_CGI as libc::c_int
                                    {
                                        current_block_64 = 224731115979188411;
                                    } else if (*r).http_version as libc::c_int
                                            >= HTTP_VERSION_2 as libc::c_int
                                        {
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        current_block_64 = 14541395414537699361;
                                    }
                                }
                                12217996289273463446 => {
                                    if (*opts).backend != BACKEND_PROXY as libc::c_int {
                                        *end
                                            .offset(
                                                0 as libc::c_int as isize,
                                            ) = '\u{0}' as i32 as libc::c_char;
                                        let mut status_1: libc::c_int = http_header_str_to_code(
                                            value,
                                        );
                                        if status_1 >= 100 as libc::c_int
                                            && status_1 < 1000 as libc::c_int
                                        {
                                            (*r).http_status = status_1;
                                            (*opts).local_redir = 0 as libc::c_int as uint8_t;
                                        } else {
                                            (*r).http_status = 502 as libc::c_int;
                                            (*r).handler_module = 0 as *const plugin;
                                        }
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        current_block_64 = 14541395414537699361;
                                    }
                                }
                                17160945220383050795 => {
                                    if (*r).resp_htags
                                        & (1 as libc::c_ulong)
                                            << HTTP_HEADER_CONTENT_LENGTH as libc::c_int != 0
                                    {
                                        (*r).resp_body_scratchpad = -(1 as libc::c_int) as off_t;
                                        http_header_response_unset(
                                            r,
                                            HTTP_HEADER_CONTENT_LENGTH,
                                            b"Content-Length\0" as *const u8 as *const libc::c_char,
                                            (::std::mem::size_of::<[libc::c_char; 15]>()
                                                as libc::c_ulong as uint32_t)
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                        );
                                    }
                                    (*r).resp_decode_chunked = 1 as libc::c_int as libc::c_char;
                                    (*r)
                                        .gw_dechunk = calloc(
                                        1 as libc::c_int as libc::c_ulong,
                                        ::std::mem::size_of::<response_dechunk>() as libc::c_ulong,
                                    ) as *mut response_dechunk;
                                    if ((*r).gw_dechunk).is_null() {
                                        ck_assert_failed(
                                            b"src/http-header-glue.c\0" as *const u8
                                                as *const libc::c_char,
                                            999 as libc::c_int as libc::c_uint,
                                            b"r->gw_dechunk\0" as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    current_block_64 = 224731115979188411;
                                }
                                3809271318960479663 => {
                                    if *k
                                        .offset(
                                            klen.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                        ) as libc::c_int == ' ' as i32
                                        || *k
                                            .offset(
                                                klen.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                            ) as libc::c_int == '\t' as i32
                                    {
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        current_block_64 = 14541395414537699361;
                                    }
                                }
                                _ => {}
                            }
                            match current_block_64 {
                                224731115979188411 => {}
                                _ => {
                                    if end.offset_from(value) as libc::c_long != 0 {
                                        http_header_response_insert(
                                            r,
                                            id,
                                            k,
                                            klen,
                                            value,
                                            end.offset_from(value) as libc::c_long as uint32_t,
                                        );
                                    }
                                }
                            }
                        }
                        _ => {
                            current_block_64 = 14541395414537699361;
                            match current_block_64 {
                                674703269050930462 => {
                                    if *value as libc::c_int == '+' as i32 {
                                        value = value.offset(1);
                                    }
                                    if (*r).resp_decode_chunked == 0
                                        && (*r).resp_htags
                                            & (1 as libc::c_ulong)
                                                << HTTP_HEADER_CONTENT_LENGTH as libc::c_int == 0
                                    {
                                        let mut err: *const libc::c_char = end;
                                        while err > value
                                            && (*err.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                == ' ' as i32
                                                || *err.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                    == '\t' as i32)
                                        {
                                            err = err.offset(-1);
                                        }
                                        if err <= value {
                                            current_block_64 = 224731115979188411;
                                        } else {
                                            let mut vlen: uint32_t = err.offset_from(value)
                                                as libc::c_long as uint32_t;
                                            (*r)
                                                .resp_body_scratchpad = li_restricted_strtoint64(
                                                value,
                                                vlen,
                                                &mut err,
                                            );
                                            if err != value.offset(vlen as isize) {
                                                (*r).resp_body_scratchpad = -(1 as libc::c_int) as off_t;
                                            }
                                            current_block_64 = 14541395414537699361;
                                        }
                                    } else {
                                        current_block_64 = 224731115979188411;
                                    }
                                }
                                9949038703110489483 => {
                                    if (*opts).backend == BACKEND_PROXY as libc::c_int {
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        if http_header_str_contains_token(
                                            value,
                                            end.offset_from(value) as libc::c_long as uint32_t,
                                            b"close\0" as *const u8 as *const libc::c_char,
                                            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                                                as uint32_t)
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                        ) != 0
                                        {
                                            (*r).keep_alive = 0 as libc::c_int as int8_t;
                                        }
                                        if (*r).http_version as libc::c_int
                                            >= HTTP_VERSION_2 as libc::c_int
                                        {
                                            current_block_64 = 224731115979188411;
                                        } else {
                                            current_block_64 = 14541395414537699361;
                                        }
                                    }
                                }
                                10618843670980330763 => {
                                    if (*opts).backend != BACKEND_PROXY as libc::c_int
                                        && (*opts).backend != BACKEND_CGI as libc::c_int
                                    {
                                        current_block_64 = 224731115979188411;
                                    } else if (*r).http_version as libc::c_int
                                            >= HTTP_VERSION_2 as libc::c_int
                                        {
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        current_block_64 = 14541395414537699361;
                                    }
                                }
                                12217996289273463446 => {
                                    if (*opts).backend != BACKEND_PROXY as libc::c_int {
                                        *end
                                            .offset(
                                                0 as libc::c_int as isize,
                                            ) = '\u{0}' as i32 as libc::c_char;
                                        let mut status_1: libc::c_int = http_header_str_to_code(
                                            value,
                                        );
                                        if status_1 >= 100 as libc::c_int
                                            && status_1 < 1000 as libc::c_int
                                        {
                                            (*r).http_status = status_1;
                                            (*opts).local_redir = 0 as libc::c_int as uint8_t;
                                        } else {
                                            (*r).http_status = 502 as libc::c_int;
                                            (*r).handler_module = 0 as *const plugin;
                                        }
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        current_block_64 = 14541395414537699361;
                                    }
                                }
                                17160945220383050795 => {
                                    if (*r).resp_htags
                                        & (1 as libc::c_ulong)
                                            << HTTP_HEADER_CONTENT_LENGTH as libc::c_int != 0
                                    {
                                        (*r).resp_body_scratchpad = -(1 as libc::c_int) as off_t;
                                        http_header_response_unset(
                                            r,
                                            HTTP_HEADER_CONTENT_LENGTH,
                                            b"Content-Length\0" as *const u8 as *const libc::c_char,
                                            (::std::mem::size_of::<[libc::c_char; 15]>()
                                                as libc::c_ulong as uint32_t)
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                        );
                                    }
                                    (*r).resp_decode_chunked = 1 as libc::c_int as libc::c_char;
                                    (*r)
                                        .gw_dechunk = calloc(
                                        1 as libc::c_int as libc::c_ulong,
                                        ::std::mem::size_of::<response_dechunk>() as libc::c_ulong,
                                    ) as *mut response_dechunk;
                                    if ((*r).gw_dechunk).is_null() {
                                        ck_assert_failed(
                                            b"src/http-header-glue.c\0" as *const u8
                                                as *const libc::c_char,
                                            999 as libc::c_int as libc::c_uint,
                                            b"r->gw_dechunk\0" as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    current_block_64 = 224731115979188411;
                                }
                                3809271318960479663 => {
                                    if *k
                                        .offset(
                                            klen.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                        ) as libc::c_int == ' ' as i32
                                        || *k
                                            .offset(
                                                klen.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                            ) as libc::c_int == '\t' as i32
                                    {
                                        current_block_64 = 224731115979188411;
                                    } else {
                                        current_block_64 = 14541395414537699361;
                                    }
                                }
                                _ => {}
                            }
                            match current_block_64 {
                                224731115979188411 => {}
                                _ => {
                                    if end.offset_from(value) as libc::c_long != 0 {
                                        http_header_response_insert(
                                            r,
                                            id,
                                            k,
                                            klen,
                                            value,
                                            end.offset_from(value) as libc::c_long as uint32_t,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if 0 as libc::c_int == (*r).http_status
        && (*r).resp_htags & (1 as libc::c_ulong) << HTTP_HEADER_LOCATION as libc::c_int
            != 0
    {
        (*r).http_status = 302 as libc::c_int;
    }
    return 0 as libc::c_int;
}
static mut http_response_send_1xx_h1: http_response_send_1xx_cb = None;
static mut http_response_send_1xx_h2: http_response_send_1xx_cb = None;
#[no_mangle]
#[cold]
pub unsafe extern "C" fn http_response_send_1xx_cb_set(
    mut fn_0: http_response_send_1xx_cb,
    mut vers: libc::c_int,
) {
    if vers >= HTTP_VERSION_2 as libc::c_int {
        http_response_send_1xx_h2 = fn_0;
    } else if vers == HTTP_VERSION_1_1 as libc::c_int {
        http_response_send_1xx_h1 = fn_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn http_response_send_1xx(r: *mut request_st) -> libc::c_int {
    let mut http_response_send_1xx_fn: http_response_send_1xx_cb = None;
    if (*r).http_version as libc::c_int >= HTTP_VERSION_2 as libc::c_int {
        http_response_send_1xx_fn = http_response_send_1xx_h2;
    } else if (*r).http_version as libc::c_int == HTTP_VERSION_1_1 as libc::c_int {
        http_response_send_1xx_fn = http_response_send_1xx_h1;
    }
    if http_response_send_1xx_fn.is_some()
        && http_response_send_1xx_fn.expect("non-null function pointer")(r, (*r).con)
            == 0
    {
        return 0 as libc::c_int;
    }
    http_response_header_clear(r);
    return 1 as libc::c_int;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn http_response_check_1xx(
    r: *mut request_st,
    b: *mut buffer,
    mut hlen: uint32_t,
    mut dlen: uint32_t,
) -> libc::c_int {
    if 0 as libc::c_int == (*r).http_status || 101 as libc::c_int == (*r).http_status {
        return 0 as libc::c_int;
    }
    if dlen != 0 {
        memmove(
            (*b).ptr as *mut libc::c_void,
            ((*b).ptr).offset(hlen as isize) as *const libc::c_void,
            dlen as libc::c_ulong,
        );
    }
    buffer_truncate(b, dlen);
    return http_response_send_1xx(r);
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn http_response_parse_headers(
    r: *mut request_st,
    opts: *mut http_response_opts,
    b: *mut buffer,
) -> handler_t {
    let mut bstart: *const libc::c_char = 0 as *const libc::c_char;
    let mut blen: uint32_t = 0;
    loop {
        let mut header_len: uint32_t = 0;
        let mut is_nph: uint32_t = 0 as libc::c_int as uint32_t;
        let mut hoff: [libc::c_ushort; 8192] = [0; 8192];
        hoff[0 as libc::c_int as usize] = 1 as libc::c_int as libc::c_ushort;
        hoff[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
        hoff[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
        blen = buffer_clen(b);
        header_len = http_header_parse_hoff((*b).ptr, blen, hoff.as_mut_ptr());
        if (if header_len != 0 { header_len } else { blen })
            > 65535 as libc::c_int as libc::c_uint
        {
            log_error(
                (*r).conf.errh,
                b"src/http-header-glue.c\0" as *const u8 as *const libc::c_char,
                1118 as libc::c_int as libc::c_uint,
                b"response headers too large for %s\0" as *const u8
                    as *const libc::c_char,
                (*r).uri.path.ptr,
            );
            (*r).http_status = 502 as libc::c_int;
            (*r).handler_module = 0 as *const plugin;
            return HANDLER_FINISHED;
        }
        if hoff[2 as libc::c_int as usize] != 0 {
            is_nph = (hoff[2 as libc::c_int as usize] as libc::c_int >= 12 as libc::c_int
                && 0 as libc::c_int
                    == memcmp(
                        (*b).ptr as *const libc::c_void,
                        b"HTTP/\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        5 as libc::c_int as libc::c_ulong,
                    )) as libc::c_int as uint32_t;
            if is_nph == 0 {
                let mut colon: *const libc::c_char = memchr(
                    (*b).ptr as *const libc::c_void,
                    ':' as i32,
                    (hoff[2 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int)
                        as libc::c_ulong,
                ) as *const libc::c_char;
                if (0 as *mut libc::c_void as *const libc::c_char == colon)
                    as libc::c_int as libc::c_long != 0
                {
                    if hoff[2 as libc::c_int as usize] as libc::c_int <= 2 as libc::c_int
                        && (1 as libc::c_int
                            == hoff[2 as libc::c_int as usize] as libc::c_int
                            || *((*b).ptr).offset(0 as libc::c_int as isize)
                                as libc::c_int == '\r' as i32)
                    {} else if (*opts).backend == BACKEND_CGI as libc::c_int {
                        if 0 as libc::c_int != http_chunk_append_buffer(r, b) {
                            return HANDLER_ERROR;
                        }
                        (*r).http_status = 200 as libc::c_int;
                        (*r).resp_body_started = 1 as libc::c_int as libc::c_char;
                        return HANDLER_GO_ON;
                    } else {
                        (*r).http_status = 502 as libc::c_int;
                        (*r).handler_module = 0 as *const plugin;
                        return HANDLER_FINISHED;
                    }
                }
            }
        }
        if 0 as libc::c_int as libc::c_uint == header_len {
            return HANDLER_GO_ON;
        }
        bstart = ((*b).ptr).offset(header_len as isize);
        blen = (blen as libc::c_uint).wrapping_sub(header_len) as uint32_t as uint32_t;
        if 0 as libc::c_int
            != http_response_process_headers(
                r,
                opts,
                (*b).ptr,
                hoff.as_mut_ptr() as *const libc::c_ushort,
                is_nph as libc::c_int,
            )
        {
            return HANDLER_ERROR;
        }
        if !((*r).http_status < 200 as libc::c_int
            && http_response_check_1xx(
                r,
                b,
                bstart.offset_from((*b).ptr) as libc::c_long as uint32_t,
                blen,
            ) != 0)
        {
            break;
        }
    }
    (*r).resp_body_started = 1 as libc::c_int as libc::c_char;
    if (*opts).authorizer != 0
        && ((*r).http_status == 0 as libc::c_int
            || (*r).http_status == 200 as libc::c_int)
    {
        return HANDLER_GO_ON;
    }
    if ((*r).handler_module).is_null() {
        return HANDLER_FINISHED;
    }
    if (*opts).local_redir as libc::c_int != 0 && (*r).http_status >= 300 as libc::c_int
        && (*r).http_status < 400 as libc::c_int
        && 0 as libc::c_int as libc::c_uint == blen
    {
        let mut rc: handler_t = http_cgi_local_redir(r);
        if rc as libc::c_uint != HANDLER_GO_ON as libc::c_int as libc::c_uint {
            return rc;
        }
    }
    if (*opts).xsendfile_allow != 0 {
        let mut vb: *mut buffer = 0 as *mut buffer;
        if (*opts).backend == BACKEND_FASTCGI as libc::c_int
            && {
                vb = http_header_response_get(
                    r,
                    HTTP_HEADER_OTHER,
                    b"X-Sendfile2\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                );
                !vb.is_null()
            }
        {
            http_response_xsendfile2(r, vb, (*opts).xsendfile_docroot);
            buffer_clear(vb);
            if ((*r).handler_module).is_null() {
                (*r).resp_body_started = 0 as libc::c_int as libc::c_char;
            }
            return HANDLER_FINISHED;
        } else {
            vb = http_header_response_get(
                r,
                HTTP_HEADER_OTHER,
                b"X-Sendfile\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            );
            if !vb.is_null()
                || (*opts).backend == BACKEND_FASTCGI as libc::c_int
                    && {
                        vb = http_header_response_get(
                            r,
                            HTTP_HEADER_OTHER,
                            b"X-LIGHTTPD-send-file\0" as *const u8
                                as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                        );
                        !vb.is_null()
                    }
            {
                http_response_xsendfile(r, vb, (*opts).xsendfile_docroot);
                buffer_clear(vb);
                if ((*r).handler_module).is_null() {
                    (*r).resp_body_started = 0 as libc::c_int as libc::c_char;
                }
                return HANDLER_FINISHED;
            }
        }
    }
    if blen > 0 as libc::c_int as libc::c_uint {
        let mut rc_0: libc::c_int = http_response_append_mem(r, bstart, blen as size_t);
        if (0 as libc::c_int != rc_0) as libc::c_int as libc::c_long != 0 {
            return HANDLER_ERROR;
        }
    }
    return (if ((*opts).headers).is_some() {
        ((*opts).headers).expect("non-null function pointer")(r, opts) as libc::c_uint
    } else {
        HANDLER_GO_ON as libc::c_int as libc::c_uint
    }) as handler_t;
}
#[no_mangle]
pub unsafe extern "C" fn http_response_read(
    r: *mut request_st,
    opts: *mut http_response_opts,
    b: *mut buffer,
    fdn: *mut fdnode,
) -> handler_t {
    let fd: libc::c_int = (*fdn).fd;
    let mut n: ssize_t = 0;
    let mut avail: size_t = 0;
    loop {
        let mut toread: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        avail = buffer_string_space(b) as size_t;
        if 0 as libc::c_int
            == fdevent_ioctl_fionread(
                fd,
                (*opts).fdfmt,
                &mut toread as *mut libc::c_uint as *mut libc::c_int,
            )
        {
            if (*opts).simple_accum != 0 {
                let mut rc: libc::c_int = http_response_append_splice(
                    r,
                    opts,
                    b,
                    fd,
                    toread,
                );
                if rc != 0 {
                    if (rc > 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
                        break;
                    }
                    return HANDLER_ERROR;
                }
            }
            if avail < toread as libc::c_ulong {
                let mut blen: uint32_t = buffer_clen(b);
                if toread.wrapping_add(blen) < 4096 as libc::c_int as libc::c_uint {
                    toread = (4095 as libc::c_int as libc::c_uint).wrapping_sub(blen);
                } else if toread > (*opts).max_per_read {
                    toread = (*opts).max_per_read;
                }
                if toread > 8192 as libc::c_int as libc::c_uint
                    && (*r).resp_body_started == 0
                {
                    toread = 8192 as libc::c_int as libc::c_uint;
                }
            } else if 0 as libc::c_int as libc::c_uint == toread {
                if (if !fdn.is_null() { (*fdn).events } else { 0 as libc::c_int })
                    & 0x1 as libc::c_int == 0
                {
                    if (*r).conf.stream_response_body as libc::c_int
                        & (1 as libc::c_int) << 15 as libc::c_int == 0
                    {
                        return HANDLER_GO_ON;
                    }
                }
                if 0 as libc::c_int as libc::c_ulong == avail {
                    toread = 1024 as libc::c_int as libc::c_uint;
                }
            }
        } else if avail < 1024 as libc::c_int as libc::c_ulong {
            toread = (4095 as libc::c_int as libc::c_ulong).wrapping_sub(avail)
                as libc::c_uint;
        }
        if (*r).conf.stream_response_body as libc::c_int
            & (1 as libc::c_int) << 1 as libc::c_int != 0
        {
            let mut cqlen: off_t = chunkqueue_length(&mut (*r).write_queue);
            if cqlen + toread as off_t
                > (65536 as libc::c_int - 4096 as libc::c_int) as libc::c_long
            {
                if (*(*r).con).is_writable == 0 {
                    fdevent_fdnode_event_clr(
                        (*(*(*r).con).srv).ev,
                        fdn,
                        0x1 as libc::c_int,
                    );
                }
                if cqlen >= (65536 as libc::c_int - 1 as libc::c_int) as libc::c_long {
                    if buffer_is_blank(b) != 0 {
                        chunk_buffer_yield(b);
                    }
                    return HANDLER_GO_ON;
                }
                toread = ((65536 as libc::c_int - 1 as libc::c_int) as libc::c_uint)
                    .wrapping_sub(cqlen as libc::c_uint);
            }
        }
        if avail < toread as libc::c_ulong {
            avail = if toread < (*opts).max_per_read && avail != 0 {
                avail
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(toread as libc::c_ulong)
            } else {
                toread as libc::c_ulong
            };
            avail = chunk_buffer_prepare_append(b, avail);
        }
        n = read(
            fd,
            ((*b).ptr).offset(buffer_clen(b) as isize) as *mut libc::c_void,
            avail,
        );
        if n < 0 as libc::c_int as libc::c_long {
            match *__errno_location() {
                11 | 4 => {
                    if buffer_is_blank(b) != 0 {
                        chunk_buffer_yield(b);
                    }
                    return HANDLER_GO_ON;
                }
                _ => {
                    log_perror(
                        (*r).conf.errh,
                        b"src/http-header-glue.c\0" as *const u8 as *const libc::c_char,
                        1331 as libc::c_int as libc::c_uint,
                        b"read() %d %d\0" as *const u8 as *const libc::c_char,
                        (*(*r).con).fd,
                        fd,
                    );
                    return HANDLER_ERROR;
                }
            }
        }
        buffer_commit(b, n as size_t);
        if ((*opts).parse).is_some() {
            let mut rc_0: handler_t = ((*opts).parse)
                .expect("non-null function pointer")(r, opts, b, n as size_t);
            if rc_0 as libc::c_uint != HANDLER_GO_ON as libc::c_int as libc::c_uint {
                return rc_0;
            }
        } else if 0 as libc::c_int as libc::c_long == n {
            if buffer_is_blank(b) != 0 {
                chunk_buffer_yield(b);
            } else if (*opts).simple_accum != 0 {
                let mut rc_1: libc::c_int = http_response_append_buffer(
                    r,
                    b,
                    0 as libc::c_int,
                );
                chunk_buffer_yield(b);
                if (0 as libc::c_int != rc_1) as libc::c_int as libc::c_long != 0 {
                    return HANDLER_ERROR;
                }
            }
            return HANDLER_FINISHED;
        } else {
            if 0 as libc::c_int == (*r).resp_body_started as libc::c_int {
                let mut rc_2: handler_t = http_response_parse_headers(r, opts, b);
                if rc_2 as libc::c_uint != HANDLER_GO_ON as libc::c_int as libc::c_uint {
                    return rc_2;
                }
                if (*r).resp_body_started != 0 {
                    buffer_clear(b);
                    if 0 as libc::c_int as libc::c_long == (*r).resp_body_scratchpad {
                        (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
                    } else if (*r).resp_decode_chunked == 0
                            && (*r).conf.stream_response_body as libc::c_int
                                & (1 as libc::c_int) << 1 as libc::c_int == 0
                        {
                        (*opts).simple_accum = 1 as libc::c_int as uint8_t;
                    }
                }
            } else {
                let mut simple_accum: libc::c_int = ((*opts).simple_accum as libc::c_int
                    != 0
                    && ((*r).conf.stream_response_body as libc::c_int
                        & (1 as libc::c_int) << 0 as libc::c_int == 0
                        || (*(*r).con).is_writable == 0)) as libc::c_int;
                let mut rc_3: libc::c_int = http_response_append_buffer(
                    r,
                    b,
                    simple_accum,
                );
                if (0 as libc::c_int != rc_3) as libc::c_int as libc::c_long != 0 {
                    return HANDLER_ERROR;
                }
            }
        }
        if (*r).conf.stream_response_body as libc::c_int
            & (1 as libc::c_int) << 1 as libc::c_int != 0
        {
            if chunkqueue_length(&mut (*r).write_queue)
                > (65536 as libc::c_int - 4096 as libc::c_int) as libc::c_long
            {
                if (*(*r).con).is_writable == 0 {
                    fdevent_fdnode_event_clr(
                        (*(*(*r).con).srv).ev,
                        fdn,
                        0x1 as libc::c_int,
                    );
                }
                break;
            }
        }
        if !((*r).resp_body_started == 0) {
            break;
        }
    }
    if buffer_is_blank(b) != 0 {
        chunk_buffer_yield(b);
    }
    return (if (*r).resp_body_finished == 0 {
        HANDLER_GO_ON as libc::c_int
    } else {
        HANDLER_FINISHED as libc::c_int
    }) as handler_t;
}
pub unsafe fn run_static_initializers() {
    octet_stream = {
        let mut init = buffer {
            ptr: b"application/octet-stream\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            used: (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint),
            size: 0 as libc::c_int as uint32_t,
        };
        init
    };
}
