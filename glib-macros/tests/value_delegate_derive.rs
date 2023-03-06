use glib::{value::FromValue, HasParamSpec, StaticType, ToValue};

#[test]
fn higher_level_types() {
    #[derive(Debug, glib::ValueDelegate)]
    pub struct MyVec(Vec<String>);

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
