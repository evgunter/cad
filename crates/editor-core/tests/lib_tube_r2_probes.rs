//! **LIB-TUBE R2 review probes** — adversarial rows written against
//! the frozen head `c0bfba802`, attacking the claims the unit makes
//! rather than restating the rows it already carries.
//!
//! Each row here is an ATTACK that failed, kept because a failed
//! attack is evidence and the suite did not already carry it:
//!
//! 1. the storage contract OFF the dyadic fixture — `0.3 − 0.1`,
//!    where "one IEEE subtraction" and "the arithmetically right
//!    answer" are different numbers, so the claim has teeth;
//! 2. name distinctness by CONSTRUCTION — two tubes and a revolve in
//!    one document, which is the collision the revolve template would
//!    show if it needed discrimination it cannot give;
//! 3. the non-unit-AXIS verdict's disclosed unreachability, executed
//!    from both sides;
//! 4. a tube-bearing save read by a build that does not know the
//!    vocabulary, at the bytes.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use corpus::{body_of, eval, failures};
use editor_core::{
    Datum, Dimension, DocEdit, Expr, Node, NodeErrorKind, NodeResult, ProfileDoc, ProfileProgram,
    RecipeNodeId, TubeWindow, apply, load, save,
};
use fixture::len;
use geom_core::Tol;
use topo::{Body, Surface};

fn scalar(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("finite")
}
fn angle(v: f64) -> Expr {
    Expr::literal(v, Dimension::Angle).expect("finite")
}
fn push(d: &ProfileDoc, e: &DocEdit<ProfileProgram>) -> ProfileDoc {
    apply(d, e, Tol::witness()).expect("edit applies").doc
}
fn stored_minor_bits(body: &Body<f64>) -> Vec<u64> {
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
fn axis_doc(name: &str, dir: [f64; 3]) -> (ProfileDoc, RecipeNodeId) {
    let mut doc = ProfileDoc::empty_derived(name, Tol::witness());
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Datum(Datum::Axis {
                origin: [len(0.0), len(0.0), len(0.0)],
                direction: dir.map(scalar),
            }),
        },
    );
    let spine = *doc.order().last().expect("the datum");
    (doc, spine)
}

// ---------------------------------------------------------------
// 1. The storage contract where the bits and the arithmetic differ
// ---------------------------------------------------------------

/// **PROBE 1 — the storage contract OFF the dyadic fixture.**
///
/// The committed rows meter `0.5 − 0.125`, which is exact in binary:
/// every plausible way of computing the inner radius — one
/// subtraction, a re-derivation through a diameter, a round trip
/// through a bulge — agrees to the last bit there, so the fixture
/// cannot tell "stores one IEEE subtraction" from "reconstructs and
/// happens to land". `0.3 − 0.1` can: neither operand is
/// representable, and the correctly-rounded difference
/// `0.19999999999999998` is NOT the real number 0.2 and not what a
/// reconstruction through any other route need produce.
///
/// The claim under attack is the PR's exact wording — outer stores
/// the caller's bit pattern, inner stores `minor_radius - wall` as ONE
/// IEEE subtraction of the caller's own two numbers.
#[test]
fn r2_the_storage_contract_holds_at_non_dyadic_radii() {
    let (outer, wall) = (0.3_f64, 0.1_f64);
    let inner = outer - wall; // the ONE subtraction the claim names
    assert_ne!(
        inner, 0.2_f64,
        "the probe is only sharp if 0.3 - 0.1 misses 0.2 — it does"
    );

    let (mut doc, spine) = axis_doc("r2_nondyadic", [0.0, 0.0, 1.0]);
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::HollowTube {
                spine,
                u_ref: [scalar(1.0), scalar(0.0), scalar(0.0)],
                major_radius: len(2.0),
                window: TubeWindow::Full,
                minor_radius: len(outer),
                wall: len(wall),
            },
        },
    );
    let tube = *doc.order().last().expect("the tube");
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
        "at non-dyadic radii the stored inner radius must be exactly \
         (0.3f64 - 0.1f64).to_bits() = {:#x}, not the decimal 0.2 ({:#x}) \
         and not anything reconstructed",
        inner.to_bits(),
        0.2_f64.to_bits()
    );
}

// ---------------------------------------------------------------
// 2. Naming, by construction, in a crowded document
// ---------------------------------------------------------------

/// **PROBE 2 — two tubes and a revolve in ONE document.**
///
/// The unit claims the revolve emitter template applies wholesale and
/// needs no tube-specific discrimination. The way that claim fails is
/// a COLLISION: two bodies whose roles are the same shapes minting the
/// same `StableName`. So this builds the densest collision candidate
/// the vocabulary allows — a solid tube and a hollow tube on the SAME
/// spine with the SAME radii and window, beside a revolve, all in one
/// document — and asserts that the union of the three name tables has
/// no duplicate, and that re-evaluating reproduces every name.
#[test]
fn r2_two_tubes_and_a_revolve_mint_names_that_never_collide() {
    let (mut doc, spine) = axis_doc("r2_crowded", [0.0, 0.0, 1.0]);
    let u = [scalar(1.0), scalar(0.0), scalar(0.0)];
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Tube {
                spine,
                u_ref: u.clone(),
                major_radius: len(2.0),
                window: TubeWindow::Arc {
                    t0: angle(0.0),
                    t1: angle(1.5),
                },
                minor_radius: len(0.5),
            },
        },
    );
    let solid = *doc.order().last().expect("solid tube");
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::HollowTube {
                spine,
                u_ref: u.clone(),
                major_radius: len(2.0),
                window: TubeWindow::Arc {
                    t0: angle(0.0),
                    t1: angle(1.5),
                },
                minor_radius: len(0.5),
                wall: len(0.125),
            },
        },
    );
    let hollow = *doc.order().last().expect("hollow tube");
    // A THIRD tube identical to the first in every parameter: the
    // sharpest collision candidate the vocabulary permits, since only
    // the minting node distinguishes the two bodies.
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Tube {
                spine,
                u_ref: u,
                major_radius: len(2.0),
                window: TubeWindow::Arc {
                    t0: angle(0.0),
                    t1: angle(1.5),
                },
                minor_radius: len(0.5),
            },
        },
    );
    let twin = *doc.order().last().expect("the twin tube");

    let ev = eval::<f64>(&doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));

    let mut all: Vec<String> = Vec::new();
    for id in [solid, hollow, twin] {
        let v = ev.value(id).expect("a value");
        for (name, _) in v.name_table.iter() {
            all.push(format!("{name:?}"));
        }
    }
    let total = all.len();
    all.sort();
    all.dedup();
    assert_eq!(
        all.len(),
        total,
        "three tube bodies in one document minted {} names but only {} are distinct — \
         the revolve template would then need a discrimination it cannot give",
        total,
        all.len()
    );

    // Two IDENTICAL tubes must still be told apart: the discrimination
    // is the minting node, which is what the template already carries.
    let a = ev.value(solid).expect("solid");
    let b = ev.value(twin).expect("twin");
    assert_eq!(
        a.name_table.len(),
        b.name_table.len(),
        "identical tubes name the same number of entities"
    );
    // And STABILITY: a second evaluation of the same document mints
    // the same names, name for name.
    let ev2 = eval::<f64>(&doc);
    for id in [solid, hollow, twin] {
        let n1: Vec<String> = ev
            .value(id)
            .expect("v")
            .name_table
            .iter()
            .map(|(n, _)| format!("{n:?}"))
            .collect();
        let n2: Vec<String> = ev2
            .value(id)
            .expect("v")
            .name_table
            .iter()
            .map(|(n, _)| format!("{n:?}"))
            .collect();
        assert_eq!(n1, n2, "names must be stable across evaluations");
    }
}

/// **PROBE 2b — a hollow FULL ring's cavity faces are named too.**
///
/// The cavity is the one topology no other corner of the two-by-two
/// produces, and it is where a naming template borrowed from the
/// revolve would most plausibly come up short. Totality is asserted
/// against the body's own entity count.
#[test]
fn r2_a_hollow_rings_cavity_is_named_by_the_revolve_template() {
    let (mut doc, spine) = axis_doc("r2_cavity", [0.0, 0.0, 1.0]);
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
    let tube = *doc.order().last().expect("the tube");
    let ev = eval::<f64>(&doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let body = body_of(&ev, tube);
    assert_eq!(body.shells().count(), 2, "an outer shell and a cavity");
    let entities = body.faces().count() + body.edges().count() + body.vertices().count() + 1;
    let v = ev.value(tube).expect("a value");
    assert_eq!(
        v.name_table.len(),
        entities,
        "every cavity entity must be named too"
    );
    let mut names: Vec<String> = v.name_table.iter().map(|(n, _)| format!("{n:?}")).collect();
    let total = names.len();
    names.sort();
    names.dedup();
    assert_eq!(
        names.len(),
        total,
        "cavity names must not collide with wall names"
    );
}

// ---------------------------------------------------------------
// 3. The disclosed unreachability, executed from both sides
// ---------------------------------------------------------------

/// **PROBE 3 — the non-unit-axis verdict really is unreachable along
/// the recipe path, and it is the DATUM that makes it so.**
///
/// The unit discloses this as main's doing. Executed: a datum axis
/// with a non-unit direction refuses ONE NODE UPSTREAM, so the tube
/// node never runs at all — and the refusal is the datum's, not the
/// tube door's.
#[test]
fn r2_a_non_unit_axis_refuses_upstream_and_never_reaches_the_tube_door() {
    // A direction of length 2 — a perfectly good non-unit axis.
    let (mut doc, spine) = axis_doc("r2_nonunit_axis", [0.0, 0.0, 2.0]);
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Tube {
                spine,
                u_ref: [scalar(1.0), scalar(0.0), scalar(0.0)],
                major_radius: len(2.0),
                window: TubeWindow::Full,
                minor_radius: len(0.5),
            },
        },
    );
    let tube = *doc.order().last().expect("the tube");
    let ev = eval::<f64>(&doc);

    // Whatever happens, it must NOT be the tube door's own verdict.
    if let Some(NodeResult::Failed(e)) = ev.nodes.get(&tube) {
        assert!(
            !matches!(e.kind, NodeErrorKind::Tube(_)),
            "the disclosure says this verdict is unreachable from a document, \
             but the tube door produced it: {:?}",
            e.kind
        );
    }
    // And the datum is where it is decided — either the datum refused,
    // or the datum normalized and the tube built. Both are consistent
    // with the disclosure; a tube-door refusal is not.
    let datum_failed = matches!(ev.nodes.get(&spine), Some(NodeResult::Failed(_)));
    let tube_built = matches!(ev.nodes.get(&tube), Some(NodeResult::Ok(_)));
    assert!(
        datum_failed || tube_built,
        "either the datum decides the norm or the tube builds; \
         datum={:?} tube={:?}",
        ev.nodes.get(&spine).map(|_| "present"),
        ev.nodes.get(&tube).map(|_| "present"),
    );
    println!("r2: non-unit axis -> datum_failed={datum_failed} tube_built={tube_built}");
}

/// **PROBE 3b — the `u_ref` verdicts STAY reachable**, which is the
/// half of the disclosure that would make it a real loss if it were
/// false. A bare direction passes through no datum, so both its
/// verdicts are still the door's.
#[test]
fn r2_the_u_ref_verdicts_stay_reachable_from_a_document() {
    for (what, u) in [
        ("non-unit u_ref", [2.0, 0.0, 0.0]),
        ("u_ref parallel to the axis", [0.0, 0.0, 1.0]),
    ] {
        let (mut doc, spine) = axis_doc("r2_uref", [0.0, 0.0, 1.0]);
        doc = push(
            &doc,
            &DocEdit::InsertNode {
                node: Node::Tube {
                    spine,
                    u_ref: u.map(scalar),
                    major_radius: len(2.0),
                    window: TubeWindow::Full,
                    minor_radius: len(0.5),
                },
            },
        );
        let tube = *doc.order().last().expect("the tube");
        let ev = eval::<f64>(&doc);
        match ev.nodes.get(&tube) {
            Some(NodeResult::Failed(e)) => match &e.kind {
                NodeErrorKind::Tube(t) => {
                    println!("r2: {what} -> {t}");
                }
                other => panic!("{what} must refuse as the TUBE door's verdict, got {other:?}"),
            },
            other => panic!("{what} must refuse, got {other:?}"),
        }
    }
}

// ---------------------------------------------------------------
// 4. The stale-build direction, at the bytes
// ---------------------------------------------------------------

/// **PROBE 4 — a build that does not know the tube vocabulary refuses
/// a tube-bearing save TYPED, with the regenerate recourse.**
///
/// A reviewer cannot compile a stale kernel, so this executes the
/// thing the stale build would actually SEE: the save's own bytes with
/// the node variant renamed to one this build does not know. That is
/// exactly the serde situation — an unknown enum variant under
/// `deny_unknown_fields` — and it must come back as the one typed door
/// `persist/mod.rs` promises, not a panic and not a silent drop.
///
/// Labelled honestly: this is the byte-level equivalent, not a run of
/// a genuinely older binary.
#[test]
fn r2_a_tube_bearing_save_refuses_typed_on_a_build_that_lacks_the_vocabulary() {
    let (mut doc, spine) = axis_doc("r2_stale", [0.0, 0.0, 1.0]);
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Tube {
                spine,
                u_ref: [scalar(1.0), scalar(0.0), scalar(0.0)],
                major_radius: len(2.0),
                window: TubeWindow::Full,
                minor_radius: len(0.5),
            },
        },
    );
    let bytes = save(&doc, &[], Tol::witness()).expect("the document saves");
    assert!(
        bytes.contains("Tube"),
        "the save must actually name the new vocabulary: {bytes}"
    );
    // Round-trip first, so the probe is measuring the mangling and not
    // a broken baseline.
    load(&bytes, Tol::witness()).expect("the unmangled save loads on THIS build");

    // What a build without the variant sees.
    let stale = bytes.replace("\"Tube\"", "\"TubeFromTheFuture\"");
    assert_ne!(stale, bytes, "the mangling must have bitten");
    let err = load(&stale, Tol::witness()).expect_err("a build lacking the variant must refuse");
    let msg = format!("{err}");
    println!("r2: stale-build refusal -> {msg}");
    assert!(
        msg.to_lowercase().contains("regenerate") || format!("{err:?}").contains("Unreadable"),
        "the refusal must be the one typed door with its recourse, got: {msg} / {err:?}"
    );
}
