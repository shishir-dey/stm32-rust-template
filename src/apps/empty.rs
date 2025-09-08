use crate::apps::App;

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

    /// Non-blocking tick that does nothing useful (just idles).
    pub fn tick(&mut self) {
        // If not initialized yet, try to init.
        if !self.initialized {
            let _ = self.init();
        }

        // Do nothing
    }
}

impl App for EmptyApp {
    fn init(&mut self) -> Result<(), i32> {
        self.init()
    }
    fn loop_step(&mut self) {
        self.tick()
    }
}

pub fn create_empty_app() -> EmptyApp {
    EmptyApp::new()
}
