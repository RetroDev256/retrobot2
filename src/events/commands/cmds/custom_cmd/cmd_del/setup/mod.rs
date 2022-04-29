use serenity::{
    builder::CreateApplicationCommands,
    model::interactions::application_command::ApplicationCommandOptionType,
};

pub fn cmd_del_setup(cmds: &mut CreateApplicationCommands) {
    cmds.create_application_command(|cmd| {
        cmd.name("cmd_del")
            .description("Delete a custom regex response to messages")
            .create_option(|opt| {
                opt.name("index")
                    .description("Index of regex to match user messages")
                    .kind(ApplicationCommandOptionType::Integer)
                    .required(true)
            })
    });
}
