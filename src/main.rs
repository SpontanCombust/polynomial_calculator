mod variable;
mod operand;
mod expression;
mod interpolation;

use float_cmp::approx_eq;

use crate::interpolation::interpolate_lagrange;
use crate::expression::expression_to_string;

fn test_lagrange() {
    let x = [1.0, 2.0, 3.0, 4.0];
    let y = [3.0, 1.0, -1.0, 2.0];
    let poly = interpolate_lagrange(&x, &y);

    assert_eq!( poly.len(), 3 );

    match &poly[0] {
        operand::Operand::CONSTANT(_) => panic!(),
        operand::Operand::VARIABLE(v) => {
            assert!( approx_eq!( f32, v.mult, 5.0/6.0, ulps = 5 ) );
            assert_eq!(v.exp, 3);
        }
    }

    match &poly[1] {
        operand::Operand::CONSTANT(_) => panic!(),
        operand::Operand::VARIABLE(v) => {
            assert!( approx_eq!( f32, v.mult, -5.0, ulps = 5 ) );
            assert_eq!(v.exp, 2);
        }
    }

    match &poly[2] {
        operand::Operand::CONSTANT(_) => panic!(),
        operand::Operand::VARIABLE(v) => {
            assert!( approx_eq!( f32, v.mult, 43.0/6.0, ulps = 5 ) );
            assert_eq!(v.exp, 1);
        }
    }

    print!( "{}\n", expression_to_string(&poly) );
}

fn main() {
    test_lagrange();
}
