use criterion::{Criterion, black_box, criterion_group, criterion_main};
use test_dyntrait_vs_enum::*;

fn criterion_benchmark(c: &mut Criterion) {
    // AST for (1+5-9/(4+3))*5-(3+7)*123
    let ast = black_box(Ast::Sub(
        Box::new(Ast::Mul(
            Box::new(Ast::Sub(
                Box::new(Ast::Add(
                    Box::new(Ast::Number(1.0)),
                    Box::new(Ast::Number(5.0)),
                )),
                Box::new(Ast::Div(
                    Box::new(Ast::Number(9.0)),
                    Box::new(Ast::Add(
                        Box::new(Ast::Number(4.0)),
                        Box::new(Ast::Number(3.0)),
                    )),
                )),
            )),
            Box::new(Ast::Number(5.0)),
        )),
        Box::new(Ast::Mul(
            Box::new(Ast::Add(
                Box::new(Ast::Number(3.0)),
                Box::new(Ast::Number(7.0)),
            )),
            Box::new(Ast::Number(123.0)),
        )),
    ));

    eprintln!("Benchmarking expr: '(1.0 + 5.0 - 9.0 / (4.0 + 3.0)) * 5.0 - (3.0 + 7.0) * 123.0'");

    c.bench_function("eval", |b| b.iter(|| eval(&ast)));

    let eval_fn = eval2(&ast);
    std::mem::drop(ast);
    c.bench_function("eval fn", |b| b.iter(|| eval_fn()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
