//! **M7-4 acceptance: the wild corpus.** STEP files nobody on this
//! project authored — real translator output from four license-
//! verified veins, kept verbatim under `tests/fixtures/wild/` with a
//! provenance comment each (see the crate's `NOTICE`).
//!
//! What makes these rows different from the M7-1 (own-corpus) and
//! M7-2 (FreeCAD) suites is the absence of a ground truth we control.
//! There is no generator whose closed form we can differentiate
//! against, and no writer whose bits we can compare to: a wild file's
//! only independent description is what an independent implementation
//! makes of it. So every census and volume here is stated against a
//! **FreeCAD 1.1.2 oracle run on the unmodified upstream file**,
//! recorded in each fixture's `.expect` sidecar with the run cited,
//! and the comparison is honest about what it is — agreement between
//! two readers, not a proof.
//!
//! Where the two disagree by construction, the sidecar says so: Open
//! CASCADE splits a periodic carrier at its seam on import and counts
//! the seam edge and its vertices, and the kernel does not (D7 —
//! adopt what the file states, do not heal it). Those fixtures carry
//! `KERNEL_EDGES` / `KERNEL_VERTICES` lines beside the oracle's.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use std::path::PathBuf;

use common::census;
use step_import::{ImportOptions, StepImport, StepImportError, import_step};

/// The imports-class corpus: files that import to a first-class,
/// tier-valid body, each with an oracle-derived `.expect` sidecar.
const WILD_IMPORTS: [&str; 7] = [
    "adafruit/328_2500mAh_battery.step",
    "adafruit/1982_MPR121.step",
    "adafruit/805_slide_switch.step",
    "adafruit/931_OLED_128x32_I2C.step",
    "adafruit/64_Halfsize_Breadboard.step",
    "nist/nist_ftc_09_asme1_rd.stp",
    "stepcode/sg1-c5-214.stp",
];

/// The refusal corpus: files whose typed refusal is itself the
/// contract being pinned. Each entry is the fixture and the substring
/// its message must carry — the class, not the prose.
const WILD_REFUSALS: [(&str, &str); 6] = [
    // A NURBS face: the named M7 frontier, whose import follows its
    // export.
    ("stepcode/dm1-id-214.stp", "B_SPLINE_SURFACE"),
    // A spline-carried edge between analytic surfaces: the file's
    // geometry is inside the subset entity by entity, and the D7
    // ladder still cannot certify any intensional description for the
    // edge. The refusal is the ladder's own, with every candidate and
    // its residual.
    (
        "stepcode/TAIL_TURBINE.stp",
        "no intensional description certifies",
    ),
    // A `\X2\` string control directive (Japanese annotation text).
    ("stepcode/io1-cm-214.stp", r"\X2\"),
    // Open CASCADE's `SURFACE_CURVE` edge geometry: a curve stated
    // three times over (3-D plus two pcurves), which the subset does
    // not read.
    ("occ-oss/b123d_nema17_bracket.step", "SURFACE_CURVE"),
    // Open CASCADE's SEAMLESS periodic faces — a band stated as its
    // two rim circles with no seam generator between them. Both files
    // below are otherwise fully in the subset; see
    // `the_periodic_band_refusal_is_a_named_kernel_gap`.
    ("nist/nist_ftc_11_asme1_rb.stp", "curved ADVANCED_FACE"),
    (
        "occ-oss/cq_red_cube_blue_cylinder.step",
        "curved ADVANCED_FACE",
    ),
];

/// A wild fixture's text (bytes as committed — CRLF line endings and
/// column-72 string folds included, which is the point).
fn wild(name: &str) -> String {
    let path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "tests",
        "fixtures",
        "wild",
        name,
    ]
    .iter()
    .collect();
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading {path:?}: {e}"))
}

/// A fixture's oracle sidecar, as `KEY=value` lines.
struct Oracle {
    census: (usize, usize, usize, usize, usize),
    volume_mm3: f64,
}

fn oracle(name: &str) -> Oracle {
    let text = wild(&format!("{name}.expect"));
    let get = |key: &str| -> String {
        text.lines()
            .find_map(|l| l.strip_prefix(&format!("{key}=")).map(str::to_owned))
            .unwrap_or_else(|| panic!("{name}.expect: no {key} line"))
    };
    let num = |key: &str| get(key).parse::<usize>().unwrap();
    // A `KERNEL_*` line, where present, is what a faithful import must
    // produce; the oracle's own count is what OCC reports after
    // splitting periodic carriers at their seams.
    let kernel = |key: &str, fallback: &str| {
        text.lines()
            .find_map(|l| l.strip_prefix(&format!("{key}=")))
            .map_or_else(|| num(fallback), |v| v.parse().unwrap())
    };
    Oracle {
        census: (
            num("ORACLE_SOLIDS"),
            num("ORACLE_SHELLS"),
            num("ORACLE_FACES"),
            kernel("KERNEL_EDGES", "ORACLE_EDGES"),
            kernel("KERNEL_VERTICES", "ORACLE_VERTICES"),
        ),
        volume_mm3: get("ORACLE_VOLUME_MM3").parse().unwrap(),
    }
}

fn solid(name: &str) -> (topo::Body<f64>, f64) {
    match import_step(&wild(name), &ImportOptions::default()) {
        Ok(StepImport::Solid { body, eps_in, .. }) => (body, eps_in),
        Ok(StepImport::Wireframe { .. }) => panic!("{name}: a solid was expected"),
        Err(e) => panic!("{name}: {e}"),
    }
}

// ---- The scale window --------------------------------------------

/// The ambient-ε window in which this corpus can CERTIFY, measured
/// end to end on the committed files.
///
/// The wild corpus is bounded on both sides, and each bound is a fact
/// about the files rather than about the kernel:
///
/// | ambient ε | outcome over the 7 imports-class fixtures |
/// |---|---|
/// | 1e-12 | 6 import; `nist_ftc_09` refuses at the D7 adoption ladder |
/// | 1e-11 | 6 import; the same one refuses |
/// | 1e-10 … 1e-8 (default 1e-9) | all 7 import, all three tiers green |
/// | 1e-7 | 6 import; `nist_ftc_09` refuses |
/// | 1e-6 | 4 import; the finer-featured ones refuse at the pcurve mint gate |
///
/// **The floor** is the wild's own contribution to this story, and the
/// FreeCAD corpus has nothing like it: a NIST inch translator prints
/// ~12 significant digits, so a coordinate carries ~1e-13 m of print
/// truncation on a 100 mm part, and at ε = 1e-12 that truncation is
/// larger than the band the adoption gates decide in. The file is not
/// wrong and the kernel is not wrong; the file simply does not state
/// itself to that precision, and the gate says so by name instead of
/// certifying a carrier it cannot.
///
/// **The ceiling** is the same millimetre-scale story the FreeCAD
/// corpus tells: above it the pcurve mint gate refuses, typed.
///
/// Outside the window the certifying rows skip LOUDLY and assert
/// [`assert_sub_tolerance_obligation`] instead — never nothing.
const WILD_EPS_FLOOR: f64 = 1e-10;
const WILD_EPS_CEILING: f64 = 1e-8;

/// Whether the ambient ε is inside the window this corpus certifies
/// in; prints a loud skip naming the numbers when it is not, and
/// asserts the obligation that holds at every ε in its place.
fn wild_scale_gate(row: &str) -> bool {
    let eps = geom_core::Tolerance::get().eps;
    if (WILD_EPS_FLOOR..=WILD_EPS_CEILING).contains(&eps) {
        return true;
    }
    println!(
        "{row}: outside the wild corpus's certifying window — ambient ε {eps:e} m is not in          [{WILD_EPS_FLOOR:e}, {WILD_EPS_CEILING:e}]. Asserting the every-ε obligation          instead of this row's certifying one."
    );
    assert_sub_tolerance_obligation(row);
    false
}

/// The obligation that holds at ANY ε: every committed wild file's
/// outcome is TYPED — imported with tiers 1 and 2 green and tier 3
/// either green or declining in band, or refused — and never a body
/// the kernel then calls geometrically false.
///
/// A row that returns early having asserted nothing is a green tick
/// for work not done. Outside the window this is what the wild corpus
/// still claims, and it is the claim that matters most there: a
/// foreign file the kernel cannot certify at the ambient tolerance is
/// REFUSED, not handed out wrong.
fn assert_sub_tolerance_obligation(row: &str) {
    let eps = geom_core::Tolerance::get().eps;
    let mut certified = 0;
    for name in WILD_IMPORTS
        .iter()
        .chain(WILD_REFUSALS.iter().map(|(n, _)| n))
    {
        match import_step(&wild(name), &ImportOptions::default()) {
            Ok(StepImport::Solid { body, .. }) => {
                assert_eq!(
                    topo::validate(&body),
                    Ok(()),
                    "{row}/{name}: tier 1 at ε {eps:e}"
                );
                assert_eq!(
                    topo::validate_closed(&body),
                    Ok(()),
                    "{row}/{name}: tier 2 at ε {eps:e}"
                );
                match topo::validate_geometric(&body) {
                    Ok(()) => certified += 1,
                    Err(errs) => {
                        assert!(
                            errs.iter().all(|e| matches!(
                                e,
                                topo::ValidationError::VolumeUncomputable { .. }
                                    | topo::ValidationError::PlanarFaceEscalated { .. }
                                    | topo::ValidationError::PlanarBoundaryEscalated { .. }
                                    | topo::ValidationError::CensusEscalated { .. }
                            )),
                            "{row}/{name}: tier 3 at ε {eps:e} reports a definite geometric \
                             falsehood, not an escalation — a foreign body like this must \
                             never have been imported: {errs:?}"
                        );
                        println!(
                            "{row}/{name}: tier 3 declines at ε {eps:e} (in-band escalation, \
                             not a wrong answer): {errs:?}"
                        );
                    }
                }
            }
            Ok(StepImport::Wireframe { .. }) => panic!("{row}/{name}: not a wireframe"),
            // A typed refusal is the other half of the obligation, and
            // outside the window it is the expected half.
            Err(e) => println!("{row}/{name}: refuses typed at ε {eps:e}: {e}"),
        }
    }
    println!("{row}: {certified} of 13 wild fixtures still certify at ε {eps:e}");
}

// ---- Row 1: the wild imports -------------------------------------

/// **Row 1.** Every imports-class wild fixture imports at default ε,
/// lands tier-1/2/3 valid, matches the oracle's census (with the
/// stated seam divergence), and agrees with the oracle's volume.
///
/// The volume tolerance is `1e-11` RELATIVE and is a claim about
/// arithmetic, not about modeling: the two readers sum the same
/// per-face divergence contributions in different orders over up to
/// 158 faces, so they may differ in the last few bits and must not
/// differ anywhere else. The measured spread over this corpus is
/// ~1e-16 to ~1e-13 relative; nothing here is near the budget, and a
/// fixture that moved to it would be reporting a real disagreement.
#[test]
fn wild_files_import_and_agree_with_the_oracle() {
    if !wild_scale_gate("wild_files_import_and_agree_with_the_oracle") {
        return;
    }
    for name in WILD_IMPORTS {
        let (body, eps_in) = solid(name);
        let e = oracle(name);
        assert_eq!(census(&body), e.census, "{name}: census");
        assert!(eps_in.is_finite() && eps_in > 0.0, "{name}: ε_in {eps_in}");

        assert_eq!(topo::validate(&body), Ok(()), "{name}: tier 1");
        assert_eq!(topo::validate_closed(&body), Ok(()), "{name}: tier 2");
        assert_eq!(topo::validate_geometric(&body), Ok(()), "{name}: tier 3");

        let props = topo::mass_properties(&body).unwrap_or_else(|e| panic!("{name}: {e}"));
        let volume_mm3 = props.volume * 1e9;
        let tolerance = 1e-11 * e.volume_mm3.abs() + props.volume_pad * 1e9;
        assert!(
            (volume_mm3 - e.volume_mm3).abs() <= tolerance,
            "{name}: volume {volume_mm3} mm3 vs the oracle's {} mm3 (tolerance {tolerance})",
            e.volume_mm3
        );
    }
}

/// **Row 1's dialect pin.** The corpus is not a set of files that
/// happen to work: each of the measured dialect gaps is present in
/// something committed here, so a regression in one of the legs
/// cannot pass this suite by importing a corpus that stopped
/// exercising it.
#[test]
fn the_committed_corpus_still_carries_the_dialects_it_was_chosen_for() {
    let all: Vec<String> = WILD_IMPORTS
        .iter()
        .chain(WILD_REFUSALS.iter().map(|(n, _)| n))
        .map(|n| wild(n))
        .collect();
    let any = |pred: &dyn Fn(&str) -> bool| all.iter().any(|t| pred(t));

    // Leg A: a string folded across a raw newline, and CRLF endings.
    assert!(
        any(&|t| t.contains("2500mAh batt\nery")),
        "an ST-Developer column-72 fold inside a string literal"
    );
    assert!(any(&|t| t.contains("\r\n")), "CRLF line endings");
    assert!(
        any(&|t| t.contains("/* name */")),
        "a comment inside an entity record"
    );
    // Leg B: conversion-based units, and a unit cluster the geometry's
    // context never references.
    assert!(any(&|t| t.contains("CONVERSION_BASED_UNIT")), "Leg B");
    assert!(
        any(&|t| t.contains("MASS_UNIT")),
        "an unreferenced unit kind"
    );
    assert!(
        any(&|t| t.contains("PARAMETRIC_REPRESENTATION_CONTEXT")),
        "an OCC 2D-SPACE parametric context"
    );
    // Leg C: a VECTOR magnitude that is not 1, and non-unit ratios.
    assert!(any(&|t| t.contains("VECTOR('',#131,10.)")), "Leg C");
    // Leg D: an assembly transform traversed.
    assert!(any(&|t| t.contains("ITEM_DEFINED_TRANSFORMATION")), "Leg D");
    // Leg E: an EDGE_CURVE stated against its carrier.
    assert!(
        any(&|t| t.contains(",.F.) ;") || t.contains(",.F.);")),
        "Leg E"
    );
    // And the schema-default placement fields.
    assert!(
        any(&|t| t.contains(",$) ;") || t.contains(",$);")),
        "a defaulted frame"
    );
}

// ---- Row 2: the cross-dialect fixed point --------------------------

/// **Row 2.** A wild body written out by this project's own writer and
/// read back is the SAME body — census and volume to the bit — and
/// the second export is byte-identical to the first.
///
/// This is the sharpest statement the wild corpus supports. The first
/// import is an interpretation of a foreign dialect; everything after
/// it is this project talking to itself, so any difference is ours.
#[test]
fn wild_bodies_are_a_fixed_point_of_our_own_dialect() {
    if !wild_scale_gate("wild_bodies_are_a_fixed_point_of_our_own_dialect") {
        return;
    }
    for name in WILD_IMPORTS {
        let (body, _) = solid(name);
        let options = step_export::StepOptions {
            product_name: name.to_owned(),
            ..step_export::StepOptions::default()
        };
        let first =
            step_export::step_string(&body, &options).unwrap_or_else(|e| panic!("{name}: {e}"));
        let Ok(StepImport::Solid { body: again, .. }) =
            import_step(&first, &ImportOptions::default())
        else {
            panic!("{name}: the re-import must be a solid");
        };
        assert_eq!(
            census(&body),
            census(&again),
            "{name}: census across the wire"
        );
        assert_eq!(
            topo::mass_properties(&body).unwrap().volume,
            topo::mass_properties(&again).unwrap().volume,
            "{name}: volume across the wire, to the bit"
        );
        let second = step_export::step_string(&again, &options).unwrap();
        assert_eq!(
            first, second,
            "{name}: the second export must be byte-identical"
        );
    }
}

// ---- Row 3: the refusals -------------------------------------------

/// **Row 3.** Every refusal-class fixture refuses TYPED, and its
/// message carries the class it was committed for. A refusal that
/// drifted to a different class would be a silent change in what this
/// importer claims to understand.
#[test]
fn wild_refusals_are_typed_and_name_their_class() {
    for (name, class) in WILD_REFUSALS {
        let err = import_step(&wild(name), &ImportOptions::default())
            .err()
            .unwrap_or_else(|| panic!("{name}: this fixture must refuse"));
        let message = err.to_string();
        assert!(
            message.contains(class),
            "{name}: refusal must name {class:?}, got: {message}"
        );
        // Typed and entity-named: every variant here points at
        // something in the file a reader can go and look at.
        let names_something = match &err {
            StepImportError::Syntax { line, .. } => *line > 0,
            StepImportError::UnsupportedEntity { id, .. }
            | StepImportError::Topology { id, .. }
            | StepImportError::Adoption { id, .. }
            | StepImportError::Structure { id, .. }
            | StepImportError::UnsupportedUnit { id, .. } => *id > 0,
            other => panic!("{name}: unexpected refusal kind: {other:?}"),
        };
        assert!(names_something, "{name}: the refusal must name an entity");
    }
}

/// **The named kernel gap behind two of those refusals.** Open
/// CASCADE never splits a periodic face: a cylinder's lateral band
/// arrives as its two rim circles with no seam generator between
/// them, and the kernel's face model has one outer loop and rings.
/// Adopting the second circle as a *ring* type-checks and produces a
/// body that passes tiers 1 and 2 — and whose volume no construction
/// in `topo` can compute (`RingOnCurvedFace`), which makes tier 3
/// refuse with it.
///
/// So the import refuses instead, at the face, and this row pins that
/// the reason is the gap and not a dialect misread: both fixtures'
/// offending faces are analytic surfaces bounded by ordinary circles,
/// every entity in them inside the subset.
#[test]
fn the_periodic_band_refusal_is_a_named_kernel_gap() {
    for name in [
        "nist/nist_ftc_11_asme1_rb.stp",
        "occ-oss/cq_red_cube_blue_cylinder.step",
    ] {
        let text = wild(name);
        assert!(
            !text.contains("B_SPLINE") && !text.contains("SURFACE_CURVE"),
            "{name}: the band refusal must not be standing in for a subset gap"
        );
        let StepImportError::Topology { what, .. } =
            import_step(&text, &ImportOptions::default()).unwrap_err()
        else {
            panic!("{name}: the band refuses at the face");
        };
        assert!(
            what.contains("seamless periodic band") && what.contains("tier-3"),
            "{name}: the refusal must say which gap it is: {what}"
        );
    }
}

// ---- Row 4: the no-panic sweep -------------------------------------

/// **Row 4 — the wild contract, pinned forever.** Every committed
/// wild fixture goes through `import_step` under `catch_unwind` and
/// must come back with a *result*: imported or refused, never a
/// panic, never a hang.
///
/// This is the row the corpus exists for. The triage that chose these
/// files measured 0 imports and 28 typed refusals across 28 foreign
/// files with zero panics; that outcome is the fail-loud contract
/// meeting data nobody here wrote, and it is worth an assertion that
/// can never be quietly dropped as the subset widens.
#[test]
fn no_wild_file_panics() {
    let names: Vec<&str> = WILD_IMPORTS
        .iter()
        .chain(WILD_REFUSALS.iter().map(|(n, _)| n))
        .copied()
        .collect();
    assert_eq!(names.len(), 13, "the whole committed wild corpus");
    for name in names {
        let text = wild(name);
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            import_step(&text, &ImportOptions::default()).map(|i| i.eps_in())
        }));
        assert!(
            outcome.is_ok(),
            "{name}: import_step panicked — the wild contract is a RESULT, always"
        );
    }
}

// ---- Row 6: ε_in through a conversion factor ------------------------

/// **Row 6.** A file whose lengths are stated in a conversion-based
/// unit carries its declared uncertainty in that unit too, and ε_in
/// must arrive in kernel metres having passed through the same
/// factor. `nist_ftc_09` states its length unit as an `INCH` that the
/// file itself defines as `2.54E1` of a `.MILLI. .METRE.`, and its
/// `distance_accuracy_value` as `1.331353158630E-3` of that inch. The
/// asserted ε_in is the product, exactly: a reader that took the
/// uncertainty at face value would be spending a budget 39× too
/// small, and one that "knew" an inch without reading the record
/// would be right here and wrong on the next file.
#[test]
fn eps_in_scales_through_the_conversion_factor_and_the_override_wins() {
    if !wild_scale_gate("eps_in_scales_through_the_conversion_factor_and_the_override_wins") {
        return;
    }
    let name = "nist/nist_ftc_09_asme1_rd.stp";
    let text = wild(name);
    assert!(
        text.contains("(CONVERSION_BASED_UNIT('INCH',#5777)LENGTH_UNIT()NAMED_UNIT(*))"),
        "the fixture's length unit is a conversion unit"
    );
    assert!(
        text.contains("#5777=LENGTH_MEASURE_WITH_UNIT(LENGTH_MEASURE(2.54E1),#5776)")
            && text.contains("#5776=(LENGTH_UNIT()NAMED_UNIT(*)SI_UNIT(.MILLI.,.METRE.))"),
        "whose factor the file states over its own millimetre"
    );
    assert!(
        text.contains("LENGTH_MEASURE(1.331353158630E-3),#5778"),
        "and whose uncertainty is stated in that inch"
    );
    let (_, eps_in) = solid(name);
    // 1.331353158630e-3 inch x 2.54e1 mm/inch x 1e-3 m/mm, in the
    // order the resolver multiplies (the factor folds into the unit
    // first, then scales the measure).
    let expected = 1.331_353_158_630e-3 * (2.54e1 * 1e-3);
    assert_eq!(eps_in, expected, "ε_in is a length like any other");
    assert!(
        (eps_in - 3.381_637e-5).abs() < 1e-11,
        "which is ~34 µm, the file's intent: {eps_in}"
    );

    let overridden = import_step(
        &text,
        &ImportOptions {
            eps_in: Some(2.5e-7),
        },
    )
    .expect("imports under an override");
    assert_eq!(overridden.eps_in(), 2.5e-7, "the per-call override wins");
}
