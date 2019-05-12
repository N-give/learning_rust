#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_add_one() {
        assert_eq!(add_one(1), 2);
    }
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}
