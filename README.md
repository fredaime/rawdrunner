# rawdrunner

Experimental bare metal loader.

## Workspace Layout

```
rawdrunner/
├── Cargo.toml          # workspace definition
├── .cargo/config.toml  # default build target
├── linker.ld           # simple memory layout
├── kernel/             # kernel crate
│   └── src/main.rs     # prints "Hello World"
└── modules/
    └── example/        # example module crate
```

The workspace builds a minimal kernel for `x86_64-unknown-none` that writes
`Hello World` to the VGA text buffer.
