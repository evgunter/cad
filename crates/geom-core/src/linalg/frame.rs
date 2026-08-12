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
    RollReference,
    /// [`path_start_frame`]'s reference ladder ran out: neither world
    /// +Z nor world +X was definitely off the tangent line. The two
    /// are orthogonal, so no unit tangent can do this — the input was
    /// not a direction (poisoned, or a magnitude outside the range
    /// where normalization is meaningful).
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
fn definitely_positive<T: Real + Decide>(
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
///   [`path_start_frame`], whose ladder is the stated policy.
/// - [`FrameError::Band`] from [`Band::linear`].
pub fn point_at<T: Real + Decide>(
    eye: Point3<T>,
    target: Point3<T>,
    roll_reference: Vec3<T>,
) -> Result<Affine3<T>, FrameError> {
    let band = Band::linear().map_err(FrameError::Band)?;
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
pub fn path_start_frame<T: Real + Decide>(
    origin: Point3<T>,
    tangent: Vec3<T>,
) -> Result<Affine3<T>, FrameError> {
    let band = Band::linear().map_err(FrameError::Band)?;
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
/// translation `q − L·q` for `q = point − O`, exactly as
/// [`Affine3::rotation_about_axis`] computes its own.
///
/// # Errors
///
/// [`FrameInput::MirrorNormal`] when the normal's length is not
/// definitely nonzero (no plane is named), or [`FrameError::Band`]
/// from [`Band::linear`].
pub fn mirror_across_plane<T: Real + Decide>(
    point: Point3<T>,
    normal: Vec3<T>,
) -> Result<Affine3<T>, FrameError> {
    let band = Band::linear().map_err(FrameError::Band)?;
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
