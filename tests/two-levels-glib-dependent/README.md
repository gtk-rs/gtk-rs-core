# glib-rs Two Levels Dependent Test

This repository is intended at checking `glib` macro
re-export detection in two levels dependencies.

The detection mechanism used to stop at the first identified crate known
to re-export `glib` and which was found in `Cargo.toml`. When used in a crate
that depends both on `gstreamer` and `gtk` and `gtk` is optional, the detection
mechanism stopped at `gtk` and prepended `gtk` to `glib`, leading to errors
compiling `glib` proc-macros.