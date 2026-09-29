//! **A tilted rod entering the half donut's cap and poking a lens out
//! of the inner equator.** No event lies on the lens's rim, so the
//! crossings the rod makes through the cap are the only ones the join
//! sees, and the body it builds drops the lens. The section certificate
//! has no arm for a torus against an oblique cylinder and refuses those
//! pairs on reach. It is raised on the built body ahead of the volume
//! backstop, so every op names that pair rather than the backstop's
//! refusal (which the closed-form lane raises here for the rod wall's
//! ellipse-arc trim).
//!
//! Other spins of the same rod, and steeper tilts, refuse earlier, in
//! the reduction, before any section is read.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::germ_pair;
use crate::revolve_common::{self, axis_y, validated};

use core::f64::consts::FRAC_PI_2;
use geom_brep::SurfaceKind;
use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::{Revolution, revolve};
use topo::{Body, BooleanError, BooleanOp as Op, BooleanResult, FaceKey};

fn half_donut() -> Body<f64> {
    let vp = validated(vec![revolve_common::donut_profile()]);
    revolve(
        &vp,
        axis_y(),
        Revolution::Partial(core::f64::consts::PI),
        Tol::witness(),
    )
    .expect("the half donut revolves")
    .body
}

/// The rod's axis direction at tilt `beta` about `y`.
fn dir(beta: f64) -> Vec3<f64> {
    Vec3::new(-beta.sin(), 0.0, -beta.cos())
}

/// The rod: radius 0.15, its axis from `(1.8, 0, 0)` along [`dir`] over
/// `t ∈ [−0.3, 1.4]`, spun `spin` about its own axis (which moves only
/// its seam).
fn rod(beta: f64, spin: f64) -> Body<f64> {
    let c = germ_pair::spin(&germ_pair::cyl(0.15, 0.85), Vec3::unit_z(), spin);
    let c = germ_pair::spin(&c, Vec3::unit_y(), beta);
    let centre = Vec3::new(1.8, 0.0, 0.0) + dir(beta) * 0.55;
    topo::transform_rigid(&c, &Affine3::translation(centre), Tol::witness()).expect("the rod moves")
}

fn in_solid(b: &Body<f64>, q: Point3<f64>) -> Option<bool> {
    let band = geom_core::Band::linear(Tol::witness()).expect("the run's band");
    match topo::point_in_solid(b, q, band, Tol::witness()) {
        Ok(topo::SolidContainment::In) => Some(true),
        Ok(topo::SolidContainment::Out) => Some(false),
        _ => None,
    }
}

fn kind_of(b: &Body<f64>, f: FaceKey) -> Option<SurfaceKind> {
    b.get_face(f)
        .and_then(|face| b.get_surface(face.surface))
        .map(SurfaceKind::of)
}

type Run = (
    Op,
    bool,
    &'static str,
    Result<BooleanResult<f64>, BooleanError>,
);

/// Every op, in both operand orders of ∖ and ∩; the flag says whether
/// the rod is operand A.
fn every_op(h: &Body<f64>, c: &Body<f64>) -> [Run; 5] {
    let t = Tol::witness();
    [
        (Op::Union, false, "h ∪ c", topo::union(h, c, t)),
        (Op::Intersect, false, "h ∩ c", topo::intersect(h, c, t)),
        (Op::Intersect, true, "c ∩ h", topo::intersect(c, h, t)),
        (Op::Subtract, false, "h ∖ c", topo::subtract(h, c, t)),
        (Op::Subtract, true, "c ∖ h", topo::subtract(c, h, t)),
    ]
}

fn outcome(r: &Result<BooleanResult<f64>, BooleanError>) -> String {
    match r {
        Err(e) => format!("{e:?}"),
        Ok(r) => format!("answered {:?}", r.body().map(|b| b.kind)),
    }
}

/// **The poses that reach the section certificate refuse every op
/// there**, on a torus × rod-wall pair, in both operand orders of ∖ and
/// ∩: the item's pose (tilt 0.5, spin π/2) and both spins at two
/// shallower tilts. Every torus × rod-wall pair reads `Err(Reach)`,
/// never a certified loop. Red against the certificate raised after
/// the volume backstop, and against a reach refusal waved through:
/// either lets the op reach the backstop's `ClassificationInvariant`.
#[test]
fn the_tilted_rod_refuses_every_op_at_the_certificate_on_reach() {
    let h = half_donut();
    for (beta, spin) in [
        (0.5, FRAC_PI_2),
        (0.3, 0.0),
        (0.3, FRAC_PI_2),
        (0.4, 0.0),
        (0.4, FRAC_PI_2),
    ] {
        let c = rod(beta, spin);
        let pose = format!("β {beta}, spin {spin:.4}");
        for (op, rod_first, what, r) in every_op(&h, &c) {
            let (kind, other) = if rod_first {
                (SurfaceKind::Cylinder, SurfaceKind::Torus)
            } else {
                (SurfaceKind::Torus, SurfaceKind::Cylinder)
            };
            match r {
                Err(BooleanError::CurvedPairUnsupported {
                    op: Some(o),
                    site,
                    kind: k,
                    other_kind: ok,
                    ..
                }) => assert_eq!(
                    (site, o, k, ok),
                    (topo::PairRefusalSite::InteriorLoopGuard, op, kind, other),
                    "{what} at {pose}"
                ),
                r => panic!(
                    "{what} at {pose}: not refused at the certificate: {}",
                    outcome(&r)
                ),
            }
        }
        let report = topo::test_support::section_report(Op::Union, &h, &c, Tol::witness())
            .expect("the reduction runs");
        let torus_wall: Vec<&str> = report
            .iter()
            .filter(|(fa, fb, _)| {
                kind_of(&h, *fa) == Some(SurfaceKind::Torus)
                    && kind_of(&c, *fb) == Some(SurfaceKind::Cylinder)
            })
            .map(|(.., v)| v.as_str())
            .collect();
        assert!(
            !torus_wall.is_empty() && torus_wall.iter().all(|v| *v == "Err(Reach)"),
            "the torus × rod-wall pairs at {pose}: {report:?}"
        );
    }
}

/// **The other spins, and the steeper tilts, refuse in the reduction**
/// with `CurvedSectorSideUnsupported`, on every op, before any body is
/// built or any section read.
#[test]
fn other_spins_and_steeper_tilts_refuse_in_the_reduction() {
    let h = half_donut();
    for (beta, spin) in [
        (0.5, 0.0),
        (0.5, 0.5),
        (0.5, 1.0),
        (0.5, 2.5),
        (0.6, 0.0),
        (0.6, FRAC_PI_2),
        (0.7, 0.0),
        (0.7, FRAC_PI_2),
    ] {
        let c = rod(beta, spin);
        for (_, _, what, r) in every_op(&h, &c) {
            assert!(
                matches!(r, Err(BooleanError::CurvedSectorSideUnsupported { .. })),
                "{what} at β {beta}, spin {spin}: {}",
                outcome(&r)
            );
        }
    }
}

/// **The lens is real.** At the rod axis's closest approach to the
/// donut's axis (`t = 1.8 sin β`, radius `1.8 cos β ≈ 1.5796`), the
/// axis point lies in both operands, and a point `0.11` toward the hole
/// (radius `≈ 1.4696`, inside the rod, short of the inner equator at
/// 1.5) lies in the rod and outside the half donut: the region the
/// join's body drops from ∪.
#[test]
fn the_rod_pokes_a_lens_out_of_the_inner_equator() {
    let (h, c) = (half_donut(), rod(0.5, FRAC_PI_2));
    let axis = Point3::new(1.8, 0.0, 0.0) + dir(0.5) * (1.8 * 0.5_f64.sin());
    let lens = axis + Vec3::new(-axis.x, 0.0, -axis.z).normalize() * 0.11;
    assert_eq!(
        (in_solid(&h, axis), in_solid(&c, axis)),
        (Some(true), Some(true)),
        "the closest-approach axis point {axis:?}"
    );
    assert_eq!(
        (in_solid(&h, lens), in_solid(&c, lens)),
        (Some(false), Some(true)),
        "the lens point {lens:?}"
    );
}
