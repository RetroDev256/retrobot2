use serenity::{
    builder::CreateApplicationCommands,
    model::interactions::application_command::ApplicationCommandOptionType,
};

pub fn cmd_add_setup(cmds: &mut CreateApplicationCommands) {
    cmds.create_application_command(|cmd| {
        cmd.name("cmd_list")
            .description("Lists servers custom regex response to messages")
    });
}
