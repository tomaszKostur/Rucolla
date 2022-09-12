use blackboard_lib::blackboard;

fn main() {
    println!("Hello Rucolla");
    println!("Somenote: {:?}", blackboard::new_note());
}

#[cfg(test)]
mod test {
    #[test]
    fn foo() {
        println!("From test foo");
        assert!(true);
    }
}