use serenity::{builder::CreateApplicationCommands, model::prelude::command::CommandOptionType};

pub fn say_setup(cmds: &mut CreateApplicationCommands) {
    cmds.create_application_command(|cmd| {
        cmd.name("say")
            .description("Melodious text to your eyes.")
            .create_option(|opt| {
                opt.name("input")
                    .description("What should I say?")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
    });
}
