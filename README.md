# [⛄ Advent Of Code 2024 ⛄](https://adventofcode.com/2024)

This year, I'm doing the advent of code using Rust.

## Usage

You can run my solutions by running the following commands:

```sh
cd day<day_number>
cargo run
```

If needed you can use the `--release` flag to run the optimized version of the code.

To run a specific part, use the following commands:

```sh
cargo run -- --part1  # Run part 1 only
cargo run -- --part2  # Run part 2 only
```

If no arguments are provided, both parts will be run by default.

In the output, you will see the answers and the time it took for each part to run.

## Benchmarks

By default, when running the code, a benchmark for the time will be run as well. However this approach is very simple and not very accurate. If you want to run a more accurate benchmark, you can use the following command:

```sh
cargo bench
```

This will run benchmarks using the `criterion` crate and will give you a more accurate time for each part.

## Solutions

The solutions for each day can be found in the following table. The time are being calculated using the `criterion` crate for accurate benchmarks.

| 🗓️Day🗓️ | ⛄Part 1 Solution⛄ | 🎁Part 2 Solution🎁 | ❄️Part 1 Time❄️ | 🎄Part 2 Time🎄 |
|:-------:|:------------------:|:------------------:|:--------------:|:--------------:|
| 🗓️1🗓️ | ⛄[/day1/src/part1.rs](/day1/src/part1.rs)⛄ | 🎁[/day1/src/part2.rs](/day1/src/part2.rs)🎁 | ❄️106.88 µs❄️ | 🎄122.75 µs🎄 |

## Template

Do you want to use my rust template for your own solutions? 

Then just get the `template` folder from this repository and use the following command to create a new day:

```sh
cargo generate --path ./template
```

Then it will ask you for the project name, just enter the name you want to give for the day and it will create a new folder with the template code for you.

## Author

⛄ [ALXS](https://github.com/ALXS-GitHub)
