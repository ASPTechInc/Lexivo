#[cfg(not(target_os = "android"))]
pub fn trigger_answer_feedback(_correct: bool, _sound_enabled: bool) {}

#[cfg(target_os = "android")]
#[allow(unsafe_code)]
pub fn trigger_answer_feedback(correct: bool, sound_enabled: bool) {
    use jni::JNIEnv;
    use jni::JavaVM;
    use jni::objects::{JClass, JObject, JValue};
    use jni::sys::{jboolean, jobject};

    fn clear_pending_exception(env: &mut JNIEnv<'_>) {
        if env.exception_check().unwrap_or(false) {
            let _ = env.exception_describe();
            let _ = env.exception_clear();
        }
    }

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

    // SAFETY: The VM pointer is provided by Android's app runtime and remains valid
    // for the process lifetime while NativeActivity is running.
    let Ok(java_vm) = (unsafe { JavaVM::from_raw(vm_ptr.cast()) }) else {
        log::error!("Failed to attach JavaVM from raw pointer in feedback bridge");
        return;
    };

    let Ok(mut env) = java_vm.attach_current_thread() else {
        log::error!("Failed to attach current thread to Java VM");
        return;
    };

    // SAFETY: The context pointer originates from NativeActivity. We intentionally
    // prevent Rust from deleting this JNI reference by forgetting it after the call.
    let context = unsafe { JObject::from_raw(context_ptr as jobject) };
    let context_local = match env.new_local_ref(&context) {
        Ok(local) => local,
        Err(_) => {
            log::error!("Failed to create local JNI ref for Android context");
            clear_pending_exception(&mut env);
            std::mem::forget(context);
            return;
        }
    };
    std::mem::forget(context);

    let class_loader = match env.call_method(
        &context_local,
        "getClassLoader",
        "()Ljava/lang/ClassLoader;",
        &[],
    ) {
        Ok(value) => match value.l() {
            Ok(loader) => loader,
            Err(_) => {
                log::error!("Failed to read class loader object from JNI value");
                clear_pending_exception(&mut env);
                return;
            }
        },
        Err(_) => {
            log::error!("Failed to call Context.getClassLoader() from feedback bridge");
            clear_pending_exception(&mut env);
            return;
        }
    };

    let class_name = match env.new_string("com.asptechinc.lexivo.FeedbackBridge") {
        Ok(name) => name,
        Err(_) => {
            log::error!("Failed to allocate class name string for FeedbackBridge");
            clear_pending_exception(&mut env);
            return;
        }
    };
    let class_name_obj: JObject<'_> = JObject::from(class_name);
    let class_obj = match env.call_method(
        &class_loader,
        "loadClass",
        "(Ljava/lang/String;)Ljava/lang/Class;",
        &[JValue::Object(&class_name_obj)],
    ) {
        Ok(value) => match value.l() {
            Ok(class_obj) => class_obj,
            Err(_) => {
                log::error!("ClassLoader.loadClass returned non-object for FeedbackBridge");
                clear_pending_exception(&mut env);
                return;
            }
        },
        Err(_) => {
            log::error!("ClassLoader.loadClass failed for FeedbackBridge");
            clear_pending_exception(&mut env);
            return;
        }
    };
    let feedback_class: JClass<'_> = JClass::from(class_obj);

    let correct_j: jboolean = if correct { 1 } else { 0 };
    let sound_enabled_j: jboolean = if sound_enabled { 1 } else { 0 };

    let call_result = env.call_static_method(
        feedback_class,
        "triggerAnswerFeedback",
        "(Landroid/content/Context;ZZ)V",
        &[
            JValue::Object(&context_local),
            JValue::Bool(correct_j),
            JValue::Bool(sound_enabled_j),
        ],
    );

    if call_result.is_err() {
        log::error!("FeedbackBridge.triggerAnswerFeedback threw a Java exception");
        clear_pending_exception(&mut env);
    } else {
        log::debug!("FeedbackBridge.triggerAnswerFeedback completed");
    }
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
    use jni::JNIEnv;
    use jni::JavaVM;
    use jni::objects::{JObject, JValue};
    use jni::sys::jobject;

    fn clear_pending_exception(env: &mut JNIEnv<'_>) {
        if env.exception_check().unwrap_or(false) {
            let _ = env.exception_describe();
            let _ = env.exception_clear();
        }
    }

    let android_context = ndk_context::android_context();
    let vm_ptr = android_context.vm();
    let context_ptr = android_context.context();

    if vm_ptr.is_null() || context_ptr.is_null() {
        log::error!("Android VM/context pointer is null in update bridge");
        return;
    }

    let Ok(java_vm) = (unsafe { JavaVM::from_raw(vm_ptr.cast()) }) else {
        log::error!("Failed to attach JavaVM in update bridge");
        return;
    };

    let Ok(mut env) = java_vm.attach_current_thread() else {
        log::error!("Failed to attach thread in update bridge");
        return;
    };

    let context = unsafe { JObject::from_raw(context_ptr as jobject) };
    let context_local = match env.new_local_ref(&context) {
        Ok(local) => local,
        Err(_) => {
            clear_pending_exception(&mut env);
            std::mem::forget(context);
            return;
        }
    };
    std::mem::forget(context);

    let class_loader = match env.call_method(
        &context_local,
        "getClassLoader",
        "()Ljava/lang/ClassLoader;",
        &[],
    ) {
        Ok(value) => value.l().unwrap_or_else(|_| JObject::null()),
        Err(_) => {
            clear_pending_exception(&mut env);
            return;
        }
    };

    let class_name = env.new_string("com.asptechinc.lexivo.UpdateBridge").unwrap();
    let class_name_obj: JObject<'_> = JObject::from(class_name);
    let class_obj = match env.call_method(
        &class_loader,
        "loadClass",
        "(Ljava/lang/String;)Ljava/lang/Class;",
        &[JValue::Object(&class_name_obj)],
    ) {
        Ok(value) => value.l().unwrap_or_else(|_| JObject::null()),
        Err(_) => {
            clear_pending_exception(&mut env);
            return;
        }
    };

    let url_j = env.new_string(url).unwrap();
    let _ = env.call_static_method(
        jni::objects::JClass::from(class_obj),
        "downloadAndInstallUpdate",
        "(Landroid/content/Context;Ljava/lang/String;)V",
        &[
            JValue::Object(&context_local),
            JValue::Object(&url_j),
        ],
    );

    clear_pending_exception(&mut env);
}
