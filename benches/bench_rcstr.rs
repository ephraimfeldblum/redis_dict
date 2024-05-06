use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};
// use iai;
// use anyhow;
// use benchmark_rs::{benchmarks::Benchmarks, stopwatch::StopWatch};

use redis_dict::{dict::Dict, dicttypes::*};
use std::rc::Rc;

fn bench_rcstr_basic(load: u64) {
    let val = "asdsasd" as *const _ as _;
    let mut d = Dict::new(RCSTR_DICT_TYPE);
    for i in 0..load {
      let key: Rc<str> = i.to_string().into();
      d.add(&key as *const _ as _, val).unwrap();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Rc<str> basic", |b| {
        b.iter(|| bench_rcstr_basic(black_box(64)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

// iai::main!(bench_rcstr_basic);

// fn benchmarkrs_rcstr_basic(
//     stop_watch: &mut StopWatch,
//     _config: String,
//     work: i64,
// ) -> Result<(), anyhow::Error> {
//     stop_watch.pause();
//     let val = "asdsasd" as *const _ as _;
//     let mut d = Dict::new(RCSTR_DICT_TYPE);
//     for i in 0..work {
//         let key: Rc<str> = i.to_string().into();
//         stop_watch.resume();
//         d.add(&key as *const _ as _, val).unwrap();
//         stop_watch.pause();
//     }

//     Ok(())
// }

// fn main() -> Result<(), anyhow::Error> {
//     let mut bench = Benchmarks::new("redis-dict benchmarks");
//     bench.add(
//         "add Rc<str>",
//         benchmarkrs_rcstr_basic,
//         "No Configuration".to_owned(),
//         (1..10).collect(),
//         2,
//         1,
//     )?;
//     bench.run()?;

//     let summary = bench.summary_as_json();
//     println!("Summary: {summary}");

//     Ok(())
// }
