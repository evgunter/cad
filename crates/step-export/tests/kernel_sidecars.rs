//! The KERNEL_* sidecar staleness row: every committed SOLID-fixture
//! `.expect` sidecar's `KERNEL_SOLIDS` / `KERNEL_SHELLS` /
//! `KERNEL_FACES` / `KERNEL_EDGES` / `KERNEL_VERTICES` /
//! `KERNEL_VOLUME_MM3` / `KERNEL_VOLUME_PAD_MM3` fields are asserted
//! against the LIVE kernel census and certified volume of the source
//! body (`common::fixture_corpus()` rebuilds all 17), so the fields
//! can never rot. `nurbs_wireframe.expect` carries no KERNEL_* fields
//! and is out of scope here: a wireframe has no census or volume to
//! pin, as its own sidecar states.
//!
//! Semantics: `KERNEL_*` is the NATIVE body's census — what the kernel
//! actually has — as opposed to the `EXPECT_*` fields, which record
//! what FreeCAD/OCC *reports after importing* the exported file (OCC
//! adds degenerate pole edges and splits periodic carriers at seams —
//! normalisations D7 forbids the kernel to mirror, so the two censuses
//! legitimately differ on several fixtures). `scripts/check_step.sh`
//! consumes `EXPECT_*` only; `crates/step-import`'s committed-corpus
//! row consumes `KERNEL_*`. The one structural divergence beyond edge
//! normalisation is `kiss_assembly`: natively ONE solid with TWO
//! shells, but STEP has no way to group two closed shells into one
//! solid (`MANIFOLD_SOLID_BREP` carries exactly one), so both the OCC
//! and the kernel-import readings are 2 solids / 2 shells — the
//! documented divergence IS the semantics, see the sidecar's comment.
//!
//! `KERNEL_VOLUME_MM3` / `KERNEL_VOLUME_PAD_MM3` are FULL precision:
//! each committed literal must be byte-identical to
//! `step_export::fmt_real`'s output for the live certified value
//! (× 1e9 mm³/m³), and that printer's pinned property is that its
//! output parses back to the identical f64 bits — so the byte equality
//! asserted here IS the bit-exact round-trip.
//!
//! The volume pair is pinned AT THE CORPUS'S DECLARED UNCERTAINTY
//! (ε = 1e-9 — the explicit `uncertainty_m` every fixture is exported
//! with): a quadrature body's certified enclosure is a FUNCTION of
//! ambient ε (`cut_cylinder`'s midpoint moves by ~0.2 mm³ between
//! ε = 1e-9 and 1e-12 and by ~2000 mm³ at 1e-6, always inside its own
//! pad), so byte equality is only defined at the ε the literal was
//! printed at. On every other ε row this suite still asserts the real
//! cross-ε claim: two certified enclosures of the SAME body's true
//! volume must overlap — `|live − pinned| ≤ live_pad + pinned_pad`.
//! The census fields are ε-independent and byte-asserted everywhere.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

/// The ambient ε the sidecars' volume literals are printed at — the
/// corpus's declared uncertainty (`uncertainty_m: Some(1e-9)` in every
/// fixture-writing call; see `tests/export.rs`'s `export` helper).
const CORPUS_EPS: f64 = 1e-9;

/// The staleness row (17/17): rebuilds every corpus body and checks
/// each committed sidecar's KERNEL_* lines against the live values. On
/// failure the message prints the full expected block per fixture —
/// paste-ready for regeneration after a deliberate builder change
/// (regenerate at DEFAULT ε: the volume literals are defined at the
/// corpus's declared 1e-9, module docs).
#[test]
fn kernel_sidecar_fields_match_live_kernel() {
    let at_corpus_eps = geom_core::Tol::witness().get().eps == CORPUS_EPS;
    let mut failures: Vec<String> = Vec::new();
    for (name, body) in common::fixture_corpus() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(format!("{name}.expect"));
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("reading {}: {e}", path.display()));
        let field = |key: &str| -> Vec<&str> {
            text.lines()
                .filter_map(|l| l.strip_prefix(&format!("{key}=")))
                .collect()
        };
        let props =
            topo::mass_properties(&body, Tol::witness()).unwrap_or_else(|e| panic!("{name}: mass properties: {e}"));
        let print = |value: f64, what: &'static str| {
            step_export::fmt_real(value * 1e9, what)
                .unwrap_or_else(|e| panic!("{name}: printing {what}: {e}"))
        };
        let mut want = vec![
            ("KERNEL_SOLIDS", body.solids().count().to_string()),
            ("KERNEL_SHELLS", body.shells().count().to_string()),
            ("KERNEL_FACES", body.faces().count().to_string()),
            ("KERNEL_EDGES", body.edges().count().to_string()),
            ("KERNEL_VERTICES", body.vertices().count().to_string()),
        ];
        if at_corpus_eps {
            want.push((
                "KERNEL_VOLUME_MM3",
                print(props.volume, "KERNEL_VOLUME_MM3"),
            ));
            want.push((
                "KERNEL_VOLUME_PAD_MM3",
                print(props.volume_pad, "KERNEL_VOLUME_PAD_MM3"),
            ));
        } else {
            // Off the corpus ε the byte pin is out of its domain
            // (module docs); the enclosure-overlap claim is the true
            // cross-ε invariant, asserted for real.
            let pin = |key: &str| -> f64 {
                field(key)
                    .first()
                    .unwrap_or_else(|| panic!("{name}.expect: missing {key}= line"))
                    .parse()
                    .unwrap_or_else(|e| panic!("{name}.expect: {key} not a float: {e}"))
            };
            let pinned = pin("KERNEL_VOLUME_MM3");
            let pinned_pad = pin("KERNEL_VOLUME_PAD_MM3");
            let dev = (props.volume * 1e9 - pinned).abs();
            let budget = props.volume_pad * 1e9 + pinned_pad;
            if dev > budget {
                failures.push(format!(
                    "{name}: live enclosure {} ± {} mm³ does not overlap the pinned \
                     {pinned} ± {pinned_pad} mm³ (both certify the same true volume)",
                    props.volume * 1e9,
                    props.volume_pad * 1e9,
                ));
            }
            println!(
                "{name}: ambient ε off the corpus's declared 1e-9 — volume byte-pin \
                 out of domain, enclosure overlap asserted (dev {dev} ≤ {budget} mm³)"
            );
        }
        for (key, value) in want {
            // Exactly one line per key, byte-equal to the live value
            // (for the volume pair, byte equality against the
            // printer's output is the bit-exact round-trip — module
            // docs).
            if field(key) != [value.as_str()] {
                let lines = field(key);
                failures.push(format!("{name}: want {key}={value}, sidecar has {lines:?}"));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "stale or missing KERNEL_* sidecar fields (live kernel census/volume vs \
         committed .expect):\n{}",
        failures.join("\n")
    );
}
use geom_core::Tol;
