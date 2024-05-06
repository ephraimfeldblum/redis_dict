use std::rc::Rc;
use libc::{c_char, c_int, c_void};

use crate::bindings;

extern "C" {
    fn strlen(str: *const c_char) -> usize;
    fn strcmp(str1: *const c_char, str2: *const c_char) -> c_int;
}
pub fn u64_as_key(i: u64) -> *mut c_void {
    unsafe { std::mem::transmute(i) }
}
fn key_as_u64(key: *const c_void) -> u64 {
    unsafe { std::mem::transmute(key) }
}
unsafe extern "C" fn u64_hash(key: *const c_void) -> u64 {
    key_as_u64(key)
}
unsafe extern "C" fn u64_cmp(
    _: *mut bindings::dict,
    key1: *const c_void,
    key2: *const c_void,
) -> c_int {
    (key_as_u64(key1) == key_as_u64(key2)) as _
}
pub const INT_DICT_TYPE: bindings::dictType = bindings::dictType {
    hashFunction: Some(u64_hash),
    keyDup: None,
    valDup: None,
    keyCompare: Some(u64_cmp),
    keyDestructor: None,
    valDestructor: None,
    resizeAllowed: None,
    rehashingStarted: None,
    rehashingCompleted: None,
    dictMetadataBytes: None,
    userdata: std::ptr::null_mut(),
    _bitfield_align_1: [],
    _bitfield_1: bindings::__BindgenBitfieldUnit::new([0; 1]),
    __bindgen_padding_0: [0; 7],
};

unsafe extern "C" fn str_hash(key: *const c_void) -> u64 {
    bindings::dictGenHashFunction(key, strlen(key as _))
}
unsafe extern "C" fn str_cmp(
    _: *mut bindings::dict,
    key1: *const c_void,
    key2: *const c_void,
) -> c_int {
    (strcmp(key1 as _, key2 as _) == 0) as _
}
pub const STR_DICT_TYPE: bindings::dictType = bindings::dictType {
    hashFunction: Some(str_hash),
    keyDup: None,
    valDup: None,
    keyCompare: Some(str_cmp),
    keyDestructor: None,
    valDestructor: None,
    resizeAllowed: None,
    rehashingStarted: None,
    rehashingCompleted: None,
    dictMetadataBytes: None,
    userdata: std::ptr::null_mut(),
    _bitfield_align_1: [],
    _bitfield_1: bindings::__BindgenBitfieldUnit::new([0; 1]),
    __bindgen_padding_0: [0; 7],
};
unsafe extern "C" fn rcstr_hash(key: *const c_void) -> u64 {
    let key = key as *const Rc<str>;
    bindings::dictGenHashFunction((*key).as_ptr() as _, (*key).as_ref().len())
}
unsafe extern "C" fn rcstr_dup(_: *mut bindings::dict, key: *const c_void) -> *mut c_void {
    let key = key as *const Rc<str>;
    Box::into_raw(Box::new(key.as_ref().unwrap().clone())) as _
}
unsafe extern "C" fn rcstr_drop(_: *mut bindings::dict, key: *mut c_void) {
    drop(Box::from_raw(key as *mut Rc<str>))
}
unsafe extern "C" fn rcstr_cmp(
    _: *mut bindings::dict,
    key1: *const c_void,
    key2: *const c_void,
) -> c_int {
    let key1 = key1 as *const Rc<str>;
    let key2 = key2 as *const Rc<str>;
    (*key1 == *key2) as _
}
pub const RCSTR_DICT_TYPE: bindings::dictType = bindings::dictType {
    hashFunction: Some(rcstr_hash),
    keyDup: Some(rcstr_dup),
    valDup: None,
    keyCompare: Some(rcstr_cmp),
    keyDestructor: Some(rcstr_drop),
    valDestructor: None,
    resizeAllowed: None,
    rehashingStarted: None,
    rehashingCompleted: None,
    dictMetadataBytes: None,
    userdata: std::ptr::null_mut(),
    _bitfield_align_1: [],
    _bitfield_1: bindings::__BindgenBitfieldUnit::new([0; 1]),
    __bindgen_padding_0: [0; 7],
};