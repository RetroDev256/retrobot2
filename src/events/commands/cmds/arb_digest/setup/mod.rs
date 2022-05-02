use serenity::{
    builder::CreateApplicationCommands,
    model::interactions::application_command::ApplicationCommandOptionType,
};

pub fn arb_digest_setup(cmds: &mut CreateApplicationCommands) {
    cmds.create_application_command(|cmd| {
        cmd.name("digest")
            .description("Digest of a file")
            .create_option(|opt| {
                opt.name("input")
                    .description("File to compute digest of")
                    .kind(ApplicationCommandOptionType::Attachment)
                    .required(true)
            })
            .create_option(|opt| {
                opt.name("bytes")
                    .description("Number of bytes for the digest")
                    .kind(ApplicationCommandOptionType::Integer)
                    .required(true)
                    .min_int_value(1)
                    .max_int_value(4194304)
            })
            .create_option(|opt| {
                opt.name("rounds")
                    .description("Number of rounds for the digest")
                    .kind(ApplicationCommandOptionType::Integer)
                    .required(true)
                    .min_int_value(1)
                    .max_int_value(256)
            })
    });
}
