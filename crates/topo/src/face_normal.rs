//! The **one door** for a face's outward normal — planar or curved:
//! the single place in this crate that folds the `sense` bit INTO a
//! chart normal. Handing the bit onward is the other legitimate answer
//! and is not a second fold — a `geom_brep` door that mints the
//! outward normal from gradients of its own takes the bit and leaves
//! nothing here to fold.
//!
//! Four doors, one flip. By key: [`face_outward_normal`] answers for a
//! PLANE, where the normal does not depend on where you stand, and
//! [`face_outward_normal_at`] answers at a POINT, which is what a
//! curved carrier requires and what the plane arm returns unchanged.
//! By value, for a caller that has already resolved the face and must
//! not discard a failed lookup: [`plane_outward_normal`] takes the
//! [`Face`] and its plane's chart normal, and [`implicit_outward_normal`]
//! takes a carrier, the face's bit and a point certified onto it.
//! Every one of them spells the fold through
//! [`OutwardNormal::from_chart`], the type's only constructor, and
//! this file is the only one under `topo/src` that names it beside a
//! plane pattern (the guard row below).
//!
//! # Why one door, and why here
//!
//! A face's outward normal is its surface's chart normal where
//! [`Face::sense`] is `true` and the negation where it is `false` —
//! the chart is the only place orientation is encoded, so on a
//! `sense: false` face the stored normal points INTO the material and
//! every consumer reading a material direction off it answers
//! backwards. One flip, in one place, is what keeps two flips from
//! drifting apart, and the bit never leaves this crate as a `±1`:
//! there is no scalar sign accessor on [`Face`], so a consumer that
//! wants an outward normal comes here or hands the bit to a
//! `geom_brep` door that mints one from gradients of its own.
//!
//! The door sits at the crate root because its consumers are in two
//! lanes: the boolean's planar consumers reach it through
//! `boolean::reduce::face_plane` (which re-exports it) and the pierce
//! lane directly, while the shared [`crate::sector_face`] walk serves
//! the splitting lane too — a module under `boolean/` could not be the
//! one door for both without a wrong-way edge.
//!
//! **A better home exists and is not reachable from this lane.**
//! The function is a `Body` query, exactly like `Body::mate` or
//! `Body::loop_cycle`, and belongs as an inherent method on
//! [`Body`]. That is `body.rs`, outside the scope
//! this module was created under; it is recorded on issue #695 with
//! the rest of the placement questions.

use geom_brep::OutwardNormal;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Real, Sign, Vec3};

use crate::body::Body;
use crate::entity::{Face, FaceKey};
use crate::validate::decide;

/// A resolved planar face's OUTWARD normal from its plane's chart
/// normal — the by-value door, for a caller that has already matched
/// the face's surface as a plane and holds the [`Face`] itself.
///
/// INVARIANT: the bit comes from the face handed in, never from a
/// caller's local, and `chart_normal` is that face's plane normal —
/// the caller's arm has just destructured it. Bit-identical to
/// [`face_outward_normal`] on the same face.
pub(crate) fn plane_outward_normal<T: Real>(
    face: &Face,
    chart_normal: Vec3<T>,
) -> OutwardNormal<T> {
    OutwardNormal::from_chart(chart_normal, face.sense)
}

/// A face's OUTWARD normal at a point on a curved carrier, by value:
/// the implicit gradient at `p`, normalized, folded through the face's
/// `sense` bit — for a door handed a carrier and the bit rather than a
/// key (a contact verifier reading two bodies' faces).
///
/// INVARIANT: `p` is certified ON `surface` by the caller before the
/// normal is read (an off-surface gradient is a direction of nothing),
/// and `sense` is a [`Face::sense`] the caller resolved, never a
/// decided sign. Unlike [`face_outward_normal_at`] this door gates
/// nothing itself, which is why it is `pub(crate)` and takes the bit.
pub(crate) fn implicit_outward_normal<T: Real>(
    surface: &geom::Surface<T>,
    sense: bool,
    p: Point3<T>,
) -> OutwardNormal<T> {
    OutwardNormal::from_chart(geom_brep::implicit_gradient(surface, p).normalize(), sense)
}

/// A planar face's OUTWARD normal — the chart normal with the face's
/// `sense` folded in, minted as an [`OutwardNormal`] so no consumer
/// can multiply again.
///
/// INVARIANT: this is where the planar lane's sense flip lives, and
/// the module docs list who goes through it. `None` for a non-planar
/// face (the caller falls through to its own curved arms, or has
/// none).
pub(crate) fn face_outward_normal<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Option<OutwardNormal<T>> {
    let f = body.get_face(face)?;
    match body.get_surface(f.surface) {
        Some(geom::Surface::Plane { normal, .. }) => Some(plane_outward_normal(f, *normal)),
        _ => None,
    }
}

/// Why [`face_outward_normal_at`] could not answer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum NormalAtError {
    /// The margin certifying `p` on the chart landed in the band.
    Escalated(Indeterminate),
    /// `p` is definitely NOT on the face's surface, so the surface has
    /// no normal there to fold a sense into. The door's contract is a
    /// point ON the face, so this is the caller's invariant, not a
    /// remainder.
    OffSurface,
}

/// A face's OUTWARD normal **at a point on it** — the same one door,
/// widened from a planar datum to a point-dependent one.
///
/// INVARIANT: the sense flip is still folded here and only here. On a
/// [`geom::Surface::Plane`] the answer is point-independent and
/// BIT-IDENTICAL to [`face_outward_normal`]'s, which is what lets the
/// pierce lane substitute this door for that one without moving a
/// planar result.
///
/// The curved arm is the implicit gradient
/// ([`geom_brep::implicit_gradient`]), which is unit-magnitude on the
/// surface and is therefore the chart normal there — computed from the
/// implicit form rather than from the chart, because the chart normal
/// of a cylinder/sphere is a `(u, v)` derivative cross product this
/// layer has no business re-deriving. `sense: true` means the outward
/// normal IS that chart normal, pointing away from the axis/centre
/// (the convention `boolean::rest::face_carrier` states).
///
/// **The gate is on the kind and on the point, in that order.** The
/// gradient is honest poison where the surface itself is singular — a
/// cone apex or axis, a torus axis — and `Nurbs`/`Approx` have no
/// implicit form at all, so those kinds get `Ok(None)` and the caller
/// mints its own typed refusal naming the kind. For the kinds that DO
/// have an arm, `p` is certified onto the chart first: the gradient's
/// magnitude is 1 exactly on the surface and degenerates to `0/0` on
/// the singular locus, so `‖∇F‖ − 1` levered by
/// [`geom_brep::curvature_lever_arm`] — the local radius of curvature,
/// the chart's own length scale — is both the on-surface certificate
/// and the singularity guard, in one margin.
///
/// **The degenerate gradient is a ZERO vector, not a poison one**, and
/// the distinction matters because only one of the two is caught by a
/// comparison: on the axis of a cylinder (or at a sphere's centre) the
/// radial component is the zero vector and `w / radius` is `0`, whose
/// norm is a perfectly ordinary `0` — so the margin `0 − 1` classifies
/// definitely negative and the guard fires on a value it can read.
/// A cone apex, where the form really is `0/0`, never reaches this
/// margin at all: the kind has no arm here.
///
/// # Errors
///
/// [`NormalAtError`] — an in-band chart certificate, or a point
/// definitely off the surface.
pub(crate) fn face_outward_normal_at<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    p: Point3<T>,
    band: Band,
) -> Result<Option<OutwardNormal<T>>, NormalAtError> {
    let Some(f) = body.get_face(face) else {
        return Ok(None);
    };
    let Some(surface) = body.get_surface(f.surface) else {
        return Ok(None);
    };
    match surface {
        geom::Surface::Plane { normal, .. } => Ok(Some(plane_outward_normal(f, *normal))),
        geom::Surface::Cylinder { .. } | geom::Surface::Sphere { .. } => {
            let arm = geom_brep::curvature_lever_arm(surface, p);
            let grad = geom_brep::implicit_gradient(surface, p);
            let margin = Margin::levered(grad.norm() - T::one(), arm);
            match decide("bool_pierce_normal_on_chart", margin, band) {
                Ok(Sign::Zero) => Ok(Some(OutwardNormal::from_chart(grad, f.sense))),
                Ok(Sign::Positive | Sign::Negative) => Err(NormalAtError::OffSurface),
                Err(diag) => Err(NormalAtError::Escalated(diag)),
            }
        }
        // Cone, Torus, Nurbs, Approx: no arm here. A cone's gradient is
        // `0/0` on its whole axis (the apex included) and a torus's on
        // its axis; the NURBS/Approx pair has no implicit form to
        // differentiate. The caller names the kind in its own refusal
        // rather than this door guessing which refusal it wants.
        _ => Ok(None),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::{Band, Point3, Tol, Vec3};

    use super::{NormalAtError, face_outward_normal, face_outward_normal_at};
    use crate::euler::FaceSurface;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// A one-face skeletal body whose face carries `surface`.
    fn face_on(surface: geom::Surface<f64>) -> (crate::body::Body<f64>, crate::entity::FaceKey) {
        let st = crate::fixtures::mvfs_state();
        let mut body = st.body;
        body.set_face_surface(st.face, FaceSurface::New(surface))
            .unwrap();
        (body, st.face)
    }

    /// **The bit-identity row for the widened door.** On a plane the
    /// point-dependent arm must return the SAME vector the
    /// point-independent one does, for both senses — that equality is
    /// the whole argument that substituting the new door into the
    /// pierce lane cannot move a planar result.
    #[test]
    fn on_a_plane_the_two_doors_agree_bit_for_bit() {
        let normal = Vec3::new(0.0, 0.0, 1.0);
        let (mut body, face) = face_on(geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 3.0),
            normal,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        });
        for sense in [true, false] {
            body.set_face_sense(face, sense).unwrap();
            let flat = face_outward_normal(&body, face).unwrap();
            // The flip itself, on the planar door: the chart normal
            // where the bit is set, its negation where it is not.
            let expected = if sense { normal } else { -normal };
            assert!(
                flat.vec().x.to_bits() == expected.x.to_bits()
                    && flat.vec().y.to_bits() == expected.y.to_bits()
                    && flat.vec().z.to_bits() == expected.z.to_bits(),
                "sense {sense}: the planar door answered {:?}, not {expected:?}",
                flat.vec()
            );
            // Deliberately far off the plane: the planar arm must not
            // have grown a dependence on the point.
            for p in [Point3::new(0.0, 0.0, 3.0), Point3::new(7.0, -2.0, -11.0)] {
                let at = face_outward_normal_at(&body, face, p, band())
                    .unwrap()
                    .unwrap();
                assert!(
                    at.vec().x == flat.vec().x
                        && at.vec().y == flat.vec().y
                        && at.vec().z == flat.vec().z,
                    "sense {sense} at {p:?}: {:?} vs {:?}",
                    at.vec(),
                    flat.vec()
                );
            }
        }
    }

    /// The curved arm: on a wall the outward normal is RADIAL at the
    /// point, and a reversed face points into the material — the same
    /// sense convention the planar arm has, now varying with `p`.
    #[test]
    fn on_a_wall_the_normal_is_radial_at_the_point() {
        let (mut body, face) = face_on(geom::Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 2.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        });
        for (p, radial) in [
            (Point3::new(2.0, 0.0, 5.0), Vec3::new(1.0, 0.0, 0.0)),
            (Point3::new(0.0, -2.0, -1.0), Vec3::new(0.0, -1.0, 0.0)),
        ] {
            body.set_face_sense(face, true).unwrap();
            let out = face_outward_normal_at(&body, face, p, band())
                .unwrap()
                .unwrap();
            assert!(
                (out.vec() - radial).norm() == 0.0,
                "outward at {p:?}: {:?}",
                out.vec()
            );
            body.set_face_sense(face, false).unwrap();
            let inward = face_outward_normal_at(&body, face, p, band())
                .unwrap()
                .unwrap();
            assert!(
                (inward.vec() + radial).norm() == 0.0,
                "reversed at {p:?}: {:?}",
                inward.vec()
            );
        }
    }

    /// The two halves of the gate, in one row each: a kind with no
    /// implicit normal to fold answers `None` (the caller mints the
    /// typed refusal naming it), and a point definitely off the chart
    /// is the caller's broken invariant rather than a remainder.
    #[test]
    fn the_gate_separates_a_missing_arm_from_a_point_off_the_chart() {
        let (body, face) = face_on(geom::Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::new(0.0, 0.0, 1.0),
            half_angle: 0.5,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        });
        assert!(
            matches!(
                face_outward_normal_at(&body, face, Point3::new(1.0, 0.0, 2.0), band()),
                Ok(None)
            ),
            "a cone has no arm here"
        );
        let (body, face) = face_on(geom::Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 2.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        });
        assert!(matches!(
            face_outward_normal_at(&body, face, Point3::new(5.0, 0.0, 0.0), band()),
            Err(NormalAtError::OffSurface)
        ));
        // The axis is where the gradient is 0/0, and the same margin
        // catches it: poison is never read.
        assert!(matches!(
            face_outward_normal_at(&body, face, Point3::new(0.0, 0.0, 1.0), band()),
            Err(NormalAtError::OffSurface)
        ));
    }

    /// **The anti-re-fork row for the planar sense flip.** The plane
    /// arm of this walk goes through [`crate::face_normal`], the one
    /// door, and no file under `topo/src` other than that one may both
    /// destructure a plane surface pattern for its normal and mint an
    /// [`OutwardNormal`] from a chart — which is what a second flip
    /// looks like textually. (The pattern is spelled ONLY in the check
    /// itself: writing it in this comment made the guard's home its own
    /// first counter-example.)
    ///
    /// **What it cannot match** — four shapes, said plainly:
    ///
    /// 1. **A flip written without `from_chart`** — a chart normal
    ///    negated under `if !face.sense` by hand and reaching a
    ///    material verdict. This row does not see those; what stands
    ///    against them is that [`crate::entity::Face`] carries no scalar sign to
    ///    multiply by, so the hand flip has to be spelled out where a
    ///    reviewer reads it.
    /// 2. **A flip in another crate.** The walk is scoped to
    ///    `topo/src`.
    /// 3. **A caller that takes the door's answer and negates it.**
    ///    [`OutwardNormal`] is a wrapper, not a capability: `.vec()` is
    ///    public and a consumer can multiply what comes back.
    /// 4. **A file that reads the plane normal through a helper** —
    ///    `face_plane(..).normal` — and flips that. No plane-surface
    ///    pattern appears, so the textual pair never matches.
    #[test]
    fn the_planar_sense_flip_lives_in_one_place() {
        let home = crate::source_walk::src_root().join("face_normal.rs");
        let files = crate::source_walk::crate_sources();
        assert!(
            files.contains(&home),
            "the walk did not find face_normal.rs"
        );
        for path in &files {
            if path == &home {
                continue;
            }
            let text = std::fs::read_to_string(path).expect("a readable source file");
            assert!(
                !(text.contains("Surface::Plane {") && text.contains("from_chart")),
                "{} mints an OutwardNormal from a plane's chart normal — the planar \
                 sense flip has been re-forked out of face_normal.rs, which must hold \
                 the only one. Call `face_outward_normal` instead.",
                path.display()
            );
        }
        let here = std::fs::read_to_string(&home).expect("the home module is readable");
        assert!(
            here.contains("Surface::Plane {") && here.contains("from_chart"),
            "the door no longer mints the flip, so this row guards nothing"
        );
    }
}
