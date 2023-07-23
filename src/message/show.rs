use json_pretty::PrettyFormatter;
use crate::message::message::get_default;

/// Client side show message
pub fn handle_show_message_command(message_type: String, pretty: bool) {
    let default_data = get_default(message_type);

    if default_data.is_some() {
        if pretty {
            let formatter = PrettyFormatter::from_str(default_data.unwrap().as_str());
            let pretty_printed = formatter.pretty();
            println!("{}", pretty_printed);
        }
        else {
            println!("{}", default_data.unwrap());
        }
    }
    else {
        panic!("Message type not found")
    }
}