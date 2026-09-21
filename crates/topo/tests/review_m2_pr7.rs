//! M2 PR 7 e2e review — attacks on the tier-3 +V orientation
//! invariant: a MIRRORED (inside-out) cube forged through the public
//! Euler/attach operators must be caught (`NegativeVolume`), incl. at
//! 1e6 scale; the Zero-exemption's honest boundary is pinned (an
//! inside-out body THINNER than ε classifies `Zero` and passes — the
//! ratified "orientation probe, not thinness gate" posture, executed
//! here so the boundary is explicit, not folklore); and the
//! clean-report gate (volume suppressed under other tier-3 errors) is
//! exercised.
//!
//! Every body here is `common::mapped_cube` under a point map. Under a
//! reflection the SAME loop traversals become inward-CCW, so every
//! Newell plane (computed from the same corner orders) flips inward:
//! a perfectly two-manifold, tier-1/2-valid, planar-consistent body
//! whose only defect is global orientation.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::Point3;
use geom_core::Tol;
use topo::{
    FaceSurface, ValidationError, mass_properties, validate, validate_closed, validate_geometric,
};

use crate::common;

/// The mirrored cube reaches tier 3 as a body whose ONLY tier-3
/// defect is orientation — and `NegativeVolume` fires. This is the
/// end-to-end pin of the +V sign convention that the implementer's
/// suite covers only at the geom-brep unit level.
#[test]
fn mirrored_cube_is_caught_by_negative_volume() {
    let body = common::mapped_cube(|x, y, z| Point3::new(-x, y, z), Tol::witness());
    assert_eq!(validate(&body), Ok(()), "tier 1 cannot see orientation");
    assert_eq!(
        validate_closed(&body),
        Ok(()),
        "tier 2 cannot see orientation"
    );
    // The exact machinery agrees the volume is −1.
    let props = mass_properties(&body, Tol::witness()).unwrap();
    assert!(
        (props.volume + 1.0).abs() < 1e-12,
        "mirrored cube volume: {}",
        props.volume
    );
    let errs = validate_geometric(&body, Tol::witness()).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::NegativeVolume)),
        "+V must fire on the mirrored cube; got {errs:?}"
    );
}

/// Same at 1e6 scale: the V/A margin is a length (≈ s/6 for the
/// s-cube), so scale must not defeat the classification.
#[test]
fn megascale_mirrored_cube_is_caught() {
    let s = 1e6;
    let body = common::mapped_cube(|x, y, z| Point3::new(-x * s, y * s, z * s), Tol::witness());
    let errs = validate_geometric(&body, Tol::witness()).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::NegativeVolume)),
        "+V must fire at 1e6 scale; got {errs:?}"
    );
}

/// The control: the unmirrored twin through the same op sequence is
/// fully valid — the mirrored failure above is orientation, not an
/// artifact of the raw construction path.
#[test]
fn unmirrored_twin_is_tier3_valid() {
    let body = common::mapped_cube(Point3::new, Tol::witness());
    assert_eq!(validate_geometric(&body, Tol::witness()), Ok(()));
}

/// EXECUTED BOUNDARY of the exemptions (ratified posture: the +V
/// check is "an orientation probe, not a thinness gate" — Zero and
/// escalated margins never flip valid → invalid). Findings, executed:
///
/// - The **Zero branch is unreachable for slab-like bodies**: an
///   inside-out slab needs thickness < 2ε for |V|/A ≤ ε, but its
///   struts' parameter intervals then fail forward certification
///   (`IntervalNotForward`) — the construction refuses first. Honest
///   fail-loud, asserted below.
/// - The **escalation band IS reachable**: thickness 2ε < t < 2Kε
///   certifies (t ≥ Kε) yet lands |V|/A = t/2 in (ε, Kε), so the
///   decide funnel escalates and the check exempts — an inside-out
///   slab of thickness ~Kε passes tier 3. Pinned as executable fact;
///   if the design ever tightens, this flips.
#[test]
fn thin_inverted_slab_exemption_boundary() {
    let eps = geom_core::Tol::witness().get().eps;
    // (a) sub-2ε thickness: refused upstream, Zero branch unreachable.
    let sub = std::panic::catch_unwind(|| {
        common::mapped_cube(|x, y, z| Point3::new(-x, y, z * eps), Tol::witness())
    });
    assert!(
        sub.is_err(),
        "a sub-ε slab should fail construction (forward certification), \
         making the Zero exemption unreachable for this family"
    );
    // (b) escalation-band thickness: builds, is inside out, passes.
    let t = 10.0 * eps; // = Kε at default K: certifies; |V|/A = 5ε ∈ (ε, Kε).
    let body = common::mapped_cube(|x, y, z| Point3::new(-x, y, z * t), Tol::witness());
    let props = mass_properties(&body, Tol::witness()).unwrap();
    assert!(props.volume < 0.0, "the slab is genuinely inside out");
    assert_eq!(
        validate_geometric(&body, Tol::witness()),
        Ok(()),
        "the escalation exemption admits an inside-out ~Kε slab (ratified posture; executed pin)"
    );
}

/// The clean-report gate: with another tier-3 error present (a face
/// plane re-attached definitely off its vertices), the volume check is
/// suppressed — the report must NOT contain
/// `NegativeVolume`/`VolumeUncomputable` cascade noise, only the real
/// defect.
#[test]
fn volume_check_is_gated_on_otherwise_clean_reports() {
    let mut body = common::mapped_cube(|x, y, z| Point3::new(-x, y, z), Tol::witness());
    // Corrupt one face's plane through the public setter: origin
    // shifted along the normal ⇒ planar residual errors at tier 3.
    let (fk, face) = body.faces().next().unwrap();
    let sk = face.surface;
    let Some(Surface::Plane {
        origin,
        normal,
        u_ref,
    }) = body.get_surface(sk).cloned()
    else {
        panic!("cube face must be planar");
    };
    body.set_face_surface(
        fk,
        FaceSurface::New(Surface::Plane {
            origin: origin + normal * 0.05,
            normal,
            u_ref,
        }),
    )
    .unwrap();
    let errs = validate_geometric(&body, Tol::witness()).unwrap_err();
    assert!(
        !errs.is_empty(),
        "the corrupted body must fail tier 3 somewhere"
    );
    assert!(
        errs.iter().all(|e| !matches!(
            e,
            ValidationError::NegativeVolume | ValidationError::VolumeUncomputable { .. }
        )),
        "volume verdicts must be suppressed on an unclean report; got {errs:?}"
    );
}
