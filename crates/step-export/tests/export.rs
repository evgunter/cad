//! M4 PR 7 STEP-export acceptance: parse-back oracles (F6 decision —
//! ruststep syntactic, truck-stepio semantic, dev-dependencies only),
//! determinism, conformance pins for the preamble, and the typed
//! refusals. External FreeCAD/OCC acceptance runs over the committed
//! fixtures via `scripts/check_step.sh` (golden-file tests below keep
//! the fixtures in lockstep with the writer).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use step_export::{StepExportError, StepOptions, step_string};
use geom_core::Tol;

/// Exports with the fixed test options: product name per shape, and an
/// EXPLICIT uncertainty (the default ε) so the suite and the golden
/// fixtures are hermetic against `CAD_EPS`-style env overrides of the
/// ambient tolerance. Must match `examples/export_fixtures.rs`.
fn export(body: &topo::Body<f64>, name: &str) -> String {
    let options = StepOptions {
        product_name: name.to_owned(),
        uncertainty_m: Some(1e-9),
        ..StepOptions::default()
    };
    step_string(body, &options, Tol::witness()).unwrap()
}

/// Counts non-overlapping occurrences of `needle`.
fn count(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).count()
}

/// Parses with ruststep (syntactic Part 21 oracle) and returns the
/// entity-instance count of the single data section.
fn ruststep_instances(text: &str) -> usize {
    let exchange = ruststep::parser::parse(text).expect("ruststep Part 21 parse");
    assert_eq!(exchange.data.len(), 1, "one data section");
    exchange.data[0].entities.len()
}

/// Reconstructs every shell with truck-stepio (semantic oracle) and
/// returns per-shell (faces, edges, vertices), in table order.
fn truck_shells(text: &str) -> Vec<(usize, usize, usize)> {
    let table = truck_stepio::r#in::Table::from_step(text).expect("truck table parse");
    let mut ids: Vec<_> = table.shell.keys().copied().collect();
    ids.sort_unstable(); // deterministic report order for assertions
    ids.iter()
        .map(|id| {
            let shell = &table.shell[id];
            let cs = table
                .to_compressed_shell(shell)
                .expect("truck shell reconstruction");
            (cs.faces.len(), cs.edges.len(), cs.vertices.len())
        })
        .collect()
}

// ---------------------------------------------------------------- cube

#[test]
fn cube_reconstructs_6_12_8() {
    let body = common::cube();
    let text = export(&body, "cube");
    assert_eq!(common::census(&body), (6, 12, 8), "kernel census");
    assert_eq!(
        truck_shells(&text),
        vec![(6, 12, 8)],
        "independent importer"
    );
}

#[test]
fn cube_vertex_coordinates_survive() {
    let text = export(&common::cube(), "cube");
    let table = truck_stepio::r#in::Table::from_step(&text).expect("truck table parse");
    let (_, shell) = table.shell.iter().next().expect("one shell");
    let cs = table.to_compressed_shell(shell).expect("reconstruction");
    let mut corners: Vec<[i64; 3]> = cs
        .vertices
        .iter()
        .map(|p| {
            // Corners are exact 0/1 floats; verify bit-exactness by
            // integer identity after an exactness check.
            assert!(p.x == 0.0 || p.x == 1.0, "x {:?}", p.x);
            assert!(p.y == 0.0 || p.y == 1.0, "y {:?}", p.y);
            assert!(p.z == 0.0 || p.z == 1.0, "z {:?}", p.z);
            [p.x as i64, p.y as i64, p.z as i64]
        })
        .collect();
    corners.sort_unstable();
    corners.dedup();
    assert_eq!(corners.len(), 8, "all eight distinct corners");
}

#[test]
fn cube_syntactic_parse_and_instance_pin() {
    let text = export(&common::cube(), "cube");
    // Entity-instance count is a determinism tripwire: the number is a
    // pure function of the walk; drift means the emission scheme
    // changed (update deliberately with the golden fixtures).
    assert_eq!(ruststep_instances(&text), 170);
}

// ------------------------------------------------- die (M3 boolean cut)

#[test]
fn die_reconstructs_matching_kernel_census() {
    let body = common::die(0.0, 0.0, 0.0);
    let text = export(&body, "die");
    let kernel = common::census(&body);
    assert_eq!(kernel, (11, 24, 16), "die census (top face carries a ring)");
    assert_eq!(truck_shells(&text), vec![kernel], "independent importer");
    // The pocket mouth is the ONE interior ring in the whole body:
    // exactly one FACE_BOUND, ten FACE_OUTER_BOUNDs... (one per face).
    assert_eq!(count(&text, "= FACE_BOUND("), 1);
    assert_eq!(count(&text, "= FACE_OUTER_BOUND("), 11);
}

// ------------------------------------- kiss assembly (multi-shell body)

#[test]
fn kiss_assembly_exports_two_solids() {
    let body = common::kiss_assembly();
    let text = export(&body, "kiss_assembly");
    // One kernel solid, two outward shells → two MANIFOLD_SOLID_BREPs.
    assert_eq!(count(&text, "MANIFOLD_SOLID_BREP("), 2);
    let shells = truck_shells(&text);
    assert_eq!(shells, vec![(11, 24, 16), (11, 24, 16)], "two die shells");
    let total: (usize, usize, usize) = shells
        .iter()
        .fold((0, 0, 0), |acc, s| (acc.0 + s.0, acc.1 + s.1, acc.2 + s.2));
    assert_eq!(total, common::census(&body), "nothing shared, nothing lost");
}

// -------------------------------------------------------- determinism

#[test]
fn byte_identity_same_body_and_same_recipe() {
    let body = common::die(0.0, 0.0, 0.0);
    let options = StepOptions::default();
    let first = step_string(&body, &options, Tol::witness()).unwrap();
    let again = step_string(&body, &options, Tol::witness()).unwrap();
    assert_eq!(first, again, "same body value ⇒ byte-identical");
    let rebuilt = common::die(0.0, 0.0, 0.0);
    let from_rebuild = step_string(&rebuilt, &options, Tol::witness()).unwrap();
    assert_eq!(first, from_rebuild, "same recipe ⇒ byte-identical");
}

// ------------------------------------------------- conformance pins

#[test]
fn preamble_conformance_pins() {
    let text = export(&common::cube(), "cube");
    // FILE_SCHEMA names the AP the data section instantiates (AP214
    // ed. 2 object identifier) — the truck-stepio disqualifier class.
    assert!(text.contains("FILE_SCHEMA(('AUTOMOTIVE_DESIGN { 1 0 10303 214 1 1 1 1 }'));"));
    assert!(text.contains(
        "APPLICATION_PROTOCOL_DEFINITION('international standard', 'automotive_design', 2000"
    ));
    // Units: unprefixed metres (kernel-internal metres, D6) — no
    // hardcoded millimetre lie.
    assert!(text.contains("SI_UNIT($, .METRE.)"));
    assert!(!text.contains(".MILLI."));
    // Uncertainty rides the LENGTH_MEASURE select, never a bare real.
    assert!(text.contains("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE("));
    // ADVANCED_FACE under ABSR (never FACE_SURFACE), one per face;
    // a cube has no rings, so no plain FACE_BOUND.
    assert!(text.contains("ADVANCED_BREP_SHAPE_REPRESENTATION("));
    assert_eq!(count(&text, "= ADVANCED_FACE("), 6);
    assert!(!text.contains("FACE_SURFACE"));
    assert_eq!(count(&text, "= FACE_BOUND("), 0);
    // Exchange-structure frame.
    assert!(text.starts_with("ISO-10303-21;\nHEADER;\n"));
    assert!(text.ends_with("ENDSEC;\nEND-ISO-10303-21;\n"));
    assert!(text.contains("FILE_DESCRIPTION((''), '2;1');"));
}

#[test]
fn uncertainty_override_and_validation() {
    let body = common::cube();
    let options = StepOptions {
        uncertainty_m: Some(2.5e-6),
        ..StepOptions::default()
    };
    let text = step_string(&body, &options, Tol::witness()).unwrap();
    assert!(text.contains("LENGTH_MEASURE(2.5E-6)"));
    for bad in [0.0, -1e-9, f64::NAN, f64::INFINITY] {
        let options = StepOptions {
            uncertainty_m: Some(bad),
            ..StepOptions::default()
        };
        match step_string(&body, &options, Tol::witness()) {
            Err(StepExportError::InvalidUncertainty { .. }) => {}
            other => panic!("expected InvalidUncertainty for {bad}, got {other:?}"),
        }
    }
}

#[test]
fn product_name_escaping() {
    let body = common::cube();
    let options = StepOptions {
        product_name: "o'brien".to_owned(),
        ..StepOptions::default()
    };
    let text = step_string(&body, &options, Tol::witness()).unwrap();
    assert!(text.contains("PRODUCT('o''brien', 'o''brien', '', ("));
    let options = StepOptions {
        product_name: "kübel".to_owned(),
        ..StepOptions::default()
    };
    match step_string(&body, &options, Tol::witness()) {
        Err(StepExportError::UnrepresentableString { .. }) => {}
        other => panic!("expected UnrepresentableString, got {other:?}"),
    }
}

// ------------------------------------------------------ typed refusals

/// The M4 refusal this row used to be — `common::ball()` refusing as
/// `UnsupportedSurface { kind: "sphere" }` — is retired: M5 PR 13
/// gives every elementary surface a printer, so the ball EXPORTS.
/// What is left of the refusal is the NURBS face, which no body at
/// rest carries; it is pinned in `m5_pr13_curved.rs` alongside the
/// arms that replaced this test.
#[test]
fn curved_bodies_export_rather_than_refuse() {
    for (name, body) in [
        ("ball", common::ball()),
        ("cone", common::cone()),
        ("donut", common::donut()),
        ("washer", common::washer()),
        ("notched", common::notched()),
        ("cut_cylinder", common::cut_cylinder()),
        ("boss_union", common::boss_union()),
    ] {
        assert!(
            step_string(&body, &StepOptions::default(), Tol::witness()).is_ok(),
            "{name} must export"
        );
    }
}

#[test]
fn voided_body_refuses_typed() {
    let body = common::voided();
    match step_string(&body, &StepOptions::default(), Tol::witness()) {
        Err(StepExportError::VoidShellUnsupported { volume, .. }) => {
            // The reverted inner shell bounds the unit cavity; its
            // outward-from-material normals point INTO the cavity, so
            // the divergence sum is exactly −1.
            assert_eq!(volume, -1.0, "cavity volume, negatively signed");
        }
        other => panic!("expected VoidShellUnsupported, got {other:?}"),
    }
}

#[test]
fn empty_body_refuses_typed() {
    let body = topo::Body::<f64>::new();
    match step_string(&body, &StepOptions::default(), Tol::witness()) {
        Err(StepExportError::NothingToExport) => {}
        other => panic!("expected NothingToExport, got {other:?}"),
    }
}

#[test]
fn mid_surgery_body_refuses_typed() {
    // mvfs's skeletal body: one face bounded by an empty loop, its
    // surface still the honest "no description yet" NURBS placeholder
    // — legal mid-construction, never exportable. The surface-first
    // walk refuses on the placeholder (had the face carried a real
    // plane, the empty loop would refuse as EmptyLoop next).
    let mut body = topo::Body::<f64>::new();
    body.mvfs(geom_core::Point3::new(0.0, 0.0, 0.0)).unwrap();
    match step_string(&body, &StepOptions::default(), Tol::witness()) {
        Err(StepExportError::UnsupportedSurface { kind, .. }) => {
            assert_eq!(kind, "nurbs placeholder");
        }
        other => panic!("expected UnsupportedSurface, got {other:?}"),
    }
}

// ------------------------------------------------------ golden fixtures

/// The committed fixtures ARE the current writer output — byte-golden.
/// This is the cross-session determinism pin and what keeps
/// `scripts/check_step.sh`'s FreeCAD acceptance in lockstep with the
/// writer. Regenerate deliberately with
/// `cargo run -p step-export --example export_fixtures -- crates/step-export/tests/fixtures`.
#[test]
fn committed_fixtures_are_byte_golden() {
    for (name, body) in common::fixture_corpus() {
        let text = export(&body, name);
        let path = format!("{}/tests/fixtures/{name}.step", env!("CARGO_MANIFEST_DIR"));
        let committed = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("fixture {path} unreadable ({e}) — regenerate"));
        assert_eq!(
            text, committed,
            "{name}.step drifted — regenerate deliberately"
        );
    }
}
