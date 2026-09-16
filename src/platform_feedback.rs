#[cfg(not(target_os = "android"))]
pub fn trigger_answer_feedback(_correct: bool, _sound_enabled: bool) {}

#[cfg(target_os = "android")]
#[allow(unsafe_code)]
pub fn trigger_answer_feedback(correct: bool, sound_enabled: bool) {
    use jni::objects::{JClass, JObject, JValue};
    use jni::sys::jobject;
    use jni::{JavaVM, jni_sig, jni_str};

    let android_context = ndk_context::android_context();
    let vm_ptr = android_context.vm();
    let context_ptr = android_context.context();

    log::debug!(
        "Feedback bridge invoked: correct={}, sound_enabled={}",
        correct,
        sound_enabled
    );

    if vm_ptr.is_null() || context_ptr.is_null() {
        log::error!("Android VM/context pointer is null in feedback bridge");
        return;
    }

    let java_vm = unsafe { JavaVM::from_raw(vm_ptr.cast()) };

    let _ = java_vm
        .attach_current_thread(|env| -> jni::errors::Result<()> {
            // SAFETY: The context pointer originates from NativeActivity.
            let context = unsafe { JObject::from_raw(env, context_ptr as jobject) };
            let context_local = env.new_local_ref(&context).map_err(|e| {
                log::error!("Failed to create local JNI ref for Android context");
                e
            })?;

            let class_loader = env
                .call_method(
                    &context_local,
                    jni_str!("getClassLoader"),
                    jni_sig!("()Ljava/lang/ClassLoader;"),
                    &[],
                )?
                .l()
                .map_err(|e| {
                    log::error!("Failed to read class loader object from JNI value");
                    e
                })?;

            let class_name = env
                .new_string("com.asptechinc.lexivo.FeedbackBridge")
                .map_err(|e| {
                    log::error!("Failed to allocate class name string for FeedbackBridge");
                    e
                })?;

            let class_obj = env
                .call_method(
                    &class_loader,
                    jni_str!("loadClass"),
                    jni_sig!("(Ljava/lang/String;)Ljava/lang/Class;"),
                    &[JValue::from(&class_name)],
                )?
                .l()
                .map_err(|e| {
                    log::error!("ClassLoader.loadClass failed for FeedbackBridge");
                    e
                })?;

            // SAFETY: We just loaded this class via loadClass, so it's a valid Class object.
            let feedback_class = unsafe { JClass::from_raw(env, class_obj.as_raw()) };

            env.call_static_method(
                &feedback_class,
                jni_str!("triggerAnswerFeedback"),
                jni_sig!("(Landroid/content/Context;ZZ)V"),
                &[
                    JValue::from(&context_local),
                    JValue::Bool(correct),
                    JValue::Bool(sound_enabled),
                ],
            )
            .map_err(|e| {
                log::error!("FeedbackBridge.triggerAnswerFeedback threw a Java exception");
                e
            })?;

            Ok(())
        })
        .map_err(|e| {
            log::error!("Feedback bridge JNI execution failed: {:?}", e);
            e
        });
}

#[cfg(not(target_os = "android"))]
pub fn init_platform_bridges() {}

#[cfg(target_os = "android")]
#[allow(unsafe_code)]
pub fn init_platform_bridges() {
    use jni::objects::{JObject, JValue};
    use jni::sys::jobject;
    use jni::{JavaVM, jni_sig, jni_str};

    let android_context = ndk_context::android_context();
    let vm_ptr = android_context.vm();
    let context_ptr = android_context.context();

    if vm_ptr.is_null() || context_ptr.is_null() {
        return;
    }

    let java_vm = unsafe { JavaVM::from_raw(vm_ptr.cast()) };

    let _ = java_vm.attach_current_thread(|env| -> jni::errors::Result<()> {
        let context = unsafe { JObject::from_raw(env, context_ptr as jobject) };
        let context_local = env.new_local_ref(&context)?;

        let class_loader = env
            .call_method(
                &context_local,
                jni_str!("getClassLoader"),
                jni_sig!("()Ljava/lang/ClassLoader;"),
                &[],
            )?
            .l()?;

        let class_name = env.new_string("com.asptechinc.lexivo.FeedbackBridge")?;
        let class_obj = env
            .call_method(
                &class_loader,
                jni_str!("loadClass"),
                jni_sig!("(Ljava/lang/String;)Ljava/lang/Class;"),
                &[JValue::from(&class_name)],
            )?
            .l()?;

        let class_obj = unsafe { jni::objects::JClass::from_raw(env, class_obj.as_raw()) };

        env.call_static_method(
            &class_obj,
            jni_str!("init"),
            jni_sig!("(Landroid/content/Context;)V"),
            &[JValue::from(&context_local)],
        )?;

        Ok(())
    });
}

#[cfg(not(target_os = "android"))]
pub fn trigger_update_check() {}

#[cfg(target_os = "android")]
#[allow(unsafe_code)]
pub fn trigger_update_check() {
    // For now, we simulate the update check in Rust (app.rs).
    // If we wanted Java to do it, we'd call a method here.
}

#[cfg(not(target_os = "android"))]
pub fn trigger_update_install(_url: &str) {}

#[cfg(target_os = "android")]
#[allow(unsafe_code)]
pub fn trigger_update_install(url: &str) {
    use jni::objects::{JClass, JObject, JValue};
    use jni::sys::jobject;
    use jni::{JavaVM, jni_sig, jni_str};

    let android_context = ndk_context::android_context();
    let vm_ptr = android_context.vm();
    let context_ptr = android_context.context();

    if vm_ptr.is_null() || context_ptr.is_null() {
        log::error!("Android VM/context pointer is null in update bridge");
        return;
    }

    let java_vm = unsafe { JavaVM::from_raw(vm_ptr.cast()) };

    let _ = java_vm
        .attach_current_thread(|env| -> jni::errors::Result<()> {
            let context = unsafe { JObject::from_raw(env, context_ptr as jobject) };
            let context_local = env.new_local_ref(&context)?;

            let class_loader = env
                .call_method(
                    &context_local,
                    jni_str!("getClassLoader"),
                    jni_sig!("()Ljava/lang/ClassLoader;"),
                    &[],
                )?
                .l()?;

            let class_name = env.new_string("com.asptechinc.lexivo.UpdateBridge")?;
            let class_obj = env
                .call_method(
                    &class_loader,
                    jni_str!("loadClass"),
                    jni_sig!("(Ljava/lang/String;)Ljava/lang/Class;"),
                    &[JValue::from(&class_name)],
                )?
                .l()?;

            // SAFETY: We just loaded this class via loadClass.
            let class_ref = unsafe { JClass::from_raw(env, class_obj.as_raw()) };

            let url_j = env.new_string(url)?;
            env.call_static_method(
                &class_ref,
                jni_str!("downloadAndInstallUpdate"),
                jni_sig!("(Landroid/content/Context;Ljava/lang/String;)V"),
                &[JValue::from(&context_local), JValue::from(&url_j)],
            )?;

            Ok(())
        })
        .map_err(|e| {
            log::error!("Update bridge JNI execution failed: {:?}", e);
            e
        });
}
