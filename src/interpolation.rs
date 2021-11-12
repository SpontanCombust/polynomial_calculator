use crate::{expression::{PolynomialExpression, multiply_expressions, simplify_expression}, operand::Operand, variable::Variable};

/// Returns a polynomial expression that describes a function that may yield given points, done with Lagrange's interpolation polynomial
/// 
/// x - vector of x coordinates
/// 
/// y - vector of y coordinates corresponding to x coordinates
/// 
/// panics if x and y vectors are not the same legth 
pub fn interpolate_lagrange( x: &[f32], y: &[f32] ) -> PolynomialExpression {
    assert_eq!(x.len(), y.len(), "X and Y vectors must have the same legth!");

    let mut w = PolynomialExpression::new();

    for i in 0..x.len() {
        let mut nominator = PolynomialExpression::new();
        let mut denominator = 1.0;
        for j in 0..x.len() {
            if i != j {
                nominator = multiply_expressions(
                    &nominator,
                    &vec![ Operand::VARIABLE( Variable::new(1.0, 1) ), Operand::CONSTANT( -x[j] ) ],
                );
                denominator *= x[i] - x[j];
            }
        }

        let mut nominator = multiply_expressions( &vec![ Operand::CONSTANT( y[i] / denominator ) ], &nominator ) ;

        w.append( &mut nominator );
    }

    simplify_expression(&mut w);

    w
}
