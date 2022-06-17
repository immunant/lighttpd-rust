use ::libc;
extern "C" {
    pub type server;
    pub type connection;
    pub type stat_cache_entry;
    pub type pcre2_real_match_data_8;
    fn crypt(
        __phrase: *const libc::c_char,
        __setting: *const libc::c_char,
    ) -> *mut libc::c_char;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn ck_memclear_s(s: *mut libc::c_void, smax: rsize_t, n: rsize_t) -> errno_t;
    fn http_auth_backend_set(backend: *const http_auth_backend_t);
    fn http_auth_match_rules(
        require: *const http_auth_require_t,
        user: *const libc::c_char,
        group: *const libc::c_char,
        host: *const libc::c_char,
    ) -> libc::c_int;
    fn http_auth_digest_len(algo: libc::c_int) -> libc::c_uint;
    fn li_hex2bin(
        bin: *mut libc::c_uchar,
        binlen: size_t,
        hexstr: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
    fn ck_memeq_const_time(
        a: *const libc::c_void,
        alen: size_t,
        b: *const libc::c_void,
        blen: size_t,
    ) -> libc::c_int;
    fn ck_memeq_const_time_fixed_len(
        a: *const libc::c_void,
        b: *const libc::c_void,
        len: size_t,
    ) -> libc::c_int;
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn SHA1_Final(digest: *mut sha1_byte, context: *mut SHA_CTX);
    fn SHA1_Update(context: *mut SHA_CTX, data: *const sha1_byte, len: libc::c_uint);
    fn SHA1_Init(context: *mut SHA_CTX);
    fn MD5_Update(_: *mut MD5_CTX, _: *const libc::c_void, _: libc::c_uint);
    fn MD5_Init(_: *mut MD5_CTX);
    fn MD5_Final(_: *mut libc::c_uchar, _: *mut MD5_CTX);
    fn li_base64_dec(
        result: *mut libc::c_uchar,
        out_length: size_t,
        in_0: *const libc::c_char,
        in_length: size_t,
        charset: base64_charset,
    ) -> size_t;
    fn fdevent_load_file(
        fn_0: *const libc::c_char,
        lim: *mut off_t,
        errh: *mut log_error_st,
        malloc_fn: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
        free_fn: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> *mut libc::c_char;
    fn log_error(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        fmt: *const libc::c_char,
        _: ...
    );
    fn config_plugin_values_init(
        srv: *mut server,
        p_d: *mut libc::c_void,
        cpk: *const config_plugin_keys_t,
        mname: *const libc::c_char,
    ) -> libc::c_int;
    fn config_check_cond(r: *mut request_st, context_ndx: libc::c_int) -> libc::c_int;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
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
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type unix_timespec64_t = timespec;
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
pub type handler_t = libc::c_uint;
pub const HANDLER_ERROR: handler_t = 4;
pub const HANDLER_WAIT_FOR_EVENT: handler_t = 3;
pub const HANDLER_COMEBACK: handler_t = 2;
pub const HANDLER_FINISHED: handler_t = 1;
pub const HANDLER_GO_ON: handler_t = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub ptr: *mut libc::c_char,
    pub used: uint32_t,
    pub size: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chunk {
    pub next: *mut chunk,
    pub type_0: C2RustUnnamed_1,
    pub mem: *mut buffer,
    pub offset: off_t,
    pub file: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub length: off_t,
    pub fd: libc::c_int,
    pub is_temp: libc::c_int,
    pub mmap: C2RustUnnamed_0,
    pub ref_0: *mut libc::c_void,
    pub refchg: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub start: *mut libc::c_char,
    pub length: size_t,
    pub offset: off_t,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const FILE_CHUNK: C2RustUnnamed_1 = 1;
pub const MEM_CHUNK: C2RustUnnamed_1 = 0;
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
pub type log_error_st = fdlog_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct const_iovec {
    pub iov_base: *const libc::c_void,
    pub iov_len: size_t,
}
pub type errno_t = libc::c_int;
pub type http_auth_digest_type = libc::c_uint;
pub const HTTP_AUTH_DIGEST_SHA512_256: http_auth_digest_type = 8;
pub const HTTP_AUTH_DIGEST_SHA256: http_auth_digest_type = 4;
pub const HTTP_AUTH_DIGEST_MD5: http_auth_digest_type = 2;
pub const HTTP_AUTH_DIGEST_SESS: http_auth_digest_type = 1;
pub const HTTP_AUTH_DIGEST_NONE: http_auth_digest_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_auth_scheme_t {
    pub name: *const libc::c_char,
    pub checkfn: Option::<
        unsafe extern "C" fn(
            *mut request_st,
            *mut libc::c_void,
            *const http_auth_require_t,
            *const http_auth_backend_t,
        ) -> handler_t,
    >,
    pub p_d: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_auth_backend_t {
    pub name: *const libc::c_char,
    pub basic: Option::<
        unsafe extern "C" fn(
            *mut request_st,
            *mut libc::c_void,
            *const http_auth_require_t,
            *const buffer,
            *const libc::c_char,
        ) -> handler_t,
    >,
    pub digest: Option::<
        unsafe extern "C" fn(
            *mut request_st,
            *mut libc::c_void,
            *mut http_auth_info_t,
        ) -> handler_t,
    >,
    pub p_d: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_auth_info_t {
    pub dalgo: libc::c_int,
    pub dlen: libc::c_uint,
    pub username: *const libc::c_char,
    pub ulen: size_t,
    pub realm: *const libc::c_char,
    pub rlen: size_t,
    pub userhash: libc::c_int,
    pub digest: [libc::c_uchar; 32],
    pub userbuf: [libc::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_auth_require_t {
    pub scheme: *const http_auth_scheme_t,
    pub realm: *const buffer,
    pub nonce_secret: *const buffer,
    pub valid_user: uint8_t,
    pub userhash: uint8_t,
    pub algorithm: libc::c_int,
    pub user: array,
    pub group: array,
    pub host: array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5_CTX {
    pub state: [uint32_t; 4],
    pub count: [uint32_t; 2],
    pub buffer: [libc::c_uchar; 64],
}
pub type sha1_quadbyte = uint32_t;
pub type sha1_byte = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SHA_CTX {
    pub state: [sha1_quadbyte; 5],
    pub count: [sha1_quadbyte; 2],
    pub buffer: [sha1_byte; 64],
}
pub type SHA_CTX = _SHA_CTX;
pub type SHA1_CTX = SHA_CTX;
pub type li_md_iov_fn = Option::<
    unsafe extern "C" fn(*mut libc::c_uchar, *const const_iovec, size_t) -> (),
>;
pub type base64_charset = libc::c_uint;
pub const BASE64_URL: base64_charset = 1;
pub const BASE64_STANDARD: base64_charset = 0;
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const T_CONFIG_SCOPE_CONNECTION: C2RustUnnamed_3 = 2;
pub const T_CONFIG_SCOPE_SERVER: C2RustUnnamed_3 = 1;
pub const T_CONFIG_SCOPE_UNSET: C2RustUnnamed_3 = 0;
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
pub struct plugin_config {
    pub auth_plain_groupfile: *const buffer,
    pub auth_plain_userfile: *const buffer,
    pub auth_htdigest_userfile: *const buffer,
    pub auth_htpasswd_userfile: *const buffer,
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
unsafe extern "C" fn ck_memzero(mut s: *mut libc::c_void, mut n: rsize_t) -> errno_t {
    return ck_memclear_s(s, n, n);
}
#[inline]
unsafe extern "C" fn MD5_iov(
    digest: *mut libc::c_uchar,
    iov: *const const_iovec,
    n: size_t,
) {
    let mut ctx: MD5_CTX = MD5_CTX {
        state: [0; 4],
        count: [0; 2],
        buffer: [0; 64],
    };
    MD5_Init(&mut ctx);
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n {
        if (*iov.offset(i as isize)).iov_len != 0 {
            MD5_Update(
                &mut ctx,
                (*iov.offset(i as isize)).iov_base,
                (*iov.offset(i as isize)).iov_len as libc::c_uint,
            );
        }
        i = i.wrapping_add(1);
    }
    MD5_Final(digest, &mut ctx);
}
#[inline]
unsafe extern "C" fn SHA1_once(
    digest: *mut libc::c_uchar,
    data: *const libc::c_void,
    n: size_t,
) {
    let mut ctx: SHA1_CTX = SHA1_CTX {
        state: [0; 5],
        count: [0; 2],
        buffer: [0; 64],
    };
    SHA1_Init(&mut ctx);
    SHA1_Update(&mut ctx, data as *const sha1_byte, n as libc::c_uint);
    SHA1_Final(digest, &mut ctx);
}
#[cold]
unsafe extern "C" fn mod_authn_file_init() -> *mut libc::c_void {
    static mut http_auth_backend_htdigest: http_auth_backend_t = unsafe {
        {
            let mut init = http_auth_backend_t {
                name: b"htdigest\0" as *const u8 as *const libc::c_char,
                basic: Some(
                    mod_authn_file_htdigest_basic
                        as unsafe extern "C" fn(
                            *mut request_st,
                            *mut libc::c_void,
                            *const http_auth_require_t,
                            *const buffer,
                            *const libc::c_char,
                        ) -> handler_t,
                ),
                digest: Some(
                    mod_authn_file_htdigest_digest
                        as unsafe extern "C" fn(
                            *mut request_st,
                            *mut libc::c_void,
                            *mut http_auth_info_t,
                        ) -> handler_t,
                ),
                p_d: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        }
    };
    static mut http_auth_backend_htpasswd: http_auth_backend_t = unsafe {
        {
            let mut init = http_auth_backend_t {
                name: b"htpasswd\0" as *const u8 as *const libc::c_char,
                basic: Some(
                    mod_authn_file_htpasswd_basic
                        as unsafe extern "C" fn(
                            *mut request_st,
                            *mut libc::c_void,
                            *const http_auth_require_t,
                            *const buffer,
                            *const libc::c_char,
                        ) -> handler_t,
                ),
                digest: None,
                p_d: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        }
    };
    static mut http_auth_backend_plain: http_auth_backend_t = unsafe {
        {
            let mut init = http_auth_backend_t {
                name: b"plain\0" as *const u8 as *const libc::c_char,
                basic: Some(
                    mod_authn_file_plain_basic
                        as unsafe extern "C" fn(
                            *mut request_st,
                            *mut libc::c_void,
                            *const http_auth_require_t,
                            *const buffer,
                            *const libc::c_char,
                        ) -> handler_t,
                ),
                digest: Some(
                    mod_authn_file_plain_digest
                        as unsafe extern "C" fn(
                            *mut request_st,
                            *mut libc::c_void,
                            *mut http_auth_info_t,
                        ) -> handler_t,
                ),
                p_d: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        }
    };
    let mut p: *mut plugin_data = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<plugin_data>() as libc::c_ulong,
    ) as *mut plugin_data;
    http_auth_backend_htdigest.p_d = p as *mut libc::c_void;
    http_auth_backend_set(&mut http_auth_backend_htdigest);
    http_auth_backend_htpasswd.p_d = p as *mut libc::c_void;
    http_auth_backend_set(&mut http_auth_backend_htpasswd);
    http_auth_backend_plain.p_d = p as *mut libc::c_void;
    http_auth_backend_set(&mut http_auth_backend_plain);
    return p as *mut libc::c_void;
}
unsafe extern "C" fn mod_authn_file_merge_config_cpv(
    pconf: *mut plugin_config,
    cpv: *const config_plugin_value_t,
) {
    match (*cpv).k_id {
        0 => {
            (*pconf).auth_plain_groupfile = (*cpv).v.b;
        }
        1 => {
            (*pconf).auth_plain_userfile = (*cpv).v.b;
        }
        2 => {
            (*pconf).auth_htdigest_userfile = (*cpv).v.b;
        }
        3 => {
            (*pconf).auth_htpasswd_userfile = (*cpv).v.b;
        }
        _ => return,
    };
}
unsafe extern "C" fn mod_authn_file_merge_config(
    pconf: *mut plugin_config,
    mut cpv: *const config_plugin_value_t,
) {
    loop {
        mod_authn_file_merge_config_cpv(pconf, cpv);
        cpv = cpv.offset(1);
        if !((*cpv).k_id != -(1 as libc::c_int)) {
            break;
        }
    };
}
unsafe extern "C" fn mod_authn_file_patch_config(
    r: *mut request_st,
    p: *mut plugin_data,
) {
    (*p).conf = (*p).defaults;
    let mut i: libc::c_int = 1 as libc::c_int;
    let mut used: libc::c_int = (*p).nconfig;
    while i < used {
        if config_check_cond(
            r,
            (*((*p).cvlist).offset(i as isize)).k_id as uint32_t as libc::c_int,
        ) != 0
        {
            mod_authn_file_merge_config(
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
unsafe extern "C" fn mod_authn_file_set_defaults(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    if config_plugin_values_init(
        srv,
        p as *mut libc::c_void,
        cpk.as_ptr(),
        b"mod_authn_file\0" as *const u8 as *const libc::c_char,
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
            let mut current_block_3: u64;
            match (*cpv).k_id {
                1 => {
                    current_block_3 = 14277861673517172765;
                }
                2 => {
                    current_block_3 = 14277861673517172765;
                }
                0 | 3 => {
                    current_block_3 = 6891228234384637087;
                }
                _ => {
                    current_block_3 = 7651349459974463963;
                }
            }
            match current_block_3 {
                14277861673517172765 => {
                    current_block_3 = 6891228234384637087;
                }
                _ => {}
            }
            match current_block_3 {
                6891228234384637087 => {
                    if buffer_is_blank((*cpv).v.b) != 0 {
                        (*cpv).v.b = 0 as *const buffer;
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
            mod_authn_file_merge_config(&mut (*p).defaults, cpv_0);
        }
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_authn_file_digest(
    mut ai: *mut http_auth_info_t,
    mut pw: *const libc::c_char,
    mut pwlen: size_t,
) {
    let mut digest_iov: li_md_iov_fn = Some(
        MD5_iov
            as unsafe extern "C" fn(*mut libc::c_uchar, *const const_iovec, size_t) -> (),
    );
    let mut iov: [const_iovec; 5] = [
        {
            let mut init = const_iovec {
                iov_base: (*ai).username as *const libc::c_void,
                iov_len: (*ai).ulen,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: b":\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                iov_len: 1 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: (*ai).realm as *const libc::c_void,
                iov_len: (*ai).rlen,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: b":\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                iov_len: 1 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: pw as *const libc::c_void,
                iov_len: pwlen,
            };
            init
        },
    ];
    digest_iov
        .expect(
            "non-null function pointer",
        )(
        ((*ai).digest).as_mut_ptr(),
        iov.as_mut_ptr(),
        (::std::mem::size_of::<[const_iovec; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<const_iovec>() as libc::c_ulong),
    );
}
unsafe extern "C" fn mod_authn_file_htdigest_get_loop(
    mut data: *const libc::c_char,
    mut auth_fn: *const buffer,
    mut ai: *mut http_auth_info_t,
    mut errh: *mut log_error_st,
) -> libc::c_int {
    let mut f_user: *const libc::c_char = data;
    let mut n: *const libc::c_char = 0 as *const libc::c_char;
    let mut current_block_20: u64;
    loop {
        n = strchr(f_user, '\n' as i32);
        if n.is_null() {
            n = f_user.offset(strlen(f_user) as isize);
        }
        let mut f_pwd: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut f_realm: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut u_len: size_t = 0;
        let mut r_len: size_t = 0;
        if !(*f_user.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32
            || *f_user.offset(0 as libc::c_int as isize) as libc::c_int == '\r' as i32
            || *f_user.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
            || *f_user.offset(0 as libc::c_int as isize) as libc::c_int
                == '\u{0}' as i32)
        {
            if !(n.offset_from(f_user) as libc::c_long
                > 1024 as libc::c_int as libc::c_long)
            {
                f_realm = memchr(
                    f_user as *const libc::c_void,
                    ':' as i32,
                    n.offset_from(f_user) as libc::c_long as libc::c_ulong,
                ) as *mut libc::c_char;
                if f_realm.is_null()
                    || {
                        f_pwd = memchr(
                            f_realm.offset(1 as libc::c_int as isize)
                                as *const libc::c_void,
                            ':' as i32,
                            n.offset_from(f_realm.offset(1 as libc::c_int as isize))
                                as libc::c_long as libc::c_ulong,
                        ) as *mut libc::c_char;
                        f_pwd.is_null()
                    }
                {
                    log_error(
                        errh,
                        b"src/mod_authn_file.c\0" as *const u8 as *const libc::c_char,
                        226 as libc::c_int as libc::c_uint,
                        b"parse error in %s expected 'username:realm:digest[:userhash]'\0"
                            as *const u8 as *const libc::c_char,
                        (*auth_fn).ptr,
                    );
                } else {
                    u_len = f_realm.offset_from(f_user) as libc::c_long as size_t;
                    f_realm = f_realm.offset(1);
                    r_len = f_pwd.offset_from(f_realm) as libc::c_long as size_t;
                    f_pwd = f_pwd.offset(1);
                    let mut f_userhash: *const libc::c_char = memchr(
                        f_pwd as *const libc::c_void,
                        ':' as i32,
                        n.offset_from(f_pwd) as libc::c_long as size_t,
                    ) as *const libc::c_char;
                    if (*ai).userhash != 0 {
                        if f_userhash.is_null() {
                            current_block_20 = 11174649648027449784;
                        } else {
                            f_userhash = f_userhash.offset(1);
                            let mut uh_len: size_t = n.offset_from(f_userhash)
                                as libc::c_long as size_t;
                            if *f_userhash
                                .offset(
                                    uh_len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as isize,
                                ) as libc::c_int == '\r' as i32
                            {
                                uh_len = uh_len.wrapping_sub(1);
                            }
                            if (*ai).ulen == uh_len && (*ai).rlen == r_len
                                && ck_memeq_const_time_fixed_len(
                                    (*ai).username as *const libc::c_void,
                                    f_userhash as *const libc::c_void,
                                    uh_len,
                                ) != 0
                                && 0 as libc::c_int
                                    == memcmp(
                                        (*ai).realm as *const libc::c_void,
                                        f_realm as *const libc::c_void,
                                        r_len,
                                    )
                                && u_len
                                    <= ::std::mem::size_of::<[libc::c_char; 256]>()
                                        as libc::c_ulong
                            {
                                (*ai).ulen = u_len;
                                (*ai)
                                    .username = memcpy(
                                    ((*ai).userbuf).as_mut_ptr() as *mut libc::c_void,
                                    f_user as *const libc::c_void,
                                    u_len,
                                ) as *const libc::c_char;
                                f_userhash = f_userhash.offset(-1);
                                current_block_20 = 4488286894823169796;
                            } else {
                                current_block_20 = 11174649648027449784;
                            }
                        }
                    } else if (*ai).ulen == u_len && (*ai).rlen == r_len
                            && 0 as libc::c_int
                                == memcmp(
                                    (*ai).username as *const libc::c_void,
                                    f_user as *const libc::c_void,
                                    u_len,
                                )
                            && 0 as libc::c_int
                                == memcmp(
                                    (*ai).realm as *const libc::c_void,
                                    f_realm as *const libc::c_void,
                                    r_len,
                                )
                        {
                        if f_userhash.is_null() {
                            f_userhash = n;
                        }
                        current_block_20 = 4488286894823169796;
                    } else {
                        current_block_20 = 11174649648027449784;
                    }
                    match current_block_20 {
                        11174649648027449784 => {}
                        _ => {
                            let mut pwd_len: size_t = f_userhash.offset_from(f_pwd)
                                as libc::c_long as size_t;
                            if *f_pwd
                                .offset(
                                    pwd_len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as isize,
                                ) as libc::c_int == '\r' as i32
                            {
                                pwd_len = pwd_len.wrapping_sub(1);
                            }
                            if !(pwd_len
                                != ((*ai).dlen << 1 as libc::c_int) as libc::c_ulong)
                            {
                                return li_hex2bin(
                                    ((*ai).digest).as_mut_ptr(),
                                    ::std::mem::size_of::<[libc::c_uchar; 32]>()
                                        as libc::c_ulong,
                                    f_pwd,
                                    pwd_len,
                                );
                            }
                        }
                    }
                }
            }
        }
        if !(*n as libc::c_int != 0
            && {
                f_user = n.offset(1 as libc::c_int as isize);
                *f_user as libc::c_int != 0
            })
        {
            break;
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mod_authn_file_htdigest_get(
    r: *mut request_st,
    mut p_d: *mut libc::c_void,
    ai: *mut http_auth_info_t,
) -> libc::c_int {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    mod_authn_file_patch_config(r, p);
    let auth_fn: *const buffer = (*p).conf.auth_htdigest_userfile;
    if auth_fn.is_null() {
        return -(1 as libc::c_int);
    }
    let mut dlen: off_t = (64 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int)
        as off_t;
    let mut data: *mut libc::c_char = fdevent_load_file(
        (*auth_fn).ptr,
        &mut dlen,
        (*r).conf.errh,
        Some(malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    if data.is_null() {
        return -(1 as libc::c_int);
    }
    let mut rc: libc::c_int = mod_authn_file_htdigest_get_loop(
        data,
        auth_fn,
        ai,
        (*r).conf.errh,
    );
    ck_memzero(data as *mut libc::c_void, dlen as size_t);
    free(data as *mut libc::c_void);
    return rc;
}
unsafe extern "C" fn mod_authn_file_htdigest_digest(
    r: *mut request_st,
    mut p_d: *mut libc::c_void,
    ai: *mut http_auth_info_t,
) -> handler_t {
    return (if 0 as libc::c_int == mod_authn_file_htdigest_get(r, p_d, ai) {
        HANDLER_GO_ON as libc::c_int
    } else {
        HANDLER_ERROR as libc::c_int
    }) as handler_t;
}
unsafe extern "C" fn mod_authn_file_htdigest_basic(
    r: *mut request_st,
    mut p_d: *mut libc::c_void,
    require: *const http_auth_require_t,
    username: *const buffer,
    pw: *const libc::c_char,
) -> handler_t {
    let mut ai: http_auth_info_t = http_auth_info_t {
        dalgo: 0,
        dlen: 0,
        username: 0 as *const libc::c_char,
        ulen: 0,
        realm: 0 as *const libc::c_char,
        rlen: 0,
        userhash: 0,
        digest: [0; 32],
        userbuf: [0; 256],
    };
    let mut htdigest: [libc::c_uchar; 32] = [0; 32];
    ai.dalgo = (*require).algorithm & !(HTTP_AUTH_DIGEST_SESS as libc::c_int);
    ai.dlen = http_auth_digest_len(ai.dalgo);
    ai.username = (*username).ptr;
    ai.ulen = buffer_clen(username) as size_t;
    ai.realm = (*(*require).realm).ptr;
    ai.rlen = buffer_clen((*require).realm) as size_t;
    ai.userhash = 0 as libc::c_int;
    if mod_authn_file_htdigest_get(r, p_d, &mut ai) != 0 {
        return HANDLER_ERROR;
    }
    if ai.dlen as libc::c_ulong
        > ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong
    {
        ck_memzero((ai.digest).as_mut_ptr() as *mut libc::c_void, ai.dlen as rsize_t);
        return HANDLER_ERROR;
    }
    memcpy(
        htdigest.as_mut_ptr() as *mut libc::c_void,
        (ai.digest).as_mut_ptr() as *const libc::c_void,
        ai.dlen as libc::c_ulong,
    );
    mod_authn_file_digest(&mut ai, pw, strlen(pw));
    let mut rc: libc::c_int = (ck_memeq_const_time_fixed_len(
        htdigest.as_mut_ptr() as *const libc::c_void,
        (ai.digest).as_mut_ptr() as *const libc::c_void,
        ai.dlen as size_t,
    ) != 0
        && http_auth_match_rules(
            require,
            (*username).ptr,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
        ) != 0) as libc::c_int;
    ck_memzero(htdigest.as_mut_ptr() as *mut libc::c_void, ai.dlen as rsize_t);
    ck_memzero((ai.digest).as_mut_ptr() as *mut libc::c_void, ai.dlen as rsize_t);
    return (if rc != 0 {
        HANDLER_GO_ON as libc::c_int
    } else {
        HANDLER_ERROR as libc::c_int
    }) as handler_t;
}
unsafe extern "C" fn mod_authn_file_htpasswd_get(
    mut auth_fn: *const buffer,
    mut username: *const libc::c_char,
    mut userlen: size_t,
    mut password: *mut buffer,
    mut errh: *mut log_error_st,
) -> libc::c_int {
    if username.is_null() {
        return -(1 as libc::c_int);
    }
    if auth_fn.is_null() {
        return -(1 as libc::c_int);
    }
    let mut dlen: off_t = (64 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int)
        as off_t;
    let mut data: *mut libc::c_char = fdevent_load_file(
        (*auth_fn).ptr,
        &mut dlen,
        errh,
        Some(malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    if data.is_null() {
        return -(1 as libc::c_int);
    }
    let mut rc: libc::c_int = -(1 as libc::c_int);
    let mut f_user: *const libc::c_char = data;
    let mut n: *const libc::c_char = 0 as *const libc::c_char;
    loop {
        n = strchr(f_user, '\n' as i32);
        if n.is_null() {
            n = f_user.offset(strlen(f_user) as isize);
        }
        let mut f_pwd: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut u_len: size_t = 0;
        if !(*f_user.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32
            || *f_user.offset(0 as libc::c_int as isize) as libc::c_int == '\r' as i32
            || *f_user.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
            || *f_user.offset(0 as libc::c_int as isize) as libc::c_int
                == '\u{0}' as i32)
        {
            if !(n.offset_from(f_user) as libc::c_long
                > 1024 as libc::c_int as libc::c_long)
            {
                f_pwd = memchr(
                    f_user as *const libc::c_void,
                    ':' as i32,
                    n.offset_from(f_user) as libc::c_long as libc::c_ulong,
                ) as *mut libc::c_char;
                if f_pwd.is_null() {
                    log_error(
                        errh,
                        b"src/mod_authn_file.c\0" as *const u8 as *const libc::c_char,
                        369 as libc::c_int as libc::c_uint,
                        b"parsed error in %s expected 'username:password'\0" as *const u8
                            as *const libc::c_char,
                        (*auth_fn).ptr,
                    );
                } else {
                    u_len = f_pwd.offset_from(f_user) as libc::c_long as size_t;
                    f_pwd = f_pwd.offset(1);
                    if userlen == u_len
                        && 0 as libc::c_int
                            == memcmp(
                                username as *const libc::c_void,
                                f_user as *const libc::c_void,
                                u_len,
                            )
                    {
                        let mut pwd_len: size_t = n.offset_from(f_pwd) as libc::c_long
                            as size_t;
                        if *f_pwd
                            .offset(
                                pwd_len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int == '\r' as i32
                        {
                            pwd_len = pwd_len.wrapping_sub(1);
                        }
                        buffer_copy_string_len(password, f_pwd, pwd_len);
                        rc = 0 as libc::c_int;
                        break;
                    }
                }
            }
        }
        if !(*n as libc::c_int != 0
            && {
                f_user = n.offset(1 as libc::c_int as isize);
                *f_user as libc::c_int != 0
            })
        {
            break;
        }
    }
    ck_memzero(data as *mut libc::c_void, dlen as size_t);
    free(data as *mut libc::c_void);
    return rc;
}
unsafe extern "C" fn mod_authn_file_plain_digest(
    r: *mut request_st,
    mut p_d: *mut libc::c_void,
    ai: *mut http_auth_info_t,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    mod_authn_file_patch_config(r, p);
    let tb: *mut buffer = (*r).tmp_buf;
    let mut rc: libc::c_int = mod_authn_file_htpasswd_get(
        (*p).conf.auth_plain_userfile,
        (*ai).username,
        (*ai).ulen,
        tb,
        (*r).conf.errh,
    );
    if 0 as libc::c_int != rc {
        return HANDLER_ERROR;
    }
    mod_authn_file_digest(ai, (*tb).ptr, buffer_clen(tb) as size_t);
    let mut tblen: size_t = ((buffer_clen(tb))
        .wrapping_add(63 as libc::c_int as libc::c_uint) & !(63 as libc::c_uint))
        as size_t;
    buffer_clear(tb);
    ck_memzero(
        (*tb).ptr as *mut libc::c_void,
        if tblen < (*tb).size as libc::c_ulong {
            tblen
        } else {
            (*tb).size as libc::c_ulong
        },
    );
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_authn_file_plain_basic(
    r: *mut request_st,
    mut p_d: *mut libc::c_void,
    require: *const http_auth_require_t,
    username: *const buffer,
    pw: *const libc::c_char,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    mod_authn_file_patch_config(r, p);
    let tb: *mut buffer = (*r).tmp_buf;
    let mut rc: libc::c_int = mod_authn_file_htpasswd_get(
        (*p).conf.auth_plain_userfile,
        (*username).ptr,
        buffer_clen(username) as size_t,
        tb,
        (*r).conf.errh,
    );
    if 0 as libc::c_int == rc {
        rc = if ck_memeq_const_time(
            (*tb).ptr as *const libc::c_void,
            buffer_clen(tb) as size_t,
            pw as *const libc::c_void,
            strlen(pw),
        ) != 0
        {
            0 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
        let mut tblen: size_t = ((buffer_clen(tb))
            .wrapping_add(63 as libc::c_int as libc::c_uint) & !(63 as libc::c_uint))
            as size_t;
        buffer_clear(tb);
        ck_memzero(
            (*tb).ptr as *mut libc::c_void,
            if tblen < (*tb).size as libc::c_ulong {
                tblen
            } else {
                (*tb).size as libc::c_ulong
            },
        );
    }
    return (if 0 as libc::c_int == rc
        && http_auth_match_rules(
            require,
            (*username).ptr,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
        ) != 0
    {
        HANDLER_GO_ON as libc::c_int
    } else {
        HANDLER_ERROR as libc::c_int
    }) as handler_t;
}
unsafe extern "C" fn to64(
    mut s: *mut libc::c_char,
    mut v: libc::c_ulong,
    mut n: libc::c_int,
) {
    static mut itoa64: [libc::c_uchar; 65] = unsafe {
        *::std::mem::transmute::<
            &[u8; 65],
            &[libc::c_uchar; 65],
        >(b"./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz\0")
    };
    loop {
        n -= 1;
        if !(n >= 0 as libc::c_int) {
            break;
        }
        let fresh0 = s;
        s = s.offset(1);
        *fresh0 = itoa64[(v & 0x3f as libc::c_int as libc::c_ulong) as usize]
            as libc::c_char;
        v >>= 6 as libc::c_int;
    };
}
unsafe extern "C" fn apr_md5_encode(
    mut pw: *const libc::c_char,
    mut salt: *const libc::c_char,
    mut result: *mut libc::c_char,
    mut nbytes: size_t,
) -> size_t {
    if !(nbytes >= 37 as libc::c_int as libc::c_ulong) {
        ck_assert_failed(
            b"src/mod_authn_file.c\0" as *const u8 as *const libc::c_char,
            472 as libc::c_int as libc::c_uint,
            b"nbytes >= 37\0" as *const u8 as *const libc::c_char,
        );
    }
    let pwlen: size_t = strlen(pw);
    let mut sl: ssize_t = 0;
    sl = 0 as libc::c_int as ssize_t;
    while sl < 8 as libc::c_int as libc::c_long
        && *salt.offset(sl as isize) as libc::c_int != '$' as i32
        && *salt.offset(sl as isize) as libc::c_int != '\u{0}' as i32
    {
        sl += 1;
    }
    memcpy(
        result as *mut libc::c_void,
        b"$apr1$\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    memcpy(
        result
            .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize)
            .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
        salt as *const libc::c_void,
        sl as libc::c_ulong,
    );
    *result
        .offset(
            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(sl as libc::c_ulong) as isize,
        ) = '$' as i32 as libc::c_char;
    let mut ctx: MD5_CTX = MD5_CTX {
        state: [0; 4],
        count: [0; 2],
        buffer: [0; 64],
    };
    let mut final_0: [libc::c_uchar; 16] = [0; 16];
    MD5_Init(&mut ctx);
    MD5_Update(&mut ctx, pw as *const libc::c_void, pwlen as libc::c_uint);
    MD5_Update(&mut ctx, salt as *const libc::c_void, sl as libc::c_uint);
    MD5_Update(&mut ctx, pw as *const libc::c_void, pwlen as libc::c_uint);
    MD5_Final(final_0.as_mut_ptr(), &mut ctx);
    MD5_Init(&mut ctx);
    MD5_Update(&mut ctx, pw as *const libc::c_void, pwlen as libc::c_uint);
    MD5_Update(
        &mut ctx,
        result as *const libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(sl as libc::c_ulong) as libc::c_uint,
    );
    let mut pl: ssize_t = pwlen as ssize_t;
    while pl > 0 as libc::c_int as libc::c_long {
        MD5_Update(
            &mut ctx,
            final_0.as_mut_ptr() as *const libc::c_void,
            (if pl > 16 as libc::c_int as libc::c_long {
                16 as libc::c_int as libc::c_long
            } else {
                pl
            }) as libc::c_uint,
        );
        pl -= 16 as libc::c_int as libc::c_long;
    }
    final_0[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    let mut i: size_t = pwlen;
    while i != 0 as libc::c_int as libc::c_ulong {
        MD5_Update(
            &mut ctx,
            (if i & 1 as libc::c_int as libc::c_ulong != 0 {
                final_0.as_mut_ptr() as *mut libc::c_char as *const libc::c_char
            } else {
                pw
            }) as *const libc::c_void,
            1 as libc::c_int as libc::c_uint,
        );
        i >>= 1 as libc::c_int;
    }
    MD5_Final(final_0.as_mut_ptr(), &mut ctx);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 1000 as libc::c_int {
        MD5_Init(&mut ctx);
        if i_0 & 1 as libc::c_int != 0 {
            MD5_Update(&mut ctx, pw as *const libc::c_void, pwlen as libc::c_uint);
        } else {
            MD5_Update(
                &mut ctx,
                final_0.as_mut_ptr() as *const libc::c_void,
                16 as libc::c_int as libc::c_uint,
            );
        }
        if i_0 % 3 as libc::c_int != 0 {
            MD5_Update(&mut ctx, salt as *const libc::c_void, sl as libc::c_uint);
        }
        if i_0 % 7 as libc::c_int != 0 {
            MD5_Update(&mut ctx, pw as *const libc::c_void, pwlen as libc::c_uint);
        }
        if i_0 & 1 as libc::c_int != 0 {
            MD5_Update(
                &mut ctx,
                final_0.as_mut_ptr() as *const libc::c_void,
                16 as libc::c_int as libc::c_uint,
            );
        } else {
            MD5_Update(&mut ctx, pw as *const libc::c_void, pwlen as libc::c_uint);
        }
        MD5_Final(final_0.as_mut_ptr(), &mut ctx);
        i_0 += 1;
    }
    result = result
        .offset(
            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(sl as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
        );
    to64(
        result,
        ((final_0[0 as libc::c_int as usize] as libc::c_int) << 16 as libc::c_int
            | (final_0[6 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int
            | final_0[12 as libc::c_int as usize] as libc::c_int) as libc::c_ulong,
        4 as libc::c_int,
    );
    to64(
        result.offset(4 as libc::c_int as isize),
        ((final_0[1 as libc::c_int as usize] as libc::c_int) << 16 as libc::c_int
            | (final_0[7 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int
            | final_0[13 as libc::c_int as usize] as libc::c_int) as libc::c_ulong,
        4 as libc::c_int,
    );
    to64(
        result.offset(8 as libc::c_int as isize),
        ((final_0[2 as libc::c_int as usize] as libc::c_int) << 16 as libc::c_int
            | (final_0[8 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int
            | final_0[14 as libc::c_int as usize] as libc::c_int) as libc::c_ulong,
        4 as libc::c_int,
    );
    to64(
        result.offset(12 as libc::c_int as isize),
        ((final_0[3 as libc::c_int as usize] as libc::c_int) << 16 as libc::c_int
            | (final_0[9 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int
            | final_0[15 as libc::c_int as usize] as libc::c_int) as libc::c_ulong,
        4 as libc::c_int,
    );
    to64(
        result.offset(16 as libc::c_int as isize),
        ((final_0[4 as libc::c_int as usize] as libc::c_int) << 16 as libc::c_int
            | (final_0[10 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int
            | final_0[5 as libc::c_int as usize] as libc::c_int) as libc::c_ulong,
        4 as libc::c_int,
    );
    to64(
        result.offset(20 as libc::c_int as isize),
        final_0[11 as libc::c_int as usize] as libc::c_ulong,
        2 as libc::c_int,
    );
    ck_memzero(
        final_0.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong,
    );
    return (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(sl as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(22 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn mod_authn_file_crypt_cmp(
    password: *const buffer,
    pw: *const libc::c_char,
) -> libc::c_int {
    let mut rc: libc::c_int = -(1 as libc::c_int);
    let mut crypted: *mut libc::c_char = 0 as *mut libc::c_char;
    crypted = crypt(pw, (*password).ptr);
    if !crypted.is_null() {
        rc = strcmp((*password).ptr, crypted);
    }
    if !crypted.is_null() {
        let mut crypwlen: size_t = strlen(crypted);
        if crypwlen >= 13 as libc::c_int as libc::c_ulong {
            ck_memzero(crypted as *mut libc::c_void, crypwlen);
        }
    }
    return rc;
}
unsafe extern "C" fn mod_authn_file_htpasswd_basic(
    r: *mut request_st,
    mut p_d: *mut libc::c_void,
    require: *const http_auth_require_t,
    username: *const buffer,
    pw: *const libc::c_char,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    mod_authn_file_patch_config(r, p);
    let tb: *mut buffer = (*r).tmp_buf;
    let mut rc: libc::c_int = mod_authn_file_htpasswd_get(
        (*p).conf.auth_htpasswd_userfile,
        (*username).ptr,
        buffer_clen(username) as size_t,
        tb,
        (*r).conf.errh,
    );
    if 0 as libc::c_int != rc {
        return HANDLER_ERROR;
    }
    let mut tblen: uint32_t = buffer_clen(tb);
    rc = -(1 as libc::c_int);
    if tblen >= 5 as libc::c_int as libc::c_uint
        && 0 as libc::c_int
            == memcmp(
                (*tb).ptr as *const libc::c_void,
                b"{SHA}\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                5 as libc::c_int as libc::c_ulong,
            )
    {
        let mut digest: [libc::c_uchar; 40] = [0; 40];
        SHA1_once(
            digest.as_mut_ptr().offset(20 as libc::c_int as isize),
            pw as *const libc::c_void,
            strlen(pw),
        );
        rc = (20 as libc::c_int as libc::c_ulong
            == li_base64_dec(
                digest.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_uchar; 40]>() as libc::c_ulong,
                ((*tb).ptr).offset(5 as libc::c_int as isize),
                tblen.wrapping_sub(5 as libc::c_int as libc::c_uint) as size_t,
                BASE64_STANDARD,
            )
            && ck_memeq_const_time_fixed_len(
                digest.as_mut_ptr() as *const libc::c_void,
                digest.as_mut_ptr().offset(20 as libc::c_int as isize)
                    as *const libc::c_void,
                20 as libc::c_int as size_t,
            ) != 0) as libc::c_int;
        rc = (rc == 0) as libc::c_int;
        ck_memzero(
            digest.as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<[libc::c_uchar; 40]>() as libc::c_ulong,
        );
    } else if tblen >= 6 as libc::c_int as libc::c_uint
            && 0 as libc::c_int
                == memcmp(
                    (*tb).ptr as *const libc::c_void,
                    b"$apr1$\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    6 as libc::c_int as libc::c_ulong,
                )
        {
        let mut sample: [libc::c_char; 40] = [0; 40];
        rc = (tblen as libc::c_ulong
            == apr_md5_encode(
                pw,
                ((*tb).ptr).offset(6 as libc::c_int as isize),
                sample.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
            )
            && ck_memeq_const_time_fixed_len(
                sample.as_mut_ptr() as *const libc::c_void,
                (*tb).ptr as *const libc::c_void,
                tblen as size_t,
            ) != 0) as libc::c_int;
        rc = (rc == 0) as libc::c_int;
        ck_memzero(
            sample.as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
        );
    } else if tblen >= 13 as libc::c_int as libc::c_uint {
        rc = mod_authn_file_crypt_cmp(tb, pw);
    }
    tblen = tblen.wrapping_add(63 as libc::c_int as libc::c_uint)
        & !(63 as libc::c_uint);
    buffer_clear(tb);
    ck_memzero(
        (*tb).ptr as *mut libc::c_void,
        (if tblen < (*tb).size { tblen } else { (*tb).size }) as rsize_t,
    );
    return (if 0 as libc::c_int == rc
        && http_auth_match_rules(
            require,
            (*username).ptr,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
        ) != 0
    {
        HANDLER_GO_ON as libc::c_int
    } else {
        HANDLER_ERROR as libc::c_int
    }) as handler_t;
}
#[no_mangle]
pub unsafe extern "C" fn mod_authn_file_plugin_init(mut p: *mut plugin) -> libc::c_int {
    (*p).version = 0x10440 as libc::c_int as size_t;
    (*p).name = b"authn_file\0" as *const u8 as *const libc::c_char;
    (*p)
        .init = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
    >(Some(mod_authn_file_init as unsafe extern "C" fn() -> *mut libc::c_void));
    (*p)
        .set_defaults = Some(
        mod_authn_file_set_defaults
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    return 0 as libc::c_int;
}
pub unsafe fn run_static_initializers() {
    cpk = [
        {
            let mut init = config_plugin_keys_t {
                k: b"auth.backend.plain.groupfile\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"auth.backend.plain.userfile\0" as *const u8 as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"auth.backend.htdigest.userfile\0" as *const u8
                    as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"auth.backend.htpasswd.userfile\0" as *const u8
                    as *const libc::c_char,
                klen: (::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
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
