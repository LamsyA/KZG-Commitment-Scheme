pub use oblast_demo::Fr;
pub use std::fmt;

/// OBJECTIVEs
/// 1. Implement a struct `Polynomial` that represents a polynomial. [Done]
/// 2. Implement the `Display` trait for `Polynomial` so that we can print it out. [Done]
/// 3. Implement a method `evaluate` for `Polynomial` that evaluates the polynomial at a given point.
/// 4. Implement a method to create a polynomial from a list of coefficients.

#[derive(Debug)]
struct Polynomial {
    pub coefficients: Vec<Fr>,
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for (i, coeff) in self.coefficients.iter().enumerate() {
            if i == 0 {
                result.push_str(&format!("{:?}", coeff));
            } else {
                result.push_str(&format!(" + {:?} x^ {:?}", coeff, i));
            }
        }
        write!(f, "{}", result)
    }
}

impl Polynomial {
    pub fn from_coefficients(coefficients: Vec<Fr>) -> Self {
        Self { coefficients }
    }

    // example 2x^2 + 3x + 1 ---> [1,3,2]
    // evaluate(2) = 2*2ˆ2 + 3*2 + 1 = 15
    // evaluate(2) = 1 + 3*2 + 2*2ˆ2 = 15
    // evaluate(2) = 15
    fn evaluate(&self, x: Fr) -> Fr {
        let mut sum = self.coefficients[0].clone();
        let mut variable = x.clone();
        for i in 1..self.coefficients.len() {
            sum = sum + self.coefficients[i] * variable;
            variable = variable * x;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomial_evaluation() {
        let poly =
            Polynomial::from_coefficients(vec![Fr::from_u64(1), Fr::from_u64(3), Fr::from_u64(2)]);
        let result = poly.evaluate(Fr::from_u64(2));
        println!("New result {:?}", poly);
        assert_eq!(result, Fr::from_u64(15));
    }
}
