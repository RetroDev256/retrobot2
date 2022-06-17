use serenity::{builder::CreateApplicationCommands, model::prelude::command::CommandOptionType};

pub fn msg_digest_setup(cmds: &mut CreateApplicationCommands) {
    cmds.create_application_command(|cmd| {
        cmd.name("msg_digest")
            .description("512 bit, 2 round arb_digest digest of a string")
            .create_option(|opt| {
                opt.name("input")
                    .description("String to compute digest of")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
    });
}
