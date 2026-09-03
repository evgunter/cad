//! **M5 PR 12 fix pass F5 — the `Fillet` recipe node**, and the
//! blocker that once kept its Band 4 document out of the corpus
//! registry (retired at `5c8540f`; `die_fillet` is registered now).
//!
//! `Node::Fillet` wires
//! [`sweep::blend::build::fillet_edges`](sweep::blend::build::fillet_edges)
//! over an authored selection of its target body's edges — here every
//! edge of the cube. A parameter edit that mints and retires no edge
//! cannot make that selection stale, which the bump row below
//! executes rather than asserts.
//!
//! The document under test is `corpus/die_fillet.rs`. It was written
//! to registry shape but held out of it while the Interval blocker
//! stood, and this file was the standard-row substitute — deliberately
//! STRICTER than the registry rows in the one place that mattered,
//! pinning the Interval behaviour EXACTLY so the blocker would retire
//! itself. It did: the row below now asserts the Interval lane GREEN,
//! and the document carries the ordinary registry rows on top of it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;

use corpus::{body_of, cone, die_fillet, eval, failures};
use editor_core::{CancelToken, EvalOptions, EvalOutcome, evaluate};
use geom_core::Tol;

/// A rounded box of side `L` at radius `r`: core + 6 slabs + 12
/// quarter-cylinders + 8 octants (the octants summing to one ball).
/// π-valued, hence NOT a dyadic pin — see the document's module docs.
fn rounded_box(l: f64, r: f64) -> (f64, f64) {
    use core::f64::consts::PI;
    let c = l - 2.0 * r;
    let volume = c.powi(3)
        + 6.0 * r * c.powi(2)
        + 12.0 * (PI * r * r / 4.0) * c
        + (4.0 / 3.0) * PI * r.powi(3);
    let area = 6.0 * c.powi(2) + 12.0 * (PI * r / 2.0) * c + 4.0 * PI * r * r;
    (volume, area)
}

/// **The f64 row**: the document evaluates green, its head is a valid
/// closed solid, and its certified mass meets the rounded-box closed
/// form at rounding scale (a relative tolerance, stated — the oracle
/// carries π and no `==` is honest against it).
#[test]
fn die_fillet_evaluates_green_and_meters_the_closed_form() {
    let d = die_fillet::document();
    let ev = eval::<f64>(&d.doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "die_fillet:\n{}", bad.join("\n"));
    assert_eq!(ev.outcome, EvalOutcome::Completed);
    assert_eq!(ev.order.len(), d.len());

    let body = body_of(&ev, d.result.expect("the blank is the head"));
    assert_eq!(topo::validate(body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(body), Ok(()), "closed");

    // Planes AND cylinders AND spheres at once: 6 shrunken supports,
    // 12 quarter-cylinder blends, 8 sphere-octant corners.
    assert_eq!(
        body.faces().count(),
        26,
        "6 supports + 12 blends + 8 corners"
    );

    let m = topo::mass_properties(body, Tol::witness()).expect("mass properties");
    let (v, a) = rounded_box(die_fillet::L, die_fillet::R);
    assert!(
        (m.volume - v).abs() <= 1e-9 * v,
        "volume {} vs closed form {v}",
        m.volume
    );
    assert!(
        (m.surface_area - a).abs() <= 1e-9 * a,
        "area {} vs closed form {a}",
        m.surface_area
    );
    assert_eq!(
        m.volume_pad, 0.0,
        "a closed-form body needs no enclosure pad"
    );
}

/// **The bump row** (D2's incremental probe, the registry's third
/// standard row): a parameter edit on the EXTRUDE recomputes exactly
/// its downstream cone and reuses the rest — and the fillet node
/// downstream of it survives the edit with NO re-selection, because
/// the bump mints and retires no edge.
#[test]
fn the_extrude_bump_recomputes_the_cone_and_the_fillet_needs_no_reselection() {
    let d = die_fillet::document();
    let full = eval::<f64>(&d.doc);
    assert!(failures(&full).is_empty());
    let bumped = d.bumped();
    let expected = cone(&bumped, d.bump_root);
    let after = evaluate::<f64>(
        &bumped,
        Some(&full),
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let bad = failures(&after);
    assert!(bad.is_empty(), "bumped die_fillet:\n{}", bad.join("\n"));
    assert_eq!(
        (after.recomputed, after.reused),
        (expected.len(), bumped.len() - expected.len()),
        "bump cone {expected:?} of {}",
        bumped.len()
    );
    assert!(after.reused > 0, "the profile must be reused");

    // The stretched blank is still a rounded box, metered the same way.
    let body = body_of(&after, d.result.expect("head"));
    assert_eq!(topo::validate_closed(body), Ok(()), "closed after the bump");
    let m = topo::mass_properties(body, Tol::witness()).expect("mass properties");
    // L × L × L_BUMPED with radius R: the general rounded-box form
    // specialises to the cube one only when the box is a cube, so the
    // pin here is the cheap invariant — the bump grew the solid.
    let (v, _) = rounded_box(die_fillet::L, die_fillet::R);
    assert!(
        m.volume > v,
        "the bump stretched the blank: {} vs {v}",
        m.volume
    );
}

/// **The persistence row** (D6.1): the document round-trips through
/// save/load bit-identically and replays to the same evaluation — the
/// `Fillet` node and its `SlotId::Radius` slot ride the schema like
/// every other node.
#[test]
fn die_fillet_round_trips_and_replays_identically() {
    let d = die_fillet::document();
    let text = editor_core::save(&d.doc, &[], Tol::witness()).expect("save");
    let loaded = editor_core::load(&text, Tol::witness()).expect("load");
    assert!(loaded.doc.bit_eq(&d.doc), "round-trip bit identity");
    let before = eval::<f64>(&d.doc);
    let after = eval::<f64>(&loaded.doc);
    assert_eq!(
        format!("{:?}|{:?}", before.order, before.nodes),
        format!("{:?}|{:?}", after.order, after.nodes),
        "replay identity"
    );
}

/// **Fail loud, never pass through**: a radius the front door cannot
/// admit is a TYPED node failure carrying the kernel's own error, not
/// a silently sharp solid. `r = 0.625` on the unit cube leaves the
/// clearance screen nothing to certify (two opposite blends at
/// `r > L/2` would erase the support between them).
#[test]
fn an_inadmissible_radius_fails_the_node_typed() {
    use editor_core::{DocEdit, NodeErrorKind, NodeResult, SlotId, apply};

    let d = die_fillet::document();
    let head = d.result.expect("head");
    let doc = apply(
        &d.doc,
        &DocEdit::SetParam {
            node: head,
            slot: SlotId::Radius,
            expr: fixture::len(0.625),
        },
        Tol::witness(),
    )
    .expect("radius edit applies")
    .doc;
    let ev = eval::<f64>(&doc);
    match ev.nodes.get(&head) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::Blend { error: inner, .. } => {
                let text = format!("{inner}");
                assert!(
                    text.contains("cannot certify"),
                    "the refusal must carry the kernel's own wording: {text}"
                );
                // The kernel's wording must survive the RENDER too, not
                // just the variant. A caller who cannot match the enum
                // — the Python bindings, whose exception carries a tag
                // plus this string — has no other channel to the face,
                // the margin and the recourse the kernel named.
                let rendered = e.to_string();
                assert!(
                    rendered.ends_with(&text),
                    "the node error's message must forward the kernel's own: {rendered}"
                );
            }
            other => panic!("expected a typed Fillet refusal, got {other:?}"),
        },
        other => panic!("the node must FAIL, never pass its input through: {other:?}"),
    }
    // And nothing downstream silently inherited the sharp cube.
    assert!(ev.value(head).is_none(), "no value on a failed fillet");
}

/// **The Interval lane, GREEN** — and the row records why it took a
/// gate to get here.
///
/// HISTORY: the battery's clearance screen seeded its gap with
/// `T::from_f64(f64::INFINITY)`. That is NaI at the certified interval
/// scalar, NaI absorbs through `min`, and the margin reaching `decide`
/// was NaN — so `fillet3_face_clearance` escalated `Invalid` for every
/// prism, the whole fillet op was outside the Interval lane, and
/// `die_fillet` had to sit BESIDE the corpus registry rather than in
/// it. The consequence the CI gate surfaced is the one that matters:
/// a `fillet3_*` family that records ZERO samples in the K corpus,
/// which is a new predicate family with no telemetry at all.
///
/// The fix is to seed the gap from the first sampled pair instead of
/// from a sentinel. This row is now the green assertion, and the
/// registry membership beside it (`corpus::documents()`) is what
/// carries the standard Interval, persistence and latency rows.
#[cfg(feature = "interval")]
#[test]
fn the_interval_lane_evaluates_the_fillet_green() {
    use editor_core::NodeResult;
    use geom_core::Interval;

    let d = die_fillet::document();
    let ev = eval::<Interval>(&d.doc);
    for &id in &ev.order {
        assert!(
            matches!(ev.nodes.get(&id), Some(NodeResult::Ok(_))),
            "node {id:?} must be green at Interval — got {:?}",
            ev.nodes.get(&id)
        );
    }
}
