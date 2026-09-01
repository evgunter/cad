//! **The chamfered die** — the die of [`crate::diefillet`], one verb
//! over: `chamfer_edges` at setback `d` where that scene calls
//! `Node::fillet` at radius `r`, with `d == r` so the montage pair
//! differs in the VERB and in nothing else.
//!
//! Two stops, both starting from the same source solid the filleted
//! die starts from (`diefillet::source_bodies`):
//!
//! - **the chamfered blank** — a unit cube with all twelve edges
//!   chamfered. Six shrunk planar faces, twelve flat strips, eight
//!   flat corner triangles: **26 faces, 48 edges, 24 vertices**, the
//!   same census the rolling-ball blank has at the same size, because
//!   the two verbs carve the same neighbourhoods with the same
//!   surgery. The scene MEASURES the rest of that claim rather than
//!   asserting it from memory: at `r == d` it matches the two bodies'
//!   vertex sets point for point and reports the largest coordinate
//!   difference it found (the ball at rest is `r` inside all three
//!   supports, so its foot on each support is where the two trimlines
//!   cross — which is where the chamfer's foot is by construction;
//!   `crates/sweep/tests/verbs_chamfer.rs` pins the fillet's half of
//!   that bit-exactly and the chamfer's to within an ulp, since a
//!   line-crossing solve and a projection are two exact forms of one
//!   point and neither promises the same `f64`). **What it measures on
//!   this cube is stronger than what either verb promises**: the gap
//!   is `0` and all 24 feet land on the same `f64` in all three
//!   coordinates. That is an OBSERVATION at `L = 1`, `r = d = 0.12`,
//!   not a contract — the scene asserts the ulp-level claim the verbs
//!   do promise and prints what it actually found, so a future run
//!   that comes back at one ulp is news rather than a failure.
//! - **the chamfered die** — the pipped cube's twelve box edges
//!   chamfered IN PLACE, the 21 pip cavities carried through as
//!   rings. The montage panel beside `diecomposed`.
//!
//! # What the pip rims do here, and why
//!
//! `diecomposed` follows its box-edge fillet with a second call that
//! replaces all 21 pip rims with torus bands. There is no such second
//! call here: the chamfer's v1 door is **plane–plane only**, so a
//! plane–sphere rim refuses `ChamferArmUnsupported` (VERBS-ARMS is
//! where curved supports arrive). The chamfered die therefore keeps
//! its pip rims sharp, and that difference is the door's scope
//! statement standing where you can see it, not an omission.
//!
//! # Findings this scene records (the demo-purpose rule)
//!
//! 1. **This scene stays kernel-direct, and that is now a CHOICE.**
//!    `Node::Chamfer` exists (LIB-G16), so the die this scene renders
//!    is sayable as a document — `select_where(CurveKind = Line)` into
//!    `Node::chamfer`, the path
//!    `crates/pncad-py/tests/test_north_star.py::TestDiechamferDie`
//!    executes. What the scene keeps recording is the KERNEL-direct
//!    seat: it evaluates the shared recipe, takes the source body OUT,
//!    and does the surgery beside it, so the result has no node and no
//!    names. That is the cost of calling the verb next to a document —
//!    the names, not the selection: [`line_edges`] below says "the
//!    twelve box edges" through the seat's own doors (the materializer
//!    filtered by the carrier-kind predicate, the same one
//!    implementation `select_where` delegates to), so what a document
//!    buys over a body is the NAMES the answer comes back in, not the
//!    reach of the question.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::geom_core::Tol;
use pncad::prelude::{CurveKind, CurveKindSet, EdgeKey, chamfer_edges, fillet_edges, query};
use pncad::topo::Body;

use crate::diefillet::{L, R};
use crate::{SceneBody, Stop, View};

/// The setback. The point of the pair is that this IS the fillet's
/// radius, so the panels compare verbs rather than parameters.
const D: f64 = R;

/// **The twelve box edges of the pipped cube**: the only edges on it
/// whose carrier is a LINE, since every pip cavity contributes circles
/// (two rim arcs, two meridian seams).
///
/// This is `select_where(CurveKind = Line)`'s geometric half, said at
/// the body seat: the kernel's own materializer filtered by its own
/// carrier-kind predicate — the same one implementation the document
/// door delegates to, answering in keys because keys are this seat's
/// vocabulary.
fn line_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    query::all_edges(body)
        .into_iter()
        .filter(|&e| query::edge_carrier_matches(body, e, CurveKindSet::just(CurveKind::Line)))
        .collect()
}

/// Every vertex point of a body in one deterministic order, so two
/// bodies' vertex sets can be compared point for point.
///
/// `crates/sweep/tests/verbs_chamfer.rs` carries the same helper and
/// the same proximity match. The copy is deliberate: the only shared
/// home available is `sweep::test_support`, which is gated behind a
/// test-support feature — a demo that linked test scaffolding to save
/// twenty lines would stop being an outside consumer, which is the
/// one property these scenes exist to have.
fn sorted_points(body: &Body<f64>) -> Vec<(f64, f64, f64)> {
    let mut pts: Vec<(f64, f64, f64)> = body
        .vertices()
        .filter_map(|(k, _)| body.get_vertex(k))
        .filter_map(|v| body.get_point(v.point))
        .map(|p| (p.x, p.y, p.z))
        .collect();
    pts.sort_by(|a, b| a.partial_cmp(b).expect("finite coordinates"));
    pts
}

/// How far apart the two blanks' feet actually land, and how many of
/// the 24 land on the same `f64` in all three coordinates. Matched by
/// PROXIMITY, not by sort order: an ulp of difference moves a
/// coordinate across the sort key, and the claim is about the points.
fn feet_agreement(filleted: &Body<f64>, chamfered: &Body<f64>) -> (f64, usize) {
    let want = sorted_points(filleted);
    let got = sorted_points(chamfered);
    assert_eq!(want.len(), got.len(), "the two blanks have the same feet");
    let mut worst: f64 = 0.0;
    let mut identical = 0usize;
    for w in &want {
        let (near, gap) = got
            .iter()
            .map(|g| {
                let d = (g.0 - w.0)
                    .abs()
                    .max((g.1 - w.1).abs())
                    .max((g.2 - w.2).abs());
                (g, d)
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).expect("finite"))
            .expect("a nearest foot");
        worst = worst.max(gap);
        if near == w {
            identical += 1;
        }
    }
    (worst, identical)
}

/// The material a chamfer of setback `d` takes off a box edge lattice
/// of side `a`: twelve triangular prisms less the eight corner
/// over-counts, `6·a·d² − (16/3)·d³` (the closed form
/// `crates/sweep/tests/verbs_chamfer.rs` derives). The pips sit far
/// enough inboard that the chamfer never reaches one, so this is the
/// WHOLE difference between the source solid and the chamfered die.
fn edge_material(a: f64, d: f64) -> f64 {
    6.0 * a * d * d - (16.0 / 3.0) * d.powi(3)
}

pub fn stops(tol: Tol) -> Vec<Stop> {
    let (cube, pipped) = crate::diefillet::source_bodies(tol);

    // ---- the blank, both verbs, at r == d ----
    let filleted = fillet_edges(&cube, &query::all_edges(&cube), R, tol)
        .expect("a cube's twelve edges fillet")
        .body;
    let chamfered = chamfer_edges(&cube, &query::all_edges(&cube), D, tol)
        .expect("a cube's twelve edges chamfer")
        .body;
    let (bf, be, bv) = (
        chamfered.faces().count(),
        chamfered.edges().count(),
        chamfered.vertices().count(),
    );
    assert_eq!(
        (bv, be, bf),
        (
            filleted.vertices().count(),
            filleted.edges().count(),
            filleted.faces().count()
        ),
        "the two verbs carve the same neighbourhoods, so the census is the same"
    );
    assert_eq!((bv, be, bf), (24, 48, 26), "census");
    let (worst, identical) = feet_agreement(&filleted, &chamfered);
    assert!(
        worst <= 1e-15,
        "the two verbs' feet coincide on a right corner; worst gap {worst:e}"
    );

    // ---- the die: the twelve box edges of the PIPPED cube ----
    let (sv, se, sf) = (
        pipped.vertices().count(),
        pipped.edges().count(),
        pipped.faces().count(),
    );
    let box_edges = line_edges(&pipped);
    assert_eq!(
        box_edges.len(),
        12,
        "the only LINES are the twelve box edges"
    );
    let die =
        chamfer_edges(&pipped, &box_edges, D, tol).expect("the pipped cube's box edges chamfer");
    assert_eq!(die.blend_faces.len(), 12, "one strip per edge");
    assert_eq!(die.corner_faces.len(), 8, "one patch per corner");
    assert!(
        die.band_faces.is_empty(),
        "a chamfer has no closed-chain band"
    );
    let die = die.body;
    let (df, de, dv) = (
        die.faces().count(),
        die.edges().count(),
        die.vertices().count(),
    );
    // The surgery's own arithmetic, over a source carrying 21 rings:
    // twelve edges and eight corners retire, twelve strips and eight
    // patches arrive, and the pip cavities are untouched.
    assert_eq!(
        (dv, de, df),
        (sv - 8 + 24, se - 12 + 48, sf + 12 + 8),
        "the chamfer's census delta over the source, rings carried through"
    );

    let src_props = pncad::topo::mass_properties(&pipped, tol).expect("source props");
    let props = pncad::topo::mass_properties(&die, tol).expect("die props");
    let want = src_props.volume - edge_material(L, D);
    let slack = 1e-9 * want + src_props.volume_pad + props.volume_pad;
    assert!(
        (props.volume - want).abs() <= slack,
        "the chamfer took exactly the box-edge material: {} vs {want}",
        props.volume
    );

    let view = || View {
        elev: 26.0,
        azim: -50.0,
        up: 'z',
    };

    vec![
        Stop {
            name: "diechamferblank",
            caption: "the die blank (chamfers)".to_string(),
            // Standalone for the same reason `diefillet`'s blank is
            // (Evan, #218 follow-up): the partial die reads as a
            // near-duplicate of the composed one on the sheet. The
            // sheet's chamfer panel is `diechamfer`.
            montage: false,
            story: "every edge of a cube broken flat at one setback — twelve strips and \
                    eight corner triangles, every face a plane",
            ops: "chamfer_edges(cube, all twelve edges, d = 0.12): the fillet's battery \
                  minus the two rolling-ball predicates, then a ruled strip where the \
                  cylinder would go",
            delta: 5e-3,
            note: Some(format!(
                "at d == r the chamfered blank and the rolling-ball blank have the SAME \
                 census — {bv} vertices, {be} edges, {bf} faces — because the two verbs \
                 carve the same neighbourhoods with the same surgery and differ only in the \
                 band grafted in. Measured here, not recalled: matching the two vertex sets \
                 point for point, the worst coordinate gap is {worst:e} m and {identical} of \
                 the {bv} feet land on the same f64 in all three coordinates. The ball at \
                 rest is r inside all three supports, so its foot is where the two trimlines \
                 cross — which is where the chamfer puts its foot by construction"
            )),
            view: view(),
            bodies: vec![SceneBody::plain(
                "diechamferblank",
                [0.66, 0.74, 0.70],
                chamfered,
            )],
        },
        Stop {
            name: "diechamfer",
            caption: "THE CHAMFERED DIE (the same die, one verb over)".to_string(),
            montage: true,
            story: "the die of `diecomposed` with its twelve box edges BROKEN instead of \
                    rolled — same source solid, same size, a flat strip where the \
                    quarter-cylinder was",
            ops: "cube ∖ 21 pips (the shared recipe), then chamfer_edges(the twelve LINE \
                  edges, d = 0.12) in place: faces split along the strips' trimlines, the \
                  21 pip rings carried through, eight corner triangles grafted",
            delta: 5e-3,
            note: Some(format!(
                "{df} faces, {de} edges, {dv} vertices — the source's {sf}/{se}/{sv} with \
                 twelve edges and eight corners retired and twelve strips and eight patches \
                 arrived. V = {:.6} m³, which is the source's {:.6} less exactly the \
                 box-edge material 6·L·d² − (16/3)·d³ = {:.6} m³: the chamfer never reaches \
                 a pip. The pip rims stay SHARP because the chamfer's v1 door is \
                 plane–plane — a plane–sphere rim refuses ChamferArmUnsupported, where \
                 `diecomposed`'s second call rolls a torus band into it. \"The twelve box \
                 edges\" is said here with the kernel's own carrier-kind selector \
                 (`query::all_edges` filtered by `query::edge_carrier_matches`), the same \
                 one implementation `select_where` delegates to. This body has no \
                 document because the scene calls the verb DIRECTLY; the recipe door \
                 (`Node::chamfer`, LIB-G16) is what a document-modelling consumer uses",
                props.volume,
                src_props.volume,
                edge_material(L, D)
            )),
            view: view(),
            bodies: vec![SceneBody::plain("diechamfer", [0.58, 0.70, 0.66], die)],
        },
    ]
}
