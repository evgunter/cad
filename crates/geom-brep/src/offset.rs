//! Analytic surfaces close under offset: the mint table and its
//! door-owned refusals.
//!
//! The **offset** of a surface S at signed distance `d` is the normal
//! pushforward `S_d(u, v) = S(u, v) + d·n(u, v)` — every point moved
//! `d` along the unit **chart normal** (`geom`'s derived normal,
//! `∂u × ∂v` normalized). Positive `d` is along that stored normal;
//! the sign conventions per kind follow from each variant's documented
//! chart normal, never from a sampled vector:
//!
//! | kind | chart normal | mint |
//! |---|---|---|
//! | plane | the stored `normal` | `origin + d·normal` |
//! | cylinder | `radial(u)` — radially outward | `radius + d` |
//! | sphere | outward radial | `radius + d` |
//! | torus | out of the tube | `minor_radius + d` |
//! | cone | `radial·cos α − axis·sin α` on the `v > 0` nappe | apex slides `−axis·(d/sin α)` |
//! | nurbs | — | typed refusal ([`OffsetError::NotClosedUnderOffset`]) |
//!
//! Every carried field (frames, axes, `u_ref`, `half_angle`) is copied
//! verbatim — the mint is a struct-update on public fields (D2), a
//! sibling of `sweep`'s `wall_surface` mint switch.
//!
//! # The cylinder sign, pinned
//!
//! `d` is along the NORMAL, and the cylinder's chart normal is
//! `radial(u)` — radially **outward** for the stored `radius > 0`
//! convention — so the radial change is `+d` by the chart convention:
//! `radius + d`, outward positive. The same one-liner pins the sphere
//! (outward radial) and the torus (out of the tube ⇒ `minor + d`).
//!
//! # The cone slide, derived from stored structure
//!
//! On the opening nappe (`v > 0`) the chart normal is
//! `n(u) = radial(u)·cos α − axis·sin α` (the variant's own doc).
//! Pushing `S(u, v) = apex + axis·(v·cos α) + radial(u)·(v·sin α)`
//! by `d·n` and regrouping against the cone form with
//! `v′ = v + d·cos α/sin α`:
//!
//! - radial: `v′·sin α = v·sin α + d·cos α` ✓
//! - axial: `v′·cos α − d/sin α = v·cos α − d·(1 − cos²α)/sin α
//!   = v·cos α − d·sin α` ✓
//!
//! so the offset locus is the cone `{apex − axis·(d/sin α), axis, α,
//! u_ref}`. The slide's minus sign IS the chart normal's axial
//! coefficient `−sin α`, negative for every stored `α ∈ (0, π/2)` —
//! structural, not sampled, and not a numeric branch.
//!
//! **The complete-locus fine print**: the chart normal negates across
//! the apex, so the pushforward along the per-point chart normal would
//! split the double cone. The door offsets along the continuous
//! extension of the `v > 0` nappe's normal field, under which the
//! pushforward is the pure parameter shift `v ↦ v′ = v + d·cos α/sin α`
//! — so nappe attribution follows the SHIFT, not nappe-to-nappe: for
//! `d > 0` the minted `v > 0` nappe's apex band `0 < v′ < d·cos α/sin α`
//! is the image of the base's MIRROR nappe (and symmetrically for
//! `d < 0`, whose band `−|d|·cos α/sin α < v′ < 0` on the minted mirror
//! nappe images the base's `v > 0` nappe); only beyond the band does
//! each minted nappe image its namesake. Kernel faces bound a single
//! nappe by their vertices, so a consumer replacing a face whose
//! `v`-window reaches the band is exactly the consumer that owns the
//! window question below.
//!
//! # Refusals (door-owned, decided BEFORE any mint)
//!
//! Named margined predicates over the inputs (DESIGN.md's
//! pre-construction stance), each metering the **realized** stored
//! quantity — the very float the mint would store — so there is no
//! large-scale rounding regime where the check passes on exact-real
//! reasoning and the mint collapses (the tube door's realized-quantity
//! lesson):
//!
//! - `offset_radius_floor` — cylinder / sphere / torus minor: margin
//!   `radius + d`. Certifiably positive mints; coincident-with-zero or
//!   negative refuses ([`OffsetError::RadiusFloor`] — a collapsed or
//!   near-collapsed offset is never minted); the ambiguity band
//!   escalates.
//! - `offset_torus_ring` — torus: margin `major − (minor + d)`, the
//!   ring convention `R > r` (the same quantity tier-3 validation
//!   nets as `DegenerateTorus`; refusing here keeps that net a second
//!   net, not the first).
//!
//! **Why the cone needs no refusal**: the mint carries `axis`,
//! `u_ref`, and `half_angle` verbatim and translates the apex along
//! the stored unit axis by the finite scalar `d/sin α` (`sin α ∈
//! (0, 1)` for the stored convention), so no stored field approaches a
//! validity edge as `d` varies — unlike the radius kinds, whose stored
//! radius itself crosses zero. A cone offset is a cone. The apex
//! crossing a face's represented region is a parameterization shift
//! (`v ↦ v + d·cos α/sin α` at matched points) — a question about a
//! bounded `v`-window this geometry-layer door does not receive; the
//! face-replacement consumers own that decision over their own
//! windows. There is no input question left to meter, and a predicate
//! without a question would meter vacuously.
//!
//! Degenerate *conventional* data (a non-unit axis, `α` outside
//! `(0, π/2)`) is carried as-is, exactly as every other mint site
//! carries it: conventional fields are unchecked by kernel-wide
//! policy, and their nets (tier-3 validation, poison evaluation) sit
//! downstream unchanged.

use geom::Surface;
use geom_core::{Band, Indeterminate, Margin, Sign};

use crate::dihedral::decide;
use crate::intersect::SurfaceKind;

/// Typed refusal of the offset door (D4 ¶3). Scalar payloads echo the
/// classified margin's ingredients — data, not a decision (the
/// [`Margin::value`] diagnostics blessing; the `PathError` payload
/// precedent).
#[derive(Clone, Debug)]
pub enum OffsetError<T: geom_core::Real> {
    /// The realized offset radius (`radius + d` for a cylinder or
    /// sphere, `minor_radius + d` for a torus) is certifiably at or
    /// below zero: the inward offset collapses the surface onto its
    /// axis, center, or tube spine before (or at) this distance. The
    /// metered margin is the realized stored value itself, so a
    /// large-scale sum that rounds to collapse refuses here rather
    /// than minting a degenerate radius.
    RadiusFloor {
        /// The refusing surface's kind (cylinder, sphere, or torus).
        kind: SurfaceKind,
        /// The realized radius the floor metered — the very value the
        /// mint would have stored, echoed as data.
        realized: T,
    },
    /// The offset torus leaves the ring convention `R > r`: the
    /// realized minor radius `minor + d` reaches the major radius, so
    /// the mint would be a spindle/horn self-intersecting torus.
    TorusRing {
        /// The realized minor radius the ring margin folded against
        /// the stored major radius, echoed as data.
        realized_minor: T,
    },
    /// The offset of a NURBS surface is not a NURBS (normalizing the
    /// chart normal introduces the square root that breaks
    /// rationality), so this door cannot close over it. The coming
    /// door is the **approximating-surface route** — an intensional
    /// `Offset(base, d)` surface description with a certified
    /// to-tolerance fit — never a silent fit here.
    NotClosedUnderOffset,
    /// The operand is already an approximating surface, so its offset
    /// would nest one description inside another — and its certificate
    /// would have to compose two precision claims, the inner ε against
    /// a base this door cannot see through. The kernel has no consumer
    /// for a nested offset today; when one arrives it brings the
    /// composition rule with it, rather than this door inventing one.
    ApproxNesting,
    /// A refusal predicate escalated: the margin landed in the
    /// ambiguity band or was poisoned (escalate-never-guess, D4 ¶3).
    Escalated {
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
}

impl<T: geom_core::Real> core::fmt::Display for OffsetError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::RadiusFloor { kind, realized } => write!(
                f,
                "offset_surface: the realized offset radius of the {kind:?} \
                 ({realized:?} m) is at or below zero — the inward offset collapses \
                 the surface, so nothing is minted"
            ),
            Self::TorusRing { realized_minor } => write!(
                f,
                "offset_surface: the offset torus leaves the ring convention R > r — the \
                 realized minor radius ({realized_minor:?} m) reaches the major radius, \
                 so the mint would self-intersect"
            ),
            Self::NotClosedUnderOffset => write!(
                f,
                "offset_surface: a NURBS surface is not closed under offset (the unit \
                 normal breaks rationality); the approximating-surface route — an \
                 intensional Offset description with a certified fit — is the door for \
                 this kind, and it is not built yet"
            ),
            Self::ApproxNesting => write!(
                f,
                "offset_surface: the operand is already an approximating surface — offsetting \
                 it would nest one Offset description inside another, whose certificate would \
                 have to compose two precision claims. No consumer needs that yet, so nothing \
                 is minted"
            ),
            Self::Escalated { source } => write!(f, "offset_surface escalated: {source}"),
        }
    }
}

impl<T: geom_core::Real> std::error::Error for OffsetError<T> {}

/// The analytic offset mint (module docs): the surface at signed
/// distance `d` along the chart normal, as the same analytic kind with
/// updated public fields. Geometry only — no topology, no fitting; the
/// `Nurbs` kind refuses typed.
///
/// # Errors
///
/// [`OffsetError`] — the two refusal predicates
/// (`offset_radius_floor`, `offset_torus_ring`) decided before any
/// mint, their escalations, and the `Nurbs` non-closure refusal.
pub fn offset_surface<T: geom_core::Decide>(
    surface: &Surface<T>,
    d: T,
    band: Band,
) -> Result<Surface<T>, OffsetError<T>> {
    let esc = |source| OffsetError::Escalated { source };
    // The realized-radius floor: the margin IS the float the mint
    // would store (module docs — no separate exact-real spelling that
    // could disagree with the stored sum at scale). A refusal echoes
    // the metered value and the refusing kind as data.
    let floor = |kind: SurfaceKind, realized: T| -> Result<(), OffsetError<T>> {
        match decide("offset_radius_floor", Margin::of(realized), band).map_err(esc)? {
            Sign::Positive => Ok(()),
            Sign::Zero | Sign::Negative => Err(OffsetError::RadiusFloor { kind, realized }),
        }
    };
    match surface {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => Ok(Surface::Plane {
            origin: *origin + *normal * d,
            normal: *normal,
            u_ref: *u_ref,
        }),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => {
            let realized = *radius + d;
            floor(SurfaceKind::Cylinder, realized)?;
            Ok(Surface::Cylinder {
                origin: *origin,
                axis: *axis,
                radius: realized,
                u_ref: *u_ref,
            })
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => {
            // The slide: `−axis·(d/sin α)`, the minus sign read off
            // the chart normal's axial coefficient `−sin α` (module
            // docs — stored structure, no numeric branch). The reverse
            // offset negates the slide exactly ((−d)/s = −(d/s) and
            // the per-component products negate exactly), so a round
            // trip's only slack is one rounded operation per leg (two
            // per round trip): the apex coordinate addition here and
            // the matching subtraction on the way back.
            let slide = d / half_angle.sin();
            Ok(Surface::Cone {
                apex: *apex - *axis * slide,
                axis: *axis,
                half_angle: *half_angle,
                u_ref: *u_ref,
            })
        }
        Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => {
            let realized = *radius + d;
            floor(SurfaceKind::Sphere, realized)?;
            Ok(Surface::Sphere {
                center: *center,
                radius: realized,
                axis: *axis,
                u_ref: *u_ref,
            })
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => {
            let realized = *minor_radius + d;
            floor(SurfaceKind::Torus, realized)?;
            let ring = Margin::of(*major_radius - realized);
            match decide("offset_torus_ring", ring, band).map_err(esc)? {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => {
                    return Err(OffsetError::TorusRing {
                        realized_minor: realized,
                    });
                }
            }
            Ok(Surface::Torus {
                center: *center,
                axis: *axis,
                major_radius: *major_radius,
                minor_radius: realized,
                u_ref: *u_ref,
            })
        }
        Surface::Nurbs(_) => Err(OffsetError::NotClosedUnderOffset),
        Surface::Approx(_) => Err(OffsetError::ApproxNesting),
    }
}
