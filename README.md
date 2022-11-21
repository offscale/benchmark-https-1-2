benchmark-https-1-2
===================
[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Tooling to benchmark HTTPS (TLS 1.2) requests.


In `stress/` you'll find a small Rust CLI application that will send HTTP GET
requests concurrently, download the resource and report some statistics to
stdout (as JSON).

## Help text
```sh
> cd stress && cargo run --release -- --help
   Compiling stress v0.1.0 (benchmark-https-1-2/stress)
    Finished release [optimized] target(s) in 4.76s
     Running `target/release/stress --help`
Usage: stress <url> -C <clients> -N <requests>

HTTP benchmark

Positional Arguments:
  url               URL to fetch

Options:
  -C, --clients     number of clients
  -N, --requests    number of total requests
  --help            display usage information
  -V, --version     print version information and exit
```

## Example usage

```sh
> cd stress
> cargo build --release
> ./target/release/stress -N 10 -C 2 https://www.ntecs.de/
```

This will build the `stress` binary and then start it. It will fetch the
`https://www.ntecs.de/` resource using 2 concurrent clients (`-C 2`) in total
10 times (`-N 10`). Each client will request the resouce 5 times.

The output it generates looks like this (this is just an example):

```json
// jq < stress/results/run_1_x1.json
{
  "failed_requests": 0,
  "num_clients": 1,
  "reqs_per_client": 1,
  "started": "2022-11-04T19:47:21.861164407Z",
  "successful_requests": 1,
  "throughput_in_mib": 5.401814642267866,
  "time_to_completion": [
    0.479767029
  ],
  "time_to_first_byte": [
    0.397834181
  ],
  "total_requests": 1,
  "total_runtime": 0.489282078,
  "total_size": 2771398
}
```

## Benchmark scripts

In `stress/` you'll find a couple of benchmark scripts. `stress/gendata.rb`
will take the JSON statistics and create `csv` files which are input to the R
scripts `plot.r` and `plot_stress.r`. These are very primitive R scripts that
generate diagrams.

---

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
