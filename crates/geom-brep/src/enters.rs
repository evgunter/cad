//! `enters_material` — the M3 sign-chain primitive (M3-PLAN fork F3).
//!
//! Every orientation-sensitive verdict in the splitting/boolean pipeline
//! (ch. 14 rule (a), ch. 15 side codes, null-edge dispatch, seam
//! orientation) is re-derived from this ONE predicate instead of being
//! sign-copied from the book — the book's GWB is CW-from-outside, our
//! kernel is the ratified mirror, and the ch. 14/ch. 15 printed signs
//! are mutually inconsistent (synthesis §B "the sign question"), so no
//! printed sign is trusted.
//!
//! # Derivation (first principles, under OUR ratified convention)
//!
//! Ratified (M1, `topo::entity` module docs): outer loops run
//! counterclockwise viewed from outside, equivalently **every face has
//! an outward normal** — the one pointing away from the solid's
//! material, which the loop winding is tied to. That normal is
//! `Face::sense_sign() * chart_normal(u, v)` (DESIGN "face orientation
//! sense", ratified M5 S10): the surface's stored chart normal is the
//! outward normal only where `Face::sense` is `true`. (TOG 1986
//! §2/§6.1 states the same interior-left convention, which is why the
//! book's rule (a) happens to be printed right for us; we still derive
//! rather than copy.)
//!
//! Take a point on a face and a direction `dir`. Decompose `dir` against
//! the outward normal `n`: the component `dot(dir, n)·n` is the part of
//! the motion leaving or entering the solid.
//!
//! - `dot(dir, n) < 0`: `dir` has a component **against** the outward
//!   normal — it moves from the face toward the side the material is on.
//!   The direction **enters material**.
//! - `dot(dir, n) > 0`: `dir` moves along the outward normal, away from
//!   material — it **exits material**.
//! - `dot(dir, n) = 0`: `dir` lies in the face's tangent plane —
//!   **tangent**, neither entering nor exiting at first order.
//!
//! So `enters_material(dir, face) := sign(dot(dir, outward_normal))`,
//! with **negative ⇒ into material**. That single sentence is the entire
//! sign convention of M3; consumers must cite it rather than introduce a
//! fresh sign choice.
//!
//! **The sense correction is carried by the type.** The
//! `outward_normal` argument is an [`OutwardNormal`], whose only
//! constructor is [`OutwardNormal::from_chart`] — it cannot be called
//! without naming the face's `sense` bit, so a raw chart normal cannot
//! reach this predicate at all. What the mint sites still say at the
//! call site is WHERE the bit came from, not whether it was applied.
//!
//! A normal that **defines** a side convention rather than carrying
//! one — a splitting plane's, an operation input belonging to no face
//! and with no sense to fold in — is the separate [`ReferenceNormal`],
//! which [`enters_material_order2`] takes. They are two types because
//! they are two claims: one asserts a face's material side, the other
//! declares the operation's convention, and only the first has a sense
//! to get wrong.
//!
//! # Margin honesty (D4 ¶1)
//!
//! For unit `dir` and unit `n`, `dot(dir, n)` is the sine of the
//! elevation angle of `dir` out of the face plane — an angular quantity.
//! An angle only means anything through the displacement it induces at a
//! lever arm, so the classified margin is `dot(d̂ir, n) · arm` in meters,
//! with `arm` the caller-named lever arm (the distance over which the
//! verdict is consumed — e.g. the face extent for a coplanar-sector
//! reclassification), against the run's linear band. This is the
//! `classify_dihedral` pattern.

use geom_core::{Band, Decide, Indeterminate, Margin, Real, Sign, Vec3};

/// The verdict of [`enters_material`]: where `dir` goes relative to the
/// face's material.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntersMaterial {
    /// `dot(dir, outward_normal)` definitely negative: `dir` moves into
    /// the solid's material.
    Enters,
    /// Definitely positive: `dir` moves away from material.
    Exits,
    /// Coincident with zero: `dir` lies in the face's tangent plane.
    Tangent,
}

/// A face's **outward normal** at a point: the unit vector pointing
/// away from the solid's material (module docs for the derivation and
/// for what the sign then means).
///
/// INVARIANT: the wrapped vector is the chart normal, negated exactly
/// where `Face::sense` is `false` (DESIGN "face orientation sense",
/// M5 S10). The type has no general constructor — no `new`, no
/// `From<Vec3>`, no public field — so the only way to obtain one is
/// [`OutwardNormal::from_chart`], which cannot be written without
/// naming the face's sense bit. A chart normal read off a
/// `sense == false` face and handed on unflipped — the silent
/// inversion this type exists to prevent — does not typecheck.
#[derive(Clone, Copy, Debug)]
pub struct OutwardNormal<T: Real>(Vec3<T>);

impl<T: Real> OutwardNormal<T> {
    /// Mints a face's outward normal from the surface's chart normal at
    /// a point and the face's `sense` bit (`Face::sense`: `true` where
    /// the chart normal already points out of the material, `false`
    /// where the face reverses its surface).
    ///
    /// The ONLY constructor: naming the sense is the whole obligation,
    /// so it is a parameter rather than a caller's remembered multiply.
    /// It takes the **bit**, not a `T` sign, because S10's flip is
    /// selected by a boolean and is never a numeric decision
    /// (`Face::sense_sign`'s own contract — the scalar backends order
    /// intervals, not signs). A `T` parameter would admit `1.0` on a
    /// reversed face, or a dot product, or an `Interval` that is not
    /// ±1 at all; the bit admits exactly two words at the call site.
    #[must_use]
    pub fn from_chart(chart_normal: Vec3<T>, sense: bool) -> Self {
        Self(if sense { chart_normal } else { -chart_normal })
    }

    /// The outward normal as a plain vector, for the sites that consume
    /// it as geometry rather than as a material claim.
    ///
    /// **Unwrapping does not make a site sign-blind.** Some consumers
    /// genuinely are — a cross product naming a LINE, a norm, a
    /// coplanarity magnitude — but the sector algebra downstream
    /// (convex/reflex verdicts, directed bisectors) reads the sign and
    /// is sound for a different reason: it pairs this normal with the
    /// STORED orbit/loop traversal, which `revert` reverses in the same
    /// breath as the sense bit, so the two flips cancel and a second
    /// `sense` factor would re-break what this type fixes. A caller
    /// reaching for `vec()` owes one of those two arguments; the type
    /// stops carrying it from here on.
    ///
    /// Extraction is safe in the direction the guard runs: it cannot be
    /// used to mint one.
    #[must_use]
    pub fn vec(self) -> Vec3<T> {
        self.0
    }
}

/// A **reference normal**: a normal that DEFINES an Above/Below
/// convention rather than carrying one. An operation input — a
/// splitting plane's normal — belongs to no face, has no
/// `Face::sense` to fold in, and flipping it redefines the operation
/// instead of contradicting a solid's material.
///
/// INVARIANT: there is no conversion from this type to an
/// [`OutwardNormal`], and that asymmetry is the point —
/// [`enters_material`]'s face slot stays closed to a vector that was
/// never given a sense. The widening the other way would be sound (a
/// face's outward normal is a valid reference side) but is not
/// written: nothing in the kernel asks [`enters_material_order2`]
/// about a face, and an impl with no caller is machinery, not a
/// contract.
#[derive(Clone, Copy, Debug)]
pub struct ReferenceNormal<T: Real>(Vec3<T>);

impl<T: Real> ReferenceNormal<T> {
    /// The normal of a **splitting plane** — the operation input that
    /// defines which side is Above. Named for what it is, so a caller
    /// holding a FACE's normal cannot reach for it by accident.
    #[must_use]
    pub fn of_split_plane(normal: Vec3<T>) -> Self {
        Self(normal)
    }

    /// The reference normal as a plain vector — crate-internal, for
    /// the predicate's own dot product.
    fn vec(self) -> Vec3<T> {
        self.0
    }
}

/// **`enters_material`** — classifies `dir` against a face's material
/// (module docs for the derivation; this is M3's F3 sign-chain
/// primitive). `outward_normal` is the face's outward normal, unit and
/// [typed][`OutwardNormal`] — the sense bit is inside its
/// constructor, not in this caller's memory; `dir` need not be unit (it is normalized
/// here); `arm` is the caller-named lever arm in meters metering the
/// angular margin; `band` is the run's linear band.
///
/// # Errors
///
/// [`Indeterminate`]:
/// - predicate `"enters_material_arm"` — the lever arm failed to
///   classify definitely positive (collapsed, in-band, or poisoned): no
///   angular verdict can be metered here (escalate-never-guess).
/// - predicate `"enters_material"` — the metered margin landed in the
///   sliver band or was poisoned (e.g. a zero `dir`).
pub fn enters_material<T: Decide>(
    dir: Vec3<T>,
    outward_normal: OutwardNormal<T>,
    arm: T,
    band: Band,
) -> Result<EntersMaterial, Indeterminate> {
    match decide("enters_material_arm", Margin::of(arm), band)? {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => {
            return Err(Indeterminate {
                margin: geom_core::MarginDiag::Invalid,
                band,
                predicate: Some("enters_material_arm"),
            });
        }
    }
    let margin = Margin::levered(dir.normalize().dot(outward_normal.vec()), arm);
    Ok(match decide("enters_material", margin, band)? {
        Sign::Negative => EntersMaterial::Enters,
        Sign::Positive => EntersMaterial::Exits,
        Sign::Zero => EntersMaterial::Tangent,
    })
}

/// **`enters_material_order2`** — the second-order descent of the
/// sector trilean (C7/C12.2, M5 PR 9): where first-order data ties
/// (a departure direction exactly IN the reference plane — the
/// [`EntersMaterial::Tangent`] graze), classification descends one
/// order and asks which side the departure **curves** to.
///
/// Inputs are the departing curve's jet at the tie point: `deriv2`
/// the second derivative (raw carrier parameter), `speed_sq` =
/// `‖deriv‖²` (normalizing the parameterization out — the margin is
/// per arc length squared), `reference_normal` the reference side's
/// unit normal as a [`ReferenceNormal`] — its one caller passes a
/// SPLITTING PLANE's normal, which defines the Above/Below convention
/// and carries no face sense; a caller holding a FACE's normal mints
/// an [`OutwardNormal`] and converts, so the sense is applied there —
/// `arm` the caller-named lever arm in meters. The
/// margin is the **displacement the curvature difference induces at
/// the lever arm** (D4 ¶1, lever arm 1/κ discipline):
/// `½ · (deriv2·n̂ / speed_sq) · arm²` — the curvature COMPARISON of
/// the tied sectors (against a plane the partner curvature is zero,
/// so the difference is the departure's own normal curvature).
///
/// Same side convention as [`enters_material`]: curving along `+n̂`
/// ⇒ `Exits`, along `−n̂` ⇒ `Enters`, and an exactly-zero
/// second-order margin stays [`EntersMaterial::Tangent`] — the
/// caller keeps its typed refusal (never guess); an in-band margin
/// escalates (F6 — an osculating pair is a sliver at this ε).
///
/// The `tangent_*` predicate family: the K funnel's second genuinely
/// ill-conditioned crop, telemetry from birth (the PR 14 K-snapshot
/// reads these names).
///
/// # Errors
///
/// [`Indeterminate`]: predicate `"tangent_sector_order2_arm"` (the
/// collapsed-arm gate, the dihedral.rs idiom) or
/// `"tangent_sector_order2"` (in-band or poisoned margin).
pub fn enters_material_order2<T: Decide>(
    deriv2: Vec3<T>,
    speed_sq: T,
    reference_normal: ReferenceNormal<T>,
    arm: T,
    band: Band,
) -> Result<EntersMaterial, Indeterminate> {
    match decide("tangent_sector_order2_arm", Margin::of(arm), band)? {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => {
            return Err(Indeterminate {
                margin: geom_core::MarginDiag::Invalid,
                band,
                predicate: Some("tangent_sector_order2_arm"),
            });
        }
    }
    let margin = Margin::sagitta(deriv2.dot(reference_normal.vec()) / speed_sq, arm);
    Ok(match decide("tangent_sector_order2", margin, band)? {
        Sign::Negative => EntersMaterial::Enters,
        Sign::Positive => EntersMaterial::Exits,
        Sign::Zero => EntersMaterial::Tangent,
    })
}

/// The crate-local funnel wrapper (the `geom-brep` pattern: one
/// greppable `sign_within` door per crate, unified recorder — M2 PR 7).
fn decide<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
) -> Result<Sign, Indeterminate> {
    geom_core::k_stats::decide(name, margin, band)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Tol;
    use super::*;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// Mirror check (F3): on a face whose outward normal is −z (a
    /// solid's bottom face — material above the face), the upward
    /// direction +z must classify as ENTERING material and −z as
    /// exiting. Any global sign flip in the primitive flips both.
    #[test]
    fn mirror_check_bottom_face() {
        // Bottom face, material above: chart normal +z on a
        // `sense == false` face — the bit is what makes it outward.
        let n_out = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), false);
        let up = Vec3::new(0.0, 0.0, 1.0);
        let e = enters_material(up, n_out, 1.0, band()).unwrap();
        assert_eq!(e, EntersMaterial::Enters);
        let e = enters_material(-up, n_out, 1.0, band()).unwrap();
        assert_eq!(e, EntersMaterial::Exits);
    }

    /// An in-plane direction is tangent; an oblique one classifies by
    /// its normal component's sign, metered by the arm.
    #[test]
    fn tangent_and_oblique() {
        let n_out = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        let t = enters_material(Vec3::new(1.0, 2.0, 0.0), n_out, 3.0, band()).unwrap();
        assert_eq!(t, EntersMaterial::Tangent);
        let o = enters_material(Vec3::new(1.0, 0.0, -1.0), n_out, 3.0, band()).unwrap();
        assert_eq!(o, EntersMaterial::Enters);
    }

    /// A collapsed lever arm escalates (never a silent verdict) and
    /// names the arm gate.
    #[test]
    fn collapsed_arm_escalates() {
        let n_out = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        let err = enters_material(n_out.vec(), n_out, 0.0, band()).unwrap_err();
        assert_eq!(err.predicate, Some("enters_material_arm"));
    }

    /// A poisoned direction (zero vector normalizes to NaN) escalates
    /// through the margin, never classifies.
    #[test]
    fn zero_dir_escalates() {
        let n_out = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        let err = enters_material(Vec3::zero(), n_out, 1.0, band()).unwrap_err();
        assert_eq!(err.predicate, Some("enters_material"));
    }
}
