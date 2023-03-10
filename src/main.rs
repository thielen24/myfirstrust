fn main() {
    println!("The subtraction result is {}", subtract(2, 3));
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(1, 2), -1);
    }
}