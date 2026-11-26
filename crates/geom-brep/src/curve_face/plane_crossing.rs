//! **The plane-crossing door** ([`super::Door::PlaneCrossing`]): where
//! a carrier's span crosses a plane, keyed on the carrier's kind
//! ([`plane_crossing_lane`]). A line's one root is its endpoints'; a
//! conic's are the roots of the plane's residual along it, read over the
//! span; a spiric or a spline has no closed form here.

use geom_core::{Band, Decide, Indeterminate};

use crate::curve_pair::Span;
use crate::edge_crossing::CrossingDecision;

/// Which lane finds where a carrier's span crosses a plane
/// ([`plane_crossing_lane`]), keyed on the carrier's kind.
#[derive(Debug)]
pub enum PlaneCrossingLane<T> {
    /// A line: its signed distance is affine, with one root that the
    /// endpoints' sides place, so the caller's endpoint lane owns it.
    Line,
    /// A circle or an ellipse: what the root lane found.
    Conic(ConicPlaneMeet<T>),
    /// A spiric or a spline: no closed form finds its crossings. Its
    /// endpoints' sides say nothing about its interior, which can cross
    /// the plane and come back between same-side ends, and a parameter
    /// interpolated between them is not a point on the plane. A spline's
    /// crossings are the curve × face table's [`super::Door::SplineComposite`]
    /// door's. A caller may not read an `Unlaned` span's crossings from
    /// its endpoints: it refuses it, reads it through another door, or
    /// passes it only behind a certificate that its whole locus clears
    /// the plane.
    Unlaned,
}

/// What a conic carrier's span meets of a plane
/// ([`PlaneCrossingLane::Conic`]).
#[derive(Debug)]
pub enum ConicPlaneMeet<T> {
    /// The conic lies in the plane: its plane parallel to the query
    /// plane, at an offset the pose read in the zero band all round
    /// ([`super::pose::Pose::Lies`]).
    InPlane,
    /// The carrier definitely never meets the plane, wholly on one side
    /// or parallel to it off it.
    Miss,
    /// The roots interior to the span, ascending (possibly none), or
    /// the rung no verdict was reached on.
    Roots(Result<Vec<T>, ConicRootFault>),
}

/// Which rung of the conic root lane ([`plane_crossing_lane`]) escalated, with its
/// diagnostics. The first two ask whether the plane coincides with the
/// conic; the other two are [`CrossingDecision`]s.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConicRootFault {
    /// Whether the conic's plane is parallel to the query plane.
    PlaneParallel(Indeterminate),
    /// Whether the conic reaches the plane, grazes it, or misses it.
    BellyGraze(Indeterminate),
    /// Whether a root lands inside the span, at an end, or outside it.
    CrossingInterior(Indeterminate),
    /// Which of two interior roots comes first.
    RootOrder(Indeterminate),
}

impl ConicRootFault {
    /// The escalation's diagnostics, whichever rung raised it.
    #[must_use]
    pub fn diag(self) -> Indeterminate {
        match self {
            Self::PlaneParallel(diag)
            | Self::BellyGraze(diag)
            | Self::CrossingInterior(diag)
            | Self::RootOrder(diag) => diag,
        }
    }

    /// The crossing decision the rung decides, or `None` for the two
    /// rungs that ask whether the plane coincides with the conic: every
    /// door that meets this fault routes it through here.
    #[must_use]
    pub fn decision(self) -> Option<CrossingDecision> {
        match self {
            Self::PlaneParallel(_) | Self::BellyGraze(_) => None,
            Self::CrossingInterior(_) => Some(CrossingDecision::OnEdge),
            Self::RootOrder(_) => Some(CrossingDecision::Order),
        }
    }

    /// What the rung decides, as a clause with no colon or dash of its
    /// own.
    #[must_use]
    pub fn subject(self) -> &'static str {
        match self {
            Self::PlaneParallel(_) => {
                "whether a curved edge's plane is parallel to the plane that cuts it"
            }
            Self::BellyGraze(_) => "whether a plane cuts a curved edge, grazes it or misses it",
            Self::CrossingInterior(_) => CrossingDecision::OnEdge.subject(),
            Self::RootOrder(_) => CrossingDecision::Order.subject(),
        }
    }
}

/// **The lane that finds where a carrier's span crosses a plane**
/// ([`PlaneCrossingLane`]): the sweep's, `slit`'s, `factored_crossing`'s
/// and classify's reading until they read [`super::cross()`]. On a conic it
/// is a wrapper: the pose ([`super::pose()`]'s plane arm, which reads the
/// conic's height above the plane `d₀ + R cos(θ − φ)` by its belly, its
/// parallel gate and offset, and a graze's side and curvature), then the
/// method's closed-form roots placed against the span
/// ([`super::cross()`]'s placement), mapped back:
///
/// - `Lies` is [`ConicPlaneMeet::InPlane`]; `Apart` is
///   [`ConicPlaneMeet::Miss`];
/// - a graze, the pose's `Tangent`, is its one double root, inserted
///   where it is inside the span (the graze net: the insertion leaves a
///   same-side ON contact for the sector classification);
/// - a pose rung in band is the rung's [`ConicRootFault`], a graze's
///   curvature in the zero band its [`ConicRootFault::BellyGraze`];
/// - a root in band of an end is [`ConicRootFault::CrossingInterior`], and
///   two roots the placement cannot part [`ConicRootFault::RootOrder`];
///   two within the band of each other by their chord are one,
///   inserted once.
///
/// **What a caller may assume of a root**: it is the closed form's root
/// at the scalar's rounding, decided strictly inside `[t0, t1]` (its
/// point's chord to each end past the band, and inside by its
/// parameter), and the roots are ascending and decided apart. It is not
/// a point certified on the plane: a caller that splits an edge at one
/// decides that split's own interiority and certifies the pieces itself.
pub fn plane_crossing_lane<T: Decide>(
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    plane_origin: geom_core::Point3<T>,
    plane_normal: geom_core::Vec3<T>,
    band: Band,
) -> PlaneCrossingLane<T> {
    match (carrier.conic(), carrier) {
        (Some(conic), _) => PlaneCrossingLane::Conic(conic_plane_meet(
            &conic,
            &Span::new(carrier, (t0, t1)),
            (plane_origin, plane_normal),
            band,
        )),
        (None, geom::Curve3::Line(_)) => PlaneCrossingLane::Line,
        (None, _) => PlaneCrossingLane::Unlaned,
    }
}

/// The conic arm of [`plane_crossing_lane`].
fn conic_plane_meet<T: Decide>(
    conic: &dyn geom::Conic<T>,
    span: &Span<'_, T>,
    plane: (geom_core::Point3<T>, geom_core::Vec3<T>),
    band: Band,
) -> ConicPlaneMeet<T> {
    use super::pose::{Formed, Pose, PoseRefusal, PoseRung};
    let roots = match super::pose::plane_conic(span, conic, plane, band).map_err(|r| r.why) {
        Ok(Pose::Lies) => return ConicPlaneMeet::InPlane,
        Ok(Pose::Apart) => return ConicPlaneMeet::Miss,
        Ok(Pose::Tangent(touch)) => vec![touch.at],
        Ok(Pose::Isolated(super::pose::Form(Formed::PlaneConic { roots }))) => roots.to_vec(),
        Ok(Pose::Isolated(_) | Pose::Through(_)) => {
            unreachable!("a conic's plane arm answers no other form")
        }
        Err(PoseRefusal::InBand {
            rung: PoseRung::PlaneParallel,
            diag,
        }) => return ConicPlaneMeet::Roots(Err(ConicRootFault::PlaneParallel(diag))),
        Err(PoseRefusal::InBand { diag, .. }) => {
            return ConicPlaneMeet::Roots(Err(ConicRootFault::BellyGraze(diag)));
        }
        Err(other) => unreachable!("the plane arm refuses in band only: {other:?}"),
    };
    let placed = super::cross::place(span, &roots, band);
    if let Some(&(_, why)) = placed.refused.first() {
        return ConicPlaneMeet::Roots(match why {
            super::cross::PlaceWhy::End(diag) => Err(ConicRootFault::CrossingInterior(diag)),
            super::cross::PlaceWhy::Apart(diag) => Err(ConicRootFault::RootOrder(diag)),
            super::cross::PlaceWhy::Together(t) => Ok(vec![t]),
            // A kernel fault this wrapper's shape has no arm for: it fails
            // loud.
            super::cross::PlaceWhy::Contradiction(what) => {
                unreachable!("the plane lane's placement read a contradiction: {what}")
            }
        });
    }
    ConicPlaneMeet::Roots(Ok(placed
        .meetings
        .into_iter()
        .filter_map(|(t, on)| matches!(on, super::cross::OnSpan::Inside { .. }).then_some(t))
        .collect()))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    //! DIRECT trilean rows for the conic crossing lane (M5 PR 5 fix
    //! pass, review M3): `curve_face_pose_belly_graze` and
    //! the span's end reading (`curve_pair_span_end_*`, through
    //! [`span_place`]) each get their definite /
    //! exactly-degenerate / in-band arms against a pure band (the
    //! geom-core test discipline: never `Band::linear` in a lib test).

    use geom::Curve3;
    use geom_core::{Band, Point3, Vec3};

    use super::{ConicPlaneMeet, PlaneCrossingLane, plane_crossing_lane};
    use crate::curve_pair::Span;

    /// **A crossing at a slow end is the span's end.** An ellipse stored
    /// swapped — `major = 0.01` along `u_ref`, `minor = 10` — runs at
    /// 0.01 m/rad at its `θ = π/2` vertex. A plane crosses it `δ = 5e-8`
    /// rad past that vertex, an arc of 5e-10 m: inside the zero band, so
    /// the crossing is the span's own end, not an interior root. Red
    /// where the gap from the end is read at the stored `minor` (10
    /// m/rad), 5e-7 m, which certifies it interior, a split 5e-10 m from
    /// a vertex.
    #[test]
    fn a_crossing_at_an_end_is_metered_at_the_smaller_semi_axis() {
        let e = Curve3::Ellipse(geom::Ellipse::new(
            geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
            0.01,
            10.0,
        ));
        let t0 = core::f64::consts::FRAC_PI_2;
        let delta = 5e-8;
        let x = 0.01 * (t0 + delta).cos();
        let got = roots_of(plane_crossing_lane(
            &e,
            t0,
            t0 + 0.3,
            Point3::new(x, 0.0, 0.0),
            Vec3::unit_x(),
            band(),
        ))
        .unwrap();
        assert!(
            got.is_empty(),
            "a root at the span's own end certified interior: {got:?}"
        );
    }

    /// **A crossing near a fast end is placed by its chord to the end,
    /// not by the conic's slowest speed.** The ellipse 0.01 × 10 runs at
    /// 10 m/rad at `θ = 0`, where its span starts. A plane crosses it
    /// `δ` in, `10·δ` metres from the end: past the band at `δ = 5·10⁻⁸`
    /// to `5·10⁻⁹`, a root inside the span; in the gap at `10⁻⁹`, an
    /// escalation; within the zero band at `5·10⁻¹¹`, the end's own.
    /// Red where the end is read as `(t − t₀)` metered at the smaller
    /// semi-axis, which puts every one of these at the end and drops the
    /// crossing the end's vertex does not hold.
    #[test]
    fn a_crossing_near_a_fast_end_is_placed_by_its_chord() {
        let e = Curve3::Ellipse(geom::Ellipse::new(
            geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
            0.01,
            10.0,
        ));
        let lane = |delta: f64| {
            let y = 10.0 * f64::sin(delta);
            roots_of(plane_crossing_lane(
                &e,
                0.0,
                0.3,
                Point3::new(0.0, y, 0.0),
                Vec3::unit_y(),
                band(),
            ))
        };
        for delta in [5e-8, 1e-8, 5e-9] {
            let got = lane(delta).unwrap();
            assert!(
                got.len() == 1 && (got[0] - delta).abs() < 1e-12,
                "a crossing {:e} m in: {got:?}",
                10.0 * delta
            );
        }
        assert!(lane(1e-9).is_err(), "a crossing in the gap escalates");
        assert_eq!(lane(5e-11).unwrap(), Vec::<f64>::new(), "at the end");
    }

    /// **Two roots apart by their chord are two, however close their
    /// parameters.** An ellipse 10 × 10⁻⁶ (major along `x`) crossed by the
    /// plane through its points at `φ ± α`, `φ` a few micro-radians off
    /// the major vertex, where it runs about `10·sin φ` m/rad against its
    /// 10⁻⁶ floor. The roots' chord is past the band (`Kε` = 10⁻⁸), so
    /// both are inserted; their parameter gap at the floor, `2α·10⁻⁶`, is
    /// inside the zero. Red where the pair is collapsed on that gap
    /// (`(r₁ − r₀)` at the inf speed), which drops a crossing. The window
    /// is narrow (the floor below `√(Rε/8K)`); these are two poses in it.
    #[test]
    fn two_roots_apart_by_their_chord_are_two() {
        let m = 1e-6_f64;
        let e = Curve3::Ellipse(geom::Ellipse::new(
            geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
            10.0,
            m,
        ));
        for (phi, alpha) in [(2e-6, 4e-4), (5e-6, 4.5e-4)] {
            let (p, q) = (e.eval(phi - alpha), e.eval(phi + alpha));
            let d = q - p;
            assert!(d.norm() > band().escalate() && 2.0 * alpha * m < band().zero());
            let n = Vec3::new(-d.y, d.x, 0.0).normalize();
            let got = roots_of(plane_crossing_lane(&e, -0.3, 0.3, p, n, band())).unwrap();
            assert!(
                got.len() == 2
                    && (got[0] - (phi - alpha)).abs() < 1e-9
                    && (got[1] - (phi + alpha)).abs() < 1e-9,
                "φ = {phi:e}, α = {alpha:e}: both roots, {got:?}"
            );
        }
    }

    /// The roots arm's payload; any other arm fails the row.
    fn roots_of<T: core::fmt::Debug>(
        m: PlaneCrossingLane<T>,
    ) -> Result<Vec<T>, geom_core::Indeterminate> {
        match m {
            PlaneCrossingLane::Conic(ConicPlaneMeet::Roots(r)) => {
                r.map_err(super::ConicRootFault::diag)
            }
            other => panic!("expected the roots arm, got {other:?}"),
        }
    }

    /// The lane against a plane through `origin` square to `normal`.
    fn on_plane<T: geom_core::Decide>(
        carrier: &Curve3<T>,
        t0: T,
        t1: T,
        (origin, normal): &(Point3<T>, Vec3<T>),
        band: Band,
    ) -> PlaneCrossingLane<T> {
        plane_crossing_lane(carrier, t0, t1, *origin, normal.normalize(), band)
    }

    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    /// Unit circle in the xy-plane about the origin (meter = 1, so
    /// parameter margins ARE meters).
    fn circle() -> Curve3<f64> {
        Curve3::circle(Point3::origin(), Vec3::unit_z(), 1.0, Vec3::unit_x())
    }

    fn plane_y(c: f64) -> (Point3<f64>, Vec3<f64>) {
        (Point3::new(0.0, c, 0.0), Vec3::unit_y())
    }

    /// `curve_face_pose_belly_graze`, all three arms: definitely-secant
    /// (two roots), definitely-missing (no roots), tangent within ε
    /// (one graze root), and in-band (typed escalation).
    #[test]
    fn belly_graze_trio() {
        use core::f64::consts::FRAC_PI_2;
        let c = circle();
        // Secant (margin R − |D| = 1, definite): the two roots of
        // sin θ = 0.5 land in the span, ascending.
        let roots = roots_of(on_plane(&c, 0.1, 6.0, &plane_y(0.5), band())).unwrap();
        assert_eq!(roots.len(), 2);
        assert!((roots[0] - core::f64::consts::FRAC_PI_6).abs() < 1e-12);
        assert!((roots[1] - (core::f64::consts::PI - core::f64::consts::FRAC_PI_6)).abs() < 1e-12);
        // Missing (margin −1, definite): no crossing at all.
        assert!(matches!(
            on_plane(&c, 0.1, 6.0, &plane_y(2.0), band()),
            PlaneCrossingLane::Conic(ConicPlaneMeet::Miss)
        ));
        // Tangent within ε, either side (margin 0, ±ε/2), above the
        // centre and below it: ONE graze root, at the extremum. Inside,
        // the residue's own roots lie √(2·5e-10) ≈ 3.2e-5 either side.
        for (c0, want) in [(1.0, FRAC_PI_2), (-1.0, 3.0 * FRAC_PI_2)] {
            for y in [c0, c0 - 5e-10, c0 + 5e-10] {
                let roots = roots_of(on_plane(&c, 0.1, 6.0, &plane_y(y), band())).unwrap();
                assert_eq!(roots.len(), 1, "y = {y}: one graze root");
                assert!(
                    (roots[0] - want).abs() < 1e-12,
                    "y = {y}: the graze root {} sits at the extremum {want}",
                    roots[0]
                );
            }
        }
        // In-band (margin −3ε): typed escalation, named.
        let diag = roots_of(on_plane(&c, 0.1, 6.0, &plane_y(1.0 + 3e-9), band())).unwrap_err();
        assert_eq!(diag.predicate, Some("curve_face_pose_belly_graze"));
    }

    /// Under `Interval` the graze root is the extremum to the phase's
    /// own width, with no acos of a ratio at ±1 widening it.
    #[test]
    fn an_interval_graze_root_is_as_tight_as_its_phase() {
        use geom_core::{Bounds, Interval, Real};
        let ex = Interval::from_f64;
        let c = Curve3::circle(
            Point3::new(ex(0.0), ex(0.0), ex(0.0)),
            Vec3::new(ex(0.0), ex(0.0), ex(1.0)),
            ex(1.0),
            Vec3::new(ex(1.0), ex(0.0), ex(0.0)),
        );
        for y in [1.0, 1.0 - 5e-10] {
            let plane = (
                Point3::new(ex(0.0), ex(y), ex(0.0)),
                Vec3::new(ex(0.0), ex(1.0), ex(0.0)),
            );
            let roots = roots_of(on_plane(&c, ex(0.1), ex(6.0), &plane, band())).unwrap();
            assert_eq!(roots.len(), 1, "y = {y}: one graze root");
            let r = roots[0];
            assert!(
                r.lo() <= core::f64::consts::FRAC_PI_2
                    && core::f64::consts::FRAC_PI_2 <= r.hi()
                    && r.hi() - r.lo() <= 1e-14,
                "y = {y}: the graze root [{}, {}] encloses π/2 tightly",
                r.lo(),
                r.hi()
            );
        }
    }

    /// A root against the span's ends ([`super::span_place`]), all three
    /// arms: definitely interior (returned), exactly at an endpoint
    /// (skipped — the vertex sweep owns it), and in-band of an endpoint
    /// (typed escalation, on the end's chord).
    #[test]
    fn crossing_root_trio() {
        let c = circle();
        // Roots of sin θ = 0 are θ ∈ {0, π}: with span [0, 2] the θ = 0
        // root sits EXACTLY at the endpoint (skipped, Zero arm) and the
        // θ = π root is definitely interior (returned).
        let roots = roots_of(on_plane(&c, 0.0, 2.0 + 2.0, &plane_y(0.0), band())).unwrap();
        assert_eq!(roots.len(), 1);
        assert!((roots[0] - core::f64::consts::PI).abs() < 1e-12);
        // Both roots definitely interior: span (−1, 4).
        let roots = roots_of(on_plane(&c, -1.0, 4.0, &plane_y(0.0), band())).unwrap();
        assert_eq!(roots.len(), 2);
        assert!(roots[0].abs() < 1e-12 || (roots[0] - core::f64::consts::PI).abs() < 1e-12);
        // In-band: a root 5e-9 (meters, meter = r = 1) inside the
        // span end — typed escalation, named.
        let diag = roots_of(on_plane(&c, -5e-9, 2.0, &plane_y(0.0), band())).unwrap_err();
        assert_eq!(diag.predicate, Some("curve_pair_span_end_chord"));
    }

    /// `curve_face_pose_plane_parallel`'s `Zero` arm reads the conic's
    /// offset: a circle in the plane, where the belly graze cannot decide
    /// a miss, lies in it; a circle in a parallel plane clear of it is
    /// `Miss`, read before the gate.
    #[test]
    fn a_parallel_frame_reads_its_offset() {
        let c = circle();
        let plane_z = |z: f64| (Point3::new(0.3, -0.2, z), Vec3::unit_z());
        match on_plane(&c, 0.1, 6.0, &plane_z(0.0), band()) {
            PlaneCrossingLane::Conic(ConicPlaneMeet::InPlane) => {}
            other => panic!("in the plane: expected the in-plane arm, got {other:?}"),
        }
        // Wholly on one side, the conic misses before its tilt is read.
        assert!(
            matches!(
                on_plane(&c, 0.1, 6.0, &plane_z(2.0), band()),
                PlaneCrossingLane::Conic(ConicPlaneMeet::Miss)
            ),
            "a parallel conic off the plane misses"
        );
    }

    /// **Each carrier kind lands on its own lane**: a line on the
    /// endpoint lane, a circle and an ellipse on the root lane, and a
    /// spiric and a spline on none. The answer is keyed on the kind
    /// alone, whatever the plane: the spiric here never meets `y = ½`
    /// (its `y ≥ √(ρ² − ¼) > 2`), and it is still `Unlaned`, because a
    /// curved carrier answered `Line` anywhere would have its crossings
    /// read off its endpoints.
    #[test]
    fn each_carrier_kind_lands_on_its_own_lane() {
        let plane = plane_y(0.5);
        for (carrier, want) in [
            (
                Curve3::line(Point3::origin(), geom_core::UnitVec3::exact_y()),
                "line",
            ),
            (circle(), "conic"),
            (
                Curve3::Ellipse(geom::Ellipse::new(
                    geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
                    2.0,
                    1.0,
                )),
                "conic",
            ),
            (
                Curve3::Spiric {
                    frame: geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
                    major_radius: 2.0,
                    minor_radius: 1.0,
                    offset: 0.5,
                },
                "unlaned",
            ),
            (
                Curve3::Nurbs(std::sync::Arc::new(
                    geom::NurbsCurve3::new(
                        geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1)
                            .unwrap(),
                        vec![Point3::origin(), Point3::new(0.0, 1.0, 0.0)],
                        vec![1.0; 2],
                    )
                    .unwrap(),
                )),
                "unlaned",
            ),
        ] {
            let got = match on_plane(&carrier, 0.0, 1.0, &plane, band()) {
                PlaneCrossingLane::Line => "line",
                PlaneCrossingLane::Conic(_) => "conic",
                PlaneCrossingLane::Unlaned => "unlaned",
            };
            assert_eq!(got, want, "{carrier:?}");
        }
    }

    /// **A root on the span's start keeps its width, at `Interval`**
    /// (issue 1191).
    ///
    /// A candidate root sitting ON the span's start is the ordinary
    /// case — a re-run split fragment meets the full circle at its own
    /// start vertex — and at `Interval` its enclosure straddles `t₀`.
    /// The rename about the middle comes back at the width of its input,
    /// so the interiority verdict is reached.
    ///
    /// Consults no tolerance: the two widths are widths, and the band
    /// is only what the site's own margins need in order to run.
    #[test]
    fn a_root_on_the_span_start_reduces_at_input_width_under_the_midpoint_anchor() {
        use geom_core::{Bounds, Interval, Real};

        let ex = Interval::from_f64;
        let c = Curve3::circle(
            Point3::new(ex(0.0), ex(0.0), ex(0.0)),
            Vec3::new(ex(0.0), ex(0.0), ex(1.0)),
            ex(1.0),
            Vec3::new(ex(1.0), ex(0.0), ex(0.0)),
        );
        let plane = (
            Point3::new(ex(0.0), ex(0.0), ex(0.0)),
            Vec3::new(ex(0.0), ex(1.0), ex(0.0)),
        );
        // The span is the upper semicircle; the plane's two crossings
        // are its own endpoints.
        let (t0, t1) = (ex(0.0), ex(core::f64::consts::PI));
        let roots = roots_of(on_plane(&c, t0, t1, &plane, band()))
            .expect("the endpoint roots classify — no anchor straddles them");
        assert!(
            roots.is_empty(),
            "both roots are the span's own endpoints and belong to the vertex sweep"
        );

        // The rename keeps a hairline box about `t₀` a hairline, here
        // and about the middle's antipode alike.
        let tau = Interval::tau();
        let mid = (t0 + t1) * ex(0.5);
        for cand in [
            Interval::from_bounds(-1e-15, 1e-15),
            mid + Interval::pi() + Interval::from_bounds(-1e-15, 1e-15),
        ] {
            let centred = geom_core::period::Period::new(tau).near(cand, mid);
            assert!(
                centred.hi() - centred.lo() <= 1e-9,
                "the rename widened: [{}, {}]",
                centred.lo(),
                centred.hi()
            );
        }
    }

    /// The plane through `o` of normal `n`, as a surface.
    fn plane_at((o, n): (Point3<f64>, Vec3<f64>)) -> geom::Surface<f64> {
        geom::Surface::Plane {
            frame: geom::Frame::new(o, n, n.orthonormal_basis().0),
        }
    }

    /// **The crossing reading reports a root at an end, where the lane
    /// drops it.** The upper unit semicircle against the plane `y = 0`
    /// meets it at both ends: the lane hands both to its callers' vertex
    /// sweep (no roots); [`crate::curve_face::cross()`] reports each at
    /// its end. Read endless, the whole turn's two roots are interior, the
    /// one at the turn's start included.
    #[test]
    fn a_crossing_reports_its_end_roots_and_an_endless_turn_has_none() {
        use crate::curve_face::{OnSpan, Pose, cross};
        use core::f64::consts::PI;
        let c = circle();
        let (o, n) = plane_y(0.0);
        assert!(
            roots_of(plane_crossing_lane(&c, 0.0, PI, o, n, band()))
                .unwrap()
                .is_empty(),
            "the lane drops roots at the span's ends"
        );
        let ended = cross(&Span::new(&c, (0.0, PI)), &plane_at((o, n)), band()).unwrap();
        assert!(
            ended.meetings.len() == 2
                && ended
                    .meetings
                    .iter()
                    .all(|m| matches!(m.on_span, OnSpan::End(_))),
            "ended: both end roots reported, each at an end: {ended:?}"
        );
        let endless = cross(
            &Span::new(&c, (0.0, 2.0 * PI)).endless(),
            &plane_at((o, n)),
            band(),
        )
        .unwrap();
        let at: Vec<f64> = endless
            .meetings
            .iter()
            .filter_map(|m| match m.on_span {
                OnSpan::Inside { t } => Some(t),
                OnSpan::End(_) => None,
            })
            .collect();
        assert!(
            at.len() == 2
                && endless.meetings.len() == 2
                && at.iter().any(|t| (t - PI).abs() < 1e-12)
                && at
                    .iter()
                    .any(|t| t.abs() < 1e-12 || (t - 2.0 * PI).abs() < 1e-12),
            "endless: the turn's roots at 0 and π, none at an end: {endless:?}"
        );
        let touch = cross(&Span::new(&c, (0.0, PI)), &plane_at(plane_y(1.0)), band()).unwrap();
        assert!(
            matches!(touch.pose, Pose::Tangent(_)),
            "a graze inside the span touches: {touch:?}"
        );
    }

    /// **A line's root is read in closed form**: the unit line along `x`
    /// over `[0, 1]` against `x = 0.3` crosses inside, against `x = 1`
    /// at its end, against `x = 2` nowhere; a plane containing its
    /// direction off it is `Apart`, and one containing the line within the
    /// band `Lies`.
    #[test]
    fn a_line_crosses_a_plane_at_its_affine_root() {
        use crate::curve_face::{OnSpan, Pose, cross};
        let line = Curve3::line(Point3::origin(), geom_core::UnitVec3::exact_x());
        let span = Span::new(&line, (0.0, 1.0));
        let at = |x: f64| {
            cross(
                &span,
                &plane_at((Point3::new(x, 0.0, 0.0), Vec3::unit_x())),
                band(),
            )
            .unwrap()
        };
        let inside = at(0.3).meetings;
        assert!(
            inside.len() == 1
                && matches!(inside[0].on_span, OnSpan::Inside { t } if (t - 0.3).abs() < 1e-15),
            "inside: {inside:?}"
        );
        let end = at(1.0).meetings;
        assert!(
            end.len() == 1 && matches!(end[0].on_span, OnSpan::End(_)),
            "at the end: {end:?}"
        );
        let past = at(2.0).meetings;
        assert!(past.is_empty(), "past the span: {past:?}");
        let parallel = |y: f64| {
            cross(
                &span,
                &plane_at((Point3::new(0.0, y, 0.0), Vec3::unit_y())),
                band(),
            )
            .unwrap()
            .pose
        };
        let off = parallel(0.25);
        assert!(
            matches!(off, Pose::Apart),
            "parallel off the plane: {off:?}"
        );
        let on = parallel(0.25 * band().zero());
        assert!(matches!(on, Pose::Lies), "in the plane: {on:?}");
    }

    /// **A line clear of a plane is apart whatever its drift reads.** A
    /// line a unit off the plane, tilted so its drift over the run lies in
    /// the band's gap: the drift rung cannot call it parallel or not, but
    /// the offset at the middle less half the drift stands past the band,
    /// so the pose is `Apart`, never an escalation. Within the band of the
    /// plane, the same drift escalates on the axis-parallel rung.
    #[test]
    fn a_line_clear_of_a_plane_is_apart_whatever_its_drift_reads() {
        use crate::curve_face::{CrossWhy, Pose, PoseRefusal, PoseRung, cross};
        let band = Band::new(1e-9, 1e-8).unwrap();
        let drift = 0.5 * (band.zero() + band.escalate());
        let line = |z: f64| {
            Curve3::line(
                Point3::new(0.0, 0.0, z),
                geom_core::UnitVec3::squared(Vec3::new(1.0, 0.0, drift)),
            )
        };
        let plane = plane_at((Point3::origin(), Vec3::unit_z()));
        let clear = line(1.0);
        let got = cross(&Span::new(&clear, (0.0, 1.0)), &plane, band);
        assert!(
            matches!(&got, Ok(read) if matches!(read.pose, Pose::Apart)),
            "clear of the plane: {got:?}"
        );
        let near = line(0.0);
        let got = cross(&Span::new(&near, (0.0, 1.0)), &plane, band);
        assert!(
            matches!(
                &got,
                Err(refusal) if matches!(
                    refusal.windows[0].why,
                    CrossWhy::Pose(PoseRefusal::InBand { rung: PoseRung::AxisParallel, .. })
                )
            ),
            "within the band of the plane: {got:?}"
        );
    }

    /// **A graze just off the span's end is not a miss** (review of PR
    /// 6609, M1): a plane cutting the unit circle `z/2` deep, its true
    /// roots `δ = acos(1 − z/2)` either side of its extremum, the extremum
    /// `δ/2` before `t₀ = 0.3`, crosses the circle once inside
    /// `[0.3, 2.0]`. The belly reads it a graze and keeps only the
    /// extremum, which lies off the span; its reach at the band's
    /// escalation covers part of the span, which is a window, never a
    /// miss. Clear of the span by twice that reach it misses.
    #[test]
    fn a_graze_whose_roots_reach_the_span_is_not_a_miss() {
        use crate::curve_face::{Pose, cross};
        let band = Band::new(1e-9, 1e-8).unwrap();
        let z = band.zero();
        // The true roots' distance from the extremum: `1 − cos δ = z/2`.
        let root = (1.0 - 0.5 * z).acos();
        let circle = Curve3::circle(
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            1.0,
            Vec3::new(1.0, 0.0, 0.0),
        );
        let meet = |theta: f64| {
            let n = Vec3::new(theta.cos(), theta.sin(), 0.0);
            let on = Point3::origin() + n * (1.0 - 0.5 * z);
            cross(&Span::new(&circle, (0.3, 2.0)), &plane_at((on, n)), band)
        };
        let near = meet(0.3 - 0.5 * root);
        assert!(
            matches!(&near, Err(refusal) if !refusal.windows.is_empty()),
            "{near:?}"
        );
        let reach = core::f64::consts::FRAC_PI_2 * (2.0 * (band.escalate() + 0.5 * z)).sqrt();
        let clear = meet(0.3 - 2.0 * reach);
        assert!(
            matches!(&clear, Ok(read) if matches!(read.pose, Pose::Apart)),
            "{clear:?}"
        );
        // A whole turn has no ends: one starting just past the touch, its
        // seam within the band of it, reads the touch as interior.
        for gap in [3e-9, 5e-9] {
            let n = Vec3::new(0.4f64.cos(), 0.4f64.sin(), 0.0);
            let whole = cross(
                &Span::new(&circle, (0.4 + gap, 0.4 + gap + core::f64::consts::TAU)).endless(),
                &plane_at((Point3::origin() + n * (1.0 - 0.5 * z), n)),
                band,
            );
            assert!(
                matches!(&whole, Ok(read) if matches!(read.pose, Pose::Tangent(_)) && read.meetings.len() == 1),
                "a whole turn from {gap:e} past the touch: {whole:?}"
            );
        }
    }
}
