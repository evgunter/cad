//! Scalar lifts of the two geometry enums: **the same curve or
//! surface, read at another scalar.**
//!
//! `DESIGN.md` makes "evaluate the same function with a different
//! scalar type" the reason the geometry layer is generic over `T`, and
//! this module holds the ENUM half of that: `Curve3::map_scalar` and
//! `Surface::map_scalar`, one exhaustive match per enum, the scalar
//! conversion as the parameter, so a caller never spells the variant
//! ladder and a variant added to either enum fails to compile here
//! rather than silently falling to a default arm.
//!
//! # Where the rest of the lift lives
//!
//! The enum arms delegate to their payloads, and each payload's lift
//! sits beside its private fields because that is the only place that
//! can construct it: [`NurbsCurve3::map_scalar`] (and `NurbsCurve2`'s)
//! in `curves/nurbs.rs`, [`NurbsSurface::map_scalar`] in
//! `surfaces/nurbs.rs`, [`SurfaceDescription::map_scalar`] and
//! [`ApproxSurface::map_scalar`] in `surfaces/approx.rs` — each through
//! a `from_validated_parts` door that takes the count argument below
//! rather than re-validating. The leaf maps are `geom_core`'s `Point2/3::map`,
//! `Vec2/3::map`, `Mat3::map` and `Affine3::map`. One name, `map_scalar`
//! on every geometry type and `map` on every leaf; a reader looking for
//! "where does this crate lift X" finds it on X.
//!
//! # The fallible direction, and where it exists
//!
//! A lift can REFUSE where whether a component has an image is decided
//! by the target scalar's TYPE rather than by any component's value —
//! the lane → `f64` crossing is the one such caller in the tree. That
//! direction is named `try_map`: the same structural walk with
//! `f: Fn(T) -> Result<U, E>`, the first refusal returned and no
//! component after it consulted.
//!
//! **It is deliberately partial, and the convention above does not
//! promise it everywhere.** `try_map` exists on `Vec3`, `Mat3` and
//! `Affine3` — whose `map`s are written AS it, so each walk's
//! component placement is stated once for both directions — and on
//! `profile`'s `SketchPlane`. The other three leaves (`Point2`,
//! `Point3`, `Vec2`) and every
//! `map_scalar` rung have no fallible twin, because nothing has asked
//! for one. The rule is one name per direction, on the types that have
//! that direction — not both names on every type; minting the rest
//! ahead of a consumer is what
//! `work/props/the-scalar-lift-convention-mints-doors-faster-than-consumers.md`
//! is measuring.
//!
//! # What a lift is, and is not
//!
//! A lift is a **structural map**: every scalar field goes through `f`,
//! every `f64` structure field (knots, weights, degrees, windows,
//! tolerances) is carried verbatim, and no arithmetic is performed. It
//! is therefore exact whenever `f` is — `Real::from_f64` at every
//! scalar, `Dual::constant` for the dual lanes — and then the lifted
//! geometry **evaluates to the source**: bit for bit in a `Dual`'s
//! value channel, as a bracket of the source's `f64` evaluation at the
//! interval scalar. The rows named `described_nurbs_lifts_as_its_payload`
//! in the enum modules' tests pin exactly that.
//!
//! # Why a structural map's counts survive it (the one home of this argument)
//!
//! Every `from_validated_parts` door in this crate — the curves', the
//! surface's — skips `new`'s validation, and this is the argument it
//! skips it on. It is stated here, once, because it is the same
//! argument in every direction and at every arity; each door's doc
//! points at this section rather than carrying a copy.
//!
//! A door takes parts that are an ALREADY-VALIDATED curve's or
//! surface's own: the knot vectors carried verbatim or re-domained, and the net and
//! weight vector under **one structural map**. These shapes qualify,
//! and the argument covers each of them:
//!
//! - **Pointwise** — every control point through a function, the
//!   weights untouched. A map over a `Vec` cannot change its length.
//! - **A grid permutation** — the same points and the same weights,
//!   re-indexed (a transpose, a reversal in one direction). A
//!   permutation is a bijection of the index set onto itself, so it
//!   changes neither the length nor the multiset of values.
//! - **Knots re-expressed on another domain** — the same net and the
//!   same weights under a knot vector from [`KnotVector::on_domain`],
//!   which changes neither the degree nor the knot count, so
//!   [`KnotVector::control_count`] is unchanged.
//! - **Any composition of those** — a pointwise map after a
//!   permutation, a re-domained knot vector over a mapped net, and so
//!   on, since none of them changes either of the two things the checks
//!   read.
//!
//! So `control.len()` still equals the knots' `control_count()` (the
//! product of the two per-direction counts, for a surface),
//! `weights.len()` still equals `control.len()`, and no weight VALUE
//! moved — every weight is still the positive finite number `new`
//! admitted, and every knot vector is still the one `KnotVector`'s own
//! constructor accepted. The `debug_assert` in each door re-derives
//! the count agreement (D2 addendum row 5: a bug detectable only by
//! re-derivation).
//!
//! **What the door cannot check, and therefore requires of its
//! caller.** The `debug_assert` reads two LENGTHS. It cannot see
//! whether the weights rode the SAME permutation as the points, and a
//! caller that permutes the net one way and the weights another hands
//! back a net whose every count is right and whose every control point
//! has the wrong weight. That pairing is the caller's obligation, owed
//! at the call site; the door states it and trusts it, and the doors'
//! own rows (the reversal's "the weight rides the same permutation")
//! are where it is pinned.
//!
//! # The NURBS and approximating variants lift their PAYLOAD
//!
//! A [`Curve3::Nurbs`] lifts to a [`Curve3::Nurbs`] whose control net
//! went through `f` ([`NurbsCurve3::map_scalar`]); a [`Surface::Nurbs`]
//! and a [`Surface::Approx`] likewise ([`NurbsSurface::map_scalar`],
//! [`ApproxSurface::map_scalar`]). No variant is mapped to the
//! placeholder: a described curve that came back "no description yet"
//! after a lift would be a silent substitution, and the whole reason a
//! lift exists is to evaluate the *same* geometry at another scalar.
//!
//! # The placeholder and the poisoned net (the one home of this argument)
//!
//! What the placeholder and a poisoned net lift to follows from the map
//! being structural. The placeholder's every control point is
//! all-poison, and every scalar embedding keeps poison (`from_f64(NaN)`
//! is the interval's poison, `Dual::constant(NaN)` a poisoned dual), so
//! **the placeholder lifts to the placeholder** — `is_placeholder` is
//! preserved through the map. A described net carrying poison in some
//! points lifts to a described net carrying poison in the same points:
//! **never the benign placeholder**, because the crate docs' rule is
//! `all`-not-`any` and a map applied pointwise cannot turn some into
//! all. That rule's own width (which channels `is_placeholder` reads)
//! is untouched here; the lift neither narrows nor widens it.

use geom_core::Real;

use crate::curves::Curve3;
use crate::surfaces::Surface;

#[cfg(doc)]
use crate::curves::NurbsCurve3;
#[cfg(doc)]
use crate::surfaces::{ApproxSurface, NurbsSurface, SurfaceDescription};
#[cfg(doc)]
use geom_core::KnotVector;

impl<T: Real> Curve3<T> {
    /// The same curve read at another scalar (module docs): analytic
    /// fields through `f`, the NURBS payload through
    /// [`NurbsCurve3::map_scalar`]. Exact whenever `f` is; poison
    /// travels; the placeholder lifts to the placeholder and a
    /// described curve to a described curve.
    #[must_use]
    pub fn map_scalar<U: Real>(&self, f: impl Fn(T) -> U) -> Curve3<U> {
        match self {
            Curve3::Line { origin, dir } => Curve3::Line {
                origin: origin.map(&f),
                dir: dir.map(&f),
            },
            Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => Curve3::Circle {
                center: center.map(&f),
                axis: axis.map(&f),
                radius: f(*radius),
                u_ref: u_ref.map(&f),
            },
            Curve3::Ellipse {
                center,
                axis,
                major,
                minor,
                u_ref,
            } => Curve3::Ellipse {
                center: center.map(&f),
                axis: axis.map(&f),
                major: f(*major),
                minor: f(*minor),
                u_ref: u_ref.map(&f),
            },
            Curve3::Spiric {
                center,
                axis,
                u_ref,
                major_radius,
                minor_radius,
                offset,
            } => Curve3::Spiric {
                center: center.map(&f),
                axis: axis.map(&f),
                u_ref: u_ref.map(&f),
                major_radius: f(*major_radius),
                minor_radius: f(*minor_radius),
                offset: f(*offset),
            },
            Curve3::Nurbs(n) => Curve3::Nurbs(std::sync::Arc::new(n.map_scalar(&f))),
        }
    }
}

impl<T: Real> Surface<T> {
    /// The same surface read at another scalar (module docs): analytic
    /// fields through `f`, the NURBS payload through
    /// [`NurbsSurface::map_scalar`], the approximating payload through
    /// [`ApproxSurface::map_scalar`]. Exact whenever `f` is; poison
    /// travels; the placeholder lifts to the placeholder and a
    /// described surface to a described surface.
    #[must_use]
    pub fn map_scalar<U: Real>(&self, f: impl Fn(T) -> U) -> Surface<U> {
        match self {
            Surface::Plane {
                origin,
                normal,
                u_ref,
            } => Surface::Plane {
                origin: origin.map(&f),
                normal: normal.map(&f),
                u_ref: u_ref.map(&f),
            },
            Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => Surface::Cylinder {
                origin: origin.map(&f),
                axis: axis.map(&f),
                radius: f(*radius),
                u_ref: u_ref.map(&f),
            },
            Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            } => Surface::Cone {
                apex: apex.map(&f),
                axis: axis.map(&f),
                half_angle: f(*half_angle),
                u_ref: u_ref.map(&f),
            },
            Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            } => Surface::Sphere {
                center: center.map(&f),
                radius: f(*radius),
                axis: axis.map(&f),
                u_ref: u_ref.map(&f),
            },
            Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            } => Surface::Torus {
                center: center.map(&f),
                axis: axis.map(&f),
                major_radius: f(*major_radius),
                minor_radius: f(*minor_radius),
                u_ref: u_ref.map(&f),
            },
            Surface::Nurbs(n) => Surface::Nurbs(std::sync::Arc::new(n.map_scalar(&f))),
            Surface::Approx(a) => Surface::Approx(std::sync::Arc::new(a.map_scalar(&f))),
        }
    }
}
