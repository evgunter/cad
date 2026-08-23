//! **M9-2 PR-2 acceptance — the A5 at-rest census door.**
//!
//! The census admits every carrier kind and certifies through its
//! arms: the exact planar sweeps (unchanged), the conformal face-pair
//! arm (shared-carrier opposed-sense curved pairs through the
//! chart-region predicate), face-granularity backing from
//! curve/patch records, the patch-record certifier, and the
//! closed-form tangent-locus helper (the M9-1 PR-2 DEV-1 ruling).
//! Three-outcome ε honesty on every new row; nothing blesses from
//! discovery (F1).
//!
//! ASM R2-b consumption (ASM-R2-SPEC-DRAFT:39-58) is exercised in its
//! own currency: a touching two-instance assembly with declared
//! planar Rest records validates at 3′, and the same pair UNDECLARED
//! is the F1 hard error — `two_instance` rows below.
//!
//! The conformal face-pair arm and the patch-record certifier are
//! pinned at the CENSUS door (`census.rs`'s in-src rows — their
//! fixtures are open sheet scaffolds below tier 3's closed-body bar);
//! the curved-at-3′ end-to-end rows are the boss suite's
//! (`sweep/tests/m5_pr9_boss_union.rs`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use geom::Surface;
use geom_core::Tol;
use geom_core::{Band, Point3, Vec3};
use topo::{
    Body, CensusContact, ContactRecords, TangentLocus, TangentLocusError, ValidationError,
    VvContact, tangent_locus, validate_pseudomanifold,
};

fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

/// A unit cube body translated by `(dx, dy, dz)`.
fn cube_at(dx: f64, dy: f64, dz: f64) -> Body<f64> {
    common::mapped_cube(|x, y, z| Point3::new(x + dx, y + dy, z + dz))
}

/// Grafts `b`'s solid into `a` (two instances in one arena).
fn assembly(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let mut out = a.clone();
    topo::graft_disjoint(&mut out, b, Tol::witness()).unwrap();
    out
}

// ---------------------------------------------------------------------
// #382 half-2: inter-instance overlap surfaces as undeclared contact
// through the SAME census arms — never an ad-hoc graft check.
// ---------------------------------------------------------------------

#[test]
fn overlapping_instances_refuse_as_undeclared_contact_naming_the_pair() {
    // Two tier-3-valid unit cubes overlapping by a half-cube: each
    // passes its own gate, the graft passes tier 3 (the #382 finding
    // — no local check compares the solids), and the 3′ census now
    // surfaces the interpenetration as the A5 hard error.
    let a = cube_at(0.0, 0.0, 0.0);
    let b = cube_at(0.5, 0.5, 0.5);
    let body = assembly(&a, &b);
    assert_eq!(
        topo::validate_geometric(&body, Tol::witness()),
        Ok(()),
        "the #382 finding: tier 3 alone cannot see inter-solid overlap"
    );
    let errors = validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness())
        .expect_err("the aggregate 3′ census refuses the overlap");
    // The guilty pair is NAMED: a proper edge-face pierce between the
    // instances (categorically undeclarable — crossing, not touching).
    assert!(
        errors.iter().any(|e| matches!(
            e,
            ValidationError::UndeclaredContact {
                contact: CensusContact::EdgeFacePierce { .. },
                ..
            }
        )),
        "the overlap surfaces as a pierce naming edge and face: {errors:?}"
    );
}

#[test]
fn disjoint_instances_validate_at_three_prime_with_no_records() {
    let a = cube_at(0.0, 0.0, 0.0);
    let b = cube_at(3.0, 0.0, 0.0);
    let body = assembly(&a, &b);
    assert_eq!(
        validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness()),
        Ok(()),
        "disjoint instances have nothing to declare"
    );
}

// ---------------------------------------------------------------------
// The R2-b evidence rows: a touching two-instance assembly with
// declared planar Rest validates; undeclared is the F1 hard error.
// ---------------------------------------------------------------------

/// The touching pair: cube on cube, stacked exactly (B's bottom on
/// A's top), corners coincident — every census finding is a corner
/// v-v coincidence plus the face-interior rests it implies.
fn stacked() -> (Body<f64>, ContactRecords) {
    let a = cube_at(0.0, 0.0, 0.0);
    let b = cube_at(0.0, 0.0, 1.0);
    let body = assembly(&a, &b);
    // Declare the interface at its census granularity: the four
    // coincident corner pairs (the planar C2 tables' currency —
    // exactly what an ASM mate's declaration resolves to).
    let mut records = ContactRecords::default();
    let mut at = |x: f64, y: f64| {
        let mut hits = Vec::new();
        for (vk, v) in body.vertices() {
            let p = body.get_point(v.point).unwrap();
            if (p.x - x).abs() < 1e-9 && (p.y - y).abs() < 1e-9 && (p.z - 1.0).abs() < 1e-9 {
                hits.push(vk);
            }
        }
        let [va, vb] = hits[..] else {
            panic!("exactly two corners at ({x}, {y}, 1)");
        };
        records.vv.push(VvContact { a: va, b: vb });
    };
    at(0.0, 0.0);
    at(1.0, 0.0);
    at(1.0, 1.0);
    at(0.0, 1.0);
    (body, records)
}

#[test]
fn a_declared_touching_two_instance_assembly_validates_at_three_prime() {
    let (body, records) = stacked();
    assert_eq!(
        validate_pseudomanifold(&body, &records, Tol::witness()),
        Ok(()),
        "the declared planar Rest interface certifies (R2-b's evidence row)"
    );
}

#[test]
fn the_same_touching_pair_undeclared_is_the_f1_hard_error() {
    let (body, _) = stacked();
    let errors = validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness())
        .expect_err("scan-to-bless is banned across the seam");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, ValidationError::UndeclaredContact { .. })),
        "the undeclared touch is the hard error: {errors:?}"
    );
}

// ---------------------------------------------------------------------
// The tangent-locus helper (DEV-1): closed forms, three-outcome rows.
// ---------------------------------------------------------------------

fn plane_at(z: f64) -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, z),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

fn cyl_r(cy: f64, r: f64) -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(0.0, cy, 0.0),
        axis: Vec3::unit_x(),
        radius: r,
        u_ref: Vec3::unit_y(),
    }
}

#[test]
fn plane_on_cylinder_tangency_mints_the_ruling_and_refuses_apart_or_crossing() {
    // The x-axis cylinder (radius 1, centre height 1) rests on z = 0:
    // the tangent ruling is the x-axis itself.
    let cyl = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 1.0),
        axis: Vec3::unit_x(),
        radius: 1.0,
        u_ref: Vec3::unit_z(),
    };
    let TangentLocus::Line { origin, dir } = tangent_locus(&plane_at(0.0), &cyl, band()).unwrap();
    assert!(
        origin.z.abs() < 1e-12 && origin.y.abs() < 1e-12,
        "{origin:?}"
    );
    assert!(
        dir.y.abs() < 1e-12 && dir.z.abs() < 1e-12,
        "the ruling is axial: {dir:?}"
    );
    // Apart and crossing refuse with the honest side named.
    match tangent_locus(&plane_at(-0.5), &cyl, band()) {
        Err(TangentLocusError::NotTangent { apart: true }) => {}
        other => panic!("clearance must refuse apart: {other:?}"),
    }
    match tangent_locus(&plane_at(0.5), &cyl, band()) {
        Err(TangentLocusError::NotTangent { apart: false }) => {}
        other => panic!("a crossing must refuse crossing: {other:?}"),
    }
    // An oblique axis is outside the closed-form lane, typed.
    let oblique = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 1.0),
        axis: Vec3::new(0.6, 0.0, 0.8),
        radius: 1.0,
        u_ref: Vec3::unit_y(),
    };
    match tangent_locus(&plane_at(0.0), &oblique, band()) {
        Err(TangentLocusError::Unsupported { .. }) => {}
        other => panic!("oblique tangency has no closed form: {other:?}"),
    }
    // An in-band gap ESCALATES (three-outcome honesty on the row).
    match tangent_locus(&plane_at(3e-9), &cyl, band()) {
        Err(TangentLocusError::Escalated(_)) => {}
        other => panic!("an in-band gap must escalate: {other:?}"),
    }
}

#[test]
fn parallel_cylinders_mint_the_external_and_internal_generators() {
    // External: radii 1 + 1, axes 2 apart in y — generator at y = 1.
    let TangentLocus::Line { origin, dir } =
        tangent_locus(&cyl_r(0.0, 1.0), &cyl_r(2.0, 1.0), band()).unwrap();
    assert!((origin.y - 1.0).abs() < 1e-12, "{origin:?}");
    assert!(dir.y.abs() < 1e-12 && dir.z.abs() < 1e-12, "{dir:?}");
    // Internal: r 1 inside r 3, axes 2 apart — generator at y = -1
    // (the small cylinder touches the big one on its far side).
    let TangentLocus::Line { origin, dir: _ } =
        tangent_locus(&cyl_r(0.0, 1.0), &cyl_r(2.0, 3.0), band()).unwrap();
    assert!((origin.y - (-1.0)).abs() < 1e-12, "{origin:?}");
    // Definitely apart / definitely overlapping refuse.
    match tangent_locus(&cyl_r(0.0, 1.0), &cyl_r(5.0, 1.0), band()) {
        Err(TangentLocusError::NotTangent { apart: true }) => {}
        other => panic!("{other:?}"),
    }
    match tangent_locus(&cyl_r(0.0, 1.0), &cyl_r(1.0, 1.0), band()) {
        Err(TangentLocusError::NotTangent { apart: false }) => {}
        other => panic!("{other:?}"),
    }
    // Skew/crossing axes and unsupported kinds are typed.
    let crossed = Surface::Cylinder {
        origin: Point3::new(0.0, 5.0, 0.0),
        axis: Vec3::unit_y(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    match tangent_locus(&cyl_r(0.0, 1.0), &crossed, band()) {
        Err(TangentLocusError::Unsupported { .. }) => {}
        other => panic!("{other:?}"),
    }
    match tangent_locus(&plane_at(0.0), &plane_at(1.0), band()) {
        Err(TangentLocusError::Unsupported { .. }) => {}
        other => panic!("plane×plane has no tangent line: {other:?}"),
    }
}

// ---------------------------------------------------------------------
// R1 review probes (m9-2b-r1)
// ---------------------------------------------------------------------

/// R1 probe (claim 3): the face-granularity backing rung is indexed
/// from the RAW records, before confirmation — a FABRICATED patch
/// record silences the v-v findings its two faces hold. The
/// two-directional certification must still refuse the body through
/// the record's own confirmation; if this ever answers Ok the backing
/// rung has silently glued a contact on an unconfirmed record.
///
/// The fabrication is a pair that is COPLANAR and touches the record's
/// held vertices, but whose trims share no area: the second instance
/// is set beside the first rather than on it, so the two z = 1 faces
/// meet along one edge and nothing more. A record claiming a
/// conformal patch there is false about the geometry — which is what
/// makes the probe's question a question.
#[test]
fn r1_probe_a_bogus_patch_record_cannot_silently_back_the_corners() {
    let body = assembly(&cube_at(0.0, 0.0, 0.0), &cube_at(1.0, 0.0, 1.0));
    // The two z = 1 planar faces (A's top, B's bottom).
    let mut ifaces: Vec<_> = body
        .faces()
        .filter_map(|(k, f)| match body.get_surface(f.surface) {
            Some(Surface::Plane { origin, .. }) if (origin.z - 1.0).abs() < 1e-9 => Some(k),
            _ => None,
        })
        .collect();
    ifaces.sort();
    let [fa, fb] = ifaces[..] else {
        panic!("exactly two z=1 faces, got {ifaces:?}");
    };
    let mut records = ContactRecords::default();
    records.patches.push(topo::PatchContact {
        face_a: fa,
        face_b: fb,
    });
    let errors = validate_pseudomanifold(&body, &records, Tol::witness())
        .expect_err("an unconfirmed patch record must never bless the corners");
    // The record DID back the four corner v-v events (no undeclared
    // finding survives) — the refusal must come from the record's own
    // confirmation instead.
    assert!(
        !errors
            .iter()
            .any(|e| matches!(e, ValidationError::UndeclaredContact { .. })),
        "backing rung reached: {errors:?}"
    );
}

/// R1 FINDING probe (claim 4), FIXED (union fix F3): NESTED parallel
/// cylinders with definite radial clearance (dist < |r1 - r2|,
/// surfaces disjoint, clearance |r1 - r2| - dist = 1.5 m) answer
/// `NotTangent { apart: true }` — the definite-clearance side, as the
/// enum documents. (Pre-fix this probe pinned the `apart: false`
/// "crossing" mislabel the R1 review found.)
#[test]
fn r1_probe_nested_clear_cylinders_are_definitely_apart() {
    // r 1 strictly inside r 3, axes 0.5 apart: clearance 1.5 m.
    match tangent_locus(&cyl_r(0.0, 1.0), &cyl_r(0.5, 3.0), band()) {
        Err(TangentLocusError::NotTangent { apart }) => {
            assert!(apart, "nested surfaces are definitely APART");
        }
        other => panic!("nested clear cylinders are not tangent: {other:?}"),
    }
}

/// R1 probe (claim 4): the tangent-locus gap row is METRE data — the
/// mm twin of an in-band metre gap decides definitely, the metre
/// fixture escalates (scale-covariant, never scale-invariant).
#[test]
fn r1_probe_tangent_locus_gap_row_is_scale_covariant() {
    // Metre twin: cylinder r 1 at height 1 + 3e-9 above the plane —
    // gap 3e-9, in Band{1e-9, 1e-8} ⇒ escalate.
    let cyl_m = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 1.0 + 3e-9),
        axis: Vec3::unit_x(),
        radius: 1.0,
        u_ref: Vec3::unit_z(),
    };
    match tangent_locus(&plane_at(0.0), &cyl_m, band()) {
        Err(TangentLocusError::Escalated(_)) => {}
        other => panic!("metre twin gap 3e-9 must escalate: {other:?}"),
    }
    // mm twin (everything x 1e-3): gap 3e-12, decisively below the
    // band ⇒ the tangency mints. A scale-invariant row would treat
    // the twins identically.
    let cyl_mm = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 1e-3 + 3e-12),
        axis: Vec3::unit_x(),
        radius: 1e-3,
        u_ref: Vec3::unit_z(),
    };
    match tangent_locus(&plane_at(0.0), &cyl_mm, band()) {
        Ok(TangentLocus::Line { .. }) => {}
        other => panic!("mm twin gap 3e-12 is decisively tangent: {other:?}"),
    }
}

/// **R1 DELTA probe (backstop jurisdiction, the record-bridged
/// exclusion): a NESTED instance pair whose solids are bridged by a
/// BOGUS v-v record.** The containment arm defers bridged pairs to
/// the confirm pass; the deferral is honest only if that pass still
/// refuses. The bogus record (two vertices 4 m apart) must answer
/// STALE — the assembly must never validate through the bridge.
#[test]
fn r1_delta_probe_bridged_nested_pair_stays_loud() {
    let outer = common::mapped_cube(|x, y, z| Point3::new(4.0 * x, 4.0 * y, 4.0 * z));
    let inner = cube_at(1.5, 1.5, 1.5);
    let body = assembly(&outer, &inner);
    // Bridge the solids with a bogus record: one vertex of each cube.
    let va = body.vertices().next().unwrap().0;
    let vb = body.vertices().last().unwrap().0;
    let mut records = ContactRecords::default();
    records.vv.push(VvContact { a: va, b: vb });
    let errs = validate_pseudomanifold(&body, &records, Tol::witness())
        .expect_err("the bridge must not silence the nested pair");
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::StaleContactDeclaration { .. })),
        "the bogus bridge is stale: {errs:?}"
    );
}
