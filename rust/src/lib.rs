// JNI bridge with a tiny Rust API

use jni::objects::JClass;
use jni::sys::{jboolean, jint};
use jni::EnvUnowned; // instead of deprecated JNIEnv - stick to ffi safe if possible

mod boot;
mod error;
mod hal;

use boot::BootManager;

fn as_jboolean(v: bool) -> jboolean {
    if v { true } else { false }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_carnx_bootmanager_BootNative_nativeGetCurrentSlot(
    _env: EnvUnowned,
    _class: JClass,
) -> jint {
    match BootManager::default().get_current_slot() {
        Ok(v) => v as jint,
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_carnx_bootmanager_BootNative_nativeSetActiveSlot(
    _env: EnvUnowned,
    _class: JClass,
    slot: jint,
) -> jboolean {
    match BootManager::default().set_active_slot(slot as u32) {
        Ok(()) => as_jboolean(true),
        Err(_) => as_jboolean(false),
    }
}
