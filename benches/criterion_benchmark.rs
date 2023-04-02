use criterion::{criterion_group, criterion_main, Criterion};
use maghemite::parser::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Lexer/Patient.name.family.replace('er', 'iams')", |b| {
        b.iter(|| {
            let mut lex = Lexer::new("Patient.name.family.replace('er', 'iams')");
            lex.tokenize().unwrap()
        })
    });

    c.bench_function("Parser/Patient.name.family.replace", |b| {
        b.iter(|| {
            let mut lex = Lexer::new("Patient.name.family.replace");
            let parser = Parser::new(lex.tokenize().unwrap());
            parser.parse()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
