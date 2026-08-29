//! CERT-2 R1 review probes (probe branch only — never for merge).
//!
//! E2E exercise of `plane_nurbs_ssi` / `cylinder_sphere_ssi` with
//! degenerate walls, recording what the refusals read like to a
//! consumer, plus reachability probes for the PR's overflow-closure
//! and sentence-producibility claims. Every probe PRINTS its result;
//! assertions are deliberately loose — the point is measurement.

use geom::{NurbsSurface, Surface};
use geom_brep::ssi::{self, SsiDomain, SsiError};
use geom_core::Tol;
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn cutting_plane() -> Surface<f64> {
    let n = Vec3::new(0.0, 0.25, 1.0);
    let n = n / n.norm();
    let u = Vec3::new(1.0, 0.0, 0.0);
    let u = (u - n * u.dot(n)) / (u - n * u.dot(n)).norm();
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.4),
        normal: n,
        u_ref: u,
    }
}

fn wall_domain() -> SsiDomain {
    SsiDomain {
        center: Point3::new(0.5, 0.0, 0.4),
        half_extent: 2.0,
        extent: 1.5,
        floor_scale: 1.0,
    }
}

fn wall_scaled(m: f64, weights: Vec<f64>) -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let mut control = Vec::with_capacity(8);
    for (x, y) in [(0.0, 0.0), (0.35, 0.18), (0.70, -0.12), (1.05, 0.04)] {
        control.push(Point3::new(x * m, y * m, 0.0));
        control.push(Point3::new(x * m, y * m, 0.8));
    }
    NurbsSurface::new(ku, kv, control, weights).unwrap()
}

fn door(tag: &str, r: &Result<geom_brep::ssi::SsiOutcome, SsiError>) {
    match r {
        Ok(o) => println!(
            "[R1 {tag}] Ok: {} branches, receipt {:?}",
            o.branches.len(),
            o.exhaustiveness
        ),
        Err(e) => println!("[R1 {tag}] Err: {e}"),
    }
}

/// Claim 4 ladder: which door answers as net magnitude climbs through
/// the claimed `+inf`-speed threshold (~1e169).
#[test]
fn r1_overflow_ladder() {
    for m in [1.0e150, 1.0e160, 1.0e165, 1.0e170, 1.0e200, 3.0e292] {
        let w = wall_scaled(m, vec![1.0; 8]);
        let r = ssi::plane_nurbs_ssi(&cutting_plane(), &w, wall_domain(), band());
        door(&format!("net={m:e}"), &r);
        if m >= 1.0e170 {
            match &r {
                Err(SsiError::UnsupportedCertificate { what })
                    if what.contains("chart speed") => {}
                other => println!("[R1 net={m:e}] NOT the speed guard: {other:?}"),
            }
        }
    }
}

/// Sentence producibility, R^4 arm second disjunct: can "homogeneous
/// arithmetic that does not stay finite over the net" be produced from
/// the public door, or does the speed guard always answer first?
#[test]
fn r1_huge_weights() {
    // Equal huge weights, order-1 points.
    let w = wall_scaled(1.0, vec![1.0e300; 8]);
    door(
        "w=1e300 equal",
        &ssi::plane_nurbs_ssi(&cutting_plane(), &w, wall_domain(), band()),
    );
    // Mixed: huge and unit weights.
    let mut ws = vec![1.0; 8];
    ws[2] = 1.0e300;
    ws[3] = 1.0e300;
    let w = wall_scaled(1.0, ws);
    door(
        "w mixed 1e300/1",
        &ssi::plane_nurbs_ssi(&cutting_plane(), &w, wall_domain(), band()),
    );
    // Huge weights at max: 1e308.
    let w = wall_scaled(1.0, vec![1.0e308; 8]);
    door(
        "w=1e308 equal",
        &ssi::plane_nurbs_ssi(&cutting_plane(), &w, wall_domain(), band()),
    );
}

/// E2E: a degenerate wall whose control net collapses to one point —
/// the zero-radius-shaped degenerate of the plane×NURBS lane.
#[test]
fn r1_collapsed_net() {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = vec![Point3::new(0.5, 0.0, 0.4); 8];
    let w = NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap();
    door(
        "collapsed net",
        &ssi::plane_nurbs_ssi(&cutting_plane(), &w, wall_domain(), band()),
    );
}

/// E2E: a zero-radius sphere against a cylinder — the degenerate
/// instance the reworded R^3 sentence names.
#[test]
fn r1_zero_radius_sphere() {
    let s = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 0.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let c = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 0.6,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let d = SsiDomain {
        center: Point3::new(0.0, 0.0, 0.0),
        half_extent: 2.0,
        extent: 1.5,
        floor_scale: 1.0,
    };
    door("zero-radius sphere", &ssi::cylinder_sphere_ssi(&c, &s, d, band()));
}

/// E2E: a net collapsed to a point OFF the plane, and a net collapsed
/// to a vertical line ON the plane — chasing the "chart speed is zero"
/// arm's reachability.
#[test]
fn r1_collapsed_variants() {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    // Off the plane.
    let control = vec![Point3::new(0.5, 0.0, 10.0); 8];
    let w = NurbsSurface::new(ku.clone(), kv.clone(), control, vec![1.0; 8]).unwrap();
    door(
        "point off plane",
        &ssi::plane_nurbs_ssi(&cutting_plane(), &w, wall_domain(), band()),
    );
    // Collapsed in u only (a vertical line crossing the plane).
    let mut control = Vec::with_capacity(8);
    for _ in 0..4 {
        control.push(Point3::new(0.5, 0.0, 0.0));
        control.push(Point3::new(0.5, 0.0, 0.8));
    }
    let w = NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap();
    door(
        "line wall",
        &ssi::plane_nurbs_ssi(&cutting_plane(), &w, wall_domain(), band()),
    );
}

/// Issue 1218 verification: a plane with a non-finite origin against a
/// healthy wall — which operand does the refusal blame?
#[test]
fn r1_plane_inf_origin() {
    let n = Vec3::new(0.0, 0.25, 1.0);
    let n = n / n.norm();
    let u = Vec3::new(1.0, 0.0, 0.0);
    let u = (u - n * u.dot(n)) / (u - n * u.dot(n)).norm();
    let plane = Surface::Plane {
        origin: Point3::new(f64::INFINITY, 0.0, 0.4),
        normal: n,
        u_ref: u,
    };
    let w = wall_scaled(1.0, vec![1.0; 8]);
    door(
        "plane inf origin",
        &ssi::plane_nurbs_ssi(&plane, &w, wall_domain(), band()),
    );
}

/// E2E: the subnormal-weight wall (the PR's D286 fixture shape) — read
/// the refusal as a consumer would.
#[test]
fn r1_subnormal_weight_reading() {
    let tiny = f64::from_bits(1);
    let w = wall_scaled(1.0, vec![tiny; 8]);
    door(
        "subnormal weight",
        &ssi::plane_nurbs_ssi(&cutting_plane(), &w, wall_domain(), band()),
    );
}
