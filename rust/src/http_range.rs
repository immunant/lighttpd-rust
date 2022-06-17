use ::libc;
extern "C" {
    pub type connection;
    pub type plugin;
    pub type stat_cache_entry;
    pub type cond_match_t;
    pub type cond_cache_t;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
    fn li_itostrn(buf: *mut libc::c_char, buf_len: size_t, val: intmax_t) -> size_t;
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
    fn chunkqueue_reset(cq: *mut chunkqueue);
    fn chunkqueue_mark_written(cq: *mut chunkqueue, len: off_t);
    fn chunkqueue_append_mem_min(
        cq: *mut chunkqueue,
        mem: *const libc::c_char,
        len: size_t,
    );
    fn chunkqueue_steal(dest: *mut chunkqueue, src: *mut chunkqueue, len: off_t);
    fn chunkqueue_append_mem(cq: *mut chunkqueue, mem: *const libc::c_char, len: size_t);
    fn chunkqueue_append_cq_range(
        dst: *mut chunkqueue,
        src: *const chunkqueue,
        offset: off_t,
        len: off_t,
    );
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
}
pub type __int8_t = libc::c_schar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type off_t = __off64_t;
pub type size_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
unsafe extern "C" fn chunkqueue_length(mut cq: *const chunkqueue) -> off_t {
    return (*cq).bytes_in - (*cq).bytes_out;
}
#[inline(never)]
unsafe extern "C" fn http_range_coalesce(
    ranges: *mut off_t,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i + 2 as libc::c_int) < n {
        let b: off_t = *ranges.offset(i as isize);
        let e: off_t = *ranges.offset((i + 1 as libc::c_int) as isize);
        let mut j: libc::c_int = i + 2 as libc::c_int;
        while j < n {
            if if b <= *ranges.offset(j as isize) {
                (e < *ranges.offset(j as isize) - 80 as libc::c_int as libc::c_long)
                    as libc::c_int
            } else {
                (*ranges.offset((j + 1 as libc::c_int) as isize)
                    < b - 80 as libc::c_int as libc::c_long) as libc::c_int
            } != 0
            {
                j += 2 as libc::c_int;
            } else {
                *ranges
                    .offset(
                        i as isize,
                    ) = if b <= *ranges.offset(j as isize) {
                    b
                } else {
                    *ranges.offset(j as isize)
                };
                *ranges
                    .offset(
                        (i + 1 as libc::c_int) as isize,
                    ) = if e >= *ranges.offset((j + 1 as libc::c_int) as isize) {
                    e
                } else {
                    *ranges.offset((j + 1 as libc::c_int) as isize)
                };
                memmove(
                    ranges.offset(j as isize) as *mut libc::c_void,
                    ranges.offset(j as isize).offset(2 as libc::c_int as isize)
                        as *const libc::c_void,
                    ((n - j - 2 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<off_t>() as libc::c_ulong),
                );
                n -= 2 as libc::c_int;
                i = -(2 as libc::c_int);
                break;
            }
        }
        i += 2 as libc::c_int;
    }
    return n;
}
unsafe extern "C" fn http_range_parse_next(
    mut s: *const libc::c_char,
    len: off_t,
    ranges: *mut off_t,
) -> *const libc::c_char {
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: off_t = strtoll(s, &mut e, 10 as libc::c_int) as off_t;
    *ranges.offset(1 as libc::c_int as isize) = -(1 as libc::c_int) as off_t;
    if n >= 0 as libc::c_int as libc::c_long {
        if n as libc::c_longlong != 9223372036854775807 as libc::c_longlong && n < len
            && s != e as *const libc::c_char
        {
            *ranges.offset(0 as libc::c_int as isize) = n;
            while *e as libc::c_int == ' ' as i32 || *e as libc::c_int == '\t' as i32 {
                e = e.offset(1);
            }
            if *e as libc::c_int == '-' as i32 {
                s = e.offset(1 as libc::c_int as isize);
                n = strtoll(s, &mut e, 10 as libc::c_int) as off_t;
                if s == e as *const libc::c_char
                    || n == 0 as libc::c_int as libc::c_long
                        && *e.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            != '0' as i32
                {
                    *ranges
                        .offset(
                            1 as libc::c_int as isize,
                        ) = len - 1 as libc::c_int as libc::c_long;
                } else if *ranges.offset(0 as libc::c_int as isize) <= n
                        && n as libc::c_longlong
                            != 9223372036854775807 as libc::c_longlong
                    {
                    *ranges
                        .offset(
                            1 as libc::c_int as isize,
                        ) = if n < len {
                        n
                    } else {
                        len - 1 as libc::c_int as libc::c_long
                    };
                }
            }
        }
    } else if n as libc::c_longlong
            != -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong
        {
        *ranges
            .offset(
                0 as libc::c_int as isize,
            ) = if len > -n { len + n } else { 0 as libc::c_int as libc::c_long };
        *ranges
            .offset(1 as libc::c_int as isize) = len - 1 as libc::c_int as libc::c_long;
    }
    while *e as libc::c_int == ' ' as i32 || *e as libc::c_int == '\t' as i32 {
        e = e.offset(1);
    }
    return e;
}
unsafe extern "C" fn http_range_parse(
    mut s: *const libc::c_char,
    content_length: off_t,
    mut ranges: *mut off_t,
) -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    loop {
        s = http_range_parse_next(s, content_length, ranges.offset(n as isize));
        if (*s as libc::c_int == '\u{0}' as i32 || *s as libc::c_int == ',' as i32)
            && *ranges.offset((n + 1 as libc::c_int) as isize)
                != -(1 as libc::c_int) as libc::c_long
        {
            n += 2 as libc::c_int;
        } else {
            while *s as libc::c_int != '\u{0}' as i32 && *s as libc::c_int != ',' as i32
            {
                s = s.offset(1);
            }
        }
        let fresh0 = s;
        s = s.offset(1);
        if !(*fresh0 as libc::c_int != '\u{0}' as i32
            && n < 10 as libc::c_int * 2 as libc::c_int)
        {
            break;
        }
    }
    if n <= 2 as libc::c_int {
        return n;
    }
    return http_range_coalesce(ranges, n);
}
unsafe extern "C" fn http_range_single(r: *mut request_st, mut ranges: *const off_t) {
    let cq: *mut chunkqueue = &mut (*r).write_queue;
    let complete_length: off_t = chunkqueue_length(cq);
    let mut len: uint32_t = (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t;
    let mut cr: [libc::c_char; 72] = *::std::mem::transmute::<
        &[u8; 72],
        &mut [libc::c_char; 72],
    >(
        b"bytes \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    len = (len as libc::c_uint)
        .wrapping_add(
            li_itostrn(
                cr.as_mut_ptr().offset(len as isize),
                (::std::mem::size_of::<[libc::c_char; 72]>() as libc::c_ulong)
                    .wrapping_sub(len as libc::c_ulong),
                *ranges.offset(0 as libc::c_int as isize),
            ) as uint32_t,
        ) as uint32_t as uint32_t;
    let fresh1 = len;
    len = len.wrapping_add(1);
    cr[fresh1 as usize] = '-' as i32 as libc::c_char;
    len = (len as libc::c_uint)
        .wrapping_add(
            li_itostrn(
                cr.as_mut_ptr().offset(len as isize),
                (::std::mem::size_of::<[libc::c_char; 72]>() as libc::c_ulong)
                    .wrapping_sub(len as libc::c_ulong),
                *ranges.offset(1 as libc::c_int as isize),
            ) as uint32_t,
        ) as uint32_t as uint32_t;
    let fresh2 = len;
    len = len.wrapping_add(1);
    cr[fresh2 as usize] = '/' as i32 as libc::c_char;
    len = (len as libc::c_uint)
        .wrapping_add(
            li_itostrn(
                cr.as_mut_ptr().offset(len as isize),
                (::std::mem::size_of::<[libc::c_char; 72]>() as libc::c_ulong)
                    .wrapping_sub(len as libc::c_ulong),
                complete_length,
            ) as uint32_t,
        ) as uint32_t as uint32_t;
    http_header_response_set(
        r,
        HTTP_HEADER_CONTENT_RANGE,
        b"Content-Range\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
        cr.as_mut_ptr(),
        len,
    );
    if (*cq).first == (*cq).last {
        if *ranges.offset(0 as libc::c_int as isize) != 0 {
            chunkqueue_mark_written(cq, *ranges.offset(0 as libc::c_int as isize));
            (*cq).bytes_out -= *ranges.offset(0 as libc::c_int as isize);
            (*cq).bytes_in -= *ranges.offset(0 as libc::c_int as isize);
        }
        (*cq).bytes_in
            -= complete_length
                - (*ranges.offset(1 as libc::c_int as isize)
                    + 1 as libc::c_int as libc::c_long);
        let c: *mut chunk = (*cq).first;
        if (*c).type_0 as libc::c_uint == FILE_CHUNK as libc::c_int as libc::c_uint {
            (*c)
                .file
                .length = (*c).offset + *ranges.offset(1 as libc::c_int as isize)
                - *ranges.offset(0 as libc::c_int as isize)
                + 1 as libc::c_int as libc::c_long;
        } else {
            (*(*c).mem)
                .used = ((*c).offset + *ranges.offset(1 as libc::c_int as isize)
                - *ranges.offset(0 as libc::c_int as isize)
                + 1 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long)
                as uint32_t;
        }
    } else {
        let mut tq: chunkqueue = chunkqueue {
            first: 0 as *mut chunk,
            last: 0 as *mut chunk,
            bytes_in: 0,
            bytes_out: 0,
            tempdirs: 0 as *const array,
            upload_temp_file_size: 0,
            tempdir_idx: 0,
        };
        memset(
            &mut tq as *mut chunkqueue as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<chunkqueue>() as libc::c_ulong,
        );
        chunkqueue_steal(&mut tq, cq, complete_length);
        (*cq).bytes_out -= complete_length;
        (*cq).bytes_in -= complete_length;
        chunkqueue_mark_written(&mut tq, *ranges.offset(0 as libc::c_int as isize));
        chunkqueue_steal(
            cq,
            &mut tq,
            *ranges.offset(1 as libc::c_int as isize)
                - *ranges.offset(0 as libc::c_int as isize)
                + 1 as libc::c_int as libc::c_long,
        );
        chunkqueue_reset(&mut tq);
    };
}
#[cold]
unsafe extern "C" fn http_range_multi(
    r: *mut request_st,
    mut ranges: *const off_t,
    n: libc::c_int,
) {
    static mut boundary_prefix: [libc::c_char; 18] = unsafe {
        *::std::mem::transmute::<
            &[u8; 18],
            &[libc::c_char; 18],
        >(b"\r\n--fkj49sn38dcn3\0")
    };
    static mut boundary_end: [libc::c_char; 22] = unsafe {
        *::std::mem::transmute::<
            &[u8; 22],
            &[libc::c_char; 22],
        >(b"\r\n--fkj49sn38dcn3--\r\n\0")
    };
    static mut multipart_type: [libc::c_char; 45] = unsafe {
        *::std::mem::transmute::<
            &[u8; 45],
            &[libc::c_char; 45],
        >(b"multipart/byteranges; boundary=fkj49sn38dcn3\0")
    };
    let tb: *mut buffer = (*r).tmp_buf;
    buffer_copy_string_len(
        tb,
        boundary_prefix.as_ptr(),
        (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    let content_type: *const buffer = http_header_response_get(
        r,
        HTTP_HEADER_CONTENT_TYPE,
        b"Content-Type\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if !content_type.is_null() {
        buffer_append_str2(
            tb,
            b"\r\nContent-Type: \0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            (*content_type).ptr,
            buffer_clen(content_type) as size_t,
        );
    }
    buffer_append_string_len(
        tb,
        b"\r\nContent-Range: bytes \0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    let prefix_len: uint32_t = buffer_clen(tb);
    http_header_response_set(
        r,
        HTTP_HEADER_CONTENT_TYPE,
        b"Content-Type\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
        multipart_type.as_ptr(),
        (::std::mem::size_of::<[libc::c_char; 45]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    let cq: *mut chunkqueue = &mut (*r).write_queue;
    let complete_length: off_t = chunkqueue_length(cq);
    let c: *mut chunk = if (*cq).first == (*cq).last
        && (*(*cq).first).type_0 as libc::c_uint
            == MEM_CHUNK as libc::c_int as libc::c_uint
    {
        (*cq).first
    } else {
        0 as *mut chunk
    };
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        buffer_truncate(tb, prefix_len);
        buffer_append_int(tb, *ranges.offset(i as isize));
        buffer_append_string_len(
            tb,
            b"-\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        buffer_append_int(tb, *ranges.offset((i + 1 as libc::c_int) as isize));
        buffer_append_string_len(
            tb,
            b"/\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        buffer_append_int(tb, complete_length);
        buffer_append_string_len(
            tb,
            b"\r\n\r\n\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        if !c.is_null() {
            chunkqueue_append_mem(cq, (*tb).ptr, buffer_clen(tb) as size_t);
        } else {
            chunkqueue_append_mem_min(cq, (*tb).ptr, buffer_clen(tb) as size_t);
        }
        chunkqueue_append_cq_range(
            cq,
            cq,
            *ranges.offset(i as isize),
            *ranges.offset((i + 1 as libc::c_int) as isize) - *ranges.offset(i as isize)
                + 1 as libc::c_int as libc::c_long,
        );
        i += 2 as libc::c_int;
    }
    chunkqueue_append_mem_min(
        cq,
        boundary_end.as_ptr(),
        (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    chunkqueue_mark_written(cq, complete_length + 2 as libc::c_int as libc::c_long);
    (*cq).bytes_out -= complete_length + 2 as libc::c_int as libc::c_long;
    (*cq).bytes_in -= complete_length + 2 as libc::c_int as libc::c_long;
}
#[cold]
unsafe extern "C" fn http_range_not_satisfiable(
    r: *mut request_st,
    content_length: off_t,
) -> libc::c_int {
    let mut len: uint32_t = (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t;
    let mut cr: [libc::c_char; 32] = *::std::mem::transmute::<
        &[u8; 32],
        &mut [libc::c_char; 32],
    >(b"bytes */\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    len = (len as libc::c_uint)
        .wrapping_add(
            li_itostrn(
                cr.as_mut_ptr().offset(len as isize),
                (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                    .wrapping_sub(len as libc::c_ulong),
                content_length,
            ) as uint32_t,
        ) as uint32_t as uint32_t;
    http_header_response_set(
        r,
        HTTP_HEADER_CONTENT_RANGE,
        b"Content-Range\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
        cr.as_mut_ptr(),
        len,
    );
    (*r).handler_module = 0 as *const plugin;
    (*r).http_status = 416 as libc::c_int;
    return (*r).http_status;
}
#[inline(never)]
unsafe extern "C" fn http_range_process(
    r: *mut request_st,
    http_range: *const buffer,
) -> libc::c_int {
    let content_length: off_t = chunkqueue_length(&mut (*r).write_queue);
    if 0 as libc::c_int as libc::c_long == content_length {
        return (*r).http_status;
    }
    if (buffer_clen(http_range) as libc::c_ulong)
        < (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        || buffer_eq_icase_ssn(
            (*http_range).ptr,
            b"bytes=\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0
    {
        return (*r).http_status;
    }
    let mut ranges: [off_t; 20] = [0; 20];
    let n: libc::c_int = http_range_parse(
        ((*http_range).ptr)
            .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize)
            .offset(-(1 as libc::c_int as isize)),
        content_length,
        ranges.as_mut_ptr(),
    );
    if 2 as libc::c_int == n {
        http_range_single(r, ranges.as_mut_ptr() as *const off_t);
    } else if 0 as libc::c_int == n {
        return http_range_not_satisfiable(r, content_length)
    } else {
        http_range_multi(r, ranges.as_mut_ptr() as *const off_t, n);
    }
    buffer_append_int(
        http_header_response_set_ptr(
            r,
            HTTP_HEADER_CONTENT_LENGTH,
            b"Content-Length\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        ),
        chunkqueue_length(&mut (*r).write_queue),
    );
    (*r).http_status = 206 as libc::c_int;
    return (*r).http_status;
}
#[no_mangle]
pub unsafe extern "C" fn http_range_rfc7233(r: *mut request_st) -> libc::c_int {
    let http_status: libc::c_int = (*r).http_status;
    if (*r).resp_body_finished == 0 {
        return http_status;
    }
    if 200 as libc::c_int != http_status {
        return http_status;
    }
    if !((*r).http_method as libc::c_int <= HTTP_METHOD_HEAD as libc::c_int) {
        return http_status;
    }
    if ((*r).http_version as libc::c_int) < HTTP_VERSION_1_1 as libc::c_int {
        return http_status;
    }
    if (*r).resp_htags
        & ((1 as libc::c_ulong) << HTTP_HEADER_TRANSFER_ENCODING as libc::c_int
            | (1 as libc::c_ulong) << HTTP_HEADER_CONTENT_ENCODING as libc::c_int) != 0
    {
        return http_status;
    }
    if (*r).resp_htags & (1 as libc::c_ulong) << HTTP_HEADER_ACCEPT_RANGES as libc::c_int
        == 0
    {
        http_header_response_set(
            r,
            HTTP_HEADER_ACCEPT_RANGES,
            b"Accept-Ranges\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            b"bytes\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    } else {
        let accept_ranges: *const buffer = http_header_response_get(
            r,
            HTTP_HEADER_ACCEPT_RANGES,
            b"Accept-Ranges\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        if buffer_eq_slen(
            accept_ranges,
            b"none\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            return http_status;
        }
    }
    if (*r).http_method as libc::c_int != HTTP_METHOD_GET as libc::c_int {
        return http_status;
    }
    let http_range: *const buffer = http_header_request_get(
        r,
        HTTP_HEADER_RANGE,
        b"Range\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if http_range.is_null() {
        return http_status;
    }
    if (*r).rqst_htags & (1 as libc::c_ulong) << HTTP_HEADER_IF_RANGE as libc::c_int != 0
    {
        let if_range: *const buffer = http_header_request_get(
            r,
            HTTP_HEADER_IF_RANGE,
            b"If-Range\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        let cmp: *const buffer = if *((*if_range).ptr).offset(0 as libc::c_int as isize)
            as libc::c_int == '"' as i32
        {
            http_header_response_get(
                r,
                HTTP_HEADER_ETAG,
                b"ETag\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            )
        } else {
            http_header_response_get(
                r,
                HTTP_HEADER_LAST_MODIFIED,
                b"Last-Modified\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            )
        };
        if cmp.is_null() || buffer_is_equal(if_range, cmp) == 0 {
            return http_status;
        }
    }
    return http_range_process(r, http_range);
}
