use brarchive::SerializeOptions;
use std::collections::{BTreeMap, HashMap};

#[test]
fn deserialize_into_hashmap() {
    let bytes = brarchive::serialize([("a.json".to_string(), "{}".to_string())]).unwrap();
    let map: HashMap<String, String> = brarchive::deserialize(&bytes).unwrap();
    assert_eq!(map["a.json"], "{}");
}

#[test]
fn deserialize_into_vec() {
    let bytes = brarchive::serialize([
        ("a.json".to_string(), "1".to_string()),
        ("b.json".to_string(), "2".to_string()),
    ]).unwrap();
    let vec: Vec<(String, String)> = brarchive::deserialize(&bytes).unwrap();
    assert_eq!(vec.len(), 2);
}

#[test]
fn serialize_str_literals() {
    let bytes = brarchive::serialize([("hello.json", "{}")]).unwrap();
    let map: BTreeMap<String, String> = brarchive::deserialize(&bytes).unwrap();
    assert_eq!(map["hello.json"], "{}");
}

#[test]
fn serialize_btreemap_ref() {
    let map = BTreeMap::from([("x.json".to_string(), "data".to_string())]);
    let bytes = brarchive::serialize(&map).unwrap();
    let result: BTreeMap<String, String> = brarchive::deserialize(&bytes).unwrap();
    assert_eq!(result, map);
}

#[test]
fn serialize_with_dedup_smaller_output() {
    let data = vec![
        ("a.json".to_string(), "same".to_string()),
        ("b.json".to_string(), "same".to_string()),
    ];
    let without = brarchive::serialize(data.clone()).unwrap();
    let with_dedup = brarchive::serialize_with(data, SerializeOptions { dedup: true }).unwrap();
    assert!(with_dedup.len() < without.len());
}

#[test]
fn serialize_with_dedup_round_trip() {
    let data = vec![
        ("a.json".to_string(), "shared".to_string()),
        ("b.json".to_string(), "shared".to_string()),
        ("c.json".to_string(), "unique".to_string()),
    ];
    let bytes = brarchive::serialize_with(data, SerializeOptions { dedup: true }).unwrap();
    let result: BTreeMap<String, String> = brarchive::deserialize(&bytes).unwrap();
    assert_eq!(result["a.json"], "shared");
    assert_eq!(result["b.json"], "shared");
    assert_eq!(result["c.json"], "unique");
}

#[test]
fn serialize_with_no_dedup_matches_serialize() {
    let data = vec![("a.json".to_string(), "content".to_string())];
    let a = brarchive::serialize(data.clone()).unwrap();
    let b = brarchive::serialize_with(data, SerializeOptions { dedup: false }).unwrap();
    assert_eq!(a, b);
}
