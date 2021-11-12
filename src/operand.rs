use crate::variable::Variable;

use std::{ cmp::Ordering, ops::Mul };

#[derive(Clone, Copy)]
pub enum Operand {
    CONSTANT(f32),
    VARIABLE(Variable)
}

impl PartialEq for Operand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::CONSTANT(lhs), Self::CONSTANT(rhs)) => lhs == rhs,
            (Self::VARIABLE(lhs), Self::VARIABLE(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

impl PartialOrd for Operand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Operand::CONSTANT(_), Operand::CONSTANT(_)) => Some(Ordering::Less),
            (Operand::CONSTANT(_), Operand::VARIABLE(_)) => Some(Ordering::Greater),
            (Operand::VARIABLE(_), Operand::CONSTANT(_)) => Some(Ordering::Less),
            (Operand::VARIABLE(lhs), Operand::VARIABLE(rhs)) => lhs.partial_cmp(rhs),
        }
    }
}

impl Mul for Operand {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Operand::CONSTANT(c1), Operand::CONSTANT(c2)) => {
                return Operand::CONSTANT( c1 * c2 );
            }
            (Operand::CONSTANT(c), Operand::VARIABLE(v)) => {
                return Operand::VARIABLE( c * v );
            }
            (Operand::VARIABLE(v), Operand::CONSTANT(c)) => {
                return Operand::VARIABLE( v * c );
            }
            (Operand::VARIABLE(v1), Operand::VARIABLE(v2)) => {
                return Operand::VARIABLE( v1 * v2 );
            }
        }
    }
}