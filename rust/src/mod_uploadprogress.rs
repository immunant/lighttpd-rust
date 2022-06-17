use ::libc;
extern "C" {
    pub type server;
    pub type connection;
    pub type stat_cache_entry;
    pub type pcre2_real_match_data_8;
    fn splaytree_delete(t: *mut splay_tree, key: libc::c_int) -> *mut splay_tree;
    fn splaytree_splay(t: *mut splay_tree, key: libc::c_int) -> *mut splay_tree;
    fn splaytree_insert(
        t: *mut splay_tree,
        key: libc::c_int,
        data: *mut libc::c_void,
    ) -> *mut splay_tree;
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_int(b: *mut buffer, val: intmax_t);
    fn buffer_eq_slen(
        b: *const buffer,
        s: *const libc::c_char,
        slen: size_t,
    ) -> libc::c_int;
    fn buffer_is_equal(a: *const buffer, b: *const buffer) -> libc::c_int;
    fn log_error(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        fmt: *const libc::c_char,
        _: ...
    );
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn chunkqueue_append_mem(cq: *mut chunkqueue, mem: *const libc::c_char, len: size_t);
    fn chunkqueue_append_buffer_open(cq: *mut chunkqueue) -> *mut buffer;
    fn chunkqueue_append_buffer_commit(cq: *mut chunkqueue);
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
    fn config_plugin_values_init(
        srv: *mut server,
        p_d: *mut libc::c_void,
        cpk: *const config_plugin_keys_t,
        mname: *const libc::c_char,
    ) -> libc::c_int;
    fn config_check_cond(r: *mut request_st, context_ndx: libc::c_int) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
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
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intmax_t = __intmax_t;
pub type unix_timespec64_t = timespec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tree_node {
    pub left: *mut tree_node,
    pub right: *mut tree_node,
    pub key: libc::c_int,
    pub data: *mut libc::c_void,
}
pub type splay_tree = tree_node;
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
pub struct request_map_entry {
    pub r_id: buffer,
    pub r: *mut request_st,
    pub ndx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_config {
    pub progress_url: *const buffer,
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
    pub request_map: *mut splay_tree,
}
#[inline]
unsafe extern "C" fn djbhash(
    mut str: *const libc::c_char,
    len: uint32_t,
    mut hash: uint32_t,
) -> uint32_t {
    let s: *const libc::c_uchar = str as *const libc::c_uchar;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < len {
        hash = (hash << 5 as libc::c_int).wrapping_add(hash)
            ^ *s.offset(i as isize) as libc::c_uint;
        i = i.wrapping_add(1);
    }
    return hash;
}
#[inline]
unsafe extern "C" fn splaytree_djbhash(
    mut str: *const libc::c_char,
    len: uint32_t,
) -> int32_t {
    return (djbhash(str, len, 5381 as libc::c_int as uint32_t)
        & !((1 as libc::c_int as uint32_t) << 31 as libc::c_int)) as int32_t;
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
unsafe extern "C" fn request_map_entry_init(
    r: *mut request_st,
    mut r_id: *const libc::c_char,
    mut idlen: size_t,
) -> *mut request_map_entry {
    let rme: *mut request_map_entry = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<request_map_entry>() as libc::c_ulong,
    ) as *mut request_map_entry;
    if rme.is_null() {
        ck_assert_failed(
            b"src/mod_uploadprogress.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int as libc::c_uint,
            b"rme\0" as *const u8 as *const libc::c_char,
        );
    }
    (*rme).r = r;
    (*rme).ndx = splaytree_djbhash(r_id, idlen as uint32_t);
    buffer_copy_string_len(&mut (*rme).r_id, r_id, idlen);
    return rme;
}
unsafe extern "C" fn request_map_entry_free(mut rme: *mut request_map_entry) {
    free((*rme).r_id.ptr as *mut libc::c_void);
    free(rme as *mut libc::c_void);
}
unsafe extern "C" fn request_map_remove(
    p: *mut plugin_data,
    rme: *mut request_map_entry,
) {
    let sptree: *mut *mut splay_tree = &mut (*p).request_map;
    *sptree = splaytree_splay(*sptree, (*rme).ndx);
    if !(*sptree).is_null() && (**sptree).key == (*rme).ndx {
        request_map_entry_free((**sptree).data as *mut request_map_entry);
        *sptree = splaytree_delete(*sptree, (**sptree).key);
    }
}
unsafe extern "C" fn request_map_insert(
    p: *mut plugin_data,
    rme: *mut request_map_entry,
) -> *mut request_map_entry {
    let sptree: *mut *mut splay_tree = &mut (*p).request_map;
    *sptree = splaytree_splay(*sptree, (*rme).ndx);
    if (*sptree).is_null() || (**sptree).key != (*rme).ndx {
        *sptree = splaytree_insert(*sptree, (*rme).ndx, rme as *mut libc::c_void);
        return rme;
    } else {
        request_map_entry_free(rme);
        return 0 as *mut request_map_entry;
    };
}
unsafe extern "C" fn request_map_get_request(
    p: *mut plugin_data,
    r_id: *const libc::c_char,
    idlen: size_t,
) -> *mut request_st {
    let sptree: *mut *mut splay_tree = &mut (*p).request_map;
    let mut ndx: libc::c_int = splaytree_djbhash(r_id, idlen as uint32_t);
    *sptree = splaytree_splay(*sptree, ndx);
    if !(*sptree).is_null() && (**sptree).key == ndx {
        let rme: *mut request_map_entry = (**sptree).data as *mut request_map_entry;
        if buffer_eq_slen(&mut (*rme).r_id, r_id, idlen) != 0 {
            return (*rme).r;
        }
    }
    return 0 as *mut request_st;
}
unsafe extern "C" fn request_map_free(p: *mut plugin_data) {
    let mut sptree: *mut splay_tree = (*p).request_map;
    (*p).request_map = 0 as *mut splay_tree;
    while !sptree.is_null() {
        request_map_entry_free((*sptree).data as *mut request_map_entry);
        sptree = splaytree_delete(sptree, (*sptree).key);
    }
}
#[cold]
unsafe extern "C" fn mod_uploadprogress_init() -> *mut libc::c_void {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<plugin_data>() as libc::c_ulong,
    );
}
#[cold]
unsafe extern "C" fn mod_uploadprogress_free(mut p_d: *mut libc::c_void) {
    request_map_free(p_d as *mut plugin_data);
}
unsafe extern "C" fn mod_uploadprogress_merge_config_cpv(
    pconf: *mut plugin_config,
    cpv: *const config_plugin_value_t,
) {
    match (*cpv).k_id {
        0 => {
            (*pconf).progress_url = (*cpv).v.b;
        }
        _ => return,
    };
}
unsafe extern "C" fn mod_uploadprogress_merge_config(
    pconf: *mut plugin_config,
    mut cpv: *const config_plugin_value_t,
) {
    loop {
        mod_uploadprogress_merge_config_cpv(pconf, cpv);
        cpv = cpv.offset(1);
        if !((*cpv).k_id != -(1 as libc::c_int)) {
            break;
        }
    };
}
unsafe extern "C" fn mod_uploadprogress_patch_config(
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
            mod_uploadprogress_merge_config(
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
static mut cpk: [config_plugin_keys_t; 2] = [config_plugin_keys_t {
    k: 0 as *const libc::c_char,
    klen: 0,
    ktype: 0,
    scope: 0,
}; 2];
#[cold]
unsafe extern "C" fn mod_uploadprogress_set_defaults(
    mut srv: *mut server,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let p: *mut plugin_data = p_d as *mut plugin_data;
    if config_plugin_values_init(
        srv,
        p as *mut libc::c_void,
        cpk.as_ptr(),
        b"mod_uploadprogress\0" as *const u8 as *const libc::c_char,
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
            match (*cpv).k_id {
                0 => {
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
            mod_uploadprogress_merge_config(&mut (*p).defaults, cpv_0);
        }
    }
    return HANDLER_GO_ON;
}
unsafe extern "C" fn mod_uploadprogress_get_reqid(
    r: *mut request_st,
) -> *const libc::c_char {
    let mut idstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: uint32_t = 0;
    let mut pathinfo: libc::c_int = 0 as libc::c_int;
    let mut h: *const buffer = http_header_request_get(
        r,
        HTTP_HEADER_OTHER,
        b"X-Progress-ID\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if !h.is_null() {
        idstr = (*h).ptr;
    } else if buffer_is_blank(&mut (*r).uri.query) == 0
            && {
                idstr = strstr(
                    (*r).uri.query.ptr,
                    b"X-Progress-ID=\0" as *const u8 as *const libc::c_char,
                );
                !idstr.is_null()
            }
        {
        idstr = idstr
            .offset(
                (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            );
    } else {
        idstr = (*r).uri.path.ptr;
        len = buffer_clen(&mut (*r).uri.path);
        if len > 32 as libc::c_int as libc::c_uint
            && *idstr
                .offset(
                    len
                        .wrapping_sub(32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                ) as libc::c_int == '/' as i32
        {
            pathinfo = 1 as libc::c_int;
            idstr = idstr
                .offset(len.wrapping_sub(32 as libc::c_int as libc::c_uint) as isize);
        } else {
            return 0 as *const libc::c_char
        }
    }
    len = 0 as libc::c_int as uint32_t;
    while light_isxdigit(*idstr.offset(len as isize) as libc::c_int) != 0 {
        len = len.wrapping_add(1);
    }
    if len != 32 as libc::c_int as libc::c_uint {
        if pathinfo == 0 {
            log_error(
                (*r).conf.errh,
                b"src/mod_uploadprogress.c\0" as *const u8 as *const libc::c_char,
                216 as libc::c_int as libc::c_uint,
                b"invalid progress-id; non-xdigit or len != %d: %s\0" as *const u8
                    as *const libc::c_char,
                32 as libc::c_int,
                idstr,
            );
        }
        return 0 as *const libc::c_char;
    }
    return idstr;
}
unsafe extern "C" fn mod_uploadprogress_uri_handler(
    mut r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    match (*r).http_method as libc::c_int {
        0 | 2 => {}
        _ => return HANDLER_GO_ON,
    }
    mod_uploadprogress_patch_config(r, p);
    if ((*p).conf.progress_url).is_null() {
        return HANDLER_GO_ON;
    }
    if (*r).http_method as libc::c_int == HTTP_METHOD_GET as libc::c_int
        && buffer_is_equal(&mut (*r).uri.path, (*p).conf.progress_url) == 0
    {
        return HANDLER_GO_ON;
    }
    let idstr: *const libc::c_char = mod_uploadprogress_get_reqid(r);
    if idstr.is_null() {
        return HANDLER_GO_ON;
    }
    if (*r).http_method as libc::c_int == HTTP_METHOD_POST as libc::c_int {
        let ref mut fresh0 = *((*r).plugin_ctx).offset((*p).id as isize);
        *fresh0 = request_map_insert(
            p,
            request_map_entry_init(r, idstr, 32 as libc::c_int as size_t),
        ) as *mut libc::c_void;
        return HANDLER_GO_ON;
    }
    (*r).resp_body_started = 1 as libc::c_int as libc::c_char;
    (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
    (*r).http_status = 200 as libc::c_int;
    (*r).handler_module = 0 as *const plugin;
    let post_r: *mut request_st = request_map_get_request(
        p,
        idstr,
        32 as libc::c_int as size_t,
    );
    if post_r.is_null() {
        log_error(
            (*r).conf.errh,
            b"src/mod_uploadprogress.c\0" as *const u8 as *const libc::c_char,
            279 as libc::c_int as libc::c_uint,
            b"ID not known: %.*s\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            idstr,
        );
        chunkqueue_append_mem(
            &mut (*r).write_queue,
            b"not in progress\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        return HANDLER_FINISHED;
    }
    http_header_response_set(
        r,
        HTTP_HEADER_CONTENT_TYPE,
        b"Content-Type\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
        b"text/xml\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    http_header_response_set(
        r,
        HTTP_HEADER_PRAGMA,
        b"Pragma\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
        b"no-cache\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    http_header_response_set(
        r,
        HTTP_HEADER_EXPIRES,
        b"Expires\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
        b"Thu, 19 Nov 1981 08:52:00 GMT\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    http_header_response_set(
        r,
        HTTP_HEADER_CACHE_CONTROL,
        b"Cache-Control\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
        b"no-store, no-cache, must-revalidate, post-check=0, pre-check=0\0" as *const u8
            as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 63]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    let b: *mut buffer = chunkqueue_append_buffer_open(&mut (*r).write_queue);
    buffer_copy_string_len(
        b,
        b"<?xml version=\"1.0\" encoding=\"iso-8859-1\"?><upload><size>\0" as *const u8
            as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 58]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(b, (*post_r).reqbody_length);
    buffer_append_string_len(
        b,
        b"</size><received>\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_int(b, (*post_r).reqbody_queue.bytes_in);
    buffer_append_string_len(
        b,
        b"</received></upload>\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    chunkqueue_append_buffer_commit(&mut (*r).write_queue);
    return HANDLER_FINISHED;
}
unsafe extern "C" fn mod_uploadprogress_request_done(
    mut r: *mut request_st,
    mut p_d: *mut libc::c_void,
) -> handler_t {
    let mut p: *mut plugin_data = p_d as *mut plugin_data;
    let rme: *mut request_map_entry = *((*r).plugin_ctx).offset((*p).id as isize)
        as *mut request_map_entry;
    if !rme.is_null() {
        let ref mut fresh1 = *((*r).plugin_ctx).offset((*p).id as isize);
        *fresh1 = 0 as *mut libc::c_void;
        request_map_remove(p, rme);
    }
    return HANDLER_GO_ON;
}
#[no_mangle]
pub unsafe extern "C" fn mod_uploadprogress_plugin_init(
    mut p: *mut plugin,
) -> libc::c_int {
    (*p).version = 0x10440 as libc::c_int as size_t;
    (*p).name = b"uploadprogress\0" as *const u8 as *const libc::c_char;
    (*p)
        .init = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
        Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
    >(Some(mod_uploadprogress_init as unsafe extern "C" fn() -> *mut libc::c_void));
    (*p)
        .handle_uri_clean = Some(
        mod_uploadprogress_uri_handler
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .handle_request_reset = Some(
        mod_uploadprogress_request_done
            as unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .set_defaults = Some(
        mod_uploadprogress_set_defaults
            as unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    );
    (*p)
        .cleanup = Some(
        mod_uploadprogress_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    cpk = [
        {
            let mut init = config_plugin_keys_t {
                k: b"upload-progress.progress-url\0" as *const u8 as *const libc::c_char,
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
