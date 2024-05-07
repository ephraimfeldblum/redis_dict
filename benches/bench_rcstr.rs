use criterion::{black_box, criterion_group, criterion_main, Criterion};

use hashbrown::{hash_map::DefaultHashBuilder, HashMap};
use redis_dict::{allocator::TrackingAllocator, dict::Dict, dicttypes::*};
use std::rc::Rc;

fn rcstr_insertion_redis(load: u64) -> Dict {
    let mut d = Dict::new(&RCSTR_DICT_TYPE);
    d.expand(load as _).unwrap();
    for i in 0..load {
        let key: Rc<str> = i.to_string().into();
        d.add(&key as *const _ as _, unsafe { std::mem::transmute(i) })
            .unwrap();
    }
    d
}

fn rcstr_insertion_std(load: u64) -> HashMap<Rc<str>, u64, DefaultHashBuilder, TrackingAllocator> {
    let mut d = HashMap::with_capacity_in(
        load as _,
        TrackingAllocator::new(std::mem::size_of::<
            HashMap<Rc<str>, u64, DefaultHashBuilder, TrackingAllocator>,
        >()),
    );
    for i in 0..load {
        let key: Rc<str> = i.to_string().into();
        d.insert(key, i);
    }
    d
}

fn rcstr_iteration_redis(dict: &mut Dict) -> u64 {
    dict.iter_mut().fold(0, |x, entry| {
        let val = entry.get_u64_val();
        x + val
    })
}

fn rcstr_iteration_std(
    dict: &mut HashMap<Rc<str>, u64, DefaultHashBuilder, TrackingAllocator>,
) -> u64 {
    dict.iter_mut().fold(0, |x, entry| {
        let val = *entry.1;
        x + val
    })
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("redis-dict Rc<str> insertion", |b| {
        b.iter(|| rcstr_insertion_redis(black_box(1024)));
    });
    c.bench_function("std::hashmap Rc<str> insertion", |b| {
        b.iter(|| rcstr_insertion_std(black_box(1024)));
    });
    c.bench_function("redis-dict Rc<str> iteration", |b| {
        let mut d = rcstr_insertion_redis(1024);
        b.iter(|| rcstr_iteration_redis(black_box(&mut d)));
    });
    c.bench_function("std::hashmap Rc<str> iteration", |b| {
        let mut d = rcstr_insertion_std(1024);
        b.iter(|| rcstr_iteration_std(black_box(&mut d)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
