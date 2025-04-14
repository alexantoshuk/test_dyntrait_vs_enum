pub enum Ast {
    Number(f64),
    Add(Box<Ast>, Box<Ast>),
    Sub(Box<Ast>, Box<Ast>),
    Mul(Box<Ast>, Box<Ast>),
    Div(Box<Ast>, Box<Ast>),
    Pow(Box<Ast>, Box<Ast>),
    Neg(Box<Ast>),
    Sin(Box<Ast>),
    Cos(Box<Ast>),
    Tan(Box<Ast>),
}

pub fn eval(ast: &Ast) -> f64 {
    match ast {
        Ast::Number(n) => *n,
        Ast::Add(l, r) => eval(l) + eval(r),
        Ast::Sub(l, r) => eval(l) - eval(r),
        Ast::Mul(l, r) => eval(l) * eval(r),
        Ast::Div(l, r) => eval(l) / eval(r),
        Ast::Pow(l, r) => eval(l).powf(eval(r)),
        Ast::Neg(e) => -eval(e),
        Ast::Sin(e) => eval(e).sin(),
        Ast::Cos(e) => eval(e).cos(),
        Ast::Tan(e) => eval(e).tan(),
    }
}

pub fn eval2(ast: &Ast) -> Box<dyn Fn() -> f64> {
    match ast {
        Ast::Number(n) => {
            let n = *n;
            Box::new(move || n)
        }
        Ast::Add(l, r) => {
            let l = eval2(l);
            let r = eval2(r);
            Box::new(move || l() + r())
        }
        Ast::Sub(l, r) => {
            let l = eval2(l);
            let r = eval2(r);
            Box::new(move || l() - r())
        }
        Ast::Mul(l, r) => {
            let l = eval2(l);
            let r = eval2(r);
            Box::new(move || l() * r())
        }
        Ast::Div(l, r) => {
            let l = eval2(l);
            let r = eval2(r);
            Box::new(move || l() / r())
        }
        Ast::Pow(l, r) => {
            let l = eval2(l);
            let r = eval2(r);
            Box::new(move || l().powf(r()))
        }
        Ast::Neg(e) => {
            let e = eval2(e);
            Box::new(move || -e())
        }
        Ast::Sin(e) => {
            let e = eval2(e);
            Box::new(move || e().sin())
        }
        Ast::Cos(e) => {
            let e = eval2(e);
            Box::new(move || e().cos())
        }
        Ast::Tan(e) => {
            let e = eval2(e);
            Box::new(move || e().tan())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_expression() {
        let expected = (1.0 + 5.0 - 9.0 / (4.0 + 3.0)) * 5.0 - (3.0 + 7.0) * 123.0;
        let ast = Ast::Sub(
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
        );

        let result = eval(&ast);
        assert_eq!(result, expected);

        let eval_fn = eval2(&ast);
        std::mem::drop(ast);
        let result = eval_fn();
        assert_eq!(result, expected);
    }
}
