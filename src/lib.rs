pub mod client;
pub mod network;

pub mod util {
    pub fn add(a: i32, b:i32) -> i32 {
        a + b
    }
}
#[cfg(test)]
mod tests {
    use util::*;

    #[test]
    fn test_add() {
        assert_eq!(12, add(4, 8))
    }
}
