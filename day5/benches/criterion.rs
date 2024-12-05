use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day5::{read_input, part1, part2};

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

criterion_group!(benches, benchmark_part1, benchmark_part2);
criterion_main!(benches);