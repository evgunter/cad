//! **FILLET-H6 review probes (lane r2)** — the cap-rim `Smooth` arm's
//! unreachability argument, read as an inequality rather than as prose.
//!
//! The argument written at `extrude::upgrade_rim`'s `Smooth` arm is:
//! the direction gates admit an in-plane component of at most ε
//! against a normal component of at least `K·ε`, so an admitted
//! extrusion vector `w` parts from the sketch normal `n` by at most
//! `1/K`, and the cap–wall wedge is therefore definite wherever the
//! lever arm is.
//!
//! Two things in that sentence are quantitative, and neither is
//! asserted by the unit's own suite:
//!
//! 1. The conclusion is **K-dependent**, and the arm's written bound
//!    `sin θ ≥ √(1 − 1/K²)` excludes the `Smooth` outcome only for
//!    `K > √2`. `Tol` accepts any `K > 1` (`CAD_AMBIGUITY_K`,
//!    `geom-core/src/tolerance.rs:460` — the predicate is `v > 1.0`),
//!    and no CI row varies it. Below the crossover the arm is
//!    reachable on inputs both direction gates admit.
//! 2. "The wedge margin `sin θ · arm` is definite wherever the arm is"
//!    is false as written at every K: the arm gate asks `arm ≥ K·ε`
//!    and the wedge gate asks `sin θ · arm ≥ K·ε`, so any `sin θ < 1`
//!    at the smallest admitted arm lands in-band — a typed `SliverRim`
//!    rather than a wrong description, but a third outcome the
//!    sentence does not admit.
//!
//! Both rows are stated symbolically in `K` and ε so they read the
//! run's tolerance rather than a fixture's metres.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_brep::{DihedralClass, classify_dihedral};
use geom_core::{Band, Point3, Tol, Vec3};

/// The cap surface an extrusion mints: a plane of normal `n`.
fn cap_plane(n: Vec3<f64>) -> Surface<f64> {
    let u = if n.z.abs() < 0.5 {
        Vec3::new(0.0, 0.0, 1.0).cross(n).normalize()
    } else {
        Vec3::new(1.0, 0.0, 0.0).cross(n).normalize()
    };
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: n,
        u_ref: u,
    }
}

/// The wall surface an extrusion mints for a line leg: the plane
/// through the leg's chord direction and the extrusion vector `w`
/// (`extrude::side_surface`'s Newell plane over the quad, whose two
/// rulings are `w`).
fn wall_plane(chord: Vec3<f64>, w: Vec3<f64>) -> Surface<f64> {
    let m = chord.cross(w).normalize();
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: m,
        u_ref: chord.normalize(),
    }
}

/// The worst extrusion vector the two direction gates admit at
/// ambiguity multiplier `k`: in-plane component exactly ε (the
/// coincidence threshold `extrusion_obliquity` still calls Zero)
/// against a normal component of exactly `k·ε` (the escalation
/// threshold `extrusion_normal_component` still calls definite).
fn worst_admitted_w(eps: f64, k: f64) -> Vec3<f64> {
    Vec3::new(eps, 0.0, k * eps)
}

/// `classify_dihedral`'s verdict at a cap rim built from the worst
/// admitted extrusion vector, on a line leg of chord length `arm`,
/// under a band of ratio `k`.
fn worst_rim_verdict(eps: f64, k: f64, arm: f64) -> Result<DihedralClass, String> {
    let band = Band::new(eps, k * eps).unwrap();
    let w = worst_admitted_w(eps, k);
    // The leg runs perpendicular to the tilt, which is what maximizes
    // the wall's tilt away from the cap.
    let cap = cap_plane(Vec3::new(0.0, 0.0, 1.0));
    let wall = wall_plane(Vec3::new(0.0, 1.0, 0.0), w);
    classify_dihedral(&cap, &wall, Point3::new(0.0, 0.0, 0.0), arm, band)
        .map_err(|e| format!("{e:?}"))
}

/// **The written bound is loose, and it closes only above `K = √2`.**
///
/// `Smooth` needs `sin θ · arm ≤ ε` while the arm gate has already
/// demanded `arm ≥ K·ε`, so `Smooth` needs `sin θ ≤ 1/K`. The arm's
/// bound `√(1 − 1/K²)` therefore rules `Smooth` out exactly when
/// `√(1 − 1/K²) > 1/K`, i.e. `K > √2` — not "regardless of ε", which
/// is the only free variable the sentence disclaims.
///
/// The true bound for the worst admitted `w` is `K/√(K² + 1)` (the
/// wall normal is `chord × w`, so `cos∠(n, m) = ε/|w|`), which is
/// strictly tighter and pushes the crossover down to `K⁴ = K² + 1`,
/// i.e. `K ≈ 1.272`. Both numbers are above 1, which is all `Tol`
/// enforces.
#[test]
fn the_arms_bound_rules_out_smooth_only_above_k_sqrt2() {
    let written = |k: f64| (1.0 - 1.0 / (k * k)).sqrt();
    let true_bound = |k: f64| k / (k * k + 1.0).sqrt();
    let smooth_ceiling = |k: f64| 1.0 / k;

    for k in [1.05_f64, 1.2, 1.4, 1.5, 2.0, 10.0] {
        assert!(
            written(k) <= true_bound(k),
            "the written bound must be a valid (if loose) lower bound at K = {k}",
        );
        assert_eq!(
            written(k) > smooth_ceiling(k),
            k > core::f64::consts::SQRT_2,
            "the written bound closes the Smooth arm iff K > sqrt(2) (K = {k})",
        );
    }
    // The tight bound's crossover: K^4 = K^2 + 1.
    let k_star = ((1.0 + 5.0_f64.sqrt()) / 2.0).sqrt();
    assert!((1.2..1.3).contains(&k_star), "K* = {k_star}");
    assert!(true_bound(k_star) - smooth_ceiling(k_star) < 1e-12);
}

/// **At `K = 1.2` the arm is reachable**: the cap–wall wedge of a body
/// both direction gates admit, on the shortest leg `vertex_separation`
/// admits (`K·ε`), classifies `Smooth`. Nothing here is out of band or
/// degenerate — every quantity is at a threshold the doors call
/// definite.
///
/// This asserts the ARM'S REACHABILITY, not the classifier's taste:
/// the inputs are the extremes the two `extrude` gates and the profile
/// door each certify as admissible at this K.
#[test]
fn the_cap_rim_smooth_arm_is_reachable_at_small_k() {
    let eps = Tol::witness().eps();

    // K = 10 (the default): the same extreme is definitely transverse,
    // with the arm four decades above the gate so the wedge clears it.
    assert_eq!(
        worst_rim_verdict(eps, 10.0, 1e4 * eps),
        Ok(DihedralClass::Transverse),
    );

    // K = 1.2, arm at the smallest chord the profile door admits.
    assert_eq!(
        worst_rim_verdict(eps, 1.2, 1.2 * eps),
        Ok(DihedralClass::Smooth),
        "the arm the PR argues unreachable is reached at K = 1.2",
    );
}

/// **"Definite wherever the arm is" is false at every K.** With the
/// arm at its smallest admitted value `K·ε`, the wedge margin is
/// `sin θ · K·ε`, which clears the escalation threshold `K·ε` only for
/// `sin θ ≥ 1`. Any admitted obliquity at all therefore escalates —
/// typed (`SliverRim`), so the arm's conclusion about `Smooth`
/// survives, but its stated dichotomy ("definite, or the collapsed arm
/// below") does not.
#[test]
fn the_smallest_admitted_arm_escalates_under_any_admitted_obliquity() {
    let eps = Tol::witness().eps();
    let k = 10.0;
    let v = worst_rim_verdict(eps, k, k * eps);
    assert!(
        v.is_err(),
        "expected the in-band escalation at the smallest admitted arm, got {v:?}",
    );
    // And it is the WEDGE that escalates, not the arm: the arm gate is
    // satisfied exactly.
    assert!(
        v.unwrap_err().contains("dihedral_wedge"),
        "the escalation must name the wedge predicate, not the arm",
    );
}
