//! `mirror_body` — the reflection of a whole body across a plane
//! (`docs/MIRROR-DESIGN.md` P1–P3): the sibling door beside
//! [`crate::transform_rigid`], which stays rigid by type.
//!
//! # The composition
//!
//! A reflection `M` (determinant `−1`) maps the material onto its mirror
//! image, but not the boundary's orientation: the interior-left rule is
//! a statement about handedness, so every loop of the pointwise image
//! runs clockwise about the image's outward normal `M·n`. The image with
//! its loops as they are is therefore the mirror body TURNED INSIDE
//! OUT. The door builds exactly that body and then reverses it:
//!
//! 1. **Every datum under `M`** ([`crate::transform`]'s isometry map at
//!    the improper hand): points by `M`; each carrier as `M ∘ c` on its
//!    own parameter, so every edge's range and forward half-edge carry
//!    over (a circle's or ellipse's axis is `−M·a`, the frame keeping its
//!    sense of turning; a spiric's plane side is negated); a sketch
//!    pushforward under `M ∘ place`, its sketch datum as authored; a
//!    plane's stored normal is `−M·n`, the inside-out outward side, which
//!    leaves its chart `M ∘ S` exactly; the analytic charts (cylinder,
//!    cone, sphere, torus) map their frame forward (P1, ratified: `u ↦
//!    −u`), so their stored surface is `M ∘ S ∘ (u ↦ −u)`, their outward
//!    normal crosses as `M·n` and the inside-out face flips its `sense`,
//!    and every chart image on them has its azimuth negated; a spline net
//!    maps by its points, its chart unmoved and its chart normal `−M·n`
//!    already inside out; an offset surface's distance is negated with
//!    its base's normal. Every edge is re-certified on the mapped data.
//! 2. **The per-entity reversal** ([`crate::Body::revert`]'s map): every
//!    half-edge cycle reversed, every outward side flipped once, a
//!    plane's chart images re-stated under `(u, v) ↦ (u, −v)`.
//!    `revert`'s refusal of two edges on one segment is not asked: it
//!    guards the COMPLEMENT, whose wedges pair the sheets differently,
//!    while a reflection maps each wedge onto a wedge.
//! 3. **Pcurve rows re-minted** on the finished body, construction-fresh,
//!    as [`crate::transform_rigid`] re-mints them: no source row is
//!    mapped, and none survives into the result.
//!
//! The net effect, face by face: a plane's normal is `M·n` and its chart
//! `v` reversed; an analytic face keeps its `sense` and its chart's
//! azimuth reversed; a spline or offset face flips its `sense`.
//!
//! P2's open claim, whether the loops' reversal under a reflection and
//! the normal flip cancel so no half-edge is reversed, is refuted at
//! step 1: they would cancel only for a body that bounds its complement,
//! which is why step 2 reverses both.
//!
//! # What the result is
//!
//! A body at rest with the source's keys, arena for arena, so names
//! read through it as through a copy. Its volume is the source's, which
//! the tier-3 `+V` invariant checks: without step 2 it is negative.
//!
//! # The audit's residuals (P3)
//!
//! - An edge image that is a spiric wall in a torus chart has no
//!   reflected image (one `sense` rides two channels), so the body
//!   refuses typed ([`TransformError::ChartReflected`]).
//! - The `Nurbs` placeholder, an approximating surface with no fit door
//!   at the scalar, and an edge needing the NURBS lane at a scalar that
//!   holds none refuse as [`crate::transform_rigid`] does.

use geom_core::linalg::frame::{FrameError, mirror_across_plane};
use geom_core::{Affine3, Decide, Point3, Real, Tol, Vec3};

use crate::body::Body;
use crate::transform::{Hand, TransformError, map_isometry};

/// A reflection across a plane, decided at its one door
/// ([`Reflection::across`]): its linear part is a Householder matrix,
/// determinant `−1`, so no rigid motion is one.
#[derive(Clone, Copy, Debug)]
pub struct Reflection<T: Real> {
    map: Affine3<T>,
}

impl<T: Decide> Reflection<T> {
    /// The reflection across the plane through `point` with normal
    /// `normal` (of any nonzero length).
    ///
    /// # Errors
    ///
    /// [`mirror_across_plane`]'s: a normal whose length is not
    /// definitely nonzero and finite, or a tolerance with no band.
    pub fn across(point: Point3<T>, normal: Vec3<T>, tol: Tol) -> Result<Self, FrameError> {
        mirror_across_plane(point, normal, tol).map(|map| Self { map })
    }
}

impl<T: Real> Reflection<T> {
    /// The affine map.
    #[must_use]
    pub fn to_affine(&self) -> Affine3<T> {
        self.map
    }
}

/// The mirror image of `body` across `reflection`'s plane (module docs):
/// a new body with the source's keys, every carrier re-certified and
/// every pcurve row re-minted.
///
/// # Errors
///
/// [`TransformError`], as [`crate::transform_rigid`], plus
/// [`TransformError::ChartReflected`] for a chart image with no mirror.
pub fn mirror_body<T: Decide + crate::props::AtRestPolicy>(
    body: &Body<T>,
    reflection: &Reflection<T>,
    tol: Tol,
) -> Result<Body<T>, TransformError> {
    let owes_rows = body.pcurves().next().is_some();
    let mut inside_out = map_isometry(body, &reflection.map, Hand::Improper, tol)?;
    inside_out.pcurves.clear();
    inside_out.joints.clear();
    let mut out = inside_out.reverted();
    if owes_rows {
        crate::pcurves::mint_pcurves(&mut out, tol)
            .map_err(|source| TransformError::Pcurve { source })?;
    }
    Ok(out)
}
