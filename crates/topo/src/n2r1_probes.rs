//! CERT-N2 R1 REVIEWER PROBES. Not part of the unit under review;
//! committed to the reviewer's own branch only.
//!
//! Row S350, executed rather than argued: what `census::face_reach`
//! answers on the masquerading net that this PR's widening newly
//! routes into its described arm, and whether a margin taken against
//! that answer actually decides a sign.
//!
//! Adopted by the CERT-N2 fix pass. The header below is the adoption's
//! only addition — the sibling in-src probe module
//! (`review_d18_probes.rs`) carries the identical line, and without it
//! a `#[cfg(test)]` module inside `src/` is linted as production code.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::census::face_reach;
use crate::euler::FaceSurface;
use crate::fixtures::mvfs_state;
use geom::{NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Margin, Point3, Sign};

fn masquerading_surface() -> Surface<f64> {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control: Vec<Point3<f64>> = (0..4)
        .map(|i| Point3::new(f64::NAN, f64::from(i), 2.0))
        .collect();
    Surface::Nurbs(std::sync::Arc::new(
        NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 4]).unwrap(),
    ))
}

/// The face box the census now hands back for a described-but-poisoned
/// net: `Some((lo, hi))` with poison in the poisoned channel and
/// FINITE bounds in the other two. Before the widening this face
/// answered `None` (unclaimable, conservative).
#[test]
fn probe_s350_face_reach_returns_a_partially_poisoned_box() {
    let mut st = mvfs_state();
    st.body
        .set_face_surface(st.face, FaceSurface::New(masquerading_surface()))
        .unwrap();
    let answer = face_reach(&st.body, st.face);
    let Some((lo, hi)) = answer else {
        panic!("PROBE: face_reach answered None — S350's premise does not hold");
    };
    println!("PROBE S350: lo={lo:?} hi={hi:?}");
    assert!(lo.x.is_nan() && hi.x.is_nan(), "the poisoned channel");
    assert!(
        lo.y.is_finite() && hi.y.is_finite() && lo.z.is_finite() && hi.z.is_finite(),
        "the finite channels — this is the partial answer S350 names"
    );
}

/// And the partial answer DECIDES: the census backstop's margin loop
/// breaks on the first definitely-negative margin, and the margins are
/// ordered x, x, y, y, z, z. A y margin that decides Negative sets
/// `cleared` and suppresses the `CensusUndecidable` error, so the pair
/// is cleared on the strength of two channels while the third's extent
/// is unknown. This row runs the same `decide` on the same margins.
#[test]
fn probe_s350_a_margin_against_the_partial_box_clears_a_pair() {
    let mut st = mvfs_state();
    st.body
        .set_face_surface(st.face, FaceSurface::New(masquerading_surface()))
        .unwrap();
    let (olo, ohi) = face_reach(&st.body, st.face).expect("the partial box");
    // An inner box far outside the outer's y extent: the census's
    // clearing direction.
    let ilo = Point3::new(0.0, 100.0, 0.0);
    let ihi = Point3::new(1.0, 101.0, 1.0);
    let band = Band::new(1e-9, 1e-8).unwrap();
    let margins = [
        ilo.x - olo.x,
        ohi.x - ihi.x,
        ilo.y - olo.y,
        ohi.y - ihi.y,
        ilo.z - olo.z,
        ohi.z - ihi.z,
    ];
    let mut cleared = false;
    for (i, m) in margins.into_iter().enumerate() {
        match crate::validate::decide("probe", Margin::of(m), band) {
            Ok(Sign::Negative) => {
                println!("PROBE S350: margin {i} decided NEGATIVE on a partially poisoned box");
                cleared = true;
                break;
            }
            Ok(Sign::Positive) => println!("PROBE S350: margin {i} POSITIVE"),
            other => println!("PROBE S350: margin {i} undecided ({other:?})"),
        }
    }
    assert!(
        cleared,
        "PROBE: no margin decided — S350's 'the finite lanes still decide' would be false"
    );
}

/// **`UncertifiableSurface` is the placeholder state's verdict and no
/// other state's.** Check 1 reads a `Nurbs` payload's `NetState` and
/// answers each state on its own: the placeholder reports
/// `UncertifiableSurface`, and the masquerading net — poison in one
/// channel of every point, so described rather than the placeholder —
/// reports `PoisonedSurfaceDescription`. What this row pins is the
/// negative half: the masquerade never draws the PLACEHOLDER's verdict,
/// whatever else it draws.
#[test]
fn probe_class9_tier3_stops_refusing_the_poisoned_face() {
    // Adoption note (orchestrator, at the merge with CERT-M2): the
    // battery this probe calls gained a sixth argument in CERT-M2's
    // split — check 7 handed in as a hook. The probe passes the empty
    // hook, which is exactly `validate_geometric_structural`'s answer
    // (the battery run without the +V check), so what it measures — check
    // 1's silence on the masquerade — is unchanged. The public structural
    // door cannot stand in here: the mvfs fixture fails an earlier tier
    // there before check 1 is reached.
    let band = Band::new(1e-9, 1e-8).unwrap();
    let tol = geom_core::Tol::witness();
    let run = |s: Surface<f64>| {
        let mut st = mvfs_state();
        st.body
            .set_face_surface(st.face, FaceSurface::New(s))
            .unwrap();
        let mut marks = slotmap::SecondaryMap::new();
        crate::validate::tier3_local_checks_marked::<f64>(
            &st.body,
            &[],
            band,
            &mut marks,
            tol,
            &|_, _, _| None,
            // The structural half's answer for check 2's plane x NURBS
            // lane as well: this probe measures check 1, and the mvfs
            // fixture carries no M7-8 edge for the lane to re-derive.
            None,
        )
        .0
    };
    let with_placeholder = run(Surface::nurbs_placeholder());
    let with_masquerade = run(masquerading_surface());
    println!("PROBE class9 placeholder: {with_placeholder:?}");
    println!("PROBE class9 masquerade:  {with_masquerade:?}");
    assert!(
        !with_placeholder.is_empty(),
        "the placeholder is refused at rest"
    );
    let named = |v: &[crate::validate::ValidationError]| {
        v.iter().any(|e| {
            matches!(
                e,
                crate::validate::ValidationError::UncertifiableSurface { .. }
            )
        })
    };
    assert!(
        named(&with_placeholder),
        "the placeholder is refused at rest"
    );
    assert!(
        !named(&with_masquerade),
        "the masquerading net is DESCRIBED, so the placeholder's verdict is \
         not its answer — check 1 names it as the poisoned state instead"
    );
}
