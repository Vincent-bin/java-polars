pub mod data_frame;

use polars::prelude::*;

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

use std::fs::File;
use std::{sync::mpsc, thread, time::Duration};

use data_frame::JDataFrame;

// #[no_mangle]
// pub extern "system" fn Java_rs_polars_api_DataFrame_nativeNew(
//     env: JNIEnv,
//     _object: JObject,
//     path: JString,
// ) -> jlong {
//     // First, we have to get the string out of java. Check out the `strings`
//     // module for more info on how this works.
//     let path: String = env
//         .get_string(path)
//         .expect("Couldn't get java string!")
//         .into();

//     let data_frame = Box::into_raw(Box::new(JDataFrame::new()));

//     data_frame as jlong
// }

// #[no_mangle]
// pub extern "system" fn Java_rs_polars_api_DataFrame_getPath(
//     env: JNIEnv,
//     // this is the class that owns our
//     // static method. Not going to be
//     // used, but still needs to have
//     // an argument slot
//     _object: JObject,
// ) -> JString {
//     // First, we have to get the string out of java. Check out the `strings`
//     // module for more info on how this works.

//     // let nativeObjectPointerID = env.get_field_id(cls, "nativeObjectPointer", "J").expect("Couldn't get field id").into();

//     let native_object_pointer : jlong = env.get_field(_object, "nativeObjectPointer", "J").expect("Couldn't get field").j().unwrap();

//     let data_frame:Box<JDataFrame> =  unsafe {

//        Box::from_raw(native_object_pointer as *mut JDataFrame)
//     };

//     let output = env.new_string(data_frame.get_path()).expect("Couldn't create java string!");
//     output.into_inner()
// }


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

#[no_mangle]
pub extern "system" fn Java_rs_polars_api_DataFrame_print(
    env: JNIEnv,
    _object: JObject,
) -> jstring {

    let native_object_pointer : jlong = env.get_field(_object, "nativeObjectPointer", "J").expect("Couldn't get field").j().unwrap();

    let data_frame:Box<JDataFrame> =  unsafe {

       Box::from_raw(native_object_pointer as *mut JDataFrame)
    };

    let output = env.new_string(data_frame.print()).expect("Couldn't create java string!");
    output.into_inner()
}

pub fn read_csv(file_path: String) -> Result<JDataFrame> {
    let p = std::path::Path::new(&file_path);
    let p = resolve_homedir(p);
    let f = File::open(&p)?;
    let df = CsvReader::new(f).finish().unwrap();
    Ok(JDataFrame { df })
}
