use crate::ui::{MessageType, UI};
use crate::utils;
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
    // For simple operations that don't need user interaction, run directly without terminal
    // This prevents unnecessary terminal windows for operations like systemctl enable/disable
    // Check if command is a simple systemctl enable/disable operation
    let is_simple_command = command.trim_start().starts_with("systemctl") 
        && (command.contains(" enable ") || command.contains(" disable "))
        && !command.contains("status")  // status commands might need output
        && !command.contains("log");     // log commands need output
    
    if is_simple_command {
        // Run directly without spawning a terminal window
        return utils::run_cmd(command.to_string(), escalate).map(|s| s.success()).unwrap_or(false);
    }
    
    // For operations that may need user interaction or output viewing, use terminal
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
