use std::{env, process::ExitCode};

use lrcli::{cmd_search, print_usage};

fn entry() -> Result<(), ()> {
    let mut args = env::args();
    let program = args.next().unwrap_or("lrcli".into());
    let action = args.next().ok_or_else(|| print_usage(&program))?;
    match action.as_str() {
        "search" => {
            cmd_search(&program, args.collect())?;
        }
        _ => {
            print_usage(&program);
        }
    }

    Ok(())
}

fn main() -> ExitCode {
    match entry() {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
