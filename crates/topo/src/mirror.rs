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
//!    its base's normal. Every certificate is carried: a reflection
//!    keeps every distance one states. A pcurve row on an analytic
//!    chart is restated with its azimuth, and a joint element there is
//!    conjugated by the negation, exactly (integers).
//! 2. **The per-entity reversal** ([`crate::Body::revert`]'s map): every
//!    half-edge cycle reversed, every outward side flipped once, a
//!    plane's chart images and rows re-stated under `(u, v) ↦ (u, −v)`,
//!    and every joint element moved onto its predecessor, inverted.
//!    `revert`'s refusal of two edges on one segment is not asked: it
//!    guards the COMPLEMENT, whose wedges pair the sheets differently,
//!    while a reflection maps each wedge onto a wedge.
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
//! A body with the source's keys, arena for arena, so names read
//! through it as through a copy. Its volume is the source's, which the
//! tier-3 `+V` invariant checks: without step 2 it is negative. A
//! spiric wall's restated offset `u0′ = −u0 − σμπ` rounds, as every
//! mapped coordinate does; every other restatement is exact.
//!
//! # The audit's residuals (P3)
//!
//! The `Nurbs` placeholder refuses as [`crate::transform_rigid`] does.
//! Every chart image has a mirror image, a spiric's included
//! (`map_isometry`'s `reflected_image`).

use geom_core::linalg::frame::{FrameError, mirror_across_plane, mirror_across_unit_plane};
use geom_core::{Affine3, Decide, Point3, Real, Tol, UnitVec3, Vec3};

use crate::body::Body;
use crate::transform::{Isometry, TransformError, map_isometry};

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
    /// The reflection across the plane through `point` with the decided
    /// unit normal `normal`.
    #[must_use]
    pub fn across_unit(point: Point3<T>, normal: UnitVec3<T>) -> Self {
        Self {
            map: mirror_across_unit_plane(point, normal),
        }
    }

    /// The affine map.
    #[must_use]
    pub fn to_affine(&self) -> Affine3<T> {
        self.map
    }
}

/// The mirror image of `body` across `reflection`'s plane (module docs):
/// a new body with the source's keys and every certificate carried. It
/// is the construction-state map; a finished body mirrors through
/// [`crate::FinishedBody::moved`], which carries its verdict too.
///
/// # Errors
///
/// [`TransformError`], as [`crate::transform_rigid`].
pub fn mirror_body<T: Decide>(
    body: &Body<T>,
    reflection: &Reflection<T>,
    _tol: Tol,
) -> Result<Body<T>, TransformError> {
    Ok(map_isometry(body, &Isometry::Improper(*reflection))?.reverted())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom::{ApproxSurface, Surface, SurfaceDescription};

    /// The offset surface's own point, `base(u, v) + d·n(u, v)`, read off
    /// its description rather than its fit.
    fn described(s: &ApproxSurface<f64>, u: f64, v: f64) -> Point3<f64> {
        let SurfaceDescription::Offset { base, d } = s.description();
        base.eval(u, v) + Surface::Nurbs(base.clone()).normal(u, v) * *d
    }

    /// **An offset surface mirrors onto its image.** Across an oblique
    /// plane, the image's description and its fit both sample `M∘S` at
    /// the source's own parameters: the base net maps by its points and
    /// the distance negates with the base's chart normal. A distance
    /// left unnegated stands `2d` off, and so does a fit re-minted on it.
    #[test]
    fn an_offset_surface_mirrors_onto_its_image() {
        let tol = Tol::witness();
        let (body, face) = crate::fixtures::approx_faced_body::<f64>();
        let reflection =
            Reflection::across(Point3::new(0.1, -0.2, 0.3), Vec3::new(1.0, 2.0, 3.0), tol).unwrap();
        let image = mirror_body(&body, &reflection, tol).expect("an offset face mirrors");
        let m = reflection.to_affine();
        let surface = |b: &Body<f64>| b.proven_face(face).surface().clone();
        let (source, mapped) = (surface(&body), surface(&image));
        let (Surface::Approx(a), Surface::Approx(b)) = (&source, &mapped) else {
            panic!("the image's face is an offset surface: {mapped:?}");
        };
        for u in [0.2, 0.5, 0.8] {
            for v in [0.25, 0.5, 0.75] {
                let want = m.transform_point(described(a, u, v));
                let got = described(b, u, v);
                assert!(
                    (got - want).norm() <= 1e-12,
                    "({u}, {v}): the description samples {got:?}, M∘S is {want:?}"
                );
                let fit = mapped.eval(u, v);
                let want = m.transform_point(source.eval(u, v));
                assert!(
                    (fit - want).norm() <= 1e-9,
                    "({u}, {v}): the fit samples {fit:?}, M∘S is {want:?}"
                );
            }
        }
    }
}
