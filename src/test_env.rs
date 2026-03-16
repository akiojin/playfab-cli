use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

pub fn env_lock() -> &'static Mutex<()> {
    &ENV_LOCK
}
