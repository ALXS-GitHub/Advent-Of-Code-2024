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

Make sure that the test input provided in the problem is set in the `inputtest.txt` file in the `day<day_number>` folder, and the real input is set in the `input.txt` file.

## Benchmarks

By default, when running the code, a benchmark for the time will be run as well. However this approach is very simple and not very accurate. If you want to run a more accurate benchmark, you can use the following command:

```sh
cargo bench
```

This will run benchmarks using the `criterion` crate and will give you a more accurate time for each part.

## Tests

Unit tests are provided in the `lib.rs` file for each day. Make sure you set the correct result for each test before running them.
There are both test for the test input provided in the problem and for the real input.

To run the tests, use the following command:

```sh
cargo test --lib
```

## Solutions

The solutions for each day can be found in the following table. The time are being calculated using the `criterion` crate for accurate benchmarks. Note that most of the challenges are done without any parallelism, except for some days where I judged it was necessary to get faster results.

| 🗓️Day🗓️ | ⛄Part 1 Solution⛄ | 🎁Part 2 Solution🎁 | ❄️Part 1 Time❄️ | 🎄Part 2 Time🎄 |
|:-------:|:------------------:|:------------------:|:--------------:|:--------------:|
| 🗓️1🗓️ | ⛄[/day1/src/part1.rs](/day1/src/part1.rs)⛄ | 🎁[/day1/src/part2.rs](/day1/src/part2.rs)🎁 | ❄️106.88 µs❄️ | 🎄122.75 µs🎄 |
| 🗓️2🗓️ | ⛄[/day2/src/part1.rs](/day2/src/part1.rs)⛄ | 🎁[/day2/src/part2.rs](/day2/src/part2.rs)🎁 | ❄️160.89 µs❄️ | 🎄368.50 µs🎄 |
| 🗓️3🗓️ | ⛄[/day3/src/part1.rs](/day3/src/part1.rs)⛄ | 🎁[/day3/src/part2.rs](/day3/src/part2.rs)🎁 | ❄️1.4189 ms❄️ | 🎄1.4674 ms🎄 |
| 🗓️4🗓️ | ⛄[/day4/src/part1.rs](/day4/src/part1.rs)⛄ | 🎁[/day4/src/part2.rs](/day4/src/part2.rs)🎁 | ❄️6.4391 ms❄️ | 🎄4.3089 ms🎄 |
| 🗓️5🗓️ | ⛄[/day5/src/part1.rs](/day5/src/part1.rs)⛄ | 🎁[/day5/src/part2.rs](/day5/src/part2.rs)🎁 | ❄️1.6586 ms❄️ | 🎄1.4714 ms🎄 |
| 🗓️6🗓️ | ⛄[/day6/src/part1.rs](/day6/src/part1.rs)⛄ | 🎁[/day6/src/part2.rs](/day6/src/part2.rs)🎁 | ❄️590.36 µs❄️ | 🎄262.13 ms🎄 |
| 🗓️7🗓️ | ⛄[/day7/src/part1.rs](/day7/src/part1.rs)⛄ | 🎁[/day7/src/part2.rs](/day7/src/part2.rs)🎁 | ❄️24.198 ms❄️ | 🎄101.33 ms🎄 |
| 🗓️8🗓️ | ⛄[/day8/src/part1.rs](/day8/src/part1.rs)⛄ | 🎁[/day8/src/part2.rs](/day8/src/part2.rs)🎁 | ❄️49.307 µs❄️ | 🎄56.206 µs🎄 |
| 🗓️9🗓️ | ⛄[/day9/src/part1.rs](/day9/src/part1.rs)⛄ | 🎁[/day9/src/part2.rs](/day9/src/part2.rs)🎁 | ❄️342.47 µs❄️ | 🎄95.048 ms🎄 |
| 🗓️10🗓️ | ⛄[/day10/src/part1.rs](/day10/src/part1.rs)⛄ | 🎁[/day10/src/part2.rs](/day10/src/part2.rs)🎁 | ❄️291.40 µs❄️ | 🎄258.84 µs🎄 |
| 🗓️11🗓️ | ⛄[/day11/src/part1.rs](/day11/src/part1.rs)⛄ | 🎁[/day11/src/part2.rs](/day11/src/part2.rs)🎁 | ❄️6.0565 ms❄️ | 🎄18.222 ms🎄 |
| 🗓️12🗓️ | ⛄[/day12/src/part1.rs](/day12/src/part1.rs)⛄ | 🎁[/day12/src/part2.rs](/day12/src/part2.rs)🎁 | ❄️7.1246 ms❄️ | 🎄11.863 ms🎄 |
| 🗓️13🗓️ | ⛄[/day13/src/part1.rs](/day13/src/part1.rs)⛄ | 🎁[/day13/src/part2.rs](/day13/src/part2.rs)🎁 | ❄️123.03 ms❄️ | 🎄113.48 ms🎄 |
| 🗓️14🗓️ | ⛄[/day14/src/part1.rs](/day14/src/part1.rs)⛄ | 🎁[/day14/src/part2.rs](/day14/src/part2.rs)🎁 | ❄️102.45 ms❄️ | 🎄366.34 ms🎄 |
| 🗓️15🗓️ | ⛄[/day15/src/part1.rs](/day15/src/part1.rs)⛄ | 🎁[/day15/src/part2.rs](/day15/src/part2.rs)🎁 | ❄️326.39 µs❄️ | 🎄374.91 µs🎄 |
| 🗓️16🗓️ | ⛄[/day16/src/part1.rs](/day16/src/part1.rs)⛄ | 🎁[/day16/src/part2.rs](/day16/src/part2.rs)🎁 | ❄️506.40 µs❄️ | 🎄1.7908 ms🎄 |
| 🗓️17🗓️ | ⛄[/day17/src/part1.rs](/day17/src/part1.rs)⛄ | 🎁[/day17/src/part2.rs](/day17/src/part2.rs)🎁 | ❄️278.14 µs❄️ | 🎄267.12 µs🎄 |
| 🗓️18🗓️ | ⛄[/day18/src/part1.rs](/day18/src/part1.rs)⛄ | 🎁[/day18/src/part2.rs](/day18/src/part2.rs)🎁 | ❄️322.84 µs❄️ | 🎄660.05 ms🎄 |
| 🗓️19🗓️ | ⛄[/day19/src/part1.rs](/day19/src/part1.rs)⛄ | 🎁[/day19/src/part2.rs](/day19/src/part2.rs)🎁 | ❄️45.136 ms❄️ | 🎄41.763 ms🎄 |
| 🗓️20🗓️ | ⛄[/day20/src/part1.rs](/day20/src/part1.rs)⛄ | 🎁[/day20/src/part2.rs](/day20/src/part2.rs)🎁 | ❄️32.051 ms❄️ | 🎄~2s🎄 |
| 🗓️21🗓️ | ⛄[/day21/src/part1.rs](/day21/src/part1.rs)⛄ | 🎁[/day21/src/part2.rs](/day21/src/part2.rs)🎁 | ❄️847.54 µs❄️ | 🎄630.58 µs🎄 |
| 🗓️22🗓️ | ⛄[/day22/src/part1.rs](/day22/src/part1.rs)⛄ | 🎁[/day22/src/part2.rs](/day22/src/part2.rs)🎁 | ❄️33.926 ms❄️ | 🎄1.7267 s🎄 |

## Template

Do you want to use my rust template for your own solutions? 

Then just get the `template` folder from this repository and use the following command to create a new day:

```sh
cargo generate --path ./template
```

Then it will ask you for the project name, just enter the name you want to give for the day and it will create a new folder with the template code for you.

## Author

⛄ [ALXS](https://github.com/ALXS-GitHub)
