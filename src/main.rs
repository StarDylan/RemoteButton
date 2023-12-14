mod remote_button_reciever;
mod remote_button_sender;

fn main() {
    let cmd = clap::Command::new("rb")
        .bin_name("rb")
        .subcommand_required(true)
        .subcommand(clap::Command::new("send").about("Send Keypresses to Receivers"))
        .subcommand(clap::Command::new("recv").about("Recv Keypresses from Senders"));

    let matches = cmd.get_matches();

    match matches.subcommand_name() {
        Some("send") => remote_button_sender::main(),
        Some("recv") => remote_button_reciever::main(),
        _ => unreachable!("Clap ensures we never get here"),
    }
}
