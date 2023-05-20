fn my_name() -> String {
    "Sasha".to_string()
}

fn main() {
    println!("Hello, {}!", my_name());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_name() {
        assert_eq!(my_name(), "Bob");
    }
}
