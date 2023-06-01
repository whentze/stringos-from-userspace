use anyhow::Error;

use stringos_from_userspace::the_syscall;

fn main() -> Result<(), Error> {
    for line in std::io::stdin().lines() {
        println!("{}", the_syscall(line?));
    }
    Ok(())
}
