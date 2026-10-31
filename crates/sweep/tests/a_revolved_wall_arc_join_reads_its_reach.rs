//! **A wall–arc join the profile reads tangent refuses its revolved
//! sliver** (#5377's fix pass, review finding F5). A line 0.05 long
//! runs into an arc of radius 1 sweeping 0.05 rad whose start heading
//! is turned `φ` off the line's. The profile reads the joint's turn at
//! the shorter leg's arm, 0.05, so at `φ = 5ε` and `10ε` it records a
//! tangent joint. Revolved 20 m off the axis, the latitude join reads
//! the walls' dihedral at the faces' reach, which runs about the rim
//! and exceeds the profile's arm: there the walls part by `φ·1`, in
//! band, and the join refuses `SliverJoin` naming the profile's joint,
//! full and partial, at `f64` and at `Interval`. Past `φ = 20ε` the
//! profile's pair pass escalates the line's clearance from the arc's
//! carrier, so no larger turn reaches the revolve as a tangent joint.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::ErrorTextReading;
use geom_core::{Arc2, Interval, Point2, Real, Tol, Vec2};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane, ValidatedProfile};
use sweep::{Revolution, RevolveAxis, RevolveError, revolve};

/// The loop, `x0` off the y axis: up the line, round the arc, and two
/// lines home.
fn wall_arc(x0: f64, phi: f64) -> Profile<f64> {
    let h = std::f64::consts::FRAC_PI_2 + phi;
    let p1 = Point2::new(x0, 0.05);
    let c = Point2::new(p1.x - h.sin(), p1.y + h.cos());
    let a0 = (p1.y - c.y).atan2(p1.x - c.x);
    let q = Point2::new(c.x + (a0 + 0.05).cos(), c.y + (a0 + 0.05).sin());
    Profile::new(
        SketchPlane::xy(),
        vec![ProfileLoop::new(vec![
            (Point2::new(x0, 0.0), Segment::Line),
            (
                p1,
                Segment::Arc(Arc2 {
                    centre: c,
                    radius: 1.0,
                    sweep: 0.05,
                }),
            ),
            (q, Segment::Line),
            (Point2::new(x0 - 0.1, 0.0), Segment::Line),
        ])],
    )
}

fn validated(p: &Profile<f64>) -> (ValidatedProfile<f64>, ValidatedProfile<Interval>) {
    let tol = Tol::witness();
    let v = p.validate(tol).expect("the profile validates");
    let lifted = Profile::new(
        SketchPlane::xy(),
        p.loops
            .iter()
            .map(|lp| lp.map_scalar(Interval::from_f64))
            .collect(),
    )
    .validate(tol)
    .expect("the profile validates at Interval");
    (v, lifted)
}

fn axis<T: Real>() -> RevolveAxis<T> {
    RevolveAxis {
        origin: Point2::new(T::zero(), T::zero()),
        dir: Vec2::new(T::zero(), T::one()),
    }
}

/// The join's margin in meters, or the panic naming what came back.
fn sliver_margin(what: &str, got: Result<(), RevolveError>) -> f64 {
    match got {
        Err(RevolveError::SliverJoin {
            loop_index: 0,
            vertex_index: 1,
            source,
            ..
        }) => match source.margin.diagnostic_f64_for_error_text() {
            ErrorTextReading::Value(m) => m,
            ErrorTextReading::Enclosure { lo, hi } => 0.5 * (lo + hi),
            ErrorTextReading::Invalid => panic!("{what}: a poisoned margin"),
        },
        other => panic!("{what}: the join must refuse its sliver, got {other:?}"),
    }
}

#[test]
fn a_join_the_profile_reads_tangent_refuses_its_revolved_sliver() {
    let tol = Tol::witness();
    let eps = tol.eps();
    for m in [5.0, 10.0] {
        let (v, lifted) = validated(&wall_arc(20.0, m * eps));
        assert_eq!(
            v.loops()[0].tangent_joints(),
            &[1],
            "φ = {m}ε: the profile reads the joint tangent at the 0.05 arm"
        );
        let rows = [
            (
                "full, f64",
                revolve(&v, axis(), Revolution::Full, tol).map(|_| ()),
            ),
            (
                "partial, f64",
                revolve(&v, axis(), Revolution::Partial(1.0), tol).map(|_| ()),
            ),
            (
                "full, Interval",
                revolve(&lifted, axis(), Revolution::Full, tol).map(|_| ()),
            ),
            (
                "partial, Interval",
                revolve(
                    &lifted,
                    axis(),
                    Revolution::Partial(Interval::from_f64(1.0)),
                    tol,
                )
                .map(|_| ()),
            ),
        ];
        for (lane, got) in rows {
            // Pinned: at ε = 1e-12 the wall's torus rim (R/r ≈ 19, inside
            // the session box) is past limb 2's reach before the join
            // reads the sliver — the torus composite's conditioning row
            // (`work/offset/the-torus-composite-bounds-a-thin-torus-far-above-its-carriers-rounding.md`),
            // the kernel's limit at f64 and the stall at Interval. Red the
            // day the rim certifies there, and the sliver must be read.
            if test_utils::kernel_limit::at_row(eps, &[1e-12]) {
                let scalar = if lane.ends_with("f64") {
                    test_utils::kernel_limit::Scalar::Point
                } else {
                    test_utils::kernel_limit::Scalar::Enclosure
                };
                assert!(
                    test_utils::kernel_limit::is_kernel_limit_at(scalar, &got),
                    "φ = {m}ε, {lane}: pinned to refuse as limb 2's stall ({scalar:?} scalar) \
                     at ε = 1e-12, got {got:?}"
                );
                continue;
            }
            let margin = sliver_margin(&format!("φ = {m}ε, {lane}"), got);
            assert!(
                (margin / (m * eps) - 1.0).abs() < 1e-3,
                "φ = {m}ε, {lane}: the walls part by φ at the unit reach, \
                 past the profile's 0.05 arm; got {:e}ε",
                margin / eps
            );
        }
    }
}
