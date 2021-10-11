pub mod spec;
pub mod state;
pub mod tasks;

use self::spec::{Spec, SpecError};
use self::state::State;
use self::tasks::{Task, TaskExecutionError};
use clap::{App, Arg};

// TODO: implement documentation in idiomatic rust way.

fn main() {
    let version = env!("CARGO_PKG_VERSION");

    let matches = App::new("Pipeline automation tool")
        .version(version)
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Pipeline definition file")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let filename = matches.value_of("file").unwrap();
    let state = State::new();

    match Spec::try_new(filename) {
        Ok(spec) => {
            for task in spec.tasks() {
                let name = task.name();

                match task.execute() {
                    Ok(_) => {
                        state.set();
                    }
                    Err(TaskExecutionError(msg)) => {
                        eprintln!("failed to execute task {} - {}", name, msg);

                        break;
                    }
                }
            }
        }
        Err(SpecError::FileError) => {
            eprintln!("failed to open {} file", filename);
        }
        Err(SpecError::SyntaxError) => {
            eprintln!("failed to parse yaml file");
        }
        Err(SpecError::ValidationError(msg)) => {
            eprintln!("spec validation error - {}", msg);
        }
    }
}
