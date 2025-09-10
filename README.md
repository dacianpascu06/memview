# memview

`memview` is a Rust-based command-line tool that allows you to inspect the memory layout of running processes. It shows the virtual memory areas (VMAs) of a process, including start addresses, size, and physical memory mappings. It also supports live updates with Git-style diffs to track changes in memory mappings.

---

## Features

- View memory mappings of a process by name or PID.
- Display VMA intervals and their corresponding physical addresses.
- Real-time updates with Git-style diff highlighting changes.
- Lightweight CLI built in Rust.
- Terminal interface powered by [`ratatui`](https://crates.io/crates/ratatui).
- Works with root privileges for full memory access.

---

## Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/memview.git
cd memview
```

2. Build with Cargo:

```bash
cargo build --release
```

3. Run the binary:

```bash
sudo ./target/release/memview -n processname
# or
sudo ./target/release/memview pid
```

## Example: memview-demo

`memview-demo` is a small C program demonstrating how `mmap` works: it reserves virtual memory space but only allocates physical memory when a page fault occurs (i.e., when a page is accessed). You can use `memview` to watch this behavior in real time.

---

### Build

Navigate to the `examples` directory and compile the demo:

```bash
cd examples
make
```

Run the demo:

```bash
./memview-demo
```

Then, in another terminal, you can use memview to observe the memory mappings:

```bash
sudo ../target/release/memview -n memview-demo
```
