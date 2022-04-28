use serenity::builder::CreateApplicationCommands;

pub fn emoji_dump_setup(cmds: &mut CreateApplicationCommands) {
    cmds.create_application_command(|cmd| {
        cmd.name("emojis")
            .description("List the emojis of a server")
    });
}
