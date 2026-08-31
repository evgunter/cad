//! Frame constructors: the placement vocabulary, as plain [`Affine3`]
//! values.
//!
//! Three constructors, each answering a pain row of the corpus survey
//! (`docs/LIBRARY-DESIGN.md` §L2):
//!
//! - [`point_at`] — a frame at an eye position whose local **+Z** aims
//!   at a target, roll fixed by an explicit reference (P6: manual
//!   axis-angle placement with antiparallel special-casing, and no
//!   point-at affordance anywhere).
//! - [`path_start_frame`] — the profile frame at the start of a swept
//!   path: local **+Z** is the path tangent, so the local XY plane is
//!   the profile plane (P1: the Gram–Schmidt recipe hand-rolled at
//!   every sweep call site, each copy carrying its own
//!   degenerate-axis dodge).
//! - [`mirror_across_plane`] — reflection across a plane (P6: no
//!   mirror anywhere, so symmetric arrangements are placed by hand,
//!   one leaf at a time).
//!
//! Nothing here is a new type. A frame IS an [`Affine3`]: the linear
//! part's columns are the frame's local +X/+Y/+Z axes in world
//! coordinates, and the translation is the frame's origin. Composition,
//! inversion, and application are [`Affine3`]'s, unchanged.
//!
//! # The roll convention (one sentence, pinned)
//!
//! For both aiming constructors the frame's local **+Z is the aim
//! direction**, and the **roll reference lies in the local +Y
//! half-plane**: the reference's component perpendicular to the aim is
//! a positive multiple of local +Y, and the reference has no local +X
//! component. Equivalently, with `ẑ` the unit aim and `r` the
//! reference: `x̂ = normalize(r × ẑ)`, `ŷ = ẑ × x̂`. The frame is
//! right-handed (`det = +1`), so `x̂ × ŷ = ẑ`.
//!
//! # The degenerate-axis policy (a rule, not a dodge)
//!
//! An aim direction alone does not determine a frame — the roll is
//! free — so every constructor needs a reference direction off the aim
//! line. The hand-rolled twins picked one with a magic-constant dodge
//! (`if n.z.abs() < 0.9 { e_z } else { e_x }`). The policy here:
//!
//! 1. **Every degeneracy is decided, never guessed.** The aim length
//!    and the reference's perpendicular offset are both *lengths*, and
//!    both are classified through the one predicate funnel against the
//!    run's linear band — so "too short to aim with" and "too close to
//!    the aim line to roll with" are tolerance decisions, not
//!    hard-coded thresholds.
//! 2. **[`point_at`] takes its reference explicitly and refuses.** The
//!    caller stated the roll; a reference on the aim line is a
//!    modelling mistake, and guessing a different one would silently
//!    change the answer the caller asked for.
//! 3. **[`path_start_frame`] has a named ladder.** No reference is
//!    authored (a sweep's profile frame is conventional), so the
//!    ladder is world **+Z, then world +X**, in that order; a rung is
//!    taken only on a *definite* off-axis decision, and both the
//!    coincident and the ambiguous outcomes advance to the next rung
//!    (an ambiguous reference is not a usable reference).
//! 4. **True degeneracy refuses, typed.** A zero-length or poisoned
//!    tangent refuses [`FrameInput::Tangent`]; a unit tangent can
//!    never miss both ladder rungs (world +Z and +X are orthogonal),
//!    so [`FrameInput::ReferenceLadder`] is reachable only for input
//!    that is not a direction at all — and it refuses rather than
//!    inventing a frame.
//!
//! The ladder is a *convention*, and conventions are discontinuous:
//! the frame flips as the tangent crosses the ladder's switch-over.
//! That is the hairy-ball residue [`Vec3::orthonormal_basis`] documents
//! for its own construction; consumers wanting a frame stable across a
//! parameter change store it as data (D2) rather than re-deriving it.
//!
//! # Resonance with the PATHS placement family (LQ3(c) amendment)
//!
//! The PATHS algebra (`docs/PATHS-DESIGN.md` §2) already has a
//! placement vocabulary for putting an authored curve value onto a
//! bound tip. This family uses **the same words for the same
//! meanings**:
//!
//! | This module | PATHS term | The shared meaning | Where the two differ |
//! |---|---|---|---|
//! | [`point_at`] | `nurbs(curve)` | **Placement is rigid**: a translation and a rotation taking one direction onto another. No scale, no deformation — the map places, it never edits. | PATHS spends the tip's 2 + 1 DOF, its roll implied by the profile plane; in 3-D the roll is a real DOF, so it is an explicit argument. |
//! | [`path_start_frame`] | the departure half of `nurbs(curve)` | The **tangent-onto-direction** binding: the frame's +Z is the path tangent exactly as the placement rotation takes the curve's start tangent onto the departure. | PATHS reads the departure from the bound tip; here there is no tip, so the missing roll DOF comes from the ladder above. |
//! | [`mirror_across_plane`] | `nurbs_mirrored(curve)` | **Mirror means reflection**: the codimension-1 fixed set, `det = −1`, handedness reversed, curvature/winding signs flipped. Still an isometry; still no scale. | PATHS reflects across the departure *line* in the 2-D profile plane; the 3-D analog is a *plane*. Both are the codimension-1 mirror of their ambient — see [`mirror_across_plane`] on why the 3-D *line* form is deliberately not called a mirror. |
//! | *(none)* | `nurbs_reversed(curve)` | — | No analog, deliberately: reversal is a *parameterization* flip on curve data, and a frame has no parameterization to flip. |
//!
//! **Unification: NOT done, and it should not be forced.** The
//! vocabulary is shared; the code is not. PATHS' placements are
//! typestate transitions in a 2-D profile algebra that consume DOFs
//! from a bound tip and record a program step; these are total 3-D
//! value constructors over [`Affine3`] with no lattice, no tip, and no
//! recording. The one thing a unification could share — the
//! tangent-onto-direction rotation — is three lines in each and
//! carries different degeneracy policies (PATHS refuses at the
//! typestate; here the ladder applies). Sharing the words is the whole
//! win; sharing an abstraction would cost a layer.

use crate::k_stats::decide;
use crate::linalg::{Affine3, Mat3, Point3, Vec3};
use crate::predicate::{
    Band, BandError, COINCIDENCE_RECOURSE, Decide, Indeterminate, Margin, Sign,
};
use crate::real::Real;
use crate::tolerance::Tol;

/// Which input a [`FrameError::Degenerate`] refusal is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameInput {
    /// [`point_at`]'s aim: the displacement from eye to target, whose
    /// length was not definitely nonzero (the two points coincide, sit
    /// inside the ambiguity band, or one of them is poisoned).
    Aim,
    /// [`path_start_frame`]'s tangent, whose length was not definitely
    /// nonzero.
    Tangent,
    /// [`point_at`]'s roll reference, whose perpendicular offset from
    /// the aim line was not definitely nonzero: it is parallel or
    /// antiparallel to the aim, too short to state a direction, or
    /// poisoned.
    ///
    /// **The name reports the decision that refused, not always the
    /// input that caused it.** One aggregated case: an *aim* beyond
    /// [`Vec3::normalize`]'s ~1e154 overflow band passes the aim
    /// decision on an infinite norm and then normalizes to the ZERO
    /// vector, so every reference's offset from it is zero and the
    /// refusal surfaces here rather than as [`FrameInput::Aim`]. This
    /// is [`FrameInput::ReferenceLadder`]'s class arriving through
    /// `point_at`'s single-reference door, which has no separate
    /// variant for it; the situation is unreachable inside the session
    /// box (D4 ¶4), and the refusal is loud either way.
    RollReference,
    /// [`path_start_frame`]'s reference ladder ran out: neither world
    /// +Z nor world +X was definitely off the tangent line. The two
    /// are orthogonal, so no unit tangent can do this: the reachable
    /// class is a tangent whose components exceed
    /// [`Vec3::normalize`]'s ~1e154 overflow band, where the length
    /// decision sees an infinite norm and normalization then collapses
    /// the direction to zero. Refused rather than returned as an
    /// all-zero frame.
    ReferenceLadder,
    /// [`mirror_across_plane`]'s plane normal, whose length was not
    /// definitely nonzero.
    MirrorNormal,
}

impl FrameInput {
    /// The input's name, for messages and pins.
    pub fn name(self) -> &'static str {
        match self {
            FrameInput::Aim => "aim (target − eye)",
            FrameInput::Tangent => "path tangent",
            FrameInput::RollReference => "roll reference",
            FrameInput::ReferenceLadder => "reference ladder (world +Z, then world +X)",
            FrameInput::MirrorNormal => "mirror plane normal",
        }
    }
}

/// A typed frame-construction refusal (D9: fail loud, never a guess,
/// never a panic).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FrameError {
    /// An input direction was not definitely usable — see
    /// [`FrameInput`] for which one and what "usable" means for it.
    /// `indeterminate` carries the classifier's payload when the
    /// margin landed in the ambiguity band, and is `None` when the
    /// margin was a definite zero.
    Degenerate {
        /// The offending input.
        input: FrameInput,
        /// The in-band classification, when that is what happened.
        indeterminate: Option<Indeterminate>,
    },
    /// The run's tolerance does not yield a usable band (see
    /// [`Band::linear`]) — reported, not worked around.
    Band(BandError),
}

impl core::fmt::Display for FrameError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FrameError::Degenerate {
                input,
                indeterminate,
            } => {
                write!(f, "frame: degenerate {}", input.name())?;
                if let Some(i) = indeterminate {
                    write!(f, " ({})", i.payload())?;
                }
                write!(f, "; {COINCIDENCE_RECOURSE}")
            }
            FrameError::Band(e) => write!(f, "frame: {e}"),
        }
    }
}

impl core::error::Error for FrameError {}

/// Classifies a **length** margin (metres) as definitely positive,
/// mapping every other outcome onto a typed refusal for `input`.
///
/// The margin is a length by construction at each call site — a vector
/// norm, or the norm of a cross product with a unit vector (which is
/// the perpendicular distance from the vector's tip to the unit
/// vector's line) — so [`Margin::of`] is the honest door and the
/// metre band applies without a lever.
fn definitely_positive<T: Decide>(
    name: &'static str,
    length: T,
    band: Band,
    input: FrameInput,
) -> Result<(), FrameError> {
    match decide(name, Margin::of(length), band) {
        Ok(Sign::Positive) => Ok(()),
        Ok(_) => Err(FrameError::Degenerate {
            input,
            indeterminate: None,
        }),
        Err(i) => Err(FrameError::Degenerate {
            input,
            indeterminate: Some(i),
        }),
    }
}

/// The one recipe, shared by [`point_at`] and [`path_start_frame`]:
/// the right-handed frame at `origin` whose local +Z is the **unit**
/// `aim` and whose roll is fixed by `reference` per the module docs'
/// convention.
///
/// `cross_len` is the already-decided `|reference × aim|` and
/// `perp` the cross product itself, passed in so the caller's ladder
/// can decide the same quantity without recomputing it (one evaluation,
/// one rounding).
///
/// Evaluation order (fixed, D9): `x̂ = perp / cross_len`, then
/// `ŷ = aim × x̂`, then the columns in the order (x̂, ŷ, aim) with
/// translation `origin − O`.
fn frame_from_unit_aim<T: Real>(
    origin: Point3<T>,
    aim: Vec3<T>,
    perp: Vec3<T>,
    cross_len: T,
) -> Affine3<T> {
    let x = perp / cross_len;
    let y = aim.cross(x);
    Affine3::from_parts(Mat3::from_cols(x, y, aim), origin - Point3::origin())
}

/// A frame at `eye` whose local **+Z axis aims at `target`**, with roll
/// fixed by `roll_reference` per the module docs' convention: the
/// reference lies in the local +Y half-plane (no local +X component,
/// positive local +Y component).
///
/// The result is a **rigid placement** — columns orthonormal, `det =
/// +1`, no scale, no deformation — so it composes with, and inverts
/// through, [`Affine3`] like any other pose. Applying it takes local
/// coordinates to world: local `(0,0,d)` lands `d` along the aim,
/// local origin lands on `eye`.
///
/// Evaluation order (fixed, D9): `aim = target − eye`; the aim length
/// is decided; `ẑ = aim / |aim|`; `perp = roll_reference × ẑ` and its
/// length decided; then [`frame_from_unit_aim`]'s order.
///
/// # Errors
///
/// - [`FrameInput::Aim`] when `target` and `eye` are not definitely
///   distinct — there is no direction to aim along.
/// - [`FrameInput::RollReference`] when the reference's perpendicular
///   offset from the aim line is not definitely nonzero: parallel,
///   antiparallel, zero, or poisoned. This constructor **refuses
///   rather than substituting a reference** — the caller stated the
///   roll, and a silent substitution would answer a different
///   question. Callers wanting a conventional roll want
///   [`path_start_frame`], whose ladder is the stated policy. This
///   variant also absorbs an *aim* past the ~1e154 overflow band,
///   which normalizes to zero after passing its own decision — see
///   [`FrameInput::RollReference`]'s docs for that aggregation, stated
///   there rather than hidden.
/// - [`FrameError::Band`] from [`Band::linear`].
pub fn point_at<T: Decide>(
    eye: Point3<T>,
    target: Point3<T>,
    roll_reference: Vec3<T>,
    tol: Tol,
) -> Result<Affine3<T>, FrameError> {
    let band = Band::linear(tol).map_err(FrameError::Band)?;
    let aim = target - eye;
    definitely_positive("frame_point_at_aim", aim.norm(), band, FrameInput::Aim)?;
    let unit = aim.normalize();
    let perp = roll_reference.cross(unit);
    let len = perp.norm();
    definitely_positive(
        "frame_point_at_roll_offset",
        len,
        band,
        FrameInput::RollReference,
    )?;
    Ok(frame_from_unit_aim(eye, unit, perp, len))
}

/// The profile frame at the start of a swept path: a frame at `origin`
/// whose local **+Z is the path `tangent`**, so the local XY plane is
/// the plane the profile is drawn in and the local X/Y axes are that
/// plane's in-plane axes.
///
/// This is P1's Gram–Schmidt recipe, written once. Roll comes from the
/// module docs' reference **ladder** — world +Z, then world +X — so no
/// call site carries a magic-constant dodge, and the switch-over is a
/// tolerance decision rather than a hard-coded cone.
///
/// `tangent` need not be unit; only its *direction* is used. Its
/// magnitude is read as a length for the degeneracy decision, so a
/// tangent in units other than the run's (a derivative with respect to
/// a non-arc-length parameter, say) should be normalized by the caller
/// — which also puts the decision safely far from the band.
///
/// # Errors
///
/// - [`FrameInput::Tangent`] when the tangent's length is not
///   definitely nonzero (a stationary point of the path, or poison).
/// - [`FrameInput::ReferenceLadder`] when neither rung is definitely
///   off the tangent line — unreachable for an actual direction; see
///   the variant's docs.
/// - [`FrameError::Band`] from [`Band::linear`].
pub fn path_start_frame<T: Decide>(
    origin: Point3<T>,
    tangent: Vec3<T>,
    tol: Tol,
) -> Result<Affine3<T>, FrameError> {
    let band = Band::linear(tol).map_err(FrameError::Band)?;
    definitely_positive(
        "frame_path_start_tangent",
        tangent.norm(),
        band,
        FrameInput::Tangent,
    )?;
    let unit = tangent.normalize();
    // The ladder, in order. A rung is taken only on a DEFINITE
    // off-axis decision: both the coincident outcome (the rung is the
    // tangent line) and the in-band outcome (too close to it to fix a
    // roll) advance. The last classification is kept so the refusal
    // can carry the payload the ladder ended on.
    let mut last = None;
    for (name, reference) in [
        ("frame_path_start_reference_z", Vec3::unit_z()),
        ("frame_path_start_reference_x", Vec3::unit_x()),
    ] {
        let perp = reference.cross(unit);
        let len = perp.norm();
        match decide(name, Margin::of(len), band) {
            Ok(Sign::Positive) => return Ok(frame_from_unit_aim(origin, unit, perp, len)),
            Ok(_) => last = None,
            Err(i) => last = Some(i),
        }
    }
    Err(FrameError::Degenerate {
        input: FrameInput::ReferenceLadder,
        indeterminate: last,
    })
}

/// Reflection across the plane through `point` with normal `normal`
/// (the normal need not be unit; only its direction is used).
///
/// The map is the Householder reflection `I − 2 n̂ n̂ᵀ` about the
/// plane's normal, translated to fix `point`. Points on the plane are
/// fixed; `n̂` maps to `−n̂`; the map is its own inverse.
///
/// # Orientation consequence (the stated one)
///
/// A reflection has **`det = −1`**: it is an isometry — lengths,
/// angles, and the "no scale, no deformation" half of *placement* all
/// survive — but it is **not a rigid motion**. Handedness reverses, so
/// everything that carries an orientation flips with it: a
/// right-handed frame becomes left-handed, surface normals point into
/// what used to be the inside, loop windings and curvature signs
/// reverse. Any consumer that pushes oriented data through this map
/// must reverse that orientation itself; a consumer that checks for
/// rigidity (`det = +1`) will refuse the map, correctly, and that
/// refusal is the reason mirroring a *body* is a topology-layer
/// operation and not merely this matrix.
///
/// # Why there is no `mirror_across_line`
///
/// In the 2-D profile plane, "mirror across a line" is the reflection
/// — codimension 1, `det = −1`, curvature signs flipped (PATHS'
/// `nurbs_mirrored`). In 3-D a line is codimension 2, and the map
/// fixing it pointwise is the **half-turn** about it, which has
/// `det = +1` and preserves handedness. Calling that "mirror" would
/// state the opposite of the truth about orientation, so it is not
/// spelled here: the half-turn already exists as
/// [`Affine3::rotation_about_axis`] with angle π.
///
/// Evaluation order (fixed, D9): decide `|normal|`; `n̂ = normal /
/// |normal|`; `t = n̂ · 2`; columns `e_j − t·n̂_j` in index order;
/// translation `q − L·q` for `q = point − O`.
///
/// That translation mentions `q` twice, and at `T = Interval` a
/// repeated operand does not cancel: the anchor is subtracted and
/// re-added, so a mirror anchored at a point of nonzero width carries
/// `2·width(point)` of translation it should not have.
/// [`Affine3::rotation_about_axis`] used to be spelled the same way and
/// no longer is; the mirror keeps the shape here because retiring it
/// moves `f64` bits in the mirror lane and needs its own golden and
/// k-lint pass. An exact replacement exists — `I − L = 2·n̂n̂ᵀ`, so the
/// translation is `n̂·(2·(n̂·q))`, the anchor mentioned once. The site is
/// carried as an audit member on issue 1143.
///
/// # Errors
///
/// [`FrameInput::MirrorNormal`] when the normal's length is not
/// definitely nonzero (no plane is named), or [`FrameError::Band`]
/// from [`Band::linear`].
pub fn mirror_across_plane<T: Decide>(
    point: Point3<T>,
    normal: Vec3<T>,
    tol: Tol,
) -> Result<Affine3<T>, FrameError> {
    let band = Band::linear(tol).map_err(FrameError::Band)?;
    definitely_positive(
        "frame_mirror_normal",
        normal.norm(),
        band,
        FrameInput::MirrorNormal,
    )?;
    let n = normal.normalize();
    let t = n * T::from_f64(2.0);
    let linear = Mat3::from_cols(
        Vec3::unit_x() - t * n.x,
        Vec3::unit_y() - t * n.y,
        Vec3::unit_z() - t * n.z,
    );
    let q = point - Point3::origin();
    Ok(Affine3::from_parts(linear, q - linear * q))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::tolerance::Tol;

    /// The frame's twelve entries, in column order then translation —
    /// the whole map, bit for bit. Orientation pins compare these, so a
    /// changed column ORDER, a flipped sign, or a re-associated product
    /// all fail loudly rather than passing a loose tolerance.
    fn bits12(a: &Affine3<f64>) -> [u64; 12] {
        let l = a.linear;
        [
            l.c0.x,
            l.c0.y,
            l.c0.z,
            l.c1.x,
            l.c1.y,
            l.c1.z,
            l.c2.x,
            l.c2.y,
            l.c2.z,
            a.translation.x,
            a.translation.y,
            a.translation.z,
        ]
        .map(f64::to_bits)
    }

    /// The seven rigidity residuals `transform_rigid` decides on
    /// (columns unit, columns mutually orthogonal, `det = +1`), as raw
    /// numbers: a frame constructor that returns a non-rigid map is a
    /// bug here, not downstream.
    fn rigidity_residuals(a: &Affine3<f64>) -> [f64; 7] {
        let l = a.linear;
        [
            l.c0.dot(l.c0) - 1.0,
            l.c1.dot(l.c1) - 1.0,
            l.c2.dot(l.c2) - 1.0,
            l.c0.dot(l.c1),
            l.c1.dot(l.c2),
            l.c0.dot(l.c2),
            l.determinant() - 1.0,
        ]
    }

    /// Aiming cases with generic (inexact) arithmetic: eye, target,
    /// roll reference.
    fn aim_cases() -> Vec<(Point3<f64>, Point3<f64>, Vec3<f64>)> {
        vec![
            (Point3::origin(), Point3::new(1.0, 2.0, 3.0), Vec3::unit_z()),
            (
                Point3::new(-4.0, 0.5, 2.0),
                Point3::new(1.0, -2.0, 0.25),
                Vec3::new(0.3, 0.7, -0.2),
            ),
            (
                Point3::new(1e-2, 1e-2, 1e-2),
                Point3::new(1e3, -1e3, 5.0),
                Vec3::unit_x(),
            ),
            (
                Point3::new(7.0, 7.0, 7.0),
                Point3::new(7.0, 7.0, 8.0),
                Vec3::new(1.0, 1.0, 0.0),
            ),
        ]
    }

    #[test]
    fn point_at_orientation_pin() {
        // Row A — aim along +Z with the +Y reference: the frame IS the
        // identity rotation (local +Z is the aim, local +Y holds the
        // reference), translated to the eye. Every entry exact.
        let a = point_at(
            Point3::new(1.0, 2.0, 3.0),
            Point3::new(1.0, 2.0, 7.0),
            Vec3::unit_y(),
            Tol::witness(),
        )
        .unwrap();
        assert_eq!(
            bits12(&a),
            [
                1.0, 0.0, 0.0, // local +X
                0.0, 1.0, 0.0, // local +Y (holds the reference)
                0.0, 0.0, 1.0, // local +Z (the aim)
                1.0, 2.0, 3.0, // the eye
            ]
            .map(f64::to_bits)
        );
        // Row B — aim along +X with the +Z reference: the cyclic frame.
        let b = point_at(
            Point3::origin(),
            Point3::new(4.0, 0.0, 0.0),
            Vec3::unit_z(),
            Tol::witness(),
        )
        .unwrap();
        assert_eq!(
            bits12(&b),
            [
                0.0, 1.0, 0.0, //
                0.0, 0.0, 1.0, //
                1.0, 0.0, 0.0, //
                0.0, 0.0, 0.0,
            ]
            .map(f64::to_bits)
        );
    }

    #[test]
    fn point_at_roll_convention_and_rigidity() {
        // The pinned convention: local +Z is the aim; the reference has
        // no local +X component and a POSITIVE local +Y one.
        for (eye, target, r) in aim_cases() {
            let f = point_at(eye, target, r, Tol::witness()).unwrap();
            let aim = (target - eye).normalize();
            let (x, y, z) = (f.linear.c0, f.linear.c1, f.linear.c2);
            assert!((z.x - aim.x).abs() <= 1e-15);
            assert!((z.y - aim.y).abs() <= 1e-15);
            assert!((z.z - aim.z).abs() <= 1e-15);
            assert!(r.dot(x).abs() <= 1e-13 * r.norm(), "reference off local X");
            assert!(r.dot(y) > 0.0, "reference on the +Y side");
            for res in rigidity_residuals(&f) {
                assert!(res.abs() <= 1e-14, "rigid: {res}");
            }
            // The frame's own origin is the eye, and local +Z walks
            // toward the target.
            let o = f.transform_point(Point3::origin());
            assert!((o - eye).norm() <= 1e-15);
            let walked = f.transform_point(Point3::new(0.0, 0.0, (target - eye).norm()));
            assert!((walked - target).norm() <= 1e-12);
        }
    }

    #[test]
    fn point_at_round_trips_through_affine3() {
        for (eye, target, r) in aim_cases() {
            let f = point_at(eye, target, r, Tol::witness()).unwrap();
            let back = f.inverse() * f;
            for res in rigidity_residuals(&back) {
                assert!(res.abs() <= 1e-13, "round-trip rigid: {res}");
            }
            let p = Point3::new(0.5, -1.5, 2.5);
            let q = f.inverse().transform_point(f.transform_point(p));
            assert!((q - p).norm() <= 1e-12);
        }
    }

    #[test]
    fn point_at_refuses_degenerate_inputs() {
        let e = Point3::new(1.0, 1.0, 1.0);
        // No aim: the two points coincide.
        assert_eq!(
            point_at(e, e, Vec3::unit_z(), Tol::witness()).unwrap_err(),
            FrameError::Degenerate {
                input: FrameInput::Aim,
                indeterminate: None
            }
        );
        let t = Point3::new(1.0, 1.0, 4.0);
        // Reference ON the aim line — parallel, antiparallel, and zero
        // are one refusal: no roll is stated.
        for r in [
            Vec3::unit_z(),
            -Vec3::unit_z(),
            Vec3::new(0.0, 0.0, 12.0),
            Vec3::zero(),
        ] {
            assert_eq!(
                point_at(e, t, r, Tol::witness()).unwrap_err(),
                FrameError::Degenerate {
                    input: FrameInput::RollReference,
                    indeterminate: None
                }
            );
        }
        // The documented aggregation: an aim past the ~1e154 overflow
        // band clears its own decision on an infinite norm, then
        // normalizes to zero, so the refusal arrives under
        // RollReference. Pinned because the docs promise it.
        assert_eq!(
            point_at(
                Point3::origin(),
                Point3::new(1e200, 1e200, 1e200),
                Vec3::unit_z(),
                Tol::witness(),
            )
            .unwrap_err(),
            FrameError::Degenerate {
                input: FrameInput::RollReference,
                indeterminate: None
            }
        );
        // Poison refuses too — as an in-band (invalid-margin) outcome,
        // never as a silently NaN frame.
        let p = point_at(
            e,
            Point3::new(f64::NAN, 0.0, 0.0),
            Vec3::unit_z(),
            Tol::witness(),
        );
        assert!(matches!(
            p,
            Err(FrameError::Degenerate {
                input: FrameInput::Aim,
                indeterminate: Some(_)
            })
        ));
    }
    #[test]
    fn path_start_frame_pole_fallback_pin() {
        // The exact case the hand-rolled dodge exists for: a tangent
        // ALONG the primary reference. Rung one (world +Z) decides
        // coincident and the ladder advances to world +X — no magic
        // cone, and the resulting frame is pinned entry for entry.
        // (The signed zeros are the cross product's exact
        // cancellations; the pin is the bits, so they are pinned too.)
        let up =
            path_start_frame(Point3::origin(), Vec3::new(0.0, 0.0, 5.0), Tol::witness()).unwrap();
        assert_eq!(
            bits12(&up),
            [
                0.0, -1.0, 0.0, // local +X
                1.0, 0.0, -0.0, // local +Y
                0.0, 0.0, 1.0, // local +Z = the tangent
                0.0, 0.0, 0.0,
            ]
            .map(f64::to_bits)
        );
        let down =
            path_start_frame(Point3::origin(), Vec3::new(0.0, 0.0, -2.0), Tol::witness()).unwrap();
        assert_eq!(
            bits12(&down),
            [
                -0.0, 1.0, 0.0, //
                1.0, 0.0, 0.0, //
                0.0, 0.0, -1.0, //
                0.0, 0.0, 0.0,
            ]
            .map(f64::to_bits)
        );
        // Both poles still produce right-handed orthonormal frames —
        // the property the dodge was protecting.
        for f in [up, down] {
            for res in rigidity_residuals(&f) {
                assert!(res.abs() <= 1e-15, "rigid at the pole: {res}");
            }
        }
    }

    #[test]
    fn path_start_frame_is_point_at_with_the_ladder_reference() {
        // The recipe is written ONCE: away from the pole the ladder
        // picks world +Z, and the frame is then bit-for-bit the
        // `point_at` frame with that reference (same origin, aim taken
        // from the same bits).
        for t in [
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(-0.25, 7.5, 0.125),
        ] {
            let a = path_start_frame(Point3::origin(), t, Tol::witness()).unwrap();
            let b = point_at(
                Point3::origin(),
                Point3::origin() + t,
                Vec3::unit_z(),
                Tol::witness(),
            )
            .unwrap();
            assert_eq!(bits12(&a), bits12(&b));
        }
    }

    #[test]
    fn path_start_frame_refuses_true_degeneracy() {
        // A stationary point of the path: no tangent, no frame.
        for t in [
            Vec3::zero(),
            Vec3::new(0.0, 0.0, 1e-200),
            Vec3::new(1e-12, 0.0, 0.0),
        ] {
            assert_eq!(
                path_start_frame(Point3::origin(), t, Tol::witness()).unwrap_err(),
                FrameError::Degenerate {
                    input: FrameInput::Tangent,
                    indeterminate: None
                }
            );
        }
        // Poison refuses as an invalid-margin outcome, carrying the
        // classifier payload.
        assert!(matches!(
            path_start_frame(
                Point3::origin(),
                Vec3::new(f64::NAN, 1.0, 0.0),
                Tol::witness()
            ),
            Err(FrameError::Degenerate {
                input: FrameInput::Tangent,
                indeterminate: Some(_)
            })
        ));
        // A tangent whose components exceed the normalization range
        // (`Vec3::normalize`'s ~1e154 overflow note) passes the length
        // decision on an infinite norm and then normalizes to ZERO —
        // no direction survives, both rungs decide coincident, and the
        // ladder refuses. This is the one reachable `ReferenceLadder`
        // class, and the alternative is a silently all-zero frame.
        assert_eq!(
            path_start_frame(
                Point3::origin(),
                Vec3::new(1e200, 1e200, 1e200),
                Tol::witness()
            )
            .unwrap_err(),
            FrameError::Degenerate {
                input: FrameInput::ReferenceLadder,
                indeterminate: None
            }
        );
        // For every input that IS a direction, the ladder always finds
        // a rung — the pole, the equator, and the near-pole included.
        for t in [
            Vec3::unit_z(),
            -Vec3::unit_z(),
            Vec3::unit_x(),
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(1e-9, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1e150),
        ] {
            assert!(
                !matches!(
                    path_start_frame(Point3::origin(), t, Tol::witness()),
                    Err(FrameError::Degenerate {
                        input: FrameInput::ReferenceLadder,
                        ..
                    })
                ),
                "ladder exhausted for {t:?}"
            );
        }
    }

    #[test]
    fn mirror_orientation_pin_and_involution() {
        // Reflection across the xy-plane: diag(1, 1, −1), no
        // translation. Exact, so the whole map pins bitwise.
        let m = mirror_across_plane(Point3::origin(), Vec3::unit_z(), Tol::witness()).unwrap();
        assert_eq!(
            bits12(&m),
            [
                1.0, 0.0, 0.0, //
                0.0, 1.0, 0.0, //
                0.0, 0.0, -1.0, //
                0.0, 0.0, 0.0,
            ]
            .map(f64::to_bits)
        );
        // The stated orientation consequence: det = −1 exactly, so the
        // rigidity door's determinant residual is −2 — a mirror is an
        // isometry that a rigid-motion check refuses, by design.
        assert_eq!(m.linear.determinant(), -1.0);
        assert_eq!(rigidity_residuals(&m)[6], -2.0);
        // A reflection is its own inverse — composed with an
        // independently built copy of itself, exactly the identity.
        let again = mirror_across_plane(Point3::origin(), Vec3::unit_z(), Tol::witness()).unwrap();
        assert_eq!(bits12(&(m * again)), bits12(&Affine3::identity()));
    }

    #[test]
    fn mirror_fixes_its_plane_and_flips_handedness() {
        let p = Point3::new(1.0, -2.0, 3.0);
        let n = Vec3::new(0.5, 1.5, -0.25);
        let m = mirror_across_plane(p, n, Tol::witness()).unwrap();
        // det = −1 and the columns stay orthonormal: an isometry, not a
        // rigid motion.
        assert!((m.linear.determinant() + 1.0).abs() <= 1e-15);
        for res in [
            m.linear.c0.dot(m.linear.c0) - 1.0,
            m.linear.c1.dot(m.linear.c1) - 1.0,
            m.linear.c2.dot(m.linear.c2) - 1.0,
            m.linear.c0.dot(m.linear.c1),
            m.linear.c1.dot(m.linear.c2),
            m.linear.c0.dot(m.linear.c2),
        ] {
            assert!(res.abs() <= 1e-15, "isometry: {res}");
        }
        // Points of the plane are fixed; the normal reverses; the map
        // is an involution.
        let u = n.cross(Vec3::unit_x()).normalize();
        for s in [-3.0, 0.0, 2.5] {
            let q = p + u * s;
            assert!((m.transform_point(q) - q).norm() <= 1e-14);
        }
        let nn = m.transform_vec(n.normalize());
        assert!((nn + n.normalize()).norm() <= 1e-15);
        let x = Point3::new(-4.0, 0.0, 6.0);
        assert!((m.transform_point(m.transform_point(x)) - x).norm() <= 1e-14);
        // Composed with a frame, the mirror is what flips its
        // handedness — the consequence a consumer must reverse.
        let f = point_at(
            Point3::origin(),
            Point3::new(1.0, 2.0, 3.0),
            Vec3::unit_z(),
            Tol::witness(),
        )
        .unwrap();
        assert!(((m * f).linear.determinant() + 1.0).abs() <= 1e-14);
    }

    #[test]
    fn mirror_refuses_a_plane_with_no_normal() {
        assert_eq!(
            mirror_across_plane(Point3::origin(), Vec3::<f64>::zero(), Tol::witness()).unwrap_err(),
            FrameError::Degenerate {
                input: FrameInput::MirrorNormal,
                indeterminate: None
            }
        );
        assert!(matches!(
            mirror_across_plane(
                Point3::origin(),
                Vec3::new(f64::NAN, 0.0, 1.0),
                Tol::witness()
            ),
            Err(FrameError::Degenerate {
                input: FrameInput::MirrorNormal,
                indeterminate: Some(_)
            })
        ));
    }

    #[test]
    fn refusals_name_the_input_and_carry_the_recourse() {
        // The two-tolerance message shape: what was degenerate, plus
        // the shared recourse sentence (pinned by `contains`, never a
        // full-string pin).
        for (e, needle) in [
            (
                point_at(
                    Point3::origin(),
                    Point3::origin(),
                    Vec3::<f64>::unit_z(),
                    Tol::witness(),
                )
                .unwrap_err(),
                "aim",
            ),
            (
                path_start_frame(Point3::origin(), Vec3::<f64>::zero(), Tol::witness())
                    .unwrap_err(),
                "path tangent",
            ),
            (
                mirror_across_plane(Point3::origin(), Vec3::<f64>::zero(), Tol::witness())
                    .unwrap_err(),
                "mirror plane normal",
            ),
        ] {
            let s = e.to_string();
            assert!(s.contains(needle), "{s}");
            assert!(s.contains(COINCIDENCE_RECOURSE), "{s}");
        }
    }
}
