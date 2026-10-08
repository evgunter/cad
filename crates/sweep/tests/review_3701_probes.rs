//! Review probes for PR 3701 (joined open bands).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::{FRAC_PI_2, PI};
use sweep::ExtrudeSide;

use geom_core::{Point2, Point3, Tol};
use profile::{Open, Profile, ProfileLoop, SketchPlane, Start};
use sweep::{Extrusion, extrude};
use topo::{Body, EdgeKey};

use crate::common::stations::{construction_state, cut_stations, joined};

/// `[0,w]×[0,2]` prism of height 2, bottom side split at `bottom`,
/// top side split at `top` (x positions, descending order on the top
/// as walked), merged. The sweep carries each side as one rim edge per
/// cap, so the splits are cut into the rims by hand
/// (`common::stations`).
fn prism(w: f64, bottom: &[f64], top: &[f64], t: Tol) -> Body<f64> {
    let xs: Vec<f64> = bottom.iter().copied().chain([w]).collect();
    let mut b = Open
        .at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(xs[0], t)
        .unwrap();
    for &x in &xs[1..] {
        b = b.continue_to(Point2::new(x, 0.0), t).unwrap();
    }
    let txs: Vec<f64> = top.iter().copied().chain([0.0]).collect();
    let mut b = b
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(w - txs[0], t)
        .unwrap();
    for &x in &txs[1..] {
        b = b.continue_to(Point2::new(x, 2.0), t).unwrap();
    }
    let lp: ProfileLoop<f64> = b.line_to(Start, t).unwrap().into();
    let v = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(t)
        .unwrap();
    let ex = extrude(
        &v,
        Extrusion::Distance {
            depth: 2.0,
            side: ExtrudeSide::Along,
        },
        t,
    )
    .unwrap();
    let mut body = ex.body;
    for wall in &ex.walls[0] {
        let (y, xs) = match wall.segments[0] {
            0 => (0.0, bottom),
            s if s == bottom.len() + 2 => (2.0, top),
            _ => continue,
        };
        for (rim, z) in [(wall.bottom_rim, 0.0), (wall.top_rim, 2.0)] {
            let at: Vec<_> = xs.iter().map(|&x| Point3::new(x, y, z)).collect();
            body = cut_stations(body, rim, &at, t);
        }
    }
    topo::test_support::merge_unjoined(&mut body, t).unwrap();
    body
}

fn rounded(a: f64, b: f64, c: f64, r: f64) -> f64 {
    let (a, b, c) = (a - 2.0 * r, b - 2.0 * r, c - 2.0 * r);
    a * b * c
        + 2.0 * r * (a * b + b * c + c * a)
        + PI * r * r * (a + b + c)
        + (4.0 / 3.0) * PI * r.powi(3)
}
/// An equal-distance chamfer of every edge of an `a×b×c` box.
fn chamfered(a: f64, b: f64, c: f64, d: f64) -> f64 {
    a * b * c - 2.0 * d * d * (a + b + c) + 8.0 * (2.0 / 3.0) * d.powi(3)
}

fn vol(b: &Body<f64>, t: Tol) -> f64 {
    topo::mass_properties(b, t).unwrap().volume
}

/// The hand-split prism is construction state — the at-rest gate
/// refuses its joints (tier 3's check 11), so no blend door takes it —
/// and joined, it is the plain box, which builds both verbs: tier 3,
/// closed-form volume, one band per box edge and no joined band, naming
/// totality. `joints` is how many joints the gate names.
fn builds(label: &str, w: f64, bottom: &[f64], top: &[f64], r: f64, joints: usize) {
    let t = Tol::witness();
    let body = prism(w, bottom, top, t);
    assert_eq!(
        construction_state(&body, t).len(),
        joints,
        "{label}: the joints"
    );
    let (body, _) = joined(body, t);
    let body = sweep::test_support::finished("body", body, t);
    let req: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
    let f = sweep::fillet::fillet_edges(&body, &req, r, t)
        .unwrap_or_else(|e| panic!("{label}: fillet: {e}"));
    assert_eq!(
        topo::validate_geometric(&f.body, t),
        Ok(()),
        "{label}: fillet tier 3"
    );
    assert_eq!(f.body.faces().count(), 26, "{label}: one band per box edge");
    let rec = f.naming.as_ref().unwrap();
    assert!(rec.joined_blends.is_empty(), "{label}: no joined band");
    let want = rounded(w, 2.0, 2.0, r);
    assert!(
        (vol(&f.body, t) - want).abs() <= 1e-12 * want,
        "{label}: fillet volume"
    );
    sweep::test_support::assert_naming_totality(&body, &f, &req, label);
    let c = sweep::chamfer::chamfer_edges(&body, &req, r, t)
        .unwrap_or_else(|e| panic!("{label}: chamfer: {e}"));
    assert_eq!(
        topo::validate_geometric(&c.body, t),
        Ok(()),
        "{label}: chamfer tier 3"
    );
    let want = chamfered(w, 2.0, 2.0, r);
    assert!(
        (vol(&c.body, t) - want).abs() <= 1e-12 * want,
        "{label}: chamfer volume"
    );
}

/// Five-link chains, mixed link lengths, joints near corners on both
/// rims, and a mid joint at r = 0.49: each joint is construction state,
/// and each prism, joined, is the plain box.
#[test]
fn joined_bands_build_across_link_counts_and_by_corners() {
    builds("3 links", 2.0, &[0.7, 1.3], &[], 0.25, 4);
    builds("5 links", 2.0, &[0.4, 0.8, 1.2, 1.6], &[], 0.1, 8);
    builds(
        "5 + 2 links, mixed, w = 7",
        7.0,
        &[1.3, 2.9, 4.4, 6.1],
        &[3.3],
        0.4,
        10,
    );
    builds(
        "joints 0.501 from a corner, on both rims",
        2.0,
        &[0.501],
        &[1.499],
        0.25,
        4,
    );
    builds("r = 0.49 at a mid joint", 2.0, &[1.0], &[], 0.49, 2);
}

/// **A joined run blends as one edge.** A short interior link and a mid
/// joint at the plain box's radii: joined, each run is one edge, so the
/// box builds at every radius it does unjoined.
#[test]
fn the_clearance_screen_lets_joined_chains_the_band_can_carve_build() {
    builds("short interior link", 2.0, &[0.9, 0.95, 1.0], &[], 0.25, 6);
    builds("mid joint, r = 0.51", 2.0, &[1.0], &[], 0.51, 2);
    builds("mid joint, r = 0.9", 2.0, &[1.0], &[], 0.9, 2);
}

/// **A joint inside a corner's reach never reaches the blend.** At
/// r = 0.25 the corner's foot sits 0.25 along the rim. A joint short of
/// that foot, at it, or beyond it is construction state alike — the
/// at-rest gate refuses it (tier 3's check 11) before any clearance is
/// read — and joined, each is the plain box, which builds.
#[test]
fn a_joint_inside_a_corners_setback_refuses() {
    let t = Tol::witness();
    let band = geom_core::Band::linear(t).expect("the run's linear band");
    let step = 2.0 * band.escalate();
    let r = 0.25;
    for (what, x) in [
        ("short of the foot", r - step),
        ("at the foot", r),
        ("beyond the foot", r + step),
    ] {
        builds(what, 2.0, &[x], &[], r, 2);
    }
}
