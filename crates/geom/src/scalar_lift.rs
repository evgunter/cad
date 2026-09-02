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
//! a door that states why the payload's count invariants survive a
//! pointwise map. The leaf maps are `geom_core`'s `Point2/3::map`,
//! `Vec2/3::map`, `Mat3::map` and `Affine3::map`. One name, `map_scalar`
//! on every geometry type and `map` on every leaf; a reader looking for
//! "where does this crate lift X" finds it on X.
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
