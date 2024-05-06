use std::{os::raw::c_void, ptr::NonNull};

use crate::bindings;

pub struct Entry {
    inner: NonNull<bindings::dictEntry>,
}

impl Entry {
    pub fn new(ptr: *mut bindings::dictEntry) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { inner: ptr })
    }

    pub fn as_ref(&self) -> &bindings::dictEntry {
        unsafe { self.inner.as_ref() }
    }
    pub fn as_mut(&mut self) -> &mut bindings::dictEntry {
        unsafe { self.inner.as_mut() }
    }
    pub fn as_ptr(self) -> *mut bindings::dictEntry {
        self.inner.as_ptr()
    }

    pub fn set_i64_val(&mut self, val: i64) {
        unsafe { bindings::dictSetSignedIntegerVal(self.as_mut(), val) }
    }

    pub fn set_u64_val(&mut self, val: u64) {
        unsafe { bindings::dictSetUnsignedIntegerVal(self.as_mut(), val) }
    }

    pub fn set_f64_val(&mut self, val: f64) {
        unsafe { bindings::dictSetDoubleVal(self.as_mut(), val) }
    }

    pub fn incr_i64_val(&mut self, val: i64) -> i64 {
        unsafe { bindings::dictIncrSignedIntegerVal(self.as_mut(), val) }
    }

    pub fn incr_u64_val(&mut self, val: u64) -> u64 {
        unsafe { bindings::dictIncrUnsignedIntegerVal(self.as_mut(), val) }
    }

    pub fn incr_f64_val(&mut self, val: f64) -> f64 {
        unsafe { bindings::dictIncrDoubleVal(self.as_mut(), val) }
    }

    pub fn get_key(&self) -> *mut c_void {
        unsafe { bindings::dictGetKey(self.as_ref()) }
    }

    pub fn get_val(&self) -> *mut c_void {
        unsafe { bindings::dictGetVal(self.as_ref()) }
    }

    pub fn get_i64_val(&self) -> i64 {
        unsafe { bindings::dictGetSignedIntegerVal(self.as_ref()) }
    }

    pub fn get_u64_val(&self) -> u64 {
        unsafe { bindings::dictGetUnsignedIntegerVal(self.as_ref()) }
    }

    pub fn get_f64_val(&self) -> f64 {
        unsafe { bindings::dictGetDoubleVal(self.as_ref()) }
    }

    pub fn get_f64_val_ptr(&mut self) -> *mut f64 {
        unsafe { bindings::dictGetDoubleValPtr(self.as_mut()) }
    }
    pub fn mem_usage() -> usize {
        unsafe { bindings::dictEntryMemUsage() }
    }
}
