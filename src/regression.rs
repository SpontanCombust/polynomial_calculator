use crate::{expression::PolynomialExpression, operand::Operand, variable::Variable};

fn calc_s( x: &[f32], m: i32 ) -> Vec<f32> {
    let mut s = Vec::new();

    for i in 0..2*m+1 {
        let sum : f32 = x.into_iter().map( |xi| xi.powi(i) ).sum();
        s.push(sum);
    }

    s
}

fn calc_t( x: &[f32], y: &[f32], m: i32 ) -> Vec<f32> {
    let mut t = Vec::new();

    for i in 0..m+1 {
        let mut sum = 0.0;
        for j in 0..y.len() {
            sum += x[j].powi(i) * y[j];
        }
        t.push(sum);
    }

    t
}

/// Returns polynomial expression which resulted from least squares regression from given points
/// This version only calculates for linear function
pub fn regress_least_squares( x: &[f32], y: &[f32] ) -> PolynomialExpression {
    assert_eq!(x.len(), y.len(), "Vectors for X and Y coordinates must have same size!" );

    let mut q = PolynomialExpression::new();
    
    let s = calc_s(x, 1);
    let t = calc_t(x, y, 1);
    
    let w = s[0] * s[2] - s[1] * s[1];
    let wa0 = t[0] * s[2] - t[1] * s[1];
    let wa1 = s[0] * t[1] - s[1] * t[0];

    q.push( Operand::VARIABLE( Variable::new( wa1 / w, 1 ) ) );
    q.push( Operand::CONSTANT( wa0 / w ) );

    q
}