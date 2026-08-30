//! The **one door** for a face's outward normal — planar or curved:
//! the single place the `sense` bit is folded into a chart normal.
//!
//! Two doors, one flip: [`face_outward_normal`] answers for a PLANE,
//! where the normal does not depend on where you stand, and
//! [`face_outward_normal_at`] answers at a POINT, which is what a
//! curved carrier requires and what the plane arm returns unchanged.
//!
//! # Why one door, and why here
//!
//! Since S10 a face's outward normal is its surface's chart normal
//! times [`Face::sense_sign`](crate::entity::Face::sense_sign) — the
//! chart is the only place orientation was ever encoded, so on a
//! `sense: false` face the stored normal points INTO the material and
//! every consumer reading a material direction off it answers
//! backwards. One flip, in one function, is what keeps two flips from
//! drifting apart.
//!
//! The door used to live in `boolean::reduce`, whose invariant named
//! its five consumers — the boolean's `plane_of`, the reduction sweep,
//! the pierce lane, the REST lane, and `sector_face`. When
//! `sector_face` became [`crate::sector_face`], shared with the
//! splitting lane, that consumer moved OUT of `boolean/` and the door
//! could no longer be the one door from inside it: a crate-root module
//! importing from `boolean/` would be the same wrong-way edge, pointed
//! the other way. So the door moved to the crate root and
//! `boolean::reduce` re-exports it — the boolean's four remaining
//! consumers are unchanged, and the shared walk uses the same flip
//! rather than a second one. **Consumers here are lanes, not call
//! sites**: three of the four reach the door through
//! `boolean::reduce::face_plane` (`recl`'s `plane_of`, the reduction
//! sweep, the REST lane) and only the pierce lane (`vtxfac`) calls it
//! directly, so counting `face_outward_normal` call sites gives two
//! and does not falsify the four.
//!
//! **"One door" is true of these consumers, not of the workspace.**
//! Other outward normals are still built by hand-multiplying a chart
//! normal by [`Face::sense_sign`](crate::entity::Face::sense_sign),
//! and this module is where they belong when smell-scan D6 is
//! executed. **Where they are is computed, not recited**: the guard
//! test below walks `topo/src`, counts every occurrence of that method
//! in code, and fails on one its dispositioned table does not carry.
//! The count is of the whole tree this crate can see; the rest of the
//! workspace is D6's scope and is not enumerated here.
//!
//! **A better home exists and is not reachable from this lane.** The
//! function is a `Body` query, exactly like `Body::mate` or
//! `Body::loop_cycle`, and belongs as an inherent method on
//! [`Body`]. That is `body.rs`, outside the scope
//! this module was created under; it is recorded on issue #695 with
//! the rest of the placement questions.

use geom_brep::OutwardNormal;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign};

use crate::body::Body;
use crate::entity::FaceKey;
use crate::validate::decide;

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
        Some(geom::Surface::Plane { normal, .. }) => {
            Some(OutwardNormal::from_chart(*normal, f.sense))
        }
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
        geom::Surface::Plane { normal, .. } => {
            Ok(Some(OutwardNormal::from_chart(*normal, f.sense)))
        }
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
    ///    multiplied by the face's own `±1` and reaching a material
    ///    verdict. This row does not see those; the inventory row
    ///    below does, and names each one.
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
                 sense flip has been re-forked out of face_normal.rs (smell scan S5 / \
                 S10). Call `face_outward_normal` instead.",
                path.display()
            );
        }
        let here = std::fs::read_to_string(&home).expect("the home module is readable");
        assert!(
            here.contains("Surface::Plane {") && here.contains("from_chart"),
            "the door no longer mints the flip, so this row guards nothing"
        );
    }

    /// **The hand-multiply inventory** — smell-scan D6's `grep
    /// sense_sign` written as a gate rather than as a sentence. Every
    /// **occurrence** of [`Face::sense_sign`](crate::entity::Face::sense_sign)
    /// in the CODE of `topo/src` (comments and literal bodies removed
    /// by [`crate::fixtures::code_only`]) is counted per file and
    /// pinned below with its disposition, so a new hand-multiply
    /// cannot land without either going through the door or being
    /// argued for in the table:
    ///
    /// - `entity.rs` — the definition itself.
    /// - `boolean/solid_contain.rs` — `face_plane` and `face_geo`, two
    ///   plane arms whose consumers are ray-parity walks and therefore
    ///   blind to the sign; they thread it to keep the doors' stated
    ///   contract (an OUTWARD normal) honest for the next consumer.
    /// - `boolean/join.rs` — `ring_run_ccw`, which picks an island's
    ///   new outer boundary and needs exactly one of its two signs
    ///   threaded.
    /// - `boolean/rest.rs` — `face_carrier`, the curved generalization
    ///   of the door; it binds the `±1` to a local before multiplying,
    ///   which is the form a textual `* ….sense_sign` sweep misses.
    /// - `merge_faces.rs` — two sense-tuple reads in the coplanarity
    ///   gate and the survivor's plane hand-multiply.
    /// - `props.rs` — the `±1` handed to `curved_face`'s closed form.
    ///   Not a normal multiply, and the only read whose consumer is a
    ///   curved carrier.
    /// - `validate.rs` — check 6's outward normal, where the sense bit
    ///   is read as a claim to be falsified rather than honored.
    /// - `face_normal.rs` — **zero**: the door takes the sense BIT
    ///   through [`OutwardNormal::from_chart`], never the `±1`. That
    ///   is why the walk below reads the method name out of `concat!`
    ///   — spelled whole, this file would be its own first hit.
    ///
    /// **The pin is per FILE, and that is wider than the invariant.**
    /// Moving a read from one file to another changes nothing about
    /// the sense flip and still reds this row; so does adding a second
    /// occurrence to a line that already has one. The narrower pin — a
    /// total plus the dispositions — would red for fewer wrong
    /// reasons, and the per-file shape is chosen because *which* file
    /// carries a read is the only thing that makes the disposition
    /// list above checkable. The cost is a tripwire over all of
    /// `topo/src` in a tree several lanes are editing at once.
    ///
    /// **What this cannot match**, and it is a work order rather than
    /// a discharge:
    ///
    /// 1. A flip written through a helper that already returns an
    ///    outward normal — `face_plane(..).normal` multiplied again.
    /// 2. A [`crate::entity::Face::sense`] bool read and branched on
    ///    without the `±1` (`step-export`'s `same_sense` bit is such a
    ///    consumer, and a legitimate one).
    /// 3. An identifier a macro assembles, which no textual walk sees.
    /// 4. **Every crate but this one.** The walk is `topo/src` because
    ///    that is the tree this crate can see, and the workspace half
    ///    is D6's. **It is deliberately not enumerated here**: a roster
    ///    of other crates is exactly the artifact this row replaced,
    ///    and reciting one beside a computed inventory would mint the
    ///    same defect one level out. The out-of-crate readers are
    ///    inventoried once, in `docs/SMELL-SCAN-2026-08.md` at S67,
    ///    beside D6's work order — a list that is recited and says so,
    ///    in the document where a work order belongs.
    #[test]
    fn every_hand_multiply_of_the_face_sign_is_inventoried() {
        const PINNED: [(&str, usize); 8] = [
            ("boolean/join.rs", 1),
            ("boolean/rest.rs", 1),
            ("boolean/solid_contain.rs", 2),
            ("entity.rs", 1),
            ("face_normal.rs", 0),
            ("merge_faces.rs", 3),
            ("props.rs", 1),
            ("validate.rs", 1),
        ];
        let needle = concat!("sense", "_sign");
        let root = crate::source_walk::src_root();
        let mut found: Vec<(String, usize)> = Vec::new();
        for path in crate::source_walk::crate_sources() {
            let text = std::fs::read_to_string(&path).expect("a readable source file");
            let reads = crate::fixtures::code_only(&text).matches(needle).count();
            let rel = path
                .strip_prefix(&root)
                .expect("a walked file lies under topo/src")
                .to_string_lossy()
                .replace('\\', "/");
            if reads > 0 || PINNED.iter().any(|(pinned, _)| *pinned == rel) {
                found.push((rel, reads));
            }
        }
        found.sort();
        let pinned: Vec<(String, usize)> = PINNED
            .iter()
            .map(|(path, reads)| ((*path).to_string(), *reads))
            .collect();
        assert_eq!(
            found, pinned,
            "the inventory of hand-multiplies moved: an occurrence of the face's own ±1 was \
             added, removed or relocated. Route it through `face_outward_normal` if it wants \
             an outward normal; otherwise add it to this table with its disposition (D6)."
        );
    }
}
