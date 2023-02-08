# dashboard

## Benchmark

In your terminal, run the following command to start the benchmark:

NOTE: Benchmark ist the name of the file `benchmark.rs`.

```sh
$ cargo run bench
$ cargo t -p dashboard --bench benchmark
```

This will run the try_main_refactor function 100 times and measure the time it takes for each iteration. The results will be displayed in your terminal. The black_box macro is used to prevent the Rust compiler from optimizing away the try_main_refactor function, as it would not have any side effects.

One option to limit the time required for benchmarking is to reduce the number of iterations. For example, you can change the sample count to 10 instead of 100. This will result in a smaller number of runs and therefore, a faster benchmarking process.

Another option is to reduce the complexity of the function being benchmarked, as this will reduce the amount of time required for each iteration.

Additionally, you can consider adjusting the target time to a smaller value, if appropriate, to further limit the time required for benchmarking.

You can modify the code as follows:

This will run the benchmark with 10 iterations in small batches.
