use glib::prelude::*;
use glib::subclass::prelude::*;

/// The ffi module includes the exported C API of the interface.
#[allow(dead_code)]
pub mod ffi {
    /// The instance pointer is used for references to the interface.
    #[repr(C)]
    pub struct Instance(std::ffi::c_void);

    /// Custom class struct for the [`super::Purrable`] interface.
    ///
    /// The class struct is used to implement the vtable for method dispatch. The first field *must*
    /// be a pointer to the parent type. For all interfaces this is required to be
    /// [`glib::gobject_ffi::GTypeInterface`]
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Interface {
        /// The first field in a class struct must always be the parent class struct
        parent_type: glib::gobject_ffi::GTypeInterface,

        /// Virtual method for the [`PurrableImpl::is_purring`](super::PurrableImpl::is_purring) method
        pub(super) is_purring: fn(&super::Purrable) -> bool,
    }

    /// Safety: This impl is unsafe because it requires the struct to be `repr(C)` and
    /// the first field must be [`glib::gobject_ffi::GTypeInterface`].
    unsafe impl glib::subclass::types::InterfaceStruct for Interface {
        type Type = super::iface::Purrable;
    }
}

/// The private module includes the interface default methods and ties together class and public type
mod iface {
    use glib::subclass::prelude::*;

    /// Interfaces require a private type to use for implementing `ObjectInterface` and default methods.
    ///
    /// We use an uninhabited enum here to make the type uninstantiatable.
    pub enum Purrable {}

    #[glib::object_interface]
    impl ObjectInterface for Purrable {
        const NAME: &'static str = "Purrable";

        type Instance = super::ffi::Instance;
        type Interface = super::ffi::Interface;

        /// Initialize the class struct with the default implementations of the
        /// virtual methods.
        fn interface_init(klass: &mut Self::Interface) {
            klass.is_purring = |_| Self::is_purring_default();
        }
    }

    impl Purrable {
        /// Default implementation of [`PurrableImpl::is_purring`](super::PurrableImpl::is_purring)
        fn is_purring_default() -> bool {
            println!("Purrable::is_purring_default: Not purring");
            false
        }
    }
}

glib::wrapper! {
    /// The `Purrable` interface provides a virtual method `is_purring`
    pub struct Purrable(ObjectInterface<iface::Purrable>);
}

/// The `PurrableExt` trait contains public methods of all [`Purrable`] objects
///
/// These methods need to call the appropriate vfunc from the vtable.
pub trait PurrableExt: IsA<Purrable> {
    /// Return the current purring status
    fn is_purring(&self) -> bool {
        let this = self.upcast_ref::<Purrable>();
        let iface = this.interface::<Purrable>().unwrap();
        (iface.as_ref().is_purring)(this)
    }
}

impl<T: IsA<Purrable>> PurrableExt for T {}

/// The `PurrableImpl` trait contains virtual function definitions for [`Purrable`] objects.
pub trait PurrableImpl: ObjectImpl
where
    <Self as ObjectSubclass>::Type: IsA<glib::Object>,
    <Self as ObjectSubclass>::Type: IsA<Purrable>,
{
    /// Return the current purring status.
    ///
    /// The default implementation chains up to the parent implementation,
    /// if we didn't do this *all* subclasses of `Purrable` classes would
    /// need to implement these methods manually.
    fn is_purring(&self) -> bool {
        self.parent_is_purring()
    }
}

/// The `PurrableImplExt` trait contains non-overridable methods for subclasses to use.
///
/// These are supposed to be called only from inside implementations of `Pet` subclasses.
pub trait PurrableImplExt: PurrableImpl
where
    <Self as ObjectSubclass>::Type: IsA<glib::Object>,
    <Self as ObjectSubclass>::Type: IsA<Purrable>,
{
    /// Chains up to the parent implementation of [`PurrableExt::is_purring`]
    fn parent_is_purring(&self) -> bool {
        let data = Self::type_data();
        let parent_iface =
            unsafe { &*(data.as_ref().parent_interface::<Purrable>() as *const ffi::Interface) };
        let is_purring = parent_iface.is_purring;

        is_purring(unsafe { self.obj().unsafe_cast_ref() })
    }
}

/// The `PurrableImplExt` trait is implemented for all classes that implement [`Purrable`].
impl<T: PurrableImpl> PurrableImplExt for T
where
    <Self as ObjectSubclass>::Type: IsA<glib::Object>,
    <Self as ObjectSubclass>::Type: IsA<Purrable>,
{
}

/// To make this interface implementable we need to implement [`IsImplementable`]
unsafe impl<Obj: PurrableImpl> IsImplementable<Obj> for Purrable
where
    <Obj as ObjectSubclass>::Type: IsA<glib::Object>,
    <Obj as ObjectSubclass>::Type: IsA<Purrable>,
{
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let klass = iface.as_mut();

        // Set the virtual method
        klass.is_purring = |obj| {
            // (Down-)cast the object as the concrete type
            let this = unsafe { obj.unsafe_cast_ref::<<Obj as ObjectSubclass>::Type>().imp() };
            // Call the trait method
            PurrableImpl::is_purring(this)
        };
    }
}
