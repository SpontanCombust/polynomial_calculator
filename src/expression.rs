use crate::{operand::Operand, variable::Variable};

// polynomial is a vector representing sum of operands
pub type PolynomialExpression = Vec<Operand>;

pub fn simplify_expression( exp: &mut PolynomialExpression ) {
    if exp.len() == 0 {
        return;
    }

    // first sort the operands for easier grouping
    exp.sort_by( |lhs, rhs| lhs.partial_cmp(rhs).unwrap() );

    // go over operand types in the vector
    let mut i = 0;
    while i < exp.len()-1 {
        match (&exp[i], &exp[i+1]) {
            // if both operands are constants add them together 
            (Operand::CONSTANT(lhs), Operand::CONSTANT(rhs)) => {
                exp[i] = Operand::CONSTANT(lhs + rhs);
                exp.remove(i+1);
            }
            // if they're both variables then attempt to couple them together if possible
            (Operand::VARIABLE(lhs), Operand::VARIABLE(rhs)) => {
                if lhs.exp == rhs.exp {
                    exp[i] = Operand::VARIABLE( Variable::new(
                        lhs.mult + rhs.mult, 
                        lhs.exp
                    ));
                    exp.remove(i+1);
                }
            }
            // if one is a constant and other a variable we can't do nothing
            _ => {}
        }

        i += 1;
    }
}

pub fn multiply_expressions( exp1: &PolynomialExpression, exp2: &PolynomialExpression ) -> PolynomialExpression {
    let mut product = PolynomialExpression::new();

    // safe guarding against empty expressions
    if exp1.len() == 0 && exp2.len() == 0 {
        return product;
    }
    else if exp1.len() == 0 && exp2.len() != 0 {
        product.copy_from_slice(exp2);
        return product;
    }
    else if exp1.len() != 0 && exp2.len() == 0 {
        product.copy_from_slice(exp1);
        return product;
    }

    for i in 0..exp1.len() {
        for j in 0..exp2.len() {
            match (&exp1[i], &exp2[j]) {
                (Operand::CONSTANT(lhs), Operand::CONSTANT(rhs)) => {
                    product.push( Operand::CONSTANT( lhs * rhs ));
                }
                (Operand::CONSTANT(lhs), Operand::VARIABLE(rhs)) => {
                    product.push( Operand::VARIABLE( lhs.to_owned() * rhs.to_owned() ));
                }
                (Operand::VARIABLE(lhs), Operand::CONSTANT(rhs)) => {
                    product.push( Operand::VARIABLE( lhs.to_owned() * rhs.to_owned() ));
                }
                (Operand::VARIABLE(lhs), Operand::VARIABLE(rhs)) => {
                    product.push( Operand::VARIABLE( lhs.to_owned() * rhs.to_owned() ));
                }
            }
        }
    }

    simplify_expression(&mut product);

    product
}

pub fn expression_to_string( exp: &PolynomialExpression ) -> String {
    let mut s = String::new();

    if exp.len() == 0 {
        return s;
    }


    fn num_to_sgn( n: f32 ) -> char {
        if n > 0.0 {
            return '+';
        }
        else {
            return '-'
        }
    }


    // first variable has to have the minus adjoint to the variable
    match &exp[0] {
        Operand::CONSTANT(c) => s.push_str(&c.to_string()),
        Operand::VARIABLE(v) => s.push_str(&v.to_string())
    }

    s.push(' ');

    for i in 1..exp.len() {
        match &exp[i] {
            Operand::CONSTANT(c) => {
                s.push( num_to_sgn( c.to_owned() ) );
                s.push(' ');
                s.push_str( &c.abs().to_string() );
            }
            Operand::VARIABLE(v) => {
                s.push( num_to_sgn( v.mult ) );
                s.push(' ');
                s.push_str( &v.abs().to_string() );
            }
        }
        s.push(' ');
    }

    s
}