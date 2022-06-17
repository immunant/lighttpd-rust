use ::libc;
extern "C" {
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn timegm(__tp: *mut tm) -> time_t;
    static mut log_epoch_secs: unix_time64_t;
}
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type unix_time64_t = time_t;
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
#[inline]
unsafe extern "C" fn light_isdigit(mut c: libc::c_int) -> libc::c_int {
    return ((c as uint32_t).wrapping_sub('0' as i32 as libc::c_uint)
        <= ('9' as i32 - '0' as i32) as libc::c_uint) as libc::c_int;
}
static mut datestrs: [libc::c_char; 68] = unsafe {
    *::std::mem::transmute::<
        &[u8; 68],
        &[libc::c_char; 68],
    >(b"\0\n\x14\x1E(2<FPZSunMonTueWedThuFriSatJanFebMarAprMayJunJulAugSepOctNovDec\0")
};
#[cold]
unsafe extern "C" fn http_date_parse_RFC_850(
    mut s: *const libc::c_char,
    tm: *mut tm,
) -> *const libc::c_char {
    static mut tm_year_last_check: unix_time64_t = 0;
    static mut tm_year_cur: libc::c_int = 0;
    static mut tm_year_base: libc::c_int = 0;
    if log_epoch_secs >= tm_year_last_check + 60 as libc::c_int as libc::c_long {
        let mut tm_cur: tm = tm {
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
        if !(gmtime_r(&mut log_epoch_secs, &mut tm_cur)).is_null() {
            tm_year_last_check = log_epoch_secs;
            if tm_cur.tm_year != tm_year_cur {
                tm_year_cur = tm_cur.tm_year;
                tm_year_base = tm_year_cur - tm_year_cur % 100 as libc::c_int;
            }
        }
    }
    (*tm).tm_isdst = 0 as libc::c_int;
    (*tm).tm_yday = 0 as libc::c_int;
    (*tm).tm_wday = 0 as libc::c_int;
    (*tm).tm_mon = 0 as libc::c_int;
    let tens: *const libc::c_char = datestrs.as_ptr();
    let mut p: *const libc::c_char = tens.offset(10 as libc::c_int as isize);
    while !(*s.offset(0 as libc::c_int as isize) as libc::c_int
        == *p.offset(0 as libc::c_int as isize) as libc::c_int
        && *s.offset(1 as libc::c_int as isize) as libc::c_int
            == *p.offset(1 as libc::c_int as isize) as libc::c_int
        && *s.offset(2 as libc::c_int as isize) as libc::c_int
            == *p.offset(2 as libc::c_int as isize) as libc::c_int)
    {
        p = p.offset(3 as libc::c_int as isize);
        (*tm).tm_wday += 1;
        if !((*tm).tm_wday < 7 as libc::c_int) {
            break;
        }
    }
    if 7 as libc::c_int == (*tm).tm_wday {
        return 0 as *const libc::c_char;
    }
    s = s.offset(3 as libc::c_int as isize);
    while *s as libc::c_int != ',' as i32 && *s as libc::c_int != '\u{0}' as i32 {
        s = s.offset(1);
    }
    if *s.offset(0 as libc::c_int as isize) as libc::c_int != ',' as i32
        || *s.offset(1 as libc::c_int as isize) as libc::c_int != ' ' as i32
        || light_isdigit(*s.offset(2 as libc::c_int as isize) as libc::c_int) == 0
        || light_isdigit(*s.offset(3 as libc::c_int as isize) as libc::c_int) == 0
    {
        return 0 as *const libc::c_char;
    }
    (*tm)
        .tm_mday = *tens
        .offset(
            (*s.offset(2 as libc::c_int as isize) as libc::c_int - '0' as i32) as isize,
        ) as libc::c_int
        + (*s.offset(3 as libc::c_int as isize) as libc::c_int - '0' as i32);
    if *s.offset(4 as libc::c_int as isize) as libc::c_int != '-' as i32 {
        return 0 as *const libc::c_char;
    }
    p = tens
        .offset(10 as libc::c_int as isize)
        .offset(::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong as isize)
        .offset(-(1 as libc::c_int as isize));
    while !(*s.offset(5 as libc::c_int as isize) as libc::c_int
        == *p.offset(0 as libc::c_int as isize) as libc::c_int
        && *s.offset(6 as libc::c_int as isize) as libc::c_int
            == *p.offset(1 as libc::c_int as isize) as libc::c_int
        && *s.offset(7 as libc::c_int as isize) as libc::c_int
            == *p.offset(2 as libc::c_int as isize) as libc::c_int)
    {
        p = p.offset(3 as libc::c_int as isize);
        (*tm).tm_mon += 1;
        if !((*tm).tm_mon < 12 as libc::c_int) {
            break;
        }
    }
    if 12 as libc::c_int == (*tm).tm_mon {
        return 0 as *const libc::c_char;
    }
    if *s.offset(8 as libc::c_int as isize) as libc::c_int != '-' as i32
        || light_isdigit(*s.offset(9 as libc::c_int as isize) as libc::c_int) == 0
        || light_isdigit(*s.offset(10 as libc::c_int as isize) as libc::c_int) == 0
    {
        return 0 as *const libc::c_char;
    }
    (*tm)
        .tm_year = *tens
        .offset(
            (*s.offset(9 as libc::c_int as isize) as libc::c_int - '0' as i32) as isize,
        ) as libc::c_int
        + (*s.offset(10 as libc::c_int as isize) as libc::c_int - '0' as i32)
        + tm_year_base;
    if (*tm).tm_year > tm_year_cur + 50 as libc::c_int {
        (*tm).tm_year -= 100 as libc::c_int;
    }
    if *s.offset(11 as libc::c_int as isize) as libc::c_int != ' ' as i32
        || light_isdigit(*s.offset(12 as libc::c_int as isize) as libc::c_int) == 0
        || light_isdigit(*s.offset(13 as libc::c_int as isize) as libc::c_int) == 0
    {
        return 0 as *const libc::c_char;
    }
    (*tm)
        .tm_hour = *tens
        .offset(
            (*s.offset(12 as libc::c_int as isize) as libc::c_int - '0' as i32) as isize,
        ) as libc::c_int
        + (*s.offset(13 as libc::c_int as isize) as libc::c_int - '0' as i32);
    if *s.offset(14 as libc::c_int as isize) as libc::c_int != ':' as i32
        || light_isdigit(*s.offset(15 as libc::c_int as isize) as libc::c_int) == 0
        || light_isdigit(*s.offset(16 as libc::c_int as isize) as libc::c_int) == 0
    {
        return 0 as *const libc::c_char;
    }
    (*tm)
        .tm_min = *tens
        .offset(
            (*s.offset(15 as libc::c_int as isize) as libc::c_int - '0' as i32) as isize,
        ) as libc::c_int
        + (*s.offset(16 as libc::c_int as isize) as libc::c_int - '0' as i32);
    if *s.offset(17 as libc::c_int as isize) as libc::c_int != ':' as i32
        || light_isdigit(*s.offset(18 as libc::c_int as isize) as libc::c_int) == 0
        || light_isdigit(*s.offset(19 as libc::c_int as isize) as libc::c_int) == 0
    {
        return 0 as *const libc::c_char;
    }
    (*tm)
        .tm_sec = *tens
        .offset(
            (*s.offset(18 as libc::c_int as isize) as libc::c_int - '0' as i32) as isize,
        ) as libc::c_int
        + (*s.offset(19 as libc::c_int as isize) as libc::c_int - '0' as i32);
    if *s.offset(20 as libc::c_int as isize) as libc::c_int != ' ' as i32
        || *s.offset(21 as libc::c_int as isize) as libc::c_int != 'G' as i32
        || *s.offset(22 as libc::c_int as isize) as libc::c_int != 'M' as i32
        || *s.offset(23 as libc::c_int as isize) as libc::c_int != 'T' as i32
    {
        return 0 as *const libc::c_char;
    }
    return s.offset(24 as libc::c_int as isize);
}
#[cold]
unsafe extern "C" fn http_date_parse_asctime(
    s: *const libc::c_char,
    tm: *mut tm,
) -> *const libc::c_char {
    (*tm).tm_isdst = 0 as libc::c_int;
    (*tm).tm_yday = 0 as libc::c_int;
    (*tm).tm_wday = 0 as libc::c_int;
    (*tm).tm_mon = 0 as libc::c_int;
    let tens: *const libc::c_char = datestrs.as_ptr();
    let mut p: *const libc::c_char = tens.offset(10 as libc::c_int as isize);
    while !(*s.offset(0 as libc::c_int as isize) as libc::c_int
        == *p.offset(0 as libc::c_int as isize) as libc::c_int
        && *s.offset(1 as libc::c_int as isize) as libc::c_int
            == *p.offset(1 as libc::c_int as isize) as libc::c_int
        && *s.offset(2 as libc::c_int as isize) as libc::c_int
            == *p.offset(2 as libc::c_int as isize) as libc::c_int)
    {
        p = p.offset(3 as libc::c_int as isize);
        (*tm).tm_wday += 1;
        if !((*tm).tm_wday < 7 as libc::c_int) {
            break;
        }
    }
    if 7 as libc::c_int == (*tm).tm_wday {
        return 0 as *const libc::c_char;
    }
    if *s.offset(3 as libc::c_int as isize) as libc::c_int != ' ' as i32 {
        return 0 as *const libc::c_char;
    }
    p = tens
        .offset(10 as libc::c_int as isize)
        .offset(::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong as isize)
        .offset(-(1 as libc::c_int as isize));
    while !(*s.offset(4 as libc::c_int as isize) as libc::c_int
        == *p.offset(0 as libc::c_int as isize) as libc::c_int
        && *s.offset(5 as libc::c_int as isize) as libc::c_int
            == *p.offset(1 as libc::c_int as isize) as libc::c_int
        && *s.offset(6 as libc::c_int as isize) as libc::c_int
            == *p.offset(2 as libc::c_int as isize) as libc::c_int)
    {
        p = p.offset(3 as libc::c_int as isize);
        (*tm).tm_mon += 1;
        if !((*tm).tm_mon < 12 as libc::c_int) {
            break;
        }
    }
    if 12 as libc::c_int == (*tm).tm_mon {
        return 0 as *const libc::c_char;
    }
    if *s.offset(7 as libc::c_int as isize) as libc::c_int != ' ' as i32
        || *s.offset(8 as libc::c_int as isize) as libc::c_int != ' ' as i32
            && light_isdigit(*s.offset(8 as libc::c_int as isize) as libc::c_int) == 0
        || light_isdigit(*s.offset(9 as libc::c_int as isize) as libc::c_int) == 0
    {
        return 0 as *const libc::c_char;
    }
    (*tm)
        .tm_mday = (if *s.offset(8 as libc::c_int as isize) as libc::c_int == ' ' as i32
    {
        0 as libc::c_int
    } else {
        *tens
            .offset(
                (*s.offset(8 as libc::c_int as isize) as libc::c_int - '0' as i32)
                    as isize,
            ) as libc::c_int
    }) + (*s.offset(9 as libc::c_int as isize) as libc::c_int - '0' as i32);
    if *s.offset(10 as libc::c_int as isize) as libc::c_int != ' ' as i32
        || light_isdigit(*s.offset(11 as libc::c_int as isize) as libc::c_int) == 0
        || light_isdigit(*s.offset(12 as libc::c_int as isize) as libc::c_int) == 0
    {
        return 0 as *const libc::c_char;
    }
    (*tm)
        .tm_hour = *tens
        .offset(
            (*s.offset(11 as libc::c_int as isize) as libc::c_int - '0' as i32) as isize,
        ) as libc::c_int
        + (*s.offset(12 as libc::c_int as isize) as libc::c_int - '0' as i32);
    if *s.offset(13 as libc::c_int as isize) as libc::c_int != ':' as i32
        || light_isdigit(*s.offset(14 as libc::c_int as isize) as libc::c_int) == 0
        || light_isdigit(*s.offset(15 as libc::c_int as isize) as libc::c_int) == 0
    {
        return 0 as *const libc::c_char;
    }
    (*tm)
        .tm_min = *tens
        .offset(
            (*s.offset(14 as libc::c_int as isize) as libc::c_int - '0' as i32) as isize,
        ) as libc::c_int
        + (*s.offset(15 as libc::c_int as isize) as libc::c_int - '0' as i32);
    if *s.offset(16 as libc::c_int as isize) as libc::c_int != ':' as i32
        || light_isdigit(*s.offset(17 as libc::c_int as isize) as libc::c_int) == 0
        || light_isdigit(*s.offset(18 as libc::c_int as isize) as libc::c_int) == 0
    {
        return 0 as *const libc::c_char;
    }
    (*tm)
        .tm_sec = *tens
        .offset(
            (*s.offset(17 as libc::c_int as isize) as libc::c_int - '0' as i32) as isize,
        ) as libc::c_int
        + (*s.offset(18 as libc::c_int as isize) as libc::c_int - '0' as i32);
    if *s.offset(19 as libc::c_int as isize) as libc::c_int != ' ' as i32
        || light_isdigit(*s.offset(20 as libc::c_int as isize) as libc::c_int) == 0
        || light_isdigit(*s.offset(21 as libc::c_int as isize) as libc::c_int) == 0
        || light_isdigit(*s.offset(22 as libc::c_int as isize) as libc::c_int) == 0
        || light_isdigit(*s.offset(23 as libc::c_int as isize) as libc::c_int) == 0
    {
        return 0 as *const libc::c_char;
    }
    (*tm)
        .tm_year = (*tens
        .offset(
            (*s.offset(20 as libc::c_int as isize) as libc::c_int - '0' as i32) as isize,
        ) as libc::c_int
        + (*s.offset(21 as libc::c_int as isize) as libc::c_int - '0' as i32))
        * 100 as libc::c_int
        + *tens
            .offset(
                (*s.offset(22 as libc::c_int as isize) as libc::c_int - '0' as i32)
                    as isize,
            ) as libc::c_int
        + (*s.offset(23 as libc::c_int as isize) as libc::c_int - '0' as i32)
        - 1900 as libc::c_int;
    return s.offset(24 as libc::c_int as isize);
}
unsafe extern "C" fn http_date_parse_IMF_fixdate(
    s: *const libc::c_char,
    tm: *mut tm,
) -> *const libc::c_char {
    (*tm).tm_isdst = 0 as libc::c_int;
    (*tm).tm_yday = 0 as libc::c_int;
    (*tm).tm_wday = 0 as libc::c_int;
    (*tm).tm_mon = 0 as libc::c_int;
    let tens: *const libc::c_char = datestrs.as_ptr();
    let mut p: *const libc::c_char = tens.offset(10 as libc::c_int as isize);
    while !(*s.offset(0 as libc::c_int as isize) as libc::c_int
        == *p.offset(0 as libc::c_int as isize) as libc::c_int
        && *s.offset(1 as libc::c_int as isize) as libc::c_int
            == *p.offset(1 as libc::c_int as isize) as libc::c_int
        && *s.offset(2 as libc::c_int as isize) as libc::c_int
            == *p.offset(2 as libc::c_int as isize) as libc::c_int)
    {
        p = p.offset(3 as libc::c_int as isize);
        (*tm).tm_wday += 1;
        if !((*tm).tm_wday < 7 as libc::c_int) {
            break;
        }
    }
    if 7 as libc::c_int == (*tm).tm_wday {
        return 0 as *const libc::c_char;
    }
    if *s.offset(3 as libc::c_int as isize) as libc::c_int != ',' as i32
        || *s.offset(4 as libc::c_int as isize) as libc::c_int != ' ' as i32
        || light_isdigit(*s.offset(5 as libc::c_int as isize) as libc::c_int) == 0
        || light_isdigit(*s.offset(6 as libc::c_int as isize) as libc::c_int) == 0
    {
        return 0 as *const libc::c_char;
    }
    (*tm)
        .tm_mday = *tens
        .offset(
            (*s.offset(5 as libc::c_int as isize) as libc::c_int - '0' as i32) as isize,
        ) as libc::c_int
        + (*s.offset(6 as libc::c_int as isize) as libc::c_int - '0' as i32);
    if *s.offset(7 as libc::c_int as isize) as libc::c_int != ' ' as i32 {
        return 0 as *const libc::c_char;
    }
    p = tens
        .offset(10 as libc::c_int as isize)
        .offset(::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong as isize)
        .offset(-(1 as libc::c_int as isize));
    while !(*s.offset(8 as libc::c_int as isize) as libc::c_int
        == *p.offset(0 as libc::c_int as isize) as libc::c_int
        && *s.offset(9 as libc::c_int as isize) as libc::c_int
            == *p.offset(1 as libc::c_int as isize) as libc::c_int
        && *s.offset(10 as libc::c_int as isize) as libc::c_int
            == *p.offset(2 as libc::c_int as isize) as libc::c_int)
    {
        p = p.offset(3 as libc::c_int as isize);
        (*tm).tm_mon += 1;
        if !((*tm).tm_mon < 12 as libc::c_int) {
            break;
        }
    }
    if 12 as libc::c_int == (*tm).tm_mon {
        return 0 as *const libc::c_char;
    }
    if *s.offset(11 as libc::c_int as isize) as libc::c_int != ' ' as i32
        || light_isdigit(*s.offset(12 as libc::c_int as isize) as libc::c_int) == 0
        || light_isdigit(*s.offset(13 as libc::c_int as isize) as libc::c_int) == 0
        || light_isdigit(*s.offset(14 as libc::c_int as isize) as libc::c_int) == 0
        || light_isdigit(*s.offset(15 as libc::c_int as isize) as libc::c_int) == 0
    {
        return 0 as *const libc::c_char;
    }
    (*tm)
        .tm_year = (*tens
        .offset(
            (*s.offset(12 as libc::c_int as isize) as libc::c_int - '0' as i32) as isize,
        ) as libc::c_int
        + (*s.offset(13 as libc::c_int as isize) as libc::c_int - '0' as i32))
        * 100 as libc::c_int
        + *tens
            .offset(
                (*s.offset(14 as libc::c_int as isize) as libc::c_int - '0' as i32)
                    as isize,
            ) as libc::c_int
        + (*s.offset(15 as libc::c_int as isize) as libc::c_int - '0' as i32)
        - 1900 as libc::c_int;
    if *s.offset(16 as libc::c_int as isize) as libc::c_int != ' ' as i32
        || light_isdigit(*s.offset(17 as libc::c_int as isize) as libc::c_int) == 0
        || light_isdigit(*s.offset(18 as libc::c_int as isize) as libc::c_int) == 0
    {
        return 0 as *const libc::c_char;
    }
    (*tm)
        .tm_hour = *tens
        .offset(
            (*s.offset(17 as libc::c_int as isize) as libc::c_int - '0' as i32) as isize,
        ) as libc::c_int
        + (*s.offset(18 as libc::c_int as isize) as libc::c_int - '0' as i32);
    if *s.offset(19 as libc::c_int as isize) as libc::c_int != ':' as i32
        || light_isdigit(*s.offset(20 as libc::c_int as isize) as libc::c_int) == 0
        || light_isdigit(*s.offset(21 as libc::c_int as isize) as libc::c_int) == 0
    {
        return 0 as *const libc::c_char;
    }
    (*tm)
        .tm_min = *tens
        .offset(
            (*s.offset(20 as libc::c_int as isize) as libc::c_int - '0' as i32) as isize,
        ) as libc::c_int
        + (*s.offset(21 as libc::c_int as isize) as libc::c_int - '0' as i32);
    if *s.offset(22 as libc::c_int as isize) as libc::c_int != ':' as i32
        || light_isdigit(*s.offset(23 as libc::c_int as isize) as libc::c_int) == 0
        || light_isdigit(*s.offset(24 as libc::c_int as isize) as libc::c_int) == 0
    {
        return 0 as *const libc::c_char;
    }
    (*tm)
        .tm_sec = *tens
        .offset(
            (*s.offset(23 as libc::c_int as isize) as libc::c_int - '0' as i32) as isize,
        ) as libc::c_int
        + (*s.offset(24 as libc::c_int as isize) as libc::c_int - '0' as i32);
    if *s.offset(25 as libc::c_int as isize) as libc::c_int != ' ' as i32
        || *s.offset(26 as libc::c_int as isize) as libc::c_int != 'G' as i32
        || *s.offset(27 as libc::c_int as isize) as libc::c_int != 'M' as i32
        || *s.offset(28 as libc::c_int as isize) as libc::c_int != 'T' as i32
    {
        return 0 as *const libc::c_char;
    }
    return s.offset(29 as libc::c_int as isize);
}
unsafe extern "C" fn http_date_str_to_tm(
    s: *const libc::c_char,
    len: uint32_t,
    tm: *mut tm,
) -> *const libc::c_char {
    if len == 29 as libc::c_int as libc::c_uint {
        return http_date_parse_IMF_fixdate(s, tm)
    } else if len > 29 as libc::c_int as libc::c_uint {
        return http_date_parse_RFC_850(s, tm)
    } else {
        return http_date_parse_asctime(s, tm)
    };
}
#[no_mangle]
pub unsafe extern "C" fn http_date_time_to_str(
    s: *mut libc::c_char,
    sz: size_t,
    t: unix_time64_t,
) -> uint32_t {
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
    let fmt: [libc::c_char; 20] = *::std::mem::transmute::<
        &[u8; 20],
        &[libc::c_char; 20],
    >(b"%a, %d %b %Y %T GMT\0");
    return if (0 as *mut libc::c_void as *mut tm != gmtime_r(&t, &mut tm)) as libc::c_int
        as libc::c_long != 0
    {
        strftime(s, sz, fmt.as_ptr(), &mut tm) as uint32_t
    } else {
        0 as libc::c_int as libc::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C" fn http_date_if_modified_since(
    ifmod: *const libc::c_char,
    ifmodlen: uint32_t,
    lmtime: unix_time64_t,
) -> libc::c_int {
    let mut ifmodtm: tm = tm {
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
    if (http_date_str_to_tm(ifmod, ifmodlen, &mut ifmodtm)).is_null() {
        return 1 as libc::c_int;
    }
    let ifmtime: time_t = timegm(&mut ifmodtm);
    return (lmtime > ifmtime) as libc::c_int;
}
