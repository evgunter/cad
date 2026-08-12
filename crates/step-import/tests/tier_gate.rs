//! **M7-7 acceptance: the shared at-rest gate over the whole corpus**
//! (issue #260, ruling (a)).
//!
//! Since M7-7, `import_step` hands each `MANIFOLD_SOLID_BREP` on its
//! own body, and then the assembled body, to
//! `topo::validate_geometric` — the kernel's own at-rest validator,
//! the same function a native body's caller runs — and ships only
//! bodies it passes. This suite is the per-file record of what that
//! means, for EVERY committed STEP file the workspace holds: the two
//! fixture roots are walked, so a fixture added without a row here
//! turns this suite red rather than quietly escaping the gate.
//!
//! Three things are asserted per file, at three tolerances (the file's
//! own ε_in, and overrides 1e-6 / 1e-12):
//!
//! * **the disposition** — solid, wireframe, or a typed refusal with
//!   its reason. 51 of the 53 files hold ONE disposition across the
//!   whole matrix, and that constancy is the claim. The two that do
//!   not are marked `EpsSensitive` and pinned cell by cell in
//!   [`EPS_ROWS`], each cell carrying the live signature of the
//!   sub-reason that actually fires there — because a row that may
//!   Pass at one ambient ε and refuse typed at another can otherwise
//!   be green for the wrong reason. No ε is ever special-cased into
//!   silence: a refusal that fires at a tighter tolerance is pinned AS
//!   the honest posture, and the reason it moved is written down;
//! * **tier-validity of every shipped body, positively** — the gate is
//!   re-run on the body `import_step` handed out. That is redundant
//!   only while the gate is wired: delete or narrow the call and these
//!   rows are what catches the invalid body going out the door; and
//! * **the census of every shipped body**, which is what makes the
//!   rows able to see entity loss at all (R1 MINOR-2: a `take(1)` in
//!   `build_body` used to leave this whole suite green). Four corpus
//!   files carry two solids each — `compound_two`,
//!   `twobody_importexport`, `cq_red_cube_blue_cylinder`,
//!   `kiss_assembly` — so the solid counts are also the standing
//!   evidence that the per-solid pass runs on real corpus geometry.
//!
//! **Scope, stated honestly** (R1 MINOR-1): the gate is tier 3, and on
//! a body with no declared contacts the tier-3′ form is strictly
//! stronger — it runs the coincidence census too. Imports declare no
//! contacts, so an imported assembly whose parts TOUCH is checked less
//! than its native twin; `kiss_assembly` is exactly that body, and
//! `review_r1_tier_gate_probes.rs` pins the difference. Banked with
//! the M8 contact program (D7 step 4), not silently absorbed here.
//!
//! **Measured (M7-7), at the ambient default:** no committed corpus
//! file fails the gate. 44 solids pass, 8 files refuse for reasons that predate this
//! unit (one of them, `band_c180`, at the gate itself — the inside-out
//! torus band, refusing now through the general mechanism that
//! replaced its band-only backstop), and one file is a wireframe. The
//! previously-'importing' body that turns out to have been invalid all
//! along — #260's "arguably the point" — is NOT in the committed
//! corpus; the one body class the gate newly refuses is the
//! rational-walled loft, which has no committed fixture and whose row
//! lives in `nurbs_import.rs`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::{Path, PathBuf};

use Disposition::{EpsSensitive, Pass, Refused, Wireframe};
use step_import::{ImportOptions, StepImport, StepImportError, import_step};

/// What a corpus file does at import, at every tolerance in the sweep.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Disposition {
    /// Imports as a solid, and the shipped body passes the gate, with
    /// this census: (solids, shells, faces, edges, vertices).
    ///
    /// The census is not decoration. A disposition alone cannot see a
    /// reader that silently drops solids — the R1 review's `take(1)`
    /// mutation of `build_body` left this suite entirely green — and
    /// the solid count is exactly the number the per-solid gate now
    /// iterates, so a body arriving with fewer solids than the file
    /// states means fewer solids were gated.
    Pass(usize, usize, usize, usize, usize),
    /// Imports as a curve-set wireframe (no body, nothing to gate).
    Wireframe,
    /// Refuses typed; the string is a distinctive fragment of the
    /// refusal's own message, so the ROW says why, not just that.
    Refused(&'static str),
    /// This file's disposition genuinely MOVES with the ambient ε, and
    /// its cells are pinned one by one in [`EPS_ROWS`] — a marker, not
    /// an outcome, so no cell of it can be satisfied by accident.
    EpsSensitive,
}

/// The ambient tolerances this suite pins cell by cell — the hosted
/// matrix's three rows (`CAD_TOLERANCE_EPS` unset, `1e-6`, `1e-12`).
const PINNED_AMBIENT: [f64; 3] = [geom_core::tolerance::DEFAULT_EPS, 1e-6, 1e-12];

/// The `eps_in` overrides swept per ambient row, tagged as the failure
/// messages name them.
const EPS_IN_ROWS: [(&str, Option<f64>); 3] =
    [("file", None), ("1e-6", Some(1e-6)), ("1e-12", Some(1e-12))];

/// **The ε-row pins.** Two corpus files' dispositions are a function of
/// the ambient ε, and hiding that behind one row per file would make
/// the suite either red for an honest reason or green for a wrong one.
/// Each cell is `(file, ambient ε, eps_in row, disposition)`; the
/// coverage test below holds this table to exactly one cell per
/// (`EpsSensitive` file × [`PINNED_AMBIENT`] × [`EPS_IN_ROWS`]), so a
/// pin can neither go missing nor rot unread.
///
/// Both movements are the kernel deciding HONESTLY at the tolerance it
/// was given, and the fragments below are each sub-reason's own live
/// signature, never the shared preamble:
///
/// * **`ftc11_uref_off`** is the deliberately-degenerate band fixture.
///   Its seam residual is ~1.6e-6 m, so at ambient 1e-6 that margin
///   lands INSIDE the ambiguity band (zero = ε, escalate = Kε) and the
///   refusal is an ESCALATION rather than a definite verdict; at
///   ambient 1e-12 the same margin is decisively outside every band, so
///   the coincidence predicates that refused it at coarser ε ("tangent
///   planes coincide", the Intersection transversality precondition)
///   no longer fire, and the file imports. A coincidence test refusing
///   what is TOO CLOSE must stop refusing as ε shrinks; that direction
///   is the predicate being right, not the gate being loosened.
/// * **`nist_ftc_09_asme1_rd`** refuses at ambient 1e-12 with definite
///   `EndpointStart` residuals on both the seam and the mapped-curve
///   arm. This is the floor `wild.rs` already measures and documents:
///   the NIST inch translator prints ~12 significant digits, so the
///   file does not state itself to 1e-12 m, and the adoption ladder
///   says so by name instead of certifying a carrier it cannot.
const EPS_ROWS: [(&str, f64, &str, Disposition); 27] = [
    // -- tests/fixtures/band/ftc11_uref_off.stp -----------------------
    (FTC11, 1e-9, "file", Refused(SEAM_HALFPLANE_DEFINITE)),
    (FTC11, 1e-9, "1e-6", Refused(TANGENT_PLANES_COINCIDE)),
    (FTC11, 1e-9, "1e-12", Refused(TANGENT_PLANES_COINCIDE)),
    (FTC11, 1e-6, "file", Refused(SEAM_HALFPLANE_ESCALATED)),
    (FTC11, 1e-6, "1e-6", Refused(PARAM_SPAN_ESCALATED)),
    (FTC11, 1e-6, "1e-12", Refused(PARAM_SPAN_ESCALATED)),
    (FTC11, 1e-12, "file", Refused(SEAM_HALFPLANE_DEFINITE)),
    (FTC11, 1e-12, "1e-6", Pass(1, 1, 6, 16, 12)),
    (FTC11, 1e-12, "1e-12", Pass(1, 1, 6, 16, 12)),
    // -- tests/fixtures/wild/nist/nist_ftc_09_asme1_rd.stp ------------
    (NIST09, 1e-9, "file", Pass(1, 1, 158, 454, 300)),
    (NIST09, 1e-9, "1e-6", Pass(1, 1, 158, 454, 300)),
    (NIST09, 1e-9, "1e-12", Pass(1, 1, 158, 454, 300)),
    (NIST09, 1e-6, "file", Pass(1, 1, 158, 454, 300)),
    (NIST09, 1e-6, "1e-6", Pass(1, 1, 158, 454, 300)),
    (NIST09, 1e-6, "1e-12", Pass(1, 1, 158, 454, 300)),
    (NIST09, 1e-12, "file", Refused(ENDPOINT_START_MAPPED_CURVE)),
    (NIST09, 1e-12, "1e-6", Refused(ENDPOINT_START_MAPPED_CURVE)),
    (NIST09, 1e-12, "1e-12", Refused(ENDPOINT_START_MAPPED_CURVE)),
    // -- tests/fixtures/wild/stepcode/dm1-id-214.stp (#327) -----------
    // Six cells at the rational-flux stall, three at the ladder's
    // `#389` gap; ε_in moves nothing, which is itself the measurement
    // (recognition certifies this file's carriers with ~14 decades of
    // margin, so no interpretation budget in this sweep changes what
    // is promoted).
    (DM1, 1e-9, "file", Refused(RATIONAL_FLUX_STALL)),
    (DM1, 1e-9, "1e-6", Refused(RATIONAL_FLUX_STALL)),
    (DM1, 1e-9, "1e-12", Refused(RATIONAL_FLUX_STALL)),
    (DM1, 1e-6, "file", Refused(LADDER_NO_DESCRIPTION)),
    (DM1, 1e-6, "1e-6", Refused(LADDER_NO_DESCRIPTION)),
    (DM1, 1e-6, "1e-12", Refused(LADDER_NO_DESCRIPTION)),
    (DM1, 1e-12, "file", Refused(RATIONAL_FLUX_STALL)),
    (DM1, 1e-12, "1e-6", Refused(RATIONAL_FLUX_STALL)),
    (DM1, 1e-12, "1e-12", Refused(RATIONAL_FLUX_STALL)),
];

const FTC11: &str = "tests/fixtures/band/ftc11_uref_off.stp";
const DM1: &str = "tests/fixtures/wild/stepcode/dm1-id-214.stp";
/// dm1's fine-band sub-reason: the shared at-rest gate cannot compute
/// the exact-B-rep volume of a RATIONAL cylinder wall — the banked
/// rational-patch-flux lane, named specifically so the gate's preamble
/// (which a tier-1/2 regression would also match) cannot stand in.
const RATIONAL_FLUX_STALL: &str = "the certified quadrature enclosure stalled at";
/// dm1's coarse-band sub-reason: the ladder's own refusal on edge
/// `#389`, a two-point `QUASI_UNIFORM_CURVE` polyline that stays NURBS
/// and is offered zero candidates.
const LADDER_NO_DESCRIPTION: &str = "edge #389: no intensional description certifies";
const NIST09: &str = "tests/fixtures/wild/nist/nist_ftc_09_asme1_rd.stp";

/// The seam carrier's residual is DECIDEDLY outside the band.
const SEAM_HALFPLANE_DEFINITE: &str =
    "SeamHalfplane residual at sample 0 definitely exceeds the tolerance band";
/// The same residual, IN the band: escalate-never-guess, by name.
const SEAM_HALFPLANE_ESCALATED: &str =
    "SeamHalfplane at sample 0 escalated: predicate 'carrier_in_seam_halfplane' indeterminate";
/// Coarse enough for the two walls to read as one: the Intersection
/// transversality precondition fails, and the ladder says which.
const TANGENT_PLANES_COINCIDE: &str = "tangent planes coincide at interior sample 1 — the Intersection transversality \
     precondition fails";
/// At ambient 1e-6 the file's own span decision is in-band too, and it
/// is reached first — at assembly, before any edge is adopted.
const PARAM_SPAN_ESCALATED: &str =
    "ParamSpan at sample 0 escalated: predicate 'interval_span_forward' indeterminate";
/// Naming the MAPPED-CURVE arm pins that BOTH candidates were tried and
/// both refused definite — the seam arm alone would match a prefix.
const ENDPOINT_START_MAPPED_CURVE: &str = "mapped curve: geometry attachment gate: certification: EndpointStart residual at sample 0 \
     definitely exceeds";

/// Every committed STEP file, with the disposition measured at M7-7.
/// Paths are relative to this crate's manifest directory (the `../`
/// rows are `step-export`'s corpus, which this crate imports from).
const CORPUS: [(&str, Disposition); 54] = [
    ("tests/fixtures/band/band_a.stp", Pass(1, 1, 2, 6, 4)),
    ("tests/fixtures/band/band_a180.stp", Pass(1, 1, 2, 6, 4)),
    ("tests/fixtures/band/band_b180.stp", Pass(1, 1, 2, 6, 4)),
    (
        "tests/fixtures/band/band_c180.stp",
        Refused("shared at-rest validation gate"),
    ),
    (
        "tests/fixtures/band/band_d180.stp",
        Refused("ORIENTATION-INVERTED cylinder band"),
    ),
    (
        "tests/fixtures/band/band_d_invcyl.stp",
        Refused("ORIENTATION-INVERTED cylinder band"),
    ),
    ("tests/fixtures/band/ftc11_uref_off.stp", EpsSensitive),
    ("tests/fixtures/band/washer180.stp", Pass(1, 1, 3, 5, 4)),
    ("tests/fixtures/band/washer90.stp", Pass(1, 1, 3, 5, 4)),
    ("tests/fixtures/freecad/box.step", Pass(1, 1, 6, 12, 8)),
    (
        "tests/fixtures/freecad/box_fillet_corner.step",
        Pass(1, 1, 10, 21, 13),
    ),
    (
        "tests/fixtures/freecad/box_fillet_edge.step",
        Pass(1, 1, 7, 15, 10),
    ),
    (
        "tests/fixtures/freecad/box_hole.step",
        Pass(1, 1, 7, 15, 10),
    ),
    (
        "tests/fixtures/freecad/box_importexport.step",
        Pass(1, 1, 6, 12, 8),
    ),
    (
        "tests/fixtures/freecad/compound_two.step",
        Pass(2, 2, 8, 14, 10),
    ),
    ("tests/fixtures/freecad/cone_apex.step", Pass(1, 1, 3, 4, 3)),
    (
        "tests/fixtures/freecad/cone_trunc.step",
        Pass(1, 1, 3, 3, 2),
    ),
    ("tests/fixtures/freecad/cylinder.step", Pass(1, 1, 3, 3, 2)),
    (
        "tests/fixtures/freecad/fuse_boxes.step",
        Pass(1, 1, 14, 32, 20),
    ),
    ("tests/fixtures/freecad/sphere.step", Pass(1, 1, 2, 2, 2)),
    ("tests/fixtures/freecad/torus.step", Pass(1, 1, 2, 4, 2)),
    (
        "tests/fixtures/freecad/twobody_importexport.step",
        Pass(2, 2, 8, 14, 10),
    ),
    (
        "tests/fixtures/wild/adafruit/1982_MPR121.step",
        Pass(1, 1, 10, 24, 16),
    ),
    (
        "tests/fixtures/wild/adafruit/328_2500mAh_battery.step",
        Pass(1, 1, 6, 12, 8),
    ),
    (
        "tests/fixtures/wild/adafruit/64_Halfsize_Breadboard.step",
        Pass(1, 1, 18, 48, 32),
    ),
    (
        "tests/fixtures/wild/adafruit/805_slide_switch.step",
        Pass(1, 1, 19, 46, 30),
    ),
    (
        "tests/fixtures/wild/adafruit/931_OLED_128x32_I2C.step",
        Pass(1, 1, 24, 60, 40),
    ),
    (
        "tests/fixtures/wild/nist/nist_ftc_09_asme1_rd.stp",
        EpsSensitive,
    ),
    (
        "tests/fixtures/wild/nist/nist_ftc_11_asme1_rb.stp",
        Pass(1, 1, 6, 14, 10),
    ),
    (
        "tests/fixtures/wild/occ-oss/b123d_nema17_bracket.step",
        Refused("(SURFACE_CURVE) is outside the imported subset"),
    ),
    (
        "tests/fixtures/wild/occ-oss/cq_red_cube_blue_cylinder.step",
        Pass(2, 2, 9, 17, 12),
    ),
    (
        // Refuses in EVERY ambient-ε cell, but the refusal SITE and
        // sub-reason shift with ε (edge #170 Surface2Residual at
        // default/1e-12; edge #177 tangent-planes at 1e-6). This row's
        // fragment is the shared adoption-ladder preamble, so the shift
        // is invisible here BY DESIGN: pinning it would cost 9 more
        // EpsSensitive cells to buy site-pinning the pin design does
        // not offer anywhere (fragments pin sub-reason, not site). The
        // preamble still pins the refusal to the adoption-ladder class
        // — a drift to a parse error, crash, or Pass goes red.
        "tests/fixtures/wild/stepcode/TAIL_TURBINE.stp",
        Refused("no intensional description certifies"),
    ),
    (
        // **Two halves of this refusal are now retired.** M8
        // instancing took the placement half (dm1's seven occurrences
        // of three component representations materialize as seven
        // placed instances); #327 took the D7 half — the rims of the
        // file's seven RATIONAL cylinders arrive as rational-quadratic
        // NURBS carriers, stage-1 CURVE recognition certifies them as
        // circles against an exact ring-composite bound and promotes
        // them, and every edge of every instance adopts with its
        // pcurve minted and certified.
        //
        // What is left is the SHARED AT-REST GATE on those same
        // rational walls: the exact-B-rep volume's quadrature
        // enclosure stalls short of its target — the banked
        // rational-patch-flux lane, the lane a NATIVELY built
        // rational-walled loft refuses on too. The fragment names that
        // stall specifically rather than the gate's preamble, because
        // the preamble would also match a tier-1/2 verdict, which
        // would be a regression and not this lane.
        //
        // **ε-SENSITIVE since #327**, and the sweep is the reason to
        // know it: at the two FINE ambient bands the frontier is the
        // rational-flux stall above, but at ambient 1e-6 the ladder
        // stops earlier, on edge `#389`. Retiring #685 is what made
        // #389 reachable at all — it had been masked behind #685 at
        // every band — so the coarse cell is a PRE-EXISTING gap newly
        // exposed, not a movement of anything #327 built. Nine cells
        // in `EPS_ROWS`; ε_in moves none of them.
        DM1,
        EpsSensitive,
    ),
    (
        "tests/fixtures/wild/stepcode/io1-cm-214.stp",
        Refused("expected a doubled backslash"),
    ),
    (
        "tests/fixtures/wild/stepcode/sg1-c5-214.stp",
        Pass(1, 1, 16, 32, 20),
    ),
    (
        "../step-export/tests/fixtures/ball.step",
        Pass(1, 1, 2, 2, 2),
    ),
    (
        "../step-export/tests/fixtures/boss_union.step",
        Pass(1, 1, 10, 21, 14),
    ),
    (
        "../step-export/tests/fixtures/composed_die.step",
        Pass(1, 1, 89, 195, 129),
    ),
    (
        "../step-export/tests/fixtures/cone.step",
        Pass(1, 1, 4, 6, 4),
    ),
    (
        "../step-export/tests/fixtures/cube.step",
        Pass(1, 1, 6, 12, 8),
    ),
    (
        "../step-export/tests/fixtures/cut_cylinder.step",
        Pass(1, 1, 4, 6, 4),
    ),
    (
        "../step-export/tests/fixtures/die.step",
        Pass(1, 1, 11, 24, 16),
    ),
    (
        "../step-export/tests/fixtures/die_pips.step",
        Pass(1, 1, 48, 96, 71),
    ),
    (
        "../step-export/tests/fixtures/donut.step",
        Pass(1, 1, 2, 4, 2),
    ),
    (
        "../step-export/tests/fixtures/filleted_die.step",
        Pass(1, 1, 26, 48, 24),
    ),
    (
        "../step-export/tests/fixtures/kiss_assembly.step",
        Pass(2, 2, 22, 48, 32),
    ),
    (
        "../step-export/tests/fixtures/lily_lantern.step",
        Pass(1, 1, 8, 14, 8),
    ),
    (
        "../step-export/tests/fixtures/loft_prism.step",
        Pass(1, 1, 6, 12, 8),
    ),
    (
        "../step-export/tests/fixtures/nonuniform_loft.step",
        Pass(1, 1, 6, 12, 8),
    ),
    (
        "../step-export/tests/fixtures/notched.step",
        Pass(1, 1, 6, 12, 8),
    ),
    (
        "../step-export/tests/fixtures/nurbs_wireframe.step",
        Wireframe,
    ),
    (
        // The #327 reporting witness: a `GEOMETRIC_CURVE_SET` stating
        // dm1's carrier form verbatim (degree 2, 7 points, weights
        // `1, ½, …`, knots at multiples of √3, the 3×120°
        // construction, r = 5 mm). It is here because the corpus table
        // is the WHOLE corpus by construction — and its disposition is
        // the same `Wireframe` every curve-set file gets: promotion
        // changes the carrier's DESCRIPTION, never the file's
        // disposition. `curve_promotion_report.rs` is what asserts the
        // promotion itself.
        "tests/fixtures/curveset_rational_circle.step",
        Wireframe,
    ),
    (
        "../step-export/tests/fixtures/swept_elbow.step",
        Pass(1, 1, 6, 12, 8),
    ),
    (
        "../step-export/tests/fixtures/washer.step",
        Pass(1, 1, 4, 8, 4),
    ),
];

/// Every `.step` / `.stp` file under `dir`, recursively, sorted.
fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("reading {dir:?}: {e}"))
        .map(|e| e.unwrap().path())
        .collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            walk(&p, out);
        } else {
            let s = p.to_string_lossy().to_lowercase();
            if s.ends_with(".step") || s.ends_with(".stp") {
                out.push(p);
            }
        }
    }
}

/// The corpus as the filesystem holds it, keyed the way the table is.
fn discovered() -> Vec<String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    walk(&root.join("tests/fixtures"), &mut files);
    walk(&root.join("../step-export/tests/fixtures"), &mut files);
    files
        .iter()
        .map(|p| {
            p.strip_prefix(&root)
                .unwrap_or(p)
                .to_string_lossy()
                .into_owned()
        })
        .collect()
}

/// The table covers the corpus exactly — no file escapes the sweep by
/// being new, and no row survives its fixture's deletion.
#[test]
fn the_table_is_the_whole_corpus() {
    let mut found = discovered();
    found.sort();
    let mut tabled: Vec<String> = CORPUS.iter().map(|(p, _)| (*p).to_owned()).collect();
    tabled.sort();
    assert_eq!(
        found, tabled,
        "the committed STEP corpus and this suite's table have diverged"
    );
}

/// The claim that holds at ANY ambient ε, pinned matrix or not: the
/// outcome is TYPED — a solid whose SHIPPED body the gate itself
/// passes, a wireframe, or a typed refusal — and never a body the
/// kernel would call geometrically false. A row that asserted nothing
/// off the matrix would be a green tick for work not done.
fn assert_typed_outcome(who: &str, got: Result<StepImport, StepImportError>) {
    match got {
        Ok(StepImport::Solid { body, .. }) => assert_eq!(
            topo::validate_geometric(&body),
            Ok(()),
            "{who}: import shipped a body its own gate refuses"
        ),
        Ok(StepImport::Wireframe { .. }) | Err(_) => {}
    }
}

/// The [`EPS_ROWS`] pins cover exactly the `EpsSensitive` files, at
/// exactly the pinned ambient tolerances, with exactly one cell per
/// `eps_in` row — so a marker cannot go unpinned and a pin cannot
/// outlive the marker that reads it.
#[test]
fn every_eps_sensitive_row_is_pinned_cell_by_cell() {
    let mut markers: Vec<&str> = CORPUS
        .iter()
        .filter(|(_, d)| *d == EpsSensitive)
        .map(|(p, _)| *p)
        .collect();
    markers.sort_unstable();
    let mut pinned: Vec<&str> = EPS_ROWS.iter().map(|(p, ..)| *p).collect();
    pinned.sort_unstable();
    pinned.dedup();
    assert_eq!(
        markers, pinned,
        "an EpsSensitive row with no cells, or cells for a file that is not EpsSensitive"
    );
    for file in &markers {
        for ambient in PINNED_AMBIENT {
            for (tag, _) in EPS_IN_ROWS {
                let hits = EPS_ROWS
                    .iter()
                    .filter(|(p, a, t, _)| p == file && *a == ambient && *t == tag)
                    .count();
                assert_eq!(
                    hits, 1,
                    "{file} @ ambient {ambient:e} / eps {tag}: {hits} pins"
                );
            }
        }
    }
}

/// What this row must do, or `None` when the ambient ε is off the
/// pinned matrix and only the every-ε obligation is claimed.
fn expected(rel: &str, row: Disposition, eps_tag: &str) -> Option<Disposition> {
    if row != EpsSensitive {
        return Some(row);
    }
    let ambient = geom_core::Tolerance::get().eps;
    EPS_ROWS
        .iter()
        .find(|(p, a, t, _)| *p == rel && *a == ambient && *t == eps_tag)
        .map(|(.., d)| *d)
}

/// Every corpus file's disposition, and the positive tier-validity of
/// every body that ships — at each tolerance in the sweep.
#[test]
fn every_corpus_import_passes_the_shared_gate() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for (eps_tag, eps_in) in EPS_IN_ROWS {
        for (rel, row) in CORPUS {
            let text = std::fs::read_to_string(root.join(rel))
                .unwrap_or_else(|e| panic!("reading {rel}: {e}"));
            let options = ImportOptions { eps_in };
            let who = format!("{rel} @ eps {eps_tag}");
            let Some(want) = expected(rel, row, eps_tag) else {
                // Off the pinned ambient matrix. The disposition of an
                // ε-sensitive file is not knowable here, but the gate's
                // claim still is, and asserting it is not nothing.
                assert_typed_outcome(&who, import_step(&text, &options));
                continue;
            };
            match (import_step(&text, &options), want) {
                (Ok(StepImport::Solid { body, .. }), Pass(s, sh, f, e, v)) => {
                    assert_eq!(
                        topo::validate_geometric(&body),
                        Ok(()),
                        "{who}: the SHIPPED body must be gate-clean — import handed out a \
                         body its own gate refuses, which can only mean the gate is no \
                         longer wired"
                    );
                    assert_eq!(
                        (
                            body.solids().count(),
                            body.shells().count(),
                            body.faces().count(),
                            body.edges().count(),
                            body.vertices().count()
                        ),
                        (s, sh, f, e, v),
                        "{who}: census (solids, shells, faces, edges, vertices) — a \
                         shipped body missing entities the file states is a silent \
                         loss, and a missing SOLID is also a solid the per-solid gate \
                         never saw"
                    );
                }
                (Ok(StepImport::Wireframe { .. }), Wireframe) => {}
                (Err(e), Refused(fragment)) => {
                    let msg = e.to_string();
                    assert!(
                        msg.contains(fragment),
                        "{who}: refused for a DIFFERENT reason than the table records \
                         (want a message containing {fragment:?}): {msg}"
                    );
                }
                (got, want) => panic!("{who}: disposition changed — want {want:?}, got {got:?}"),
            }
        }
    }
}

/// The reader touches the kernel's validator at exactly ONE place (the
/// #260 ask: make skipping it structurally hard). The gate is asked
/// about several subjects — each solid alone, then the assembled body
/// — but always through `lib.rs`'s `gate`, which maps the verdicts to
/// a typed refusal and does nothing else. Import owns no validation
/// logic of its own: no second entry, no kind predicate deciding who
/// is asked, no verdict filter deciding which failures count. This
/// counts the validator calls in the crate's sources and pins the
/// count at one; a second call is not automatically wrong, but it is
/// exactly the shape the old band-only backstop had, so it must be
/// argued for here rather than appear.
#[test]
fn exactly_one_validation_call_site_in_the_reader() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut sites = Vec::new();
    let mut files = Vec::new();
    let mut stack = vec![src];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).unwrap() {
            let p = entry.unwrap().path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|e| e == "rs") {
                files.push(p);
            }
        }
    }
    files.sort();
    for path in files {
        let text = std::fs::read_to_string(&path).unwrap();
        for (i, line) in text.lines().enumerate() {
            let code = line.trim_start();
            if code.starts_with("//") {
                continue;
            }
            if code.contains("validate_geometric(") || code.contains("validate_pseudomanifold(") {
                sites.push(format!("{}:{}", path.display(), i + 1));
            }
        }
    }
    assert_eq!(
        sites.len(),
        1,
        "the reader must call the kernel's at-rest validator exactly once: {sites:?}"
    );
}

/// The typed refusal is a VALIDITY refusal about the file's geometry,
/// carrying the kernel's verdicts verbatim — not prose, and not the
/// Corrupt-class kernel-bug voice. The inside-out torus band is the
/// standing fixture for it.
#[test]
fn the_refusal_carries_the_kernels_verdicts() {
    let text = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/band/band_c180.stp"),
    )
    .unwrap();
    let e = import_step(&text, &ImportOptions::default()).unwrap_err();
    let StepImportError::TierInvalid { solid, errors } = &e else {
        panic!("expected the gate's typed refusal, got: {e:?}");
    };
    assert_eq!(
        *solid, None,
        "a one-solid file's subject is the assembled body itself"
    );
    assert_eq!(
        errors.as_slice(),
        [topo::ValidationError::NegativeVolume],
        "the verdicts are the kernel's own, unfiltered and unrephrased"
    );
    let msg = e.to_string();
    for want in [
        "shared at-rest validation gate",
        "NegativeVolume",
        "signed volume is definitely negative",
    ] {
        assert!(msg.contains(want), "the message must name {want:?}: {msg}");
    }
}
