use glib::{value::FromValue, StaticType, ToValue};

#[test]
fn higher_level_types() {
    #[derive(Debug, glib::ValueDelegate)]
    pub struct MyVec(Vec<String>);

    #[derive(Debug, glib::ValueDelegate)]
    #[value_delegate(nullable)]
    pub struct MyString(Box<str>);

    #[derive(Debug, glib::ValueDelegate)]
    #[value_delegate(from = Option<String>)]
    struct MyVecManualFrom(Vec<String>);

    impl From<Option<String>> for MyVecManualFrom {
        fn from(v: Option<String>) -> Self {
            Self(v.into_iter().collect::<Vec<_>>())
        }
    }
    impl<'a> From<&'a MyVecManualFrom> for Option<String> {
        fn from(v: &'a MyVecManualFrom) -> Self {
            v.0.iter().next().cloned()
        }
    }

    let vec = vec!["foo".to_string(), "bar".to_string()];
    let vec_value = vec.to_value();
    let my_vec_value = MyVec(vec.clone()).to_value();
    assert_eq!(MyVec::static_type(), Vec::<String>::static_type());
    assert_eq!(
        vec_value.get::<Vec<String>>().unwrap(),
        my_vec_value.get::<Vec<String>>().unwrap(),
    );
    assert_eq!(vec_value.value_type(), my_vec_value.value_type());
    assert_eq!(unsafe { Vec::<String>::from_value(&vec_value) }, unsafe {
        MyVec::from_value(&vec_value).0
    });
    assert_eq!(
        unsafe { Vec::<String>::from_value(&my_vec_value) },
        unsafe { MyVec::from_value(&my_vec_value).0 }
    );

    let string = "foo".to_string();
    let string_value = string.to_value();
    let my_string_value = MyString(string.into()).to_value();
    assert_eq!(MyString::static_type(), Box::<str>::static_type());
    assert_eq!(
        string_value.get::<Box<str>>().unwrap(),
        my_string_value.get::<Box<str>>().unwrap(),
    );
    assert_eq!(string_value.value_type(), my_string_value.value_type());
    assert_eq!(unsafe { Box::<str>::from_value(&string_value) }, unsafe {
        MyString::from_value(&string_value).0
    });
    assert_eq!(
        unsafe { Box::<str>::from_value(&my_string_value) },
        unsafe { MyString::from_value(&my_string_value).0 }
    );

    let string_some = Some("foo".to_string());
    let string_some_value = string_some.to_value();
    let string_none_value = None::<String>.to_value();
    let my_string_some_value = MyString(string_some.unwrap().into()).to_value();
    let my_string_none_value = None::<MyString>.to_value();
    assert_eq!(
        Option::<MyString>::static_type(),
        Option::<Box<str>>::static_type()
    );
    assert_eq!(
        string_some_value
            .get::<Option<Box<str>>>()
            .unwrap()
            .unwrap(),
        my_string_some_value
            .get::<Option<Box<str>>>()
            .unwrap()
            .unwrap(),
    );
    assert_eq!(
        string_none_value
            .get::<Option<Box<str>>>()
            .unwrap()
            .is_none(),
        my_string_none_value
            .get::<Option<Box<str>>>()
            .unwrap()
            .is_none(),
    );
    assert_eq!(
        string_some_value.value_type(),
        my_string_some_value.value_type()
    );
    assert_eq!(
        string_none_value.value_type(),
        my_string_none_value.value_type()
    );
    assert_eq!(
        unsafe { Option::<Box<str>>::from_value(&string_some_value).unwrap() },
        unsafe {
            Option::<MyString>::from_value(&string_some_value)
                .unwrap()
                .0
        }
    );
    assert_eq!(
        unsafe { Option::<Box<str>>::from_value(&string_none_value).is_none() },
        unsafe { Option::<MyString>::from_value(&string_none_value).is_none() }
    );
    assert_eq!(
        unsafe { Option::<Box<str>>::from_value(&my_string_some_value).unwrap() },
        unsafe {
            Option::<MyString>::from_value(&my_string_some_value)
                .unwrap()
                .0
        }
    );
    assert_eq!(
        unsafe { Option::<Box<str>>::from_value(&my_string_none_value).is_none() },
        unsafe { Option::<MyString>::from_value(&my_string_none_value).is_none() }
    );

    let opt = Some("foo".to_string());
    let opt_value = opt.to_value();
    let my_vec_manual_from_value = MyVecManualFrom::from(opt).to_value();
    assert_eq!(
        MyVecManualFrom::static_type(),
        Option::<String>::static_type()
    );
    assert_eq!(
        opt_value.get::<Option<String>>().unwrap(),
        my_vec_manual_from_value.get::<Option<String>>().unwrap(),
    );
    assert_eq!(
        opt_value.value_type(),
        my_vec_manual_from_value.value_type()
    );
    assert_eq!(
        unsafe {
            Option::<String>::from_value(&opt_value)
                .into_iter()
                .collect::<Vec<_>>()
        },
        unsafe { MyVecManualFrom::from_value(&opt_value).0 }
    );
    assert_eq!(
        unsafe {
            Option::<String>::from_value(&my_vec_manual_from_value)
                .into_iter()
                .collect::<Vec<_>>()
        },
        unsafe { MyVecManualFrom::from_value(&my_vec_manual_from_value).0 }
    );
}
