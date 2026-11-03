//! Frame constructors: the placement vocabulary.
//!
//! Two constructors, each answering a pain row of the corpus survey
//! (`docs/LIBRARY-DESIGN.md` §L2):
//!
//! - [`point_at`] — a frame at an eye position whose local **+Z** aims
//!   at a target, roll fixed by an explicit reference (P6: manual
//!   axis-angle placement with antiparallel special-casing, and no
//!   point-at affordance anywhere).
//! - [`mirror_across_plane`] — reflection across a plane (P6: no
//!   mirror anywhere, so symmetric arrangements are placed by hand,
//!   one leaf at a time).
//!
//! # Which door gives a frame across a direction
//!
//! A direction leaves the roll about it free, and the door is chosen
//! by what states that roll:
//!
//! - **a stated reference**: [`point_at`], or
//!   [`RigidMotion::from_axis_and_reference`] when the axis is in hand
//!   rather than two points;
//! - **no reference**: a frame the author writes (`axes_xy(p)` for a
//!   sweep's start) turned onto the direction by the least turn,
//!   [`RigidMotion::turned_onto`]. It refuses at the written frame's
//!   pole, `−w`, and the recourse there is to write another frame;
//! - **along a path**: `sweep::sweep_places`, the start frame carried
//!   station to station by the rotation-minimizing frame;
//! - [`Vec3::orthonormal_basis`] is for scratch and input coordinates
//!   only, never a frame that is kept: its roll is a property of its
//!   own arithmetic, not of anything the author wrote.
//!
//! [`point_at`] returns a [`RigidMotion`], the witness that RECORDS
//! the orthonormality it decided: its axes are the
//! frame's local +X/+Y/+Z in world coordinates and its origin the
//! frame's, and [`RigidMotion::to_affine`] is the placement map.
//! [`mirror_across_plane`] returns an [`Affine3`], because a reflection
//! is not a frame (`det = −1`).
//!
//! # The roll convention (one sentence, pinned)
//!
//! For [`point_at`] the frame's local **+Z is the aim direction**, and the **roll reference lies in the local +Y
//! half-plane**: the reference's component perpendicular to the aim is
//! a positive multiple of local +Y, and the reference has no local +X
//! component. Equivalently, with `ẑ` the unit aim and `r` the
//! reference: `x̂ = normalize(r × ẑ)`, `ŷ = ẑ × x̂`. The frame is
//! right-handed (`det = +1`), so `x̂ × ŷ = ẑ`.
//!
//! # The degenerate-axis policy (a rule, not a dodge)
//!
//! An aim direction alone does not determine a frame — the roll is
//! free — so [`point_at`] needs a reference direction off the aim
//! line. The hand-rolled twins picked one with a magic-constant dodge
//! (`if n.z.abs() < 0.9 { e_z } else { e_x }`); a frame with no stated
//! reference is a written one turned onto the direction (above), which
//! picks none. The policy here:
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
//! 3. **True degeneracy refuses, typed.** A zero-length aim or mirror
//!    normal refuses [`FrameError::Degenerate`] at its own
//!    [`FrameInput`] — one that merely MEASURES zero because its
//!    length underflowed is clause 4's, not this one's.
//! 4. **Both format questions are asked before sign.** Every length
//!    here is a vector's own norm decided by [`UnitVec3::new`] — the
//!    aim's and the mirror normal's directly, the roll offset's inside
//!    `RigidMotion::from_aim` — so all of them are classified by
//!    the same three questions in the same order, and the first is
//!    [`is_finite_length`](crate::is_finite_length): a direction past
//!    [`Vec3::norm`]'s ~1e154 overflow band has an infinite norm,
//!    which is maximally DEFINITE to the classifier. That refusal is
//!    [`FrameError::NonFiniteLength`], and it names the
//!    [`FrameInput`] whose length is not a number.
//!
//!    It then asks [`is_underflowed_length`](crate::is_underflowed_length), against that vector's
//!    largest `|component|` as the witness ([`Vec3::norm_witness`]).
//!    A direction below [`Vec3::norm`]'s ~1e-162 underflow band
//!    squares to zero, so its norm is EXACTLY zero and the sign
//!    decision answers `Zero` definitely — a true statement about the
//!    arithmetic and a false one about the input, which has a
//!    perfectly good direction the format cannot measure. That
//!    refusal is [`FrameError::UnderflowedLength`], separated from
//!    clause 3's degeneracy because its recourse is the overflow
//!    end's (scale the geometry) and no band reaches it.
//!
//!    **Both gates are POINT-scalar gates.** They ask through the
//!    value channel, and at `T = Interval` neither bites:
//!    `Interval::is_poison` is `is_nai() || is_empty()`, so
//!    `[1e200, ∞] − [1e200, ∞]` is `[−∞, ∞]`, which answers
//!    finite; and a norm whose lower end underflowed still ENCLOSES
//!    the true length, so the underflow ratio is an unbounded
//!    enclosure rather than poison and the question answers `false`.
//!    So clause 4 bites at `f64` and `Probe` and waves an enclosure
//!    through to the sign decision below. No live caller instantiates
//!    this module at `Interval` today; the honest scope is stated at
//!    [`is_finite_length`](crate::is_finite_length) itself.
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
//! | [`RigidMotion::turned_onto`] | the departure half of `nurbs(curve)` | The **tangent-onto-direction** binding: the frame's +Z is carried onto the path tangent exactly as the placement rotation takes the curve's start tangent onto the departure. | PATHS reads the departure from the bound tip; here there is no tip, so the missing roll DOF is the written frame's, carried by the least turn. |
//! | [`mirror_across_plane`] | `nurbs_mirrored(curve)` | **Mirror means reflection**: the codimension-1 fixed set, `det = −1`, handedness reversed, curvature/winding signs flipped. Still an isometry; still no scale. | PATHS reflects across the departure *line* in the 2-D profile plane; the 3-D analog is a *plane*. Both are the codimension-1 mirror of their ambient — see [`mirror_across_plane`] on why the 3-D *line* form is deliberately not called a mirror. |
//! | *(none)* | `nurbs_reversed(curve)` | — | No analog, deliberately: reversal is a *parameterization* flip on curve data, and a frame has no parameterization to flip. |
//!
//! **Unification: NOT done, and it should not be forced.** The
//! vocabulary is shared; the code is not. PATHS' placements are
//! typestate transitions in a 2-D profile algebra that consume DOFs
//! from a bound tip and record a program step; these are total 3-D
//! value constructors over [`RigidMotion`] with no lattice, no tip, and no
//! recording. The one thing a unification could share — the
//! tangent-onto-direction rotation — is three lines in each and
//! carries different degeneracy policies (PATHS refuses at the
//! typestate; here the least turn refuses at its pole). Sharing the words is the whole
//! win; sharing an abstraction would cost a layer.

use crate::linalg::{Affine3, Mat3, Point3, RigidMotion, UnitVec3, UnitVec3Error, Vec3};
use crate::predicate::{Band, BandError, COINCIDENCE_RECOURSE, Decide, Indeterminate};

use crate::real::Real;
use crate::tolerance::Tol;

/// Which caller-supplied input a [`FrameError`] refusal is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FrameInput {
    /// [`point_at`]'s aim: the displacement from eye to target, whose
    /// length was not definitely nonzero (the two points coincide, sit
    /// inside the ambiguity band, or one of them is poisoned).
    Aim,
    /// [`point_at`]'s roll reference, whose perpendicular offset from
    /// the aim line was not definitely nonzero: it is parallel or
    /// antiparallel to the aim, too short to state a direction, or
    /// poisoned.
    ///
    /// It also names a reference whose own perpendicular offset has no
    /// finite length — a reference past [`Vec3::norm`]'s ~1e154
    /// overflow band, whose cross product with the unit aim overflows
    /// — through [`FrameError::NonFiniteLength`]. That is this
    /// input's own failure, decided here: the aim is asked about
    /// first, at its own name.
    RollReference,
    /// [`mirror_across_plane`]'s plane normal, whose length was not
    /// definitely nonzero.
    MirrorNormal,
}

impl FrameInput {
    /// The input's name, for messages and pins.
    pub fn name(self) -> &'static str {
        match self {
            FrameInput::Aim => "aim (target − eye)",
            FrameInput::RollReference => "roll reference",
            FrameInput::MirrorNormal => "mirror plane normal",
        }
    }

    /// The decision an in-band refusal about this input left open, in
    /// words.
    fn subject(self) -> &'static str {
        match self {
            FrameInput::Aim => "whether the frame's aim (target − eye) has any length",
            FrameInput::RollReference => {
                "whether the frame's roll reference has any length off the aim line"
            }
            FrameInput::MirrorNormal => "whether the frame's mirror plane normal has any length",
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
    /// An input direction's length is **not a finite number**, so no
    /// sign decision about it means anything. Distinct from
    /// [`FrameError::Degenerate`] on purpose: the direction is not
    /// zero and no tolerance lever reaches it.
    NonFiniteLength {
        /// The offending vector.
        input: FrameInput,
    },
    /// An input direction's length **underflowed out of the format**:
    /// its components are small enough (below ~1e-162 at `f64`) that
    /// `norm_squared` flushed to zero, so the norm is exactly zero for
    /// a vector that has a perfectly good direction.
    ///
    /// Distinct from [`FrameError::Degenerate`] on purpose, and the
    /// distinction is what carries the recourse: a degenerate input
    /// names no direction, and the answer is to move the geometry or
    /// to widen the band; this one names a direction the format cannot
    /// measure, and **no tolerance lever reaches it** — the squared
    /// norm is zero at every eps. The recourse is the overflow end's,
    /// scale, which is why this variant sits beside
    /// [`FrameError::NonFiniteLength`] rather than beside the
    /// degenerate arm.
    UnderflowedLength {
        /// The offending vector.
        input: FrameInput,
    },
    /// The run's tolerance does not yield a usable band (see
    /// [`Band::linear`]) — reported, not worked around.
    Band(BandError),
}

impl core::fmt::Display for FrameError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            // No frame door takes a declaration, and the mate's
            // declares a contact class, not a direction's length: the
            // recourse is the levers every door here has.
            FrameError::Degenerate {
                input,
                indeterminate: None,
            } => write!(
                f,
                "the frame's {} is degenerate. Recourse: {COINCIDENCE_RECOURSE}",
                input.name()
            ),
            FrameError::Degenerate {
                input,
                indeterminate: Some(i),
            } => i
                .undecided(input.subject(), i.lever_ending(COINCIDENCE_RECOURSE))
                .fmt(f),
            FrameError::NonFiniteLength { input } => write!(
                f,
                "the frame's {} has no finite length (a component overflows the norm or \
                 is not a number). Recourse: {}",
                input.name(),
                crate::predicate::RANGE_RECOURSE
            ),
            FrameError::UnderflowedLength { input } => write!(
                f,
                "the frame's {} has a length that underflowed out of the format, though \
                 it still names a direction. Recourse: {}",
                input.name(),
                crate::predicate::RANGE_RECOURSE
            ),
            FrameError::Band(e) => write!(f, "{e}"),
        }
    }
}

impl core::error::Error for FrameError {}

/// A refusal of [`UnitVec3::new`] in this module's vocabulary, for
/// the caller-supplied vector it was about. Total: the constructor
/// asks its three questions — finite, underflowed, which side of zero
/// — on the vector's own norm, so every normalizing door here mints
/// the witness through it under its own funnel name and maps the four
/// facts onto this module's four refusals, arm for arm.
fn refused_direction(e: UnitVec3Error, input: FrameInput) -> FrameError {
    match e {
        UnitVec3Error::NonFiniteLength => FrameError::NonFiniteLength { input },
        UnitVec3Error::UnderflowedLength => FrameError::UnderflowedLength { input },
        UnitVec3Error::Degenerate => FrameError::Degenerate {
            input,
            indeterminate: None,
        },
        UnitVec3Error::Escalated(i) => FrameError::Degenerate {
            input,
            indeterminate: Some(i),
        },
    }
}

/// A frame at `eye` whose local **+Z axis aims at `target`**, with roll
/// fixed by `roll_reference` per the module docs' convention: the
/// reference lies in the local +Y half-plane (no local +X component,
/// positive local +Y component).
///
/// The result is a **rigid motion** — axes orthonormal, `det = +1`,
/// no scale, no deformation — so it composes with, and inverts
/// through, [`RigidMotion`] like any other pose. Applying it takes local
/// coordinates to world: local `(0,0,d)` lands `d` along the aim,
/// local origin lands on `eye`.
///
/// Evaluation order (fixed, D9): `aim = target − eye`; the aim length
/// is decided; `ẑ = aim / |aim|`; `perp = roll_reference × ẑ`; then
/// `RigidMotion::from_aim`'s order, which decides `perp`'s length,
/// divides by it, and crosses.
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
///   question. A caller with no reference writes a frame and turns it
///   onto the direction ([`RigidMotion::turned_onto`]).
/// - [`FrameError::NonFiniteLength`] when the aim's length, or the
///   reference's perpendicular offset from the aim line, is not a
///   finite number — each at its own [`FrameInput`].
/// - [`FrameError::UnderflowedLength`] when either of those lengths
///   underflowed out of the format, at the same two [`FrameInput`]s.
///   Asked before the sign, so it precedes the degenerate rows above:
///   the input has a direction, and only scale recovers it.
/// - [`FrameError::Band`] from [`Band::linear`].
pub fn point_at<T: Decide>(
    eye: Point3<T>,
    target: Point3<T>,
    roll_reference: Vec3<T>,
    tol: Tol,
) -> Result<RigidMotion<T>, FrameError> {
    let band = Band::linear(tol).map_err(FrameError::Band)?;
    let aim = target - eye;
    let unit = UnitVec3::new(aim, "frame_point_at_aim", band)
        .map_err(|e| refused_direction(e, FrameInput::Aim))?;
    let perp = roll_reference.cross(unit.get());
    RigidMotion::from_aim(eye, unit, perp, "frame_point_at_roll_offset", band)
        .map_err(|e| refused_direction(e.error, FrameInput::RollReference))
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
/// translation `n̂ · (2·(n̂·q))` for `q = point − O` — the dot product
/// first, in [`Vec3::dot`]'s own association, then the factor 2 on that
/// scalar, then the componentwise scale of `n̂`.
///
/// That last step is a projection onto a UNIT direction written out
/// here rather than routed through [`Vec3::project_onto`], which is the
/// one regrouping that method's own contract permits: `project_onto`
/// divides by `|onto|²` in every case, one code path and one rounding
/// story, and `n̂` is already unit — so the division would be by a
/// quantity that is 1 up to rounding and could only add a rounding to
/// an otherwise exact scale. The doubling and the scale are both exact
/// in the binary sense; going through the door would not be.
///
/// **The anchor is mentioned once.** A Householder reflection has
/// `I − L = 2·n̂n̂ᵀ`, so the translation that fixes `point` is
/// `2·n̂(n̂·q)`. At `T = Interval` a repeated operand does not cancel, so
/// a spelling that subtracts the anchor and adds it back charges
/// `2·width(point)` to EVERY component — including the components where
/// the plane's normal vanishes and the translation is exactly zero.
/// This spelling attains the width of the IMAGE of the anchor's
/// enclosure under the map, up to its own rounding floor; no sound
/// enclosure of that image is narrower, though the floor itself is a
/// rounding accident and can fall either way. Reflecting an anchor box
/// of half-width 1e-9 across the `xy` plane gives a translation of width
/// `[0, 0, 4e-9]` rather than `[4e-9, 4e-9, 4e-9]`; per component, over
/// the corpus in `geom-core/tests/props1_evidence.rs`, the exact-normal
/// rows range from 0.78× (a floor accident) to 5.3× narrower plus ten
/// components that become exactly zero, and the wide-normal rows from
/// 1.0× to 3.3e5× narrower.
///
/// The `2·n̂n̂ᵀ` operator has no named home the way `I − R` has
/// [`Mat3::identity_minus_rotation_about`], deliberately: `I − R`'s
/// vanishing factor lives inside a transcendental (`1 − cos θ`) that a
/// caller cannot spell correctly on its own, whereas this one is a
/// scalar times a unit vector, correct at the only site that wants it,
/// and a `Mat3` operator would materialize nine entries to read three.
///
/// # Errors
///
/// - [`FrameInput::MirrorNormal`] when the normal's length is not
///   definitely nonzero — no plane is named.
/// - [`FrameError::NonFiniteLength`] at [`FrameInput::MirrorNormal`]
///   when that length is not a finite number. Asked first: an
///   overflowed normal used to read maximally definite and return the
///   IDENTITY, a mirror that mirrors nothing.
/// - [`FrameError::UnderflowedLength`] at [`FrameInput::MirrorNormal`]
///   when that length underflowed out of the format. Asked second: an
///   underflowed normal names a plane, and reporting it as a
///   degenerate one offers a band the arithmetic cannot reach.
/// - [`FrameError::Band`] from [`Band::linear`].
pub fn mirror_across_plane<T: Decide>(
    point: Point3<T>,
    normal: Vec3<T>,
    tol: Tol,
) -> Result<Affine3<T>, FrameError> {
    let band = Band::linear(tol).map_err(FrameError::Band)?;
    // The witness is read back at once: the Householder entries are
    // its components, and no door below takes the type.
    let n = UnitVec3::new(normal, "frame_mirror_normal", band)
        .map_err(|e| refused_direction(e, FrameInput::MirrorNormal))?;
    Ok(mirror_across_unit_plane(point, n))
}

/// [`mirror_across_plane`] across the plane through `point` whose unit
/// normal is already decided: the Householder map, total.
#[must_use]
pub fn mirror_across_unit_plane<T: Real>(point: Point3<T>, normal: UnitVec3<T>) -> Affine3<T> {
    let n = normal.get();
    let two = T::from_f64(2.0);
    let t = n * two;
    let linear = Mat3::from_cols(
        Vec3::unit_x() - t * n.x,
        Vec3::unit_y() - t * n.y,
        Vec3::unit_z() - t * n.z,
    );
    let q = point - Point3::origin();
    Affine3::from_parts(linear, n * (n.dot(q) * two))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::tolerance::Tol;

    /// The placement map a test reads, from either kind of door.
    trait Placed {
        fn map(&self) -> Affine3<f64>;
    }

    impl Placed for Affine3<f64> {
        fn map(&self) -> Affine3<f64> {
            *self
        }
    }

    impl Placed for RigidMotion<f64> {
        fn map(&self) -> Affine3<f64> {
            self.to_affine()
        }
    }

    /// The frame's twelve entries, in column order then translation —
    /// the whole map, bit for bit. Orientation pins compare these, so a
    /// changed column ORDER, a flipped sign, or a re-associated product
    /// all fail loudly rather than passing a loose tolerance.
    fn bits12(a: &impl Placed) -> [u64; 12] {
        let a = a.map();
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

    /// The seven rigidity residuals `RigidMotion::check_rigid` decides
    /// on
    /// (columns unit, columns mutually orthogonal, `det = +1`), as raw
    /// numbers: a frame constructor that returns a non-rigid map is a
    /// bug here, not downstream.
    fn rigidity_residuals(a: &impl Placed) -> [f64; 7] {
        let l = a.map().linear;
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
            let (x, y, z) = (f.u().get(), f.v().get(), f.w().get());
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
            let f = point_at(eye, target, r, Tol::witness())
                .unwrap()
                .to_affine();
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
        // An aim past the ~1e154 overflow band is the aim's own
        // failure and is named as such — see the overflow row below
        // for the whole family.
        assert_eq!(
            point_at(
                Point3::origin(),
                Point3::new(1e200, 1e200, 1e200),
                Vec3::unit_z(),
                Tol::witness(),
            )
            .unwrap_err(),
            FrameError::NonFiniteLength {
                input: FrameInput::Aim
            }
        );
        // Poison refuses too — as a length that is not a number,
        // never as a silently NaN frame. A poisoned length is the
        // same question as an overflowed one and the same predicate
        // answers it: `NaN − NaN` is poison exactly as `∞ − ∞` is.
        let p = point_at(
            e,
            Point3::new(f64::NAN, 0.0, 0.0),
            Vec3::unit_z(),
            Tol::witness(),
        );
        assert!(
            matches!(
                p,
                Err(FrameError::NonFiniteLength {
                    input: FrameInput::Aim
                })
            ),
            "{p:?}"
        );
    }

    /// Components below ~1e-162 square to zero, so the norm is EXACTLY
    /// zero for a direction the caller stated perfectly well. Every one
    /// of the doors decided that zero and named its input
    /// DEGENERATE, offering a recourse — move the geometry, widen the
    /// band — that cannot reach a squared norm of zero at any eps. The
    /// input is not degenerate; it is outside the range its own
    /// arithmetic can measure, and the recourse is the overflow end's.
    #[test]
    fn every_door_separates_an_underflowed_length_from_a_degenerate_one() {
        let tiny = Vec3::new(0.0, 0.0, 1e-200);
        // The premise, asserted rather than assumed: the norm flushed,
        // and the direction is still there to be read off the witness.
        assert_eq!(tiny.norm(), 0.0);
        assert_eq!(tiny.norm_witness(), 1e-200);

        assert_eq!(
            point_at(
                Point3::origin(),
                Point3::origin() + tiny,
                Vec3::unit_x(),
                Tol::witness()
            )
            .unwrap_err(),
            FrameError::UnderflowedLength {
                input: FrameInput::Aim
            }
        );
        assert_eq!(
            mirror_across_plane(Point3::origin(), tiny, Tol::witness()).unwrap_err(),
            FrameError::UnderflowedLength {
                input: FrameInput::MirrorNormal
            }
        );
        // The roll reference is asked about its PERPENDICULAR OFFSET
        // from the aim line, so the vector that underflows is the cross
        // product rather than the reference: a reference 1e-200 across a
        // unit aim has an offset whose norm flushes to zero while the
        // reference itself is nowhere near the aim line.
        assert_eq!(
            point_at(
                Point3::origin(),
                Point3::new(0.0, 0.0, 1.0),
                Vec3::new(1e-200, 0.0, 0.0),
                Tol::witness()
            )
            .unwrap_err(),
            FrameError::UnderflowedLength {
                input: FrameInput::RollReference
            }
        );

        // The refusal says which end of the format it is, and names the
        // one recourse that works.
        let msg = FrameError::UnderflowedLength {
            input: FrameInput::MirrorNormal,
        }
        .to_string();
        assert!(msg.contains("mirror plane normal"), "{msg}");
        assert!(msg.contains("underflowed out of the format"), "{msg}");
        assert!(msg.contains("scale the geometry"), "{msg}");
        assert!(!msg.contains("degenerate"), "{msg}");
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
        .unwrap()
        .to_affine();
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
            Err(FrameError::NonFiniteLength {
                input: FrameInput::MirrorNormal
            })
        ));
    }

    #[test]
    fn refusals_name_the_input_and_carry_the_recourse() {
        // What was degenerate, plus the levers a frame door has — no
        // frame door takes a declaration, so the coincidence recourse
        // is absent (pinned by `contains`, never a full-string pin).
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
                mirror_across_plane(Point3::origin(), Vec3::<f64>::zero(), Tol::witness())
                    .unwrap_err(),
                "mirror plane normal",
            ),
        ] {
            let s = e.to_string();
            assert!(s.contains(needle), "{s}");
            assert!(s.contains(COINCIDENCE_RECOURSE), "{s}");
            assert!(!s.contains("declare"), "{s}");
        }
        // The in-band arm: the decision in words, the classifier's
        // payload, the same levers.
        let band = Band::linear(Tol::witness()).unwrap();
        let in_band = 0.5 * (band.zero() + band.escalate());
        let e = point_at(
            Point3::origin(),
            Point3::new(in_band, 0.0, 0.0),
            Vec3::<f64>::unit_z(),
            Tol::witness(),
        )
        .unwrap_err();
        assert!(
            matches!(
                e,
                FrameError::Degenerate {
                    input: FrameInput::Aim,
                    indeterminate: Some(_),
                }
            ),
            "{e:?}"
        );
        let s = e.to_string();
        assert!(
            s.contains("whether the frame's aim (target − eye) has any length is undecided: "),
            "{s}"
        );
        assert!(s.contains("ambiguity band"), "{s}");
        assert!(s.contains(COINCIDENCE_RECOURSE), "{s}");
        assert!(!s.contains("declare"), "{s}");
    }

    /// **The overflow end, at each normalizing site.** A
    /// direction whose components pass ~1e154 overflows its norm to
    /// ∞, and an infinite margin is maximally DEFINITE to the
    /// classifier. Each site is exercised at the value that reaches IT, and each
    /// must refuse naming the input whose length is not a number.
    ///
    /// What each row did before the finiteness question went first
    /// (measured, not argued): the mirror returned the IDENTITY; the
    /// roll-reference row returned a frame whose first two columns
    /// were `(0, −0, 0)` and `(0, 0, −0)`; the aim row
    /// refused, but named the downstream input that inherited the
    /// collapse rather than the one that had no length.
    #[test]
    fn a_non_finite_length_refuses_at_the_input_that_has_it() {
        let big = 1e200_f64;
        let tol = Tol::witness();
        let rows = [
            (
                FrameInput::MirrorNormal,
                mirror_across_plane(Point3::origin(), Vec3::new(big, 0.0, 0.0), tol),
            ),
            (
                FrameInput::Aim,
                point_at(
                    Point3::origin(),
                    Point3::new(big, 0.0, 0.0),
                    Vec3::unit_z(),
                    tol,
                )
                .map(RigidMotion::to_affine),
            ),
            (
                // A finite aim, a reference whose perpendicular offset
                // overflows: the offset is this input's own length.
                FrameInput::RollReference,
                point_at(
                    Point3::origin(),
                    Point3::new(0.0, 0.0, 1.0),
                    Vec3::new(big, 0.0, 0.0),
                    tol,
                )
                .map(RigidMotion::to_affine),
            ),
        ];
        for (input, got) in rows {
            assert_eq!(
                got.err(),
                Some(FrameError::NonFiniteLength { input }),
                "{input:?} must refuse at its own name"
            );
        }
        // The sentence names the cause and the recourse, and does NOT
        // claim the direction is zero — it is not — nor offer the
        // coincidence recourse, which no tolerance lever can reach.
        let s = FrameError::NonFiniteLength {
            input: FrameInput::Aim,
        }
        .to_string();
        assert!(s.contains("aim"), "{s}");
        assert!(s.contains("no finite length"), "{s}");
        assert!(s.contains(crate::predicate::RANGE_RECOURSE), "{s}");
        assert!(!s.contains(crate::predicate::COINCIDENCE_RECOURSE), "{s}");
    }

    /// **A degenerate input's escalation ends in the lever, and notes a
    /// poisoned margin.** The input's length is a gauge, so an in-band
    /// margin carries the lever alone; an unreadable one adds the note
    /// that the margin itself could not be read.
    #[test]
    fn a_degenerate_escalation_ends_in_the_lever_and_notes_a_poisoned_margin() {
        let band = Band::linear(Tol::witness()).unwrap();
        let render = |margin: f64| {
            let i = crate::k_stats::decide(
                "frame_test_aim",
                crate::predicate::Margin::of(margin),
                band,
            )
            .unwrap_err();
            FrameError::Degenerate {
                input: FrameInput::Aim,
                indeterminate: Some(i),
            }
            .to_string()
        };
        let in_band = render(0.5 * (band.zero() + band.escalate()));
        assert!(
            in_band.ends_with(&format!(". Recourse: {COINCIDENCE_RECOURSE}")),
            "in band: the lever alone: {in_band}"
        );
        let poisoned = render(f64::NAN);
        assert!(
            poisoned.ends_with(&format!(
                ". Recourse: {COINCIDENCE_RECOURSE}; {}",
                crate::predicate::UNREADABLE_MARGIN_NOTE
            )),
            "poisoned: the lever, then the note: {poisoned}"
        );
    }
}
