//! CERT-N2 R1 REVIEWER PROBES. Not part of the unit under review;
//! committed to the reviewer's own branch only.
//!
//! Row S350, executed rather than argued: what `census::face_reach`
//! answers on the masquerading net that this PR's widening newly
//! routes into its described arm, and whether a margin taken against
//! that answer actually decides a sign.

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

/// **Class 9 falsified.** The PR body says tier-3 validation "refuses
/// either way; the variant changes from `UncertifiableSurface` to the
/// described arm's own refusal". `validate.rs`'s surface match has NO
/// described-NURBS arm — it falls to `_ => {}` — so the widening turns
/// a tier-3 REFUSAL into SILENCE at this door. Executed: the same body
/// with the placeholder reports `UncertifiableSurface`; with the
/// masquerade it reports nothing at all.
#[test]
fn probe_class9_tier3_stops_refusing_the_poisoned_face() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let tol = geom_core::Tol::witness();
    let run = |s: Surface<f64>| {
        let mut st = mvfs_state();
        st.body
            .set_face_surface(st.face, FaceSurface::New(s))
            .unwrap();
        let mut marks = slotmap::SecondaryMap::new();
        crate::validate::tier3_local_checks_marked::<f64>(&st.body, &[], band, &mut marks, tol)
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
        "PROBE: tier-3 check 1 no longer names the poisoned face at all — \
         the surface match has no described-NURBS arm, so the body's class-9 \
         'refuses either way' is false at this door (the one error left is the \
         mvfs fixture's empty loop, not a verdict on the surface)"
    );
}
