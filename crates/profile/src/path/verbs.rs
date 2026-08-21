//! **The sealed verbs kernel (PATHS-DESIGN §2c, round 12).**
//!
//! Every value here holds NOTHING but binding bits and verb-authored
//! arguments, and every function is PURE over its parameters. The
//! enforcement is SIGNATURE PURITY, stated precisely: the state types
//! carry no carrier field (`pending.carrier` is E0609 — no such
//! field), so a verb that consults "what leg produced this point" has
//! nothing in scope to read, and re-introducing carrier-awareness
//! requires WIDENING A SIGNATURE here — the loud reviewable act such a
//! change should be. (Rust module visibility alone is NOT the seal:
//! as a child of `path`, this module could name `super::Core`; it is
//! the parameter types that make the old wall unwritable.)
//!
//! The chain (`path.rs`) threads these values through the verb
//! functions and applies their EMISSIONS (append-leg / insert-arc /
//! extend-ray) to the accumulating loop on the far side of this module
//! boundary; the §4 junction/identity checks read the chain's own
//! intrinsic leg data and stay chain-side. The arc-carrier RESOLUTION
//! machinery (`arc_fillet::resolve`, unchanged bit for bit) is the
//! kernel's arc half: it is already pure over `FilletSide` values.
//!
//! # The spec family (§2c rounds 5–9): `ArcData` as standalone types
//!
//! The six modes are standalone value types so that admissibility is a
//! TRAIT MATRIX: an inadmissible (state, mode) pair is a missing impl —
//! unrepresentable, not refused. The matrix (DOF-derived, state-keyed)
//! is documented at each impl; the wire records the one unified
//! [`ArcData`](super::program::ArcData) enum.

use geom_core::k_stats::decide;
use geom_core::{Tol, Band, Decide, Margin, Point2, Real, Sign, Tolerance, Vec2};

use super::arc_fillet::{ArcFilletTrims, FilletSide, SideCarrier};
use super::{Dir, PathError, unit_from_components};
use crate::sugar::ArcSweep;

/// A monomorphized handle on the arc-carrier resolution machinery
/// (`arc_fillet::resolve`), captured by the arc-involving verb AT CALL
/// TIME — which is what keeps the `Decide + Bounds` obligation on the
/// fused verbs alone (§2c "Bounds"): the generic lattice doors that
/// later complete the arrival call through this pointer without ever
/// naming the bound.
pub(crate) type ArcResolver<T> =
    fn(FilletSide<T>, FilletSide<T>, T, Tol) -> Result<ArcFilletTrims<T>, PathError<T>>;

// ------------------------------------------------------------------
// Bare state values (the binding bits, nothing else).
// ------------------------------------------------------------------

/// A directed point: position + tangent — the §2 binding bits and
/// NOTHING else. The kernel's one incoming-state currency.
#[derive(Clone, Copy, Debug)]
pub struct DirectedPoint<T: Real> {
    /// The bound position.
    pub at: Point2<T>,
    /// The bound (or intrinsic incoming) direction.
    pub dir: super::Dir<T>,
}

/// An opened fillet whose incoming side is the tangent RAY of the
/// directed point it consumed (§2c round 10: bare `fillet` ⇔ ray
/// extension, uniform across incomings). **No carrier field exists**:
/// the old carrier-keyed wall (a spelling refusal at resolution) is
/// unwritable against this type.
#[derive(Clone, Copy, Debug)]
pub(crate) struct PendingRay<T: Real> {
    /// The ray's origin (the consumed directed point).
    pub origin: Point2<T>,
    /// The ray's direction.
    pub dir: Dir<T>,
    /// The fillet radius.
    pub radius: T,
}

/// An opened fillet whose incoming side is an ARC the fused verb
/// AUTHORED (§2c: a fillet that needs an arc carrier cannot LEARN it —
/// so it authors it). The carrier here is the verb's own argument
/// data, not knowledge about a neighbour.
#[derive(Clone, Copy, Debug)]
pub(crate) struct PendingArc<T: Real> {
    /// The incoming side's anchoring on-path point.
    pub anchor: Point2<T>,
    /// The authored carrier centre.
    pub centre: Point2<T>,
    /// The authored travel sense.
    pub winding: ArcSweep,
    /// The fillet radius.
    pub radius: T,
    /// The captured resolution machinery (see [`ArcResolver`]).
    pub resolver: ArcResolver<T>,
}

/// An opened fillet: the arrival-side state value. The variant is the
/// FILLET VERB's own authorship (its incoming half), carried forward as
/// the verb's output state — never read back from the chain.
#[derive(Clone, Debug)]
pub(crate) enum Pending<T: Real> {
    /// `fillet(r)` / `fillet_arc(r, …)` — line incoming (the ray).
    Ray(PendingRay<T>),
    /// `arc_fillet(…, r)` / `arc_fillet_arc(…, r, …)` — authored arc
    /// incoming.
    Arc(PendingArc<T>),
}

impl<T: Real> Pending<T> {
    /// The fillet radius (mode-independent).
    pub fn radius(&self) -> T {
        match self {
            Pending::Ray(p) => p.radius,
            Pending::Arc(p) => p.radius,
        }
    }

    /// The incoming side as the resolution machinery's value.
    pub fn side(&self) -> FilletSide<T> {
        match self {
            Pending::Ray(p) => FilletSide {
                anchor: p.origin,
                carrier: SideCarrier::Ray(p.dir.unit),
            },
            Pending::Arc(p) => FilletSide {
                anchor: p.anchor,
                carrier: SideCarrier::Circle {
                    centre: p.centre,
                    winding: p.winding,
                },
            },
        }
    }
}

// ------------------------------------------------------------------
// The spec family: standalone value types + the side bit.
// ------------------------------------------------------------------

/// The one discrete bit every endpoint-free/derived mode carries in its
/// own dress (§2c): which half-plane of the departure tangent the
/// carrier centre sits on. `Left` of travel curves the arc CCW.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArcSide {
    /// Centre on the left of travel (counterclockwise arc).
    Left,
    /// Centre on the right of travel (clockwise arc).
    Right,
}

impl ArcSide {
    /// The winding an arc travelled with its centre on this side.
    pub(crate) fn winding(self) -> ArcSweep {
        match self {
            ArcSide::Left => ArcSweep::Ccw,
            ArcSide::Right => ArcSweep::Cw,
        }
    }

    /// The signed left-normal factor: +1 for `Left`, −1 for `Right`.
    pub(crate) fn sign<T: Real>(self) -> T {
        match self {
            ArcSide::Left => T::one(),
            ArcSide::Right => -T::one(),
        }
    }
}

/// Arrival mode `Radius { r, side }`: a tangent circle at a directed
/// point has one free length + a side bit, so the centre is DERIVED —
/// never authored (§2c "arrival halves").
#[derive(Clone, Copy, Debug)]
pub struct Radius<T> {
    /// The carrier radius.
    pub r: T,
    /// Which side of the tangent the centre sits on.
    pub side: ArcSide,
}

/// Leg / fused-incoming mode `Bulge { p, b }`: chord-relative, so it is
/// admissible exactly where the chord exists — leg targets and the
/// fused verbs' incoming specs (the target is in the args), never
/// arrivals (§2c round 6).
#[derive(Clone, Copy, Debug)]
pub struct Bulge<T, Tgt> {
    /// The authored endpoint (or [`Start`](super::Start) to close).
    pub p: Tgt,
    /// The authored bulge (M2 convention, tan(θ/4)).
    pub b: T,
}

/// Mode `Via { q, p }`: the arc through `q`. Completes a directed
/// anchor (tangent-at-anchor circle through `q` — determined) and a
/// bare POINT tip (three points); underdetermines a bare anchor.
#[derive(Clone, Copy, Debug)]
pub struct Via<T: Real, Tgt> {
    /// A point the arc passes through.
    pub q: Point2<T>,
    /// The authored endpoint (or `Start` to close).
    pub p: Tgt,
}

/// Mode `Center { c, winding, p }`: the arc about an authored centre.
/// `Center@Point` supplies the direction retroactively (exactly what
/// the carrier-bound entry did); `Center@Directed` is EXCLUDED — the bound direction
/// would have to value-match the derived tangent (§2c round 6).
#[derive(Clone, Copy, Debug)]
pub struct Center<T: Real, Tgt> {
    /// The carrier centre.
    pub c: Point2<T>,
    /// Travel sense about the centre (structural).
    pub winding: ArcSweep,
    /// The authored anchor/endpoint (or `Start` to close).
    pub p: Tgt,
}

/// Endpoint-free leg mode `Sweep { r, side, angle }` — the arc analog
/// of `line(len)`: tangent-departing, endpoint DERIVED (§2c round 8).
#[derive(Clone, Copy, Debug)]
pub struct Sweep<T> {
    /// The carrier radius.
    pub r: T,
    /// Which side of the departure tangent the centre sits on.
    pub side: ArcSide,
    /// The swept central angle, radians (definitely positive).
    pub angle: T,
}

/// Endpoint-free leg mode `ArcLen { r, side, len }`: as [`Sweep`], the
/// extent authored as an arc length (`angle = len / r`).
#[derive(Clone, Copy, Debug)]
pub struct ArcLen<T> {
    /// The carrier radius.
    pub r: T,
    /// Which side of the departure tangent the centre sits on.
    pub side: ArcSide,
    /// The arc length, meters (definitely positive).
    pub len: T,
}

// ------------------------------------------------------------------
// Pure derivations (each consumes binding bits + authored args only).
// ------------------------------------------------------------------

/// Gates a kernel length/radius definitely positive through the funnel.
pub(crate) fn gate_positive<T: Decide>(
    name: &'static str,
    value: T,
    band: Band,
    err: impl FnOnce(T) -> PathError<T>,
) -> Result<(), PathError<T>> {
    match decide(name, Margin::of(value), band) {
        Ok(Sign::Positive) => Ok(()),
        Ok(_) => Err(err(value)),
        Err(source) => Err(PathError::Escalated { source }),
    }
}

/// [`Radius`]'s derivation: the tangent circle at a directed point —
/// centre = p + side·r·n̂ (n̂ the left normal of the tangent), winding
/// from the side bit. The radius is gated definitely positive on the
/// fillet-radius predicate's sibling (`path_arc_center_radius`, the
/// carrier-radius gate).
pub(crate) fn radius_carrier<T: Decide>(
    dp: DirectedPoint<T>,
    spec: Radius<T>,
    band: Band,
) -> Result<(Point2<T>, ArcSweep), PathError<T>> {
    gate_positive("path_arc_center_radius", spec.r, band, |radius| {
        PathError::DegenerateArcCenter { radius }
    })?;
    let n = Vec2::new(-dp.dir.unit.y, dp.dir.unit.x);
    let centre = dp.at + n * (spec.side.sign::<T>() * spec.r);
    Ok((centre, spec.side.winding()))
}

/// [`Via`]'s directed-anchor derivation: the circle tangent to `dir` at
/// `at`, through `q` — centre on the normal at `at`, at the parameter
/// where the perpendicular bisector of (at, q) crosses it:
/// t = |q−at|² / (2·(q−at)·n̂). A `q` on the tangent line names no
/// circle (the chord-degenerate class) and refuses as the collinear
/// via ([`PathError::ArcViaCollinear`]); a `q` at `at` is the
/// degenerate chord.
pub(crate) fn via_carrier<T: Decide>(
    dp: DirectedPoint<T>,
    q: Point2<T>,
    band: Band,
) -> Result<(Point2<T>, ArcSweep), PathError<T>> {
    let d = q - dp.at;
    let chord = d.norm_squared().sqrt();
    gate_positive("path_arc_chord", chord, band, |c| {
        PathError::DegenerateArcChord { chord: c }
    })?;
    let n = Vec2::new(-dp.dir.unit.y, dp.dir.unit.x);
    let across = d.dot(n);
    // The signed offset of q from the tangent LINE, meters — zero means
    // no tangent circle reaches q (the collinear class).
    let winding = match decide("path_arc_via_offset", Margin::of(across), band) {
        Ok(Sign::Zero) => return Err(PathError::ArcViaCollinear { offset: across }),
        Ok(Sign::Positive) => ArcSweep::Ccw,
        Ok(Sign::Negative) => ArcSweep::Cw,
        Err(source) => return Err(PathError::Escalated { source }),
    };
    let t = d.norm_squared() / (T::from_f64(2.0) * across);
    let centre = dp.at + n * t;
    Ok((centre, winding))
}

/// The endpoint-free legs' shared derivation ([`Sweep`] / [`ArcLen`]):
/// carrier centre from the directed start (as [`radius_carrier`]), the
/// endpoint by rotating the start about it through the swept angle in
/// the side's travel sense, and the leg's bulge tan(angle/4) signed by
/// the side. Everything closed-form; the swept angle is gated
/// definitely positive (`path_arc_sweep`).
pub struct TangentArcLeg<T: Real> {
    /// The derived endpoint.
    pub end: Point2<T>,
    /// The derived carrier centre.
    pub centre: Point2<T>,
    /// The travel sense (from the side bit).
    pub winding: ArcSweep,
    /// The leg's bulge.
    pub bulge: T,
    /// The end tangent (departure rotated by the signed sweep).
    pub end_dir: Dir<T>,
    /// The chord length, meters (the junction-lever cap).
    pub chord: T,
}

pub(crate) fn tangent_arc_leg<T: Decide>(
    dp: DirectedPoint<T>,
    r: T,
    side: ArcSide,
    angle: T,
    band: Band,
) -> Result<TangentArcLeg<T>, PathError<T>> {
    gate_positive("path_arc_center_radius", r, band, |radius| {
        PathError::DegenerateArcCenter { radius }
    })?;
    gate_positive("path_arc_sweep", angle, band, |value| {
        PathError::DegenerateArcSpec { value }
    })?;
    let sgn = side.sign::<T>();
    let n = Vec2::new(-dp.dir.unit.y, dp.dir.unit.x);
    let centre = dp.at + n * (sgn * r);
    // Rotate (at − centre) about the centre by the SIGNED sweep.
    let signed = sgn * angle;
    let (s, c) = signed.sin_cos();
    let v = dp.at - centre;
    let end = centre + Vec2::new(v.x * c - v.y * s, v.x * s + v.y * c);
    let bulge = (signed / T::from_f64(4.0)).tan();
    let end_dir = Dir::from_angle(dp.dir.ang + signed);
    let chord = (end - dp.at).norm_squared().sqrt();
    Ok(TangentArcLeg {
        end,
        centre,
        winding: side.winding(),
        bulge,
        end_dir,
        chord,
    })
}

/// Re-exported director construction so arrival builders normalize
/// components through the ONE shared door.
pub(crate) fn director<T: Decide>(dx: T, dy: T, tol: Tol) -> Result<Dir<T>, PathError<T>> {
    unit_from_components(dx, dy, tol)
}
