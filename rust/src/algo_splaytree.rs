use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tree_node {
    pub left: *mut tree_node,
    pub right: *mut tree_node,
    pub key: libc::c_int,
    pub data: *mut libc::c_void,
}
pub type splay_tree = tree_node;
#[no_mangle]
pub unsafe extern "C" fn splaytree_splay(
    mut t: *mut splay_tree,
    mut i: libc::c_int,
) -> *mut splay_tree {
    let mut N: splay_tree = splay_tree {
        left: 0 as *mut tree_node,
        right: 0 as *mut tree_node,
        key: 0,
        data: 0 as *mut libc::c_void,
    };
    let mut l: *mut splay_tree = 0 as *mut splay_tree;
    let mut r: *mut splay_tree = 0 as *mut splay_tree;
    let mut y: *mut splay_tree = 0 as *mut splay_tree;
    let mut comp: libc::c_int = 0;
    if t.is_null() {
        return t;
    }
    N.right = 0 as *mut tree_node;
    N.left = N.right;
    r = &mut N;
    l = r;
    loop {
        comp = i - (*t).key;
        if comp < 0 as libc::c_int {
            if ((*t).left).is_null() {
                break;
            }
            if i - (*(*t).left).key < 0 as libc::c_int {
                y = (*t).left;
                (*t).left = (*y).right;
                (*y).right = t;
                t = y;
                if ((*t).left).is_null() {
                    break;
                }
            }
            (*r).left = t;
            r = t;
            t = (*t).left;
        } else {
            if !(comp > 0 as libc::c_int) {
                break;
            }
            if ((*t).right).is_null() {
                break;
            }
            if i - (*(*t).right).key > 0 as libc::c_int {
                y = (*t).right;
                (*t).right = (*y).left;
                (*y).left = t;
                t = y;
                if ((*t).right).is_null() {
                    break;
                }
            }
            (*l).right = t;
            l = t;
            t = (*t).right;
        }
    }
    (*l).right = (*t).left;
    (*r).left = (*t).right;
    (*t).left = N.right;
    (*t).right = N.left;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn splaytree_insert(
    mut t: *mut splay_tree,
    mut i: libc::c_int,
    mut data: *mut libc::c_void,
) -> *mut splay_tree {
    let mut new: *mut splay_tree = 0 as *mut splay_tree;
    if !t.is_null() {
        t = splaytree_splay(t, i);
        if i - (*t).key == 0 as libc::c_int {
            return t;
        }
    }
    new = malloc(::std::mem::size_of::<splay_tree>() as libc::c_ulong)
        as *mut splay_tree;
    if !new.is_null() {} else {
        __assert_fail(
            b"new\0" as *const u8 as *const libc::c_char,
            b"src/algo_splaytree.c\0" as *const u8 as *const libc::c_char,
            121 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"splay_tree *splaytree_insert(splay_tree *, int, void *)\0"))
                .as_ptr(),
        );
    }
    if t.is_null() {
        (*new).right = 0 as *mut tree_node;
        (*new).left = (*new).right;
    } else if i - (*t).key < 0 as libc::c_int {
        (*new).left = (*t).left;
        (*new).right = t;
        (*t).left = 0 as *mut tree_node;
    } else {
        (*new).right = (*t).right;
        (*new).left = t;
        (*t).right = 0 as *mut tree_node;
    }
    (*new).key = i;
    (*new).data = data;
    return new;
}
#[no_mangle]
pub unsafe extern "C" fn splaytree_delete(
    mut t: *mut splay_tree,
    mut i: libc::c_int,
) -> *mut splay_tree {
    let mut x: *mut splay_tree = 0 as *mut splay_tree;
    if t.is_null() {
        return 0 as *mut splay_tree;
    }
    t = splaytree_splay(t, i);
    if i - (*t).key == 0 as libc::c_int {
        if ((*t).left).is_null() {
            x = (*t).right;
        } else {
            x = splaytree_splay((*t).left, i);
            (*x).right = (*t).right;
        }
        free(t as *mut libc::c_void);
        return x;
    } else {
        return t
    };
}
