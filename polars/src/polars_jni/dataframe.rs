// This is the interface to the JVM that we'll
// call the majority of our methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native function.
// They carry extra lifetime information to prevent them escaping this context
// and getting used after being GC'd.
use jni::objects::{GlobalRef, JClass, JObject, JString};

// This is just a pointer. We'll be returning it from our function.
// We can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jbyteArray, jint, jlong, jstring};

use crate::JDataFrame;

#[no_mangle]
pub extern "system" fn Java_rs_polars_api_DataFrame_print(
    env: JNIEnv,
    _object: JObject,
) -> jstring {
    let native_object_pointer: jlong = env
        .get_field(_object, "nativeObjectPointer", "J")
        .expect("Couldn't get field")
        .j()
        .unwrap();

    let data_frame: Box<JDataFrame> =
        unsafe { Box::from_raw(native_object_pointer as *mut JDataFrame) };

    let output = env
        .new_string(data_frame.print())
        .expect("Couldn't create java string!");
    output.into_inner()
}
