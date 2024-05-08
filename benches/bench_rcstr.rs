use criterion::{black_box, criterion_group, criterion_main, Criterion};

use hashbrown::{hash_map::DefaultHashBuilder, HashMap};
use redis_custom_allocator::MemoryConsumption;
use redis_dict::{allocator::TrackingAllocator, dict::Dict, dicttypes::*};
use std::{rc::Rc, time::Duration};

fn rcstr_insertion_redis(load: u64) -> Dict {
    let mut d = Dict::new(&RCSTR_DICT_TYPE);
    // d.expand(load as _).unwrap();
    for i in 0..load {
        let key: Rc<str> = i.to_string().into();
        d.add(&key as *const _ as _, unsafe { std::mem::transmute(i) })
            .unwrap();
    }
    d
}

fn rcstr_insertion_std(load: u64) -> HashMap<Rc<str>, u64, DefaultHashBuilder, TrackingAllocator> {
    let mut d = HashMap::new_in(
        // load as _,
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

fn large_val_insertion_redis(load: u64) -> Dict {
    let mut d = Dict::new(&LARGE_VAL_DICT_TYPE);
    // d.expand(load as _).unwrap();
    for i in 0..load {
        let key: Rc<str> = i.to_string().into();
        d.add(&key as *const _ as _, Box::into_raw(Box::new([i; 8])) as _)
            .unwrap();
    }
    d
}

fn large_val_insertion_std(
    load: u64,
) -> HashMap<Rc<str>, [u64; 8], DefaultHashBuilder, TrackingAllocator> {
    let mut d = HashMap::new_in(
        // load as _,
        TrackingAllocator::new(std::mem::size_of::<
            HashMap<Rc<str>, [u64; 8], DefaultHashBuilder, TrackingAllocator>,
        >()),
    );
    for i in 0..load {
        let key: Rc<str> = i.to_string().into();
        d.insert(key, [i; 8]);
    }
    d
}

fn large_val_iteration_redis(dict: &mut Dict) -> u64 {
    dict.iter_mut().fold(0, |x, entry| {
        let val = entry.get_val();
        let val = unsafe { *(val as *const [u64; 8]) };
        x + val.iter().sum::<u64>()
    })
}

fn large_val_iteration_std(
    dict: &mut HashMap<Rc<str>, [u64; 8], DefaultHashBuilder, TrackingAllocator>,
) -> u64 {
    dict.iter_mut().fold(0, |x, entry| {
        let val = *entry.1;
        x + val.iter().sum::<u64>()
    })
}

fn criterion_benchmark(c: &mut Criterion) {
    for &n in [1024, 1024 * 1024].iter() {
        let mut g = c.benchmark_group(format!("{n} elements small-val"));
        g.bench_function("redis-dict insertion", |b| {
            b.iter_with_large_drop(|| rcstr_insertion_redis(black_box(n)));
        });
        g.bench_function("hashbrown insertion", |b| {
            b.iter_with_large_drop(|| rcstr_insertion_std(black_box(n)));
        });
        let mut rd = rcstr_insertion_redis(n);
        g.bench_function("redis-dict iteration", |b| {
            b.iter(|| rcstr_iteration_redis(black_box(&mut rd)));
        });
        let mut hd = rcstr_insertion_std(n);
        g.bench_function("hashbrown iteration", |b| {
            b.iter(|| rcstr_iteration_std(black_box(&mut hd)));
        });
        println!(
            "redis-dict mem usage: {}",
            rd.mem_usage()
                + rd.len() * std::mem::size_of::<Rc<str>>()
        );
        println!(
            "hashbrown mem usage: {}",
            { hd.allocator() as &dyn MemoryConsumption }.memory_consumption()
        );
        g.finish();

        let mut g = c.benchmark_group(format!("{n} elements large-val"));
        g.bench_function("redis-dict insertion", |b| {
            b.iter_with_large_drop(|| large_val_insertion_redis(black_box(n)));
        });
        g.bench_function("hashbrown insertion", |b| {
            b.iter_with_large_drop(|| large_val_insertion_std(black_box(n)));
        });
        let mut rd = large_val_insertion_redis(n);
        g.bench_function("redis-dict iteration", |b| {
            b.iter(|| large_val_iteration_redis(black_box(&mut rd)));
        });
        let mut hd = large_val_insertion_std(n);
        g.bench_function("hashbrown iteration", |b| {
            b.iter(|| large_val_iteration_std(black_box(&mut hd)));
        });
        println!(
            "redis-dict mem usage: {}",
            rd.mem_usage()
                + rd.len() * (std::mem::size_of::<Rc<str>>() + std::mem::size_of::<[u64; 8]>())
        );
        println!(
            "hashbrown mem usage: {}",
            { hd.allocator() as &dyn MemoryConsumption }.memory_consumption()
        );
        g.finish();
    }
}

criterion_group! {
  name = benches;
  config = Criterion::default().measurement_time(Duration::from_secs(10));
  targets = criterion_benchmark,
}
criterion_main!(benches);
