use glib::prelude::*;
use glib::subclass::prelude::*;

/// The ffi module includes the exported C API of the object.
#[allow(dead_code)]
pub mod ffi {
    /// The instance pointer is used for references to the object.
    pub type Instance = <super::Pet as super::ObjectSubclass>::Instance;

    /// Custom class struct for the [`Pet`](super::Pet) class.
    ///
    /// The class struct is used to implement the vtable for method dispatch. The first field
    /// *must* be a pointer to the parent type. In our case, this is
    /// [`GObjectClass`](glib::gobject_ffi::GObjectClass)
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Class {
        /// The first field in a class struct must always be the parent class struct
        parent_class: glib::gobject_ffi::GObjectClass,

        /// Virtual method for the [`PetImpl::pet`](super::PetImpl::pet) trait method
        pub(super) pet: fn(&super::Pet) -> bool,

        /// Virtual method for the [`PetImpl::feed`](super::PetImpl::feed) trait method
        pub(super) feed: fn(&super::Pet),
    }

    /// Every class struct is required to implement the `ClassStruct` trait
    ///
    /// Safety: This impl is unsafe because it requires the struct to be `repr(C)` and
    /// the first field must be the parent class.
    unsafe impl glib::subclass::types::ClassStruct for Class {
        type Type = super::imp::Pet;
    }

    /// Implement Deref to the parent class for convenience.
    impl std::ops::Deref for Class {
        type Target = glib::gobject_ffi::GObjectClass;

        fn deref(&self) -> &Self::Target {
            &self.parent_class
        }
    }
}

/// Private (implementation) module of the class. This is not exported.
mod imp {
    use glib::subclass::prelude::*;

    /// The Pet implementation struct
    #[derive(Default)]
    pub struct Pet {}

    #[glib::object_subclass]
    impl ObjectSubclass for Pet {
        /// This name is exported to the gobject type system and must be unique between all loaded
        /// shared libraries.
        ///
        /// Usually this is achieved by adding a short prefix to all names coming from a
        /// particular app / library.
        const NAME: &'static str = "Pet";

        /// The [`Pet`](super::Pet) class is abstract and instances to it can't be created.
        ///
        /// If an instance of this class were instantiated it would panic.
        const ABSTRACT: bool = true;

        type Type = super::Pet;

        /// Override the class struct with the custom [class](super::ffi::Class)
        type Class = super::ffi::Class;

        /// Initialize the [class struct](super::ffi::Class) with the default implementations of the
        /// virtual methods.
        fn class_init(klass: &mut Self::Class) {
            klass.pet = |obj| obj.imp().pet_default();
            klass.feed = |obj| obj.imp().feed_default();
        }
    }

    impl ObjectImpl for Pet {}

    impl Pet {
        /// Default implementation of [`PetImpl::pet`](super::PetImpl::pet)
        fn pet_default(&self) -> bool {
            // The default behaviour is unsuccessful pets
            println!("Pet::pet_default");
            false
        }

        /// Default implementation of [`PetImpl::feed`](super::PetImpl::feed)
        fn feed_default(&self) {
            println!("Pet::feed_default");
        }
    }
}

glib::wrapper! {
    /// The `Pet` class acts as a base class for pets and provides two virtual methods.
    pub struct Pet(ObjectSubclass<imp::Pet>);
}

/// The `PetExt` trait contains public methods of all [`Pet`] objects
///
/// These methods need to call the appropriate vfunc from the vtable.
pub trait PetExt: IsA<Pet> {
    /// Calls the [`PetImpl::pet`] vfunc
    fn pet(&self) -> bool {
        let this = self.upcast_ref();
        let class = this.class();

        (class.as_ref().pet)(this)
    }

    /// Calls the [`PetImpl::feed`] vfunc
    fn feed(&self) {
        let this = self.upcast_ref();
        let class = this.class();

        (class.as_ref().feed)(this)
    }
}

/// Implement PetExt for all [`Pet`] subclasses (and `Pet` itself)
impl<T: IsA<Pet>> PetExt for T {}

/// The `PetImpl` trait contains overridable virtual function definitions for [`Pet`] objects.
pub trait PetImpl: ObjectImpl + ObjectSubclass<Type: IsA<Pet>> {
    /// Default implementation of a virtual method.
    ///
    /// This always calls into the implementation of the parent class so that if
    /// the subclass does not explicitly implement it, the behaviour of its
    /// parent class will be preserved.
    fn pet(&self) -> bool {
        self.parent_pet()
    }

    /// Default implementation of a virtual method.
    ///
    /// This always calls into the implementation of the parent class so that if
    /// the subclass does not explicitly implement it, the behaviour of its
    /// parent class will be preserved.
    fn feed(&self) {
        self.parent_feed();
    }
}

/// The `PetImplExt` trait contains non-overridable methods for subclasses to use.
///
/// These are supposed to be called only from inside implementations of `Pet` subclasses.
pub trait PetImplExt: PetImpl {
    /// Chains up to the parent implementation of [`PetImpl::pet`]
    fn parent_pet(&self) -> bool {
        let data = Self::type_data();
        let parent_class = unsafe { &*(data.as_ref().parent_class() as *const ffi::Class) };
        let pet = parent_class.pet;

        unsafe { pet(self.obj().unsafe_cast_ref()) }
    }

    /// Chains up to the parent implementation of [`PetImpl::feed`]
    fn parent_feed(&self) {
        let data = Self::type_data();
        let parent_class = unsafe { &*(data.as_ref().parent_class() as *const ffi::Class) };
        let feed = parent_class.feed;

        unsafe { feed(self.obj().unsafe_cast_ref()) };
    }
}

/// The `PetImplExt` trait is implemented for all subclasses that have [`Pet`] in the class hierarchy
impl<T: PetImpl> PetImplExt for T {}

/// To make this class subclassable we need to implement IsSubclassable
unsafe impl<Obj: PetImpl> IsSubclassable<Obj> for Pet {
    /// Override the virtual method function pointers in subclasses to call directly into the
    /// `PetImpl` of the subclass.
    ///
    /// Note that this is only called for actual subclasses and not `Pet` itself: `Pet` does
    /// not implement `PetImpl` and handles this inside `ObjectSubclass::class_init()` for
    /// providing the default implementation of the virtual methods.
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<Obj>(class);

        let klass = class.as_mut();

        klass.pet = |obj| {
            let this = unsafe { obj.unsafe_cast_ref::<<Obj as ObjectSubclass>::Type>().imp() };
            PetImpl::pet(this)
        };

        klass.feed = |obj| {
            let this = unsafe { obj.unsafe_cast_ref::<<Obj as ObjectSubclass>::Type>().imp() };
            PetImpl::feed(this);
        };
    }
}
