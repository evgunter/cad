//! **M7-7 acceptance: the shared at-rest gate over the whole corpus**
//! (issue #260, ruling (a)).
//!
//! Since M7-7, `import_step` hands every assembled solid to
//! `topo::validate_geometric` — the kernel's own at-rest validator,
//! the same function a native body's caller runs — and ships only
//! bodies it passes. This suite is the per-file record of what that
//! means, for EVERY committed STEP file the workspace holds: the two
//! fixture roots are walked, so a fixture added without a row here
//! turns this suite red rather than quietly escaping the gate.
//!
//! Two things are asserted per file, at three tolerances (the file's
//! own ε_in, and overrides 1e-6 / 1e-12):
//!
//! * **the disposition** — solid, wireframe, or a typed refusal with
//!   its reason — held constant across all three, so an ε-row
//!   dependence in what the corpus does becomes a red row and not a
//!   surprise; and
//! * **tier-validity of every shipped body, positively** — the gate is
//!   re-run on the body `import_step` handed out. That is redundant
//!   only while the gate is wired: delete or narrow the call and these
//!   rows are what catches the invalid body going out the door.
//!
//! **Measured at the wiring (M7-7): no committed corpus file fails the
//! gate.** 44 solids pass, 8 files refuse for reasons that predate this
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

use Disposition::{Pass, Refused, Wireframe};
use step_import::{ImportOptions, StepImport, StepImportError, import_step};

/// What a corpus file does at import, at every tolerance in the sweep.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Disposition {
    /// Imports as a solid, and the shipped body passes the gate.
    Pass,
    /// Imports as a curve-set wireframe (no body, nothing to gate).
    Wireframe,
    /// Refuses typed; the string is a distinctive fragment of the
    /// refusal's own message, so the ROW says why, not just that.
    Refused(&'static str),
}

/// Every committed STEP file, with the disposition measured at M7-7.
/// Paths are relative to this crate's manifest directory (the `../`
/// rows are `step-export`'s corpus, which this crate imports from).
const CORPUS: [(&str, Disposition); 53] = [
    ("tests/fixtures/band/band_a.stp", Pass),
    ("tests/fixtures/band/band_a180.stp", Pass),
    ("tests/fixtures/band/band_b180.stp", Pass),
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
    (
        "tests/fixtures/band/ftc11_uref_off.stp",
        Refused("no intensional description certifies"),
    ),
    ("tests/fixtures/band/washer180.stp", Pass),
    ("tests/fixtures/band/washer90.stp", Pass),
    ("tests/fixtures/freecad/box.step", Pass),
    ("tests/fixtures/freecad/box_fillet_corner.step", Pass),
    ("tests/fixtures/freecad/box_fillet_edge.step", Pass),
    ("tests/fixtures/freecad/box_hole.step", Pass),
    ("tests/fixtures/freecad/box_importexport.step", Pass),
    ("tests/fixtures/freecad/compound_two.step", Pass),
    ("tests/fixtures/freecad/cone_apex.step", Pass),
    ("tests/fixtures/freecad/cone_trunc.step", Pass),
    ("tests/fixtures/freecad/cylinder.step", Pass),
    ("tests/fixtures/freecad/fuse_boxes.step", Pass),
    ("tests/fixtures/freecad/sphere.step", Pass),
    ("tests/fixtures/freecad/torus.step", Pass),
    ("tests/fixtures/freecad/twobody_importexport.step", Pass),
    ("tests/fixtures/wild/adafruit/1982_MPR121.step", Pass),
    (
        "tests/fixtures/wild/adafruit/328_2500mAh_battery.step",
        Pass,
    ),
    (
        "tests/fixtures/wild/adafruit/64_Halfsize_Breadboard.step",
        Pass,
    ),
    ("tests/fixtures/wild/adafruit/805_slide_switch.step", Pass),
    (
        "tests/fixtures/wild/adafruit/931_OLED_128x32_I2C.step",
        Pass,
    ),
    ("tests/fixtures/wild/nist/nist_ftc_09_asme1_rd.stp", Pass),
    ("tests/fixtures/wild/nist/nist_ftc_11_asme1_rb.stp", Pass),
    (
        "tests/fixtures/wild/occ-oss/b123d_nema17_bracket.step",
        Refused("(SURFACE_CURVE) is outside the imported subset"),
    ),
    (
        "tests/fixtures/wild/occ-oss/cq_red_cube_blue_cylinder.step",
        Pass,
    ),
    (
        "tests/fixtures/wild/stepcode/TAIL_TURBINE.stp",
        Refused("no intensional description certifies"),
    ),
    (
        "tests/fixtures/wild/stepcode/dm1-id-214.stp",
        Refused("a second, different assembly placement"),
    ),
    (
        "tests/fixtures/wild/stepcode/io1-cm-214.stp",
        Refused("expected a doubled backslash"),
    ),
    ("tests/fixtures/wild/stepcode/sg1-c5-214.stp", Pass),
    ("../step-export/tests/fixtures/ball.step", Pass),
    ("../step-export/tests/fixtures/boss_union.step", Pass),
    ("../step-export/tests/fixtures/composed_die.step", Pass),
    ("../step-export/tests/fixtures/cone.step", Pass),
    ("../step-export/tests/fixtures/cube.step", Pass),
    ("../step-export/tests/fixtures/cut_cylinder.step", Pass),
    ("../step-export/tests/fixtures/die.step", Pass),
    ("../step-export/tests/fixtures/die_pips.step", Pass),
    ("../step-export/tests/fixtures/donut.step", Pass),
    ("../step-export/tests/fixtures/filleted_die.step", Pass),
    ("../step-export/tests/fixtures/kiss_assembly.step", Pass),
    ("../step-export/tests/fixtures/lily_lantern.step", Pass),
    ("../step-export/tests/fixtures/loft_prism.step", Pass),
    ("../step-export/tests/fixtures/nonuniform_loft.step", Pass),
    ("../step-export/tests/fixtures/notched.step", Pass),
    (
        "../step-export/tests/fixtures/nurbs_wireframe.step",
        Wireframe,
    ),
    ("../step-export/tests/fixtures/swept_elbow.step", Pass),
    ("../step-export/tests/fixtures/washer.step", Pass),
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

/// Every corpus file's disposition, and the positive tier-validity of
/// every body that ships — at each tolerance in the sweep.
#[test]
fn every_corpus_import_passes_the_shared_gate() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for (eps_tag, eps_in) in [("file", None), ("1e-6", Some(1e-6)), ("1e-12", Some(1e-12))] {
        for (rel, want) in CORPUS {
            let text = std::fs::read_to_string(root.join(rel))
                .unwrap_or_else(|e| panic!("reading {rel}: {e}"));
            let options = ImportOptions { eps_in };
            let who = format!("{rel} @ eps {eps_tag}");
            match (import_step(&text, &options), want) {
                (Ok(StepImport::Solid { body, .. }), Pass) => assert_eq!(
                    topo::validate_geometric(&body),
                    Ok(()),
                    "{who}: the SHIPPED body must be gate-clean — import handed out a \
                     body its own gate refuses, which can only mean the gate is no \
                     longer wired"
                ),
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

/// The gate is wired at exactly ONE place (the #260 ask: make skipping
/// it structurally hard). Import owns no validation logic of its own —
/// no second entry, no kind predicate deciding who is gated, no
/// verdict filter deciding which failures count — so this counts the
/// validator calls in the crate's sources and pins the count at one.
/// A second call is not automatically wrong, but it is exactly the
/// shape the old band-only backstop had, so it must be argued for
/// here rather than appear.
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
    let StepImportError::TierInvalid { errors } = &e else {
        panic!("expected the gate's typed refusal, got: {e:?}");
    };
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
