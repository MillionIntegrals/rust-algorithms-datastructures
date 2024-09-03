pub mod analysis;
pub mod ds;

use analysis::commands::CommandMap;

use clap::Command;

/// Register commands in a given CommandMap into a clap::Command
pub fn register_commands(map: &CommandMap, command: Command) -> Command {
    let mut c = command;

    for d in map.commands.values() {
        let subcommand = Command::new(d.name.clone()).about(d.description.clone());

        c = c.subcommand(subcommand);
    }

    c
}

/// Run registered main command
fn main() {
    let mut cm = CommandMap::new();

    // BEGIN REGISTRATION BLOCK
    analysis::heaps::register_commands(&mut cm);
    // END REGISTRATION BLOCK

    let mut command = Command::new("algods")
        .version("0.1")
        .about("Rust algorithms and data structures")
        .subcommand_required(true)
        .propagate_version(true);
    command = register_commands(&cm, command);

    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("help", _)) => {
            // We don't need to execute it, clap does that for us it seems
        }
        Some((subcommand, _)) => match cm.get(subcommand) {
            Some(desc) => desc.execute(),
            None => {
                let err = Command::new("").error(
                    clap::error::ErrorKind::InvalidSubcommand,
                    format!("Invalid command: {subcommand}"),
                );
                err.exit()
            }
        },
        None => {
            panic!("Unreachable branch reached")
        }
    }
}
