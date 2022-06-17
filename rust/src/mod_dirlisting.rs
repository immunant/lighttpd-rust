use ::libc;
extern "C" {
    pub type pcre2_real_match_data_8;
    pub type h2con;
    pub type fdevents;
    pub type burl_parts_t;
    pub type pcre_keyvalue;
    pub type __dirstream;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
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
    fn buffer_append_iovec(b: *mut buffer, iov: *const const_iovec, n: size_t);
    fn buffer_append_strftime(
        b: *mut buffer,
        format: *const libc::c_char,
        tm: *const tm,
    );
    fn li_itostrn(buf: *mut libc::c_char, buf_len: size_t, val: intmax_t) -> size_t;
    fn buffer_eq_slen(
        b: *const buffer,
        s: *const libc::c_char,
        slen: size_t,
    ) -> libc::c_int;
    fn buffer_append_string_encoded(
        b: *mut buffer,
        s: *const libc::c_char,
        s_len: size_t,
        encoding: buffer_encoding_t,
    );
    fn buffer_append_string_encoded_json(
        b: *mut buffer,
        s: *const libc::c_char,
        len: size_t,
    );
    fn buffer_append_path_len(b: *mut buffer, a: *const libc::c_char, alen: size_t);
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
    fn array_get_element_klen(
        a: *const array,
        key: *const libc::c_char,
        klen: uint32_t,
    ) -> *const data_unset;
    fn chunk_buffer_acquire() -> *mut buffer;
    fn chunk_buffer_release(b: *mut buffer);
    fn chunkqueue_append_mem(cq: *mut chunkqueue, mem: *const libc::c_char, len: size_t);
    fn chunkqueue_append_chunkqueue(cq: *mut chunkqueue, src: *mut chunkqueue);
    fn chunkqueue_append_buffer_open(cq: *mut chunkqueue) -> *mut buffer;
    fn chunkqueue_append_buffer_commit(cq: *mut chunkqueue);
    fn chunkqueue_append_mem_to_tempfile(
        cq: *mut chunkqueue,
        mem: *const libc::c_char,
        len: size_t,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn chunkqueue_remove_finished_chunks(cq: *mut chunkqueue);
    fn chunkqueue_steal(dest: *mut chunkqueue, src: *mut chunkqueue, len: off_t);
    fn chunkqueue_write_chunk(
        fd: libc::c_int,
        cq: *mut chunkqueue,
        errh: *mut log_error_st,
    ) -> ssize_t;
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
    static mut log_con_jqueue: *mut connection;
    static mut log_epoch_secs: unix_time64_t;
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
    fn fdevent_mkostemp(path: *mut libc::c_char, flags: libc::c_int) -> libc::c_int;
    fn fdevent_open_dirname(
        path: *mut libc::c_char,
        symlinks: libc::c_int,
    ) -> libc::c_int;
    fn http_chunk_append_buffer(r: *mut request_st, mem: *mut buffer) -> libc::c_int;
    fn http_chunk_append_file_ref(
        r: *mut request_st,
        sce: *mut stat_cache_entry,
    ) -> libc::c_int;
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
    fn pcre_keyvalue_buffer_init() -> *mut pcre_keyvalue_buffer;
    fn pcre_keyvalue_buffer_append(
        errh: *mut log_error_st,
        kvb: *mut pcre_keyvalue_buffer,
        key: *const buffer,
        value: *const buffer,
        pcre_jit: libc::c_int,
    ) -> libc::c_int;
    fn pcre_keyvalue_buffer_free(kvb: *mut pcre_keyvalue_buffer);
    fn pcre_keyvalue_buffer_process(
        kvb: *const pcre_keyvalue_buffer,
        ctx: *mut pcre_keyvalue_ctx,
        input: *const buffer,
        result: *mut buffer,
    ) -> handler_t;
    fn http_response_body_clear(r: *mut request_st, preserve_length: libc::c_int);
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
    fn stat_cache_mimetype_by_ext(
        mimetypes: *const array,
        name: *const libc::c_char,
        nlen: uint32_t,
    ) -> *const buffer;
    fn stat_cache_invalidate_entry(name: *const libc::c_char, len: uint32_t);
    fn stat_cache_get_entry_open(
        name: *const buffer,
        symlinks: libc::c_int,
    ) -> *mut stat_cache_entry;
    fn stat_cache_path_isdir(name: *const buffer) -> libc::c_int;
    fn fstatat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fdopendir(__fd: libc::c_int) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
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
pub type __ino64_t = libc::c_ulong;
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
pub type intmax_t = __intmax_t;
pub type unix_time64_t = time_t;
pub type unix_timespec64_t = timespec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
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
pub struct const_iovec {
    pub iov_base: *const libc::c_void,
    pub iov_len: size_t,
}
pub type buffer_encoding_t = libc::c_uint;
pub const ENCODING_MINIMAL_XML: buffer_encoding_t = 3;
pub const ENCODING_HTML: buffer_encoding_t = 2;
pub const ENCODING_REL_URI_PART: buffer_encoding_t = 1;
pub const ENCODING_REL_URI: buffer_encoding_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_string {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
    pub ext: libc::c_int,
    pub value: buffer,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pcre_keyvalue_ctx {
    pub cache: *mut cond_match_t,
    pub burl: *mut burl_parts_t,
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub ovec: *mut libc::c_void,
    pub subject: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pcre_keyvalue_buffer {
    pub kv: *mut pcre_keyvalue,
    pub used: uint32_t,
    pub x0: libc::c_int,
    pub x1: libc::c_int,
    pub cfgidx: libc::c_int,
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
pub type C2RustUnnamed_5 = libc::c_uint;
pub const T_CONFIG_SCOPE_CONNECTION: C2RustUnnamed_5 = 2;
pub const T_CONFIG_SCOPE_SERVER: C2RustUnnamed_5 = 1;
pub const T_CONFIG_SCOPE_UNSET: C2RustUnnamed_5 = 0;
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
pub struct dirent {
    pub d_ino: __ino64_t,
    pub d_off: __off64_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirlist_cache {
    pub max_age: int32_t,
    pub path: *mut buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_config {
    pub dir_listing: libc::c_char,
    pub json: libc::c_char,
    pub hide_dot_files: libc::c_char,
    pub hide_readme_file: libc::c_char,
    pub encode_readme: libc::c_char,
    pub hide_header_file: libc::c_char,
    pub encode_header: libc::c_char,
    pub auto_layout: libc::c_char,
    pub excludes: *mut pcre_keyvalue_buffer,
    pub show_readme: *const buffer,
    pub show_header: *const buffer,
    pub external_css: *const buffer,
    pub external_js: *const buffer,
    pub encoding: *const buffer,
    pub set_footer: *const buffer,
    pub cache: *const dirlist_cache,
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
    pub processing: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirls_entry_t {
    pub namelen: uint32_t,
    pub mtime: unix_time64_t,
    pub size: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirls_list_t {
    pub ent: *mut *mut dirls_entry_t,
    pub used: uint32_t,
    pub size: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct handler_ctx {
    pub dp: *mut DIR,
    pub dirs: dirls_list_t,
    pub files: dirls_list_t,
    pub path: *mut libc::c_char,
    pub path_file: *mut libc::c_char,
    pub dfd: libc::c_int,
    pub name_max: uint32_t,
    pub jb: *mut buffer,
    pub jcomma: libc::c_int,
    pub jfd: libc::c_int,
    pub jfn: *mut libc::c_char,
    pub jfn_len: uint32_t,
    pub conf: plugin_config,
}
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
unsafe extern "C" fn buffer_truncate(mut b: *mut buffer, mut len: uint32_t) {
    *((*b).ptr).offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    (*b).used = len.wrapping_add(1 as libc::c_int as libc::c_uint);
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
static mut dirlist_max_in_progress: libc::c_int = 0;
unsafe extern "C" fn mod_dirlisting_handler_ctx_init(
    p: *mut plugin_data,
) -> *mut handler_ctx {
    let mut hctx: *mut handler_ctx = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<handler_ctx>() as libc::c_ulong,
    ) as *mut handler_ctx;
    if hctx.is_null() {
        ck_assert_failed(
            b"src/mod_dirlisting.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int as libc::c_uint,
            b"hctx\0" as *const u8 as *const libc::c_char,
        );
    }
    memcpy(
        &mut (*hctx).conf as *mut plugin_config as *mut libc::c_void,
        &mut (*p).conf as *mut plugin_config as *const libc::c_void,
        ::std::mem::size_of::<plugin_config>() as libc::c_ulong,
    );
    return hctx;
}
unsafe extern "C" fn mod_dirlisting_handler_ctx_free(mut hctx: *mut handler_ctx) {
    if !((*hctx).dp).is_null() {
        closedir((*hctx).dp);
    }
    if !((*hctx).files.ent).is_null() {
        let ent: *mut *mut dirls_entry_t = (*hctx).files.ent;
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        let mut used: uint32_t = (*hctx).files.used;
        while i < used {
            free(*ent.offset(i as isize) as *mut libc::c_void);
            i = i.wrapping_add(1);
        }
        free(ent as *mut libc::c_void);
    }
    if !((*hctx).dirs.ent).is_null() {
        let ent_0: *mut *mut dirls_entry_t = (*hctx).dirs.ent;
        let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
        let mut used_0: uint32_t = (*hctx).dirs.used;
        while i_0 < used_0 {
            free(*ent_0.offset(i_0 as isize) as *mut libc::c_void);
            i_0 = i_0.wrapping_add(1);
        }
        free(ent_0 as *mut libc::c_void);
    }
    if !((*hctx).jb).is_null() {
        chunk_buffer_release((*hctx).jb);
        if !((*hctx).jfn).is_null() {
            unlink((*hctx).jfn);
            free((*hctx).jfn as *mut libc::c_void);
        }
        if -(1 as libc::c_int) != (*hctx).jfd {
            close((*hctx).jfd);
        }
    }
    free((*hctx).path as *mut libc::c_void);
    free(hctx as *mut libc::c_void);
}
unsafe extern "C" fn mod_dirlisting_parse_cache(
    mut srv: *mut server,
    mut a: *const array,
) -> *mut dirlist_cache {
    let mut du: *const data_unset = 0 as *const data_unset;
    du = array_get_element_klen(
        a,
        b"max-age\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    let max_age: int32_t = config_plugin_value_to_int32(du, 15 as libc::c_int);
    let mut path: *mut buffer = 0 as *mut buffer;
    du = array_get_element_klen(
        a,
        b"path\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if du.is_null() {
        if 0 as libc::c_int != max_age {
            log_error(
                (*srv).errh,
                b"src/mod_dirlisting.c\0" as *const u8 as *const libc::c_char,
                184 as libc::c_int as libc::c_uint,
                b"dir-listing.cache must include \"path\"\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as *mut dirlist_cache;
        }
    } else {
        if (*du).type_0 as libc::c_uint != TYPE_STRING as libc::c_int as libc::c_uint {
            log_error(
                (*srv).errh,
                b"src/mod_dirlisting.c\0" as *const u8 as *const libc::c_char,
                191 as libc::c_int as libc::c_uint,
                b"dir-listing.cache \"path\" must have string value\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as *mut dirlist_cache;
        }
        path = &mut (*(du as *mut data_string)).value;
        if stat_cache_path_isdir(path) == 0 {
            if *__errno_location() == 20 as libc::c_int {
                log_error(
                    (*srv).errh,
                    b"src/mod_dirlisting.c\0" as *const u8 as *const libc::c_char,
                    198 as libc::c_int as libc::c_uint,
                    b"dir-listing.cache \"path\" => \"%s\" is not a dir\0" as *const u8
                        as *const libc::c_char,
                    (*path).ptr,
                );
                return 0 as *mut dirlist_cache;
            }
            if *__errno_location() == 2 as libc::c_int {
                log_error(
                    (*srv).errh,
                    b"src/mod_dirlisting.c\0" as *const u8 as *const libc::c_char,
                    204 as libc::c_int as libc::c_uint,
                    b"dir-listing.cache \"path\" => \"%s\" does not exist\0" as *const u8
                        as *const libc::c_char,
                    (*path).ptr,
                );
            }
        }
    }
    let cache: *mut dirlist_cache = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<dirlist_cache>() as libc::c_ulong,
    ) as *mut dirlist_cache;
    if cache.is_null() {
        ck_assert_failed(
            b"src/mod_dirlisting.c\0" as *const u8 as *const libc::c_char,
            213 as libc::c_int as libc::c_uint,
            b"cache\0" as *const u8 as *const libc::c_char,
        );
    }
    (*cache).max_age = max_age;
    (*cache).path = path;
    return cache;
}
unsafe extern "C" fn mod_dirlisting_parse_excludes(
    mut srv: *mut server,
    mut a: *const array,
) -> *mut pcre_keyvalue_buffer {
    let pcre_jit: libc::c_int = config_feature_bool(
        srv,
        b"server.pcre_jit\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    let kvb: *mut pcre_keyvalue_buffer = pcre_keyvalue_buffer_init();
    let mut empty: buffer = {
        let mut init = buffer {
            ptr: 0 as *mut libc::c_char,
            used: 0 as libc::c_int as uint32_t,
            size: 0 as libc::c_int as uint32_t,
        };
        init
    };
    let mut j: uint32_t = 0 as libc::c_int as uint32_t;
    while j < (*a).used {
        let mut ds: *const data_string = *((*a).data).offset(j as isize)
            as *mut data_string;
        if pcre_keyvalue_buffer_append(
            (*srv).errh,
            kvb,
            &(*ds).value,
            &mut empty,
            pcre_jit,
        ) == 0
        {
            log_error(
                (*srv).errh,
                b"src/mod_dirlisting.c\0" as *const u8 as *const libc::c_char,
                228 as libc::c_int as libc::c_uint,
                b"pcre_compile failed for %s\0" as *const u8 as *const libc::c_char,
                (*ds).key.ptr,
            );
            pcre_keyvalue_buffer_free(kvb);
            return 0 as *mut pcre_keyvalue_buffer;
        }
        j = j.wrapping_add(1);
    }
    return kvb;
}
unsafe extern "C" fn mod_dirlisting_exclude(
    kvb: *mut pcre_keyvalue_buffer,
    name: *const libc::c_char,
    len: uint32_t,
) -> libc::c_int {
    let mut input: buffer = {
        let mut init = buffer {
            ptr: 0 as *mut libc::c_char,
            used: len.wrapping_add(1 as libc::c_int as libc::c_uint),
            size: 0 as libc::c_int as uint32_t,
        };
        init
    };
    let ref mut fresh0 = *(&mut input.ptr as *mut *mut libc::c_char
        as *mut *const libc::c_char);
    *fresh0 = name;
    let mut ctx: pcre_keyvalue_ctx = {
        let mut init = pcre_keyvalue_ctx {
            cache: 0 as *mut cond_match_t,
            burl: 0 as *mut burl_parts_t,
            m: -(1 as libc::c_int),
            n: 0 as libc::c_int,
            ovec: 0 as *mut libc::c_void,
            subject: 0 as *const libc::c_char,
        };
        init
    };
    return (HANDLER_ERROR as libc::c_int as libc::c_uint
        == pcre_keyvalue_buffer_process(kvb, &mut ctx, &mut input, 0 as *mut buffer)
            as libc::c_uint || -(1 as libc::c_int) != ctx.m) as libc::c_int;
}
#[cold]
unsafe extern "C" fn mod_dirlisting_init() -> *mut libc::c_void {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<plugin_data>() as libc::c_ulong,
    );
}
#[cold]
unsafe extern "C" fn mod_dirlisting_free(mut p_d: *mut libc::c_void) {
    let p: *mut plugin_data = p_d as *mut plugin_data;
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
                2 => {
                    if !((*cpv).vtype as libc::c_uint
                        != T_CONFIG_LOCAL as libc::c_int as libc::c_uint)
                    {
                        pcre_keyvalue_buffer_free(
                            (*cpv).v.v as *mut pcre_keyvalue_buffer,
                        );
                    }
                }
                15 => {
                    if !((*cpv).vtype as libc::c_uint
                        != T_CONFIG_LOCAL as libc::c_int as libc::c_uint)
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
unsafe extern "C" fn mod_dirlisting_merge_config_cpv(
    pconf: *mut plugin_config,
    cpv: *const config_plugin_value_t,
) {
    let mut current_block_17: u64;
    match (*cpv).k_id {
        0 => {
            current_block_17 = 6825510417907909175;
        }
        1 => {
            current_block_17 = 6825510417907909175;
        }
        2 => {
            if (*cpv).vtype as libc::c_uint
                == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
            {
                (*pconf).excludes = (*cpv).v.v as *mut pcre_keyvalue_buffer;
            }
            current_block_17 = 14648156034262866959;
        }
        3 => {
            (*pconf).hide_dot_files = (*cpv).v.u as libc::c_char;
            current_block_17 = 14648156034262866959;
        }
        4 => {
            (*pconf).external_css = (*cpv).v.b;
            current_block_17 = 14648156034262866959;
        }
        5 => {
            (*pconf).external_js = (*cpv).v.b;
            current_block_17 = 14648156034262866959;
        }
        6 => {
            (*pconf).encoding = (*cpv).v.b;
            current_block_17 = 14648156034262866959;
        }
        7 => {
            (*pconf).show_readme = (*cpv).v.b;
            current_block_17 = 14648156034262866959;
        }
        8 => {
            (*pconf).hide_readme_file = (*cpv).v.u as libc::c_char;
            current_block_17 = 14648156034262866959;
        }
        9 => {
            (*pconf).show_header = (*cpv).v.b;
            current_block_17 = 14648156034262866959;
        }
        10 => {
            (*pconf).hide_header_file = (*cpv).v.u as libc::c_char;
            current_block_17 = 14648156034262866959;
        }
        11 => {
            (*pconf).set_footer = (*cpv).v.b;
            current_block_17 = 14648156034262866959;
        }
        12 => {
            (*pconf).encode_readme = (*cpv).v.u as libc::c_char;
            current_block_17 = 14648156034262866959;
        }
        13 => {
            (*pconf).encode_header = (*cpv).v.u as libc::c_char;
            current_block_17 = 14648156034262866959;
        }
        14 => {
            (*pconf).auto_layout = (*cpv).v.u as libc::c_char;
            current_block_17 = 14648156034262866959;
        }
        15 => {
            if (*cpv).vtype as libc::c_uint
                == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
            {
                (*pconf).cache = (*cpv).v.v as *const dirlist_cache;
            }
            current_block_17 = 14648156034262866959;
        }
        _ => return,
    }
    match current_block_17 {
        6825510417907909175 => {
            (*pconf).dir_listing = (*cpv).v.u as libc::c_char;
        }
        _ => {}
    };
}
unsafe extern "C" fn mod_dirlisting_merge_config(
    pconf: *mut plugin_config,
    mut cpv: *const config_plugin_value_t,
) {
    loop {
        mod_dirlisting_merge_config_cpv(pconf, cpv);
        cpv = cpv.offset(1);
        if !((*cpv).k_id != -(1 as libc::c_int)) {
            break;
        }
    };
}
unsafe extern "C" fn mod_dirlisting_patch_config(
    r: *mut request_st,
    p: *mut plugin_data,
) {
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
            mod_dirlisting_merge_config(
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
static mut cpk: [config_plugin_keys_t; 17] = [config_plugin_keys_t {
    k: 0 as *const libc::c_char,
    klen: 0,
    ktype: 0,
    scope: 0,
}; 17];
#[cold]
unsafe extern "C" fn mod_dirlisting_set_defaults(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    if config_plugin_values_init(
        srv,
        p as *mut libc::c_void,
        cpk.as_ptr(),
        b"mod_dirlisting\0" as *const u8 as *const libc::c_char,
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
            let mut current_block_33: u64;
            match (*cpv).k_id {
                0 => {
                    current_block_33 = 1345366029464561491;
                }
                1 => {
                    current_block_33 = 1345366029464561491;
                }
                2 => {
                    (*cpv)
                        .v
                        .v = mod_dirlisting_parse_excludes(srv, (*cpv).v.a)
                        as *mut libc::c_void;
                    if ((*cpv).v.v).is_null() {
                        return HANDLER_ERROR;
                    }
                    (*cpv).vtype = T_CONFIG_LOCAL;
                    current_block_33 = 1345366029464561491;
                }
                3 => {
                    current_block_33 = 1345366029464561491;
                }
                4 => {
                    current_block_33 = 4521361012956345576;
                }
                5 | 6 => {
                    current_block_33 = 4521361012956345576;
                }
                7 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        let mut b: *mut buffer = 0 as *mut buffer;
                        let ref mut fresh1 = *(&mut b as *mut *mut buffer
                            as *mut *const buffer);
                        *fresh1 = (*cpv).v.b;
                        if buffer_eq_slen(
                            b,
                            b"enable\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        ) != 0
                        {
                            buffer_copy_string_len(
                                b,
                                b"README.txt\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 11]>()
                                    as libc::c_ulong as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                            );
                        } else if buffer_eq_slen(
                                b,
                                b"disable\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                                    as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                            ) != 0
                            {
                            buffer_clear(b);
                        }
                    } else {
                        (*cpv).v.b = 0 as *const buffer;
                    }
                    current_block_33 = 1345366029464561491;
                }
                8 => {
                    current_block_33 = 1345366029464561491;
                }
                9 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        let mut b_0: *mut buffer = 0 as *mut buffer;
                        let ref mut fresh2 = *(&mut b_0 as *mut *mut buffer
                            as *mut *const buffer);
                        *fresh2 = (*cpv).v.b;
                        if buffer_eq_slen(
                            b_0,
                            b"enable\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        ) != 0
                        {
                            buffer_copy_string_len(
                                b_0,
                                b"HEADER.txt\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 11]>()
                                    as libc::c_ulong as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                            );
                        } else if buffer_eq_slen(
                                b_0,
                                b"disable\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                                    as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                            ) != 0
                            {
                            buffer_clear(b_0);
                        }
                    } else {
                        (*cpv).v.b = 0 as *const buffer;
                    }
                    current_block_33 = 1345366029464561491;
                }
                10 => {
                    current_block_33 = 1345366029464561491;
                }
                11 => {
                    if buffer_is_blank((*cpv).v.b) != 0 {
                        (*cpv).v.b = 0 as *const buffer;
                    }
                    current_block_33 = 1345366029464561491;
                }
                12 => {
                    current_block_33 = 13872683252508214801;
                }
                13 => {
                    current_block_33 = 13872683252508214801;
                }
                14 => {
                    current_block_33 = 17527121841775069174;
                }
                15 => {
                    (*cpv)
                        .v
                        .v = mod_dirlisting_parse_cache(srv, (*cpv).v.a)
                        as *mut libc::c_void;
                    if ((*cpv).v.v).is_null() {
                        return HANDLER_ERROR;
                    }
                    if 0 as libc::c_int == (*((*cpv).v.v as *mut dirlist_cache)).max_age
                    {
                        free((*cpv).v.v);
                        (*cpv).v.v = 0 as *mut libc::c_void;
                    }
                    (*cpv).vtype = T_CONFIG_LOCAL;
                    current_block_33 = 1345366029464561491;
                }
                _ => {
                    current_block_33 = 1345366029464561491;
                }
            }
            match current_block_33 {
                4521361012956345576 => {
                    if buffer_is_blank((*cpv).v.b) != 0 {
                        (*cpv).v.b = 0 as *const buffer;
                    }
                    current_block_33 = 1345366029464561491;
                }
                13872683252508214801 => {
                    current_block_33 = 17527121841775069174;
                }
                _ => {}
            }
            match current_block_33 {
                17527121841775069174 => {}
                _ => {}
            }
            cpv = cpv.offset(1);
        }
        i += 1;
    }
    dirlist_max_in_progress = (*srv).srvconf.max_conns as libc::c_int
        >> 4 as libc::c_int;
    if 0 as libc::c_int == dirlist_max_in_progress {
        dirlist_max_in_progress = 1 as libc::c_int;
    }
    (*p).defaults.dir_listing = 0 as libc::c_int as libc::c_char;
    (*p).defaults.json = 0 as libc::c_int as libc::c_char;
    (*p).defaults.hide_dot_files = 1 as libc::c_int as libc::c_char;
    (*p).defaults.hide_readme_file = 0 as libc::c_int as libc::c_char;
    (*p).defaults.hide_header_file = 0 as libc::c_int as libc::c_char;
    (*p).defaults.encode_readme = 1 as libc::c_int as libc::c_char;
    (*p).defaults.encode_header = 1 as libc::c_int as libc::c_char;
    (*p).defaults.auto_layout = 1 as libc::c_int as libc::c_char;
    if (*p).nconfig > 0 as libc::c_int
        && (*(*p).cvlist).v.u2[1 as libc::c_int as usize] != 0
    {
        let mut cpv_0: *const config_plugin_value_t = ((*p).cvlist)
            .offset((*(*p).cvlist).v.u2[0 as libc::c_int as usize] as isize);
        if -(1 as libc::c_int) != (*cpv_0).k_id {
            mod_dirlisting_merge_config(&mut (*p).defaults, cpv_0);
        }
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn http_dirls_sort(
    mut ent: *mut *mut dirls_entry_t,
    mut num: libc::c_int,
) {
    let mut gap: libc::c_int = num;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut swapped: libc::c_int = 0;
    let mut tmp: *mut dirls_entry_t = 0 as *mut dirls_entry_t;
    loop {
        gap = gap * 10 as libc::c_int / 13 as libc::c_int;
        if gap == 9 as libc::c_int || gap == 10 as libc::c_int {
            gap = 11 as libc::c_int;
        }
        if gap < 1 as libc::c_int {
            gap = 1 as libc::c_int;
        }
        swapped = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < num - gap {
            j = i + gap;
            if strcmp(
                (*ent.offset(i as isize) as *mut libc::c_char)
                    .offset(
                        ::std::mem::size_of::<dirls_entry_t>() as libc::c_ulong as isize,
                    ),
                (*ent.offset(j as isize) as *mut libc::c_char)
                    .offset(
                        ::std::mem::size_of::<dirls_entry_t>() as libc::c_ulong as isize,
                    ),
            ) > 0 as libc::c_int
            {
                tmp = *ent.offset(i as isize);
                let ref mut fresh3 = *ent.offset(i as isize);
                *fresh3 = *ent.offset(j as isize);
                let ref mut fresh4 = *ent.offset(j as isize);
                *fresh4 = tmp;
                swapped = 1 as libc::c_int;
            }
            i += 1;
        }
        if !(gap > 1 as libc::c_int || swapped != 0) {
            break;
        }
    };
}
unsafe extern "C" fn http_list_directory_sizefmt(
    mut buf: *mut libc::c_char,
    mut bufsz: size_t,
    mut size: off_t,
) -> size_t {
    let mut remain: libc::c_int = 0;
    let mut u: libc::c_int = -(1 as libc::c_int);
    let mut buflen: size_t = 0;
    if (0 as libc::c_int as libc::c_long) < size
        && size < 100 as libc::c_int as libc::c_long
    {
        size += 99 as libc::c_int as libc::c_long;
    }
    loop {
        remain = (size & 1023 as libc::c_int as libc::c_long) as libc::c_int;
        size >>= 10 as libc::c_int;
        u += 1;
        if !(size & !(1023 as libc::c_int) as libc::c_long != 0) {
            break;
        }
    }
    remain /= 100 as libc::c_int;
    if remain > 9 as libc::c_int {
        remain = 9 as libc::c_int;
    }
    if size > 999 as libc::c_int as libc::c_long {
        size = 0 as libc::c_int as off_t;
        remain = 9 as libc::c_int;
        u += 1;
    }
    buflen = li_itostrn(buf, bufsz, size);
    if buflen.wrapping_add(3 as libc::c_int as libc::c_ulong) >= bufsz {
        return buflen;
    }
    *buf
        .offset(
            buflen.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
        ) = '.' as i32 as libc::c_char;
    *buf
        .offset(
            buflen.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
        ) = (remain + '0' as i32) as libc::c_char;
    *buf
        .offset(
            buflen.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
        ) = (*::std::mem::transmute::<
        &[u8; 7],
        &[libc::c_char; 7],
    >(b"KMGTPE\0"))[u as usize];
    *buf
        .offset(
            buflen.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
        ) = '\u{0}' as i32 as libc::c_char;
    return buflen.wrapping_add(3 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn http_list_directory_include_file(
    r: *mut request_st,
    p: *const handler_ctx,
    mut is_header: libc::c_int,
) {
    let mut path: *const buffer = 0 as *const buffer;
    let mut encode: libc::c_int = 0 as libc::c_int;
    if is_header != 0 {
        path = (*p).conf.show_header;
        encode = (*p).conf.encode_header as libc::c_int;
    } else {
        path = (*p).conf.show_readme;
        encode = (*p).conf.encode_readme as libc::c_int;
    }
    if path.is_null() {
        return;
    }
    let mut len: uint32_t = 0 as libc::c_int as uint32_t;
    if *((*path).ptr).offset(0 as libc::c_int as isize) as libc::c_int != '/' as i32 {
        len = buffer_clen(&mut (*r).physical.path);
        buffer_append_path_len(
            &mut (*r).physical.path,
            (*path).ptr,
            buffer_clen(path) as size_t,
        );
        path = &mut (*r).physical.path;
    }
    let sce: *mut stat_cache_entry = stat_cache_get_entry_open(
        path,
        (*r).conf.follow_symlink as libc::c_int,
    );
    if len != 0 {
        buffer_truncate(&mut (*r).physical.path, len);
    }
    if sce.is_null() || (*sce).fd < 0 as libc::c_int
        || 0 as libc::c_int as libc::c_long == (*sce).st.st_size
    {
        return;
    }
    let cq: *mut chunkqueue = &mut (*r).write_queue;
    if encode != 0 {
        if is_header != 0 {
            chunkqueue_append_mem(
                cq,
                b"<pre class=\"header\">\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        } else {
            chunkqueue_append_mem(
                cq,
                b"<pre class=\"readme\">\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
        let tb: *mut buffer = (*r).tmp_buf;
        let out: *mut buffer = if (*sce).st.st_size
            <= 32768 as libc::c_int as libc::c_long
        {
            chunkqueue_append_buffer_open(cq)
        } else {
            tb
        };
        buffer_clear(out);
        let fd: libc::c_int = (*sce).fd;
        let mut rd: ssize_t = 0;
        let mut buf: [libc::c_char; 8192] = [0; 8192];
        loop {
            rd = read(
                fd,
                buf.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
            );
            if !(rd > 0 as libc::c_int as libc::c_long) {
                break;
            }
            buffer_append_string_encoded(
                out,
                buf.as_mut_ptr(),
                rd as size_t,
                ENCODING_MINIMAL_XML,
            );
            if !(out == tb) {
                continue;
            }
            if 0 as libc::c_int
                != chunkqueue_append_mem_to_tempfile(
                    cq,
                    (*out).ptr,
                    buffer_clen(out) as size_t,
                    (*r).conf.errh,
                )
            {
                break;
            }
            buffer_clear(out);
        }
        if out != tb {
            chunkqueue_append_buffer_commit(cq);
        }
        chunkqueue_append_mem(
            cq,
            b"</pre>\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    } else {
        http_chunk_append_file_ref(r, sce);
    };
}
static mut js_simple_table_resort: [libc::c_char; 3654] = unsafe {
    *::std::mem::transmute::<
        &[u8; 3654],
        &[libc::c_char; 3654],
    >(
        b"var click_column;\nvar name_column = 0;\nvar date_column = 1;\nvar size_column = 2;\nvar type_column = 3;\nvar prev_span = null;\n\nif (typeof(String.prototype.localeCompare) === 'undefined') {\n String.prototype.localeCompare = function(str, locale, options) {\n   return ((this == str) ? 0 : ((this > str) ? 1 : -1));\n };\n}\n\nif (typeof(String.prototype.toLocaleUpperCase) === 'undefined') {\n String.prototype.toLocaleUpperCase = function() {\n  return this.toUpperCase();\n };\n}\n\nfunction get_inner_text(el) {\n if((typeof el == 'string')||(typeof el == 'undefined'))\n  return el;\n if(el.innerText)\n  return el.innerText;\n else {\n  var str = \"\";\n  var cs = el.childNodes;\n  var l = cs.length;\n  for (var i=0;i<l;i++) {\n   if (cs[i].nodeType==1) str += get_inner_text(cs[i]);\n   else if (cs[i].nodeType==3) str += cs[i].nodeValue;\n  }\n }\n return str;\n}\n\nfunction isdigit(c) {\n return (c >= '0' && c <= '9');\n}\n\nfunction unit_multiplier(unit) {\n return (unit=='K') ? 1000\n      : (unit=='M') ? 1000000\n      : (unit=='G') ? 1000000000\n      : (unit=='T') ? 1000000000000\n      : (unit=='P') ? 1000000000000000\n      : (unit=='E') ? 1000000000000000000 : 1;\n}\n\nvar li_date_regex=/(\\d{4})-(\\w{3})-(\\d{2}) (\\d{2}):(\\d{2}):(\\d{2})/;\n\nvar li_mon = ['Jan','Feb','Mar','Apr','May','Jun',\n              'Jul','Aug','Sep','Oct','Nov','Dec'];\n\nfunction li_mon_num(mon) {\n var i; for (i = 0; i < 12 && mon != li_mon[i]; ++i); return i;\n}\n\nfunction li_date_cmp(s1, s2) {\n var dp1 = li_date_regex.exec(s1)\n var dp2 = li_date_regex.exec(s2)\n for (var i = 1; i < 7; ++i) {\n  var cmp = (2 != i)\n   ? parseInt(dp1[i]) - parseInt(dp2[i])\n   : li_mon_num(dp1[2]) - li_mon_num(dp2[2]);\n  if (0 != cmp) return cmp;\n }\n return 0;\n}\n\nfunction sortfn_then_by_name(a,b,sort_column) {\n if (sort_column == name_column || sort_column == type_column) {\n  var ad = (a.cells[type_column].innerHTML === 'Directory');\n  var bd = (b.cells[type_column].innerHTML === 'Directory');\n  if (ad != bd) return (ad ? -1 : 1);\n }\n var at = get_inner_text(a.cells[sort_column]);\n var bt = get_inner_text(b.cells[sort_column]);\n var cmp;\n if (sort_column == name_column) {\n  if (at == '../') return -1;\n  if (bt == '../') return  1;\n }\n if (a.cells[sort_column].className == 'int') {\n  cmp = parseInt(at)-parseInt(bt);\n } else if (sort_column == date_column) {\n  var ad = isdigit(at.substr(0,1));\n  var bd = isdigit(bt.substr(0,1));\n  if (ad != bd) return (!ad ? -1 : 1);\n  cmp = li_date_cmp(at,bt);\n } else if (sort_column == size_column) {\n  var ai = parseInt(at, 10) * unit_multiplier(at.substr(-1,1));\n  var bi = parseInt(bt, 10) * unit_multiplier(bt.substr(-1,1));\n  if (at.substr(0,1) == '-') ai = -1;\n  if (bt.substr(0,1) == '-') bi = -1;\n  cmp = ai - bi;\n } else {\n  cmp = at.toLocaleUpperCase().localeCompare(bt.toLocaleUpperCase());\n  if (0 != cmp) return cmp;\n  cmp = at.localeCompare(bt);\n }\n if (0 != cmp || sort_column == name_column) return cmp;\n return sortfn_then_by_name(a,b,name_column);\n}\n\nfunction sortfn(a,b) {\n return sortfn_then_by_name(a,b,click_column);\n}\n\nfunction resort(lnk) {\n var span = lnk.childNodes[1];\n var table = lnk.parentNode.parentNode.parentNode.parentNode;\n var rows = new Array();\n for (var j=1;j<table.rows.length;j++)\n  rows[j-1] = table.rows[j];\n click_column = lnk.parentNode.cellIndex;\n rows.sort(sortfn);\n\n if (prev_span != null) prev_span.innerHTML = '';\n if (span.getAttribute('sortdir')=='down') {\n  span.innerHTML = '&uarr;';\n  span.setAttribute('sortdir','up');\n  rows.reverse();\n } else {\n  span.innerHTML = '&darr;';\n  span.setAttribute('sortdir','down');\n }\n for (var i=0;i<rows.length;i++)\n  table.tBodies[0].appendChild(rows[i]);\n prev_span = span;\n}\n\0",
    )
};
static mut js_simple_table_init_sort: [libc::c_char; 1544] = unsafe {
    *::std::mem::transmute::<
        &[u8; 1544],
        &[libc::c_char; 1544],
    >(
        b"\nfunction init_sort(init_sort_column, ascending) {\n var tables = document.getElementsByTagName(\"table\");\n for (var i = 0; i < tables.length; i++) {\n  var table = tables[i];\n  //var c = table.getAttribute(\"class\")\n  //if (-1 != c.split(\" \").indexOf(\"sort\")) {\n   var row = table.rows[0].cells;\n   for (var j = 0; j < row.length; j++) {\n    var n = row[j];\n    if (n.childNodes.length == 1 && n.childNodes[0].nodeType == 3) {\n     var link = document.createElement(\"a\");\n     var title = n.childNodes[0].nodeValue.replace(/:$/, \"\");\n     link.appendChild(document.createTextNode(title));\n     link.setAttribute(\"href\", \"#\");\n     link.setAttribute(\"class\", \"sortheader\");\n     link.setAttribute(\"onclick\", \"resort(this);return false;\");\n     var arrow = document.createElement(\"span\");\n     arrow.setAttribute(\"class\", \"sortarrow\");\n     arrow.appendChild(document.createTextNode(\":\"));\n     link.appendChild(arrow)\n     n.replaceChild(link, n.firstChild);\n    }\n   }\n   var lnk = row[init_sort_column].firstChild;\n   if (ascending) {\n    var span = lnk.childNodes[1];\n    span.setAttribute('sortdir','down');\n   }\n   resort(lnk);\n  //}\n }\n}\n\nfunction init_sort_from_query() {\n  var urlParams = new URLSearchParams(location.search);\n  var c = 0;\n  var o = 0;\n  switch (urlParams.get('C')) {\n    case \"N\": c=0; break;\n    case \"M\": c=1; break;\n    case \"S\": c=2; break;\n    case \"T\":\n    case \"D\": c=3; break;\n  }\n  switch (urlParams.get('O')) {\n    case \"A\": o=1; break;\n    case \"D\": o=0; break;\n  }\n  init_sort(c,o);\n}\ninit_sort_from_query();\n\0",
    )
};
unsafe extern "C" fn http_dirlist_append_js_table_resort(b: *mut buffer) {
    let mut iov: [const_iovec; 4] = [
        {
            let mut init = const_iovec {
                iov_base: b"\n<script type=\"text/javascript\">\n// <!--\n\n\0"
                    as *const u8 as *const libc::c_char as *const libc::c_void,
                iov_len: (::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: js_simple_table_resort.as_ptr() as *const libc::c_void,
                iov_len: (::std::mem::size_of::<[libc::c_char; 3654]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: js_simple_table_init_sort.as_ptr() as *const libc::c_void,
                iov_len: (::std::mem::size_of::<[libc::c_char; 1544]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: b"\n// -->\n</script>\n\n\0" as *const u8
                    as *const libc::c_char as *const libc::c_void,
                iov_len: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            };
            init
        },
    ];
    buffer_append_iovec(
        b,
        iov.as_mut_ptr(),
        (::std::mem::size_of::<[const_iovec; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<const_iovec>() as libc::c_ulong),
    );
}
unsafe extern "C" fn http_list_directory_header(
    r: *mut request_st,
    p: *const handler_ctx,
) {
    let cq: *mut chunkqueue = &mut (*r).write_queue;
    if (*p).conf.auto_layout != 0 {
        let out: *mut buffer = chunkqueue_append_buffer_open(cq);
        buffer_append_string_len(
            out,
            b"<!DOCTYPE html>\n<html>\n<head>\n\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        if !((*p).conf.encoding).is_null() {
            buffer_append_str3(
                out,
                b"<meta charset=\"\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                (*(*p).conf.encoding).ptr,
                buffer_clen((*p).conf.encoding) as size_t,
                b"\">\n\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
        buffer_append_string_len(
            out,
            b"<title>Index of \0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        buffer_append_string_encoded(
            out,
            (*r).uri.path.ptr,
            buffer_clen(&mut (*r).uri.path) as size_t,
            ENCODING_MINIMAL_XML,
        );
        buffer_append_string_len(
            out,
            b"</title>\n\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        if !((*p).conf.external_css).is_null() {
            buffer_append_str3(
                out,
                b"<meta name=\"viewport\" content=\"initial-scale=1\"><link rel=\"stylesheet\" type=\"text/css\" href=\"\0"
                    as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 94]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                (*(*p).conf.external_css).ptr,
                buffer_clen((*p).conf.external_css) as size_t,
                b"\">\n\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        } else {
            buffer_append_string_len(
                out,
                b"<style type=\"text/css\">\na, a:active {text-decoration: none; color: blue;}\na:visited {color: #48468F;}\na:hover, a:focus {text-decoration: underline; color: red;}\nbody {background-color: #F5F5F5;}\nh2 {margin-bottom: 12px;}\ntable {margin-left: 12px;}\nth, td { font: 90% monospace; text-align: left;}\nth { font-weight: bold; padding-right: 14px; padding-bottom: 3px;}\ntd {padding-right: 14px;}\ntd.s, th.s {text-align: right;}\ndiv.list { background-color: white; border-top: 1px solid #646464; border-bottom: 1px solid #646464; padding-top: 10px; padding-bottom: 14px;}\ndiv.foot { font: 90% monospace; color: #787878; padding-top: 4px;}\n</style>\n\0"
                    as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 642]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
        buffer_append_string_len(
            out,
            b"</head>\n<body>\n\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        chunkqueue_append_buffer_commit(cq);
    }
    if !((*p).conf.show_header).is_null() {
        http_list_directory_include_file(r, p, 1 as libc::c_int);
    }
    let out_0: *mut buffer = chunkqueue_append_buffer_open(cq);
    buffer_append_string_len(
        out_0,
        b"<h2>Index of \0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_string_encoded(
        out_0,
        (*r).uri.path.ptr,
        buffer_clen(&mut (*r).uri.path) as size_t,
        ENCODING_MINIMAL_XML,
    );
    buffer_append_string_len(
        out_0,
        b"</h2>\n<div class=\"list\">\n<table summary=\"Directory Listing\" cellpadding=\"0\" cellspacing=\"0\">\n<thead><tr><th class=\"n\">Name</th><th class=\"m\">Last Modified</th><th class=\"s\">Size</th><th class=\"t\">Type</th></tr></thead>\n<tbody>\n\0"
            as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 228]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    if buffer_eq_slen(
        &mut (*r).uri.path,
        b"/\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    ) == 0
    {
        buffer_append_string_len(
            out_0,
            b"<tr class=\"d\"><td class=\"n\"><a href=\"../\">..</a>/</td><td class=\"m\">&nbsp;</td><td class=\"s\">- &nbsp;</td><td class=\"t\">Directory</td></tr>\n\0"
                as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 141]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    chunkqueue_append_buffer_commit(cq);
}
unsafe extern "C" fn http_list_directory_footer(
    r: *mut request_st,
    p: *const handler_ctx,
) {
    let cq: *mut chunkqueue = &mut (*r).write_queue;
    chunkqueue_append_mem(
        cq,
        b"</tbody>\n</table>\n</div>\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    if !((*p).conf.show_readme).is_null() {
        http_list_directory_include_file(r, p, 0 as libc::c_int);
    }
    if (*p).conf.auto_layout != 0 {
        let out: *mut buffer = chunkqueue_append_buffer_open(cq);
        let footer: *const buffer = if !((*p).conf.set_footer).is_null() {
            (*p).conf.set_footer
        } else if !((*r).conf.server_tag).is_null() {
            (*r).conf.server_tag
        } else {
            0 as *const buffer
        };
        if !footer.is_null() {
            buffer_append_str3(
                out,
                b"<div class=\"foot\">\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                (*footer).ptr,
                buffer_clen(footer) as size_t,
                b"</div>\n\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
        if !((*p).conf.external_js).is_null() {
            buffer_append_str3(
                out,
                b"<script type=\"text/javascript\" src=\"\0" as *const u8
                    as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                (*(*p).conf.external_js).ptr,
                buffer_clen((*p).conf.external_js) as size_t,
                b"\"></script>\n\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        } else {
            http_dirlist_append_js_table_resort(out);
        }
        buffer_append_string_len(
            out,
            b"</body>\n</html>\n\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        chunkqueue_append_buffer_commit(cq);
    }
}
unsafe extern "C" fn http_open_directory(
    r: *mut request_st,
    hctx: *mut handler_ctx,
) -> libc::c_int {
    let dlen: uint32_t = buffer_clen(&mut (*r).physical.path);
    (*hctx)
        .name_max = (4096 as libc::c_int as libc::c_uint)
        .wrapping_sub(dlen)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    (*hctx)
        .path = malloc(
        dlen
            .wrapping_add((*hctx).name_max)
            .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
    ) as *mut libc::c_char;
    if ((*hctx).path).is_null() {
        ck_assert_failed(
            b"src/mod_dirlisting.c\0" as *const u8 as *const libc::c_char,
            999 as libc::c_int as libc::c_uint,
            b"((void*)0) != hctx->path\0" as *const u8 as *const libc::c_char,
        );
    }
    memcpy(
        (*hctx).path as *mut libc::c_void,
        (*r).physical.path.ptr as *const libc::c_void,
        dlen.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
    );
    (*hctx)
        .dfd = fdevent_open_dirname(
        (*hctx).path,
        (*r).conf.follow_symlink as libc::c_int,
    );
    (*hctx)
        .dp = if (*hctx).dfd >= 0 as libc::c_int {
        fdopendir((*hctx).dfd)
    } else {
        0 as *mut DIR
    };
    if ((*hctx).dp).is_null() {
        log_perror(
            (*r).conf.errh,
            b"src/mod_dirlisting.c\0" as *const u8 as *const libc::c_char,
            1013 as libc::c_int as libc::c_uint,
            b"opendir %s\0" as *const u8 as *const libc::c_char,
            (*hctx).path,
        );
        if (*hctx).dfd >= 0 as libc::c_int {
            close((*hctx).dfd);
            (*hctx).dfd = -(1 as libc::c_int);
        }
        return -(1 as libc::c_int);
    }
    if (*hctx).conf.json != 0 {
        return 0 as libc::c_int;
    }
    let dirs: *mut dirls_list_t = &mut (*hctx).dirs;
    let files: *mut dirls_list_t = &mut (*hctx).files;
    (*dirs)
        .ent = malloc(
        (::std::mem::size_of::<*mut dirls_entry_t>() as libc::c_ulong)
            .wrapping_mul(16 as libc::c_int as libc::c_ulong),
    ) as *mut *mut dirls_entry_t;
    if ((*dirs).ent).is_null() {
        ck_assert_failed(
            b"src/mod_dirlisting.c\0" as *const u8 as *const libc::c_char,
            1027 as libc::c_int as libc::c_uint,
            b"dirs->ent\0" as *const u8 as *const libc::c_char,
        );
    }
    (*dirs).size = 16 as libc::c_int as uint32_t;
    (*dirs).used = 0 as libc::c_int as uint32_t;
    (*files)
        .ent = malloc(
        (::std::mem::size_of::<*mut dirls_entry_t>() as libc::c_ulong)
            .wrapping_mul(16 as libc::c_int as libc::c_ulong),
    ) as *mut *mut dirls_entry_t;
    if ((*files).ent).is_null() {
        ck_assert_failed(
            b"src/mod_dirlisting.c\0" as *const u8 as *const libc::c_char,
            1032 as libc::c_int as libc::c_uint,
            b"files->ent\0" as *const u8 as *const libc::c_char,
        );
    }
    (*files).size = 16 as libc::c_int as uint32_t;
    (*files).used = 0 as libc::c_int as uint32_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn http_read_directory(p: *mut handler_ctx) -> libc::c_int {
    let mut dent: *mut dirent = 0 as *mut dirent;
    let hide_dotfiles: libc::c_int = (*p).conf.hide_dot_files as libc::c_int;
    let name_max: uint32_t = (*p).name_max;
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
    let mut count: libc::c_int = -(1 as libc::c_int);
    loop {
        count += 1;
        if !(count < 32 as libc::c_int
            && {
                dent = readdir((*p).dp);
                !dent.is_null()
            })
        {
            break;
        }
        let d_name: *const libc::c_char = ((*dent).d_name).as_mut_ptr();
        let dsz: uint32_t = strlen(((*dent).d_name).as_mut_ptr()) as uint32_t;
        if *d_name.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
            if hide_dotfiles != 0 {
                continue;
            }
            if *d_name.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
            {
                continue;
            }
            if *d_name.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *d_name.offset(2 as libc::c_int as isize) as libc::c_int
                    == '\u{0}' as i32
            {
                continue;
            }
        }
        if (*p).conf.hide_readme_file as libc::c_int != 0
            && !((*p).conf.show_readme).is_null()
            && buffer_eq_slen((*p).conf.show_readme, d_name, dsz as size_t) != 0
        {
            continue;
        }
        if (*p).conf.hide_header_file as libc::c_int != 0
            && !((*p).conf.show_header).is_null()
            && buffer_eq_slen((*p).conf.show_header, d_name, dsz as size_t) != 0
        {
            continue;
        }
        if !((*p).conf.excludes).is_null()
            && mod_dirlisting_exclude((*p).conf.excludes, d_name, dsz) != 0
        {
            continue;
        }
        if dsz > name_max {
            continue;
        }
        if 0 as libc::c_int != fstatat((*p).dfd, d_name, &mut st, 0 as libc::c_int) {
            continue;
        }
        if !((*p).jb).is_null() {
            if (*p).jcomma as libc::c_long != 0 {
                buffer_append_string_len(
                    (*p).jb,
                    b",{\"name\":\"\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
            } else {
                (*p).jcomma = 1 as libc::c_int;
                buffer_append_string_len(
                    (*p).jb,
                    b"{\"name\":\"\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
            }
            buffer_append_string_encoded_json((*p).jb, d_name, dsz as size_t);
            let mut t: *const libc::c_char = 0 as *const libc::c_char;
            let mut tlen: size_t = 0;
            if !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint)
            {
                t = b"\",\"type\":\"file\",\"size\":\0" as *const u8
                    as *const libc::c_char;
                tlen = (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            } else {
                t = b"\",\"type\":\"dir\",\"size\":\0" as *const u8
                    as *const libc::c_char;
                tlen = (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            }
            let mut sstr: [libc::c_char; 22] = [0; 22];
            let mut mstr: [libc::c_char; 22] = [0; 22];
            let mut iov: [const_iovec; 5] = [
                {
                    let mut init = const_iovec {
                        iov_base: t as *const libc::c_void,
                        iov_len: tlen,
                    };
                    init
                },
                {
                    let mut init = const_iovec {
                        iov_base: sstr.as_mut_ptr() as *const libc::c_void,
                        iov_len: li_itostrn(
                            sstr.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong,
                            st.st_size,
                        ),
                    };
                    init
                },
                {
                    let mut init = const_iovec {
                        iov_base: b",\"mtime\":\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        iov_len: (::std::mem::size_of::<[libc::c_char; 10]>()
                            as libc::c_ulong as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    };
                    init
                },
                {
                    let mut init = const_iovec {
                        iov_base: mstr.as_mut_ptr() as *const libc::c_void,
                        iov_len: li_itostrn(
                            mstr.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong,
                            st.st_mtim.tv_sec,
                        ),
                    };
                    init
                },
                {
                    let mut init = const_iovec {
                        iov_base: b"}\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        iov_len: (::std::mem::size_of::<[libc::c_char; 2]>()
                            as libc::c_ulong as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    };
                    init
                },
            ];
            buffer_append_iovec(
                (*p).jb,
                iov.as_mut_ptr(),
                (::std::mem::size_of::<[const_iovec; 5]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<const_iovec>() as libc::c_ulong),
            );
        } else {
            let list: *mut dirls_list_t = if !(st.st_mode
                & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint)
            {
                &mut (*p).files
            } else {
                &mut (*p).dirs
            };
            if (*list).used == (*list).size {
                (*list)
                    .size = ((*list).size as libc::c_uint)
                    .wrapping_add(16 as libc::c_int as libc::c_uint) as uint32_t
                    as uint32_t;
                (*list)
                    .ent = realloc(
                    (*list).ent as *mut libc::c_void,
                    (::std::mem::size_of::<*mut dirls_entry_t>() as libc::c_ulong)
                        .wrapping_mul((*list).size as libc::c_ulong),
                ) as *mut *mut dirls_entry_t;
                if ((*list).ent).is_null() {
                    ck_assert_failed(
                        b"src/mod_dirlisting.c\0" as *const u8 as *const libc::c_char,
                        1130 as libc::c_int as libc::c_uint,
                        b"list->ent\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            let fresh5 = (*list).used;
            (*list).used = ((*list).used).wrapping_add(1);
            let ref mut fresh6 = *((*list).ent).offset(fresh5 as isize);
            *fresh6 = malloc(
                (::std::mem::size_of::<dirls_entry_t>() as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(dsz as libc::c_ulong),
            ) as *mut dirls_entry_t;
            let tmp: *mut dirls_entry_t = *fresh6;
            if tmp.is_null() {
                ck_assert_failed(
                    b"src/mod_dirlisting.c\0" as *const u8 as *const libc::c_char,
                    1134 as libc::c_int as libc::c_uint,
                    b"tmp\0" as *const u8 as *const libc::c_char,
                );
            }
            (*tmp).mtime = st.st_mtim.tv_sec;
            (*tmp).size = st.st_size;
            (*tmp).namelen = dsz;
            memcpy(
                (tmp as *mut libc::c_char)
                    .offset(
                        ::std::mem::size_of::<dirls_entry_t>() as libc::c_ulong as isize,
                    ) as *mut libc::c_void,
                d_name as *const libc::c_void,
                dsz.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
            );
        }
    }
    if count == 32 as libc::c_int {
        return HANDLER_WAIT_FOR_EVENT as libc::c_int;
    }
    closedir((*p).dp);
    (*p).dp = 0 as *mut DIR;
    return HANDLER_FINISHED as libc::c_int;
}
unsafe extern "C" fn http_list_directory(r: *mut request_st, hctx: *mut handler_ctx) {
    let dirs: *mut dirls_list_t = &mut (*hctx).dirs;
    let files: *mut dirls_list_t = &mut (*hctx).files;
    if (*dirs).used != 0 {
        http_dirls_sort((*dirs).ent, (*dirs).used as libc::c_int);
    }
    if (*files).used != 0 {
        http_dirls_sort((*files).ent, (*files).used as libc::c_int);
    }
    let mut sizebuf: [libc::c_char; 7] = [0; 7];
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let cq: *mut chunkqueue = &mut (*r).write_queue;
    let tb: *mut buffer = (*r).tmp_buf;
    buffer_clear(tb);
    let out: *mut buffer = if ((*dirs).used).wrapping_add((*files).used)
        <= 256 as libc::c_int as libc::c_uint
    {
        chunkqueue_append_buffer_open(cq)
    } else {
        tb
    };
    buffer_clear(out);
    let dirs_ent: *mut *mut dirls_entry_t = (*dirs).ent;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let mut used: uint32_t = (*dirs).used;
    while i < used {
        let tmp: *mut dirls_entry_t = *dirs_ent.offset(i as isize);
        buffer_append_string_len(
            out,
            b"<tr class=\"d\"><td class=\"n\"><a href=\"\0" as *const u8
                as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 38]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        buffer_append_string_encoded(
            out,
            (tmp as *mut libc::c_char)
                .offset(
                    ::std::mem::size_of::<dirls_entry_t>() as libc::c_ulong as isize,
                ),
            (*tmp).namelen as size_t,
            ENCODING_REL_URI_PART,
        );
        buffer_append_string_len(
            out,
            b"/\">\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        buffer_append_string_encoded(
            out,
            (tmp as *mut libc::c_char)
                .offset(
                    ::std::mem::size_of::<dirls_entry_t>() as libc::c_ulong as isize,
                ),
            (*tmp).namelen as size_t,
            ENCODING_MINIMAL_XML,
        );
        buffer_append_string_len(
            out,
            b"</a>/</td><td class=\"m\">\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        buffer_append_strftime(
            out,
            b"%Y-%b-%d %T\0" as *const u8 as *const libc::c_char,
            localtime_r(&mut (*tmp).mtime, &mut tm),
        );
        buffer_append_string_len(
            out,
            b"</td><td class=\"s\">- &nbsp;</td><td class=\"t\">Directory</td></tr>\n\0"
                as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 67]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        if buffer_string_space(out) < 256 as libc::c_int as libc::c_uint {
            if out == tb {
                if 0 as libc::c_int
                    != chunkqueue_append_mem_to_tempfile(
                        cq,
                        (*out).ptr,
                        buffer_clen(out) as size_t,
                        (*r).conf.errh,
                    )
                {
                    break;
                }
                buffer_clear(out);
            }
        }
        i = i.wrapping_add(1);
    }
    let mimetypes: *const array = (*r).conf.mimetypes;
    let files_ent: *mut *mut dirls_entry_t = (*files).ent;
    let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
    let mut used_0: uint32_t = (*files).used;
    while i_0 < used_0 {
        let tmp_0: *mut dirls_entry_t = *files_ent.offset(i_0 as isize);
        let mut content_type: *const buffer = 0 as *const buffer;
        content_type = stat_cache_mimetype_by_ext(
            mimetypes,
            (tmp_0 as *mut libc::c_char)
                .offset(
                    ::std::mem::size_of::<dirls_entry_t>() as libc::c_ulong as isize,
                ),
            (*tmp_0).namelen,
        );
        if content_type.is_null() {
            static mut octet_stream: buffer = {
                let mut init = buffer {
                    ptr: b"application/octet-stream\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    used: ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                        as uint32_t,
                    size: 0 as libc::c_int as uint32_t,
                };
                init
            };
            content_type = &octet_stream;
        }
        buffer_append_string_len(
            out,
            b"<tr><td class=\"n\"><a href=\"\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        buffer_append_string_encoded(
            out,
            (tmp_0 as *mut libc::c_char)
                .offset(
                    ::std::mem::size_of::<dirls_entry_t>() as libc::c_ulong as isize,
                ),
            (*tmp_0).namelen as size_t,
            ENCODING_REL_URI_PART,
        );
        buffer_append_string_len(
            out,
            b"\">\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        buffer_append_string_encoded(
            out,
            (tmp_0 as *mut libc::c_char)
                .offset(
                    ::std::mem::size_of::<dirls_entry_t>() as libc::c_ulong as isize,
                ),
            (*tmp_0).namelen as size_t,
            ENCODING_MINIMAL_XML,
        );
        buffer_append_string_len(
            out,
            b"</a></td><td class=\"m\">\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        buffer_append_strftime(
            out,
            b"%Y-%b-%d %T\0" as *const u8 as *const libc::c_char,
            localtime_r(&mut (*tmp_0).mtime, &mut tm),
        );
        let mut buflen: size_t = http_list_directory_sizefmt(
            sizebuf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong,
            (*tmp_0).size,
        );
        let mut iov: [const_iovec; 5] = [
            {
                let mut init = const_iovec {
                    iov_base: b"</td><td class=\"s\">\0" as *const u8
                        as *const libc::c_char as *const libc::c_void,
                    iov_len: (::std::mem::size_of::<[libc::c_char; 20]>()
                        as libc::c_ulong as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                };
                init
            },
            {
                let mut init = const_iovec {
                    iov_base: sizebuf.as_mut_ptr() as *const libc::c_void,
                    iov_len: buflen,
                };
                init
            },
            {
                let mut init = const_iovec {
                    iov_base: b"</td><td class=\"t\">\0" as *const u8
                        as *const libc::c_char as *const libc::c_void,
                    iov_len: (::std::mem::size_of::<[libc::c_char; 20]>()
                        as libc::c_ulong as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                };
                init
            },
            {
                let mut init = const_iovec {
                    iov_base: (*content_type).ptr as *const libc::c_void,
                    iov_len: buffer_clen(content_type) as size_t,
                };
                init
            },
            {
                let mut init = const_iovec {
                    iov_base: b"</td></tr>\n\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    iov_len: (::std::mem::size_of::<[libc::c_char; 12]>()
                        as libc::c_ulong as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                };
                init
            },
        ];
        buffer_append_iovec(
            out,
            iov.as_mut_ptr(),
            (::std::mem::size_of::<[const_iovec; 5]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<const_iovec>() as libc::c_ulong),
        );
        if buffer_string_space(out) < 256 as libc::c_int as libc::c_uint {
            if out == tb {
                if 0 as libc::c_int
                    != chunkqueue_append_mem_to_tempfile(
                        cq,
                        (*out).ptr,
                        buffer_clen(out) as size_t,
                        (*r).conf.errh,
                    )
                {
                    break;
                }
                buffer_clear(out);
            }
        }
        i_0 = i_0.wrapping_add(1);
    }
    if out == tb {
        buffer_is_blank(out) == 0
            && 0 as libc::c_int
                != chunkqueue_append_mem_to_tempfile(
                    cq,
                    (*out).ptr,
                    buffer_clen(out) as size_t,
                    (*r).conf.errh,
                );
    } else {
        chunkqueue_append_buffer_commit(cq);
    };
}
unsafe extern "C" fn mod_dirlisting_content_type(
    r: *mut request_st,
    encoding: *const buffer,
) {
    let vb: *mut buffer = http_header_response_set_ptr(
        r,
        HTTP_HEADER_CONTENT_TYPE,
        b"Content-Type\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if encoding.is_null() {
        buffer_copy_string_len(
            vb,
            b"text/html\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    } else {
        buffer_append_str2(
            vb,
            b"text/html; charset=\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            (*encoding).ptr,
            buffer_clen(encoding) as size_t,
        );
    };
}
unsafe extern "C" fn mod_dirlisting_response(
    r: *mut request_st,
    hctx: *mut handler_ctx,
) {
    http_list_directory_header(r, hctx);
    http_list_directory(r, hctx);
    http_list_directory_footer(r, hctx);
    mod_dirlisting_content_type(r, (*hctx).conf.encoding);
    (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn mod_dirlisting_json_append(
    r: *mut request_st,
    hctx: *mut handler_ctx,
    fin: libc::c_int,
) {
    let jb: *mut buffer = (*hctx).jb;
    if fin != 0 {
        buffer_append_string_len(
            jb,
            b"]}\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    } else if buffer_clen(jb)
            < (16384 as libc::c_int - 1024 as libc::c_int) as libc::c_uint
        {
        return
    }
    if !((*hctx).jfn).is_null() {
        if (write_all(
            (*hctx).jfd,
            (*jb).ptr as *const libc::c_void,
            buffer_clen(jb) as size_t,
        ) < 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long != 0
        {
            unlink((*hctx).jfn);
            free((*hctx).jfn as *mut libc::c_void);
            (*hctx).jfn = 0 as *mut libc::c_char;
            close((*hctx).jfd);
            (*hctx).jfd = -(1 as libc::c_int);
        }
    }
    http_chunk_append_buffer(r, jb);
}
unsafe extern "C" fn mod_dirlisting_subrequest_start(
    mut r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    if !((*r).handler_module).is_null() {
        return HANDLER_GO_ON;
    }
    if buffer_has_slash_suffix(&mut (*r).uri.path) == 0 {
        return HANDLER_GO_ON;
    }
    if !((*r).http_method as libc::c_int <= HTTP_METHOD_HEAD as libc::c_int) {
        return HANDLER_GO_ON;
    }
    mod_dirlisting_patch_config(r, p);
    if (*p).conf.dir_listing == 0 {
        return HANDLER_GO_ON;
    }
    if (*r).conf.log_request_handling != 0 {
        log_error(
            (*r).conf.errh,
            b"src/mod_dirlisting.c\0" as *const u8 as *const libc::c_char,
            1334 as libc::c_int as libc::c_uint,
            b"-- handling the request as Dir-Listing\0" as *const u8
                as *const libc::c_char,
        );
        log_error(
            (*r).conf.errh,
            b"src/mod_dirlisting.c\0" as *const u8 as *const libc::c_char,
            1336 as libc::c_int as libc::c_uint,
            b"URI          : %s\0" as *const u8 as *const libc::c_char,
            (*r).uri.path.ptr,
        );
    }
    if !((*p).conf.cache).is_null() {
        let mut rc: handler_t = mod_dirlisting_cache_check(r, p);
        if rc as libc::c_uint != HANDLER_GO_ON as libc::c_int as libc::c_uint {
            return rc;
        }
    }
    if (*p).processing == dirlist_max_in_progress {
        (*r).http_status = 503 as libc::c_int;
        http_header_response_set(
            r,
            HTTP_HEADER_OTHER,
            b"Retry-After\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            b"2\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        return HANDLER_FINISHED;
    }
    let hctx: *mut handler_ctx = mod_dirlisting_handler_ctx_init(p);
    if 0 as libc::c_int != http_open_directory(r, hctx) {
        (*r).http_status = 403 as libc::c_int;
        mod_dirlisting_handler_ctx_free(hctx);
        return HANDLER_FINISHED;
    }
    (*p).processing += 1;
    if (*p).conf.json != 0 {
        (*hctx).jfd = -(1 as libc::c_int);
        (*hctx).jb = chunk_buffer_acquire();
        buffer_append_string_len(
            (*hctx).jb,
            b"{[\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        if !((*p).conf.cache).is_null() {
            mod_dirlisting_cache_json_init(r, hctx);
        }
        http_header_response_set(
            r,
            HTTP_HEADER_CONTENT_TYPE,
            b"Content-Type\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            b"application/json\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        (*r).http_status = 200 as libc::c_int;
        (*r).resp_body_started = 1 as libc::c_int as libc::c_char;
    }
    let ref mut fresh7 = *((*r).plugin_ctx).offset((*p).id as isize);
    *fresh7 = hctx as *mut libc::c_void;
    (*r).handler_module = (*p).self_0;
    return mod_dirlisting_subrequest(r, p as *mut libc::c_void);
}
unsafe extern "C" fn mod_dirlisting_subrequest(
    mut r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    let hctx: *mut handler_ctx = *((*r).plugin_ctx).offset((*p).id as isize)
        as *mut handler_ctx;
    if hctx.is_null() {
        return HANDLER_GO_ON;
    }
    let mut rc: handler_t = http_read_directory(hctx) as handler_t;
    match rc as libc::c_uint {
        1 => {
            if !((*hctx).jb).is_null() {
                mod_dirlisting_json_append(r, hctx, 1 as libc::c_int);
                (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
                if !((*hctx).jfn).is_null() {
                    mod_dirlisting_cache_json(r, hctx);
                }
            } else {
                mod_dirlisting_response(r, hctx);
                if !((*hctx).conf.cache).is_null() {
                    mod_dirlisting_cache_add(r, hctx);
                }
            }
            mod_dirlisting_reset(r, p as *mut libc::c_void);
        }
        3 => {
            if !((*hctx).jb).is_null() {
                mod_dirlisting_json_append(r, hctx, 0 as libc::c_int);
            }
            connection_jq_append((*r).con);
        }
        _ => {}
    }
    return rc;
}
unsafe extern "C" fn mod_dirlisting_reset(
    mut r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let dptr: *mut *mut libc::c_void = &mut *((*r).plugin_ctx)
        .offset((*(p_d as *mut plugin_data)).id as isize) as *mut *mut libc::c_void;
    if !(*dptr).is_null() {
        let ref mut fresh8 = (*(p_d as *mut plugin_data)).processing;
        *fresh8 -= 1;
        mod_dirlisting_handler_ctx_free(*dptr as *mut handler_ctx);
        *dptr = 0 as *mut libc::c_void;
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_dirlisting_cache_check(
    r: *mut request_st,
    p: *mut plugin_data,
) -> handler_t {
    let tb: *mut buffer = (*r).tmp_buf;
    buffer_copy_path_len2(
        tb,
        (*(*(*p).conf.cache).path).ptr,
        buffer_clen((*(*p).conf.cache).path) as size_t,
        (*r).physical.path.ptr,
        buffer_clen(&mut (*r).physical.path) as size_t,
    );
    buffer_append_string_len(
        tb,
        if (*p).conf.json as libc::c_int != 0 {
            b"dirlist.json\0" as *const u8 as *const libc::c_char
        } else {
            b"dirlist.html\0" as *const u8 as *const libc::c_char
        },
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    let sce: *mut stat_cache_entry = stat_cache_get_entry_open(tb, 1 as libc::c_int);
    if sce.is_null() || (*sce).fd == -(1 as libc::c_int) {
        return HANDLER_GO_ON;
    }
    if ((*sce).st.st_mtim.tv_sec + (*(*p).conf.cache).max_age as libc::c_long)
        < log_epoch_secs
    {
        return HANDLER_GO_ON;
    }
    if (*p).conf.json == 0 {
        mod_dirlisting_content_type(r, (*p).conf.encoding);
    } else {
        http_header_response_set(
            r,
            HTTP_HEADER_CONTENT_TYPE,
            b"Content-Type\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            b"application/json\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    };
    if 0 as libc::c_int != http_chunk_append_file_ref(r, sce) {
        http_header_response_unset(
            r,
            HTTP_HEADER_CONTENT_TYPE,
            b"Content-Type\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        http_response_body_clear(r, 0 as libc::c_int);
        return HANDLER_GO_ON;
    }
    (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
    return HANDLER_FINISHED;
}
unsafe extern "C" fn mod_dirlisting_write_cq(
    fd: libc::c_int,
    cq: *mut chunkqueue,
    errh: *mut log_error_st,
) -> libc::c_int {
    let mut in_0: chunkqueue = chunkqueue {
        first: 0 as *mut chunk,
        last: 0 as *mut chunk,
        bytes_in: 0,
        bytes_out: 0,
        tempdirs: 0 as *const array,
        upload_temp_file_size: 0,
        tempdir_idx: 0,
    };
    memset(
        &mut in_0 as *mut chunkqueue as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<chunkqueue>() as libc::c_ulong,
    );
    chunkqueue_append_chunkqueue(&mut in_0, cq);
    (*cq).bytes_in -= in_0.bytes_in;
    (*cq).bytes_out -= in_0.bytes_in;
    chunkqueue_remove_finished_chunks(&mut in_0);
    while chunkqueue_is_empty(&mut in_0) == 0 {
        let mut wr: ssize_t = chunkqueue_write_chunk(fd, &mut in_0, errh);
        if wr > 0 as libc::c_int as libc::c_long {
            chunkqueue_steal(cq, &mut in_0, wr);
        } else if wr < 0 as libc::c_int as libc::c_long {
            chunkqueue_append_chunkqueue(cq, &mut in_0);
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn mkdir_recursive(
    mut dir: *mut libc::c_char,
    mut off: size_t,
) -> libc::c_int {
    let mut p: *mut libc::c_char = dir.offset(off as isize);
    if *p as libc::c_int != '/' as i32 {
        if off != 0
            && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '/' as i32
        {
            p = p.offset(-1);
        } else {
            *__errno_location() = 20 as libc::c_int;
            return -(1 as libc::c_int);
        }
    }
    loop {
        *p = '\u{0}' as i32 as libc::c_char;
        let mut rc: libc::c_int = mkdir(dir, 0o700 as libc::c_int as __mode_t);
        *p = '/' as i32 as libc::c_char;
        if 0 as libc::c_int != rc && *__errno_location() != 17 as libc::c_int {
            return -(1 as libc::c_int);
        }
        p = strchr(p.offset(1 as libc::c_int as isize), '/' as i32);
        if p.is_null() {
            break;
        }
    }
    return 0 as libc::c_int;
}
#[inline(never)]
unsafe extern "C" fn mod_dirlisting_cache_add(
    r: *mut request_st,
    hctx: *mut handler_ctx,
) {
    let mut oldpath: [libc::c_char; 4096] = [0; 4096];
    let mut newpath: [libc::c_char; 4096] = [0; 4096];
    let tb: *mut buffer = (*r).tmp_buf;
    buffer_copy_path_len2(
        tb,
        (*(*(*hctx).conf.cache).path).ptr,
        buffer_clen((*(*hctx).conf.cache).path) as size_t,
        (*r).physical.path.ptr,
        buffer_clen(&mut (*r).physical.path) as size_t,
    );
    if stat_cache_path_isdir(tb) == 0
        && 0 as libc::c_int
            != mkdir_recursive(
                (*tb).ptr,
                buffer_clen((*(*hctx).conf.cache).path) as size_t,
            )
    {
        return;
    }
    buffer_append_string_len(
        tb,
        b"dirlist.html\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    let len: size_t = buffer_clen(tb) as size_t;
    if len.wrapping_add(7 as libc::c_int as libc::c_ulong)
        >= 4096 as libc::c_int as libc::c_ulong
    {
        return;
    }
    memcpy(
        newpath.as_mut_ptr() as *mut libc::c_void,
        (*tb).ptr as *const libc::c_void,
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    buffer_append_string_len(
        tb,
        b".XXXXXX\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    memcpy(
        oldpath.as_mut_ptr() as *mut libc::c_void,
        (*tb).ptr as *const libc::c_void,
        len
            .wrapping_add(7 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    let fd: libc::c_int = fdevent_mkostemp(oldpath.as_mut_ptr(), 0 as libc::c_int);
    if fd < 0 as libc::c_int {
        return;
    }
    if mod_dirlisting_write_cq(fd, &mut (*r).write_queue, (*r).conf.errh) != 0
        && 0 as libc::c_int == rename(oldpath.as_mut_ptr(), newpath.as_mut_ptr())
    {
        stat_cache_invalidate_entry(newpath.as_mut_ptr(), len as uint32_t);
    } else {
        unlink(oldpath.as_mut_ptr());
    }
    close(fd);
}
#[inline(never)]
unsafe extern "C" fn mod_dirlisting_cache_json_init(
    r: *mut request_st,
    hctx: *mut handler_ctx,
) {
    let tb: *mut buffer = (*r).tmp_buf;
    buffer_copy_path_len2(
        tb,
        (*(*(*hctx).conf.cache).path).ptr,
        buffer_clen((*(*hctx).conf.cache).path) as size_t,
        (*r).physical.path.ptr,
        buffer_clen(&mut (*r).physical.path) as size_t,
    );
    if stat_cache_path_isdir(tb) == 0
        && 0 as libc::c_int
            != mkdir_recursive(
                (*tb).ptr,
                buffer_clen((*(*hctx).conf.cache).path) as size_t,
            )
    {
        return;
    }
    buffer_append_string_len(
        tb,
        b"dirlist.json.XXXXXX\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    let fd: libc::c_int = fdevent_mkostemp((*tb).ptr, 0 as libc::c_int);
    if fd < 0 as libc::c_int {
        return;
    }
    (*hctx).jfn_len = buffer_clen(tb);
    (*hctx).jfd = fd;
    (*hctx)
        .jfn = malloc(
        ((*hctx).jfn_len).wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
    ) as *mut libc::c_char;
    if ((*hctx).jfn).is_null() {
        ck_assert_failed(
            b"src/mod_dirlisting.c\0" as *const u8 as *const libc::c_char,
            1608 as libc::c_int as libc::c_uint,
            b"hctx->jfn\0" as *const u8 as *const libc::c_char,
        );
    }
    memcpy(
        (*hctx).jfn as *mut libc::c_void,
        (*tb).ptr as *const libc::c_void,
        ((*hctx).jfn_len).wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
    );
}
#[inline(never)]
unsafe extern "C" fn mod_dirlisting_cache_json(
    r: *mut request_st,
    hctx: *mut handler_ctx,
) {
    let mut newpath: [libc::c_char; 4096] = [0; 4096];
    let len: size_t = ((*hctx).jfn_len).wrapping_sub(7 as libc::c_int as libc::c_uint)
        as size_t;
    if !(len < 4096 as libc::c_int as libc::c_ulong) {
        ck_assert_failed(
            b"src/mod_dirlisting.c\0" as *const u8 as *const libc::c_char,
            1621 as libc::c_int as libc::c_uint,
            b"len < 4096\0" as *const u8 as *const libc::c_char,
        );
    }
    memcpy(
        newpath.as_mut_ptr() as *mut libc::c_void,
        (*hctx).jfn as *const libc::c_void,
        len,
    );
    newpath[len as usize] = '\u{0}' as i32 as libc::c_char;
    if 0 as libc::c_int == rename((*hctx).jfn, newpath.as_mut_ptr()) {
        stat_cache_invalidate_entry(newpath.as_mut_ptr(), len as uint32_t);
    } else {
        unlink((*hctx).jfn);
    }
    free((*hctx).jfn as *mut libc::c_void);
    (*hctx).jfn = 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn mod_dirlisting_plugin_init(mut p: *mut plugin) -> libc::c_int {
    (*p).version = 0x10440 as libc::c_int as size_t;
    (*p).name = b"dirlisting\0" as *const u8 as *const libc::c_char;
    (*p)
        .init = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
    >(Some(mod_dirlisting_init as unsafe extern "C" fn() -> *mut libc::c_void));
    (*p)
        .handle_subrequest_start = Some(
        mod_dirlisting_subrequest_start
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_subrequest = Some(
        mod_dirlisting_subrequest
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_request_reset = Some(
        mod_dirlisting_reset
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .set_defaults = Some(
        mod_dirlisting_set_defaults
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .cleanup = Some(
        mod_dirlisting_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    return 0 as libc::c_int;
}
pub unsafe fn run_static_initializers() {
    cpk = [
        {
            let mut init = config_plugin_keys_t {
                k: b"dir-listing.activate\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.dir-listing\0" as *const u8 as *const libc::c_char,
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
                k: b"dir-listing.exclude\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_VLIST as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"dir-listing.hide-dotfiles\0" as *const u8 as *const libc::c_char,
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
                k: b"dir-listing.external-css\0" as *const u8 as *const libc::c_char,
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
                k: b"dir-listing.external-js\0" as *const u8 as *const libc::c_char,
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
                k: b"dir-listing.encoding\0" as *const u8 as *const libc::c_char,
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
                k: b"dir-listing.show-readme\0" as *const u8 as *const libc::c_char,
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
                k: b"dir-listing.hide-readme-file\0" as *const u8 as *const libc::c_char,
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
                k: b"dir-listing.show-header\0" as *const u8 as *const libc::c_char,
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
                k: b"dir-listing.hide-header-file\0" as *const u8 as *const libc::c_char,
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
                k: b"dir-listing.set-footer\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"dir-listing.encode-readme\0" as *const u8 as *const libc::c_char,
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
                k: b"dir-listing.encode-header\0" as *const u8 as *const libc::c_char,
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
                k: b"dir-listing.auto-layout\0" as *const u8 as *const libc::c_char,
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
                k: b"dir-listing.cache\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
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
