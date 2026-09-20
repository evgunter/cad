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
//! The rows below drive that through the public doors. The census at
//! the end says a narrower thing than it looks: no file under this
//! crate's `src` still SPELLS an `Indeterminate` literal outside a test
//! body. That closes the eight sites' own shape, and it is a census
//! over one spelling in one crate — a mint through `sign_within` plus
//! `with_predicate`, through an alias, or from a helper one module over
//! walks straight past it. Its own doc comment lists the routes, each
//! one executed rather than imagined.
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

/// **The structural half, at the eight sites: this crate spells no
/// `Indeterminate` literal of its own.** Every escalation `geom-brep`
/// hands a caller comes back from a funnel door, which is the same call
/// that put it on the frame's log — so "does the log hold it" is not a
/// property each of those sites has to remember.
///
/// **What this row is NOT.** It is a census over one spelling in one
/// crate, not a proof that the defect cannot return. The blind spots
/// below are executed, not hypothetical — the first two were reproduced
/// by the review of the change that added this row:
///
/// - **The raw classifier route.** `margin.sign_within(band)` followed
///   by `.with_predicate(name)` mints the defect's exact payload with no
///   `Indeterminate` token anywhere in the file. Nothing here sees it.
/// - **The same shape through `k_stats::decide` and a hand-built
///   error.** Ask the funnel, match `Positive`, build the refusal — with
///   the payload assembled anywhere but at the call site, this row is
///   silent and the log is empty.
/// - **An alias or a helper.** `use geom_core::Indeterminate as Diag;`,
///   or a constructor in another module whose return value is the mint.
/// - **A file this walk does not read**: one reached by `include!` or
///   `#[path]`, and every crate other than this one.
/// - **The `#[cfg(test)]` exclusion is ITEM-shaped.** A test-only helper
///   gated some other way (a cargo feature, a `cfg(debug_assertions)`)
///   reads as shipped and reds this row. That direction is loud, which
///   is the one it should be.
#[test]
fn shipped_geom_brep_code_spells_no_indeterminate_literal() {
    let src = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut mints = Vec::new();
    for path in test_utils::source::rust_sources(&src) {
        let text = std::fs::read_to_string(&path).expect("a readable source file");
        let rel_path = path.strip_prefix(&src).unwrap_or(&path);
        for at in shipped_mints(&test_utils::source::code_only(&text)) {
            mints.push(format!(
                "{}:{}",
                rel_path.display(),
                test_utils::source::line(&text, at)
            ));
        }
    }
    assert!(
        mints.is_empty(),
        "shipped geom-brep code builds an `Indeterminate` directly instead of taking one back \
         from a funnel door, so the escalation it hands its caller is on no frame's log: {mints:?}"
    );
}

/// **The scanner, exercised on text this file owns** — so a change that
/// broke it reds here instead of reading as a clean tree.
///
/// Each case is one line of the contract: the shipped mint is found, a
/// fixture's is not, a brace-less `#[cfg(test)]` item does not carry its
/// exclusion into the shipped code after it, and a signature that NAMES
/// the type is not a construction of it.
#[test]
fn the_mint_scanner_finds_a_shipped_literal_and_nothing_else() {
    let found = |src: &str| shipped_mints(&test_utils::source::code_only(src)).len();

    assert_eq!(
        found("fn f() -> R { Err(Indeterminate { margin: m, band, predicate: None }) }"),
        1,
        "a shipped mint"
    );
    assert_eq!(
        found("#[cfg(test)]\nmod tests {\n fn g() { let _ = Indeterminate { band }; }\n}"),
        0,
        "a fixture inside a `#[cfg(test)]` body is not a shipped mint"
    );
    assert_eq!(
        found("#[cfg(test)]\nmod tests;\nfn f() { let _ = Indeterminate { band }; }"),
        1,
        "a BRACE-LESS `#[cfg(test)]` item has no body, so its exclusion must not run over \
         the next shipped one — the regression this case pins"
    );
    assert_eq!(
        found("#[cfg(test)]\nconst N: [u8; 2] = [0, 0];\nfn f() { let _ = Indeterminate { b }; }"),
        1,
        "a `;` inside brackets does not terminate the item either"
    );
    assert_eq!(
        found("fn f(x: T) -> Indeterminate {\n    g(x)\n}"),
        0,
        "a return type NAMES the type; it does not construct one"
    );
    assert_eq!(
        found("impl Indeterminate {\n    fn f() {}\n}"),
        0,
        "nor does an inherent impl block"
    );
}

/// The byte offset of every `Indeterminate { … }` struct literal in
/// `code` (a code-only view) that is not inside a `#[cfg(test)]` item.
fn shipped_mints(code: &str) -> Vec<usize> {
    let fixtures = cfg_test_bodies(code);
    let mut out = Vec::new();
    let mut at = 0usize;
    while let Some(rel) = code[at..].find("Indeterminate") {
        let start = at + rel;
        at = start + "Indeterminate".len();
        if !test_utils::source::boundary_before(code, start) {
            continue;
        }
        if !code[at..].trim_start().starts_with('{') {
            continue;
        }
        // `-> Indeterminate {`, `impl Indeterminate {`, `struct …` and
        // friends NAME the type where a literal CONSTRUCTS one, and a
        // `{` follows either way.
        let before = code[..start].trim_end();
        let names_rather_than_builds = before.ends_with("->")
            || ["impl", "struct", "enum", "union", "for"].iter().any(|kw| {
                before.ends_with(kw)
                    && test_utils::source::boundary_before(before, before.len() - kw.len())
            });
        if names_rather_than_builds || fixtures.iter().any(|r| r.contains(&start)) {
            continue;
        }
        out.push(start);
    }
    out
}

/// The byte range of every `#[cfg(test)]` item's body in `code`.
///
/// **An item's body is found by its TERMINATOR, not by the next `{`.**
/// A brace-less item — `mod tests;`, a `use`, a `const` — ends at a `;`
/// and has no body at all; taking the next `{` for one silently ran the
/// exclusion over the following SHIPPED item, which is a guard that
/// stops guarding without saying so. `;` and `{` are both read at
/// bracket depth zero, so a `;` inside a type (`[u8; 2]`) terminates
/// nothing.
fn cfg_test_bodies(code: &str) -> Vec<std::ops::Range<usize>> {
    let mut out = Vec::new();
    let mut at = 0usize;
    while let Some(rel) = code[at..].find("#[cfg(test)]") {
        let start = at + rel;
        at = start + "#[cfg(test)]".len();
        let mut depth = 0i32;
        let mut open = None;
        for (i, c) in code[at..].char_indices() {
            match c {
                '(' | '[' => depth += 1,
                ')' | ']' => depth -= 1,
                ';' if depth <= 0 => break,
                '{' if depth <= 0 => {
                    open = Some(at + i);
                    break;
                }
                _ => {}
            }
        }
        let Some(open) = open else { continue };
        if let Some(close) = test_utils::source::balanced_end(code, open) {
            out.push(open..close);
            at = close;
        }
    }
    out
}
