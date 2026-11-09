//! **The plane-crossing door** ([`super::Door::PlaneCrossing`]): where
//! a carrier's span crosses a plane, keyed on the carrier's kind
//! ([`plane_crossing_lane`]). A line's one root is its endpoints'; a
//! conic's are the roots of the plane's residual along it, read over the
//! span; a spiric or a spline has no closed form here.

use geom_core::k_stats::{NonzeroSign, decide, decide_nonzero};
use geom_core::period::Period;
use geom_core::{Band, Decide, Indeterminate, Margin, Sign};

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
/// ([`PlaneCrossingLane`]). On a **conic** it finds the crossing roots over the span: the
/// signed distance along the carrier is
/// the sinusoid `d(θ) = D + R·cos(θ − φ)` with `D = (center − q)·n̂`,
/// `R·cos φ = s_u·(û·n̂)`, `R·sin φ = s_v·(v̂·n̂)` (`s_u/s_v` the
/// semi-axes — `r/r` for a circle). Unlike a line, a conic edge can
/// cross the plane an EVEN number of times between same-side
/// endpoints (the belly case) or once beyond an ON endpoint — so
/// crossing detection is **root-based, endpoint-verdict-free**:
///
/// 1. `plane_crossing_belly_graze` — margin `R − |D|` (meters): Negative
///    ⇒ the carrier never meets the plane — no crossing, read before the
///    parallel gate, so a conic wholly on one side misses whatever its
///    tilt reads;
///    Positive ⇒ two distinct roots `φ ± acos(−D/R)`; Zero ⇒ the
///    plane grazes the carrier's extremum — ONE (double) root, `φ` or
///    `φ + π` as `plane_crossing_graze_side` decides D's sign, whose
///    insertion (if in-span) leaves a same-side ON contact for the
///    sector classification (the established graze net); in-band ⇒
///    typed escalation (F6).
/// 2. Each root, re-centred on the span's midpoint (retried on `t₀`
///    where that reading is indeterminate), is classified
///    against the span by `plane_crossing_root_in_span` — the two
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
/// **What a caller may assume of a root**: it is the closed form's root
/// at the scalar's rounding, decided strictly inside `[t0, t1]` (its
/// distance from each end, metered at the smaller semi-axis, decided
/// positive), and the roots are ascending and decided apart. It is not a point certified on the
/// plane, and the door certifies nothing about the edge it lies on: a
/// caller that splits an edge at one decides that split's own
/// interiority and certifies the pieces itself.
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
    let near = geom::mid_param(t0, t1);
    let candidates = match conic_plane_candidates(conic, plane_origin, plane_normal, near, band) {
        Ok((candidates, _)) => candidates,
        Err(meet) => return meet,
    };
    let (s_u, s_v) = conic.semi_axes();
    span_roots(candidates, t0, t1, s_u, s_v, band)
}

/// **The carrier half of [`conic_plane_meet`]**: where the whole conic
/// meets the plane, as carrier parameters (one for a graze, two for a
/// crossing), before any span is read, named within half a turn of
/// `near` or a little past it, and whether it is a graze. `Err` carries
/// the answer that needs no span — parallel, a miss, or the fault a rung
/// escalated. The belly graze is read before the parallel gate: a conic
/// wholly on one side of the plane is a miss whatever its tilt reads, the
/// parallel argument before the swing.
fn conic_plane_candidates<T: Decide>(
    conic: &dyn geom::Conic<T>,
    plane_origin: geom_core::Point3<T>,
    plane_normal: geom_core::Vec3<T>,
    near: T,
    band: Band,
) -> Result<([Option<T>; 2], bool), ConicPlaneMeet<T>> {
    // The conic's height above the plane, `d0 + a·cos θ + b·sin θ`: the
    // first harmonic a curve pair's transverse reading is built on
    // ([`crate::curve_pair::pose`]), one generator for both.
    let (d0, a, b) = crate::curve_pair::pose::height(conic, plane_origin, plane_normal);
    // powi, NEVER a*a: both amplitudes straddle zero on near-parallel
    // frames (a rim circle against a perpendicular side plane), and a
    // plain interval product's spurious negative low end makes the
    // sqrt refuse (the interval-square class).
    let r = (a.powi(2) + b.powi(2)).sqrt();
    // The miss first: a conic wholly on one side of the plane never meets
    // it, whatever its tilt reads (the parallel argument before the
    // swing). Its other arms are read at step 1.
    let belly = decide("plane_crossing_belly_graze", Margin::of(r - d0.abs()), band);
    if let Ok(Sign::Negative) = belly {
        return Err(ConicPlaneMeet::Miss);
    }
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
    // 1. How many roots, where the sinusoid reaches zero?
    let both_roots = match belly {
        Ok(Sign::Negative) => return Err(ConicPlaneMeet::Miss),
        Ok(Sign::Positive) => true,
        // Graze: the double root, processed once (processing both
        // would split twice at coincident parameters and escalate on
        // the second interiority check — same refusal, worse site).
        Ok(Sign::Zero) => false,
        Err(diag) => return Err(ConicPlaneMeet::Roots(Err(ConicRootFault::BellyGraze(diag)))),
    };
    // The sinusoid's phase within half a turn of `near`, read clear of
    // `atan2`'s cut ([`geom::Azimuth::near`]): an interval `b` touching
    // zero on the negative-`a` axis (a revolve-built frame's honest trig
    // slop) stays narrow rather than spanning a turn.
    let phi = geom::Azimuth::of(b, a).near(near);
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
    Ok((candidates, !both_roots))
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
    let meter = conic_meter(s_u, s_v);
    let mut roots: Vec<T> = Vec::with_capacity(2);
    // A root at an endpoint vertex (already swept) or outside the span
    // has nothing to insert; an undecided one is the root's own and
    // escalates (F6).
    for t in near_span(candidates, (t0, t1)) {
        match span_place(t, (t0, t1), meter, band) {
            Some(RootPlace::Inside) => roots.push(t),
            Some(RootPlace::AtEnd) | None => {}
            Some(RootPlace::Undecided(diag)) => {
                return ConicPlaneMeet::Roots(Err(ConicRootFault::CrossingInterior(diag)));
            }
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

/// Whether a span has ends ([`span_plane_meet`]): an edge's or a bounded
/// leg's, or a whole turn's, whose first and last parameters are one
/// point of its interior.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpanEnds {
    /// Both ends are ends.
    Ended,
    /// A whole turn of a periodic carrier, read over `[t0, t1]` with no
    /// end.
    Endless,
}

/// A root of a span against a plane ([`span_plane_meet`]).
#[derive(Clone, Copy, Debug)]
pub struct SpanRoot<T> {
    /// The carrier's parameter, within half a turn of the span's middle
    /// on a periodic carrier.
    pub t: T,
    /// Where it lies against the span's ends, metered as
    /// [`plane_crossing_lane`] meters it.
    pub place: RootPlace,
}

/// Where a root lies against its span's ends ([`SpanRoot::place`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RootPlace {
    /// Inside the span, clear of both ends.
    Inside,
    /// Within the band of an end.
    AtEnd,
    /// In band of an end: the caller decides whether the root matters
    /// before it refuses on it (a root of an edge past a leg's own end
    /// is no meeting of the two).
    Undecided(Indeterminate),
}

/// What a span meets of a plane ([`span_plane_meet`]).
#[derive(Debug)]
pub enum SpanPlaneMeet<T> {
    /// The carrier is parallel to the plane, wholly at `offset`, its
    /// signed distance from it (a conic's centre's, a line's at the
    /// span's middle): in the plane or never meeting it, the caller's to
    /// decide.
    Parallel {
        /// The carrier's signed distance from the plane.
        offset: T,
    },
    /// The plane grazes the carrier at a point of the span: one double
    /// root, no crossing.
    Touch,
    /// The roots on the span (possibly none), each with where it lies
    /// against the span's ends; unordered.
    Roots(Vec<SpanRoot<T>>),
}

/// **Where a span crosses a plane, ends included**: the door's lane
/// ([`plane_crossing_lane`]) read for a caller that places what it finds
/// itself. It shares the lane's carrier half (the belly graze, the
/// parallel gate and the phase) and its span read ([`span_place`]), and
/// differs from it in four ways:
///
/// - a root within the band of an end of an [`SpanEnds::Ended`] span is
///   reported [`RootPlace::AtEnd`], where the lane drops it (its callers'
///   vertex sweep owns it), and one in band of an end is reported
///   [`RootPlace::Undecided`], where the lane refuses the whole span;
/// - an [`SpanEnds::Endless`] span reads every root of the whole turn,
///   none at an end, each named within half a turn of the span's middle;
/// - a line's one root is read here, in closed form: its signed distance
///   `d(t) = (o − q)·n̂ + t·(d̂·n̂)` is affine, parallel where the slope
///   times the span's length is in the zero band
///   (`plane_crossing_line_parallel`), and otherwise the root
///   `−d(0)/(d̂·n̂)` is read against the span as a conic's is, at the
///   line's speed;
/// - a graze is [`SpanPlaneMeet::Touch`] where it touches the span.
///
/// # Errors
///
/// The rung that escalated ([`ConicRootFault`]); `Ok(None)` for a carrier
/// with no closed form (the lane's [`PlaneCrossingLane::Unlaned`]).
pub fn span_plane_meet<T: Decide>(
    carrier: &geom::Curve3<T>,
    (t0, t1): (T, T),
    ends: SpanEnds,
    plane_origin: geom_core::Point3<T>,
    plane_normal: geom_core::Vec3<T>,
    band: Band,
) -> Result<Option<SpanPlaneMeet<T>>, ConicRootFault> {
    let found = match (carrier.conic(), carrier) {
        (Some(conic), _) => {
            let (candidates, graze) = match conic_plane_candidates(
                conic,
                plane_origin,
                plane_normal,
                geom::mid_param(t0, t1),
                band,
            ) {
                Ok(found) => found,
                Err(ConicPlaneMeet::Parallel { offset }) => {
                    return Ok(Some(SpanPlaneMeet::Parallel { offset }));
                }
                Err(ConicPlaneMeet::Miss) => return Ok(Some(SpanPlaneMeet::Roots(Vec::new()))),
                Err(ConicPlaneMeet::Roots(Err(fault))) => return Err(fault),
                Err(ConicPlaneMeet::Roots(Ok(_))) => {
                    unreachable!("the carrier half answers no roots")
                }
            };
            let (s_u, s_v) = conic.semi_axes();
            let meter = conic_meter(s_u, s_v);
            let mut roots = Vec::with_capacity(2);
            for t in near_span(candidates, (t0, t1)) {
                let place = match ends {
                    SpanEnds::Endless => Some(RootPlace::Inside),
                    SpanEnds::Ended => span_place(t, (t0, t1), meter, band),
                };
                if let Some(place) = place {
                    roots.push(SpanRoot { t, place });
                }
            }
            if graze {
                return Ok(Some(if roots.is_empty() {
                    SpanPlaneMeet::Roots(roots)
                } else {
                    SpanPlaneMeet::Touch
                }));
            }
            roots
        }
        (None, geom::Curve3::Line(line)) => {
            let slope = line.dir.dot(plane_normal);
            let d0 = geom::plane_offset(plane_origin, plane_normal, line.origin);
            match decide(
                "plane_crossing_line_parallel",
                Margin::of(slope * (t1 - t0)),
                band,
            ) {
                Ok(Sign::Zero) => {
                    let offset = d0 + slope * geom::mid_param(t0, t1);
                    return Ok(Some(SpanPlaneMeet::Parallel { offset }));
                }
                Ok(Sign::Positive | Sign::Negative) => {}
                Err(diag) => return Err(ConicRootFault::PlaneParallel(diag)),
            }
            let t = (T::zero() - d0) / slope;
            let meter = geom_core::InfSpeed::new(line.dir.norm());
            span_place(t, (t0, t1), meter, band)
                .map(|place| SpanRoot { t, place })
                .into_iter()
                .collect()
        }
        (None, _) => return Ok(None),
    };
    Ok(Some(SpanPlaneMeet::Roots(found)))
}

/// **A conic's span meter** (radians → metres): the smaller semi-axis,
/// in either stored order. An INF bound on the speed by being the
/// smaller principal rate, which is what an interiority claim needs: a
/// root this meter proves clear of an end is clear of it in metres.
fn conic_meter<T: Decide>(s_u: T, s_v: T) -> geom_core::InfSpeed<T> {
    geom_core::InfSpeed::new(s_u.min(s_v))
}

/// **The carrier's roots named on the span**: each within half a turn
/// of its middle, by `Period::near`'s span argument, so a root at either
/// end of a closed edge's full-period span reads alike against both
/// ends. The rename keeps an enclosure at its input's width everywhere,
/// the middle's antipode included.
fn near_span<T: Decide>(candidates: [Option<T>; 2], (t0, t1): (T, T)) -> impl Iterator<Item = T> {
    let mid = geom::mid_param(t0, t1);
    candidates
        .into_iter()
        .flatten()
        .map(move |c| Period::new(T::tau()).near(c, mid))
}

/// **A parameter against a span's ends**, on the lane's own row
/// (`plane_crossing_root_in_span`, the two margins `(t − t₀)·meter` and
/// `(t₁ − t)·meter`, read in that order and no further than the first
/// that is not positive): where it lies ([`RootPlace`]), or `None` past
/// an end.
pub fn span_place<T: Decide>(
    t: T,
    (t0, t1): (T, T),
    meter: geom_core::InfSpeed<T>,
    band: Band,
) -> Option<RootPlace> {
    for margin in [
        Margin::metered(t - t0, meter),
        Margin::metered(t1 - t, meter),
    ] {
        match decide("plane_crossing_root_in_span", margin, band) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero) => return Some(RootPlace::AtEnd),
            Ok(Sign::Negative) => return None,
            Err(diag) => return Some(RootPlace::Undecided(diag)),
        }
    }
    Some(RootPlace::Inside)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    //! DIRECT trilean rows for the conic crossing lane (M5 PR 5 fix
    //! pass, review M3): `plane_crossing_belly_graze` and
    //! `plane_crossing_root_in_span` each get their definite /
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

    /// `plane_crossing_root_in_span`, all three arms: definitely
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
        assert_eq!(diag.predicate, Some("plane_crossing_root_in_span"));
    }

    /// `plane_crossing_plane_parallel`'s `Zero` arm hands back the conic's
    /// offset for a circle in the plane, where the belly graze cannot
    /// decide a miss; a circle in a parallel plane clear of it is `Miss`,
    /// read before the gate.
    #[test]
    fn a_parallel_frame_reports_its_offset() {
        let c = circle();
        let plane_z = |z: f64| (Point3::new(0.3, -0.2, z), Vec3::unit_z());
        match on_plane(&c, 0.1, 6.0, &plane_z(0.0), band()) {
            PlaneCrossingLane::Conic(ConicPlaneMeet::Parallel { offset }) => {
                assert!(offset.abs() < 1e-12, "offset in the plane: {offset}");
            }
            other => panic!("in the plane: expected the parallel arm, got {other:?}"),
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

    /// **The span reading reports a root at an end, where the lane drops
    /// it.** The upper unit semicircle against the plane `y = 0` meets it
    /// at both ends: the lane hands both to its callers' vertex sweep (no
    /// roots); [`super::span_plane_meet`] reports each `at_end`. Read
    /// endless, the whole turn's two roots are interior, the one at the
    /// turn's start included.
    #[test]
    fn a_span_reading_reports_its_end_roots_and_an_endless_turn_has_none() {
        use super::{SpanEnds, SpanPlaneMeet, span_plane_meet};
        use core::f64::consts::PI;
        let c = circle();
        let (o, n) = plane_y(0.0);
        assert!(
            roots_of(plane_crossing_lane(&c, 0.0, PI, o, n, band()))
                .unwrap()
                .is_empty(),
            "the lane drops roots at the span's ends"
        );
        let Ok(Some(SpanPlaneMeet::Roots(ended))) =
            span_plane_meet(&c, (0.0, PI), SpanEnds::Ended, o, n, band())
        else {
            panic!("ended: expected roots")
        };
        assert_eq!(ended.len(), 2, "ended: both end roots reported: {ended:?}");
        assert!(
            ended.iter().all(|r| r.place == super::RootPlace::AtEnd),
            "ended: each at an end: {ended:?}"
        );
        let Ok(Some(SpanPlaneMeet::Roots(endless))) =
            span_plane_meet(&c, (0.0, 2.0 * PI), SpanEnds::Endless, o, n, band())
        else {
            panic!("endless: expected roots")
        };
        let mut at: Vec<f64> = endless.iter().map(|r| r.t).collect();
        at.sort_by(f64::total_cmp);
        assert!(
            endless.iter().all(|r| r.place == super::RootPlace::Inside)
                && at.len() == 2
                && ((at[0] - PI).abs() < 1e-12)
                && (at[1].abs() < 1e-12 || (at[1] - 2.0 * PI).abs() < 1e-12),
            "endless: the turn's roots at 0 and π, none at an end: {endless:?}"
        );
        let (o, n) = plane_y(1.0);
        assert!(
            matches!(
                span_plane_meet(&c, (0.0, PI), SpanEnds::Ended, o, n, band()),
                Ok(Some(SpanPlaneMeet::Touch))
            ),
            "a graze inside the span touches"
        );
    }

    /// **A line's root is read in closed form**: the unit line along `x`
    /// over `[0, 1]` against `x = 0.3` crosses inside, against `x = 1`
    /// at its end, against `x = 2` nowhere; a plane containing its
    /// direction is parallel, at the line's offset.
    #[test]
    fn a_line_crosses_a_plane_at_its_affine_root() {
        use super::{SpanEnds, SpanPlaneMeet, span_plane_meet};
        let line = Curve3::line(Point3::origin(), Vec3::unit_x());
        let at = |x: f64| {
            span_plane_meet(
                &line,
                (0.0, 1.0),
                SpanEnds::Ended,
                Point3::new(x, 0.0, 0.0),
                Vec3::unit_x(),
                band(),
            )
        };
        let Ok(Some(SpanPlaneMeet::Roots(inside))) = at(0.3) else {
            panic!("inside: expected roots")
        };
        assert!(
            inside.len() == 1
                && (inside[0].t - 0.3).abs() < 1e-15
                && inside[0].place == super::RootPlace::Inside,
            "inside: {inside:?}"
        );
        let Ok(Some(SpanPlaneMeet::Roots(end))) = at(1.0) else {
            panic!("end: expected roots")
        };
        assert!(
            end.len() == 1 && end[0].place == super::RootPlace::AtEnd,
            "at the end: {end:?}"
        );
        let Ok(Some(SpanPlaneMeet::Roots(past))) = at(2.0) else {
            panic!("past: expected roots")
        };
        assert!(past.is_empty(), "past the span: {past:?}");
        let parallel = span_plane_meet(
            &line,
            (0.0, 1.0),
            SpanEnds::Ended,
            Point3::new(0.0, 0.25, 0.0),
            Vec3::unit_y(),
            band(),
        );
        assert!(
            matches!(parallel, Ok(Some(SpanPlaneMeet::Parallel { offset })) if (offset + 0.25).abs() < 1e-15),
            "parallel at its offset: {parallel:?}"
        );
    }
}
