use serenity::{builder::CreateApplicationCommands, model::prelude::command::CommandOptionType};

pub fn acr_generator_setup(cmds: &mut CreateApplicationCommands) {
    cmds.create_application_command(|cmd| {
        cmd.name("acr")
            .description("Generate random acronym")
            .create_option(|opt| {
                opt.name("input")
                    .description("Input to the acronym generator")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
    });
}
