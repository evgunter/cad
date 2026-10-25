//! Certified-conservative [`Aabb`] constructors for curve carriers
//! (C10, M5 PR 8): a box is a cache with a **containment contract** —
//! the carrier's true locus over the stated span lies inside it — and
//! every computation here errs outward only: by the certified
//! [`Interval`] arithmetic the conic doors run in, or, for the spiric,
//! by the charge [`crate::spiric_period_box`] states for its own
//! arithmetic.
//!
//! **Outward is the only sound direction; it is not a free one.** A
//! wider box is a weaker answer, not a cheaper one: a consumer that
//! reads non-overlap as its ANSWER loses that answer to width, and
//! which consumers read a box that way is not visible from here —
//! `topo` depends on `geom`, not the reverse. So these docs state the
//! containment contract and stop. What looseness costs is the reading
//! door's to state, per door, where the doors are.
//!
//! They live HERE, not in the `bvh` crate, so the tree stays below the
//! geometry crate (the SSI subdivision duty inside them must be able
//! to consume it — see `bvh`'s crate docs) and each constructor sits
//! next to the invariant it cites. [`conic_arc_aabb`] is the one door
//! a consumer boxing an edge of unknown conic kind reads; the two kind
//! doors under it are its arms. A conic arc's extent along an
//! ARBITRARY direction is [`conic_arc_extent_along`] (at the carrier's
//! scalar) and [`conic_arc_extent_along_bracketed`] (certified); the
//! box doors are the same closed form read along the world axes, so
//! the extremum has one body ([`ArcWindow`]).
//!
//! This is certified-box driver code, a **sole**-bound [`Bounds`] seam
//! under the 2026-07-29 amendment (geom-core `real.rs`, Bounds scope
//! rule): every scalar enters as the [`Interval`] of its
//! `[lo(), hi()]` bracket — save the spiric's, which
//! [`spiric_arc_aabb`] runs in its scalar's own arithmetic and
//! brackets once done — and poison (NaN) flows to the poison box,
//! which overlaps everything — the honest answer when no cheap
//! superset is known.
//!
//! **Not "allowlisted", which is what this said before.** The amendment
//! ratifies the box constructors to write the COMPOUND `Decide + Bounds`
//! form; these write the sole form, which needs no ratification, so
//! `scripts/gates/bounds-allowlist.sh` neither lists this file nor is
//! able to see it — a sole bracket bound is its planted must-not-fire
//! case. The rule covers this module; the gate does not.

use bvh::Aabb;
use geom_core::{Bounds, Interval, Point3, Real, Vec3};

use crate::curves::Curve3;
use crate::curves::nurbs::NurbsCurve3;
use crate::net;

/// A conic's frame as the extremum reads it: the centre, the plane
/// normal, the `t = 0` reference direction and the two semi-axes — a
/// circle is the ellipse with `semi_u = semi_v = radius`. Each
/// constructor refuses (`None`) every kind it does not name.
#[derive(Clone, Copy)]
struct ConicFrame<T: Real> {
    center: Point3<T>,
    axis: Vec3<T>,
    u_ref: Vec3<T>,
    semi_u: T,
    semi_v: T,
}

impl<T: Real> ConicFrame<T> {
    fn of(carrier: &Curve3<T>) -> Option<Self> {
        Self::circle(carrier).or_else(|| Self::ellipse(carrier))
    }

    fn circle(carrier: &Curve3<T>) -> Option<Self> {
        let Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } = *carrier
        else {
            return None;
        };
        Some(Self {
            center,
            axis,
            u_ref,
            semi_u: radius,
            semi_v: radius,
        })
    }

    fn ellipse(carrier: &Curve3<T>) -> Option<Self> {
        let Curve3::Ellipse {
            center,
            axis,
            semi_u,
            semi_v,
            u_ref,
        } = *carrier
        else {
            return None;
        };
        Some(Self {
            center,
            axis,
            u_ref,
            semi_u,
            semi_v,
        })
    }

    /// `axis × u_ref`, the `t = π/2` reference direction.
    fn v_ref(&self) -> Vec3<T> {
        self.axis.cross(self.u_ref)
    }
}

impl<T: Bounds> ConicFrame<T> {
    /// The frame with every scalar entered as the [`Interval`] of its
    /// bracket. The second reference direction is crossed AFTER the
    /// lift, so its rounding is the interval's and not `T`'s.
    fn lifted(self) -> ConicFrame<Interval> {
        let point = |p: Point3<T>| Point3::new(lift(p.x), lift(p.y), lift(p.z));
        let vector = |v: Vec3<T>| Vec3::new(lift(v.x), lift(v.y), lift(v.z));
        ConicFrame {
            center: point(self.center),
            axis: vector(self.axis),
            u_ref: vector(self.u_ref),
            semi_u: lift(self.semi_u),
            semi_v: lift(self.semi_v),
        }
    }
}

/// A bracket as the certified scalar's enclosure of it. The box doors
/// read only the result's bounds, never its decoration, so the
/// decoration this rebuild discards is not a channel anything here
/// decides on; a poisoned bracket (a NaN end) enters as NaI and leaves
/// as NaN bounds.
fn lift<T: Bounds>(x: T) -> Interval {
    Interval::from_bounds(x.lo(), x.hi())
}

/// **The extremum of an arc along one direction — the one home of it.**
///
/// Along any direction a conic arc's coordinate is a harmonic of its
/// parameter, `c + p·cos t + q·sin t`, with `p = semi_u·(û·n)` and
/// `q = semi_v·(v̂·n)`. Its crests `c ± A`, `A = √(p² + q²)`, stand
/// where `cos(t − t*) = ±1`. A span of half-width `h ≥ π` holds every
/// crest, which the window decides from `h` alone; a shorter span
/// holds a crest iff its cosine against the span's middle `m` is at
/// least `cos h` ([`ArcWindow::crest_outside`]). So each extreme is a
/// crest where the span holds it, and an end's reading where it does
/// not — between its two crests the harmonic is monotone.
///
/// **No comparison is written**: the in-span question is read by
/// [`Real::select_le_zero`], so the body runs at every scalar. At `f64`
/// it is the exact extremum to rounding. The cosine test runs only for
/// `h < π`, where, with `d ∈ [0, π]` the crest's angle from `m`,
/// `1 − cos(h − d) ≤ |cos h − cos d|`: a test misjudged by `δ` leaves
/// the end nearest the crest within `A·δ` of it. The turn test,
/// misjudged by `δ` of angle at `h ≈ π`, puts an end within `δ` of the
/// crest, which reads within `A·δ²/2` of it. At [`Interval`] an
/// undecided reading selects the hull of both candidates, so an
/// enclosure of the inputs yields an enclosure of the extent: the
/// certified reading the box doors take. Poison propagates through
/// every operation.
#[derive(Clone, Copy, Debug)]
pub struct ArcWindow<T: Real> {
    cos_m: T,
    sin_m: T,
    cos_h: T,
    /// `π − h`: at most zero iff the span covers a full turn.
    turn_left: T,
    ends: [(T, T); 2],
}

impl<T: Real> ArcWindow<T> {
    /// The window of the span `[t0, t1]`, in either orientation.
    pub fn new(t0: T, t1: T) -> Self {
        let half = T::from_f64(0.5);
        let (sin_m, cos_m) = ((t0 + t1) * half).sin_cos();
        let h = ((t1 - t0) * half).abs();
        Self {
            cos_m,
            sin_m,
            cos_h: h.min(T::pi()).cos(),
            turn_left: T::pi() - h,
            ends: [t0.sin_cos(), t1.sin_cos()],
        }
    }

    /// **Whether the crest of `p·cos t + q·sin t` lies in the span**, as
    /// a reading at most zero iff it does: `π − h` where the span covers
    /// a full turn, else `A·cos h − (p·cos m + q·sin m)`. The trough is
    /// the crest of `(−p, −q)`. Read it with [`Real::select_le_zero`]:
    /// an enclosure that cannot tell takes both arms.
    pub fn crest_outside(&self, p: T, q: T) -> T {
        let tilt = (p.powi(2) + q.powi(2)).sqrt() * self.cos_h - (p * self.cos_m + q * self.sin_m);
        self.turn_left.select_le_zero(self.turn_left, tilt)
    }

    /// The `(lo, hi)` of `c + p·cos t + q·sin t` over the span.
    pub fn extent(&self, c: T, p: T, q: T) -> (T, T) {
        let crest = (p.powi(2) + q.powi(2)).sqrt();
        let [(s0, c0), (s1, c1)] = self.ends;
        let (r0, r1) = (p * c0 + q * s0, p * c1 + q * s1);
        let (end_lo, end_hi) = (r0.min(r1), r0.max(r1));
        let hi = self.crest_outside(p, q).select_le_zero(crest, end_hi);
        let lo = self.crest_outside(-p, -q).select_le_zero(-crest, end_lo);
        (c + end_lo.min(lo), c + end_hi.max(hi))
    }
}

/// The extent of `frame`'s arc over `window` along `n`, about `origin`.
fn extent_along<T: Real>(
    frame: &ConicFrame<T>,
    window: &ArcWindow<T>,
    origin: Point3<T>,
    n: Vec3<T>,
) -> (T, T) {
    window.extent(
        (frame.center - origin).dot(n),
        frame.semi_u * frame.u_ref.dot(n),
        frame.semi_v * frame.v_ref().dot(n),
    )
}

/// **A circle or ellipse arc's extent along a direction**: the
/// `(lo, hi)` of `(P(t) − origin)·n` over the certified span
/// `[theta0, theta1]` (either orientation), at the carrier's own
/// scalar — the exact extremum where it lies in the span and the
/// end's value where it does not ([`ArcWindow`] carries the closed
/// form). `n` need not be unit: the extent is linear in it.
///
/// Comparison-free, so it serves a lane whose scalar has no ordering.
/// At [`Interval`] it is an enclosure of the extent; at `f64` it is
/// the extent to rounding, and a lane that must contain the locus
/// reads [`conic_arc_extent_along_bracketed`] instead.
///
/// `None` when `carrier` is not a `Circle` or an `Ellipse` (refuse
/// rather than guess).
pub fn conic_arc_extent_along<T: Real>(
    carrier: &Curve3<T>,
    theta0: T,
    theta1: T,
    origin: Point3<T>,
    n: Vec3<T>,
) -> Option<(T, T)> {
    let frame = ConicFrame::of(carrier)?;
    Some(extent_along(
        &frame,
        &ArcWindow::new(theta0, theta1),
        origin,
        n,
    ))
}

/// [`conic_arc_extent_along`] as a certified `f64` bracket: every
/// scalar enters as the [`Interval`] of its bracket and the extent is
/// read off that enclosure, so it contains the extent of every
/// realization of a bracketed description. Poison yields NaN ends.
pub fn conic_arc_extent_along_bracketed<T: Bounds>(
    carrier: &Curve3<T>,
    theta0: T,
    theta1: T,
    origin: Point3<T>,
    n: Vec3<T>,
) -> Option<(f64, f64)> {
    let frame = ConicFrame::of(carrier)?.lifted();
    let origin = Point3::new(lift(origin.x), lift(origin.y), lift(origin.z));
    let n = Vec3::new(lift(n.x), lift(n.y), lift(n.z));
    let (lo, hi) = extent_along(
        &frame,
        &ArcWindow::new(lift(theta0), lift(theta1)),
        origin,
        n,
    );
    Some((lo.lo(), hi.hi()))
}

/// The box of a circle or ellipse arc: per world axis the
/// [`ArcWindow`] extent of that coordinate — the directional extremum
/// read along `x̂`, `ŷ`, `ẑ` — at the certified scalar, hulled with the
/// endpoint seeds.
fn conic_frame_aabb<T: Bounds>(
    frame: ConicFrame<T>,
    theta0: T,
    theta1: T,
    end0: Point3<T>,
    end1: Point3<T>,
) -> Aabb {
    // Endpoint hull: always inside the box (2 points — never empty).
    let mut b = Aabb::from_points([end0, end1]).unwrap_or_else(Aabb::poison);
    let frame = frame.lifted();
    let window = ArcWindow::new(lift(theta0), lift(theta1));
    for (n, min, max) in [
        (Vec3::unit_x(), &mut b.min_x, &mut b.max_x),
        (Vec3::unit_y(), &mut b.min_y, &mut b.max_y),
        (Vec3::unit_z(), &mut b.min_z, &mut b.max_z),
    ] {
        let (lo, hi) = extent_along(&frame, &window, Point3::origin(), n);
        *min = pfold(*min, lo.lo(), f64::min);
        *max = pfold(*max, hi.hi(), f64::max);
    }
    b
}

/// NaN-propagating fold: `f64::min`/`max` ignore NaN, which would
/// silently narrow poison away (D4 ¶2).
fn pfold(a: f64, b: f64, f: fn(f64, f64) -> f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else {
        f(a, b)
    }
}

/// The certified-conservative box of a **circular arc**: the carrier's
/// `Circle` frame over the certified span `[theta0, theta1]`, seeded
/// with the arc's (certified) endpoint points `end0`/`end1`.
///
/// Per world axis the coordinate is `cᵢ + r·uᵢ·cos t + r·vᵢ·sin t`
/// (`v = axis × u_ref`), whose extent over the span is the
/// [`ArcWindow`] closed form: `cᵢ ± r·√(uᵢ² + vᵢ²)` where that
/// extremum lies in the span, an end's value where it does not. It is
/// evaluated at [`Interval`] over every scalar's bracket, so a
/// bracketed radius, frame or span yields a box containing every
/// realization. The endpoints enter as brackets; residual padding
/// (vertices sit on carriers only up to certification) is the
/// CALLER's `Aabb::padded` obligation, exactly as for vertex-extent
/// boxes.
///
/// `None` when `carrier` is not a `Circle` (the caller named the wrong
/// lane — refuse loudly rather than guess). Poison anywhere yields
/// poison bounds, which overlap everything.
pub fn circle_arc_aabb<T: Bounds>(
    carrier: &Curve3<T>,
    theta0: T,
    theta1: T,
    end0: Point3<T>,
    end1: Point3<T>,
) -> Option<Aabb> {
    Some(conic_frame_aabb(
        ConicFrame::circle(carrier)?,
        theta0,
        theta1,
        end0,
        end1,
    ))
}

/// The certified-conservative box of an **ellipse arc**: the carrier's
/// `Ellipse` frame over the certified span `[theta0, theta1]`, seeded
/// with the arc's certified endpoints — [`circle_arc_aabb`] with the
/// frame components scaled by their semi-axes, per world axis
/// `cᵢ + major·uᵢ·cos t + minor·vᵢ·sin t`.
///
/// `None` when `carrier` is not an `Ellipse` (wrong lane — refuse
/// loudly rather than guess). Residual padding stays the caller's
/// `Aabb::padded` obligation, as for every certified box.
pub fn ellipse_arc_aabb<T: Bounds>(
    carrier: &Curve3<T>,
    theta0: T,
    theta1: T,
    end0: Point3<T>,
    end1: Point3<T>,
) -> Option<Aabb> {
    Some(conic_frame_aabb(
        ConicFrame::ellipse(carrier)?,
        theta0,
        theta1,
        end0,
        end1,
    ))
}

/// The certified-conservative box of a SPIRIC arc — the whole
/// period's ([`crate::spiric_period_box`], the one construction, which
/// charges for its own rounding), run in `T`'s own arithmetic, its two
/// corners then bracketed, and seeded with the endpoint hull exactly as
/// [`ellipse_arc_aabb`] is. A
/// C10 superset: the exact per-axis extremes need a transcendental root
/// and are not owed; a wider box is a weaker answer, never a wrong one.
/// Off-regime data (`ρ_min² < d²` beyond the construction's charge) is
/// poison, and poison flows to the poison box.
///
/// `None` when `carrier` is not a `Spiric` (wrong lane — refuse loudly
/// rather than guess). Residual padding stays the caller's
/// `Aabb::padded` obligation, as for every certified box.
pub fn spiric_arc_aabb<T: Bounds>(
    carrier: &Curve3<T>,
    end0: Point3<T>,
    end1: Point3<T>,
) -> Option<Aabb> {
    let Curve3::Spiric {
        center,
        axis,
        u_ref,
        major_radius,
        minor_radius,
        offset,
    } = carrier
    else {
        return None;
    };
    let (lo, hi) = crate::spiric_period_box(
        *center,
        *axis,
        *u_ref,
        *major_radius,
        *minor_radius,
        *offset,
    );
    let ends = Aabb::from_points([end0, end1]).unwrap_or_else(Aabb::poison);
    let period = Aabb::from_points([lo, hi]).unwrap_or_else(Aabb::poison);
    Some(ends.hull(&period))
}

/// The certified-conservative box of a conic ARC, whichever conic the
/// carrier is: [`circle_arc_aabb`] for a `Circle`, [`ellipse_arc_aabb`]
/// for an `Ellipse`, [`spiric_arc_aabb`] for a `Spiric` (a quartic, not
/// a conic, but the same closed-form-kind lane) — one match on the
/// kind, so a consumer that has already decided "this edge is a
/// closed-form kind" reads one door. `None` for a
/// `Line` or `Nurbs` carrier: those kinds have their own rule (the
/// chord; the control hull, [`nurbs_curve_aabb`]) and this door refuses
/// rather than guesses. Everything else — span restriction, outward
/// rounding (for the spiric, its construction's charge), poison — is
/// the kind door's contract, verbatim.
pub fn conic_arc_aabb<T: Bounds>(
    carrier: &Curve3<T>,
    theta0: T,
    theta1: T,
    end0: Point3<T>,
    end1: Point3<T>,
) -> Option<Aabb> {
    match carrier {
        Curve3::Circle { .. } => circle_arc_aabb(carrier, theta0, theta1, end0, end1),
        Curve3::Ellipse { .. } => ellipse_arc_aabb(carrier, theta0, theta1, end0, end1),
        Curve3::Spiric { .. } => spiric_arc_aabb(carrier, end0, end1),
        Curve3::Line { .. } | Curve3::Nurbs(_) => None,
    }
}

/// The certified-conservative box of a NURBS curve: the AABB of its
/// control-point brackets. Sound by the convex-hull property — every
/// curve point is a convex combination of control points because the
/// basis functions are nonnegative and the weights are **strictly
/// positive by construction** (the PR 3 positive-weights invariant,
/// enforced by [`NurbsCurve3::new`]; negative weights would void
/// convexity, Book p. 293). Valid over the whole domain, a fortiori
/// over any certified span (span-tight hulls via knot refinement are a
/// later sharpening; a wider box still contains the locus). No
/// arithmetic — brackets only — so no rounding to pad.
///
/// **A net carrying poison ANYWHERE yields the poison box**, which
/// overlaps everything: the placeholder curve (all-poison by
/// construction) and equally a described net poisoned in one channel
/// of one point. The screen is `net::any_poison`, whose docs carry why
/// a box asks the wider question than the state discriminator does.
pub fn nurbs_curve_aabb<T: Bounds>(curve: &NurbsCurve3<T>) -> Aabb {
    if net::any_poison(curve.control()) {
        return Aabb::poison();
    }
    Aabb::from_points(curve.control().iter().copied()).unwrap_or_else(Aabb::poison)
}
