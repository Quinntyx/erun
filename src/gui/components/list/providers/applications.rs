use std::process::{Command, Stdio};
use std::rc::Rc;
use std::sync::Mutex;

use crate::gui::components::list::providers::ListContentProvider;
use crate::gui::components::list::ListEntry;
use crate::model::context::Context;
use crate::utils::load_icon_to_uri;

pub fn provide_contents<'a>(
    p: &mut ListContentProvider,
    app_ctx: &mut Context,
    show_icons: bool,
    exit_after_selection: bool,
    run_command: String,
    extra_arguments: Vec<String>,
    bridge_stdin: bool,
    bridge_stdout: bool,
    bridge_stderr: bool,
) -> Vec<ListEntry<'a>> {
    match p {
        ListContentProvider::Applications => (),
        _ => unreachable!(),
    }

    app_ctx
        .apps
        .iter()
        .map(|app| {
            let mut cmd = Command::new(&run_command);
            cmd.arg(app.app_desktop_path.clone())
                .args(extra_arguments.clone());

            if !bridge_stdin {
                cmd.stdin(Stdio::null());
            }
            if !bridge_stdout {
                cmd.stdout(Stdio::null());
            }
            if !bridge_stderr {
                cmd.stderr(Stdio::null());
            }

            let mut icon = None;

            if show_icons {
                icon = app
                    .icon_path
                    .clone()
                    .map(|i| Some(egui::Image::from_uri(load_icon_to_uri(i).ok()?)))
                    .flatten();
            }

            ListEntry {
                display_title: app.name.clone(),
                filter_string: None,
                icon,
                exit_after_selection,
                omnipresent: false,
                click_callback: Rc::new(Mutex::new(cmd)),
            }
        })
        .collect()
}
