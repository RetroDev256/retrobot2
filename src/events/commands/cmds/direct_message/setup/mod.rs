use serenity::{
    builder::CreateApplicationCommands,
    model::interactions::application_command::ApplicationCommandOptionType,
};

pub fn direct_message_setup(cmds: &mut CreateApplicationCommands) {
    cmds.create_application_command(|cmd| {
        cmd.name("dm")
            .description("Direct message a user")
            .create_option(|opt| {
                opt.name("user")
                    .description("User to message")
                    .kind(ApplicationCommandOptionType::User)
                    .required(true)
            })
            .create_option(|opt| {
                opt.name("text")
                    .description("Text to message user")
                    .kind(ApplicationCommandOptionType::String)
                    .required(true)
            })
    });
}
