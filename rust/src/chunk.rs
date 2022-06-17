use ::libc;
extern "C" {
    fn buffer_init() -> *mut buffer;
    fn buffer_free(b: *mut buffer);
    fn buffer_move(b: *mut buffer, src: *mut buffer);
    fn buffer_string_prepare_copy(b: *mut buffer, size: size_t) -> *mut libc::c_char;
    fn buffer_extend(b: *mut buffer, x: size_t) -> *mut libc::c_char;
    fn buffer_commit(b: *mut buffer, size: size_t);
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
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
    fn fdevent_dup_cloexec(fd: libc::c_int) -> libc::c_int;
    fn fdevent_open_cloexec(
        pathname: *const libc::c_char,
        symlinks: libc::c_int,
        flags: libc::c_int,
        mode: mode_t,
    ) -> libc::c_int;
    fn fdevent_pipe_cloexec(
        fds: *mut libc::c_int,
        bufsz_hint: libc::c_uint,
    ) -> libc::c_int;
    fn fdevent_mkostemp(path: *mut libc::c_char, flags: libc::c_int) -> libc::c_int;
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
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off64_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn splice(
        __fdin: libc::c_int,
        __offin: *mut __off64_t,
        __fdout: libc::c_int,
        __offout: *mut __off64_t,
        __len: size_t,
        __flags: libc::c_uint,
    ) -> __ssize_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pread(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __nbytes: size_t,
        __offset: __off64_t,
    ) -> ssize_t;
    fn pwrite(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __nbytes: size_t,
        __offset: __off64_t,
    ) -> ssize_t;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
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
    fn pwritev(
        __fd: libc::c_int,
        __iovec: *const iovec,
        __count: libc::c_int,
        __offset: __off64_t,
    ) -> ssize_t;
    fn sendfile(
        __out_fd: libc::c_int,
        __in_fd: libc::c_int,
        __offset: *mut __off64_t,
        __count: size_t,
    ) -> ssize_t;
}
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __loff_t = __off64_t;
pub type loff_t = __loff_t;
pub type mode_t = __mode_t;
pub type off_t = __off64_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint32_t = __uint32_t;
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
pub struct fdlog_st {
    pub mode: C2RustUnnamed,
    pub fd: libc::c_int,
    pub b: buffer,
    pub fn_0: *const libc::c_char,
}
pub type C2RustUnnamed = libc::c_uint;
pub const FDLOG_PIPE: C2RustUnnamed = 3;
pub const FDLOG_SYSLOG: C2RustUnnamed = 2;
pub const FDLOG_FD: C2RustUnnamed = 1;
pub const FDLOG_FILE: C2RustUnnamed = 0;
pub type log_error_st = fdlog_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chunk {
    pub next: *mut chunk,
    pub type_0: C2RustUnnamed_2,
    pub mem: *mut buffer,
    pub offset: off_t,
    pub file: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub length: off_t,
    pub fd: libc::c_int,
    pub is_temp: libc::c_int,
    pub mmap: C2RustUnnamed_1,
    pub ref_0: *mut libc::c_void,
    pub refchg: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub start: *mut libc::c_char,
    pub length: size_t,
    pub offset: off_t,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const FILE_CHUNK: C2RustUnnamed_2 = 1;
pub const MEM_CHUNK: C2RustUnnamed_2 = 0;
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
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
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
pub const _SC_PAGESIZE: C2RustUnnamed_3 = 30;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const _SC_SIGSTKSZ: C2RustUnnamed_3 = 250;
pub const _SC_MINSIGSTKSZ: C2RustUnnamed_3 = 249;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed_3 = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed_3 = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed_3 = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed_3 = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed_3 = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed_3 = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed_3 = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed_3 = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed_3 = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed_3 = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed_3 = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed_3 = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed_3 = 236;
pub const _SC_IPV6: C2RustUnnamed_3 = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed_3 = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed_3 = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed_3 = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed_3 = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed_3 = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed_3 = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed_3 = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed_3 = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed_3 = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed_3 = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed_3 = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed_3 = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed_3 = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed_3 = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed_3 = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed_3 = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed_3 = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed_3 = 182;
pub const _SC_TRACE: C2RustUnnamed_3 = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed_3 = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed_3 = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed_3 = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed_3 = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed_3 = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed_3 = 175;
pub const _SC_STREAMS: C2RustUnnamed_3 = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed_3 = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed_3 = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed_3 = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed_3 = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed_3 = 169;
pub const _SC_2_PBS: C2RustUnnamed_3 = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed_3 = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed_3 = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed_3 = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed_3 = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed_3 = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed_3 = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed_3 = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed_3 = 160;
pub const _SC_SPAWN: C2RustUnnamed_3 = 159;
pub const _SC_SIGNALS: C2RustUnnamed_3 = 158;
pub const _SC_SHELL: C2RustUnnamed_3 = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed_3 = 156;
pub const _SC_REGEXP: C2RustUnnamed_3 = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed_3 = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed_3 = 153;
pub const _SC_NETWORKING: C2RustUnnamed_3 = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed_3 = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed_3 = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed_3 = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed_3 = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed_3 = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed_3 = 146;
pub const _SC_PIPE: C2RustUnnamed_3 = 145;
pub const _SC_FIFO: C2RustUnnamed_3 = 144;
pub const _SC_FD_MGMT: C2RustUnnamed_3 = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed_3 = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed_3 = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed_3 = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed_3 = 139;
pub const _SC_CPUTIME: C2RustUnnamed_3 = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed_3 = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed_3 = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed_3 = 135;
pub const _SC_BASE: C2RustUnnamed_3 = 134;
pub const _SC_BARRIERS: C2RustUnnamed_3 = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed_3 = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed_3 = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed_3 = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed_3 = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed_3 = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed_3 = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed_3 = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed_3 = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed_3 = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed_3 = 123;
pub const _SC_NL_NMAX: C2RustUnnamed_3 = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed_3 = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed_3 = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed_3 = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed_3 = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed_3 = 117;
pub const _SC_UINT_MAX: C2RustUnnamed_3 = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed_3 = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed_3 = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed_3 = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed_3 = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed_3 = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed_3 = 110;
pub const _SC_NZERO: C2RustUnnamed_3 = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed_3 = 108;
pub const _SC_WORD_BIT: C2RustUnnamed_3 = 107;
pub const _SC_LONG_BIT: C2RustUnnamed_3 = 106;
pub const _SC_INT_MIN: C2RustUnnamed_3 = 105;
pub const _SC_INT_MAX: C2RustUnnamed_3 = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed_3 = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed_3 = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed_3 = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed_3 = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed_3 = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed_3 = 98;
pub const _SC_2_UPE: C2RustUnnamed_3 = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed_3 = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed_3 = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed_3 = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed_3 = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed_3 = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed_3 = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed_3 = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed_3 = 89;
pub const _SC_PASS_MAX: C2RustUnnamed_3 = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed_3 = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed_3 = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed_3 = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed_3 = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed_3 = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed_3 = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed_3 = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed_3 = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed_3 = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed_3 = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed_3 = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed_3 = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed_3 = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed_3 = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed_3 = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed_3 = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed_3 = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed_3 = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed_3 = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed_3 = 68;
pub const _SC_THREADS: C2RustUnnamed_3 = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed_3 = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed_3 = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed_3 = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed_3 = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed_3 = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed_3 = 61;
pub const _SC_IOV_MAX: C2RustUnnamed_3 = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed_3 = 60;
pub const _SC_SELECT: C2RustUnnamed_3 = 59;
pub const _SC_POLL: C2RustUnnamed_3 = 58;
pub const _SC_PII_OSI: C2RustUnnamed_3 = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed_3 = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed_3 = 55;
pub const _SC_PII_XTI: C2RustUnnamed_3 = 54;
pub const _SC_PII: C2RustUnnamed_3 = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed_3 = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed_3 = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed_3 = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed_3 = 49;
pub const _SC_2_C_DEV: C2RustUnnamed_3 = 48;
pub const _SC_2_C_BIND: C2RustUnnamed_3 = 47;
pub const _SC_2_VERSION: C2RustUnnamed_3 = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed_3 = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed_3 = 44;
pub const _SC_LINE_MAX: C2RustUnnamed_3 = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed_3 = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed_3 = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed_3 = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed_3 = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed_3 = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed_3 = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed_3 = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed_3 = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed_3 = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed_3 = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed_3 = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed_3 = 31;
pub const _SC_VERSION: C2RustUnnamed_3 = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed_3 = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed_3 = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed_3 = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed_3 = 25;
pub const _SC_AIO_MAX: C2RustUnnamed_3 = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed_3 = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed_3 = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed_3 = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed_3 = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed_3 = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed_3 = 18;
pub const _SC_MEMLOCK: C2RustUnnamed_3 = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed_3 = 16;
pub const _SC_FSYNC: C2RustUnnamed_3 = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed_3 = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed_3 = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed_3 = 12;
pub const _SC_TIMERS: C2RustUnnamed_3 = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed_3 = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed_3 = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed_3 = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed_3 = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed_3 = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed_3 = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed_3 = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed_3 = 3;
pub const _SC_CLK_TCK: C2RustUnnamed_3 = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed_3 = 1;
pub const _SC_ARG_MAX: C2RustUnnamed_3 = 0;
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
unsafe extern "C" fn buffer_clear(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn chunkqueue_length(mut cq: *const chunkqueue) -> off_t {
    return (*cq).bytes_in - (*cq).bytes_out;
}
static mut chunk_buf_sz: size_t = 8192 as libc::c_int as size_t;
static mut chunks: *mut chunk = 0 as *const chunk as *mut chunk;
static mut chunks_oversized: *mut chunk = 0 as *const chunk as *mut chunk;
static mut chunks_filechunk: *mut chunk = 0 as *const chunk as *mut chunk;
static mut chunk_buffers: *mut chunk = 0 as *const chunk as *mut chunk;
static mut chunks_oversized_n: libc::c_int = 0;
static mut chunkqueue_default_tempdirs: *const array = 0 as *const array;
static mut chunkqueue_default_tempfile_size: off_t = (1 as libc::c_int
    * 1024 as libc::c_int * 1024 as libc::c_int) as off_t;
#[no_mangle]
#[cold]
pub unsafe extern "C" fn chunkqueue_set_chunk_size(mut sz: size_t) {
    let mut x: size_t = 1024 as libc::c_int as size_t;
    while x < sz && x < ((1 as libc::c_uint) << 30 as libc::c_int) as libc::c_ulong {
        x <<= 1 as libc::c_int;
    }
    chunk_buf_sz = if sz > 0 as libc::c_int as libc::c_ulong {
        x
    } else {
        8192 as libc::c_int as libc::c_ulong
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn chunkqueue_set_tempdirs_default_reset() {
    chunk_buf_sz = 8192 as libc::c_int as size_t;
    chunkqueue_default_tempdirs = 0 as *const array;
    chunkqueue_default_tempfile_size = (1 as libc::c_int * 1024 as libc::c_int
        * 1024 as libc::c_int) as off_t;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_init(mut cq: *mut chunkqueue) -> *mut chunkqueue {
    if cq.is_null() {
        cq = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<chunkqueue>() as libc::c_ulong,
        ) as *mut chunkqueue;
        if cq.is_null() {
            ck_assert_failed(
                b"src/chunk.c\0" as *const u8 as *const libc::c_char,
                52 as libc::c_int as libc::c_uint,
                b"((void*)0) != cq\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    (*cq).first = 0 as *mut chunk;
    (*cq).last = 0 as *mut chunk;
    (*cq).tempdirs = chunkqueue_default_tempdirs;
    (*cq).upload_temp_file_size = chunkqueue_default_tempfile_size;
    return cq;
}
unsafe extern "C" fn chunk_init() -> *mut chunk {
    let c: *mut chunk = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<chunk>() as libc::c_ulong,
    ) as *mut chunk;
    if c.is_null() {
        ck_assert_failed(
            b"src/chunk.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int as libc::c_uint,
            b"((void*)0) != c\0" as *const u8 as *const libc::c_char,
        );
    }
    (*c).file.fd = -(1 as libc::c_int);
    (*c).file.mmap.start = -(1 as libc::c_int) as *mut libc::c_void as *mut libc::c_char;
    (*c).mem = buffer_init();
    return c;
}
unsafe extern "C" fn chunk_init_sz(mut sz: size_t) -> *mut chunk {
    let c: *mut chunk = chunk_init();
    buffer_string_prepare_copy(
        (*c).mem,
        sz.wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    return c;
}
unsafe extern "C" fn chunk_reset_file_chunk(mut c: *mut chunk) {
    if (*c).file.is_temp != 0 {
        (*c).file.is_temp = 0 as libc::c_int;
        if buffer_is_blank((*c).mem) == 0 {
            unlink((*(*c).mem).ptr);
        }
    }
    if ((*c).file.refchg).is_some() {
        ((*c).file.refchg)
            .expect("non-null function pointer")((*c).file.ref_0, -(1 as libc::c_int));
        (*c).file.refchg = None;
        (*c).file.ref_0 = 0 as *mut libc::c_void;
    } else if (*c).file.fd != -(1 as libc::c_int) {
        close((*c).file.fd);
    }
    if -(1 as libc::c_int) as *mut libc::c_void
        != (*c).file.mmap.start as *mut libc::c_void
    {
        munmap((*c).file.mmap.start as *mut libc::c_void, (*c).file.mmap.length);
        (*c)
            .file
            .mmap
            .start = -(1 as libc::c_int) as *mut libc::c_void as *mut libc::c_char;
        (*c).file.mmap.offset = 0 as libc::c_int as off_t;
        (*c).file.mmap.length = (*c).file.mmap.offset as size_t;
    }
    (*c).file.fd = -(1 as libc::c_int);
    (*c).file.length = 0 as libc::c_int as off_t;
    (*c).type_0 = MEM_CHUNK;
}
unsafe extern "C" fn chunk_reset(mut c: *mut chunk) {
    if (*c).type_0 as libc::c_uint == FILE_CHUNK as libc::c_int as libc::c_uint {
        chunk_reset_file_chunk(c);
    }
    buffer_clear((*c).mem);
    (*c).offset = 0 as libc::c_int as off_t;
}
unsafe extern "C" fn chunk_free(mut c: *mut chunk) {
    if (*c).type_0 as libc::c_uint == FILE_CHUNK as libc::c_int as libc::c_uint {
        chunk_reset_file_chunk(c);
    }
    buffer_free((*c).mem);
    free(c as *mut libc::c_void);
}
unsafe extern "C" fn chunk_pop_oversized(mut sz: size_t) -> *mut chunk {
    if !chunks_oversized.is_null()
        && (*(*chunks_oversized).mem).size as libc::c_ulong >= sz
    {
        chunks_oversized_n -= 1;
        let mut c: *mut chunk = chunks_oversized;
        chunks_oversized = (*c).next;
        return c;
    }
    return 0 as *mut chunk;
}
unsafe extern "C" fn chunk_push_oversized(c: *mut chunk, sz: size_t) {
    if chunks_oversized_n < 64 as libc::c_int
        && chunk_buf_sz >= 4096 as libc::c_int as libc::c_ulong
    {
        chunks_oversized_n += 1;
        let mut co: *mut *mut chunk = &mut chunks_oversized;
        while !(*co).is_null() && sz < (*(**co).mem).size as libc::c_ulong {
            co = &mut (**co).next;
        }
        (*c).next = *co;
        *co = c;
    } else {
        let tb: *mut buffer = if !chunks_oversized.is_null() {
            (*chunks_oversized).mem
        } else {
            0 as *mut buffer
        };
        if !tb.is_null() && ((*tb).size as libc::c_ulong) < sz {
            (*chunks_oversized).mem = (*c).mem;
            (*c).mem = tb;
        }
        chunk_free(c);
    };
}
unsafe extern "C" fn chunk_buffer_acquire_sz(sz: size_t) -> *mut buffer {
    let mut c: *mut chunk = 0 as *mut chunk;
    let mut b: *mut buffer = 0 as *mut buffer;
    if sz <= chunk_buf_sz | 1 as libc::c_int as libc::c_ulong {
        if !chunks.is_null() {
            c = chunks;
            chunks = (*c).next;
        } else {
            c = chunk_init_sz(chunk_buf_sz);
        }
    } else {
        c = chunk_pop_oversized(sz);
        if c.is_null() {
            c = chunk_init_sz(
                (sz & !(1 as libc::c_ulong))
                    .wrapping_add(
                        chunk_buf_sz.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) & !chunk_buf_sz.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
        }
    }
    (*c).next = chunk_buffers;
    chunk_buffers = c;
    b = (*c).mem;
    (*c).mem = 0 as *mut buffer;
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn chunk_buffer_acquire() -> *mut buffer {
    return chunk_buffer_acquire_sz(chunk_buf_sz);
}
#[no_mangle]
pub unsafe extern "C" fn chunk_buffer_release(mut b: *mut buffer) {
    if b.is_null() {
        return;
    }
    if !chunk_buffers.is_null() {
        let mut c: *mut chunk = chunk_buffers;
        chunk_buffers = (*c).next;
        (*c).mem = b;
        buffer_clear(b);
        if (*b).size as libc::c_ulong == chunk_buf_sz | 1 as libc::c_int as libc::c_ulong
        {
            (*c).next = chunks;
            chunks = c;
        } else if (*b).size as libc::c_ulong > chunk_buf_sz {
            chunk_push_oversized(c, (*b).size as size_t);
        } else {
            chunk_free(c);
        }
    } else {
        buffer_free(b);
    };
}
#[no_mangle]
pub unsafe extern "C" fn chunk_buffer_yield(mut b: *mut buffer) {
    if (*b).size as libc::c_ulong == chunk_buf_sz | 1 as libc::c_int as libc::c_ulong {
        return;
    }
    let cb: *mut buffer = chunk_buffer_acquire_sz(chunk_buf_sz);
    let mut tb: buffer = *b;
    *b = *cb;
    *cb = tb;
    chunk_buffer_release(cb);
}
#[no_mangle]
pub unsafe extern "C" fn chunk_buffer_prepare_append(
    b: *mut buffer,
    mut sz: size_t,
) -> size_t {
    if sz > buffer_string_space(b) as libc::c_ulong {
        sz = (sz as libc::c_ulong)
            .wrapping_add(
                (if (*b).used != 0 {
                    (*b).used
                } else {
                    1 as libc::c_int as libc::c_uint
                }) as libc::c_ulong,
            ) as size_t as size_t;
        let cb: *mut buffer = chunk_buffer_acquire_sz(sz);
        let mut tb: buffer = *b;
        *b = *cb;
        *cb = tb;
        (*b).used = tb.used;
        if (*b).used != 0 {
            memcpy(
                (*b).ptr as *mut libc::c_void,
                tb.ptr as *const libc::c_void,
                tb.used as libc::c_ulong,
            );
        }
        chunk_buffer_release(cb);
    }
    return buffer_string_space(b) as size_t;
}
unsafe extern "C" fn chunk_acquire(mut sz: size_t) -> *mut chunk {
    if sz <= chunk_buf_sz | 1 as libc::c_int as libc::c_ulong {
        if !chunks.is_null() {
            let mut c: *mut chunk = chunks;
            chunks = (*c).next;
            return c;
        }
        sz = chunk_buf_sz;
    } else {
        sz = sz
            .wrapping_add(chunk_buf_sz.wrapping_sub(1 as libc::c_int as libc::c_ulong))
            & !chunk_buf_sz.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut c_0: *mut chunk = chunk_pop_oversized(sz);
        if !c_0.is_null() {
            return c_0;
        }
    }
    return chunk_init_sz(sz);
}
unsafe extern "C" fn chunk_release(mut c: *mut chunk) {
    let sz: size_t = (*(*c).mem).size as size_t;
    if sz == chunk_buf_sz | 1 as libc::c_int as libc::c_ulong {
        chunk_reset(c);
        (*c).next = chunks;
        chunks = c;
    } else if sz > chunk_buf_sz {
        chunk_reset(c);
        chunk_push_oversized(c, sz);
    } else if (*c).type_0 as libc::c_uint == FILE_CHUNK as libc::c_int as libc::c_uint {
        chunk_reset(c);
        (*c).next = chunks_filechunk;
        chunks_filechunk = c;
    } else {
        chunk_free(c);
    };
}
unsafe extern "C" fn chunk_acquire_filechunk() -> *mut chunk {
    if !chunks_filechunk.is_null() {
        let mut c: *mut chunk = chunks_filechunk;
        chunks_filechunk = (*c).next;
        return c;
    }
    return chunk_init();
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_chunk_pool_clear() {
    let mut next: *mut chunk = 0 as *mut chunk;
    let mut c: *mut chunk = chunks;
    while !c.is_null() {
        next = (*c).next;
        chunk_free(c);
        c = next;
    }
    chunks = 0 as *mut chunk;
    let mut next_0: *mut chunk = 0 as *mut chunk;
    let mut c_0: *mut chunk = chunks_oversized;
    while !c_0.is_null() {
        next_0 = (*c_0).next;
        chunk_free(c_0);
        c_0 = next_0;
    }
    chunks_oversized = 0 as *mut chunk;
    chunks_oversized_n = 0 as libc::c_int;
    let mut next_1: *mut chunk = 0 as *mut chunk;
    let mut c_1: *mut chunk = chunks_filechunk;
    while !c_1.is_null() {
        next_1 = (*c_1).next;
        chunk_free(c_1);
        c_1 = next_1;
    }
    chunks_filechunk = 0 as *mut chunk;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_chunk_pool_free() {
    chunkqueue_chunk_pool_clear();
    let mut next: *mut chunk = 0 as *mut chunk;
    let mut c: *mut chunk = chunk_buffers;
    while !c.is_null() {
        next = (*c).next;
        free(c as *mut libc::c_void);
        c = next;
    }
    chunk_buffers = 0 as *mut chunk;
}
unsafe extern "C" fn chunk_remaining_length(mut c: *const chunk) -> off_t {
    return (if (*c).type_0 as libc::c_uint == MEM_CHUNK as libc::c_int as libc::c_uint {
        buffer_clen((*c).mem) as off_t
    } else {
        (*c).file.length
    }) - (*c).offset;
}
unsafe extern "C" fn chunkqueue_release_chunks(mut cq: *mut chunkqueue) {
    (*cq).last = 0 as *mut chunk;
    let mut c: *mut chunk = 0 as *mut chunk;
    loop {
        c = (*cq).first;
        if c.is_null() {
            break;
        }
        (*cq).first = (*c).next;
        chunk_release(c);
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn chunkqueue_free(mut cq: *mut chunkqueue) {
    if cq.is_null() {
        return;
    }
    chunkqueue_release_chunks(cq);
    free(cq as *mut libc::c_void);
}
unsafe extern "C" fn chunkqueue_prepend_chunk(cq: *mut chunkqueue, c: *mut chunk) {
    (*c).next = (*cq).first;
    if ((*c).next).is_null() {
        (*cq).last = c;
    }
    (*cq).first = c;
}
unsafe extern "C" fn chunkqueue_append_chunk(cq: *mut chunkqueue, c: *mut chunk) {
    (*c).next = 0 as *mut chunk;
    let ref mut fresh0 = *if !((*cq).last).is_null() {
        &mut (*(*cq).last).next
    } else {
        &mut (*cq).first
    };
    *fresh0 = c;
    (*cq).last = c;
}
unsafe extern "C" fn chunkqueue_prepend_mem_chunk(
    mut cq: *mut chunkqueue,
    mut sz: size_t,
) -> *mut chunk {
    let mut c: *mut chunk = chunk_acquire(sz);
    chunkqueue_prepend_chunk(cq, c);
    return c;
}
unsafe extern "C" fn chunkqueue_append_mem_chunk(
    mut cq: *mut chunkqueue,
    mut sz: size_t,
) -> *mut chunk {
    let mut c: *mut chunk = chunk_acquire(sz);
    chunkqueue_append_chunk(cq, c);
    return c;
}
unsafe extern "C" fn chunkqueue_append_file_chunk(
    cq: *mut chunkqueue,
    fn_0: *const buffer,
    mut offset: off_t,
    mut len: off_t,
) -> *mut chunk {
    let c: *mut chunk = chunk_acquire_filechunk();
    chunkqueue_append_chunk(cq, c);
    (*c).type_0 = FILE_CHUNK;
    (*c).offset = offset;
    (*c).file.length = offset + len;
    (*cq).bytes_in += len;
    buffer_copy_buffer((*c).mem, fn_0);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_reset(mut cq: *mut chunkqueue) {
    chunkqueue_release_chunks(cq);
    (*cq).bytes_in = 0 as libc::c_int as off_t;
    (*cq).bytes_out = 0 as libc::c_int as off_t;
    (*cq).tempdir_idx = 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_file_fd(
    cq: *mut chunkqueue,
    fn_0: *const buffer,
    mut fd: libc::c_int,
    mut offset: off_t,
    mut len: off_t,
) {
    if len > 0 as libc::c_int as libc::c_long {
        (*chunkqueue_append_file_chunk(cq, fn_0, offset, len)).file.fd = fd;
    } else {
        close(fd);
    };
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_file(
    cq: *mut chunkqueue,
    fn_0: *const buffer,
    mut offset: off_t,
    mut len: off_t,
) {
    if len > 0 as libc::c_int as libc::c_long {
        chunkqueue_append_file_chunk(cq, fn_0, offset, len);
    }
}
unsafe extern "C" fn chunkqueue_append_mem_extend_chunk(
    cq: *mut chunkqueue,
    mem: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut c: *mut chunk = (*cq).last;
    if 0 as libc::c_int as libc::c_ulong == len {
        return 1 as libc::c_int;
    }
    if !c.is_null()
        && (*c).type_0 as libc::c_uint == MEM_CHUNK as libc::c_int as libc::c_uint
        && buffer_string_space((*c).mem) as libc::c_ulong >= len
    {
        buffer_append_string_len((*c).mem, mem, len);
        (*cq)
            .bytes_in = ((*cq).bytes_in as libc::c_ulong).wrapping_add(len) as off_t
            as off_t;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_buffer(
    cq: *mut chunkqueue,
    mem: *mut buffer,
) {
    let mut c: *mut chunk = 0 as *mut chunk;
    let len: size_t = buffer_clen(mem) as size_t;
    if len < 1024 as libc::c_int as libc::c_ulong
        && chunkqueue_append_mem_extend_chunk(cq, (*mem).ptr, len) != 0
    {
        buffer_clear(mem);
        return;
    }
    c = chunkqueue_append_mem_chunk(cq, chunk_buf_sz);
    (*cq)
        .bytes_in = ((*cq).bytes_in as libc::c_ulong).wrapping_add(len) as off_t
        as off_t;
    buffer_move((*c).mem, mem);
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_mem(
    cq: *mut chunkqueue,
    mem: *const libc::c_char,
    mut len: size_t,
) {
    let mut c: *mut chunk = 0 as *mut chunk;
    if len < chunk_buf_sz && chunkqueue_append_mem_extend_chunk(cq, mem, len) != 0 {
        return;
    }
    c = chunkqueue_append_mem_chunk(
        cq,
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    (*cq)
        .bytes_in = ((*cq).bytes_in as libc::c_ulong).wrapping_add(len) as off_t
        as off_t;
    buffer_copy_string_len((*c).mem, mem, len);
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_mem_min(
    cq: *mut chunkqueue,
    mem: *const libc::c_char,
    mut len: size_t,
) {
    let mut c: *mut chunk = 0 as *mut chunk;
    if len < chunk_buf_sz && chunkqueue_append_mem_extend_chunk(cq, mem, len) != 0 {
        return;
    }
    c = chunk_init_sz(len.wrapping_add(1 as libc::c_int as libc::c_ulong));
    chunkqueue_append_chunk(cq, c);
    (*cq)
        .bytes_in = ((*cq).bytes_in as libc::c_ulong).wrapping_add(len) as off_t
        as off_t;
    buffer_copy_string_len((*c).mem, mem, len);
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_chunkqueue(
    cq: *mut chunkqueue,
    src: *mut chunkqueue,
) {
    if ((*src).first).is_null() {
        return;
    }
    if ((*cq).first).is_null() {
        (*cq).first = (*src).first;
    } else {
        (*(*cq).last).next = (*src).first;
    }
    (*cq).last = (*src).last;
    (*cq).bytes_in += chunkqueue_length(src);
    (*src).first = 0 as *mut chunk;
    (*src).last = 0 as *mut chunk;
    (*src).bytes_out = (*src).bytes_in;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_prepend_buffer_open_sz(
    mut cq: *mut chunkqueue,
    mut sz: size_t,
) -> *mut buffer {
    let c: *mut chunk = chunkqueue_prepend_mem_chunk(cq, sz);
    return (*c).mem;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_prepend_buffer_open(
    mut cq: *mut chunkqueue,
) -> *mut buffer {
    return chunkqueue_prepend_buffer_open_sz(cq, chunk_buf_sz);
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_prepend_buffer_commit(mut cq: *mut chunkqueue) {
    (*cq).bytes_in += buffer_clen((*(*cq).first).mem) as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_buffer_open_sz(
    mut cq: *mut chunkqueue,
    mut sz: size_t,
) -> *mut buffer {
    let c: *mut chunk = chunkqueue_append_mem_chunk(cq, sz);
    return (*c).mem;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_buffer_open(
    mut cq: *mut chunkqueue,
) -> *mut buffer {
    return chunkqueue_append_buffer_open_sz(cq, chunk_buf_sz);
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_buffer_commit(mut cq: *mut chunkqueue) {
    (*cq).bytes_in += buffer_clen((*(*cq).last).mem) as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_get_memory(
    cq: *mut chunkqueue,
    len: *mut size_t,
) -> *mut libc::c_char {
    let mut sz: size_t = if *len != 0 { *len } else { chunk_buf_sz >> 1 as libc::c_int };
    let mut b: *mut buffer = 0 as *mut buffer;
    let mut c: *mut chunk = (*cq).last;
    if !c.is_null()
        && MEM_CHUNK as libc::c_int as libc::c_uint == (*c).type_0 as libc::c_uint
    {
        let mut avail: size_t = buffer_string_space((*c).mem) as size_t;
        if avail >= sz {
            *len = avail;
            b = (*c).mem;
            return ((*b).ptr).offset(buffer_clen(b) as isize);
        }
    }
    b = chunkqueue_append_buffer_open_sz(cq, sz);
    *len = buffer_string_space(b) as size_t;
    return (*b).ptr;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_use_memory(
    cq: *mut chunkqueue,
    mut ckpt: *mut chunk,
    mut len: size_t,
) {
    let mut b: *mut buffer = (*(*cq).last).mem;
    if len > 0 as libc::c_int as libc::c_ulong {
        buffer_commit(b, len);
        (*cq)
            .bytes_in = ((*cq).bytes_in as libc::c_ulong).wrapping_add(len) as off_t
            as off_t;
        if (*cq).last == ckpt || ckpt.is_null()
            || MEM_CHUNK as libc::c_int as libc::c_uint != (*ckpt).type_0 as libc::c_uint
            || len > buffer_string_space((*ckpt).mem) as libc::c_ulong
        {
            return;
        }
        buffer_append_buffer((*ckpt).mem, b);
    } else if buffer_is_blank(b) == 0 {
        return
    }
    chunk_release((*cq).last);
    (*cq).last = ckpt;
    let ref mut fresh1 = *if !ckpt.is_null() {
        &mut (*ckpt).next
    } else {
        &mut (*cq).first
    };
    *fresh1 = 0 as *mut chunk;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_update_file(
    cq: *mut chunkqueue,
    mut c: *mut chunk,
    mut len: off_t,
) {
    (*c).file.length += len;
    (*cq).bytes_in += len;
    if 0 as libc::c_int as libc::c_long == chunk_remaining_length(c) {
        chunkqueue_remove_empty_chunks(cq);
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn chunkqueue_set_tempdirs_default(
    mut tempdirs: *const array,
    mut upload_temp_file_size: off_t,
) {
    if upload_temp_file_size == 0 as libc::c_int as libc::c_long {
        upload_temp_file_size = (1 as libc::c_int * 1024 as libc::c_int
            * 1024 as libc::c_int) as off_t;
    }
    chunkqueue_default_tempdirs = tempdirs;
    chunkqueue_default_tempfile_size = upload_temp_file_size;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_set_tempdirs(
    cq: *mut chunkqueue,
    tempdirs: *const array,
    mut upload_temp_file_size: off_t,
) {
    if upload_temp_file_size == 0 as libc::c_int as libc::c_long {
        upload_temp_file_size = chunkqueue_default_tempfile_size;
    }
    (*cq).tempdirs = tempdirs;
    (*cq).upload_temp_file_size = upload_temp_file_size;
    (*cq).tempdir_idx = 0 as libc::c_int as libc::c_uint;
}
#[inline(never)]
unsafe extern "C" fn chunkqueue_dup_file_chunk_fd(d: *mut chunk, c: *const chunk) {
    if (*c).file.fd >= 0 as libc::c_int {
        if ((*c).file.refchg).is_some() {
            (*d).file.fd = (*c).file.fd;
            (*d).file.ref_0 = (*c).file.ref_0;
            (*d).file.refchg = (*c).file.refchg;
            ((*d).file.refchg)
                .expect("non-null function pointer")((*d).file.ref_0, 1 as libc::c_int);
        } else {
            (*d).file.fd = fdevent_dup_cloexec((*c).file.fd);
        }
    }
}
#[inline(never)]
unsafe extern "C" fn chunkqueue_steal_partial_file_chunk(
    dest: *mut chunkqueue,
    c: *const chunk,
    len: off_t,
) {
    chunkqueue_append_file(dest, (*c).mem, (*c).offset, len);
    chunkqueue_dup_file_chunk_fd((*dest).last, c);
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_steal(
    dest: *mut chunkqueue,
    src: *mut chunkqueue,
    mut len: off_t,
) {
    let mut clen: off_t = 0;
    while len > 0 as libc::c_int as libc::c_long {
        let c: *mut chunk = (*src).first;
        if (0 as *mut libc::c_void as *mut chunk == c) as libc::c_int as libc::c_long
            != 0
        {
            break;
        }
        clen = chunk_remaining_length(c);
        if len >= clen {
            (*src).first = (*c).next;
            if c == (*src).last {
                (*src).last = 0 as *mut chunk;
            }
            if (0 as libc::c_int as libc::c_long != clen) as libc::c_int as libc::c_long
                != 0
            {
                chunkqueue_append_chunk(dest, c);
                (*dest).bytes_in += clen;
            } else {
                chunk_release(c);
            }
        } else {
            match (*c).type_0 as libc::c_uint {
                0 => {
                    chunkqueue_append_mem(
                        dest,
                        ((*(*c).mem).ptr).offset((*c).offset as isize),
                        len as size_t,
                    );
                }
                1 => {
                    chunkqueue_steal_partial_file_chunk(dest, c, len);
                }
                _ => {}
            }
            (*c).offset += len;
            clen = len;
        }
        (*src).bytes_out += clen;
        len -= clen;
    }
}
unsafe extern "C" fn chunkqueue_get_append_mkstemp(
    b: *mut buffer,
    mut path: *const libc::c_char,
    len: uint32_t,
) -> libc::c_int {
    buffer_copy_path_len2(
        b,
        path,
        len as size_t,
        b"lighttpd-upload-XXXXXX\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    return fdevent_mkostemp((*b).ptr, 0 as libc::c_int);
}
unsafe extern "C" fn chunkqueue_get_append_newtempfile(
    cq: *mut chunkqueue,
    errh: *mut log_error_st,
) -> *mut chunk {
    static mut emptyb: buffer = {
        let mut init = buffer {
            ptr: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            used: 0 as libc::c_int as uint32_t,
            size: 0 as libc::c_int as uint32_t,
        };
        init
    };
    let last: *mut chunk = (*cq).last;
    let c: *mut chunk = chunkqueue_append_file_chunk(
        cq,
        &emptyb,
        0 as libc::c_int as off_t,
        0 as libc::c_int as off_t,
    );
    let template: *mut buffer = (*c).mem;
    (*c).file.is_temp = 1 as libc::c_int;
    if !((*cq).tempdirs).is_null() && (*(*cq).tempdirs).used != 0 {
        *__errno_location() = 5 as libc::c_int;
        while (*cq).tempdir_idx < (*(*cq).tempdirs).used {
            let mut ds: *mut data_string = *((*(*cq).tempdirs).data)
                .offset((*cq).tempdir_idx as isize) as *mut data_string;
            (*c)
                .file
                .fd = chunkqueue_get_append_mkstemp(
                template,
                (*ds).value.ptr,
                buffer_clen(&mut (*ds).value),
            );
            if -(1 as libc::c_int) != (*c).file.fd {
                return c;
            }
            (*cq).tempdir_idx = ((*cq).tempdir_idx).wrapping_add(1);
        }
    } else {
        (*c)
            .file
            .fd = chunkqueue_get_append_mkstemp(
            template,
            b"/var/tmp\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        if -(1 as libc::c_int) != (*c).file.fd {
            return c;
        }
    }
    log_perror(
        errh,
        b"src/chunk.c\0" as *const u8 as *const libc::c_char,
        668 as libc::c_int as libc::c_uint,
        b"opening temp-file failed: %s\0" as *const u8 as *const libc::c_char,
        (*template).ptr,
    );
    (*c).file.is_temp = 0 as libc::c_int;
    (*cq).last = last;
    if !((*cq).last).is_null() {
        (*last).next = 0 as *mut chunk;
    } else {
        (*cq).first = 0 as *mut chunk;
    }
    chunk_release(c);
    return 0 as *mut chunk;
}
unsafe extern "C" fn chunkqueue_get_append_tempfile(
    cq: *mut chunkqueue,
    errh: *mut log_error_st,
) -> *mut chunk {
    let c: *mut chunk = (*cq).last;
    if !c.is_null() && (*c).file.is_temp != 0 && (*c).file.fd >= 0 as libc::c_int {
        if (*c).file.length < (*cq).upload_temp_file_size {
            return c;
        }
        if ((*c).file.refchg).is_some() {
            ck_assert_failed(
                b"src/chunk.c\0" as *const u8 as *const libc::c_char,
                696 as libc::c_int as libc::c_uint,
                b"0 == c->file.refchg\0" as *const u8 as *const libc::c_char,
            );
        }
        let mut rc: libc::c_int = close((*c).file.fd);
        (*c).file.fd = -(1 as libc::c_int);
        if 0 as libc::c_int != rc {
            log_perror(
                errh,
                b"src/chunk.c\0" as *const u8 as *const libc::c_char,
                700 as libc::c_int as libc::c_uint,
                b"close() temp-file %s failed\0" as *const u8 as *const libc::c_char,
                (*(*c).mem).ptr,
            );
            return 0 as *mut chunk;
        }
    }
    return chunkqueue_get_append_newtempfile(cq, errh);
}
#[cold]
unsafe extern "C" fn chunkqueue_append_tempfile_err(
    cq: *mut chunkqueue,
    errh: *mut log_error_st,
    c: *mut chunk,
) -> libc::c_int {
    let errnum: libc::c_int = *__errno_location();
    if errnum == 4 as libc::c_int {
        return 1 as libc::c_int;
    }
    let mut retry: libc::c_int = (errnum == 28 as libc::c_int
        && !((*cq).tempdirs).is_null()
        && {
            (*cq).tempdir_idx = ((*cq).tempdir_idx).wrapping_add(1);
            (*cq).tempdir_idx < (*(*cq).tempdirs).used
        }) as libc::c_int;
    if retry == 0 {
        log_perror(
            errh,
            b"src/chunk.c\0" as *const u8 as *const libc::c_char,
            716 as libc::c_int as libc::c_uint,
            b"write() temp-file %s failed\0" as *const u8 as *const libc::c_char,
            (*(*c).mem).ptr,
        );
    }
    if 0 as libc::c_int as libc::c_long == chunk_remaining_length(c) {
        chunkqueue_remove_empty_chunks(cq);
    } else {
        if ((*c).file.refchg).is_some() {
            ck_assert_failed(
                b"src/chunk.c\0" as *const u8 as *const libc::c_char,
                724 as libc::c_int as libc::c_uint,
                b"0 == c->file.refchg\0" as *const u8 as *const libc::c_char,
            );
        }
        let mut rc: libc::c_int = close((*c).file.fd);
        (*c).file.fd = -(1 as libc::c_int);
        if 0 as libc::c_int != rc {
            log_perror(
                errh,
                b"src/chunk.c\0" as *const u8 as *const libc::c_char,
                728 as libc::c_int as libc::c_uint,
                b"close() temp-file %s failed\0" as *const u8 as *const libc::c_char,
                (*(*c).mem).ptr,
            );
            retry = 0 as libc::c_int;
        }
    }
    return retry;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn chunkqueue_to_tempfiles(
    dest: *mut chunkqueue,
    errh: *mut log_error_st,
) -> libc::c_int {
    let cqlen: off_t = chunkqueue_length(dest);
    let mut src: chunkqueue = *dest;
    (*dest).last = 0 as *mut chunk;
    (*dest).first = (*dest).last;
    (*dest).bytes_in -= cqlen;
    if 0 as libc::c_int == chunkqueue_steal_with_tempfiles(dest, &mut src, cqlen, errh) {
        return 0 as libc::c_int
    } else {
        let errnum: libc::c_int = *__errno_location();
        chunkqueue_release_chunks(&mut src);
        return -errnum;
    };
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_mem_to_tempfile(
    dest: *mut chunkqueue,
    mut mem: *const libc::c_char,
    mut len: size_t,
    errh: *mut log_error_st,
) -> libc::c_int {
    let mut dst_c: *mut chunk = (*dest).first;
    if !dst_c.is_null()
        && (*dst_c).type_0 as libc::c_uint == MEM_CHUNK as libc::c_int as libc::c_uint
        && 0 as libc::c_int != chunkqueue_to_tempfiles(dest, errh)
    {
        return -(1 as libc::c_int);
    }
    loop {
        dst_c = chunkqueue_get_append_tempfile(dest, errh);
        if dst_c.is_null() {
            return -(1 as libc::c_int);
        }
        let written: ssize_t = pwrite(
            (*dst_c).file.fd,
            mem as *const libc::c_void,
            len,
            (*dst_c).file.length,
        );
        if written as size_t == len {
            (*dst_c)
                .file
                .length = ((*dst_c).file.length as libc::c_ulong).wrapping_add(len)
                as off_t as off_t;
            (*dest)
                .bytes_in = ((*dest).bytes_in as libc::c_ulong).wrapping_add(len)
                as off_t as off_t;
            return 0 as libc::c_int;
        } else {
            if written >= 0 as libc::c_int as libc::c_long {
                (*dest).bytes_in += written;
                mem = mem.offset(written as isize);
                len = (len as libc::c_ulong).wrapping_sub(written as size_t) as size_t
                    as size_t;
                (*dst_c)
                    .file
                    .length = ((*dst_c).file.length as libc::c_ulong)
                    .wrapping_add(written as size_t) as off_t as off_t;
            } else if chunkqueue_append_tempfile_err(dest, errh, dst_c) == 0 {
                break;
            }
            if !(len != 0) {
                break;
            }
        }
    }
    return -(1 as libc::c_int);
}
#[cold]
#[inline(never)]
unsafe extern "C" fn chunkqueue_append_cqmem_to_tempfile_partial(
    dest: *mut chunkqueue,
    c: *mut chunk,
    mut wr: ssize_t,
    errh: *mut log_error_st,
) -> ssize_t {
    let mut ckpt: *mut chunk = (*dest).first;
    while (*ckpt).next != c {
        ckpt = (*ckpt).next;
    }
    (*ckpt).next = 0 as *mut chunk;
    (*dest).last = ckpt;
    (*dest).bytes_in -= wr;
    (*dest).bytes_out -= wr;
    chunkqueue_mark_written(dest, wr);
    (*c).next = (*dest).first;
    (*dest).first = c;
    return (if 0 as libc::c_int == chunkqueue_to_tempfiles(dest, errh) {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    }) as ssize_t;
}
unsafe extern "C" fn chunkqueue_append_cqmem_to_tempfile(
    dest: *mut chunkqueue,
    src: *mut chunkqueue,
    mut len: off_t,
    errh: *mut log_error_st,
) -> ssize_t {
    let mut iovcnt: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut iov: [iovec; 16] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 16];
    let mut dlen: off_t = 0 as libc::c_int as off_t;
    let mut c: *mut chunk = 0 as *mut chunk;
    c = (*dest).first;
    while !c.is_null()
        && (*c).type_0 as libc::c_uint == MEM_CHUNK as libc::c_int as libc::c_uint
    {
        let clen: off_t = chunk_remaining_length(c);
        iov[iovcnt as usize]
            .iov_base = ((*(*c).mem).ptr).offset((*c).offset as isize)
            as *mut libc::c_void;
        iov[iovcnt as usize].iov_len = clen as size_t;
        dlen += clen;
        iovcnt = iovcnt.wrapping_add(1);
        if (iovcnt as libc::c_ulong
            == (::std::mem::size_of::<[iovec; 16]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<iovec>() as libc::c_ulong))
            as libc::c_int as libc::c_long != 0
        {
            break;
        }
        c = (*c).next;
    }
    if (c != 0 as *mut libc::c_void as *mut chunk) as libc::c_int as libc::c_long != 0
        && (*(*dest).first).type_0 as libc::c_uint
            == MEM_CHUNK as libc::c_int as libc::c_uint
    {
        if 0 as libc::c_int != chunkqueue_to_tempfiles(dest, errh) {
            return -(1 as libc::c_int) as ssize_t;
        }
        dlen = 0 as libc::c_int as off_t;
        iovcnt = 0 as libc::c_int as libc::c_uint;
    }
    if ((iovcnt as libc::c_ulong)
        < (::std::mem::size_of::<[iovec; 16]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<iovec>() as libc::c_ulong))
        as libc::c_int as libc::c_long != 0
    {
        c = (*src).first;
        while !c.is_null()
            && (*c).type_0 as libc::c_uint == MEM_CHUNK as libc::c_int as libc::c_uint
        {
            let mut clen_0: off_t = chunk_remaining_length(c);
            if clen_0 > len {
                clen_0 = len;
            }
            iov[iovcnt as usize]
                .iov_base = ((*(*c).mem).ptr).offset((*c).offset as isize)
                as *mut libc::c_void;
            iov[iovcnt as usize].iov_len = clen_0 as size_t;
            len -= clen_0;
            iovcnt = iovcnt.wrapping_add(1);
            if 0 as libc::c_int as libc::c_long == len {
                break;
            }
            if (iovcnt as libc::c_ulong
                == (::std::mem::size_of::<[iovec; 16]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<iovec>() as libc::c_ulong))
                as libc::c_int as libc::c_long != 0
            {
                break;
            }
            c = (*c).next;
        }
    }
    if (0 as libc::c_int as libc::c_uint == iovcnt) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as ssize_t;
    }
    c = chunkqueue_get_append_tempfile(dest, errh);
    if c.is_null() {
        return -(1 as libc::c_int) as ssize_t;
    }
    let mut wr: ssize_t = pwritev(
        (*c).file.fd,
        iov.as_mut_ptr(),
        iovcnt as libc::c_int,
        (*c).file.length,
    );
    if wr >= 0 as libc::c_int as libc::c_long {
        (*c).file.length += wr;
        (*dest).bytes_in += wr;
        if dlen != 0 {
            if (wr < dlen) as libc::c_int as libc::c_long != 0 {
                return chunkqueue_append_cqmem_to_tempfile_partial(dest, c, wr, errh);
            }
            wr -= dlen;
            (*dest).bytes_in -= dlen;
            (*dest).bytes_out -= dlen;
            chunkqueue_mark_written(dest, dlen);
        }
    }
    return wr;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn chunkqueue_append_drain_pipe_tempfile(
    cq: *mut chunkqueue,
    fd: libc::c_int,
    mut len: libc::c_uint,
    errh: *mut log_error_st,
) -> ssize_t {
    let mut buf: [libc::c_char; 16384] = [0; 16384];
    let mut rd: ssize_t = 0;
    loop {
        loop {
            rd = read(
                fd,
                buf.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong,
            );
            if !(rd < 0 as libc::c_int as libc::c_long
                && *__errno_location() == 4 as libc::c_int)
            {
                break;
            }
        }
        if rd < 0 as libc::c_int as libc::c_long {
            break;
        }
        if 0 as libc::c_int
            != chunkqueue_append_mem_to_tempfile(
                cq,
                buf.as_mut_ptr(),
                rd as size_t,
                errh,
            )
        {
            break;
        }
        len = len.wrapping_sub(rd as libc::c_uint);
        if !(len != 0) {
            break;
        }
    }
    if 0 as libc::c_int as libc::c_uint == len {
        return 0 as libc::c_int as ssize_t
    } else {
        let errnum: libc::c_int = *__errno_location();
        if !((*cq).last).is_null()
            && 0 as libc::c_int as libc::c_long == chunk_remaining_length((*cq).last)
        {
            chunkqueue_remove_empty_chunks(cq);
        }
        return -errnum as ssize_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_splice_pipe_tempfile(
    cq: *mut chunkqueue,
    fd: libc::c_int,
    mut len: libc::c_uint,
    errh: *mut log_error_st,
) -> ssize_t {
    if !((*cq).first).is_null()
        && (*(*cq).first).type_0 as libc::c_uint
            == MEM_CHUNK as libc::c_int as libc::c_uint
    {
        let mut rc: libc::c_int = chunkqueue_to_tempfiles(cq, errh);
        if (0 as libc::c_int != rc) as libc::c_int as libc::c_long != 0 {
            return rc as ssize_t;
        }
    }
    let mut total: ssize_t = 0 as libc::c_int as ssize_t;
    loop {
        let c: *mut chunk = chunkqueue_get_append_tempfile(cq, errh);
        if (0 as *mut libc::c_void as *mut chunk == c) as libc::c_int as libc::c_long
            != 0
        {
            return -*__errno_location() as ssize_t;
        }
        let mut off: loff_t = (*c).file.length;
        let mut wr: ssize_t = splice(
            fd,
            0 as *mut __off64_t,
            (*c).file.fd,
            &mut off,
            len as size_t,
            (1 as libc::c_int | 2 as libc::c_int) as libc::c_uint,
        );
        if (wr as size_t == len as libc::c_ulong) as libc::c_int as libc::c_long != 0 {
            (*c).file.length += len as libc::c_long;
            (*cq).bytes_in += len as libc::c_long;
            return total + len as libc::c_long;
        } else {
            if wr >= 0 as libc::c_int as libc::c_long {
                (*cq).bytes_in += wr;
                total += wr;
                len = (len as libc::c_ulong).wrapping_sub(wr as size_t) as libc::c_uint
                    as libc::c_uint;
                (*c)
                    .file
                    .length = ((*c).file.length as libc::c_ulong)
                    .wrapping_add(wr as size_t) as off_t as off_t;
            } else {
                let errnum: libc::c_int = *__errno_location();
                match errnum {
                    11 => {
                        if 0 as libc::c_int as libc::c_long == chunk_remaining_length(c)
                        {
                            chunkqueue_remove_empty_chunks(cq);
                        }
                        return total;
                    }
                    22 => {
                        wr = chunkqueue_append_drain_pipe_tempfile(cq, fd, len, errh);
                        return if 0 as libc::c_int as libc::c_long == wr {
                            total + len as ssize_t
                        } else {
                            wr
                        };
                    }
                    _ => {
                        if chunkqueue_append_tempfile_err(cq, errh, c) == 0 {
                            return -errnum as ssize_t;
                        }
                    }
                }
            }
        }
        if !(len != 0) {
            break;
        }
    }
    return -(5 as libc::c_int) as ssize_t;
}
static mut cqpipes: [libc::c_int; 2] = [-(1 as libc::c_int), -(1 as libc::c_int)];
#[no_mangle]
#[cold]
#[inline(never)]
pub unsafe extern "C" fn chunkqueue_internal_pipes(mut init: libc::c_int) {
    if -(1 as libc::c_int) != cqpipes[0 as libc::c_int as usize] {
        close(cqpipes[0 as libc::c_int as usize]);
        cqpipes[0 as libc::c_int as usize] = -(1 as libc::c_int);
    }
    if -(1 as libc::c_int) != cqpipes[1 as libc::c_int as usize] {
        close(cqpipes[1 as libc::c_int as usize]);
        cqpipes[1 as libc::c_int as usize] = -(1 as libc::c_int);
    }
    if init != 0 {
        0 as libc::c_int
            != fdevent_pipe_cloexec(
                cqpipes.as_mut_ptr(),
                262144 as libc::c_int as libc::c_uint,
            );
    }
}
#[cold]
#[inline(never)]
unsafe extern "C" fn chunkqueue_pipe_read_discard() {
    let mut buf: [libc::c_char; 16384] = [0; 16384];
    let mut rd: ssize_t = 0;
    loop {
        rd = read(
            cqpipes[0 as libc::c_int as usize],
            buf.as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong,
        );
        if !(rd > 0 as libc::c_int as libc::c_long
            || rd < 0 as libc::c_int as libc::c_long
                && *__errno_location() == 4 as libc::c_int)
        {
            break;
        }
    }
    if rd < 0 as libc::c_int as libc::c_long && *__errno_location() != 11 as libc::c_int
    {
        chunkqueue_internal_pipes(1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_splice_sock_tempfile(
    cq: *mut chunkqueue,
    fd: libc::c_int,
    mut len: libc::c_uint,
    errh: *mut log_error_st,
) -> ssize_t {
    let pipes: *mut libc::c_int = cqpipes.as_mut_ptr();
    if -(1 as libc::c_int) == *pipes.offset(1 as libc::c_int as isize) {
        return -(22 as libc::c_int) as ssize_t;
    }
    let mut wr: ssize_t = splice(
        fd,
        0 as *mut __off64_t,
        *pipes.offset(1 as libc::c_int as isize),
        0 as *mut __off64_t,
        len as size_t,
        (1 as libc::c_int | 2 as libc::c_int) as libc::c_uint,
    );
    if (wr <= 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long != 0 {
        return -(22 as libc::c_int) as ssize_t;
    }
    len = wr as libc::c_uint;
    wr = chunkqueue_append_splice_pipe_tempfile(
        cq,
        *pipes.offset(0 as libc::c_int as isize),
        len,
        errh,
    );
    if wr < 0 as libc::c_int as libc::c_long {
        chunkqueue_pipe_read_discard();
    }
    return wr;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_steal_with_tempfiles(
    dest: *mut chunkqueue,
    src: *mut chunkqueue,
    mut len: off_t,
    errh: *mut log_error_st,
) -> libc::c_int {
    let mut clen: off_t = 0;
    while len > 0 as libc::c_int as libc::c_long {
        let c: *mut chunk = (*src).first;
        if (0 as *mut libc::c_void as *mut chunk == c) as libc::c_int as libc::c_long
            != 0
        {
            break;
        }
        if (*c).type_0 as libc::c_uint == MEM_CHUNK as libc::c_int as libc::c_uint {
            clen = chunkqueue_append_cqmem_to_tempfile(dest, src, len, errh);
            if (clen < 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long
                != 0
            {
                return -(1 as libc::c_int);
            }
            chunkqueue_mark_written(src, clen);
        } else {
            clen = chunk_remaining_length(c);
            if len < clen {
                clen = len;
            }
            chunkqueue_steal(dest, src, clen);
        }
        len -= clen;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_cq_range(
    dst: *mut chunkqueue,
    src: *const chunkqueue,
    mut offset: off_t,
    mut len: off_t,
) {
    let mut c: *const chunk = (*src).first;
    while len > 0 as libc::c_int as libc::c_long && !c.is_null() {
        let mut clen: off_t = chunk_remaining_length(c);
        if offset >= clen {
            offset -= clen;
        } else {
            clen -= offset;
            if len < clen {
                clen = len;
            }
            len -= clen;
            if (*c).type_0 as libc::c_uint == FILE_CHUNK as libc::c_int as libc::c_uint {
                chunkqueue_append_file(dst, (*c).mem, (*c).offset + offset, clen);
                chunkqueue_dup_file_chunk_fd((*dst).last, c);
            } else {
                chunkqueue_append_mem(
                    dst,
                    ((*(*c).mem).ptr)
                        .offset((*c).offset as isize)
                        .offset(offset as isize),
                    clen as size_t,
                );
            }
            offset = 0 as libc::c_int as off_t;
        }
        c = (*c).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_mark_written(
    mut cq: *mut chunkqueue,
    mut len: off_t,
) {
    (*cq).bytes_out += len;
    let mut c: *mut chunk = (*cq).first;
    while !c.is_null() {
        let mut c_len: off_t = chunk_remaining_length(c);
        if len >= c_len {
            let x: *mut chunk = c;
            c = (*c).next;
            len -= c_len;
            chunk_release(x);
        } else {
            (*c).offset += len;
            (*cq).first = c;
            return;
        }
    }
    (*cq).last = 0 as *mut chunk;
    (*cq).first = (*cq).last;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_remove_finished_chunks(mut cq: *mut chunkqueue) {
    let mut c: *mut chunk = 0 as *mut chunk;
    loop {
        c = (*cq).first;
        if !(!c.is_null()
            && 0 as libc::c_int as libc::c_long == chunk_remaining_length(c))
        {
            break;
        }
        (*cq).first = (*c).next;
        if ((*cq).first).is_null() {
            (*cq).last = 0 as *mut chunk;
        }
        chunk_release(c);
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn chunkqueue_remove_empty_chunks(mut cq: *mut chunkqueue) {
    let mut c: *mut chunk = 0 as *mut chunk;
    chunkqueue_remove_finished_chunks(cq);
    c = (*cq).first;
    while !c.is_null() && !((*c).next).is_null() {
        if 0 as libc::c_int as libc::c_long == chunk_remaining_length((*c).next) {
            let mut empty: *mut chunk = (*c).next;
            (*c).next = (*empty).next;
            if empty == (*cq).last {
                (*cq).last = c;
            }
            chunk_release(empty);
        }
        c = (*c).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_compact_mem_offset(cq: *mut chunkqueue) {
    let c: *mut chunk = (*cq).first;
    if 0 as libc::c_int as libc::c_long == (*c).offset {
        return;
    }
    if (*c).type_0 as libc::c_uint != MEM_CHUNK as libc::c_int as libc::c_uint {
        return;
    }
    let b: *mut buffer = (*c).mem;
    let mut len: size_t = (buffer_clen(b) as libc::c_long - (*c).offset) as size_t;
    memmove(
        (*b).ptr as *mut libc::c_void,
        ((*b).ptr).offset((*c).offset as isize) as *const libc::c_void,
        len,
    );
    (*c).offset = 0 as libc::c_int as off_t;
    buffer_truncate(b, len as uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_compact_mem(
    mut cq: *mut chunkqueue,
    mut clen: size_t,
) {
    let mut c: *mut chunk = (*cq).first;
    let mut b: *mut buffer = (*c).mem;
    let mut len: size_t = (buffer_clen(b) as libc::c_long - (*c).offset) as size_t;
    if len >= clen {
        return;
    }
    if (*b).size as libc::c_ulong > clen {
        if (buffer_string_space(b) as libc::c_ulong) < clen.wrapping_sub(len) {
            chunkqueue_compact_mem_offset(cq);
        }
    } else {
        b = chunkqueue_prepend_buffer_open_sz(
            cq,
            clen.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        buffer_append_string_len(b, ((*(*c).mem).ptr).offset((*c).offset as isize), len);
        (*(*cq).first).next = (*c).next;
        if ((*c).next).is_null() {
            (*cq).last = (*cq).first;
        }
        chunk_release(c);
        c = (*cq).first;
    }
    let mut fc: *mut chunk = c;
    loop {
        clen = (clen as libc::c_ulong).wrapping_sub(len) as size_t as size_t;
        if !(clen != 0
            && {
                c = (*fc).next;
                !c.is_null()
            })
        {
            break;
        }
        len = (buffer_clen((*c).mem) as libc::c_long - (*c).offset) as size_t;
        if len > clen {
            buffer_append_string_len(
                b,
                ((*(*c).mem).ptr).offset((*c).offset as isize),
                clen,
            );
            (*c)
                .offset = ((*c).offset as libc::c_ulong).wrapping_add(clen) as off_t
                as off_t;
            break;
        } else {
            buffer_append_string_len(
                b,
                ((*(*c).mem).ptr).offset((*c).offset as isize),
                len,
            );
            (*fc).next = (*c).next;
            if ((*c).next).is_null() {
                (*cq).last = fc;
            }
            chunk_release(c);
        }
    };
}
unsafe extern "C" fn chunk_open_file_chunk(
    c: *mut chunk,
    errh: *mut log_error_st,
) -> libc::c_int {
    if -(1 as libc::c_int) == (*c).file.fd {
        (*c)
            .file
            .fd = fdevent_open_cloexec(
            (*(*c).mem).ptr,
            1 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int as mode_t,
        );
        if -(1 as libc::c_int) == (*c).file.fd {
            log_perror(
                errh,
                b"src/chunk.c\0" as *const u8 as *const libc::c_char,
                1252 as libc::c_int as libc::c_uint,
                b"open failed: %s\0" as *const u8 as *const libc::c_char,
                (*(*c).mem).ptr,
            );
            return -(1 as libc::c_int);
        }
    }
    if (*c).file.is_temp != 0 {
        return 0 as libc::c_int;
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
    if -(1 as libc::c_int) == fstat((*c).file.fd, &mut st) {
        log_perror(
            errh,
            b"src/chunk.c\0" as *const u8 as *const libc::c_char,
            1262 as libc::c_int as libc::c_uint,
            b"fstat failed\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let offset: off_t = (*c).offset;
    let len: off_t = (*c).file.length - (*c).offset;
    if !(offset >= 0 as libc::c_int as libc::c_long
        && len >= 0 as libc::c_int as libc::c_long)
    {
        ck_assert_failed(
            b"src/chunk.c\0" as *const u8 as *const libc::c_char,
            1268 as libc::c_int as libc::c_uint,
            b"offset >= 0 && len >= 0\0" as *const u8 as *const libc::c_char,
        );
    }
    if offset > st.st_size - len {
        log_error(
            errh,
            b"src/chunk.c\0" as *const u8 as *const libc::c_char,
            1270 as libc::c_int as libc::c_uint,
            b"file shrunk: %s\0" as *const u8 as *const libc::c_char,
            (*(*c).mem).ptr,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_open_file_chunk(
    cq: *mut chunkqueue,
    errh: *mut log_error_st,
) -> libc::c_int {
    return chunk_open_file_chunk((*cq).first, errh);
}
unsafe extern "C" fn chunkqueue_write_data(
    fd: libc::c_int,
    mut buf: *const libc::c_void,
    mut count: size_t,
) -> ssize_t {
    let mut wr: ssize_t = 0;
    loop {
        wr = write(fd, buf, count);
        if !(-(1 as libc::c_int) as libc::c_long == wr
            && *__errno_location() == 4 as libc::c_int)
        {
            break;
        }
    }
    return wr;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn chunkqueue_write_chunk_file_intermed(
    fd: libc::c_int,
    c: *mut chunk,
    errh: *mut log_error_st,
) -> ssize_t {
    let mut buf: [libc::c_char; 16384] = [0; 16384];
    let mut data: *mut libc::c_char = buf.as_mut_ptr();
    let count: off_t = (*c).file.length - (*c).offset;
    let mut dlen: uint32_t = (if count
        < ::std::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong as off_t
    {
        count as uint32_t as libc::c_ulong
    } else {
        ::std::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong
    }) as uint32_t;
    let mut cq: chunkqueue = {
        let mut init = chunkqueue {
            first: c,
            last: c,
            bytes_in: 0 as libc::c_int as off_t,
            bytes_out: 0 as libc::c_int as off_t,
            tempdirs: 0 as *const array,
            upload_temp_file_size: 0 as libc::c_int as off_t,
            tempdir_idx: 0 as libc::c_int as libc::c_uint,
        };
        init
    };
    if 0 as libc::c_int != chunkqueue_peek_data(&mut cq, &mut data, &mut dlen, errh)
        && 0 as libc::c_int as libc::c_uint == dlen
    {
        return -(1 as libc::c_int) as ssize_t;
    }
    return chunkqueue_write_data(fd, data as *const libc::c_void, dlen as size_t);
}
unsafe extern "C" fn mmap_align_offset(mut start: off_t) -> off_t {
    static mut pagemask: off_t = 0 as libc::c_int as off_t;
    if 0 as libc::c_int as libc::c_long == pagemask {
        let mut pagesize: libc::c_long = sysconf(_SC_PAGESIZE as libc::c_int);
        if -(1 as libc::c_int) as libc::c_long == pagesize {
            pagesize = 4096 as libc::c_int as libc::c_long;
        }
        pagemask = !(pagesize - 1 as libc::c_int as libc::c_long);
    }
    return start & pagemask;
}
#[inline(never)]
unsafe extern "C" fn chunkqueue_mmap_chunk_len(
    c: *mut chunk,
    mut len: off_t,
) -> *mut libc::c_char {
    if -(1 as libc::c_int) as *mut libc::c_void
        == (*c).file.mmap.start as *mut libc::c_void
        || (*c).offset < (*c).file.mmap.offset
        || (*c).offset + len
            > ((*c).file.mmap.offset as libc::c_ulong)
                .wrapping_add((*c).file.mmap.length) as off_t
    {
        if -(1 as libc::c_int) as *mut libc::c_void
            != (*c).file.mmap.start as *mut libc::c_void
        {
            munmap((*c).file.mmap.start as *mut libc::c_void, (*c).file.mmap.length);
        }
        (*c).file.mmap.offset = mmap_align_offset((*c).offset);
        (*c).file.mmap.length = ((*c).file.length - (*c).file.mmap.offset) as size_t;
        (*c)
            .file
            .mmap
            .start = mmap(
            0 as *mut libc::c_void,
            (*c).file.mmap.length,
            0x1 as libc::c_int,
            0x2 as libc::c_int,
            (*c).file.fd,
            (*c).file.mmap.offset,
        ) as *mut libc::c_char;
        if -(1 as libc::c_int) as *mut libc::c_void
            == (*c).file.mmap.start as *mut libc::c_void
        {
            return 0 as *mut libc::c_char;
        }
    }
    return ((*c).file.mmap.start)
        .offset((*c).offset as isize)
        .offset(-((*c).file.mmap.offset as isize));
}
unsafe extern "C" fn chunkqueue_write_chunk_file(
    fd: libc::c_int,
    c: *mut chunk,
    errh: *mut log_error_st,
) -> ssize_t {
    if 0 as libc::c_int != chunk_open_file_chunk(c, errh) {
        return -(1 as libc::c_int) as ssize_t;
    }
    let count: off_t = (*c).file.length - (*c).offset;
    if 0 as libc::c_int as libc::c_long == count {
        return 0 as libc::c_int as ssize_t;
    }
    let mut offset: off_t = (*c).offset;
    let wr: ssize_t = sendfile(
        fd,
        (*c).file.fd,
        &mut offset,
        (if count < 2147483647 as libc::c_int as libc::c_long {
            count
        } else {
            2147483647 as libc::c_int as libc::c_long
        }) as size_t,
    );
    if (wr >= 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long != 0
        || *__errno_location() != 22 as libc::c_int
            && *__errno_location() != 38 as libc::c_int
    {
        return wr;
    }
    let data: *const libc::c_char = chunkqueue_mmap_chunk_len(c, count);
    if !data.is_null() {
        return chunkqueue_write_data(fd, data as *const libc::c_void, count as size_t);
    }
    return chunkqueue_write_chunk_file_intermed(fd, c, errh);
}
unsafe extern "C" fn chunkqueue_write_chunk_mem(
    fd: libc::c_int,
    c: *const chunk,
) -> ssize_t {
    let buf: *const libc::c_void = ((*(*c).mem).ptr).offset((*c).offset as isize)
        as *const libc::c_void;
    let count: size_t = (buffer_clen((*c).mem) as libc::c_ulong)
        .wrapping_sub((*c).offset as size_t);
    let mut wr: ssize_t = 0;
    loop {
        wr = write(fd, buf, count);
        if !(-(1 as libc::c_int) as libc::c_long == wr
            && *__errno_location() == 4 as libc::c_int)
        {
            break;
        }
    }
    return wr;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_write_chunk(
    fd: libc::c_int,
    cq: *mut chunkqueue,
    errh: *mut log_error_st,
) -> ssize_t {
    let c: *mut chunk = (*cq).first;
    match (*c).type_0 as libc::c_uint {
        0 => return chunkqueue_write_chunk_mem(fd, c),
        1 => return chunkqueue_write_chunk_file(fd, c, errh),
        _ => {
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int) as ssize_t;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_write_chunk_to_pipe(
    fd: libc::c_int,
    cq: *mut chunkqueue,
    errh: *mut log_error_st,
) -> ssize_t {
    let c: *mut chunk = (*cq).first;
    if (*c).type_0 as libc::c_uint == FILE_CHUNK as libc::c_int as libc::c_uint {
        let mut abs_offset: loff_t = (*c).offset;
        return if 0 as libc::c_int == chunk_open_file_chunk(c, errh) {
            splice(
                (*c).file.fd,
                &mut abs_offset,
                fd,
                0 as *mut __off64_t,
                ((*c).file.length - (*c).offset) as size_t,
                2 as libc::c_int as libc::c_uint,
            )
        } else {
            -(1 as libc::c_int) as libc::c_long
        };
    }
    return chunkqueue_write_chunk(fd, cq, errh);
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_small_resp_optim(cq: *mut chunkqueue) {
    let mut c: *mut chunk = (*cq).first;
    let filec: *mut chunk = (*c).next;
    if filec != (*cq).last
        || (*filec).type_0 as libc::c_uint != FILE_CHUNK as libc::c_int as libc::c_uint
        || (*filec).file.fd < 0 as libc::c_int
    {
        return;
    }
    let mut len: off_t = (*filec).file.length - (*filec).offset;
    if len as size_t > buffer_string_space((*c).mem) as libc::c_ulong {
        (*c)
            .next = chunk_acquire(
            (len as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        c = (*c).next;
    }
    (*c).next = 0 as *mut chunk;
    (*cq).last = c;
    let mut rd: ssize_t = 0;
    let mut offset: off_t = 0 as libc::c_int as off_t;
    let ptr: *mut libc::c_char = buffer_extend((*c).mem, len as size_t);
    loop {
        rd = pread(
            (*filec).file.fd,
            ptr.offset(offset as isize) as *mut libc::c_void,
            len as size_t,
            (*filec).offset + offset,
        );
        if !(if rd > 0 as libc::c_int as libc::c_long {
            offset += rd;
            len -= rd;
            len
        } else {
            (*__errno_location() == 4 as libc::c_int) as libc::c_int as libc::c_long
        } != 0)
        {
            break;
        }
    }
    if (0 as libc::c_int as libc::c_long == len) as libc::c_int as libc::c_long != 0 {
        chunk_release(filec);
    } else {
        buffer_truncate(
            (*c).mem,
            ptr.offset(offset as isize).offset_from((*(*c).mem).ptr) as libc::c_long
                as uint32_t,
        );
        (*c).next = filec;
        (*cq).last = (*c).next;
        if offset != 0 {
            (*filec).offset += offset;
        } else if ((*cq).first != c) as libc::c_int as libc::c_long != 0 {
            (*(*cq).first).next = filec;
            chunk_release(c);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_peek_data(
    cq: *mut chunkqueue,
    data: *mut *mut libc::c_char,
    dlen: *mut uint32_t,
    errh: *mut log_error_st,
) -> libc::c_int {
    let data_in: *mut libc::c_char = *data;
    let data_insz: uint32_t = *dlen;
    *dlen = 0 as libc::c_int as uint32_t;
    let mut c: *mut chunk = (*cq).first;
    while !c.is_null() {
        let mut space: uint32_t = data_insz.wrapping_sub(*dlen);
        match (*c).type_0 as libc::c_uint {
            0 => {
                let mut have: uint32_t = (buffer_clen((*c).mem))
                    .wrapping_sub((*c).offset as uint32_t);
                if have > space {
                    have = space;
                }
                if *dlen != 0 {
                    memcpy(
                        data_in.offset(*dlen as isize) as *mut libc::c_void,
                        ((*(*c).mem).ptr).offset((*c).offset as isize)
                            as *const libc::c_void,
                        have as libc::c_ulong,
                    );
                } else {
                    *data = ((*(*c).mem).ptr).offset((*c).offset as isize);
                }
                *dlen = (*dlen as libc::c_uint).wrapping_add(have) as uint32_t
                    as uint32_t;
            }
            1 => {
                if (*c).file.fd >= 0 as libc::c_int
                    || 0 as libc::c_int == chunk_open_file_chunk(c, errh)
                {
                    let mut offset: off_t = (*c).offset;
                    let mut len: off_t = (*c).file.length - (*c).offset;
                    if len > space as off_t {
                        len = space as off_t;
                    }
                    if !(0 as libc::c_int as libc::c_long == len) {
                        let mut rd: ssize_t = 0;
                        loop {
                            rd = pread(
                                (*c).file.fd,
                                data_in.offset(*dlen as isize) as *mut libc::c_void,
                                len as size_t,
                                offset,
                            );
                            if !(-(1 as libc::c_int) as libc::c_long == rd
                                && *__errno_location() == 4 as libc::c_int)
                            {
                                break;
                            }
                        }
                        if rd <= 0 as libc::c_int as libc::c_long {
                            log_perror(
                                errh,
                                b"src/chunk.c\0" as *const u8 as *const libc::c_char,
                                1604 as libc::c_int as libc::c_uint,
                                b"read(\"%s\")\0" as *const u8 as *const libc::c_char,
                                (*(*c).mem).ptr,
                            );
                            return -(1 as libc::c_int);
                        }
                        *dlen = (*dlen as libc::c_uint).wrapping_add(rd as uint32_t)
                            as uint32_t as uint32_t;
                    }
                } else {
                    return -(1 as libc::c_int)
                }
            }
            _ => return -(1 as libc::c_int),
        }
        if *dlen == data_insz {
            break;
        }
        c = (*c).next;
        if c.is_null() {
            break;
        }
        if *dlen != 0 && *data != data_in {
            memcpy(
                data_in as *mut libc::c_void,
                *data as *const libc::c_void,
                *dlen as libc::c_ulong,
            );
            *data = data_in;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_read_data(
    cq: *mut chunkqueue,
    data: *mut libc::c_char,
    dlen: uint32_t,
    errh: *mut log_error_st,
) -> libc::c_int {
    let mut ptr: *mut libc::c_char = data;
    let mut len: uint32_t = dlen;
    if chunkqueue_peek_data(cq, &mut ptr, &mut len, errh) < 0 as libc::c_int
        || len != dlen
    {
        return -(1 as libc::c_int);
    }
    if data != ptr {
        memcpy(
            data as *mut libc::c_void,
            ptr as *const libc::c_void,
            len as libc::c_ulong,
        );
    }
    chunkqueue_mark_written(cq, len as off_t);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_read_squash(
    cq: *mut chunkqueue,
    errh: *mut log_error_st,
) -> *mut buffer {
    let mut cqlen: off_t = chunkqueue_length(cq);
    if cqlen >= 4294967295 as libc::c_uint as libc::c_long {
        return 0 as *mut buffer;
    }
    if !((*cq).first).is_null() && ((*(*cq).first).next).is_null()
        && (*(*cq).first).type_0 as libc::c_uint
            == MEM_CHUNK as libc::c_int as libc::c_uint
    {
        return (*(*cq).first).mem;
    }
    let c: *mut chunk = chunk_acquire(
        (cqlen as uint32_t).wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
    );
    let mut data: *mut libc::c_char = (*(*c).mem).ptr;
    let mut dlen: uint32_t = cqlen as uint32_t;
    let mut rc: libc::c_int = chunkqueue_peek_data(cq, &mut data, &mut dlen, errh);
    if rc < 0 as libc::c_int {
        chunk_release(c);
        return 0 as *mut buffer;
    }
    buffer_truncate((*c).mem, dlen);
    chunkqueue_release_chunks(cq);
    chunkqueue_append_chunk(cq, c);
    return (*c).mem;
}
