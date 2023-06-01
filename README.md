<img align="center" src="https://blog.sunfishcode.online/StringOS.png" alt="String OS logo">
... from User Space!

# Introduction

Did you read about the amazing [String OS](https://blog.sunfishcode.online/measuring-system-interface-complexity/)?
Did you find yourself longing for its amazingly simple and elegant API?
Are you stuck on Linux like the rest of us?
Do you worry  that systems software research has become irrelevant?

Worry no more! String OS from User Space is here!

# What is it?

A port of the String OS API to Linux, as a Rust crate.

# API Reference

```rs
fn the_syscall(arg: String) -> String;
```

# Usage

```rs
the_syscall("write(1, \"hello, world!\", 13)");
```

# What if I'm not a Rust crate?

No problem! You can use the **S**tringOS from **U**serspace **S**hell (sus), like so:

```sh
cargo run --bin sus
> sync()
0
> getppid()
3125
> 
```

# Semver Policy

100% compatibility is guaranteed for eternity.
String OS from userspace will never add or remove a syscall, nor change the function signature of a syscall in any way.

There might be changes to the input and output formatting of `the_syscall`, but your code will always keep compiling no matter what!

