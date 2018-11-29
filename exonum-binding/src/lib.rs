extern crate exonum;
extern crate jni;

use exonum::required_by_exonum_binding;
use jni::{
    objects::JClass,
    sys::{JavaVM, jint, JNI_VERSION_1_8},
    JNIEnv
};
use std::os::raw::c_void;

fn twice_internal(i: i32) -> i32 {
    i * 2
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn JNI_OnLoad(_vm: JavaVM, _: *mut c_void) -> jint {
    eprint!("\t[NATIVE LOG: EXONUM BINDINGS LOADED]\t");
    JNI_VERSION_1_8
}

// Provides JNI interface not depending on anything
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_Exonum_callExonumBinding(_env: JNIEnv, _class: JClass, i: jint) -> jint {
    twice_internal(i)
}


// Provides JNI interface depending on exonum lib
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_Exonum_callExonumBindingDepExonum(_env: JNIEnv, _class: JClass, i: jint) -> jint {
    required_by_exonum_binding(i * 2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(twice_internal(3), 6);
    }
}
