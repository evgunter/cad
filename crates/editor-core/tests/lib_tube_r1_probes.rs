//! **LIB-TUBE R1 review probes** — adversarial rows written against PR
//! #1628's claims, beyond the fixtures the unit chose.
//!
//! Adopted as PERMANENT rows at the fix pass, authorship preserved.
//! The non-dyadic row below and `lib_tube_r2_probes`'s
//! `r2_the_storage_contract_holds_at_non_dyadic_radii` were written
//! independently and are BOTH kept: they pin different spellings —
//! this one drives a WINDOWED (arc) hollow tube at two value pairs
//! and adds the solid door's verbatim store, while R2's drives a FULL
//! ring at one pair and guards its own sharpness against the decimal
//! `0.2`'s bits. Neither subsumes the other; do not dedupe them
//! without re-reading both.
//!
//! 1. The storage contract at NON-DYADIC values: the committed fixture
//!    subtracts 0.5 − 0.125 (exact); these rows subtract 0.3 − 0.1 and
//!    0.123456789 − 0.017, where `minor_radius − wall` is NOT the real
//!    difference and only the bits can see whether the door stored the
//!    caller's one IEEE subtraction or re-derived the radius some
//!    other way.
//! 2. Naming under the wholesale revolve template: two tube nodes of
//!    identical parameters in ONE document must mint disjoint name
//!    tables (the minting node is the discriminator the template
//!    relies on), and a full-ring hollow tube's cavity shell must be
//!    named TOTALLY by the unmodified revolve emitter.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use corpus::{body_of, eval, failures};
use editor_core::{
    Datum, Dimension, DocEdit, Expr, Node, ProfileDoc, ProfileProgram, RecipeNodeId, TubeWindow,
    apply,
};
use fixture::len;
use geom_core::Tol;
use topo::Surface;

fn scalar(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("finite")
}

fn angle(v: f64) -> Expr {
    Expr::literal(v, Dimension::Angle).expect("finite")
}

fn push(d: &ProfileDoc, e: &DocEdit<ProfileProgram>) -> ProfileDoc {
    apply(d, e, Tol::witness()).expect("edit applies").doc
}

fn axis_doc() -> (ProfileDoc, RecipeNodeId) {
    let mut doc = ProfileDoc::empty_derived("lib_tube_r1_probes", Tol::witness());
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Datum(Datum::Axis {
                origin: [len(0.0), len(0.0), len(0.0)],
                direction: [scalar(0.0), scalar(0.0), scalar(1.0)],
            }),
        },
    );
    let spine = *doc.order().last().expect("datum");
    (doc, spine)
}

fn stored_minor_bits(body: &topo::Body<f64>) -> Vec<u64> {
    let mut bits: Vec<u64> = body
        .faces()
        .filter_map(|(_, face)| match body.get_surface(face.surface) {
            Some(Surface::Torus { minor_radius, .. }) => Some(minor_radius.to_bits()),
            _ => None,
        })
        .collect();
    bits.sort_unstable();
    bits
}

/// The storage contract where the subtraction is INEXACT: 0.3 − 0.1 is
/// 0.19999999999999998, one ulp under 0.2, and a door that re-derived
/// the inner radius any other way (0.2 typed directly, a rescale, a
/// profile reconstruction) would store a different bit pattern.
#[test]
fn the_storage_contract_holds_at_non_dyadic_values() {
    for (outer, wall) in [(0.3_f64, 0.1_f64), (0.123456789_f64, 0.017_f64)] {
        let inner = outer - wall; // the caller's own one IEEE subtraction
        let (mut doc, spine) = axis_doc();
        doc = push(
            &doc,
            &DocEdit::InsertNode {
                node: Node::HollowTube {
                    spine,
                    u_ref: [scalar(1.0), scalar(0.0), scalar(0.0)],
                    major_radius: len(2.0),
                    window: TubeWindow::Arc {
                        t0: angle(0.0),
                        t1: angle(1.5),
                    },
                    minor_radius: len(outer),
                    wall: len(wall),
                },
            },
        );
        let tube = *doc.order().last().expect("tube");
        let ev = eval::<f64>(&doc);
        assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
        let body = body_of(&ev, tube);
        let mut want = vec![
            outer.to_bits(),
            outer.to_bits(),
            inner.to_bits(),
            inner.to_bits(),
        ];
        want.sort_unstable();
        assert_eq!(
            stored_minor_bits(body),
            want,
            "outer {outer} wall {wall}: stored radii are not the caller's own numbers \
             bit for bit (inner must be {inner:?} = {:#018x})",
            inner.to_bits()
        );
    }
    // And the solid door at a non-dyadic minor radius stores it verbatim.
    let (mut doc, spine) = axis_doc();
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Tube {
                spine,
                u_ref: [scalar(1.0), scalar(0.0), scalar(0.0)],
                major_radius: len(2.0),
                window: TubeWindow::Full,
                minor_radius: len(0.3),
            },
        },
    );
    let tube = *doc.order().last().expect("tube");
    let ev = eval::<f64>(&doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    assert_eq!(
        stored_minor_bits(body_of(&ev, tube)),
        vec![0.3_f64.to_bits(); 2],
        "the solid door must store a non-dyadic minor radius verbatim"
    );
}

/// Two tube nodes of IDENTICAL parameters in one document: the revolve
/// template mints their names apart (the minting node discriminates),
/// every table is total, and the memo layer is entitled to share their
/// content key without sharing a single name.
#[test]
fn identical_tubes_in_one_document_mint_disjoint_total_name_tables() {
    let (mut doc, spine) = axis_doc();
    let mk = || Node::Tube {
        spine,
        u_ref: [scalar(1.0), scalar(0.0), scalar(0.0)],
        major_radius: len(2.0),
        window: TubeWindow::Full,
        minor_radius: len(0.5),
    };
    doc = push(&doc, &DocEdit::InsertNode { node: mk() });
    let first = *doc.order().last().expect("first tube");
    doc = push(&doc, &DocEdit::InsertNode { node: mk() });
    let second = *doc.order().last().expect("second tube");

    let ev = eval::<f64>(&doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));

    let table = |id| {
        ev.value(id)
            .expect("a value")
            .name_table
            .iter()
            .map(|(name, _)| name.clone())
            .collect::<std::collections::BTreeSet<_>>()
    };
    let (a, b) = (table(first), table(second));
    // Total over each body...
    for (id, t) in [(first, &a), (second, &b)] {
        let body = body_of(&ev, id);
        let entities = body.faces().count() + body.edges().count() + body.vertices().count() + 1;
        assert_eq!(t.len(), entities, "{id:?}: name table not total");
    }
    // ...and DISJOINT between the two nodes, identical parameters or
    // not: a shared name would let a selector on one body resolve
    // against the other.
    assert!(
        a.is_disjoint(&b),
        "two tubes of identical parameters share minted names: {:?}",
        a.intersection(&b).collect::<Vec<_>>()
    );
}

/// The corner the corpus does not register (`HollowTube` + `Full`) —
/// the CAVITY topology — is named totally by the unmodified revolve
/// emitter: every face, edge and vertex of BOTH shells.
#[test]
fn a_hollow_full_rings_cavity_faces_are_named_totally() {
    let (mut doc, spine) = axis_doc();
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::HollowTube {
                spine,
                u_ref: [scalar(1.0), scalar(0.0), scalar(0.0)],
                major_radius: len(2.0),
                window: TubeWindow::Full,
                minor_radius: len(0.5),
                wall: len(0.125),
            },
        },
    );
    let tube = *doc.order().last().expect("tube");
    let ev = eval::<f64>(&doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let body = body_of(&ev, tube);
    assert_eq!(body.shells().count(), 2, "a torus shell plus its cavity");
    let entities = body.faces().count() + body.edges().count() + body.vertices().count() + 1;
    assert_eq!(
        ev.value(tube).expect("a value").name_table.len(),
        entities,
        "the cavity shell's entities must be named — a template that only reached \
         the outer shell would leave a silent naming dead end"
    );
}
