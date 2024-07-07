use applications::{App, AppInfo, AppInfoContext};

pub struct Context {
    pub apps: Vec<App>,
}

impl Context {
    pub fn new() -> Self {
        let mut ctx = AppInfoContext::new();
        ctx.refresh_apps().unwrap();
        let apps = ctx.get_all_apps();
        Self { apps }
    }

    pub fn update(&mut self) -> Result<(), ()> {
        Ok(())
    }
}
