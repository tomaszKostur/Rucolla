pub mod blackboard_shapes {
    use std::str::FromStr;
    #[derive(Debug)]
    pub struct StickyNote {
        pub message: String,
        placement: Point,
        representation: Rectangle
    }

    #[derive(Debug)]
    struct Rectangle {
        dx: usize,
        dy: usize
    }
    #[derive(Debug)]
    struct Point {
        x: isize,
        y: isize
    }

    pub fn new_note() -> StickyNote {
        StickyNote { message: String::from_str("Sticky note!").unwrap(), placement: Point{x:0, y:0}, representation: Rectangle { dx: 10, dy: 10 } }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_sticky() {
        let sn = blackboard_shapes::new_note();
        assert_eq!(sn.message, "Sticky note!");
    }
}