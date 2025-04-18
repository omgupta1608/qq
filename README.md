# qq

A simple and minimal command-line todo manager written in Rust.

---

##  Introduction

`qq` is a CLI-based personal todo tracker that helps you manage daily tasks with a focus on simplicity. It stores your tasks in a local config file, organized by date, and supports basic operations like adding, completing, and reviewing your daily and spillover tasks.

---

##  Description

- Tasks are stored by date (local time) (e.g., `19 April 2025`) in a config file using the [`confy`](https://crates.io/crates/confy) crate.
- Tasks can be added with content, marked as done, and listed.
- "Spillover tasks" are tasks from previous days that were not marked as done.
- All tasks are persisted automatically — no manual saving needed.

---

##  Installation

1. Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.

2. Clone the repo and build it:

```bash
git clone https://github.com/your-username/qq.git
cd qq
cargo build --release
```

## Usage

```
qq
```
Output:
```
Today is: 19 April 2025

no items found for today


--- SPILL OVERS

No spillover tasks 🎉
```

### Add a new task
```
qq add "Write README for qq"
```
Output:
```
Today is: 19 April 2025

1. Write README for qq


--- SPILL OVERS

No spillover tasks 🎉
```

### Mark a task as done (today's task)
```
qq done 2
```

### Mark a spillover task as done (from previous dates)
```
qq done --spill-over 1
```

### Get the config file path
```
qq about
```

### Reset config
Resets config and all data is deleted
```
qq reset
```
