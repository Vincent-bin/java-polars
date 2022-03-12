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

use crate::data_frame::read_csv;

#[no_mangle]
pub extern "system" fn Java_rs_polars_api_Polars_fromCSV(
    env: JNIEnv,
    _class: JClass,
    file_name: JString,
) -> jlong {
    let file_name: String = env
        .get_string(file_name)
        .expect("Couldn't get java string!")
        .into();

    let data_frame = Box::into_raw(Box::new(read_csv(file_name).unwrap()));

    data_frame as jlong
}
