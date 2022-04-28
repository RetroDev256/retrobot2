pub mod setup;

use std::{
    error::Error,
    io::{Read, Write},
    process::{Command, Stdio},
    time::Duration,
};

use serenity::{
    client::Context,
    model::interactions::application_command::{
        ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
    },
};

use tempfile::Builder;
use wait_timeout::ChildExt;

pub async fn calc_ti_call(
    int: ApplicationCommandInteraction,
    ctx: Context,
) -> Result<(), Box<dyn Error>> {
    let input = match int.data.options.get(0) {
        Some(input_arg) => match input_arg.resolved.as_ref() {
            Some(ApplicationCommandInteractionDataOptionValue::String(input)) => {
                Some(input.clone())
            }
            _ => None,
        },
        _ => None,
    }
    .expect("Input was not supplied.");
    int.create_interaction_response(&ctx.http, |resp| {
        resp.interaction_response_data(|data| data.content("Starting the calc process..."))
    })
    .await?;
    let message = call_calc(input);
    let content = match message.is_empty() {
        true => "No output was obtained.".to_string(),
        false => message,
    };
    match content.len() <= 2000 {
        true => {
            int.create_followup_message(ctx.http, |followup| followup.content(content))
                .await?;
        }
        false => {
            let mut tmp_content = Builder::new().suffix(".txt").tempfile()?;
            tmp_content.as_file_mut().write_all(content.as_bytes())?;
            int.create_followup_message(ctx.http, |followup| followup.add_file(tmp_content.path()))
                .await?;
        }
    }
    Ok(())
}

fn call_calc(input: String) -> String {
    let child_opt = Command::new("calc")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg(input)
        .spawn();
    let max_duration = Duration::from_secs(15);
    match child_opt {
        Ok(mut child) => match child.wait_timeout(max_duration) {
            Ok(exit) => match exit {
                Some(_status) => match (child.stdout.take(), child.stderr.take()) {
                    (Some(mut c_stdout), Some(mut c_stderr)) => {
                        let (mut out, mut err) = (vec![], vec![]);
                        match (
                            c_stdout.read_to_end(&mut out),
                            c_stderr.read_to_end(&mut err),
                        ) {
                            (Ok(_), Ok(_)) => {
                                let output = String::from_utf8_lossy(&out);
                                let errput = String::from_utf8_lossy(&err);
                                match (out.is_empty(), err.is_empty()) {
                                    (false, false) => {
                                        format!("STDOUT:\n{}\nSTDERR:\n{}", output, errput)
                                    }
                                    (false, true) => {
                                        format!("STDOUT:\n{}", output)
                                    }
                                    (true, false) => {
                                        format!("STDERR:\n{}", errput)
                                    }
                                    (true, true) => "No output was obtained from calc.".to_string(),
                                }
                            }
                            _ => "Failed to read IO handles from calc.".to_string(),
                        }
                    }
                    _ => "Failed to obtain IO handles for calc.".to_string(),
                },
                None => match child.kill() {
                    Ok(_) => "Calc program timed out.".to_string(),
                    _ => "Calc program timed out, and could not be killed.".to_string(),
                },
            },
            _ => "Failed to wait for calc timeout.".to_string(),
        },
        _ => "Failed to launch calc program.".to_string(),
    }
}
