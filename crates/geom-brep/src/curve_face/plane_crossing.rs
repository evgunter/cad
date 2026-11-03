//! **The plane-crossing door** ([`super::Door::PlaneCrossing`]): where
//! a carrier's span crosses a plane, keyed on the carrier's kind
//! ([`plane_crossing_lane`]). A line's one root is its endpoints'; a
//! conic's are the roots of the plane's residual along it, read over the
//! span; a spiric or a spline has no closed form here.

use geom_core::k_stats::{NonzeroSign, decide, decide_nonzero};
use geom_core::{Band, Decide, Indeterminate, Margin, Sign};

use crate::recourse::{RefusedArm, RefusedSide, SizedDecision, SizedPass};

/// Where a crossing lands along its edge, in words: the subject every
/// decision on it states, wherever an edge is split at a crossing.
pub const CROSSING_INTERIOR: &str = "whether a crossing lands strictly inside its edge";

/// The lever every door that splits an edge at a crossing has (the
/// split, the blend, the Boolean): the geometry.
pub const CROSSING_LEVER: &str =
    "move the geometry so the crossing lands clearly away from the edge's ends";

/// What a crossing's interiority margin measures.
pub const CROSSING_SIZE: &str = "distance from the edge's end";

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
    /// crossings are the curve × face table's ring-composite arm's, which
    /// the boolean's planar lane reads; every other caller refuses it, or
    /// passes it only behind a certificate that its whole locus clears
    /// the plane.
    Unlaned,
}

/// What a conic carrier's span meets of a plane
/// ([`PlaneCrossingLane::Conic`]).
#[derive(Debug)]
pub enum ConicPlaneMeet<T> {
    /// The conic's plane is parallel to the query plane: the carrier
    /// lies wholly at `offset`, its centre's signed distance, so it is
    /// either IN the plane or never meets it. Which is the caller's
    /// decision.
    Parallel {
        /// The centre's signed distance from the plane.
        offset: T,
    },
    /// The carrier definitely never meets the plane.
    Miss,
    /// The roots interior to the span, ascending (possibly none), or
    /// the rung no verdict was reached on.
    Roots(Result<Vec<T>, ConicRootFault>),
}

/// A decision on where a crossing lands along its edge, where every
/// definite answer passes and only an undecided one refuses: the conic
/// root lane's and the Boolean's wall-root lane's. Its ending is the one
/// every door that splits an edge at a crossing states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(any(test, feature = "test-support"), derive(strum::EnumIter))]
pub enum CrossingDecision {
    /// Whether a crossing lands strictly inside its edge (inside, at an
    /// end, and outside all pass).
    OnEdge,
    /// Which of two crossings on an edge comes first (either order, or
    /// one crossing, all pass).
    Order,
}

impl CrossingDecision {
    /// What the decision decides, as a clause with no colon or dash of
    /// its own.
    #[must_use]
    pub const fn subject(self) -> &'static str {
        match self {
            Self::OnEdge => CROSSING_INTERIOR,
            Self::Order => "which of two crossings on an edge comes first",
        }
    }

    /// The decision's table row: a size the user may intend, passing on
    /// every definite sign.
    #[must_use]
    pub const fn sized(self) -> SizedDecision {
        let (lever, size) = match self {
            Self::OnEdge => (CROSSING_LEVER, CROSSING_SIZE),
            Self::Order => (
                "move the geometry so the two crossings on that edge lie clearly apart",
                "distance between the crossings",
            ),
        };
        SizedDecision {
            lever,
            size,
            passes: SizedPass::AnySign,
            side: RefusedSide::Lever,
            no_size: None,
        }
    }

    /// The one ending `diag`'s escalation carries, read at `at`.
    #[must_use]
    pub fn ending_of(self, diag: &Indeterminate, at: impl Into<crate::recourse::ReadAt>) -> String {
        self.sized().recourse(RefusedArm::Undecided(diag), at)
    }
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

/// The lane that finds where a carrier's span crosses a plane
/// ([`PlaneCrossingLane`]), the split lane's and the boolean sweep's
/// alike. On a **conic** it finds the crossing roots over the span: the
/// signed distance along the carrier is
/// the sinusoid `d(θ) = D + R·cos(θ − φ)` with `D = (center − q)·n̂`,
/// `R·cos φ = s_u·(û·n̂)`, `R·sin φ = s_v·(v̂·n̂)` (`s_u/s_v` the
/// semi-axes — `r/r` for a circle). Unlike a line, a conic edge can
/// cross the plane an EVEN number of times between same-side
/// endpoints (the belly case) or once beyond an ON endpoint — so
/// crossing detection is **root-based, endpoint-verdict-free**:
///
/// 1. `plane_crossing_belly_graze` — margin `R − |D|` (meters): Negative
///    ⇒ the carrier never meets the split plane — no crossing;
///    Positive ⇒ two distinct roots `φ ± acos(−D/R)`; Zero ⇒ the
///    plane grazes the carrier's extremum — ONE (double) root, `φ` or
///    `φ + π` as `plane_crossing_graze_side` decides D's sign, whose
///    insertion (if in-span) leaves a same-side ON contact for the
///    sector classification (the established graze net); in-band ⇒
///    typed escalation (F6).
/// 2. Each root, re-centred on the span's midpoint (retried on `t₀`
///    where that reading is indeterminate), is classified
///    against the span by `plane_crossing_crossing_root` — the two
///    margins `(t − t₀)·meter` and `(t₁ − t)·meter` (meters at the
///    conservative minor-semi-axis meter): both Positive ⇒ a genuine
///    interior crossing (returned for insertion); any Zero ⇒ the root
///    sits at an existing endpoint vertex (the ON-endpoint belly case
///    — the vertex sweep already recorded it, nothing to insert); any
///    Negative ⇒ outside the span; in-band ⇒ typed escalation (the
///    crossing grazes an edge end — an ill-conditioned operand/plane
///    pair).
/// 3. Two interior roots are ordered ascending through
///    `plane_crossing_root_order` (never a raw comparison); ε-close
///    roots collapse to one insertion (Zero) or escalate (in-band).
///
/// Downstream, `split_edge`'s own interiority trilean and child
/// certification re-verify every insertion — this lane proposes,
/// never silently commits.
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
            conic,
            t0,
            t1,
            plane_origin,
            plane_normal,
            band,
        )),
        (None, geom::Curve3::Line(_)) => PlaneCrossingLane::Line,
        (None, _) => PlaneCrossingLane::Unlaned,
    }
}

/// The conic arm of [`plane_crossing_lane`].
fn conic_plane_meet<T: Decide>(
    conic: &dyn geom::Conic<T>,
    t0: T,
    t1: T,
    plane_origin: geom_core::Point3<T>,
    plane_normal: geom_core::Vec3<T>,
    band: Band,
) -> ConicPlaneMeet<T> {
    let candidates = match conic_plane_candidates(conic, plane_origin, plane_normal, band) {
        Ok(candidates) => candidates,
        Err(meet) => return meet,
    };
    let (s_u, s_v) = conic.semi_axes();
    span_roots(candidates, t0, t1, s_u, s_v, band)
}

/// **The carrier half of [`conic_plane_meet`]**: where the whole conic
/// meets the plane, as carrier parameters (one for a graze, two for a
/// crossing), before any span is read. `Err` carries the answer that
/// needs no span — parallel, a miss, or the fault a rung escalated.
fn conic_plane_candidates<T: Decide>(
    conic: &dyn geom::Conic<T>,
    plane_origin: geom_core::Point3<T>,
    plane_normal: geom_core::Vec3<T>,
    band: Band,
) -> Result<[Option<T>; 2], ConicPlaneMeet<T>> {
    let geom::Frame {
        center,
        axis,
        u_ref,
    } = *conic.frame();
    let (s_u, s_v) = conic.semi_axes();
    let v_ref = axis.cross(u_ref);
    let d0 = (center - plane_origin).dot(plane_normal);
    let a = u_ref.dot(plane_normal) * s_u;
    let b = v_ref.dot(plane_normal) * s_v;
    // powi, NEVER a*a: both amplitudes straddle zero on near-parallel
    // frames (a rim circle against a perpendicular side plane), and a
    // plain interval product's spurious negative low end makes the
    // sqrt refuse — the M2 interval-square bug class, found live here when
    // the boolean's IDEALIZED sweep (M5 PR 9) first drove this lane
    // over distant conic×plane pairs under the Interval scalar (the
    // realized lane's boxes never examine them, so only the brute
    // path escalated: a strategy divergence, the exact thing the
    // differential suite exists to catch).
    let r = (a.powi(2) + b.powi(2)).sqrt();
    // 0. The PARALLEL-frame gate: a conic whose plane is parallel to
    // the query plane (both amplitudes zero) either never meets it or
    // lies wholly IN it, and has no roots to find; `offset` tells the
    // two apart. Without the gate the in-plane case reaches the graze
    // arm with a 0/0 phase. The in-band twin escalates (F6).
    match decide("plane_crossing_plane_parallel", Margin::of(r), band) {
        Ok(Sign::Zero) => return Err(ConicPlaneMeet::Parallel { offset: d0 }),
        Ok(Sign::Positive | Sign::Negative) => {}
        Err(diag) => {
            return Err(ConicPlaneMeet::Roots(Err(ConicRootFault::PlaneParallel(
                diag,
            ))));
        }
    }
    // 1. Does the sinusoid reach zero at all — and how many roots?
    let both_roots = match decide("plane_crossing_belly_graze", Margin::of(r - d0.abs()), band) {
        Ok(Sign::Negative) => return Err(ConicPlaneMeet::Miss),
        Ok(Sign::Positive) => true,
        // Graze: the double root, processed once (processing both
        // would split twice at coincident parameters and escalate on
        // the second interiority check — same refusal, worse site).
        Ok(Sign::Zero) => false,
        Err(diag) => return Err(ConicPlaneMeet::Roots(Err(ConicRootFault::BellyGraze(diag)))),
    };
    // The sinusoid's phase, branch-stabilized (M5 S13): `atan2`'s cut
    // sits on the negative-`a` axis, and an interval `b` that touches
    // zero there (a revolve-built frame's honest trig slop) explodes
    // the enclosure to a full period even though the ROOT SET —
    // everything downstream consumes phi mod τ — is unchanged. On the
    // definitely-negative-`a` frame the same phase is computed as
    // `atan2(−b, −a) + π` (identical roots mod τ, cut now on the
    // benign axis). The frame trilean is a computation choice between
    // two mathematically identical formulas, so its degenerate and
    // in-band arms keep the direct formula (a deterministic tie-break,
    // D9 — near a ≈ 0 the cut is far from both).
    let phi = match decide("plane_crossing_phase_frame", Margin::of(a), band) {
        Ok(Sign::Negative) => (T::zero() - b).atan2(T::zero() - a) + T::pi(),
        Ok(Sign::Positive | Sign::Zero) | Err(_) => b.atan2(a),
    };
    // A graze's one root is the sinusoid's extremum nearest the plane:
    // φ with the centre below the plane (D < 0), φ + π with it above. The
    // residue's own roots lie up to √(2ε/R) radians either side, far
    // outside ε.
    let candidates: [Option<T>; 2] = if both_roots {
        // The roots solve cos(θ − φ) = −D/R. Clamped acos (rounding can
        // push the ratio a hair outside ±1; min/max are Real lattice ops).
        let delta = ((T::zero() - d0) / r)
            .min(T::one())
            .max(T::zero() - T::one())
            .acos();
        [Some(phi + delta), Some(phi - delta)]
    } else {
        // |D| is within ε of R ≥ Kε, so it decides zero only when K ≤ 2
        // and the conic's reach is itself inside the band; the gate's
        // rejection carries that margin.
        match decide_nonzero("plane_crossing_graze_side", Margin::of(d0), band) {
            Ok(NonzeroSign::Negative) => [Some(phi), None],
            Ok(NonzeroSign::Positive) => [Some(phi + T::pi()), None],
            Err(diag) => {
                return Err(ConicPlaneMeet::Roots(Err(ConicRootFault::BellyGraze(diag))));
            }
        }
    };
    Ok(candidates)
}

/// The span half of [`conic_plane_meet`]: the candidates interior to
/// `[t0, t1]`, ascending.
fn span_roots<T: Decide>(
    candidates: [Option<T>; 2],
    t0: T,
    t1: T,
    s_u: T,
    s_v: T,
    band: Band,
) -> ConicPlaneMeet<T> {
    let tau = T::tau();
    // The conservative meter (radians → meters): the smaller semi-axis,
    // in either stored order. An INF bound on the speed by being the
    // smaller principal rate, which is what the interiority claim below
    // needs — a root this meter proves clear of an endpoint is clear of
    // it in metres.
    let meter = geom_core::InfSpeed::new(s_u.min(s_v));
    let mut roots: Vec<T> = Vec::with_capacity(2);
    let half = T::from_f64(0.5);
    let mid = (t0 + t1) * half;
    // Per-candidate interiority under TWO reduction anchors (M5 S13).
    // The verdict is about `c mod τ` against `[t₀, t₁]`, but any single
    // periodic-reduction anchor has one degenerate point where the
    // Interval floor straddles an integer and the enclosure explodes to
    // a full period. The two anchors are the two windows: the midpoint
    // one is `reduce_periodic_centred` about `mid`, the fallback is
    // `reduce_periodic` about `t₀`. Their bad points are DISTINCT for
    // every sub-period span, which is what makes the retry a different
    // representation rather than a second try at the same one — they
    // sit **(τ − span)/2 apart** on the circle, half the span's
    // COMPLEMENT. Not half a period: that is the span → 0
    // limit, and the separation SHRINKS as the span grows, reaching
    // zero only at a full period. So the retry has room exactly when
    // the span is well under τ, and is degenerate exactly where the
    // sentence below says it is.
    // Anchored at the span MIDPOINT the bad point is the
    // midpoint's antipode (a definitely-EXTERIOR root — the re-run of a
    // split fragment meets the full circle's opposite intersection
    // exactly there), anchored at t₀ it is t₀ itself (a
    // definitely-ON-endpoint root — the fragment's own start vertex,
    // already swept). The two bad points coincide only on a full-period
    // span (a closed edge's lone vertex on the plane), so an
    // indeterminate verdict under one anchor retries the other — the
    // SAME margin in a non-degenerate representation, never a second
    // tolerance — and only a double failure escalates (F6).
    let verdict_at = |t: T| -> Result<(bool, T), Indeterminate> {
        for margin in [
            Margin::metered(t - t0, meter),
            Margin::metered(t1 - t, meter),
        ] {
            match decide("plane_crossing_crossing_root", margin, band) {
                Ok(Sign::Positive) => {}
                // At an endpoint vertex (already swept) or outside
                // the span: nothing to insert for this root.
                Ok(Sign::Zero | Sign::Negative) => return Ok((false, t)),
                Err(diag) => return Err(diag),
            }
        }
        Ok((true, t))
    };
    // The midpoint anchor is sound at every scalar by `geom::on_span_branch`'s
    // window re-centring argument: `verdict_at` decides a root at either
    // end of a closed edge's full-period span alike (not interior), and
    // reads it only against the span's ends, never as an identity. The
    // `t₀` retry below serves its straddle.
    for c in candidates.into_iter().flatten() {
        let centred = crate::half_angle::near_midpoint(mid, c);
        let (interior, t) = match verdict_at(centred) {
            Ok(v) => v,
            Err(first) => {
                let anchored = t0 + (c - t0).reduce_periodic(tau);
                match verdict_at(anchored) {
                    Ok(v) => v,
                    Err(_) => {
                        return ConicPlaneMeet::Roots(Err(ConicRootFault::CrossingInterior(first)));
                    }
                }
            }
        };
        if interior {
            roots.push(t);
        }
    }
    // Ascending insertion order, decided through the trilean door
    // (never a raw scalar comparison).
    if roots.len() == 2 {
        match decide(
            "plane_crossing_root_order",
            Margin::metered(roots[1] - roots[0], meter),
            band,
        ) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Negative) => roots.swap(0, 1),
            // Coincident-but-both-interior roots: a graze the graze
            // margin called Positive — insert once.
            Ok(Sign::Zero) => {
                roots.truncate(1);
            }
            Err(diag) => return ConicPlaneMeet::Roots(Err(ConicRootFault::RootOrder(diag))),
        }
    }
    ConicPlaneMeet::Roots(Ok(roots))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    //! DIRECT trilean rows for the conic crossing lane (M5 PR 5 fix
    //! pass, review M3): `plane_crossing_belly_graze` and
    //! `plane_crossing_crossing_root` each get their definite /
    //! exactly-degenerate / in-band arms against a pure band (the
    //! geom-core test discipline: never `Band::linear` in a lib test).

    use geom::Curve3;
    use geom_core::{Band, Point3, Vec3};

    use super::{ConicPlaneMeet, PlaneCrossingLane, plane_crossing_lane};

    /// **A crossing's gap from an end is metered at the smaller semi-axis
    /// MAGNITUDE, in either stored order.** An ellipse stored swapped —
    /// `major = 0.01` along `u_ref`, `minor = 10` — runs at 0.01 m/rad at
    /// its `θ = π/2` vertex. A plane crosses it `δ = 5e-8` rad past that
    /// vertex, an arc of 5e-10 m: inside the zero band, so the crossing
    /// is the span's own end, not an interior root. Metered at the
    /// stored `minor` (10 m/rad) the gap read 5e-7 m and the root was
    /// certified interior, a split 5e-10 m from a vertex.
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

    /// `plane_crossing_belly_graze`, all three arms: definitely-secant
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
        assert_eq!(diag.predicate, Some("plane_crossing_belly_graze"));
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

    /// `plane_crossing_crossing_root`, all three arms: definitely
    /// interior (returned), exactly at an endpoint (skipped — the
    /// vertex sweep owns it), and in-band of an endpoint (typed
    /// escalation).
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
        assert_eq!(diag.predicate, Some("plane_crossing_crossing_root"));
    }

    /// `plane_crossing_plane_parallel`'s `Zero` arm hands back the conic's
    /// offset, so a caller can tell a circle IN the plane from one in a
    /// parallel plane; a non-parallel miss is `Miss`, never this arm.
    #[test]
    fn a_parallel_frame_reports_its_offset() {
        let c = circle();
        let plane_z = |z: f64| (Point3::new(0.3, -0.2, z), Vec3::unit_z());
        for (z, want) in [(0.0, 0.0), (2.0, -2.0)] {
            match on_plane(&c, 0.1, 6.0, &plane_z(z), band()) {
                PlaneCrossingLane::Conic(ConicPlaneMeet::Parallel { offset }) => {
                    assert!((offset - want).abs() < 1e-12, "offset at z = {z}: {offset}");
                }
                other => panic!("z = {z}: expected the parallel arm, got {other:?}"),
            }
        }
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
            (Curve3::line(Point3::origin(), Vec3::unit_y()), "line"),
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
                    center: Point3::origin(),
                    axis: Vec3::unit_z(),
                    u_ref: Vec3::unit_x(),
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

    /// **The midpoint anchor's straddle row, at `Interval`** (issue
    /// 1191). Nothing in the shipped suites drives either anchor into a
    /// straddle, so this row does it on the site's own numbers.
    ///
    /// A candidate root sitting ON the span's start is the ordinary
    /// case — a re-run split fragment meets the full circle at its own
    /// start vertex — and at `Interval` its enclosure straddles `t₀`.
    /// The MIDPOINT anchor's jump is half a period away from there, so
    /// the recentred parameter comes back at the width of its input and
    /// the interiority verdict is reached; the `t₀` anchor's jump is
    /// exactly on it, which is why it is the FALLBACK and not the first
    /// try. The row measures both reductions on the same box so the
    /// design's premise — two jumps, half a period apart — is pinned
    /// rather than described.
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

        // The premise, measured: a hairline box about `t₀` is a
        // hairline under the midpoint anchor and a whole period under
        // the `t₀` anchor.
        //
        // DISPOSITION: this half re-derives both anchorings inline and
        // so pins the two WINDOWS rather than the site. The site's own
        // pin is the `on_plane` call above, which reds if the
        // anchor order changes; this half says why that order is the
        // right one.
        let tau = Interval::tau();
        let mid = (t0 + t1) * ex(0.5);
        let cand = Interval::from_bounds(-1e-15, 1e-15);
        let centred = mid + (cand - mid).reduce_periodic_centred(tau);
        let anchored = t0 + (cand - t0).reduce_periodic(tau);
        assert!(
            centred.hi() - centred.lo() <= 1e-9,
            "the midpoint anchor widened: [{}, {}]",
            centred.lo(),
            centred.hi()
        );
        assert!(
            anchored.hi() - anchored.lo() >= core::f64::consts::TAU,
            "the t0 anchor is supposed to be the wide one here; it gave [{}, {}]",
            anchored.lo(),
            anchored.hi()
        );
    }
}
