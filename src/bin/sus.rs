use std::io::Write;

use anyhow::Error;

use stringos_from_userspace::the_syscall;

fn prompt() -> Result<(), Error> {
    print!("> ");
    std::io::stdout().flush()?;
    Ok(())
}

fn main() -> Result<(), Error> {
    prompt()?;
    for line in std::io::stdin().lines() {
        println!("{}", the_syscall(line?));
        prompt()?;
    }
    Ok(())
}
