include!(concat!(env!("OUT_DIR"), "/arced_field.rs"));

use self::outer::OneofField;
use alloc::vec;

#[test]
fn test_arced_field() {
    use alloc::sync::Arc;
    use alloc::vec::Vec;

    let outer = Outer {
        inner: Some(Arc::new(Inner {
            data: "test".into(),
            value: 42,
        })),
        oneof_field: Some(OneofField::ArcInner(Arc::new(Inner {
            data: "oneof".into(),
            value: 100,
        }))),
        inner_list: Vec::from([Inner {
            data: "list".into(),
            value: 1,
        }]),
    };

    // Test clone-on-write: cloning the outer should share the Arc
    let outer2 = outer.clone();
    assert!(Arc::ptr_eq(
        outer.inner.as_ref().unwrap(),
        outer2.inner.as_ref().unwrap()
    ));
}

#[test]
fn test_arced_field_encode_decode() {
    use alloc::sync::Arc;
    use prost::Message;

    let original = Outer {
        inner: Some(Arc::new(Inner {
            data: "hello".into(),
            value: 123,
        })),
        oneof_field: Some(outer::OneofField::Text("world".into())),
        inner_list: vec![],
    };

    let encoded = original.encode_to_vec();
    let decoded = Outer::decode(&encoded[..]).unwrap();

    assert_eq!(
        original.inner.as_ref().unwrap().data,
        decoded.inner.as_ref().unwrap().data
    );
    assert_eq!(
        original.inner.as_ref().unwrap().value,
        decoded.inner.as_ref().unwrap().value
    );
}

#[test]
fn test_arced_field_merge() {
    use alloc::sync::Arc;
    use prost::Message;

    let mut outer = Outer {
        inner: Some(Arc::new(Inner {
            data: "original".into(),
            value: 1,
        })),
        oneof_field: None,
        inner_list: vec![],
    };

    // Clone to test clone-on-write
    let _cloned = outer.clone();

    // Merging should trigger Arc::make_mut
    let update = Outer {
        inner: Some(Arc::new(Inner {
            data: "updated".into(),
            value: 2,
        })),
        oneof_field: None,
        inner_list: vec![],
    };

    let encoded = update.encode_to_vec();
    outer.merge(&encoded[..]).unwrap();

    // After merge, the inner should have the updated value
    assert_eq!(outer.inner.as_ref().unwrap().data, "updated");
}

#[test]
fn test_arced_oneof_variant() {
    use alloc::sync::Arc;

    let outer = Outer {
        inner: None,
        oneof_field: Some(OneofField::ArcInner(Arc::new(Inner {
            data: "oneof_test".into(),
            value: 999,
        }))),
        inner_list: vec![],
    };

    match &outer.oneof_field {
        Some(OneofField::ArcInner(inner)) => {
            assert_eq!(inner.data, "oneof_test");
            assert_eq!(inner.value, 999);
        }
        _ => panic!("Expected ArcInner variant"),
    }
}

#[test]
fn test_arced_field_clone_semantics() {
    use alloc::sync::Arc;

    let inner = Arc::new(Inner {
        data: "shared".into(),
        value: 42,
    });

    let outer1 = Outer {
        inner: Some(inner.clone()),
        oneof_field: None,
        inner_list: vec![],
    };

    let outer2 = Outer {
        inner: Some(inner.clone()),
        oneof_field: None,
        inner_list: vec![],
    };

    // Both should point to the same Arc
    assert!(Arc::ptr_eq(
        outer1.inner.as_ref().unwrap(),
        outer2.inner.as_ref().unwrap()
    ));

    // Reference count should be 3 (inner, outer1, outer2)
    assert_eq!(Arc::strong_count(&inner), 3);
}
