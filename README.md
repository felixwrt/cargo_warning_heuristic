# Cargo Warning Suppression Heuristic

Simple repo showing that cargo suppresses warnings from dependencies if they are pulled in via git. If the same dependency is used as a path dependency instead, warnings are shown.

This repo contains three crates:
- `crate_with_warnings`: contains a warning
- `main_crate_git_dependency`: uses `crate_with_warning` as a git dependency
- `main_crate_path_dependency`: uses `crate_with_warning` as a path dependency

Building `crate_with_warnings` displays a warning:

```
$ cargo build -p crate_with_warnings
warning: unused variable: `right`
 --> crate_with_warnings/src/lib.rs:3:25
  |
3 | pub fn add(left: usize, right: usize) -> usize {
  |                         ^^^^^ help: if this is intentional, prefix it with an underscore: `_right`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `crate_with_warnings` (lib) generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
```

Building `main_crate_path_dependency` shows the same warning:

```
$ cargo build -p main_crate_path_dependency
warning: unused variable: `right`
 --> crate_with_warnings/src/lib.rs:3:25
  |
3 | pub fn add(left: usize, right: usize) -> usize {
  |                         ^^^^^ help: if this is intentional, prefix it with an underscore: `_right`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `crate_with_warnings` (lib) generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
```

When using a git dependency instead (`main_crate_git_dependency`), warnings are suppressed by cargo:

```
$ cargo build -p main_crate_git_dependency
   Compiling crate_with_warnings v0.1.0 (https://github.com/felixwrt/cargo_warning_heuristic.git?branch=main#b5085bc1)
   Compiling main_crate_git_dependency v0.1.0 (.../rust-warning-heuristic/main_crate_git_dependency)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
```