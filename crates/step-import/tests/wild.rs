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
//! adopt what the file states, do not heal it); conversely the
//! kernel's M7-5 band re-mint seams a SEAMLESS periodic band at the
//! surface's own u_ref azimuth (a reported `StructureNormalization`,
//! never silent), which can count MORE than OCC's healing where OCC
//! re-charts a seam onto an existing rim vertex instead. Those
//! fixtures carry `KERNEL_EDGES` / `KERNEL_VERTICES` lines beside
//! the oracle's.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use std::path::PathBuf;

use common::census;
use step_import::{ImportOptions, StepImport, StepImportError, import_step};

/// The imports-class corpus: files that import to a first-class,
/// tier-valid body, each with an oracle-derived `.expect` sidecar.
const WILD_IMPORTS: [&str; 9] = [
    "adafruit/328_2500mAh_battery.step",
    "adafruit/1982_MPR121.step",
    "adafruit/805_slide_switch.step",
    "adafruit/931_OLED_128x32_I2C.step",
    "adafruit/64_Halfsize_Breadboard.step",
    "nist/nist_ftc_09_asme1_rd.stp",
    "stepcode/sg1-c5-214.stp",
    // The two band-class fixtures, refusal-class until M7-5's seam
    // re-mint (`the_band_re_mint_reports_its_normalizations`).
    "nist/nist_ftc_11_asme1_rb.stp",
    "occ-oss/cq_red_cube_blue_cylinder.step",
];

/// The refusal corpus: files whose typed refusal is itself the
/// contract being pinned. Each entry is the fixture and the substring
/// its message must carry — the class, not the prose.
const WILD_REFUSALS: [(&str, &str); 4] = [
    // **Two refusals retired under this row, and it is still a
    // refusal.** The M7-4 Leg D instancing gate went in M8 (all seven
    // occurrences materialize under their own frames); the D7 ladder's
    // edge #685 went in #327 — its carrier is a rational quadratic
    // that IS a circle (deg 2, 7 points, weights `1, ½, …`, the 3×120°
    // form, r = 5 mm), stage-1 curve recognition certifies it against
    // an EXACT ring-composite bound and promotes it to `Curve3::Circle`,
    // and the existing arc-rim rungs (`arc_rim_on_wall_boundary` +
    // `RevolvedPoint` at the full period) carry it. Every edge of
    // every instance now adopts and every pcurve mints and certifies.
    //
    // What refuses now is the SHARED AT-REST GATE, on the file's
    // rational cylinder walls: `VolumeUncomputable` /
    // `QuadratureBudget` — the exact-B-rep volume enclosure stalls at
    // a mean boundary displacement of ~2.7·10⁻⁴ m against a 1.024·10⁻⁶
    // m target. That is the **banked rational-patch-flux lane** this
    // crate's own docs name ("Arc-bearing profiles export and read,
    // but their rational walls have no volume quadrature yet"), it is
    // the same lane a NATIVELY built rational-walled loft refuses on,
    // and import refuses it for exactly the reason the crate promises
    // to: an imported body is held to the same tiers, by the same
    // function, as a native one. Retiring THAT is its own unit.
    //
    // **The fragment carries the stalled quadrature by name**, not the
    // gate's preamble: the preamble alone would also match a tier-1/2
    // structural verdict, which would be a regression rather than the
    // banked lane. Still the class and not the prose — no widths, no
    // face key.
    //
    // **This fragment is checked elsewhere.** dm1 stays in the table —
    // the obligation sweep and the dialect pin read the whole corpus —
    // but `wild_refusals_are_typed_and_name_their_class` skips it (see
    // that row's `continue`): importing dm1 costs ~30× the other three
    // refusal fixtures together, and the same fragment is already
    // asserted by `tier_gate.rs`'s `RATIONAL_FLUX_STALL` at three ε_in
    // values per run, with the coarse band's `#389` cell beside it, and
    // structurally by `r1_dm1_probe`.
    (
        "stepcode/dm1-id-214.stp",
        "the certified quadrature enclosure stalled at",
    ),
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
    // Open CASCADE's SEAMLESS periodic bands (`nist_ftc_11_asme1_rb`,
    // `cq_red_cube_blue_cylinder`) were refusal-class here until the
    // M7-5 seam re-mint flipped both to `WILD_IMPORTS`.
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
/// about the files rather than about the kernel (re-measured
/// 2026-08-07 at the M7-5 band flip, 9 imports-class fixtures):
///
/// | ambient ε | outcome over the 9 imports-class fixtures |
/// |---|---|
/// | 1e-12 | 8 certify; `nist_ftc_09` refuses at the D7 adoption ladder |
/// | 1e-11 | 8 certify; the same one refuses |
/// | 1e-10 | 8 certify; `nist_ftc_09` refuses at the pcurve loop unwrap |
/// | 1e-9 (default) … 1e-8 | all 9 import, all three tiers green |
/// | 1e-7 | 9 certify (outside the pinned window; see below) |
/// | 1e-6 | 9 certify (outside the pinned window; see below) |
///
/// **The floor** is the wild's own contribution to this story, and the
/// FreeCAD corpus has nothing like it: a NIST inch translator prints
/// ~12 significant digits, so a coordinate carries ~1e-13 m of print
/// truncation on a 100 mm part, and at ε = 1e-12 that truncation is
/// larger than the band the adoption gates decide in. The file is not
/// wrong and the kernel is not wrong; the file simply does not state
/// itself to that precision, and the gate says so by name instead of
/// certifying a carrier it cannot. The floor MOVED between M7-4's
/// measurement and M7-5's re-measurement — at ε = 1e-10 `nist_ftc_09`
/// now refuses typed at the pcurve re-mint's single-branch unwrap
/// (measured identically on this branch and on the main it forked
/// from, so the drift predates the band unit; the two band fixtures
/// themselves import and certify at EVERY decade 1e-12 … 1e-6) — so
/// the pinned floor is now the measured 1e-9.
///
/// **The ceiling**: M7-4 measured refusals above 1e-8 (the pcurve
/// mint gate); at the M7-5 re-measurement all 9 certify at 1e-7 and
/// 1e-6 under [`assert_sub_tolerance_obligation`]. Certifying is a
/// weaker row than the full oracle comparison, and re-ratifying a
/// wider window would change what the hosted 1e-6 matrix row
/// exercises — that re-widening is a recorded pickup, not done here;
/// the pinned ceiling stays 1e-8.
///
/// Outside the window the certifying rows skip LOUDLY, and
/// [`no_wild_file_panics`] carries [`assert_sub_tolerance_obligation`]
/// over the whole corpus in their place — never nothing.
const WILD_EPS_FLOOR: f64 = 1e-9;
const WILD_EPS_CEILING: f64 = 1e-8;

/// Whether the ambient ε is inside the window this corpus certifies in.
///
/// INVARIANT: the obligation that replaces a skipped certifying row is
/// a property of the CORPUS, not of the row — it sweeps all 13
/// fixtures and asserts the same thing whichever row asked. So it is
/// asserted ONCE per run, by [`no_wild_file_panics`], and this gate
/// only says which side of the window we are on and prints the loud
/// skip. Three rows call it; running the sweep here ran the whole
/// corpus three extra times for one claim.
fn wild_scale_gate(row: &str) -> bool {
    let eps = geom_core::Tolerance::get().eps;
    if (WILD_EPS_FLOOR..=WILD_EPS_CEILING).contains(&eps) {
        return true;
    }
    println!(
        "{row}: outside the wild corpus's certifying window — ambient ε {eps:e} m is not in          [{WILD_EPS_FLOOR:e}, {WILD_EPS_CEILING:e}]. The every-ε obligation is asserted          over the whole corpus by `no_wild_file_panics` instead of this row's certifying one."
    );
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
///
/// **Sibling, same job, same crate:** `freecad.rs`'s
/// `the_committed_freecad_corpus_still_says_what_chart_and_units_quote`
/// pins the FreeCAD corpus's dialect facts. The two ask different
/// questions on purpose — this row is `any(corpus contains X)`, because
/// its claim is *"each gap is present in something committed"*; that one
/// is all-or-none per file, because `chart`'s and `units`' claims are
/// universally quantified over their corpus. A third corpus claim
/// belongs beside whichever of these matches its quantifier.
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
    // Leg D: an assembly transform PRESENT and — since M7-6 —
    // genuinely TRAVERSED on a wild file: `dm1-id-214`'s geometry
    // resolves clean through `resolve_shape` (stage-1 promotion), so
    // the assembly layer reads its seven per-component transforms for
    // real. Since M8 it no longer refuses them: each occurrence's
    // `REPRESENTATION_RELATIONSHIP` says which component the map
    // places, and the seven materialize as seven placed instances of
    // the three breps. The file's remaining refusal is a geometry one
    // (the WILD_REFUSALS row). Per-instance placement CORRECTNESS —
    // that each frame lands on its own component and on no other — is
    // pinned on planted mutations of `twobody_importexport`'s real
    // transforms, per solid, in
    // `freecad.rs::refusals_survive_the_dialect_relaxations` (d).
    assert!(any(&|t| t.contains("ITEM_DEFINED_TRANSFORMATION")), "Leg D");
    // The knots-implied spline sub-types (M7-6 vocabulary): the
    // corpus carries both QUASI_UNIFORM forms (dm1's 31 curves + 5
    // surfaces), read with synthesized clamped knots.
    assert!(
        any(&|t| t.contains("QUASI_UNIFORM_CURVE"))
            && any(&|t| t.contains("QUASI_UNIFORM_SURFACE")),
        "the QUASI_UNIFORM vocabulary"
    );
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
/// read back is the SAME body — census exactly, every geometric datum
/// bit-preserved, volume to summation order (the in-row comment says
/// why that last budget is ulps rather than zero) — and the second
/// export is byte-identical to the first.
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
        // Volume across the wire: the same per-face contributions,
        // summed in each body's face-ARENA order — which is the
        // assembly walk's mef order, keyed by the input's entity
        // NUMBERING. A wild file may number a shell's records in an
        // order our canonical re-numbering does not reproduce
        // (measured on `cq_red_cube_blue_cylinder` at the M7-5 flip:
        // its cube shell's interleaved ids close faces in a different
        // order than the re-import of our own export does, while
        // every carrier, parameter interval, and vertex of the two
        // bodies compares BIT-equal). Only the summation order moves,
        // so the honest exactness budget is a few ulps of the total,
        // not zero; the sharper claims still hold exactly — the
        // census above, and the byte-identical second export below.
        let (v1, v2) = (
            topo::mass_properties(&body).unwrap().volume,
            topo::mass_properties(&again).unwrap().volume,
        );
        assert!(
            (v1 - v2).abs() <= 4.0 * f64::EPSILON * v1.abs(),
            "{name}: volume across the wire, to summation order: {v1} vs {v2}"
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
///
/// `dm1-id-214` is in [`WILD_REFUSALS`] — the obligation sweep and the
/// dialect pin read the whole table — but its disposition is pinned by
/// `r1_dm1_probe::dm1_no_longer_refuses_at_the_instancing_gate`, not
/// here; see the `continue` below.
#[test]
fn wild_refusals_are_typed_and_name_their_class() {
    for (name, class) in WILD_REFUSALS {
        // **dm1's row lives in `r1_dm1_probe`.** Its two ε cells (the
        // fine bands' rational-flux stall, ambient 1e-6's `#389`
        // ladder gap) are pinned there STRUCTURALLY — the typed
        // variant, `id == 389`, `attempts.is_empty()`, and the
        // stalled-quadrature fragment — which is strictly more than
        // the substring this loop checks, plus the entity-naming
        // check moved there with it. dm1 alone costs ~30× the other
        // three fixtures put together to import, so it is imported
        // once per run, where the sharper assertions are.
        if name.contains("dm1-id-214") {
            continue;
        }
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
            // The shared at-rest gate's verdict names the
            // `MANIFOLD_SOLID_BREP` it was asked about, and each
            // verdict inside names the kernel entity it is about —
            // the same "go and look at it" obligation, one layer in
            // (dm1 since #327: its D7 half is retired and what refuses
            // is the banked rational-patch-flux lane).
            StepImportError::TierInvalid { solid, errors } => {
                solid.is_some_and(|id| id > 0) && !errors.is_empty()
            }
            other => panic!("{name}: unexpected refusal kind: {other:?}"),
        };
        assert!(names_something, "{name}: the refusal must name an entity");
    }
}

/// **The band re-mint, pinned as data.** Open CASCADE never splits a
/// periodic face: a cylinder's or torus's lateral band arrives as its
/// two full-period rim circles with no seam generator between them,
/// and the kernel's face model (one outer loop plus rings) has no
/// volume construction for the ring adoption would make of the second
/// rim (`RingOnCurvedFace`). Until M7-5 that was a NAMED refusal on
/// both fixtures here; the band seam re-mint (`normalize::band_seam`)
/// retired it by minting the seam generator at the surface's own
/// u_ref azimuth and re-writing each band as one single-loop face.
///
/// This row pins the flip's honesty: the mint is REPORTED, never
/// silent, and its census mapping is exactly the one derived from
/// first principles — a rim whose vertex already sits at the u_ref
/// azimuth splits nowhere (ftc_11's cylinders), every other rim
/// splits once (cq's two rims; ftc_11's tori, whose rim vertices sit
/// at −π/2 and π), and every band gains exactly one seam edge.
/// One pinned band mapping: the `ADVANCED_FACE` entity id, the
/// boundary census the file states for it, and the census the re-mint
/// leaves — each census as (faces, edges, vertices).
type BandCensusRow = (u64, (usize, usize, usize), (usize, usize, usize));

#[test]
fn the_band_re_mint_reports_its_normalizations() {
    use step_import::{FaceCensus, NormalizationKind};
    let rows: [(&str, &[BandCensusRow]); 2] = [
        (
            "occ-oss/cq_red_cube_blue_cylinder.step",
            // One cylinder band; both rim vertices half a turn from
            // u_ref, so both rims split.
            &[(54, (1, 2, 2), (1, 5, 4))],
        ),
        (
            "nist/nist_ftc_11_asme1_rb.stp",
            // Two cylinder bands whose rim vertices sit AT u_ref (no
            // splits), two torus bands splitting both rims each —
            // rims #128 and #91 shared ACROSS bands, so those splits
            // also patch the neighbouring band's minted loop.
            &[
                (95, (1, 2, 2), (1, 3, 2)),
                (135, (1, 2, 2), (1, 3, 2)),
                (175, (1, 2, 2), (1, 5, 4)),
                (187, (1, 2, 2), (1, 5, 4)),
            ],
        ),
    ];
    for (name, expected) in rows {
        let Ok(StepImport::Solid { normalizations, .. }) =
            import_step(&wild(name), &ImportOptions::default())
        else {
            panic!("{name}: the band fixture imports first-class since M7-5");
        };
        let census = |(faces, edges, vertices)| FaceCensus {
            faces,
            edges,
            vertices,
        };
        let got: Vec<_> = normalizations
            .iter()
            .map(|n| (n.face, n.kind, n.file_census, n.kernel_census))
            .collect();
        let want: Vec<_> = expected
            .iter()
            .map(|&(face, file, kernel)| {
                (
                    face,
                    NormalizationKind::SeamlessPeriodicBand,
                    census(file),
                    census(kernel),
                )
            })
            .collect();
        assert_eq!(got, want, "{name}: the reported band normalizations");
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
/// files with zero panics — a one-time reading over a 28-file candidate
/// set that no longer exists in the tree, so nothing can re-take it and
/// no guard is available for it; what survives the triage is the 13
/// committed fixtures, and the INVARIANT below is their guard. That
/// outcome is the fail-loud contract
/// meeting data nobody here wrote, and it is worth an assertion that
/// can never be quietly dropped as the subset widens.
/// **Two cells, because in-window the corpus is already swept.**
///
/// INVARIANT: every one of the 13 committed fixtures goes through
/// `import_step` on every run, and a panic in any of them is red.
///
/// * INSIDE the window, this row does nothing but pin the corpus
///   count. All 13 are imported with an EXACT disposition by the two
///   rows above — [`wild_files_import_and_agree_with_the_oracle`]
///   walks all 9 `WILD_IMPORTS` (census, three tiers, oracle volume)
///   and [`wild_refusals_are_typed_and_name_their_class`] walks 3 of
///   the 4 `WILD_REFUSALS`, with `dm1-id-214`'s disposition pinned by
///   `r1_dm1_probe::dm1_no_longer_refuses_at_the_instancing_gate`.
///   A panic anywhere in those 13 imports fails a row that asserts
///   strictly more than "did not panic", so a fourth sweep of the same
///   corpus would buy nothing.
/// * OUTSIDE it, those rows skip (their certifying claims do not hold
///   there), so this row is the corpus's only sweep — and it runs
///   [`assert_sub_tolerance_obligation`] ONCE, under the `catch_unwind`
///   that makes "never a panic" a distinct verdict from "refused
///   typed".
#[test]
fn no_wild_file_panics() {
    let names: Vec<&str> = WILD_IMPORTS
        .iter()
        .chain(WILD_REFUSALS.iter().map(|(n, _)| n))
        .copied()
        .collect();
    assert_eq!(names.len(), 13, "the whole committed wild corpus");
    let eps = geom_core::Tolerance::get().eps;
    if (WILD_EPS_FLOOR..=WILD_EPS_CEILING).contains(&eps) {
        println!(
            "no_wild_file_panics: ambient ε {eps:e} m is inside [{WILD_EPS_FLOOR:e}, \
             {WILD_EPS_CEILING:e}] — all 13 fixtures are imported with an exact \
             disposition by the certifying rows, which a panic would fail first."
        );
        return;
    }
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert_sub_tolerance_obligation("no_wild_file_panics");
    }));
    assert!(
        outcome.is_ok(),
        "the wild corpus unwound at ε {eps:e} — the wild contract is a RESULT, always \
         (the panic's own message and location are on stderr above)"
    );
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
            ..ImportOptions::default()
        },
    )
    .expect("imports under an override");
    assert_eq!(overridden.eps_in(), 2.5e-7, "the per-call override wins");
}
