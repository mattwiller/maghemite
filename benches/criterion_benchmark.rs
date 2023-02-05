use criterion::{black_box, criterion_group, criterion_main, Criterion};
use maghemite::parser::Lexer;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Lexer/Patient.name.family.replace('er', 'iams')", |b| {
        b.iter(|| {
            let mut lex = Lexer::new(black_box("Patient.name.family.replace('er', 'iams')"));
            lex.tokenize().unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
