//! LIB-PLACEDUNION — the schema-v12 bump, pinned.
//!
//! The call: GROUP-BOOLEAN-DESIGN (ratified A′) grew the node
//! vocabulary a `PlacedUnion` and the placement-rule vocabulary an
//! `Explicit` list of absolute frames. ONE vocabulary change, one
//! version — the two ship together because neither is expressible
//! without the other at the site that motivated both.
//!
//! A break in BOTH directions, on the v3/v9 precedent: a v12 file's
//! new variants are unknown to a v11 reader, and this reader has no
//! v11-shaped meaning to migrate from, so the gate refuses TYPED with
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
use geom_core::Tol;

/// The pre-bump bytes, kept verbatim as the refusal fixture (the file
/// `m4_pr6_golden.rs` pinned as LIVE until this bump).
const V11: &str = include_str!("golden/v11_golden.cad");

#[test]
fn schema_version_is_current() {
    // Moved three times since this row was written (ASM-R2a's v13
    // `Node::Mate` arm, ASM-R2b's v14 interface record, M10-1's v15
    // parameter distributions) — the convention is that a bump updates every
    // pin it invalidates, so the number stays exact here. Named for
    // the PROPERTY rather than the number, since the number is exactly
    // what keeps moving.
    assert_eq!(SCHEMA_VERSION, 17);
}

#[test]
fn the_checked_in_v11_file_is_really_v11() {
    assert_eq!(V11.lines().next(), Some("schema: 11"));
}

/// The break, demonstrated: a v11 file refuses TYPED at the version
/// door, naming the version found, the version supported, and the step
/// that does not exist.
#[test]
fn v11_refuses_too_old() {
    match load(V11, Tol::witness()) {
        Err(PersistError::SchemaTooOld {
            found,
            supported,
            missing,
        }) => {
            assert_eq!(found, 11);
            assert_eq!(supported, SCHEMA_VERSION);
            assert_eq!(
                missing, 11,
                "the 11 → 12 step is the one that does not exist"
            );
        }
        other => panic!("v11 must refuse SchemaTooOld, got {other:?}"),
    }
}

/// The recourse is the standing one — regenerate, never a shim.
#[test]
fn the_refusal_names_the_regenerate_recourse() {
    let err = load(V11, Tol::witness()).expect_err("v11 refuses");
    assert!(
        err.to_string().contains(REGENERATE_RECOURSE),
        "the refusal must carry the regenerate recourse: {err}"
    );
}

/// The other direction: a file claiming a FUTURE version refuses as
/// unknown — the newest this build supports is named.
#[test]
fn a_future_version_refuses_unknown() {
    // Derived from the constant, never a literal: the number this row
    // needs is "one past the newest", and the newest keeps moving
    // (ASM-R2a took v13 after this row was written).
    let next = SCHEMA_VERSION + 1;
    let future = V11.replacen("schema: 11", &format!("schema: {next}"), 1);
    match load(&future, Tol::witness()) {
        Err(PersistError::UnknownSchema { found, newest }) => {
            assert_eq!(found, u64::from(next));
            assert_eq!(newest, SCHEMA_VERSION);
        }
        other => panic!("a future-version file must refuse UnknownSchema, got {other:?}"),
    }
}

/// A document carrying a group node under EACH rule saves at v12 and
/// reloads as itself — the new vocabulary crosses the wire, which is
/// the half of a version bump that a refusal fixture cannot show.
#[test]
fn both_rules_round_trip_at_v12() {
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
    let text = save(&r.doc, &[], Tol::witness()).expect("the document saves");
    assert_eq!(
        text.lines().next(),
        Some(format!("schema: {SCHEMA_VERSION}").as_str()),
        "a fresh save carries the CURRENT version, whatever bumped it last"
    );
    let back = load(&text, Tol::witness()).expect("the current version loads");
    assert_eq!(back.doc.node(stepped), r.doc.node(stepped));
    assert_eq!(back.doc.node(listed), r.doc.node(listed));
    // Bit-exact frames: a placement is data, and `-0.0` is a different
    // placement from `0.0` to the content key, so it must be one on
    // the wire too.
    assert_eq!(
        save(&back.doc, &[], Tol::witness()).expect("the reloaded document saves"),
        text,
        "save ∘ load is a fixpoint"
    );
}
