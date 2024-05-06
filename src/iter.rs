use std::ptr::NonNull;

use crate::{bindings, dict::Dict, entry::Entry};

pub struct RedisDictIterator {
    inner: NonNull<bindings::dictIterator>,
}

impl RedisDictIterator {
    pub fn new(ptr: *mut bindings::dictIterator) -> Self {
        Self {
            inner: unsafe { NonNull::new_unchecked(ptr) },
        }
    }
    fn next(&mut self) -> Option<Entry> {
        unsafe { Entry::new(bindings::dictNext(self.as_mut())) }
    }
    fn as_mut(&mut self) -> &mut bindings::dictIterator {
        unsafe { self.inner.as_mut() }
    }
    // fn as_ref(&self) -> &bindings::dictIterator {
    //     unsafe { self.inner.as_ref() }
    // }
    // fn as_ptr(self) -> *mut bindings::dictIterator {
    //     self.inner.as_ptr()
    // }
}

impl Drop for RedisDictIterator {
    fn drop(&mut self) {
        unsafe { bindings::dictReleaseIterator(self.as_mut()) }
    }
}

pub struct IterMut {
    it: RedisDictIterator,
    len: usize,
}

impl Iterator for IterMut {
    type Item = Entry;
    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.len -= 1;
        }
        self.it.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl ExactSizeIterator for IterMut {
    fn len(&self) -> usize {
        self.len
    }
}

impl Dict {
    pub fn iter_mut(&mut self) -> IterMut {
        IterMut {
            it: unsafe { RedisDictIterator::new(bindings::dictGetSafeIterator(self.as_mut())) },
            len: self.len(),
        }
    }
}

impl IntoIterator for &mut Dict {
    type IntoIter = IterMut;
    type Item = Entry;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
