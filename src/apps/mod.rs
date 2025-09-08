pub mod blink;
pub mod empty;

use alloc::boxed::Box;
use alloc::vec::Vec;

pub trait App {
    fn init(&mut self) -> Result<(), i32>;
    fn loop_step(&mut self);
}

static mut APPS: Option<Vec<Box<dyn App>>> = None;

pub fn init_app_registry() {
    unsafe {
        APPS = Some(Vec::new());
    }
}

pub fn register_app(app: Box<dyn App>) {
    unsafe {
        if let Some(ref mut apps) = APPS {
            apps.push(app);
        }
    }
}

pub fn init_all_apps() -> Result<(), i32> {
    unsafe {
        if let Some(ref mut apps) = APPS {
            for app in apps.iter_mut() {
                app.init()?;
            }
        }
    }
    Ok(())
}

pub fn run_all_loop_steps() {
    unsafe {
        if let Some(ref mut apps) = APPS {
            for app in apps.iter_mut() {
                app.loop_step();
            }
        }
    }
}

pub mod interrupts;
