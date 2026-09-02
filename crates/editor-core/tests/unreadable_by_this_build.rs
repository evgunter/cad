//! BOOL-13 deliverable-1 probe (temporary shape): is serde_json's
//! rejection of an unknown enum variant / a missing required field
//! typed and NAME-CARRYING once it has passed through `persist`'s
//! error plumbing?

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{Node, PersistError, ProfileDoc, REGENERATE_RECOURSE, load, save};
use fixture::{desc, insert, len};
use geom_core::Tol;

fn small() -> String {
    let doc = ProfileDoc::empty_derived("bool13-probe", Tol::witness());
    let (doc, profile) = insert(
        doc,
        Node::Profile(desc(
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        )),
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    save(&doc, &[], Tol::witness()).expect("saves")
}

/// Splits a save into (header lines, body json value).
fn split(text: &str) -> (String, serde_json::Value) {
    let (schema, rest) = text.split_once('\n').unwrap();
    let (id, body) = rest.split_once('\n').unwrap();
    (
        format!("{schema}\n{id}\n"),
        serde_json::from_str(body).unwrap(),
    )
}

fn extrude_mut(v: &mut serde_json::Value) -> &mut serde_json::Map<String, serde_json::Value> {
    v["snapshot"]["nodes"]["1"].as_object_mut().unwrap()
}

#[test]
fn probe_unknown_variant() {
    let text = small();
    let (header, mut v) = split(&text);
    let node = extrude_mut(&mut v);
    let payload = node.remove("Extrude").unwrap();
    node.insert("Extrudez".to_string(), payload);
    let mutated = format!("{header}{}\n", serde_json::to_string_pretty(&v).unwrap());
    let raw: Result<serde_json::Value, _> = serde_json::from_str::<serde_json::Value>(&mutated.split_once('\n').unwrap().1.split_once('\n').unwrap().1);
    assert!(raw.is_ok());
    let err = load(&mutated, Tol::witness()).expect_err("must refuse");
    println!("PROBE unknown-variant Debug: {err:?}");
    println!("PROBE unknown-variant Display: {err}");
    let msg = err.to_string();
    assert!(msg.contains("Extrudez"), "the refusal names the variant: {msg}");
    assert_eq!(msg.matches(REGENERATE_RECOURSE).count(), 1, "{msg}");
}

#[test]
fn probe_missing_field() {
    let text = small();
    let (header, mut v) = split(&text);
    let node = extrude_mut(&mut v);
    let removed = node["Extrude"].as_object_mut().unwrap().remove("distance");
    assert!(removed.is_some(), "the fixture's extrude carries `distance`");
    let mutated = format!("{header}{}\n", serde_json::to_string_pretty(&v).unwrap());
    let err = load(&mutated, Tol::witness()).expect_err("must refuse");
    println!("PROBE missing-field Debug: {err:?}");
    println!("PROBE missing-field Display: {err}");
    let msg = err.to_string();
    assert!(msg.contains("distance"), "the refusal names the field: {msg}");
    assert_eq!(msg.matches(REGENERATE_RECOURSE).count(), 1, "{msg}");
}

/// The raw serde_json classification, for the record (not through
/// the plumbing): both are `Category::Data`.
#[test]
fn probe_raw_serde_json_classification() {
    #[derive(serde::Deserialize, Debug)]
    #[allow(dead_code)]
    enum E {
        A { x: u32 },
    }
    let unknown = serde_json::from_str::<E>(r#"{"B":{"x":1}}"#).unwrap_err();
    let missing = serde_json::from_str::<E>(r#"{"A":{}}"#).unwrap_err();
    println!("PROBE raw unknown: {:?} / {unknown}", unknown.classify());
    println!("PROBE raw missing: {:?} / {missing}", missing.classify());
    assert_eq!(unknown.classify(), serde_json::error::Category::Data);
    assert_eq!(missing.classify(), serde_json::error::Category::Data);
}

/// RED-FIRST scaffold: the older-shaped document is today's save with
/// the version line cut away (the shape the demolition writes).
#[test]
fn probe_older_shaped_document_loads() {
    let text = small();
    let (_, rest) = text.split_once('\n').unwrap();
    println!("PROBE header-less document:\n{rest}");
    match load(rest, Tol::witness()) {
        Ok(_) | Err(PersistError::ToleranceConflict { .. }) => {}
        Err(other) => panic!("an older-shaped document must load, got {other:?}"),
    }
}
