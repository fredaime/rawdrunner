# rawdrunner

Rawdrunner is an experimental workspace for bare metal Rust
projects.  It contains a minimal "Hello World" kernel as well
as a more feature rich prototype called `aetherum-kernel`.
Both are built for the `x86_64-unknown-none` target and can be
booted with QEMU.

## Repository layout

```
rawdrunner/
├── Cargo.toml          # workspace definition
├── .cargo/config.toml  # default build target and runner
├── linker.ld           # simple memory layout for the basic kernel
├── kernel/             # small example kernel
├── modules/            # out-of-tree modules used by the kernel
└── aetherum-kernel/    # prototype OS with scheduler, memory mgmt, …
```

### Kernel
The `kernel` crate is intentionally tiny. Its `main` function
writes `Hello World` directly to the VGA text buffer using no
standard library.  An example module is included in
`modules/example` and linked as a static dependency.

### aetherum-kernel
The `aetherum-kernel` directory contains a much more advanced
prototype.  It is split into several subsystems:

- **arch** – initialization code for supported architectures.
  Only `x86_64` is implemented; an ARM64 stub exists for future work.
- **memory** – boot-time frame allocator and a very small `kmalloc`
  implementation backed by a static heap.
- **scheduler** – cooperative task scheduler with a run queue that
  predicts CPU burst length for simple prioritisation.
- **telemetry** – ring buffer that records events such as task
  switches using `rdtsc` timestamps.

Booting `aetherum-kernel` will initialise these subsystems, spawn an
example task and then run the scheduler.

## Building
The workspace requires **Rust nightly** with the `rust-src` component
and the `x86_64-unknown-none` target installed.  For the advanced
kernel, additional tools like `cargo-xbuild`, `bootimage`, `xorriso`
and `qemu-system-x86_64` are needed.

```
# install target and tooling
rustup component add rust-src
rustup target add x86_64-unknown-none
cargo install bootimage cargo-xbuild
```

### Minimal kernel
Compile the small example kernel with:

```
cargo build -p kernel
```

Running it (`cargo run -p kernel`) will attempt to launch QEMU using
the runner defined in `.cargo/config.toml`.

### aetherum-kernel
A `Makefile` is provided for convenience.  It builds an ISO image and
boots it in QEMU:

```
cd aetherum-kernel
make run
```

`make bootimage` may be used if only the bootable binary is desired.

## Status
Both kernels are experimental and primarily serve as a playground for
low level Rust development.  The code can be built successfully, but
no stable API should be assumed.
