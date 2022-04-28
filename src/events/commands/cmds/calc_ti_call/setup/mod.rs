use serenity::{
    builder::CreateApplicationCommands,
    model::interactions::application_command::ApplicationCommandOptionType,
};

pub fn calc_ti_call_setup(cmds: &mut CreateApplicationCommands) {
    cmds.create_application_command(|cmd| {
        cmd.name("calc")
            .description("Calls the C-style arbitrary precision calculator")
            .create_option(|opt| {
                opt.name("input")
                    .description("Command line input to calculator")
                    .kind(ApplicationCommandOptionType::String)
                    .required(true)
            })
    });
}
