use serenity::builder::CreateApplicationCommands;

pub fn cmd_list_setup(cmds: &mut CreateApplicationCommands) {
    cmds.create_application_command(|cmd| {
        cmd.name("cmd_list")
            .description("Lists servers custom regex response to messages")
    });
}
