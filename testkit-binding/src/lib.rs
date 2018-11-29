extern crate testkit;
extern crate jni;

use jni::{
    objects::JClass,
    sys::{JavaVM, jint, JNI_VERSION_1_8},
    JNIEnv
};
use std::os::raw::c_void;

fn mult_internal(i: i32) -> i32 {
    i * 2
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn JNI_OnLoad(_vm: JavaVM, _: *mut c_void) -> jint {
    eprint!("\t[NATIVE LOG: TESTKIT BINDINGS LOADED]\t");
    JNI_VERSION_1_8
}


// Provides JNI interface not depending on anything
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_Testkit_callTestkitBinding(_env: JNIEnv, _class: JClass, i: jint) -> jint {
    mult_internal(i)
}

// Provides JNI interface depending on testkit lib
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_Testkit_callTestkitBindingDepTestkit(_env: JNIEnv, _class: JClass, i: jint) -> jint {
    testkit::required_by_testkit_binding(i * 2)
}

// Provides JNI interface depending on testkit lib that dinamycally depends on exonum lib
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_Testkit_callTestkitBindingDepTestkitExonum(_env: JNIEnv, _class: JClass, i: jint) -> jint {
    testkit::calls_exonum_dynamically(i * 2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(mult_internal(21), 42);
    }
}
