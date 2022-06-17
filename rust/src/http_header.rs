use ::libc;
extern "C" {
    pub type connection;
    pub type plugin;
    pub type stat_cache_entry;
    pub type cond_match_t;
    pub type cond_cache_t;
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
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn buffer_string_prepare_append(b: *mut buffer, size: size_t) -> *mut libc::c_char;
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
    fn buffer_eq_icase_ssn(
        a: *const libc::c_char,
        b: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
    fn array_get_element_klen_ext(
        a: *const array,
        ext: libc::c_int,
        key: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut data_unset;
    fn array_get_element_klen(
        a: *const array,
        key: *const libc::c_char,
        klen: uint32_t,
    ) -> *const data_unset;
    fn array_get_buf_ptr_ext(
        a: *mut array,
        ext: libc::c_int,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn array_get_buf_ptr(
        a: *mut array,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
}
pub type __int8_t = libc::c_schar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type off_t = __off64_t;
pub type size_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub struct keyvlenvalue {
    pub key: int16_t,
    pub vlen: uint16_t,
    pub value: [libc::c_char; 28],
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
#[inline]
unsafe extern "C" fn light_isdigit(mut c: libc::c_int) -> libc::c_int {
    return ((c as uint32_t).wrapping_sub('0' as i32 as libc::c_uint)
        <= ('9' as i32 - '0' as i32) as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn buffer_truncate(mut b: *mut buffer, mut len: uint32_t) {
    *((*b).ptr).offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    (*b).used = len.wrapping_add(1 as libc::c_int as libc::c_uint);
}
#[inline]
unsafe extern "C" fn buffer_is_blank(mut b: *const buffer) -> libc::c_int {
    return ((*b).used < 2 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn buffer_clear(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn array_set_key_value(
    a: *mut array,
    k: *const libc::c_char,
    klen: uint32_t,
    v: *const libc::c_char,
    vlen: uint32_t,
) {
    buffer_copy_string_len(array_get_buf_ptr(a, k, klen), v, vlen as size_t);
}
static mut http_headers_off: [int8_t; 28] = [
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    0 as libc::c_int as int8_t,
    1 as libc::c_int as int8_t,
    4 as libc::c_int as int8_t,
    9 as libc::c_int as int8_t,
    11 as libc::c_int as int8_t,
    17 as libc::c_int as int8_t,
    21 as libc::c_int as int8_t,
    25 as libc::c_int as int8_t,
    27 as libc::c_int as int8_t,
    -(1 as libc::c_int) as int8_t,
    30 as libc::c_int as int8_t,
    31 as libc::c_int as int8_t,
    37 as libc::c_int as int8_t,
    40 as libc::c_int as int8_t,
    45 as libc::c_int as int8_t,
    49 as libc::c_int as int8_t,
    -(1 as libc::c_int) as int8_t,
    52 as libc::c_int as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    53 as libc::c_int as int8_t,
    54 as libc::c_int as int8_t,
    -(1 as libc::c_int) as int8_t,
    55 as libc::c_int as int8_t,
    -(1 as libc::c_int) as int8_t,
    57 as libc::c_int as int8_t,
];
static mut http_headers: [keyvlenvalue; 59] = [keyvlenvalue {
    key: 0,
    vlen: 0,
    value: [0; 28],
}; 59];
#[no_mangle]
pub unsafe extern "C" fn http_header_hkey_get(
    s: *const libc::c_char,
    slen: size_t,
) -> http_header_e {
    if (slen < ::std::mem::size_of::<[int8_t; 28]>() as libc::c_ulong) as libc::c_int
        as libc::c_long != 0
    {
        let i: libc::c_int = http_headers_off[slen as usize] as libc::c_int;
        let c: libc::c_int = *s.offset(0 as libc::c_int as isize) as libc::c_int
            | 0x20 as libc::c_int;
        let mut kv: *const keyvlenvalue = http_headers.as_ptr().offset(i as isize);
        if (i != -(1 as libc::c_int)) as libc::c_int as libc::c_long != 0 {
            loop {
                if !((c != (*kv).value[0 as libc::c_int as usize] as libc::c_int)
                    as libc::c_int as libc::c_long != 0)
                {
                    if buffer_eq_icase_ssn(
                        s.offset(1 as libc::c_int as isize),
                        ((*kv).value).as_ptr().offset(1 as libc::c_int as isize),
                        slen.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) != 0
                    {
                        return (*kv).key as http_header_e;
                    }
                }
                kv = kv.offset(1);
                if !(slen == (*kv).vlen as libc::c_ulong) {
                    break;
                }
            }
        }
    }
    return HTTP_HEADER_OTHER;
}
#[no_mangle]
pub unsafe extern "C" fn http_header_hkey_get_lc(
    s: *const libc::c_char,
    slen: size_t,
) -> http_header_e {
    if (slen < ::std::mem::size_of::<[int8_t; 28]>() as libc::c_ulong) as libc::c_int
        as libc::c_long != 0
    {
        let i: libc::c_int = http_headers_off[slen as usize] as libc::c_int;
        let c: libc::c_int = *s.offset(0 as libc::c_int as isize) as libc::c_int;
        let mut kv: *const keyvlenvalue = http_headers.as_ptr().offset(i as isize);
        if (i != -(1 as libc::c_int)) as libc::c_int as libc::c_long != 0 {
            loop {
                if !((c != (*kv).value[0 as libc::c_int as usize] as libc::c_int)
                    as libc::c_int as libc::c_long != 0)
                {
                    if 0 as libc::c_int
                        == memcmp(
                            s.offset(1 as libc::c_int as isize) as *const libc::c_void,
                            ((*kv).value).as_ptr().offset(1 as libc::c_int as isize)
                                as *const libc::c_void,
                            slen.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        )
                    {
                        return (*kv).key as http_header_e;
                    }
                }
                kv = kv.offset(1);
                if !(slen == (*kv).vlen as libc::c_ulong) {
                    break;
                }
            }
        }
    }
    return HTTP_HEADER_OTHER;
}
#[no_mangle]
pub unsafe extern "C" fn http_header_str_to_code(s: *const libc::c_char) -> libc::c_int {
    return if light_isdigit(*s.offset(0 as libc::c_int as isize) as libc::c_int) != 0
        && light_isdigit(*s.offset(1 as libc::c_int as isize) as libc::c_int) != 0
        && light_isdigit(*s.offset(2 as libc::c_int as isize) as libc::c_int) != 0
        && (*s.offset(3 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
            || *s.offset(3 as libc::c_int as isize) as libc::c_int == ' ' as i32
            || *s.offset(3 as libc::c_int as isize) as libc::c_int == '\t' as i32)
    {
        (*s.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32)
            * 100 as libc::c_int
            + (*s.offset(1 as libc::c_int as isize) as libc::c_int - '0' as i32)
                * 10 as libc::c_int
            + (*s.offset(2 as libc::c_int as isize) as libc::c_int - '0' as i32)
    } else {
        -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn http_header_str_contains_token(
    s: *const libc::c_char,
    slen: uint32_t,
    m: *const libc::c_char,
    mlen: uint32_t,
) -> libc::c_int {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    loop {
        while i < slen
            && (*s.offset(i as isize) as libc::c_int == ' ' as i32
                || *s.offset(i as isize) as libc::c_int == '\t' as i32
                || *s.offset(i as isize) as libc::c_int == ',' as i32)
        {
            i = i.wrapping_add(1);
        }
        if slen.wrapping_sub(i) < mlen {
            return 0 as libc::c_int;
        }
        if buffer_eq_icase_ssn(s.offset(i as isize), m, mlen as size_t) != 0 {
            i = (i as libc::c_uint).wrapping_add(mlen) as uint32_t as uint32_t;
            if i == slen || *s.offset(i as isize) as libc::c_int == ' ' as i32
                || *s.offset(i as isize) as libc::c_int == '\t' as i32
                || *s.offset(i as isize) as libc::c_int == ',' as i32
                || *s.offset(i as isize) as libc::c_int == ';' as i32
            {
                return 1 as libc::c_int;
            }
        }
        while i < slen && *s.offset(i as isize) as libc::c_int != ',' as i32 {
            i = i.wrapping_add(1);
        }
        if !(i < slen) {
            break;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn http_header_remove_token(
    b: *mut buffer,
    m: *const libc::c_char,
    mlen: uint32_t,
) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut s: *mut libc::c_char = (*b).ptr;
    while !s.is_null() {
        while *s as libc::c_int == ' ' as i32 || *s as libc::c_int == '\t' as i32
            || *s as libc::c_int == ',' as i32
        {
            s = s.offset(1);
        }
        if 0 as libc::c_int == strncasecmp(s, m, mlen as libc::c_ulong) {
            s = s.offset(mlen as isize);
            if *s as libc::c_int == '\u{0}' as i32 || *s as libc::c_int == ' ' as i32
                || *s as libc::c_int == '\t' as i32 || *s as libc::c_int == ',' as i32
                || *s as libc::c_int == ';' as i32
            {
                memset(
                    s.offset(-(mlen as isize)) as *mut libc::c_void,
                    ' ' as i32,
                    mlen as libc::c_ulong,
                );
                while *s as libc::c_int != '\u{0}' as i32
                    && *s as libc::c_int != ',' as i32
                {
                    s = s.offset(1);
                }
                rc = 1 as libc::c_int;
                if *s as libc::c_int == ',' as i32 {
                    let fresh0 = s;
                    s = s.offset(1);
                    *fresh0 = ' ' as i32 as libc::c_char;
                    continue;
                } else {
                    s = s.offset(-(mlen as isize));
                    while *s as libc::c_int != ',' as i32 && s != (*b).ptr {
                        s = s.offset(-1);
                    }
                    buffer_truncate(
                        b,
                        s.offset_from((*b).ptr) as libc::c_long as size_t as uint32_t,
                    );
                    break;
                }
            }
        }
        s = strchr(s, ',' as i32);
    }
    return rc;
}
#[inline]
unsafe extern "C" fn http_header_token_append(
    vb: *mut buffer,
    v: *const libc::c_char,
    vlen: uint32_t,
) {
    if buffer_is_blank(vb) == 0 {
        buffer_append_string_len(
            vb,
            b", \0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    buffer_append_string_len(vb, v, vlen as size_t);
}
#[cold]
#[inline]
unsafe extern "C" fn http_header_token_append_cookie(
    vb: *mut buffer,
    v: *const libc::c_char,
    vlen: uint32_t,
) {
    if buffer_is_blank(vb) == 0 {
        buffer_append_string_len(
            vb,
            b"; \0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    buffer_append_string_len(vb, v, vlen as size_t);
}
#[inline]
unsafe extern "C" fn http_header_generic_get_ifnotempty(
    a: *const array,
    id: http_header_e,
    k: *const libc::c_char,
    klen: uint32_t,
) -> *mut buffer {
    let ds: *mut data_string = array_get_element_klen_ext(a, id as libc::c_int, k, klen)
        as *mut data_string;
    return if !ds.is_null() && buffer_is_blank(&mut (*ds).value) == 0 {
        &mut (*ds).value
    } else {
        0 as *mut buffer
    };
}
#[inline]
unsafe extern "C" fn http_header_set_key_value(
    a: *mut array,
    mut id: http_header_e,
    k: *const libc::c_char,
    klen: size_t,
    v: *const libc::c_char,
    vlen: size_t,
) {
    buffer_copy_string_len(
        array_get_buf_ptr_ext(a, id as libc::c_int, k, klen as uint32_t),
        v,
        vlen,
    );
}
#[no_mangle]
pub unsafe extern "C" fn http_header_response_get(
    r: *const request_st,
    mut id: http_header_e,
    mut k: *const libc::c_char,
    mut klen: uint32_t,
) -> *mut buffer {
    return if (*r).resp_htags & (1 as libc::c_ulong) << id as libc::c_uint != 0 {
        http_header_generic_get_ifnotempty(&(*r).resp_headers, id, k, klen)
    } else {
        0 as *mut buffer
    };
}
#[no_mangle]
pub unsafe extern "C" fn http_header_response_set_ptr(
    r: *mut request_st,
    mut id: http_header_e,
    mut k: *const libc::c_char,
    mut klen: uint32_t,
) -> *mut buffer {
    (*r).resp_htags |= (1 as libc::c_ulong) << id as libc::c_uint;
    let vb: *mut buffer = array_get_buf_ptr_ext(
        &mut (*r).resp_headers,
        id as libc::c_int,
        k,
        klen,
    );
    buffer_clear(vb);
    return vb;
}
#[no_mangle]
pub unsafe extern "C" fn http_header_response_unset(
    r: *mut request_st,
    mut id: http_header_e,
    mut k: *const libc::c_char,
    mut klen: uint32_t,
) {
    if (*r).resp_htags & (1 as libc::c_ulong) << id as libc::c_uint != 0 {
        if id as libc::c_uint > HTTP_HEADER_OTHER as libc::c_int as libc::c_uint {
            (*r).resp_htags &= !((1 as libc::c_ulong) << id as libc::c_uint);
        }
        http_header_set_key_value(
            &mut (*r).resp_headers,
            id,
            k,
            klen as size_t,
            b"\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn http_header_response_set(
    r: *mut request_st,
    mut id: http_header_e,
    mut k: *const libc::c_char,
    mut klen: uint32_t,
    mut v: *const libc::c_char,
    mut vlen: uint32_t,
) {
    if vlen != 0 {
        (*r).resp_htags |= (1 as libc::c_ulong) << id as libc::c_uint;
    } else {
        if id as libc::c_uint > HTTP_HEADER_OTHER as libc::c_int as libc::c_uint {
            (*r).resp_htags &= !((1 as libc::c_ulong) << id as libc::c_uint);
        } else {};
    };
    http_header_set_key_value(
        &mut (*r).resp_headers,
        id,
        k,
        klen as size_t,
        v,
        vlen as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn http_header_response_append(
    r: *mut request_st,
    mut id: http_header_e,
    mut k: *const libc::c_char,
    mut klen: uint32_t,
    mut v: *const libc::c_char,
    mut vlen: uint32_t,
) {
    if 0 as libc::c_int as libc::c_uint == vlen {
        return;
    }
    (*r).resp_htags |= (1 as libc::c_ulong) << id as libc::c_uint;
    let vb: *mut buffer = array_get_buf_ptr_ext(
        &mut (*r).resp_headers,
        id as libc::c_int,
        k,
        klen,
    );
    http_header_token_append(vb, v, vlen);
}
#[cold]
unsafe extern "C" fn http_header_response_insert_addtl(
    r: *mut request_st,
    mut id: http_header_e,
    mut k: *const libc::c_char,
    mut klen: uint32_t,
    vb: *mut buffer,
    mut vlen: uint32_t,
) {
    let mut h: *mut libc::c_char = buffer_string_prepare_append(
        vb,
        (2 as libc::c_int as libc::c_uint)
            .wrapping_add(klen)
            .wrapping_add(vlen)
            .wrapping_add(2 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_str3(
        vb,
        b"\r\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        k,
        klen as size_t,
        b": \0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    if (*r).http_version as libc::c_int >= HTTP_VERSION_2 as libc::c_int {
        (*r).resp_header_repeated = 1 as libc::c_int as libc::c_char;
        h = h.offset(2 as libc::c_int as isize);
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while i < klen {
            if (*h.offset(i as isize) as uint32_t)
                .wrapping_sub('A' as i32 as libc::c_uint)
                <= ('Z' as i32 - 'A' as i32) as libc::c_uint
            {
                let ref mut fresh1 = *h.offset(i as isize);
                *fresh1 = (*fresh1 as libc::c_int | 0x20 as libc::c_int) as libc::c_char;
            }
            i = i.wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn http_header_response_insert(
    r: *mut request_st,
    mut id: http_header_e,
    mut k: *const libc::c_char,
    mut klen: uint32_t,
    mut v: *const libc::c_char,
    mut vlen: uint32_t,
) {
    if 0 as libc::c_int as libc::c_uint == vlen {
        return;
    }
    (*r).resp_htags |= (1 as libc::c_ulong) << id as libc::c_uint;
    let vb: *mut buffer = array_get_buf_ptr_ext(
        &mut (*r).resp_headers,
        id as libc::c_int,
        k,
        klen,
    );
    if buffer_is_blank(vb) == 0 {
        http_header_response_insert_addtl(r, id, k, klen, vb, vlen);
    }
    buffer_append_string_len(vb, v, vlen as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn http_header_request_get(
    r: *const request_st,
    mut id: http_header_e,
    mut k: *const libc::c_char,
    mut klen: uint32_t,
) -> *mut buffer {
    return if (*r).rqst_htags & (1 as libc::c_ulong) << id as libc::c_uint != 0 {
        http_header_generic_get_ifnotempty(&(*r).rqst_headers, id, k, klen)
    } else {
        0 as *mut buffer
    };
}
#[no_mangle]
pub unsafe extern "C" fn http_header_request_set_ptr(
    r: *mut request_st,
    mut id: http_header_e,
    mut k: *const libc::c_char,
    mut klen: uint32_t,
) -> *mut buffer {
    (*r).rqst_htags |= (1 as libc::c_ulong) << id as libc::c_uint;
    let vb: *mut buffer = array_get_buf_ptr_ext(
        &mut (*r).rqst_headers,
        id as libc::c_int,
        k,
        klen,
    );
    buffer_clear(vb);
    return vb;
}
#[no_mangle]
pub unsafe extern "C" fn http_header_request_unset(
    r: *mut request_st,
    mut id: http_header_e,
    mut k: *const libc::c_char,
    mut klen: uint32_t,
) {
    if (*r).rqst_htags & (1 as libc::c_ulong) << id as libc::c_uint != 0 {
        if id as libc::c_uint > HTTP_HEADER_OTHER as libc::c_int as libc::c_uint {
            (*r).rqst_htags &= !((1 as libc::c_ulong) << id as libc::c_uint);
        }
        http_header_set_key_value(
            &mut (*r).rqst_headers,
            id,
            k,
            klen as size_t,
            b"\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn http_header_request_set(
    r: *mut request_st,
    mut id: http_header_e,
    mut k: *const libc::c_char,
    mut klen: uint32_t,
    mut v: *const libc::c_char,
    mut vlen: uint32_t,
) {
    if vlen != 0 {
        (*r).rqst_htags |= (1 as libc::c_ulong) << id as libc::c_uint;
    } else {
        if id as libc::c_uint > HTTP_HEADER_OTHER as libc::c_int as libc::c_uint {
            (*r).rqst_htags &= !((1 as libc::c_ulong) << id as libc::c_uint);
        } else {};
    };
    http_header_set_key_value(
        &mut (*r).rqst_headers,
        id,
        k,
        klen as size_t,
        v,
        vlen as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn http_header_request_append(
    r: *mut request_st,
    mut id: http_header_e,
    mut k: *const libc::c_char,
    mut klen: uint32_t,
    mut v: *const libc::c_char,
    mut vlen: uint32_t,
) {
    if 0 as libc::c_int as libc::c_uint == vlen {
        return;
    }
    (*r).rqst_htags |= (1 as libc::c_ulong) << id as libc::c_uint;
    let vb: *mut buffer = array_get_buf_ptr_ext(
        &mut (*r).rqst_headers,
        id as libc::c_int,
        k,
        klen,
    );
    if id as libc::c_uint != HTTP_HEADER_COOKIE as libc::c_int as libc::c_uint {
        http_header_token_append(vb, v, vlen);
    } else {
        http_header_token_append_cookie(vb, v, vlen);
    };
}
#[no_mangle]
pub unsafe extern "C" fn http_header_env_get(
    r: *const request_st,
    mut k: *const libc::c_char,
    mut klen: uint32_t,
) -> *mut buffer {
    let ds: *mut data_string = array_get_element_klen(&(*r).env, k, klen)
        as *mut data_string;
    return if !ds.is_null() && buffer_is_blank(&mut (*ds).value) == 0 {
        &mut (*ds).value
    } else {
        0 as *mut buffer
    };
}
#[no_mangle]
pub unsafe extern "C" fn http_header_env_set_ptr(
    mut r: *mut request_st,
    mut k: *const libc::c_char,
    mut klen: uint32_t,
) -> *mut buffer {
    let vb: *mut buffer = array_get_buf_ptr(&mut (*r).env, k, klen);
    buffer_clear(vb);
    return vb;
}
#[no_mangle]
pub unsafe extern "C" fn http_header_env_set(
    r: *mut request_st,
    mut k: *const libc::c_char,
    mut klen: uint32_t,
    mut v: *const libc::c_char,
    mut vlen: uint32_t,
) {
    array_set_key_value(&mut (*r).env, k, klen, v, vlen);
}
#[no_mangle]
pub unsafe extern "C" fn http_header_env_append(
    r: *mut request_st,
    mut k: *const libc::c_char,
    mut klen: uint32_t,
    mut v: *const libc::c_char,
    mut vlen: uint32_t,
) {
    let vb: *mut buffer = array_get_buf_ptr(&mut (*r).env, k, klen);
    if 0 as libc::c_int as libc::c_uint == vlen {
        return;
    }
    http_header_token_append(vb, v, vlen);
}
#[no_mangle]
pub unsafe extern "C" fn http_header_parse_hoff(
    mut n: *const libc::c_char,
    clen: uint32_t,
    mut hoff: *mut libc::c_ushort,
) -> uint32_t {
    let mut hlen: uint32_t = 0 as libc::c_int as uint32_t;
    let mut b: *const libc::c_char = 0 as *const libc::c_char;
    loop {
        b = n;
        n = memchr(
            b as *const libc::c_void,
            '\n' as i32,
            clen.wrapping_sub(hlen) as libc::c_ulong,
        ) as *const libc::c_char;
        if n.is_null() {
            break;
        }
        let mut x: uint32_t = (n.offset_from(b) as libc::c_long
            + 1 as libc::c_int as libc::c_long) as uint32_t;
        hlen = (hlen as libc::c_uint).wrapping_add(x) as uint32_t as uint32_t;
        if x <= 2 as libc::c_int as libc::c_uint
            && (x == 1 as libc::c_int as libc::c_uint
                || *n.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\r' as i32)
        {
            *hoff
                .offset(
                    (*hoff.offset(0 as libc::c_int as isize) as libc::c_int
                        + 1 as libc::c_int) as isize,
                ) = hlen as libc::c_ushort;
            return hlen;
        }
        let ref mut fresh2 = *hoff.offset(0 as libc::c_int as isize);
        *fresh2 = (*fresh2).wrapping_add(1);
        if *fresh2 as libc::c_int >= 8192 as libc::c_int - 1 as libc::c_int {
            break;
        }
        *hoff
            .offset(
                *hoff.offset(0 as libc::c_int as isize) as isize,
            ) = hlen as libc::c_ushort;
        n = n.offset(1);
    }
    return 0 as libc::c_int as uint32_t;
}
pub unsafe fn run_static_initializers() {
    http_headers = [
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_TE as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"te\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_AGE as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"age\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_DNT as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"dnt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_P3P as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"p3p\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_HOST as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"host\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_DATE as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"date\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_ETAG as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"etag\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_VARY as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"vary\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_LINK as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"link\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_ALLOW as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"allow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_RANGE as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"range\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_COOKIE as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"cookie\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_ACCEPT as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"accept\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_STATUS as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"status\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_SERVER as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"server\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_EXPECT as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"expect\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_PRAGMA as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"pragma\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_UPGRADE as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"upgrade\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_REFERER as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"referer\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_EXPIRES as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"expires\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_ALT_SVC as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"alt-svc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_LOCATION as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"location\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_IF_MATCH as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"if-match\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_IF_RANGE as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"if-range\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_ALT_USED as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"alt-used\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_FORWARDED as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"forwarded\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_EXPECT_CT as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"expect-ct\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_CONNECTION as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"connection\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_SET_COOKIE as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"set-cookie\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_USER_AGENT as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"user-agent\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_CONTENT_TYPE as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"content-type\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_LAST_MODIFIED as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"last-modified\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_AUTHORIZATION as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"authorization\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_IF_NONE_MATCH as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"if-none-match\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_CACHE_CONTROL as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"cache-control\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_ACCEPT_RANGES as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"accept-ranges\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_CONTENT_RANGE as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"content-range\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_CONTENT_LENGTH as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"content-length\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_HTTP2_SETTINGS as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"http2-settings\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_ONION_LOCATION as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"onion-location\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_ACCEPT_ENCODING as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"accept-encoding\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_ACCEPT_LANGUAGE as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"accept-language\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_REFERRER_POLICY as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"referrer-policy\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_X_FORWARDED_FOR as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"x-forwarded-for\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_X_FRAME_OPTIONS as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"x-frame-options\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_WWW_AUTHENTICATE as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"www-authenticate\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_CONTENT_ENCODING as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"content-encoding\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_CONTENT_LOCATION as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"content-location\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_X_XSS_PROTECTION as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"x-xss-protection\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_IF_MODIFIED_SINCE as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"if-modified-since\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_TRANSFER_ENCODING as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"transfer-encoding\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_X_FORWARDED_PROTO as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"x-forwarded-proto\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_IF_UNMODIFIED_SINCE as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"if-unmodified-since\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_X_CONTENT_TYPE_OPTIONS as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"x-content-type-options\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_CONTENT_SECURITY_POLICY as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"content-security-policy\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_STRICT_TRANSPORT_SECURITY as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"strict-transport-security\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_UPGRADE_INSECURE_REQUESTS as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"upgrade-insecure-requests\0\0\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_ACCESS_CONTROL_ALLOW_ORIGIN as libc::c_int as int16_t,
                vlen: (::std::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"access-control-allow-origin\0"),
            };
            init
        },
        {
            let mut init = keyvlenvalue {
                key: HTTP_HEADER_OTHER as libc::c_int as int16_t,
                vlen: 0 as libc::c_int as uint16_t,
                value: *::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
    ];
}
