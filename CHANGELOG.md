# Change Log

## [Unreleased]
Yuri Izmer:
- glib-macros: Add `derived_properties` macro 

## [0.17.10]
Ben Kimock:
 - Fix heap buffer overflow due to operator precedence

Benji Smith:
- Upgrade plain-HTTP links to HTTPS in Cargo.toml files

Bilal Elmoussaoui:
- pango: Lower pkg-config version requirement for `v1_52`
- gio: Add missing manual traits

Fina Wilke:
- glib-macros: Allow to omit set for `construct_only` properties
- glib-macros: Update docs for the properties macro `construct_only` attribute

Jan Alexander Steffens (heftig):
- glib: Do not use `ptr::offset/offset_from` for private/impl offset

Maximiliano Sandoval R:
- glib: strv: Implement `From` for constant `GStr` slices

Sebastian Dröge:
- Fix a couple of trivial clippy warnings

## [0.17.9]

Paskal Sitepu:
- glib-macros: Strip out `r#` prefix from property names inside the `GObject`
- glib-macros: Add a test for the `Properties` macro where keywords are being used
- glib-macros: Update the doc of the `Properties` macro to elaborate on usage of keywords as property names

Sebastian Dröge:
- glib: Enable various smallvec features
- Fix various new Rust 1.69 clippy warnings
- glib: Fix compiletest expected error output for Rust 1.69

## [0.17.8]

Paskal Sitepu:
- glib-macros: Import `ParamSpecBuilderExt` inside the scope of `DerivedObjectProperties::derived_properties`
- glib-macros: Disambiguate `TryFrom<usize>::Error` for `DerivedPropertiesEnum`
- glib-macros: Strip raw identifier prefix from struct members for the `Properties` macro

Sebastian Dröge:
- glib: Allow using `Path` / `PathBuf` in `glib::Value`s
- glib: Fix inverted boolean conditions when deciding whether to reserve new space
- glib: collections: Change early return to assertion in collection reserve functions

## [0.17.7]

Fina Wilke:
- glib-macros: Specify quoted types explicitly

## [0.17.6]

Fabio Valentini:
- glib-macros: enable default features of syn

SeaDve:
- glib-macros: add docs on supported `#[property]` attributes

## [0.17.5]

Bilal Elmoussaoui:
- glib-macros: Don't assume edition=2021

Johan Bjäreholt:
- glib: Fix building for architectures without 64-bit atomics

SeaDve:
- glib-macros: generate "From<Ident> for Value" on ValueDelegate

## [0.17.4]

Bilal Elmoussaoui:
- Fix nightly clippy warnings

Jason Francis:
- pango: rename removed clippy lint
- glib: update error messages in compiletests for rust 1.68
- glib: implement `From<GStringPtr>` for `GString`

SeaDve:
- glib-macros: qualify `ToValue::to_value`
- glib-macros: support higher level types on `ValueDelegate`
- glib: impl `StaticType`, `FromValue`, `ToValue`, `HasParamSpec` for `Box<str>`

Sebastian Dröge:
- Update gir
- Update gir-files
- Regenerate with latest gir / gir-files
- Require glib 2.76.0 for the `v2_76` feature

## [0.17.3]

Fabio Valentini:
- pangocairo-sys: fix package.description in Cargo.toml

Jason Francis:
- gio: add `FileEnumerator::into_stream`
- glib: Only optimize `IntoGStr` for `String` when capacity allows

Marco Mastropaolo:
- gio: Added subclassing support for `gio::SocketControlMessage`
- gio: Fixed unit tests under macOS and possibly other \*nix flavors

Marco Melorio:
- glib: Add `connect_notify*` methods to `SignalGroup`

Matteo Biggio:
- glib: strv: when calling `g_strv` FFI method, use our `as_ptr` implementation

Sebastian Dröge:
- glib: Optimize `IntoGStr` impl for `String` by simply appending a NUL-byte
- glib: Depend on glib-macros 0.17.3 for `ValueDelegate` derive macro

ranfdev:
- glib: Add `ValueDelegate` derive macro
- glib: impl `HasParamSpec` on `glib::Variant`
- glib: Add nullable attribute on `Properties` macro
- glib: Explain `nullable` attribute

## [0.17.2]

Andrey Kutejko:
- glib: Implement `PartialEq` and `PartialOrd` for `WeakRef`

Bilal Elmoussaoui:
- glib: Add missing `ObjectImpl` vfuncs overrides
- glib: Mention lack of finalize method on `ObjectImpl`

Jason Francis:
- glib-macros: allow properties macro generated functions to be unused
- glib: implement `WatchedObject` for `BorrowedObject`

Mathieu Duponchelle:
- glib: object: address misleading casting docs

Matteo Biggio:
- gio: `SimpleAction`: take ownership of value without leaking it

Paolo Borelli:
- glib: properties: impl `HasParamSpec` for `Vec<String>` and `StrV`
- glib-macros: Derive `HasParamSpec` for `SharedBoxed`
- glib: Add `StrV::join()` method
- glib: Add `StrV::contains()` method

SeaDve:
- glib-macros: slightly improve `Properties` macro docs

Sebastian Dröge:
 - Update gir-files
 - Regenerate with latest gir-files
 - glib: Ignore new 2.76 free functions
 - gio: Don't pass `NULL` to `g_list_store_find_with_equal_func_full()`

## [0.17.1]

Guillaume Desmottes:
- glib: param\_spec: add `ParamSpecEnumBuilder::default_value()`

Mițca Dumitru:
- glib: Make `WeakRef` useable with the `Properties` derive macro
- glib: Make `SendWeakRef` useable with the `Properties` derive macro

Sebastian Dröge:
- glib: Add various `IntoStrV` impls

YuraIz:
- graphene: Implement `Default` trait for vectors, matrix, points and rect

ranfdev:
- glib: properties: improve conversion error reporting inside setter

## [0.17.0]

Aaron Erhardt:
- glib-build-tools: Fix reporting of errors

Alberto Ruiz:
- gdk-pixbuf: check if either width/height is null before assignment in `animation_get_size()`

Andrey Kutejko:
- glib-build-tools: Do not hard-code path separator

Bilal Elmoussaoui:
- gio: `Make GioFuture` handle infallible futures
- glib: Ignore useless new constant
- glib: Implement more `From` traits for `StrV`
- clippy: Drop/move certain lints guards

Guillaume Gomez:
- glib: Greatly improve compiler errors in `clone!` proc macro
- glib: Add span info into `clone!` UI tests

Jason Francis:
- glib: `impl From<T> for Variant` to move into variants
- gio: use `Into<Variant>` for `SettingsExtManual::set`
- add `impl From<T> for Value` for manually implemented types
- glib: use `Into<Value>` instead of `ToValue` when possible
- glib-sys: ignore some unsupported types
- sys: add a few extra preprocessor bits to fix abi tests
- gio: support sending/receiving socket control messages
- gio: implement `DatagramBased`
- gio: implement output stream `writev` methods
- gio: support `writev` in stream impls for `io::Write`, `AsyncWrite`
- cairo: fix some misc warnings
- glib: remove unsafe from `SourceId::as_raw`
- glib: support return values and catching panics in `spawn`/`spawn_local`
- glib: catch panics in `ThreadPool` push methods
- gio: add `spawn_blocking`
- gio: fix clippy lints for rust 1.64
- glib: Add `#[inline]` to more `GString` trait methods
- glib: implement `Display for GStr`
- glib: rename `GStr::to_byte`s to `as_bytes`
- glib: refactor `GStr` and `GString` constructors
- glib: bump required Rust version to 1.64
- glib: make some `GStr` methods `const`
- glib: have `ToGlibPtr` check interior string nuls in debug mode
- glib: add more `From` implementations for `GString`
- glib: add `ToGlibPtr` implementations for more string types
- glib: add `GStringBuilder::as_gstr` and `AsRef<GStr>`
- glib: add `gformat!` for directly formatting into a `GString`
- glib: add `IntoGStr` traits
- glib: allow `GString` to store small inline strings
- glib: convert some functions to use `IntoGStr`
- gio: bind `GFileDescriptorBased`
- glib: use `strcmp` for `GStringPtr` comparisons
- glib: add `GStr::from_ptr_checked()`

Matteo Biggio:
- glib: implement `ToGlibPtr<*mut _>` for boxed types

Maximiliano Sandoval R:
- gio: application: Return `ExitCode`

Mițca Dumitru:
- glib: Bind more `g_utf8` APIs
- build-tools: Allow passing multiple source dirs to `compile_resources`

Paolo Borelli:
- gio: `ActionEntry`: take proper types instead of strings
- gio: `SimpleAction`: take state by value
- glib-macros: Suggest kebab-case for the error domain in error domain derive macro
- gio: simplify async initable
- Bump MSRV to 1.64 everywhere
- glib-build-tools: fix documentation link
- gio: use `StrV` for the simple proxy resolver API
- gio: use `StrV` for `TlsConnection`
- pango: use the new List api to simplify `reorder_items`
- gio: settings: implement strv setter and getter manually
- gio: use `StrV` for the `file_info` API
- gio: use `GStr` for the manual extension point implementation
- glib: list: mark as transparent and impl TransparentPtr
- glib: Rename `StrVItem` to `GStrPtr` and make it cloneable and transparent
- glib: `key_file`: return `PtrSlice<GStrPtr>`
- glib-macros: further tweak docs
- glib: Rename `GStrPtr` to `GStringPtr`
- glib: `GStringPtr`: remove `impl AsRef`, provide explicit `to_gstr()` and `to_str()`
- glib: strv: make `as_slice` const
- gio: implement `FromIterator` for `ListStore`
- glib-macros: move the test foo object out of the unit test function
- glib: properties: accept `default = value` attribute
- glib-macros: support overrides in the properties macro
- gio: make` ListModel::iter()` infallible

Sebastian Dröge:
- ci: Add 0.16 release
- glib: Add a getter for `ObjectBuilder::type_`
- glib: Add `ObjectSubclass::obj()` as a shorter alias for `instance()`
- glib: Add unsafe bindings to `g_object_run_dispose()`
- Move from `imp.instance()` to `imp.obj()`
- pango: Make `pango::Language::from_string()` infallible
- glib: Document the value guarantees for `ObjectImpl::set_property()` and `property()`
- pango: Autogenerate `Language::get_preferred()`
- glib/gio: Add `v2_76` feature
- cairo: Update to freetype 0.32
- glib: Implement `IntoGlibPtr` for `Option<T>`
- glib: Add `function_name!` macro and make use of that for `glib::BoolError`
- glib: Use actual function name for structured logging and also log source file/line number
- glib: Implement `From<&GStr>` and `From<GString>` for `Cow<GStr>`
- glib: Add `GStr::from_ptr_lossy()` and `GString::from_ptr_lossy()`
- glib: Fix docs typo
- glib: Implement `GStringBuilder` as `BoxedInline` to avoid a useless additional heap allocation
- glib: `GStringBuilder` by construction always returns an UTF-8 string and never a NULL pointer
- glib: Micro-optimize `GStringBuilder` construction to have fewer function calls
- Group imports and use prelude
- Fix various new beta clippy warnings
- Use `PhantomData` as `Stash::Storage` if nothing has to be stored except for a lifetime
- glib: Don't create a temporary copy of `&GStr` for `ToGlibPtr::to_glib_none()`
- glib: Optimize `[Type]::to_glib_none()` to not have an additional heap allocation
- glib: Only use a single temporary `Vec` for `[T]::to_glib_none()`
- glib: Make sure to keep the original values alive in `[T]::to_glib_none()`
- glib: Remove useless `UserDirectory::NDirectories` enum variant
- gio: Use `OsStr::to_str()` instead of `OsString::into_string()` to avoid unnecessary heap allocations
- glib: Assert immediately after type registration that the returned type is valid
- Inline various trivial functions
- Change some assertions to debug assertions
- glib: Add new marker traits for transparent types
- glib: Simplify `as_ptr()` implementation and add new function to borrow values from pointers directly
- glib: Make `glib::PtrSlice` API more complete and similar to `Vec`
- glib: Remove a couple of unnecessary trait impls for `glib::ObjectRef`
- glib: Use plain `g_malloc()` instead of `g_malloc0()` if we initialize the result anyway
- glib: Optimize `ToGlibPtrFromSlice` impls for shared/boxed/object/boxed-inline
- glib: Make `glib::Slice` API more complete and similar to `Vec`
- glib: Add zero-copy conversion from `PtrSlice` to `Slice` and back
- glib: Make `glib::SList` and `glib::List` API more complete and similar to `LinkedList`
- glib: Implement `glib::StrV` for `NULL`-terminated string arrays
- glib: Minor optimization for `List`/`SList` with `Copy` types
- glib: Remove unnecessary forwarding of attributes in `glib::wrapper!` for `BoxedInline`
- glib: Fix `glib::wrapper!` for `BoxedInline` with generic parameters
- glib: Remove an unnecessary `FromGlibContainerAsVec` impl
- glib: Optimize various from/to `Vec` FFI translation functions
- glib: Use `IntoGlibPtr` instead of `to_glib_full()` in more places
- glib: Fix `IntoGlibPtr` implementation for `BoxedInline`
- glib: Return a reference to the `ValueArray` pspec element spec
- glib: Get rid of some unnecessary `Option` wrapping
- glib: Add `Value::from_type_unchecked()`
- Use `Value::from_type_unchecked()` where applicable
- glib: Use `IntoGStr` trait in a couple of places
- glib: Fix usage of `gformat!` macro if `GString` is not in scope
- glib: Add `MainContext::spawn_from_within()` for spawning non-`Send` futures from another thread
- gdk-pixbuf: Trust return value nullability
- glib: Add `ParamSpec::is()` helper function
- glib: Deprecate paramspec `new()` functions in favour of the builder
- glib: Deprecate `ObjectSubclass::instance()` / `from_instance()` in favour of the shorter versions
- glib: Make `ObjectBuilder` a bit more efficient
- glib: Implement enum paramspec builder variant that builds the default value automatically
- glib: Add new object constructor for constructing an object with default property values
- glib: Implement `From<&String> for GString`
- glib: Deprecate suboptimal `Object` constructors
- Stop usage of deprecated constructor functions
- gio: Make `(Async)InitableBuilder` public
- glib: Implement various traits on `GStr` manually
- gio: Make sure to have `IntoStrV` in scope for `v2_60` too
- glib: Manually implement `TimeZone::adjust_time()` instead of ignoring it
- ci: Only deny clippy warnings with stable clippy
- glib: Rename `Object::new_default()` to `Object::new()` and remove deprecated API
- gdk-pixbuf: Return a `Option<Duration>` from `AnimationIter::delay_time()`
- gdk-pixbuf: Use `SystemTime` instead of `Duration` for `PixbufAnimation::iter()` and `PixbufAnimationIter::advance()`
- gdk-pixbuf: Ensure that `transfer-none` return values in subclassing are staying alive long enough
- gio: Don't require a `'static` `&str` in `File::enumerate_children_async()` and `enumerate_children_future()`
- glib: Add `NULL` debug assertion to `from_glib_full()` and others for GObjects
- glib: Implement `ValueArray` `Value` traits manually because of the custom paramspec

YuraIz:
- ci: Enable introspection
- ci: Install gobject-introspection-devel into image

ranfdev:
- glib: Add a `#[properties]` derive macro
- glib: Remove `construct_cell`, too experimental
- glib-macros: Improve doc about `Boxed`
- glib: properties: Update syntax for custom flags and other builder fields
- glib: properties: infer inline get type
- glib: properties: `impl Parse` for `ReceivedAttrs`
- glib: properties: improve errors of missing properties, refactor
- glib: properties: impl `PropertyGet` for `T: HasParamSpec`
- glib: properties: Improve properties macro docs

## [0.16.7]

Paolo Borelli:
- examples: spawn async gio task on the current thread context

Sebastian Dröge:
- Fix various new beta clippy warnings

## [0.16.6]

Bilal Elmoussaoui:
- gio: Add a `GioInfallibleFuture`

Sebastian Dröge:
- glib: Implement `IntoGlibPtr` for `Option<T>`
- glib: Implement `From<&GStr>` and `From<GString>` for `Cow<GStr>`
- glib: Fix docs typo

## [0.16.5]

Bilal Elmoussaoui:
- pango: Auto generate `Language`
- pango: Manually implement `Language::to_string()` & `Language::default()`

Sebastian Dröge:
- pango: Make `pango::Language::from_string()` infallible

ranfdev:
 - glib: Add `CastNone` trait

## [0.16.4]

Bilal Elmoussaoui:
- gdk-pixbuf: Add `PixbufAnimation` subclassing support
- gdk-pixbuf: Correct `PixbufAnimationIter` definition
- gdk-pixbuf: Add `PixbufAnimationIter` subclassing support
- gdk-pixbuf: Add `PixbufLoader` subclassing support

Colin Walters:
- glib: Add a doc string for `as_ptr` generated impls

Jason Francis:
- glib: fix undefined behavior in `types::register_type()`
- ci: update windows CI to glib 2.74

Sebastian Dröge:
- glib: Document the value guarantees for `ObjectImpl::set_property()` and `property()`

## [0.16.3]

Aaron Erhardt:
- image: Rebuild once every week

Colin Walters:
- gio: Implement `g_cancellable_set_error_if_cancelled()` manually to work
  around non-standard `GError` behaviour

Guillaume Gomez:
- Fix clippy lints

Jason Francis:
- glib: fix more clippy lints for rust 1.66
- glib: update compiletest output errors for rust 1.65

Sebastian Dröge:
- Move all code from `imp.instance()` to `imp.obj()`
- gio: Require glib 0.16.2 for `ObjectSubclass::obj()`

## [0.16.2]

Bilal Elmoussaoui:
- gio: Add helpers for setting `SettingBinding` flags
- glib: Fix docs links
- gio: Add `set_only`/`get_only` helpers to `BindingBuilder`

Sebastian Dröge:
- glib: Add `ObjectSubclass::obj()` as a shorter alias for `instance()`
- glib: Add unsafe bindings to `g_object_run_dispose()`

anteater:
- Correct outdated references to `subclass::simple`

## [0.16.1]

Bilal Elmoussaoui
- glib: Add helpers for setting property binding flags
- cairo: Fix rectangle getter

Sebastian Dröge
- glib: Add a getter for `ObjectBuilder::type_`
- glib: Add `ObjectSubclass::obj()` as a shorter alias for `instance()`

