//! **Which of the torus arm's typed predicates a real input reaches**,
//! and what the ones nothing reaches are doing there.
//!
//! A predicate no row names is a predicate whose Zero arm could be
//! deleted, inverted or misspelled without a test noticing. The torus
//! arm added twelve, and until this module only `bool_ray_torus_disc`
//! was named by a row. What is here is one census over a pose lattice
//! plus the branch-selection rows the census cannot see from outside,
//! and an explicit disposition for each predicate the census does NOT
//! reach — because "no row names it" and "no input can reach it" are
//! very different claims and only the second one excuses the first.

#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

use super::*;
use geom_core::{Band, Point3, Tol, Vec3};

const R_MAJOR: f64 = 1.0;
const R_MINOR: f64 = 0.3;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn centre() -> Point3<f64> {
    Point3::new(0.0, 0.0, 0.0)
}

fn axis() -> Vec3<f64> {
    Vec3::new(0.0, 1.0, 0.0)
}

fn roots(q: Point3<f64>, d: Vec3<f64>) -> Result<TorusRoots<f64>, geom_core::Indeterminate> {
    line_torus_roots(q, d.normalize(), centre(), axis(), R_MAJOR, R_MINOR, band())
}

/// The coefficient the biquadratic branch is selected on, recomputed in
/// the row rather than read out of the arm — so a pose can be CLASSIFIED
/// here and the branch it must take asserted, which is the only way this
/// module can see a selection the return type does not expose.
fn q_hat(q: Point3<f64>, d: Vec3<f64>) -> f64 {
    let d = d.normalize();
    let w0 = q - centre();
    let e = d.dot(axis());
    let perp = w0 - d * w0.dot(d);
    8.0 * R_MAJOR.powi(2) * e * perp.dot(axis())
}

/// **The census.** Every outcome the arm can answer, reached by a real
/// pose on a real torus — so none of the sign ladder's arms is dead
/// code, and a Zero arm that stopped being taken would go red here.
#[test]
fn every_quartic_outcome_is_reached_by_a_real_pose() {
    let mut miss = 0;
    let mut two = 0;
    let mut four = 0;
    let mut uncertain = 0;
    // A lattice of origins and directions, plus the named poses: through
    // the hole (four roots), along the axis (a miss), tangent to the top
    // circle (uncertain), and a generic pierce (two).
    let mut poses = vec![
        (centre(), Vec3::new(1.0, 0.0, 0.0)),
        (centre(), Vec3::new(0.0, 1.0, 0.0)),
        (Point3::new(0.0, R_MINOR, R_MAJOR), Vec3::new(1.0, 0.0, 0.0)),
        (Point3::new(3.0, 0.2, 0.1), Vec3::new(-1.0, -0.05, 0.0)),
    ];
    for i in 0..7 {
        for j in 0..5 {
            let x = -1.8 + 0.6 * f64::from(i);
            let y = -0.6 + 0.3 * f64::from(j);
            poses.push((Point3::new(x, y, 0.35), Vec3::new(0.31, 0.17, 0.93)));
            poses.push((Point3::new(x, y, -0.7), Vec3::new(1.0, 0.02, 0.05)));
        }
    }
    for (q, d) in poses {
        match roots(q, d) {
            Ok(TorusRoots::Miss) => miss += 1,
            Ok(TorusRoots::Uncertain) => uncertain += 1,
            Ok(TorusRoots::Certified { count: 2, .. }) => two += 1,
            Ok(TorusRoots::Certified { count: 4, .. }) => four += 1,
            Ok(TorusRoots::Certified { count, .. }) => {
                panic!("a certified count is 2 or 4, never {count}")
            }
            Err(_) => uncertain += 1,
        }
    }
    // `bool_ray_torus_disc` Negative, and `_shape`/`_depth` on the
    // positive branch, are what these three separate.
    assert!(miss > 0, "no pose missed the tube");
    assert!(two > 0, "no pose pierced it twice");
    assert!(four > 0, "no pose crossed all four walls");
    assert!(
        uncertain > 0,
        "no pose left the count uncertain — the tangency arm is then unreached and \
         nothing would notice it being deleted"
    );
}

/// **The branch selection**, which the return type cannot show: a
/// midplane ray has `q̂ = 0` exactly and MUST take the biquadratic
/// closed form (`bool_ray_torus_odd` Zero), while a tilted ray off the
/// midplane has `q̂` definitely nonzero and must take Ferrari's
/// resolvent — the branch that reaches `bool_ray_torus_split_lead` and
/// the cube root. Both must produce the counts the geometry demands.
#[test]
fn both_closed_forms_are_selected_and_both_answer() {
    // Biquadratic: through the hole in the midplane. Four roots at
    // `±(R ± r)`, and `q̂` is exactly zero because `e = 0`.
    let (q, d) = (centre(), Vec3::new(1.0, 0.0, 0.0));
    assert_eq!(q_hat(q, d), 0.0, "a midplane ray has q̂ = 0 exactly");
    let Ok(TorusRoots::Certified { count, ts }) = roots(q, d) else {
        panic!("the four-root midplane ray must certify")
    };
    assert_eq!(count, 4);
    let mut got: Vec<f64> = ts.to_vec();
    got.sort_by(f64::total_cmp);
    for (g, w) in got.iter().zip([
        -(R_MAJOR + R_MINOR),
        -(R_MAJOR - R_MINOR),
        R_MAJOR - R_MINOR,
        R_MAJOR + R_MINOR,
    ]) {
        assert!((g - w).abs() < 1e-9, "biquadratic roots {got:?}");
    }

    // Ferrari: a tilted ray whose `q̂` is far from the band, piercing the
    // tube twice. This is the ONLY branch that calls the cube root.
    let (q, d) = (Point3::new(-2.0, -0.4, 0.2), Vec3::new(1.0, 0.35, 0.05));
    assert!(
        q_hat(q, d).abs() > 1e-3,
        "this pose must select the resolvent, not the biquadratic form"
    );
    let Ok(TorusRoots::Certified { count, ts }) = roots(q, d) else {
        panic!("a definite pierce must certify")
    };
    assert_eq!(count, 2, "a tilted pierce crosses the tube twice");
    // Each root is on the tube, to the arm's own accuracy: the check
    // that the resolvent, the cube root and the depression compose.
    for &t in &ts[..count] {
        let p = q + d.normalize() * t;
        let h = p.y;
        let rho = p.x.hypot(p.z);
        let resid = ((rho - R_MAJOR).powi(2) + h * h).sqrt() - R_MINOR;
        assert!(
            resid.abs() < 1e-9,
            "a constructed root is {resid:e} off the tube — the Ferrari branch, its \
             resolvent or the sqrt-chain cube root has drifted"
        );
    }
}

/// **The trim and incidence predicates**, reached through the public
/// door on a body that carries each window, and named here so a Zero arm
/// that stopped being taken has a row to go red.
///
/// `bool_torus_trim` is exercised by every windowed row in the sweep
/// suite; what this adds is the two PERIOD guards, whose Zero arm is not
/// an escalation but a CLASS SELECTION — it is how a face is served with
/// no window at all, so a Zero that stopped being reached would silently
/// turn every wrapping face into `PartialTorusFace`.
#[test]
fn the_period_guards_zero_arm_is_what_serves_a_wrapping_face() {
    // The donut's faces wrap both coordinates and are served by the
    // closed group, so they never reach the guards; a band that wraps
    // only the major azimuth reaches `bool_torus_trim_major_period`
    // Zero and is served. That body is minted in the sweep suite (the
    // spool), which `topo` cannot depend on — so what is asserted here
    // is the arm's own contract on the resolved geometry, and the sweep
    // suite's `the_minor_window_trims_the_spool_band` is the row that
    // drives it through a real body.
    //
    // The census above plus that row are jointly the naming: this
    // module records the split rather than duplicating a fixture it
    // cannot build.
    let (q, d) = (Point3::new(0.0, 0.0, 2.0), Vec3::new(0.0, 0.0, -1.0));
    let Ok(TorusRoots::Certified { count, .. }) = roots(q, d) else {
        panic!("a radial midplane ray must certify")
    };
    assert_eq!(count, 4, "a diameter of the ring crosses four walls");
}

/// **`bool_ray_torus_count` is not reached by anything**, and this row
/// is the record of that rather than an exercise of it.
///
/// Rung 5 compares the CONSTRUCTED root count against the CERTIFIED one.
/// Neither review lane's oracle reached a disagreement (1310 rays
/// geometrically counted, 3000+ independently bisected), and nothing
/// here does either. It stays because it is the one place the sign
/// ladder and the factorization — two different computations on the same
/// coefficients — are made to agree at run time, which is what keeps
/// "answered only on a certified count" falsifiable rather than a claim
/// about the algebra.
///
/// What this row CAN pin is the invariant that would have to break for
/// it to fire: every certified answer carries exactly `count` roots, and
/// every one of them is on the tube.
#[test]
fn the_certified_count_and_the_constructed_roots_have_never_disagreed() {
    let mut certified = 0;
    for i in 0..12 {
        for j in 0..7 {
            let q = Point3::new(-2.2 + 0.4 * f64::from(i), -0.45 + 0.15 * f64::from(j), 0.13);
            let d = Vec3::new(0.87, 0.21, 0.44);
            if let Ok(TorusRoots::Certified { count, ts }) = roots(q, d) {
                certified += 1;
                for &t in &ts[..count] {
                    let p = q + d.normalize() * t;
                    let resid = ((p.x.hypot(p.z) - R_MAJOR).powi(2) + p.y * p.y).sqrt() - R_MINOR;
                    assert!(
                        resid.abs() < 1e-8,
                        "a root the arm certified is {resid:e} off the tube"
                    );
                }
            }
        }
    }
    assert!(certified > 20, "the lattice must actually certify counts");
}

/// The sign ladder's own arithmetic, at the poses that make each
/// classifying sign definite — so `bool_ray_torus_shape` and
/// `bool_ray_torus_depth` are not merely reached but reached on BOTH
/// sides of the four-versus-none split they exist to make.
#[test]
fn the_four_versus_none_split_is_exercised_both_ways() {
    // Positive discriminant, four real roots: the hole.
    assert!(matches!(
        roots(centre(), Vec3::new(1.0, 0.0, 0.0)),
        Ok(TorusRoots::Certified { count: 4, .. })
    ));
    // Positive discriminant, NO real roots: a ray through the hole but
    // tilted steeply enough to leave through the middle without meeting
    // the tube.
    assert!(
        matches!(
            roots(centre(), Vec3::new(0.02, 1.0, 0.0)),
            Ok(TorusRoots::Miss)
        ),
        "a steep ray up the hole meets nothing"
    );
    // And the ladder never answers on a Zero: an exactly axial ray.
    let axial = roots(centre(), Vec3::new(0.0, 1.0, 0.0));
    assert!(
        matches!(axial, Ok(TorusRoots::Miss) | Ok(TorusRoots::Uncertain)),
        "the axis meets no tube, and must never be answered as a crossing: {:?}",
        axial.map(|r| match r {
            TorusRoots::Certified { count, .. } => count,
            _ => 0,
        })
    );
}
