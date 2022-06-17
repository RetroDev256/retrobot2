use serenity::{builder::CreateApplicationCommands, model::prelude::command::CommandOptionType};

pub fn arb_digest_setup(cmds: &mut CreateApplicationCommands) {
    cmds.create_application_command(|cmd| {
        cmd.name("digest")
            .description("512 bit, 2 round arb_digest digest of a file")
            .create_option(|opt| {
                opt.name("input")
                    .description("File to compute digest of")
                    .kind(CommandOptionType::Attachment)
                    .required(true)
            })
    });
}
