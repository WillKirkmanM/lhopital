#[derive(Debug, Clone)]
enum Expression {
    Constant(f64),
    Variable, 
    Sum(Box<Expression>, Box<Expression>),
    Difference(Box<Expression>, Box<Expression>),
    Product(Box<Expression>, Box<Expression>),
    Power(Box<Expression>, f64),
}

impl Expression {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            Expression::Constant(c) => *c,
            Expression::Variable => x,
            Expression::Sum(a, b) => a.evaluate(x) + b.evaluate(x),
            Expression::Difference(a, b) => a.evaluate(x) - b.evaluate(x),
            Expression::Product(a, b) => a.evaluate(x) * b.evaluate(x),
            Expression::Power(base, exp) => base.evaluate(x).powf(*exp),
        }
    }

    fn differentiate(&self) -> Self {
        match self {
            // d/dx(c) = 0
            Expression::Constant(_) => Expression::Constant(0.0),
            // d/dx(x) = 1
            Expression::Variable => Expression::Constant(1.0),
            // d/dx(f+g) = f' + g'
            Expression::Sum(a, b) => Expression::Sum(
                Box::new(a.differentiate()),
                Box::new(b.differentiate()),
            ),
            // d/dx(f-g) = f' - g'
            Expression::Difference(a, b) => Expression::Difference(
                Box::new(a.differentiate()),
                Box::new(b.differentiate()),
            ),
            // d/dx(x^n) = n*x^(n-1) (Power Rule simplified for this example)
            Expression::Power(base, exp) => {
                if let Expression::Variable = **base {
                    Expression::Product(
                        Box::new(Expression::Constant(*exp)),
                        Box::new(Expression::Power(base.clone(), exp - 1.0)),
                    )
                } else {
                    panic!("Differentiation for this power expression is not implemented.");
                }
            }
            _ => panic!("Differentiation rule not implemented for this expression."),
        }
    }
}

fn lhopital_solve(
    numerator: &Expression,
    denominator: &Expression,
    at: f64,
    max_iterations: u32,
) -> Result<f64, String> {
    let mut num = numerator.clone();
    let mut den = denominator.clone();

    for i in 0..max_iterations {
        println!("Iteration {}:", i);
        println!("  Numerator: {:?}", num);
        println!("  Denominator: {:?}", den);

        let num_val = num.evaluate(at);
        let den_val = den.evaluate(at);

        println!("  Evaluated at x = {}: {:.4} / {:.4}", at, num_val, den_val);

        if num_val.abs() < 1e-9 && den_val.abs() < 1e-9 {
             println!("  Result is 0/0. Applying L'Hôpital's Rule.");
            num = num.differentiate();
            den = den.differentiate();
        } else if den_val.abs() < 1e-9 {
            return Err(String::from("Limit results in division by zero."));
        } else {
            println!("  Limit found.");
            return Ok(num_val / den_val);
        }
    }

    Err(String::from(
        "Exceeded max iterations, could not find a determinate form.",
    ))
}

fn main() {
    // We want to find the limit as x -> 2 of (x^2 - 4) / (x - 2)

    // Numerator: x^2 - 4
    let numerator = Expression::Difference(
        Box::new(Expression::Power(Box::new(Expression::Variable), 2.0)),
        Box::new(Expression::Constant(4.0)),
    );

    // Denominator: x - 2
    let denominator = Expression::Difference(
        Box::new(Expression::Variable),
        Box::new(Expression::Constant(2.0)),
    );

    let limit_point = 2.0;

    println!(
        "Calculating limit of f(x) = (x^2 - 4) / (x - 2) as x -> {}\n",
        limit_point
    );

    match lhopital_solve(&numerator, &denominator, limit_point, 5) {
        Ok(result) => println!("\nFinal Result: {}", result),
        Err(e) => eprintln!("\nError: {}", e),
    }
}