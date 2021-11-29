#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub struct Guess {
    value: i32,
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

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
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
    
    #[test]
    fn greetings() {
        let result = greeting("Chris");
        assert!(result.contains("Chris"), 
        "Greeting did not contain name, value instead was `{}`",
        result);
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn using_result() -> Result<(), String> {
        if 99 + 1 == 100 {
            print!("Looks like everything is A-Okay");
            Ok(())
        } else {
            Err(String::from("Somehow, some way, 99 + 1 does not equal 100"))
        }
    }

}
