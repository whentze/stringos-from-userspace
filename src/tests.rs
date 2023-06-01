use crate::the_syscall;
use std::str::FromStr;

#[test]
fn getpid() {
    assert!(usize::from_str(&the_syscall("getpid()".into())).unwrap() as isize > 0);
}

#[test]
fn open() {
    assert!(usize::from_str(&the_syscall(r#"open("/", 0)"#.into())).unwrap() as isize > 0);
}
