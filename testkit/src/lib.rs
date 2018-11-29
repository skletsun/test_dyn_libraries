extern crate exonum;

#[no_mangle]
pub fn calls_exonum_dynamically(i: i32) -> i32 {
    exonum::required_by_testkit(i * 2)
}

#[no_mangle]
pub fn required_by_testkit_binding(i: i32) -> i32 {
    i * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(required_by_testkit_binding(4), 8);
    }
}
