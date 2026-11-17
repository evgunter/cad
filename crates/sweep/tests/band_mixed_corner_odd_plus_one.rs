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
use sweep::blend::{BlendError, CornerConfig, FILLET3_MIXED_CORNER_RECOURSE};
use sweep::chamfer::chamfer_edges;
use sweep::test_support::{assert_naming_totality, at_rest, corners, prism_at};
use topo::boolean::solid_contain::{SolidContainment, point_in_solid};
use topo::{Body, EdgeKey, validate_geometric};

use crate::band_closed_mixed_chain_shapes::tightest_interval;
use crate::band_mixed_corner::{
    P, Scalar, at, boreless, bowed_corner, disc_corner, edge, flush, flush_at, inset, inset_at,
    l_corner, l_prism, leaning, leaning_odd_and_one, tol, volume,
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
/// and one turn foot at each of `corners` corners, `arcs` end curves
/// and two trimlines per requested edge, and every edge the corners
/// minted (the end curves, the mitres, the trimlines) on both its faces
/// within `1e-12`. A single corner's end curves are three: the odd
/// strip's chord in `E`, and each strip's far end.
fn builds<T: Scalar>(
    body: &Body<T>,
    edges: &[EdgeKey],
    (s, truth, corners, arcs): (f64, f64, usize, usize),
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
    assert_eq!(
        (rec.arcs.len(), rec.trims.len()),
        (arcs, 2 * edges.len()),
        "{what}: the end curves, and two trimlines per requested edge"
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
        worst < 1e-12,
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
        for (pair, which) in [(y, "first"), (z, "second")] {
            let what = format!("{kind}, odd edge and {which} pair edge, {scalar}");
            builds(&body, &[odd, pair], (S, truth, 1, 3), &what);
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
/// same edge's pair mitre escalates at this same site, so it is the
/// fixture's, not the rule's; the all-three corner escalates there too,
/// at another site (`mixed host arc mef`, `carrier_matches_mapped_source`).
/// Filed as
/// `work/band/an-odd-plus-one-chamfer-at-an-inset-120-degree-corner-escalates-at-the-tightest-interval.md`.
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
                builds(&body, &[odd, pair], (S, truth, 1, 3), &what);
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
pub(crate) fn bracket_request<T: Scalar>(body: &Body<T>) -> Vec<EdgeKey> {
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
        (s, -2.0 * s * s, 4, 8),
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

/// The short-edge boundaries at scalar `T`, each read `d = 100ε` either
/// side (outside the predicates' bands at every ε) and 5% either side.
fn short_edge_rows<T: Scalar>(scalar: &str) {
    let run_out = |got: Result<Blended<T>, BlendError>, needle: &str, what: &str| {
        assert!(
            matches!(&got, Err(BlendError::UnsupportedRunOut { detail, .. }) if detail.contains(needle)),
            "{what}, {scalar}: refuses with {needle:?}, got {got:?}",
            got = got.as_ref().map(|_| ())
        );
    };
    const CROSS: &str = "shorter than its two ends' feet need";
    const INSIDE: &str = "its foot lands inside a face";
    let d = 100.0 * tol().eps();
    // The pair edge: the near band's foot on `E` stands `2s` along it,
    // and under `s` long its crossing on the shared face lands past its
    // far end.
    for y in [2.0 * S + d, 1.05 * 2.0 * S] {
        let (body, edges) = pair_of_length::<T>(y);
        let truth = flush_form(PI / 2.0, S, (2.0, y));
        builds(
            &body,
            &edges,
            (S, truth, 1, 3),
            &format!("the pair edge {y} long, {scalar}"),
        );
    }
    for (y, needle) in [
        (2.0 * S - d, CROSS),
        (0.95 * 2.0 * S, CROSS),
        (S + d, CROSS),
        (S - d, INSIDE),
        (0.9 * S, INSIDE),
    ] {
        let (body, edges) = pair_of_length::<T>(y);
        run_out(
            chamfer(&body, &edges, S),
            needle,
            &format!("the pair edge {y} long"),
        );
    }
    // The odd edge: the pair strip's trimline on the shared face stands
    // `s` up it.
    let tall = |h: f64| {
        let body = l_prism::<T>(h);
        let [odd, y, _] = l_corner(&body, h);
        (body, [odd, y])
    };
    for h in [S + d, 1.05 * S] {
        let (body, edges) = tall(h);
        let truth = flush_form(PI / 2.0, S, (h, 1.0));
        builds(
            &body,
            &edges,
            (S, truth, 1, 3),
            &format!("the odd edge {h} long, {scalar}"),
        );
    }
    for h in [S - d, 0.95 * S] {
        let (body, edges) = tall(h);
        run_out(
            chamfer(&body, &edges, S),
            INSIDE,
            &format!("the odd edge {h} long"),
        );
    }
}

/// **Short edges refuse at their meters**, both sides of each boundary,
/// `100ε` and 5% off it, on the flush L at size `s = 0.1`: a pair edge
/// just over `2s` long builds and one just under refuses as its two
/// ends' feet cross on its trimline in `E` (the overrun's foot `2s`
/// along it), as does one just over `s`; one just under `s` refuses as
/// its foot lands inside a face. An odd edge just over `s` long builds
/// and one just under refuses as the pair strip's foot on the shared
/// face lands past its far end. The reach meters none of these first.
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

/// **A curved link at the corner refuses typed**: at a mixed corner
/// where one face is curved, two of the three edges bound it, so any two
/// of them name one of those. A pair naming the third edge, whose
/// supports the chamfer's arm does not cover, refuses
/// `ChamferArmUnsupported` before the corner is read; the curved link
/// (a frustum's arc, which the chamfer admits inside a chain over other
/// supports) with the straight one refuses `ChainNotG1`, the two meeting
/// at an angle. On the quarter-disc boss the curved link is the odd
/// edge; on the bowed one a pair edge.
#[test]
fn a_curved_link_at_the_corner_refuses_the_chamfer_arm() {
    for (what, body, o) in [
        ("the curved odd link", disc_corner(), (0.0, -1.0, 0.0)),
        ("the curved pair link", bowed_corner(), (0.0, 0.0, 0.0)),
    ] {
        let es = at(&body, o);
        assert_eq!(es.len(), 3, "{what}: the corner's three edges");
        for (i, j) in [(0, 1), (0, 2), (1, 2)] {
            let got = chamfer(&body, &[es[i], es[j]], S);
            let typed = if j == 2 {
                matches!(got, Err(BlendError::ChamferArmUnsupported { edge, .. }) if edge == es[2])
            } else {
                matches!(got, Err(BlendError::ChainNotG1 { .. }))
            };
            assert!(
                typed,
                "{what}, edges {i} and {j}: refuses typed, got {got:?}",
                got = got.as_ref().map(|_| ())
            );
        }
    }
}

/// **The mixed corner's recourse builds where it says**, on the flush
/// and inset corners at 60°, 90° and 120° (`E` square to the odd edge,
/// the faces flat) and on the leaning one (`E` oblique), both verbs: the
/// odd edge with one other refuses the fillet `MixedConvexity` with the
/// recourse, and then the odd edge alone builds everywhere; the two that
/// round alike build where `E` is square to the odd edge and refuse on
/// the oblique one; all three, and the chamfer of the odd edge with one
/// other, build where it is square. A pair edge alone, which the text
/// does not name, refuses everywhere: its foot lands inside the odd
/// edge's face.
#[test]
fn the_mixed_corner_recourse_builds_where_it_says() {
    let mut rows: Vec<(String, Body<f64>, [EdgeKey; 3], bool)> = Vec::new();
    for deg in [60.0f64, 90.0, 120.0] {
        let phi = deg.to_radians();
        let (body, edges) = flush_at::<f64>(phi);
        rows.push((format!("flush at {deg}°"), body, edges, true));
        let (body, edges) = inset_at::<f64>(phi);
        rows.push((format!("inset at {deg}°"), body, edges, true));
    }
    let body = leaning();
    let [odd, up] = leaning_odd_and_one(&body);
    let along = edge(&body, (0.5, 0.0, 0.0), (1.0, 1.0, 0.0));
    rows.push(("leaning".into(), body, [odd, up, along], false));
    let fillet = |body: &Body<f64>, edges: &[EdgeKey]| {
        fillet_edges(&at_rest(body, tol()), edges, S, tol()).map_err(|r| r.error)
    };
    for (what, body, [odd, y, z], square) in &rows {
        let (odd, y, z) = (*odd, *y, *z);
        let refused = fillet(body, &[odd, y]).map(|_| ()).unwrap_err();
        assert!(
            matches!(
                refused,
                BlendError::UnsupportedCorner {
                    corner: CornerConfig::MixedConvexity { .. },
                    ..
                }
            ) && refused
                .render(geom_brep::recourse::Reading::Build)
                .contains(FILLET3_MIXED_CORNER_RECOURSE),
            "{what}: the fillet of the odd edge with one other ships the recourse, got {refused:?}"
        );
        type Run = fn(&Body<f64>, &[EdgeKey]) -> Result<(), BlendError>;
        let verbs: [(&str, Run); 2] = [
            ("chamfer", |b, e| chamfer(b, e, S).map(|_| ())),
            ("fillet", |b, e| {
                fillet_edges(&at_rest(b, tol()), e, S, tol())
                    .map(|_| ())
                    .map_err(|r| r.error)
            }),
        ];
        for (verb, run) in verbs {
            run(body, &[odd])
                .unwrap_or_else(|e| panic!("{what} ({verb}): the odd edge alone, {e:?}"));
            for pair in [y, z] {
                assert!(
                    run(body, &[pair]).is_err(),
                    "{what} ({verb}): a pair edge alone refuses"
                );
            }
            let alike = run(body, &[y, z]);
            assert_eq!(
                alike.is_ok(),
                *square,
                "{what} ({verb}): the two that round alike build where E is square, got {alike:?}"
            );
            if *square {
                run(body, &[odd, y, z])
                    .unwrap_or_else(|e| panic!("{what} ({verb}): all three, {e:?}"));
            }
        }
        if *square {
            for pair in [y, z] {
                chamfer(body, &[odd, pair], S).unwrap_or_else(|e| {
                    panic!("{what}: the bevel of the odd edge with one other, {e:?}")
                });
            }
        }
    }
}

/// Whether `p` lies in `body`, or `None` on its boundary.
fn inside(body: &Body<f64>, p: P) -> Option<bool> {
    let band = geom_core::Band::linear(tol()).expect("the witness band");
    match point_in_solid(body, geom_core::Point3::new(p.0, p.1, p.2), band, tol()) {
        Ok(SolidContainment::In) => Some(true),
        Ok(SolidContainment::Out) => Some(false),
        _ => None,
    }
}

/// **The odd edge with one pair edge moves the material its strips
/// bound, and no other**: a point-membership oracle on the flush corner
/// at 60°, 90° and 120°, each pair edge. In plan, with the wedge's sides
/// `d₀ = +x` and `d₁` at `φ`, bisector `b`: the odd strip fills the
/// notch where `p·b < s·cos(φ/2)`, up the odd edge; the pair strip
/// along `d_k` cuts away `q + z < s` (`q` the distance from the pair
/// edge into its side face's material), past the odd strip's plane
/// (`p·b > s·cos(φ/2)`) and short of the pair edge's far end. A point
/// is in the build where it was in the body, flipped if either holds.
/// Sampled at 250 seeded points about the corner; points the
/// containment test reads on a boundary are skipped, and they are few.
#[test]
fn the_odd_plus_one_chamfer_moves_the_material_its_strips_bound() {
    let flips = |p: P, phi: f64, k: usize| {
        let (x, y, z) = p;
        let d = [(1.0, 0.0), (phi.cos(), phi.sin())];
        let b = ((phi / 2.0).cos(), (phi / 2.0).sin());
        let (pb, c) = (x * b.0 + y * b.1, S * (phi / 2.0).cos());
        let det = phi.sin();
        let (u, v) = (
            (x * d[1].1 - y * d[1].0) / det,
            (d[0].0 * y - d[0].1 * x) / det,
        );
        if u > 0.0 && v > 0.0 && pb < c && z > 0.0 && z < 2.0 {
            return true;
        }
        let (dk, other) = (d[k], d[1 - k]);
        let od = other.0 * dk.0 + other.1 * dk.1;
        let w0 = (other.0 - od * dk.0, other.1 - od * dk.1);
        let w = (-w0.0 / w0.0.hypot(w0.1), -w0.1 / w0.0.hypot(w0.1));
        let q = x * w.0 + y * w.1;
        let along = x * dk.0 + y * dk.1;
        q > 0.0 && q < S && z > 0.0 && z < S - q && pb > c && along < 1.0
    };
    let mut seed = 0x5eed_7038_u64;
    let mut next = move || {
        seed = seed
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        #[allow(clippy::cast_precision_loss)]
        let unit = (seed >> 11) as f64 / (1u64 << 53) as f64;
        unit
    };
    for deg in [60.0f64, 90.0, 120.0] {
        let phi = deg.to_radians();
        let (body, [odd, y, z]) = flush_at::<f64>(phi);
        for (k, pair) in [y, z].into_iter().enumerate() {
            let out = chamfer(&body, &[odd, pair], S)
                .unwrap_or_else(|e| panic!("flush at {deg}°, pair {k}: builds, got {e:?}"));
            let (mut read, mut wrong) = (0, Vec::new());
            for _ in 0..250 {
                let p = (
                    -0.6 + 1.2 * next(),
                    -0.6 + 1.2 * next(),
                    -0.05 + 0.35 * next(),
                );
                let (Some(was), Some(is)) = (inside(&body, p), inside(&out.body, p)) else {
                    continue;
                };
                read += 1;
                if is != (was ^ flips(p, phi, k)) {
                    wrong.push(p);
                }
            }
            assert!(
                read > 240 && wrong.is_empty(),
                "flush at {deg}°, pair {k}: {read} points read, misplaced at {wrong:?}"
            );
        }
    }
}
