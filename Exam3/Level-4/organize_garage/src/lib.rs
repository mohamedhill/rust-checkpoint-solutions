use std::ops::Add;

use std::ops::Add;

pub struct Garage<T: Add<Output = T> + Copy> {
    pub left: Option<T>,
    pub right: Option<T>,
}

impl<T: Add<Output = T> + Copy> Garage<T> {
    // methods
}








#[derive(Debug, PartialEq, Eq)]
pub struct Garage<T>
where
    T: Add<Output = T> + Copy,
{
    pub left: Option<T>,
    pub right: Option<T>,
}

impl<T> Garage<T>
where
    T: Add<Output = T> + Copy,
{
    pub fn move_to_right(&mut self) {
        if let (Some(l), Some(r)) = (self.left, self.right) {
            self.right = Some(l + r);
            self.left = None;
        }
    }

    pub fn move_to_left(&mut self) {
        if let (Some(l), Some(r)) = (self.left, self.right) {
            self.left = Some(l + r);
            self.right = None;
        }
    }
}



#[cfg(test)]
mod tests {
    use organize_garage::*;

    #[test]
    fn test_move_to_right() {
        let mut garage_int = Garage {
            left: Some(5),
            right: Some(2),
        };

        garage_int.move_to_right();
        assert_eq!(
            garage_int,
            Garage {
                left: None,
                right: Some(7)
            }
        );
        garage_int.move_to_right();
        assert_eq!(
            garage_int,
            Garage {
                left: None,
                right: Some(7)
            }
        );

        let mut garage_float = Garage {
            left: Some(4.25),
            right: Some(1.11),
        };

        garage_float.move_to_right();
        assert_eq!(
            garage_float,
            Garage {
                left: None,
                right: Some(5.36)
            }
        );
    }

    #[test]
    fn test_move_to_left() {
        let mut garage_int = Garage {
            left: Some(10),
            right: Some(2),
        };

        garage_int.move_to_left();
        assert_eq!(
            garage_int,
            Garage {
                left: Some(12),
                right: None
            }
        );
        garage_int.move_to_left();
        assert_eq!(
            garage_int,
            Garage {
                left: Some(12),
                right: None
            }
        );

        let mut garage_float = Garage {
            left: Some(4.25),
            right: Some(1.11),
        };

        garage_float.move_to_left();
        assert_eq!(
            garage_float,
            Garage {
                left: Some(5.36),
                right: None
            }
        );
    }
}