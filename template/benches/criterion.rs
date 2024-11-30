use criterion::{black_box, criterion_group, criterion_main, Criterion};
use {{project-name}}::{read_input, part1, part2};

fn benchmark_part1(c: &mut Criterion) {
    let input = read_input("input.txt");
    c.bench_function("part1", |b| b.iter(|| part1::part1(black_box(&input))));
}

fn benchmark_part2(c: &mut Criterion) {
    let input = read_input("input.txt");
    c.bench_function("part2", |b| b.iter(|| part2::part2(black_box(&input))));
}

criterion_group!(benches, benchmark_part1, benchmark_part2);
criterion_main!(benches);