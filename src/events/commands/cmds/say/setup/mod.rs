use serenity::{
    builder::CreateApplicationCommands,
    model::interactions::application_command::ApplicationCommandOptionType,
};

pub fn say_setup(cmds: &mut CreateApplicationCommands) {
    cmds.create_application_command(|cmd| {
        cmd.name("say")
            .description("Melodious text to your eyes.")
            .create_option(|opt| {
                opt.name("input")
                    .description("What should I say?")
                    .kind(ApplicationCommandOptionType::String)
                    .required(true)
            })
    });
}
