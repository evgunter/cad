//! The void-insertion door (`topo::insert_void`) probed directly —
//! VERBS-RING's factoring of the boolean containment fallback's cavity
//! step into the one birthplace of cavities (DESIGN's M2 bullet).
//!
//! Two claims:
//! - **The door inserts**: a certified-strictly-contained brick lands
//!   as a reversed interior shell — two shells in one solid, tiers
//!   1–2 clean, volume = outer − cavity (the closed forms), agreeing
//!   with what the boolean fallback's subtraction builds for the same
//!   operands.
//! - **Containment-evidence honesty** (the planted-corruption rows):
//!   absent evidence, non-strict evidence (a probed `Out`/`OnBoundary`
//!   verdict, a carried `Zero`/`Negative` sign), and evidence naming a
//!   foreign shell each refuse TYPED, before any mutation — the
//!   destination is bitwise untouched by a refused call.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::prism_z;
use geom_core::{Sign, Tol};
use topo::{
    Body, BooleanResult, BooleanResultKind, SolidContainment, VoidContainment, VoidEvidence,
    VoidInsertError, insert_void, mass_properties, subtract, validate, validate_closed,
};

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    prism_z::<f64>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1).body
}

fn outer_and_cavity() -> (Body<f64>, Body<f64>) {
    (
        brick((0.0, 3.0), (0.0, 3.0), (0.0, 3.0)),
        brick((1.0, 2.0), (1.0, 2.0), (1.0, 2.0)),
    )
}

fn probed_in(cavity: &Body<f64>) -> VoidEvidence {
    VoidEvidence {
        shells: cavity
            .shells()
            .map(|(s, _)| (s, VoidContainment::Probed(SolidContainment::In)))
            .collect(),
    }
}

#[test]
fn door_inserts_a_certified_cavity() {
    let (mut dst, cavity) = outer_and_cavity();
    let (solid, _) = dst.solids().next().unwrap();
    let src_shell = cavity.shells().next().unwrap().0;
    let evidence = probed_in(&cavity);
    let inserted = insert_void(&mut dst, solid, cavity, &evidence, Tol::witness()).unwrap();

    assert_eq!(validate(&dst), Ok(()));
    assert_eq!(validate_closed(&dst), Ok(()));
    assert_eq!(dst.shells().count(), 2);
    let cavity_shell = inserted.shell(src_shell).expect("shell bridged");
    assert_eq!(dst.get_shell(cavity_shell).unwrap().solid, solid);
    // Volume: 27 − 1 (the reversed interior shell subtracts); area:
    // both boundaries count, 54 + 6.
    let props = mass_properties(&dst, Tol::witness()).unwrap();
    assert!((props.volume - 26.0).abs() < 1e-12);
    assert!((props.surface_area - 60.0).abs() < 1e-12);
}

/// The carried-sign arm (the revolve's evidence shape) inserts the
/// same way.
#[test]
fn door_accepts_carried_strict_evidence() {
    let (mut dst, cavity) = outer_and_cavity();
    let (solid, _) = dst.solids().next().unwrap();
    let evidence = VoidEvidence {
        shells: cavity
            .shells()
            .map(|(s, _)| {
                (
                    s,
                    VoidContainment::Carried {
                        sign: Sign::Positive,
                    },
                )
            })
            .collect(),
    };
    insert_void(&mut dst, solid, cavity, &evidence, Tol::witness()).unwrap();
    assert_eq!(dst.shells().count(), 2);
    assert_eq!(validate_closed(&dst), Ok(()));
}

/// The door and the boolean fallback build the same result for the
/// same operands (the fallback now CALLS the door; this pins the
/// factoring's observable agreement at the suite level too).
#[test]
fn door_agrees_with_the_subtract_fallback() {
    let (mut dst, cavity) = outer_and_cavity();
    let (a, b) = outer_and_cavity();
    let (solid, _) = dst.solids().next().unwrap();
    let evidence = probed_in(&cavity);
    insert_void(&mut dst, solid, cavity, &evidence, Tol::witness()).unwrap();

    let r = subtract(&a, &b, Tol::witness()).unwrap();
    let BooleanResult::Body(bb) = r else {
        panic!("voided body")
    };
    assert_eq!(bb.kind, BooleanResultKind::Voided);
    assert_eq!(bb.body.shells().count(), dst.shells().count());
    assert_eq!(bb.body.faces().count(), dst.faces().count());
    let pd = mass_properties(&dst, Tol::witness()).unwrap();
    let pb = mass_properties(&bb.body, Tol::witness()).unwrap();
    assert_eq!(pd.volume.to_bits(), pb.volume.to_bits());
    assert_eq!(pd.surface_area.to_bits(), pb.surface_area.to_bits());
}

/// Planted corruption: every dishonest evidence shape refuses typed,
/// and a refused call mutates nothing.
#[test]
fn dishonest_evidence_refuses_typed_before_mutation() {
    let tol = Tol::witness();

    // Absent: no certificate for the cavity's shell.
    let (mut dst, cavity) = outer_and_cavity();
    let (solid, _) = dst.solids().next().unwrap();
    let pristine = format!("{dst:?}");
    let e = insert_void(&mut dst, solid, cavity, &VoidEvidence::default(), tol).unwrap_err();
    assert!(
        matches!(e, VoidInsertError::MissingEvidence { .. }),
        "{e:?}"
    );
    assert_eq!(
        format!("{dst:?}"),
        pristine,
        "refusal mutated the destination"
    );

    // Failed: probed verdicts that are not strict-inside.
    for verdict in [SolidContainment::Out, SolidContainment::OnBoundary] {
        let (mut dst, cavity) = outer_and_cavity();
        let (solid, _) = dst.solids().next().unwrap();
        let evidence = VoidEvidence {
            shells: cavity
                .shells()
                .map(|(s, _)| (s, VoidContainment::Probed(verdict)))
                .collect(),
        };
        let pristine = format!("{dst:?}");
        let e = insert_void(&mut dst, solid, cavity, &evidence, tol).unwrap_err();
        assert!(
            matches!(e, VoidInsertError::NotStrictlyContained { .. }),
            "{verdict:?}: {e:?}"
        );
        assert_eq!(
            format!("{dst:?}"),
            pristine,
            "refusal mutated the destination"
        );
    }

    // Failed: carried signs that are not strictly positive (touching
    // or escaped 2-D containment).
    for sign in [Sign::Zero, Sign::Negative] {
        let (mut dst, cavity) = outer_and_cavity();
        let (solid, _) = dst.solids().next().unwrap();
        let evidence = VoidEvidence {
            shells: cavity
                .shells()
                .map(|(s, _)| (s, VoidContainment::Carried { sign }))
                .collect(),
        };
        let pristine = format!("{dst:?}");
        let e = insert_void(&mut dst, solid, cavity, &evidence, tol).unwrap_err();
        assert!(
            matches!(e, VoidInsertError::NotStrictlyContained { .. }),
            "{sign:?}: {e:?}"
        );
        assert_eq!(
            format!("{dst:?}"),
            pristine,
            "refusal mutated the destination"
        );
    }

    // Foreign: evidence naming a shell the cavity body does not hold.
    // (A two-shell donor guarantees a key outside the single-shell
    // cavity's arena slots.)
    let (mut donor, donor_cavity) = outer_and_cavity();
    let (donor_solid, _) = donor.solids().next().unwrap();
    let donor_evidence = probed_in(&donor_cavity);
    insert_void(&mut donor, donor_solid, donor_cavity, &donor_evidence, tol).unwrap();
    let foreign = donor.shells().nth(1).unwrap().0;

    let (mut dst, cavity) = outer_and_cavity();
    let (solid, _) = dst.solids().next().unwrap();
    let mut evidence = probed_in(&cavity);
    evidence
        .shells
        .push((foreign, VoidContainment::Probed(SolidContainment::In)));
    let pristine = format!("{dst:?}");
    let e = insert_void(&mut dst, solid, cavity, &evidence, tol).unwrap_err();
    assert!(matches!(e, VoidInsertError::ForeignShell { .. }), "{e:?}");
    assert_eq!(
        format!("{dst:?}"),
        pristine,
        "refusal mutated the destination"
    );

    // Duplicate: two certificates for one shell is a caller desync,
    // refused typed (never first-match-wins), and mutates nothing.
    let (mut dst, cavity) = outer_and_cavity();
    let (solid, _) = dst.solids().next().unwrap();
    let mut evidence = probed_in(&cavity);
    let dup = evidence.shells[0].0;
    evidence
        .shells
        .push((dup, VoidContainment::Carried { sign: Sign::Zero }));
    let pristine = format!("{dst:?}");
    let e = insert_void(&mut dst, solid, cavity, &evidence, tol).unwrap_err();
    assert!(
        matches!(e, VoidInsertError::DuplicateEvidence { .. }),
        "{e:?}"
    );
    assert_eq!(
        format!("{dst:?}"),
        pristine,
        "refusal mutated the destination"
    );
}
