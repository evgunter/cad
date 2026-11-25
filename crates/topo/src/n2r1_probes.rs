//! CERT-N2 R1 reviewer probes, adopted as rows.
//!
//! What check 1 answers on a net poisoned in one channel of every
//! point, beside a face whose surface slot is pending.
//!
//! Adopted by the CERT-N2 fix pass. The header below is the adoption's
//! only addition — the sibling in-src probe module
//! (`review_d18_probes.rs`) carries the identical line, and without it
//! a `#[cfg(test)]` module inside `src/` is linted as production code.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::euler::FaceSurface;
use crate::fixtures::mvfs_state;
use geom::Surface;

/// **`UncertifiableSurface` is the pending slot's verdict and no net's.**
/// Check 1 reports a pending slot as `UncertifiableSurface`, and a net
/// that is not finite — poison in one channel of every point — as
/// `PoisonedSurfaceDescription`. What this row pins is the negative
/// half: the corrupt net never draws the pending slot's verdict,
/// whatever else it draws.
#[test]
fn probe_class9_tier3_stops_refusing_the_poisoned_face() {
    // Adoption note (orchestrator, at the merge with CERT-M2): the
    // battery this probe calls gained a sixth argument in CERT-M2's
    // split — check 7 handed in as a hook. The probe passes the empty
    // hook, which is exactly the composed `validate_geometric`'s
    // structural phase (the battery run without the +V check), so what it
    // measures — check 1's silence on the masquerade — is unchanged. The
    // public structural door cannot stand in here: the mvfs fixture fails
    // an earlier tier there before check 1 is reached.
    let tol = geom_core::Tol::witness();
    let run = |s: Surface<f64>| {
        let mut st = mvfs_state();
        st.body
            .set_face_surface(
                st.face,
                FaceSurface::New {
                    surface: s,
                    sense: true,
                },
            )
            .unwrap();
        let mut marks = slotmap::SecondaryMap::new();
        crate::validate::tier3_local_checks_marked::<f64>(
            &st.body,
            &mut marks,
            tol,
            crate::validate::PlusVCheck::NotMade,
            // This probe is concrete at `f64`, so it names the offset
            // fit door by hand; the face it runs is on a net that is not
            // finite, which does not reach it.
            Some(geom_brep::OffsetFitLane::fit()),
        )
        .0
    };
    let with_pending = {
        let mut st = crate::fixtures::mvfs_state();
        st.body
            .set_face_surface(st.face, FaceSurface::Pending { sense: true })
            .unwrap();
        let mut marks = slotmap::SecondaryMap::new();
        crate::validate::tier3_local_checks_marked::<f64>(
            &st.body,
            &mut marks,
            tol,
            crate::validate::PlusVCheck::NotMade,
            Some(geom_brep::OffsetFitLane::fit()),
        )
        .0
    };
    let with_masquerade = run(crate::fixtures::poisoned_net(|_| true));
    println!("PROBE class9 pending:    {with_pending:?}");
    println!("PROBE class9 masquerade:  {with_masquerade:?}");
    assert!(
        !with_pending.is_empty(),
        "the pending face is refused at rest"
    );
    let named = |v: &[crate::validate::ValidationError]| {
        v.iter().any(|e| {
            matches!(
                e,
                crate::validate::ValidationError::UncertifiableSurface { .. }
            )
        })
    };
    assert!(named(&with_pending), "the pending face is refused at rest");
    assert!(
        !named(&with_masquerade),
        "the poisoned net is a description, so the pending slot's verdict is \
         not its answer — check 1 names it as poisoned instead"
    );
}
