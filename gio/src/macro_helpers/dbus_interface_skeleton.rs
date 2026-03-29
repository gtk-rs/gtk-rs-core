// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{DBusConnection, DBusMethodInfo, DBusMethodInvocation, ffi};

#[diagnostic::on_unimplemented(message = "No #[gio::dbus_methods] impl block found for {Self}")]
pub trait DBusMethods {
    fn method_infos() -> impl IntoIterator<Item = DBusMethodInfo>;

    fn method_call(
        &self,
        connection: DBusConnection,
        sender: Option<&str>,
        object_path: &str,
        interface_name: Option<&str>,
        method_name: &str,
        parameters: glib::Variant,
        invocation: DBusMethodInvocation,
    );
}

pub struct InterfaceVTable(pub ffi::GDBusInterfaceVTable);

unsafe impl Send for InterfaceVTable {}
unsafe impl Sync for InterfaceVTable {}

pub mod variant_type {
    use glib::{VariantTy, VariantType};
    use std::borrow::Cow;

    pub fn ensure_tuple(variant_type: Cow<'static, VariantTy>) -> Cow<'static, VariantTy> {
        if variant_type.is_tuple() {
            variant_type
        } else {
            Cow::Owned(VariantType::new_tuple([variant_type]))
        }
    }
}

pub mod result {
    //! Implements [autoref specialization] so that
    //! functions can choose to return a future, a `Result<impl ToVariant>` or an `impl ToVariant`.
    //!
    //! [autoref specialization]: https://github.com/dtolnay/case-studies/tree/master/autoref-specialization#realistic-application
    //!
    //! Usage:
    //! ```no_compile
    //! #[allow(clippy::needless_borrow)]
    //! use gio::__macro_helpers::result::*;
    //! match $expr {
    //!     expr => (&&&expr).kind().to_method_call_result::<_>(expr),
    //! }
    //! ```

    #[allow(clippy::needless_borrow)]
    fn _test(
        a: String,
        b: Result<String, glib::Error>,
        c: impl Future<Output = String>,
        d: impl Future<Output = Result<String, glib::Error>>,
    ) {
        let _result_a = ((&&&a).kind()).to_method_call_result(a);
        let _result_b = (&&&b).kind().to_method_call_result(b);
        let _result_c = (&&&c).kind().to_method_call_result(c);
        let _result_d = (&&&d).kind().to_method_call_result(d);
    }

    use futures_util::FutureExt as _;
    use glib::variant::ToVariant;

    // Matches what [`DBusMethodInvocation::return_result`] expects.
    // Note: the Option is super pointless, it just translates to an empty tuple.
    type MethodCallResult = Result<Option<glib::Variant>, glib::Error>;

    pub struct FutureResultTag;

    impl FutureResultTag {
        pub fn to_method_call_result<T: ToVariant, F: Future<Output = Result<T, glib::Error>>>(
            &self,
            future: F,
        ) -> impl Future<Output = MethodCallResult> + use<T, F> {
            future.map(|result| result.map(|value| Some(value.to_variant())))
        }
    }

    pub trait FutureResultKind {
        fn kind(&self) -> FutureResultTag {
            FutureResultTag
        }
    }

    impl<T: ToVariant, F: Future<Output = Result<T, glib::Error>>> FutureResultKind for F {}

    pub struct FutureTag;

    impl FutureTag {
        pub fn to_method_call_result<T: ToVariant, F: Future<Output = T>>(
            &self,
            future: F,
        ) -> impl Future<Output = MethodCallResult> + use<T, F> {
            future.map(|value| Ok(Some(value.to_variant())))
        }
    }

    pub trait FutureKind {
        fn kind(&self) -> FutureTag {
            FutureTag
        }
    }

    impl<T: ToVariant, F: Future<Output = T>> FutureKind for &F {}

    pub struct ResultTag;

    impl ResultTag {
        pub fn to_method_call_result<T: ToVariant>(
            &self,
            result: Result<T, glib::Error>,
        ) -> impl Future<Output = MethodCallResult> + use<T> {
            std::future::ready(result.map(|value| Some(value.to_variant())))
        }
    }

    pub trait ResultKind {
        fn kind(&self) -> ResultTag {
            ResultTag
        }
    }

    impl<T: ToVariant> ResultKind for &&Result<T, glib::Error> {}

    #[derive(Copy, Clone)]
    pub struct ValueTag;

    impl ValueTag {
        pub fn to_method_call_result<T: ToVariant>(
            &self,
            value: T,
        ) -> impl Future<Output = MethodCallResult> + use<T> {
            std::future::ready(Ok(Some(value.to_variant())))
        }
    }

    pub trait ValueKind {
        fn kind(&self) -> ValueTag {
            ValueTag
        }
    }

    impl<T: ToVariant> ValueKind for &&&T {}
}

pub mod static_return_type {
    use glib::VariantTy;
    use glib::variant::StaticVariantType;
    use std::borrow::Cow;

    pub fn type_of<T: ?Sized>() -> TypeOf<T> {
        TypeOf::default()
    }

    pub struct TypeOf<T: ?Sized>(std::marker::PhantomData<T>);

    impl<T: ?Sized> Default for TypeOf<T> {
        fn default() -> Self {
            Self(std::marker::PhantomData)
        }
    }

    #[cfg(test)]
    #[allow(clippy::needless_borrow)]
    fn _test() {
        // NOTE: Replace `impl` with `dyn` before applying this recipe.
        (&&&TypeOf::<String>::default()).static_variant_type();
        (&&&TypeOf::<Result<String, glib::Error>>::default()).static_variant_type();
        (&&&TypeOf::<Result<String, glib::Error>>::default()).static_variant_type();
        (&&&TypeOf::<dyn Future<Output = String>>::default()).static_variant_type();
        (&&&TypeOf::<dyn Future<Output = Result<String, glib::Error>>>::default())
            .static_variant_type();
    }

    pub trait FutureResultSpec {
        fn static_variant_type(&self) -> Cow<'static, VariantTy>;
    }

    impl<T: StaticVariantType> FutureResultSpec
        for TypeOf<dyn Future<Output = Result<T, glib::Error>>>
    {
        fn static_variant_type(&self) -> Cow<'static, VariantTy> {
            T::static_variant_type()
        }
    }

    pub trait FutureSpec {
        fn static_variant_type(&self) -> Cow<'static, VariantTy>;
    }

    impl<T: StaticVariantType> FutureSpec for &TypeOf<dyn Future<Output = T>> {
        fn static_variant_type(&self) -> Cow<'static, VariantTy> {
            T::static_variant_type()
        }
    }

    pub trait ResultSpec {
        fn static_variant_type(&self) -> Cow<'static, VariantTy>;
    }

    impl<T: StaticVariantType> ResultSpec for &&TypeOf<Result<T, glib::Error>> {
        fn static_variant_type(&self) -> Cow<'static, VariantTy> {
            T::static_variant_type()
        }
    }

    pub trait ValueSpec {
        fn static_variant_type(&self) -> Cow<'static, VariantTy>;
    }

    impl<T: StaticVariantType> ValueSpec for &&&TypeOf<T> {
        fn static_variant_type(&self) -> Cow<'static, VariantTy> {
            T::static_variant_type()
        }
    }
}
