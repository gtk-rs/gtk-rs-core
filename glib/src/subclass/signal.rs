// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;
use crate::utils::is_canonical_pspec_name;
use crate::value::FromValue;
use crate::Closure;
use crate::IsA;
use crate::Object;
use crate::SignalFlags;
use crate::StaticType;
use crate::ToValue;
use crate::Type;
use crate::Value;

use std::ops::ControlFlow;
use std::ptr;
use std::sync::Mutex;
use std::{fmt, num::NonZeroU32};

type SignalClassHandler = Box<dyn Fn(&[Value]) -> Option<Value> + Send + Sync + 'static>;

type SignalAccumulator =
    Box<dyn Fn(&SignalInvocationHint, &mut Value, &Value) -> bool + Send + Sync + 'static>;

// rustdoc-stripper-ignore-next
/// Builder for signals.
#[allow(clippy::type_complexity)]
#[must_use = "The builder must be built to be used"]
pub struct SignalBuilder<'a> {
    name: &'a str,
    flags: SignalFlags,
    param_types: &'a [SignalType],
    return_type: SignalType,
    class_handler: Option<SignalClassHandler>,
    accumulator: Option<SignalAccumulator>,
}

// rustdoc-stripper-ignore-next
/// Signal metadata.
pub struct Signal {
    name: String,
    flags: SignalFlags,
    param_types: Vec<SignalType>,
    return_type: SignalType,
    registration: Mutex<SignalRegistration>,
}

// rustdoc-stripper-ignore-next
/// Signal invocation hint passed to signal accumulators.
#[repr(transparent)]
pub struct SignalInvocationHint(gobject_ffi::GSignalInvocationHint);

impl SignalInvocationHint {
    // rustdoc-stripper-ignore-next
    /// Gets the hint of the innermost signal emitting on `instance`. Returns `None` if no signal
    /// is being emitted.
    ///
    /// # Thread-safety
    ///
    /// This section only applies when `instance` implements `Send+Sync`. Retreiving the hint is
    /// thread-safe, but can result in logic errors if multiple signals are emitting concurrently on
    /// the same object across threads. If you call this function on an object that is `Send+Sync`,
    /// you must wrap every signal emission on that object with a mutex lock. Note this restriction
    /// applies to **all** signal emissions on that object, not just overridden signals. A lock
    /// such as [`ReentrantMutex`] can be used to prevent deadlocks in the case of recursive signal
    /// emissions.
    ///
    /// [`ReentrantMutex`]: https://docs.rs/lock_api/latest/lock_api/struct.ReentrantMutex.html
    #[doc(alias = "g_signal_get_invocation_hint")]
    pub fn for_object<T: IsA<Object>>(instance: &T) -> Option<Self> {
        unsafe {
            from_glib_none(gobject_ffi::g_signal_get_invocation_hint(
                instance.as_ref().to_glib_none().0,
            ))
        }
    }
    pub fn signal_id(&self) -> SignalId {
        unsafe { from_glib(self.0.signal_id) }
    }
    pub fn detail(&self) -> Option<crate::Quark> {
        unsafe { try_from_glib(self.0.detail).ok() }
    }

    pub fn run_type(&self) -> SignalFlags {
        unsafe { from_glib(self.0.run_type) }
    }
}

impl FromGlibPtrNone<*mut gobject_ffi::GSignalInvocationHint> for SignalInvocationHint {
    unsafe fn from_glib_none(hint: *mut gobject_ffi::GSignalInvocationHint) -> Self {
        assert!(!hint.is_null());
        Self(*hint)
    }
}

impl fmt::Debug for SignalInvocationHint {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("SignalInvocationHint")
            .field("detail", &self.detail())
            .field("run_type", &self.run_type())
            .finish()
    }
}

// rustdoc-stripper-ignore-next
/// In-depth information of a specific signal
pub struct SignalQuery(gobject_ffi::GSignalQuery);

unsafe impl Send for SignalQuery {}
unsafe impl Sync for SignalQuery {}

impl SignalQuery {
    // rustdoc-stripper-ignore-next
    /// The name of the signal.
    pub fn signal_name<'a>(&self) -> &'a str {
        unsafe {
            let ptr = self.0.signal_name;
            std::ffi::CStr::from_ptr(ptr).to_str().unwrap()
        }
    }

    // rustdoc-stripper-ignore-next
    /// The ID of the signal.
    pub fn signal_id(&self) -> SignalId {
        unsafe { SignalId::from_glib(self.0.signal_id) }
    }

    // rustdoc-stripper-ignore-next
    /// The instance type this signal can be emitted for.
    pub fn type_(&self) -> Type {
        unsafe { from_glib(self.0.itype) }
    }

    // rustdoc-stripper-ignore-next
    /// The signal flags.
    pub fn flags(&self) -> SignalFlags {
        unsafe { from_glib(self.0.signal_flags) }
    }

    // rustdoc-stripper-ignore-next
    /// The return type for the user callback.
    pub fn return_type(&self) -> SignalType {
        unsafe { from_glib(self.0.return_type) }
    }

    // rustdoc-stripper-ignore-next
    /// The number of parameters the user callback takes.
    pub fn n_params(&self) -> u32 {
        self.0.n_params
    }

    // rustdoc-stripper-ignore-next
    /// The parameters for the user callback.
    pub fn param_types(&self) -> &[SignalType] {
        if self.n_params() == 0 {
            return &[];
        }

        unsafe {
            std::slice::from_raw_parts(
                self.0.param_types as *const SignalType,
                self.0.n_params as usize,
            )
        }
    }
}

impl fmt::Debug for SignalQuery {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("SignalQuery")
            .field("signal_name", &self.signal_name())
            .field("type", &self.type_())
            .field("flags", &self.flags())
            .field("return_type", &self.return_type())
            .field("param_types", &self.param_types())
            .finish()
    }
}

// rustdoc-stripper-ignore-next
/// Signal ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SignalId(NonZeroU32);

impl SignalId {
    // rustdoc-stripper-ignore-next
    /// Create a new Signal Identifier.
    ///
    /// # Safety
    ///
    /// The caller has to ensure it's a valid signal identifier.
    pub unsafe fn new(id: NonZeroU32) -> Self {
        Self(id)
    }

    #[doc(alias = "g_signal_parse_name")]
    pub fn parse_name(
        name: &str,
        type_: Type,
        force_detail: bool,
    ) -> Option<(Self, Option<crate::Quark>)> {
        let mut signal_id = std::mem::MaybeUninit::uninit();
        let mut detail_quark = std::mem::MaybeUninit::uninit();
        unsafe {
            let found: bool = from_glib(gobject_ffi::g_signal_parse_name(
                name.to_glib_none().0,
                type_.into_glib(),
                signal_id.as_mut_ptr(),
                detail_quark.as_mut_ptr(),
                force_detail.into_glib(),
            ));

            if found {
                Some((
                    from_glib(signal_id.assume_init()),
                    crate::Quark::try_from_glib(detail_quark.assume_init()).ok(),
                ))
            } else {
                None
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Find a SignalId by its `name`, and the `type` it connects to.
    #[doc(alias = "g_signal_lookup")]
    pub fn lookup(name: &str, type_: Type) -> Option<Self> {
        unsafe {
            let signal_id = gobject_ffi::g_signal_lookup(name.to_glib_none().0, type_.into_glib());
            if signal_id == 0 {
                None
            } else {
                Some(Self::new(NonZeroU32::new_unchecked(signal_id)))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Queries more in-depth information about the current signal.
    #[doc(alias = "g_signal_query")]
    pub fn query(&self) -> SignalQuery {
        unsafe {
            let mut query_ptr = std::mem::MaybeUninit::uninit();
            gobject_ffi::g_signal_query(self.into_glib(), query_ptr.as_mut_ptr());
            let query = query_ptr.assume_init();
            assert_ne!(query.signal_id, 0);
            SignalQuery(query)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Find the signal name.
    #[doc(alias = "g_signal_name")]
    pub fn name<'a>(&self) -> &'a str {
        unsafe {
            let ptr = gobject_ffi::g_signal_name(self.into_glib());
            std::ffi::CStr::from_ptr(ptr).to_str().unwrap()
        }
    }
}

#[doc(hidden)]
impl FromGlib<u32> for SignalId {
    unsafe fn from_glib(signal_id: u32) -> Self {
        assert_ne!(signal_id, 0);
        Self::new(NonZeroU32::new_unchecked(signal_id))
    }
}

#[doc(hidden)]
impl IntoGlib for SignalId {
    type GlibType = u32;

    fn into_glib(self) -> u32 {
        self.0.into()
    }
}

#[derive(Copy, Clone, Hash)]
#[repr(transparent)]
pub struct SignalType(ffi::GType);

impl SignalType {
    pub fn with_static_scope(type_: Type) -> Self {
        Self(type_.into_glib() | gobject_ffi::G_TYPE_FLAG_RESERVED_ID_BIT)
    }

    pub fn static_scope(&self) -> bool {
        (self.0 & gobject_ffi::G_TYPE_FLAG_RESERVED_ID_BIT) != 0
    }

    pub fn type_(&self) -> Type {
        (*self).into()
    }
}

impl From<Type> for SignalType {
    fn from(type_: Type) -> Self {
        Self(type_.into_glib())
    }
}

impl From<SignalType> for Type {
    fn from(type_: SignalType) -> Self {
        // Remove the extra-bit used for G_SIGNAL_TYPE_STATIC_SCOPE
        let type_ = type_.0 & (!gobject_ffi::G_TYPE_FLAG_RESERVED_ID_BIT);
        unsafe { from_glib(type_) }
    }
}

impl PartialEq<Type> for SignalType {
    fn eq(&self, other: &Type) -> bool {
        let type_: Type = (*self).into();
        type_.eq(other)
    }
}

impl std::fmt::Debug for SignalType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let type_: Type = (*self).into();
        f.debug_struct("SignalType")
            .field("name", &type_.name())
            .field("static_scope", &self.static_scope())
            .finish()
    }
}

impl std::fmt::Display for SignalType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let type_: Type = (*self).into();
        f.debug_struct("SignalType")
            .field("name", &type_.name())
            .field("static_scope", &self.static_scope())
            .finish()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GType> for SignalType {
    unsafe fn from_glib(type_: ffi::GType) -> Self {
        Self(type_)
    }
}

#[doc(hidden)]
impl IntoGlib for SignalType {
    type GlibType = ffi::GType;

    fn into_glib(self) -> ffi::GType {
        self.0
    }
}

impl FromGlibContainerAsVec<Type, *const ffi::GType> for SignalType {
    unsafe fn from_glib_none_num_as_vec(ptr: *const ffi::GType, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib(*ptr.add(i)));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(_: *const ffi::GType, _: usize) -> Vec<Self> {
        // Can't really free a *const
        unimplemented!();
    }

    unsafe fn from_glib_full_num_as_vec(_: *const ffi::GType, _: usize) -> Vec<Self> {
        // Can't really free a *const
        unimplemented!();
    }
}

#[allow(clippy::type_complexity)]
enum SignalRegistration {
    Unregistered {
        class_handler: Option<SignalClassHandler>,
        accumulator: Option<SignalAccumulator>,
    },
    Registered {
        type_: Type,
        signal_id: SignalId,
    },
}

impl<'a> SignalBuilder<'a> {
    // rustdoc-stripper-ignore-next
    /// Run the signal class handler in the first emission stage.
    pub fn run_first(mut self) -> Self {
        self.flags |= SignalFlags::RUN_FIRST;
        self
    }

    // rustdoc-stripper-ignore-next
    /// Run the signal class handler in the third emission stage.
    pub fn run_last(mut self) -> Self {
        self.flags |= SignalFlags::RUN_LAST;
        self
    }

    // rustdoc-stripper-ignore-next
    /// Run the signal class handler in the last emission stage.
    pub fn run_cleanup(mut self) -> Self {
        self.flags |= SignalFlags::RUN_CLEANUP;
        self
    }

    // rustdoc-stripper-ignore-next
    /// Signals being emitted for an object while currently being in emission for this very object
    /// will not be emitted recursively, but instead cause the first emission to be restarted.
    pub fn no_recurse(mut self) -> Self {
        self.flags |= SignalFlags::NO_RECURSE;
        self
    }

    // rustdoc-stripper-ignore-next
    /// This signal supports "::detail" appendices to the signal name upon handler connections and
    /// emissions.
    pub fn detailed(mut self) -> Self {
        self.flags |= SignalFlags::DETAILED;
        self
    }

    // rustdoc-stripper-ignore-next
    /// Action signals are signals that may freely be emitted on alive objects from user code.
    pub fn action(mut self) -> Self {
        self.flags |= SignalFlags::ACTION;
        self
    }

    // rustdoc-stripper-ignore-next
    /// No emissions hooks are supported for this signal.
    pub fn no_hooks(mut self) -> Self {
        self.flags |= SignalFlags::NO_HOOKS;
        self
    }

    // rustdoc-stripper-ignore-next
    /// Varargs signal emission will always collect the arguments, even if there are no signal
    /// handlers connected.
    pub fn must_collect(mut self) -> Self {
        self.flags |= SignalFlags::MUST_COLLECT;
        self
    }

    // rustdoc-stripper-ignore-next
    /// The signal is deprecated and will be removed in a future version.
    pub fn deprecated(mut self) -> Self {
        self.flags |= SignalFlags::DEPRECATED;
        self
    }

    // rustdoc-stripper-ignore-next
    /// Explicitly set all flags.
    ///
    /// This overrides previously set flags on this builder.
    pub fn flags(mut self, flags: SignalFlags) -> Self {
        self.flags = flags;
        self
    }

    // rustdoc-stripper-ignore-next
    /// Class handler for this signal.
    pub fn class_handler<F>(mut self, func: F) -> Self
    where
        F: Fn(&[Value]) -> Option<Value> + Send + Sync + 'static,
    {
        self.class_handler = Some(Box::new(func));
        self
    }

    // rustdoc-stripper-ignore-next
    /// Accumulator for the return values of the signal.
    ///
    /// This is called if multiple signal handlers are connected to the signal for accumulating the
    /// return values into a single value. Panics if `T` and `R` do not return the same `Type` from
    /// [`crate::StaticType::static_type`]. The first `T` is the currently accumulated value and
    /// the second `T` is the return value from the current signal emission. If a `Some` value is
    /// returned then that value will be used to replace the current accumulator. Retuning
    /// `ControlFlow::Break` will abort the current signal emission.
    ///
    /// Either call this or [`Self::accumulator_with_values`], but not both.
    pub fn accumulator<
        T: for<'v> FromValue<'v> + StaticType,
        R: ToValue + StaticType,
        F: Fn(&SignalInvocationHint, T, T) -> ControlFlow<Option<R>, Option<R>>
            + Send
            + Sync
            + 'static,
    >(
        mut self,
        func: F,
    ) -> Self {
        assert_eq!(T::static_type(), R::static_type());
        self.accumulator = Some(Box::new(move |hint, accu, value| {
            let curr_accu = accu.get::<T>().unwrap();
            let value = value.get::<T>().unwrap();
            let (next, ret) = match func(hint, curr_accu, value) {
                ControlFlow::Continue(next) => (next, true),
                ControlFlow::Break(next) => (next, false),
            };
            if let Some(next) = next {
                *accu = ToValue::to_value(&next);
            }
            ret
        }));
        self
    }

    // rustdoc-stripper-ignore-next
    /// Accumulator for the return values of the signal.
    ///
    /// This is called if multiple signal handlers are connected to the signal for accumulating the
    /// return values into a single value.. The first [`Value`] is the currently accumulated value and
    /// the second [`Value`] is the return value from the current signal emission. If a `Some`
    /// value is returned then that value will be used to replace the current accumulator. Retuning
    /// `false` will abort the current signal emission.
    ///
    /// Either call this or [`Self::accumulator`], but not both.
    pub fn accumulator_with_values<
        F: Fn(&SignalInvocationHint, &mut Value, &Value) -> bool + Send + Sync + 'static,
    >(
        mut self,
        func: F,
    ) -> Self {
        self.accumulator = Some(Box::new(func));
        self
    }

    // rustdoc-stripper-ignore-next
    /// Build the signal.
    ///
    /// This does not register the signal yet, which only happens as part of object type
    /// registration.
    #[must_use = "Signal returned from the builder must be used for it to be registered"]
    pub fn build(self) -> Signal {
        let flags = if self.flags
            & (SignalFlags::RUN_FIRST | SignalFlags::RUN_LAST | SignalFlags::RUN_CLEANUP)
            == SignalFlags::empty()
        {
            self.flags | SignalFlags::RUN_LAST
        } else {
            self.flags
        };

        Signal {
            name: String::from(self.name),
            flags,
            param_types: self.param_types.to_vec(),
            return_type: self.return_type,
            registration: Mutex::new(SignalRegistration::Unregistered {
                class_handler: self.class_handler,
                accumulator: self.accumulator,
            }),
        }
    }
}

impl Signal {
    // rustdoc-stripper-ignore-next
    /// Create a new builder for a signal.
    pub fn builder<'a>(
        name: &'a str,
        param_types: &'a [SignalType],
        return_type: SignalType,
    ) -> SignalBuilder<'a> {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical signal name",
            name
        );
        SignalBuilder {
            name,
            param_types,
            return_type,
            flags: SignalFlags::empty(),
            class_handler: None,
            accumulator: None,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Name of the signal.
    pub fn name(&self) -> &str {
        &self.name
    }

    // rustdoc-stripper-ignore-next
    /// Flags of the signal.
    pub fn flags(&self) -> SignalFlags {
        self.flags
    }

    // rustdoc-stripper-ignore-next
    /// Parameter types of the signal.
    pub fn param_types(&self) -> &[SignalType] {
        &self.param_types
    }

    // rustdoc-stripper-ignore-next
    /// Return type of the signal.
    pub fn return_type(&self) -> SignalType {
        self.return_type
    }

    // rustdoc-stripper-ignore-next
    /// Signal ID.
    ///
    /// This will panic if called before the signal was registered.
    pub fn signal_id(&self) -> SignalId {
        match &*self.registration.lock().unwrap() {
            SignalRegistration::Unregistered { .. } => panic!("Signal not registered yet"),
            SignalRegistration::Registered { signal_id, .. } => *signal_id,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Type this signal was registered for.
    ///
    /// This will panic if called before the signal was registered.
    pub fn type_(&self) -> Type {
        match &*self.registration.lock().unwrap() {
            SignalRegistration::Unregistered { .. } => panic!("Signal not registered yet"),
            SignalRegistration::Registered { type_, .. } => *type_,
        }
    }

    pub(super) fn register(&self, type_: Type) {
        let mut registration = self.registration.lock().unwrap();

        let (class_handler, accumulator) = match &mut *registration {
            SignalRegistration::Unregistered {
                class_handler,
                accumulator,
            } => (class_handler.take(), accumulator.take()),
            SignalRegistration::Registered { .. } => unreachable!(),
        };

        let return_type = self.return_type;

        let class_handler = class_handler.map(|class_handler| {
            Closure::new(move |values| {
                let res = class_handler(values);

                if return_type == Type::UNIT {
                    if let Some(ref v) = res {
                        panic!("Signal has no return value but class handler returned a value of type {}", v.type_());
                    }
                } else {
                    match res {
                        None => {
                            panic!("Signal has a return value but class handler returned none");
                        }
                        Some(ref v) => {
                            assert!(v.type_().is_a(return_type.into()), "Signal has a return type of {} but class handler returned {}", Type::from(return_type), v.type_());
                        }
                    }
                }

                res
            })
        });

        unsafe extern "C" fn accumulator_trampoline(
            ihint: *mut gobject_ffi::GSignalInvocationHint,
            return_accu: *mut gobject_ffi::GValue,
            handler_return: *const gobject_ffi::GValue,
            data: ffi::gpointer,
        ) -> ffi::gboolean {
            let accumulator = &*(data as *const (
                SignalType,
                Box<
                    dyn Fn(&SignalInvocationHint, &mut Value, &Value) -> bool
                        + Send
                        + Sync
                        + 'static,
                >,
            ));

            let return_accu = &mut *(return_accu as *mut Value);
            let handler_return = &*(handler_return as *const Value);
            let return_type = accumulator.0;

            assert!(
                handler_return.type_().is_a(return_type.into()),
                "Signal has a return type of {} but handler returned {}",
                Type::from(return_type),
                handler_return.type_()
            );

            let res =
                (accumulator.1)(&from_glib_none(ihint), return_accu, handler_return).into_glib();

            assert!(
                return_accu.type_().is_a(return_type.into()),
                "Signal has a return type of {} but accumulator returned {}",
                Type::from(return_type),
                return_accu.type_()
            );

            res
        }

        let (accumulator, accumulator_trampoline) =
            if let (Some(accumulator), true) = (accumulator, return_type != Type::UNIT) {
                (
                    Box::into_raw(Box::new((return_type, accumulator))),
                    Some::<unsafe extern "C" fn(_, _, _, _) -> _>(accumulator_trampoline),
                )
            } else {
                (ptr::null_mut(), None)
            };

        unsafe {
            let signal_id = gobject_ffi::g_signal_newv(
                self.name.to_glib_none().0,
                type_.into_glib(),
                self.flags.into_glib(),
                class_handler.to_glib_none().0,
                accumulator_trampoline,
                accumulator as ffi::gpointer,
                None,
                return_type.into_glib(),
                self.param_types.len() as u32,
                self.param_types.as_ptr() as *mut _,
            );
            *registration = SignalRegistration::Registered {
                type_,
                signal_id: SignalId::from_glib(signal_id),
            };
        }
    }
}
