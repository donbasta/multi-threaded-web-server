#[cfg(test)]
mod adder {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn purposedly_failing_test() {
        let result = 2 + 2;
        assert_ne!(result, 5);
    }
}

#[cfg(test)]
mod shapes {
    pub trait Shape {
        fn area(&self) -> u32;
    }

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Shape for Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    #[derive(Debug)]
    struct Square {
        sidelength: u32,
    }

    impl Shape for Square {
        fn area(&self) -> u32 {
            self.sidelength * self.sidelength
        }
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn return_correct_area() {
        let square = Square { sidelength: 10 };
        let area = square.area();
        assert_eq!(area, 100)
    }
}
