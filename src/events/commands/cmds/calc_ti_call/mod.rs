pub mod setup;

use std::{
    error::Error,
    io::{Read, Write},
    process::{Command, Stdio},
    thread::sleep,
    time::{Duration, Instant},
};

use serenity::{
    client::Context,
    model::interactions::application_command::{
        ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
    },
    utils::MessageBuilder,
};

use tempfile::Builder;

pub async fn calc_ti_call(
    int: ApplicationCommandInteraction,
    ctx: Context,
) -> Result<(), Box<dyn Error>> {
    let input_option = match int.data.options.get(0) {
        Some(input_arg) => match input_arg.resolved.as_ref() {
            Some(ApplicationCommandInteractionDataOptionValue::String(input)) => Some(input),
            _ => None,
        },
        _ => None,
    };
    let mut builder = MessageBuilder::new();
    int.create_interaction_response(&ctx.http, |resp| {
        resp.interaction_response_data(|data| data.content("Starting the calc process..."))
    })
    .await?;
    match input_option {
        Some(input) => match Command::new("calc")
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .arg(input)
            .spawn()
        {
            Ok(mut child) => {
                let start = Instant::now();
                let max_dur = Duration::from_secs(15);
                while child.try_wait().is_err() && start.elapsed() < max_dur {
                    sleep(Duration::from_millis(150));
                }
                match child.try_wait() {
                    Ok(_) => match child.stdout.take() {
                        Some(mut child_out) => {
                            let mut buf = Vec::new();
                            child_out.read_to_end(&mut buf)?;
                            builder.push(String::from_utf8_lossy(&buf).trim());
                        }
                        None => {
                            builder.push("Unable to pipe calc output.");
                        }
                    },
                    _ => {
                        builder.push("Calc took too long.");
                        child.kill()?;
                    }
                }
            }
            _ => {
                builder.push("Calc failed to start.");
            }
        },
        _ => {
            builder.push("You need to give some input to the program!");
        }
    }
    let message = builder.build();
    let content = match message.is_empty() {
        true => "No output was obtained, other than erros.".to_string(),
        false => message,
    };
    let mut tmp_content = Builder::new().suffix(".txt").tempfile()?;
    tmp_content.as_file_mut().write_all(content.as_bytes())?;
    int.create_followup_message(ctx.http, |followup| followup.add_file(tmp_content.path()))
        .await?;
    Ok(())
}
