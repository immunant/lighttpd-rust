use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn rewind(__stream: *mut FILE);
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn free(_: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type intptr_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action {
    pub sp: *mut symbol,
    pub type_0: e_action,
    pub x: C2RustUnnamed_0,
    pub next: *mut action,
    pub collide: *mut action,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub stp: *mut state,
    pub rp: *mut rule,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rule {
    pub lhs: *mut symbol,
    pub lhsalias: *mut libc::c_char,
    pub ruleline: libc::c_int,
    pub nrhs: libc::c_int,
    pub rhs: *mut *mut symbol,
    pub rhsalias: *mut *mut libc::c_char,
    pub line: libc::c_int,
    pub code: *mut libc::c_char,
    pub precsym: *mut symbol,
    pub index: libc::c_int,
    pub canReduce: Boolean,
    pub nextlhs: *mut rule,
    pub next: *mut rule,
}
pub type Boolean = libc::c_uint;
pub const Bo_TRUE: Boolean = 1;
pub const Bo_FALSE: Boolean = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct symbol {
    pub name: *mut libc::c_char,
    pub index: libc::c_int,
    pub type_0: C2RustUnnamed_1,
    pub rule: *mut rule,
    pub fallback: *mut symbol,
    pub prec: libc::c_int,
    pub assoc: e_assoc,
    pub firstset: *mut libc::c_char,
    pub lambda: Boolean,
    pub destructor: *mut libc::c_char,
    pub destructorln: libc::c_int,
    pub datatype: *mut libc::c_char,
    pub dtnum: libc::c_int,
}
pub type e_assoc = libc::c_uint;
pub const UNK: e_assoc = 3;
pub const NONE: e_assoc = 2;
pub const RIGHT: e_assoc = 1;
pub const LEFT: e_assoc = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const NONTERMINAL: C2RustUnnamed_1 = 1;
pub const TERMINAL: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state {
    pub bp: *mut config,
    pub cfp: *mut config,
    pub index: libc::c_int,
    pub ap: *mut action,
    pub nTknAct: libc::c_int,
    pub nNtAct: libc::c_int,
    pub iTknOfst: libc::c_int,
    pub iNtOfst: libc::c_int,
    pub iDflt: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config {
    pub rp: *mut rule,
    pub dot: libc::c_int,
    pub fws: *mut libc::c_char,
    pub fplp: *mut plink,
    pub bplp: *mut plink,
    pub stp: *mut state,
    pub status: C2RustUnnamed_2,
    pub next: *mut config,
    pub bp: *mut config,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const INCOMPLETE: C2RustUnnamed_2 = 1;
pub const COMPLETE: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plink {
    pub cfp: *mut config,
    pub next: *mut plink,
}
pub type e_action = libc::c_uint;
pub const NOT_USED: e_action = 7;
pub const RD_RESOLVED: e_action = 6;
pub const SH_RESOLVED: e_action = 5;
pub const CONFLICT: e_action = 4;
pub const ERROR: e_action = 3;
pub const REDUCE: e_action = 2;
pub const ACCEPT: e_action = 1;
pub const SHIFT: e_action = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lemon {
    pub sorted: *mut *mut state,
    pub rule: *mut rule,
    pub nstate: libc::c_int,
    pub nrule: libc::c_int,
    pub nsymbol: libc::c_int,
    pub nterminal: libc::c_int,
    pub symbols: *mut *mut symbol,
    pub errorcnt: libc::c_int,
    pub errsym: *mut symbol,
    pub name: *mut libc::c_char,
    pub arg: *mut libc::c_char,
    pub tokentype: *mut libc::c_char,
    pub vartype: *mut libc::c_char,
    pub start: *mut libc::c_char,
    pub stacksize: *mut libc::c_char,
    pub include: *mut libc::c_char,
    pub includeln: libc::c_int,
    pub error: *mut libc::c_char,
    pub errorln: libc::c_int,
    pub overflow: *mut libc::c_char,
    pub overflowln: libc::c_int,
    pub failure: *mut libc::c_char,
    pub failureln: libc::c_int,
    pub accept: *mut libc::c_char,
    pub acceptln: libc::c_int,
    pub extracode: *mut libc::c_char,
    pub extracodeln: libc::c_int,
    pub tokendest: *mut libc::c_char,
    pub tokendestln: libc::c_int,
    pub vardest: *mut libc::c_char,
    pub vardestln: libc::c_int,
    pub filename: *mut libc::c_char,
    pub tmplname: *mut libc::c_char,
    pub outname: *mut libc::c_char,
    pub tokenprefix: *mut libc::c_char,
    pub nconflict: libc::c_int,
    pub tablesize: libc::c_int,
    pub basisflag: libc::c_int,
    pub has_fallback: libc::c_int,
    pub argv0: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_x4node {
    pub data: *mut config,
    pub next: *mut s_x4node,
    pub from: *mut *mut s_x4node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_x4 {
    pub size: libc::c_int,
    pub count: libc::c_int,
    pub tbl: *mut s_x4node,
    pub ht: *mut *mut s_x4node,
}
pub type x4node = s_x4node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_x3node {
    pub data: *mut state,
    pub key: *mut config,
    pub next: *mut s_x3node,
    pub from: *mut *mut s_x3node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_x3 {
    pub size: libc::c_int,
    pub count: libc::c_int,
    pub tbl: *mut s_x3node,
    pub ht: *mut *mut s_x3node,
}
pub type x3node = s_x3node;
pub type x2node = s_x2node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_x2node {
    pub data: *mut symbol,
    pub key: *mut libc::c_char,
    pub next: *mut s_x2node,
    pub from: *mut *mut s_x2node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_x2 {
    pub size: libc::c_int,
    pub count: libc::c_int,
    pub tbl: *mut s_x2node,
    pub ht: *mut *mut s_x2node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_options {
    pub type_0: C2RustUnnamed_3,
    pub label: *mut libc::c_char,
    pub arg: *mut libc::c_void,
    pub message: *const libc::c_char,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const OPT_FSTR: C2RustUnnamed_3 = 8;
pub const OPT_FDBL: C2RustUnnamed_3 = 7;
pub const OPT_FINT: C2RustUnnamed_3 = 6;
pub const OPT_FFLAG: C2RustUnnamed_3 = 5;
pub const OPT_STR: C2RustUnnamed_3 = 4;
pub const OPT_DBL: C2RustUnnamed_3 = 3;
pub const OPT_INT: C2RustUnnamed_3 = 2;
pub const OPT_FLAG: C2RustUnnamed_3 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pstate {
    pub filename: *mut libc::c_char,
    pub tokenlineno: libc::c_int,
    pub errorcnt: libc::c_int,
    pub tokenstart: *mut libc::c_char,
    pub gp: *mut lemon,
    pub state: e_state,
    pub fallback: *mut symbol,
    pub lhs: *mut symbol,
    pub lhsalias: *mut libc::c_char,
    pub nrhs: libc::c_int,
    pub rhs: [*mut symbol; 1000],
    pub alias: [*mut libc::c_char; 1000],
    pub prevrule: *mut rule,
    pub declkeyword: *mut libc::c_char,
    pub declargslot: *mut *mut libc::c_char,
    pub decllnslot: *mut libc::c_int,
    pub declassoc: e_assoc,
    pub preccounter: libc::c_int,
    pub firstrule: *mut rule,
    pub lastrule: *mut rule,
}
pub type e_state = libc::c_uint;
pub const WAITING_FOR_FALLBACK_ID: e_state = 18;
pub const WAITING_FOR_DATATYPE_SYMBOL: e_state = 17;
pub const WAITING_FOR_DESTRUCTOR_SYMBOL: e_state = 16;
pub const RESYNC_AFTER_DECL_ERROR: e_state = 15;
pub const RESYNC_AFTER_RULE_ERROR: e_state = 14;
pub const PRECEDENCE_MARK_2: e_state = 13;
pub const PRECEDENCE_MARK_1: e_state = 12;
pub const RHS_ALIAS_2: e_state = 11;
pub const RHS_ALIAS_1: e_state = 10;
pub const LHS_ALIAS_3: e_state = 9;
pub const LHS_ALIAS_2: e_state = 8;
pub const LHS_ALIAS_1: e_state = 7;
pub const IN_RHS: e_state = 6;
pub const WAITING_FOR_ARROW: e_state = 5;
pub const WAITING_FOR_PRECEDENCE_SYMBOL: e_state = 4;
pub const WAITING_FOR_DECL_ARG: e_state = 3;
pub const WAITING_FOR_DECL_KEYWORD: e_state = 2;
pub const WAITING_FOR_DECL_OR_RULE: e_state = 1;
pub const INITIALIZE: e_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_x1node {
    pub data: *mut libc::c_char,
    pub next: *mut s_x1node,
    pub from: *mut *mut s_x1node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_x1 {
    pub size: libc::c_int,
    pub count: libc::c_int,
    pub tbl: *mut s_x1node,
    pub ht: *mut *mut s_x1node,
}
pub type x1node = s_x1node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct acttab {
    pub nAction: libc::c_int,
    pub nActionAlloc: libc::c_int,
    pub aAction: *mut C2RustUnnamed_4,
    pub aLookahead: *mut C2RustUnnamed_4,
    pub mnLookahead: libc::c_int,
    pub mnAction: libc::c_int,
    pub mxLookahead: libc::c_int,
    pub nLookahead: libc::c_int,
    pub nLookaheadAlloc: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub lookahead: libc::c_int,
    pub action: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct axset {
    pub stp: *mut state,
    pub isTkn: libc::c_int,
    pub nAction: libc::c_int,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Action_new() -> *mut action {
    static mut freelist_0: *mut action = 0 as *const action as *mut action;
    let mut new: *mut action = 0 as *mut action;
    if freelist_0.is_null() {
        let mut i: libc::c_int = 0;
        let mut amt: libc::c_int = 100 as libc::c_int;
        freelist_0 = malloc(
            (::std::mem::size_of::<action>() as libc::c_ulong)
                .wrapping_mul(amt as libc::c_ulong),
        ) as *mut action;
        if freelist_0.is_null() {
            fprintf(
                stderr,
                b"Unable to allocate memory for a new parser action.\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        i = 0 as libc::c_int;
        while i < amt - 1 as libc::c_int {
            let ref mut fresh0 = (*freelist_0.offset(i as isize)).next;
            *fresh0 = &mut *freelist_0.offset((i + 1 as libc::c_int) as isize)
                as *mut action;
            i += 1;
        }
        let ref mut fresh1 = (*freelist_0.offset((amt - 1 as libc::c_int) as isize))
            .next;
        *fresh1 = 0 as *mut action;
    }
    new = freelist_0;
    freelist_0 = (*freelist_0).next;
    return new;
}
unsafe extern "C" fn actioncmp(
    mut ap1: *mut action,
    mut ap2: *mut action,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    rc = (*(*ap1).sp).index - (*(*ap2).sp).index;
    if rc == 0 as libc::c_int {
        rc = (*ap1).type_0 as libc::c_int - (*ap2).type_0 as libc::c_int;
    }
    if rc == 0 as libc::c_int {
        if !((*ap1).type_0 as libc::c_uint == REDUCE as libc::c_int as libc::c_uint
            || (*ap1).type_0 as libc::c_uint
                == RD_RESOLVED as libc::c_int as libc::c_uint
            || (*ap1).type_0 as libc::c_uint == CONFLICT as libc::c_int as libc::c_uint)
        {
            myassert(
                b"src/lemon.c\0" as *const u8 as *const libc::c_char,
                383 as libc::c_int,
            );
        }
        if !((*ap2).type_0 as libc::c_uint == REDUCE as libc::c_int as libc::c_uint
            || (*ap2).type_0 as libc::c_uint
                == RD_RESOLVED as libc::c_int as libc::c_uint
            || (*ap2).type_0 as libc::c_uint == CONFLICT as libc::c_int as libc::c_uint)
        {
            myassert(
                b"src/lemon.c\0" as *const u8 as *const libc::c_char,
                384 as libc::c_int,
            );
        }
        rc = (*(*ap1).x.rp).index - (*(*ap2).x.rp).index;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn Action_sort(mut ap: *mut action) -> *mut action {
    ap = msort(
        ap as *mut libc::c_void,
        &mut (*ap).next as *mut *mut action as *mut *mut libc::c_void,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> libc::c_int>,
            Option::<
                unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
            >,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(*mut action, *mut action) -> libc::c_int,
                    unsafe extern "C" fn() -> libc::c_int,
                >(actioncmp),
            ),
        ),
    ) as *mut action;
    return ap;
}
#[no_mangle]
pub unsafe extern "C" fn Action_add(
    mut app: *mut *mut action,
    mut type_0: e_action,
    mut sp: *mut symbol,
    mut arg: *mut libc::c_void,
) {
    let mut new: *mut action = 0 as *mut action;
    new = Action_new();
    (*new).next = *app;
    *app = new;
    (*new).type_0 = type_0;
    (*new).sp = sp;
    if type_0 as libc::c_uint == SHIFT as libc::c_int as libc::c_uint {
        (*new).x.stp = arg as *mut state;
    } else {
        (*new).x.rp = arg as *mut rule;
    };
}
unsafe extern "C" fn acttab_free(mut p: *mut acttab) {
    free((*p).aAction as *mut libc::c_void);
    free((*p).aLookahead as *mut libc::c_void);
    free(p as *mut libc::c_void);
}
unsafe extern "C" fn acttab_alloc() -> *mut acttab {
    let mut p: *mut acttab = malloc(::std::mem::size_of::<acttab>() as libc::c_ulong)
        as *mut acttab;
    if p.is_null() {
        fprintf(
            stderr,
            b"Unable to allocate memory for a new acttab.\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    memset(
        p as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<acttab>() as libc::c_ulong,
    );
    return p;
}
unsafe extern "C" fn acttab_action(
    mut p: *mut acttab,
    mut lookahead: libc::c_int,
    mut action: libc::c_int,
) {
    if (*p).nLookahead >= (*p).nLookaheadAlloc {
        (*p).nLookaheadAlloc += 25 as libc::c_int;
        (*p)
            .aLookahead = realloc(
            (*p).aLookahead as *mut libc::c_void,
            (::std::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong)
                .wrapping_mul((*p).nLookaheadAlloc as libc::c_ulong),
        ) as *mut C2RustUnnamed_4;
        if ((*p).aLookahead).is_null() {
            fprintf(stderr, b"malloc failed\n\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
    }
    if (*p).nLookahead == 0 as libc::c_int {
        (*p).mxLookahead = lookahead;
        (*p).mnLookahead = lookahead;
        (*p).mnAction = action;
    } else {
        if (*p).mxLookahead < lookahead {
            (*p).mxLookahead = lookahead;
        }
        if (*p).mnLookahead > lookahead {
            (*p).mnLookahead = lookahead;
            (*p).mnAction = action;
        }
    }
    (*((*p).aLookahead).offset((*p).nLookahead as isize)).lookahead = lookahead;
    (*((*p).aLookahead).offset((*p).nLookahead as isize)).action = action;
    (*p).nLookahead += 1;
}
unsafe extern "C" fn acttab_insert(mut p: *mut acttab) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if !((*p).nLookahead > 0 as libc::c_int) {
        myassert(
            b"src/lemon.c\0" as *const u8 as *const libc::c_char,
            505 as libc::c_int,
        );
    }
    n = (*p).mxLookahead + 1 as libc::c_int;
    if (*p).nAction + n >= (*p).nActionAlloc {
        let mut oldAlloc: libc::c_int = (*p).nActionAlloc;
        (*p).nActionAlloc = (*p).nAction + n + (*p).nActionAlloc + 20 as libc::c_int;
        (*p)
            .aAction = realloc(
            (*p).aAction as *mut libc::c_void,
            (::std::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong)
                .wrapping_mul((*p).nActionAlloc as libc::c_ulong),
        ) as *mut C2RustUnnamed_4;
        if ((*p).aAction).is_null() {
            fprintf(stderr, b"malloc failed\n\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        i = oldAlloc;
        while i < (*p).nActionAlloc {
            (*((*p).aAction).offset(i as isize)).lookahead = -(1 as libc::c_int);
            (*((*p).aAction).offset(i as isize)).action = -(1 as libc::c_int);
            i += 1;
        }
    }
    i = 0 as libc::c_int;
    while i < (*p).nAction + (*p).mnLookahead {
        if (*((*p).aAction).offset(i as isize)).lookahead < 0 as libc::c_int {
            j = 0 as libc::c_int;
            while j < (*p).nLookahead {
                k = (*((*p).aLookahead).offset(j as isize)).lookahead - (*p).mnLookahead
                    + i;
                if k < 0 as libc::c_int {
                    break;
                }
                if (*((*p).aAction).offset(k as isize)).lookahead >= 0 as libc::c_int {
                    break;
                }
                j += 1;
            }
            if !(j < (*p).nLookahead) {
                j = 0 as libc::c_int;
                while j < (*p).nAction {
                    if (*((*p).aAction).offset(j as isize)).lookahead
                        == j + (*p).mnLookahead - i
                    {
                        break;
                    }
                    j += 1;
                }
                if j == (*p).nAction {
                    break;
                }
            }
        } else if (*((*p).aAction).offset(i as isize)).lookahead == (*p).mnLookahead {
            if !((*((*p).aAction).offset(i as isize)).action != (*p).mnAction) {
                j = 0 as libc::c_int;
                while j < (*p).nLookahead {
                    k = (*((*p).aLookahead).offset(j as isize)).lookahead
                        - (*p).mnLookahead + i;
                    if k < 0 as libc::c_int || k >= (*p).nAction {
                        break;
                    }
                    if (*((*p).aLookahead).offset(j as isize)).lookahead
                        != (*((*p).aAction).offset(k as isize)).lookahead
                    {
                        break;
                    }
                    if (*((*p).aLookahead).offset(j as isize)).action
                        != (*((*p).aAction).offset(k as isize)).action
                    {
                        break;
                    }
                    j += 1;
                }
                if !(j < (*p).nLookahead) {
                    n = 0 as libc::c_int;
                    j = 0 as libc::c_int;
                    while j < (*p).nAction {
                        if !((*((*p).aAction).offset(j as isize)).lookahead
                            < 0 as libc::c_int)
                        {
                            if (*((*p).aAction).offset(j as isize)).lookahead
                                == j + (*p).mnLookahead - i
                            {
                                n += 1;
                            }
                        }
                        j += 1;
                    }
                    if n == (*p).nLookahead {
                        break;
                    }
                }
            }
        }
        i += 1;
    }
    j = 0 as libc::c_int;
    while j < (*p).nLookahead {
        k = (*((*p).aLookahead).offset(j as isize)).lookahead - (*p).mnLookahead + i;
        *((*p).aAction).offset(k as isize) = *((*p).aLookahead).offset(j as isize);
        if k >= (*p).nAction {
            (*p).nAction = k + 1 as libc::c_int;
        }
        j += 1;
    }
    (*p).nLookahead = 0 as libc::c_int;
    return i - (*p).mnLookahead;
}
#[no_mangle]
pub unsafe extern "C" fn myassert(
    mut file: *mut libc::c_char,
    mut line: libc::c_int,
) -> ! {
    fprintf(
        stderr,
        b"Assertion failed on line %d of file \"%s\"\n\0" as *const u8
            as *const libc::c_char,
        line,
        file,
    );
    exit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn FindRulePrecedences(mut xp: *mut lemon) {
    let mut rp: *mut rule = 0 as *mut rule;
    rp = (*xp).rule;
    while !rp.is_null() {
        if ((*rp).precsym).is_null() {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < (*rp).nrhs {
                if (**((*rp).rhs).offset(i as isize)).prec >= 0 as libc::c_int {
                    (*rp).precsym = *((*rp).rhs).offset(i as isize);
                    break;
                } else {
                    i += 1;
                }
            }
        }
        rp = (*rp).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn FindFirstSets(mut lemp: *mut lemon) {
    let mut i: libc::c_int = 0;
    let mut rp: *mut rule = 0 as *mut rule;
    let mut progress: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*lemp).nsymbol {
        (**((*lemp).symbols).offset(i as isize)).lambda = Bo_FALSE;
        i += 1;
    }
    i = (*lemp).nterminal;
    while i < (*lemp).nsymbol {
        let ref mut fresh2 = (**((*lemp).symbols).offset(i as isize)).firstset;
        *fresh2 = SetNew();
        i += 1;
    }
    loop {
        progress = 0 as libc::c_int;
        rp = (*lemp).rule;
        while !rp.is_null() {
            if !((*(*rp).lhs).lambda as u64 != 0) {
                i = 0 as libc::c_int;
                while i < (*rp).nrhs {
                    if (**((*rp).rhs).offset(i as isize)).lambda as libc::c_uint
                        == Bo_FALSE as libc::c_int as libc::c_uint
                    {
                        break;
                    }
                    i += 1;
                }
                if i == (*rp).nrhs {
                    (*(*rp).lhs).lambda = Bo_TRUE;
                    progress = 1 as libc::c_int;
                }
            }
            rp = (*rp).next;
        }
        if !(progress != 0) {
            break;
        }
    }
    loop {
        let mut s1: *mut symbol = 0 as *mut symbol;
        let mut s2: *mut symbol = 0 as *mut symbol;
        progress = 0 as libc::c_int;
        rp = (*lemp).rule;
        while !rp.is_null() {
            s1 = (*rp).lhs;
            i = 0 as libc::c_int;
            while i < (*rp).nrhs {
                s2 = *((*rp).rhs).offset(i as isize);
                if (*s2).type_0 as libc::c_uint
                    == TERMINAL as libc::c_int as libc::c_uint
                {
                    progress += SetAdd((*s1).firstset, (*s2).index);
                    break;
                } else {
                    if s1 == s2 {
                        if (*s1).lambda as libc::c_uint
                            == Bo_FALSE as libc::c_int as libc::c_uint
                        {
                            break;
                        }
                    } else {
                        progress += SetUnion((*s1).firstset, (*s2).firstset);
                        if (*s2).lambda as libc::c_uint
                            == Bo_FALSE as libc::c_int as libc::c_uint
                        {
                            break;
                        }
                    }
                    i += 1;
                }
            }
            rp = (*rp).next;
        }
        if !(progress != 0) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn FindStates(mut lemp: *mut lemon) {
    let mut sp: *mut symbol = 0 as *mut symbol;
    let mut rp: *mut rule = 0 as *mut rule;
    Configlist_init();
    if !((*lemp).start).is_null() {
        sp = Symbol_find((*lemp).start);
        if sp.is_null() {
            ErrorMsg(
                (*lemp).filename,
                0 as libc::c_int,
                b"The specified start symbol \"%s\" is not in a nonterminal of the grammar.  \"%s\" will be used as the start symbol instead.\0"
                    as *const u8 as *const libc::c_char,
                (*lemp).start,
                (*(*(*lemp).rule).lhs).name,
            );
            (*lemp).errorcnt += 1;
            sp = (*(*lemp).rule).lhs;
        }
    } else {
        sp = (*(*lemp).rule).lhs;
    }
    rp = (*lemp).rule;
    while !rp.is_null() {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < (*rp).nrhs {
            if *((*rp).rhs).offset(i as isize) == sp {
                ErrorMsg(
                    (*lemp).filename,
                    0 as libc::c_int,
                    b"The start symbol \"%s\" occurs on the right-hand side of a rule. This will result in a parser which does not work properly.\0"
                        as *const u8 as *const libc::c_char,
                    (*sp).name,
                );
                (*lemp).errorcnt += 1;
            }
            i += 1;
        }
        rp = (*rp).next;
    }
    rp = (*sp).rule;
    while !rp.is_null() {
        let mut newcfp: *mut config = 0 as *mut config;
        newcfp = Configlist_addbasis(rp, 0 as libc::c_int);
        SetAdd((*newcfp).fws, 0 as libc::c_int);
        rp = (*rp).nextlhs;
    }
    getstate(lemp);
}
unsafe extern "C" fn getstate(mut lemp: *mut lemon) -> *mut state {
    let mut cfp: *mut config = 0 as *mut config;
    let mut bp: *mut config = 0 as *mut config;
    let mut stp: *mut state = 0 as *mut state;
    Configlist_sortbasis();
    bp = Configlist_basis();
    stp = State_find(bp);
    if !stp.is_null() {
        let mut x: *mut config = 0 as *mut config;
        let mut y: *mut config = 0 as *mut config;
        x = bp;
        y = (*stp).bp;
        while !x.is_null() && !y.is_null() {
            Plink_copy(&mut (*y).bplp, (*x).bplp);
            Plink_delete((*x).fplp);
            (*x).bplp = 0 as *mut plink;
            (*x).fplp = (*x).bplp;
            x = (*x).bp;
            y = (*y).bp;
        }
        cfp = Configlist_return();
        Configlist_eat(cfp);
    } else {
        Configlist_closure(lemp);
        Configlist_sort();
        cfp = Configlist_return();
        stp = State_new();
        if stp.is_null() {
            memory_error();
        }
        (*stp).bp = bp;
        (*stp).cfp = cfp;
        let fresh3 = (*lemp).nstate;
        (*lemp).nstate = (*lemp).nstate + 1;
        (*stp).index = fresh3;
        (*stp).ap = 0 as *mut action;
        State_insert(stp, (*stp).bp);
        buildshifts(lemp, stp);
    }
    return stp;
}
unsafe extern "C" fn buildshifts(mut lemp: *mut lemon, mut stp: *mut state) {
    let mut cfp: *mut config = 0 as *mut config;
    let mut bcfp: *mut config = 0 as *mut config;
    let mut new: *mut config = 0 as *mut config;
    let mut sp: *mut symbol = 0 as *mut symbol;
    let mut bsp: *mut symbol = 0 as *mut symbol;
    let mut newstp: *mut state = 0 as *mut state;
    cfp = (*stp).cfp;
    while !cfp.is_null() {
        (*cfp).status = INCOMPLETE;
        cfp = (*cfp).next;
    }
    cfp = (*stp).cfp;
    while !cfp.is_null() {
        if !((*cfp).status as libc::c_uint == COMPLETE as libc::c_int as libc::c_uint) {
            if !((*cfp).dot >= (*(*cfp).rp).nrhs) {
                Configlist_reset();
                sp = *((*(*cfp).rp).rhs).offset((*cfp).dot as isize);
                bcfp = cfp;
                while !bcfp.is_null() {
                    if !((*bcfp).status as libc::c_uint
                        == COMPLETE as libc::c_int as libc::c_uint)
                    {
                        if !((*bcfp).dot >= (*(*bcfp).rp).nrhs) {
                            bsp = *((*(*bcfp).rp).rhs).offset((*bcfp).dot as isize);
                            if !(bsp != sp) {
                                (*bcfp).status = COMPLETE;
                                new = Configlist_addbasis(
                                    (*bcfp).rp,
                                    (*bcfp).dot + 1 as libc::c_int,
                                );
                                Plink_add(&mut (*new).bplp, bcfp);
                            }
                        }
                    }
                    bcfp = (*bcfp).next;
                }
                newstp = getstate(lemp);
                Action_add(
                    &mut (*stp).ap,
                    SHIFT as libc::c_int as libc::c_uint,
                    sp,
                    newstp as *mut libc::c_void,
                );
            }
        }
        cfp = (*cfp).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn FindLinks(mut lemp: *mut lemon) {
    let mut i: libc::c_int = 0;
    let mut cfp: *mut config = 0 as *mut config;
    let mut other: *mut config = 0 as *mut config;
    let mut stp: *mut state = 0 as *mut state;
    let mut plp: *mut plink = 0 as *mut plink;
    i = 0 as libc::c_int;
    while i < (*lemp).nstate {
        stp = *((*lemp).sorted).offset(i as isize);
        cfp = (*stp).cfp;
        while !cfp.is_null() {
            (*cfp).stp = stp;
            cfp = (*cfp).next;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*lemp).nstate {
        stp = *((*lemp).sorted).offset(i as isize);
        cfp = (*stp).cfp;
        while !cfp.is_null() {
            plp = (*cfp).bplp;
            while !plp.is_null() {
                other = (*plp).cfp;
                Plink_add(&mut (*other).fplp, cfp);
                plp = (*plp).next;
            }
            cfp = (*cfp).next;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn FindFollowSets(mut lemp: *mut lemon) {
    let mut i: libc::c_int = 0;
    let mut cfp: *mut config = 0 as *mut config;
    let mut stp: *mut state = 0 as *mut state;
    let mut plp: *mut plink = 0 as *mut plink;
    let mut progress: libc::c_int = 0;
    let mut change: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*lemp).nstate {
        stp = *((*lemp).sorted).offset(i as isize);
        cfp = (*stp).cfp;
        while !cfp.is_null() {
            (*cfp).status = INCOMPLETE;
            cfp = (*cfp).next;
        }
        i += 1;
    }
    loop {
        progress = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*lemp).nstate {
            stp = *((*lemp).sorted).offset(i as isize);
            cfp = (*stp).cfp;
            while !cfp.is_null() {
                if !((*cfp).status as libc::c_uint
                    == COMPLETE as libc::c_int as libc::c_uint)
                {
                    plp = (*cfp).fplp;
                    while !plp.is_null() {
                        change = SetUnion((*(*plp).cfp).fws, (*cfp).fws);
                        if change != 0 {
                            (*(*plp).cfp).status = INCOMPLETE;
                            progress = 1 as libc::c_int;
                        }
                        plp = (*plp).next;
                    }
                    (*cfp).status = COMPLETE;
                }
                cfp = (*cfp).next;
            }
            i += 1;
        }
        if !(progress != 0) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn FindActions(mut lemp: *mut lemon) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cfp: *mut config = 0 as *mut config;
    let mut sp: *mut symbol = 0 as *mut symbol;
    let mut rp: *mut rule = 0 as *mut rule;
    i = 0 as libc::c_int;
    while i < (*lemp).nstate {
        let mut stp: *mut state = 0 as *mut state;
        stp = *((*lemp).sorted).offset(i as isize);
        cfp = (*stp).cfp;
        while !cfp.is_null() {
            if (*(*cfp).rp).nrhs == (*cfp).dot {
                j = 0 as libc::c_int;
                while j < (*lemp).nterminal {
                    if *((*cfp).fws).offset(j as isize) != 0 {
                        Action_add(
                            &mut (*stp).ap,
                            REDUCE as libc::c_int as libc::c_uint,
                            *((*lemp).symbols).offset(j as isize),
                            (*cfp).rp as *mut libc::c_void,
                        );
                    }
                    j += 1;
                }
            }
            cfp = (*cfp).next;
        }
        i += 1;
    }
    if !((*lemp).start).is_null() {
        sp = Symbol_find((*lemp).start);
        if sp.is_null() {
            sp = (*(*lemp).rule).lhs;
        }
    } else {
        sp = (*(*lemp).rule).lhs;
    }
    if (*lemp).nstate != 0 {
        let mut stp_0: *mut state = 0 as *mut state;
        stp_0 = *((*lemp).sorted).offset(0 as libc::c_int as isize);
        Action_add(
            &mut (*stp_0).ap,
            ACCEPT as libc::c_int as libc::c_uint,
            sp,
            0 as *mut libc::c_void,
        );
    }
    i = 0 as libc::c_int;
    while i < (*lemp).nstate {
        let mut ap: *mut action = 0 as *mut action;
        let mut nap: *mut action = 0 as *mut action;
        let mut stp_1: *mut state = 0 as *mut state;
        stp_1 = *((*lemp).sorted).offset(i as isize);
        if ((*stp_1).ap).is_null() {
            myassert(
                b"src/lemon.c\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                963 as libc::c_int,
            );
        }
        (*stp_1).ap = Action_sort((*stp_1).ap);
        ap = (*stp_1).ap;
        while !ap.is_null() && !((*ap).next).is_null() {
            nap = (*ap).next;
            while !nap.is_null() && (*nap).sp == (*ap).sp {
                (*lemp).nconflict += resolve_conflict(ap, nap, (*lemp).errsym);
                nap = (*nap).next;
            }
            ap = (*ap).next;
        }
        i += 1;
    }
    rp = (*lemp).rule;
    while !rp.is_null() {
        (*rp).canReduce = Bo_FALSE;
        rp = (*rp).next;
    }
    i = 0 as libc::c_int;
    while i < (*lemp).nstate {
        let mut ap_0: *mut action = 0 as *mut action;
        ap_0 = (**((*lemp).sorted).offset(i as isize)).ap;
        while !ap_0.is_null() {
            if (*ap_0).type_0 as libc::c_uint == REDUCE as libc::c_int as libc::c_uint {
                (*(*ap_0).x.rp).canReduce = Bo_TRUE;
            }
            ap_0 = (*ap_0).next;
        }
        i += 1;
    }
    rp = (*lemp).rule;
    while !rp.is_null() {
        if !((*rp).canReduce as u64 != 0) {
            ErrorMsg(
                (*lemp).filename,
                (*rp).ruleline,
                b"This rule can not be reduced.\n\0" as *const u8 as *const libc::c_char,
            );
            (*lemp).errorcnt += 1;
        }
        rp = (*rp).next;
    }
}
unsafe extern "C" fn resolve_conflict(
    mut apx: *mut action,
    mut apy: *mut action,
    mut errsym: *mut symbol,
) -> libc::c_int {
    let mut spx: *mut symbol = 0 as *mut symbol;
    let mut spy: *mut symbol = 0 as *mut symbol;
    let mut errcnt: libc::c_int = 0 as libc::c_int;
    if !((*apx).sp == (*apy).sp) {
        myassert(
            b"src/lemon.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1010 as libc::c_int,
        );
    }
    if (*apx).type_0 as libc::c_uint == SHIFT as libc::c_int as libc::c_uint
        && (*apy).type_0 as libc::c_uint == REDUCE as libc::c_int as libc::c_uint
    {
        spx = (*apx).sp;
        spy = (*(*apy).x.rp).precsym;
        if spy.is_null() || (*spx).prec < 0 as libc::c_int
            || (*spy).prec < 0 as libc::c_int
        {
            (*apy).type_0 = CONFLICT;
            errcnt += 1;
        } else if (*spx).prec > (*spy).prec {
            (*apy).type_0 = RD_RESOLVED;
        } else if (*spx).prec < (*spy).prec {
            (*apx).type_0 = SH_RESOLVED;
        } else if (*spx).prec == (*spy).prec
                && (*spx).assoc as libc::c_uint == RIGHT as libc::c_int as libc::c_uint
            {
            (*apy).type_0 = RD_RESOLVED;
        } else if (*spx).prec == (*spy).prec
                && (*spx).assoc as libc::c_uint == LEFT as libc::c_int as libc::c_uint
            {
            (*apx).type_0 = SH_RESOLVED;
        } else {
            if !((*spx).prec == (*spy).prec
                && (*spx).assoc as libc::c_uint == NONE as libc::c_int as libc::c_uint)
            {
                myassert(
                    b"src/lemon.c\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    1027 as libc::c_int,
                );
            }
            (*apy).type_0 = CONFLICT;
            errcnt += 1;
        }
    } else if (*apx).type_0 as libc::c_uint == REDUCE as libc::c_int as libc::c_uint
            && (*apy).type_0 as libc::c_uint == REDUCE as libc::c_int as libc::c_uint
        {
        spx = (*(*apx).x.rp).precsym;
        spy = (*(*apy).x.rp).precsym;
        if spx.is_null() || spy.is_null() || (*spx).prec < 0 as libc::c_int
            || (*spy).prec < 0 as libc::c_int || (*spx).prec == (*spy).prec
        {
            (*apy).type_0 = CONFLICT;
            errcnt += 1;
        } else if (*spx).prec > (*spy).prec {
            (*apy).type_0 = RD_RESOLVED;
        } else if (*spx).prec < (*spy).prec {
            (*apx).type_0 = RD_RESOLVED;
        }
    } else if !((*apx).type_0 as libc::c_uint
            == SH_RESOLVED as libc::c_int as libc::c_uint
            || (*apx).type_0 as libc::c_uint
                == RD_RESOLVED as libc::c_int as libc::c_uint
            || (*apx).type_0 as libc::c_uint == CONFLICT as libc::c_int as libc::c_uint
            || (*apy).type_0 as libc::c_uint
                == SH_RESOLVED as libc::c_int as libc::c_uint
            || (*apy).type_0 as libc::c_uint
                == RD_RESOLVED as libc::c_int as libc::c_uint
            || (*apy).type_0 as libc::c_uint == CONFLICT as libc::c_int as libc::c_uint)
        {
        myassert(
            b"src/lemon.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1051 as libc::c_int,
        );
    }
    return errcnt;
}
static mut freelist: *mut config = 0 as *const config as *mut config;
static mut current: *mut config = 0 as *const config as *mut config;
static mut currentend: *mut *mut config = 0 as *const *mut config as *mut *mut config;
static mut basis: *mut config = 0 as *const config as *mut config;
static mut basisend: *mut *mut config = 0 as *const *mut config as *mut *mut config;
unsafe extern "C" fn newconfig() -> *mut config {
    let mut new: *mut config = 0 as *mut config;
    if freelist.is_null() {
        let mut i: libc::c_int = 0;
        let mut amt: libc::c_int = 3 as libc::c_int;
        freelist = malloc(
            (::std::mem::size_of::<config>() as libc::c_ulong)
                .wrapping_mul(amt as libc::c_ulong),
        ) as *mut config;
        if freelist.is_null() {
            fprintf(
                stderr,
                b"Unable to allocate memory for a new configuration.\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        i = 0 as libc::c_int;
        while i < amt - 1 as libc::c_int {
            let ref mut fresh4 = (*freelist.offset(i as isize)).next;
            *fresh4 = &mut *freelist.offset((i + 1 as libc::c_int) as isize)
                as *mut config;
            i += 1;
        }
        let ref mut fresh5 = (*freelist.offset((amt - 1 as libc::c_int) as isize)).next;
        *fresh5 = 0 as *mut config;
    }
    new = freelist;
    freelist = (*freelist).next;
    return new;
}
unsafe extern "C" fn deleteconfig(mut old: *mut config) {
    (*old).next = freelist;
    freelist = old;
}
#[no_mangle]
pub unsafe extern "C" fn Configlist_init() {
    current = 0 as *mut config;
    currentend = &mut current;
    basis = 0 as *mut config;
    basisend = &mut basis;
    Configtable_init();
}
#[no_mangle]
pub unsafe extern "C" fn Configlist_reset() {
    current = 0 as *mut config;
    currentend = &mut current;
    basis = 0 as *mut config;
    basisend = &mut basis;
    Configtable_clear(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Configlist_add(
    mut rp: *mut rule,
    mut dot: libc::c_int,
) -> *mut config {
    let mut cfp: *mut config = 0 as *mut config;
    let mut model: config = config {
        rp: 0 as *mut rule,
        dot: 0,
        fws: 0 as *mut libc::c_char,
        fplp: 0 as *mut plink,
        bplp: 0 as *mut plink,
        stp: 0 as *mut state,
        status: COMPLETE,
        next: 0 as *mut config,
        bp: 0 as *mut config,
    };
    if currentend.is_null() {
        myassert(
            b"src/lemon.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1124 as libc::c_int,
        );
    }
    model.rp = rp;
    model.dot = dot;
    cfp = Configtable_find(&mut model);
    if cfp.is_null() {
        cfp = newconfig();
        (*cfp).rp = rp;
        (*cfp).dot = dot;
        (*cfp).fws = SetNew();
        (*cfp).stp = 0 as *mut state;
        (*cfp).bplp = 0 as *mut plink;
        (*cfp).fplp = (*cfp).bplp;
        (*cfp).next = 0 as *mut config;
        (*cfp).bp = 0 as *mut config;
        *currentend = cfp;
        currentend = &mut (*cfp).next;
        Configtable_insert(cfp);
    }
    return cfp;
}
#[no_mangle]
pub unsafe extern "C" fn Configlist_addbasis(
    mut rp: *mut rule,
    mut dot: libc::c_int,
) -> *mut config {
    let mut cfp: *mut config = 0 as *mut config;
    let mut model: config = config {
        rp: 0 as *mut rule,
        dot: 0,
        fws: 0 as *mut libc::c_char,
        fplp: 0 as *mut plink,
        bplp: 0 as *mut plink,
        stp: 0 as *mut state,
        status: COMPLETE,
        next: 0 as *mut config,
        bp: 0 as *mut config,
    };
    if basisend.is_null() {
        myassert(
            b"src/lemon.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1151 as libc::c_int,
        );
    }
    if currentend.is_null() {
        myassert(
            b"src/lemon.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1152 as libc::c_int,
        );
    }
    model.rp = rp;
    model.dot = dot;
    cfp = Configtable_find(&mut model);
    if cfp.is_null() {
        cfp = newconfig();
        (*cfp).rp = rp;
        (*cfp).dot = dot;
        (*cfp).fws = SetNew();
        (*cfp).stp = 0 as *mut state;
        (*cfp).bplp = 0 as *mut plink;
        (*cfp).fplp = (*cfp).bplp;
        (*cfp).next = 0 as *mut config;
        (*cfp).bp = 0 as *mut config;
        *currentend = cfp;
        currentend = &mut (*cfp).next;
        *basisend = cfp;
        basisend = &mut (*cfp).bp;
        Configtable_insert(cfp);
    }
    return cfp;
}
#[no_mangle]
pub unsafe extern "C" fn Configlist_closure(mut lemp: *mut lemon) {
    let mut cfp: *mut config = 0 as *mut config;
    let mut newcfp: *mut config = 0 as *mut config;
    let mut rp: *mut rule = 0 as *mut rule;
    let mut newrp: *mut rule = 0 as *mut rule;
    let mut sp: *mut symbol = 0 as *mut symbol;
    let mut xsp: *mut symbol = 0 as *mut symbol;
    let mut i: libc::c_int = 0;
    let mut dot: libc::c_int = 0;
    if currentend.is_null() {
        myassert(
            b"src/lemon.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1183 as libc::c_int,
        );
    }
    cfp = current;
    while !cfp.is_null() {
        rp = (*cfp).rp;
        dot = (*cfp).dot;
        if !(dot >= (*rp).nrhs) {
            sp = *((*rp).rhs).offset(dot as isize);
            if (*sp).type_0 as libc::c_uint == NONTERMINAL as libc::c_int as libc::c_uint
            {
                if ((*sp).rule).is_null() && sp != (*lemp).errsym {
                    ErrorMsg(
                        (*lemp).filename,
                        (*rp).line,
                        b"Nonterminal \"%s\" has no rules.\0" as *const u8
                            as *const libc::c_char,
                        (*sp).name,
                    );
                    (*lemp).errorcnt += 1;
                }
                newrp = (*sp).rule;
                while !newrp.is_null() {
                    newcfp = Configlist_add(newrp, 0 as libc::c_int);
                    i = dot + 1 as libc::c_int;
                    while i < (*rp).nrhs {
                        xsp = *((*rp).rhs).offset(i as isize);
                        if (*xsp).type_0 as libc::c_uint
                            == TERMINAL as libc::c_int as libc::c_uint
                        {
                            SetAdd((*newcfp).fws, (*xsp).index);
                            break;
                        } else {
                            SetUnion((*newcfp).fws, (*xsp).firstset);
                            if (*xsp).lambda as libc::c_uint
                                == Bo_FALSE as libc::c_int as libc::c_uint
                            {
                                break;
                            }
                            i += 1;
                        }
                    }
                    if i == (*rp).nrhs {
                        Plink_add(&mut (*cfp).fplp, newcfp);
                    }
                    newrp = (*newrp).nextlhs;
                }
            }
        }
        cfp = (*cfp).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Configlist_sort() {
    current = msort(
        current as *mut libc::c_void,
        &mut (*current).next as *mut *mut config as *mut *mut libc::c_void,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> libc::c_int>,
            Option::<
                unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
            >,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(*mut config, *mut config) -> libc::c_int,
                    unsafe extern "C" fn() -> libc::c_int,
                >(Configcmp),
            ),
        ),
    ) as *mut config;
    currentend = 0 as *mut *mut config;
}
#[no_mangle]
pub unsafe extern "C" fn Configlist_sortbasis() {
    basis = msort(
        current as *mut libc::c_void,
        &mut (*current).bp as *mut *mut config as *mut *mut libc::c_void,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> libc::c_int>,
            Option::<
                unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
            >,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(*mut config, *mut config) -> libc::c_int,
                    unsafe extern "C" fn() -> libc::c_int,
                >(Configcmp),
            ),
        ),
    ) as *mut config;
    basisend = 0 as *mut *mut config;
}
#[no_mangle]
pub unsafe extern "C" fn Configlist_return() -> *mut config {
    let mut old: *mut config = 0 as *mut config;
    old = current;
    current = 0 as *mut config;
    currentend = 0 as *mut *mut config;
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn Configlist_basis() -> *mut config {
    let mut old: *mut config = 0 as *mut config;
    old = basis;
    basis = 0 as *mut config;
    basisend = 0 as *mut *mut config;
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn Configlist_eat(mut cfp: *mut config) {
    let mut nextcfp: *mut config = 0 as *mut config;
    while !cfp.is_null() {
        nextcfp = (*cfp).next;
        if !((*cfp).fplp).is_null() {
            myassert(
                b"src/lemon.c\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                1255 as libc::c_int,
            );
        }
        if !((*cfp).bplp).is_null() {
            myassert(
                b"src/lemon.c\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                1256 as libc::c_int,
            );
        }
        if !((*cfp).fws).is_null() {
            SetFree((*cfp).fws);
        }
        deleteconfig(cfp);
        cfp = nextcfp;
    }
}
unsafe extern "C" fn findbreak(
    mut msg: *mut libc::c_char,
    mut min: libc::c_int,
    mut max: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut spot: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    spot = min;
    i = spot;
    while i <= max {
        c = *msg.offset(i as isize);
        if c as libc::c_int == '\t' as i32 {
            *msg.offset(i as isize) = ' ' as i32 as libc::c_char;
        }
        if c as libc::c_int == '\n' as i32 {
            *msg.offset(i as isize) = ' ' as i32 as libc::c_char;
            spot = i;
            break;
        } else if c as libc::c_int == 0 as libc::c_int {
            spot = i;
            break;
        } else {
            if c as libc::c_int == '-' as i32 && i < max - 1 as libc::c_int {
                spot = i + 1 as libc::c_int;
            }
            if c as libc::c_int == ' ' as i32 {
                spot = i;
            }
            i += 1;
        }
    }
    return spot;
}
#[no_mangle]
pub unsafe extern "C" fn ErrorMsg(
    mut filename: *const libc::c_char,
    mut lineno: libc::c_int,
    mut format: *const libc::c_char,
    mut args: ...
) {
    let mut errmsg: [libc::c_char; 10000] = [0; 10000];
    let mut prefix: [libc::c_char; 40] = [0; 40];
    let mut errmsgsize: libc::c_int = 0;
    let mut prefixsize: libc::c_int = 0;
    let mut availablewidth: libc::c_int = 0;
    let mut ap: ::std::ffi::VaListImpl;
    let mut end: libc::c_int = 0;
    let mut restart: libc::c_int = 0;
    let mut base: libc::c_int = 0;
    ap = args.clone();
    if lineno > 0 as libc::c_int {
        sprintf(
            prefix.as_mut_ptr(),
            b"%.*s:%d: \0" as *const u8 as *const libc::c_char,
            30 as libc::c_int - 10 as libc::c_int,
            filename,
            lineno,
        );
    } else {
        sprintf(
            prefix.as_mut_ptr(),
            b"%.*s: \0" as *const u8 as *const libc::c_char,
            30 as libc::c_int - 10 as libc::c_int,
            filename,
        );
    }
    prefixsize = strlen(prefix.as_mut_ptr()) as libc::c_int;
    availablewidth = 79 as libc::c_int - prefixsize;
    vsprintf(errmsg.as_mut_ptr(), format, ap.as_va_list());
    errmsgsize = strlen(errmsg.as_mut_ptr()) as libc::c_int;
    while errmsgsize > 0 as libc::c_int
        && errmsg[(errmsgsize - 1 as libc::c_int) as usize] as libc::c_int == '\n' as i32
    {
        errmsgsize -= 1;
        errmsg[errmsgsize as usize] = 0 as libc::c_int as libc::c_char;
    }
    base = 0 as libc::c_int;
    while errmsg[base as usize] as libc::c_int != 0 as libc::c_int {
        restart = findbreak(
            &mut *errmsg.as_mut_ptr().offset(base as isize),
            0 as libc::c_int,
            availablewidth,
        );
        end = restart;
        restart += base;
        while errmsg[restart as usize] as libc::c_int == ' ' as i32 {
            restart += 1;
        }
        fprintf(
            stdout,
            b"%s%.*s\n\0" as *const u8 as *const libc::c_char,
            prefix.as_mut_ptr(),
            end,
            &mut *errmsg.as_mut_ptr().offset(base as isize) as *mut libc::c_char,
        );
        base = restart;
    }
}
unsafe extern "C" fn memory_error() -> ! {
    fprintf(
        stderr,
        b"Out of memory.  Aborting...\n\0" as *const u8 as *const libc::c_char,
    );
    exit(1 as libc::c_int);
}
static mut out_dir: *const libc::c_char = b".\0" as *const u8 as *const libc::c_char;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv_0: *mut *mut libc::c_char,
) -> libc::c_int {
    static mut version: libc::c_int = 0 as libc::c_int;
    static mut rpflag: libc::c_int = 0 as libc::c_int;
    static mut basisflag: libc::c_int = 0 as libc::c_int;
    static mut compress: libc::c_int = 0 as libc::c_int;
    static mut quiet: libc::c_int = 0 as libc::c_int;
    static mut statistics: libc::c_int = 0 as libc::c_int;
    static mut mhflag: libc::c_int = 0 as libc::c_int;
    static mut options: [s_options; 9] = unsafe {
        [
            {
                let mut init = s_options {
                    type_0: OPT_FLAG,
                    label: b"b\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    arg: &basisflag as *const libc::c_int as *mut libc::c_int
                        as *mut libc::c_char as *mut libc::c_void,
                    message: b"Print only the basis in report.\0" as *const u8
                        as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FLAG,
                    label: b"c\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    arg: &compress as *const libc::c_int as *mut libc::c_int
                        as *mut libc::c_char as *mut libc::c_void,
                    message: b"Don't compress the action table.\0" as *const u8
                        as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FLAG,
                    label: b"g\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    arg: &rpflag as *const libc::c_int as *mut libc::c_int
                        as *mut libc::c_char as *mut libc::c_void,
                    message: b"Print grammar without actions.\0" as *const u8
                        as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FLAG,
                    label: b"m\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    arg: &mhflag as *const libc::c_int as *mut libc::c_int
                        as *mut libc::c_char as *mut libc::c_void,
                    message: b"Output a makeheaders compatible file\0" as *const u8
                        as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FLAG,
                    label: b"q\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    arg: &quiet as *const libc::c_int as *mut libc::c_int
                        as *mut libc::c_char as *mut libc::c_void,
                    message: b"(Quiet) Don't print the report file.\0" as *const u8
                        as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FLAG,
                    label: b"s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    arg: &statistics as *const libc::c_int as *mut libc::c_int
                        as *mut libc::c_char as *mut libc::c_void,
                    message: b"Print parser stats to standard output.\0" as *const u8
                        as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FLAG,
                    label: b"x\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    arg: &version as *const libc::c_int as *mut libc::c_int
                        as *mut libc::c_char as *mut libc::c_void,
                    message: b"Print the version number.\0" as *const u8
                        as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_STR,
                    label: b"o\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    arg: &out_dir as *const *const libc::c_char
                        as *mut *const libc::c_char as *mut libc::c_char
                        as *mut libc::c_void,
                    message: b"Customize output directory.\0" as *const u8
                        as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FLAG,
                    label: 0 as *const libc::c_char as *mut libc::c_char,
                    arg: 0 as *const libc::c_void as *mut libc::c_void,
                    message: 0 as *const libc::c_char,
                };
                init
            },
        ]
    };
    let mut i: libc::c_int = 0;
    let mut lem: lemon = lemon {
        sorted: 0 as *mut *mut state,
        rule: 0 as *mut rule,
        nstate: 0,
        nrule: 0,
        nsymbol: 0,
        nterminal: 0,
        symbols: 0 as *mut *mut symbol,
        errorcnt: 0,
        errsym: 0 as *mut symbol,
        name: 0 as *mut libc::c_char,
        arg: 0 as *mut libc::c_char,
        tokentype: 0 as *mut libc::c_char,
        vartype: 0 as *mut libc::c_char,
        start: 0 as *mut libc::c_char,
        stacksize: 0 as *mut libc::c_char,
        include: 0 as *mut libc::c_char,
        includeln: 0,
        error: 0 as *mut libc::c_char,
        errorln: 0,
        overflow: 0 as *mut libc::c_char,
        overflowln: 0,
        failure: 0 as *mut libc::c_char,
        failureln: 0,
        accept: 0 as *mut libc::c_char,
        acceptln: 0,
        extracode: 0 as *mut libc::c_char,
        extracodeln: 0,
        tokendest: 0 as *mut libc::c_char,
        tokendestln: 0,
        vardest: 0 as *mut libc::c_char,
        vardestln: 0,
        filename: 0 as *mut libc::c_char,
        tmplname: 0 as *mut libc::c_char,
        outname: 0 as *mut libc::c_char,
        tokenprefix: 0 as *mut libc::c_char,
        nconflict: 0,
        tablesize: 0,
        basisflag: 0,
        has_fallback: 0,
        argv0: 0 as *mut libc::c_char,
    };
    let mut def_tmpl_name: *mut libc::c_char = b"lempar.c\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    OptInit(argv_0, options.as_mut_ptr(), stderr);
    if version != 0 {
        printf(b"Lemon version 1.0\n\0" as *const u8 as *const libc::c_char);
        exit(0 as libc::c_int);
    }
    if OptNArgs() < 1 as libc::c_int {
        fprintf(
            stderr,
            b"Exactly one filename argument is required.\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    lem.errorcnt = 0 as libc::c_int;
    Strsafe_init();
    Symbol_init();
    State_init();
    lem.argv0 = *argv_0.offset(0 as libc::c_int as isize);
    lem.filename = OptArg(0 as libc::c_int);
    lem
        .tmplname = if OptNArgs() == 2 as libc::c_int {
        OptArg(1 as libc::c_int)
    } else {
        def_tmpl_name
    };
    lem.basisflag = basisflag;
    lem.has_fallback = 0 as libc::c_int;
    lem.nconflict = 0 as libc::c_int;
    lem.start = 0 as *mut libc::c_char;
    lem.tokentype = lem.start;
    lem.arg = lem.tokentype;
    lem.include = lem.arg;
    lem.name = lem.include;
    lem.vartype = 0 as *mut libc::c_char;
    lem.stacksize = 0 as *mut libc::c_char;
    lem.extracode = 0 as *mut libc::c_char;
    lem.outname = lem.extracode;
    lem.tokenprefix = lem.outname;
    lem.tokendest = lem.tokenprefix;
    lem.accept = lem.tokendest;
    lem.failure = lem.accept;
    lem.overflow = lem.failure;
    lem.error = lem.overflow;
    lem.vardest = 0 as *mut libc::c_char;
    lem.tablesize = 0 as libc::c_int;
    Symbol_new(b"$\0" as *const u8 as *const libc::c_char);
    lem.errsym = Symbol_new(b"error\0" as *const u8 as *const libc::c_char);
    Parse(&mut lem);
    if lem.errorcnt != 0 {
        exit(lem.errorcnt);
    }
    if (lem.rule).is_null() {
        fprintf(stderr, b"Empty grammar.\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    Symbol_new(b"{default}\0" as *const u8 as *const libc::c_char);
    lem.nsymbol = Symbol_count();
    lem.symbols = Symbol_arrayof();
    i = 0 as libc::c_int;
    while i < lem.nsymbol {
        (**(lem.symbols).offset(i as isize)).index = i;
        i += 1;
    }
    qsort(
        lem.symbols as *mut libc::c_void,
        lem.nsymbol as size_t,
        ::std::mem::size_of::<*mut symbol>() as libc::c_ulong,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> libc::c_int>,
            __compar_fn_t,
        >(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> libc::c_int>,
                Option::<unsafe extern "C" fn() -> libc::c_int>,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(
                            *mut *mut symbol,
                            *mut *mut symbol,
                        ) -> libc::c_int,
                        unsafe extern "C" fn() -> libc::c_int,
                    >(Symbolcmpp),
                ),
            ),
        ),
    );
    i = 0 as libc::c_int;
    while i < lem.nsymbol {
        (**(lem.symbols).offset(i as isize)).index = i;
        i += 1;
    }
    i = 1 as libc::c_int;
    while i < lem.nsymbol
        && *(*__ctype_b_loc())
            .offset(
                *((**(lem.symbols).offset(i as isize)).name)
                    .offset(0 as libc::c_int as isize) as libc::c_int as isize,
            ) as libc::c_int & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        i += 1;
    }
    lem.nsymbol -= 1;
    lem.nterminal = i;
    if rpflag != 0 {
        Reprint(&mut lem);
    } else {
        SetSize(lem.nterminal);
        FindRulePrecedences(&mut lem);
        FindFirstSets(&mut lem);
        lem.nstate = 0 as libc::c_int;
        FindStates(&mut lem);
        lem.nstate = State_count();
        lem.sorted = State_arrayof();
        FindLinks(&mut lem);
        FindFollowSets(&mut lem);
        FindActions(&mut lem);
        if compress == 0 as libc::c_int {
            CompressTables(&mut lem);
        }
        if quiet == 0 {
            ReportOutput(&mut lem);
        }
        ReportTable(&mut lem, mhflag);
        if mhflag == 0 {
            ReportHeader(&mut lem);
        }
    }
    if statistics != 0 {
        printf(
            b"Parser statistics: %d terminals, %d nonterminals, %d rules\n\0"
                as *const u8 as *const libc::c_char,
            lem.nterminal,
            lem.nsymbol - lem.nterminal,
            lem.nrule,
        );
        printf(
            b"                   %d states, %d parser table entries, %d conflicts\n\0"
                as *const u8 as *const libc::c_char,
            lem.nstate,
            lem.tablesize,
            lem.nconflict,
        );
    }
    if lem.nconflict != 0 {
        fprintf(
            stderr,
            b"%d parsing conflicts.\n\0" as *const u8 as *const libc::c_char,
            lem.nconflict,
        );
    }
    exit(lem.errorcnt + lem.nconflict);
}
unsafe extern "C" fn merge(
    mut a: *mut libc::c_char,
    mut b: *mut libc::c_char,
    mut cmp: Option::<unsafe extern "C" fn() -> libc::c_int>,
    mut offset: libc::c_int,
) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut head: *mut libc::c_char = 0 as *mut libc::c_char;
    if a.is_null() {
        head = b;
    } else if b.is_null() {
        head = a;
    } else {
        if ::std::mem::transmute::<
            _,
            fn(_, _) -> libc::c_int,
        >(
            (Some(cmp.expect("non-null function pointer")))
                .expect("non-null function pointer"),
        )(a, b) < 0 as libc::c_int
        {
            ptr = a;
            a = *((a as libc::c_ulong).wrapping_add(offset as libc::c_ulong)
                as *mut *mut libc::c_char);
        } else {
            ptr = b;
            b = *((b as libc::c_ulong).wrapping_add(offset as libc::c_ulong)
                as *mut *mut libc::c_char);
        }
        head = ptr;
        while !a.is_null() && !b.is_null() {
            if ::std::mem::transmute::<
                _,
                fn(_, _) -> libc::c_int,
            >(
                (Some(cmp.expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(a, b) < 0 as libc::c_int
            {
                let ref mut fresh6 = *((ptr as libc::c_ulong)
                    .wrapping_add(offset as libc::c_ulong) as *mut *mut libc::c_char);
                *fresh6 = a;
                ptr = a;
                a = *((a as libc::c_ulong).wrapping_add(offset as libc::c_ulong)
                    as *mut *mut libc::c_char);
            } else {
                let ref mut fresh7 = *((ptr as libc::c_ulong)
                    .wrapping_add(offset as libc::c_ulong) as *mut *mut libc::c_char);
                *fresh7 = b;
                ptr = b;
                b = *((b as libc::c_ulong).wrapping_add(offset as libc::c_ulong)
                    as *mut *mut libc::c_char);
            }
        }
        if !a.is_null() {
            let ref mut fresh8 = *((ptr as libc::c_ulong)
                .wrapping_add(offset as libc::c_ulong) as *mut *mut libc::c_char);
            *fresh8 = a;
        } else {
            let ref mut fresh9 = *((ptr as libc::c_ulong)
                .wrapping_add(offset as libc::c_ulong) as *mut *mut libc::c_char);
            *fresh9 = b;
        }
    }
    return head;
}
#[no_mangle]
pub unsafe extern "C" fn msort(
    mut list: *mut libc::c_void,
    mut next: *mut *mut libc::c_void,
    mut cmp: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
    >,
) -> *mut libc::c_void {
    let mut offset: libc::c_ulong = 0;
    let mut ep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut set: [*mut libc::c_char; 30] = [0 as *mut libc::c_char; 30];
    let mut i: libc::c_int = 0;
    offset = (next as libc::c_ulong).wrapping_sub(list as libc::c_ulong);
    i = 0 as libc::c_int;
    while i < 30 as libc::c_int {
        set[i as usize] = 0 as *mut libc::c_char;
        i += 1;
    }
    while !list.is_null() {
        ep = list as *mut libc::c_char;
        list = *((list as libc::c_ulong).wrapping_add(offset) as *mut *mut libc::c_char)
            as *mut libc::c_void;
        let ref mut fresh10 = *((ep as libc::c_ulong).wrapping_add(offset)
            as *mut *mut libc::c_char);
        *fresh10 = 0 as *mut libc::c_char;
        i = 0 as libc::c_int;
        while i < 30 as libc::c_int - 1 as libc::c_int && !(set[i as usize]).is_null() {
            ep = merge(
                ep,
                set[i as usize],
                ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                    >,
                    Option::<unsafe extern "C" fn() -> libc::c_int>,
                >(cmp),
                offset as libc::c_int,
            );
            set[i as usize] = 0 as *mut libc::c_char;
            i += 1;
        }
        set[i as usize] = ep;
    }
    ep = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < 30 as libc::c_int {
        if !(set[i as usize]).is_null() {
            ep = merge(
                ep,
                set[i as usize],
                ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                    >,
                    Option::<unsafe extern "C" fn() -> libc::c_int>,
                >(cmp),
                offset as libc::c_int,
            );
        }
        i += 1;
    }
    return ep as *mut libc::c_void;
}
static mut argv: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut op: *mut s_options = 0 as *const s_options as *mut s_options;
static mut errstream: *mut FILE = 0 as *const FILE as *mut FILE;
unsafe extern "C" fn errline(
    mut n: libc::c_int,
    mut k: libc::c_int,
    mut err: *mut FILE,
) {
    let mut spcnt: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    if !(*argv.offset(0 as libc::c_int as isize)).is_null() {
        fprintf(
            err,
            b"%s\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        spcnt = (spcnt as libc::c_ulong)
            .wrapping_add(
                (strlen(*argv.offset(0 as libc::c_int as isize)))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as libc::c_int as libc::c_int;
    }
    i = 1 as libc::c_int;
    while i < n && !(*argv.offset(i as isize)).is_null() {
        fprintf(
            err,
            b" %s\0" as *const u8 as *const libc::c_char,
            *argv.offset(i as isize),
        );
        spcnt = (spcnt as libc::c_ulong)
            .wrapping_add(
                (strlen(*argv.offset(i as isize)))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as libc::c_int as libc::c_int;
        i += 1;
    }
    spcnt += k;
    while !(*argv.offset(i as isize)).is_null() {
        fprintf(
            err,
            b" %s\0" as *const u8 as *const libc::c_char,
            *argv.offset(i as isize),
        );
        i += 1;
    }
    if spcnt < 20 as libc::c_int {
        fprintf(
            err,
            b"\n%*s^-- here\n\0" as *const u8 as *const libc::c_char,
            spcnt,
            b"\0" as *const u8 as *const libc::c_char,
        );
    } else {
        fprintf(
            err,
            b"\n%*shere --^\n\0" as *const u8 as *const libc::c_char,
            spcnt - 7 as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn argindex(mut n: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut dashdash: libc::c_int = 0 as libc::c_int;
    if !argv.is_null() && !(*argv).is_null() {
        i = 1 as libc::c_int;
        while !(*argv.offset(i as isize)).is_null() {
            if dashdash != 0
                || !(*(*argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
                    || *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                        as libc::c_int == '+' as i32
                    || !(strchr(*argv.offset(i as isize), '=' as i32)).is_null())
            {
                if n == 0 as libc::c_int {
                    return i;
                }
                n -= 1;
            }
            if strcmp(
                *argv.offset(i as isize),
                b"--\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                dashdash = 1 as libc::c_int;
            }
            i += 1;
        }
    }
    return -(1 as libc::c_int);
}
static mut emsg: [libc::c_char; 28] = unsafe {
    *::std::mem::transmute::<
        &[u8; 28],
        &mut [libc::c_char; 28],
    >(b"Command line syntax error: \0")
};
unsafe extern "C" fn handleflags(mut i: libc::c_int, mut err: *mut FILE) -> libc::c_int {
    let mut v: libc::c_int = 0;
    let mut errcnt: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while !((*op.offset(j as isize)).label).is_null() {
        if strcmp(
            &mut *(*argv.offset(i as isize)).offset(1 as libc::c_int as isize),
            (*op.offset(j as isize)).label,
        ) == 0 as libc::c_int
        {
            break;
        }
        j += 1;
    }
    v = if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
        == '-' as i32
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    if ((*op.offset(j as isize)).label).is_null() {
        if !err.is_null() {
            fprintf(
                err,
                b"%sundefined option.\n\0" as *const u8 as *const libc::c_char,
                emsg.as_mut_ptr(),
            );
            errline(i, 1 as libc::c_int, err);
        }
        errcnt += 1;
    } else if (*op.offset(j as isize)).type_0 as libc::c_uint
            == OPT_FLAG as libc::c_int as libc::c_uint
        {
        *((*op.offset(j as isize)).arg as *mut libc::c_int) = v;
    } else if (*op.offset(j as isize)).type_0 as libc::c_uint
            == OPT_FFLAG as libc::c_int as libc::c_uint
        {
        ::std::mem::transmute::<
            _,
            fn(_),
        >(
            (Some(
                (::std::mem::transmute::<
                    libc::intptr_t,
                    Option::<unsafe extern "C" fn() -> ()>,
                >((*op.offset(j as isize)).arg as intptr_t as libc::intptr_t))
                    .expect("non-null function pointer"),
            ))
                .expect("non-null function pointer"),
        )(v);
    } else {
        if !err.is_null() {
            fprintf(
                err,
                b"%smissing argument on switch.\n\0" as *const u8 as *const libc::c_char,
                emsg.as_mut_ptr(),
            );
            errline(i, 1 as libc::c_int, err);
        }
        errcnt += 1;
    }
    return errcnt;
}
unsafe extern "C" fn handleswitch(
    mut i: libc::c_int,
    mut err: *mut FILE,
) -> libc::c_int {
    let mut lv: libc::c_int = 0 as libc::c_int;
    let mut dv: libc::c_double = 0.0f64;
    let mut sv: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: libc::c_int = 0;
    let mut errcnt: libc::c_int = 0 as libc::c_int;
    cp = strchr(*argv.offset(i as isize), '=' as i32);
    if cp.is_null() {
        return 1 as libc::c_int;
    }
    *cp = 0 as libc::c_int as libc::c_char;
    j = 0 as libc::c_int;
    while !((*op.offset(j as isize)).label).is_null() {
        if strcmp(*argv.offset(i as isize), (*op.offset(j as isize)).label)
            == 0 as libc::c_int
        {
            break;
        }
        j += 1;
    }
    *cp = '=' as i32 as libc::c_char;
    if ((*op.offset(j as isize)).label).is_null() {
        if !err.is_null() {
            fprintf(
                err,
                b"%sundefined option.\n\0" as *const u8 as *const libc::c_char,
                emsg.as_mut_ptr(),
            );
            errline(i, 0 as libc::c_int, err);
        }
        errcnt += 1;
    } else {
        cp = cp.offset(1);
        match (*op.offset(j as isize)).type_0 as libc::c_uint {
            1 | 5 => {
                if !err.is_null() {
                    fprintf(
                        err,
                        b"%soption requires an argument.\n\0" as *const u8
                            as *const libc::c_char,
                        emsg.as_mut_ptr(),
                    );
                    errline(i, 0 as libc::c_int, err);
                }
                errcnt += 1;
            }
            3 | 7 => {
                dv = strtod(cp, &mut end);
                if *end != 0 {
                    if !err.is_null() {
                        fprintf(
                            err,
                            b"%sillegal character in floating-point argument.\n\0"
                                as *const u8 as *const libc::c_char,
                            emsg.as_mut_ptr(),
                        );
                        errline(
                            i,
                            (end as libc::c_ulong)
                                .wrapping_sub(*argv.offset(i as isize) as libc::c_ulong)
                                as libc::c_int,
                            err,
                        );
                    }
                    errcnt += 1;
                }
            }
            2 | 6 => {
                lv = strtol(cp, &mut end, 0 as libc::c_int) as libc::c_int;
                if *end != 0 {
                    if !err.is_null() {
                        fprintf(
                            err,
                            b"%sillegal character in integer argument.\n\0" as *const u8
                                as *const libc::c_char,
                            emsg.as_mut_ptr(),
                        );
                        errline(
                            i,
                            (end as libc::c_ulong)
                                .wrapping_sub(*argv.offset(i as isize) as libc::c_ulong)
                                as libc::c_int,
                            err,
                        );
                    }
                    errcnt += 1;
                }
            }
            4 | 8 => {
                sv = cp;
            }
            _ => {}
        }
        match (*op.offset(j as isize)).type_0 as libc::c_uint {
            3 => {
                *((*op.offset(j as isize)).arg as *mut libc::c_double) = dv;
            }
            7 => {
                ::std::mem::transmute::<
                    _,
                    fn(_),
                >(
                    (Some(
                        (::std::mem::transmute::<
                            libc::intptr_t,
                            Option::<unsafe extern "C" fn() -> ()>,
                        >((*op.offset(j as isize)).arg as intptr_t as libc::intptr_t))
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer"),
                )(dv);
            }
            2 => {
                *((*op.offset(j as isize)).arg as *mut libc::c_int) = lv;
            }
            6 => {
                ::std::mem::transmute::<
                    _,
                    fn(_),
                >(
                    (Some(
                        (::std::mem::transmute::<
                            libc::intptr_t,
                            Option::<unsafe extern "C" fn() -> ()>,
                        >((*op.offset(j as isize)).arg as intptr_t as libc::intptr_t))
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer"),
                )(lv);
            }
            4 => {
                let ref mut fresh11 = *((*op.offset(j as isize)).arg
                    as *mut *mut libc::c_char);
                *fresh11 = sv;
            }
            8 => {
                ::std::mem::transmute::<
                    _,
                    fn(_),
                >(
                    (Some(
                        (::std::mem::transmute::<
                            libc::intptr_t,
                            Option::<unsafe extern "C" fn() -> ()>,
                        >((*op.offset(j as isize)).arg as intptr_t as libc::intptr_t))
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer"),
                )(sv);
            }
            1 | 5 | _ => {}
        }
    }
    return errcnt;
}
#[no_mangle]
pub unsafe extern "C" fn OptInit(
    mut a: *mut *mut libc::c_char,
    mut o: *mut s_options,
    mut err: *mut FILE,
) -> libc::c_int {
    let mut errcnt: libc::c_int = 0 as libc::c_int;
    argv = a;
    op = o;
    errstream = err;
    if !argv.is_null() && !(*argv).is_null() && !op.is_null() {
        let mut i: libc::c_int = 0;
        i = 1 as libc::c_int;
        while !(*argv.offset(i as isize)).is_null() {
            if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int == '+' as i32
                || *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
            {
                errcnt += handleflags(i, err);
            } else if !(strchr(*argv.offset(i as isize), '=' as i32)).is_null() {
                errcnt += handleswitch(i, err);
            }
            i += 1;
        }
    }
    if errcnt > 0 as libc::c_int {
        fprintf(
            err,
            b"Valid command line options for \"%s\" are:\n\0" as *const u8
                as *const libc::c_char,
            *a,
        );
        OptPrint();
        exit(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn OptNArgs() -> libc::c_int {
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut dashdash: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    if !argv.is_null() && !(*argv.offset(0 as libc::c_int as isize)).is_null() {
        i = 1 as libc::c_int;
        while !(*argv.offset(i as isize)).is_null() {
            if dashdash != 0
                || !(*(*argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
                    || *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                        as libc::c_int == '+' as i32
                    || !(strchr(*argv.offset(i as isize), '=' as i32)).is_null())
            {
                cnt += 1;
            }
            if strcmp(
                *argv.offset(i as isize),
                b"--\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                dashdash = 1 as libc::c_int;
            }
            i += 1;
        }
    }
    return cnt;
}
#[no_mangle]
pub unsafe extern "C" fn OptArg(mut n: libc::c_int) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    i = argindex(n);
    return if i >= 0 as libc::c_int {
        *argv.offset(i as isize)
    } else {
        0 as *mut libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn OptErr(mut n: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = argindex(n);
    if i >= 0 as libc::c_int {
        errline(i, 0 as libc::c_int, errstream);
    }
}
#[no_mangle]
pub unsafe extern "C" fn OptPrint() {
    let mut i: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    max = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while !((*op.offset(i as isize)).label).is_null() {
        len = (strlen((*op.offset(i as isize)).label))
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        match (*op.offset(i as isize)).type_0 as libc::c_uint {
            2 | 6 => {
                len += 9 as libc::c_int;
            }
            3 | 7 => {
                len += 6 as libc::c_int;
            }
            4 | 8 => {
                len += 8 as libc::c_int;
            }
            1 | 5 | _ => {}
        }
        if len > max {
            max = len;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while !((*op.offset(i as isize)).label).is_null() {
        match (*op.offset(i as isize)).type_0 as libc::c_uint {
            1 | 5 => {
                fprintf(
                    errstream,
                    b"  -%-*s  %s\n\0" as *const u8 as *const libc::c_char,
                    max,
                    (*op.offset(i as isize)).label,
                    (*op.offset(i as isize)).message,
                );
            }
            2 | 6 => {
                fprintf(
                    errstream,
                    b"  %s=<integer>%*s  %s\n\0" as *const u8 as *const libc::c_char,
                    (*op.offset(i as isize)).label,
                    (max as libc::c_ulong)
                        .wrapping_sub(strlen((*op.offset(i as isize)).label))
                        .wrapping_sub(9 as libc::c_int as libc::c_ulong) as libc::c_int,
                    b"\0" as *const u8 as *const libc::c_char,
                    (*op.offset(i as isize)).message,
                );
            }
            3 | 7 => {
                fprintf(
                    errstream,
                    b"  %s=<real>%*s  %s\n\0" as *const u8 as *const libc::c_char,
                    (*op.offset(i as isize)).label,
                    (max as libc::c_ulong)
                        .wrapping_sub(strlen((*op.offset(i as isize)).label))
                        .wrapping_sub(6 as libc::c_int as libc::c_ulong) as libc::c_int,
                    b"\0" as *const u8 as *const libc::c_char,
                    (*op.offset(i as isize)).message,
                );
            }
            4 | 8 => {
                fprintf(
                    errstream,
                    b"  %s=<string>%*s  %s\n\0" as *const u8 as *const libc::c_char,
                    (*op.offset(i as isize)).label,
                    (max as libc::c_ulong)
                        .wrapping_sub(strlen((*op.offset(i as isize)).label))
                        .wrapping_sub(8 as libc::c_int as libc::c_ulong) as libc::c_int,
                    b"\0" as *const u8 as *const libc::c_char,
                    (*op.offset(i as isize)).message,
                );
            }
            _ => {}
        }
        i += 1;
    }
}
unsafe extern "C" fn parseonetoken(mut psp: *mut pstate) {
    let mut x: *mut libc::c_char = 0 as *mut libc::c_char;
    x = Strsafe((*psp).tokenstart);
    let mut current_block_315: u64;
    match (*psp).state as libc::c_uint {
        0 => {
            (*psp).prevrule = 0 as *mut rule;
            (*psp).preccounter = 0 as libc::c_int;
            (*psp).lastrule = 0 as *mut rule;
            (*psp).firstrule = (*psp).lastrule;
            (*(*psp).gp).nrule = 0 as libc::c_int;
            current_block_315 = 3417363963158526890;
        }
        1 => {
            current_block_315 = 3417363963158526890;
        }
        12 => {
            if *(*__ctype_b_loc())
                .offset(*x.offset(0 as libc::c_int as isize) as libc::c_int as isize)
                as libc::c_int & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                == 0
            {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"The precedence symbol must be a terminal.\0" as *const u8
                        as *const libc::c_char,
                );
                (*psp).errorcnt += 1;
            } else if ((*psp).prevrule).is_null() {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"There is no prior rule to assign precedence \"[%s]\".\0"
                        as *const u8 as *const libc::c_char,
                    x,
                );
                (*psp).errorcnt += 1;
            } else if !((*(*psp).prevrule).precsym).is_null() {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Precedence mark on this line is not the first to follow the previous rule.\0"
                        as *const u8 as *const libc::c_char,
                );
                (*psp).errorcnt += 1;
            } else {
                (*(*psp).prevrule).precsym = Symbol_new(x);
            }
            (*psp).state = PRECEDENCE_MARK_2;
            current_block_315 = 2093145933725580773;
        }
        13 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int != ']' as i32 {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Missing \"]\" on precedence mark.\0" as *const u8
                        as *const libc::c_char,
                );
                (*psp).errorcnt += 1;
            }
            (*psp).state = WAITING_FOR_DECL_OR_RULE;
            current_block_315 = 2093145933725580773;
        }
        5 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32
                && *x.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32
                && *x.offset(2 as libc::c_int as isize) as libc::c_int == '=' as i32
            {
                (*psp).state = IN_RHS;
            } else if *x.offset(0 as libc::c_int as isize) as libc::c_int == '(' as i32 {
                (*psp).state = LHS_ALIAS_1;
            } else {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Expected to see a \":\" following the LHS symbol \"%s\".\0"
                        as *const u8 as *const libc::c_char,
                    (*(*psp).lhs).name,
                );
                (*psp).errorcnt += 1;
                (*psp).state = RESYNC_AFTER_RULE_ERROR;
            }
            current_block_315 = 2093145933725580773;
        }
        7 => {
            if *(*__ctype_b_loc())
                .offset(*x.offset(0 as libc::c_int as isize) as libc::c_int as isize)
                as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                (*psp).lhsalias = x;
                (*psp).state = LHS_ALIAS_2;
            } else {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"\"%s\" is not a valid alias for the LHS \"%s\"\n\0" as *const u8
                        as *const libc::c_char,
                    x,
                    (*(*psp).lhs).name,
                );
                (*psp).errorcnt += 1;
                (*psp).state = RESYNC_AFTER_RULE_ERROR;
            }
            current_block_315 = 2093145933725580773;
        }
        8 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == ')' as i32 {
                (*psp).state = LHS_ALIAS_3;
            } else {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Missing \")\" following LHS alias name \"%s\".\0" as *const u8
                        as *const libc::c_char,
                    (*psp).lhsalias,
                );
                (*psp).errorcnt += 1;
                (*psp).state = RESYNC_AFTER_RULE_ERROR;
            }
            current_block_315 = 2093145933725580773;
        }
        9 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32
                && *x.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32
                && *x.offset(2 as libc::c_int as isize) as libc::c_int == '=' as i32
            {
                (*psp).state = IN_RHS;
            } else {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Missing \"->\" following: \"%s(%s)\".\0" as *const u8
                        as *const libc::c_char,
                    (*(*psp).lhs).name,
                    (*psp).lhsalias,
                );
                (*psp).errorcnt += 1;
                (*psp).state = RESYNC_AFTER_RULE_ERROR;
            }
            current_block_315 = 2093145933725580773;
        }
        6 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
                let mut rp: *mut rule = 0 as *mut rule;
                rp = malloc(
                    (::std::mem::size_of::<rule>() as libc::c_ulong)
                        .wrapping_add(
                            (::std::mem::size_of::<*mut symbol>() as libc::c_ulong)
                                .wrapping_mul((*psp).nrhs as libc::c_ulong),
                        )
                        .wrapping_add(
                            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                                .wrapping_mul((*psp).nrhs as libc::c_ulong),
                        ),
                ) as *mut rule;
                if rp.is_null() {
                    ErrorMsg(
                        (*psp).filename,
                        (*psp).tokenlineno,
                        b"Can't allocate enough memory for this rule.\0" as *const u8
                            as *const libc::c_char,
                    );
                    (*psp).errorcnt += 1;
                    (*psp).prevrule = 0 as *mut rule;
                } else {
                    let mut i: libc::c_int = 0;
                    (*rp).ruleline = (*psp).tokenlineno;
                    (*rp)
                        .rhs = &mut *rp.offset(1 as libc::c_int as isize) as *mut rule
                        as *mut *mut symbol;
                    (*rp)
                        .rhsalias = &mut *((*rp).rhs).offset((*psp).nrhs as isize)
                        as *mut *mut symbol as *mut *mut libc::c_char;
                    i = 0 as libc::c_int;
                    while i < (*psp).nrhs {
                        let ref mut fresh12 = *((*rp).rhs).offset(i as isize);
                        *fresh12 = (*psp).rhs[i as usize];
                        let ref mut fresh13 = *((*rp).rhsalias).offset(i as isize);
                        *fresh13 = (*psp).alias[i as usize];
                        i += 1;
                    }
                    (*rp).lhs = (*psp).lhs;
                    (*rp).lhsalias = (*psp).lhsalias;
                    (*rp).nrhs = (*psp).nrhs;
                    (*rp).code = 0 as *mut libc::c_char;
                    (*rp).precsym = 0 as *mut symbol;
                    let fresh14 = (*(*psp).gp).nrule;
                    (*(*psp).gp).nrule = (*(*psp).gp).nrule + 1;
                    (*rp).index = fresh14;
                    (*rp).nextlhs = (*(*rp).lhs).rule;
                    (*(*rp).lhs).rule = rp;
                    (*rp).next = 0 as *mut rule;
                    if ((*psp).firstrule).is_null() {
                        (*psp).lastrule = rp;
                        (*psp).firstrule = (*psp).lastrule;
                    } else {
                        (*(*psp).lastrule).next = rp;
                        (*psp).lastrule = rp;
                    }
                    (*psp).prevrule = rp;
                }
                (*psp).state = WAITING_FOR_DECL_OR_RULE;
            } else if *(*__ctype_b_loc())
                    .offset(*x.offset(0 as libc::c_int as isize) as libc::c_int as isize)
                    as libc::c_int
                    & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                if (*psp).nrhs >= 1000 as libc::c_int {
                    ErrorMsg(
                        (*psp).filename,
                        (*psp).tokenlineno,
                        b"Too many symbol on RHS or rule beginning at \"%s\".\0"
                            as *const u8 as *const libc::c_char,
                        x,
                    );
                    (*psp).errorcnt += 1;
                    (*psp).state = RESYNC_AFTER_RULE_ERROR;
                } else {
                    (*psp).rhs[(*psp).nrhs as usize] = Symbol_new(x);
                    (*psp).alias[(*psp).nrhs as usize] = 0 as *mut libc::c_char;
                    (*psp).nrhs += 1;
                }
            } else if *x.offset(0 as libc::c_int as isize) as libc::c_int == '(' as i32
                    && (*psp).nrhs > 0 as libc::c_int
                {
                (*psp).state = RHS_ALIAS_1;
            } else {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Illegal character on RHS of rule: \"%s\".\0" as *const u8
                        as *const libc::c_char,
                    x,
                );
                (*psp).errorcnt += 1;
                (*psp).state = RESYNC_AFTER_RULE_ERROR;
            }
            current_block_315 = 2093145933725580773;
        }
        10 => {
            if *(*__ctype_b_loc())
                .offset(*x.offset(0 as libc::c_int as isize) as libc::c_int as isize)
                as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                (*psp).alias[((*psp).nrhs - 1 as libc::c_int) as usize] = x;
                (*psp).state = RHS_ALIAS_2;
            } else {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"\"%s\" is not a valid alias for the RHS symbol \"%s\"\n\0"
                        as *const u8 as *const libc::c_char,
                    x,
                    (*(*psp).rhs[((*psp).nrhs - 1 as libc::c_int) as usize]).name,
                );
                (*psp).errorcnt += 1;
                (*psp).state = RESYNC_AFTER_RULE_ERROR;
            }
            current_block_315 = 2093145933725580773;
        }
        11 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == ')' as i32 {
                (*psp).state = IN_RHS;
            } else {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Missing \")\" following LHS alias name \"%s\".\0" as *const u8
                        as *const libc::c_char,
                    (*psp).lhsalias,
                );
                (*psp).errorcnt += 1;
                (*psp).state = RESYNC_AFTER_RULE_ERROR;
            }
            current_block_315 = 2093145933725580773;
        }
        2 => {
            if *(*__ctype_b_loc())
                .offset(*x.offset(0 as libc::c_int as isize) as libc::c_int as isize)
                as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                (*psp).declkeyword = x;
                (*psp).declargslot = 0 as *mut *mut libc::c_char;
                (*psp).decllnslot = 0 as *mut libc::c_int;
                (*psp).state = WAITING_FOR_DECL_ARG;
                if strcmp(x, b"name\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    (*psp).declargslot = &mut (*(*psp).gp).name;
                } else if strcmp(x, b"include\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                    (*psp).declargslot = &mut (*(*psp).gp).include;
                    (*psp).decllnslot = &mut (*(*psp).gp).includeln;
                } else if strcmp(x, b"code\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                    (*psp).declargslot = &mut (*(*psp).gp).extracode;
                    (*psp).decllnslot = &mut (*(*psp).gp).extracodeln;
                } else if strcmp(
                        x,
                        b"token_destructor\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                    (*psp).declargslot = &mut (*(*psp).gp).tokendest;
                    (*psp).decllnslot = &mut (*(*psp).gp).tokendestln;
                } else if strcmp(
                        x,
                        b"default_destructor\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                    (*psp).declargslot = &mut (*(*psp).gp).vardest;
                    (*psp).decllnslot = &mut (*(*psp).gp).vardestln;
                } else if strcmp(
                        x,
                        b"token_prefix\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                    (*psp).declargslot = &mut (*(*psp).gp).tokenprefix;
                } else if strcmp(
                        x,
                        b"syntax_error\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                    (*psp).declargslot = &mut (*(*psp).gp).error;
                    (*psp).decllnslot = &mut (*(*psp).gp).errorln;
                } else if strcmp(
                        x,
                        b"parse_accept\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                    (*psp).declargslot = &mut (*(*psp).gp).accept;
                    (*psp).decllnslot = &mut (*(*psp).gp).acceptln;
                } else if strcmp(
                        x,
                        b"parse_failure\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                    (*psp).declargslot = &mut (*(*psp).gp).failure;
                    (*psp).decllnslot = &mut (*(*psp).gp).failureln;
                } else if strcmp(
                        x,
                        b"stack_overflow\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                    (*psp).declargslot = &mut (*(*psp).gp).overflow;
                    (*psp).decllnslot = &mut (*(*psp).gp).overflowln;
                } else if strcmp(
                        x,
                        b"extra_argument\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                    (*psp).declargslot = &mut (*(*psp).gp).arg;
                } else if strcmp(x, b"token_type\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                    (*psp).declargslot = &mut (*(*psp).gp).tokentype;
                } else if strcmp(
                        x,
                        b"default_type\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                    (*psp).declargslot = &mut (*(*psp).gp).vartype;
                } else if strcmp(x, b"stack_size\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                    (*psp).declargslot = &mut (*(*psp).gp).stacksize;
                } else if strcmp(
                        x,
                        b"start_symbol\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                    (*psp).declargslot = &mut (*(*psp).gp).start;
                } else if strcmp(x, b"left\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                    (*psp).preccounter += 1;
                    (*psp).declassoc = LEFT;
                    (*psp).state = WAITING_FOR_PRECEDENCE_SYMBOL;
                } else if strcmp(x, b"right\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                    (*psp).preccounter += 1;
                    (*psp).declassoc = RIGHT;
                    (*psp).state = WAITING_FOR_PRECEDENCE_SYMBOL;
                } else if strcmp(x, b"nonassoc\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                    (*psp).preccounter += 1;
                    (*psp).declassoc = NONE;
                    (*psp).state = WAITING_FOR_PRECEDENCE_SYMBOL;
                } else if strcmp(x, b"destructor\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                    (*psp).state = WAITING_FOR_DESTRUCTOR_SYMBOL;
                } else if strcmp(x, b"type\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                    (*psp).state = WAITING_FOR_DATATYPE_SYMBOL;
                } else if strcmp(x, b"fallback\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                    (*psp).fallback = 0 as *mut symbol;
                    (*psp).state = WAITING_FOR_FALLBACK_ID;
                } else {
                    ErrorMsg(
                        (*psp).filename,
                        (*psp).tokenlineno,
                        b"Unknown declaration keyword: \"%%%s\".\0" as *const u8
                            as *const libc::c_char,
                        x,
                    );
                    (*psp).errorcnt += 1;
                    (*psp).state = RESYNC_AFTER_DECL_ERROR;
                }
            } else {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Illegal declaration keyword: \"%s\".\0" as *const u8
                        as *const libc::c_char,
                    x,
                );
                (*psp).errorcnt += 1;
                (*psp).state = RESYNC_AFTER_DECL_ERROR;
            }
            current_block_315 = 2093145933725580773;
        }
        16 => {
            if *(*__ctype_b_loc())
                .offset(*x.offset(0 as libc::c_int as isize) as libc::c_int as isize)
                as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                == 0
            {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Symbol name missing after %%destructor keyword\0" as *const u8
                        as *const libc::c_char,
                );
                (*psp).errorcnt += 1;
                (*psp).state = RESYNC_AFTER_DECL_ERROR;
            } else {
                let mut sp: *mut symbol = Symbol_new(x);
                (*psp).declargslot = &mut (*sp).destructor;
                (*psp).decllnslot = &mut (*sp).destructorln;
                (*psp).state = WAITING_FOR_DECL_ARG;
            }
            current_block_315 = 2093145933725580773;
        }
        17 => {
            if *(*__ctype_b_loc())
                .offset(*x.offset(0 as libc::c_int as isize) as libc::c_int as isize)
                as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                == 0
            {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Symbol name missing after %%destructor keyword\0" as *const u8
                        as *const libc::c_char,
                );
                (*psp).errorcnt += 1;
                (*psp).state = RESYNC_AFTER_DECL_ERROR;
            } else {
                let mut sp_0: *mut symbol = Symbol_new(x);
                (*psp).declargslot = &mut (*sp_0).datatype;
                (*psp).decllnslot = 0 as *mut libc::c_int;
                (*psp).state = WAITING_FOR_DECL_ARG;
            }
            current_block_315 = 2093145933725580773;
        }
        4 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
                (*psp).state = WAITING_FOR_DECL_OR_RULE;
            } else if *(*__ctype_b_loc())
                    .offset(*x.offset(0 as libc::c_int as isize) as libc::c_int as isize)
                    as libc::c_int
                    & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                let mut sp_1: *mut symbol = 0 as *mut symbol;
                sp_1 = Symbol_new(x);
                if (*sp_1).prec >= 0 as libc::c_int {
                    ErrorMsg(
                        (*psp).filename,
                        (*psp).tokenlineno,
                        b"Symbol \"%s\" has already be given a precedence.\0"
                            as *const u8 as *const libc::c_char,
                        x,
                    );
                    (*psp).errorcnt += 1;
                } else {
                    (*sp_1).prec = (*psp).preccounter;
                    (*sp_1).assoc = (*psp).declassoc;
                }
            } else {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Can't assign a precedence to \"%s\".\0" as *const u8
                        as *const libc::c_char,
                    x,
                );
                (*psp).errorcnt += 1;
            }
            current_block_315 = 2093145933725580773;
        }
        3 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == '{' as i32
                || *x.offset(0 as libc::c_int as isize) as libc::c_int == '"' as i32
                || *(*__ctype_b_loc())
                    .offset(*x.offset(0 as libc::c_int as isize) as libc::c_int as isize)
                    as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                if !(*(*psp).declargslot).is_null() {
                    ErrorMsg(
                        (*psp).filename,
                        (*psp).tokenlineno,
                        b"The argument \"%s\" to declaration \"%%%s\" is not the first.\0"
                            as *const u8 as *const libc::c_char,
                        if *x.offset(0 as libc::c_int as isize) as libc::c_int
                            == '"' as i32
                        {
                            &mut *x.offset(1 as libc::c_int as isize)
                                as *mut libc::c_char
                        } else {
                            x
                        },
                        (*psp).declkeyword,
                    );
                    (*psp).errorcnt += 1;
                    (*psp).state = RESYNC_AFTER_DECL_ERROR;
                } else {
                    *(*psp)
                        .declargslot = if *x.offset(0 as libc::c_int as isize)
                        as libc::c_int == '"' as i32
                        || *x.offset(0 as libc::c_int as isize) as libc::c_int
                            == '{' as i32
                    {
                        &mut *x.offset(1 as libc::c_int as isize) as *mut libc::c_char
                    } else {
                        x
                    };
                    if !((*psp).decllnslot).is_null() {
                        *(*psp).decllnslot = (*psp).tokenlineno;
                    }
                    (*psp).state = WAITING_FOR_DECL_OR_RULE;
                }
            } else {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Illegal argument to %%%s: %s\0" as *const u8
                        as *const libc::c_char,
                    (*psp).declkeyword,
                    x,
                );
                (*psp).errorcnt += 1;
                (*psp).state = RESYNC_AFTER_DECL_ERROR;
            }
            current_block_315 = 2093145933725580773;
        }
        18 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
                (*psp).state = WAITING_FOR_DECL_OR_RULE;
            } else if *(*__ctype_b_loc())
                    .offset(*x.offset(0 as libc::c_int as isize) as libc::c_int as isize)
                    as libc::c_int
                    & _ISupper as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"%%fallback argument \"%s\" should be a token\0" as *const u8
                        as *const libc::c_char,
                    x,
                );
                (*psp).errorcnt += 1;
            } else {
                let mut sp_2: *mut symbol = Symbol_new(x);
                if ((*psp).fallback).is_null() {
                    (*psp).fallback = sp_2;
                } else if !((*sp_2).fallback).is_null() {
                    ErrorMsg(
                        (*psp).filename,
                        (*psp).tokenlineno,
                        b"More than one fallback assigned to token %s\0" as *const u8
                            as *const libc::c_char,
                        x,
                    );
                    (*psp).errorcnt += 1;
                } else {
                    (*sp_2).fallback = (*psp).fallback;
                    (*(*psp).gp).has_fallback = 1 as libc::c_int;
                }
            }
            current_block_315 = 2093145933725580773;
        }
        14 | 15 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
                (*psp).state = WAITING_FOR_DECL_OR_RULE;
            }
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32 {
                (*psp).state = WAITING_FOR_DECL_KEYWORD;
            }
            current_block_315 = 2093145933725580773;
        }
        _ => {
            current_block_315 = 2093145933725580773;
        }
    }
    match current_block_315 {
        3417363963158526890 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32 {
                (*psp).state = WAITING_FOR_DECL_KEYWORD;
            } else if *(*__ctype_b_loc())
                    .offset(*x.offset(0 as libc::c_int as isize) as libc::c_int as isize)
                    as libc::c_int
                    & _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                (*psp).lhs = Symbol_new(x);
                (*psp).nrhs = 0 as libc::c_int;
                (*psp).lhsalias = 0 as *mut libc::c_char;
                (*psp).state = WAITING_FOR_ARROW;
            } else if *x.offset(0 as libc::c_int as isize) as libc::c_int == '{' as i32 {
                if ((*psp).prevrule).is_null() {
                    ErrorMsg(
                        (*psp).filename,
                        (*psp).tokenlineno,
                        b"There is not prior rule opon which to attach the code fragment which begins on this line.\0"
                            as *const u8 as *const libc::c_char,
                    );
                    (*psp).errorcnt += 1;
                } else if !((*(*psp).prevrule).code).is_null() {
                    ErrorMsg(
                        (*psp).filename,
                        (*psp).tokenlineno,
                        b"Code fragment beginning on this line is not the first to follow the previous rule.\0"
                            as *const u8 as *const libc::c_char,
                    );
                    (*psp).errorcnt += 1;
                } else {
                    (*(*psp).prevrule).line = (*psp).tokenlineno;
                    (*(*psp).prevrule)
                        .code = &mut *x.offset(1 as libc::c_int as isize)
                        as *mut libc::c_char;
                }
            } else if *x.offset(0 as libc::c_int as isize) as libc::c_int == '[' as i32 {
                (*psp).state = PRECEDENCE_MARK_1;
            } else {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Token \"%s\" should be either \"%%\" or a nonterminal name.\0"
                        as *const u8 as *const libc::c_char,
                    x,
                );
                (*psp).errorcnt += 1;
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub static mut ps: pstate = pstate {
    filename: 0 as *const libc::c_char as *mut libc::c_char,
    tokenlineno: 0,
    errorcnt: 0,
    tokenstart: 0 as *const libc::c_char as *mut libc::c_char,
    gp: 0 as *const lemon as *mut lemon,
    state: INITIALIZE,
    fallback: 0 as *const symbol as *mut symbol,
    lhs: 0 as *const symbol as *mut symbol,
    lhsalias: 0 as *const libc::c_char as *mut libc::c_char,
    nrhs: 0,
    rhs: [0 as *const symbol as *mut symbol; 1000],
    alias: [0 as *const libc::c_char as *mut libc::c_char; 1000],
    prevrule: 0 as *const rule as *mut rule,
    declkeyword: 0 as *const libc::c_char as *mut libc::c_char,
    declargslot: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    decllnslot: 0 as *const libc::c_int as *mut libc::c_int,
    declassoc: LEFT,
    preccounter: 0,
    firstrule: 0 as *const rule as *mut rule,
    lastrule: 0 as *const rule as *mut rule,
};
#[no_mangle]
pub unsafe extern "C" fn Parse(mut gp: *mut lemon) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut filebuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filesize: size_t = 0;
    let mut lineno: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nextcp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut startline: libc::c_int = 0 as libc::c_int;
    let mut epos: libc::c_long = 0;
    ps.gp = gp;
    ps.filename = (*gp).filename;
    ps.errorcnt = 0 as libc::c_int;
    ps.state = INITIALIZE;
    fp = fopen(ps.filename, b"rb\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        ErrorMsg(
            ps.filename,
            0 as libc::c_int,
            b"Can't open this file for reading.\0" as *const u8 as *const libc::c_char,
        );
        (*gp).errorcnt += 1;
        return;
    }
    fseek(fp, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
    epos = ftell(fp);
    if -(1 as libc::c_int) as libc::c_long == epos {
        ErrorMsg(
            ps.filename,
            0 as libc::c_int,
            b"Can't determine file size.\0" as *const u8 as *const libc::c_char,
        );
        fclose(fp);
        (*gp).errorcnt += 1;
        return;
    }
    filesize = epos as size_t;
    rewind(fp);
    filebuf = malloc(filesize.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    if filebuf.is_null() {
        ErrorMsg(
            ps.filename,
            0 as libc::c_int,
            b"Can't allocate %zu of memory to hold this file.\0" as *const u8
                as *const libc::c_char,
            filesize.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        fclose(fp);
        (*gp).errorcnt += 1;
        return;
    }
    if fread(
        filebuf as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        filesize,
        fp,
    ) != filesize
    {
        ErrorMsg(
            ps.filename,
            0 as libc::c_int,
            b"Can't read in all %zu bytes of this file.\0" as *const u8
                as *const libc::c_char,
            filesize,
        );
        free(filebuf as *mut libc::c_void);
        fclose(fp);
        (*gp).errorcnt += 1;
        return;
    }
    fclose(fp);
    *filebuf.offset(filesize as isize) = 0 as libc::c_int as libc::c_char;
    lineno = 1 as libc::c_int;
    cp = filebuf;
    loop {
        c = *cp as libc::c_int;
        if !(c != 0 as libc::c_int) {
            break;
        }
        if c == '\n' as i32 {
            lineno += 1;
        }
        if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            cp = cp.offset(1);
        } else if c == '/' as i32
                && *cp.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
            {
            cp = cp.offset(2 as libc::c_int as isize);
            loop {
                c = *cp as libc::c_int;
                if !(c != 0 as libc::c_int && c != '\n' as i32) {
                    break;
                }
                cp = cp.offset(1);
            }
        } else if c == '/' as i32
                && *cp.offset(1 as libc::c_int as isize) as libc::c_int == '*' as i32
            {
            cp = cp.offset(2 as libc::c_int as isize);
            loop {
                c = *cp as libc::c_int;
                if !(c != 0 as libc::c_int
                    && (c != '/' as i32
                        || *cp.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            != '*' as i32))
                {
                    break;
                }
                if c == '\n' as i32 {
                    lineno += 1;
                }
                cp = cp.offset(1);
            }
            if c != 0 {
                cp = cp.offset(1);
            }
        } else {
            ps.tokenstart = cp;
            ps.tokenlineno = lineno;
            if c == '"' as i32 {
                cp = cp.offset(1);
                loop {
                    c = *cp as libc::c_int;
                    if !(c != 0 as libc::c_int && c != '"' as i32) {
                        break;
                    }
                    if c == '\n' as i32 {
                        lineno += 1;
                    }
                    cp = cp.offset(1);
                }
                if c == 0 as libc::c_int {
                    ErrorMsg(
                        ps.filename,
                        startline,
                        b"String starting on this line is not terminated before the end of the file.\0"
                            as *const u8 as *const libc::c_char,
                    );
                    ps.errorcnt += 1;
                    nextcp = cp;
                } else {
                    nextcp = cp.offset(1 as libc::c_int as isize);
                }
            } else if c == '{' as i32 {
                let mut level: libc::c_int = 0;
                cp = cp.offset(1);
                level = 1 as libc::c_int;
                loop {
                    c = *cp as libc::c_int;
                    if !(c != 0 as libc::c_int
                        && (level > 1 as libc::c_int || c != '}' as i32))
                    {
                        break;
                    }
                    if c == '\n' as i32 {
                        lineno += 1;
                    } else if c == '{' as i32 {
                        level += 1;
                    } else if c == '}' as i32 {
                        level -= 1;
                    } else if c == '/' as i32
                            && *cp.offset(1 as libc::c_int as isize) as libc::c_int
                                == '*' as i32
                        {
                        let mut prevc: libc::c_int = 0;
                        cp = &mut *cp.offset(2 as libc::c_int as isize)
                            as *mut libc::c_char;
                        prevc = 0 as libc::c_int;
                        loop {
                            c = *cp as libc::c_int;
                            if !(c != 0 as libc::c_int
                                && (c != '/' as i32 || prevc != '*' as i32))
                            {
                                break;
                            }
                            if c == '\n' as i32 {
                                lineno += 1;
                            }
                            prevc = c;
                            cp = cp.offset(1);
                        }
                    } else if c == '/' as i32
                            && *cp.offset(1 as libc::c_int as isize) as libc::c_int
                                == '/' as i32
                        {
                        cp = &mut *cp.offset(2 as libc::c_int as isize)
                            as *mut libc::c_char;
                        loop {
                            c = *cp as libc::c_int;
                            if !(c != 0 as libc::c_int && c != '\n' as i32) {
                                break;
                            }
                            cp = cp.offset(1);
                        }
                        if c != 0 {
                            lineno += 1;
                        }
                    } else if c == '\'' as i32 || c == '"' as i32 {
                        let mut startchar: libc::c_int = 0;
                        let mut prevc_0: libc::c_int = 0;
                        startchar = c;
                        prevc_0 = 0 as libc::c_int;
                        cp = cp.offset(1);
                        loop {
                            c = *cp as libc::c_int;
                            if !(c != 0 as libc::c_int
                                && (c != startchar || prevc_0 == '\\' as i32))
                            {
                                break;
                            }
                            if c == '\n' as i32 {
                                lineno += 1;
                            }
                            if prevc_0 == '\\' as i32 {
                                prevc_0 = 0 as libc::c_int;
                            } else {
                                prevc_0 = c;
                            }
                            cp = cp.offset(1);
                        }
                    }
                    cp = cp.offset(1);
                }
                if c == 0 as libc::c_int {
                    ErrorMsg(
                        ps.filename,
                        ps.tokenlineno,
                        b"C code starting on this line is not terminated before the end of the file.\0"
                            as *const u8 as *const libc::c_char,
                    );
                    ps.errorcnt += 1;
                    nextcp = cp;
                } else {
                    nextcp = cp.offset(1 as libc::c_int as isize);
                }
            } else if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                loop {
                    c = *cp as libc::c_int;
                    if !(c != 0 as libc::c_int
                        && (*(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                            != 0 || c == '_' as i32))
                    {
                        break;
                    }
                    cp = cp.offset(1);
                }
                nextcp = cp;
            } else if c == ':' as i32
                    && *cp.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32
                    && *cp.offset(2 as libc::c_int as isize) as libc::c_int == '=' as i32
                {
                cp = cp.offset(3 as libc::c_int as isize);
                nextcp = cp;
            } else {
                cp = cp.offset(1);
                nextcp = cp;
            }
            c = *cp as libc::c_int;
            *cp = 0 as libc::c_int as libc::c_char;
            parseonetoken(&mut ps);
            *cp = c as libc::c_char;
            cp = nextcp;
        }
    }
    free(filebuf as *mut libc::c_void);
    (*gp).rule = ps.firstrule;
    (*gp).errorcnt = ps.errorcnt;
}
static mut plink_freelist: *mut plink = 0 as *const plink as *mut plink;
#[no_mangle]
pub unsafe extern "C" fn Plink_new() -> *mut plink {
    let mut new: *mut plink = 0 as *mut plink;
    if plink_freelist.is_null() {
        let mut i: libc::c_int = 0;
        let mut amt: libc::c_int = 100 as libc::c_int;
        plink_freelist = malloc(
            (::std::mem::size_of::<plink>() as libc::c_ulong)
                .wrapping_mul(amt as libc::c_ulong),
        ) as *mut plink;
        if plink_freelist.is_null() {
            fprintf(
                stderr,
                b"Unable to allocate memory for a new follow-set propagation link.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        i = 0 as libc::c_int;
        while i < amt - 1 as libc::c_int {
            let ref mut fresh15 = (*plink_freelist.offset(i as isize)).next;
            *fresh15 = &mut *plink_freelist.offset((i + 1 as libc::c_int) as isize)
                as *mut plink;
            i += 1;
        }
        let ref mut fresh16 = (*plink_freelist.offset((amt - 1 as libc::c_int) as isize))
            .next;
        *fresh16 = 0 as *mut plink;
    }
    new = plink_freelist;
    plink_freelist = (*plink_freelist).next;
    return new;
}
#[no_mangle]
pub unsafe extern "C" fn Plink_add(mut plpp: *mut *mut plink, mut cfp: *mut config) {
    let mut new: *mut plink = 0 as *mut plink;
    new = Plink_new();
    (*new).next = *plpp;
    *plpp = new;
    (*new).cfp = cfp;
}
#[no_mangle]
pub unsafe extern "C" fn Plink_copy(mut to: *mut *mut plink, mut from: *mut plink) {
    let mut nextpl: *mut plink = 0 as *mut plink;
    while !from.is_null() {
        nextpl = (*from).next;
        (*from).next = *to;
        *to = from;
        from = nextpl;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Plink_delete(mut plp: *mut plink) {
    let mut nextpl: *mut plink = 0 as *mut plink;
    while !plp.is_null() {
        nextpl = (*plp).next;
        (*plp).next = plink_freelist;
        plink_freelist = plp;
        plp = nextpl;
    }
}
unsafe extern "C" fn file_makename(
    mut lemp: *mut lemon,
    mut suffix: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    name = malloc(
        (strlen(out_dir))
            .wrapping_add(strlen((*lemp).filename))
            .wrapping_add(strlen(suffix))
            .wrapping_add(6 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if name.is_null() {
        fprintf(
            stderr,
            b"Can't allocate space for a filename.\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    cp = strrchr((*lemp).filename, '/' as i32);
    if cp.is_null() {
        cp = (*lemp).filename;
    } else {
        cp = cp.offset(1);
    }
    strcpy(name, out_dir);
    strcat(name, b"/\0" as *const u8 as *const libc::c_char);
    strcat(name, cp);
    cp = strrchr(name, '.' as i32);
    if !cp.is_null() {
        *cp = 0 as libc::c_int as libc::c_char;
    }
    strcat(name, suffix);
    return name;
}
unsafe extern "C" fn file_open(
    mut lemp: *mut lemon,
    mut suffix: *mut libc::c_char,
    mut mode: *mut libc::c_char,
) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    if !((*lemp).outname).is_null() {
        free((*lemp).outname as *mut libc::c_void);
    }
    (*lemp).outname = file_makename(lemp, suffix);
    fp = fopen((*lemp).outname, mode);
    if fp.is_null() && *mode as libc::c_int == 'w' as i32 {
        fprintf(
            stderr,
            b"Can't open file \"%s\".\n\0" as *const u8 as *const libc::c_char,
            (*lemp).outname,
        );
        (*lemp).errorcnt += 1;
        return 0 as *mut FILE;
    }
    return fp;
}
#[no_mangle]
pub unsafe extern "C" fn Reprint(mut lemp: *mut lemon) {
    let mut rp: *mut rule = 0 as *mut rule;
    let mut sp: *mut symbol = 0 as *mut symbol;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut maxlen: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ncolumns: libc::c_int = 0;
    let mut skip: libc::c_int = 0;
    printf(
        b"// Reprint of input file \"%s\".\n// Symbols:\n\0" as *const u8
            as *const libc::c_char,
        (*lemp).filename,
    );
    maxlen = 10 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*lemp).nsymbol {
        sp = *((*lemp).symbols).offset(i as isize);
        len = strlen((*sp).name) as libc::c_int;
        if len > maxlen {
            maxlen = len;
        }
        i += 1;
    }
    ncolumns = 76 as libc::c_int / (maxlen + 5 as libc::c_int);
    if ncolumns < 1 as libc::c_int {
        ncolumns = 1 as libc::c_int;
    }
    skip = ((*lemp).nsymbol + ncolumns - 1 as libc::c_int) / ncolumns;
    i = 0 as libc::c_int;
    while i < skip {
        printf(b"//\0" as *const u8 as *const libc::c_char);
        j = i;
        while j < (*lemp).nsymbol {
            sp = *((*lemp).symbols).offset(j as isize);
            if !((*sp).index == j) {
                myassert(
                    b"src/lemon.c\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    2610 as libc::c_int,
                );
            }
            printf(
                b" %3d %-*.*s\0" as *const u8 as *const libc::c_char,
                j,
                maxlen,
                maxlen,
                (*sp).name,
            );
            j += skip;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    rp = (*lemp).rule;
    while !rp.is_null() {
        printf(b"%s\0" as *const u8 as *const libc::c_char, (*(*rp).lhs).name);
        printf(b" ::=\0" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < (*rp).nrhs {
            printf(
                b" %s\0" as *const u8 as *const libc::c_char,
                (**((*rp).rhs).offset(i as isize)).name,
            );
            i += 1;
        }
        printf(b".\0" as *const u8 as *const libc::c_char);
        if !((*rp).precsym).is_null() {
            printf(
                b" [%s]\0" as *const u8 as *const libc::c_char,
                (*(*rp).precsym).name,
            );
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        rp = (*rp).next;
    }
}
unsafe extern "C" fn ConfigPrint(mut fp: *mut FILE, mut cfp: *mut config) {
    let mut rp: *mut rule = 0 as *mut rule;
    let mut i: libc::c_int = 0;
    rp = (*cfp).rp;
    fprintf(fp, b"%s ::=\0" as *const u8 as *const libc::c_char, (*(*rp).lhs).name);
    i = 0 as libc::c_int;
    while i <= (*rp).nrhs {
        if i == (*cfp).dot {
            fprintf(fp, b" *\0" as *const u8 as *const libc::c_char);
        }
        if i == (*rp).nrhs {
            break;
        }
        fprintf(
            fp,
            b" %s\0" as *const u8 as *const libc::c_char,
            (**((*rp).rhs).offset(i as isize)).name,
        );
        i += 1;
    }
}
unsafe extern "C" fn PrintAction(
    mut ap: *mut action,
    mut fp: *mut FILE,
    mut indent: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 1 as libc::c_int;
    match (*ap).type_0 as libc::c_uint {
        0 => {
            fprintf(
                fp,
                b"%*s shift  %d\0" as *const u8 as *const libc::c_char,
                indent,
                (*(*ap).sp).name,
                (*(*ap).x.stp).index,
            );
        }
        2 => {
            fprintf(
                fp,
                b"%*s reduce %d\0" as *const u8 as *const libc::c_char,
                indent,
                (*(*ap).sp).name,
                (*(*ap).x.rp).index,
            );
        }
        1 => {
            fprintf(
                fp,
                b"%*s accept\0" as *const u8 as *const libc::c_char,
                indent,
                (*(*ap).sp).name,
            );
        }
        3 => {
            fprintf(
                fp,
                b"%*s error\0" as *const u8 as *const libc::c_char,
                indent,
                (*(*ap).sp).name,
            );
        }
        4 => {
            fprintf(
                fp,
                b"%*s reduce %-3d ** Parsing conflict **\0" as *const u8
                    as *const libc::c_char,
                indent,
                (*(*ap).sp).name,
                (*(*ap).x.rp).index,
            );
        }
        5 | 6 | 7 => {
            result = 0 as libc::c_int;
        }
        _ => {}
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ReportOutput(mut lemp: *mut lemon) {
    let mut i: libc::c_int = 0;
    let mut stp: *mut state = 0 as *mut state;
    let mut cfp: *mut config = 0 as *mut config;
    let mut ap: *mut action = 0 as *mut action;
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = file_open(
        lemp,
        b".out\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if fp.is_null() {
        return;
    }
    fprintf(fp, b" \x08\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < (*lemp).nstate {
        stp = *((*lemp).sorted).offset(i as isize);
        fprintf(fp, b"State %d:\n\0" as *const u8 as *const libc::c_char, (*stp).index);
        if (*lemp).basisflag != 0 {
            cfp = (*stp).bp;
        } else {
            cfp = (*stp).cfp;
        }
        while !cfp.is_null() {
            let mut buf: [libc::c_char; 20] = [0; 20];
            if (*cfp).dot == (*(*cfp).rp).nrhs {
                sprintf(
                    buf.as_mut_ptr(),
                    b"(%d)\0" as *const u8 as *const libc::c_char,
                    (*(*cfp).rp).index,
                );
                fprintf(
                    fp,
                    b"    %5s \0" as *const u8 as *const libc::c_char,
                    buf.as_mut_ptr(),
                );
            } else {
                fprintf(fp, b"          \0" as *const u8 as *const libc::c_char);
            }
            ConfigPrint(fp, cfp);
            fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
            if (*lemp).basisflag != 0 {
                cfp = (*cfp).bp;
            } else {
                cfp = (*cfp).next;
            }
        }
        fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
        ap = (*stp).ap;
        while !ap.is_null() {
            if PrintAction(ap, fp, 30 as libc::c_int) != 0 {
                fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
            }
            ap = (*ap).next;
        }
        fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    fclose(fp);
}
unsafe extern "C" fn pathsearch(
    mut argv0: *mut libc::c_char,
    mut name: *mut libc::c_char,
    mut modemask: libc::c_int,
) -> *mut libc::c_char {
    let mut pathlist: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    cp = strrchr(argv0, '/' as i32);
    if !cp.is_null() {
        c = *cp;
        *cp = 0 as libc::c_int as libc::c_char;
        path = malloc(
            (strlen(argv0))
                .wrapping_add(strlen(name))
                .wrapping_add(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        if !path.is_null() {
            sprintf(path, b"%s/%s\0" as *const u8 as *const libc::c_char, argv0, name);
        }
        *cp = c;
    } else {
        pathlist = getenv(b"PATH\0" as *const u8 as *const libc::c_char);
        if pathlist.is_null() {
            pathlist = b".:/bin:/usr/bin\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        path = malloc(
            (strlen(pathlist))
                .wrapping_add(strlen(name))
                .wrapping_add(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        if !path.is_null() {
            while *pathlist != 0 {
                cp = strchr(pathlist, ':' as i32);
                if cp.is_null() {
                    cp = &mut *pathlist
                        .offset(
                            (strlen
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                ) -> libc::c_ulong)(pathlist) as isize,
                        ) as *mut libc::c_char;
                }
                c = *cp;
                *cp = 0 as libc::c_int as libc::c_char;
                sprintf(
                    path,
                    b"%s/%s\0" as *const u8 as *const libc::c_char,
                    pathlist,
                    name,
                );
                *cp = c;
                if c as libc::c_int == 0 as libc::c_int {
                    pathlist = b"\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                } else {
                    pathlist = &mut *cp.offset(1 as libc::c_int as isize)
                        as *mut libc::c_char;
                }
                if access(path, modemask) == 0 as libc::c_int {
                    break;
                }
            }
        }
    }
    return path;
}
unsafe extern "C" fn compute_action(
    mut lemp: *mut lemon,
    mut ap: *mut action,
) -> libc::c_int {
    let mut act: libc::c_int = 0;
    match (*ap).type_0 as libc::c_uint {
        0 => {
            act = (*(*ap).x.stp).index;
        }
        2 => {
            act = (*(*ap).x.rp).index + (*lemp).nstate;
        }
        3 => {
            act = (*lemp).nstate + (*lemp).nrule;
        }
        1 => {
            act = (*lemp).nstate + (*lemp).nrule + 1 as libc::c_int;
        }
        _ => {
            act = -(1 as libc::c_int);
        }
    }
    return act;
}
unsafe extern "C" fn tplt_xfer(
    mut name: *mut libc::c_char,
    mut in_0: *mut FILE,
    mut out: *mut FILE,
    mut lineno: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut iStart: libc::c_int = 0;
    let mut line: [libc::c_char; 1000] = [0; 1000];
    while !(fgets(line.as_mut_ptr(), 1000 as libc::c_int, in_0)).is_null()
        && (line[0 as libc::c_int as usize] as libc::c_int != '%' as i32
            || line[1 as libc::c_int as usize] as libc::c_int != '%' as i32)
    {
        *lineno += 1;
        iStart = 0 as libc::c_int;
        if !name.is_null() {
            i = 0 as libc::c_int;
            while line[i as usize] != 0 {
                if line[i as usize] as libc::c_int == 'P' as i32
                    && strncmp(
                        &mut *line.as_mut_ptr().offset(i as isize),
                        b"Parse\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    && (i == 0 as libc::c_int
                        || *(*__ctype_b_loc())
                            .offset(
                                line[(i - 1 as libc::c_int) as usize] as libc::c_int
                                    as isize,
                            ) as libc::c_int
                            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                            == 0)
                {
                    if i > iStart {
                        fprintf(
                            out,
                            b"%.*s\0" as *const u8 as *const libc::c_char,
                            i - iStart,
                            &mut *line.as_mut_ptr().offset(iStart as isize)
                                as *mut libc::c_char,
                        );
                    }
                    fprintf(out, b"%s\0" as *const u8 as *const libc::c_char, name);
                    i += 4 as libc::c_int;
                    iStart = i + 1 as libc::c_int;
                }
                i += 1;
            }
        }
        fprintf(
            out,
            b"%s\0" as *const u8 as *const libc::c_char,
            &mut *line.as_mut_ptr().offset(iStart as isize) as *mut libc::c_char,
        );
    }
}
unsafe extern "C" fn tplt_open(mut lemp: *mut lemon) -> *mut FILE {
    let mut buf: [libc::c_char; 1000] = [0; 1000];
    let mut in_0: *mut FILE = 0 as *mut FILE;
    let mut tpltname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tpltname_alloc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    cp = strrchr((*lemp).filename, '.' as i32);
    if !cp.is_null() {
        sprintf(
            buf.as_mut_ptr(),
            b"%.*s.lt\0" as *const u8 as *const libc::c_char,
            cp.offset_from((*lemp).filename) as libc::c_long as libc::c_int,
            (*lemp).filename,
        );
    } else {
        sprintf(
            buf.as_mut_ptr(),
            b"%s.lt\0" as *const u8 as *const libc::c_char,
            (*lemp).filename,
        );
    }
    if access(buf.as_mut_ptr(), 0o4 as libc::c_int) == 0 as libc::c_int {
        tpltname = buf.as_mut_ptr();
    } else if access((*lemp).tmplname, 0o4 as libc::c_int) == 0 as libc::c_int {
        tpltname = (*lemp).tmplname;
    } else {
        tpltname_alloc = pathsearch((*lemp).argv0, (*lemp).tmplname, 0 as libc::c_int);
        tpltname = tpltname_alloc;
    }
    if tpltname.is_null() {
        fprintf(
            stderr,
            b"Can't find the parser driver template file \"%s\".\n\0" as *const u8
                as *const libc::c_char,
            (*lemp).tmplname,
        );
        (*lemp).errorcnt += 1;
        return 0 as *mut FILE;
    }
    in_0 = fopen(tpltname, b"r\0" as *const u8 as *const libc::c_char);
    if in_0.is_null() {
        fprintf(
            stderr,
            b"Can't open the template file \"%s\".\n\0" as *const u8
                as *const libc::c_char,
            tpltname,
        );
        (*lemp).errorcnt += 1;
    }
    if !tpltname_alloc.is_null() {
        free(tpltname_alloc as *mut libc::c_void);
    }
    return in_0;
}
unsafe extern "C" fn tplt_print(
    mut out: *mut FILE,
    mut lemp: *mut lemon,
    mut str: *mut libc::c_char,
    mut strln: libc::c_int,
    mut lineno: *mut libc::c_int,
) {
    if str.is_null() {
        return;
    }
    fprintf(
        out,
        b"#line %d \"%s\"\n\0" as *const u8 as *const libc::c_char,
        strln,
        (*lemp).filename,
    );
    *lineno += 1;
    while *str != 0 {
        if *str as libc::c_int == '\n' as i32 {
            *lineno += 1;
        }
        putc(*str as libc::c_int, out);
        str = str.offset(1);
    }
    fprintf(
        out,
        b"\n#line %d \"%s\"\n\0" as *const u8 as *const libc::c_char,
        *lineno + 2 as libc::c_int,
        (*lemp).outname,
    );
    *lineno += 2 as libc::c_int;
}
unsafe extern "C" fn emit_destructor_code(
    mut out: *mut FILE,
    mut sp: *mut symbol,
    mut lemp: *mut lemon,
    mut lineno: *mut libc::c_int,
) {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut linecnt: libc::c_int = 0 as libc::c_int;
    if (*sp).type_0 as libc::c_uint == TERMINAL as libc::c_int as libc::c_uint {
        cp = (*lemp).tokendest;
        if cp.is_null() {
            return;
        }
        fprintf(
            out,
            b"#line %d \"%s\"\n{\0" as *const u8 as *const libc::c_char,
            (*lemp).tokendestln,
            (*lemp).filename,
        );
    } else if !((*sp).destructor).is_null() {
        cp = (*sp).destructor;
        fprintf(
            out,
            b"#line %d \"%s\"\n{\0" as *const u8 as *const libc::c_char,
            (*sp).destructorln,
            (*lemp).filename,
        );
    } else {
        cp = (*lemp).vardest;
        if cp.is_null() {
            return;
        }
        fprintf(
            out,
            b"#line %d \"%s\"\n{\0" as *const u8 as *const libc::c_char,
            (*lemp).vardestln,
            (*lemp).filename,
        );
    }
    while *cp != 0 {
        if *cp as libc::c_int == '$' as i32
            && *cp.offset(1 as libc::c_int as isize) as libc::c_int == '$' as i32
        {
            fprintf(
                out,
                b"(yypminor->yy%d)\0" as *const u8 as *const libc::c_char,
                (*sp).dtnum,
            );
            cp = cp.offset(1);
        } else {
            if *cp as libc::c_int == '\n' as i32 {
                linecnt += 1;
            }
            fputc(*cp as libc::c_int, out);
        }
        cp = cp.offset(1);
    }
    *lineno += 3 as libc::c_int + linecnt;
    fprintf(
        out,
        b"}\n#line %d \"%s\"\n\0" as *const u8 as *const libc::c_char,
        *lineno,
        (*lemp).outname,
    );
}
unsafe extern "C" fn has_destructor(
    mut sp: *mut symbol,
    mut lemp: *mut lemon,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if (*sp).type_0 as libc::c_uint == TERMINAL as libc::c_int as libc::c_uint {
        ret = ((*lemp).tokendest != 0 as *mut libc::c_char) as libc::c_int;
    } else {
        ret = (!((*lemp).vardest).is_null() || !((*sp).destructor).is_null())
            as libc::c_int;
    }
    return ret;
}
unsafe extern "C" fn emit_code(
    mut out: *mut FILE,
    mut rp: *mut rule,
    mut lemp: *mut lemon,
    mut lineno: *mut libc::c_int,
) {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut xp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut linecnt: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut lhsused: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut used: [libc::c_char; 1000] = [0; 1000];
    i = 0 as libc::c_int;
    while i < (*rp).nrhs {
        used[i as usize] = 0 as libc::c_int as libc::c_char;
        i += 1;
    }
    lhsused = 0 as libc::c_int as libc::c_char;
    if !((*rp).code).is_null() {
        fprintf(
            out,
            b"#line %d \"%s\"\n{\0" as *const u8 as *const libc::c_char,
            (*rp).line,
            (*lemp).filename,
        );
        cp = (*rp).code;
        while *cp != 0 {
            if *(*__ctype_b_loc()).offset(*cp as libc::c_int as isize) as libc::c_int
                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
                && (cp == (*rp).code
                    || *(*__ctype_b_loc())
                        .offset(
                            *cp.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                as isize,
                        ) as libc::c_int
                        & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int == 0
                        && *cp.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            != '_' as i32)
            {
                let mut saved: libc::c_char = 0;
                xp = &mut *cp.offset(1 as libc::c_int as isize) as *mut libc::c_char;
                while *(*__ctype_b_loc()).offset(*xp as libc::c_int as isize)
                    as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                    || *xp as libc::c_int == '_' as i32
                {
                    xp = xp.offset(1);
                }
                saved = *xp;
                *xp = 0 as libc::c_int as libc::c_char;
                if !((*rp).lhsalias).is_null()
                    && strcmp(cp, (*rp).lhsalias) == 0 as libc::c_int
                {
                    fprintf(
                        out,
                        b"yygotominor.yy%d\0" as *const u8 as *const libc::c_char,
                        (*(*rp).lhs).dtnum,
                    );
                    cp = xp;
                    lhsused = 1 as libc::c_int as libc::c_char;
                } else {
                    i = 0 as libc::c_int;
                    while i < (*rp).nrhs {
                        if !(*((*rp).rhsalias).offset(i as isize)).is_null()
                            && strcmp(cp, *((*rp).rhsalias).offset(i as isize))
                                == 0 as libc::c_int
                        {
                            fprintf(
                                out,
                                b"yymsp[%d].minor.yy%d\0" as *const u8
                                    as *const libc::c_char,
                                i - (*rp).nrhs + 1 as libc::c_int,
                                (**((*rp).rhs).offset(i as isize)).dtnum,
                            );
                            cp = xp;
                            used[i as usize] = 1 as libc::c_int as libc::c_char;
                            break;
                        } else {
                            i += 1;
                        }
                    }
                }
                *xp = saved;
            }
            if *cp as libc::c_int == '\n' as i32 {
                linecnt += 1;
            }
            fputc(*cp as libc::c_int, out);
            cp = cp.offset(1);
        }
        *lineno += 3 as libc::c_int + linecnt;
        fprintf(
            out,
            b"}\n#line %d \"%s\"\n\0" as *const u8 as *const libc::c_char,
            *lineno,
            (*lemp).outname,
        );
    }
    if !((*rp).lhsalias).is_null() && lhsused == 0 {
        ErrorMsg(
            (*lemp).filename,
            (*rp).ruleline,
            b"Label \"%s\" for \"%s(%s)\" is never used.\0" as *const u8
                as *const libc::c_char,
            (*rp).lhsalias,
            (*(*rp).lhs).name,
            (*rp).lhsalias,
        );
        (*lemp).errorcnt += 1;
    }
    i = 0 as libc::c_int;
    while i < (*rp).nrhs {
        if !(*((*rp).rhsalias).offset(i as isize)).is_null() && used[i as usize] == 0 {
            ErrorMsg(
                (*lemp).filename,
                (*rp).ruleline,
                b"Label %s for \"%s(%s)\" is never used.\0" as *const u8
                    as *const libc::c_char,
                *((*rp).rhsalias).offset(i as isize),
                (**((*rp).rhs).offset(i as isize)).name,
                *((*rp).rhsalias).offset(i as isize),
            );
            (*lemp).errorcnt += 1;
        } else if (*((*rp).rhsalias).offset(i as isize)).is_null() {
            if has_destructor(*((*rp).rhs).offset(i as isize), lemp) != 0 {
                fprintf(
                    out,
                    b"  yy_destructor(%d,&yymsp[%d].minor);\n\0" as *const u8
                        as *const libc::c_char,
                    (**((*rp).rhs).offset(i as isize)).index,
                    i - (*rp).nrhs + 1 as libc::c_int,
                );
                *lineno += 1;
            } else {
                fprintf(
                    out,
                    b"        /* No destructor defined for %s */\n\0" as *const u8
                        as *const libc::c_char,
                    (**((*rp).rhs).offset(i as isize)).name,
                );
                *lineno += 1;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn print_stack_union(
    mut out: *mut FILE,
    mut lemp: *mut lemon,
    mut plineno: *mut libc::c_int,
    mut mhflag: libc::c_int,
) {
    let mut lineno: libc::c_int = 0;
    let mut types: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut arraysize: libc::c_int = 0;
    let mut maxdtlength: libc::c_int = 0;
    let mut stddt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut hash: libc::c_int = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    arraysize = (*lemp).nsymbol * 2 as libc::c_int;
    types = malloc(
        (arraysize as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < arraysize {
        let ref mut fresh17 = *types.offset(i as isize);
        *fresh17 = 0 as *mut libc::c_char;
        i += 1;
    }
    maxdtlength = 0 as libc::c_int;
    if !((*lemp).vartype).is_null() {
        maxdtlength = strlen((*lemp).vartype) as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (*lemp).nsymbol {
        let mut len: libc::c_int = 0;
        let mut sp: *mut symbol = *((*lemp).symbols).offset(i as isize);
        if !((*sp).datatype).is_null() {
            len = strlen((*sp).datatype) as libc::c_int;
            if len > maxdtlength {
                maxdtlength = len;
            }
        }
        i += 1;
    }
    stddt = malloc((maxdtlength * 2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
        as *mut libc::c_char;
    if types.is_null() || stddt.is_null() {
        fprintf(stderr, b"Out of memory.\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < (*lemp).nsymbol {
        let mut sp_0: *mut symbol = *((*lemp).symbols).offset(i as isize);
        let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
        if sp_0 == (*lemp).errsym {
            (*sp_0).dtnum = arraysize + 1 as libc::c_int;
        } else if (*sp_0).type_0 as libc::c_uint
                != NONTERMINAL as libc::c_int as libc::c_uint
                || ((*sp_0).datatype).is_null() && ((*lemp).vartype).is_null()
            {
            (*sp_0).dtnum = 0 as libc::c_int;
        } else {
            cp = (*sp_0).datatype;
            if cp.is_null() {
                cp = (*lemp).vartype;
            }
            j = 0 as libc::c_int;
            while *(*__ctype_b_loc()).offset(*cp as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                cp = cp.offset(1);
            }
            while *cp != 0 {
                let fresh18 = cp;
                cp = cp.offset(1);
                let fresh19 = j;
                j = j + 1;
                *stddt.offset(fresh19 as isize) = *fresh18;
            }
            while j > 0 as libc::c_int
                && *(*__ctype_b_loc())
                    .offset(
                        *stddt.offset((j - 1 as libc::c_int) as isize) as libc::c_int
                            as isize,
                    ) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                j -= 1;
            }
            *stddt.offset(j as isize) = 0 as libc::c_int as libc::c_char;
            hash = 0 as libc::c_int;
            j = 0 as libc::c_int;
            while *stddt.offset(j as isize) != 0 {
                hash = (hash as libc::c_uint)
                    .wrapping_mul(53 as libc::c_uint)
                    .wrapping_add(*stddt.offset(j as isize) as libc::c_uint)
                    as libc::c_int;
                j += 1;
            }
            hash = (hash & 0x7fffffff as libc::c_int) % arraysize;
            while !(*types.offset(hash as isize)).is_null() {
                if strcmp(*types.offset(hash as isize), stddt) == 0 as libc::c_int {
                    (*sp_0).dtnum = hash + 1 as libc::c_int;
                    break;
                } else {
                    hash += 1;
                    if hash >= arraysize {
                        hash = 0 as libc::c_int;
                    }
                }
            }
            if (*types.offset(hash as isize)).is_null() {
                (*sp_0).dtnum = hash + 1 as libc::c_int;
                let ref mut fresh20 = *types.offset(hash as isize);
                *fresh20 = malloc(
                    (strlen(stddt)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                if (*types.offset(hash as isize)).is_null() {
                    fprintf(
                        stderr,
                        b"Out of memory.\n\0" as *const u8 as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                strcpy(*types.offset(hash as isize), stddt);
            }
        }
        i += 1;
    }
    name = (if !((*lemp).name).is_null() {
        (*lemp).name as *const libc::c_char
    } else {
        b"Parse\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
    lineno = *plineno;
    if mhflag != 0 {
        fprintf(out, b"#if INTERFACE\n\0" as *const u8 as *const libc::c_char);
        lineno += 1;
    }
    fprintf(
        out,
        b"#define %sTOKENTYPE %s\n\0" as *const u8 as *const libc::c_char,
        name,
        if !((*lemp).tokentype).is_null() {
            (*lemp).tokentype as *const libc::c_char
        } else {
            b"void*\0" as *const u8 as *const libc::c_char
        },
    );
    lineno += 1;
    if mhflag != 0 {
        fprintf(out, b"#endif\n\0" as *const u8 as *const libc::c_char);
        lineno += 1;
    }
    fprintf(out, b"typedef union {\n\0" as *const u8 as *const libc::c_char);
    lineno += 1;
    fprintf(out, b"  %sTOKENTYPE yy0;\n\0" as *const u8 as *const libc::c_char, name);
    lineno += 1;
    i = 0 as libc::c_int;
    while i < arraysize {
        if !(*types.offset(i as isize)).is_null() {
            fprintf(
                out,
                b"  %s yy%d;\n\0" as *const u8 as *const libc::c_char,
                *types.offset(i as isize),
                i + 1 as libc::c_int,
            );
            lineno += 1;
            free(*types.offset(i as isize) as *mut libc::c_void);
        }
        i += 1;
    }
    fprintf(
        out,
        b"  int yy%d;\n\0" as *const u8 as *const libc::c_char,
        (*(*lemp).errsym).dtnum,
    );
    lineno += 1;
    free(stddt as *mut libc::c_void);
    free(types as *mut libc::c_void);
    fprintf(out, b"} YYMINORTYPE;\n\0" as *const u8 as *const libc::c_char);
    lineno += 1;
    *plineno = lineno;
}
unsafe extern "C" fn minimum_size_type(
    mut lwr: libc::c_int,
    mut upr: libc::c_int,
) -> *const libc::c_char {
    if lwr >= 0 as libc::c_int {
        if upr <= 255 as libc::c_int {
            return b"unsigned char\0" as *const u8 as *const libc::c_char
        } else if upr < 65535 as libc::c_int {
            return b"unsigned short int\0" as *const u8 as *const libc::c_char
        } else {
            return b"unsigned int\0" as *const u8 as *const libc::c_char
        }
    } else if lwr >= -(127 as libc::c_int) && upr <= 127 as libc::c_int {
        return b"signed char\0" as *const u8 as *const libc::c_char
    } else if lwr >= -(32767 as libc::c_int) && upr < 32767 as libc::c_int {
        return b"short\0" as *const u8 as *const libc::c_char
    } else {
        return b"int\0" as *const u8 as *const libc::c_char
    };
}
unsafe extern "C" fn axset_compare(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut p1: *mut axset = a as *mut axset;
    let mut p2: *mut axset = b as *mut axset;
    return (*p2).nAction - (*p1).nAction;
}
#[no_mangle]
pub unsafe extern "C" fn ReportTable(mut lemp: *mut lemon, mut mhflag: libc::c_int) {
    let mut out: *mut FILE = 0 as *mut FILE;
    let mut in_0: *mut FILE = 0 as *mut FILE;
    let mut line: [libc::c_char; 1000] = [0; 1000];
    let mut lineno: libc::c_int = 0;
    let mut stp: *mut state = 0 as *mut state;
    let mut ap: *mut action = 0 as *mut action;
    let mut rp: *mut rule = 0 as *mut rule;
    let mut pActtab: *mut acttab = 0 as *mut acttab;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut mnTknOfst: libc::c_int = 0;
    let mut mxTknOfst: libc::c_int = 0;
    let mut mnNtOfst: libc::c_int = 0;
    let mut mxNtOfst: libc::c_int = 0;
    let mut ax: *mut axset = 0 as *mut axset;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    in_0 = tplt_open(lemp);
    if in_0.is_null() {
        return;
    }
    out = file_open(
        lemp,
        b".c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if out.is_null() {
        fclose(in_0);
        return;
    }
    lineno = 1 as libc::c_int;
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    tplt_print(out, lemp, (*lemp).include, (*lemp).includeln, &mut lineno);
    if mhflag != 0 {
        name = file_makename(
            lemp,
            b".h\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        fprintf(out, b"#include \"%s\"\n\0" as *const u8 as *const libc::c_char, name);
        lineno += 1;
        free(name as *mut libc::c_void);
    }
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    if mhflag != 0 {
        let mut prefix: *mut libc::c_char = 0 as *mut libc::c_char;
        fprintf(out, b"#if INTERFACE\n\0" as *const u8 as *const libc::c_char);
        lineno += 1;
        if !((*lemp).tokenprefix).is_null() {
            prefix = (*lemp).tokenprefix;
        } else {
            prefix = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        i = 1 as libc::c_int;
        while i < (*lemp).nterminal {
            fprintf(
                out,
                b"#define %s%-30s %2d\n\0" as *const u8 as *const libc::c_char,
                prefix,
                (**((*lemp).symbols).offset(i as isize)).name,
                i,
            );
            lineno += 1;
            i += 1;
        }
        fprintf(out, b"#endif\n\0" as *const u8 as *const libc::c_char);
        lineno += 1;
    }
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    fprintf(out, b"/* \x01 */\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        out,
        b"#define YYCODETYPE %s\n\0" as *const u8 as *const libc::c_char,
        minimum_size_type(0 as libc::c_int, (*lemp).nsymbol + 5 as libc::c_int),
    );
    lineno += 1;
    fprintf(
        out,
        b"#define YYNOCODE %d\n\0" as *const u8 as *const libc::c_char,
        (*lemp).nsymbol + 1 as libc::c_int,
    );
    lineno += 1;
    fprintf(
        out,
        b"#define YYACTIONTYPE %s\n\0" as *const u8 as *const libc::c_char,
        minimum_size_type(
            0 as libc::c_int,
            (*lemp).nstate + (*lemp).nrule + 5 as libc::c_int,
        ),
    );
    lineno += 1;
    print_stack_union(out, lemp, &mut lineno, mhflag);
    if !((*lemp).stacksize).is_null() {
        if atoi((*lemp).stacksize) <= 0 as libc::c_int {
            ErrorMsg(
                (*lemp).filename,
                0 as libc::c_int,
                b"Illegal stack size: [%s].  The stack size should be an integer constant.\0"
                    as *const u8 as *const libc::c_char,
                (*lemp).stacksize,
            );
            (*lemp).errorcnt += 1;
            (*lemp)
                .stacksize = b"100\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        fprintf(
            out,
            b"#define YYSTACKDEPTH %s\n\0" as *const u8 as *const libc::c_char,
            (*lemp).stacksize,
        );
        lineno += 1;
    } else {
        fprintf(
            out,
            b"#define YYSTACKDEPTH 100\n\0" as *const u8 as *const libc::c_char,
        );
        lineno += 1;
    }
    if mhflag != 0 {
        fprintf(out, b"#if INTERFACE\n\0" as *const u8 as *const libc::c_char);
        lineno += 1;
    }
    name = (if !((*lemp).name).is_null() {
        (*lemp).name as *const libc::c_char
    } else {
        b"Parse\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
    if !((*lemp).arg).is_null()
        && *((*lemp).arg).offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        i = strlen((*lemp).arg) as libc::c_int;
        while i >= 1 as libc::c_int
            && *(*__ctype_b_loc())
                .offset(
                    *((*lemp).arg).offset((i - 1 as libc::c_int) as isize) as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            i -= 1;
        }
        while i >= 1 as libc::c_int
            && (*(*__ctype_b_loc())
                .offset(
                    *((*lemp).arg).offset((i - 1 as libc::c_int) as isize) as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                || *((*lemp).arg).offset((i - 1 as libc::c_int) as isize) as libc::c_int
                    == '_' as i32)
        {
            i -= 1;
        }
        fprintf(
            out,
            b"#define %sARG_SDECL %s;\n\0" as *const u8 as *const libc::c_char,
            name,
            (*lemp).arg,
        );
        lineno += 1;
        fprintf(
            out,
            b"#define %sARG_PDECL ,%s\n\0" as *const u8 as *const libc::c_char,
            name,
            (*lemp).arg,
        );
        lineno += 1;
        fprintf(
            out,
            b"#define %sARG_FETCH %s = yypParser->%s\n\0" as *const u8
                as *const libc::c_char,
            name,
            (*lemp).arg,
            &mut *((*lemp).arg).offset(i as isize) as *mut libc::c_char,
        );
        lineno += 1;
        fprintf(
            out,
            b"#define %sARG_STORE yypParser->%s = %s\n\0" as *const u8
                as *const libc::c_char,
            name,
            &mut *((*lemp).arg).offset(i as isize) as *mut libc::c_char,
            &mut *((*lemp).arg).offset(i as isize) as *mut libc::c_char,
        );
        lineno += 1;
    } else {
        fprintf(
            out,
            b"#define %sARG_SDECL\n\0" as *const u8 as *const libc::c_char,
            name,
        );
        lineno += 1;
        fprintf(
            out,
            b"#define %sARG_PDECL\n\0" as *const u8 as *const libc::c_char,
            name,
        );
        lineno += 1;
        fprintf(
            out,
            b"#define %sARG_FETCH\n\0" as *const u8 as *const libc::c_char,
            name,
        );
        lineno += 1;
        fprintf(
            out,
            b"#define %sARG_STORE\n\0" as *const u8 as *const libc::c_char,
            name,
        );
        lineno += 1;
    }
    if mhflag != 0 {
        fprintf(out, b"#endif\n\0" as *const u8 as *const libc::c_char);
        lineno += 1;
    }
    fprintf(
        out,
        b"#define YYNSTATE %d\n\0" as *const u8 as *const libc::c_char,
        (*lemp).nstate,
    );
    lineno += 1;
    fprintf(
        out,
        b"#define YYNRULE %d\n\0" as *const u8 as *const libc::c_char,
        (*lemp).nrule,
    );
    lineno += 1;
    fprintf(
        out,
        b"#define YYERRORSYMBOL %d\n\0" as *const u8 as *const libc::c_char,
        (*(*lemp).errsym).index,
    );
    lineno += 1;
    fprintf(
        out,
        b"#define YYERRSYMDT yy%d\n\0" as *const u8 as *const libc::c_char,
        (*(*lemp).errsym).dtnum,
    );
    lineno += 1;
    if (*lemp).has_fallback != 0 {
        fprintf(out, b"#define YYFALLBACK 1\n\0" as *const u8 as *const libc::c_char);
        lineno += 1;
    }
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    ax = malloc(
        (::std::mem::size_of::<axset>() as libc::c_ulong)
            .wrapping_mul((*lemp).nstate as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong),
    ) as *mut axset;
    if ax.is_null() {
        fprintf(stderr, b"malloc failed\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < (*lemp).nstate {
        stp = *((*lemp).sorted).offset(i as isize);
        (*stp).nNtAct = 0 as libc::c_int;
        (*stp).nTknAct = (*stp).nNtAct;
        (*stp).iDflt = (*lemp).nstate + (*lemp).nrule;
        (*stp).iTknOfst = -(2147483647 as libc::c_int);
        (*stp).iNtOfst = -(2147483647 as libc::c_int);
        ap = (*stp).ap;
        while !ap.is_null() {
            if compute_action(lemp, ap) >= 0 as libc::c_int {
                if (*(*ap).sp).index < (*lemp).nterminal {
                    (*stp).nTknAct += 1;
                } else if (*(*ap).sp).index < (*lemp).nsymbol {
                    (*stp).nNtAct += 1;
                } else {
                    (*stp).iDflt = compute_action(lemp, ap);
                }
            }
            ap = (*ap).next;
        }
        let ref mut fresh21 = (*ax.offset((i * 2 as libc::c_int) as isize)).stp;
        *fresh21 = stp;
        (*ax.offset((i * 2 as libc::c_int) as isize)).isTkn = 1 as libc::c_int;
        (*ax.offset((i * 2 as libc::c_int) as isize)).nAction = (*stp).nTknAct;
        let ref mut fresh22 = (*ax
            .offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize))
            .stp;
        *fresh22 = stp;
        (*ax.offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize))
            .isTkn = 0 as libc::c_int;
        (*ax.offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize))
            .nAction = (*stp).nNtAct;
        i += 1;
    }
    mnTknOfst = 0 as libc::c_int;
    mxTknOfst = mnTknOfst;
    mnNtOfst = 0 as libc::c_int;
    mxNtOfst = mnNtOfst;
    qsort(
        ax as *mut libc::c_void,
        ((*lemp).nstate * 2 as libc::c_int) as size_t,
        ::std::mem::size_of::<axset>() as libc::c_ulong,
        Some(
            axset_compare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    pActtab = acttab_alloc();
    i = 0 as libc::c_int;
    while i < (*lemp).nstate * 2 as libc::c_int
        && (*ax.offset(i as isize)).nAction > 0 as libc::c_int
    {
        stp = (*ax.offset(i as isize)).stp;
        if (*ax.offset(i as isize)).isTkn != 0 {
            ap = (*stp).ap;
            while !ap.is_null() {
                let mut action: libc::c_int = 0;
                if !((*(*ap).sp).index >= (*lemp).nterminal) {
                    action = compute_action(lemp, ap);
                    if !(action < 0 as libc::c_int) {
                        acttab_action(pActtab, (*(*ap).sp).index, action);
                    }
                }
                ap = (*ap).next;
            }
            (*stp).iTknOfst = acttab_insert(pActtab);
            if (*stp).iTknOfst < mnTknOfst {
                mnTknOfst = (*stp).iTknOfst;
            }
            if (*stp).iTknOfst > mxTknOfst {
                mxTknOfst = (*stp).iTknOfst;
            }
        } else {
            ap = (*stp).ap;
            while !ap.is_null() {
                let mut action_0: libc::c_int = 0;
                if !((*(*ap).sp).index < (*lemp).nterminal) {
                    if !((*(*ap).sp).index == (*lemp).nsymbol) {
                        action_0 = compute_action(lemp, ap);
                        if !(action_0 < 0 as libc::c_int) {
                            acttab_action(pActtab, (*(*ap).sp).index, action_0);
                        }
                    }
                }
                ap = (*ap).next;
            }
            (*stp).iNtOfst = acttab_insert(pActtab);
            if (*stp).iNtOfst < mnNtOfst {
                mnNtOfst = (*stp).iNtOfst;
            }
            if (*stp).iNtOfst > mxNtOfst {
                mxNtOfst = (*stp).iNtOfst;
            }
        }
        i += 1;
    }
    free(ax as *mut libc::c_void);
    fprintf(
        out,
        b"static YYACTIONTYPE yy_action[] = {\n\0" as *const u8 as *const libc::c_char,
    );
    lineno += 1;
    n = (*pActtab).nAction;
    j = 0 as libc::c_int;
    i = j;
    while i < n {
        let mut action_1: libc::c_int = (*((*pActtab).aAction).offset(i as isize))
            .action;
        if action_1 < 0 as libc::c_int {
            action_1 = (*lemp).nsymbol + (*lemp).nrule + 2 as libc::c_int;
        }
        if j == 0 as libc::c_int {
            fprintf(out, b" /* %5d */ \0" as *const u8 as *const libc::c_char, i);
        }
        fprintf(out, b" %4d,\0" as *const u8 as *const libc::c_char, action_1);
        if j == 9 as libc::c_int || i == n - 1 as libc::c_int {
            fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
            lineno += 1;
            j = 0 as libc::c_int;
        } else {
            j += 1;
        }
        i += 1;
    }
    fprintf(out, b"};\n\0" as *const u8 as *const libc::c_char);
    lineno += 1;
    fprintf(
        out,
        b"static YYCODETYPE yy_lookahead[] = {\n\0" as *const u8 as *const libc::c_char,
    );
    lineno += 1;
    j = 0 as libc::c_int;
    i = j;
    while i < n {
        let mut la: libc::c_int = (*((*pActtab).aAction).offset(i as isize)).lookahead;
        if la < 0 as libc::c_int {
            la = (*lemp).nsymbol;
        }
        if j == 0 as libc::c_int {
            fprintf(out, b" /* %5d */ \0" as *const u8 as *const libc::c_char, i);
        }
        fprintf(out, b" %4d,\0" as *const u8 as *const libc::c_char, la);
        if j == 9 as libc::c_int || i == n - 1 as libc::c_int {
            fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
            lineno += 1;
            j = 0 as libc::c_int;
        } else {
            j += 1;
        }
        i += 1;
    }
    fprintf(out, b"};\n\0" as *const u8 as *const libc::c_char);
    lineno += 1;
    fprintf(
        out,
        b"#define YY_SHIFT_USE_DFLT (%d)\n\0" as *const u8 as *const libc::c_char,
        mnTknOfst - 1 as libc::c_int,
    );
    lineno += 1;
    fprintf(
        out,
        b"static %s yy_shift_ofst[] = {\n\0" as *const u8 as *const libc::c_char,
        minimum_size_type(mnTknOfst - 1 as libc::c_int, mxTknOfst),
    );
    lineno += 1;
    n = (*lemp).nstate;
    j = 0 as libc::c_int;
    i = j;
    while i < n {
        let mut ofst: libc::c_int = 0;
        stp = *((*lemp).sorted).offset(i as isize);
        ofst = (*stp).iTknOfst;
        if ofst == -(2147483647 as libc::c_int) {
            ofst = mnTknOfst - 1 as libc::c_int;
        }
        if j == 0 as libc::c_int {
            fprintf(out, b" /* %5d */ \0" as *const u8 as *const libc::c_char, i);
        }
        fprintf(out, b" %4d,\0" as *const u8 as *const libc::c_char, ofst);
        if j == 9 as libc::c_int || i == n - 1 as libc::c_int {
            fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
            lineno += 1;
            j = 0 as libc::c_int;
        } else {
            j += 1;
        }
        i += 1;
    }
    fprintf(out, b"};\n\0" as *const u8 as *const libc::c_char);
    lineno += 1;
    fprintf(
        out,
        b"#define YY_REDUCE_USE_DFLT (%d)\n\0" as *const u8 as *const libc::c_char,
        mnNtOfst - 1 as libc::c_int,
    );
    lineno += 1;
    fprintf(
        out,
        b"static %s yy_reduce_ofst[] = {\n\0" as *const u8 as *const libc::c_char,
        minimum_size_type(mnNtOfst - 1 as libc::c_int, mxNtOfst),
    );
    lineno += 1;
    n = (*lemp).nstate;
    j = 0 as libc::c_int;
    i = j;
    while i < n {
        let mut ofst_0: libc::c_int = 0;
        stp = *((*lemp).sorted).offset(i as isize);
        ofst_0 = (*stp).iNtOfst;
        if ofst_0 == -(2147483647 as libc::c_int) {
            ofst_0 = mnNtOfst - 1 as libc::c_int;
        }
        if j == 0 as libc::c_int {
            fprintf(out, b" /* %5d */ \0" as *const u8 as *const libc::c_char, i);
        }
        fprintf(out, b" %4d,\0" as *const u8 as *const libc::c_char, ofst_0);
        if j == 9 as libc::c_int || i == n - 1 as libc::c_int {
            fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
            lineno += 1;
            j = 0 as libc::c_int;
        } else {
            j += 1;
        }
        i += 1;
    }
    fprintf(out, b"};\n\0" as *const u8 as *const libc::c_char);
    lineno += 1;
    fprintf(
        out,
        b"static YYACTIONTYPE yy_default[] = {\n\0" as *const u8 as *const libc::c_char,
    );
    lineno += 1;
    n = (*lemp).nstate;
    j = 0 as libc::c_int;
    i = j;
    while i < n {
        stp = *((*lemp).sorted).offset(i as isize);
        if j == 0 as libc::c_int {
            fprintf(out, b" /* %5d */ \0" as *const u8 as *const libc::c_char, i);
        }
        fprintf(out, b" %4d,\0" as *const u8 as *const libc::c_char, (*stp).iDflt);
        if j == 9 as libc::c_int || i == n - 1 as libc::c_int {
            fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
            lineno += 1;
            j = 0 as libc::c_int;
        } else {
            j += 1;
        }
        i += 1;
    }
    fprintf(out, b"};\n\0" as *const u8 as *const libc::c_char);
    lineno += 1;
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    if (*lemp).has_fallback != 0 {
        i = 0 as libc::c_int;
        while i < (*lemp).nterminal {
            let mut p: *mut symbol = *((*lemp).symbols).offset(i as isize);
            if ((*p).fallback).is_null() {
                fprintf(
                    out,
                    b"    0,  /* %10s => nothing */\n\0" as *const u8
                        as *const libc::c_char,
                    (*p).name,
                );
            } else {
                fprintf(
                    out,
                    b"  %3d,  /* %10s => %s */\n\0" as *const u8 as *const libc::c_char,
                    (*(*p).fallback).index,
                    (*p).name,
                    (*(*p).fallback).name,
                );
            }
            lineno += 1;
            i += 1;
        }
    }
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    i = 0 as libc::c_int;
    while i < (*lemp).nsymbol {
        sprintf(
            line.as_mut_ptr(),
            b"\"%s\",\0" as *const u8 as *const libc::c_char,
            (**((*lemp).symbols).offset(i as isize)).name,
        );
        fprintf(
            out,
            b"  %-15s\0" as *const u8 as *const libc::c_char,
            line.as_mut_ptr(),
        );
        if i & 3 as libc::c_int == 3 as libc::c_int {
            fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
            lineno += 1;
        }
        i += 1;
    }
    if i & 3 as libc::c_int != 0 as libc::c_int {
        fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
        lineno += 1;
    }
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    i = 0 as libc::c_int;
    rp = (*lemp).rule;
    while !rp.is_null() {
        if !((*rp).index == i) {
            myassert(
                b"src/lemon.c\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                3513 as libc::c_int,
            );
        }
        fprintf(
            out,
            b" /* %3d */ \"%s ::=\0" as *const u8 as *const libc::c_char,
            i,
            (*(*rp).lhs).name,
        );
        j = 0 as libc::c_int;
        while j < (*rp).nrhs {
            fprintf(
                out,
                b" %s\0" as *const u8 as *const libc::c_char,
                (**((*rp).rhs).offset(j as isize)).name,
            );
            j += 1;
        }
        fprintf(out, b"\",\n\0" as *const u8 as *const libc::c_char);
        lineno += 1;
        rp = (*rp).next;
        i += 1;
    }
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    if !((*lemp).tokendest).is_null() {
        i = 0 as libc::c_int;
        while i < (*lemp).nsymbol {
            let mut sp: *mut symbol = *((*lemp).symbols).offset(i as isize);
            if !(sp.is_null()
                || (*sp).type_0 as libc::c_uint
                    != TERMINAL as libc::c_int as libc::c_uint)
            {
                fprintf(
                    out,
                    b"    case %d:\n\0" as *const u8 as *const libc::c_char,
                    (*sp).index,
                );
                lineno += 1;
            }
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < (*lemp).nsymbol
            && (**((*lemp).symbols).offset(i as isize)).type_0 as libc::c_uint
                != TERMINAL as libc::c_int as libc::c_uint
        {
            i += 1;
        }
        if i < (*lemp).nsymbol {
            emit_destructor_code(
                out,
                *((*lemp).symbols).offset(i as isize),
                lemp,
                &mut lineno,
            );
            fprintf(out, b"      break;\n\0" as *const u8 as *const libc::c_char);
            lineno += 1;
        }
    }
    i = 0 as libc::c_int;
    while i < (*lemp).nsymbol {
        let mut sp_0: *mut symbol = *((*lemp).symbols).offset(i as isize);
        if !(sp_0.is_null()
            || (*sp_0).type_0 as libc::c_uint == TERMINAL as libc::c_int as libc::c_uint
            || ((*sp_0).destructor).is_null())
        {
            fprintf(
                out,
                b"    case %d:\n\0" as *const u8 as *const libc::c_char,
                (*sp_0).index,
            );
            lineno += 1;
            emit_destructor_code(
                out,
                *((*lemp).symbols).offset(i as isize),
                lemp,
                &mut lineno,
            );
            fprintf(out, b"      break;\n\0" as *const u8 as *const libc::c_char);
            lineno += 1;
        }
        i += 1;
    }
    if !((*lemp).vardest).is_null() {
        let mut dflt_sp: *mut symbol = 0 as *mut symbol;
        i = 0 as libc::c_int;
        while i < (*lemp).nsymbol {
            let mut sp_1: *mut symbol = *((*lemp).symbols).offset(i as isize);
            if !(sp_1.is_null()
                || (*sp_1).type_0 as libc::c_uint
                    == TERMINAL as libc::c_int as libc::c_uint
                || (*sp_1).index <= 0 as libc::c_int || !((*sp_1).destructor).is_null())
            {
                fprintf(
                    out,
                    b"    case %d:\n\0" as *const u8 as *const libc::c_char,
                    (*sp_1).index,
                );
                lineno += 1;
                dflt_sp = sp_1;
            }
            i += 1;
        }
        if !dflt_sp.is_null() {
            emit_destructor_code(out, dflt_sp, lemp, &mut lineno);
            fprintf(out, b"      break;\n\0" as *const u8 as *const libc::c_char);
            lineno += 1;
        }
    }
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    tplt_print(out, lemp, (*lemp).overflow, (*lemp).overflowln, &mut lineno);
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    rp = (*lemp).rule;
    while !rp.is_null() {
        fprintf(
            out,
            b"  { %d, %d },\n\0" as *const u8 as *const libc::c_char,
            (*(*rp).lhs).index,
            (*rp).nrhs,
        );
        lineno += 1;
        rp = (*rp).next;
    }
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    rp = (*lemp).rule;
    while !rp.is_null() {
        fprintf(
            out,
            b"      case %d:\n\0" as *const u8 as *const libc::c_char,
            (*rp).index,
        );
        lineno += 1;
        emit_code(out, rp, lemp, &mut lineno);
        fprintf(out, b"        break;\n\0" as *const u8 as *const libc::c_char);
        lineno += 1;
        rp = (*rp).next;
    }
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    tplt_print(out, lemp, (*lemp).failure, (*lemp).failureln, &mut lineno);
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    tplt_print(out, lemp, (*lemp).error, (*lemp).errorln, &mut lineno);
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    tplt_print(out, lemp, (*lemp).accept, (*lemp).acceptln, &mut lineno);
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    tplt_print(out, lemp, (*lemp).extracode, (*lemp).extracodeln, &mut lineno);
    acttab_free(pActtab);
    fclose(in_0);
    fclose(out);
}
#[no_mangle]
pub unsafe extern "C" fn ReportHeader(mut lemp: *mut lemon) {
    let mut out: *mut FILE = 0 as *mut FILE;
    let mut in_0: *mut FILE = 0 as *mut FILE;
    let mut prefix: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: [libc::c_char; 1000] = [0; 1000];
    let mut pattern: [libc::c_char; 1000] = [0; 1000];
    let mut i: libc::c_int = 0;
    if !((*lemp).tokenprefix).is_null() {
        prefix = (*lemp).tokenprefix;
    } else {
        prefix = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    in_0 = file_open(
        lemp,
        b".h\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !in_0.is_null() {
        i = 1 as libc::c_int;
        while i < (*lemp).nterminal
            && !(fgets(line.as_mut_ptr(), 1000 as libc::c_int, in_0)).is_null()
        {
            sprintf(
                pattern.as_mut_ptr(),
                b"#define %s%-30s %2d\n\0" as *const u8 as *const libc::c_char,
                prefix,
                (**((*lemp).symbols).offset(i as isize)).name,
                i,
            );
            if strcmp(line.as_mut_ptr(), pattern.as_mut_ptr()) != 0 {
                break;
            }
            i += 1;
        }
        fclose(in_0);
        if i == (*lemp).nterminal {
            return;
        }
    }
    out = file_open(
        lemp,
        b".h\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !out.is_null() {
        i = 1 as libc::c_int;
        while i < (*lemp).nterminal {
            fprintf(
                out,
                b"#define %s%-30s %2d\n\0" as *const u8 as *const libc::c_char,
                prefix,
                (**((*lemp).symbols).offset(i as isize)).name,
                i,
            );
            i += 1;
        }
        fclose(out);
    }
}
#[no_mangle]
pub unsafe extern "C" fn CompressTables(mut lemp: *mut lemon) {
    let mut stp: *mut state = 0 as *mut state;
    let mut ap: *mut action = 0 as *mut action;
    let mut ap2: *mut action = 0 as *mut action;
    let mut rp: *mut rule = 0 as *mut rule;
    let mut rp2: *mut rule = 0 as *mut rule;
    let mut rbest: *mut rule = 0 as *mut rule;
    let mut nbest: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*lemp).nstate {
        stp = *((*lemp).sorted).offset(i as isize);
        nbest = 0 as libc::c_int;
        rbest = 0 as *mut rule;
        ap = (*stp).ap;
        while !ap.is_null() {
            if !((*ap).type_0 as libc::c_uint != REDUCE as libc::c_int as libc::c_uint) {
                rp = (*ap).x.rp;
                if !(rp == rbest) {
                    n = 1 as libc::c_int;
                    ap2 = (*ap).next;
                    while !ap2.is_null() {
                        if !((*ap2).type_0 as libc::c_uint
                            != REDUCE as libc::c_int as libc::c_uint)
                        {
                            rp2 = (*ap2).x.rp;
                            if !(rp2 == rbest) {
                                if rp2 == rp {
                                    n += 1;
                                }
                            }
                        }
                        ap2 = (*ap2).next;
                    }
                    if n > nbest {
                        nbest = n;
                        rbest = rp;
                    }
                }
            }
            ap = (*ap).next;
        }
        if !(nbest < 2 as libc::c_int) {
            ap = (*stp).ap;
            while !ap.is_null() {
                if (*ap).type_0 as libc::c_uint == REDUCE as libc::c_int as libc::c_uint
                    && (*ap).x.rp == rbest
                {
                    break;
                }
                ap = (*ap).next;
            }
            if ap.is_null() {
                myassert(
                    b"src/lemon.c\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    3682 as libc::c_int,
                );
            }
            (*ap).sp = Symbol_new(b"{default}\0" as *const u8 as *const libc::c_char);
            ap = (*ap).next;
            while !ap.is_null() {
                if (*ap).type_0 as libc::c_uint == REDUCE as libc::c_int as libc::c_uint
                    && (*ap).x.rp == rbest
                {
                    (*ap).type_0 = NOT_USED;
                }
                ap = (*ap).next;
            }
            (*stp).ap = Action_sort((*stp).ap);
        }
        i += 1;
    }
}
static mut global_size: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn SetSize(mut n: libc::c_int) {
    global_size = n + 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SetNew() -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    s = malloc(global_size as libc::c_ulong) as *mut libc::c_char;
    if s.is_null() {
        memory_error();
    }
    i = 0 as libc::c_int;
    while i < global_size {
        *s.offset(i as isize) = 0 as libc::c_int as libc::c_char;
        i += 1;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn SetFree(mut s: *mut libc::c_char) {
    free(s as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn SetAdd(
    mut s: *mut libc::c_char,
    mut e: libc::c_int,
) -> libc::c_int {
    let mut rv: libc::c_int = 0;
    rv = *s.offset(e as isize) as libc::c_int;
    *s.offset(e as isize) = 1 as libc::c_int as libc::c_char;
    return (rv == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SetUnion(
    mut s1: *mut libc::c_char,
    mut s2: *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut progress: libc::c_int = 0;
    progress = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < global_size {
        if !(*s2.offset(i as isize) as libc::c_int == 0 as libc::c_int) {
            if *s1.offset(i as isize) as libc::c_int == 0 as libc::c_int {
                progress = 1 as libc::c_int;
                *s1.offset(i as isize) = 1 as libc::c_int as libc::c_char;
            }
        }
        i += 1;
    }
    return progress;
}
unsafe extern "C" fn strhash(mut x: *mut libc::c_char) -> libc::c_int {
    let mut h: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while *x != 0 {
        let fresh23 = x;
        x = x.offset(1);
        h = h.wrapping_mul(13 as libc::c_uint).wrapping_add(*fresh23 as libc::c_uint);
    }
    return h as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Strsafe(mut y: *mut libc::c_char) -> *mut libc::c_char {
    let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
    z = Strsafe_find(y);
    if z.is_null()
        && {
            z = malloc((strlen(y)).wrapping_add(1 as libc::c_int as libc::c_ulong))
                as *mut libc::c_char;
            !z.is_null()
        }
    {
        strcpy(z, y);
        Strsafe_insert(z);
    }
    if z.is_null() {
        memory_error();
    }
    return z;
}
static mut x1a: *mut s_x1 = 0 as *const s_x1 as *mut s_x1;
#[no_mangle]
pub unsafe extern "C" fn Strsafe_init() {
    if !x1a.is_null() {
        return;
    }
    x1a = malloc(::std::mem::size_of::<s_x1>() as libc::c_ulong) as *mut s_x1;
    if !x1a.is_null() {
        (*x1a).size = 1024 as libc::c_int;
        (*x1a).count = 0 as libc::c_int;
        (*x1a)
            .tbl = malloc(
            (::std::mem::size_of::<x1node>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<*mut x1node>() as libc::c_ulong)
                .wrapping_mul(1024 as libc::c_int as libc::c_ulong),
        ) as *mut x1node;
        if ((*x1a).tbl).is_null() {
            free(x1a as *mut libc::c_void);
            x1a = 0 as *mut s_x1;
        } else {
            let mut i: libc::c_int = 0;
            (*x1a)
                .ht = &mut *((*x1a).tbl).offset(1024 as libc::c_int as isize)
                as *mut s_x1node as *mut *mut x1node;
            i = 0 as libc::c_int;
            while i < 1024 as libc::c_int {
                let ref mut fresh24 = *((*x1a).ht).offset(i as isize);
                *fresh24 = 0 as *mut s_x1node;
                i += 1;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Strsafe_insert(mut data: *mut libc::c_char) -> libc::c_int {
    let mut np: *mut x1node = 0 as *mut x1node;
    let mut h: libc::c_int = 0;
    let mut ph: libc::c_int = 0;
    if x1a.is_null() {
        return 0 as libc::c_int;
    }
    ph = strhash(data);
    h = ph & (*x1a).size - 1 as libc::c_int;
    np = *((*x1a).ht).offset(h as isize);
    while !np.is_null() {
        if strcmp((*np).data, data) == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        np = (*np).next;
    }
    if (*x1a).count >= (*x1a).size {
        let mut i: libc::c_int = 0;
        let mut size: libc::c_int = 0;
        let mut array: s_x1 = s_x1 {
            size: 0,
            count: 0,
            tbl: 0 as *mut s_x1node,
            ht: 0 as *mut *mut s_x1node,
        };
        size = (*x1a).size * 2 as libc::c_int;
        array.size = size;
        array.count = (*x1a).count;
        array
            .tbl = malloc(
            (::std::mem::size_of::<x1node>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<*mut x1node>() as libc::c_ulong)
                .wrapping_mul(size as libc::c_ulong),
        ) as *mut x1node;
        if (array.tbl).is_null() {
            return 0 as libc::c_int;
        }
        array
            .ht = &mut *(array.tbl).offset(size as isize) as *mut s_x1node
            as *mut *mut x1node;
        i = 0 as libc::c_int;
        while i < size {
            let ref mut fresh25 = *(array.ht).offset(i as isize);
            *fresh25 = 0 as *mut s_x1node;
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < (*x1a).count {
            let mut oldnp: *mut x1node = 0 as *mut x1node;
            let mut newnp: *mut x1node = 0 as *mut x1node;
            oldnp = &mut *((*x1a).tbl).offset(i as isize) as *mut s_x1node;
            h = strhash((*oldnp).data) & size - 1 as libc::c_int;
            newnp = &mut *(array.tbl).offset(i as isize) as *mut s_x1node;
            if !(*(array.ht).offset(h as isize)).is_null() {
                let ref mut fresh26 = (**(array.ht).offset(h as isize)).from;
                *fresh26 = &mut (*newnp).next;
            }
            (*newnp).next = *(array.ht).offset(h as isize);
            (*newnp).data = (*oldnp).data;
            (*newnp).from = &mut *(array.ht).offset(h as isize) as *mut *mut s_x1node;
            let ref mut fresh27 = *(array.ht).offset(h as isize);
            *fresh27 = newnp;
            i += 1;
        }
        free((*x1a).tbl as *mut libc::c_void);
        memcpy(
            x1a as *mut libc::c_void,
            &mut array as *mut s_x1 as *const libc::c_void,
            ::std::mem::size_of::<s_x1>() as libc::c_ulong,
        );
    }
    h = ph & (*x1a).size - 1 as libc::c_int;
    let fresh28 = (*x1a).count;
    (*x1a).count = (*x1a).count + 1;
    np = &mut *((*x1a).tbl).offset(fresh28 as isize) as *mut s_x1node;
    (*np).data = data;
    if !(*((*x1a).ht).offset(h as isize)).is_null() {
        let ref mut fresh29 = (**((*x1a).ht).offset(h as isize)).from;
        *fresh29 = &mut (*np).next;
    }
    (*np).next = *((*x1a).ht).offset(h as isize);
    let ref mut fresh30 = *((*x1a).ht).offset(h as isize);
    *fresh30 = np;
    (*np).from = &mut *((*x1a).ht).offset(h as isize) as *mut *mut s_x1node;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Strsafe_find(mut key: *mut libc::c_char) -> *mut libc::c_char {
    let mut h: libc::c_int = 0;
    let mut np: *mut x1node = 0 as *mut x1node;
    if x1a.is_null() {
        return 0 as *mut libc::c_char;
    }
    h = strhash(key) & (*x1a).size - 1 as libc::c_int;
    np = *((*x1a).ht).offset(h as isize);
    while !np.is_null() {
        if strcmp((*np).data, key) == 0 as libc::c_int {
            break;
        }
        np = (*np).next;
    }
    return if !np.is_null() { (*np).data } else { 0 as *mut libc::c_char };
}
#[no_mangle]
pub unsafe extern "C" fn Symbol_new(mut x: *mut libc::c_char) -> *mut symbol {
    let mut sp: *mut symbol = 0 as *mut symbol;
    sp = Symbol_find(x);
    if sp.is_null() {
        sp = malloc(::std::mem::size_of::<symbol>() as libc::c_ulong) as *mut symbol;
        if sp.is_null() {
            memory_error();
        }
        (*sp).name = Strsafe(x);
        (*sp)
            .type_0 = (if *(*__ctype_b_loc()).offset(*x as libc::c_int as isize)
            as libc::c_int & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            TERMINAL as libc::c_int
        } else {
            NONTERMINAL as libc::c_int
        }) as C2RustUnnamed_1;
        (*sp).rule = 0 as *mut rule;
        (*sp).fallback = 0 as *mut symbol;
        (*sp).prec = -(1 as libc::c_int);
        (*sp).assoc = UNK;
        (*sp).firstset = 0 as *mut libc::c_char;
        (*sp).lambda = Bo_FALSE;
        (*sp).destructor = 0 as *mut libc::c_char;
        (*sp).datatype = 0 as *mut libc::c_char;
        Symbol_insert(sp, (*sp).name);
    }
    return sp;
}
#[no_mangle]
pub unsafe extern "C" fn Symbolcmpp(
    mut a: *mut *mut symbol,
    mut b: *mut *mut symbol,
) -> libc::c_int {
    let mut i1: libc::c_int = (**a).index
        + 10000000 as libc::c_int
            * (*((**a).name).offset(0 as libc::c_int as isize) as libc::c_int
                > 'Z' as i32) as libc::c_int;
    let mut i2: libc::c_int = (**b).index
        + 10000000 as libc::c_int
            * (*((**b).name).offset(0 as libc::c_int as isize) as libc::c_int
                > 'Z' as i32) as libc::c_int;
    return i1 - i2;
}
static mut x2a: *mut s_x2 = 0 as *const s_x2 as *mut s_x2;
#[no_mangle]
pub unsafe extern "C" fn Symbol_init() {
    if !x2a.is_null() {
        return;
    }
    x2a = malloc(::std::mem::size_of::<s_x2>() as libc::c_ulong) as *mut s_x2;
    if !x2a.is_null() {
        (*x2a).size = 128 as libc::c_int;
        (*x2a).count = 0 as libc::c_int;
        (*x2a)
            .tbl = malloc(
            (::std::mem::size_of::<x2node>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<*mut x2node>() as libc::c_ulong)
                .wrapping_mul(128 as libc::c_int as libc::c_ulong),
        ) as *mut x2node;
        if ((*x2a).tbl).is_null() {
            free(x2a as *mut libc::c_void);
            x2a = 0 as *mut s_x2;
        } else {
            let mut i: libc::c_int = 0;
            (*x2a)
                .ht = &mut *((*x2a).tbl).offset(128 as libc::c_int as isize)
                as *mut s_x2node as *mut *mut x2node;
            i = 0 as libc::c_int;
            while i < 128 as libc::c_int {
                let ref mut fresh31 = *((*x2a).ht).offset(i as isize);
                *fresh31 = 0 as *mut s_x2node;
                i += 1;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Symbol_insert(
    mut data: *mut symbol,
    mut key: *mut libc::c_char,
) -> libc::c_int {
    let mut np: *mut x2node = 0 as *mut x2node;
    let mut h: libc::c_int = 0;
    let mut ph: libc::c_int = 0;
    if x2a.is_null() {
        return 0 as libc::c_int;
    }
    ph = strhash(key);
    h = ph & (*x2a).size - 1 as libc::c_int;
    np = *((*x2a).ht).offset(h as isize);
    while !np.is_null() {
        if strcmp((*np).key, key) == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        np = (*np).next;
    }
    if (*x2a).count >= (*x2a).size {
        let mut i: libc::c_int = 0;
        let mut size: libc::c_int = 0;
        let mut array: s_x2 = s_x2 {
            size: 0,
            count: 0,
            tbl: 0 as *mut s_x2node,
            ht: 0 as *mut *mut s_x2node,
        };
        size = (*x2a).size * 2 as libc::c_int;
        array.size = size;
        array.count = (*x2a).count;
        array
            .tbl = malloc(
            (::std::mem::size_of::<x2node>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<*mut x2node>() as libc::c_ulong)
                .wrapping_mul(size as libc::c_ulong),
        ) as *mut x2node;
        if (array.tbl).is_null() {
            return 0 as libc::c_int;
        }
        array
            .ht = &mut *(array.tbl).offset(size as isize) as *mut s_x2node
            as *mut *mut x2node;
        i = 0 as libc::c_int;
        while i < size {
            let ref mut fresh32 = *(array.ht).offset(i as isize);
            *fresh32 = 0 as *mut s_x2node;
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < (*x2a).count {
            let mut oldnp: *mut x2node = 0 as *mut x2node;
            let mut newnp: *mut x2node = 0 as *mut x2node;
            oldnp = &mut *((*x2a).tbl).offset(i as isize) as *mut s_x2node;
            h = strhash((*oldnp).key) & size - 1 as libc::c_int;
            newnp = &mut *(array.tbl).offset(i as isize) as *mut s_x2node;
            if !(*(array.ht).offset(h as isize)).is_null() {
                let ref mut fresh33 = (**(array.ht).offset(h as isize)).from;
                *fresh33 = &mut (*newnp).next;
            }
            (*newnp).next = *(array.ht).offset(h as isize);
            (*newnp).key = (*oldnp).key;
            (*newnp).data = (*oldnp).data;
            (*newnp).from = &mut *(array.ht).offset(h as isize) as *mut *mut s_x2node;
            let ref mut fresh34 = *(array.ht).offset(h as isize);
            *fresh34 = newnp;
            i += 1;
        }
        free((*x2a).tbl as *mut libc::c_void);
        memcpy(
            x2a as *mut libc::c_void,
            &mut array as *mut s_x2 as *const libc::c_void,
            ::std::mem::size_of::<s_x2>() as libc::c_ulong,
        );
    }
    h = ph & (*x2a).size - 1 as libc::c_int;
    let fresh35 = (*x2a).count;
    (*x2a).count = (*x2a).count + 1;
    np = &mut *((*x2a).tbl).offset(fresh35 as isize) as *mut s_x2node;
    (*np).key = key;
    (*np).data = data;
    if !(*((*x2a).ht).offset(h as isize)).is_null() {
        let ref mut fresh36 = (**((*x2a).ht).offset(h as isize)).from;
        *fresh36 = &mut (*np).next;
    }
    (*np).next = *((*x2a).ht).offset(h as isize);
    let ref mut fresh37 = *((*x2a).ht).offset(h as isize);
    *fresh37 = np;
    (*np).from = &mut *((*x2a).ht).offset(h as isize) as *mut *mut s_x2node;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Symbol_find(mut key: *mut libc::c_char) -> *mut symbol {
    let mut h: libc::c_int = 0;
    let mut np: *mut x2node = 0 as *mut x2node;
    if x2a.is_null() {
        return 0 as *mut symbol;
    }
    h = strhash(key) & (*x2a).size - 1 as libc::c_int;
    np = *((*x2a).ht).offset(h as isize);
    while !np.is_null() {
        if strcmp((*np).key, key) == 0 as libc::c_int {
            break;
        }
        np = (*np).next;
    }
    return if !np.is_null() { (*np).data } else { 0 as *mut symbol };
}
#[no_mangle]
pub unsafe extern "C" fn Symbol_Nth(mut n: libc::c_int) -> *mut symbol {
    let mut data: *mut symbol = 0 as *mut symbol;
    if !x2a.is_null() && n > 0 as libc::c_int && n <= (*x2a).count {
        data = (*((*x2a).tbl).offset((n - 1 as libc::c_int) as isize)).data;
    } else {
        data = 0 as *mut symbol;
    }
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn Symbol_count() -> libc::c_int {
    return if !x2a.is_null() { (*x2a).count } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn Symbol_arrayof() -> *mut *mut symbol {
    let mut array: *mut *mut symbol = 0 as *mut *mut symbol;
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    if x2a.is_null() {
        return 0 as *mut *mut symbol;
    }
    size = (*x2a).count;
    array = malloc(
        (::std::mem::size_of::<*mut symbol>() as libc::c_ulong)
            .wrapping_mul(size as libc::c_ulong),
    ) as *mut *mut symbol;
    if !array.is_null() {
        i = 0 as libc::c_int;
        while i < size {
            let ref mut fresh38 = *array.offset(i as isize);
            *fresh38 = (*((*x2a).tbl).offset(i as isize)).data;
            i += 1;
        }
    }
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn Configcmp(
    mut a: *mut config,
    mut b: *mut config,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    x = (*(*a).rp).index - (*(*b).rp).index;
    if x == 0 as libc::c_int {
        x = (*a).dot - (*b).dot;
    }
    return x;
}
unsafe extern "C" fn statecmp(mut a: *mut config, mut b: *mut config) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    rc = 0 as libc::c_int;
    while rc == 0 as libc::c_int && !a.is_null() && !b.is_null() {
        rc = (*(*a).rp).index - (*(*b).rp).index;
        if rc == 0 as libc::c_int {
            rc = (*a).dot - (*b).dot;
        }
        a = (*a).bp;
        b = (*b).bp;
    }
    if rc == 0 as libc::c_int {
        if !a.is_null() {
            rc = 1 as libc::c_int;
        }
        if !b.is_null() {
            rc = -(1 as libc::c_int);
        }
    }
    return rc;
}
unsafe extern "C" fn statehash(mut a: *mut config) -> libc::c_int {
    let mut h: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while !a.is_null() {
        h = h
            .wrapping_mul(571 as libc::c_uint)
            .wrapping_add(
                ((*(*a).rp).index as libc::c_uint).wrapping_mul(37 as libc::c_uint),
            )
            .wrapping_add((*a).dot as libc::c_uint);
        a = (*a).bp;
    }
    return h as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn State_new() -> *mut state {
    let mut new: *mut state = 0 as *mut state;
    new = malloc(::std::mem::size_of::<state>() as libc::c_ulong) as *mut state;
    if new.is_null() {
        memory_error();
    }
    return new;
}
static mut x3a: *mut s_x3 = 0 as *const s_x3 as *mut s_x3;
#[no_mangle]
pub unsafe extern "C" fn State_init() {
    if !x3a.is_null() {
        return;
    }
    x3a = malloc(::std::mem::size_of::<s_x3>() as libc::c_ulong) as *mut s_x3;
    if !x3a.is_null() {
        (*x3a).size = 128 as libc::c_int;
        (*x3a).count = 0 as libc::c_int;
        (*x3a)
            .tbl = malloc(
            (::std::mem::size_of::<x3node>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<*mut x3node>() as libc::c_ulong)
                .wrapping_mul(128 as libc::c_int as libc::c_ulong),
        ) as *mut x3node;
        if ((*x3a).tbl).is_null() {
            free(x3a as *mut libc::c_void);
            x3a = 0 as *mut s_x3;
        } else {
            let mut i: libc::c_int = 0;
            (*x3a)
                .ht = &mut *((*x3a).tbl).offset(128 as libc::c_int as isize)
                as *mut s_x3node as *mut *mut x3node;
            i = 0 as libc::c_int;
            while i < 128 as libc::c_int {
                let ref mut fresh39 = *((*x3a).ht).offset(i as isize);
                *fresh39 = 0 as *mut s_x3node;
                i += 1;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn State_insert(
    mut data: *mut state,
    mut key: *mut config,
) -> libc::c_int {
    let mut np: *mut x3node = 0 as *mut x3node;
    let mut h: libc::c_int = 0;
    let mut ph: libc::c_int = 0;
    if x3a.is_null() {
        return 0 as libc::c_int;
    }
    ph = statehash(key);
    h = ph & (*x3a).size - 1 as libc::c_int;
    np = *((*x3a).ht).offset(h as isize);
    while !np.is_null() {
        if statecmp((*np).key, key) == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        np = (*np).next;
    }
    if (*x3a).count >= (*x3a).size {
        let mut i: libc::c_int = 0;
        let mut size: libc::c_int = 0;
        let mut array: s_x3 = s_x3 {
            size: 0,
            count: 0,
            tbl: 0 as *mut s_x3node,
            ht: 0 as *mut *mut s_x3node,
        };
        size = (*x3a).size * 2 as libc::c_int;
        array.size = size;
        array.count = (*x3a).count;
        array
            .tbl = malloc(
            (::std::mem::size_of::<x3node>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<*mut x3node>() as libc::c_ulong)
                .wrapping_mul(size as libc::c_ulong),
        ) as *mut x3node;
        if (array.tbl).is_null() {
            return 0 as libc::c_int;
        }
        array
            .ht = &mut *(array.tbl).offset(size as isize) as *mut s_x3node
            as *mut *mut x3node;
        i = 0 as libc::c_int;
        while i < size {
            let ref mut fresh40 = *(array.ht).offset(i as isize);
            *fresh40 = 0 as *mut s_x3node;
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < (*x3a).count {
            let mut oldnp: *mut x3node = 0 as *mut x3node;
            let mut newnp: *mut x3node = 0 as *mut x3node;
            oldnp = &mut *((*x3a).tbl).offset(i as isize) as *mut s_x3node;
            h = statehash((*oldnp).key) & size - 1 as libc::c_int;
            newnp = &mut *(array.tbl).offset(i as isize) as *mut s_x3node;
            if !(*(array.ht).offset(h as isize)).is_null() {
                let ref mut fresh41 = (**(array.ht).offset(h as isize)).from;
                *fresh41 = &mut (*newnp).next;
            }
            (*newnp).next = *(array.ht).offset(h as isize);
            (*newnp).key = (*oldnp).key;
            (*newnp).data = (*oldnp).data;
            (*newnp).from = &mut *(array.ht).offset(h as isize) as *mut *mut s_x3node;
            let ref mut fresh42 = *(array.ht).offset(h as isize);
            *fresh42 = newnp;
            i += 1;
        }
        free((*x3a).tbl as *mut libc::c_void);
        memcpy(
            x3a as *mut libc::c_void,
            &mut array as *mut s_x3 as *const libc::c_void,
            ::std::mem::size_of::<s_x3>() as libc::c_ulong,
        );
    }
    h = ph & (*x3a).size - 1 as libc::c_int;
    let fresh43 = (*x3a).count;
    (*x3a).count = (*x3a).count + 1;
    np = &mut *((*x3a).tbl).offset(fresh43 as isize) as *mut s_x3node;
    (*np).key = key;
    (*np).data = data;
    if !(*((*x3a).ht).offset(h as isize)).is_null() {
        let ref mut fresh44 = (**((*x3a).ht).offset(h as isize)).from;
        *fresh44 = &mut (*np).next;
    }
    (*np).next = *((*x3a).ht).offset(h as isize);
    let ref mut fresh45 = *((*x3a).ht).offset(h as isize);
    *fresh45 = np;
    (*np).from = &mut *((*x3a).ht).offset(h as isize) as *mut *mut s_x3node;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn State_find(mut key: *mut config) -> *mut state {
    let mut h: libc::c_int = 0;
    let mut np: *mut x3node = 0 as *mut x3node;
    if x3a.is_null() {
        return 0 as *mut state;
    }
    h = statehash(key) & (*x3a).size - 1 as libc::c_int;
    np = *((*x3a).ht).offset(h as isize);
    while !np.is_null() {
        if statecmp((*np).key, key) == 0 as libc::c_int {
            break;
        }
        np = (*np).next;
    }
    return if !np.is_null() { (*np).data } else { 0 as *mut state };
}
#[no_mangle]
pub unsafe extern "C" fn State_count() -> libc::c_int {
    return if !x3a.is_null() { (*x3a).count } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn State_arrayof() -> *mut *mut state {
    let mut array: *mut *mut state = 0 as *mut *mut state;
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    if x3a.is_null() {
        return 0 as *mut *mut state;
    }
    size = (*x3a).count;
    array = malloc(
        (::std::mem::size_of::<*mut state>() as libc::c_ulong)
            .wrapping_mul(size as libc::c_ulong),
    ) as *mut *mut state;
    if !array.is_null() {
        i = 0 as libc::c_int;
        while i < size {
            let ref mut fresh46 = *array.offset(i as isize);
            *fresh46 = (*((*x3a).tbl).offset(i as isize)).data;
            i += 1;
        }
    }
    return array;
}
unsafe extern "C" fn confighash(mut a: *mut config) -> libc::c_int {
    let mut h: libc::c_int = 0 as libc::c_int;
    h = h * 571 as libc::c_int + (*(*a).rp).index * 37 as libc::c_int + (*a).dot;
    return h;
}
static mut x4a: *mut s_x4 = 0 as *const s_x4 as *mut s_x4;
#[no_mangle]
pub unsafe extern "C" fn Configtable_init() {
    if !x4a.is_null() {
        return;
    }
    x4a = malloc(::std::mem::size_of::<s_x4>() as libc::c_ulong) as *mut s_x4;
    if !x4a.is_null() {
        (*x4a).size = 64 as libc::c_int;
        (*x4a).count = 0 as libc::c_int;
        (*x4a)
            .tbl = malloc(
            (::std::mem::size_of::<x4node>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<*mut x4node>() as libc::c_ulong)
                .wrapping_mul(64 as libc::c_int as libc::c_ulong),
        ) as *mut x4node;
        if ((*x4a).tbl).is_null() {
            free(x4a as *mut libc::c_void);
            x4a = 0 as *mut s_x4;
        } else {
            let mut i: libc::c_int = 0;
            (*x4a)
                .ht = &mut *((*x4a).tbl).offset(64 as libc::c_int as isize)
                as *mut s_x4node as *mut *mut x4node;
            i = 0 as libc::c_int;
            while i < 64 as libc::c_int {
                let ref mut fresh47 = *((*x4a).ht).offset(i as isize);
                *fresh47 = 0 as *mut s_x4node;
                i += 1;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Configtable_insert(mut data: *mut config) -> libc::c_int {
    let mut np: *mut x4node = 0 as *mut x4node;
    let mut h: libc::c_int = 0;
    let mut ph: libc::c_int = 0;
    if x4a.is_null() {
        return 0 as libc::c_int;
    }
    ph = confighash(data);
    h = ph & (*x4a).size - 1 as libc::c_int;
    np = *((*x4a).ht).offset(h as isize);
    while !np.is_null() {
        if Configcmp((*np).data, data) == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        np = (*np).next;
    }
    if (*x4a).count >= (*x4a).size {
        let mut i: libc::c_int = 0;
        let mut size: libc::c_int = 0;
        let mut array: s_x4 = s_x4 {
            size: 0,
            count: 0,
            tbl: 0 as *mut s_x4node,
            ht: 0 as *mut *mut s_x4node,
        };
        size = (*x4a).size * 2 as libc::c_int;
        array.size = size;
        array.count = (*x4a).count;
        array
            .tbl = malloc(
            (::std::mem::size_of::<x4node>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<*mut x4node>() as libc::c_ulong)
                .wrapping_mul(size as libc::c_ulong),
        ) as *mut x4node;
        if (array.tbl).is_null() {
            return 0 as libc::c_int;
        }
        array
            .ht = &mut *(array.tbl).offset(size as isize) as *mut s_x4node
            as *mut *mut x4node;
        i = 0 as libc::c_int;
        while i < size {
            let ref mut fresh48 = *(array.ht).offset(i as isize);
            *fresh48 = 0 as *mut s_x4node;
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < (*x4a).count {
            let mut oldnp: *mut x4node = 0 as *mut x4node;
            let mut newnp: *mut x4node = 0 as *mut x4node;
            oldnp = &mut *((*x4a).tbl).offset(i as isize) as *mut s_x4node;
            h = confighash((*oldnp).data) & size - 1 as libc::c_int;
            newnp = &mut *(array.tbl).offset(i as isize) as *mut s_x4node;
            if !(*(array.ht).offset(h as isize)).is_null() {
                let ref mut fresh49 = (**(array.ht).offset(h as isize)).from;
                *fresh49 = &mut (*newnp).next;
            }
            (*newnp).next = *(array.ht).offset(h as isize);
            (*newnp).data = (*oldnp).data;
            (*newnp).from = &mut *(array.ht).offset(h as isize) as *mut *mut s_x4node;
            let ref mut fresh50 = *(array.ht).offset(h as isize);
            *fresh50 = newnp;
            i += 1;
        }
        free((*x4a).tbl as *mut libc::c_void);
        memcpy(
            x4a as *mut libc::c_void,
            &mut array as *mut s_x4 as *const libc::c_void,
            ::std::mem::size_of::<s_x4>() as libc::c_ulong,
        );
    }
    h = ph & (*x4a).size - 1 as libc::c_int;
    let fresh51 = (*x4a).count;
    (*x4a).count = (*x4a).count + 1;
    np = &mut *((*x4a).tbl).offset(fresh51 as isize) as *mut s_x4node;
    (*np).data = data;
    if !(*((*x4a).ht).offset(h as isize)).is_null() {
        let ref mut fresh52 = (**((*x4a).ht).offset(h as isize)).from;
        *fresh52 = &mut (*np).next;
    }
    (*np).next = *((*x4a).ht).offset(h as isize);
    let ref mut fresh53 = *((*x4a).ht).offset(h as isize);
    *fresh53 = np;
    (*np).from = &mut *((*x4a).ht).offset(h as isize) as *mut *mut s_x4node;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Configtable_find(mut key: *mut config) -> *mut config {
    let mut h: libc::c_int = 0;
    let mut np: *mut x4node = 0 as *mut x4node;
    if x4a.is_null() {
        return 0 as *mut config;
    }
    h = confighash(key) & (*x4a).size - 1 as libc::c_int;
    np = *((*x4a).ht).offset(h as isize);
    while !np.is_null() {
        if Configcmp((*np).data, key) == 0 as libc::c_int {
            break;
        }
        np = (*np).next;
    }
    return if !np.is_null() { (*np).data } else { 0 as *mut config };
}
#[no_mangle]
pub unsafe extern "C" fn Configtable_clear(
    mut f: Option::<unsafe extern "C" fn() -> libc::c_int>,
) {
    let mut i: libc::c_int = 0;
    if x4a.is_null() || (*x4a).count == 0 as libc::c_int {
        return;
    }
    if f.is_some() {
        i = 0 as libc::c_int;
        while i < (*x4a).count {
            ::std::mem::transmute::<
                _,
                fn(_) -> libc::c_int,
            >(
                (Some(f.expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )((*((*x4a).tbl).offset(i as isize)).data);
            i += 1;
        }
    }
    i = 0 as libc::c_int;
    while i < (*x4a).size {
        let ref mut fresh54 = *((*x4a).ht).offset(i as isize);
        *fresh54 = 0 as *mut s_x4node;
        i += 1;
    }
    (*x4a).count = 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
