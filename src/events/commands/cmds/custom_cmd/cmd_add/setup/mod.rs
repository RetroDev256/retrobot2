use serenity::{builder::CreateApplicationCommands, model::prelude::command::CommandOptionType};

pub fn cmd_add_setup(cmds: &mut CreateApplicationCommands) {
    cmds.create_application_command(|cmd| {
        cmd.name("cmd_add")
            .description("Add a custom regex response to messages")
            .create_option(|opt| {
                opt.name("regex")
                    .description("Regex to match user messages")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
            .create_option(|opt| {
                opt.name("response")
                    .description("Response to matching messages")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
    });
}
