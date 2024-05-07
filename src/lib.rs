
#![feature(allocator_api)]

#[allow(warnings)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(unused)]
#[allow(missing_docs)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod dict;
pub mod dicttypes;
pub mod entry;
pub mod iter;
pub mod allocator;

#[cfg(test)]
mod dict_tests {
    use std::rc::Rc;
    use threadpool::ThreadPool;
    use std::sync::mpsc::channel;
    use crate::{
        dict::*,
        dicttypes::*,
        entry::*,
    };

    #[test]
    fn creation() {
        let dict = Dict::new(&INT_DICT_TYPE);
        assert_eq!(dict.len(), 0);
        assert_eq!(dict.num_buckets(), 0);
        assert_eq!(dict.mem_usage(), 0);
    }
    #[test]
    fn insertion() {
        let mut dict = Dict::new(&INT_DICT_TYPE);
        assert!(dict.add(u64_as_key(0), "hello" as *const _ as _).is_ok());
        assert_eq!(dict.len(), 1);
        assert_ne!(dict.num_buckets(), 0);
        assert_ne!(dict.mem_usage(), 0);
        assert!(dict.add(u64_as_key(1), "world" as *const _ as _).is_ok());
        assert_eq!(dict.len(), 2);
    }
    #[test]
    fn deletion() {
        let mut dict = Dict::new(&INT_DICT_TYPE);
        let hello = "hello" as *const _ as _;
        assert!(dict.add(u64_as_key(0), hello).is_ok());
        assert_eq!(dict.len(), 1);
        assert_ne!(dict.num_buckets(), 0);
        assert_ne!(dict.mem_usage(), 0);
        assert!(dict.delete(u64_as_key(0)).is_ok());
        assert_eq!(dict.len(), 0);
        assert_ne!(dict.num_buckets(), 0);
        assert_ne!(dict.mem_usage(), 0);
        assert!(dict.delete(u64_as_key(0)).is_err());
        assert!(dict.add(u64_as_key(0), hello).is_ok());
    }
    #[test]
    fn clear() {
        let hello = "hello" as *const _ as _;
        let world = "world" as *const _ as _;
        let mut dict = Dict::new(&INT_DICT_TYPE);
        assert!(dict.add(u64_as_key(0), hello).is_ok());
        assert!(dict.add(u64_as_key(1), world).is_ok());
        dict.clear();
        assert_eq!(dict.len(), 0);
        assert_eq!(dict.num_buckets(), 0);
        assert_eq!(dict.mem_usage(), 0);
        assert!(dict.delete(u64_as_key(0)).is_err());
        assert!(dict.add(u64_as_key(0), hello).is_ok());
    }
    #[test]
    fn retrieval() {
        let hello = "hello" as *const _ as _;
        let mut dict = Dict::new(&INT_DICT_TYPE);
        assert!(dict.fetch_value(u64_as_key(0)).is_none());
        assert!(dict.add(u64_as_key(0), hello).is_ok());
        let val = dict.fetch_value(u64_as_key(0));
        assert!(val.is_some());
        assert_eq!(val.unwrap().as_ptr(), hello);
        assert!(dict.delete(u64_as_key(0)).is_ok());
        assert!(dict.fetch_value(u64_as_key(0)).is_none());
    }
    #[test]
    fn basic_mem() {
        let entry_size = Entry::mem_usage();
        let sizeof_entry = std::mem::size_of::<Entry>();
        let mut dict = Dict::new(&INT_DICT_TYPE);
        assert_eq!(0, dict.mem_usage());
        let hello = "hello" as *const _ as _;
        assert!(dict.add(u64_as_key(0), hello).is_ok());
        assert_eq!(1 * entry_size + 4 * sizeof_entry, dict.mem_usage());
        let world = "world" as *const _ as _;
        assert!(dict.add(u64_as_key(1), world).is_ok());
        assert_eq!(2 * entry_size + 4 * sizeof_entry, dict.mem_usage());
        let foo = "foo" as *const _ as _;
        let bar = "bar" as *const _ as _;
        let baz = "baz" as *const _ as _;
        assert!(dict.add(u64_as_key(2), foo).is_ok());
        assert!(dict.add(u64_as_key(3), bar).is_ok());
        assert!(dict.add(u64_as_key(4), baz).is_ok());
        assert_eq!(5 * entry_size + (4 + 8) * sizeof_entry, dict.mem_usage());
        assert!(dict.add(u64_as_key(5), foo).is_ok());
        assert!(dict.add(u64_as_key(6), bar).is_ok());
        assert!(dict.add(u64_as_key(7), baz).is_ok());
        assert!(dict.add(u64_as_key(8), foo).is_ok());
        assert_eq!(9 * entry_size + (8 + 16) * sizeof_entry, dict.mem_usage());
        assert!(dict.add(u64_as_key(9), bar).is_ok());
        assert!(dict.add(u64_as_key(10), baz).is_ok());
        assert!(dict.add(u64_as_key(11), foo).is_ok());
        assert!(dict.add(u64_as_key(12), bar).is_ok());
        assert!(dict.add(u64_as_key(13), baz).is_ok());
        assert!(dict.add(u64_as_key(14), foo).is_ok());
        assert!(dict.add(u64_as_key(15), bar).is_ok());
        assert!(dict.add(u64_as_key(16), baz).is_ok());
        assert_eq!(17 * entry_size + (16 + 32) * sizeof_entry, dict.mem_usage());
    }




    #[test]
    fn rcstr_creation() {
        let dict = Dict::new(&RCSTR_DICT_TYPE);
        assert_eq!(dict.len(), 0);
        assert_eq!(dict.num_buckets(), 0);
        assert_eq!(dict.mem_usage(), 0);
    }
    #[test]
    fn rcstr_insertion() {
        let mut dict = Dict::new(&RCSTR_DICT_TYPE);
        let key0: Rc<str> = "0".into();
        let key1: Rc<str> = "1".into();
        assert!(dict.add(&key0 as *const _ as _, "hello" as *const _ as *const _ as _).is_ok());
        assert_eq!(dict.len(), 1);
        assert_ne!(dict.num_buckets(), 0);
        assert_ne!(dict.mem_usage(), 0);
        assert!(dict.add(&key1 as *const _ as _, "world" as *const _ as *const _ as _).is_ok());
        assert_eq!(dict.len(), 2);
    }
    #[test]
    fn rcstr_deletion() {
        let mut dict = Dict::new(&RCSTR_DICT_TYPE);
        let key: Rc<str> = "0".into();
        let hello = "hello" as *const _ as *const _ as _;
        assert!(dict.add(&key as *const _ as _, hello).is_ok());
        assert_eq!(dict.len(), 1);
        assert_ne!(dict.num_buckets(), 0);
        assert_ne!(dict.mem_usage(), 0);
        assert!(dict.delete(&key as *const _ as _).is_ok());
        assert_eq!(dict.len(), 0);
        assert_ne!(dict.num_buckets(), 0);
        assert_ne!(dict.mem_usage(), 0);
        assert!(dict.delete(&key as *const _ as _).is_err());
        assert!(dict.add(&key as *const _ as _, hello).is_ok());
    }
    #[test]
    fn rcstr_clear() {
        let hello = "hello" as *const _ as *const _ as _;
        let world = "world" as *const _ as *const _ as _;
        let key0: Rc<str> = "0".into();
        let key1: Rc<str> = "1".into();
        let mut dict = Dict::new(&RCSTR_DICT_TYPE);
        assert!(dict.add(&key0 as *const _ as _, hello).is_ok());
        assert!(dict.add(&key1 as *const _ as _, world).is_ok());
        dict.clear();
        assert_eq!(dict.len(), 0);
        assert_eq!(dict.num_buckets(), 0);
        assert_eq!(dict.mem_usage(), 0);
        assert!(dict.delete(&key0 as *const _ as _).is_err());
        assert!(dict.add(&key0 as *const _ as _, hello).is_ok());
    }
    #[test]
    fn rcstr_retrieval() {
        let hello = "hello" as *const _ as *const _ as _;
        let key: Rc<str> = "0".into();
        let mut dict = Dict::new(&RCSTR_DICT_TYPE);
        assert!(dict.fetch_value(&key as *const _ as _).is_none());
        assert!(dict.add(&key as *const _ as _, hello).is_ok());
        let val = dict.fetch_value(&key as *const _ as _);
        assert!(val.is_some());
        assert_eq!(val.unwrap().as_ptr(), hello);
        assert!(dict.delete(&key as *const _ as _).is_ok());
        assert!(dict.fetch_value(&key as *const _ as _).is_none());
    }
    #[test]
    fn rcstr_basic_mem() {
        let entry_size = Entry::mem_usage();
        let sizeof_entry = std::mem::size_of::<Entry>();
        let mut dict = Dict::new(&RCSTR_DICT_TYPE);
        let key: Rc<str> = "0".into();
        assert_eq!(0, dict.mem_usage());
        let hello = "hello" as *const _ as _;
        assert!(dict.add(&key as *const _ as _, hello).is_ok());
        assert_eq!(1 * entry_size + 4 * sizeof_entry, dict.mem_usage());
        let world = "world" as *const _ as _;
        let key: Rc<str> = "1".into();
        assert!(dict.add(&key as *const _ as _, world).is_ok());
        assert_eq!(2 * entry_size + 4 * sizeof_entry, dict.mem_usage());
        let vals = [
            "foo" as *const _ as _,
            "bar" as *const _ as _,
            "baz" as *const _ as _,
        ];
        for i in 2..=4 {
            let key: Rc<str> = i.to_string().into();
            assert!(dict.add(&key as *const _ as _, vals[i % 3]).is_ok());
        }
        assert_eq!(5 * entry_size + (4 + 8) * sizeof_entry, dict.mem_usage());
        for i in 5..=8 {
            let key: Rc<str> = i.to_string().into();
            assert!(dict.add(&key as *const _ as _, vals[i % 3]).is_ok());
        }
        assert_eq!(9 * entry_size + (8 + 16) * sizeof_entry, dict.mem_usage());
        for i in 9..=16 {
            let key: Rc<str> = i.to_string().into();
            assert!(dict.add(&key as *const _ as _, vals[i % 3]).is_ok());
        }
        assert_eq!(17 * entry_size + (16 + 32) * sizeof_entry, dict.mem_usage());
    }

    #[test]
    fn rcstr_multithreaded() {
        let n_workers = 4;
        let n_jobs = 64;
        let pool = ThreadPool::new(n_workers);

        let (tx, _) = channel();
        for _ in 0..n_jobs {
            let tx = tx.clone();
            pool.execute(move|| {
                tx.send(|| {
                    let val = "asdsasd" as *const _ as _;
                    let mut d = Dict::new(&RCSTR_DICT_TYPE);
                    let key: Rc<str> = "0".to_string().into();
                    assert!(d.add(&key as *const _ as _, val).is_ok());
                }).unwrap();
            });
        }
    }
}
