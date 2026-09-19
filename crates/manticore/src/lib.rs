pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod bullets;
pub mod date;
pub mod profile;
pub mod validate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
