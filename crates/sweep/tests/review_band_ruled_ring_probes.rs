//! Review probes for the ring-cycle ruled cut-off (PR 3243).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{Profile, ProfileLoop, SketchPlane, test_support::bulge_loop};
use sweep::blend::fillet_edges;
use sweep::test_support::rod_creases;
use sweep::{Extrusion, extrude};
use topo::{Body, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn volume(body: &Body<f64>) -> f64 {
    let p = mass_properties(body, tol()).expect("closed-form props");
    p.volume
}

fn square(h: f64) -> ProfileLoop<f64> {
    bulge_loop(
        [(-h, -h), (h, -h), (h, h), (-h, h)]
            .into_iter()
            .map(|(x, y)| (Point2::new(x, y), 0.0))
            .collect(),
    )
}

const BR: f64 = 0.5;
const W: f64 = 0.2; // keyhole slot half-width
const XS: f64 = 0.8; // slot end

/// A keyhole through-hole: a disc of radius BR plus the slot
/// [xs0, XS] x [-W, W]. At the two disc/slot junctions the HOLE is
/// reflex, so the cylinder-plane creases there are CONVEX, and they end
/// in the caps' rings.
fn keyhole_block() -> Body<f64> {
    let xs0 = (BR * BR - W * W).sqrt();
    let sweep = core::f64::consts::TAU - 2.0 * W.atan2(xs0);
    let ring = bulge_loop(vec![
        (Point2::new(xs0, W), (sweep / 4.0).tan()),
        (Point2::new(xs0, -W), 0.0),
        (Point2::new(XS, -W), 0.0),
        (Point2::new(XS, W), 0.0),
    ]);
    let p = Profile::new(SketchPlane::xy(), vec![square(1.0), ring])
        .validate(tol())
        .expect("the keyholed profile validates");
    extrude(&p, Extrusion::Distance(1.0), tol())
        .expect("the keyholed profile extrudes")
        .body
}

/// The area the convex band removes at one keyhole junction, per unit
/// length: V = (xs0, W); ball centre c = (cx, W + r) with |c| = BR + r;
/// F_b = (cx, W) on the plane, F_a = c·BR/(BR + r) on the hole wall.
fn keyhole_cut(r: f64) -> f64 {
    let xs0 = (BR * BR - W * W).sqrt();
    let cy = W + r;
    let cx = ((BR + r).powi(2) - cy * cy).sqrt();
    let v = (xs0, W);
    let fb = (cx, W);
    let c = (cx, cy);
    let s = BR / (BR + r);
    let fa = (cx * s, cy * s);
    let quad = [v, fb, c, fa];
    let mut twice = 0.0;
    for i in 0..4 {
        let (p, q) = (quad[i], quad[(i + 1) % 4]);
        twice += p.0 * q.1 - q.0 * p.1;
    }
    let quad_area = 0.5 * twice.abs();
    // Ball sector at c between F_b (straight down) and F_a (towards O).
    let ang_b = (-1.0f64).atan2(0.0); // -pi/2
    let ang_a = (-cy).atan2(-cx);
    let mut dth = (ang_b - ang_a).abs();
    if dth > core::f64::consts::PI {
        dth = core::f64::consts::TAU - dth;
    }
    let sector = 0.5 * r * r * dth;
    // Disc segment between V and F_a on the hole wall, inside the quad.
    let phi = (fa.1.atan2(fa.0) - v.1.atan2(v.0)).abs();
    let segment = 0.5 * BR * BR * (phi - phi.sin());
    quad_area - sector - segment
}

/// **A keyhole's CONVEX creases end in the caps' rings and carve at
/// the closed form**: the disc/slot junctions are reflex in the hole,
/// so the band REMOVES `A` per unit length at each, `ΔV = −2·A·L`.
#[test]
fn a_keyhole_fillets_its_convex_ring_creases_at_the_closed_form() {
    let body = keyhole_block();
    validate_geometric(&body, tol()).expect("the keyholed block is tier-3 valid");
    let creases = rod_creases(&body);
    assert_eq!(creases.len(), 2, "the two disc/slot junctions");
    let vol0 = volume(&body);
    for r in [0.05, 0.1] {
        let out = fillet_edges(&body, &creases, r, tol())
            .unwrap_or_else(|e| panic!("r {r}: both convex ring creases carve, got {e}"));
        validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("r {r}: tier 3, {e:?}"));
        let dv = volume(&out.body) - vol0;
        let want = -2.0 * keyhole_cut(r);
        assert!((dv - want).abs() < 1e-12, "r {r}: ΔV {dv} vs {want}");
    }
}
