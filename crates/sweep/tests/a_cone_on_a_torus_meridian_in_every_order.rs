//! **Two cones on a torus's meridian circle, in every operation and
//! order.** The torus `R = 2`, `r = 1` about `y`, against the triangle
//! `(1, 0), (3/2, 1/2), (1/2, 1/2)` revolved about the line through
//! `(2, 0, 0)` along `z`: its vertex sweeps the torus's meridian circle
//! in the `xy` plane, and its two cones (apexes `(2, 0, ±1)`, both
//! inside the tube, half-angle 45°) cross the tube there and along a
//! second curve each. Each pair is a cone off the torus's axis with its
//! apex clear of the torus, the section value's general cone × torus
//! pose, marched.
//!
//! Each run builds a valid body. `|A ∩ B|` matches an oracle that reads
//! no boolean: the solids' own membership along rays of the triangle's
//! revolution ([`intersection_by_rays`]). The other volumes then close as
//! the operations must, `|A ∪ B| + |A ∩ B| = |A| + |B|` and
//! `|A ∖ B| = |A| − |A ∩ B|`, within the measured pads: those identities
//! alone hold whatever the faces are labelled, so they are not the check.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};
use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::finished;
use sweep::{Revolution, RevolveAxis, revolve};
use topo::AtRestBody;

use crate::revolve_common::axis_y;

fn revolved(points: &[(f64, f64, f64)], axis: RevolveAxis<f64>, tol: Tol) -> AtRestBody<f64> {
    let lp = bulge_loop(
        points
            .iter()
            .map(|&(x, y, bulge)| (Point2::new(x, y), bulge))
            .collect(),
    );
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .unwrap();
    revolve(&vp, axis, Revolution::Full, tol)
        .expect("the profile revolves")
        .body
}

/// The torus `R = 2`, `r = 1` about `y`.
fn torus(tol: Tol) -> AtRestBody<f64> {
    revolved(&[(2.0, -1.0, 1.0), (2.0, 1.0, 1.0)], axis_y(), tol)
}

/// The triangle revolved about the line `x = 2` along the sketch's `−y`
/// (the profile on the axis's positive side),
/// then turned a quarter about the `x` axis through `(2, 0, 0)`, so that
/// line runs along `z` and the triangle's vertex `(1, 0)` sweeps the
/// meridian circle in `z = 0`.
fn triangle(tol: Tol) -> AtRestBody<f64> {
    let body = revolved(
        &[(1.0, 0.0, 0.0), (1.5, 0.5, 0.0), (0.5, 0.5, 0.0)],
        RevolveAxis {
            origin: Point2::new(2.0, 0.0),
            dir: Vec2::new(0.0, -1.0),
        },
        tol,
    );
    let turn = Affine3::rotation_about_axis(Point3::new(2.0, 0.0, 0.0), Vec3::unit_x(), FRAC_PI_2);
    let turned = topo::transform_rigid(&body, &geom_core::test_support::rigid(&turn, tol)).unwrap();
    finished("the meridian triangle", turned, tol)
}

fn volume(body: &AtRestBody<f64>, tol: Tol) -> (f64, f64) {
    let m = topo::mass_properties(body, tol, topo::test_support::pinned_resolution(tol))
        .unwrap_or_else(|e| panic!("the body does not measure: {e:?}"))
        .enclosure;
    (m.volume, m.volume_pad)
}

/// The measured volume of a run's result, zero for an empty one.
fn result_volume(
    label: &str,
    got: Result<topo::BooleanResult<f64>, topo::BooleanError>,
    tol: Tol,
) -> (f64, f64) {
    let got = got.unwrap_or_else(|e| panic!("{label}: refused: {e:?}"));
    got.body().map_or((0.0, 0.0), |b| volume(&b.body, tol))
}

/// Whether `q` lies in the solid torus `R = 2`, `r = 1` about `y`.
fn in_torus(q: Point3<f64>) -> bool {
    (q.x.hypot(q.z) - 2.0).hypot(q.y) <= 1.0
}

/// **`|A ∩ B|`, measured apart from the boolean**: the triangle's solid
/// in its own cylindrical coordinates about the `z`-line through
/// `(2, 0, 0)` — height `z ∈ [0, 1/2]`, radius `ρ ∈ [1 − z, 1 + z]`,
/// angle `ψ` — and along each ray `(z, ψ)` the intervals of `ρ` inside
/// the solid torus, each end bisected to rounding, integrated as
/// `Σ (b² − a²)/2` by the midpoint rule over `n_z × n_ψ` rays.
fn intersection_by_rays(n_z: usize, n_psi: usize) -> f64 {
    const SCAN: usize = 64;
    let mut total = 0.0;
    for iz in 0..n_z {
        let z = 0.5 * (iz as f64 + 0.5) / n_z as f64;
        for ip in 0..n_psi {
            let psi = 2.0 * PI * (ip as f64 + 0.5) / n_psi as f64;
            let at = |rho: f64| in_torus(Point3::new(2.0 + rho * psi.cos(), rho * psi.sin(), z));
            let (lo, hi) = (1.0 - z, 1.0 + z);
            let mut ray = 0.0;
            let mut start = at(lo).then_some(lo);
            for k in 0..SCAN {
                let (a, b) = (
                    lo + (hi - lo) * k as f64 / SCAN as f64,
                    lo + (hi - lo) * (k + 1) as f64 / SCAN as f64,
                );
                if at(a) == at(b) {
                    continue;
                }
                let (mut x, mut y) = (a, b);
                for _ in 0..60 {
                    let mid = 0.5 * (x + y);
                    if at(mid) == at(a) {
                        x = mid;
                    } else {
                        y = mid;
                    }
                }
                let cut = 0.5 * (x + y);
                match start.take() {
                    Some(s) => ray += (cut * cut - s * s) / 2.0,
                    None => start = Some(cut),
                }
            }
            if let Some(s) = start {
                ray += (hi * hi - s * s) / 2.0;
            }
            total += ray;
        }
    }
    total * (0.5 / n_z as f64) * (2.0 * PI / n_psi as f64)
}

/// What the ray oracle may be off by, in cubic metres: its midpoint
/// rule meets the ray intervals' square-root ends, and halving its step
/// is held below half of this.
const RAY_SLACK: f64 = 5e-3;

#[test]
fn the_meridian_triangle_and_its_torus_build_in_every_order() {
    let tol = Tol::witness();
    let (t, k) = (torus(tol), triangle(tol));
    let (vt, pt) = volume(&t, tol);
    let (vk, pk) = volume(&k, tol);
    assert!(
        (vt - 4.0 * PI * PI).abs() <= pt + 1e-9 && (vk - PI / 2.0).abs() <= pk + 1e-9,
        "the operands measure as Pappus has them: {vt} ± {pt}, {vk} ± {pk}"
    );
    let oracle = intersection_by_rays(128, 512);
    let coarse = intersection_by_rays(64, 256);
    assert!(
        (oracle - coarse).abs() <= RAY_SLACK / 2.0,
        "the ray oracle converges: {coarse} at half the rays, {oracle}"
    );
    for (name, a, b, va, pa) in [
        ("torus first", &t, &k, vt, pt),
        ("triangle first", &k, &t, vk, pk),
    ] {
        let (vu, pu) = result_volume(&format!("{name}: ∪"), topo::union(a, b, tol), tol);
        let (vi, pi) = result_volume(&format!("{name}: ∩"), topo::intersect(a, b, tol), tol);
        let (vd, pd) = result_volume(&format!("{name}: ∖"), topo::subtract(a, b, tol), tol);
        assert!(
            (vi - oracle).abs() <= pi + RAY_SLACK,
            "{name}: |A ∩ B| = {vi} ± {pi} misses the ray oracle's {oracle}"
        );
        assert!(
            (vu + vi - vt - vk).abs() <= pu + pi + pt + pk + 1e-9,
            "{name}: |A ∪ B| + |A ∩ B| = {} misses |A| + |B| = {}",
            vu + vi,
            vt + vk
        );
        assert!(
            (vd - (va - vi)).abs() <= pd + pa + pi + 1e-9,
            "{name}: |A ∖ B| = {vd} misses |A| − |A ∩ B| = {}",
            va - vi
        );
    }
}

/// **A cone coaxial with the torus is read elsewhere, never as a
/// desync.** The solid cone `(0, 0), (3, 0), (0, 3)` revolved about `y`,
/// its wall crossing the tube of the torus about `y`: the pair is served
/// by kind and its coaxial pose is the closed form's, so the section
/// table holds nothing for it. Each operation, in both orders, ends as
/// the pin below says (five build; the torus less the cone refuses);
/// a `JoinDesync` (the kernel-bug class) is red wherever it appears.
#[test]
fn a_coaxial_cone_against_the_torus_ends_as_pinned() {
    let tol = Tol::witness();
    let t = torus(tol);
    let k = revolved(
        &[(0.0, 0.0, 0.0), (3.0, 0.0, 0.0), (0.0, 3.0, 0.0)],
        axis_y(),
        tol,
    );
    let mut got = Vec::new();
    for (name, a, b) in [("torus first", &t, &k), ("cone first", &k, &t)] {
        for (op, run) in [
            ("∪", topo::union(a, b, tol)),
            ("∩", topo::intersect(a, b, tol)),
            ("∖", topo::subtract(a, b, tol)),
        ] {
            assert!(
                !matches!(run, Err(topo::BooleanError::JoinDesync { .. })),
                "{name} {op}: a coaxial cone × torus is no desync: {run:?}"
            );
            let outcome = match &run {
                Ok(_) => "builds".to_owned(),
                Err(e) => format!("{e:?}")
                    .split([' ', '{', '('])
                    .next()
                    .unwrap_or_default()
                    .to_owned(),
            };
            got.push(format!("{name} {op}: {outcome}"));
        }
    }
    // Measured at the default ε, 1e-6 and 1e-12 alike. The one refusal
    // is `work/sectcone/a-torus-less-a-coaxial-cone-refuses-edge-in-face`.
    let pinned: Vec<String> = [
        "torus first ∪: builds",
        "torus first ∩: builds",
        "torus first ∖: EdgeInFace",
        "cone first ∪: builds",
        "cone first ∩: builds",
        "cone first ∖: builds",
    ]
    .map(str::to_owned)
    .to_vec();
    assert_eq!(got, pinned, "the coaxial pair's outcomes");
}
