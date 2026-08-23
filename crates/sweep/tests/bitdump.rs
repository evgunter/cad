//! **Reviewer bit-identity dump (PR #932, claim C1).** Builds the
//! merge-base fixtures the PR claims are bit-identical — the die
//! (open chains + corners), the pipped die's pip rim (the N-link
//! closed LADDER path), and the chamfered cube — and writes a
//! bit-faithful text dump of every output body to
//! `$BITDUMP_DIR/<name>.txt`. Run at the merge base and at the head,
//! then `diff` the files: any moved bit shows as a text change
//! (shortest-roundtrip f64 formatting is injective on non-NaN
//! values, and `-0.0` prints signed).
//!
//! Not part of the PR under review; lives only on the reviewer's
//! probe branch.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    dead_code,
    missing_docs
)]

use std::fmt::Write as _;

use geom::Surface;
use geom_core::{Affine3, Band, Point2, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::chamfer::chamfer_edges;
use sweep::fillet::build::fillet_edges;
use sweep::test_support::cube;
use sweep::{Revolution, RevolveAxis, revolve};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanDeclarations, EdgeKey};

fn band() -> Band {
    let tol = Tol::witness().get();
    Band::new(tol.eps, tol.k * tol.eps).unwrap()
}

fn all_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    body.edges().map(|(k, _)| k).collect()
}

/// Dump one body, bit for bit, in key iteration order (identical
/// operation sequences produce identical key orders).
fn dump(body: &Body<f64>) -> String {
    let mut s = String::new();
    let _ = writeln!(
        s,
        "census V={} E={} F={}",
        body.vertices().count(),
        body.edges().count(),
        body.faces().count()
    );
    for (k, _) in body.vertices() {
        let p = body
            .get_vertex(k)
            .and_then(|v| body.get_point(v.point))
            .unwrap();
        let _ = writeln!(s, "V {k:?} ({:?}, {:?}, {:?})", p.x, p.y, p.z);
    }
    for (k, e) in body.edges() {
        let _ = write!(s, "E {k:?} he+={:?} he-={:?}", e.he_plus, e.he_minus);
        match body.get_curve_geom(e.curve).and_then(|g| g.certified()) {
            Some(c) => {
                let (t0, t1) = c.params();
                let _ = writeln!(
                    s,
                    " carrier={:?} params=({t0:?}, {t1:?}) desc={:?}",
                    c.carrier(),
                    c.description()
                );
            }
            None => {
                let _ = writeln!(s, " UNCERTIFIED");
            }
        }
    }
    for (k, _) in body.faces() {
        let fd = body.get_face(k).unwrap();
        let surf = body.get_surface(fd.surface).unwrap();
        let _ = writeln!(
            s,
            "F {k:?} sense={:?} rings={} surface={surf:?}",
            fd.sense,
            fd.rings.len()
        );
    }
    let props = topo::mass_properties(body, Tol::witness()).unwrap();
    let _ = writeln!(
        s,
        "props volume={:?} pad={:?} area={:?} apad={:?}",
        props.volume, props.volume_pad, props.surface_area, props.area_pad
    );
    s
}

fn save(name: &str, text: &str) {
    let dir = std::env::var("BITDUMP_DIR").expect("set BITDUMP_DIR");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(format!("{dir}/{name}.txt"), text).unwrap();
}

// --- fixtures, verbatim from the merge-base suites -----------------

fn ball_at(r: f64, c: Vec3<f64>) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(0.0, -r), 1.0),
        ProfileVertex::new(Point2::new(0.0, r), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let ball = revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body;
    topo::transform_rigid(&ball, &Affine3::translation(c), Tol::witness()).unwrap()
}

fn ball_poled_z(r: f64, c: Vec3<f64>) -> Body<f64> {
    let ball = ball_at(r, Vec3::new(0.0, 0.0, 0.0));
    let placed = topo::transform_rigid(
        &ball,
        &Affine3::rotation_about_axis(
            geom_core::Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            core::f64::consts::FRAC_PI_2,
        ),
        Tol::witness(),
    )
    .unwrap();
    topo::transform_rigid(&placed, &Affine3::translation(c), Tol::witness()).unwrap()
}

fn rim_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    let kind_of = |f: topo::FaceKey| -> Option<u8> {
        match body.get_surface(body.get_face(f)?.surface)? {
            Surface::Plane { .. } => Some(0),
            Surface::Sphere { .. } => Some(1),
            _ => Some(2),
        }
    };
    let face_of = |he: topo::HalfEdgeKey| -> Option<topo::FaceKey> {
        Some(body.get_loop(body.get_half_edge(he)?.parent_loop)?.face)
    };
    let mut out = Vec::new();
    for (k, e) in body.edges() {
        let (Some(fa), Some(fb)) = (face_of(e.he_plus), face_of(e.he_minus)) else {
            continue;
        };
        if matches!(
            (kind_of(fa), kind_of(fb)),
            (Some(0), Some(1)) | (Some(1), Some(0))
        ) {
            out.push(k);
        }
    }
    out
}

fn pipped_die() -> (Body<f64>, Vec<EdgeKey>, Vec<EdgeKey>) {
    const DIE_L: f64 = 1.0;
    const PIP_R: f64 = 0.09;
    const PIP_H: f64 = 0.05;
    let cube0 = cube(DIE_L, Tol::witness());
    let box_keys: Vec<_> = cube0.edges().map(|(k, _)| k).collect();
    let pip = ball_poled_z(PIP_R, Vec3::new(0.5, 0.5, DIE_L + (PIP_R - PIP_H)));
    let pipped = boolean_op_with(
        BooleanOp::Subtract,
        &cube0,
        &pip,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .unwrap()
    .body()
    .expect("a body")
    .body
    .clone();
    let box_edges: Vec<_> = box_keys
        .into_iter()
        .filter(|k| pipped.get_edge(*k).is_some())
        .collect();
    let rims = rim_edges(&pipped);
    (pipped, box_edges, rims)
}

// --- the dumps -----------------------------------------------------

/// The die: twelve open chains + eight corners, fillet r = 0.15.
#[test]
fn bitdump_die() {
    let body = cube(1.0, Tol::witness());
    let out = fillet_edges(&body, &all_edges(&body), 0.15, band(), Tol::witness()).unwrap();
    let mut text = dump(&out.body);
    let _ = writeln!(
        text,
        "blend={:?} corner={:?} band={:?}",
        out.blend_faces, out.corner_faces, out.band_faces
    );
    save("die", &text);
}

/// The pip rim: the two-arc closed LADDER chain plus the twelve box
/// edges, in one call (the F-e form), fillet r = 0.05.
#[test]
fn bitdump_pip_rims() {
    let (pipped, box_edges, rims) = pipped_die();
    assert_eq!(rims.len(), 2, "the pip rim is two arcs");
    let mut all = box_edges;
    all.extend(rims);
    let out = fillet_edges(&pipped, &all, 0.05, band(), Tol::witness()).unwrap();
    let mut text = dump(&out.body);
    let _ = writeln!(
        text,
        "blend={:?} corner={:?} band={:?}",
        out.blend_faces, out.corner_faces, out.band_faces
    );
    save("pip_rims", &text);
}

/// The chamfered cube: twelve strips + eight corner planes, d = 0.1.
#[test]
fn bitdump_chamfered_cube() {
    let body = cube(1.0, Tol::witness());
    let out = chamfer_edges(&body, &all_edges(&body), 0.1, band(), Tol::witness()).unwrap();
    let mut text = dump(&out.body);
    let _ = writeln!(
        text,
        "blend={:?} corner={:?} band={:?}",
        out.blend_faces, out.corner_faces, out.band_faces
    );
    save("chamfered_cube", &text);
}
