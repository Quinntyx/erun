pub mod applications;
pub mod command;

use serde::{Deserialize, Serialize};

use crate::gui::components::list::ListEntry;
use crate::model::context::Context;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub enum ListContentProvider {
    #[default]
    Applications,
    Command {
        // TODO: Implement
        #[serde(default)]
        pass_filter_content: Option<bool>,
        #[serde(default)]
        filter_results: Option<bool>,
    },
}

impl ListContentProvider {
    pub fn provide_contents(
        &mut self,
        app_ctx: &mut Context,
        show_icons: bool,
        exit_after_selection: bool,
        run_command: String,
        extra_arguments: Vec<String>,
        bridge_stdin: bool,
        bridge_stdout: bool,
        bridge_stderr: bool,
    ) -> Vec<ListEntry> {
        use ListContentProvider::*;
        match self {
            Applications => applications::provide_contents(
                self,
                app_ctx,
                show_icons,
                exit_after_selection,
                run_command,
                extra_arguments,
                bridge_stdin,
                bridge_stdout,
                bridge_stderr,
            ),
            Command { .. } => todo!("Implement ListContentProvider::Command"),
        }
    }

    pub fn update(&mut self) {}
}
