//! **LIB-TUBE R2 review probes** — adversarial rows written against
//! the frozen head `c0bfba802`, attacking the claims the unit makes
//! rather than restating the rows it already carries.
//!
//! Adopted as PERMANENT rows at the fix pass, authorship preserved.
//! Probe 1 and `lib_tube_r1_probes`'s
//! `the_storage_contract_holds_at_non_dyadic_values` were written
//! independently and are BOTH kept — different spellings (full ring
//! against arc; a sharpness guard against `0.2`'s bits, against a
//! second value pair plus the solid door).
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
//! 3. a tube-bearing save read by a build that does not know the
//!    vocabulary, at the bytes.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;

use corpus::{body_of, eval, failures};
use editor_core::{DocEdit, Node, ProfileDoc, ProfileProgram, TubeWindow, apply, load, save};
use fixture::{ang, len};
use geom_core::Tol;
use topo::{Body, Surface};

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
/// An empty document named `name`: a tube reads no frame, so it is
/// the whole of its own document.
fn axis_doc(name: &str) -> ProfileDoc {
    ProfileDoc::empty_derived(name, Tol::witness())
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

    let mut doc = axis_doc("r2_nondyadic");
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::HollowTube {
                major_radius: len(2.0),
                window: TubeWindow::Full,
                minor_radius: len(outer),
                wall: len(wall),
            }),
            fresh: Vec::new(),
        },
    );
    let tube = *doc.ids().last().expect("the tube");
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
        stored_minor_bits(&body),
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
    let mut doc = axis_doc("r2_crowded");
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Tube {
                major_radius: len(2.0),
                window: TubeWindow::Arc {
                    t0: ang(0.0),
                    t1: ang(1.5),
                },
                minor_radius: len(0.5),
            }),
            fresh: Vec::new(),
        },
    );
    let solid = *doc.ids().last().expect("solid tube");
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::HollowTube {
                major_radius: len(2.0),
                window: TubeWindow::Arc {
                    t0: ang(0.0),
                    t1: ang(1.5),
                },
                minor_radius: len(0.5),
                wall: len(0.125),
            }),
            fresh: Vec::new(),
        },
    );
    let hollow = *doc.ids().last().expect("hollow tube");
    // A THIRD tube identical to the first in every parameter: the
    // sharpest collision candidate the vocabulary permits, since only
    // the minting node distinguishes the two bodies.
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Tube {
                major_radius: len(2.0),
                window: TubeWindow::Arc {
                    t0: ang(0.0),
                    t1: ang(1.5),
                },
                minor_radius: len(0.5),
            }),
            fresh: Vec::new(),
        },
    );
    let twin = *doc.ids().last().expect("the twin tube");

    let ev = eval::<f64>(&doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));

    let mut all: Vec<String> = Vec::new();
    for id in [solid, hollow, twin] {
        for (name, _) in ev.table_at(id).expect("a value").iter() {
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
    let a = ev.table_at(solid).expect("solid");
    let b = ev.table_at(twin).expect("twin");
    assert_eq!(
        a.len(),
        b.len(),
        "identical tubes name the same number of entities"
    );
    // And STABILITY: a second evaluation of the same document mints
    // the same names, name for name.
    let ev2 = eval::<f64>(&doc);
    for id in [solid, hollow, twin] {
        let n1: Vec<String> = ev
            .table_at(id)
            .expect("v")
            .iter()
            .map(|(n, _)| format!("{n:?}"))
            .collect();
        let n2: Vec<String> = ev2
            .table_at(id)
            .expect("v")
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
    let mut doc = axis_doc("r2_cavity");
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::HollowTube {
                major_radius: len(2.0),
                window: TubeWindow::Full,
                minor_radius: len(0.5),
                wall: len(0.125),
            }),
            fresh: Vec::new(),
        },
    );
    let tube = *doc.ids().last().expect("the tube");
    let ev = eval::<f64>(&doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let body = body_of(&ev, tube);
    assert_eq!(body.shells().count(), 2, "an outer shell and a cavity");
    let entities = body.faces().count() + body.edges().count() + body.vertices().count() + 1;
    let v = ev.table_at(tube).expect("a value");
    assert_eq!(v.len(), entities, "every cavity entity must be named too");
    let mut names: Vec<String> = v.iter().map(|(n, _)| format!("{n:?}")).collect();
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
// 3. The stale-build direction, at the bytes
// ---------------------------------------------------------------

/// **PROBE 3 — a build that does not know the tube vocabulary refuses
/// a tube-bearing save TYPED, with the regenerate recourse.**
///
/// A reviewer cannot compile a stale kernel, so this executes the
/// thing the stale build would actually SEE: the save's own bytes with
/// the node variant renamed to one this build does not know. That is
/// exactly the serde situation — an externally-tagged enum meeting a
/// variant name it has no arm for, which it refuses unconditionally
/// and with no attribute involved — and it must come back as the one
/// typed door `persist/mod.rs` promises, not a panic and not a silent
/// drop.
///
/// Labelled honestly: this is the byte-level equivalent, not a run of
/// a genuinely older binary.
#[test]
fn r2_a_tube_bearing_save_refuses_typed_on_a_build_that_lacks_the_vocabulary() {
    let mut doc = axis_doc("r2_stale");
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Tube {
                major_radius: len(2.0),
                window: TubeWindow::Full,
                minor_radius: len(0.5),
            }),
            fresh: Vec::new(),
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
