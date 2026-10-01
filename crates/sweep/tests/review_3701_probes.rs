//! Review probes for PR 3701 (joined open bands).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::{FRAC_PI_2, PI};

use geom_core::{Point2, Tol};
use profile::{Open, Profile, ProfileLoop, SketchPlane, Start};
use sweep::{Extrusion, extrude};
use topo::{Body, EdgeKey, FaceKey};

/// `[0,w]×[0,2]` prism of height 2, bottom side split at `bottom`,
/// top side split at `top` (x positions, descending order on the top
/// as walked), merged.
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
    let mut body = extrude(&v, Extrusion::Distance(2.0), t).unwrap().body;
    body.merge_coplanar_faces(t).unwrap();
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

fn band_radial_err(body: &Body<f64>, f: FaceKey) -> f64 {
    let s = body.get_face(f).unwrap().surface;
    let Some(geom::Surface::Cylinder {
        origin,
        axis,
        radius,
        ..
    }) = body.get_surface(s)
    else {
        return f64::NAN;
    };
    let mut worst: f64 = 0.0;
    for (v, vd) in body.vertices() {
        if !body.faces_of_vertex(v).is_some_and(|fs| fs.contains(&f)) {
            continue;
        }
        let p = *body.get_point(vd.point).unwrap();
        let d = p - *origin;
        let along = d.dot(*axis);
        let rad = (d - *axis * along).norm();
        worst = worst.max((rad - radius).abs());
    }
    worst
}

/// Builds both verbs, tier 3, closed-form volume, feet on the band
/// cylinder, naming totality.
fn builds(label: &str, w: f64, bottom: &[f64], top: &[f64], r: f64, joined: usize) {
    let t = Tol::witness();
    let body = prism(w, bottom, top, t);
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
    assert_eq!(rec.joined_blends.len(), joined, "{label}: joined bands");
    for (fk, _) in &rec.joined_blends {
        assert!(
            band_radial_err(&f.body, *fk) <= 1e-14,
            "{label}: a foot is off the band"
        );
    }
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
/// rims, and a mid joint at r = 0.49.
#[test]
fn joined_bands_build_across_link_counts_and_by_corners() {
    builds("3 links", 2.0, &[0.7, 1.3], &[], 0.25, 2);
    builds("5 links", 2.0, &[0.4, 0.8, 1.2, 1.6], &[], 0.1, 2);
    builds(
        "5 + 2 links, mixed, w = 7",
        7.0,
        &[1.3, 2.9, 4.4, 6.1],
        &[3.3],
        0.4,
        4,
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

/// **The clearance screen reads a joined run as one feature.** Two
/// links of one run are never a clearance pair and the run's
/// neighbours are the edges at its ends, so a short interior link and
/// a mid joint at the plain box's radii build: the joint caps nothing.
#[test]
fn the_clearance_screen_lets_joined_chains_the_band_can_carve_build() {
    builds("short interior link", 2.0, &[0.9, 0.95, 1.0], &[], 0.25, 2);
    builds("mid joint, r = 0.51", 2.0, &[1.0], &[], 0.51, 2);
    builds("mid joint, r = 0.9", 2.0, &[1.0], &[], 0.9, 2);
}

/// **A joint inside a corner's reach still refuses — to the boundary.**
/// At r = 0.25 the corner's foot sits 0.25 along the rim, so a joint at
/// x = 0.2499999 puts the band's foot inside the corner patch and
/// refuses on both verbs, while one at x = 0.2500001 builds: the screen
/// meters the joint's own foot, not a setback-sum stand-in for it.
#[test]
fn a_joint_inside_a_corners_setback_refuses() {
    let t = Tol::witness();
    let body = prism(2.0, &[0.2499999], &[], t);
    let req: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
    for (verb, err) in [
        (
            "fillet",
            sweep::fillet::fillet_edges(&body, &req, 0.25, t).unwrap_err(),
        ),
        (
            "chamfer",
            sweep::chamfer::chamfer_edges(&body, &req, 0.25, t).unwrap_err(),
        ),
    ] {
        assert!(
            matches!(
                err.error,
                sweep::blend::BlendError::FaceClearanceUncertified {
                    cross_chain: false,
                    ..
                }
            ),
            "{verb}: {err}"
        );
    }
    builds(
        "joint just beyond the corner's foot",
        2.0,
        &[0.2500001],
        &[],
        0.25,
        2,
    );
}
