mod blackboard_items;

fn main() {
    println!("Hello Rucolla");
}

#[cfg(test)]
mod test {
    #[test]
    fn foo() {
        println!("From test foo");
        assert!(true);
    }
}