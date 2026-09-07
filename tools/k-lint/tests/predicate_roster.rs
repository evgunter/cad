//! **[`EPS_COUPLED_PREDICATES`] names predicates the kernel still
//! mints, and their margins are still ε-scaled where it mints them** —
//! the two pins this roster can carry across the cargo-root boundary.
//!
//! # Why the kernel's source is read and not depended on
//!
//! A predicate name is not a type. `geom_core::k_stats::decide` takes a
//! `&'static str`, so the kernel's predicate vocabulary has no
//! enumerable definition to import — unlike `SampleOutcome`, which
//! `tests/outcome_vocabulary.rs` pins variant by variant through the
//! `geom-core` dev-dependency this crate already carries. What is left
//! is the `D204` shape (`tools/tess-meter/tests/derivations.rs`): read
//! the producer's source text, lex it into blanked views, and locate a
//! declaration in one view while reading its literals out of another.
//! Blanked rather than stripped, so every offset means the same byte in
//! both — that property is asserted in [`quad_view`] rather than assumed
//! at each pin.
//!
//! # What these pins CANNOT see
//!
//! **A predicate the kernel ADDS to the ε-coupled class is not on the
//! roster, and nothing here can tell that it should be.** Membership is
//! an explicit allow-list selected from an open vocabulary by a
//! property, and that property is written nowhere a test can evaluate —
//! `work/meter/k-lint-eps-coupled-criterion-unwritten` is where that
//! gap is scheduled, and it is the file to read before widening these
//! pins.
//!
//! That gap is a diagnosis gap, not a hole in the gate. An ε-coupled
//! family missing from the roster stays under rules (2) and (3), where
//! its ε-scaled margins land below `BASELINE_FLOOR_MARGIN` at the tight
//! rows and FLAG — the fail-loud direction the lint's module docs claim
//! and `tests/review_probes.rs`'s
//! `new_eps_coupled_predicate_is_silent_at_1e6_loud_at_tight_rows`
//! measures. `docs/K-REPORT.md`'s "this roster is a RECORD" ruling
//! states the consequence: a roster omission cannot silently weaken the
//! gate. What it CAN do is fire the gate in the wrong voice, whose
//! recourse (`main.rs`) sends the reader to re-derive a baseline that
//! never moved. These pins name the cause where the cause is.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use k_lint::EPS_COUPLED_PREDICATES;
use std::sync::OnceLock;
use test_utils::source;

/// Every kernel source the roster's names are minted in, as
/// `(path, text)`.
///
/// One entry because the roster has one member. A roster entry minted
/// somewhere not listed here REDS
/// [`every_rostered_predicate_is_still_minted_by_the_kernel`] rather
/// than passing quietly, and the fix is a line in this table — the same
/// fail-loud posture the allow-list itself takes.
const MINT_SOURCES: [(&str, &str); 1] = [(
    "crates/geom-brep/src/props/quad.rs",
    include_str!("../../../crates/geom-brep/src/props/quad.rs"),
)];

/// What the pins searched, for their refusals.
const SEARCHED: &str = "the kernel's quadrature mint sites";

/// The call that mints a predicate name in `props/quad.rs`: the file's
/// funnel wrapper over `geom_core::k_stats::decide`, whose first
/// argument IS the name.
const MINT_CALL: &str = "classify_len::<T>(";

/// A view of every [`MINT_SOURCES`] text, lexed once and held in
/// `cache`, concatenated in table order.
///
/// The blanked views are the original's bytes blanked in place, so an
/// offset located in the code view reads the same bytes in the literal
/// one. Concatenating preserves that, since each text's view is exactly
/// as long as the text.
fn quad_view(view: fn(&str) -> String, cache: &'static OnceLock<String>) -> &'static str {
    cache.get_or_init(|| {
        let mut out = String::new();
        for (path, text) in MINT_SOURCES {
            let blanked = view(text);
            assert_eq!(
                blanked.len(),
                text.len(),
                "a blanked view of {path} is the original's bytes, blanked in place"
            );
            out.push_str(&blanked);
        }
        out
    })
}

/// [`MINT_SOURCES`] with prose AND string literals blanked — the view a
/// call is LOCATED in.
fn quad_code() -> &'static str {
    static CODE: OnceLock<String> = OnceLock::new();
    quad_view(source::code_only, &CODE)
}

/// [`MINT_SOURCES`] with prose blanked and literals kept — the view a
/// located literal's VALUE is read out of.
fn quad_literals() -> &'static str {
    static LITERALS: OnceLock<String> = OnceLock::new();
    quad_view(source::code_and_literals, &LITERALS)
}

/// The value of a plain string literal, or `None` where `text` is not
/// one. Nothing is decoded: a `concat!`, a raw string or an escape is
/// not a literal this answers for, and the caller refuses rather than
/// guessing.
fn plain_string_literal(text: &str) -> Option<&str> {
    text.trim()
        .strip_prefix('"')
        .and_then(|q| q.strip_suffix('"'))
}

/// Every [`MINT_CALL`] site in `code`, as `(name, margin_argument)` read
/// out of `literals`.
///
/// **Located over the code view, which is what makes an answer a call.**
/// Over raw text a doc comment quoting the call sorts ahead of the call
/// itself, so the pin would read prose; over `code_only` every
/// occurrence, every bracket and every top-level comma is real code, so
/// `balanced_end` and `top_level_split` ARE the parse. The name is then
/// read from the literal view, where the argument still carries its
/// text.
fn mint_sites(code: &str, literals: &str, searched: &str) -> Vec<(String, String)> {
    assert_eq!(
        code.len(),
        literals.len(),
        "the two views are the same bytes, blanked differently"
    );
    let sites: Vec<(String, String)> = code
        .match_indices(MINT_CALL)
        .map(|(at, _)| {
            let open = at + MINT_CALL.len() - 1;
            let close = source::balanced_end(code, open).expect("the call closes");
            let inner = open + 1..close;
            let args = source::top_level_split(&code[inner.clone()], ',');
            assert!(
                args.len() >= 2,
                "a `{MINT_CALL}` site in {searched} takes {} arguments, not a name and a margin",
                args.len()
            );
            let at_name = inner.start + args[0].start..inner.start + args[0].end;
            let text = literals[at_name].trim();
            let name = plain_string_literal(text).unwrap_or_else(|| {
                panic!(
                    "a `{MINT_CALL}` site in {searched} names {text:?}, not a plain string literal"
                )
            });
            let at_margin = inner.start + args[1].start..inner.start + args[1].end;
            (name.to_string(), code[at_margin].trim().to_string())
        })
        .collect();
    assert!(
        !sites.is_empty(),
        "no `{MINT_CALL}` site in {searched} — the kernel's mint spelling moved and \
         every pin in this file is now vacuous"
    );
    sites
}

/// [`mint_sites`] over the kernel's own views.
fn kernel_mint_sites() -> Vec<(String, String)> {
    mint_sites(quad_code(), quad_literals(), SEARCHED)
}

/// **Every name on the roster is still minted by the kernel** — the
/// direction a name pin closes completely.
///
/// A predicate the kernel RENAMES leaves [`EPS_COUPLED_PREDICATES`]
/// naming nothing: rule (4) stops applying to the family, rules (2) and
/// (3) start, and the gate fires on `props_quad_converged`'s successor
/// at the tight rows with the CLI's recourse pointing at a baseline
/// re-derivation. Nothing moved in the distribution; a name did. This
/// test says so.
#[test]
fn every_rostered_predicate_is_still_minted_by_the_kernel() {
    let minted = kernel_mint_sites();
    let paths: Vec<&str> = MINT_SOURCES.iter().map(|(p, _)| *p).collect();
    for name in EPS_COUPLED_PREDICATES {
        assert!(
            minted.iter().any(|(m, _)| m == name),
            "EPS_COUPLED_PREDICATES rosters {name:?}, which no `{MINT_CALL}` site in {paths:?} \
             mints — either the kernel renamed it (rule 4 now applies to nothing and its family \
             is judged by the metre rules) or it moved to a source this table does not list. \
             minted here: {:?}",
            minted.iter().map(|(m, _)| m).collect::<Vec<_>>()
        );
    }
}

/// **The rostered family's margin is still ε-SCALED where it is
/// minted** — the direction that is otherwise silent, closed for
/// today's roster only.
///
/// Rule (4) exempts the roster from the metre rules because its margin
/// is a headroom against an ε-scaled target, not a model-scale
/// distance. A name pin cannot see that premise fail: re-meter the
/// quadrature stopping test against a fixed length and the name is
/// still minted, the roster still matches, rule (4) still exempts it,
/// and a genuinely fragile model-scale margin is judged by a rule that
/// will essentially never fire on it. So the premise is pinned too,
/// textually: every rostered mint site's margin derives from
/// `target_len`, and every `target_len` in these sources is
/// `QUAD_TARGET_LEN_FACTOR · ε` with that factor a plain finite
/// positive literal.
///
/// **Textual, and for THIS roster only.** It reads the kernel's
/// spelling, so a refactor that keeps the ε-coupling and changes the
/// spelling reds it — deliberately, because a tooling crate cannot tell
/// the two apart and answering either way silently is the failure these
/// pins exist to refuse. It does not generalise to a roster entry the
/// kernel has not minted yet; that wants the criterion
/// `work/meter/k-lint-eps-coupled-criterion-unwritten` schedules.
#[test]
fn the_rostered_familys_margin_is_still_eps_scaled_at_its_mint() {
    for (name, margin) in kernel_mint_sites() {
        if !EPS_COUPLED_PREDICATES.contains(&name.as_str()) {
            continue;
        }
        assert!(
            margin.contains(TARGET),
            "{name} is rostered ε-coupled and its margin at the mint reads {margin:?}, which \
             does not derive from `{TARGET}` — rule (4)'s premise is that this margin is a \
             headroom against an ε-scaled target"
        );
    }
    let code = quad_code();
    let bindings = initializers(code, TARGET_BINDING);
    assert!(
        !bindings.is_empty(),
        "no `{TARGET_BINDING}` in {SEARCHED} — the pin above reads a name nothing defines"
    );
    for at in bindings {
        assert_eq!(
            code[at].trim(),
            TARGET_VALUE,
            "a `{TARGET}` in {SEARCHED} is not {TARGET_VALUE:?} — rule (4)'s exemption assumes \
             this family's margin scales with ε"
        );
    }
    let factor: f64 = code[sole_initializer(code, SEARCHED, FACTOR_DECL)]
        .trim()
        .parse()
        .expect("QUAD_TARGET_LEN_FACTOR is a float literal");
    assert!(
        factor.is_finite() && factor > 0.0,
        "QUAD_TARGET_LEN_FACTOR is {factor}, so the target is not a positive multiple of ε"
    );
}

/// The `let` the ε-scaled target is bound by, the value it must have,
/// and the constant that value names.
const TARGET: &str = "target_len";
const TARGET_BINDING: &str = "let target_len =";
const TARGET_VALUE: &str = "QUAD_TARGET_LEN_FACTOR * eps";
const FACTOR_DECL: &str = "const QUAD_TARGET_LEN_FACTOR: f64 = ";

/// Every initializer following `decl` in `view`: from the end of the
/// declaration head to the `;` that closes it. The closing `;` is
/// sought in the same blanked view, so one inside a string cannot end
/// the statement early.
fn initializers(view: &str, decl: &str) -> Vec<std::ops::Range<usize>> {
    view.match_indices(decl)
        .map(|(at, _)| {
            let start = at + decl.len();
            let end = start + view[start..].find(';').expect("the declaration ends");
            start..end
        })
        .collect()
}

/// The ONE initializer `decl` has in `view`. A second declaration of the
/// same name is an ambiguity a textual pin cannot resolve, and answering
/// with either silently is what this refuses.
fn sole_initializer(view: &str, searched: &str, decl: &str) -> std::ops::Range<usize> {
    let mut found = initializers(view, decl);
    assert!(
        found.len() == 1,
        "`{decl}` is declared {} times in {searched}, not once",
        found.len()
    );
    found.remove(0)
}

/// The locator reads a mint and not prose about it, and a roster the
/// kernel does not mint reds.
///
/// Both failures would otherwise be silent greens. A doc comment and a
/// quoted string spelling the same call sort ahead of the call itself,
/// and over the code view neither can answer; a name absent from the
/// mints must red, so the shortfall is constructed here rather than
/// assumed.
#[test]
fn the_pin_reads_a_mint_and_a_missing_name_reds_it() {
    let decoyed = concat!(
        "/// classify_len::<T>(\"decoy_doc\", Margin::of(target_len - w), band)\n",
        "const Q: &str = \"classify_len::<T>(\\\"decoy_str\\\", Margin::of(x), band)\";\n",
        "    if classify_len::<T>(\n        \"props_quad_converged\",\n",
        "        Margin::of(target_len - width_len),\n        band,\n    )? == Sign::Positive\n",
        "    { classify_len::<T>(\"props_quad_face_extent\", Margin::over_lever(a, p), band)?; }\n"
    );
    assert_eq!(
        decoyed.matches(MINT_CALL).count(),
        4,
        "four spellings in the text"
    );
    let sites = mint_sites(
        &source::code_only(decoyed),
        &source::code_and_literals(decoyed),
        "the decoy fixture",
    );
    let names: Vec<&str> = sites.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(
        names,
        ["props_quad_converged", "props_quad_face_extent"],
        "only the two real calls answer"
    );
    assert_eq!(sites[0].1, "Margin::of(target_len - width_len)");

    let short = source::code_only("let x = classify_len::<T>(\"other_name\", Margin::of(m), b);");
    let shortl =
        source::code_and_literals("let x = classify_len::<T>(\"other_name\", Margin::of(m), b);");
    let minted = mint_sites(&short, &shortl, "the short fixture");
    let missing: Vec<&str> = EPS_COUPLED_PREDICATES
        .iter()
        .copied()
        .filter(|n| !minted.iter().any(|(m, _)| m == n))
        .collect();
    assert_eq!(
        missing,
        ["props_quad_converged"],
        "the containment separates a mint short a rostered name from a complete one"
    );
}
