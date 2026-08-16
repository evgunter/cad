//! M9-2 PR-2 blinded-review probes (R2), executed against frozen head
//! a1b78954. Each probe asserts OBSERVED behavior: a probe that pins a
//! gap documents reality so the finding is reproducible, not a wish.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use geom_core::{Band, Point3, Vec3};
use geom_surfaces::Surface;
use topo::{
    Body, ContactRecords, PatchContact, TangentLocus, TangentLocusError, ValidationError,
    tangent_locus, validate_pseudomanifold,
};

fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

fn cube_scaled_at(s: f64, dx: f64, dy: f64, dz: f64) -> Body<f64> {
    common::mapped_cube(|x, y, z| Point3::new(s * x + dx, s * y + dy, s * z + dz))
}

fn assembly(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let mut out = a.clone();
    topo::graft_disjoint(&mut out, b).unwrap();
    out
}

/// PROBE (claim 6 / instance.rs language): a unit cube strictly INSIDE
/// a 4-cube — two instances whose material definitely overlaps with NO
/// boundary proximity. instance.rs claims "an inter-instance overlap
/// surfaces as the undeclared-contact hard error"; the census sweeps
/// are boundary-pair sweeps, so a NESTED overlap produces no event.
/// This probe records which way the gate actually answers.
#[test]
fn probe_nested_instance_overlap_at_three_prime() {
    let outer = cube_scaled_at(4.0, 0.0, 0.0, 0.0);
    let inner = cube_scaled_at(1.0, 1.5, 1.5, 1.5);
    let body = assembly(&outer, &inner);
    let verdict = validate_pseudomanifold(&body, &ContactRecords::default());
    // OBSERVED: record it either way; the assert pins the outcome.
    println!("nested overlap verdict: {verdict:?}");
    assert_eq!(
        verdict,
        Ok(()),
        "REVIEW FINDING if this fails: the gate DOES see nested overlap"
    );
}

/// PROBE (claim 3): a bogus PATCH record naming the two touching
/// coplanar faces of a stacked-cube assembly. The face-granularity
/// backing rung will treat the corner v-v events as SUBORDINATE to the
/// record — the probe checks the record itself cannot silently bless
/// the assembly (its own confirm must refuse somehow: contradicted,
/// stale, escalated, or unsupported).
#[test]
fn probe_bogus_planar_patch_record_never_silently_blesses() {
    let a = cube_scaled_at(1.0, 0.0, 0.0, 0.0);
    let b = cube_scaled_at(1.0, 0.0, 0.0, 1.0);
    let body = assembly(&a, &b);
    // The touching pair: A's top face (z = 1, outward +z) and B's
    // bottom face (z = 1, outward -z).
    let mut top = None;
    let mut bottom = None;
    for (k, f) in body.faces() {
        if let Some(Surface::Plane { origin, normal, .. }) = body.get_surface(f.surface) {
            if (origin.z - 1.0).abs() < 1e-12 {
                let out = if f.sense { *normal } else { -*normal };
                if out.z > 0.5 {
                    top = Some(k);
                } else if out.z < -0.5 {
                    bottom = Some(k);
                }
            }
        }
    }
    let mut records = ContactRecords::default();
    records.patches.push(PatchContact {
        face_a: top.expect("A top"),
        face_b: bottom.expect("B bottom"),
    });
    let verdict = validate_pseudomanifold(&body, &records);
    println!("bogus planar patch verdict: {verdict:?}");
    assert!(
        verdict.is_err(),
        "MAJOR if this fails: a fabricated patch record silently blessed \
         a touching assembly while suppressing its corner events"
    );
}

fn cyl_at(cy: f64, r: f64) -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(0.0, cy, 0.0),
        axis: Vec3::unit_x(),
        radius: r,
        u_ref: Vec3::unit_y(),
    }
}

/// PROBE (claim 4): NESTED parallel cylinders (axis offset 0.5 <
/// |r1 - r2| = 2): the surfaces definitely do NOT intersect (minimum
/// surface-to-surface distance 1.5), yet `tangent_locus` answers
/// `NotTangent { apart: false }` — documented as "definite crossing".
/// The probe pins the mislabel.
#[test]
fn probe_tangent_locus_nested_cylinders_labeled_crossing() {
    match tangent_locus(&cyl_at(0.0, 1.0), &cyl_at(0.5, 3.0), band()) {
        Err(TangentLocusError::NotTangent { apart }) => {
            println!("nested cylinders: apart = {apart}");
            assert!(
                !apart,
                "if this fails the mislabel was fixed: nested surfaces are APART"
            );
        }
        other => panic!("nested pair must be NotTangent, got {other:?}"),
    }
}

/// PROBE (claim 4): mm-vs-metre behavior of the tangent-locus rows.
/// The rows are metre data: an absolute in-band gap escalates at any
/// model scale, and a definite gap stays definite when the model
/// scales up 1000x (no hidden normalization by model size).
#[test]
fn probe_tangent_locus_rows_are_metre_dimensioned() {
    // mm-scale model, in-band absolute gap: escalates.
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let mm_cyl = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 1e-3 + 3e-9),
        axis: Vec3::unit_x(),
        radius: 1e-3,
        u_ref: Vec3::unit_z(),
    };
    match tangent_locus(&plane, &mm_cyl, band()) {
        Err(TangentLocusError::Escalated(_)) => {}
        other => panic!("mm twin with in-band gap must escalate: {other:?}"),
    }
    // 1000x-scale exact tangency still mints, with the ruling scaled.
    let big = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 1e3),
        axis: Vec3::unit_x(),
        radius: 1e3,
        u_ref: Vec3::unit_z(),
    };
    match tangent_locus(&plane, &big, band()) {
        Ok(TangentLocus::Line { origin, .. }) => {
            assert!(origin.z.abs() < 1e-9, "{origin:?}");
        }
        other => panic!("1000x tangency must mint: {other:?}"),
    }
}
