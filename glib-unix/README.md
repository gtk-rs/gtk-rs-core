# Rust GLib Unix bindings

__Rust__ bindings and wrappers for [glib](https://docs.gtk.org/glib/), part of [gtk-rs-core](https://github.com/gtk-rs/gtk-rs-core).

GLib __2.56__ is the lowest supported version for the underlying library.

## Minimum supported Rust version

Currently, the minimum supported Rust version is `1.83.0`.

## Documentation

 * [Rust API - Stable](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib_unix/)
 * [Rust API - Development](https://gtk-rs.org/gtk-rs-core/git/docs/glib_unix)
 * [GTK Installation instructions](https://www.gtk.org/docs/installations/)

## Using

We recommend using [crates from crates.io](https://crates.io/keywords/gtk-rs),
as [demonstrated here](https://gtk-rs.org/#using).

If you want to track the bleeding edge, use the git dependency instead:

```toml
[dependencies]
glib = { git = "https://github.com/gtk-rs/gtk-rs-core.git", package = "glib" }
```

Avoid mixing versioned and git crates like this:

```toml
# This will not compile
[dependencies]
glib = "0.13"
glib = { git = "https://github.com/gtk-rs/gtk-rs-core.git", package = "glib" }
```

### See Also

 * [gio](https://crates.io/crates/gio)

## License

__glib-unix__ is available under the MIT License, please refer to it.
