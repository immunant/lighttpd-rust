use ::libc;
extern "C" {
    pub type connection;
    pub type plugin;
    pub type stat_cache_entry;
    pub type cond_match_t;
    pub type cond_cache_t;
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_copy_string_len_lc(b: *mut buffer, s: *const libc::c_char, len: size_t);
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
    fn buffer_urldecode_path(b: *mut buffer);
    fn buffer_path_simplify(b: *mut buffer);
    fn get_http_method_key(s: *const libc::c_char, slen: size_t) -> http_method_t;
    fn http_method_buf(i: http_method_t) -> *const buffer;
    fn burl_normalize(b: *mut buffer, t: *mut buffer, flags: libc::c_int) -> libc::c_int;
    fn http_header_hkey_get(s: *const libc::c_char, slen: size_t) -> http_header_e;
    fn http_header_hkey_get_lc(s: *const libc::c_char, slen: size_t) -> http_header_e;
    fn http_header_str_contains_token(
        s: *const libc::c_char,
        slen: uint32_t,
        m: *const libc::c_char,
        mlen: uint32_t,
    ) -> libc::c_int;
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
    fn http_header_request_unset(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    );
    fn http_header_request_append(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
        v: *const libc::c_char,
        vlen: uint32_t,
    );
    fn log_error(
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sock_addr_inet_ntop(
        saddr: *const sock_addr,
        buf: *mut libc::c_char,
        sz: socklen_t,
    ) -> *const libc::c_char;
    fn sock_addr_inet_pton(
        saddr: *mut sock_addr,
        str: *const libc::c_char,
        family: libc::c_int,
        port: libc::c_ushort,
    ) -> libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn sock_addr_inet_ntop_copy_buffer(
        b: *mut buffer,
        saddr: *const sock_addr,
    ) -> libc::c_int;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type off_t = __off64_t;
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
pub type uint_fast32_t = libc::c_ulong;
pub type intmax_t = __intmax_t;
pub type unix_timespec64_t = timespec;
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
    pub __in6_u: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type log_error_st = fdlog_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_header_parse_ctx {
    pub k: *mut libc::c_char,
    pub v: *mut libc::c_char,
    pub klen: uint32_t,
    pub vlen: uint32_t,
    pub hlen: uint32_t,
    pub pseudo: uint8_t,
    pub scheme: uint8_t,
    pub trailers: uint8_t,
    pub id: int8_t,
    pub max_request_field_size: uint32_t,
    pub http_parseopts: libc::c_uint,
}
pub const HTTP_PARSEOPT_URL_NORMALIZE_CTRLS_REJECT: burl_opts_e = 64;
pub const HTTP_PARSEOPT_HEADER_STRICT: burl_opts_e = 1;
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
pub const HTTP_HEADER_H2_UNKNOWN: http_header_h2_e = -1;
pub const HTTP_HEADER_H2_SCHEME_HTTPS: http_header_h2_e = -8;
pub const HTTP_HEADER_H2_SCHEME_HTTP: http_header_h2_e = -7;
pub const HTTP_HEADER_H2_PATH_INDEX_HTML: http_header_h2_e = -6;
pub const HTTP_HEADER_H2_PATH: http_header_h2_e = -5;
pub const HTTP_HEADER_H2_METHOD_POST: http_header_h2_e = -4;
pub const HTTP_HEADER_H2_METHOD_GET: http_header_h2_e = -3;
pub const HTTP_HEADER_H2_AUTHORITY: http_header_h2_e = -2;
pub const HTTP_PARSEOPT_METHOD_GET_BODY: burl_opts_e = 32768;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub s: [libc::c_char; 46],
    pub n: size_t,
}
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub s: [libc::c_char; 16],
    pub n: size_t,
}
pub const HTTP_PARSEOPT_HOST_NORMALIZE: burl_opts_e = 4;
pub const HTTP_PARSEOPT_HOST_STRICT: burl_opts_e = 2;
pub const HTTP_PARSEOPT_URL_NORMALIZE: burl_opts_e = 8;
#[derive(Copy, Clone)]
#[repr(C)]
pub union proto_un {
    pub c: [libc::c_char; 8],
    pub u: uint64_t,
}
pub type burl_opts_e = libc::c_uint;
pub const HTTP_PARSEOPT_URL_NORMALIZE_QUERY_20_PLUS: burl_opts_e = 4096;
pub const HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REJECT: burl_opts_e = 2048;
pub const HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REMOVE: burl_opts_e = 1024;
pub const HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_REJECT: burl_opts_e = 512;
pub const HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_DECODE: burl_opts_e = 256;
pub const HTTP_PARSEOPT_URL_NORMALIZE_PATH_BACKSLASH_TRANS: burl_opts_e = 128;
pub const HTTP_PARSEOPT_URL_NORMALIZE_REQUIRED: burl_opts_e = 32;
pub const HTTP_PARSEOPT_URL_NORMALIZE_UNRESERVED: burl_opts_e = 16;
pub type http_header_h2_e = libc::c_int;
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
unsafe extern "C" fn light_isxdigit(mut c: libc::c_int) -> libc::c_int {
    return (light_isdigit(c) != 0
        || (c as uint32_t | 0x20 as libc::c_int as libc::c_uint)
            .wrapping_sub('a' as i32 as libc::c_uint)
            <= ('f' as i32 - 'a' as i32) as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn light_isalpha(mut c: libc::c_int) -> libc::c_int {
    return ((c as uint32_t | 0x20 as libc::c_int as libc::c_uint)
        .wrapping_sub('a' as i32 as libc::c_uint)
        <= ('z' as i32 - 'a' as i32) as libc::c_uint) as libc::c_int;
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
unsafe extern "C" fn buffer_truncate(mut b: *mut buffer, mut len: uint32_t) {
    *((*b).ptr).offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    (*b).used = len.wrapping_add(1 as libc::c_int as libc::c_uint);
}
#[inline(never)]
unsafe extern "C" fn http_request_check_uri_strict(
    s: *const uint8_t,
    len: uint_fast32_t,
) -> *const libc::c_char {
    let mut i: uint_fast32_t = 0 as libc::c_int as uint_fast32_t;
    while i < len {
        if (*s.offset(i as isize) as libc::c_int <= 32 as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
            return (s as *const libc::c_char).offset(i as isize);
        }
        if (*s.offset(i as isize) as libc::c_int == 127 as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
            return (s as *const libc::c_char).offset(i as isize);
        }
        if (*s.offset(i as isize) as libc::c_int == 255 as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
            return (s as *const libc::c_char).offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn http_request_check_line_strict(
    s: *const libc::c_char,
    len: uint_fast32_t,
) -> *const libc::c_char {
    let mut i: uint_fast32_t = 0 as libc::c_int as uint_fast32_t;
    while i < len {
        if ((*(s as *const uint8_t).offset(i as isize) as libc::c_int)
            < 32 as libc::c_int) as libc::c_int as libc::c_long != 0
            && *s.offset(i as isize) as libc::c_int != '\t' as i32
        {
            return s.offset(i as isize);
        }
        if (*s.offset(i as isize) as libc::c_int == 127 as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
            return s.offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn http_request_check_line_minimal(
    s: *const libc::c_char,
    len: uint_fast32_t,
) -> *const libc::c_char {
    let mut i: uint_fast32_t = 0 as libc::c_int as uint_fast32_t;
    while i < len {
        if (*s.offset(i as isize) as libc::c_int == '\u{0}' as i32) as libc::c_int
            as libc::c_long != 0
        {
            return s.offset(i as isize);
        }
        if (*s.offset(i as isize) as libc::c_int == '\n' as i32) as libc::c_int
            as libc::c_long != 0
        {
            return s.offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn request_check_hostname(host: *mut buffer) -> libc::c_int {
    let mut h: *const libc::c_char = (*host).ptr;
    if *h as libc::c_int != '[' as i32 {
        let mut len: uint32_t = buffer_clen(host);
        let colon: *const libc::c_char = memchr(
            h as *const libc::c_void,
            ':' as i32,
            len as libc::c_ulong,
        ) as *const libc::c_char;
        let mut hlen: uint32_t = if !colon.is_null() {
            colon.offset_from(h) as libc::c_long as uint32_t
        } else {
            len
        };
        if (0 as libc::c_int as libc::c_uint == hlen) as libc::c_int as libc::c_long != 0
        {
            return -(1 as libc::c_int);
        }
        if (*h.offset(hlen.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
            as libc::c_int == '.' as i32) as libc::c_int as libc::c_long != 0
        {
            hlen = hlen.wrapping_sub(1);
            if hlen == 0 as libc::c_int as libc::c_uint {
                return -(1 as libc::c_int);
            }
            len = len.wrapping_sub(1);
            if !colon.is_null() {
                memmove(
                    ((*host).ptr).offset(hlen as isize) as *mut libc::c_void,
                    colon as *const libc::c_void,
                    len.wrapping_sub(hlen) as libc::c_ulong,
                );
            }
            buffer_truncate(host, len);
        }
        let mut label_len: libc::c_int = 0 as libc::c_int;
        let mut allnumeric: libc::c_int = 1 as libc::c_int;
        let mut numeric: libc::c_int = 1 as libc::c_int;
        let mut level: libc::c_int = 0 as libc::c_int;
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while i < hlen {
            let ch: libc::c_int = *h.offset(i as isize) as libc::c_int;
            label_len += 1;
            if !(light_isdigit(ch) != 0) {
                if light_isalpha(ch) != 0
                    || ch == '-' as i32 && i != 0 as libc::c_int as libc::c_uint
                {
                    numeric = 0 as libc::c_int;
                } else if ch == '.' as i32 && 1 as libc::c_int != label_len
                        && '-' as i32
                            != *h
                                .offset(
                                    i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                                ) as libc::c_int
                    {
                    allnumeric &= numeric;
                    numeric = 1 as libc::c_int;
                    label_len = 0 as libc::c_int;
                    level += 1;
                } else {
                    return -(1 as libc::c_int)
                }
            }
            i = i.wrapping_add(1);
        }
        if 0 as libc::c_int == label_len
            || numeric != 0 && (level != 3 as libc::c_int || allnumeric == 0)
        {
            return -(1 as libc::c_int);
        }
        h = h.offset(hlen as isize);
    } else {
        h = h.offset(1);
        let mut cnt: libc::c_int = 0 as libc::c_int;
        while light_isxdigit(*h as libc::c_int) != 0 || *h as libc::c_int == '.' as i32
            || *h as libc::c_int == ':' as i32
                && {
                    cnt += 1;
                    cnt < 8 as libc::c_int
                }
        {
            h = h.offset(1);
        }
        if *h as libc::c_int != ']' as i32
            || h.offset_from((*host).ptr) as libc::c_long
                == 1 as libc::c_int as libc::c_long
        {
            return -(1 as libc::c_int);
        }
        h = h.offset(1);
    }
    if *h as libc::c_int == ':' as i32 {
        if (*h.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32)
            as libc::c_int as libc::c_long != 0
        {
            buffer_truncate(
                host,
                h.offset_from((*host).ptr) as libc::c_long as uint32_t,
            );
        }
        loop {
            h = h.offset(1);
            if !(light_isdigit(*h as libc::c_int) != 0) {
                break;
            }
        }
    }
    return if *h as libc::c_int == '\u{0}' as i32 {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn http_request_host_normalize(
    b: *mut buffer,
    scheme_port: libc::c_int,
) -> libc::c_int {
    let p: *const libc::c_char = (*b).ptr;
    let blen: size_t = buffer_clen(b) as size_t;
    let mut port: libc::c_long = 0 as libc::c_int as libc::c_long;
    if *p as libc::c_int != '[' as i32 {
        let colon: *mut libc::c_char = memchr(p as *const libc::c_void, ':' as i32, blen)
            as *mut libc::c_char;
        if !colon.is_null() {
            if *p as libc::c_int == ':' as i32 {
                return -(1 as libc::c_int);
            }
            if *colon.offset(1 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
            {
                let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
                port = strtol(
                    colon.offset(1 as libc::c_int as isize),
                    &mut e,
                    0 as libc::c_int,
                );
                if (0 as libc::c_int as libc::c_long) < port
                    && port
                        <= (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                            as libc::c_long && *e as libc::c_int == '\u{0}' as i32
                {} else {
                    return -(1 as libc::c_int)
                }
            }
            buffer_truncate(
                b,
                colon.offset_from(p) as libc::c_long as size_t as uint32_t,
            );
        }
        if light_isdigit(*p as libc::c_int) != 0 {
            static mut laddr: C2RustUnnamed_5 = C2RustUnnamed_5 {
                s: [0; 16],
                n: 0,
            };
            let mut n: size_t = if !colon.is_null() {
                colon.offset_from(p) as libc::c_long as size_t
            } else {
                blen
            };
            let mut addr: sock_addr = sock_addr {
                ipv6: sockaddr_in6 {
                    sin6_family: 0,
                    sin6_port: 0,
                    sin6_flowinfo: 0,
                    sin6_addr: in6_addr {
                        __in6_u: C2RustUnnamed_3 {
                            __u6_addr8: [0; 16],
                        },
                    },
                    sin6_scope_id: 0,
                },
            };
            if !(n == laddr.n
                && 0 as libc::c_int
                    == memcmp(
                        p as *const libc::c_void,
                        (laddr.s).as_mut_ptr() as *const libc::c_void,
                        n,
                    ))
            {
                if 1 as libc::c_int
                    == sock_addr_inet_pton(
                        &mut addr,
                        p,
                        2 as libc::c_int,
                        0 as libc::c_int as libc::c_ushort,
                    )
                {
                    sock_addr_inet_ntop_copy_buffer(b, &mut addr);
                    n = buffer_clen(b) as size_t;
                    if n < ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong {
                        laddr.n = n;
                        memcpy(
                            (laddr.s).as_mut_ptr() as *mut libc::c_void,
                            (*b).ptr as *const libc::c_void,
                            laddr.n,
                        );
                    }
                }
            }
        }
    } else {
        static mut laddr_0: C2RustUnnamed_4 = C2RustUnnamed_4 {
            s: [0; 46],
            n: 0,
        };
        let mut addr_0: sock_addr = sock_addr {
            ipv6: sockaddr_in6 {
                sin6_family: 0,
                sin6_port: 0,
                sin6_flowinfo: 0,
                sin6_addr: in6_addr {
                    __in6_u: C2RustUnnamed_3 {
                        __u6_addr8: [0; 16],
                    },
                },
                sin6_scope_id: 0,
            },
        };
        let mut bracket: *mut libc::c_char = ((*b).ptr)
            .offset(blen as isize)
            .offset(-(1 as libc::c_int as isize));
        let mut percent: *mut libc::c_char = strchr(
            ((*b).ptr).offset(1 as libc::c_int as isize),
            '%' as i32,
        );
        let mut len: size_t = 0;
        let mut rc: libc::c_int = 0;
        let mut buf: [libc::c_char; 62] = [0; 62];
        if blen <= 2 as libc::c_int as libc::c_ulong {
            return -(1 as libc::c_int);
        }
        if *bracket as libc::c_int != ']' as i32 {
            bracket = memchr(
                ((*b).ptr).offset(1 as libc::c_int as isize) as *const libc::c_void,
                ']' as i32,
                blen.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            if bracket.is_null()
                || *bracket.offset(1 as libc::c_int as isize) as libc::c_int
                    != ':' as i32
                || bracket.offset_from((*b).ptr) as libc::c_long
                    == 1 as libc::c_int as libc::c_long
            {
                return -(1 as libc::c_int);
            }
            if *bracket.offset(2 as libc::c_int as isize) as libc::c_int
                != '\u{0}' as i32
            {
                let mut e_0: *mut libc::c_char = 0 as *mut libc::c_char;
                port = strtol(
                    bracket.offset(2 as libc::c_int as isize),
                    &mut e_0,
                    0 as libc::c_int,
                );
                if (0 as libc::c_int as libc::c_long) < port
                    && port
                        <= (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                            as libc::c_long && *e_0 as libc::c_int == '\u{0}' as i32
                {} else {
                    return -(1 as libc::c_int)
                }
            }
        }
        len = (if !percent.is_null() { percent } else { bracket })
            .offset_from(((*b).ptr).offset(1 as libc::c_int as isize)) as libc::c_long
            as size_t;
        if laddr_0.n == len
            && 0 as libc::c_int
                == memcmp(
                    (laddr_0.s).as_mut_ptr() as *const libc::c_void,
                    ((*b).ptr).offset(1 as libc::c_int as isize) as *const libc::c_void,
                    len,
                )
        {
            buffer_truncate(
                b,
                (bracket.offset_from((*b).ptr) as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as size_t as uint32_t,
            );
        } else {
            *bracket = '\u{0}' as i32 as libc::c_char;
            if !percent.is_null() {
                *percent = '\u{0}' as i32 as libc::c_char;
            }
            rc = sock_addr_inet_pton(
                &mut addr_0,
                ((*b).ptr).offset(1 as libc::c_int as isize),
                10 as libc::c_int,
                0 as libc::c_int as libc::c_ushort,
            );
            if !percent.is_null() {
                *percent = '%' as i32 as libc::c_char;
            }
            *bracket = ']' as i32 as libc::c_char;
            if 1 as libc::c_int != rc {
                return -(1 as libc::c_int);
            }
            sock_addr_inet_ntop(
                &mut addr_0,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 62]>() as libc::c_ulong as socklen_t,
            );
            len = strlen(buf.as_mut_ptr());
            if !percent.is_null() {
                if percent > bracket {
                    return -(1 as libc::c_int);
                }
                if len
                    .wrapping_add(bracket.offset_from(percent) as libc::c_long as size_t)
                    >= ::std::mem::size_of::<[libc::c_char; 62]>() as libc::c_ulong
                {
                    return -(1 as libc::c_int);
                }
                if len < ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong {
                    laddr_0.n = len;
                    memcpy(
                        (laddr_0.s).as_mut_ptr() as *mut libc::c_void,
                        buf.as_mut_ptr() as *const libc::c_void,
                        laddr_0.n,
                    );
                }
                memcpy(
                    buf.as_mut_ptr().offset(len as isize) as *mut libc::c_void,
                    percent as *const libc::c_void,
                    bracket.offset_from(percent) as libc::c_long as size_t,
                );
                len = (len as libc::c_ulong)
                    .wrapping_add(bracket.offset_from(percent) as libc::c_long as size_t)
                    as size_t as size_t;
            }
            buffer_truncate(b, 1 as libc::c_int as uint32_t);
            buffer_append_str2(
                b,
                buf.as_mut_ptr(),
                len,
                b"]\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
    }
    if 0 as libc::c_int as libc::c_long != port && port != scheme_port as libc::c_long {
        buffer_append_string_len(
            b,
            b":\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        buffer_append_int(b, port as libc::c_int as intmax_t);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn http_request_host_policy(
    b: *mut buffer,
    http_parseopts: libc::c_uint,
    scheme_port: libc::c_int,
) -> libc::c_int {
    return ((if http_parseopts & HTTP_PARSEOPT_HOST_STRICT as libc::c_int as libc::c_uint
        != 0
    {
        (0 as libc::c_int != request_check_hostname(b)) as libc::c_int
    } else {
        (0 as *mut libc::c_void as *const libc::c_char
            != http_request_check_line_minimal(
                (*b).ptr,
                buffer_clen(b) as uint_fast32_t,
            )) as libc::c_int
    }) != 0
        || http_parseopts & HTTP_PARSEOPT_HOST_NORMALIZE as libc::c_int as libc::c_uint
            != 0 && 0 as libc::c_int != http_request_host_normalize(b, scheme_port))
        as libc::c_int;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn http_request_header_line_invalid(
    r: *mut request_st,
    status: libc::c_int,
    msg: *const libc::c_char,
) -> libc::c_int {
    if (*r).conf.log_request_header_on_error != 0 {
        if !msg.is_null() {
            log_error(
                (*r).conf.errh,
                b"src/request.c\0" as *const u8 as *const libc::c_char,
                278 as libc::c_int as libc::c_uint,
                b"%s\0" as *const u8 as *const libc::c_char,
                msg,
            );
        }
    }
    return status;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn http_request_header_char_invalid(
    r: *mut request_st,
    ch: libc::c_char,
    msg: *const libc::c_char,
) -> libc::c_int {
    if (*r).conf.log_request_header_on_error != 0 {
        if ch as libc::c_uchar as libc::c_int > 32 as libc::c_int
            && ch as libc::c_int != 127 as libc::c_int
        {
            log_error(
                (*r).conf.errh,
                b"src/request.c\0" as *const u8 as *const libc::c_char,
                288 as libc::c_int as libc::c_uint,
                b"%s ('%c')\0" as *const u8 as *const libc::c_char,
                msg,
                ch as libc::c_int,
            );
        } else {
            log_error(
                (*r).conf.errh,
                b"src/request.c\0" as *const u8 as *const libc::c_char,
                291 as libc::c_int as libc::c_uint,
                b"%s (0x%x)\0" as *const u8 as *const libc::c_char,
                msg,
                ch as libc::c_int,
            );
        }
    }
    return 400 as libc::c_int;
}
#[inline(never)]
unsafe extern "C" fn http_request_header_set_Host(
    r: *mut request_st,
    h: *const libc::c_char,
    mut hlen: size_t,
) {
    (*r)
        .http_host = http_header_request_set_ptr(
        r,
        HTTP_HEADER_HOST,
        b"Host\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    buffer_copy_string_len_lc((*r).http_host, h, hlen);
}
#[no_mangle]
pub unsafe extern "C" fn li_restricted_strtoint64(
    mut v: *const libc::c_char,
    vlen: uint32_t,
    err: *mut *const libc::c_char,
) -> int64_t {
    let mut rv: int64_t = 0 as libc::c_int as int64_t;
    let mut i: uint32_t = 0;
    i = 0 as libc::c_int as uint32_t;
    while i < vlen {
        let c: uint8_t = (*(v as *mut uint8_t).offset(i as isize) as libc::c_int
            - '0' as i32) as uint8_t;
        if c as libc::c_int > 9 as libc::c_int {
            break;
        }
        if rv > 9223372036854775807 as libc::c_long / 10 as libc::c_int as libc::c_long {
            break;
        }
        rv *= 10 as libc::c_int as libc::c_long;
        if rv > 9223372036854775807 as libc::c_long - c as libc::c_long {
            break;
        }
        rv += c as libc::c_long;
        i = i.wrapping_add(1);
    }
    *err = v.offset(i as isize);
    return rv;
}
#[cold]
unsafe extern "C" fn http_request_parse_duplicate(
    r: *mut request_st,
    id: http_header_e,
    k: *const libc::c_char,
    klen: size_t,
    v: *const libc::c_char,
    vlen: size_t,
) -> libc::c_int {
    let vb: *const buffer = http_header_request_get(r, id, k, klen as uint32_t);
    if !vb.is_null() && buffer_eq_icase_slen(vb, v, vlen) != 0 {
        return 0 as libc::c_int;
    }
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    match id as libc::c_uint {
        27 => {
            errmsg = b"duplicate Host header -> 400\0" as *const u8
                as *const libc::c_char;
        }
        18 => {
            errmsg = b"duplicate Content-Type header -> 400\0" as *const u8
                as *const libc::c_char;
        }
        30 => {
            errmsg = b"duplicate If-Modified-Since header -> 400\0" as *const u8
                as *const libc::c_char;
        }
        28 => {
            errmsg = b"duplicate HTTP2-Settings header -> 400\0" as *const u8
                as *const libc::c_char;
        }
        31 => return 0 as libc::c_int,
        _ => {
            errmsg = b"duplicate header -> 400\0" as *const u8 as *const libc::c_char;
        }
    }
    return http_request_header_line_invalid(r, 400 as libc::c_int, errmsg);
}
unsafe extern "C" fn http_request_parse_single_header(
    r: *mut request_st,
    id: http_header_e,
    k: *const libc::c_char,
    klen: size_t,
    v: *const libc::c_char,
    vlen: size_t,
) -> libc::c_int {
    let mut current_block_32: u64;
    match id as libc::c_uint {
        27 => {
            if (*r).rqst_htags & (1 as libc::c_ulong) << HTTP_HEADER_HOST as libc::c_int
                == 0
            {
                if vlen >= 1024 as libc::c_int as libc::c_ulong {
                    return http_request_header_line_invalid(
                        r,
                        400 as libc::c_int,
                        b"uri-authority too long -> 400\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                http_request_header_set_Host(r, v, vlen);
                return 0 as libc::c_int;
            } else {
                if !((*r).http_host).is_null()
                    && buffer_eq_slen((*r).http_host, v, vlen) as libc::c_long != 0
                {
                    return 0 as libc::c_int;
                }
            }
            current_block_32 = 18416564117582698220;
        }
        30 | 31 | 18 | 28 => {
            current_block_32 = 18416564117582698220;
        }
        12 => {
            if vlen == 5 as libc::c_int as libc::c_ulong
                && buffer_eq_icase_ssn(
                    v,
                    b"close\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                ) != 0
                || http_header_str_contains_token(
                    v,
                    vlen as uint32_t,
                    b"close\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                ) != 0
            {
                (*r).keep_alive = 0 as libc::c_int as int8_t;
            } else if http_header_str_contains_token(
                    v,
                    vlen as uint32_t,
                    b"keep-alive\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                ) != 0
                {
                (*r).keep_alive = 1 as libc::c_int as int8_t;
            }
            current_block_32 = 15597372965620363352;
        }
        14 => {
            if (*r).rqst_htags
                & (1 as libc::c_ulong) << HTTP_HEADER_CONTENT_LENGTH as libc::c_int == 0
            {
                let mut err: *const libc::c_char = 0 as *const libc::c_char;
                let mut clen: off_t = li_restricted_strtoint64(
                    v,
                    vlen as uint32_t,
                    &mut err,
                );
                if err == v.offset(vlen as isize) {
                    if 0 as libc::c_int as libc::c_long == (*r).reqbody_length {
                        (*r).reqbody_length = clen;
                    }
                } else {
                    return http_request_header_line_invalid(
                        r,
                        400 as libc::c_int,
                        b"invalid Content-Length header -> 400\0" as *const u8
                            as *const libc::c_char,
                    )
                }
            } else {
                return http_request_header_line_invalid(
                    r,
                    400 as libc::c_int,
                    b"duplicate Content-Length header -> 400\0" as *const u8
                        as *const libc::c_char,
                )
            }
            current_block_32 = 15597372965620363352;
        }
        48 => {
            if HTTP_VERSION_1_1 as libc::c_int != (*r).http_version as libc::c_int {
                return http_request_header_line_invalid(
                    r,
                    400 as libc::c_int,
                    if HTTP_VERSION_1_0 as libc::c_int
                        == (*r).http_version as libc::c_int
                    {
                        b"HTTP/1.0 with Transfer-Encoding (bad HTTP/1.0 proxy?) -> 400\0"
                            as *const u8 as *const libc::c_char
                    } else {
                        b"HTTP/2 with Transfer-Encoding is invalid -> 400\0" as *const u8
                            as *const libc::c_char
                    },
                );
            }
            if buffer_eq_icase_ss(
                v,
                vlen,
                b"chunked\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) == 0
            {
                return http_request_header_line_invalid(
                    r,
                    501 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            (*r).reqbody_length = -(1 as libc::c_int) as off_t;
            return 0 as libc::c_int;
        }
        _ => {
            current_block_32 = 15597372965620363352;
        }
    }
    match current_block_32 {
        18416564117582698220 => {
            if (*r).rqst_htags & (1 as libc::c_ulong) << id as libc::c_uint != 0 {
                return http_request_parse_duplicate(r, id, k, klen, v, vlen);
            }
        }
        _ => {}
    }
    http_header_request_append(r, id, k, klen as uint32_t, v, vlen as uint32_t);
    return 0 as libc::c_int;
}
#[cold]
unsafe extern "C" fn http_request_parse_proto_loose(
    r: *mut request_st,
    ptr: *const libc::c_char,
    len: size_t,
    http_parseopts: libc::c_uint,
) -> libc::c_int {
    let mut proto: *const libc::c_char = memchr(
        ptr as *const libc::c_void,
        ' ' as i32,
        len,
    ) as *const libc::c_char;
    if proto.is_null() {
        return http_request_header_line_invalid(
            r,
            400 as libc::c_int,
            b"incomplete request line -> 400\0" as *const u8 as *const libc::c_char,
        );
    }
    proto = memchr(
        proto.offset(1 as libc::c_int as isize) as *const libc::c_void,
        ' ' as i32,
        len
            .wrapping_sub(
                proto.offset(1 as libc::c_int as isize).offset_from(ptr) as libc::c_long
                    as libc::c_ulong,
            ),
    ) as *const libc::c_char;
    if proto.is_null() {
        return http_request_header_line_invalid(
            r,
            400 as libc::c_int,
            b"incomplete request line -> 400\0" as *const u8 as *const libc::c_char,
        );
    }
    proto = proto.offset(1);
    if *proto.offset(0 as libc::c_int as isize) as libc::c_int == 'H' as i32
        && *proto.offset(1 as libc::c_int as isize) as libc::c_int == 'T' as i32
        && *proto.offset(2 as libc::c_int as isize) as libc::c_int == 'T' as i32
        && *proto.offset(3 as libc::c_int as isize) as libc::c_int == 'P' as i32
        && *proto.offset(4 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        if *proto.offset(5 as libc::c_int as isize) as libc::c_int == '1' as i32
            && *proto.offset(6 as libc::c_int as isize) as libc::c_int == '.' as i32
            && (*proto.offset(7 as libc::c_int as isize) as libc::c_int == '1' as i32
                || *proto.offset(7 as libc::c_int as isize) as libc::c_int == '0' as i32)
        {
            if http_parseopts
                & HTTP_PARSEOPT_HEADER_STRICT as libc::c_int as libc::c_uint != 0
            {
                return http_request_header_line_invalid(
                    r,
                    400 as libc::c_int,
                    b"incomplete request line -> 400\0" as *const u8
                        as *const libc::c_char,
                );
            }
            (*r)
                .http_version = (if *proto.offset(7 as libc::c_int as isize)
                as libc::c_int == '1' as i32
            {
                HTTP_VERSION_1_1 as libc::c_int
            } else {
                HTTP_VERSION_1_0 as libc::c_int
            }) as http_version_t;
        } else {
            return http_request_header_line_invalid(
                r,
                505 as libc::c_int,
                b"unknown HTTP version -> 505\0" as *const u8 as *const libc::c_char,
            )
        }
    } else {
        return http_request_header_line_invalid(
            r,
            400 as libc::c_int,
            b"unknown protocol -> 400\0" as *const u8 as *const libc::c_char,
        )
    }
    (*r)
        .keep_alive = (HTTP_VERSION_1_0 as libc::c_int
        != (*r).http_version as libc::c_int) as libc::c_int as int8_t;
    return 0 as libc::c_int;
}
#[cold]
unsafe extern "C" fn http_request_parse_reqline_uri(
    r: *mut request_st,
    uri: *const libc::c_char,
    len: size_t,
    http_parseopts: libc::c_uint,
) -> *const libc::c_char {
    let mut nuri: *const libc::c_char = 0 as *const libc::c_char;
    if len > 7 as libc::c_int as libc::c_ulong
        && buffer_eq_icase_ssn(
            uri,
            b"http://\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as size_t,
        ) != 0
        && {
            nuri = memchr(
                uri.offset(7 as libc::c_int as isize) as *const libc::c_void,
                '/' as i32,
                len.wrapping_sub(7 as libc::c_int as libc::c_ulong),
            ) as *const libc::c_char;
            !nuri.is_null()
        }
        || len > 8 as libc::c_int as libc::c_ulong
            && buffer_eq_icase_ssn(
                uri,
                b"https://\0" as *const u8 as *const libc::c_char,
                8 as libc::c_int as size_t,
            ) != 0
            && {
                nuri = memchr(
                    uri.offset(8 as libc::c_int as isize) as *const libc::c_void,
                    '/' as i32,
                    len.wrapping_sub(8 as libc::c_int as libc::c_ulong),
                ) as *const libc::c_char;
                !nuri.is_null()
            }
    {
        let host: *const libc::c_char = uri
            .offset(
                (if *uri.offset(4 as libc::c_int as isize) as libc::c_int == ':' as i32 {
                    7 as libc::c_int
                } else {
                    8 as libc::c_int
                }) as isize,
            );
        let hostlen: size_t = nuri.offset_from(host) as libc::c_long as size_t;
        if 0 as libc::c_int as libc::c_ulong == hostlen
            || hostlen >= 1024 as libc::c_int as libc::c_ulong
        {
            http_request_header_line_invalid(
                r,
                400 as libc::c_int,
                b"uri-authority empty or too long -> 400\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as *const libc::c_char;
        }
        http_request_header_set_Host(r, host, hostlen);
        return nuri;
    } else if http_parseopts & HTTP_PARSEOPT_HEADER_STRICT as libc::c_int as libc::c_uint
            == 0
            || HTTP_METHOD_CONNECT as libc::c_int == (*r).http_method as libc::c_int
                && (*uri.offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32
                    || light_isdigit(
                        *uri.offset(0 as libc::c_int as isize) as libc::c_int,
                    ) != 0)
            || HTTP_METHOD_OPTIONS as libc::c_int == (*r).http_method as libc::c_int
                && *uri.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32
                && 1 as libc::c_int as libc::c_ulong == len
        {
        return uri
    } else {
        http_request_header_line_invalid(
            r,
            400 as libc::c_int,
            b"request-URI parse error -> 400\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *const libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn http_request_validate_pseudohdrs(
    r: *mut request_st,
    scheme: libc::c_int,
    http_parseopts: libc::c_uint,
) -> libc::c_int {
    if HTTP_METHOD_UNSET as libc::c_int == (*r).http_method as libc::c_int {
        return http_request_header_line_invalid(
            r,
            400 as libc::c_int,
            b"missing pseudo-header method -> 400\0" as *const u8 as *const libc::c_char,
        );
    }
    if (HTTP_METHOD_CONNECT as libc::c_int != (*r).http_method as libc::c_int)
        as libc::c_int as libc::c_long != 0
    {
        if scheme == 0 {
            return http_request_header_line_invalid(
                r,
                400 as libc::c_int,
                b"missing pseudo-header scheme -> 400\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if buffer_is_blank(&mut (*r).target) != 0 {
            return http_request_header_line_invalid(
                r,
                400 as libc::c_int,
                b"missing pseudo-header path -> 400\0" as *const u8
                    as *const libc::c_char,
            );
        }
        let uri: *const libc::c_char = (*r).target.ptr;
        if *uri as libc::c_int != '/' as i32 {
            if *uri.offset(0 as libc::c_int as isize) as libc::c_int != '*' as i32
                || *uri.offset(1 as libc::c_int as isize) as libc::c_int
                    != '\u{0}' as i32
                || HTTP_METHOD_OPTIONS as libc::c_int != (*r).http_method as libc::c_int
            {
                return http_request_header_line_invalid(
                    r,
                    400 as libc::c_int,
                    b"invalid pseudo-header path -> 400\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
    } else {
        if ((*r).http_host).is_null() {
            return http_request_header_line_invalid(
                r,
                400 as libc::c_int,
                b"missing pseudo-header authority -> 400\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if buffer_is_blank(&mut (*r).target) == 0 || scheme != 0 {
            return http_request_header_line_invalid(
                r,
                400 as libc::c_int,
                b"invalid pseudo-header with CONNECT -> 400\0" as *const u8
                    as *const libc::c_char,
            );
        }
        buffer_copy_buffer(&mut (*r).target, (*r).http_host);
    }
    buffer_copy_buffer(&mut (*r).target_orig, &mut (*r).target);
    let len: uint32_t = buffer_clen(&mut (*r).target);
    let x: *const libc::c_char = if http_parseopts
        & HTTP_PARSEOPT_HEADER_STRICT as libc::c_int as libc::c_uint != 0
    {
        if http_parseopts
            & HTTP_PARSEOPT_URL_NORMALIZE_CTRLS_REJECT as libc::c_int as libc::c_uint
            != 0
        {
            0 as *const libc::c_char
        } else {
            http_request_check_uri_strict(
                (*r).target.ptr as *const uint8_t,
                len as uint_fast32_t,
            )
        }
    } else {
        http_request_check_line_minimal((*r).target.ptr, len as uint_fast32_t)
    };
    return if x.is_null() {
        0 as libc::c_int
    } else {
        http_request_header_char_invalid(
            r,
            *x,
            b"invalid character in URI -> 400\0" as *const u8 as *const libc::c_char,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn http_request_parse_header(
    r: *mut request_st,
    hpctx: *mut http_header_parse_ctx,
) -> libc::c_int {
    let k: *const libc::c_char = (*hpctx).k;
    let v: *const libc::c_char = (*hpctx).v;
    let klen: uint32_t = (*hpctx).klen;
    let vlen: uint32_t = (*hpctx).vlen;
    if 0 as libc::c_int as libc::c_uint == klen {
        return http_request_header_line_invalid(
            r,
            400 as libc::c_int,
            b"invalid header key -> 400\0" as *const u8 as *const libc::c_char,
        );
    }
    (*hpctx)
        .hlen = ((*hpctx).hlen as libc::c_uint)
        .wrapping_add(
            klen.wrapping_add(vlen).wrapping_add(4 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    if (*hpctx).hlen > (*hpctx).max_request_field_size {
        log_error(
            (*r).conf.errh,
            b"src/request.c\0" as *const u8 as *const libc::c_char,
            604 as libc::c_int as libc::c_uint,
            b"oversized request header -> 431\0" as *const u8 as *const libc::c_char,
        );
        return 431 as libc::c_int;
    }
    if (*hpctx).trailers == 0 {
        if *k as libc::c_int == ':' as i32 {
            if (*hpctx).pseudo == 0 {
                return http_request_header_line_invalid(
                    r,
                    400 as libc::c_int,
                    b"invalid pseudo-header -> 400\0" as *const u8 as *const libc::c_char,
                );
            }
            if 0 as libc::c_int as libc::c_uint == vlen {
                return http_request_header_line_invalid(
                    r,
                    400 as libc::c_int,
                    b"invalid header value -> 400\0" as *const u8 as *const libc::c_char,
                );
            }
            if ((*hpctx).id as libc::c_int == HTTP_HEADER_H2_UNKNOWN as libc::c_int)
                as libc::c_int as libc::c_long != 0
            {
                match klen.wrapping_sub(1 as libc::c_int as libc::c_uint) {
                    4 => {
                        if 0 as libc::c_int
                            == memcmp(
                                k.offset(1 as libc::c_int as isize) as *const libc::c_void,
                                b"path\0" as *const u8 as *const libc::c_char
                                    as *const libc::c_void,
                                4 as libc::c_int as libc::c_ulong,
                            )
                        {
                            (*hpctx).id = HTTP_HEADER_H2_PATH as libc::c_int as int8_t;
                        }
                    }
                    6 => {
                        if 0 as libc::c_int
                            == memcmp(
                                k.offset(1 as libc::c_int as isize) as *const libc::c_void,
                                b"method\0" as *const u8 as *const libc::c_char
                                    as *const libc::c_void,
                                6 as libc::c_int as libc::c_ulong,
                            )
                        {
                            (*hpctx)
                                .id = HTTP_HEADER_H2_METHOD_GET as libc::c_int as int8_t;
                        } else if 0 as libc::c_int
                                == memcmp(
                                    k.offset(1 as libc::c_int as isize) as *const libc::c_void,
                                    b"scheme\0" as *const u8 as *const libc::c_char
                                        as *const libc::c_void,
                                    6 as libc::c_int as libc::c_ulong,
                                )
                            {
                            (*hpctx)
                                .id = HTTP_HEADER_H2_SCHEME_HTTP as libc::c_int as int8_t;
                        }
                    }
                    9 => {
                        if 0 as libc::c_int
                            == memcmp(
                                k.offset(1 as libc::c_int as isize) as *const libc::c_void,
                                b"authority\0" as *const u8 as *const libc::c_char
                                    as *const libc::c_void,
                                9 as libc::c_int as libc::c_ulong,
                            )
                        {
                            (*hpctx)
                                .id = HTTP_HEADER_H2_AUTHORITY as libc::c_int as int8_t;
                        }
                    }
                    _ => {}
                }
                if (*hpctx).id as libc::c_int >= HTTP_HEADER_H2_UNKNOWN as libc::c_int {
                    return http_request_header_line_invalid(
                        r,
                        400 as libc::c_int,
                        b"invalid pseudo-header -> 400\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            match (*hpctx).id as libc::c_int {
                -2 => {
                    if !(((*r).http_host != 0 as *mut libc::c_void as *mut buffer)
                        as libc::c_int as libc::c_long != 0)
                    {
                        if vlen >= 1024 as libc::c_int as libc::c_uint {
                            return http_request_header_line_invalid(
                                r,
                                400 as libc::c_int,
                                b"invalid pseudo-header authority too long -> 400\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        http_request_header_set_Host(r, v, vlen as size_t);
                        return 0 as libc::c_int;
                    }
                }
                -3 | -4 => {
                    if !((HTTP_METHOD_UNSET as libc::c_int
                        != (*r).http_method as libc::c_int) as libc::c_int
                        as libc::c_long != 0)
                    {
                        (*r).http_method = get_http_method_key(v, vlen as size_t);
                        if HTTP_METHOD_UNSET as libc::c_int
                            >= (*r).http_method as libc::c_int
                        {
                            return http_request_header_line_invalid(
                                r,
                                501 as libc::c_int,
                                b"unknown http-method -> 501\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        return 0 as libc::c_int;
                    }
                }
                -5 | -6 => {
                    if !((buffer_is_blank(&mut (*r).target) == 0) as libc::c_int
                        as libc::c_long != 0)
                    {
                        buffer_copy_string_len(&mut (*r).target, v, vlen as size_t);
                        return 0 as libc::c_int;
                    }
                }
                -7 | -8 => {
                    if !((*hpctx).scheme as libc::c_long != 0) {
                        (*hpctx).scheme = 1 as libc::c_int as uint8_t;
                        return 0 as libc::c_int;
                    }
                }
                _ => {
                    return http_request_header_line_invalid(
                        r,
                        400 as libc::c_int,
                        b"invalid pseudo-header -> 400\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            return http_request_header_line_invalid(
                r,
                400 as libc::c_int,
                b"repeated pseudo-header -> 400\0" as *const u8 as *const libc::c_char,
            );
        } else {
            if (*hpctx).pseudo != 0 {
                (*hpctx).pseudo = 0 as libc::c_int as uint8_t;
                let mut status: libc::c_int = http_request_validate_pseudohdrs(
                    r,
                    (*hpctx).scheme as libc::c_int,
                    (*hpctx).http_parseopts,
                );
                if 0 as libc::c_int != status {
                    return status;
                }
            }
            if 0 as libc::c_int as libc::c_uint == vlen {
                return 0 as libc::c_int;
            }
            let http_header_strict: libc::c_uint = (*hpctx).http_parseopts
                & HTTP_PARSEOPT_HEADER_STRICT as libc::c_int as libc::c_uint;
            let x: *const libc::c_char = if http_header_strict != 0 {
                http_request_check_line_strict(v, vlen as uint_fast32_t)
            } else {
                http_request_check_line_minimal(v, vlen as uint_fast32_t)
            };
            if !x.is_null() {
                return http_request_header_char_invalid(
                    r,
                    *x,
                    b"invalid character in header -> 400\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if ((*hpctx).id as libc::c_int == HTTP_HEADER_H2_UNKNOWN as libc::c_int)
                as libc::c_int as libc::c_long != 0
            {
                let mut j: uint32_t = 0 as libc::c_int as uint32_t;
                while j < klen
                    && ((*k.offset(j as isize) as uint32_t)
                        .wrapping_sub('a' as i32 as libc::c_uint)
                        <= ('z' as i32 - 'a' as i32) as libc::c_uint
                        || *k.offset(j as isize) as libc::c_int == '-' as i32)
                {
                    j = j.wrapping_add(1);
                }
                if (j != klen) as libc::c_int as libc::c_long != 0 {
                    if (*k.offset(j as isize) as uint32_t)
                        .wrapping_sub('A' as i32 as libc::c_uint)
                        <= ('Z' as i32 - 'A' as i32) as libc::c_uint
                    {
                        return 400 as libc::c_int;
                    }
                    if 0 as libc::c_int
                        != http_request_parse_header_other(
                            r,
                            k.offset(j as isize),
                            klen.wrapping_sub(j) as libc::c_int,
                            http_header_strict,
                        )
                    {
                        return 400 as libc::c_int;
                    }
                }
                (*hpctx).id = http_header_hkey_get_lc(k, klen as size_t) as int8_t;
            }
            let id: http_header_e = (*hpctx).id as http_header_e;
            if (id as libc::c_uint == HTTP_HEADER_TE as libc::c_int as libc::c_uint)
                as libc::c_int as libc::c_long != 0
                && buffer_eq_icase_ss(
                    v,
                    vlen as size_t,
                    b"trailers\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                ) == 0
            {
                return http_request_header_line_invalid(
                    r,
                    400 as libc::c_int,
                    b"invalid TE header value with HTTP/2 -> 400\0" as *const u8
                        as *const libc::c_char,
                );
            }
            return http_request_parse_single_header(
                r,
                id,
                k,
                klen as size_t,
                v,
                vlen as size_t,
            );
        }
    } else {
        if *k as libc::c_int == ':' as i32 {
            return http_request_header_line_invalid(
                r,
                400 as libc::c_int,
                b"invalid pseudo-header in trailers -> 400\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn http_request_parse_reqline(
    r: *mut request_st,
    ptr: *const libc::c_char,
    hoff: *const libc::c_ushort,
    http_parseopts: libc::c_uint,
) -> libc::c_int {
    let mut len: size_t = *hoff.offset(2 as libc::c_int as isize) as size_t;
    if len < 13 as libc::c_int as libc::c_ulong {
        return http_request_header_line_invalid(
            r,
            400 as libc::c_int,
            b"invalid request line (too short) -> 400\0" as *const u8
                as *const libc::c_char,
        );
    }
    if *ptr.offset(len.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int == '\r' as i32
    {
        len = (len as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    } else if http_parseopts & HTTP_PARSEOPT_HEADER_STRICT as libc::c_int as libc::c_uint
            == 0
        {
        len = (len as libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    } else {
        return http_request_header_line_invalid(
            r,
            400 as libc::c_int,
            b"missing CR before LF in header -> 400\0" as *const u8
                as *const libc::c_char,
        )
    }
    static mut http_1_1: proto_un = proto_un {
        c: [
            'H' as i32 as libc::c_char,
            'T' as i32 as libc::c_char,
            'T' as i32 as libc::c_char,
            'P' as i32 as libc::c_char,
            '/' as i32 as libc::c_char,
            '1' as i32 as libc::c_char,
            '.' as i32 as libc::c_char,
            '1' as i32 as libc::c_char,
        ],
    };
    static mut http_1_0: proto_un = proto_un {
        c: [
            'H' as i32 as libc::c_char,
            'T' as i32 as libc::c_char,
            'T' as i32 as libc::c_char,
            'P' as i32 as libc::c_char,
            '/' as i32 as libc::c_char,
            '1' as i32 as libc::c_char,
            '.' as i32 as libc::c_char,
            '0' as i32 as libc::c_char,
        ],
    };
    let mut p: *const libc::c_char = ptr
        .offset(len as isize)
        .offset(-(8 as libc::c_int as isize));
    let mut proto8: proto_un = proto_un { c: [0; 8] };
    proto8.c[0 as libc::c_int as usize] = *p.offset(0 as libc::c_int as isize);
    proto8.c[1 as libc::c_int as usize] = *p.offset(1 as libc::c_int as isize);
    proto8.c[2 as libc::c_int as usize] = *p.offset(2 as libc::c_int as isize);
    proto8.c[3 as libc::c_int as usize] = *p.offset(3 as libc::c_int as isize);
    proto8.c[4 as libc::c_int as usize] = *p.offset(4 as libc::c_int as isize);
    proto8.c[5 as libc::c_int as usize] = *p.offset(5 as libc::c_int as isize);
    proto8.c[6 as libc::c_int as usize] = *p.offset(6 as libc::c_int as isize);
    proto8.c[7 as libc::c_int as usize] = *p.offset(7 as libc::c_int as isize);
    if *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == ' ' as i32
        && http_1_1.u == proto8.u
    {
        (*r).http_version = HTTP_VERSION_1_1;
        (*r).keep_alive = 1 as libc::c_int as int8_t;
    } else if *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == ' ' as i32
            && http_1_0.u == proto8.u
        {
        (*r).http_version = HTTP_VERSION_1_0;
        (*r).keep_alive = 0 as libc::c_int as int8_t;
    } else {
        let mut status: libc::c_int = http_request_parse_proto_loose(
            r,
            ptr,
            len,
            http_parseopts,
        );
        if 0 as libc::c_int != status {
            return status;
        }
        p = ptr.offset(len as isize).offset(-(9 as libc::c_int as isize));
        while *p.offset(-(1 as libc::c_int) as isize) as libc::c_int != ' ' as i32 {
            p = p.offset(-1);
        }
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    while *ptr.offset(i as isize) as libc::c_int != ' ' as i32 {
        i = i.wrapping_add(1);
    }
    (*r).http_method = get_http_method_key(ptr, i);
    if HTTP_METHOD_UNSET as libc::c_int >= (*r).http_method as libc::c_int {
        return http_request_header_line_invalid(
            r,
            501 as libc::c_int,
            b"unknown http-method -> 501\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut uri: *const libc::c_char = ptr
        .offset(i as isize)
        .offset(1 as libc::c_int as isize);
    if uri == p {
        return http_request_header_line_invalid(
            r,
            400 as libc::c_int,
            b"no uri specified -> 400\0" as *const u8 as *const libc::c_char,
        );
    }
    len = (p.offset_from(uri) as libc::c_long - 1 as libc::c_int as libc::c_long)
        as size_t;
    if *uri as libc::c_int != '/' as i32 {
        uri = http_request_parse_reqline_uri(r, uri, len, http_parseopts);
        if uri.is_null() {
            return 400 as libc::c_int;
        }
        len = (p.offset_from(uri) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t;
    }
    if 0 as libc::c_int as libc::c_ulong == len {
        return http_request_header_line_invalid(
            r,
            400 as libc::c_int,
            b"no uri specified -> 400\0" as *const u8 as *const libc::c_char,
        );
    }
    let x: *const libc::c_char = (if http_parseopts
        & HTTP_PARSEOPT_HEADER_STRICT as libc::c_int as libc::c_uint != 0
    {
        (if http_parseopts
            & HTTP_PARSEOPT_URL_NORMALIZE_CTRLS_REJECT as libc::c_int as libc::c_uint
            != 0
        {
            0 as *const libc::c_char
        } else {
            http_request_check_uri_strict(uri as *const uint8_t, len)
        }) as *const libc::c_void
    } else {
        memchr(
            ptr as *const libc::c_void,
            '\u{0}' as i32,
            *hoff.offset(*hoff.offset(0 as libc::c_int as isize) as isize)
                as libc::c_ulong,
        )
    }) as *const libc::c_char;
    if !x.is_null() {
        http_request_header_char_invalid(
            r,
            *x,
            b"invalid character in URI -> 400\0" as *const u8 as *const libc::c_char,
        );
    }
    buffer_copy_string_len(&mut (*r).target, uri, len);
    buffer_copy_string_len(&mut (*r).target_orig, uri, len);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn http_request_parse_target(
    r: *mut request_st,
    mut scheme_port: libc::c_int,
) -> libc::c_int {
    buffer_copy_string_len(
        &mut (*r).uri.scheme,
        b"https\0" as *const u8 as *const libc::c_char,
        (if scheme_port == 443 as libc::c_int {
            5 as libc::c_int
        } else {
            4 as libc::c_int
        }) as size_t,
    );
    let target: *mut buffer = &mut (*r).target;
    if (*r).http_method as libc::c_int == HTTP_METHOD_CONNECT as libc::c_int
        || (*r).http_method as libc::c_int == HTTP_METHOD_OPTIONS as libc::c_int
            && *((*target).ptr).offset(0 as libc::c_int as isize) as libc::c_int
                == '*' as i32
            && *((*target).ptr).offset(1 as libc::c_int as isize) as libc::c_int
                == '\u{0}' as i32
    {
        buffer_copy_buffer(&mut (*r).uri.path, target);
        buffer_clear(&mut (*r).uri.query);
        return 0 as libc::c_int;
    }
    let mut qstr: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*r).conf.http_parseopts
        & HTTP_PARSEOPT_URL_NORMALIZE as libc::c_int as libc::c_uint != 0
    {
        let mut qs: libc::c_int = burl_normalize(
            target,
            (*r).tmp_buf,
            (*r).conf.http_parseopts as libc::c_int,
        );
        if -(2 as libc::c_int) == qs {
            return http_request_header_line_invalid(
                r,
                400 as libc::c_int,
                b"invalid character in URI -> 400\0" as *const u8 as *const libc::c_char,
            );
        }
        qstr = if -(1 as libc::c_int) == qs {
            0 as *mut libc::c_char
        } else {
            ((*target).ptr).offset(qs as isize)
        };
    } else {
        let mut rlen: size_t = buffer_clen(target) as size_t;
        qstr = memchr((*target).ptr as *const libc::c_void, '#' as i32, rlen)
            as *mut libc::c_char;
        if !qstr.is_null() {
            rlen = qstr.offset_from((*target).ptr) as libc::c_long as size_t;
            buffer_truncate(target, rlen as uint32_t);
        }
        qstr = memchr((*target).ptr as *const libc::c_void, '?' as i32, rlen)
            as *mut libc::c_char;
    }
    let pstr: *const libc::c_char = (*target).ptr;
    let rlen_0: uint32_t = buffer_clen(target);
    let mut plen: uint32_t = 0;
    if !qstr.is_null() {
        plen = qstr.offset_from(pstr) as libc::c_long as uint32_t;
        buffer_copy_string_len(
            &mut (*r).uri.query,
            qstr.offset(1 as libc::c_int as isize),
            rlen_0.wrapping_sub(plen).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as size_t,
        );
    } else {
        plen = rlen_0;
        buffer_clear(&mut (*r).uri.query);
    }
    buffer_copy_string_len(&mut (*r).uri.path, pstr, plen as size_t);
    buffer_urldecode_path(&mut (*r).uri.path);
    buffer_path_simplify(&mut (*r).uri.path);
    if *((*r).uri.path.ptr).offset(0 as libc::c_int as isize) as libc::c_int
        != '/' as i32
    {
        return http_request_header_line_invalid(
            r,
            400 as libc::c_int,
            b"uri-path does not begin with '/' -> 400\0" as *const u8
                as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn http_request_parse_header_other(
    r: *mut request_st,
    k: *const libc::c_char,
    klen: libc::c_int,
    http_header_strict: libc::c_uint,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < klen {
        if !(light_isalpha(*k.offset(i as isize) as libc::c_int) != 0
            || *k.offset(i as isize) as libc::c_int == '-' as i32)
        {
            match *k.offset(i as isize) as libc::c_int {
                32 | 9 => {
                    return http_request_header_line_invalid(
                        r,
                        400 as libc::c_int,
                        b"WS character in key -> 400\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                13 | 10 | 40 | 41 | 60 | 62 | 64 | 44 | 58 | 59 | 92 | 34 | 47 | 91 | 93
                | 63 | 61 | 123 | 125 => {
                    return http_request_header_char_invalid(
                        r,
                        *k.offset(i as isize),
                        b"invalid character in header key -> 400\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                _ => {
                    if if http_header_strict != 0 {
                        ((*k.offset(i as isize) as libc::c_int) < 32 as libc::c_int
                            || *(k as *mut libc::c_uchar).offset(i as isize)
                                as libc::c_int >= 127 as libc::c_int) as libc::c_int
                    } else {
                        (*k.offset(i as isize) as libc::c_int == '\u{0}' as i32)
                            as libc::c_int
                    } != 0
                    {
                        return http_request_header_char_invalid(
                            r,
                            *k.offset(i as isize),
                            b"invalid character in header key -> 400\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
            }
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn http_request_parse_headers(
    r: *mut request_st,
    ptr: *mut libc::c_char,
    hoff: *const libc::c_ushort,
    http_parseopts: libc::c_uint,
) -> libc::c_int {
    let http_header_strict: libc::c_uint = http_parseopts
        & HTTP_PARSEOPT_HEADER_STRICT as libc::c_int as libc::c_uint;
    let mut i: libc::c_int = 2 as libc::c_int;
    while i < *hoff.offset(0 as libc::c_int as isize) as libc::c_int {
        let mut k: *const libc::c_char = ptr
            .offset(*hoff.offset(i as isize) as libc::c_int as isize);
        let mut end: *mut libc::c_char = ptr
            .offset(
                *hoff.offset((i + 1 as libc::c_int) as isize) as libc::c_int as isize,
            );
        let mut colon: *const libc::c_char = memchr(
            k as *const libc::c_void,
            ':' as i32,
            end.offset_from(k) as libc::c_long as libc::c_ulong,
        ) as *const libc::c_char;
        if colon.is_null() {
            return http_request_header_line_invalid(
                r,
                400 as libc::c_int,
                b"invalid header missing ':' -> 400\0" as *const u8
                    as *const libc::c_char,
            );
        }
        let mut v: *const libc::c_char = colon.offset(1 as libc::c_int as isize);
        if *colon.offset(-(1 as libc::c_int) as isize) as libc::c_int == ' ' as i32
            || *colon.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\t' as i32
        {
            if http_header_strict != 0 {
                return http_request_header_line_invalid(
                    r,
                    400 as libc::c_int,
                    b"invalid whitespace between field-name and colon -> 400\0"
                        as *const u8 as *const libc::c_char,
                )
            } else {
                loop {
                    colon = colon.offset(-1);
                    if !(*colon.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == ' ' as i32
                        || *colon.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            == '\t' as i32)
                    {
                        break;
                    }
                }
            }
        }
        let klen: libc::c_int = colon.offset_from(k) as libc::c_long as libc::c_int;
        if 0 as libc::c_int == klen {
            return http_request_header_line_invalid(
                r,
                400 as libc::c_int,
                b"invalid header key -> 400\0" as *const u8 as *const libc::c_char,
            );
        }
        let id: http_header_e = http_header_hkey_get(k, klen as size_t);
        if id as libc::c_uint == HTTP_HEADER_OTHER as libc::c_int as libc::c_uint {
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < klen {
                if light_isalpha(*k.offset(j as isize) as libc::c_int) != 0
                    || *k.offset(j as isize) as libc::c_int == '-' as i32
                {
                    j += 1;
                } else {
                    if 0 as libc::c_int
                        != http_request_parse_header_other(
                            r,
                            k.offset(j as isize),
                            klen - j,
                            http_header_strict,
                        )
                    {
                        return 400 as libc::c_int;
                    }
                    break;
                }
            }
        }
        while *v as libc::c_int == ' ' as i32 || *v as libc::c_int == '\t' as i32 {
            v = v.offset(1);
        }
        while i + 1 as libc::c_int
            <= *hoff.offset(0 as libc::c_int as isize) as libc::c_int
        {
            end = ptr
                .offset(
                    *hoff.offset((i + 1 as libc::c_int) as isize) as libc::c_int as isize,
                );
            if *end.offset(0 as libc::c_int as isize) as libc::c_int != ' ' as i32
                && *end.offset(0 as libc::c_int as isize) as libc::c_int != '\t' as i32
            {
                break;
            }
            if *end.offset(-(2 as libc::c_int) as isize) as libc::c_int == '\r' as i32 {
                *end.offset(-(2 as libc::c_int) as isize) = ' ' as i32 as libc::c_char;
            } else if http_header_strict != 0 {
                return http_request_header_line_invalid(
                    r,
                    400 as libc::c_int,
                    b"missing CR before LF in header -> 400\0" as *const u8
                        as *const libc::c_char,
                )
            }
            *end.offset(-(1 as libc::c_int) as isize) = ' ' as i32 as libc::c_char;
            i += 1;
        }
        if *end.offset(-(2 as libc::c_int) as isize) as libc::c_int == '\r' as i32 {
            end = end.offset(-1);
        } else if http_header_strict != 0 {
            return http_request_header_line_invalid(
                r,
                400 as libc::c_int,
                b"missing CR before LF in header -> 400\0" as *const u8
                    as *const libc::c_char,
            )
        }
        loop {
            end = end.offset(-1);
            if !(*end.offset(-(1 as libc::c_int) as isize) as libc::c_int == ' ' as i32
                || *end.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == '\t' as i32)
            {
                break;
            }
        }
        let vlen: libc::c_int = end.offset_from(v) as libc::c_long as libc::c_int;
        if !(vlen <= 0 as libc::c_int) {
            if http_header_strict != 0 {
                let x: *const libc::c_char = http_request_check_line_strict(
                    v,
                    vlen as uint_fast32_t,
                );
                if !x.is_null() {
                    return http_request_header_char_invalid(
                        r,
                        *x,
                        b"invalid character in header -> 400\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            let mut status: libc::c_int = http_request_parse_single_header(
                r,
                id,
                k,
                klen as size_t,
                v,
                vlen as size_t,
            );
            if 0 as libc::c_int != status {
                return status;
            }
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn http_request_parse(
    r: *mut request_st,
    scheme_port: libc::c_int,
) -> libc::c_int {
    let mut status: libc::c_int = http_request_parse_target(r, scheme_port);
    if 0 as libc::c_int != status {
        return status;
    }
    let http_parseopts: libc::c_uint = (*r).conf.http_parseopts;
    if ((*r).http_host != 0 as *mut libc::c_void as *mut buffer) as libc::c_int
        as libc::c_long != 0
    {
        if 0 as libc::c_int
            != http_request_host_policy((*r).http_host, http_parseopts, scheme_port)
        {
            return http_request_header_line_invalid(
                r,
                400 as libc::c_int,
                b"Invalid Hostname -> 400\0" as *const u8 as *const libc::c_char,
            );
        }
        buffer_copy_buffer(&mut (*r).uri.authority, (*r).http_host);
    } else {
        buffer_copy_string_len(
            &mut (*r).uri.authority,
            b"\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        if (*r).http_version as libc::c_int >= HTTP_VERSION_1_1 as libc::c_int {
            return http_request_header_line_invalid(
                r,
                400 as libc::c_int,
                b"HTTP/1.1 but Host missing -> 400\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if HTTP_VERSION_1_1 as libc::c_int != (*r).http_version as libc::c_int
        && (*r).rqst_htags
            & ((1 as libc::c_ulong) << HTTP_HEADER_UPGRADE as libc::c_int
                | (1 as libc::c_ulong) << HTTP_HEADER_HTTP2_SETTINGS as libc::c_int) != 0
    {
        return http_request_header_line_invalid(
            r,
            400 as libc::c_int,
            b"invalid hop-by-hop header w/o HTTP/1.1 -> 400\0" as *const u8
                as *const libc::c_char,
        );
    }
    if 0 as libc::c_int as libc::c_long == (*r).reqbody_length {
        if HTTP_METHOD_POST as libc::c_int == (*r).http_method as libc::c_int
            && (*r).rqst_htags
                & (1 as libc::c_ulong) << HTTP_HEADER_CONTENT_LENGTH as libc::c_int == 0
        {
            return http_request_header_line_invalid(
                r,
                411 as libc::c_int,
                b"POST-request, but content-length missing -> 411\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else {
        if -(1 as libc::c_int) as libc::c_long == (*r).reqbody_length
            && (*r).rqst_htags
                & (1 as libc::c_ulong) << HTTP_HEADER_CONTENT_LENGTH as libc::c_int != 0
        {
            let http_header_strict: libc::c_uint = http_parseopts
                & HTTP_PARSEOPT_HEADER_STRICT as libc::c_int as libc::c_uint;
            if http_header_strict != 0 {
                return http_request_header_line_invalid(
                    r,
                    400 as libc::c_int,
                    b"invalid Transfer-Encoding + Content-Length -> 400\0" as *const u8
                        as *const libc::c_char,
                )
            } else {
                http_header_request_unset(
                    r,
                    HTTP_HEADER_CONTENT_LENGTH,
                    b"Content-Length\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                );
            }
        }
        if (*r).http_method as libc::c_int <= HTTP_METHOD_HEAD as libc::c_int
            && http_parseopts
                & HTTP_PARSEOPT_METHOD_GET_BODY as libc::c_int as libc::c_uint == 0
        {
            return http_request_header_line_invalid(
                r,
                400 as libc::c_int,
                b"GET/HEAD with content-length -> 400\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn http_request_parse_hoff(
    r: *mut request_st,
    hdrs: *mut libc::c_char,
    hoff: *const libc::c_ushort,
    scheme_port: libc::c_int,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let http_parseopts: libc::c_uint = (*r).conf.http_parseopts;
    status = http_request_parse_reqline(r, hdrs, hoff, http_parseopts);
    if 0 as libc::c_int != status {
        return status;
    }
    status = http_request_parse_headers(r, hdrs, hoff, http_parseopts);
    if 0 as libc::c_int != status {
        return status;
    }
    return http_request_parse(r, scheme_port);
}
unsafe extern "C" fn http_request_headers_fin(r: *mut request_st) {
    if 0 as libc::c_int == (*r).http_status {
        (*r).conditional_is_valid = !(0 as libc::c_uint);
    } else {
        (*r).keep_alive = 0 as libc::c_int as int8_t;
        (*r).reqbody_length = 0 as libc::c_int as off_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn http_request_headers_process(
    r: *mut request_st,
    hdrs: *mut libc::c_char,
    hoff: *const libc::c_ushort,
    scheme_port: libc::c_int,
) {
    (*r).http_status = http_request_parse_hoff(r, hdrs, hoff, scheme_port);
    http_request_headers_fin(r);
    if (0 as libc::c_int != (*r).http_status) as libc::c_int as libc::c_long != 0 {
        if (*r).conf.log_request_header_on_error != 0 {
            log_error_multiline(
                (*r).conf.errh,
                b"src/request.c\0" as *const u8 as *const libc::c_char,
                1277 as libc::c_int as libc::c_uint,
                hdrs,
                (*r).rqst_header_len as size_t,
                b"rqst: \0" as *const u8 as *const libc::c_char,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn http_request_headers_process_h2(
    r: *mut request_st,
    scheme_port: libc::c_int,
) {
    if 0 as libc::c_int == (*r).http_status {
        (*r).http_status = http_request_parse(r, scheme_port);
    }
    if 0 as libc::c_int == (*r).http_status {
        if (*r).rqst_htags
            & (1 as libc::c_ulong) << HTTP_HEADER_CONNECTION as libc::c_int != 0
        {
            (*r)
                .http_status = http_request_header_line_invalid(
                r,
                400 as libc::c_int,
                b"invalid Connection header with HTTP/2 -> 400\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    http_request_headers_fin(r);
    if (0 as libc::c_int != (*r).http_status) as libc::c_int as libc::c_long != 0 {
        if (*r).conf.log_request_header_on_error != 0 {
            log_error(
                (*r).conf.errh,
                b"src/request.c\0" as *const u8 as *const libc::c_char,
                1301 as libc::c_int as libc::c_uint,
                b"request-header:\n:authority: %s\n:method: %s\n:path: %s\0" as *const u8
                    as *const libc::c_char,
                if !((*r).http_host).is_null() {
                    (*(*r).http_host).ptr as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                (*http_method_buf((*r).http_method)).ptr,
                if buffer_is_blank(&mut (*r).target) == 0 {
                    (*r).target.ptr as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
        }
    }
    if (*r).rqst_htags & (1 as libc::c_ulong) << HTTP_HEADER_UPGRADE as libc::c_int != 0
    {
        http_header_request_unset(
            r,
            HTTP_HEADER_UPGRADE,
            b"upgrade\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
}
