//! **A name whose minted digest is above `i64::MAX` becomes metadata**:
//! a minted id carries a 64-bit digest, so about half of all ids hold
//! one above `i64::MAX`, and a name carrying one goes through
//! `to_value` and `from_value` as far as any other name does, and
//! through save and load inside a metadata record.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;

use editor_core::{
    CapEnd, DocEdit, EntityKind, MetaError, MetaValue, Node, ProfileDoc, ProfileProgram,
    RecipeNodeId, RoleSeg, StableName, content_pin, from_value, load, save, to_value,
};
use fixture::{insert, len, on_frame, square};
use geom_core::Tol;

/// A document whose newest node is an extrude minted above `i64::MAX`,
/// and that extrude. The extrudes differ in their distance alone, and
/// the first of these 64 whose id is above `i64::MAX` is taken: the
/// inputs are fixed, so this finds the same extrude on every run.
fn minted_high() -> (ProfileDoc, RecipeNodeId) {
    (1..=64)
        .map(|d| {
            let doc = ProfileDoc::empty_derived("meta-minted-ids", Tol::witness());
            let (doc, profile) = on_frame(
                doc,
                [0.0; 3],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                vec![square(0.0, 0.0, 1.0)],
            );
            insert(
                doc,
                Node::Extrude {
                    profile,
                    distance: len(f64::from(d)),
                    side: ExtrudeSide::Along,
                },
            )
        })
        .find(|(_, extrude)| i64::try_from(extrude.0.digest()).is_err())
        .expect("one of 64 extrudes is minted above i64::MAX")
}

/// The end cap of the extrude `node`.
fn cap(node: RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Cap(CapEnd::End)],
    }
}

/// `name` merged into itself `levels` times, so it nests one name per
/// level.
fn merged(name: StableName, levels: usize) -> StableName {
    (0..levels).fold(name, |of, _| StableName {
        kind: of.kind,
        node: of.node,
        path: vec![RoleSeg::Merged(vec![of])],
    })
}

/// How many levels of `merged` over `name` go through `to_value`, and
/// what refuses the next: every depth up to it round-trips through
/// `from_value` to the same name.
fn deepest_metadata(name: &StableName) -> (usize, MetaError) {
    for levels in 0.. {
        let deep = merged(name.clone(), levels);
        match to_value(&deep) {
            Ok(value) => assert_eq!(
                from_value::<StableName>(&value).as_ref(),
                Ok(&deep),
                "a name {levels} levels deep round-trips through metadata"
            ),
            Err(refusal) => return (levels, refusal),
        }
    }
    unreachable!("a name nests past the metadata bound eventually")
}

/// **A name minted above `i64::MAX` round-trips through metadata at
/// one level and at the deepest a name goes**, and stops at the same
/// depth, for the same reason, as one with a small id: its id decides
/// nothing about whether it becomes metadata.
#[test]
fn a_name_minted_above_i64_max_becomes_metadata_as_deep_as_any_name() {
    let (_, extrude) = minted_high();
    let high = cap(extrude);
    let value = to_value(&high).expect("a name minted above i64::MAX becomes metadata");
    assert_eq!(
        from_value::<StableName>(&value).as_ref(),
        Ok(&high),
        "and comes back the same name"
    );
    let (high_depth, high_refusal) = deepest_metadata(&high);
    let (low_depth, low_refusal) = deepest_metadata(&cap(RecipeNodeId::new(0, 1)));
    assert!(high_depth > 1, "a nested name becomes metadata too");
    assert_eq!(
        (high_depth, &high_refusal),
        (low_depth, &low_refusal),
        "a name minted above i64::MAX nests as deep as one with a small id"
    );
    assert!(
        matches!(high_refusal, MetaError::NestedTooDeep { .. }),
        "past the bound it refuses as a deep value: {high_refusal:?}"
    );
}

/// **A document whose metadata holds a name minted above `i64::MAX`
/// saves and loads**, the value whole and the pin unmoved.
#[test]
fn metadata_holding_a_name_minted_above_i64_max_saves_and_loads() {
    let tol = Tol::witness();
    let (doc, extrude) = minted_high();
    let name = cap(extrude);
    let mut record = std::collections::BTreeMap::new();
    record.insert("v".to_owned(), MetaValue::Int(1.into()));
    record.insert(
        "at".to_owned(),
        to_value(&name).expect("the name is metadata"),
    );
    let (doc, _) = fixture::step(
        doc,
        DocEdit::SetAppearanceMeta {
            name: name.clone(),
            key: "probe".to_owned(),
            value: MetaValue::map(record).expect("a shallow value"),
        },
    );
    let text = save(&doc, &[], tol).expect("the document saves");
    let loaded = load(&text, tol).expect("the document loads back");
    assert!(loaded.doc == doc, "the loaded document is the saved one");
    assert_eq!(
        save(&loaded.doc, &[], tol).expect("the loaded document saves"),
        text,
        "and saves the same text"
    );
    assert_eq!(
        content_pin(&loaded.doc, tol).unwrap(),
        content_pin(&doc, tol).unwrap(),
        "and pins the same"
    );
    let stored = &loaded
        .doc
        .appearance_of(&name)
        .expect("the record loads")
        .metadata["probe"];
    let MetaValue::Map(entries) = stored else {
        panic!("the record is a map: {stored:?}")
    };
    assert_eq!(
        from_value::<StableName>(&entries["at"]).as_ref(),
        Ok(&name),
        "the loaded value reads back as the name"
    );
}

/// **A node id read by a door with its own visitor comes back through
/// `from_value` at every id**, as it does through the saved text: a
/// profile's `plane` reads through `plane_ref`, whose visitor takes the
/// id's string alone, so an id spelled any other way would be refused.
#[test]
fn a_profile_program_comes_back_through_metadata_at_every_plane_id() {
    for (ordinal, id) in [
        (1, 5),
        (2, i64::MAX as u64),
        (3, i64::MAX as u64 + 1),
        (u32::MAX, u64::MAX),
    ] {
        let program: ProfileProgram = ProfileProgram {
            plane: RecipeNodeId::new(ordinal, id),
            loops: Vec::new(),
            ids: Vec::new(),
        };
        let value = to_value(&program).expect("a profile program is metadata");
        let back = from_value::<ProfileProgram>(&value)
            .unwrap_or_else(|e| panic!("plane {id} comes back through from_value: {e}"));
        assert_eq!(back.plane, program.plane, "plane {id} comes back as itself");
        let json = serde_json::to_string(&program).unwrap();
        let read = serde_json::from_str::<ProfileProgram>(&json)
            .unwrap_or_else(|e| panic!("plane {id} reads back from text: {e}"));
        assert_eq!(read.plane, program.plane, "plane {id} reads back from text");
    }
}
