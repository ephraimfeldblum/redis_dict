use std::ptr::NonNull;

use libc::c_void;

use crate::bindings;
use crate::entry::Entry;

type RedisDictResult = Result<(), ()>;
trait IntoResult<T> {
    fn into_result(&self) -> RedisDictResult;
}
impl IntoResult<i32> for i32 {
    fn into_result(&self) -> RedisDictResult {
        match self {
            0 => Ok(()),
            _ => Err(()),
        }
    }
}

pub struct Dict {
    inner: NonNull<bindings::dict>,
}

impl Dict {
    pub fn new(dt: &bindings::dictType) -> Self {
        Self {
            inner: unsafe { NonNull::new_unchecked(bindings::dictCreate(dt as *const _ as _)) },
        }
    }

    pub(crate) fn as_ref(&self) -> &bindings::dict {
        unsafe { self.inner.as_ref() }
    }
    pub(crate) fn as_mut(&mut self) -> &mut bindings::dict {
        unsafe { self.inner.as_mut() }
    }
    // pub(crate) fn as_ptr(self) -> *mut bindings::dict {
    //   self.inner.as_ptr()
    // }

    pub fn len(&self) -> usize {
        (self.as_ref().ht_used[0] + self.as_ref().ht_used[1]) as _
    }
    pub fn num_buckets(&self) -> usize {
        let d = self.as_ref();
        (if d.ht_size_exp[0] == -1 {
            0
        } else {
            1usize << (d.ht_size_exp[0])
        }) + (if d.ht_size_exp[1] == -1 {
            0
        } else {
            1usize << (d.ht_size_exp[1])
        })
    }
    pub fn expand(&mut self, size: usize) -> RedisDictResult {
        unsafe { bindings::dictExpand(self.as_mut(), size as _).into_result() }
    }
    pub fn try_expand(&mut self, size: usize) -> RedisDictResult {
        unsafe { bindings::dictTryExpand(self.as_mut(), size as _).into_result() }
    }
    pub fn shrink(&mut self, size: usize) -> RedisDictResult {
        unsafe { bindings::dictShrink(self.as_mut(), size as _).into_result() }
    }
    pub fn add(&mut self, key: *mut c_void, val: *mut c_void) -> RedisDictResult {
        unsafe { bindings::dictAdd(self.as_mut(), key, val).into_result() }
    }
    // pub fn add_raw(
    //     &mut self,
    //     key: *mut c_void,
    //     existing: &mut Entry,
    // ) -> Option<Entry> {
    //     Entry::new(unsafe {
    //         bindings::dictAddRaw(self.as_mut(), key, &mut existing.as_ptr())
    //     })
    // }
    // pub fn find_pos_for_insert(
    //     &mut self,
    //     key: *const c_void,
    //     existing: &mut Entry,
    // ) -> *mut c_void {
    //     unsafe {
    //         bindings::dictFindPositionForInsert(
    //             self.as_mut(),
    //             key,
    //             &mut existing.as_ptr(),
    //         )
    //     }
    // }
    // pub fn insert_at_pos(
    //     &mut self,
    //     key: *mut c_void,
    //     position: *mut c_void,
    // ) -> Option<Entry> {
    //     Entry::new(unsafe {
    //         bindings::dictInsertAtPosition(self.as_mut(), key, position)
    //     })
    // }

    pub fn add_or_find(&mut self, key: *mut c_void) -> Entry {
        Entry::new(unsafe { bindings::dictAddOrFind(self.as_mut(), key) }).unwrap()
    }
    pub fn replace(&mut self, key: *mut c_void, val: *mut c_void) -> i32 {
        unsafe { bindings::dictReplace(self.as_mut(), key, val) }
    }
    pub fn delete(&mut self, key: *const c_void) -> RedisDictResult {
        unsafe { bindings::dictDelete(self.as_mut(), key).into_result() }
    }
    pub fn unlink(&mut self, key: *const c_void) -> Option<Entry> {
        Entry::new(unsafe { bindings::dictUnlink(self.as_mut(), key) })
    }
    pub fn free_unlinked_entry(&mut self, he: &mut Entry) {
        unsafe { bindings::dictFreeUnlinkedEntry(self.as_mut(), he.as_mut()) }
    }
    pub fn find(&mut self, key: *const c_void) -> Option<Entry> {
        Entry::new(unsafe { bindings::dictFind(self.as_mut(), key) })
    }
    pub fn fetch_value(&mut self, key: *const c_void) -> Option<NonNull<c_void>> {
        NonNull::new(unsafe { bindings::dictFetchValue(self.as_mut(), key) })
    }
    pub fn shrink_if_needed(&mut self) -> RedisDictResult {
        unsafe { bindings::dictShrinkIfNeeded(self.as_mut()).into_result() }
    }
    pub fn expand_if_needed(&mut self) -> RedisDictResult {
        unsafe { bindings::dictExpandIfNeeded(self.as_mut()).into_result() }
    }
    pub fn set_key(&mut self, de: &mut Entry, key: *mut c_void) {
        unsafe { bindings::dictSetKey(self.as_mut(), de.as_mut(), key) }
    }
    pub fn set_val(&mut self, de: &mut Entry, val: *mut c_void) {
        unsafe { bindings::dictSetVal(self.as_mut(), de.as_mut(), val) }
    }
    pub fn clear(&mut self) {
        unsafe { bindings::dictEmpty(self.as_mut(), None) }
    }
    pub fn mem_usage(&self) -> usize {
        unsafe { bindings::dictMemUsage(self.as_ref()) }
    }
}

impl Drop for Dict {
    fn drop(&mut self) {
        unsafe { bindings::dictRelease(self.as_mut()) };
    }
}
