Intel 8080
==========

This repository contains an emulator for the Intel 8080 processor.

Ressource:
http://www.classiccmp.org/dunfield/r/8080.txt


Use this repo:
--------------

You must include this repository as a library in your code:
```
rust-8080 = { git = "https://github.com/irevoire/rust-8080.git" }
```

The only existing functions are: `Cpu::from_filename(&str)` and `Cpu::from_bytes(Vec<u8>)`.
Once you created a `Cpu` struct you can only call the `cycle` method which execute one CPU cycle.
