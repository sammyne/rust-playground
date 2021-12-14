use jni::objects::JValue;
use jni::sys::jint;
use jni::{InitArgsBuilder, JNIVersion, JavaVM};

fn main() -> jni::errors::Result<()> {
    let jvm_args = InitArgsBuilder::new()
        // Pass the JNI API version (default is 8)
        .version(JNIVersion::V8)
        // You can additionally pass any JVM options (standard, like a system property,
        // or VM-specific).
        // Here we enable some extra JNI checks useful during development
        .option("-Xcheck:jni")
        .option("-Djava.class.path=./java")
        .build()
        .unwrap();

    // Create a new VM
    let jvm = JavaVM::new(jvm_args)?;

    // Attach the current thread to call into Java — see extra options in
    // "Attaching Native Threads" section.
    //
    // This method returns the guard that will detach the current thread when dropped,
    // also freeing any local references created in it
    let env = jvm.attach_current_thread()?;

    // Call Java Math#abs(-10)
    let x = JValue::from(-10);
    // sig see Type Signatures at https://docs.oracle.com/javase/8/docs/technotes/guides/jni/spec/types.html
    let val: jint = env
        .call_static_method("java/lang/Math", "abs", "(I)I", &[x])?
        .i()?;

    assert_eq!(val, 10);

    {
        let hello_world = env.find_class("HelloWorld")?;
        env.call_static_method(hello_world, "SayHello", "()V", &[])?;
    }

    Ok(())
}
