pub mod setup;

use std::{
    error::Error,
    io::{Read, Write},
    process::{Command, Stdio},
    time::Duration,
};

use serenity::{
    builder::CreateEmbed,
    client::Context,
    model::interactions::application_command::{
        ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
    },
};

use tempfile::Builder;
use wait_timeout::ChildExt;

enum CalcOut {
    Stdio(String, String),
    Error(String),
}

pub async fn calc_ti_call(
    int: ApplicationCommandInteraction,
    ctx: Context,
) -> Result<(), Box<dyn Error>> {
    let input_option = match int.data.options.get(0) {
        Some(input_arg) => match input_arg.resolved.as_ref() {
            Some(ApplicationCommandInteractionDataOptionValue::String(input)) => {
                Some(input.clone())
            }
            _ => None,
        },
        _ => None,
    };
    let response = match input_option {
        Some(_) => "Starting the calc process...",
        _ => "An input must be supplied",
    };
    int.create_interaction_response(&ctx.http, |resp| {
        resp.interaction_response_data(|data| data.content(response))
    })
    .await?;
    if let Some(input) = input_option {
        calc_followup(int, ctx, call_calc(input)).await?;
    }
    Ok(())
}

fn call_calc(input: String) -> CalcOut {
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
                                let output = String::from_utf8_lossy(&out).trim().to_owned();
                                let errput = String::from_utf8_lossy(&err).trim().to_owned();
                                CalcOut::Stdio(output, errput)
                            }
                            _ => CalcOut::Error("Failed to read IO handles from calc.".to_string()),
                        }
                    }
                    _ => CalcOut::Error("Failed to obtain IO handles for calc.".to_string()),
                },
                None => match child.kill() {
                    Ok(_) => CalcOut::Error("Calc program timed out.".to_string()),
                    _ => CalcOut::Error(
                        "Calc program timed out, and could not be killed.".to_string(),
                    ),
                },
            },
            _ => CalcOut::Error("Failed to wait for calc timeout.".to_string()),
        },
        _ => CalcOut::Error("Failed to launch calc program.".to_string()),
    }
}

async fn calc_followup(
    int: ApplicationCommandInteraction,
    ctx: Context,
    result: CalcOut,
) -> Result<(), Box<dyn Error>> {
    match result {
        CalcOut::Stdio(out, err) => match out.len() < 25600 && err.len() < 25600 {
            true => {
                let mut stdout_embed = CreateEmbed::default();
                let mut stderr_embed = CreateEmbed::default();
                stdout_embed.title("Calc STDOUT:");
                stderr_embed.title("Calc STDERR:");
                out.chars()
                    .collect::<Vec<char>>()
                    .chunks(1024)
                    .enumerate()
                    .for_each(|(i, chunk)| {
                        stdout_embed.field(format!("Chunk {}", i), chunk.iter().collect::<String>(), true);
                    });
                err.chars()
                    .collect::<Vec<char>>()
                    .chunks(1024)
                    .enumerate()
                    .for_each(|(i, chunk)| {
                        stderr_embed.field(format!("Chunk {}", i), chunk.iter().collect::<String>(), true);
                    });
                int.create_followup_message(ctx.http, |followup| {
                    if !out.is_empty() {
                        followup.add_embed(stdout_embed.clone());
                    }
                    if !err.is_empty() {
                        followup.add_embed(stderr_embed.clone());
                    }
                    followup
                })
                .await?;
            }
            false => {
                let mut tmp_content = Builder::new().suffix(".txt").tempfile()?;
                writeln!(
                    tmp_content.as_file_mut(),
                    "Calc STDOUT:\n{}\nCalc STDERR:\n{}",
                    out,
                    err
                )?;
                int.create_followup_message(ctx.http, |followup| {
                    followup.add_file(tmp_content.path())
                })
                .await?;
            }
        },
        CalcOut::Error(err) => {
            int.create_followup_message(ctx.http, |followup| followup.content(err))
                .await?;
        }
    }
    Ok(())
}
