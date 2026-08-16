//! LIB-PLACEDUNION — the schema-v11 bump, pinned.
//!
//! The call: GROUP-BOOLEAN-DESIGN (ratified A′) grew the node
//! vocabulary a `PlacedUnion` and the placement-rule vocabulary an
//! `Explicit` list of absolute frames. ONE vocabulary change, one
//! version — the two ship together because neither is expressible
//! without the other at the site that motivated both.
//!
//! A break in BOTH directions, on the v3/v9 precedent: a v11 file's
//! new variants are unknown to a v10 reader, and this reader has no
//! v10-shaped meaning to migrate from, so the gate refuses TYPED with
//! the regenerate recourse, the migration table stays empty, and the
//! v9 bytes stay checked in as the refusal fixture (a break nobody can
//! demonstrate is a break nobody can trust).
//!
//! The round trip in the OTHER direction — that a document carrying
//! the new vocabulary saves and reloads as itself — is asserted here
//! on both rules, because a version that only refuses old files proves
//! nothing about the new ones.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use editor_core::{
    Expr, Frame, Node, PatternKind, PersistError, REGENERATE_RECOURSE, SCHEMA_VERSION, load, save,
};
use fixture::{desc, len, scl};

/// The pre-bump bytes, kept verbatim as the refusal fixture (the file
/// `m4_pr6_golden.rs` pinned as LIVE until this bump).
const V10: &str = include_str!("golden/v10_golden.cad");

#[test]
fn schema_version_is_eleven() {
    assert_eq!(SCHEMA_VERSION, 11);
}

#[test]
fn the_checked_in_v10_file_is_really_v10() {
    assert_eq!(V10.lines().next(), Some("schema: 10"));
}

/// The break, demonstrated: a v9 file refuses TYPED at the version
/// door, naming the version found, the version supported, and the step
/// that does not exist.
#[test]
fn v10_refuses_too_old() {
    match load(V10) {
        Err(PersistError::SchemaTooOld {
            found,
            supported,
            missing,
        }) => {
            assert_eq!(found, 10);
            assert_eq!(supported, SCHEMA_VERSION);
            assert_eq!(
                missing, 10,
                "the 10 → 11 step is the one that does not exist"
            );
        }
        other => panic!("v9 must refuse SchemaTooOld, got {other:?}"),
    }
}

/// The recourse is the standing one — regenerate, never a shim.
#[test]
fn the_refusal_names_the_regenerate_recourse() {
    let err = load(V10).expect_err("v10 refuses");
    assert!(
        err.to_string().contains(REGENERATE_RECOURSE),
        "the refusal must carry the regenerate recourse: {err}"
    );
}

/// The other direction: a file claiming a FUTURE version refuses as
/// unknown — the newest this build supports is named.
#[test]
fn a_future_version_refuses_unknown() {
    let future = V10.replacen("schema: 10", "schema: 12", 1);
    match load(&future) {
        Err(PersistError::UnknownSchema { found, newest }) => {
            assert_eq!(found, 12);
            assert_eq!(newest, SCHEMA_VERSION);
        }
        other => panic!("a v12 file must refuse UnknownSchema, got {other:?}"),
    }
}

/// A document carrying a group node under EACH rule saves at v11 and
/// reloads as itself — the new vocabulary crosses the wire, which is
/// the half of a version bump that a refusal fixture cannot show.
#[test]
fn both_rules_round_trip_at_v11() {
    let mut r = corpus::Recorder::new();
    let p = r.insert(Node::Profile(desc(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    )));
    let solid = r.insert(Node::Extrude {
        profile: p,
        distance: len(1.0),
    });
    let stepped = r.insert(
        Node::placed_union(
            solid,
            Expr::count(3),
            PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(4.0),
            },
        )
        .expect("a stepped rule takes a count"),
    );
    let listed = r.insert(Node::placed_union_at(
        solid,
        vec![
            Frame::translation([0.0, 20.0, 0.0]),
            Frame::rotate_then_translate(
                [0.0, 0.0, 1.0],
                std::f64::consts::FRAC_PI_2,
                [0.0, 40.0, 0.0],
            ),
        ],
    ));
    let text = save(&r.doc, &[]).expect("the document saves");
    assert_eq!(text.lines().next(), Some("schema: 11"));
    let back = load(&text).expect("a v11 file loads");
    assert_eq!(back.doc.node(stepped), r.doc.node(stepped));
    assert_eq!(back.doc.node(listed), r.doc.node(listed));
    // Bit-exact frames: a placement is data, and `-0.0` is a different
    // placement from `0.0` to the content key, so it must be one on
    // the wire too.
    assert_eq!(
        save(&back.doc, &[]).expect("the reloaded document saves"),
        text,
        "save ∘ load is a fixpoint"
    );
}
