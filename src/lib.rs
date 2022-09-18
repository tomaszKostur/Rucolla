use core::fmt::Debug;

#[allow(dead_code)]
pub mod blackboard {

    use std::str::FromStr;

    use super::Pinnable;
    #[derive(Debug)]
    pub struct StickyNote {
        pub message: String,
        pub placement: Point,
        pub representation: Rectangle,
    }

    impl Pinnable for StickyNote {}

    #[derive(Debug)]
    pub struct Rectangle {
        pub dx: usize,
        pub dy: usize,
    }
    #[derive(Debug, Clone)]
    pub struct Point {
        pub x: isize,
        pub y: isize,
    }

    pub fn new_note() -> StickyNote {
        StickyNote {
            message: String::from_str("Sticky note!").unwrap(),
            placement: Point { x: 0, y: 0 },
            representation: Rectangle { dx: 100, dy: 100 },
        }
    }

    impl StickyNote {
        pub fn change_pos(&mut self, dx: isize, dy: isize) {
            self.placement.x += dx;
            self.placement.y += dy;
        }
    }
}

trait Pinnable: Debug {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_sticky() {
        let sn = blackboard::new_note();
        assert_eq!(sn.message, "Sticky note!");
        println!("new sticky note: {:?}\n", sn);

        let item1 = Box::new(blackboard::new_note());
        let item2 = Box::new(blackboard::new_note());
        let item3 = Box::new(blackboard::new_note());
        let board: Vec<Box<dyn Pinnable>> = vec![item1, item2, item3];
        println!("The board is: {:?}", board);
    }
}
