use ranch::{RangedNonZeroI32, RangedU32};

#[derive(PartialEq, Eq, Debug, serde::Deserialize, serde::Serialize)]
struct MyObject {
    a: RangedNonZeroI32<-10, 10>,
    b: RangedU32<0, 100>,
}

#[test]
fn serde() {
    let valid_a = r#"{ "a": -5, "b": 0 }"#;
    let valid_b = r#"{ "a": 10, "b": 100 }"#;
    let valid_c = r#"{ "a": 1, "b": 50 }"#;

    let invalid_a = r#"{ "a": 0, "b": 0 }"#;
    let invalid_b = r#"{ "a": 10, "b": 101 }"#;
    let invalid_c = r#"{ "a": 11, "b": 50 }"#;

    let valid_a: MyObject = serde_json::from_str(valid_a).unwrap();
    let valid_b: MyObject = serde_json::from_str(valid_b).unwrap();
    let valid_c: MyObject = serde_json::from_str(valid_c).unwrap();

    serde_json::from_str::<'_, MyObject>(invalid_a).unwrap_err();
    serde_json::from_str::<'_, MyObject>(invalid_b).unwrap_err();
    serde_json::from_str::<'_, MyObject>(invalid_c).unwrap_err();

    assert_eq!(
        valid_a,
        serde_json::from_str(&serde_json::to_string(&valid_a).unwrap())
            .unwrap(),
    );
    assert_eq!(
        valid_b,
        serde_json::from_str(&serde_json::to_string(&valid_b).unwrap())
            .unwrap(),
    );
    assert_eq!(
        valid_c,
        serde_json::from_str(&serde_json::to_string(&valid_c).unwrap())
            .unwrap(),
    );
}
