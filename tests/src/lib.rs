#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// Buggy piece of code for demo
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equation() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_will_fail() {
        assert_eq!(2 + 2, 5);
    }

    #[test]
    fn another() {
        panic!("Ensure this test will result in a failure,");
    }

    #[test]
    fn large_or_small() {
        let larger = Rectangle {
            width: 12,
            height: 6,
        };
        let smaller = Rectangle {
            width: 4,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn small_or_large() {
        let larger = Rectangle {
            width: 12,
            height: 6,
        };
        let smaller = Rectangle {
            width: 4,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn not_two() {
        assert_ne!(9999, add_two(2))
    }
    

}
