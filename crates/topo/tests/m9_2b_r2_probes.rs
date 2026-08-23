//! M9-2 PR-2 blinded-review probes (R2), executed against frozen head
//! a1b78954. Each probe asserts OBSERVED behavior: a probe that pins a
//! gap documents reality so the finding is reproducible, not a wish.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use geom::Surface;
use geom_core::Tol;
use geom_core::{Band, Point3, Vec3};
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
    topo::graft_disjoint(&mut out, b, Tol::witness()).unwrap();
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
    let verdict = validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness());
    // The union fix (F1): the containment arm of the loudness
    // backstop refuses the nested instance pair as UNDECIDABLE —
    // C6's interference class, named — instead of validating
    // silently. (Pre-fix this probe pinned the silent Ok(()) the R2
    // review found.)
    println!("nested overlap verdict: {verdict:?}");
    let errs = verdict.expect_err("nested instance extents refuse loudly");
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::CensusUndecidable { .. })),
        "the backstop names the contained pair: {errs:?}"
    );
}

/// PROBE (claim 3): a FABRICATED patch record naming two coplanar
/// faces of a two-instance assembly. The face-granularity backing rung
/// will treat the v-v events those faces hold as SUBORDINATE to the
/// record — the probe checks the record itself cannot silently bless
/// the assembly (its own confirm must refuse somehow: contradicted,
/// stale, escalated, or unsupported).
///
/// The fabrication is what makes the question a question: the second
/// instance stands BESIDE the first, so the two z = 1 faces are one
/// carrier that meets along a single edge and shares no area, and a
/// record claiming a conformal patch there is false about the
/// geometry.
#[test]
fn probe_bogus_planar_patch_record_never_silently_blesses() {
    let a = cube_scaled_at(1.0, 0.0, 0.0, 0.0);
    let b = cube_scaled_at(1.0, 1.0, 0.0, 1.0);
    let body = assembly(&a, &b);
    // The declared pair: A's top face (z = 1, outward +z) and B's
    // bottom face (z = 1, outward -z).
    let mut top = None;
    let mut bottom = None;
    for (k, f) in body.faces() {
        if let Some(Surface::Plane { origin, normal, .. }) = body.get_surface(f.surface)
            && (origin.z - 1.0).abs() < 1e-12
        {
            let out = if f.sense { *normal } else { -*normal };
            if out.z > 0.5 {
                top = Some(k);
            } else if out.z < -0.5 {
                bottom = Some(k);
            }
        }
    }
    let mut records = ContactRecords::default();
    records.patches.push(PatchContact {
        face_a: top.expect("A top"),
        face_b: bottom.expect("B bottom"),
    });
    let verdict = validate_pseudomanifold(&body, &records, Tol::witness());
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

/// PROBE (claim 4), FIXED (union fix F3): NESTED parallel cylinders
/// (axis offset 0.5 < |r1 - r2| = 2, minimum surface distance 1.5)
/// answer `NotTangent { apart: true }` — the definite-clearance side.
/// (Pre-fix this probe pinned the "crossing" mislabel.)
#[test]
fn probe_tangent_locus_nested_cylinders_are_apart() {
    match tangent_locus(&cyl_at(0.0, 1.0), &cyl_at(0.5, 3.0), band()) {
        Err(TangentLocusError::NotTangent { apart }) => {
            println!("nested cylinders: apart = {apart}");
            assert!(apart, "nested surfaces are definitely APART");
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

/// F4 (union fix): the backstop rows are METRE data — scaling the
/// geometry alone (the gate's band is the ambient metre band) leaves
/// the DEFINITE verdicts standing at mm and km scale: a nested pair
/// refuses undecidable, a far-separated pair stays clean, at every
/// scale. A scale-invariant (dimensionless) margin would flip one.
#[test]
fn the_backstop_rows_are_metre_dimensioned() {
    for s in [1e-3, 1.0, 1e3] {
        let outer = cube_scaled_at(4.0 * s, 0.0, 0.0, 0.0);
        let inner = cube_scaled_at(s, 1.5 * s, 1.5 * s, 1.5 * s);
        let nested = assembly(&outer, &inner);
        let errs = validate_pseudomanifold(&nested, &ContactRecords::default(), Tol::witness())
            .expect_err("nested refuses at every scale");
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::CensusUndecidable { .. })),
            "scale {s}: {errs:?}"
        );
        let far = cube_scaled_at(s, 10.0 * s, 0.0, 0.0);
        let apart = assembly(&outer, &far);
        assert_eq!(
            validate_pseudomanifold(&apart, &ContactRecords::default(), Tol::witness()),
            Ok(()),
            "scale {s}: separated instances stay clean"
        );
    }
}
