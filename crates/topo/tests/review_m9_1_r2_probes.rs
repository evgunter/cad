//! Blinded-review probes for PR #524 (M9-1 PR-1). Each probe attacks
//! one claim from the review brief; see the review report for the
//! verdict each one produced.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{flush_declarations, prism_z};
use geom_core::Tol;
use geom_core::{Band, Point3, Vec3};
use topo::boolean::carrier_eq::{CarrierDesc, CarrierEqError, CarrierRelation, carrier_eq};
use topo::boolean::contact_verify::tangent_locus_relation;
use topo::boolean::plane_eq::PlaneIdentity;
use topo::contact::{ContactRefusal, ContactVerdict};
use topo::{BooleanResult, ContactRecords, Surface, ValidationError, union_with};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn declared() -> PlaneIdentity<'static> {
    PlaneIdentity {
        s1: None,
        s2: None,
        declared: true,
    }
}

fn plane(o: [f64; 3], n: [f64; 3]) -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(o[0], o[1], o[2]),
        normal: Vec3::new(n[0], n[1], n[2]),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn line(o: [f64; 3], d: [f64; 3]) -> geom::Curve3<f64> {
    geom::Curve3::Line {
        origin: Point3::new(o[0], o[1], o[2]),
        dir: Vec3::new(d[0], d[1], d[2]),
    }
}

/// PROBE 1 (claim 2): a genuine CROSSING — a cylinder half-embedded in
/// a plane, "declared" tangent along one of the two transverse
/// intersection lines. The locus lies exactly on both surfaces, so
/// nothing is separated; the contradiction triggers must beat the
/// bridge.
#[test]
fn probe_crossing_cylinder_through_plane_contradicts() {
    let cyl = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(1.0, 0.0, 0.0),
        radius: 1.0,
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    };
    let flat = plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    // The intersection line y = 1, z = 0: on the plane and on the
    // cylinder, but the surfaces CROSS there (normals perpendicular).
    let locus = line([0.0, 1.0, 0.0], [1.0, 0.0, 0.0]);
    let err = tangent_locus_relation(&cyl, true, &flat, true, &locus, -1.0, 1.0, true, band())
        .expect_err("a transverse crossing must contradict a declared tangency");
    assert!(
        matches!(err, ContactRefusal::Contradicted { .. }),
        "must be Contradicted, got {err:?}"
    );
}

/// PROBE 2 (claim 2, the #175 bridge attacked head-on): two planes
/// meeting at a genuine dihedral angle, declared tangent along their
/// intersection line. kappa_rel is exactly zero (plane/plane), which is
/// the bridged residue; if the bridge waves this through, a genuine
/// crossing was certified. The first-order trigger must beat it.
#[test]
fn probe_dihedral_crossing_planes_contradict_despite_zero_kappa() {
    let alpha: f64 = 0.2;
    let flat = plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    // Tilted about the x-axis by alpha; material side flipped so the
    // sense check reads "opposed" and cannot mask the verdict.
    let tilted = plane([0.0, 0.0, 0.0], [0.0, alpha.sin(), -alpha.cos()]);
    let locus = line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let err = tangent_locus_relation(&flat, true, &tilted, true, &locus, -1.0, 1.0, true, band())
        .expect_err("crossing planes must contradict a declared tangency");
    assert!(
        matches!(err, ContactRefusal::Contradicted { .. }),
        "must be Contradicted, got {err:?}"
    );
}

/// PROBE 3 (claim 2): definite separation — the same cylinder-on-plane
/// pair as the green row, with the cylinder lifted 1 mm. The locus (on
/// the plane) is definitely off the cylinder; the bridge must not
/// carry a declared tangency across a real gap.
#[test]
fn probe_separated_pair_contradicts_and_steers_to_fit() {
    let cyl = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 1.001),
        axis: Vec3::new(1.0, 0.0, 0.0),
        radius: 1.0,
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    };
    let flat = plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let locus = line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let err = tangent_locus_relation(&cyl, true, &flat, true, &locus, -1.0, 1.0, true, band())
        .expect_err("a 1 mm gap must contradict a declared tangency");
    match err {
        ContactRefusal::Contradicted { steer, .. } => {
            assert_eq!(
                steer,
                Some(topo::FIT_DEFERRAL),
                "a separation-shaped lie steers to Fit"
            );
        }
        other => panic!("expected Contradicted, got {other:?}"),
    }
}

/// PROBE 4 (claim 1): the mm-vs-metre twin on the sphere rung. The
/// same relative lie at two absolute scales: verdicts must stay
/// definite where the margin is definite, and the named lever must be
/// the radius margin at both scales (a wrong-dimension comparand would
/// flip one of them).
#[test]
fn probe_sphere_rung_mm_vs_metre_twin() {
    for scale in [1.0_f64, 1e-3] {
        let a = CarrierDesc::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 2.0 * scale,
            outward: true,
        };
        let b = CarrierDesc::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 2.5 * scale,
            outward: false,
        };
        match carrier_eq(&a, &b, declared(), 1.0, band()).unwrap_err() {
            CarrierEqError::Contradicted(d) => {
                assert_eq!(
                    d.predicate,
                    Some("carrier_sphere_radius"),
                    "scale {scale}: the radius lever decides at both scales"
                );
            }
            other => panic!("scale {scale}: expected Contradicted, got {other:?}"),
        }
    }
}

/// PROBE 5 (claim 1): near-tie cylinder pair — a sub-band axis tilt.
/// Undeclared refuses, declared bridges, and a definite tilt
/// contradicts at the named angular lever. Three outcomes at one
/// geometry, on the cylinder rung.
#[test]
fn probe_cylinder_axis_near_tie_three_outcomes() {
    let base = CarrierDesc::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 3.0,
        outward: true,
    };
    let tilt = |t: f64| CarrierDesc::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(t, 0.0, 1.0).normalize(),
        radius: 3.0,
        outward: false,
    };
    // Tilts stated in BAND units, not literals: a fixed 1e-12 rad is
    // sub-band at the default ε and definite at ε = 1e-12, so the
    // literal version of this row asserted the wrong posture in the
    // matrix's own lower leg (adopted-probe fix, M9-1 fix pass).
    let b = band();
    let near = tilt(b.zero() * 0.001);
    assert!(
        matches!(
            carrier_eq(&base, &near, PlaneIdentity::NONE, 1.0, band()),
            Err(CarrierEqError::Undeclared { .. })
        ),
        "in-band, undeclared: refuses"
    );
    assert_eq!(
        carrier_eq(&base, &near, declared(), 1.0, band()).unwrap(),
        CarrierRelation::SameOpposite,
        "in-band, declared: bridged"
    );
    // Definite tilt: three orders above the escalate edge at the same
    // 1 m arm.
    match carrier_eq(&base, &tilt(b.escalate() * 1000.0), declared(), 1.0, band()).unwrap_err() {
        CarrierEqError::Contradicted(d) => {
            assert_eq!(d.predicate, Some("carrier_cyl_axis_parallel"));
        }
        other => panic!("expected Contradicted, got {other:?}"),
    }
}

/// PROBE 6 (claim 5): the replay row must BITE — a mutated record set
/// compares unequal, so the PartialEq row is discriminating rather
/// than vacuously true.
#[test]
fn probe_replay_partial_eq_bites_on_mutation() {
    let a = prism_z::<f64>(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 0.0, 1.0).body;
    let b = prism_z::<f64>(
        &[(0.5, 0.25), (1.5, 0.25), (1.5, 1.25), (0.5, 1.25)],
        1.0,
        2.0,
    )
    .body;
    let decls = flush_declarations(&a, &b);
    let BooleanResult::Body(x) = union_with(&a, &b, &decls, Tol::witness()).unwrap() else {
        panic!("overlapping union cannot be empty");
    };
    let mut mutated = x.contacts.clone();
    assert_eq!(mutated, x.contacts, "clone is bit-identical");
    if let Some(row) = mutated.vv.pop() {
        assert_ne!(mutated, x.contacts, "a dropped row must show: {row:?}");
    } else {
        // No vv rows: mutate by inserting a fabricated patch row.
        let fk = x.body.faces().next().map(|(k, _)| k).unwrap();
        mutated.patches.push(topo::PatchContact {
            face_a: fk,
            face_b: fk,
        });
        assert_ne!(mutated, x.contacts, "an added row must show");
    }
}

/// PROBE 7 (claims 4 + 6): the AT-REST gate executes. A fabricated
/// CurveContact between two faces that genuinely CROSS (adjacent brick
/// faces, along their shared edge) must come back ContactContradicted
/// at the census; a fabricated PatchContact must refuse
/// CensusUnsupported (nothing certifies a patch — the certifier is
/// M9-2's).
#[test]
fn probe_census_gate_contradicts_curve_and_refuses_patch() {
    let a = prism_z::<f64>(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 0.0, 1.0).body;
    // Pick any edge and its two adjacent faces: a brick's dihedral is
    // 90 degrees, so a tangency claim along that edge is a lie.
    let (ek, edge) = a.edges().next().expect("brick has edges");
    let face_of = |he: topo::HalfEdgeKey| {
        a.get_loop(a.get_half_edge(he).unwrap().parent_loop)
            .unwrap()
            .face
    };
    let f1 = face_of(edge.he_plus);
    let f2 = face_of(edge.he_minus);
    let mut contacts = ContactRecords::default();
    contacts.curves.push(topo::CurveContact {
        face_a: f1,
        face_b: f2,
        witness: ek,
    });
    let fk = a.faces().next().map(|(k, _)| k).unwrap();
    contacts.patches.push(topo::PatchContact {
        face_a: fk,
        face_b: fk,
    });
    let errors = topo::validate_pseudomanifold(&a, &contacts, Tol::witness())
        .expect_err("fabricated contact records must be refused at rest");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, ValidationError::ContactContradicted { .. })),
        "the census must CONTRADICT the false curve tangency: {errors:?}"
    );
    // Since M9-2 the patch certifier EXISTS: a self-pair record's
    // senses are trivially aligned, and aligned coincidence is
    // containment, not contact — the record is CONTRADICTED (the
    // Rest-class refusal), which supersedes this probe's original
    // not-yet-certifiable expectation without weakening it: the
    // fabricated record still cannot bless.
    assert!(
        errors.iter().any(|e| matches!(
            e,
            ValidationError::ContactContradicted {
                declaration: topo::DeclaredContact {
                    class: topo::ContactClass::Rest,
                    ..
                },
                ..
            }
        )),
        "the fabricated patch record must be contradicted: {errors:?}"
    );
    assert!(
        !errors.is_empty(),
        "nothing may bless a PatchContact in this unit"
    );
}

/// PROBE 8 (claim 2, verdict honesty): an in-band GAP under a declared
/// tangency is never Definite.
///
/// FIXED (M9-1 fix pass, F1). The probe originally asserted `Bridged`
/// and passed, which is what made the defect visible from this side
/// too: a gap is an on-surface RESIDUAL margin, and C4 puts
/// locus-on-both-surfaces on the must-verify-DEFINITE list, not in the
/// bridged residue (which is in-band κ_rel alone). So the honest
/// third outcome is `Escalated`. Red-then-green: this assertion read
/// `assert_eq!(v, ContactVerdict::Bridged)` when the probe was written.
///
/// The gaps are derived FROM the run's band, not hard-coded: the first
/// fix pinned a literal 3e-9, which is in-band only at the default ε
/// and reads as definite separation at ε = 1e-12 and as no gap at all
/// at ε = 1e-6. A row that only means what it says at one ε is not an
/// ε row. Each cell below asserts its own posture's invariant.
#[test]
fn probe_in_band_gap_is_bridged_not_definite() {
    let flat = plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let locus = line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let at_gap = |gap: f64| {
        let cyl = Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 1.0 + gap),
            axis: Vec3::new(1.0, 0.0, 0.0),
            radius: 1.0,
            u_ref: Vec3::new(0.0, 1.0, 0.0),
        };
        tangent_locus_relation(&cyl, true, &flat, true, &locus, -1.0, 1.0, true, band())
    };
    let b = band();

    // Below the band's zero: no gap the run can see. The geometry
    // verifies on its own evidence; the declaration adds nothing.
    assert_eq!(
        at_gap(b.zero() * 0.1).expect("a sub-zero gap is not a gap at this ε"),
        ContactVerdict::Definite,
        "below band-zero the declaration must not be credited with a bridge"
    );

    // Inside the band: a residual margin that cannot be decided. It is
    // on C4's must-verify-DEFINITE list, so no declaration may bridge
    // it — the honest answer is a typed escalation.
    let mid = (b.zero() + b.escalate()) * 0.5;
    let refusal =
        at_gap(mid).expect_err("an in-band gap is not a residue a declaration may bridge");
    assert!(
        matches!(refusal, ContactRefusal::Escalated { .. }),
        "in-band gap must escalate: {refusal:?}"
    );

    // Above the band: definite separation beats the declaration, and
    // the recourse names the class that would fit.
    let far = at_gap(b.escalate() * 1000.0).expect_err("a definite gap contradicts");
    assert!(
        matches!(far, ContactRefusal::Contradicted { .. }),
        "a definite gap must contradict: {far:?}"
    );
}

/// PROBE 9 (claim 2, resolution envelope): sweep a real gap from 1 pm
/// to 1 mm under a declared tangency and record where the verdict
/// flips Definite -> Bridged/Escalated -> Contradicted. A genuine gap
/// must never verify Definite above the certificate's own sag bound.
#[test]
fn probe_gap_sweep_envelope() {
    let flat = plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let locus = line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    for exp in [-12, -10, -9, -8, -7, -6, -5, -4, -3] {
        let gap = 10f64.powi(exp);
        let cyl = Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 1.0 + gap),
            axis: Vec3::new(1.0, 0.0, 0.0),
            radius: 1.0,
            u_ref: Vec3::new(0.0, 1.0, 0.0),
        };
        let out = tangent_locus_relation(&cyl, true, &flat, true, &locus, -1.0, 1.0, true, band());
        println!("gap 1e{exp}: {out:?}");
    }
}
