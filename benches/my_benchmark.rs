// use criterion::black_box;
use criterion::Criterion;
use criterion::*;

use icfp2019::prelude::*;

// fn fibonacci(n: u64) -> u64 {
//     match n {
//         0 => 1,
//         1 => 1,
//         n => fibonacci(n - 1) + fibonacci(n - 2),
//     }
// }

fn system_solve(id: u64) -> Result<()> {
    icfp2019::run::run_benchmark(id)
}

// fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
// }

// fn criterion_benchmark2(c: &mut Criterion) {
//     static KB: usize = 1024;
//     static INPUTS: [usize; 5] = [KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB];

//     // Criterion::default()
//     c.bench_function_over_inputs(
//         "from_elem",
//         |b, &&size| {
//             b.iter(|| std::iter::repeat(0u8).take(size).collect::<Vec<_>>());
//         },
//         // &[KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB],
//         &INPUTS,
//     );
// }

// fn system_benchmark_1(c: &mut Criterion) {
//     c.bench_function("system 1", |b| b.iter(|| system_solve(1)));
// }

fn system_benchmark(c: &mut Criterion) {
    static INPUTS: [u64; 4] = [1, 2, 21, 100];
    c.bench_function_over_inputs(
        "system",
        |b, &&size| {
            b.iter(|| system_solve(size));
        },
        &INPUTS,
    );
}

// criterion_group!(benches, criterion_benchmark, criterion_benchmark2);
// criterion_group!(benches, system_benchmark_1, system_benchmark_2);
criterion_group!(benches, system_benchmark);
criterion_main!(benches);
