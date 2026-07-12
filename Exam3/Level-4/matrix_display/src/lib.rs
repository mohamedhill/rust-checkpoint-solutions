





use std::fmt;

pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        let vector = slice.iter().map(|&row| row.to_vec()).collect();
        
        
        Matrix(vector)
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       
        for (i, row) in self.0.iter().enumerate() {
            write!(f, "(")?;
            
            for (j, val) in row.iter().enumerate() {
                if j != 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", val)?;
            }
            
            write!(f, ")")?;
            
            if i != self.0.len() - 1 {
                writeln!(f)?;
            }
        }
        
        Ok(())
    }
}



#[test]
fn it_works() {
    assert_eq!(
        Matrix::new(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]).to_string(),
        "(1 2 3)\n(4 5 6)\n(7 8 9)"
    );
}

#[test]
fn test_matrix_col() {
    assert_eq!(
        Matrix::new(&[&[1], &[2], &[3]]).to_string(),
        "(1)\n(2)\n(3)"
    );
}

#[test]
fn test_matrix_row() {
    assert_eq!(Matrix::new(&[&[1, 2, 3]]).to_string(), "(1 2 3)");
}

#[test]
fn test_m_by_n_matrix() {
    assert_eq!(
        Matrix::new(&[&[1, 2, 3, 4, 5], &[6, 7, 8, 9, 10], &[11, 12, 13, 14, 15]]).to_string(),
        "(1 2 3 4 5)\n(6 7 8 9 10)\n(11 12 13 14 15)"
    );
}

#[test]
fn test_empty_matrix() {
    assert_eq!(Matrix::new(&[&[]]).to_string(), "()");
}