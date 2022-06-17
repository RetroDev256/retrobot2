use serenity::{builder::CreateApplicationCommands, model::prelude::command::CommandOptionType};

pub fn direct_message_setup(cmds: &mut CreateApplicationCommands) {
    cmds.create_application_command(|cmd| {
        cmd.name("dm")
            .description("Direct message a user")
            .create_option(|opt| {
                opt.name("user")
                    .description("User to message")
                    .kind(CommandOptionType::User)
                    .required(true)
            })
            .create_option(|opt| {
                opt.name("text")
                    .description("Text to message user")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
    });
}
