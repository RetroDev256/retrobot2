use serenity::{
    builder::CreateApplicationCommands,
    model::interactions::application_command::ApplicationCommandOptionType,
};

pub fn acr_generator_setup(cmds: &mut CreateApplicationCommands) {
    cmds.create_application_command(|cmd| {
        cmd.name("acr")
            .description("Generate random acronym")
            .create_option(|opt| {
                opt.name("input")
                    .description("Input to the acronym generator")
                    .kind(ApplicationCommandOptionType::String)
                    .required(true)
            })
    });
}
