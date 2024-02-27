//essa anotação é para poder imprimir certinho
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn greeting(name: &str) -> String {
    return format!("Hello {}!", name);
}

pub struct Guess {
    value: i32,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            length: 7,
        };

        let smaller = Rectangle {
            width: 5,
            length: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            length: 7,
        };

        let smaller = Rectangle {
            width: 5,
            length: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_constains_name() {
        let result = greeting("Arthur");
        assert!(
            result.contains("Arthur"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 200.")]
    fn greater_than_100() {
        let result = Guess::new(200);
    }
}
