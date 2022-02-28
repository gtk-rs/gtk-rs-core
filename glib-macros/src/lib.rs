// Take a look at the license at the top of the repository in the LICENSE file.

mod boxed_derive;
mod clone;
mod closure;
mod downgrade_derive;
mod enum_derive;
mod error_domain_derive;
mod flags_attribute;
mod object_interface_attribute;
mod object_subclass_attribute;
mod shared_boxed_derive;
mod variant_derive;

mod utils;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput, NestedMeta};

/// Macro for passing variables as strong or weak references into a closure.
///
/// This macro can be useful in combination with closures, e.g. signal handlers, to reduce the
/// boilerplate required for passing strong or weak references into the closure. It will
/// automatically create the new reference and pass it with the same name into the closure.
///
/// If upgrading the weak reference to a strong reference inside the closure is failing, the
/// closure is immediately returning an optional default return value. If none is provided, `()` is
/// returned.
///
/// **⚠️ IMPORTANT ⚠️**
///
/// `glib` needs to be in scope, so unless it's one of the direct crate dependencies, you need to
/// import it because `clone!` is using it. For example:
///
/// ```rust,ignore
/// use gtk::glib;
/// ```
///
/// ### Debugging
///
/// In case something goes wrong inside the `clone!` macro, we use the [`g_debug`] macro. Meaning
/// that if you want to see these debug messages, you'll have to set the `G_MESSAGES_DEBUG`
/// environment variable when running your code (either in the code directly or when running the
/// binary) to either "all" or [`CLONE_MACRO_LOG_DOMAIN`]:
///
/// [`g_debug`]: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/macro.g_debug.html
/// [`CLONE_MACRO_LOG_DOMAIN`]: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/constant.CLONE_MACRO_LOG_DOMAIN.html
///
/// ```rust,ignore
/// use glib::CLONE_MACRO_LOG_DOMAIN;
///
/// std::env::set_var("G_MESSAGES_DEBUG", CLONE_MACRO_LOG_DOMAIN);
/// std::env::set_var("G_MESSAGES_DEBUG", "all");
/// ```
///
/// Or:
///
/// ```bash
/// $ G_MESSAGES_DEBUG=all ./binary
/// ```
///
/// ### Passing a strong reference
///
/// ```
/// use glib;
/// use glib_macros::clone;
/// use std::rc::Rc;
///
/// let v = Rc::new(1);
/// let closure = clone!(@strong v => move |x| {
///     println!("v: {}, x: {}", v, x);
/// });
///
/// closure(2);
/// ```
///
/// ### Passing a weak reference
///
/// ```
/// use glib;
/// use glib_macros::clone;
/// use std::rc::Rc;
///
/// let u = Rc::new(2);
/// let closure = clone!(@weak u => move |x| {
///     println!("u: {}, x: {}", u, x);
/// });
///
/// closure(3);
/// ```
///
/// #### Allowing a nullable weak reference
///
/// In some cases, even if the weak references can't be retrieved, you might want to still have
/// your closure called. In this case, you need to use `@weak-allow-none`:
///
/// ```
/// use glib;
/// use glib_macros::clone;
/// use std::rc::Rc;
///
/// let closure = {
///     // This `Rc` won't be available in the closure because it's dropped at the end of the
///     // current block
///     let u = Rc::new(2);
///     clone!(@weak-allow-none u => @default-return false, move |x| {
///         // We need to use a Debug print for `u` because it'll be an `Option`.
///         println!("u: {:?}, x: {}", u, x);
///         true
///     })
/// };
///
/// assert_eq!(closure(3), true);
/// ```
///
/// ### Renaming variables
///
/// ```
/// use glib;
/// use glib_macros::clone;
/// use std::rc::Rc;
///
/// let v = Rc::new(1);
/// let u = Rc::new(2);
/// let closure = clone!(@strong v as y, @weak u => move |x| {
///     println!("v as y: {}, u: {}, x: {}", y, u, x);
/// });
///
/// closure(3);
/// ```
///
/// ### Providing a default return value if upgrading a weak reference fails
///
/// You can do it in two different ways:
///
/// Either by providing the value yourself using `@default-return`:
///
/// ```
/// use glib;
/// use glib_macros::clone;
/// use std::rc::Rc;
///
/// let v = Rc::new(1);
/// let closure = clone!(@weak v => @default-return false, move |x| {
///     println!("v: {}, x: {}", v, x);
///     true
/// });
///
/// // Drop value so that the weak reference can't be upgraded.
/// drop(v);
///
/// assert_eq!(closure(2), false);
/// ```
///
/// Or by using `@default-panic` (if the value fails to get upgraded, it'll panic):
///
/// ```should_panic
/// # use glib;
/// # use glib_macros::clone;
/// # use std::rc::Rc;
/// # let v = Rc::new(1);
/// let closure = clone!(@weak v => @default-panic, move |x| {
///     println!("v: {}, x: {}", v, x);
///     true
/// });
/// # drop(v);
/// # assert_eq!(closure(2), false);
/// ```
///
/// ### Errors
///
/// Here is a list of errors you might encounter:
///
/// **Missing `@weak` or `@strong`**:
///
/// ```compile_fail
/// # use glib;
/// # use glib_macros::clone;
/// # use std::rc::Rc;
/// let v = Rc::new(1);
///
/// let closure = clone!(v => move |x| println!("v: {}, x: {}", v, x));
/// # drop(v);
/// # closure(2);
/// ```
///
/// **Passing `self` as an argument**:
///
/// ```compile_fail
/// # use glib;
/// # use glib_macros::clone;
/// # use std::rc::Rc;
/// #[derive(Debug)]
/// struct Foo;
///
/// impl Foo {
///     fn foo(&self) {
///         let closure = clone!(@strong self => move |x| {
///             println!("self: {:?}", self);
///         });
///         # closure(2);
///     }
/// }
/// ```
///
/// If you want to use `self` directly, you'll need to rename it:
///
/// ```
/// # use glib;
/// # use glib_macros::clone;
/// # use std::rc::Rc;
/// #[derive(Debug)]
/// struct Foo;
///
/// impl Foo {
///     fn foo(&self) {
///         let closure = clone!(@strong self as this => move |x| {
///             println!("self: {:?}", this);
///         });
///         # closure(2);
///     }
/// }
/// ```
///
/// **Passing fields directly**
///
/// ```compile_fail
/// # use glib;
/// # use glib_macros::clone;
/// # use std::rc::Rc;
/// #[derive(Debug)]
/// struct Foo {
///     v: Rc<usize>,
/// }
///
/// impl Foo {
///     fn foo(&self) {
///         let closure = clone!(@strong self.v => move |x| {
///             println!("self.v: {:?}", v);
///         });
///         # closure(2);
///     }
/// }
/// ```
///
/// You can do it by renaming it:
///
/// ```
/// # use glib;
/// # use glib_macros::clone;
/// # use std::rc::Rc;
/// # struct Foo {
/// #     v: Rc<usize>,
/// # }
/// impl Foo {
///     fn foo(&self) {
///         let closure = clone!(@strong self.v as v => move |x| {
///             println!("self.v: {}", v);
///         });
///         # closure(2);
///     }
/// }
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn clone(item: TokenStream) -> TokenStream {
    clone::clone_inner(item)
}

/// Macro for creating a [`Closure`] object. This is a wrapper around [`Closure::new`] that
/// automatically type checks its arguments at run-time.
///
/// A `Closure` takes [`Value`] objects as inputs and output. This macro will automatically convert
/// the inputs to Rust types when invoking its callback, and then will convert the output back to a
/// `Value`. All inputs must implement the [`FromValue`] trait, and outputs must either implement
/// the [`ToValue`] trait or be the unit type `()`. Type-checking of inputs is done at run-time; if
/// incorrect types are passed via [`Closure::invoke`] then the closure will panic. Note that when
/// passing input types derived from [`Object`] or [`Interface`], you must take care to upcast to
/// the exact object or interface type that is being received.
///
/// Similarly to [`clone!`](crate::clone!), this macro can be useful in combination with signal
/// handlers to reduce boilerplate when passing references. Unique to `Closure` objects is the
/// ability to watch an object using a the `@watch` directive. Only an [`Object`] value can be
/// passed to `@watch`, and only one object can be watched per closure. When an object is watched,
/// a weak reference to the object is held in the closure. When the object is destroyed, the
/// closure will become invalidated: all signal handlers connected to the closure will become
/// disconnected, and any calls to [`Closure::invoke`] on the closure will be silently ignored.
/// Internally, this is accomplished using [`Object::watch_closure`] on the watched object.
///
/// The `@weak-allow-none` and `@strong` captures are also supported and behave the same as in
/// [`clone!`](crate::clone!), as is aliasing captures with the `as` keyword. Notably, these
/// captures are able to reference `Rc` and `Arc` values in addition to `Object` values.
///
/// [`Closure`]: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/closure/struct.Closure.html
/// [`Closure::new`]: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/closure/struct.Closure.html#method.new
/// [`Closure::new_local`]: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/closure/struct.Closure.html#method.new_local
/// [`Closure::invoke`]: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/closure/struct.Closure.html#method.invoke
/// [`Value`]: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/value/struct.Value.html
/// [`FromValue`]: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/value/trait.FromValue.html
/// [`ToValue`]: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/value/trait.ToValue.html
/// [`Interface`]: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/object/struct.Interface.html
/// [`Object`]: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/object/struct.Object.html
/// [`Object::watch_closure`]: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/object/trait.ObjectExt.html#tymethod.watch_closure
/// **⚠️ IMPORTANT ⚠️**
///
/// `glib` needs to be in scope, so unless it's one of the direct crate dependencies, you need to
/// import it because `closure!` is using it. For example:
///
/// ```rust,ignore
/// use gtk::glib;
/// ```
///
/// ### Using as a closure object
///
/// ```
/// use glib_macros::closure;
///
/// let concat_str = closure!(|s: &str| s.to_owned() + " World");
/// let result = concat_str.invoke::<String>(&[&"Hello"]);
/// assert_eq!(result, "Hello World");
/// ```
///
/// ### Connecting to a signal
///
/// For wrapping closures that can't be sent across threads, the
/// [`closure_local!`](crate::closure_local!) macro can be used. It has the same syntax as
/// `closure!`, but instead uses [`Closure::new_local`] internally.
///
/// ```
/// use glib;
/// use glib::prelude::*;
/// use glib_macros::closure_local;
///
/// let obj = glib::Object::new::<glib::Object>(&[]).unwrap();
/// obj.connect_closure(
///     "notify", false,
///     closure_local!(|_obj: glib::Object, pspec: glib::ParamSpec| {
///         println!("property notify: {}", pspec.name());
///     }));
/// ```
///
/// ### Object Watching
///
/// ```
/// use glib;
/// use glib::prelude::*;
/// use glib_macros::closure_local;
///
/// let closure = {
///     let obj = glib::Object::new::<glib::Object>(&[]).unwrap();
///     let closure = closure_local!(@watch obj => move || {
///         obj.type_().name()
///     });
///     assert_eq!(closure.invoke::<String>(&[]), "GObject");
///     closure
/// };
/// // `obj` is dropped, closure invalidated so it always does nothing and returns None
/// closure.invoke::<()>(&[]);
/// ```
///
/// `@watch` has special behavior when connected to a signal:
///
/// ```
/// use glib;
/// use glib::prelude::*;
/// use glib_macros::closure_local;
///
/// let obj = glib::Object::new::<glib::Object>(&[]).unwrap();
/// {
///     let other = glib::Object::new::<glib::Object>(&[]).unwrap();
///     obj.connect_closure(
///         "notify", false,
///         closure_local!(@watch other as b => move |a: glib::Object, pspec: glib::ParamSpec| {
///             let value = a.property_value(pspec.name());
///             b.set_property(pspec.name(), &value);
///         }));
///     // The signal handler will disconnect automatically at the end of this
///     // block when `other` is dropped.
/// }
/// ```
///
/// ### Weak and Strong References
///
/// ```
/// use glib;
/// use glib::prelude::*;
/// use glib_macros::closure;
/// use std::sync::Arc;
///
/// let closure = {
///     let a = Arc::new(String::from("Hello"));
///     let b = Arc::new(String::from("World"));
///     let closure = closure!(@strong a, @weak-allow-none b => move || {
///         // `a` is Arc<String>, `b` is Option<Arc<String>>
///         format!("{} {}", a, b.as_ref().map(|b| b.as_str()).unwrap_or_else(|| "Moon"))
///     });
///     assert_eq!(closure.invoke::<String>(&[]), "Hello World");
///     closure
/// };
/// // `a` still kept alive, `b` is dropped
/// assert_eq!(closure.invoke::<String>(&[]), "Hello Moon");
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn closure(item: TokenStream) -> TokenStream {
    closure::closure_inner(item, "new")
}

/// The same as [`closure!`](crate::closure!) but uses [`Closure::new_local`] as a constructor.
/// This is useful for closures which can't be sent across threads. See the documentation of
/// [`closure!`](crate::closure!) for details.
///
/// [`Closure::new_local`]: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/closure/struct.Closure.html#method.new_local
#[proc_macro]
#[proc_macro_error]
pub fn closure_local(item: TokenStream) -> TokenStream {
    closure::closure_inner(item, "new_local")
}

/// Derive macro for register a rust enum in the glib type system and derive the
/// the [`glib::Value`] traits.
///
/// # Example
///
/// ```
/// use glib::prelude::*;
/// use glib::subclass::prelude::*;
///
/// #[derive(Debug, Copy, Clone, PartialEq, Eq, glib::Enum)]
/// #[enum_type(name = "MyEnum")]
/// enum MyEnum {
///     Val,
///     #[enum_value(name = "My Val")]
///     ValWithCustomName,
///     #[enum_value(name = "My Other Val", nick = "other")]
///     ValWithCustomNameAndNick,
/// }
/// ```
#[proc_macro_derive(Enum, attributes(enum_type, enum_value))]
#[proc_macro_error]
pub fn enum_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let gen = enum_derive::impl_enum(&input);
    gen.into()
}

/// Attribute macro for defining flags using the `bitflags` crate.
/// This macro will also define a `GFlags::type_` function and
/// the [`glib::Value`] traits.
///
/// The expected `GType` name has to be passed as macro attribute.
/// The name and nick of each flag can also be optionally defined.
/// Default name is the flag identifier in CamelCase and default nick
/// is the identifier in kebab-case.
/// Combined flags should not be registered with the `GType` system
/// and so needs to be tagged with the `#[flags_value(skip)]` attribute.
///
/// # Example
///
/// ```
/// use glib::prelude::*;
/// use glib::subclass::prelude::*;
///
/// #[glib::flags(name = "MyFlags")]
/// enum MyFlags {
///     #[flags_value(name = "Flag A", nick = "nick-a")]
///     A = 0b00000001,
///     #[flags_value(name = "Flag B")]
///     B = 0b00000010,
///     #[flags_value(skip)]
///     AB = Self::A.bits() | Self::B.bits(),
///     C = 0b00000100,
/// }
/// ```
///
/// [`glib::Value`]: value/struct.Value.html
#[proc_macro_attribute]
#[proc_macro_error]
pub fn flags(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_meta = parse_macro_input!(attr as NestedMeta);
    let input = parse_macro_input!(item as DeriveInput);
    let gen = flags_attribute::impl_flags(&attr_meta, &input);
    gen.into()
}

/// Derive macro for defining a GLib error domain and its associated
/// [`ErrorDomain`] trait.
///
/// # Example
///
/// ```
/// use glib::prelude::*;
/// use glib::subclass::prelude::*;
///
/// #[derive(Debug, Copy, Clone, glib::ErrorDomain)]
/// #[error_domain(name = "ExFoo")]
/// enum Foo {
///     Blah,
///     Baaz,
/// }
/// ```
///
/// [`ErrorDomain`]: error/trait.ErrorDomain.html
#[proc_macro_derive(ErrorDomain, attributes(error_domain))]
#[proc_macro_error]
pub fn error_domain_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let gen = error_domain_derive::impl_error_domain(&input);
    gen.into()
}

/// Derive macro for defining a [`BoxedType`]`::type_` function and
/// the [`glib::Value`] traits.
///
/// # Example
///
/// ```
/// use glib::prelude::*;
/// use glib::subclass::prelude::*;
///
/// #[derive(Clone, Debug, PartialEq, Eq, glib::Boxed)]
/// #[boxed_type(name = "MyBoxed")]
/// struct MyBoxed(String);
/// ```
///
/// [`BoxedType`]: subclass/boxed/trait.BoxedType.html
/// [`glib::Value`]: value/struct.Value.html
#[proc_macro_derive(Boxed, attributes(boxed_nullable, boxed_type))]
#[proc_macro_error]
pub fn boxed_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let gen = boxed_derive::impl_boxed(&input);
    gen.into()
}

/// Derive macro for defining a [`SharedType`]`::get_type` function and
/// the [`glib::Value`] traits.
///
/// # Example
///
/// ```
/// use glib::prelude::*;
/// use glib::subclass::prelude::*;
///
/// #[derive(Clone, Debug, PartialEq, Eq)]
/// struct MySharedInner {
///   foo: String,
/// }
/// #[derive(Clone, Debug, PartialEq, Eq, glib::SharedBoxed)]
/// #[shared_boxed_type(name = "MySharedBoxed")]
/// struct MyShared(std::sync::Arc<MySharedInner>);
/// ```
///
/// [`SharedType`]: subclass/shared/trait.SharedType.html
/// [`glib::Value`]: value/struct.Value.html
#[proc_macro_derive(SharedBoxed, attributes(shared_boxed_nullable, shared_boxed_type))]
#[proc_macro_error]
pub fn shared_boxed_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let gen = shared_boxed_derive::impl_shared_boxed(&input);
    gen.into()
}

/// Macro for boilerplate of [`ObjectSubclass`] implementations.
///
/// This adds implementations for the `type_data()` and `type_()` methods,
/// which should probably never be defined differently.
///
/// It provides default values for the `Instance`, `Class`, and `Interfaces`
/// type parameters. If these are present, the macro will use the provided value
/// instead of the default.
///
/// Usually the defaults for `Instance` and `Class` will work. `Interfaces` is
/// necessary for types that implement interfaces.
///
/// ```ignore
/// type Instance = glib::subclass::simple::InstanceStruct<Self>;
/// type Class = glib::subclass::simple::ClassStruct<Self>;
/// type Interfaces = ();
/// ```
///
/// If no `new()` or `with_class()` method is provide, the macro adds a `new()`
/// implementation calling `Default::default()`. So the type needs to implement
/// `Default`, or this should be overridden.
///
/// ```ignore
/// fn new() -> Self {
///     Default::default()
/// }
/// ```
///
/// [`ObjectSubclass`]: subclass/types/trait.ObjectSubclass.html
#[proc_macro_attribute]
#[proc_macro_error]
pub fn object_subclass(_attr: TokenStream, item: TokenStream) -> TokenStream {
    use proc_macro_error::abort_call_site;
    match syn::parse::<syn::ItemImpl>(item) {
        Ok(input) => object_subclass_attribute::impl_object_subclass(&input).into(),
        Err(_) => abort_call_site!(object_subclass_attribute::WRONG_PLACE_MSG),
    }
}

/// Macro for boilerplate of [`ObjectInterface`] implementations.
///
/// This adds implementations for the `get_type()` method, which should probably never be defined
/// differently.
///
/// It provides default values for the `Prerequisites` type parameter. If this present, the macro
/// will use the provided value instead of the default.
///
/// `Prerequisites` is interfaces for types that require a specific base class or interfaces.
///
/// ```ignore
/// type Prerequisites = ();
/// ```
///
/// [`ObjectInterface`]: interface/types/trait.ObjectInterface.html
#[proc_macro_attribute]
#[proc_macro_error]
pub fn object_interface(_attr: TokenStream, item: TokenStream) -> TokenStream {
    use proc_macro_error::abort_call_site;
    match syn::parse::<syn::ItemImpl>(item) {
        Ok(input) => object_interface_attribute::impl_object_interface(&input).into(),
        Err(_) => abort_call_site!(object_interface_attribute::WRONG_PLACE_MSG),
    }
}

/// Macro for deriving implementations of [`glib::clone::Downgrade`] and
/// [`glib::clone::Upgrade`] traits and a weak type.
///
/// # Examples
///
/// ## New Type Idiom
///
/// ```rust,ignore
/// #[derive(glib::Downgrade)]
/// pub struct FancyLabel(gtk::Label);
///
/// impl FancyLabel {
///     pub fn new(label: &str) -> Self {
///         Self(gtk::LabelBuilder::new().label(label).build())
///     }
///
///     pub fn flip(&self) {
///         self.0.set_angle(180.0 - self.0.angle());
///     }
/// }
///
/// let fancy_label = FancyLabel::new("Look at me!");
/// let button = gtk::ButtonBuilder::new().label("Click me!").build();
/// button.connect_clicked(clone!(@weak fancy_label => move || fancy_label.flip()));
/// ```
///
/// ## Generic New Type
///
/// ```rust,ignore
/// #[derive(glib::Downgrade)]
/// pub struct TypedEntry<T>(gtk::Entry, std::marker::PhantomData<T>);
///
/// impl<T: ToString + FromStr> for TypedEntry<T> {
///     // ...
/// }
/// ```
///
/// ## Structures and Enums
///
/// ```rust,ignore
/// #[derive(Clone, glib::Downgrade)]
/// pub struct ControlButtons {
///     pub up: gtk::Button,
///     pub down: gtk::Button,
///     pub left: gtk::Button,
///     pub right: gtk::Button,
/// }
///
/// #[derive(Clone, glib::Downgrade)]
/// pub enum DirectionButton {
///     Left(gtk::Button),
///     Right(gtk::Button),
///     Up(gtk::Button),
///     Down(gtk::Button),
/// }
/// ```
///
/// [`glib::clone::Downgrade`]: clone/trait.Downgrade.html
/// [`glib::clone::Upgrade`]: clone/trait.Upgrade.html
#[proc_macro_derive(Downgrade)]
pub fn downgrade(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    downgrade_derive::impl_downgrade(input)
}

/// Derive macro for serializing/deserializing custom structs as [`glib::Variant`]s.
///
/// # Example
///
/// ```
/// use glib::prelude::*;
///
/// #[derive(Debug, PartialEq, Eq, glib::Variant)]
/// struct Foo {
///     some_string: String,
///     some_int: i32,
/// }
///
/// let v = Foo { some_string: String::from("bar"), some_int: 1 };
/// let var = v.to_variant();
/// assert_eq!(var.get::<Foo>(), Some(v));
/// ```
///
/// When storing `Vec`s of fixed size types it is a good idea to wrap these in
/// `glib::FixedSizeVariantArray` as serialization/deserialization will be more efficient.
///
/// # Example
///
/// ```
/// use glib::prelude::*;
///
/// #[derive(Debug, PartialEq, Eq, glib::Variant)]
/// struct Foo {
///     some_vec: glib::FixedSizeVariantArray<Vec<u32>, u32>,
///     some_int: i32,
/// }
///
/// let v = Foo { some_vec: vec![1u32, 2u32].into(), some_int: 1 };
/// let var = v.to_variant();
/// assert_eq!(var.get::<Foo>(), Some(v));
/// ```
///
/// [`glib::Variant`]: variant/struct.Variant.html
#[proc_macro_derive(Variant)]
pub fn variant_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    variant_derive::impl_variant(input)
}

#[proc_macro]
pub fn cstr_bytes(item: TokenStream) -> TokenStream {
    syn::parse::Parser::parse2(
        |stream: syn::parse::ParseStream<'_>| {
            let literal = stream.parse::<syn::LitStr>()?;
            stream.parse::<syn::parse::Nothing>()?;
            let bytes = std::ffi::CString::new(literal.value())
                .map_err(|e| syn::Error::new_spanned(&literal, format!("{}", e)))?
                .into_bytes_with_nul();
            let bytes = proc_macro2::Literal::byte_string(&bytes);
            Ok(quote::quote! { #bytes }.into())
        },
        item.into(),
    )
    .unwrap_or_else(|e| e.into_compile_error().into())
}
