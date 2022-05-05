use serenity::{
    builder::CreateApplicationCommands,
    model::interactions::application_command::ApplicationCommandOptionType,
};

pub fn msg_digest_setup(cmds: &mut CreateApplicationCommands) {
    cmds.create_application_command(|cmd| {
        cmd.name("digest")
            .description("512 bit, 2 round arb_digest digest of a string")
            .create_option(|opt| {
                opt.name("input")
                    .description("String to compute digest of")
                    .kind(ApplicationCommandOptionType::String)
                    .required(true)
            })
    });
}