# GIO VFS

This example demonstrates the usage of GIO VFS. Built artifact is a dynamic system library that is used as a GIO module
(see https://docs.gtk.org/gio/overview.html#running-gio-applications)
and that implement support of file operations for files with uri starting with `myvfs:///`

Build, install and configure it by executing:
```bash
cargo build -p gtk-rs-examples --lib
export GIO_EXTRA_MODULES=/tmp/gio_modules
mkdir -p $GIO_EXTRA_MODULES && cp ./target/debug/libgio_vfs.so $GIO_EXTRA_MODULES
export MYVFS_ROOT=/tmp/myvfs
mkdir -p $MYVFS_ROOT
```

`GIO_EXTRA_MODULES` specify additional directories for `gio` command line tool to automatically load modules.

`MYVFS_ROOT` specify the local directory that is used by as backend directory for uri starting with `myvfs:///` (e.g. if `MYVFS_ROOT-/tmp` `myvfs:///foo` points to `/tmp/foo`).

`gio` commandline tool (see https://gnome.pages.gitlab.gnome.org/libsoup/gio/gio.html) automatically loads this extra module.

Run it by executing the following commands:

Basic operations:
```bash
echo "foo" | gio save myvfs:///foo
gio cat myvfs:///foo
gio set -t string myvfs:///foo xattr::my_string value
gio info myvfs:///foo
gio mkdir myvfs:///bar
gio copy myvfs:///foo myvfs:///bar/
gio list myvfs:///
gio tree myvfs:///
gio move -b myvfs:///bar/foo myvfs:///foo
gio tree myvfs:///
gio remove myvfs:///foo myvfs:///foo~ myvfs:///bar
gio list myvfs:///
```

Monitor `myvfs:///`:
```bash
# monitor is a blocking operation. kill it with Ctrl+C
gio monitor myvfs:///
```

```bash
# in another terminal (ensure MYVFS_ROOT is defined)
touch $MYVFS_ROOT/foo
echo "foo" > $MYVFS_ROOT/foo
mkdir $MYVFS_ROOT/bar
cp $MYVFS_ROOT/foo $MYVFS_ROOT/foo2
mv -b $MYVFS_ROOT/foo2 $MYVFS_ROOT/foo
rm -rf $MYVFS_ROOT/*
```
