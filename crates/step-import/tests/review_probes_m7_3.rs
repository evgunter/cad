//! Adversarial review probes for M7-3 — authored on the review/m7-3
//! branch, adopted BY MERGE at the fix pass. The two probes that
//! FAILED BY DESIGN as findings are now regression pins: the F1
//! rim-off-wall plant asserts the residual-gate refusal
//! (`probe_arm_b_rim_off_wall_arc`; its positive control is the merged
//! row in `nurbs_import.rs`, see that probe's docs), and the F2
//! sub-unit-direction probe asserts every `PlacedSegment` placement in
//! a certified import is an honest rigid frame.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{census, fixture};
use geom_core::{Affine3, Point2, Vec3};
use profile::RawLoop;
use step_import::{ImportOptions, StepImport, import_step};
use geom_core::Tol;

fn import(text: &str) -> Result<StepImport, step_import::StepImportError> {
    import_step(text, &ImportOptions::default())
}

fn solid(text: &str, who: &str) -> topo::Body<f64> {
    match import(text) {
        Ok(StepImport::Solid { body, .. }) => body,
        other => panic!("{who}: expected a solid import, got {other:?}"),
    }
}

/// The committed fixture's native twin (step-export/tests/common).
fn native_loft_prism() -> topo::Body<f64> {
    let quad = |pts: [(f64, f64); 4]| -> sweep::Section {
        vec![profile::ProfileLoop::polygon(
            pts.iter().map(|&(x, y)| Point2::new(x, y)),
        )]
    };
    let square = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let trapezoid = [(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let sections = vec![quad(square), quad(trapezoid), quad(square)];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
        Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
    ];
    sweep::loft_body::<f64>(&sections, &places, 2)
        .expect("native loft_prism builds")
        .body
}

/// V2: a seam carrier byte-equal to NEITHER wall's boundary iso (a
/// "legitimately refit" foreign seam) is disposed of by the TOLERANCE,
/// never by a gate of its own.
///
/// The plant moves one seam control point by 1e-7 m, and whether that
/// is a falsification is an ε question: at ε_in ≤ 1e-9 the displacement
/// is OUTSIDE tolerance and the seam must refuse TYPED — the probe's
/// original teeth, and where the bitwise `IsoCurve` rung's absence must
/// not become a silent adoption. At ε_in = 1e-6 the displacement is
/// INSIDE tolerance: the file's carrier is honest evidence at the
/// tolerance the caller asked for, declare-and-check certifies it, and
/// refusing it would be a gate tighter than ε_in.
///
/// Both cells are pinned with the plant's own number. The falsifier
/// with real teeth at EVERY ε is `recognize_pins`'s 1e-3 m displaced
/// carrier, which refuses carrying its measured residual.
#[test]
fn probe_refit_seam_refuses_typed() {
    /// The plant's displacement, in metres.
    const PLANT: f64 = 1e-7;
    let eps = geom_core::Tol::witness().get().eps;
    let text = fixture("loft_prism", "step").replace(
        "#90 = CARTESIAN_POINT('', (-1.75, -1.0, 1.0));",
        "#90 = CARTESIAN_POINT('', (-1.7499999, -1.0, 1.0));",
    );
    assert_ne!(text, fixture("loft_prism", "step"), "mutation applied");
    match import(&text) {
        Err(e) => {
            let msg = e.to_string();
            eprintln!("PROBE refit-seam refusal @ eps={eps:e}: {msg}");
            assert!(
                PLANT > eps,
                "a displacement INSIDE ε_in must not be refused: {msg}"
            );
            // The refusal is the ADOPTION verdict on the plant, and it
            // carries the number that caught it: the declare-and-check
            // lane's measured on-locus residual, which must exceed ε_in
            // and be the plant's own order. A refusal from any later
            // stage would mean the seam was adopted and something
            // downstream objected — a different fact, and not this
            // probe's.
            let step_import::StepImportError::Adoption { attempts, .. } = &e else {
                panic!("the plant must be caught at ADOPTION, not downstream: {msg}");
            };
            let measured = attempts.iter().find_map(|a| match a.refusal {
                topo::EulerOpError::Certification {
                    error:
                        geom_brep::CertifyError::PlaneNurbs(geom_brep::PlaneNurbsRefusal::Limb {
                            value,
                            ..
                        }),
                } => Some(value),
                _ => None,
            });
            let Some(measured) = measured else {
                panic!("the refusal must carry the lane's measured bound: {attempts:?}");
            };
            assert!(
                measured > eps && measured < 1e-6,
                "the refusal's own number explains it: on-locus residual {measured:e} m \
                 past ε_in {eps:e}, and of the plant's own order ({PLANT:e} m)"
            );
        }
        Ok(StepImport::Solid { body, .. }) => {
            assert!(
                PLANT < eps,
                "MISADOPTION: refit seam displaced {PLANT:e} m — past ε_in {eps:e} — \
                 imported as a solid (census {:?})",
                census(&body)
            );
            topo::validate_geometric(&body)
                .expect("a seam adopted inside ε_in leaves a body that is valid at rest");
        }
        Ok(other) => panic!("unexpected disposition: {other:?}"),
    }
}

/// V1/V3: a RATIONAL complex instance whose weights are all bitwise
/// 1.0 describes the same kernel surface as the simple arm. The import
/// must land in the identical body state (t3 green, V in bracket).
#[test]
fn probe_all_unit_weight_rational_instance_imports_identically() {
    let orig = fixture("loft_prism", "step");
    let simple = "#87 = B_SPLINE_SURFACE_WITH_KNOTS('', 1, 2, ((#81, #82, #83), (#84, #85, #86)), .UNSPECIFIED., .U., .U., .U., (2, 2), (3, 3), (0.0, 1.0), (0.0, 1.0), .UNSPECIFIED.);";
    let complexed = "#87 = ( BOUNDED_SURFACE() B_SPLINE_SURFACE(1, 2, ((#81, #82, #83), (#84, #85, #86)), .UNSPECIFIED., .U., .U., .U.) B_SPLINE_SURFACE_WITH_KNOTS((2, 2), (3, 3), (0.0, 1.0), (0.0, 1.0), .UNSPECIFIED.) GEOMETRIC_REPRESENTATION_ITEM() RATIONAL_B_SPLINE_SURFACE(((1.0, 1.0, 1.0), (1.0, 1.0, 1.0))) REPRESENTATION_ITEM('') SURFACE() );";
    let text = orig.replace(simple, complexed);
    assert_ne!(text, orig, "wall #87 rewritten as complex instance");
    let body = solid(&text, "all-unit-weight complex instance");
    let base = solid(&orig, "committed fixture");
    assert_eq!(census(&body), census(&base), "census unchanged");
    assert_eq!(topo::validate(&body), Ok(()), "t1");
    assert_eq!(topo::validate_closed(&body), Ok(()), "t2");
    assert_eq!(
        topo::validate_geometric(&body),
        Ok(()),
        "t3 green: all-1.0 weights ARE non-rational"
    );
    let keys: std::collections::BTreeSet<_> = body.faces().map(|(_, f)| f.surface).collect();
    assert_eq!(keys.len(), 6, "surface keys consistent with simple arm");
}

/// V3: a file where the RATIONAL wall's weights are snapped to bitwise
/// 1.0 while its CIRCLE rims remain — the wall becomes mintable, the
/// arc rims are not mintable, and the arc carrier no longer lies on
/// the (now non-rational) wall. Must refuse typed, never launder.
fn native_arc_loft_for_probe() -> topo::Body<f64> {
    let arc_section = |s: f64| -> sweep::Section {
        let v = |x: f64, y: f64, bulge: f64| profile::ProfileVertex::new(Point2::new(x, y), bulge);
        vec![profile::ProfileLoop::new(vec![
            v(-s, -s, 0.0),
            v(s, -s, 0.4142135623730951),
            v(s, s, 0.0),
            v(-s, s, 0.0),
        ])]
    };
    let sections = vec![arc_section(1.0), arc_section(1.25), arc_section(1.0)];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
        Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
    ];
    sweep::loft_body::<f64>(&sections, &places, 2)
        .expect("arc loft builds")
        .body
}

#[test]
fn probe_arc_loft_weights_snapped_to_one_refuses() {
    let native = native_arc_loft_for_probe();
    let text = step_export::step_string(&native, &step_export::StepOptions::default())
        .expect("arc loft exports");
    // Find the RATIONAL_B_SPLINE_SURFACE weight list and replace every
    // weight with 1.0.
    let start = text
        .find("RATIONAL_B_SPLINE_SURFACE(")
        .expect("rational wall present");
    let open = start + "RATIONAL_B_SPLINE_SURFACE(".len();
    let mut depth = 1usize;
    let mut end = open;
    for (i, c) in text[open..].char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    end = open + i;
                    break;
                }
            }
            _ => {}
        }
    }
    let weights = &text[open..end];
    let snapped: String = {
        // Replace every real token with 1.0, keeping list structure.
        let mut out = String::new();
        for part in weights.split(',') {
            let trimmed = part.trim();
            let lead: String = trimmed.chars().take_while(|c| *c == '(').collect();
            let tail: String = trimmed.chars().rev().take_while(|c| *c == ')').collect();
            out.push_str(&format!("{lead}1.0{tail},"));
        }
        out.pop();
        out
    };
    let mutated = format!("{}{}{}", &text[..open], snapped, &text[end..]);
    assert_ne!(mutated, text, "weights snapped");
    match import(&mutated) {
        Err(e) => eprintln!("PROBE weight-snap refusal (honest): {e}"),
        Ok(StepImport::Solid { body, .. }) => {
            let t3 = topo::validate_geometric(&body);
            panic!(
                "LAUNDERED: weight-snapped rational wall imported; t3 = {t3:?}, census {:?}",
                census(&body)
            );
        }
        Ok(other) => panic!("unexpected disposition: {other:?}"),
    }
}

/// V3/ARM-B boundary — review F1, **FLIPPED at the fix pass**: on a
/// RATIONAL wall nothing mints, tier-3 check 4 is kind-exempt, and
/// the conventional rung's coincidence gate is kind-exempt too, so as
/// reviewed NOTHING certified an ARC rim against its rational wall —
/// this probe originally executed exactly that (the mutated file
/// imported t1/t2-valid with the verbatim native t3 refusal,
/// indistinguishable from correct). The fix is the import-side
/// residual gate (`adopt::arc_rim_on_wall_boundary`): the wall's own
/// boundary column, sampled at the certification schedule, must lie
/// on the rim's circle within the ambient tolerance. The plant —
/// the z=0 cap's quarter arc (center ~(0,0,0), r = √2) replaced by a
/// DIFFERENT circle through the same two endpoints
/// (center (1−√24, 0, 0), r = 5) — must now REFUSE, typed, naming
/// the residual.
///
/// **The positive control is
/// `nurbs_import::arc_loft_natively_computes_its_rational_volume`**,
/// which builds the character-identical arc loft and pins that the
/// UNMUTATED file clears this same residual gate and imports (or, at
/// a tight enough ε, stops only at the quadrature budget, with the
/// verdict list pinned structurally). The discrimination this probe
/// exists for — the plant refuses `RimOffWallBoundary`, the true arc
/// does not — is therefore stated across the two rows, and the true
/// arc's rational quadrature is paid once instead of twice.
#[test]
fn probe_arm_b_rim_off_wall_arc() {
    let native = native_arc_loft_for_probe();
    let text = step_export::step_string(&native, &step_export::StepOptions::default())
        .expect("arc loft exports");
    // Find the CIRCLE whose placement's location is (0.0, 0.0, 0.0).
    let mut center_line = None;
    let mut circle_line = None;
    for line in text.lines() {
        if line.contains("= CIRCLE(") {
            let place_ref = line
                .split("#")
                .nth(2)
                .and_then(|s| s.split(&[',', ')'][..]).next())
                .expect("placement ref");
            let place_line = text
                .lines()
                .find(|l| l.starts_with(&format!("#{place_ref} = ")))
                .expect("placement line");
            let loc_ref = place_line
                .split("#")
                .nth(2)
                .and_then(|s| s.split(&[',', ')'][..]).next())
                .expect("location ref");
            let loc_line = text
                .lines()
                .find(|l| l.starts_with(&format!("#{loc_ref} = ")))
                .expect("location line");
            if loc_line.contains("(1.1102230246251565E-16, 0.0, 0.0)") {
                center_line = Some(loc_line.to_owned());
                circle_line = Some(line.to_owned());
                break;
            }
        }
    }
    if center_line.is_none() {
        for line in text.lines() {
            if line.contains("= CIRCLE(")
                || ["#26 ", "#27 ", "#28 ", "#67 ", "#68 ", "#69 "]
                    .iter()
                    .any(|p| line.starts_with(p))
            {
                eprintln!("DUMP: {line}");
            }
        }
    }
    let center_line = center_line.expect("z=0 cap arc center found");
    let circle_line = circle_line.expect("z=0 cap arc found");
    let new_center = center_line.replace(
        "(1.1102230246251565E-16, 0.0, 0.0)",
        "(-3.898979485566356, 0.0, 0.0)",
    );
    let new_circle = circle_line.replace("1.414213562373095", "5.0");
    assert_ne!(new_circle, circle_line, "radius token found and changed");
    let mutated = text
        .replace(&center_line, &new_center)
        .replace(&circle_line, &new_circle);
    match import(&mutated) {
        Err(step_import::StepImportError::RimOffWallBoundary { id, residual }) => {
            eprintln!("PROBE rim-off-wall arc refused typed: edge #{id}, residual {residual:e}");
            assert!(
                residual > 1e-3,
                "the r=5 plant misses the wall boundary by a macroscopic margin, got {residual:e}"
            );
        }
        Err(other) => {
            panic!("the plant must refuse AT THE RESIDUAL GATE (RimOffWallBoundary), got: {other}")
        }
        Ok(StepImport::Solid { body, .. }) => {
            let t1 = topo::validate(&body);
            let t2 = topo::validate_closed(&body);
            let t3 = topo::validate_geometric(&body).map(|_| "t3 GREEN");
            panic!(
                "LAUNDERED (the F1 hole is back): rim-off-wall arc imported as a solid; \
                 t1 = {t1:?}, t2 = {t2:?}, t3 = {t3:?}"
            );
        }
        Ok(other) => panic!("unexpected disposition: {other:?}"),
    }
}

/// V4 — review F5's measurement: the synthesized descriptions vs the
/// native builder's, payload by payload. Seam IsoCurves reproduce the
/// native payload verbatim (modulo arena keys); rim PlacedSegments
/// share the CLASS and certification surface but carry an equivalent
/// carrier-frame parameterization, not the native sketch-coordinate
/// one (the sketch plane never crosses the wire). The fix pass
/// corrected the prose to that class-level statement; this probe
/// keeps the measurement honest.
#[test]
fn probe_descriptions_native_vs_imported_bitwise() {
    let native = native_loft_prism();
    let imported = solid(&fixture("loft_prism", "step"), "loft_prism");
    let dump = |body: &topo::Body<f64>| -> Vec<String> {
        let mut rows: Vec<String> = body
            .edges()
            .filter_map(|(_, e)| match body.get_curve_geom(e.curve) {
                Some(topo::CurveGeom::Certified(c)) => {
                    // Key rows by the edge's endpoint coordinates so the
                    // native and imported bodies can be aligned.
                    let d = format!("{:?}", c.description());
                    Some(d)
                }
                _ => None,
            })
            .collect();
        rows.sort();
        rows
    };
    let n = dump(&native);
    let i = dump(&imported);
    let only_native: Vec<_> = n.iter().filter(|r| !i.contains(r)).collect();
    let only_imported: Vec<_> = i.iter().filter(|r| !n.contains(r)).collect();
    eprintln!(
        "PROBE description diff: native-only {} rows, imported-only {} rows",
        only_native.len(),
        only_imported.len()
    );
    for r in only_native.iter().take(6) {
        eprintln!("  native-only:   {r}");
    }
    for r in only_imported.iter().take(6) {
        eprintln!("  imported-only: {r}");
    }
    // Informational: the load-bearing claim is class-level; this probe
    // measures the payload-level claim in the report prose.
    assert_eq!(n.len(), i.len(), "same number of certified descriptions");
}

/// V4 — review F2, now a regression pin: every PlacedSegment
/// placement in an imported body must be a rigid frame (the
/// synthesizer's own doc: "honest perpendiculars"). A sub-unit
/// x-aligned DIRECTION (within eps_in of unit, kept verbatim) fed the
/// OLD `|x| < 1.0` seed pick an x-parallel candidate — y ∥ dir,
/// z = 0, det-0 claimed-rigid data in a certified body (executed).
/// The smallest-component seed fix makes this import with an honest
/// near-unit-determinant frame, which is what the assert now pins.
#[test]
fn probe_subunit_x_direction_rim_frame_rigidity() {
    let orig = fixture("loft_prism", "step");
    let text = orig.replace(
        "#19 = DIRECTION('', (1.0, 0.0, 0.0));",
        "#19 = DIRECTION('', (0.99999999999999989, 0.0, 0.0));",
    );
    assert_ne!(text, orig, "direction mutated");
    match import(&text) {
        Err(e) => eprintln!("PROBE subunit-dir refused (honest): {e}"),
        Ok(StepImport::Solid { body, .. }) => {
            let mut bad = Vec::new();
            for (ek, e) in body.edges() {
                if let Some(topo::CurveGeom::Certified(c)) = body.get_curve_geom(e.curve)
                    && let geom_brep::EdgeGeometry::MappedCurve(
                        geom_brep::MappedCurve::PlacedSegment { place, .. },
                    ) = c.description()
                {
                    let l = place.linear;
                    let det = l.determinant();
                    if (det.abs() - 1.0).abs() > 1e-9 {
                        bad.push((ek, det, format!("{l:?}")));
                    }
                }
            }
            assert!(
                bad.is_empty(),
                "DEGENERATE claimed-rigid frames in a certified import: {bad:?}"
            );
        }
        Ok(other) => panic!("unexpected disposition: {other:?}"),
    }
}

/// V4: flip a rim EDGE_CURVE's same_sense — must not silently produce
/// a different body.
#[test]
fn probe_rim_same_sense_flip_is_honest() {
    let orig = fixture("loft_prism", "step");
    let text = orig.replace(
        "#22 = EDGE_CURVE('', #9, #17, #21, .T.);",
        "#22 = EDGE_CURVE('', #9, #17, #21, .F.);",
    );
    assert_ne!(text, orig, "same_sense flipped");
    match import(&text) {
        Err(e) => eprintln!("PROBE same_sense flip refused (honest): {e}"),
        Ok(StepImport::Solid { body, .. }) => {
            // If accepted, it must be the SAME body (flag interpretive).
            let base = solid(&orig, "committed fixture");
            assert_eq!(census(&body), census(&base), "census must match");
            assert_eq!(topo::validate(&body), Ok(()), "t1");
            assert_eq!(topo::validate_closed(&body), Ok(()), "t2");
            assert_eq!(topo::validate_geometric(&body), Ok(()), "t3");
            eprintln!("PROBE same_sense flip: accepted and fully valid");
        }
        Ok(other) => panic!("unexpected disposition: {other:?}"),
    }
}

/// V5, RE-STATED at M7-6 per the #256 ruling. The old pin (token
/// streams aligned, exactly three `-0.0 → 0.0` flips) described the
/// pre-promotion world; since D7 stage-1 always-promote, loft_prism's
/// two exactly-planar walls (faces #104/#142, residual 0.0) import as
/// PLANES, and the first re-export states them analytically — a
/// STRUCTURAL divergence from the committed bytes that is exactly the
/// promotion and nothing else. The re-stated pin:
///
/// * the divergence is the promotion: 2 of the 4 committed
///   `B_SPLINE_SURFACE_WITH_KNOTS` walls re-export as `PLANE` records
///   (2 planes → 4, 4 splines → 2), and the promotion is REPORTED
///   (both records, census-identity, residual exactly 0.0);
/// * the promoted ONE-CYCLE fixed point: the first re-export
///   re-imports and re-exports byte-identically (censuses and
///   certified volumes across the cycle are `roundtrip.rs`'s
///   `fixed_point` row, green over the whole corpus; the volume
///   against the NATIVE body is `committed_corpus`'s, also green —
///   "volumes equal across the promotion", executed).
#[test]
fn probe_reexport_promotion_divergence() {
    let orig = fixture("loft_prism", "step");
    let StepImport::Solid {
        body,
        normalizations,
        ..
    } = import(&orig).expect("loft_prism imports")
    else {
        panic!("loft_prism must import as a solid");
    };
    let promos: Vec<_> = normalizations
        .iter()
        .filter_map(|n| match n.kind {
            step_import::NormalizationKind::SurfacePromotion { to, residual } => {
                assert_eq!(
                    n.file_census, n.kernel_census,
                    "promotion never re-tessellates"
                );
                Some((n.face, to, residual))
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        promos,
        vec![
            (104, step_import::PromotedKind::Plane, 0.0),
            (142, step_import::PromotedKind::Plane, 0.0),
        ],
        "the enumerated promotions: the two exactly-planar walls, residual 0.0"
    );
    let options = step_export::StepOptions {
        product_name: "loft_prism".to_owned(),
        ..step_export::StepOptions::default()
    };
    let out = step_export::step_string(&body, &options).expect("re-export");
    let count = |s: &str, pat: &str| s.matches(pat).count();
    assert_eq!(
        (
            count(&orig, "= PLANE("),
            count(&orig, "B_SPLINE_SURFACE_WITH_KNOTS(")
        ),
        (2, 4),
        "committed: 2 cap planes + 4 spline walls"
    );
    assert_eq!(
        (
            count(&out, "= PLANE("),
            count(&out, "B_SPLINE_SURFACE_WITH_KNOTS(")
        ),
        (4, 2),
        "re-export: the two promoted walls state their planes"
    );
    // The promoted one-cycle fixed point.
    let body2 = solid(&out, "first re-export");
    let out2 = step_export::step_string(&body2, &options).expect("second re-export");
    assert_eq!(out, out2, "fixed point from the first re-export on");
}

/// V2 determinism: two imports of the same file produce identical
/// IsoCurve payloads (adopted wall key and u arm).
#[test]
fn probe_iso_adoption_deterministic() {
    let orig = fixture("loft_prism", "step");
    let dump = |body: &topo::Body<f64>| -> Vec<String> {
        let mut rows: Vec<String> = body
            .edges()
            .filter_map(|(_, e)| match body.get_curve_geom(e.curve) {
                Some(topo::CurveGeom::Certified(c)) => match c.description() {
                    geom_brep::EdgeGeometry::IsoCurve { surface, u, v0, v1 } => {
                        Some(format!("{surface:?} u={u} v0={v0} v1={v1}"))
                    }
                    _ => None,
                },
                _ => None,
            })
            .collect();
        rows.sort();
        rows
    };
    let a = solid(&orig, "first");
    let b = solid(&orig, "second");
    assert_eq!(dump(&a), dump(&b), "adoption deterministic");
    eprintln!("PROBE iso payloads: {:?}", dump(&a));
}
