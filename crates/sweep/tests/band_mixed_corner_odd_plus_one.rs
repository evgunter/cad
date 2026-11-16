//! **A mixed corner's odd edge with one pair edge**
//! (`work/band/a-boss-on-a-plate-meets-it-at-a-mixed-convexity-corner-no-run-out-rounds.md`,
//! "The odd edge and one pair edge"): the chamfer builds, the pair strip
//! cut off in the odd strip's plane. That is a turn whose overrun has
//! the odd band the far one: the odd strip is cut off in `E`, and its
//! face runs on past the face it shares with the pair strip, down to
//! the pair strip's foot on `E`, a coplanar triangle. The fillet
//! refuses: the pair band would have to stop where its edge runs
//! smoothly into the cut-off arc.
//!
//! The closed form, at plan angle `φ` (the angle of `E`'s notch at the
//! vertex, flush; of the boss, inset), `t = tan(φ/2)`, the odd edge `L`
//! long and the requested pair edge `Y` long from the vertex, each
//! other end cut off square:
//!
//! - flush: `ΔV = (s²/2)(L·sin φ − Y) + (s³/2)(1 + t/3)`, at a right
//!   angle `(s²/2)(L − Y) + (2/3)s³`: the odd strip's fill
//!   `(s²/2)·sin φ` over `L`, less the pair strip's section over the
//!   pair edge from the odd plane's trace, `x ≥ s − y·t` across the
//!   section's leg `y ∈ [−s, 0]`;
//! - inset: the flush one negated, the inset's void being the flush's
//!   material reflected.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::Surface;
use geom_core::Interval;
use sweep::blend::build::{Blended, fillet_edges};
use sweep::blend::{BlendError, CornerConfig};
use sweep::chamfer::chamfer_edges;
use sweep::test_support::{assert_naming_totality, at_rest, corners, prism_at};
use topo::{Body, EdgeKey, validate_geometric};

use crate::band_closed_mixed_chain_shapes::tightest_interval;
use crate::band_mixed_corner::{
    P, Scalar, at, boreless, edge, flush, flush_at, inset, inset_at, l_corner, l_prism, leaning,
    leaning_odd_and_one, tol, volume,
};

/// The size every single-corner row asks for, meters.
const S: f64 = 0.1;

/// The flush corner's chamfer of its odd edge with one pair edge, at
/// plan angle `phi`, the odd edge `l` long and the pair edge `y`.
fn flush_form(phi: f64, s: f64, (l, y): (f64, f64)) -> f64 {
    let t = (phi / 2.0).tan();
    s * s / 2.0 * (l * phi.sin() - y) + s.powi(3) / 2.0 * (1.0 + t / 3.0)
}

fn chamfer<T: Scalar>(body: &Body<T>, edges: &[EdgeKey], s: f64) -> Result<Blended<T>, BlendError> {
    chamfer_edges(&at_rest(body, tol()), edges, T::from_f64(s), tol()).map_err(|r| r.error)
}

/// How far `p` lies from a stored plane, as the largest magnitude its
/// bounds allow.
fn off<T: Scalar>(s: &Surface<T>, p: geom_core::Point3<T>) -> f64 {
    match s {
        Surface::Plane { origin, normal, .. } => {
            let d = (p - *origin).dot(*normal);
            d.lo().abs().max(d.hi().abs())
        }
        other => panic!("a chamfer's corner edge joins a {other:?}"),
    }
}

/// One build: tier 3, one closed genus-0 shell, naming totality, `ΔV`
/// at `truth` within the certified enclosure and `1e-12`, one mitre
/// and one turn foot at each of `corners` corners, and every edge the
/// corners minted (the overrun's end curves, the mitres, the trimlines)
/// on both its faces within `1e-12`.
fn builds<T: Scalar>(
    body: &Body<T>,
    edges: &[EdgeKey],
    (s, truth, corners): (f64, f64, usize),
    what: &str,
) -> Blended<T> {
    let out = chamfer(body, edges, s).unwrap_or_else(|e| panic!("{what}: builds, got {e:?}"));
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, got {e:?}"));
    let c = topo::readback::euler_counts(&out.body);
    assert_eq!(
        (c.s, c.genus()),
        (1, Ok(0)),
        "{what}: one closed genus-0 shell"
    );
    assert_naming_totality(body, &out, edges, what);
    let ((v0, pad0), (v1, pad1)) = (volume(body), volume(&out.body));
    let (dv, pad) = (v1 - v0, pad0 + pad1);
    assert!(
        pad < 1e-9 && (dv - truth).abs() < 1e-12 + pad,
        "{what}: ΔV {dv} ± {pad} vs the closed form {truth}"
    );
    let rec = out.naming.as_ref().expect("the carve records its births");
    assert_eq!(
        (rec.mitres.len(), rec.turn_feet.len(), rec.host_feet.len()),
        (corners, corners, 0),
        "{what}: a mitre and a turn foot at each corner, and no mixed corner's host feet"
    );
    let mut worst = 0.0f64;
    let minted: Vec<EdgeKey> = rec
        .arcs
        .iter()
        .map(|(e, _, _)| *e)
        .chain(rec.trims.iter().map(|(e, _, _)| *e))
        .chain(rec.mitres.iter().map(|(e, _)| *e))
        .collect();
    for &e in &minted {
        let edge = out.body.get_edge(e).expect("a minted edge is live");
        let curve = out
            .body
            .get_curve_geom(edge.curve)
            .and_then(|g| g.certified())
            .expect("a certified edge");
        let faces = [edge.he_plus, edge.he_minus].map(|h| {
            let f = out.body.face_of_half_edge(h).expect("an edge's face");
            out.body.face_surface(f).expect("a stored surface").clone()
        });
        let (t0, t1) = curve.params();
        for i in 0..=16 {
            let p = curve
                .carrier()
                .eval(t0 + (t1 - t0) * T::from_f64(f64::from(i) / 16.0));
            for f in &faces {
                worst = worst.max(off(f, p));
            }
        }
    }
    assert!(
        worst < 1e-12 && minted.len() >= 3 * corners,
        "{what}: a minted edge strays {worst} from a face it joins ({} read)",
        minted.len()
    );
    out
}

/// The flush and inset corners at a right angle, each pair edge, at
/// scalar `T`.
fn right_angle_rows<T: Scalar>(scalar: &str) {
    for (kind, (body, [odd, y, z]), (l, sign)) in [
        ("flush", flush::<T>(), (2.0, 1.0)),
        ("inset", inset::<T>(), (1.0, -1.0)),
    ] {
        let truth = sign * (S * S / 2.0 * (l - 1.0) + 2.0 / 3.0 * S.powi(3));
        assert!(
            (truth - sign * flush_form(PI / 2.0, S, (l, 1.0))).abs() < 1e-15,
            "the general form at a right angle"
        );
        for (pair, which) in [(y, "first"), (z, "second")] {
            let what = format!("{kind}, odd edge and {which} pair edge, {scalar}");
            builds(&body, &[odd, pair], (S, truth, 1), &what);
        }
    }
}

#[test]
fn the_odd_edge_with_one_pair_edge_chamfers_at_its_closed_form() {
    right_angle_rows::<f64>("f64");
    right_angle_rows::<Interval>("Interval");
}

/// **What the inset corner at 120° does at `Interval` at ε 1e-12**: its
/// second pair edge's trimline chord escalates its endpoint
/// certificate, the enclosure of a coordinate read through the
/// fixture's trigonometry being a hair wider than the band's zero. The
/// same edge's pair mitre and all-three corner escalate there too, so
/// it is the fixture's, not the rule's.
fn assert_endpoint_escalation<T: Scalar>(what: &str, got: Result<Blended<T>, BlendError>) {
    let text = format!("{:?}", got.as_ref().map(|_| ()));
    assert!(
        text.contains("trimline mef")
            && text.contains("Escalated { check: EndpointEnd")
            && text.contains("carrier_endpoint_end"),
        "{what}: at Interval at ε 1e-12 the trimline's endpoint escalates, got {text}"
    );
}

/// The flush and inset corners at plan angles 60° and 120°, each pair
/// edge, at scalar `T`.
fn plan_angle_rows<T: Scalar>(scalar: &str) {
    for phi in [PI / 3.0, 2.0 * PI / 3.0] {
        let deg = phi.to_degrees();
        for (kind, (body, [odd, y, z]), (l, sign)) in [
            ("flush", flush_at::<T>(phi), (2.0, 1.0)),
            ("inset", inset_at::<T>(phi), (1.0, -1.0)),
        ] {
            let truth = sign * flush_form(phi, S, (l, 1.0));
            for (pair, which) in [(y, "first"), (z, "second")] {
                let what = format!("{kind} at {deg:.0}°, {which} pair edge, {scalar}");
                if tightest_interval::<T>() && kind == "inset" && which == "second" && deg > 90.0 {
                    assert_endpoint_escalation(&what, chamfer(&body, &[odd, pair], S));
                    continue;
                }
                builds(&body, &[odd, pair], (S, truth, 1), &what);
            }
        }
    }
}

#[test]
fn the_odd_edge_with_one_pair_edge_chamfers_at_a_non_right_plan_angle() {
    plan_angle_rows::<f64>("f64");
    plan_angle_rows::<Interval>("Interval");
}

/// The bore-less bracket's request with the odd edge and one pair edge
/// at each of its four mixed corners: at each flush corner the root
/// along `x` (odd) with the boss's vertical there at `y = 0.5` and with
/// the plate's side edge at `y = 3.5`; at each inset corner the boss's
/// vertical (odd) with that same root.
fn bracket_request<T: Scalar>(body: &Body<T>) -> Vec<EdgeKey> {
    vec![
        edge(body, (0.0, 0.5, 0.5), (0.5, 0.5, 0.5)),
        edge(body, (0.0, 3.5, 0.5), (0.5, 3.5, 0.5)),
        edge(body, (0.0, 0.5, 0.5), (0.0, 0.5, 2.0)),
        edge(body, (0.0, 3.5, 0.5), (0.0, 4.0, 0.5)),
        edge(body, (0.5, 0.5, 0.5), (0.5, 0.5, 2.0)),
        edge(body, (0.5, 3.5, 0.5), (0.5, 3.5, 2.0)),
    ]
}

/// **The bore-less bracket chamfered with the odd edge and one pair
/// edge at every mixed corner**: each band runs its edge's whole length
/// where it is odd or cut off square, and stops `(4/3)·s` short (in
/// effect) where it is the pair at an odd-plus-one corner. The roots
/// (concave, 0.5 each) are odd at the flush end and pair at the inset;
/// the flush verticals and side edges (convex, 1.5 and 0.5) pair; the
/// inset verticals (convex, 1.5) odd. The `s³` terms of the flush and
/// inset corners cancel: `ΔV = (s²/2)(2·0.5 − 1.5 − 0.5 − 2·1.5) = −2s²`.
fn bracket_rows<T: Scalar>(scalar: &str) {
    let body = boreless::<T>();
    let s = 0.05;
    builds(
        &body,
        &bracket_request(&body),
        (s, -2.0 * s * s, 4),
        &format!("the bracket, odd plus one at every mixed corner, {scalar}"),
    );
}

#[test]
fn the_bracket_chamfers_with_the_odd_edge_and_one_other_at_every_mixed_corner() {
    bracket_rows::<f64>("f64");
    bracket_rows::<Interval>("Interval");
}

/// **The flush L with its `+x` pair edge `y` long**, ending square at a
/// side `x = y`: the odd edge (the reflex vertical, 2 long) and that
/// pair edge.
fn pair_of_length<T: Scalar>(y: f64) -> (Body<T>, [EdgeKey; 2]) {
    let body = prism_at(
        corners(&[
            (-1.0, -1.0),
            (y, -1.0),
            (y, 0.0),
            (0.0, 0.0),
            (0.0, 1.0),
            (-1.0, 1.0),
        ]),
        T::zero(),
        T::from_f64(2.0),
        tol(),
    );
    let o: P = (0.0, 0.0, 0.0);
    let edges = [
        edge(&body, o, (0.0, 0.0, 2.0)),
        edge(&body, o, (y, 0.0, 0.0)),
    ];
    (body, edges)
}

/// The short-edge boundaries at scalar `T`.
fn short_edge_rows<T: Scalar>(scalar: &str) {
    let run_out = |got: Result<Blended<T>, BlendError>, needle: &str, what: &str| {
        assert!(
            matches!(&got, Err(BlendError::UnsupportedRunOut { detail, .. }) if detail.contains(needle)),
            "{what}, {scalar}: refuses with {needle:?}, got {got:?}",
            got = got.as_ref().map(|_| ())
        );
    };
    // The pair edge: the near band's foot on `E` stands `2s` along it.
    let (body, edges) = pair_of_length::<T>(0.21);
    let truth = flush_form(PI / 2.0, S, (2.0, 0.21));
    builds(
        &body,
        &edges,
        (S, truth, 1),
        &format!("the pair edge 0.21 long, {scalar}"),
    );
    let (body, edges) = pair_of_length::<T>(0.19);
    run_out(
        chamfer(&body, &edges, S),
        "shorter than its two ends' feet need",
        "the pair edge 0.19 long",
    );
    // Shorter than the size, the pair strip's crossing on the shared
    // face lands past the pair edge's far end.
    let (body, edges) = pair_of_length::<T>(0.09);
    run_out(
        chamfer(&body, &edges, S),
        "its foot lands inside a face",
        "the pair edge 0.09 long",
    );
    // The odd edge: the pair strip's trimline on the shared face stands
    // `s` up it.
    let tall = |h: f64| {
        let body = l_prism::<T>(h);
        let [odd, y, _] = l_corner(&body, h);
        (body, [odd, y])
    };
    let (body, edges) = tall(0.11);
    let truth = flush_form(PI / 2.0, S, (0.11, 1.0));
    builds(
        &body,
        &edges,
        (S, truth, 1),
        &format!("the odd edge 0.11 long, {scalar}"),
    );
    let (body, edges) = tall(0.09);
    run_out(
        chamfer(&body, &edges, S),
        "its foot lands inside a face",
        "the odd edge 0.09 long",
    );
}

/// **Short edges refuse at their meters**, both sides of each boundary,
/// on the flush L at size `0.1`: a pair edge `0.21` long builds and one
/// `0.19` long refuses as its two ends' feet cross on its trimline in
/// `E` (the overrun's foot `2s` along it), one under `s` long refuses
/// as its foot lands inside a face; an odd edge `0.11` long builds and
/// one `0.09` long refuses as the pair strip's foot on the shared face
/// lands past its far end. The reach meters none of these first.
#[test]
fn a_short_odd_or_pair_edge_refuses_at_its_meter() {
    short_edge_rows::<f64>("f64");
    short_edge_rows::<Interval>("Interval");
}

/// **What the odd edge with one other still refuses**: the fillet at the
/// flush and the inset corner (its pair band would have to stop where
/// its edge runs smoothly into the cut-off arc), and the chamfer where
/// `E` leans off square to the odd edge, with either pair edge.
#[test]
fn the_odd_edge_with_one_other_refuses_the_fillet_and_an_oblique_shared_face() {
    let mixed = |got: Result<(), BlendError>, convex: usize, what: &str| {
        assert!(
            matches!(
                got,
                Err(BlendError::UnsupportedCorner {
                    corner: CornerConfig::MixedConvexity { convex: c },
                    policy: None,
                    ..
                }) if c == convex
            ),
            "{what}: refuses MixedConvexity {{ convex: {convex} }}, got {got:?}"
        );
    };
    for (kind, (body, [odd, y, z]), convex) in
        [("flush", flush::<f64>(), 2), ("inset", inset::<f64>(), 1)]
    {
        for pair in [y, z] {
            let got = fillet_edges(&at_rest(&body, tol()), &[odd, pair], S, tol());
            mixed(
                got.map(|_| ()).map_err(|r| r.error),
                convex,
                &format!("the {kind} fillet"),
            );
        }
    }
    let body = leaning();
    let o: P = (0.5, 0.0, 0.0);
    let [odd, up] = leaning_odd_and_one(&body);
    assert_eq!(at(&body, o).len(), 3, "the leaning corner's three edges");
    for pair in [up, edge(&body, o, (1.0, 1.0, 0.0))] {
        mixed(
            chamfer(&body, &[odd, pair], S).map(|_| ()),
            2,
            "the chamfer on an oblique E",
        );
    }
}
