# Advent of Code 2023 solutions

To run the solutions:

```
$ cargo run
```

This assumes [Cargo][cargo] is installed, and that the input files are placed at
`inputs/dayXX.in` relative to the current working directory.

To run an individual day, specify the day as a command line argument:

```
$ cargo run 1
```

To run with a different input, specify a file name as a command line argument.
The file name `-` means standard input:

```
$ cargo run 1 foo.txt
$ cargo run 1 - < foo.txt
```

To run the benchmarks:

```
$ cargo bench
```


## License

This is [free and unencumbered software released into the public domain][unlicense].


[cargo]: https://doc.rust-lang.org/stable/cargo/
[unlicense]: https://unlicense.org/
