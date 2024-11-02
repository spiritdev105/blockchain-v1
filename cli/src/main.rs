use types::Error;
use crate::cli::RunCmd;

mod cli;

fn main() -> Result<(), Error> {
    RunCmd::run()
}
