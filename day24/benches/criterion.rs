use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day24::{read_input, part1, part2};

fn benchmark_part1(c: &mut Criterion) {
    let input = read_input("input.txt");
    let result = part1::part1(&input);
    println!("\x1b[34m\x1b[1mPart 1 result: {}\x1b[0m", result);
    c.bench_function("part1", |b| b.iter(|| part1::part1(black_box(&input))));
}

fn benchmark_part2(c: &mut Criterion) {
    let input = read_input("input.txt");
    let result = part2::part2(&input);
    println!("\x1b[34m\x1b[1mPart 2 result: {}\x1b[0m", result);
    c.bench_function("part2", |b| b.iter(|| part2::part2(black_box(&input))));
}


// * part2 is not benchmarked because it the solution was found on the fly with hand made hypothesis
criterion_group!(benches, benchmark_part1);
// criterion_group!(benches, benchmark_part1, benchmark_part2);
criterion_main!(benches);