pub mod setup;

use std::{
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

use crate::tools::filter_pings;

use super::get_element;

enum CalcOut {
    Stdio(String, String),
    Error(String),
}

pub async fn calc_ti_call(int: ApplicationCommandInteraction, ctx: Context) {
    let input = match get_element(&int, 0) {
        ApplicationCommandInteractionDataOptionValue::String(input) => Some(input),
        _ => None,
    }
    .unwrap();
    int.create_interaction_response(&ctx.http, |resp| {
        resp.interaction_response_data(|data| data.content("Starting the calc process..."))
    })
    .await
    .unwrap();
    calc_followup(&int, ctx, call_calc(input.to_owned())).await;
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
                            (Ok(_), Ok(_)) => match out.is_empty() && err.is_empty() {
                                false => CalcOut::Stdio(
                                    String::from_utf8_lossy(&out).trim().to_owned(),
                                    String::from_utf8_lossy(&err).trim().to_owned(),
                                ),
                                true => CalcOut::Error("No output recieved from calc.".to_string()),
                            },
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

async fn calc_followup(int: &ApplicationCommandInteraction, ctx: Context, result: CalcOut) {
    match result {
        CalcOut::Stdio(out, err) => {
            if out.len() <= 1983 && err.len() <= 1983 {
                for (name, io) in [("OUT", out), ("ERR", err)] {
                    if !io.is_empty() {
                        int.create_followup_message(&ctx.http, |followup| {
                            followup.content(filter_pings(&format!("```\nSTD{}:\n{}```", name, io)))
                        })
                        .await
                        .unwrap();
                    }
                }
            } else {
                let mut tmp_content = Builder::new().suffix(".txt").tempfile().unwrap();
                writeln!(
                    tmp_content.as_file_mut(),
                    "Calc STDOUT:\n{}\nCalc STDERR:\n{}",
                    out,
                    err
                )
                .unwrap();
                int.create_followup_message(ctx.http, |followup| {
                    followup.add_file(tmp_content.path())
                })
                .await
                .unwrap();
            }
        }
        CalcOut::Error(err) => {
            int.create_followup_message(ctx.http, |followup| followup.content(filter_pings(&err)))
                .await
                .unwrap();
        }
    }
}
