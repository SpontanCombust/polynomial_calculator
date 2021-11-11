mod variable;
mod operand;
mod expression;

use crate::operand::Operand;
use crate::expression::{PolynomialExpression, expression_to_string, multiply_expressions};
use crate::variable::Variable;

fn main() {
    let p1 : PolynomialExpression = vec![ 
        Operand::VARIABLE( Variable::new( 1.0, 1 ) ),
        Operand::CONSTANT(-2.0)
    ];
    let p2 : PolynomialExpression = vec![ 
        Operand::VARIABLE( Variable::new( 1.0, 1 ) ),
        Operand::CONSTANT(-3.0)
    ];
    let p3 : PolynomialExpression = vec![ 
        Operand::VARIABLE( Variable::new( 1.0, 1 ) ),
        Operand::CONSTANT(-4.0)
    ];

    let p4 = multiply_expressions( &multiply_expressions( &p1, &p2 ), &p3 );
    print!( "{}\n", expression_to_string(&p4) );
}
