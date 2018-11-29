// Dynamically called by exonum-binding
#[no_mangle]
pub fn required_by_exonum_binding(i: i32) -> i32 {
    i * 2
}

// Dynamically called by testkit
#[no_mangle]
pub fn required_by_testkit(i: i32) -> i32 {
    i * 2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(required_by_exonum_binding(2), 4);
        assert_eq!(required_by_testkit(4), 8);
    }
}
