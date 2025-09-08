pub struct EmptyApp {
    initialized: bool,
}

impl EmptyApp {
    pub fn new() -> Self {
        Self { initialized: false }
    }

    /// Initialize the app. Nothing to do for the empty app, but keep the same
    /// Result signature so callers can treat it like other apps.
    pub fn init(&mut self) -> Result<(), i32> {
        if self.initialized {
            return Ok(());
        }

        // No hardware to initialize for empty app.
        self.initialized = true;
        Ok(())
    }

    /// Never-returning run loop that does nothing useful (just idles).
    /// This preserves the `-> !` contract expected by `#[entry]` callers.
    pub fn run(&mut self) -> ! {
        // If not initialized yet, try to init; otherwise hang.
        if !self.initialized && self.init().is_err() {
            loop {
                cortex_m::asm::nop();
            }
        }

        // Forever idle (app "does nothing")
        loop {
            cortex_m::asm::nop();
        }
    }
}

/* API functions similar to your blink app */

static mut EMPTY_APP: Option<EmptyApp> = None;

pub fn init_empty_app() -> Result<(), i32> {
    unsafe {
        EMPTY_APP = Some(EmptyApp::new());
        if let Some(ref mut app) = EMPTY_APP {
            app.init()?;
        }
    }
    Ok(())
}

pub fn run_empty_app() -> ! {
    unsafe {
        if let Some(ref mut app) = EMPTY_APP {
            app.run();
        } else {
            loop {
                cortex_m::asm::nop();
            }
        }
    }
}

pub fn create_empty_app() -> EmptyApp {
    EmptyApp::new()
}
