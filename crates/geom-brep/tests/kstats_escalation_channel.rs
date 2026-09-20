//! What the verdict bracket's escalation channel carries, pinned at the
//! ops that used to mint outside it.
//!
//! A predicate whose question is only validly posed under a condition on
//! the margin — a lever arm that must be definitely positive, a
//! discriminant that must be definitely nonzero — states that condition
//! by choosing a GATE door (`k_stats::decide_positive`,
//! `decide_nonzero`, `gate_measured`). The gate's refusal is therefore
//! the funnel's own escalation, recorded on the open frame beside the
//! definite verdict the classifier reached, and no op mints an
//! `Indeterminate` its caller's log cannot see.
//!
//! The rows below drive that through the public doors; the census at the
//! end is the structural half — this crate's shipped code holds no
//! hand-built `Indeterminate` at all, so there is no site left that
//! could forget.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_brep::{
    OutwardNormal, ReferenceNormal, classify_dihedral, classify_material_pairing, enters_material,
    enters_material_order2,
};
use geom_core::k_stats::{Bracket, Recorded};
use geom_core::{Band, MarginDiag, Point3, Sign, Tol, Vec3};

/// The one escalation `recorded` holds, as `(predicate, margin)`.
fn sole_escalation(recorded: &Recorded) -> (&'static str, MarginDiag) {
    assert_eq!(
        recorded.escalations.len(),
        1,
        "one gated rejection records one escalation: {:?}",
        recorded.escalations
    );
    let e = &recorded.escalations[0];
    (e.predicate(), e.source.margin)
}

/// The verdicts, as `(predicate, sign)` pairs.
fn verdicts(recorded: &Recorded) -> Vec<(&'static str, Sign)> {
    recorded
        .verdicts
        .iter()
        .map(|v| (v.predicate, v.sign))
        .collect()
}

/// **The row the escalation channel's hole was pinned by.**
///
/// `enters_material` asks the funnel for its lever arm and receives a
/// DEFINITE `Zero`. The collapsed arm means no angular verdict can be
/// metered here, so the predicate escalates — and because the
/// requirement travels into the funnel rather than being applied to
/// `decide`'s answer afterwards, the frame holds BOTH the definite
/// verdict the classifier reached and the escalation the gate produced.
#[test]
fn an_indeterminate_minted_after_a_definite_verdict_is_on_the_escalation_log() {
    let band = Band::linear(Tol::witness()).unwrap();
    let bracket = Bracket::open();
    let out = enters_material(
        Vec3::new(1.0f64, 0.0, 0.0),
        OutwardNormal::from_chart(Vec3::new(0.0f64, 0.0, 1.0), true),
        // A collapsed lever arm: the funnel decides Zero DEFINITELY.
        0.0f64,
        band,
    );
    let recorded = bracket.finish();
    let escalated = out.expect_err("the predicate escalates to its caller");
    assert_eq!(escalated.predicate, Some("enters_material_arm"));
    assert_eq!(escalated.margin, MarginDiag::Invalid);
    assert_eq!(
        verdicts(&recorded),
        [("enters_material_arm", Sign::Zero)],
        "the frame still holds the funnel's definite verdict"
    );
    assert_eq!(
        sole_escalation(&recorded),
        ("enters_material_arm", MarginDiag::Invalid),
        "and the escalation the caller received is on the log beside it"
    );
}

/// The second-order sector descent's own collapsed-arm gate, the same
/// shape one order up.
#[test]
fn the_order2_sector_arm_gate_records_its_escalation() {
    let band = Band::linear(Tol::witness()).unwrap();
    let bracket = Bracket::open();
    let out = enters_material_order2(
        Vec3::new(0.0f64, 0.0, 1.0),
        1.0f64,
        ReferenceNormal::of_split_plane(Vec3::new(0.0f64, 0.0, 1.0)),
        0.0f64,
        band,
    );
    let recorded = bracket.finish();
    assert_eq!(
        out.expect_err("a collapsed arm escalates").predicate,
        Some("tangent_sector_order2_arm")
    );
    assert_eq!(
        verdicts(&recorded),
        [("tangent_sector_order2_arm", Sign::Zero)]
    );
    assert_eq!(
        sole_escalation(&recorded),
        ("tangent_sector_order2_arm", MarginDiag::Invalid)
    );
}

fn plane(normal: Vec3<f64>, u_ref: Vec3<f64>) -> Surface<f64> {
    Surface::Plane {
        origin: Point3::origin(),
        normal,
        u_ref,
    }
}

/// The wedge predicate's folded lever arm, collapsed by a zero extent.
#[test]
fn the_dihedral_arm_gate_records_its_escalation() {
    let band = Band::linear(Tol::witness()).unwrap();
    let s1 = plane(Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
    let s2 = plane(Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    let bracket = Bracket::open();
    let out = classify_dihedral(&s1, &s2, Point3::origin(), 0.0f64, band);
    let recorded = bracket.finish();
    assert_eq!(
        out.expect_err("a collapsed arm escalates").predicate,
        Some("dihedral_arm")
    );
    assert_eq!(verdicts(&recorded), [("dihedral_arm", Sign::Zero)]);
    assert_eq!(
        sole_escalation(&recorded),
        ("dihedral_arm", MarginDiag::Invalid)
    );
}

/// The material-pairing discriminant, definitely zero: perpendicular
/// outward normals name no side, so the gate that admits only a nonzero
/// sign escalates — and records it.
#[test]
fn the_material_pairing_gate_records_its_escalation() {
    let band = Band::linear(Tol::witness()).unwrap();
    let s_plus = plane(Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
    let s_minus = plane(Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    let bracket = Bracket::open();
    let out = classify_material_pairing(
        &s_plus,
        true,
        &s_minus,
        true,
        Point3::origin(),
        1.0f64,
        band,
    );
    let recorded = bracket.finish();
    assert_eq!(
        out.expect_err("a zero discriminant escalates").predicate,
        Some("material_wedge_side")
    );
    assert_eq!(verdicts(&recorded), [("material_wedge_side", Sign::Zero)]);
    assert_eq!(
        sole_escalation(&recorded),
        ("material_wedge_side", MarginDiag::Invalid)
    );
}

/// **The structural half: this crate mints no `Indeterminate` of its
/// own.** Every escalation `geom-brep` hands a caller now comes back
/// from a funnel door, which is the same call that put it on the frame's
/// log — so "does the log hold it" is not a property each site has to
/// remember, and a new site cannot reintroduce the gap without this row
/// going red.
///
/// **Blind spot**, stated rather than implied: the scan reads
/// `crates/geom-brep/src/**/*.rs` with comments and string literals
/// blanked, and excludes the body of every `#[cfg(test)]` item — a
/// fixture is not a shipped mint. It matches the struct literal
/// `Indeterminate {`, so a mint spelled through a helper that builds one
/// elsewhere, through a type alias, or in a file reached by `include!`
/// is invisible to it. It is a guard on the spelling the eight sites
/// used, not a proof about the crate.
#[test]
fn shipped_geom_brep_code_builds_no_indeterminate_of_its_own() {
    let src = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut mints = Vec::new();
    for path in test_utils::source::rust_sources(&src) {
        let text = std::fs::read_to_string(&path).expect("a readable source file");
        let code = test_utils::source::code_only(&text);
        let fixtures = cfg_test_bodies(&code);
        let mut at = 0usize;
        while let Some(rel) = code[at..].find("Indeterminate") {
            let start = at + rel;
            at = start + "Indeterminate".len();
            if !test_utils::source::boundary_before(&code, start) {
                continue;
            }
            if code[at..].trim_start().starts_with('{')
                && !fixtures.iter().any(|r| r.contains(&start))
            {
                let rel_path = path.strip_prefix(&src).unwrap_or(&path);
                mints.push(format!(
                    "{}:{}",
                    rel_path.display(),
                    test_utils::source::line(&text, start)
                ));
            }
        }
    }
    assert!(
        mints.is_empty(),
        "shipped geom-brep code builds an `Indeterminate` directly instead of taking one back \
         from a funnel door, so the escalation it hands its caller is on no frame's log: {mints:?}"
    );
}

/// The byte range of every `#[cfg(test)]` item's body in `code`.
fn cfg_test_bodies(code: &str) -> Vec<std::ops::Range<usize>> {
    let mut out = Vec::new();
    let mut at = 0usize;
    while let Some(rel) = code[at..].find("#[cfg(test)]") {
        let start = at + rel;
        at = start + "#[cfg(test)]".len();
        let Some(open) = code[at..].find('{').map(|o| at + o) else {
            continue;
        };
        if let Some(close) = test_utils::source::balanced_end(code, open) {
            out.push(open..close);
            at = close;
        }
    }
    out
}
