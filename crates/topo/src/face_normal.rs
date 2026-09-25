//! The **door** for a face's outward normal — planar or curved: where
//! this crate's PLANAR consumers fold the `sense` bit INTO a chart
//! normal, and the gate its point-dependent curved consumers pass.
//! Handing the bit onward is the other legitimate answer and is not a
//! second fold — a `geom_brep` door that mints the outward normal from
//! gradients of its own takes the bit and leaves nothing here to fold,
//! and the shared [`crate::sector_face`] walk names the constructor
//! itself for its curved arms.
//!
//! Three doors, one flip. By key: [`face_outward_normal`] answers for
//! a PLANE, where the normal does not depend on where you stand, and
//! [`face_outward_normal_at`] answers at a POINT, which is what a
//! curved carrier requires and what the plane arm returns unchanged.
//! By value, for a caller that has already resolved the face and must
//! not discard a failed lookup: [`plane_outward_normal`] takes the
//! [`Face`] and its plane's chart normal. Every one of them spells the
//! fold through [`OutwardNormal::from_chart`], the type's only
//! constructor, and this file is the only one under `topo/src` that
//! names it in a file that ALSO destructures a plane pattern — that
//! conjunction, not the constructor's name alone, is what the guard
//! row below reads.
//!
//! **Two curved readings, and they differ.** [`face_outward_normal_at`]
//! is a GATE: it certifies `p` onto the chart (`‖∇F‖ − 1` in band) and
//! folds the RAW gradient it has just certified unit-magnitude. The
//! by-value curved fold is [`geom_brep::implicit_outward_normal`],
//! which normalizes and gates nothing — its callers (the contact
//! verifier, the dihedral's material pairing, a blend battery's
//! supports) certify the point themselves. The two can differ in the
//! last ulps on the same input, so neither substitutes for the other:
//! a caller that wants the gate takes the key; a caller handed a
//! carrier and the bit takes the `geom_brep` door.
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
//! wants an outward normal comes here, names the constructor itself
//! from outside this crate, or hands the bit to a `geom_brep` door
//! that mints one from gradients of its own. What holds that in place
//! mechanically is two rows in this file's tests, each naming what it
//! cannot see: `the_planar_sense_flip_lives_in_one_place` (no other
//! `topo/src` file re-forks the planar fold) and
//! `no_source_file_folds_the_bit_by_hand` (no `crates/*/src` file
//! negates a value under the bit by hand, the sanctioned scalar
//! negations named).
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
/// INVARIANT: the bit comes from the face handed in — a type fact —
/// and `chart_normal` is that face's plane normal, which nothing here
/// checks: the caller's arm has just destructured it, and the
/// enforcement is the caller's. Bit-identical to
/// [`face_outward_normal`] on the same face.
pub(crate) fn plane_outward_normal<T: Real>(
    face: &Face,
    chart_normal: Vec3<T>,
) -> OutwardNormal<T> {
    OutwardNormal::from_chart(chart_normal, face.sense)
}

/// A planar face's OUTWARD normal — the chart normal with the face's
/// `sense` folded in, minted as an [`OutwardNormal`] so no consumer
/// can multiply again. The keyed door, and the one a consumer outside
/// this crate holding a body and a face key takes (a blend's planar
/// supports); the module docs list the in-crate callers.
///
/// `None` for a non-planar face (the caller falls through to its own
/// curved arms, or has none), and for a face or surface key that no
/// longer resolves.
pub fn face_outward_normal<T: Decide>(body: &Body<T>, face: FaceKey) -> Option<OutwardNormal<T>> {
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
/// cone apex, which lies ON the cone — and `Nurbs`/`Approx` have no
/// implicit form at all, so those kinds get `Ok(None)` and the caller
/// mints its own typed refusal naming the kind. A ring torus is
/// singular only on its axis, which no point of the tube reaches
/// (`ρ ≥ R − r > 0`), so it has an arm: a point the margin below
/// certifies onto the tube is off the axis, and one ON the axis reads
/// `0/0`, which the margin escalates rather than classifies. For the kinds that DO
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
        geom::Surface::Cylinder { .. }
        | geom::Surface::Sphere { .. }
        | geom::Surface::Torus { .. } => {
            let arm = geom_brep::curvature_lever_arm(surface, p);
            let grad = geom_brep::implicit_gradient(surface, p);
            let margin = Margin::levered(grad.norm() - T::one(), arm);
            match decide("bool_pierce_normal_on_chart", margin, band) {
                Ok(Sign::Zero) => Ok(Some(OutwardNormal::from_chart(grad, f.sense))),
                Ok(Sign::Positive | Sign::Negative) => Err(NormalAtError::OffSurface),
                Err(diag) => Err(NormalAtError::Escalated(diag)),
            }
        }
        // Cone, Nurbs, Approx: no arm here. A cone's gradient is `0/0`
        // on its whole axis, the apex ON the surface included; the
        // NURBS/Approx pair has no implicit form to differentiate. The
        // caller names the kind in its own refusal rather than this
        // door guessing which refusal it wants.
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
    /// looks like textually. The row reads each file's CODE view
    /// (comments and literals blanked), so it sees mints and not
    /// mentions: a doc comment may name the constructor, which is what
    /// the door's design asks of its callers. (The pattern is spelled
    /// ONLY in the check itself, and in code: writing it in this
    /// comment once made the guard's home its own first
    /// counter-example.)
    ///
    /// **What it cannot match** — five shapes, said plainly:
    ///
    /// 1. **A flip written without `from_chart`** — a chart normal
    ///    negated under `if !face.sense` by hand and reaching a
    ///    material verdict. This row does not see those;
    ///    `no_source_file_folds_the_bit_by_hand` below is the row that
    ///    does, tree-wide, with its own blind spots.
    /// 2. **A flip in another crate.** The walk is scoped to
    ///    `topo/src`; the sibling row walks `crates/*/src`.
    /// 3. **A caller that takes the door's answer and negates it.**
    ///    [`OutwardNormal`] is a wrapper, not a capability: `.vec()` is
    ///    public and a consumer can multiply what comes back.
    /// 4. **A file that reads the plane normal through a helper** —
    ///    `face_plane(..).normal` — and flips that. No plane-surface
    ///    pattern appears, so the textual pair never matches.
    /// 5. **A mint inside a `macro_rules!` body or assembled from a
    ///    string literal.** The code view blanks literals and does not
    ///    expand macros.
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
            let code = test_utils::source::code_only(&text);
            assert!(
                !(code.contains("Surface::Plane {") && code.contains("from_chart")),
                "{} mints an OutwardNormal from a plane's chart normal — the planar \
                 sense flip has been re-forked out of face_normal.rs, which must hold \
                 the only one. Call `face_outward_normal` instead.",
                path.display()
            );
        }
        let here = test_utils::source::code_only(
            &std::fs::read_to_string(&home).expect("the home module is readable"),
        );
        assert!(
            here.contains("Surface::Plane {") && here.contains("from_chart"),
            "the door no longer mints the flip, so this row guards nothing"
        );
    }

    /// The hand folds of a face's bit in one source file's CODE view:
    /// for every `if … <ident containing "sense"> … { A } else { -A }`
    /// (and the reversed `{ -A } else { A }`), the negated `A`. The
    /// view has comments and string literals blanked and its whitespace
    /// collapsed, so a comment cannot hit and a rustfmt line break
    /// cannot hide an arm.
    fn hand_folds_of_sense(code: &str) -> Vec<String> {
        let view = code.split_whitespace().collect::<Vec<_>>().join(" ");
        let is_ident = |c: char| c.is_alphanumeric() || c == '_';
        let mut out = Vec::new();
        let mut from = 0;
        while let Some(at) = view[from..].find("} else {") {
            let close = from + at;
            from = close + "} else {".len();
            let Some(open) = view[..close].rfind('{') else {
                continue;
            };
            let a = view[open + 1..close].trim();
            let Some(end) = view[from..].find('}') else {
                continue;
            };
            let b = view[from..from + end].trim();
            if a.is_empty() || b.is_empty() || a.contains(';') || b.contains(';') {
                continue;
            }
            let negation = b == format!("-{a}") || a == format!("-{b}");
            if !negation {
                continue;
            }
            // The condition: from the nearest `if` back to the arm.
            let Some(if_at) = view[..open].rfind("if ") else {
                continue;
            };
            let cond = &view[if_at + 3..open];
            let conditioned_on_sense = cond
                .split(|c: char| !is_ident(c))
                .any(|word| word.contains("sense"));
            if conditioned_on_sense {
                out.push(a.trim_start_matches('-').to_string());
            }
        }
        out
    }

    /// **The tree-wide absence pin for the class D6 names**: a value
    /// negated by hand under a face's `sense` bit — a chart normal, an
    /// axis, a `±1` — in any file under `crates/*/src`. The type
    /// system retires the literal `sense_sign` accessor; it cannot see
    /// `if f.sense { *normal } else { -*normal }`, and `Face::sense`
    /// is `pub`, so this row is what covers the shape the compiler
    /// does not. Its expected hit set is the SANCTIONED scalar
    /// negations — the spelling D6 §0 prescribes for a genuinely
    /// signed scalar, which is textually the same shape — each named
    /// with what it negates, so a new hit is either a vector fold that
    /// belongs at a door or a scalar that is added here in the open
    /// with its reason. The vector class's expected count is zero.
    ///
    /// Exempt: the constructor's own home (`geom-brep/src/enters.rs`)
    /// and this file (the planar door's flip row spells the
    /// expectation by hand on purpose).
    ///
    /// **What it cannot match**: a bit renamed at a boundary before it
    /// is folded (`side`, `inside`, `reversed` — the blend arms' `sided`
    /// is keyed on `side`); a fold spelled as a `copysign`, a multiply
    /// by a `±1` local minted elsewhere, a `match` on the bool, or an
    /// arm whose expression contains a `;` or a brace; a fold in
    /// `crates/*/tests`, `demos/`, `tools/` or `benches/`, which the
    /// walk does not enter; a macro body. The walk is over the source
    /// tree at test time, so a crate outside `crates/` is not seen.
    #[test]
    fn no_source_file_folds_the_bit_by_hand() {
        let root = test_utils::source::repo_root(env!("CARGO_MANIFEST_DIR"));
        let exempt = [
            "crates/geom-brep/src/enters.rs",
            "crates/topo/src/face_normal.rs",
        ];
        // Each a genuinely signed SCALAR, negated at its point of use
        // as D6 spells it: a jet curvature measured along the plus
        // face's outward normal, a loop's chart-frame area, an offset
        // distance into the material.
        let sanctioned: Vec<(String, String)> = [
            ("crates/geom-brep/src/dihedral.rs", "kappa_rel"),
            ("crates/mesh/src/walk.rs", "area"),
            ("crates/topo/src/shell.rs", "thickness"),
        ]
        .into_iter()
        .map(|(f, a)| (f.to_string(), a.to_string()))
        .collect();
        let mut hits = Vec::new();
        let mut walked = 0usize;
        for entry in std::fs::read_dir(root.join("crates")).expect("crates/ is readable") {
            let src = entry.expect("a readable entry").path().join("src");
            if !src.is_dir() {
                continue;
            }
            for path in test_utils::source::rust_sources(&src) {
                let rel = path
                    .strip_prefix(&root)
                    .expect("under the repository root")
                    .to_string_lossy()
                    .replace('\\', "/");
                if exempt.contains(&rel.as_str()) {
                    continue;
                }
                walked += 1;
                let text = std::fs::read_to_string(&path).expect("a readable source file");
                let code = test_utils::source::code_only(&text);
                for negated in hand_folds_of_sense(&code) {
                    hits.push((rel.clone(), negated));
                }
            }
        }
        assert!(
            walked > 100,
            "the walk saw {walked} files — it is not reading crates/*/src"
        );
        hits.sort();
        assert_eq!(
            hits, sanctioned,
            "a value is negated by hand under a face's sense bit outside the sanctioned \
             scalar negations. A NORMAL folds through `OutwardNormal::from_chart` or a \
             `face_normal` door; a genuinely signed scalar is named in this row's list \
             with what it negates."
        );
    }

    /// The scanner's own reach, pinned so the row above cannot go
    /// green by matching nothing: the two shapes it exists for, in
    /// both orders and across a line break, and the two it must not
    /// read — a negation keyed on something other than the bit, and a
    /// pair that is not a negation.
    #[test]
    fn the_hand_fold_scanner_sees_both_shapes_and_nothing_else() {
        let seen = hand_folds_of_sense(
            "let n = if f.sense { *normal } else { -*normal };\n\
             let s = if sense_plus {\n    T::one()\n} else {\n    -T::one()\n};\n\
             let r = if !data.sense { -axis } else { axis };\n\
             let k = if inside { rim - s } else { -(rim - s) };\n\
             let m = if pose.sense { a } else { b };",
        );
        assert_eq!(seen, vec!["*normal", "T::one()", "axis"]);
    }

    /// **The torus arm is the tube's own normal, and it refuses on the
    /// axis.** At a point a general minor angle round the tube the
    /// outward normal is the offset from the point's foot on the core
    /// circle, over `r` — not the radial from the axis (a cylinder's
    /// answer) and not the offset from the centre (a sphere's), both of
    /// which this pose tells apart. Flipped by the sense bit like every
    /// other arm. A point ON the axis is the torus's one singular locus;
    /// no tube point is there, and the door must refuse rather than
    /// hand the pierce lane a direction of nothing.
    #[test]
    fn the_torus_arm_is_the_tube_normal_and_refuses_on_the_axis() {
        let (big_r, r, v) = (0.8, 0.5, 2.3_f64);
        let (mut body, face) = face_on(geom::Surface::Torus {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 1.0, 0.0),
            major_radius: big_r,
            minor_radius: r,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        });
        let (sv, cv) = v.sin_cos();
        let p = Point3::new(big_r + r * cv, r * sv, 0.0);
        let tube = Vec3::new(cv, sv, 0.0);
        for sense in [true, false] {
            body.set_face_sense(face, sense).unwrap();
            let n = face_outward_normal_at(&body, face, p, band())
                .unwrap()
                .expect("the torus has an arm")
                .vec();
            let want = geom_brep::OutwardNormal::from_chart(tube, sense).vec();
            assert!(
                (n - want).norm() < 1e-12,
                "sense {sense}: {n:?} is not the tube normal {want:?}"
            );
        }
        assert!(
            face_outward_normal_at(&body, face, Point3::new(0.0, 0.2, 0.0), band()).is_err(),
            "a point on the torus axis has no normal and must refuse"
        );
    }
}
