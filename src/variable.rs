use std::{cmp::Ordering, string::ToString, ops::{ Neg, Mul } };

// For now there's support only for one variable type per expression

pub const VARIABLE_NAME : char = 'x';

#[derive(PartialEq, Clone, Copy)]
pub struct Variable {
    pub mult:   f32,
    pub exp:    i32
}

impl Variable {
    pub fn new( mult: f32, exp: i32 ) -> Self {
        Variable {
            mult,
            exp
        }
    }

    pub fn abs(&self) -> Self {
        Variable {
            mult: self.mult.abs(),
            exp: self.exp
        }
    }
}

impl PartialOrd for Variable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // first compare exponents
        match self.exp.partial_cmp(&other.exp) {
            Some(Ordering::Greater) => return Some(Ordering::Less),
            Some(Ordering::Less) => return Some(Ordering::Greater),
            _ => {}
        }

        // multipliers only afterwards
        self.mult.partial_cmp(&other.mult)
    }
}

impl ToString for Variable {
    fn to_string(&self) -> String {
        let mut s = String::new();

        if self.mult == -1.0 {
            s.push('-');
        }
        else if self.mult != 1.0 {
            s.push_str(&self.mult.to_string());
        }

        s.push(VARIABLE_NAME);
        
        if self.exp != 1 {
            s.push('^');
            s.push_str(&self.exp.to_string());
        }

        s
    }
}

impl Neg for Variable {
    type Output = Variable;

    fn neg(self) -> Self::Output {
        Variable {
            mult: -self.mult,
            exp: self.exp
        }
    }
}

impl Mul<Variable> for Variable {
    type Output = Variable;

    fn mul(self, rhs: Self) -> Self::Output {
        Variable {
            mult: self.mult * rhs.mult,
            exp: self.exp + rhs.exp
        }
    }
}

impl Mul<f32> for Variable {
    type Output = Variable;

    fn mul(self, rhs: f32) -> Self::Output {
        Variable {
            mult: self.mult * rhs,
            exp: self.exp
        }
    }
}

impl Mul<Variable> for f32 {
    type Output = Variable;

    fn mul(self, rhs: Variable) -> Self::Output {
        Variable {
            mult: self * rhs.mult,
            exp: rhs.exp
        }
    }
}
