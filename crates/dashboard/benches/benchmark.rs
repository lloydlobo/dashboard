//! `benchmark`
//!
//! The black_box macro is used to prevent the compiler from optimizing out the function call in the
//! loop, as it can make the results of the benchmark unreliable.
//!
//! Times a `routine` that requires some input by generating a batch of input, then timing the
//! iteration of the benchmark over the input. See [`BatchSize`](https://docs.rs/criterion/0.4.0/criterion/bencher/enum.BatchSize.html) for
//! details on choosing the batch size. Use this when the routine must consume its input.
//!
//! `iter_custom` Times a `routine` by executing it many times and relying on `routine` to measure
//! its own execution time.

use std::time::Instant;

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use dashboard::app::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref TRY_MAIN_REFACTOR: &'static str = stringify!(try_main_refactor);
}

fn bench_try_main_ref(c: &mut Criterion) {
    let id = format!("{}_ref", *TRY_MAIN_REFACTOR);
    c.bench_function(&id, |b| {
        b.iter(|| {
            let _ = black_box(dashboard::app::try_main_refactor());
        })
    });
}

fn bench_try_main_ref_batchsize(c: &mut Criterion) {
    let id = format!("{}_ref_batchsize", *TRY_MAIN_REFACTOR);
    c.bench_function(&id, |b| {
        b.iter_batched(
            || (),
            |_| {
                let _ = black_box(try_main_refactor());
            },
            BatchSize::SmallInput,
        );
    });
}

// Custom, the timing model is whatever is returned as the Duration from `routine`.
#[allow(clippy::all)]
fn bench_try_main_ref_iter_custom(c: &mut Criterion) {
    let id = format!("{}_ref_iter_custom", *TRY_MAIN_REFACTOR);
    c.bench_function(&id, move |b| {
        b.iter_custom(|iters: u64| {
            let start = Instant::now();
            for _i in 0..iters {
                let _ = black_box(try_main_refactor());
                // black_box(try_main_refactor().expect( "Should run function and return Duration
                // due to `iter_custom` benchmarking",));
            }
            start.elapsed()
        })
    });
}

criterion_group!(
    benches,
    bench_try_main_ref,
    bench_try_main_ref_batchsize,
    bench_try_main_ref_iter_custom,
);
criterion_main!(benches);

/// This macro takes an expression $func as an argument and returns the string representation of
/// that expression using the stringify! macro. You can use the stringify! macro to format a
/// function's name as a string in Rust. Here's an example of how to use it:
///
/// # Example
///
/// ```rust,ignore
/// fn example_function() {
///     println!("Function name: {}", fn_name_as_str!(example_function));
/// }
/// ```
#[allow(unused)]
macro_rules! fn_name_as_str {
    ($func:expr) => {
        stringify!($func)
    };
}

// pub fn iter_custom<F>(bb: &mut Bencher, inner: F)
// where
//     F: FnMut(&mut Bencher) + 'static,
// {
//     // c.bench_function("try_main_refactor", |b| {
//     //     b.iter_custom(|bencher| {
//     //         bencher.iter(
//     //             || {
//     //                 let _ = black_box(try_main_refactor());
//     //             },
//     //             10,
//     //         );
//     //     });
//     // });
//     //
//     // c.bench_function("try_main_refactor", |b| {
//     //     b.iter_custom(|bencher| {
//     //         bencher.iter(
//     //             || {
//     //                 let _ = black_box(try_main_refactor());
//     //             },
//     //             10,
//     //         );
//     //     });
//     // });
// }
