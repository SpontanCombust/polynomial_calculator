use crate::{operand::Operand, variable::Variable};

// polynomial is a vector representing sum of operands
pub type PolynomialExpression = Vec<Operand>;

pub fn simplify_expression( expr: &mut PolynomialExpression ) {
    if expr.len() == 0 {
        return;
    }

    // first sort the operands for easier grouping
    expr.sort_by( |lhs, rhs| lhs.partial_cmp(rhs).unwrap() );

    // go over operand types in the vector
    let mut i = 0;
    while i < expr.len()-1 {
        match (&expr[i], &expr[i+1]) {
            // if both operands are constants add them together 
            (Operand::CONSTANT(lhs), Operand::CONSTANT(rhs)) => {
                expr[i] = Operand::CONSTANT(lhs + rhs);
                expr.remove(i+1);
            }
            // if they're both variables then attempt to couple them together if possible
            (Operand::VARIABLE(lhs), Operand::VARIABLE(rhs)) => {
                if lhs.exp == rhs.exp {
                    expr[i] = Operand::VARIABLE( Variable::new(
                        lhs.mult + rhs.mult, 
                        lhs.exp
                    ));
                    expr.remove(i+1);
                }
                else {
                    i += 1;
                }
            }
            // if one is a constant and other a variable we can't do nothing, so move onto next pair
            _ => i += 1
        }
    }

    // get rid of trailing zero constant
    match &expr[ expr.len() - 1 ] {
        Operand::CONSTANT(c) => {
            if c == &0.0 {
                expr.pop();
            }
        }
        _ => {}
    }
}

pub fn multiply_expressions( expr1: &PolynomialExpression, expr2: &PolynomialExpression ) -> PolynomialExpression {
    let mut product = PolynomialExpression::new();

    // safe guarding against empty expressions
    if expr1.len() == 0 && expr2.len() == 0 {
        return product;
    }
    else if expr1.len() == 0 && expr2.len() != 0 {
        product.extend_from_slice(expr2);
        return product;
    }
    else if expr1.len() != 0 && expr2.len() == 0 {
        product.extend_from_slice(expr1);
        return product;
    }

    for i in 0..expr1.len() {
        for j in 0..expr2.len() {
            match (&expr1[i], &expr2[j]) {
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

pub fn expression_to_string( expr: &PolynomialExpression ) -> String {
    let mut s = String::new();

    if expr.len() == 0 {
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
    match &expr[0] {
        Operand::CONSTANT(c) => s.push_str(&c.to_string()),
        Operand::VARIABLE(v) => s.push_str(&v.to_string())
    }

    s.push(' ');

    for i in 1..expr.len() {
        match &expr[i] {
            Operand::CONSTANT(c) => {
                if c != &0.0 {
                    s.push( num_to_sgn( c.to_owned() ) );
                    s.push(' ');
                    s.push_str( &c.abs().to_string() );
                    s.push(' ');
                }
            }
            Operand::VARIABLE(v) => {
                s.push( num_to_sgn( v.mult ) );
                s.push(' ');
                s.push_str( &v.abs().to_string() );
                s.push(' ');
            }
        }
    }

    s
}