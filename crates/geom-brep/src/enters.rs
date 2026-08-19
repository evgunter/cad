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
//! **The sense correction is the caller's**, and no type enforces it:
//! the `outward_normal` argument is a bare [`Vec3`], so a caller that
//! reads a face's chart normal without multiplying by
//! `Face::sense_sign()` gets a silently inverted verdict on every
//! `sense == false` face. Callers that hold a face say at the call site
//! which vector they are passing and where the sense was folded in
//! (`topo::boolean::sectors::sector_face`, `splitting::rules`).
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

use geom_core::{Band, Decide, Indeterminate, Margin, Sign, Vec3};

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

/// **`enters_material`** — classifies `dir` against a face's material
/// (module docs for the derivation; this is M3's F3 sign-chain
/// primitive). `outward_normal` is the face's outward normal
/// (unit; `Face::sense_sign() * chart_normal`, module docs — the
/// caller applies the sense); `dir` need not be unit (it is normalized
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
    outward_normal: Vec3<T>,
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
    let margin = Margin::levered(dir.normalize().dot(outward_normal), arm);
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
/// per arc length squared), `outward_normal` the reference side's
/// unit normal, `arm` the caller-named lever arm in meters. The
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
    outward_normal: Vec3<T>,
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
    let margin = Margin::sagitta(deriv2.dot(outward_normal) / speed_sq, arm);
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
    use super::*;

    fn band() -> Band {
        Band::linear().unwrap()
    }

    /// Mirror check (F3): on a face whose outward normal is −z (a
    /// solid's bottom face — material above the face), the upward
    /// direction +z must classify as ENTERING material and −z as
    /// exiting. Any global sign flip in the primitive flips both.
    #[test]
    fn mirror_check_bottom_face() {
        let n_out = Vec3::new(0.0, 0.0, -1.0); // bottom face, material above
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
        let n_out = Vec3::new(0.0, 0.0, 1.0);
        let t = enters_material(Vec3::new(1.0, 2.0, 0.0), n_out, 3.0, band()).unwrap();
        assert_eq!(t, EntersMaterial::Tangent);
        let o = enters_material(Vec3::new(1.0, 0.0, -1.0), n_out, 3.0, band()).unwrap();
        assert_eq!(o, EntersMaterial::Enters);
    }

    /// A collapsed lever arm escalates (never a silent verdict) and
    /// names the arm gate.
    #[test]
    fn collapsed_arm_escalates() {
        let n_out = Vec3::new(0.0, 0.0, 1.0);
        let err = enters_material(n_out, n_out, 0.0, band()).unwrap_err();
        assert_eq!(err.predicate, Some("enters_material_arm"));
    }

    /// A poisoned direction (zero vector normalizes to NaN) escalates
    /// through the margin, never classifies.
    #[test]
    fn zero_dir_escalates() {
        let n_out = Vec3::new(0.0, 0.0, 1.0);
        let err = enters_material(Vec3::zero(), n_out, 1.0, band()).unwrap_err();
        assert_eq!(err.predicate, Some("enters_material"));
    }
}
