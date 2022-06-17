use ::libc;
extern "C" {
    pub type __dirstream;
    pub type pcre2_real_match_data_8;
    pub type h2con;
    pub type fdevents;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off64_t,
    ) -> *mut libc::c_void;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn fstatat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn fdopendir(__fd: libc::c_int) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn __errno_location() -> *mut libc::c_int;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn renameat2(
        __oldfd: libc::c_int,
        __old: *const libc::c_char,
        __newfd: libc::c_int,
        __new: *const libc::c_char,
        __flags: libc::c_uint,
    ) -> libc::c_int;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
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
    fn memrchr(
        __s: *const libc::c_void,
        __c: libc::c_int,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn lseek(__fd: libc::c_int, __offset: __off64_t, __whence: libc::c_int) -> __off64_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn getpid() -> __pid_t;
    fn linkat(
        __fromfd: libc::c_int,
        __from: *const libc::c_char,
        __tofd: libc::c_int,
        __to: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn unlinkat(
        __fd: libc::c_int,
        __name: *const libc::c_char,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn rmdir(__path: *const libc::c_char) -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off64_t) -> libc::c_int;
    fn copy_file_range(
        __infd: libc::c_int,
        __pinoff: *mut __off64_t,
        __outfd: libc::c_int,
        __poutoff: *mut __off64_t,
        __length: size_t,
        __flags: libc::c_uint,
    ) -> ssize_t;
    fn chunkqueue_reset(cq: *mut chunkqueue);
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
    fn buffer_append_uint_hex_lc(b: *mut buffer, len: uintmax_t);
    fn buffer_append_int(b: *mut buffer, val: intmax_t);
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
    fn buffer_urldecode_path(b: *mut buffer);
    fn buffer_is_valid_UTF8(b: *const buffer) -> libc::c_int;
    fn buffer_path_simplify(b: *mut buffer);
    fn buffer_to_lower(b: *mut buffer);
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
    fn chunk_buffer_acquire() -> *mut buffer;
    fn chunk_buffer_release(b: *mut buffer);
    fn chunk_buffer_prepare_append(b: *mut buffer, sz: size_t) -> size_t;
    fn chunkqueue_set_tempdirs(
        cq: *mut chunkqueue,
        tempdirs: *const array,
        upload_temp_file_size: off_t,
    );
    fn chunkqueue_append_file_fd(
        cq: *mut chunkqueue,
        fn_0: *const buffer,
        fd: libc::c_int,
        offset: off_t,
        len: off_t,
    );
    fn chunkqueue_append_mem(cq: *mut chunkqueue, mem: *const libc::c_char, len: size_t);
    fn chunkqueue_prepend_buffer_open(cq: *mut chunkqueue) -> *mut buffer;
    fn chunkqueue_prepend_buffer_commit(cq: *mut chunkqueue);
    fn chunkqueue_append_buffer_open_sz(cq: *mut chunkqueue, sz: size_t) -> *mut buffer;
    fn chunkqueue_append_buffer_commit(cq: *mut chunkqueue);
    fn chunkqueue_mark_written(cq: *mut chunkqueue, len: off_t);
    fn chunkqueue_remove_finished_chunks(cq: *mut chunkqueue);
    fn chunkqueue_write_chunk(
        fd: libc::c_int,
        cq: *mut chunkqueue,
        errh: *mut log_error_st,
    ) -> ssize_t;
    fn chunkqueue_read_squash(
        cq: *mut chunkqueue,
        errh: *mut log_error_st,
    ) -> *mut buffer;
    fn http_status_append(b: *mut buffer, status: libc::c_int);
    fn buffer_extend(b: *mut buffer, x: size_t) -> *mut libc::c_char;
    fn buffer_string_prepare_append(b: *mut buffer, size: size_t) -> *mut libc::c_char;
    fn fdevent_open_cloexec(
        pathname: *const libc::c_char,
        symlinks: libc::c_int,
        flags: libc::c_int,
        mode: mode_t,
    ) -> libc::c_int;
    fn fdevent_mkostemp(path: *mut libc::c_char, flags: libc::c_int) -> libc::c_int;
    fn fdevent_open_dirname(
        path: *mut libc::c_char,
        symlinks: libc::c_int,
    ) -> libc::c_int;
    fn http_chunk_append_buffer(r: *mut request_st, mem: *mut buffer) -> libc::c_int;
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
    fn http_etag_create(etag: *mut buffer, st: *const stat, flags: libc::c_int);
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
    fn http_header_response_append(
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
    fn log_error(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        fmt: *const libc::c_char,
        _: ...
    );
    fn http_response_redirect_to_directory(
        r: *mut request_st,
        status: libc::c_int,
    ) -> libc::c_int;
    fn stat_cache_mimetype_by_ext(
        mimetypes: *const array,
        name: *const libc::c_char,
        nlen: uint32_t,
    ) -> *const buffer;
    fn stat_cache_update_entry(
        name: *const libc::c_char,
        len: uint32_t,
        st: *const stat,
        etagb: *const buffer,
    );
    fn stat_cache_delete_entry(name: *const libc::c_char, len: uint32_t);
    fn stat_cache_delete_dir(name: *const libc::c_char, len: uint32_t);
    fn stat_cache_invalidate_entry(name: *const libc::c_char, len: uint32_t);
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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn sendfile(
        __out_fd: libc::c_int,
        __in_fd: libc::c_int,
        __offset: *mut __off64_t,
        __count: size_t,
    ) -> ssize_t;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
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
pub type __loff_t = __off64_t;
pub type loff_t = __loff_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
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
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type unix_time64_t = time_t;
pub type unix_timespec64_t = timespec;
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
pub struct dirent {
    pub d_ino: __ino64_t,
    pub d_off: __off64_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type C2RustUnnamed = libc::c_uint;
pub const DT_WHT: C2RustUnnamed = 14;
pub const DT_SOCK: C2RustUnnamed = 12;
pub const DT_LNK: C2RustUnnamed = 10;
pub const DT_REG: C2RustUnnamed = 8;
pub const DT_BLK: C2RustUnnamed = 6;
pub const DT_DIR: C2RustUnnamed = 4;
pub const DT_CHR: C2RustUnnamed = 2;
pub const DT_FIFO: C2RustUnnamed = 1;
pub const DT_UNKNOWN: C2RustUnnamed = 0;
pub type DIR = __dirstream;
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
    pub plugins: C2RustUnnamed_0,
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
pub struct C2RustUnnamed_0 {
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
    pub __in6_u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
pub type physical_st = physical;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_data {
    pub id: libc::c_int,
    pub nconfig: libc::c_int,
    pub cvlist: *mut config_plugin_value_t,
    pub self_0: *mut plugin,
    pub defaults: plugin_config,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_config {
    pub enabled: libc::c_ushort,
    pub is_readonly: libc::c_ushort,
    pub log_xml: libc::c_ushort,
    pub opts: libc::c_ushort,
    pub sql: *mut sql_config,
    pub tmpb: *mut buffer,
    pub sqlite_db_name: *mut buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sql_config {
    pub dummy: libc::c_int,
}
pub const MOD_WEBDAV_UNSAFE_PROPFIND_FOLLOW_SYMLINK: C2RustUnnamed_8 = 2;
pub const MOD_WEBDAV_PROPFIND_DEPTH_INFINITY: C2RustUnnamed_8 = 4;
pub const MOD_WEBDAV_UNSAFE_PARTIAL_PUT_COMPAT: C2RustUnnamed_8 = 1;
pub const _ISupper: C2RustUnnamed_7 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct webdav_propfind_bufs {
    pub r: *mut request_st,
    pub pconf: *const plugin_config,
    pub dst: *mut physical_st,
    pub b: *mut buffer,
    pub b_200: *mut buffer,
    pub b_404: *mut buffer,
    pub proplist: webdav_property_names,
    pub allprop: libc::c_int,
    pub propname: libc::c_int,
    pub lockdiscovery: libc::c_int,
    pub depth: libc::c_int,
    pub recursed: libc::c_int,
    pub atflags: libc::c_int,
    pub st: stat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct webdav_property_names {
    pub ptr: *mut webdav_property_name,
    pub used: libc::c_int,
    pub size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct webdav_property_name {
    pub ns: *const libc::c_char,
    pub name: *const libc::c_char,
    pub nslen: uint32_t,
    pub namelen: uint32_t,
}
pub type webdav_live_props_e = libc::c_int;
pub const WEBDAV_PROP_SUPPORTEDLOCK: webdav_live_props_e = 6;
pub const WEBDAV_PROP_RESOURCETYPE: webdav_live_props_e = 5;
pub const WEBDAV_PROP_GETLASTMODIFIED: webdav_live_props_e = 4;
pub const WEBDAV_PROP_GETETAG: webdav_live_props_e = 3;
pub const WEBDAV_PROP_GETCONTENTTYPE: webdav_live_props_e = 2;
pub const WEBDAV_PROP_GETCONTENTLENGTH: webdav_live_props_e = 1;
pub const WEBDAV_PROP_ALL: webdav_live_props_e = 0;
pub const WEBDAV_PROP_UNSET: webdav_live_props_e = -1;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_7 = 8;
pub const _ISpunct: C2RustUnnamed_7 = 4;
pub const _IScntrl: C2RustUnnamed_7 = 2;
pub const _ISblank: C2RustUnnamed_7 = 1;
pub const _ISgraph: C2RustUnnamed_7 = 32768;
pub const _ISprint: C2RustUnnamed_7 = 16384;
pub const _ISspace: C2RustUnnamed_7 = 8192;
pub const _ISxdigit: C2RustUnnamed_7 = 4096;
pub const _ISdigit: C2RustUnnamed_7 = 2048;
pub const _ISalpha: C2RustUnnamed_7 = 1024;
pub const _ISlower: C2RustUnnamed_7 = 512;
pub type C2RustUnnamed_8 = libc::c_uint;
#[inline]
unsafe extern "C" fn buffer_has_slash_suffix(b: *const buffer) -> libc::c_int {
    return ((*b).used > 1 as libc::c_int as libc::c_uint
        && *((*b).ptr)
            .offset(((*b).used).wrapping_sub(2 as libc::c_int as libc::c_uint) as isize)
            as libc::c_int == '/' as i32) as libc::c_int;
}
#[inline]
unsafe extern "C" fn buffer_has_pathsep_suffix(b: *const buffer) -> libc::c_int {
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
unsafe extern "C" fn buffer_clear(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn chunkqueue_is_empty(mut cq: *const chunkqueue) -> libc::c_int {
    return (0 as *mut libc::c_void as *mut chunk == (*cq).first) as libc::c_int;
}
static mut has_proc_self_fd: libc::c_int = 0;
#[cold]
#[inline(never)]
unsafe extern "C" fn http_status_set_error(
    r: *mut request_st,
    mut status: libc::c_int,
) -> libc::c_int {
    (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
    (*r).handler_module = 0 as *const plugin;
    (*r).http_status = status;
    return (*r).http_status;
}
#[no_mangle]
pub unsafe extern "C" fn mod_webdav_plugin_init(mut p: *mut plugin) -> libc::c_int {
    (*p).version = 0x10440 as libc::c_int as size_t;
    (*p).name = b"webdav\0" as *const u8 as *const libc::c_char;
    (*p)
        .init = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
    >(Some(mod_webdav_init as unsafe extern "C" fn() -> *mut libc::c_void));
    (*p)
        .cleanup = Some(
        mod_webdav_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*p)
        .set_defaults = Some(
        mod_webdav_set_defaults
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .worker_init = Some(
        mod_webdav_worker_init
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_uri_clean = Some(
        mod_webdav_uri_handler
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_physical = Some(
        mod_webdav_physical_handler
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_subrequest = Some(
        mod_webdav_subrequest_handler
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_request_reset = Some(
        mod_webdav_handle_reset
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline(never)]
unsafe extern "C" fn webdav_str_len_to_lower(ss: *mut libc::c_char, len: uint32_t) {
    let s: *mut libc::c_uchar = ss as *mut libc::c_uchar;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len as libc::c_int {
        if *(*__ctype_b_loc()).offset(*s.offset(i as isize) as libc::c_int as isize)
            as libc::c_int & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            *s
                .offset(
                    i as isize,
                ) = ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *s.offset(i as isize) as libc::c_int;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = tolower(*s.offset(i as isize) as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(*s.offset(i as isize) as libc::c_int as isize);
                }
                __res
            }) as libc::c_uchar;
        }
        i += 1;
    }
}
#[cold]
unsafe extern "C" fn mod_webdav_init() -> *mut libc::c_void {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<plugin_data>() as libc::c_ulong,
    );
}
#[cold]
unsafe extern "C" fn mod_webdav_free(mut p_d: *mut libc::c_void) {
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
                _ => {}
            }
            cpv = cpv.offset(1);
        }
        i += 1;
    }
}
unsafe extern "C" fn mod_webdav_merge_config_cpv(
    pconf: *mut plugin_config,
    cpv: *const config_plugin_value_t,
) {
    match (*cpv).k_id {
        0 => {
            if (*cpv).vtype as libc::c_uint
                == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
            {
                (*pconf).sql = (*cpv).v.v as *mut sql_config;
            }
        }
        1 => {
            (*pconf).enabled = (*cpv).v.u as libc::c_ushort;
        }
        2 => {
            (*pconf).is_readonly = (*cpv).v.u as libc::c_ushort;
        }
        3 => {
            (*pconf).log_xml = (*cpv).v.u as libc::c_ushort;
        }
        4 => {
            if (*cpv).vtype as libc::c_uint
                == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
            {
                (*pconf).opts = (*cpv).v.u as libc::c_ushort;
            }
        }
        _ => return,
    };
}
unsafe extern "C" fn mod_webdav_merge_config(
    pconf: *mut plugin_config,
    mut cpv: *const config_plugin_value_t,
) {
    loop {
        mod_webdav_merge_config_cpv(pconf, cpv);
        cpv = cpv.offset(1);
        if !((*cpv).k_id != -(1 as libc::c_int)) {
            break;
        }
    };
}
unsafe extern "C" fn mod_webdav_patch_config(
    r: *mut request_st,
    p: *mut plugin_data,
    pconf: *mut plugin_config,
) {
    memcpy(
        pconf as *mut libc::c_void,
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
            mod_webdav_merge_config(
                pconf,
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
unsafe extern "C" fn mod_webdav_set_defaults(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    if config_plugin_values_init(
        srv,
        p as *mut libc::c_void,
        cpk.as_ptr(),
        b"mod_webdav\0" as *const u8 as *const libc::c_char,
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
            let mut current_block_15: u64;
            match (*cpv).k_id {
                0 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        if mod_webdav_sqlite3_init((*(*cpv).v.b).ptr, (*srv).errh) == 0 {
                            return HANDLER_ERROR;
                        }
                    }
                    current_block_15 = 11636175345244025579;
                }
                1 => {
                    current_block_15 = 16264355343275444748;
                }
                2 => {
                    current_block_15 = 16264355343275444748;
                }
                3 => {
                    current_block_15 = 3528444967502634402;
                }
                4 => {
                    if (*(*cpv).v.a).used != 0 {
                        let mut opts: libc::c_ushort = 0 as libc::c_int
                            as libc::c_ushort;
                        let mut j: uint32_t = 0 as libc::c_int as uint32_t;
                        let mut used: uint32_t = (*(*cpv).v.a).used;
                        while j < used {
                            let mut ds: *mut data_string = *((*(*cpv).v.a).data)
                                .offset(j as isize) as *mut data_string;
                            if buffer_eq_slen(
                                &mut (*ds).key,
                                b"deprecated-unsafe-partial-put\0" as *const u8
                                    as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 30]>()
                                    as libc::c_ulong as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                            ) != 0
                                && config_plugin_value_tobool(
                                    ds as *mut data_unset,
                                    0 as libc::c_int,
                                ) != 0
                            {
                                opts = (opts as libc::c_int
                                    | MOD_WEBDAV_UNSAFE_PARTIAL_PUT_COMPAT as libc::c_int)
                                    as libc::c_ushort;
                            } else if buffer_eq_slen(
                                    &mut (*ds).key,
                                    b"propfind-depth-infinity\0" as *const u8
                                        as *const libc::c_char,
                                    (::std::mem::size_of::<[libc::c_char; 24]>()
                                        as libc::c_ulong as uint32_t)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                ) != 0
                                    && config_plugin_value_tobool(
                                        ds as *mut data_unset,
                                        0 as libc::c_int,
                                    ) != 0
                                {
                                opts = (opts as libc::c_int
                                    | MOD_WEBDAV_PROPFIND_DEPTH_INFINITY as libc::c_int)
                                    as libc::c_ushort;
                            } else if buffer_eq_slen(
                                    &mut (*ds).key,
                                    b"unsafe-propfind-follow-symlink\0" as *const u8
                                        as *const libc::c_char,
                                    (::std::mem::size_of::<[libc::c_char; 31]>()
                                        as libc::c_ulong as uint32_t)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                ) != 0
                                    && config_plugin_value_tobool(
                                        ds as *mut data_unset,
                                        0 as libc::c_int,
                                    ) != 0
                                {
                                opts = (opts as libc::c_int
                                    | MOD_WEBDAV_UNSAFE_PROPFIND_FOLLOW_SYMLINK as libc::c_int)
                                    as libc::c_ushort;
                            } else {
                                log_error(
                                    (*srv).errh,
                                    b"src/mod_webdav.c\0" as *const u8 as *const libc::c_char,
                                    568 as libc::c_int as libc::c_uint,
                                    b"unrecognized webdav.opts: %s\0" as *const u8
                                        as *const libc::c_char,
                                    (*ds).key.ptr,
                                );
                                return HANDLER_ERROR;
                            }
                            j = j.wrapping_add(1);
                        }
                        (*cpv).v.u = opts as libc::c_uint;
                        (*cpv).vtype = T_CONFIG_LOCAL;
                    }
                    current_block_15 = 11636175345244025579;
                }
                _ => {
                    current_block_15 = 11636175345244025579;
                }
            }
            match current_block_15 {
                16264355343275444748 => {
                    current_block_15 = 3528444967502634402;
                }
                _ => {}
            }
            match current_block_15 {
                3528444967502634402 => {}
                _ => {}
            }
            cpv = cpv.offset(1);
        }
        i += 1;
    }
    (*p).defaults.tmpb = (*srv).tmp_buf;
    if (*p).nconfig > 0 as libc::c_int
        && (*(*p).cvlist).v.u2[1 as libc::c_int as usize] != 0
    {
        let mut cpv_0: *const config_plugin_value_t = ((*p).cvlist)
            .offset((*(*p).cvlist).v.u2[0 as libc::c_int as usize] as isize);
        if -(1 as libc::c_int) != (*cpv_0).k_id {
            mod_webdav_merge_config(&mut (*p).defaults, cpv_0);
        }
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
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    has_proc_self_fd = (0 as libc::c_int
        == stat(b"/proc/self/fd\0" as *const u8 as *const libc::c_char, &mut st))
        as libc::c_int;
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_webdav_uri_handler(
    mut r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    if (*r).http_method as libc::c_int != HTTP_METHOD_OPTIONS as libc::c_int {
        return HANDLER_GO_ON;
    }
    let mut pconf: plugin_config = plugin_config {
        enabled: 0,
        is_readonly: 0,
        log_xml: 0,
        opts: 0,
        sql: 0 as *mut sql_config,
        tmpb: 0 as *mut buffer,
        sqlite_db_name: 0 as *mut buffer,
    };
    mod_webdav_patch_config(r, p_d as *mut plugin_data, &mut pconf);
    if pconf.enabled == 0 {
        return HANDLER_GO_ON;
    }
    http_header_response_set(
        r,
        HTTP_HEADER_OTHER,
        b"DAV\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
        b"1,3\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    http_header_response_set(
        r,
        HTTP_HEADER_OTHER,
        b"MS-Author-Via\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
        b"DAV\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if pconf.is_readonly != 0 {
        http_header_response_append(
            r,
            HTTP_HEADER_ALLOW,
            b"Allow\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            b"PROPFIND\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    } else {
        http_header_response_append(
            r,
            HTTP_HEADER_ALLOW,
            b"Allow\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            b"PROPFIND, DELETE, MKCOL, PUT, MOVE, COPY\0" as *const u8
                as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 41]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn webdav_double_buffer(r: *mut request_st, b: *mut buffer) {
    if buffer_clen(b) > 60000 as libc::c_int as libc::c_uint {
        http_chunk_append_buffer(r, b);
    }
}
#[cold]
#[inline(never)]
unsafe extern "C" fn webdav_xml_log_response(r: *mut request_st) {
    let cq: *mut chunkqueue = &mut (*r).write_queue;
    let errh: *mut log_error_st = (*r).conf.errh;
    if chunkqueue_length(cq) <= 65536 as libc::c_int as libc::c_long {
        chunkqueue_read_squash(cq, errh);
    }
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: uint32_t = 0;
    let mut current_block_6: u64;
    let mut c: *mut chunk = (*cq).first;
    while !c.is_null() {
        match (*c).type_0 as libc::c_uint {
            0 => {
                s = ((*(*c).mem).ptr).offset((*c).offset as isize);
                len = (buffer_clen((*c).mem)).wrapping_sub((*c).offset as uint32_t);
                current_block_6 = 4166486009154926805;
            }
            1 => {
                s = webdav_mmap_file_chunk(c);
                len = (*c).file.length as uint32_t;
                if s.is_null() {
                    current_block_6 = 820271813250567934;
                } else {
                    current_block_6 = 4166486009154926805;
                }
            }
            _ => {
                current_block_6 = 820271813250567934;
            }
        }
        match current_block_6 {
            4166486009154926805 => {
                log_error(
                    errh,
                    b"src/mod_webdav.c\0" as *const u8 as *const libc::c_char,
                    771 as libc::c_int as libc::c_uint,
                    b"XML-response-body: %.*s\0" as *const u8 as *const libc::c_char,
                    len as libc::c_int,
                    s,
                );
            }
            _ => {}
        }
        c = (*c).next;
    }
}
unsafe extern "C" fn webdav_xml_doctype(b: *mut buffer, r: *mut request_st) {
    http_header_response_set(
        r,
        HTTP_HEADER_CONTENT_TYPE,
        b"Content-Type\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
        b"application/xml; charset=\"utf-8\"\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    buffer_copy_string_len(
        b,
        b"<?xml version=\"1.0\" encoding=\"utf-8\"?>\n\0" as *const u8
            as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
}
unsafe extern "C" fn webdav_xml_prop(
    b: *mut buffer,
    prop: *const webdav_property_name,
    value: *const libc::c_char,
    vlen: uint32_t,
) {
    if 0 as libc::c_int as libc::c_uint == vlen {
        let mut iov: [const_iovec; 5] = [
            {
                let mut init = const_iovec {
                    iov_base: b"<\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    iov_len: (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                };
                init
            },
            {
                let mut init = const_iovec {
                    iov_base: (*prop).name as *const libc::c_void,
                    iov_len: (*prop).namelen as size_t,
                };
                init
            },
            {
                let mut init = const_iovec {
                    iov_base: b" xmlns=\"\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    iov_len: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                };
                init
            },
            {
                let mut init = const_iovec {
                    iov_base: (*prop).ns as *const libc::c_void,
                    iov_len: (*prop).nslen as size_t,
                };
                init
            },
            {
                let mut init = const_iovec {
                    iov_base: b"\"/>\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    iov_len: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                };
                init
            },
        ];
        buffer_append_iovec(
            b,
            iov.as_mut_ptr(),
            (::std::mem::size_of::<[const_iovec; 5]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<const_iovec>() as libc::c_ulong),
        );
    } else {
        let mut iov_0: [const_iovec; 9] = [
            {
                let mut init = const_iovec {
                    iov_base: b"<\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    iov_len: (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                };
                init
            },
            {
                let mut init = const_iovec {
                    iov_base: (*prop).name as *const libc::c_void,
                    iov_len: (*prop).namelen as size_t,
                };
                init
            },
            {
                let mut init = const_iovec {
                    iov_base: b" xmlns=\"\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    iov_len: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                };
                init
            },
            {
                let mut init = const_iovec {
                    iov_base: (*prop).ns as *const libc::c_void,
                    iov_len: (*prop).nslen as size_t,
                };
                init
            },
            {
                let mut init = const_iovec {
                    iov_base: b"\">\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    iov_len: (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                };
                init
            },
            {
                let mut init = const_iovec {
                    iov_base: value as *const libc::c_void,
                    iov_len: vlen as size_t,
                };
                init
            },
            {
                let mut init = const_iovec {
                    iov_base: b"</\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    iov_len: (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                };
                init
            },
            {
                let mut init = const_iovec {
                    iov_base: (*prop).name as *const libc::c_void,
                    iov_len: (*prop).namelen as size_t,
                };
                init
            },
            {
                let mut init = const_iovec {
                    iov_base: b">\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    iov_len: (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                };
                init
            },
        ];
        buffer_append_iovec(
            b,
            iov_0.as_mut_ptr(),
            (::std::mem::size_of::<[const_iovec; 9]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<const_iovec>() as libc::c_ulong),
        );
    };
}
unsafe extern "C" fn webdav_xml_href(b: *mut buffer, href: *const buffer) {
    buffer_append_string_len(
        b,
        b"<D:href>\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_string_encoded(
        b,
        (*href).ptr,
        buffer_clen(href) as size_t,
        ENCODING_REL_URI,
    );
    buffer_append_string_len(
        b,
        b"</D:href>\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
}
unsafe extern "C" fn webdav_xml_status(b: *mut buffer, status: libc::c_int) {
    buffer_append_string_len(
        b,
        b"<D:status>HTTP/1.1 \0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    http_status_append(b, status);
    buffer_append_string_len(
        b,
        b"</D:status>\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
}
unsafe extern "C" fn webdav_xml_propstat(
    b: *mut buffer,
    value: *mut buffer,
    status: libc::c_int,
) {
    buffer_append_str3(
        b,
        b"<D:propstat>\n<D:prop>\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        (*value).ptr,
        buffer_clen(value) as size_t,
        b"</D:prop>\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    webdav_xml_status(b, status);
    buffer_append_string_len(
        b,
        b"</D:propstat>\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
}
#[cold]
unsafe extern "C" fn webdav_xml_response_status(
    r: *mut request_st,
    href: *const buffer,
    status: libc::c_int,
) {
    let b: *mut buffer = chunk_buffer_acquire();
    buffer_append_string_len(
        b,
        b"<D:response>\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    webdav_xml_href(b, href);
    webdav_xml_status(b, status);
    buffer_append_string_len(
        b,
        b"</D:response>\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    http_chunk_append_buffer(r, b);
    chunk_buffer_release(b);
}
unsafe extern "C" fn webdav_xml_doc_multistatus(
    r: *mut request_st,
    pconf: *const plugin_config,
) {
    (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
    (*r).handler_module = 0 as *const plugin;
    (*r).http_status = 207 as libc::c_int;
    let cq: *mut chunkqueue = &mut (*r).write_queue;
    let b: *mut buffer = chunkqueue_prepend_buffer_open(cq);
    webdav_xml_doctype(b, r);
    buffer_append_string_len(
        b,
        b"<D:multistatus xmlns:D=\"DAV:\">\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    chunkqueue_prepend_buffer_commit(cq);
    chunkqueue_append_mem(
        cq,
        b"</D:multistatus>\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    if (*pconf).log_xml != 0 {
        webdav_xml_log_response(r);
    }
}
#[cold]
unsafe extern "C" fn webdav_xml_doc_error_propfind_finite_depth(r: *mut request_st) {
    (*r).http_status = 403 as libc::c_int;
    (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
    let b: *mut buffer = chunkqueue_append_buffer_open_sz(
        &mut (*r).write_queue,
        256 as libc::c_int as size_t,
    );
    webdav_xml_doctype(b, r);
    buffer_append_string_len(
        b,
        b"<D:error><DAV:propfind-finite-depth/></D:error>\n\0" as *const u8
            as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 49]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    chunkqueue_append_buffer_commit(&mut (*r).write_queue);
}
#[cold]
unsafe extern "C" fn mod_webdav_sqlite3_init(
    dbname: *const libc::c_char,
    errh: *mut log_error_st,
) -> libc::c_int {
    log_error(
        errh,
        b"src/mod_webdav.c\0" as *const u8 as *const libc::c_char,
        1282 as libc::c_int as libc::c_uint,
        b"Sorry, no sqlite3 and libxml2 support include, compile with --with-webdav-props\0"
            as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
#[cold]
unsafe extern "C" fn mod_webdav_worker_init(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    return HANDLER_GO_ON;
}
unsafe extern "C" fn webdav_lock_delete_uri(
    pconf: *const plugin_config,
    uri: *const buffer,
) -> libc::c_int {
    return 1 as libc::c_int;
}
unsafe extern "C" fn webdav_lock_delete_uri_col(
    pconf: *const plugin_config,
    uri: *const buffer,
) -> libc::c_int {
    return 1 as libc::c_int;
}
unsafe extern "C" fn webdav_prop_move_uri(
    pconf: *const plugin_config,
    src: *const buffer,
    dst: *const buffer,
) -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn webdav_prop_move_uri_col(
    pconf: *const plugin_config,
    src: *const buffer,
    dst: *const buffer,
) -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn webdav_prop_delete_uri(
    pconf: *const plugin_config,
    uri: *const buffer,
) -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn webdav_prop_copy_uri(
    pconf: *const plugin_config,
    src: *const buffer,
    dst: *const buffer,
) -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn webdav_prop_select_prop(
    pconf: *const plugin_config,
    uri: *const buffer,
    prop: *const webdav_property_name,
    b: *mut buffer,
) -> libc::c_int {
    return -(1 as libc::c_int);
}
unsafe extern "C" fn webdav_prop_select_props(
    pconf: *const plugin_config,
    uri: *const buffer,
    b: *mut buffer,
) {}
unsafe extern "C" fn webdav_prop_select_propnames(
    pconf: *const plugin_config,
    uri: *const buffer,
    b: *mut buffer,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[inline(never)]
unsafe extern "C" fn webdav_fcopyfile_sz(
    mut ifd: libc::c_int,
    mut ofd: libc::c_int,
    mut isz: off_t,
) -> libc::c_int {
    if 0 as libc::c_int as libc::c_long == isz {
        return 0 as libc::c_int;
    }
    let mut offset: off_t = 0 as libc::c_int as off_t;
    while offset < isz
        && sendfile(ifd, ofd, &mut offset, (isz - offset) as size_t)
            >= 0 as libc::c_int as libc::c_long
    {}
    if offset == isz {
        return 0 as libc::c_int;
    }
    if 0 as libc::c_int as libc::c_long
        != lseek(ofd, 0 as libc::c_int as __off64_t, 0 as libc::c_int)
    {
        return -(1 as libc::c_int);
    }
    let mut rd: ssize_t = 0;
    let mut wr: ssize_t = 0;
    let mut off: ssize_t = 0;
    let mut buf: [libc::c_char; 16384] = [0; 16384];
    isz = 0 as libc::c_int as off_t;
    loop {
        loop {
            rd = read(
                ifd,
                buf.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong,
            );
            if !(-(1 as libc::c_int) as libc::c_long == rd
                && *__errno_location() == 4 as libc::c_int)
            {
                break;
            }
        }
        if rd <= 0 as libc::c_int as libc::c_long {
            break;
        }
        off = 0 as libc::c_int as ssize_t;
        loop {
            wr = write(
                ofd,
                buf.as_mut_ptr().offset(off as isize) as *const libc::c_void,
                (rd - off) as size_t,
            );
            if !(if wr >= 0 as libc::c_int as libc::c_long {
                off += wr;
                (off != rd) as libc::c_int
            } else {
                (*__errno_location() == 4 as libc::c_int) as libc::c_int
            } != 0)
            {
                break;
            }
        }
        if wr < 0 as libc::c_int as libc::c_long {
            return -(1 as libc::c_int);
        }
        isz += rd;
        if !(isz != 0) {
            break;
        }
    }
    if 0 as libc::c_int as libc::c_long == rd {
        return ftruncate(ofd, isz);
    }
    return rd as libc::c_int;
}
unsafe extern "C" fn webdav_if_match_or_unmodified_since(
    r: *mut request_st,
    mut st: *mut stat,
) -> libc::c_int {
    let mut im: *const buffer = if 0 as libc::c_int
        != (*r).conf.etag_flags as libc::c_int
    {
        http_header_request_get(
            r,
            HTTP_HEADER_IF_MATCH,
            b"If-Match\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        )
    } else {
        0 as *mut buffer
    };
    let mut inm: *const buffer = if 0 as libc::c_int
        != (*r).conf.etag_flags as libc::c_int
    {
        http_header_request_get(
            r,
            HTTP_HEADER_IF_NONE_MATCH,
            b"If-None-Match\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        )
    } else {
        0 as *mut buffer
    };
    let mut ius: *const buffer = http_header_request_get(
        r,
        HTTP_HEADER_IF_UNMODIFIED_SINCE,
        b"If-Unmodified-Since\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if im.is_null() && inm.is_null() && ius.is_null() {
        return 0 as libc::c_int;
    }
    let mut stp: stat = stat {
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
    if st.is_null() {
        st = if 0 as libc::c_int == lstat((*r).physical.path.ptr, &mut stp) {
            &mut stp
        } else {
            0 as *mut stat
        };
    }
    let etagb: *mut buffer = (*r).tmp_buf;
    buffer_clear(etagb);
    if !st.is_null() && (!im.is_null() || !inm.is_null()) {
        http_etag_create(etagb, st, (*r).conf.etag_flags as libc::c_int);
    }
    if !im.is_null() {
        if st.is_null() || http_etag_matches(etagb, (*im).ptr, 0 as libc::c_int) == 0 {
            return 412 as libc::c_int;
        }
    }
    if !inm.is_null() {
        if if st.is_null() {
            (buffer_eq_slen(
                inm,
                b"*\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
                || *__errno_location() != 2 as libc::c_int
                    && *__errno_location() != 20 as libc::c_int) as libc::c_int
        } else {
            http_etag_matches(etagb, (*inm).ptr, 1 as libc::c_int)
        } != 0
        {
            return 412 as libc::c_int;
        }
    }
    if !ius.is_null() {
        if st.is_null() {
            return 412 as libc::c_int;
        }
        if http_date_if_modified_since(
            (*ius).ptr,
            buffer_clen(ius),
            (*st).st_mtim.tv_sec,
        ) != 0
        {
            return 412 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn webdav_response_etag(r: *mut request_st, mut st: *mut stat) {
    let mut etagb: *mut buffer = 0 as *mut buffer;
    if 0 as libc::c_int != (*r).conf.etag_flags as libc::c_int {
        etagb = http_header_response_set_ptr(
            r,
            HTTP_HEADER_ETAG,
            b"ETag\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        http_etag_create(etagb, st, (*r).conf.etag_flags as libc::c_int);
    }
    stat_cache_update_entry(
        (*r).physical.path.ptr,
        buffer_clen(&mut (*r).physical.path),
        st,
        etagb,
    );
}
unsafe extern "C" fn webdav_parent_modified(mut path: *const buffer) {
    let mut dirlen: uint32_t = buffer_clen(path);
    let mut fn_0: *const libc::c_char = (*path).ptr;
    if *fn_0.offset(dirlen.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
        as libc::c_int == '/' as i32
    {
        dirlen = dirlen.wrapping_sub(1);
    }
    if 0 as libc::c_int as libc::c_uint != dirlen {
        loop {
            dirlen = dirlen.wrapping_sub(1);
            if !(*fn_0.offset(dirlen as isize) as libc::c_int != '/' as i32) {
                break;
            }
        }
    }
    if 0 as libc::c_int as libc::c_uint == dirlen {
        dirlen = 1 as libc::c_int as uint32_t;
    }
    stat_cache_invalidate_entry(fn_0, dirlen);
}
unsafe extern "C" fn webdav_parse_Depth(r: *const request_st) -> libc::c_int {
    let h: *const buffer = http_header_request_get(
        r,
        HTTP_HEADER_OTHER,
        b"Depth\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if !h.is_null() {
        match *(*h).ptr as libc::c_int {
            48 => return 0 as libc::c_int,
            49 => return 1 as libc::c_int,
            _ => return -(1 as libc::c_int),
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn webdav_unlinkat(
    pconf: *const plugin_config,
    dst: *const physical_st,
    dfd: libc::c_int,
    d_name: *const libc::c_char,
) -> libc::c_int {
    if 0 as libc::c_int == unlinkat(dfd, d_name, 0 as libc::c_int) {
        stat_cache_delete_entry((*dst).path.ptr, buffer_clen(&(*dst).path));
        return webdav_prop_delete_uri(pconf, &(*dst).rel_path);
    }
    match *__errno_location() {
        13 | 1 => return 403 as libc::c_int,
        2 => return 404 as libc::c_int,
        _ => return 501 as libc::c_int,
    };
}
unsafe extern "C" fn webdav_delete_file(
    pconf: *const plugin_config,
    dst: *const physical_st,
) -> libc::c_int {
    if 0 as libc::c_int == unlink((*dst).path.ptr) {
        stat_cache_delete_entry((*dst).path.ptr, buffer_clen(&(*dst).path));
        return webdav_prop_delete_uri(pconf, &(*dst).rel_path);
    }
    match *__errno_location() {
        13 | 1 => return 403 as libc::c_int,
        2 => return 404 as libc::c_int,
        _ => return 501 as libc::c_int,
    };
}
unsafe extern "C" fn webdav_delete_dir(
    pconf: *const plugin_config,
    dst: *mut physical_st,
    r: *mut request_st,
    flags: libc::c_int,
) -> libc::c_int {
    let mut multi_status: libc::c_int = 0 as libc::c_int;
    let dfd: libc::c_int = fdevent_open_dirname((*dst).path.ptr, 0 as libc::c_int);
    let dir: *mut DIR = if dfd >= 0 as libc::c_int {
        fdopendir(dfd)
    } else {
        0 as *mut DIR
    };
    if dir.is_null() {
        if dfd >= 0 as libc::c_int {
            close(dfd);
        }
        webdav_xml_response_status(r, &mut (*dst).rel_path, 403 as libc::c_int);
        return 1 as libc::c_int;
    }
    let dst_path_used: uint32_t = (*dst).path.used;
    let dst_rel_path_used: uint32_t = (*dst).rel_path.used;
    let mut s_isdir: libc::c_int = 0;
    let mut de: *mut dirent = 0 as *mut dirent;
    loop {
        de = readdir(dir);
        if de.is_null() {
            break;
        }
        if (*de).d_name[0 as libc::c_int as usize] as libc::c_int == '.' as i32
            && ((*de).d_name[1 as libc::c_int as usize] as libc::c_int == '\u{0}' as i32
                || (*de).d_name[1 as libc::c_int as usize] as libc::c_int == '.' as i32
                    && (*de).d_name[2 as libc::c_int as usize] as libc::c_int
                        == '\u{0}' as i32)
        {
            continue;
        }
        if (*de).d_type as libc::c_int != DT_UNKNOWN as libc::c_int {
            s_isdir = ((*de).d_type as libc::c_int == DT_DIR as libc::c_int)
                as libc::c_int;
        } else {
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
            if 0 as libc::c_int
                != fstatat(
                    dfd,
                    ((*de).d_name).as_mut_ptr(),
                    &mut st,
                    0x100 as libc::c_int,
                )
            {
                continue;
            }
            s_isdir = (st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int;
        }
        let len: uint32_t = strlen(((*de).d_name).as_mut_ptr()) as uint32_t;
        if flags & 0x1 as libc::c_int != 0 {
            webdav_str_len_to_lower(((*de).d_name).as_mut_ptr(), len);
        }
        buffer_append_string_len(
            &mut (*dst).path,
            ((*de).d_name).as_mut_ptr(),
            len as size_t,
        );
        buffer_append_string_len(
            &mut (*dst).rel_path,
            ((*de).d_name).as_mut_ptr(),
            len as size_t,
        );
        if s_isdir != 0 {
            buffer_append_string_len(
                &mut (*dst).path,
                b"/\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            buffer_append_string_len(
                &mut (*dst).rel_path,
                b"/\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            multi_status |= webdav_delete_dir(pconf, dst, r, flags);
        } else {
            let mut status: libc::c_int = webdav_unlinkat(
                pconf,
                dst,
                dfd,
                ((*de).d_name).as_mut_ptr(),
            );
            if 0 as libc::c_int != status {
                webdav_xml_response_status(r, &mut (*dst).rel_path, status);
                multi_status = 1 as libc::c_int;
            }
        }
        (*dst).path.used = dst_path_used;
        *((*dst).path.ptr)
            .offset(
                ((*dst).path.used).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as isize,
            ) = '\u{0}' as i32 as libc::c_char;
        (*dst).rel_path.used = dst_rel_path_used;
        *((*dst).rel_path.ptr)
            .offset(
                ((*dst).rel_path.used).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as isize,
            ) = '\u{0}' as i32 as libc::c_char;
    }
    closedir(dir);
    if 0 as libc::c_int == multi_status {
        let mut rmdir_status: libc::c_int = 0;
        if 0 as libc::c_int == rmdir((*dst).path.ptr) {
            rmdir_status = webdav_prop_delete_uri(pconf, &mut (*dst).rel_path);
        } else {
            match *__errno_location() {
                13 | 1 => {
                    rmdir_status = 403 as libc::c_int;
                }
                2 => {
                    rmdir_status = 404 as libc::c_int;
                }
                _ => {
                    rmdir_status = 501 as libc::c_int;
                }
            }
        }
        if 0 as libc::c_int != rmdir_status {
            webdav_xml_response_status(r, &mut (*dst).rel_path, rmdir_status);
            multi_status = 1 as libc::c_int;
        }
    }
    return multi_status;
}
unsafe extern "C" fn webdav_linktmp_rename(
    pconf: *const plugin_config,
    src: *const buffer,
    dst: *const buffer,
) -> libc::c_int {
    let tmpb: *mut buffer = (*pconf).tmpb;
    let mut rc: libc::c_int = -(1 as libc::c_int);
    buffer_clear(tmpb);
    buffer_append_str2(
        tmpb,
        (*dst).ptr,
        buffer_clen(dst) as size_t,
        b".\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(tmpb, getpid() as libc::c_long);
    buffer_append_string_len(
        tmpb,
        b".\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_uint_hex_lc(tmpb, pconf as uintptr_t);
    buffer_append_string_len(
        tmpb,
        b"~\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    if buffer_clen(tmpb) < 4096 as libc::c_int as libc::c_uint
        && 0 as libc::c_int
            == linkat(
                -(100 as libc::c_int),
                (*src).ptr,
                -(100 as libc::c_int),
                (*tmpb).ptr,
                0 as libc::c_int,
            )
    {
        rc = rename((*tmpb).ptr, (*dst).ptr);
        unlink((*tmpb).ptr);
    }
    return rc;
}
unsafe extern "C" fn webdav_copytmp_rename(
    pconf: *const plugin_config,
    src: *const physical_st,
    dst: *const physical_st,
    flags: *mut libc::c_int,
) -> libc::c_int {
    let tmpb: *mut buffer = (*pconf).tmpb;
    buffer_clear(tmpb);
    buffer_append_str2(
        tmpb,
        (*dst).path.ptr,
        buffer_clen(&(*dst).path) as size_t,
        b".\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(tmpb, getpid() as libc::c_long);
    buffer_append_string_len(
        tmpb,
        b".\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_uint_hex_lc(tmpb, pconf as uintptr_t);
    buffer_append_string_len(
        tmpb,
        b"~\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    if buffer_clen(tmpb) >= 4096 as libc::c_int as libc::c_uint {
        return 414 as libc::c_int;
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
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let ifd: libc::c_int = fdevent_open_cloexec(
        (*src).path.ptr,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int as mode_t,
    );
    if ifd < 0 as libc::c_int {
        return 403 as libc::c_int;
    }
    if 0 as libc::c_int != fstat(ifd, &mut st)
        || !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint)
    {
        close(ifd);
        return 403 as libc::c_int;
    }
    let ofd: libc::c_int = fdevent_open_cloexec(
        (*tmpb).ptr,
        0 as libc::c_int,
        0o1 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int
            | 0o1000 as libc::c_int,
        (0o400 as libc::c_int | 0o200 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as mode_t,
    );
    if ofd < 0 as libc::c_int {
        close(ifd);
        return 403 as libc::c_int;
    }
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut current_block_33: u64;
    if !(0 as libc::c_int as libc::c_long == st.st_size) {
        if *flags & 0x40 as libc::c_int == 0 {
            let mut ioff: loff_t = 0 as libc::c_int as loff_t;
            let mut ooff: loff_t = 0 as libc::c_int as loff_t;
            let mut ilen: off_t = st.st_size;
            let mut wr: ssize_t = 0;
            loop {
                wr = copy_file_range(
                    ifd,
                    &mut ioff,
                    ofd,
                    &mut ooff,
                    ilen as size_t,
                    0 as libc::c_int as libc::c_uint,
                );
                if !(wr > 0 as libc::c_int as libc::c_long
                    && {
                        ilen -= wr;
                        ilen != 0
                    })
                {
                    break;
                }
            }
            if (0 as libc::c_int as libc::c_long == ilen) as libc::c_int as libc::c_long
                != 0
            {
                current_block_33 = 16799951812150840583;
            } else {
                if -(1 as libc::c_int) as libc::c_long == wr {
                    rc = *__errno_location();
                    if rc == 28 as libc::c_int {
                        current_block_33 = 16799951812150840583;
                    } else {
                        if rc == 18 as libc::c_int {
                            *flags |= 0x40 as libc::c_int;
                            if *flags & 0x8 as libc::c_int != 0 {
                                *flags &= !(0x8 as libc::c_int);
                                *flags |= 0x20 as libc::c_int;
                            }
                        }
                        current_block_33 = 14832935472441733737;
                    }
                } else {
                    current_block_33 = 14832935472441733737;
                }
                match current_block_33 {
                    16799951812150840583 => {}
                    _ => {
                        if 0 as libc::c_int as libc::c_long != ooff
                            && 0 as libc::c_int
                                != ftruncate(ofd, 0 as libc::c_int as __off64_t)
                        {
                            if 0 as libc::c_int == rc {
                                rc = *__errno_location();
                            }
                            current_block_33 = 16799951812150840583;
                        } else {
                            current_block_33 = 5330834795799507926;
                        }
                    }
                }
            }
        } else {
            current_block_33 = 5330834795799507926;
        }
        match current_block_33 {
            16799951812150840583 => {}
            _ => {
                rc = webdav_fcopyfile_sz(ifd, ofd, st.st_size);
                if (0 as libc::c_int != rc) as libc::c_int as libc::c_long != 0 {
                    rc = *__errno_location();
                }
            }
        }
    }
    close(ifd);
    let wc: libc::c_int = close(ofd);
    if (0 as libc::c_int != wc) as libc::c_int as libc::c_long != 0
        && 0 as libc::c_int == rc
    {
        rc = *__errno_location();
    }
    if (0 as libc::c_int != rc) as libc::c_int as libc::c_long != 0 {
        rc = if rc == 28 as libc::c_int {
            507 as libc::c_int
        } else {
            403 as libc::c_int
        };
        unlink((*tmpb).ptr);
        return rc;
    }
    let overwrite: libc::c_int = *flags & 0x2 as libc::c_int;
    if 0 as libc::c_int
        == renameat2(
            -(100 as libc::c_int),
            (*tmpb).ptr,
            -(100 as libc::c_int),
            (*dst).path.ptr,
            (if overwrite != 0 {
                0 as libc::c_int
            } else {
                (1 as libc::c_int) << 0 as libc::c_int
            }) as libc::c_uint,
        )
    {
        stat_cache_delete_entry((*dst).path.ptr, buffer_clen(&(*dst).path));
        return 0 as libc::c_int;
    } else {
        let errnum: libc::c_int = *__errno_location();
        unlink((*tmpb).ptr);
        match errnum {
            2 | 20 | 21 => return 409 as libc::c_int,
            17 => return 412 as libc::c_int,
            _ => return 403 as libc::c_int,
        }
    };
}
unsafe extern "C" fn webdav_copymove_file(
    pconf: *const plugin_config,
    src: *const physical_st,
    dst: *const physical_st,
    flags: *mut libc::c_int,
) -> libc::c_int {
    let overwrite: libc::c_int = *flags & 0x2 as libc::c_int;
    if *flags & 0x4 as libc::c_int != 0 {
        if 0 as libc::c_int
            == renameat2(
                -(100 as libc::c_int),
                (*src).path.ptr,
                -(100 as libc::c_int),
                (*dst).path.ptr,
                (if overwrite != 0 {
                    0 as libc::c_int
                } else {
                    (1 as libc::c_int) << 0 as libc::c_int
                }) as libc::c_uint,
            )
        {
            if overwrite != 0 {
                unlink((*src).path.ptr);
            }
            stat_cache_delete_entry((*dst).path.ptr, buffer_clen(&(*dst).path));
            stat_cache_delete_entry((*src).path.ptr, buffer_clen(&(*src).path));
            webdav_prop_move_uri(pconf, &(*src).rel_path, &(*dst).rel_path);
            return 0 as libc::c_int;
        } else {
            if *__errno_location() == 17 as libc::c_int {
                return 412 as libc::c_int;
            }
        }
    } else if *flags & 0x8 as libc::c_int != 0 {
        if 0 as libc::c_int
            == linkat(
                -(100 as libc::c_int),
                (*src).path.ptr,
                -(100 as libc::c_int),
                (*dst).path.ptr,
                0 as libc::c_int,
            )
        {
            webdav_prop_copy_uri(pconf, &(*src).rel_path, &(*dst).rel_path);
            return 0 as libc::c_int;
        } else {
            if *__errno_location() == 17 as libc::c_int {
                if overwrite == 0 {
                    return 412 as libc::c_int;
                }
                if 0 as libc::c_int
                    == webdav_linktmp_rename(pconf, &(*src).path, &(*dst).path)
                {
                    webdav_prop_copy_uri(pconf, &(*src).rel_path, &(*dst).rel_path);
                    return 0 as libc::c_int;
                }
            } else if *__errno_location() == 18 as libc::c_int {
                *flags &= !(0x8 as libc::c_int);
                *flags |= 0x20 as libc::c_int;
            }
        }
    }
    let mut status: libc::c_int = webdav_copytmp_rename(pconf, src, dst, flags);
    if 0 as libc::c_int == status {
        webdav_prop_copy_uri(pconf, &(*src).rel_path, &(*dst).rel_path);
        if *flags & (0x4 as libc::c_int | 0x10 as libc::c_int) != 0 {
            webdav_delete_file(pconf, src);
        }
    }
    return status;
}
unsafe extern "C" fn webdav_mkdir(
    pconf: *const plugin_config,
    dst: *const physical_st,
    overwrite: libc::c_int,
) -> libc::c_int {
    if 0 as libc::c_int
        == mkdir(
            (*dst).path.ptr,
            (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
                | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                    >> 3 as libc::c_int
                | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                    >> 3 as libc::c_int >> 3 as libc::c_int) as __mode_t,
        )
    {
        webdav_parent_modified(&(*dst).path);
        return 0 as libc::c_int;
    }
    match *__errno_location() {
        17 | 20 => {}
        2 => return 409 as libc::c_int,
        1 | _ => return 403 as libc::c_int,
    }
    if overwrite < 0 as libc::c_int {
        return if *__errno_location() != 20 as libc::c_int {
            405 as libc::c_int
        } else {
            409 as libc::c_int
        };
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
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut status: libc::c_int = 0;
    *((*dst).path.ptr)
        .offset(
            ((*dst).path.used).wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
        ) = '\u{0}' as i32 as libc::c_char;
    status = lstat((*dst).path.ptr, &mut st);
    *((*dst).path.ptr)
        .offset(
            ((*dst).path.used).wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
        ) = '/' as i32 as libc::c_char;
    if 0 as libc::c_int != status {
        return 409 as libc::c_int;
    }
    if overwrite == 0 {
        return 409 as libc::c_int;
    }
    if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *((*dst).path.ptr)
        .offset(
            ((*dst).path.used).wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
        ) = '\u{0}' as i32 as libc::c_char;
    *((*dst).rel_path.ptr)
        .offset(
            ((*dst).rel_path.used).wrapping_sub(2 as libc::c_int as libc::c_uint)
                as isize,
        ) = '\u{0}' as i32 as libc::c_char;
    status = webdav_delete_file(pconf, dst);
    *((*dst).path.ptr)
        .offset(
            ((*dst).path.used).wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
        ) = '/' as i32 as libc::c_char;
    *((*dst).rel_path.ptr)
        .offset(
            ((*dst).rel_path.used).wrapping_sub(2 as libc::c_int as libc::c_uint)
                as isize,
        ) = '/' as i32 as libc::c_char;
    if 0 as libc::c_int != status {
        return status;
    }
    webdav_parent_modified(&(*dst).path);
    return if 0 as libc::c_int
        == mkdir(
            (*dst).path.ptr,
            (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
                | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                    >> 3 as libc::c_int
                | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                    >> 3 as libc::c_int >> 3 as libc::c_int) as __mode_t,
        )
    {
        0 as libc::c_int
    } else {
        409 as libc::c_int
    };
}
unsafe extern "C" fn webdav_copymove_dir(
    pconf: *const plugin_config,
    src: *mut physical_st,
    dst: *mut physical_st,
    r: *mut request_st,
    mut flags: libc::c_int,
) -> libc::c_int {
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
    let mut status: libc::c_int = 0;
    let mut dfd: libc::c_int = 0;
    let mut make_destdir: libc::c_int = 1 as libc::c_int;
    let overwrite: libc::c_int = flags & 0x2 as libc::c_int;
    if flags & 0x4 as libc::c_int != 0 {
        if 0 as libc::c_int
            == renameat2(
                -(100 as libc::c_int),
                (*src).path.ptr,
                -(100 as libc::c_int),
                (*dst).path.ptr,
                (if overwrite != 0 {
                    0 as libc::c_int
                } else {
                    (1 as libc::c_int) << 0 as libc::c_int
                }) as libc::c_uint,
            )
        {
            webdav_prop_move_uri_col(pconf, &mut (*src).rel_path, &mut (*dst).rel_path);
            return 0 as libc::c_int;
        } else {
            match *__errno_location() {
                17 | 39 => {
                    if overwrite == 0 {
                        webdav_xml_response_status(
                            r,
                            &mut (*src).rel_path,
                            412 as libc::c_int,
                        );
                        return 412 as libc::c_int;
                    }
                    make_destdir = 0 as libc::c_int;
                }
                20 => {
                    if overwrite == 0 {
                        webdav_xml_response_status(
                            r,
                            &mut (*src).rel_path,
                            409 as libc::c_int,
                        );
                        return 409 as libc::c_int;
                    }
                    *((*dst).path.ptr)
                        .offset(
                            ((*dst).path.used)
                                .wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
                        ) = '\u{0}' as i32 as libc::c_char;
                    status = lstat((*dst).path.ptr, &mut st);
                    *((*dst).path.ptr)
                        .offset(
                            ((*dst).path.used)
                                .wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
                        ) = '/' as i32 as libc::c_char;
                    if 0 as libc::c_int == status {
                        if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                            == 0o40000 as libc::c_int as libc::c_uint
                        {
                            make_destdir = 0 as libc::c_int;
                        } else {
                            *((*dst).path.ptr)
                                .offset(
                                    ((*dst).path.used)
                                        .wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
                                ) = '\u{0}' as i32 as libc::c_char;
                            *((*dst).rel_path.ptr)
                                .offset(
                                    ((*dst).rel_path.used)
                                        .wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
                                ) = '\u{0}' as i32 as libc::c_char;
                            status = webdav_delete_file(pconf, dst);
                            *((*dst).path.ptr)
                                .offset(
                                    ((*dst).path.used)
                                        .wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
                                ) = '/' as i32 as libc::c_char;
                            *((*dst).rel_path.ptr)
                                .offset(
                                    ((*dst).rel_path.used)
                                        .wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
                                ) = '/' as i32 as libc::c_char;
                            if 0 as libc::c_int != status {
                                webdav_xml_response_status(r, &mut (*src).rel_path, status);
                                return status;
                            }
                            if 0 as libc::c_int
                                == rename((*src).path.ptr, (*dst).path.ptr)
                            {
                                webdav_prop_move_uri_col(
                                    pconf,
                                    &mut (*src).rel_path,
                                    &mut (*dst).rel_path,
                                );
                                return 0 as libc::c_int;
                            }
                        }
                    }
                }
                18 => {
                    flags &= !(0x4 as libc::c_int);
                    flags |= 0x10 as libc::c_int;
                }
                _ => {}
            }
        }
    }
    if make_destdir != 0 {
        status = webdav_mkdir(pconf, dst, overwrite);
        if 0 as libc::c_int != status {
            webdav_xml_response_status(r, &mut (*src).rel_path, status);
            return status;
        }
    }
    webdav_prop_copy_uri(pconf, &mut (*src).rel_path, &mut (*dst).rel_path);
    let src_path_used: uint32_t = (*src).path.used;
    let src_rel_path_used: uint32_t = (*src).rel_path.used;
    let dst_path_used: uint32_t = (*dst).path.used;
    let dst_rel_path_used: uint32_t = (*dst).rel_path.used;
    dfd = fdevent_open_dirname((*src).path.ptr, 0 as libc::c_int);
    let srcdir: *mut DIR = if dfd >= 0 as libc::c_int {
        fdopendir(dfd)
    } else {
        0 as *mut DIR
    };
    if srcdir.is_null() {
        if dfd >= 0 as libc::c_int {
            close(dfd);
        }
        webdav_xml_response_status(r, &mut (*src).rel_path, 403 as libc::c_int);
        return 403 as libc::c_int;
    }
    let mut d_type: mode_t = 0;
    let mut multi_status: libc::c_int = 0 as libc::c_int;
    let mut de: *mut dirent = 0 as *mut dirent;
    loop {
        de = readdir(srcdir);
        if de.is_null() {
            break;
        }
        if (*de).d_name[0 as libc::c_int as usize] as libc::c_int == '.' as i32
            && ((*de).d_name[1 as libc::c_int as usize] as libc::c_int == '\u{0}' as i32
                || (*de).d_name[1 as libc::c_int as usize] as libc::c_int == '.' as i32
                    && (*de).d_name[2 as libc::c_int as usize] as libc::c_int
                        == '\u{0}' as i32)
        {
            continue;
        }
        if (*de).d_type as libc::c_int != DT_UNKNOWN as libc::c_int {
            d_type = (((*de).d_type as libc::c_int) << 12 as libc::c_int) as mode_t;
        } else {
            if 0 as libc::c_int
                != fstatat(
                    dfd,
                    ((*de).d_name).as_mut_ptr(),
                    &mut st,
                    0x100 as libc::c_int,
                )
            {
                continue;
            }
            d_type = st.st_mode;
        }
        let len: uint32_t = strlen(((*de).d_name).as_mut_ptr()) as uint32_t;
        if flags & 0x1 as libc::c_int != 0 {
            webdav_str_len_to_lower(((*de).d_name).as_mut_ptr(), len);
        }
        buffer_append_string_len(
            &mut (*src).path,
            ((*de).d_name).as_mut_ptr(),
            len as size_t,
        );
        buffer_append_string_len(
            &mut (*dst).path,
            ((*de).d_name).as_mut_ptr(),
            len as size_t,
        );
        buffer_append_string_len(
            &mut (*src).rel_path,
            ((*de).d_name).as_mut_ptr(),
            len as size_t,
        );
        buffer_append_string_len(
            &mut (*dst).rel_path,
            ((*de).d_name).as_mut_ptr(),
            len as size_t,
        );
        if d_type & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint
        {
            buffer_append_string_len(
                &mut (*src).path,
                b"/\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            buffer_append_string_len(
                &mut (*dst).path,
                b"/\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            buffer_append_string_len(
                &mut (*src).rel_path,
                b"/\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            buffer_append_string_len(
                &mut (*dst).rel_path,
                b"/\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            status = webdav_copymove_dir(pconf, src, dst, r, flags);
            if 0 as libc::c_int != status {
                multi_status = 1 as libc::c_int;
            }
        } else if d_type & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint
            {
            status = webdav_copymove_file(pconf, src, dst, &mut flags);
            if 0 as libc::c_int != status {
                webdav_xml_response_status(r, &mut (*src).rel_path, status);
            }
        } else {
            status = 0 as libc::c_int;
        }
        (*src).path.used = src_path_used;
        *((*src).path.ptr)
            .offset(
                ((*src).path.used).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as isize,
            ) = '\u{0}' as i32 as libc::c_char;
        (*src).rel_path.used = src_rel_path_used;
        *((*src).rel_path.ptr)
            .offset(
                ((*src).rel_path.used).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as isize,
            ) = '\u{0}' as i32 as libc::c_char;
        (*dst).path.used = dst_path_used;
        *((*dst).path.ptr)
            .offset(
                ((*dst).path.used).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as isize,
            ) = '\u{0}' as i32 as libc::c_char;
        (*dst).rel_path.used = dst_rel_path_used;
        *((*dst).rel_path.ptr)
            .offset(
                ((*dst).rel_path.used).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as isize,
            ) = '\u{0}' as i32 as libc::c_char;
        if !(507 as libc::c_int == status) {
            continue;
        }
        multi_status = 507 as libc::c_int;
        break;
    }
    closedir(srcdir);
    if 0 as libc::c_int == multi_status {
        if flags & (0x4 as libc::c_int | 0x10 as libc::c_int) != 0 {
            status = webdav_delete_dir(pconf, src, r, flags);
            if 0 as libc::c_int != status {
                webdav_xml_response_status(r, &mut (*src).rel_path, status);
                multi_status = 1 as libc::c_int;
            }
        }
    }
    return multi_status;
}
unsafe extern "C" fn webdav_propfind_live_props(
    pb: *const webdav_propfind_bufs,
    pnum: webdav_live_props_e,
) -> libc::c_int {
    let b: *mut buffer = (*pb).b_200;
    let mut current_block_43: u64;
    match pnum as libc::c_int {
        0 => {
            current_block_43 = 14823300835005246908;
        }
        1 => {
            current_block_43 = 14823300835005246908;
        }
        2 => {
            current_block_43 = 9812540127707587631;
        }
        3 => {
            current_block_43 = 16125801332347828972;
        }
        4 => {
            current_block_43 = 18156650521570657681;
        }
        5 => {
            current_block_43 = 9138990599349761258;
        }
        _ => {
            current_block_43 = 9106417282296633740;
        }
    }
    match current_block_43 {
        14823300835005246908 => {
            buffer_append_string_len(
                b,
                b"<D:getcontentlength>\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            buffer_append_int(b, (*pb).st.st_size);
            buffer_append_string_len(
                b,
                b"</D:getcontentlength>\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            if pnum as libc::c_int != WEBDAV_PROP_ALL as libc::c_int {
                return 0 as libc::c_int;
            }
            current_block_43 = 9812540127707587631;
        }
        _ => {}
    }
    match current_block_43 {
        9812540127707587631 => {
            if (*pb).st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint
            {
                buffer_append_string_len(
                    b,
                    b"<D:getcontenttype>httpd/unix-directory</D:getcontenttype>\0"
                        as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 58]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
            } else {
                let mtypes: *const array = (*(*pb).r).conf.mimetypes;
                let mut ct: *const buffer = stat_cache_mimetype_by_ext(
                    mtypes,
                    (*(*pb).dst).path.ptr,
                    buffer_clen(&mut (*(*pb).dst).path),
                );
                if !ct.is_null() {
                    buffer_append_str3(
                        b,
                        b"<D:getcontenttype>\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        (*ct).ptr,
                        buffer_clen(ct) as size_t,
                        b"</D:getcontenttype>\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    );
                } else if pnum as libc::c_int != WEBDAV_PROP_ALL as libc::c_int {
                    return -(1 as libc::c_int)
                }
            }
            if pnum as libc::c_int != WEBDAV_PROP_ALL as libc::c_int {
                return 0 as libc::c_int;
            }
            current_block_43 = 16125801332347828972;
        }
        _ => {}
    }
    match current_block_43 {
        16125801332347828972 => {
            if 0 as libc::c_int != (*(*pb).r).conf.etag_flags as libc::c_int {
                let etagb: *mut buffer = (*(*pb).r).tmp_buf;
                http_etag_create(
                    etagb,
                    &(*pb).st,
                    (*(*pb).r).conf.etag_flags as libc::c_int,
                );
                buffer_append_str3(
                    b,
                    b"<D:getetag>\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    (*etagb).ptr,
                    buffer_clen(etagb) as size_t,
                    b"</D:getetag>\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
            } else if pnum as libc::c_int != WEBDAV_PROP_ALL as libc::c_int {
                return -(1 as libc::c_int)
            }
            if pnum as libc::c_int != WEBDAV_PROP_ALL as libc::c_int {
                return 0 as libc::c_int;
            }
            current_block_43 = 18156650521570657681;
        }
        _ => {}
    }
    match current_block_43 {
        18156650521570657681 => {
            buffer_append_string_len(
                b,
                b"<D:getlastmodified ns0:dt=\"dateTime.rfc1123\">\0" as *const u8
                    as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            if http_date_time_to_str(
                buffer_extend(b, (30 as libc::c_int - 1 as libc::c_int) as size_t),
                30 as libc::c_int as size_t,
                (*pb).st.st_mtim.tv_sec,
            ) == 0
            {
                buffer_truncate(
                    b,
                    ((*b).used).wrapping_sub(30 as libc::c_int as libc::c_uint),
                );
            }
            buffer_append_string_len(
                b,
                b"</D:getlastmodified>\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            if pnum as libc::c_int != WEBDAV_PROP_ALL as libc::c_int {
                return 0 as libc::c_int;
            }
            current_block_43 = 9138990599349761258;
        }
        _ => {}
    }
    match current_block_43 {
        9138990599349761258 => {
            if (*pb).st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint
            {
                buffer_append_string_len(
                    b,
                    b"<D:resourcetype><D:collection/></D:resourcetype>\0" as *const u8
                        as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 49]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
            } else {
                buffer_append_string_len(
                    b,
                    b"<D:resourcetype/>\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
            }
            if pnum as libc::c_int != WEBDAV_PROP_ALL as libc::c_int {
                return 0 as libc::c_int;
            }
        }
        _ => {}
    }
    if pnum as libc::c_int == WEBDAV_PROP_ALL as libc::c_int {} else {
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn webdav_propfind_resource_props(pb: *const webdav_propfind_bufs) {
    let props: *const webdav_property_names = &(*pb).proplist;
    if (*props).used != 0 {
        let mut prop: *const webdav_property_name = (*props).ptr;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*props).used {
            if !(if ((*prop).name).is_null() {
                (0 as libc::c_int
                    == webdav_propfind_live_props(
                        pb,
                        (*prop).namelen as webdav_live_props_e,
                    )) as libc::c_int
            } else {
                (0 as libc::c_int
                    == webdav_prop_select_prop(
                        (*pb).pconf,
                        &mut (*(*pb).dst).rel_path,
                        prop,
                        (*pb).b_200,
                    )) as libc::c_int
            } != 0)
            {
                if !((*prop).name).is_null() {
                    webdav_xml_prop(
                        (*pb).b_404,
                        prop,
                        0 as *const libc::c_char,
                        0 as libc::c_int as uint32_t,
                    );
                }
            }
            i += 1;
            prop = prop.offset(1);
        }
    }
    if (*pb).allprop != 0 {
        webdav_propfind_live_props(pb, WEBDAV_PROP_ALL);
        webdav_prop_select_props((*pb).pconf, &mut (*(*pb).dst).rel_path, (*pb).b_200);
    }
}
unsafe extern "C" fn webdav_propfind_resource_propnames(
    pb: *const webdav_propfind_bufs,
) {
    static mut live_propnames: [libc::c_char; 85] = unsafe {
        *::std::mem::transmute::<
            &[u8; 85],
            &[libc::c_char; 85],
        >(
            b"<getcontentlength/>\n<getcontenttype/>\n<getetag/>\n<getlastmodified/>\n<resourcetype/>\n\0",
        )
    };
    buffer_append_string_len(
        (*pb).b_200,
        live_propnames.as_ptr(),
        (::std::mem::size_of::<[libc::c_char; 85]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    webdav_prop_select_propnames((*pb).pconf, &mut (*(*pb).dst).rel_path, (*pb).b_200);
}
#[cold]
unsafe extern "C" fn webdav_propfind_resource_403(pb: *const webdav_propfind_bufs) {
    let b: *mut buffer = (*pb).b;
    buffer_append_string_len(
        b,
        b"<D:response>\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    webdav_xml_href(b, &mut (*(*pb).dst).rel_path);
    buffer_append_string_len(
        b,
        b"<D:propstat>\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    webdav_xml_status(b, 403 as libc::c_int);
    buffer_append_string_len(
        b,
        b"</D:propstat>\n</D:response>\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    webdav_double_buffer((*pb).r, b);
}
unsafe extern "C" fn webdav_propfind_resource(pb: *const webdav_propfind_bufs) {
    buffer_clear((*pb).b_200);
    buffer_clear((*pb).b_404);
    if (*pb).propname == 0 {
        webdav_propfind_resource_props(pb);
    } else {
        webdav_propfind_resource_propnames(pb);
    }
    let b: *mut buffer = (*pb).b;
    let b_200: *mut buffer = (*pb).b_200;
    let b_404: *mut buffer = (*pb).b_404;
    if ((*b).size).wrapping_sub((*b).used)
        < ((*b_200).used)
            .wrapping_add((*b_404).used)
            .wrapping_add(1024 as libc::c_int as libc::c_uint)
    {
        let mut sz: size_t = ((*b).used)
            .wrapping_add(8192 as libc::c_int as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_add((*b_200).used)
            .wrapping_add((*b_404).used)
            .wrapping_add(1024 as libc::c_int as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t;
        buffer_string_prepare_append(
            b,
            sz & (8192 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
        );
    }
    buffer_append_string_len(
        b,
        b"<D:response>\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    webdav_xml_href(b, &mut (*(*pb).dst).rel_path);
    if buffer_is_blank(b_200) == 0 {
        webdav_xml_propstat(b, b_200, 200 as libc::c_int);
    }
    if buffer_is_blank(b_404) == 0 {
        webdav_xml_propstat(b, b_404, 404 as libc::c_int);
    }
    buffer_append_string_len(
        b,
        b"</D:response>\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    webdav_double_buffer((*pb).r, b);
}
unsafe extern "C" fn webdav_propfind_dir(pb: *mut webdav_propfind_bufs) {
    (*pb).recursed += 1;
    if (*pb).recursed > 100 as libc::c_int {
        return;
    }
    let dst: *mut physical_st = (*pb).dst;
    let dfd: libc::c_int = fdevent_open_dirname((*dst).path.ptr, 0 as libc::c_int);
    let dir: *mut DIR = if dfd >= 0 as libc::c_int {
        fdopendir(dfd)
    } else {
        0 as *mut DIR
    };
    if dir.is_null() {
        let mut errnum: libc::c_int = *__errno_location();
        if dfd >= 0 as libc::c_int {
            close(dfd);
        }
        if errnum != 2 as libc::c_int {
            webdav_propfind_resource_403(pb);
        }
        return;
    }
    webdav_propfind_resource(pb);
    if (*pb).lockdiscovery > 0 as libc::c_int {
        (*pb).lockdiscovery = -(*pb).lockdiscovery;
    }
    let dst_path_used: uint32_t = (*dst).path.used;
    let dst_rel_path_used: uint32_t = (*dst).rel_path.used;
    let flags: libc::c_int = if (*(*pb).r).conf.force_lowercase_filenames as libc::c_int
        != 0
    {
        0x1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut de: *mut dirent = 0 as *mut dirent;
    loop {
        de = readdir(dir);
        if de.is_null() {
            break;
        }
        if (*de).d_name[0 as libc::c_int as usize] as libc::c_int == '.' as i32
            && ((*de).d_name[1 as libc::c_int as usize] as libc::c_int == '\u{0}' as i32
                || (*de).d_name[1 as libc::c_int as usize] as libc::c_int == '.' as i32
                    && (*de).d_name[2 as libc::c_int as usize] as libc::c_int
                        == '\u{0}' as i32)
        {
            continue;
        }
        if 0 as libc::c_int
            != fstatat(dfd, ((*de).d_name).as_mut_ptr(), &mut (*pb).st, (*pb).atflags)
        {
            continue;
        }
        let len: uint32_t = strlen(((*de).d_name).as_mut_ptr()) as uint32_t;
        if flags & 0x1 as libc::c_int != 0 {
            webdav_str_len_to_lower(((*de).d_name).as_mut_ptr(), len);
        }
        buffer_append_string_len(
            &mut (*dst).path,
            ((*de).d_name).as_mut_ptr(),
            len as size_t,
        );
        buffer_append_string_len(
            &mut (*dst).rel_path,
            ((*de).d_name).as_mut_ptr(),
            len as size_t,
        );
        if (*pb).st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint
        {
            buffer_append_string_len(
                &mut (*dst).path,
                b"/\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            buffer_append_string_len(
                &mut (*dst).rel_path,
                b"/\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
        if (*pb).st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint
            && -(1 as libc::c_int) == (*pb).depth
        {
            webdav_propfind_dir(pb);
        } else {
            webdav_propfind_resource(pb);
        }
        (*dst).path.used = dst_path_used;
        *((*dst).path.ptr)
            .offset(
                ((*dst).path.used).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as isize,
            ) = '\u{0}' as i32 as libc::c_char;
        (*dst).rel_path.used = dst_rel_path_used;
        *((*dst).rel_path.ptr)
            .offset(
                ((*dst).rel_path.used).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as isize,
            ) = '\u{0}' as i32 as libc::c_char;
    }
    closedir(dir);
}
unsafe extern "C" fn webdav_open_chunk_file_rd(c: *mut chunk) -> libc::c_int {
    if (*c).file.fd < 0 as libc::c_int {
        (*c)
            .file
            .fd = fdevent_open_cloexec(
            (*(*c).mem).ptr,
            1 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int as mode_t,
        );
    }
    return (*c).file.fd;
}
unsafe extern "C" fn webdav_mmap_file_rd(
    addr: *mut *mut libc::c_void,
    length: size_t,
    fd: libc::c_int,
    offset: off_t,
) -> libc::c_int {
    if 0 as libc::c_int as libc::c_ulong == length {
        *addr = 0 as *mut libc::c_void;
        return 0 as libc::c_int;
    }
    *addr = mmap(
        0 as *mut libc::c_void,
        length,
        0x1 as libc::c_int,
        0x1 as libc::c_int,
        fd,
        offset,
    );
    if *addr == -(1 as libc::c_int) as *mut libc::c_void
        && *__errno_location() == 22 as libc::c_int
    {
        *addr = mmap(
            0 as *mut libc::c_void,
            length,
            0x1 as libc::c_int,
            0x2 as libc::c_int,
            fd,
            offset,
        );
    }
    return if *addr != -(1 as libc::c_int) as *mut libc::c_void {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
unsafe extern "C" fn webdav_mmap_file_chunk(c: *mut chunk) -> *mut libc::c_char {
    if -(1 as libc::c_int) as *mut libc::c_void
        != (*c).file.mmap.start as *mut libc::c_void
    {
        return ((*c).file.mmap.start)
            .offset((*c).offset as isize)
            .offset(-((*c).file.mmap.offset as isize));
    }
    if webdav_open_chunk_file_rd(c) < 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    webdav_mmap_file_rd(
        &mut (*c).file.mmap.start as *mut *mut libc::c_char as *mut *mut libc::c_void,
        (*c).file.length as size_t,
        (*c).file.fd,
        0 as libc::c_int as off_t,
    );
    if -(1 as libc::c_int) as *mut libc::c_void
        == (*c).file.mmap.start as *mut libc::c_void
    {
        return 0 as *mut libc::c_char;
    }
    close((*c).file.fd);
    (*c).file.fd = -(1 as libc::c_int);
    (*c).file.mmap.length = (*c).file.length as size_t;
    return ((*c).file.mmap.start)
        .offset((*c).offset as isize)
        .offset(-((*c).file.mmap.offset as isize));
}
unsafe extern "C" fn mod_webdav_propfind(
    r: *mut request_st,
    pconf: *const plugin_config,
) -> handler_t {
    (*r).reqbody_length != 0;
    let mut pb: webdav_propfind_bufs = webdav_propfind_bufs {
        r: 0 as *mut request_st,
        pconf: 0 as *const plugin_config,
        dst: 0 as *mut physical_st,
        b: 0 as *mut buffer,
        b_200: 0 as *mut buffer,
        b_404: 0 as *mut buffer,
        proplist: webdav_property_names {
            ptr: 0 as *mut webdav_property_name,
            used: 0,
            size: 0,
        },
        allprop: 0,
        propname: 0,
        lockdiscovery: 0,
        depth: 0,
        recursed: 0,
        atflags: 0,
        st: stat {
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
        },
    };
    pb.allprop = 0 as libc::c_int;
    pb.propname = 0 as libc::c_int;
    pb.lockdiscovery = 0 as libc::c_int;
    pb.recursed = 0 as libc::c_int;
    pb.depth = webdav_parse_Depth(r);
    if -(1 as libc::c_int) == pb.depth
        && (*pconf).opts as libc::c_int
            & MOD_WEBDAV_PROPFIND_DEPTH_INFINITY as libc::c_int == 0
    {
        webdav_xml_doc_error_propfind_finite_depth(r);
        return HANDLER_FINISHED;
    }
    pb
        .atflags = if (*pconf).opts as libc::c_int
        & MOD_WEBDAV_UNSAFE_PROPFIND_FOLLOW_SYMLINK as libc::c_int != 0
        && (*pconf).is_readonly as libc::c_int != 0
    {
        0 as libc::c_int
    } else {
        0x100 as libc::c_int
    };
    if if pb.atflags == 0x100 as libc::c_int {
        (0 as libc::c_int != lstat((*r).physical.path.ptr, &mut pb.st)) as libc::c_int
    } else {
        (0 as libc::c_int != stat((*r).physical.path.ptr, &mut pb.st)) as libc::c_int
    } != 0
    {
        http_status_set_error(
            r,
            if *__errno_location() == 2 as libc::c_int {
                404 as libc::c_int
            } else {
                403 as libc::c_int
            },
        );
        return HANDLER_FINISHED;
    } else {
        if pb.st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint
        {
            if buffer_has_pathsep_suffix(&mut (*r).physical.path) == 0 {
                let mut vb: *const buffer = http_header_request_get(
                    r,
                    HTTP_HEADER_USER_AGENT,
                    b"User-Agent\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                );
                if !vb.is_null()
                    && 0 as libc::c_int
                        == strncmp(
                            (*vb).ptr,
                            b"Microsoft-WebDAV-MiniRedir/\0" as *const u8
                                as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 28]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        )
                {
                    http_response_redirect_to_directory(r, 308 as libc::c_int);
                    return HANDLER_FINISHED;
                }
                if !vb.is_null()
                    && 0 as libc::c_int
                        == strncmp(
                            (*vb).ptr,
                            b"gvfs/\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        )
                {
                    http_response_redirect_to_directory(r, 308 as libc::c_int);
                    return HANDLER_FINISHED;
                }
                if http_response_redirect_to_directory(r, 0 as libc::c_int) == 0 {
                    return HANDLER_FINISHED;
                }
                buffer_append_string_len(
                    &mut (*r).physical.path,
                    b"/\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
                buffer_append_string_len(
                    &mut (*r).physical.rel_path,
                    b"/\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
            }
        } else if buffer_has_pathsep_suffix(&mut (*r).physical.path) != 0 {
            http_status_set_error(r, 403 as libc::c_int);
            return HANDLER_FINISHED;
        } else {
            pb.depth = 0 as libc::c_int;
        }
    }
    pb.proplist.ptr = 0 as *mut webdav_property_name;
    pb.proplist.used = 0 as libc::c_int;
    pb.proplist.size = 0 as libc::c_int;
    if (pb.proplist.ptr).is_null() && pb.propname == 0 {
        pb.lockdiscovery = 1 as libc::c_int;
        pb.allprop = pb.lockdiscovery;
    }
    pb.r = r;
    pb.pconf = pconf;
    pb.dst = &mut (*r).physical;
    pb.b = chunk_buffer_acquire();
    pb.b_200 = chunk_buffer_acquire();
    pb.b_404 = chunk_buffer_acquire();
    chunk_buffer_prepare_append(pb.b, 8192 as libc::c_int as size_t);
    webdav_xml_doctype(pb.b, r);
    buffer_append_string_len(
        pb.b,
        b"<D:multistatus xmlns:D=\"DAV:\" xmlns:ns0=\"urn:uuid:c2f41010-65b3-11d1-a29f-00aa00c14882/\">\n\0"
            as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 91]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    if 0 as libc::c_int != pb.depth {
        webdav_propfind_dir(&mut pb);
    } else {
        webdav_propfind_resource(&mut pb);
    }
    buffer_append_string_len(
        pb.b,
        b"</D:multistatus>\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    http_chunk_append_buffer(r, pb.b);
    chunk_buffer_release(pb.b);
    (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
    (*r).handler_module = 0 as *const plugin;
    (*r).http_status = 207 as libc::c_int;
    chunk_buffer_release(pb.b_404);
    chunk_buffer_release(pb.b_200);
    if (*pconf).log_xml != 0 {
        webdav_xml_log_response(r);
    }
    return HANDLER_FINISHED;
}
unsafe extern "C" fn mod_webdav_mkcol(
    r: *mut request_st,
    pconf: *const plugin_config,
) -> handler_t {
    let status: libc::c_int = webdav_mkdir(
        pconf,
        &mut (*r).physical,
        -(1 as libc::c_int),
    );
    if 0 as libc::c_int == status {
        (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
        (*r).handler_module = 0 as *const plugin;
        (*r).http_status = 201 as libc::c_int;
    } else {
        http_status_set_error(r, status);
    }
    return HANDLER_FINISHED;
}
unsafe extern "C" fn mod_webdav_delete(
    r: *mut request_st,
    pconf: *const plugin_config,
) -> handler_t {
    if !(strchr((*r).target_orig.ptr, '#' as i32)).is_null() {
        http_status_set_error(r, 403 as libc::c_int);
        return HANDLER_FINISHED;
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
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if -(1 as libc::c_int) == lstat((*r).physical.path.ptr, &mut st) {
        http_status_set_error(
            r,
            if *__errno_location() == 2 as libc::c_int {
                404 as libc::c_int
            } else {
                403 as libc::c_int
            },
        );
        return HANDLER_FINISHED;
    }
    if 0 as libc::c_int != webdav_if_match_or_unmodified_since(r, &mut st) {
        http_status_set_error(r, 412 as libc::c_int);
        return HANDLER_FINISHED;
    }
    if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint
    {
        if buffer_has_pathsep_suffix(&mut (*r).physical.path) == 0 {
            buffer_append_string_len(
                &mut (*r).physical.path,
                b"/\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            buffer_append_string_len(
                &mut (*r).physical.rel_path,
                b"/\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
        if -(1 as libc::c_int) != webdav_parse_Depth(r) {
            http_status_set_error(r, 400 as libc::c_int);
            return HANDLER_FINISHED;
        }
        let flags: libc::c_int = if (*r).conf.force_lowercase_filenames as libc::c_int
            != 0
        {
            0x1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        if 0 as libc::c_int == webdav_delete_dir(pconf, &mut (*r).physical, r, flags) {
            webdav_lock_delete_uri_col(pconf, &mut (*r).physical.rel_path);
            (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
            (*r).handler_module = 0 as *const plugin;
            (*r).http_status = 204 as libc::c_int;
        } else {
            webdav_xml_doc_multistatus(r, pconf);
        }
        stat_cache_delete_dir(
            (*r).physical.path.ptr,
            buffer_clen(&mut (*r).physical.path),
        );
    } else if buffer_has_pathsep_suffix(&mut (*r).physical.path) != 0 {
        http_status_set_error(r, 403 as libc::c_int);
    } else {
        let status: libc::c_int = webdav_delete_file(pconf, &mut (*r).physical);
        if 0 as libc::c_int == status {
            webdav_lock_delete_uri(pconf, &mut (*r).physical.rel_path);
            (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
            (*r).handler_module = 0 as *const plugin;
            (*r).http_status = 204 as libc::c_int;
        } else {
            http_status_set_error(r, status);
        }
    }
    return HANDLER_FINISHED;
}
#[inline(never)]
unsafe extern "C" fn mod_webdav_write_cq(
    r: *mut request_st,
    cq: *mut chunkqueue,
    fd: libc::c_int,
) -> libc::c_int {
    chunkqueue_remove_finished_chunks(cq);
    while chunkqueue_is_empty(cq) == 0 {
        let mut wr: ssize_t = chunkqueue_write_chunk(fd, cq, (*r).conf.errh);
        if wr > 0 as libc::c_int as libc::c_long {
            chunkqueue_mark_written(cq, wr);
        } else if wr < 0 as libc::c_int as libc::c_long {
            http_status_set_error(
                r,
                if *__errno_location() == 28 as libc::c_int {
                    507 as libc::c_int
                } else {
                    403 as libc::c_int
                },
            );
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn mod_webdav_write_single_file_chunk(
    r: *mut request_st,
    cq: *mut chunkqueue,
) -> libc::c_int {
    let c: *mut chunk = (*cq).first;
    (*cq).first = (*c).next;
    let len: off_t = chunkqueue_length(cq);
    let bytes_out: off_t = (*cq).bytes_out;
    if mod_webdav_write_cq(r, cq, (*c).file.fd) != 0 {
        (*cq).bytes_out = bytes_out;
        (*c).file.length = len;
        (*c).next = 0 as *mut chunk;
        (*cq).last = c;
        (*cq).first = (*cq).last;
        return 1 as libc::c_int;
    } else {
        (*c).next = (*cq).first;
        (*cq).first = c;
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn mod_webdav_put_0(
    r: *mut request_st,
    pconf: *const plugin_config,
) -> handler_t {
    if 0 as libc::c_int != webdav_if_match_or_unmodified_since(r, 0 as *mut stat) {
        http_status_set_error(r, 412 as libc::c_int);
        return HANDLER_FINISHED;
    }
    let mut fd: libc::c_int = 0;
    fd = fdevent_open_cloexec(
        (*r).physical.path.ptr,
        0 as libc::c_int,
        0o1 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int
            | 0o1000 as libc::c_int,
        (0o400 as libc::c_int | 0o200 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as mode_t,
    );
    if fd >= 0 as libc::c_int {
        if 0 as libc::c_int != (*r).conf.etag_flags as libc::c_int {
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
            if 0 as libc::c_int == fstat(fd, &mut st) {
                webdav_response_etag(r, &mut st);
            }
        }
        close(fd);
        webdav_parent_modified(&mut (*r).physical.path);
        (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
        (*r).handler_module = 0 as *const plugin;
        (*r).http_status = 201 as libc::c_int;
        return HANDLER_FINISHED;
    } else {
        if *__errno_location() == 21 as libc::c_int {
            http_status_set_error(r, 405 as libc::c_int);
            return HANDLER_FINISHED;
        }
    }
    if *__errno_location() == 40 as libc::c_int {
        webdav_delete_file(pconf, &mut (*r).physical);
    }
    fd = fdevent_open_cloexec(
        (*r).physical.path.ptr,
        0 as libc::c_int,
        0o1 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
        (0o400 as libc::c_int | 0o200 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as mode_t,
    );
    if fd >= 0 as libc::c_int {
        close(fd);
        (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
        (*r).handler_module = 0 as *const plugin;
        (*r).http_status = 204 as libc::c_int;
        return HANDLER_FINISHED;
    }
    http_status_set_error(r, 500 as libc::c_int);
    return HANDLER_FINISHED;
}
unsafe extern "C" fn mod_webdav_put_prep(
    r: *mut request_st,
    pconf: *const plugin_config,
) -> handler_t {
    if (*r).rqst_htags & (1 as libc::c_ulong) << HTTP_HEADER_CONTENT_RANGE as libc::c_int
        != 0
    {
        if (*pconf).opts as libc::c_int
            & MOD_WEBDAV_UNSAFE_PARTIAL_PUT_COMPAT as libc::c_int != 0
        {
            return HANDLER_GO_ON;
        }
        http_status_set_error(r, 400 as libc::c_int);
        return HANDLER_FINISHED;
    }
    let used: uint32_t = (*r).physical.path.used;
    let mut slash: *mut libc::c_char = ((*r).physical.path.ptr)
        .offset(used as isize)
        .offset(-(2 as libc::c_int as isize));
    if *slash as libc::c_int == '/' as i32 {
        http_status_set_error(r, 400 as libc::c_int);
        return HANDLER_FINISHED;
    }
    if 0 as libc::c_int as libc::c_long == (*r).reqbody_length {
        return mod_webdav_put_0(r, pconf);
    }
    let mut fd: libc::c_int = 0;
    let mut len: size_t = buffer_clen(&mut (*r).physical.path) as size_t;
    slash = memrchr((*r).physical.path.ptr as *const libc::c_void, '/' as i32, len)
        as *mut libc::c_char;
    if slash == (*r).physical.path.ptr {
        slash = 0 as *mut libc::c_char;
    }
    if !slash.is_null() {
        *slash = '\u{0}' as i32 as libc::c_char;
    }
    fd = fdevent_open_cloexec(
        (*r).physical.path.ptr,
        1 as libc::c_int,
        0o2 as libc::c_int | (0o20000000 as libc::c_int | 0o200000 as libc::c_int)
            | 0o2000 as libc::c_int,
        (0o400 as libc::c_int | 0o200 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as mode_t,
    );
    if !slash.is_null() {
        *slash = '/' as i32 as libc::c_char;
    }
    if fd < 0 as libc::c_int {
        buffer_append_string_len(
            &mut (*r).physical.path,
            b"-XXXXXX\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        fd = fdevent_mkostemp((*r).physical.path.ptr, 0 as libc::c_int);
        if fd >= 0 as libc::c_int {
            unlink((*r).physical.path.ptr);
        }
        buffer_truncate(&mut (*r).physical.path, len as uint32_t);
    }
    if fd < 0 as libc::c_int {
        http_status_set_error(r, 500 as libc::c_int);
        return HANDLER_FINISHED;
    }
    let cq: *mut chunkqueue = &mut (*r).reqbody_queue;
    let mut cqlen: off_t = chunkqueue_length(cq);
    if mod_webdav_write_cq(r, cq, fd) == 0 {
        close(fd);
        return HANDLER_FINISHED;
    }
    chunkqueue_reset(cq);
    if 0 as libc::c_int as libc::c_long != cqlen {
        chunkqueue_append_file_fd(
            cq,
            &mut (*r).physical.path,
            fd,
            0 as libc::c_int as off_t,
            cqlen,
        );
    } else {
        chunkqueue_append_file_fd(
            cq,
            &mut (*r).physical.path,
            fd,
            0 as libc::c_int as off_t,
            1 as libc::c_int as off_t,
        );
        (*(*cq).last).file.length = 0 as libc::c_int as off_t;
        (*cq).bytes_in = 0 as libc::c_int as off_t;
    }
    buffer_clear((*(*cq).last).mem);
    (*cq).upload_temp_file_size = 9223372036854775807 as libc::c_long;
    (*(*cq).last).file.is_temp = 1 as libc::c_int;
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_webdav_put_linkat_rename(
    r: *mut request_st,
    pathtemp: *const libc::c_char,
) -> libc::c_int {
    if has_proc_self_fd == 0 {
        return 0 as libc::c_int;
    }
    let cq: *mut chunkqueue = &mut (*r).reqbody_queue;
    let mut c: *mut chunk = (*cq).first;
    let mut pathproc: [libc::c_char; 32] = *::std::mem::transmute::<
        &[u8; 32],
        &mut [libc::c_char; 32],
    >(b"/proc/self/fd/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    let mut plen: size_t = li_itostrn(
        pathproc
            .as_mut_ptr()
            .offset(
                ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as isize,
            )
            .offset(-(1 as libc::c_int as isize)),
        (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
            .wrapping_sub(
                (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ),
        (*c).file.fd as intmax_t,
    );
    pathproc[(::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(plen) as usize] = '\u{0}' as i32 as libc::c_char;
    if 0 as libc::c_int
        == linkat(
            -(100 as libc::c_int),
            pathproc.as_mut_ptr(),
            -(100 as libc::c_int),
            pathtemp,
            0x400 as libc::c_int,
        )
    {
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
        if 0 as libc::c_int
            == renameat2(
                -(100 as libc::c_int),
                pathtemp,
                -(100 as libc::c_int),
                (*r).physical.path.ptr,
                ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint,
            )
        {
            (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
            (*r).handler_module = 0 as *const plugin;
            (*r).http_status = 201 as libc::c_int;
        } else if 0 as libc::c_int == rename(pathtemp, (*r).physical.path.ptr) {
            (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
            (*r).handler_module = 0 as *const plugin;
            (*r).http_status = 204 as libc::c_int;
        } else {
            if *__errno_location() == 21 as libc::c_int {
                http_status_set_error(r, 405 as libc::c_int);
            } else {
                http_status_set_error(r, 403 as libc::c_int);
            }
            unlink(pathtemp);
        }
        if 0 as libc::c_int != (*r).conf.etag_flags as libc::c_int
            && (*r).http_status < 300 as libc::c_int
        {
            if 0 as libc::c_int == fstat((*c).file.fd, &mut st) {
                webdav_response_etag(r, &mut st);
            }
        }
        chunkqueue_mark_written(cq, (*c).file.length);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[cold]
unsafe extern "C" fn mod_webdav_put_deprecated_unsafe_partial_put_compat(
    r: *mut request_st,
    h: *const buffer,
) -> handler_t {
    let mut num: *const libc::c_char = (*h).ptr;
    let mut offset: off_t = 0;
    let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 as libc::c_int
        != strncmp(
            num,
            b"bytes \0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        )
    {
        http_status_set_error(r, 501 as libc::c_int);
        return HANDLER_FINISHED;
    }
    num = num
        .offset(
            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        );
    offset = strtoll(num, &mut err, 10 as libc::c_int) as off_t;
    if num == err as *const libc::c_char || *err as libc::c_int != '-' as i32
        || offset < 0 as libc::c_int as libc::c_long
    {
        http_status_set_error(r, 501 as libc::c_int);
        return HANDLER_FINISHED;
    }
    let fd: libc::c_int = fdevent_open_cloexec(
        (*r).physical.path.ptr,
        0 as libc::c_int,
        0o1 as libc::c_int,
        (0o400 as libc::c_int | 0o200 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as mode_t,
    );
    if fd < 0 as libc::c_int {
        http_status_set_error(
            r,
            if *__errno_location() == 2 as libc::c_int {
                404 as libc::c_int
            } else {
                403 as libc::c_int
            },
        );
        return HANDLER_FINISHED;
    }
    let cq: *mut chunkqueue = &mut (*r).reqbody_queue;
    let mut c: *mut chunk = (*cq).first;
    let mut cqlen: off_t = chunkqueue_length(cq);
    if (*c).type_0 as libc::c_uint == FILE_CHUNK as libc::c_int as libc::c_uint
        && ((*c).next).is_null() && (*c).file.fd >= 0 as libc::c_int
    {
        let mut zoff: loff_t = 0 as libc::c_int as loff_t;
        let mut ooff: loff_t = offset;
        let mut wr: ssize_t = 0;
        loop {
            wr = copy_file_range(
                (*c).file.fd,
                &mut zoff,
                fd,
                &mut ooff,
                cqlen as size_t,
                0 as libc::c_int as libc::c_uint,
            );
            if !(wr > 0 as libc::c_int as libc::c_long
                && {
                    cqlen -= wr;
                    cqlen != 0
                })
            {
                break;
            }
        }
    }
    if 0 as libc::c_int as libc::c_long != cqlen {
        if -(1 as libc::c_int) as libc::c_long == lseek(fd, offset, 0 as libc::c_int) {
            close(fd);
            http_status_set_error(r, 500 as libc::c_int);
            return HANDLER_FINISHED;
        }
        mod_webdav_write_cq(r, &mut (*r).reqbody_queue, fd);
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
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if 0 as libc::c_int != (*r).conf.etag_flags as libc::c_int
        && !(0 as libc::c_int != (*r).http_status)
    {
        if 0 as libc::c_int != fstat(fd, &mut st) {
            (*r).conf.etag_flags = 0 as libc::c_int as libc::c_uchar;
        }
    }
    let wc: libc::c_int = close(fd);
    if 0 as libc::c_int != wc && !(0 as libc::c_int != (*r).http_status) {
        http_status_set_error(
            r,
            if *__errno_location() == 28 as libc::c_int {
                507 as libc::c_int
            } else {
                403 as libc::c_int
            },
        );
    }
    if !(0 as libc::c_int != (*r).http_status) {
        (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
        (*r).handler_module = 0 as *const plugin;
        (*r).http_status = 204 as libc::c_int;
        if 0 as libc::c_int != (*r).conf.etag_flags as libc::c_int {
            webdav_response_etag(r, &mut st);
        }
    }
    return HANDLER_FINISHED;
}
unsafe extern "C" fn mod_webdav_put(
    r: *mut request_st,
    pconf: *const plugin_config,
) -> handler_t {
    if (*r).state as libc::c_uint == CON_STATE_READ_POST as libc::c_int as libc::c_uint {
        let mut first_read: libc::c_int = chunkqueue_is_empty(&mut (*r).reqbody_queue);
        let mut rc: handler_t = ((*(*r).con).reqbody_read)
            .expect("non-null function pointer")(r);
        if rc as libc::c_uint != HANDLER_GO_ON as libc::c_int as libc::c_uint {
            if first_read != 0
                && rc as libc::c_uint
                    == HANDLER_WAIT_FOR_EVENT as libc::c_int as libc::c_uint
                && 0 as libc::c_int
                    != webdav_if_match_or_unmodified_since(r, 0 as *mut stat)
            {
                http_status_set_error(r, 412 as libc::c_int);
                return HANDLER_FINISHED;
            }
            return rc;
        }
    }
    if 0 as libc::c_int != webdav_if_match_or_unmodified_since(r, 0 as *mut stat) {
        http_status_set_error(r, 412 as libc::c_int);
        return HANDLER_FINISHED;
    }
    if (*pconf).opts as libc::c_int & MOD_WEBDAV_UNSAFE_PARTIAL_PUT_COMPAT as libc::c_int
        != 0
    {
        let h: *const buffer = http_header_request_get(
            r,
            HTTP_HEADER_CONTENT_RANGE,
            b"Content-Range\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        if !h.is_null() {
            return mod_webdav_put_deprecated_unsafe_partial_put_compat(r, h);
        }
    }
    let cq: *mut chunkqueue = &mut (*r).reqbody_queue;
    let mut c: *mut chunk = (*cq).first;
    let tmpb: *mut buffer = (*pconf).tmpb;
    buffer_clear(tmpb);
    buffer_append_str2(
        tmpb,
        (*r).physical.path.ptr,
        buffer_clen(&mut (*r).physical.path) as size_t,
        b".\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(tmpb, getpid() as libc::c_long);
    buffer_append_string_len(
        tmpb,
        b".\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    if (*c).type_0 as libc::c_uint == MEM_CHUNK as libc::c_int as libc::c_uint {
        buffer_append_uint_hex_lc(tmpb, pconf as uintptr_t);
    } else {
        buffer_append_int(tmpb, (*c).file.fd as libc::c_long);
    }
    buffer_append_string_len(
        tmpb,
        b"~\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    if buffer_clen(tmpb) >= 4096 as libc::c_int as libc::c_uint {
        http_status_set_error(r, 500 as libc::c_int);
        return HANDLER_FINISHED;
    }
    let mut pathtemp: *const libc::c_char = (*tmpb).ptr;
    if (*c).type_0 as libc::c_uint == FILE_CHUNK as libc::c_int as libc::c_uint {
        if !((*c).next).is_null() {
            if mod_webdav_write_single_file_chunk(r, cq) == 0 {
                return HANDLER_FINISHED;
            }
        }
        if mod_webdav_put_linkat_rename(r, pathtemp) != 0 {
            return HANDLER_FINISHED;
        }
    }
    let fd: libc::c_int = fdevent_open_cloexec(
        pathtemp,
        0 as libc::c_int,
        0o1 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int
            | 0o1000 as libc::c_int,
        (0o400 as libc::c_int | 0o200 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as mode_t,
    );
    if fd < 0 as libc::c_int {
        http_status_set_error(r, 500 as libc::c_int);
        return HANDLER_FINISHED;
    }
    mod_webdav_write_cq(r, cq, fd);
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
    if 0 as libc::c_int != (*r).conf.etag_flags as libc::c_int
        && !(0 as libc::c_int != (*r).http_status)
    {
        if 0 as libc::c_int != fstat(fd, &mut st) {
            (*r).conf.etag_flags = 0 as libc::c_int as libc::c_uchar;
        }
    }
    let wc: libc::c_int = close(fd);
    if 0 as libc::c_int != wc && !(0 as libc::c_int != (*r).http_status) {
        http_status_set_error(
            r,
            if *__errno_location() == 28 as libc::c_int {
                507 as libc::c_int
            } else {
                403 as libc::c_int
            },
        );
    }
    if !(0 as libc::c_int != (*r).http_status) {
        let mut ste: stat = stat {
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
        (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
        (*r).handler_module = 0 as *const plugin;
        (*r)
            .http_status = (if 0 as libc::c_int
            == lstat((*r).physical.path.ptr, &mut ste)
        {
            204 as libc::c_int
        } else {
            201 as libc::c_int
        });
        if 201 as libc::c_int == (*r).http_status {
            webdav_parent_modified(&mut (*r).physical.path);
        }
        if 0 as libc::c_int == rename(pathtemp, (*r).physical.path.ptr) {
            if 0 as libc::c_int != (*r).conf.etag_flags as libc::c_int {
                webdav_response_etag(r, &mut st);
            }
        } else {
            if *__errno_location() == 21 as libc::c_int {
                http_status_set_error(r, 405 as libc::c_int);
            } else {
                http_status_set_error(r, 500 as libc::c_int);
            }
            unlink(pathtemp);
        }
    } else {
        unlink(pathtemp);
    }
    return HANDLER_FINISHED;
}
unsafe extern "C" fn mod_webdav_copymove_b(
    r: *mut request_st,
    pconf: *const plugin_config,
    dst: *mut physical_st,
) -> handler_t {
    let dst_path: *mut buffer = &mut (*dst).path;
    let dst_rel_path: *mut buffer = &mut (*dst).rel_path;
    let mut flags: libc::c_int = 0x2 as libc::c_int
        | (if (*r).conf.force_lowercase_filenames as libc::c_int != 0 {
            0x1 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if (*r).http_method as libc::c_int == HTTP_METHOD_MOVE as libc::c_int {
            0x4 as libc::c_int
        } else {
            (if (*pconf).opts as libc::c_int
                & MOD_WEBDAV_UNSAFE_PARTIAL_PUT_COMPAT as libc::c_int != 0
            {
                0 as libc::c_int
            } else {
                0x8 as libc::c_int
            })
        });
    let h: *const buffer = http_header_request_get(
        r,
        HTTP_HEADER_OTHER,
        b"Overwrite\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if !h.is_null() {
        if (*h).used != 2 as libc::c_int as libc::c_uint
            || *((*h).ptr).offset(0 as libc::c_int as isize) as libc::c_int
                & 0xdf as libc::c_int != 'F' as i32
                && *((*h).ptr).offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xdf as libc::c_int != 'T' as i32
        {
            http_status_set_error(r, 400 as libc::c_int);
            return HANDLER_FINISHED;
        }
        if *((*h).ptr).offset(0 as libc::c_int as isize) as libc::c_int
            & 0xdf as libc::c_int == 'F' as i32
        {
            flags &= !(0x2 as libc::c_int);
        }
    }
    let destination: *const buffer = http_header_request_get(
        r,
        HTTP_HEADER_OTHER,
        b"Destination\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if destination.is_null() {
        http_status_set_error(r, 400 as libc::c_int);
        return HANDLER_FINISHED;
    }
    let mut sep: *const libc::c_char = (*destination).ptr;
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    if *sep as libc::c_int != '/' as i32 {
        start = sep;
        sep = start.offset(buffer_clen(&mut (*r).uri.scheme) as isize);
        if 0 as libc::c_int
            != strncmp(
                start,
                (*r).uri.scheme.ptr,
                sep.offset_from(start) as libc::c_long as libc::c_ulong,
            ) || *sep.offset(0 as libc::c_int as isize) as libc::c_int != ':' as i32
            || *sep.offset(1 as libc::c_int as isize) as libc::c_int != '/' as i32
            || *sep.offset(2 as libc::c_int as isize) as libc::c_int != '/' as i32
        {
            http_status_set_error(r, 400 as libc::c_int);
            return HANDLER_FINISHED;
        }
        start = sep.offset(3 as libc::c_int as isize);
        sep = strchr(start, '/' as i32);
        if sep.is_null() {
            http_status_set_error(r, 400 as libc::c_int);
            return HANDLER_FINISHED;
        }
        if buffer_eq_slen(
            &mut (*r).uri.authority,
            start,
            sep.offset_from(start) as libc::c_long as size_t,
        ) == 0
            && {
                start = memchr(
                    start as *const libc::c_void,
                    '@' as i32,
                    sep.offset_from(start) as libc::c_long as libc::c_ulong,
                ) as *mut libc::c_char;
                start.is_null()
                    || {
                        start = start.offset(1);
                        buffer_eq_slen(
                            &mut (*r).uri.authority,
                            start,
                            sep.offset_from(start) as libc::c_long as size_t,
                        ) == 0
                    }
            }
        {
            http_status_set_error(r, 502 as libc::c_int);
            return HANDLER_FINISHED;
        }
    }
    start = sep;
    sep = strchr(start, '?' as i32);
    buffer_copy_string_len(
        dst_rel_path,
        start,
        (if sep.is_null() {
            ((*destination).ptr)
                .offset((*destination).used as isize)
                .offset(-(1 as libc::c_int as isize))
                .offset_from(start) as libc::c_long
        } else {
            sep.offset_from(start) as libc::c_long
        }) as size_t,
    );
    if buffer_clen(dst_rel_path) >= 4096 as libc::c_int as libc::c_uint {
        http_status_set_error(r, 403 as libc::c_int);
        return HANDLER_FINISHED;
    }
    buffer_urldecode_path(dst_rel_path);
    if buffer_is_valid_UTF8(dst_rel_path) == 0 {
        http_status_set_error(r, 400 as libc::c_int);
        return HANDLER_FINISHED;
    }
    buffer_path_simplify(dst_rel_path);
    if buffer_is_blank(dst_rel_path) != 0
        || *((*dst_rel_path).ptr).offset(0 as libc::c_int as isize) as libc::c_int
            != '/' as i32
    {
        http_status_set_error(r, 400 as libc::c_int);
        return HANDLER_FINISHED;
    }
    if flags & 0x1 as libc::c_int != 0 {
        buffer_to_lower(dst_rel_path);
    }
    let mut i: uint32_t = 0;
    let mut remain: uint32_t = 0;
    let p1: *const libc::c_char = (*r).physical.rel_path.ptr;
    let p2: *const libc::c_char = (*dst_rel_path).ptr;
    i = 0 as libc::c_int as uint32_t;
    while *p1.offset(i as isize) as libc::c_int != 0
        && *p1.offset(i as isize) as libc::c_int == *p2.offset(i as isize) as libc::c_int
    {
        i = i.wrapping_add(1);
    }
    while i != 0 as libc::c_int as libc::c_uint
        && {
            i = i.wrapping_sub(1);
            *p1.offset(i as isize) as libc::c_int != '/' as i32
        }
    {}
    remain = ((*r).physical.rel_path.used)
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        .wrapping_sub(i);
    if ((*r).physical.path.used).wrapping_sub(1 as libc::c_int as libc::c_uint) <= remain
    {
        http_status_set_error(r, 403 as libc::c_int);
        return HANDLER_FINISHED;
    }
    if 0 as libc::c_int
        == memcmp(
            ((*r).physical.rel_path.ptr).offset(i as isize) as *const libc::c_void,
            ((*r).physical.path.ptr)
                .offset((*r).physical.path.used as isize)
                .offset(-(1 as libc::c_int as isize))
                .offset(-(remain as isize)) as *const libc::c_void,
            remain as libc::c_ulong,
        )
    {
        buffer_copy_path_len2(
            dst_path,
            (*r).physical.path.ptr,
            ((*r).physical.path.used)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_sub(remain) as size_t,
            ((*dst_rel_path).ptr).offset(i as isize),
            ((*dst_rel_path).used)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_sub(i) as size_t,
        );
        if buffer_clen(dst_path) >= 4096 as libc::c_int as libc::c_uint {
            http_status_set_error(r, 403 as libc::c_int);
            return HANDLER_FINISHED;
        }
    } else {
        buffer_copy_path_len2(
            dst_path,
            (*r).physical.doc_root.ptr,
            buffer_clen(&mut (*r).physical.doc_root) as size_t,
            (*dst_rel_path).ptr,
            buffer_clen(dst_rel_path) as size_t,
        );
        if buffer_clen(dst_path) >= 4096 as libc::c_int as libc::c_uint {
            http_status_set_error(r, 403 as libc::c_int);
            return HANDLER_FINISHED;
        }
    }
    if (*r).physical.path.used <= (*dst_path).used
        && 0 as libc::c_int
            == memcmp(
                (*r).physical.path.ptr as *const libc::c_void,
                (*dst_path).ptr as *const libc::c_void,
                ((*r).physical.path.used).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as libc::c_ulong,
            )
        && (buffer_has_pathsep_suffix(&mut (*r).physical.path) != 0
            || *((*dst_path).ptr)
                .offset(
                    ((*r).physical.path.used)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                ) as libc::c_int == '/' as i32
            || *((*dst_path).ptr)
                .offset(
                    ((*r).physical.path.used)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                ) as libc::c_int == '\u{0}' as i32)
    {
        http_status_set_error(r, 403 as libc::c_int);
        return HANDLER_FINISHED;
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
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if -(1 as libc::c_int) == lstat((*r).physical.path.ptr, &mut st) {
        http_status_set_error(
            r,
            if *__errno_location() == 2 as libc::c_int {
                404 as libc::c_int
            } else {
                403 as libc::c_int
            },
        );
        return HANDLER_FINISHED;
    }
    if 0 as libc::c_int != webdav_if_match_or_unmodified_since(r, &mut st) {
        http_status_set_error(r, 412 as libc::c_int);
        return HANDLER_FINISHED;
    }
    if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint
    {
        if buffer_has_pathsep_suffix(&mut (*r).physical.path) == 0 {
            http_response_redirect_to_directory(r, 308 as libc::c_int);
            return HANDLER_FINISHED;
        }
        if buffer_has_slash_suffix(dst_rel_path) == 0 {
            buffer_append_slash(dst_rel_path);
            buffer_append_slash(dst_path);
        }
        if 1 as libc::c_int == 0 {
            return HANDLER_FINISHED;
        }
        let depth: libc::c_int = webdav_parse_Depth(r);
        if 1 as libc::c_int == depth {
            http_status_set_error(r, 400 as libc::c_int);
            return HANDLER_FINISHED;
        }
        if 0 as libc::c_int == depth {
            if (*r).http_method as libc::c_int == HTTP_METHOD_MOVE as libc::c_int {
                http_status_set_error(r, 400 as libc::c_int);
                return HANDLER_FINISHED;
            }
            let mut status: libc::c_int = 0;
            if 0 as libc::c_int == lstat((*dst_path).ptr, &mut st) {
                if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint
                {
                    status = 204 as libc::c_int;
                } else if flags & 0x2 as libc::c_int != 0 {
                    status = webdav_mkdir(pconf, dst, 1 as libc::c_int);
                    if 0 as libc::c_int == status {
                        status = 204 as libc::c_int;
                    }
                } else {
                    status = 412 as libc::c_int;
                }
            } else if *__errno_location() == 2 as libc::c_int {
                status = webdav_mkdir(
                    pconf,
                    dst,
                    (flags & 0x2 as libc::c_int != 0) as libc::c_int,
                );
                if 0 as libc::c_int == status {
                    status = 201 as libc::c_int;
                }
            } else {
                status = 403 as libc::c_int;
            }
            if status < 300 as libc::c_int {
                (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
                (*r).handler_module = 0 as *const plugin;
                (*r).http_status = status;
                webdav_prop_copy_uri(pconf, &mut (*r).physical.rel_path, dst_rel_path);
            } else {
                http_status_set_error(r, status);
            }
            return HANDLER_FINISHED;
        }
        if 0 as libc::c_int
            == webdav_copymove_dir(pconf, &mut (*r).physical, dst, r, flags)
        {
            if (*r).http_method as libc::c_int == HTTP_METHOD_MOVE as libc::c_int {
                webdav_lock_delete_uri_col(pconf, &mut (*r).physical.rel_path);
            }
            (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
            (*r).handler_module = 0 as *const plugin;
            (*r).http_status = 200 as libc::c_int;
        } else {
            webdav_xml_doc_multistatus(r, pconf);
        }
        if (*r).http_method as libc::c_int == HTTP_METHOD_MOVE as libc::c_int {
            stat_cache_delete_dir(
                (*r).physical.path.ptr,
                buffer_clen(&mut (*r).physical.path),
            );
        }
        return HANDLER_FINISHED;
    } else if buffer_has_pathsep_suffix(&mut (*r).physical.path) != 0 {
        http_status_set_error(r, 403 as libc::c_int);
        return HANDLER_FINISHED;
    } else {
        if 1 as libc::c_int == 0 {
            return HANDLER_FINISHED;
        }
        let mut rc: libc::c_int = lstat((*dst_path).ptr, &mut st);
        if 0 as libc::c_int == rc
            && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint
        {
            sep = strrchr((*r).physical.path.ptr, '/' as i32);
            if !sep.is_null() {
                let mut len: size_t = (((*r).physical.path.used)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_long
                    - sep.offset_from((*r).physical.path.ptr) as libc::c_long) as size_t;
                if buffer_has_pathsep_suffix(dst_path) != 0 {
                    sep = sep.offset(1);
                    len = len.wrapping_sub(1);
                }
                buffer_append_string_len(dst_path, sep, len);
                buffer_append_string_len(dst_rel_path, sep, len);
                if buffer_clen(dst_path) >= 4096 as libc::c_int as libc::c_uint {
                    http_status_set_error(r, 403 as libc::c_int);
                    return HANDLER_FINISHED;
                }
                rc = lstat((*dst_path).ptr, &mut st);
                (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
                (*r).handler_module = 0 as *const plugin;
                (*r).http_status = 204 as libc::c_int;
            }
        }
        if -(1 as libc::c_int) == rc {
            let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
            's_792: {
                let mut current_block_165: u64;
                match *__errno_location() {
                    2 => {
                        if 0 as libc::c_int != (*r).http_status {
                            current_block_165 = 6186816898867308296;
                        } else {
                            slash = strrchr((*dst_path).ptr, '/' as i32);
                            if !slash.is_null() {
                                *slash = '\u{0}' as i32 as libc::c_char;
                                if 0 as libc::c_int == lstat((*dst_path).ptr, &mut st)
                                    && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                                        == 0o40000 as libc::c_int as libc::c_uint
                                {
                                    *slash = '/' as i32 as libc::c_char;
                                    if !(0 as libc::c_int != (*r).http_status) {
                                        webdav_parent_modified(dst_path);
                                        (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
                                        (*r).handler_module = 0 as *const plugin;
                                        (*r).http_status = 201 as libc::c_int;
                                    }
                                    current_block_165 = 6186816898867308296;
                                } else {
                                    current_block_165 = 8196604930743550980;
                                }
                            } else {
                                current_block_165 = 8196604930743550980;
                            }
                        }
                        match current_block_165 {
                            8196604930743550980 => {}
                            _ => {
                                break 's_792;
                            }
                        }
                    }
                    _ => {}
                }
                http_status_set_error(r, 409 as libc::c_int);
                return HANDLER_FINISHED;
            }
        } else if flags & 0x2 as libc::c_int == 0 {
            http_status_set_error(r, 412 as libc::c_int);
            return HANDLER_FINISHED;
        } else {
            if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint
            {
                http_status_set_error(r, 409 as libc::c_int);
                return HANDLER_FINISHED;
            } else {
                (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
                (*r).handler_module = 0 as *const plugin;
                (*r).http_status = 204 as libc::c_int;
            }
        }
        rc = webdav_copymove_file(pconf, &mut (*r).physical, dst, &mut flags);
        if 0 as libc::c_int == rc {
            if (*r).http_method as libc::c_int == HTTP_METHOD_MOVE as libc::c_int {
                webdav_lock_delete_uri(pconf, &mut (*r).physical.rel_path);
            }
        } else {
            http_status_set_error(r, rc);
        }
        return HANDLER_FINISHED;
    };
}
unsafe extern "C" fn mod_webdav_copymove(
    r: *mut request_st,
    pconf: *const plugin_config,
) -> handler_t {
    let mut dst_path: *mut buffer = chunk_buffer_acquire();
    let mut dst_rel_path: *mut buffer = chunk_buffer_acquire();
    let mut dst: physical_st = physical_st {
        path: buffer {
            ptr: 0 as *mut libc::c_char,
            used: 0,
            size: 0,
        },
        basedir: buffer {
            ptr: 0 as *mut libc::c_char,
            used: 0,
            size: 0,
        },
        doc_root: buffer {
            ptr: 0 as *mut libc::c_char,
            used: 0,
            size: 0,
        },
        rel_path: buffer {
            ptr: 0 as *mut libc::c_char,
            used: 0,
            size: 0,
        },
    };
    dst.path = *dst_path;
    dst.rel_path = *dst_rel_path;
    let mut rc: handler_t = mod_webdav_copymove_b(r, pconf, &mut dst);
    *dst_path = dst.path;
    *dst_rel_path = dst.rel_path;
    chunk_buffer_release(dst_rel_path);
    chunk_buffer_release(dst_path);
    return rc;
}
unsafe extern "C" fn mod_webdav_subrequest_handler(
    mut r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let pconf: *const plugin_config = *((*r).plugin_ctx)
        .offset((*(p_d as *mut plugin_data)).id as isize) as *mut plugin_config;
    if pconf.is_null() {
        return HANDLER_GO_ON;
    }
    match (*r).http_method as libc::c_int {
        26 => return mod_webdav_propfind(r, pconf),
        20 => return mod_webdav_mkcol(r, pconf),
        4 => return mod_webdav_delete(r, pconf),
        3 => return mod_webdav_put(r, pconf),
        23 | 13 => return mod_webdav_copymove(r, pconf),
        _ => {
            http_status_set_error(r, 501 as libc::c_int);
            return HANDLER_FINISHED;
        }
    };
}
unsafe extern "C" fn mod_webdav_physical_handler(
    mut r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut check_readonly: libc::c_int = 0 as libc::c_int;
    let mut check_lock_src: libc::c_int = 0 as libc::c_int;
    let mut reject_reqbody: libc::c_int = 0 as libc::c_int;
    let mut current_block_5: u64;
    match (*r).http_method as libc::c_int {
        26 | 16 => {
            current_block_5 = 11050875288958768710;
        }
        34 => {
            reject_reqbody = 1 as libc::c_int;
            current_block_5 = 11050875288958768710;
        }
        4 | 23 => {
            reject_reqbody = 1 as libc::c_int;
            current_block_5 = 2778583837755122761;
        }
        27 | 3 => {
            current_block_5 = 2778583837755122761;
        }
        13 | 20 => {
            reject_reqbody = 1 as libc::c_int;
            check_readonly = reject_reqbody;
            current_block_5 = 11050875288958768710;
        }
        0 | 1 | 2 | _ => return HANDLER_GO_ON,
    }
    match current_block_5 {
        2778583837755122761 => {
            check_lock_src = 1 as libc::c_int;
            check_readonly = check_lock_src;
        }
        _ => {}
    }
    let mut pconf: plugin_config = plugin_config {
        enabled: 0,
        is_readonly: 0,
        log_xml: 0,
        opts: 0,
        sql: 0 as *mut sql_config,
        tmpb: 0 as *mut buffer,
        sqlite_db_name: 0 as *mut buffer,
    };
    mod_webdav_patch_config(r, p_d as *mut plugin_data, &mut pconf);
    if pconf.enabled == 0 {
        return HANDLER_GO_ON;
    }
    if check_readonly != 0 && pconf.is_readonly as libc::c_int != 0 {
        http_status_set_error(r, 403 as libc::c_int);
        return HANDLER_FINISHED;
    }
    if reject_reqbody != 0 && (*r).reqbody_length != 0 {
        http_status_set_error(r, 415 as libc::c_int);
        return HANDLER_FINISHED;
    }
    if check_lock_src != 0 && 1 as libc::c_int == 0 {
        return HANDLER_FINISHED;
    }
    match (*r).http_method as libc::c_int {
        3 => {
            if mod_webdav_put_prep(r, &mut pconf) as libc::c_uint
                == HANDLER_FINISHED as libc::c_int as libc::c_uint
            {
                return HANDLER_FINISHED;
            }
        }
        _ => {}
    }
    (*r).handler_module = (*(p_d as *mut plugin_data)).self_0;
    (*r)
        .conf
        .stream_request_body = ((*r).conf.stream_request_body as libc::c_int
        & !((1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 1 as libc::c_int)) as libc::c_ushort;
    let ref mut fresh0 = *((*r).plugin_ctx)
        .offset((*(p_d as *mut plugin_data)).id as isize);
    *fresh0 = &mut pconf as *mut plugin_config as *mut libc::c_void;
    let rc: handler_t = mod_webdav_subrequest_handler(r, p_d);
    if rc as libc::c_uint == HANDLER_FINISHED as libc::c_int as libc::c_uint
        || rc as libc::c_uint == HANDLER_ERROR as libc::c_int as libc::c_uint
    {
        let ref mut fresh1 = *((*r).plugin_ctx)
            .offset((*(p_d as *mut plugin_data)).id as isize);
        *fresh1 = 0 as *mut libc::c_void;
    } else {
        let save_pconf: *mut plugin_config = malloc(
            ::std::mem::size_of::<plugin_config>() as libc::c_ulong,
        ) as *mut plugin_config;
        if save_pconf.is_null() {
            ck_assert_failed(
                b"src/mod_webdav.c\0" as *const u8 as *const libc::c_char,
                6034 as libc::c_int as libc::c_uint,
                b"save_pconf\0" as *const u8 as *const libc::c_char,
            );
        }
        memcpy(
            save_pconf as *mut libc::c_void,
            &mut pconf as *mut plugin_config as *const libc::c_void,
            ::std::mem::size_of::<plugin_config>() as libc::c_ulong,
        );
        let ref mut fresh2 = *((*r).plugin_ctx)
            .offset((*(p_d as *mut plugin_data)).id as isize);
        *fresh2 = save_pconf as *mut libc::c_void;
    }
    return rc;
}
unsafe extern "C" fn mod_webdav_handle_reset(
    mut r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let dptr: *mut *mut libc::c_void = &mut *((*r).plugin_ctx)
        .offset((*(p_d as *mut plugin_data)).id as isize) as *mut *mut libc::c_void;
    if !(*dptr).is_null() {
        free(*dptr);
        *dptr = 0 as *mut libc::c_void;
        chunkqueue_set_tempdirs(
            &mut (*r).reqbody_queue,
            (*r).reqbody_queue.tempdirs,
            0 as libc::c_int as off_t,
        );
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn run_static_initializers() {
    cpk = [
        {
            let mut init = config_plugin_keys_t {
                k: b"webdav.sqlite-db-name\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"webdav.activate\0" as *const u8 as *const libc::c_char,
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
                k: b"webdav.is-readonly\0" as *const u8 as *const libc::c_char,
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
                k: b"webdav.log-xml\0" as *const u8 as *const libc::c_char,
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
                k: b"webdav.opts\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
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
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
