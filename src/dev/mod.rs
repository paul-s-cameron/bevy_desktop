use bevy::prelude::*;
use bevy_console::{reply, AddConsoleCommand, ConsoleCommand, ConsolePlugin};
use clap::Parser;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(ConsolePlugin);
    app.add_console_command::<ExampleCommand, _>(example_command);
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "example")]
struct ExampleCommand {
    msg: String,
}

fn example_command(mut log: ConsoleCommand<ExampleCommand>) {
    if let Some(Ok(ExampleCommand { msg })) = log.take() {
        reply!(log, "Example command: {}", msg);
    }
}