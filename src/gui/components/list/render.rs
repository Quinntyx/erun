use egui::ScrollArea;
use egui::Ui;

use crate::gui::components::list::providers::ListContentProvider;
use crate::gui::components::Component;
use crate::model::context::Context;

pub fn render(c: &Component, app_ctx: &mut Context, ui: &mut Ui) {
    let Component::List {
        show_icons,
        exit_after_selection,
        run_command,
        extra_arguments,
        bridge_stdin,
        bridge_stdout,
        bridge_stderr,
        content,
        // see TODO in mod.rs
        // update_on,
    } = c
    else {
        unreachable!()
    };

    let mut search_key = String::new();

    let _searchbox = ui.add(egui::TextEdit::singleline(&mut search_key));

    ScrollArea::vertical().show(ui, |ui| {
        let mut content = content.clone().unwrap_or(ListContentProvider::Applications);
        let entries = content.provide_contents(
            app_ctx,
            show_icons.unwrap_or(false),
            exit_after_selection.unwrap_or(true),
            run_command.clone().unwrap_or(String::from("open")),
            extra_arguments.clone(),
            bridge_stdin.unwrap_or(false),
            bridge_stdout.unwrap_or(false),
            bridge_stderr.unwrap_or(false),
        );

        let _responses = entries
            .iter()
            .filter(|i| i.matches_filter(&search_key))
            .map(|i| ui.add((*i).clone()))
            .collect::<Vec<_>>();
    });
}

/*
ListContent=> {
        for app in &mut app_ctx.apps {
            let button = ui.add(
                Button::opt_image_and_text(
                    if show_icons.is_some_and(|i| i) {
                        app.icon_path
                            .clone()
                            .map(|i| {
                                Some(egui::Image::from_uri(load_icon_to_uri(i).ok()?))
                            })
                            .flatten()
                    } else {
                        None
                    },
                    Some(app.name.clone().into()),
                )
                .min_size(vec2(ui.available_width(), 0.)),
            );
            if button.clicked() {
                let mut cmd =
                    Command::new(run_command.clone().unwrap_or("open".to_string()));
                cmd.arg(app.app_desktop_path.clone()).args(extra_arguments);

                if bridge_stdin.unwrap_or(false) {
                    cmd.stdin(Stdio::null());
                }
                if bridge_stdout.unwrap_or(false) {
                    cmd.stdout(Stdio::null());
                }
                if bridge_stderr.unwrap_or(false) {
                    cmd.stderr(Stdio::null());
                }

                cmd.spawn().unwrap();
                if exit_after_selection.unwrap_or(true) {
                    println!("app exiting");
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        }
    }
    ListProvider::Command {
        pass_filter_content,
        filter_results,
        update_on,
    } => todo!("Implement commands"),

*/
