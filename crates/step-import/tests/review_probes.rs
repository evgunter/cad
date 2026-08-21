//! ADVERSARIAL REVIEW PROBES for M7-1 (PR #183). Review-branch only —
//! not part of the PR's two-target acceptance layout.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{census, fixture};
use geom_core::Tol;
use step_import::{ImportOptions, StepImport, StepImportError, import_step};

fn import_text(text: &str) -> Result<StepImport, StepImportError> {
    import_step(text, &ImportOptions::default())
}

fn solid(text: &str, what: &str) -> topo::Body<f64> {
    let options = if what.contains("kiss") {
        // The corner kiss is declared through the M9-2 import channel
        // (D7 step 4); undeclared it refuses at the shared 3′ gate.
        ImportOptions {
            declared_contacts: vec![step_import::ImportContact::VertexRest {
                at: [1.0, 1.0, 1.0],
            }],
            ..ImportOptions::default()
        }
    } else {
        ImportOptions::default()
    };
    match import_step(text, &options).unwrap_or_else(|e| panic!("{what}: {e}")) {
        StepImport::Solid { body, .. } => body,
        StepImport::Wireframe { .. } => panic!("{what}: wireframe"),
    }
}

fn export(body: &topo::Body<f64>, name: &str) -> String {
    let options = step_export::StepOptions {
        product_name: name.to_owned(),
        ..step_export::StepOptions::default()
    };
    step_export::step_string(body, &options, Tol::witness()).unwrap()
}

/// A1: a consistently perturbed radius must flow through import into
/// the re-export (proves the re-export derives from parsed values, not
/// laundered file text).
#[test]
fn a1_perturbed_value_flows_to_reexport() {
    let text = fixture("ball", "step")
        .replace(", 1.0);", ", 1.25);") // 2 sphere radii + 2 circle radii
        .replace(
            "#6 = CARTESIAN_POINT('', (0.0, 1.0, 0.0));",
            "#6 = CARTESIAN_POINT('', (0.0, 1.25, 0.0));",
        )
        .replace(
            "#8 = CARTESIAN_POINT('', (0.0, -1.0, 0.0));",
            "#8 = CARTESIAN_POINT('', (0.0, -1.25, 0.0));",
        );
    let body = solid(&text, "perturbed ball");
    let v = topo::mass_properties(&body, Tol::witness()).unwrap().volume;
    let expected = 4.0 / 3.0 * std::f64::consts::PI * 1.25f64.powi(3);
    assert!(
        (v - expected).abs() < 1e-9,
        "volume must reflect the parsed radius: {v} vs {expected}"
    );
    let out = export(&body, "ball");
    assert!(
        out.contains("1.25"),
        "re-export must carry the parsed value"
    );
    assert_ne!(
        out,
        fixture("ball", "step"),
        "must differ from committed text"
    );
}

/// A1/E5 teeth: a tiny (1e-7) inconsistency-free radius change must
/// push the volume outside the row-1 tolerance (the row would FAIL).
#[test]
fn a1_small_perturbation_breaks_row1_tolerance() {
    let text = fixture("ball", "step")
        .replace(", 1.0);", ", 1.0000001);")
        .replace(
            "#6 = CARTESIAN_POINT('', (0.0, 1.0, 0.0));",
            "#6 = CARTESIAN_POINT('', (0.0, 1.0000001, 0.0));",
        )
        .replace(
            "#8 = CARTESIAN_POINT('', (0.0, -1.0, 0.0));",
            "#8 = CARTESIAN_POINT('', (0.0, -1.0000001, 0.0));",
        );
    let body = solid(&text, "slightly perturbed ball");
    let props = topo::mass_properties(&body, Tol::witness()).unwrap();
    let expect = common::expect_sidecar("ball");
    let expected = expect.kernel_volume_mm3 * 1e-9;
    // Row-1 tolerance for ball (the tightened KERNEL_VOLUME_MM3 form —
    // roundtrip.rs derives it): pad + native pad + (2×faces + 2) = 6
    // ulps; no print-precision term, the literal is full precision.
    let tol = props.volume_pad
        + expect.kernel_volume_pad_mm3 * 1e-9
        + 6.0 * (expected.next_up() - expected);
    let dev = (props.volume - expected).abs();
    assert!(
        dev > tol,
        "a 1e-7 radius error must exceed the row tolerance: dev {dev} vs tol {tol} (pad {})",
        props.volume_pad
    );
}

/// A1: deleting one face from the shell must refuse (structure is
/// re-verified, not echoed).
#[test]
fn a1_deleted_face_refuses() {
    let text = fixture("cube", "step").replace("(#40, ", "(");
    let err = import_text(&text).expect_err("a shell missing a face must refuse");
    assert!(
        matches!(err, StepImportError::Topology { .. }),
        "expected a typed topology refusal, got: {err}"
    );
}

/// C3(a): a CIRCLE whose axis is rotated so the curve leaves both
/// adjacent surfaces (cut_cylinder's top rim, tilted with a
/// still-orthogonal frame) must produce a structured Adoption refusal.
/// (First attempt used ball's seam meridian — a tilted GREAT circle
/// stays on the sphere and the ladder honestly certified the
/// legally-different body; recorded in the review report.)
#[test]
fn c3a_rotated_circle_axis_refuses_adoption() {
    let text = fixture("cut_cylinder", "step")
        .replace(
            "#11 = DIRECTION('', (0.0, 0.0, 1.0));",
            "#11 = DIRECTION('', (0.6, 0.0, 0.8));",
        )
        .replace(
            "#12 = DIRECTION('', (1.0, 0.0, 0.0));",
            "#12 = DIRECTION('', (0.0, 1.0, 0.0));",
        );
    let err = import_text(&text).expect_err("a tilted rim circle must not adopt");
    match err {
        StepImportError::Adoption { id, attempts } => {
            assert_eq!(id, 15, "the refusal names the EDGE_CURVE");
            assert!(!attempts.is_empty(), "attempts carried as data");
            println!(
                "c3a refusals: {}",
                StepImportError::Adoption { id, attempts }
            );
        }
        other => panic!("expected Adoption, got: {other}"),
    }
}

/// C3(b): a CYLINDRICAL_SURFACE radius perturbed by 1e-3 against its
/// (unchanged) circular edges must refuse via the adoption ladder.
#[test]
fn c3b_cylinder_radius_mismatch_refuses_adoption() {
    let text = fixture("cut_cylinder", "step").replace(
        "#31 = CYLINDRICAL_SURFACE('', #30, 1.0);",
        "#31 = CYLINDRICAL_SURFACE('', #30, 1.001);",
    );
    let err = import_text(&text).expect_err("radius mismatch must not adopt");
    match err {
        StepImportError::Adoption { id, attempts } => {
            println!(
                "c3b refusals: {}",
                StepImportError::Adoption { id, attempts }
            );
        }
        other => panic!("expected Adoption, got: {other}"),
    }
}

/// C3(c): a face pointed at the WRONG surface (the opposite plane)
/// must refuse — its edges cannot certify against the wrong pair.
#[test]
fn c3c_wrong_surface_pair_refuses_adoption() {
    let text = fixture("cube", "step").replace(
        "#40 = ADVANCED_FACE('', (#39), #5, .T.);",
        "#40 = ADVANCED_FACE('', (#39), #45, .T.);",
    );
    let err = import_text(&text).expect_err("wrong surface pair must not adopt");
    match err {
        StepImportError::Adoption { id, attempts } => {
            println!(
                "c3c refusals: {}",
                StepImportError::Adoption { id, attempts }
            );
        }
        other => panic!("expected Adoption, got: {other}"),
    }
}

/// C3(d): a line moved to lie in ONE adjacent plane but off the
/// intersection locus (vertices unchanged) must refuse.
#[test]
fn c3d_curve_off_intersection_refuses_adoption() {
    let text = fixture("cube", "step").replace(
        "#10 = CARTESIAN_POINT('', (0.0, 1.0, 1.0));",
        "#10 = CARTESIAN_POINT('', (0.5, 1.0, 1.0));",
    );
    let err = import_text(&text).expect_err("off-locus carrier must not adopt");
    match err {
        StepImportError::Adoption { id, attempts } => {
            println!(
                "c3d refusals: {}",
                StepImportError::Adoption { id, attempts }
            );
        }
        other => panic!("expected Adoption, got: {other}"),
    }
}

/// D4: reversing the DATA-section line order (legal — references are
/// by id) must still assemble, and one adoption pass must still be a
/// fixed point of the writer.
#[test]
fn d4_reversed_data_lines_still_assemble() {
    let text = fixture("die", "step");
    let (head, rest) = text.split_once("DATA;\n").unwrap();
    let (entities, tail) = rest.split_once("ENDSEC;\nEND-ISO").unwrap();
    let mut lines: Vec<&str> = entities.lines().collect();
    lines.reverse();
    let permuted = format!("{head}DATA;\n{}\nENDSEC;\nEND-ISO{tail}", lines.join("\n"));
    let body = solid(&permuted, "reversed die");
    let reference = solid(&text, "die");
    assert_eq!(census(&body), census(&reference));
    let v1 = topo::mass_properties(&body, Tol::witness()).unwrap().volume;
    let v2 = topo::mass_properties(&reference, Tol::witness())
        .unwrap()
        .volume;
    assert_eq!(v1.to_bits(), v2.to_bits());
    let e1 = export(&body, "die");
    let body2 = solid(&e1, "reversed die reimport");
    let e2 = export(&body2, "die");
    assert_eq!(e1, e2, "fixed point must hold on a non-writer-ordered file");
    println!(
        "d4 reversed-lines first re-export {} the committed fixture",
        if e1 == text {
            "matches"
        } else {
            "diverges from"
        }
    );
}

/// D4: renumbering every entity id (reversing id order — BTreeMap
/// iteration order no longer the writer's emission order) must still
/// assemble the two-solid kiss assembly, and the fixed point must
/// still hold.
#[test]
fn d4_renumbered_kiss_assembly_still_assembles() {
    let text = fixture("kiss_assembly", "step");
    let max_id = 3000u64;
    let mut renumbered = text.clone();
    for k in (1..=max_id).rev() {
        let from = format!("#{k}");
        let to = format!("@{}", max_id + 1 - k);
        renumbered = renumbered.replace(&from, &to);
    }
    renumbered = renumbered.replace('@', "#");
    let body = solid(&renumbered, "renumbered kiss");
    let reference = solid(&text, "kiss");
    assert_eq!(census(&body), census(&reference));
    let v1 = topo::mass_properties(&body, Tol::witness()).unwrap().volume;
    let v2 = topo::mass_properties(&reference, Tol::witness())
        .unwrap()
        .volume;
    assert_eq!(v1.to_bits(), v2.to_bits());
    let e1 = export(&body, "kiss_assembly");
    let body2 = solid(&e1, "renumbered kiss reimport");
    let e2 = export(&body2, "kiss_assembly");
    assert_eq!(e1, e2, "fixed point must hold on a renumbered file");
    println!(
        "d4 renumbered kiss first re-export {} the committed fixture",
        if e1 == text {
            "matches"
        } else {
            "diverges from"
        }
    );
}

/// F6: flipping a face's same_sense .T. -> .F. must not be silently
/// normalized away: either a typed refusal, or a genuinely different
/// body (tier-3 invalid or different volume) whose re-export keeps .F.
#[test]
fn f6_face_sense_flip_is_not_healed() {
    let text = fixture("cube", "step").replace(
        "#40 = ADVANCED_FACE('', (#39), #5, .T.);",
        "#40 = ADVANCED_FACE('', (#39), #5, .F.);",
    );
    match import_text(&text) {
        Err(e) => println!("f6 .T.->.F.: typed refusal: {e}"),
        Ok(StepImport::Solid { body, .. }) => {
            let tier3 = topo::validate_geometric(&body, Tol::witness());
            let out = export(&body, "cube");
            let kept = out.contains("ADVANCED_FACE('', (#39), #5, .F.)")
                || out.matches(", .F.);").count() > 0;
            println!("f6 .T.->.F.: imported; tier3 = {tier3:?}; re-export keeps .F.: {kept}");
            assert!(
                tier3.is_err() || kept,
                "a flipped sense must be measurable or retained, never healed"
            );
            assert!(kept, "re-export must carry the flipped sense verbatim");
        }
        Ok(_) => panic!("wireframe?"),
    }
}

/// F6 mirror: un-reversing a genuinely reversed corpus face (washer
/// #33 is .F.) must likewise not be healed back.
#[test]
fn f6_reversed_face_unflip_is_not_healed() {
    let text = fixture("washer", "step").replace(
        "#33 = ADVANCED_FACE('', (#32), #5, .F.);",
        "#33 = ADVANCED_FACE('', (#32), #5, .T.);",
    );
    match import_text(&text) {
        Err(e) => {
            println!("f6 .F.->.T.: typed refusal: {e}");
            // PINNED (M6-6): the un-reversal is not healed — it is
            // refused pre-body with the orientation-inverted diagnosis
            // (the import rider's cylinder/cone wall cross-check).
            assert!(
                e.to_string().contains("ORIENTATION-INVERTED"),
                "the refusal must carry the orientation-inverted diagnosis: {e}"
            );
        }
        Ok(StepImport::Solid { body, .. }) => {
            let tier3 = topo::validate_geometric(&body, Tol::witness());
            let out = export(&body, "washer");
            let flipped_back = out.contains(", #5, .F.);");
            let v = topo::mass_properties(&body, Tol::witness()).unwrap().volume;
            let v0 =
                topo::mass_properties(&solid(&fixture("washer", "step"), "washer"), Tol::witness())
                    .unwrap()
                    .volume;
            // Finding data, not a hard assert: tier-3 detectability of a
            // curved-face sense flip is kernel scope, not import scope.
            println!(
                "f6 .F.->.T.: imported; tier3 = {tier3:?}; healed back: {flipped_back}; \
                 volume flipped {v} vs original {v0}"
            );
            assert!(!flipped_back, "the sense must not be silently restored");
            assert_ne!(
                out,
                fixture("washer", "step"),
                "the re-export must differ from the committed text (sense honored)"
            );
        }
        Ok(_) => panic!("wireframe?"),
    }
}

/// F6: an EDGE_CURVE same_sense .F. is a declared subset boundary —
/// must refuse typed, naming the edge.
#[test]
fn f6_edge_sense_flip_refuses_typed() {
    let text = fixture("cube", "step").replace(
        "#14 = EDGE_CURVE('', #7, #9, #13, .T.);",
        "#14 = EDGE_CURVE('', #7, #9, #13, .F.);",
    );
    let err = import_text(&text).expect_err("edge same_sense .F. is outside the subset");
    match err {
        StepImportError::Topology { id, .. } => assert_eq!(id, 14),
        other => panic!("expected Topology, got: {other}"),
    }
}

/// G7: the FULL truncation sweep (all 7318 prefix cuts of cube.step,
/// not the committed test's first 400): every strict prefix must
/// refuse, except a cut that leaves a semantically complete exchange
/// file (only the final newline may be dropped).
#[test]
fn g7_full_truncation_sweep() {
    let text = fixture("cube", "step");
    let mut ok_cuts = Vec::new();
    for cut in 0..text.len() {
        if !text.is_char_boundary(cut) {
            continue;
        }
        if import_text(&text[..cut]).is_ok() {
            ok_cuts.push(cut);
        }
    }
    println!("g7 cuts that import: {ok_cuts:?} (len {})", text.len());
    assert!(
        ok_cuts.iter().all(|&c| c + 1 == text.len()),
        "only the trailing-newline cut may leave a complete file: {ok_cuts:?}"
    );
}

/// G7, **flipped by M7-2 Leg C**: a non-apex `CONICAL_SURFACE`
/// (radius != 0) is no longer a syntactic refusal — it is the only
/// form Open CASCADE writes, and the apex is derived from it. What
/// this probe still owes is that the derivation cannot launder a
/// WRONG cone in: giving the apex-placed cone a base radius of 0.5
/// while leaving the placement alone moves its apex half a unit, so
/// the surface no longer explains the file's own edges and the
/// kernel's certification gates must say so.
///
/// The refusal therefore MOVED, from the record reader to the geometry
/// — which is the honest place for it, since the record is now well
/// formed and it is the geometry that is wrong.
#[test]
fn g7_nonapex_cone_derives_but_cannot_launder_a_wrong_apex() {
    let text = fixture("cone", "step").replace(
        "#5 = CONICAL_SURFACE('', #4, 0.0, 0.7853981633974483);",
        "#5 = CONICAL_SURFACE('', #4, 0.5, 0.7853981633974483);",
    );
    let err = import_text(&text).expect_err("a cone whose apex moved must not import");
    assert!(
        matches!(
            err,
            StepImportError::Adoption { .. }
                | StepImportError::Assembly { .. }
                | StepImportError::Topology { .. }
        ),
        "expected a geometric refusal from the kernel's own gates, got: {err}"
    );
}

/// G7, **flipped by M7-2 Leg B**: a reversed `FACE_OUTER_BOUND` (.F.)
/// is no longer outside the subset — it is honored as a loop reversal,
/// because FreeCAD writes 25 of them and 4 measured planar caps prove
/// the flag is independent of the owning face's `same_sense`.
///
/// Honoring it is not the same as ignoring it. Reversing ONE face's
/// bound in an otherwise consistent cube makes that face traverse its
/// four edges the other way, so each of them is now used twice in the
/// SAME direction — no longer a closed oriented 2-manifold. The
/// refusal stands, at the manifold precondition, naming the edge.
#[test]
fn g7_reversed_bound_is_honored_and_breaks_the_manifold() {
    let text = fixture("cube", "step").replace(
        "#39 = FACE_OUTER_BOUND('', #38, .T.);",
        "#39 = FACE_OUTER_BOUND('', #38, .F.);",
    );
    let err = import_text(&text).expect_err("one reversed bound un-closes the shell");
    match err {
        StepImportError::Topology { id, .. } => assert_ne!(
            id, 39,
            "the refusal must come from the manifold check on an edge, not from the \
             bound record (which is now legal)"
        ),
        other => panic!("expected Topology naming the doubly-traversed edge, got: {other}"),
    }
}

/// **G7, FLIPPED (M7-4 Leg C).** A non-unit `VECTOR` magnitude used
/// to refuse typed, naming the vector. The wild made that refusal
/// untenable — ST-Developer writes `10.` on every line — and it turned
/// out to cost nothing: no trim parameter crosses the wire, so the
/// carrier interval is re-derived from the vertices and the magnitude
/// has nowhere to land. The probe now pins that the rescaled file is
/// the SAME solid, and that a magnitude which is not a positive scale
/// still refuses, naming the vector.
#[test]
fn g7_nonunit_vector_rescales_without_moving_the_solid() {
    let plain = import_text(&fixture("cube", "step")).expect("the cube imports");
    let text = fixture("cube", "step")
        .replace("#12 = VECTOR('', #11, 1.0);", "#12 = VECTOR('', #11, 2.0);");
    let rescaled = import_text(&text).expect("any positive magnitude imports");
    let (StepImport::Solid { body: a, .. }, StepImport::Solid { body: b, .. }) =
        (&plain, &rescaled)
    else {
        panic!("both are solids");
    };
    assert_eq!(
        topo::mass_properties(a, Tol::witness()).unwrap().volume,
        topo::mass_properties(b, Tol::witness()).unwrap().volume,
        "the line's parameter scale is not the line"
    );
    for bad in ["0.0", "-2.0"] {
        let text = fixture("cube", "step").replace(
            "#12 = VECTOR('', #11, 1.0);",
            &format!("#12 = VECTOR('', #11, {bad});"),
        );
        match import_text(&text).expect_err("a non-positive magnitude describes no line") {
            StepImportError::MalformedRecord { id, .. } => assert_eq!(id, 12),
            other => panic!("expected MalformedRecord naming the VECTOR, got: {other}"),
        }
    }
}

/// **G7/Leg B attack, FLIPPED (M7-4 Leg B).** A length unit declared
/// via `CONVERSION_BASED_UNIT` (no `SI_UNIT` record at all — an inch
/// file) used to refuse, and the attack it was defending against is
/// real: if such a file imports at metre scale, the part is silently
/// 1/25.4 of its stated size. What retires the refusal is not
/// tolerance for the entity but a resolved factor — so the attack is
/// re-run here in its sharpest form: a conversion-based context whose
/// factor entity is MISSING (the original probe's dangling `#9990`)
/// must still refuse, because there is no number to scale by.
#[test]
fn g7_conversion_based_unit_without_a_factor_must_refuse() {
    let text = fixture("cube", "step").replace(
        "( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT($, .METRE.) )",
        "( CONVERSION_BASED_UNIT('INCH', #9990) LENGTH_UNIT() NAMED_UNIT(#9991) )",
    );
    assert!(
        text.contains("CONVERSION_BASED_UNIT"),
        "the unit substitution must have fired"
    );
    let err = import_text(&text).expect_err("a factorless conversion unit must refuse typed");
    assert!(
        matches!(err, StepImportError::DanglingReference { to: 9990, .. }),
        "expected the missing factor to be named, got: {err}"
    );
}

/// E5: the closed forms behind the sidecar volume literals.
#[test]
fn e5_closed_forms() {
    let pi = std::f64::consts::PI;
    // filleted_die: Minkowski rounded cube, a = 1 - 2r, r = 0.12.
    let filleted = (0.854848 + 0.035136 * pi) * 1e9;
    println!("e5 filleted_die closed form: {filleted:.4} mm³ (sidecar 965231000)");
    assert!((filleted - 965231000.0).abs() < 500.0 + 0.5);
    assert!(
        (filleted - 965230999.4765).abs() < 0.001,
        "reported exact form"
    );
    // die_pips: 1 - 21·π·h²(3r−h)/3, r = 0.09, h = 0.05.
    let (r, h) = (0.09f64, 0.05f64);
    let pips = (1.0 - 21.0 * pi * h * h * (3.0 * r - h) / 3.0) * 1e9;
    println!("e5 die_pips closed form: {pips:.7} mm³ (sidecar 987904868.2836792)");
    assert!((pips - 987904868.2836792).abs() < 1e-4);
    // die_pips ulp distance of the imported volume from the sidecar.
    let body = solid(&fixture("die_pips", "step"), "die_pips");
    let v = topo::mass_properties(&body, Tol::witness()).unwrap().volume;
    let expected: f64 = 987904868.2836792e-9;
    let ulp = expected.next_up() - expected;
    println!(
        "e5 die_pips imported volume is {:.2} ulps from the sidecar (pad {})",
        (v - expected).abs() / ulp,
        topo::mass_properties(&body, Tol::witness())
            .unwrap()
            .volume_pad
    );
}

/// H8: ε_in must not be consumed by certification — an extreme
/// override in either direction changes nothing about what is built.
#[test]
fn h8_eps_in_not_consumed() {
    let text = fixture("cube", "step");
    for eps in [1e-30, 1.0] {
        let import = import_step(
            &text,
            &ImportOptions {
                eps_in: Some(eps),
                ..ImportOptions::default()
            },
        )
        .unwrap_or_else(|e| panic!("eps_in {eps} must not affect certification: {e}"));
        let StepImport::Solid { body, eps_in, .. } = import else {
            panic!("wireframe?")
        };
        assert_eq!(eps_in, eps);
        let v = topo::mass_properties(&body, Tol::witness()).unwrap().volume;
        assert_eq!(
            v.to_bits(),
            1.0f64.to_bits(),
            "identical body under eps_in {eps}"
        );
    }
}
