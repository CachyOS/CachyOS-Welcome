use crate::ui::{MessageType, UI};
use gtk::prelude::*;

pub struct Gui {
    window: gtk::Window,
}

impl Gui {
    pub fn new(window: gtk::Window) -> Self {
        Gui { window }
    }
}

impl UI for Gui {
    fn show_message(&self, message_type: MessageType, message: &str, title: String) {
        let dialog_msg_type = match message_type {
            MessageType::Info => gtk::MessageType::Info,
            MessageType::Warning => gtk::MessageType::Warning,
            MessageType::Error => gtk::MessageType::Error,
        };

        let dialog = gtk::MessageDialog::builder()
            .transient_for(&self.window)
            .message_type(dialog_msg_type)
            .text(message)
            .title(title)
            .modal(true)
            .buttons(gtk::ButtonsType::Ok)
            .build();
        dialog.connect_response(|dialog, _| dialog.close());

        dialog.show();
        // block until user responds
        dialog.run();
        // we are required to close/hide manually according to the docs
        dialog.close();
    }
}

pub fn run_command(command: &str, escalate: bool) -> bool {
    let cmd_formated = format!("{command}; read -p 'Press enter to exit'");
    let mut args: Vec<&str> = vec![];
    if escalate {
        args.extend_from_slice(&["-s", "pkexec /usr/share/cachyos-hello/scripts/rootshell.sh"]);
    }
    args.push(cmd_formated.as_str());

    let exit_status = subprocess::Exec::cmd("/usr/share/cachyos-hello/scripts/terminal-helper")
        .args(args.as_slice())
        .stdout(subprocess::Redirection::Pipe)
        .join()
        .unwrap();
    exit_status.success()
}
