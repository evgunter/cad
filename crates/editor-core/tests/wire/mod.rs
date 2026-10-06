//! **Wire surgery** — a saved document, corrupted at one path of its
//! wire form, for the suites that measure what the LOAD door does with
//! a file the edit doors could not have produced.
//!
//! Its own helper tree rather than a block in [`crate::fixture`],
//! because `tests/fixture/` is SYMLINKED into `crates/viewer/tests/`
//! and is compiled as part of viewer's test binary: viewer's
//! dev-dependencies deliberately carry no serde edge (the comment on
//! them says so), and this helper's whole subject is
//! `serde_json::Value`. Nothing in `crates/viewer/tests/` mounts this
//! module, so the edge stays where it belongs.
#![allow(dead_code)] // one instance per binary; not every consumer uses all of it
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza

/// **A saved document, corrupted at one path of its wire form.**
///
/// The load-door suites need files the edit doors could not have
/// produced, and the only honest way to get one is to save a good
/// document and edit the TEXT: the header passes through untouched,
/// the JSON body is parsed, `edit` moves what the row is about, and
/// the result is re-rendered.
///
/// **BY PATH, not by byte.** A byte substitution proves only that a
/// byte moved; going through the parsed value proves the INTENDED
/// field moved, because the path names it and the caller's `edit`
/// asserts what it found there — so a fixture or field-order change
/// breaks the surgery loudly instead of silently landing it on a
/// neighbour. The `assert_ne!` below is the same guard one level up: a
/// surgery whose path missed writes the file back unchanged, and a row
/// over it would pass while measuring nothing.
///
/// One body, read by every suite that doctors a save: the copies it
/// replaces — each cutting the file for itself, and each free to cut
/// it somewhere else — are the reason it lives here.
pub fn doctored(text: &str, edit: impl FnOnce(&mut serde_json::Value)) -> String {
    let (header, mut wire) = split_body(text);
    edit(&mut wire);
    let out = format!("{header}{wire}");
    assert_ne!(out, text, "the corruption really landed");
    out
}

/// **A saved document's JSON body, parsed** — the READ-ONLY half of
/// the same surgery.
///
/// A row that only READS one place of the wire — which keys an object
/// carries, what a reference was written as — needs the two steps
/// [`doctored`] begins with (find the body after the id header, parse
/// it) and none of its write-back. Spelling those two again beside
/// such a row is how the split this module exists to own comes back
/// one suite at a time, so the read-only walk lives here too and both
/// halves cut the file in the same place by construction.
///
/// It wears no door's name: what it answers is the file's own wire
/// form, and which object inside it a row is about is the row's
/// business.
pub fn wire_body(text: &str) -> serde_json::Value {
    split_body(text).1
}

/// The id header and the parsed body — the one place a save's two
/// parts are told apart.
fn split_body(text: &str) -> (&str, serde_json::Value) {
    let split = text.find('{').expect("the JSON body follows the id header");
    let (header, body) = text.split_at(split);
    (header, serde_json::from_str(body).expect("the body parses"))
}

/// The id key a saved snapshot holds the variable `name` under.
///
/// # Panics
///
/// When the snapshot names no such variable.
pub fn wire_var_key(wire: &serde_json::Value, name: &str) -> String {
    wire["snapshot"]["var_names"]
        .as_object()
        .and_then(|names| {
            names
                .iter()
                .find_map(|(id, held)| (held == name).then(|| id.clone()))
        })
        .unwrap_or_else(|| panic!("the snapshot names no variable {name}"))
}

/// The variable `name` taken out of a saved snapshot, its name with it:
/// the declaration gone from under every reader of the name.
///
/// # Panics
///
/// When the snapshot names no such variable.
pub fn wire_undeclare(wire: &mut serde_json::Value, name: &str) {
    let id = wire_var_key(wire, name);
    let names = wire["snapshot"]["var_names"]
        .as_object_mut()
        .expect("the names are a map");
    assert!(names.remove(&id).is_some(), "the surgery removes the name");
    let vars = wire["snapshot"]["vars"]
        .as_object_mut()
        .expect("the variables are a map");
    assert!(vars.remove(&id).is_some(), "and the variable");
    let order = wire["snapshot"]["var_order"]
        .as_array_mut()
        .expect("the declaration order is a list");
    let before = order.len();
    order.retain(|listed| *listed != serde_json::json!(id.parse::<u64>().expect("an id key")));
    assert_eq!(order.len(), before - 1, "and its place in the order");
}

/// The variable `name` removed from a saved snapshot AND from its mint
/// log, so its readers read an id the document never minted — the
/// file-only fault ([`wire_undeclare`] alone leaves them a deleted
/// variable's readers, which is legal).
///
/// # Panics
///
/// As [`wire_undeclare`]'s, and when the log does not hold the id.
pub fn wire_unmint(wire: &mut serde_json::Value, name: &str) {
    let id: u64 = wire_var_key(wire, name).parse().expect("an id key");
    wire_undeclare(wire, name);
    let log = wire["snapshot"]["mint"]["log"]
        .as_array_mut()
        .expect("the mint log is a list");
    let before = log.len();
    log.retain(|entry| *entry != serde_json::json!({ "var": id }));
    assert_eq!(log.len(), before - 1, "and its mint log entry");
}

/// The continuous variable `name` retyped in a saved snapshot, from
/// `from` to `to`, its kind and display unit moved with it so the
/// document is broken in exactly one way: the pairing between the
/// variable and the dimension its readers read it at.
///
/// # Panics
///
/// When the snapshot names no such variable, or it is not `from`.
pub fn wire_retype(wire: &mut serde_json::Value, name: &str, from: &str, to: &str, unit: &str) {
    let id = wire_var_key(wire, name);
    let var = &mut wire["snapshot"]["vars"][id.as_str()];
    assert_eq!(
        var["kind"],
        serde_json::json!(from),
        "the surgery is aimed at the declared kind"
    );
    var["kind"] = serde_json::json!(to);
    let decl = &mut var["def"]["Free"]["Continuous"];
    decl["dim"] = serde_json::json!(to);
    decl["display_unit"] = serde_json::json!(unit);
}

/// **Retypes, in a saved document's wire, the free variable a slot
/// reads**: the slot at `slot`'s JSON holds its variable's id, and the
/// variable's kind, dimension and display unit all move to `dim` and
/// `unit`, so the variable stays well-formed and only a rule about
/// what reads it can refuse it.
pub fn retype_slot_var(
    wire: &mut serde_json::Value,
    slot: impl Fn(&serde_json::Value) -> &serde_json::Value,
    dim: &str,
    unit: &str,
) {
    let var = slot(wire)
        .as_u64()
        .unwrap_or_else(|| panic!("a stored slot holds its variable's id, got {}", slot(wire)));
    let held = &mut wire["snapshot"]["vars"][var.to_string()];
    let def = &mut held["def"]["Free"]["Continuous"];
    assert!(
        def.is_object(),
        "the surgery is aimed at a free continuous variable: {held}"
    );
    def["dim"] = serde_json::json!(dim);
    def["display_unit"] = serde_json::json!(unit);
    held["kind"] = serde_json::json!(dim);
}

/// **The free continuous definition, in a saved document's wire, of
/// the variable a slot reads** — the slot at `slot`'s JSON holds the
/// variable's id — for a row that doctors a written value.
pub fn slot_var_def(
    wire: &mut serde_json::Value,
    slot: impl Fn(&serde_json::Value) -> &serde_json::Value,
) -> &mut serde_json::Value {
    let var = slot(wire)
        .as_u64()
        .unwrap_or_else(|| panic!("a stored slot holds its variable's id, got {}", slot(wire)));
    &mut wire["snapshot"]["vars"][var.to_string()]["def"]["Free"]["Continuous"]
}
