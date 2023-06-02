use anyhow::{anyhow, bail};
use nom::{
    branch::alt,
    bytes::{complete::tag as t, complete::is_a, streaming::is_not},
    character::complete::{alphanumeric1, multispace0 as ws},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};
use std::{ffi::CString, str::FromStr};
use syscalls::{
    raw::{syscall0, syscall1, syscall2, syscall3, syscall4, syscall5, syscall6},
    Sysno,
};

#[cfg(test)]
mod tests;

#[cfg(not(target_os = "linux"))]
compile_error!("only Linux is supported at the moment!");

pub fn the_syscall(arg: String) -> String {
    match the_syscall_impl(&arg) {
        Ok(s) => s,
        Err(s) => format!("{s}"),
    }
}

fn the_syscall_impl(arg: &str) -> Result<String, anyhow::Error> {
    let (_, (syscall, args)) = parse_syscall(arg)?;

    let sysno = Sysno::from_str(syscall).map_err(|()| anyhow!("ENOSYS"))?;

    unsafe {
        Ok(format!(
            "{}",
            match args.len() {
                0 => syscall0(sysno),
                1 => syscall1(sysno, args[0]),
                2 => syscall2(sysno, args[0], args[1]),
                3 => syscall3(sysno, args[0], args[1], args[2]),
                4 => syscall4(sysno, args[0], args[1], args[2], args[3]),
                5 => syscall5(sysno, args[0], args[1], args[2], args[3], args[4]),
                6 => syscall6(sysno, args[0], args[1], args[2], args[3], args[4], args[5]),
                _ => bail!("too many args"),
            }
        ))
    }
}

pub(crate) fn parse_syscall(i: &str) -> Result<(&str, (&str, Vec<usize>)), anyhow::Error> {
    delimited(ws, separated_pair(alphanumeric1, ws, syscall_args), ws)(i).map_err(|e| anyhow!("syntax error: {e}"))
}

fn syscall_args(i: &str) -> IResult<&str, Vec<usize>> {
    delimited(t("("), separated_list0(delimited(ws, t(","), ws), syscall_arg), t(")"))(i)
}

fn syscall_arg(i: &str) -> IResult<&str, usize> {
    alt((string_arg, hex_int, dec_int))(i)
}

fn string_arg(i: &str) -> IResult<&str, usize> {
    map(delimited(t("\""), is_not("\""), t("\"")), |s| {
        CString::new(s).unwrap().into_raw() as usize
    })(i)
}

fn hex_int(i: &str) -> IResult<&str, usize> {
    map_res(
        preceded(
            alt((t("0x"), t("0X"))),
            is_a("0123456789abcdefABCDEF"),
        ),
        |out: &str| usize::from_str_radix(out, 16),
    )(i)
}

fn dec_int(i: &str) -> IResult<&str, usize> {
    map_res(is_a("0123456789"), usize::from_str)(i)
}
