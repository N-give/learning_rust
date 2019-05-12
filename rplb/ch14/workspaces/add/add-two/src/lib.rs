#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_add_two() {
        assert_eq!(add_two(2), 4);
    }
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}
