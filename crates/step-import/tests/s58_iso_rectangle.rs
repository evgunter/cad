//! **S58 / #649: the two proven doors to a wrongly-measured body, both
//! closed.**
//!
//! Before this unit, `props`' rim-group span-SUM rule accepted a
//! cross-shaped iso domain on a cylindrical wall, and
//! `topo::mass_properties` then returned a volume **19% low with
//! `volume_pad = 0.0`** — a certificate of exactness on a wrong number,
//! with tier-3 `validate_geometric` green. Two doors reached it, both
//! executed in #649 and both pinned here:
//!
//! * **STEP import** — `cross.step` is a real, manifold, closed,
//!   tier-3-clean keyed shaft (24 V / 36 E / 14 F, χ = 2) whose two
//!   cylindrical walls have a plus-shaped domain. It imported and
//!   measured wrong. It now refuses at import's own at-rest gate.
//! * **`Body::merge_coplanar_faces()`** — the worse of the two, and the
//!   reason this suite exists rather than a note in `tier_gate`: a
//!   PUBLIC composition of Euler operators that moves no geometry and
//!   conserves χ turned a body measuring EXACTLY 8.96e-7 m³ into the
//!   same body measuring 7.2533e-7. `xsplit.step` is that same solid
//!   authored with each wall as three rectangular sub-faces on one
//!   shared cylindrical surface; merging them is what builds the plus.
//!
//! The fixtures are #649's own, committed here with their generator
//! (`fixtures/iso-rect/gen_iso_rect.py`). `rect.step` is the CONTROL —
//! the same annular sector with no arms — and it must still import and
//! still measure EXACTLY, so a rule that refused everything could not
//! make this suite green.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use step_import::{ImportOptions, StepImport, import_step};
use geom_core::Tol;

fn fixture(name: &str) -> String {
    let path = format!(
        "{}/tests/fixtures/iso-rect/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading {path}: {e}"))
}

/// The generator's parameters (mm in the file, SI here): R = 10,
/// RI = 6, H = 20, Z1 = 6, Z2 = 14, UC = 0.5 rad, UO = 1.0 rad.
const DR2: f64 = 0.010 * 0.010 - 0.006 * 0.006;
/// Volume of an annular sector of angular width `du` and height `h`.
fn ring(du: f64, h: f64) -> f64 {
    0.5 * du * DR2 * h
}

/// `rect.step`: one annular sector, `u ∈ [−0.5, 0.5]`, full height.
const RECT_VOLUME: f64 = 6.4e-7;
/// `cross.step` / merged `xsplit.step`: the three-layer stack.
fn cross_volume() -> f64 {
    ring(1.0, 0.006) + ring(2.0, 0.008) + ring(1.0, 0.006)
}

/// **The control.** A genuine iso-rectangle domain still imports and
/// still measures exactly, pad 0 — the closed-form lane is untouched.
#[test]
fn rect_the_control_still_measures_exactly() {
    let Ok(StepImport::Solid { body, .. }) =
        import_step(&fixture("rect.step"), &ImportOptions::default())
    else {
        panic!("rect.step (the CONTROL) must still import as a solid");
    };
    topo::validate_geometric(&body, Tol::witness()).expect("rect.step must still pass tier 3");
    let mp = topo::mass_properties(&body, Tol::witness()).expect("rect.step must still measure");
    assert_eq!(mp.volume_pad, 0.0, "the closed-form lane's pad is 0");
    let rel = (mp.volume - RECT_VOLUME).abs() / RECT_VOLUME;
    assert!(
        rel < 1e-12,
        "rect.step volume {:.15e} != exact {RECT_VOLUME:.15e} (rel {rel:.3e})",
        mp.volume
    );
}

/// **Door 1: STEP import.** The cross-shaped domain is refused at the
/// at-rest gate rather than measured 19% low. The refusal names the one
/// predicate, so a regression that re-widens the rule (rather than
/// merely moving the refusal) fails this row.
#[test]
fn cross_the_649_fixture_refuses_at_import() {
    let err = match import_step(&fixture("cross.step"), &ImportOptions::default()) {
        Err(e) => e.to_string(),
        Ok(other) => {
            panic!("cross.step imported ({other:?}) — #649's 19%-low certified volume is back")
        }
    };
    assert!(
        err.contains("props_rim_level"),
        "the refusal must name the iso-rectangle predicate, got: {err}"
    );
}

/// The one-sided variant, refused before this unit by the span-sum rule
/// (its group sums differ) and refused now by the level rule. Kept as
/// the reminder that the sum rule caught the EASY shape and that is
/// what made #649 look covered.
#[test]
fn tee_the_one_sided_variant_still_refuses() {
    assert!(
        import_step(&fixture("tee.step"), &ImportOptions::default()).is_err(),
        "tee.step must stay refused"
    );
}

/// **Door 2: `Body::merge_coplanar_faces()`.** The same solid authored
/// with rectangular sub-faces measures EXACTLY. The public merge then
/// fuses the same-surface-key wall fragments into plus-shaped faces —
/// no geometry moves, χ is conserved — and the result must now REFUSE
/// rather than certify 7.2533e-7 with pad 0.
#[test]
fn merge_coplanar_faces_no_longer_turns_an_exact_body_into_a_wrong_one() {
    let Ok(StepImport::Solid { mut body, .. }) =
        import_step(&fixture("xsplit.step"), &ImportOptions::default())
    else {
        panic!("xsplit.step (rectangular sub-faces) must import as a solid");
    };
    let exact = cross_volume();
    let before = topo::mass_properties(&body, Tol::witness()).expect("the sub-faced body measures");
    let rel = (before.volume - exact).abs() / exact;
    assert!(
        rel < 1e-12 && before.volume_pad == 0.0,
        "BEFORE merge: {:.15e} (pad {:.3e}) != exact {exact:.15e}",
        before.volume,
        before.volume_pad
    );

    let out = body
        .merge_coplanar_faces(Tol::witness())
        .expect("the merge itself is a legal Euler-op composition and still runs");
    // Exactly the two cylindrical walls, each of whose three
    // rectangular sub-faces shares one SurfaceKey. `is_empty()` alone
    // would stay green if the merge found one wall, or found something
    // else entirely; the count is what says it did the thing this row
    // is about. (The load-bearing assertion is still the refusal
    // below — this one only keeps a no-op merge from making it
    // vacuous.)
    assert_eq!(
        out.groups.len(),
        2,
        "the merge must fuse exactly the two cylindrical walls, got {:?}",
        out.groups
    );

    // The door: a wrong number is no longer available on the other side.
    match topo::mass_properties(&body, Tol::witness()) {
        Err(e) => {
            let s = format!("{e:?}");
            assert!(
                s.contains("props_rim_level"),
                "the refusal must name the iso-rectangle predicate, got: {s}"
            );
        }
        Ok(mp) => panic!(
            "AFTER merge the body still measured: {:.15e} (exact {exact:.15e}, pad {:.3e}) \
             — #649's second door is open again",
            mp.volume, mp.volume_pad
        ),
    }
}
