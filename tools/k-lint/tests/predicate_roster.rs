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
//! both — `test_utils::source::blanked` is where that precondition is
//! refused, for every pin of this shape at once.
//!
//! **Source text is the instrument the fence left, not the one the
//! problem asked for.** A `#[test]` inside `geom-brep` asserting its
//! own minted names, or a `const` slice exported from
//! `geom_core::k_stats` that this roster imports and no longer has to
//! parse, would each be cheaper and neither needs a lexer; both are
//! edits under `crates/`, which is PROPS' seam and not METER's.
//! `work/props/k-lint-roster-wants-a-kernel-side-vocabulary` carries
//! that trade with both shapes written out.
//!
//! # What these pins can and cannot see
//!
//! Over [`MINT_SOURCES`], and only over them, they see:
//!
//! - **every rostered name is still minted** — a rename reds
//!   [`every_rostered_predicate_is_still_minted_by_the_kernel`];
//! - **every rostered mint's margin still derives from an ε-scaled
//!   `target_len`**, with the identifier matched WHOLE, the binding
//!   found in the mint's own enclosing function, and that binding's
//!   value `QUAD_TARGET_LEN_FACTOR * eps` with the factor finite and
//!   positive;
//! - **every mint whose margin derives from the ε-scaled target is
//!   rostered** — a margin naming `target_len`, `interim_resolution` or
//!   `QUAD_TARGET_LEN_FACTOR` — or excused by name in
//!   `EPS_COUPLED_UNRULED` (empty today): the ADDED direction, closed
//!   for those spellings in these sources and for nothing wider;
//! - **every funnel call in these sources is a parsed mint or the
//!   function's own declaration** (`classify_len` in the lanes,
//!   `decide_reported` in topo's measurement door) — a site respelled
//!   out of the parse reds rather than dropping quietly out of the
//!   population every pin above quantifies over.
//!
//! "These sources" is their KERNEL code: each file's
//! `#[cfg(test)] mod tests` is blanked out of the views first
//! ([`kernel_only`]), so a test calling the funnel is not a mint.
//!
//! They cannot see:
//!
//! - **a predicate the kernel adds to the ε-coupled class by a route
//!   that is not `target_len`.** Membership is a property of the
//!   margin, that property is written nowhere a test can evaluate over
//!   a name, and `target_len` is this family's spelling for it rather
//!   than the criterion —
//!   `work/instr/k-lint-eps-coupled-criterion-unwritten` is where the
//!   criterion is scheduled;
//! - **anything outside [`MINT_SOURCES`].** A rostered mint that moves
//!   to an unlisted file reds the name pin; a NEW ε-coupled family
//!   minted in an unlisted file is invisible, and the table is
//!   hand-maintained;
//! - **whether a `target_len` passed in as a PARAMETER is ε-scaled.**
//!   That is the caller's property and this walk does not follow it, so
//!   a rostered mint of that shape is REFUSED rather than answered;
//! - **a refactor that keeps the ε-coupling and changes the spelling.**
//!   That reds too, for the same reason: a tooling crate cannot tell it
//!   from a de-coupling, and answering either way silently is the
//!   failure these pins exist to refuse.
//!
//! The first blind spot is a diagnosis gap, not a hole in the gate. An
//! ε-coupled family missing from the roster stays under rules (2) and
//! (3), where its ε-scaled margins land below `BASELINE_FLOOR_MARGIN`
//! at the tight rows and FLAG — the fail-loud direction the lint's
//! module docs claim and `tests/review_probes.rs`'s
//! `new_eps_coupled_predicate_is_silent_at_1e6_loud_at_tight_rows`
//! measures. What a roster omission CAN do is fire the gate in the
//! wrong voice, whose recourse (`main.rs`) sends the reader to
//! re-derive a baseline that never moved. These pins name the cause
//! where the cause is.
//!
//! # Running them
//!
//! **`cargo test` in this crate does not run the whole obligation.**
//! `tools/k-lint` is excluded from the workspace, and reading kernel
//! source through `test_utils::source` puts this file in
//! `crates/test-utils/tests/reader_census.rs`'s ledger — a suite that
//! walks the repo root and is structurally unreachable from a cargo
//! invocation rooted here. A local run covering this file is `cargo
//! test` here AND `cargo test -p test-utils --test reader_census` from
//! the workspace root. The hosted matrix runs both.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use k_lint::{EPS_COUPLED_PREDICATES, EPS_COUPLED_UNRULED, ERA_EPS_COUPLED_PREDICATES};
use std::sync::OnceLock;
use test_utils::source;

/// **How a source spells its mints**: the funnel call a mint is, the
/// call's bare name and its declaration head (where the source declares
/// it), and whether a mint's name must be a plain string literal there.
#[derive(Clone, Copy)]
struct MintSpelling {
    /// The call that mints a predicate name, whose first argument IS the
    /// name.
    call: &'static str,
    /// The call's bare name.
    bare: &'static str,
    /// The head of its declaration, where the source declares it.
    decl: Option<&'static str>,
    /// Whether every mint names a plain string literal. topo's
    /// measurement door decides under names its callers hand it, so a
    /// mint there names a parameter and is read for its margin alone.
    literal_names: bool,
}

/// The lanes' spelling: `props/quad.rs`'s funnel wrapper over
/// `geom_core::k_stats::decide`.
const LANE_MINTS: MintSpelling = MintSpelling {
    call: "classify_len::<T>(",
    bare: "classify_len",
    decl: Some("fn classify_len"),
    literal_names: true,
};

/// topo's measurement door's spelling: the funnel's reporting call.
const DOOR_MINTS: MintSpelling = MintSpelling {
    call: "decide_reported(",
    bare: "decide_reported",
    decl: None,
    literal_names: false,
};

/// Every kernel source the roster's names are minted in, as
/// `(path, text, spelling)`.
///
/// The lanes' own mints (`props_quad_face_extent`), and topo's
/// measurement door, where a deciding predicate reads a volume enclosure
/// (`props.rs`'s role read, `props/settle.rs`'s relations). The roster is
/// empty (its one family, `props_quad_converged`, is no longer minted:
/// the reporting target became a resolution, compared and not decided),
/// so what this table still guards is the ADDED direction: a mint here
/// whose margin derives from the ε-scaled target must be ruled on. A
/// roster entry minted somewhere not listed here REDS
/// [`every_rostered_predicate_is_still_minted_by_the_kernel`] rather
/// than passing quietly, and the fix is a line in this table — the same
/// fail-loud posture the allow-list itself takes.
///
/// **Each entry is parsed on its own**, and each must yield at least
/// one mint, so a second entry that mints nothing reds instead of
/// riding the first one's sites — and a `let target_len` in one file
/// can never answer for a mint in another.
///
/// **A path here is also a trigger** in `.github/workflows/ci.yml`'s
/// `tools` row, since a PR editing only the kernel file is this
/// roster's subject; a new entry adds its path there.
const MINT_SOURCES: [(&str, &str, MintSpelling); 3] = [
    (
        "crates/geom-brep/src/props/quad.rs",
        include_str!("../../../crates/geom-brep/src/props/quad.rs"),
        LANE_MINTS,
    ),
    (
        "crates/topo/src/props.rs",
        include_str!("../../../crates/topo/src/props.rs"),
        DOOR_MINTS,
    ),
    (
        "crates/topo/src/props/settle.rs",
        include_str!("../../../crates/topo/src/props/settle.rs"),
        DOOR_MINTS,
    ),
];

/// The call the lanes mint with, as the pins below name it.
///
/// [`mint_sites`] parses a source's call exactly; its bare name and
/// declaration head are what make that parse's COVERAGE checkable.
/// Every occurrence of the bare name in a source must be a parsed call
/// or the declaration itself, so a site respelled `classify_len:: <T>(`
/// — still a call, no longer the spelling — reds rather than silently
/// leaving the population the other pins quantify over.
const MINT_CALL: &str = LANE_MINTS.call;

/// A `(code_only, code_and_literals)` pair per [`MINT_SOURCES`] entry,
/// lexed once, with [`kernel_only`] applied, and held in `cache`.
fn source_views(cache: &'static OnceLock<Vec<(String, String)>>) -> &'static [(String, String)] {
    cache.get_or_init(|| {
        MINT_SOURCES
            .iter()
            .map(|(path, text, _)| kernel_only(path, text))
            .collect()
    })
}

/// `text`'s `(code_only, code_and_literals)` views with every
/// `#[cfg(test)] mod tests { … }` item blanked out of both.
///
/// **The roster pins KERNEL mints.** A call inside the source file's
/// own test module is a test's, and counting it would make the census
/// depend on how tests are written: a test that calls the funnel at a
/// concrete scalar (`classify_len::<f64>(`) is neither a [`MINT_CALL`]
/// site nor the declaration, and reds the coverage check over code the
/// kernel never runs.
///
/// **Blanked rather than stripped**, as the views themselves are, so
/// every offset still means the same byte in `text`. **And exact**: the
/// region runs from the attribute through the module's balanced closing
/// brace ([`source::attributed_extent`]), never to end-of-file, so a
/// kernel item after a test module is still read. A `#[cfg(test)]` on
/// anything but `mod tests` — a statement inside a kernel `fn` that
/// feeds a test hook — is kernel code and stays.
fn kernel_only(searched: &str, text: &str) -> (String, String) {
    let code = source::blanked(source::code_only, searched, text);
    let literals = source::blanked(source::code_and_literals, searched, text);
    let regions = test_modules(&code, searched);
    let blank = |view: String| {
        let mut bytes = view.into_bytes();
        for region in &regions {
            for b in &mut bytes[region.clone()] {
                if *b != b'\n' {
                    *b = b' ';
                }
            }
        }
        String::from_utf8(bytes).expect("a region starts at `#` and ends after `}`")
    };
    (blank(code), blank(literals))
}

/// The byte range of every `#[cfg(test)] mod tests` item in `code`, a
/// [`source::code_only`] view: from the attribute through the module's
/// closing brace.
fn test_modules(code: &str, searched: &str) -> Vec<std::ops::Range<usize>> {
    code.match_indices("#[cfg(test)]")
        .filter_map(|(at, _)| {
            let mut head = at;
            while code[head..].starts_with("#[") {
                let close = source::balanced_end(code, head + 1).expect("an attribute closes");
                head = source::skip_ws(code, close + 1);
            }
            let is_module = source::word_at(code, head, "mod")
                && source::ident(code, source::skip_ws(code, head + "mod".len())) == "tests";
            is_module.then(|| {
                source::attributed_extent(code, at).unwrap_or_else(|| {
                    panic!("the `#[cfg(test)] mod tests` at byte {at} in {searched} never closes")
                })
            })
        })
        .collect()
}

/// `(path, code_view, literal_view, spelling)` for every [`MINT_SOURCES`]
/// entry.
///
/// The code view has prose AND string literals blanked — where a call
/// is LOCATED. The literal view keeps the literals — where a located
/// argument's VALUE is read. `source::blanked` refuses either unless it
/// is the original's bytes blanked in place, so an offset means the
/// same byte in both.
fn kernel_views() -> Vec<(&'static str, &'static str, &'static str, MintSpelling)> {
    static VIEWS: OnceLock<Vec<(String, String)>> = OnceLock::new();
    MINT_SOURCES
        .iter()
        .zip(source_views(&VIEWS))
        .map(|((path, _, spelling), (code, literals))| {
            (*path, code.as_str(), literals.as_str(), *spelling)
        })
        .collect()
}

/// One parsed mint: the predicate name (the name argument's text, where
/// the source names a parameter), its margin argument, and the byte the
/// call starts at in its source's code view.
#[derive(Debug)]
struct Mint {
    name: String,
    margin: String,
    at: usize,
}

/// Every `spelling.call` site in `code`, with names read out of
/// `literals`.
///
/// **Located over the code view, which is what makes an answer a
/// call.** Over raw text a doc comment quoting the call sorts ahead of
/// the call itself, so the pin would read prose; over `code_only` every
/// occurrence, every bracket and every top-level comma is real code, so
/// `balanced_end` and `top_level_split` ARE the parse.
///
/// **And the parse must cover the file.** Every `spelling.bare` in
/// `code` is either a site this returns or the declaration head, else
/// this refuses. Without that check a respelled call is not a failure but an
/// absence, and every pin downstream quantifies over a smaller
/// population without saying so.
fn mint_sites(spelling: MintSpelling, code: &str, literals: &str, searched: &str) -> Vec<Mint> {
    let MintSpelling {
        call,
        bare,
        decl,
        literal_names,
    } = spelling;
    assert_eq!(
        code.len(),
        literals.len(),
        "the two views are the same bytes, blanked differently"
    );
    let sites: Vec<Mint> = code
        .match_indices(call)
        .map(|(at, _)| {
            let open = at + call.len() - 1;
            let close = source::balanced_end(code, open).expect("the call closes");
            let inner = open + 1..close;
            let args = source::top_level_split(&code[inner.clone()], ',');
            assert!(
                args.len() >= 2,
                "a `{call}` site in {searched} takes {} arguments, not a name and a margin",
                args.len()
            );
            let at_name = inner.start + args[0].start..inner.start + args[0].end;
            let text = literals[at_name].trim();
            let name = match source::plain_string_literal(text) {
                Some(name) => name,
                None if !literal_names => text,
                None => panic!(
                    "a `{call}` site in {searched} names {text:?}, not a plain string literal"
                ),
            };
            let at_margin = inner.start + args[1].start..inner.start + args[1].end;
            Mint {
                name: name.to_string(),
                margin: code[at_margin].trim().to_string(),
                at,
            }
        })
        .collect();
    assert!(
        !sites.is_empty(),
        "no `{call}` site in {searched} — the kernel's mint spelling moved and every pin \
         over it is now vacuous"
    );
    for (at, _) in code.match_indices(bare) {
        let is_call = sites.iter().any(|m| m.at == at);
        let is_decl = decl.is_some_and(|decl| {
            (at + bare.len())
                .checked_sub(decl.len())
                .is_some_and(|d| code[d..].starts_with(decl))
        });
        assert!(
            is_call || is_decl,
            "`{bare}` occurs in {searched} at byte {at}, spelled {:?} — neither a \
             `{call}` site this parse read nor the {decl:?} declaration. A call the \
             parse cannot see leaves every pin over these mints quantifying over fewer sites \
             without reddening",
            &code[at..code.len().min(at + call.len() + 8)]
        );
    }
    sites
}

/// [`mint_sites`] over every [`MINT_SOURCES`] entry, each parsed on its
/// own so a listed source that mints nothing reds rather than riding
/// another's sites. One `(path, code_view, mint)` per site.
fn kernel_mint_sites() -> Vec<(&'static str, &'static str, Mint)> {
    let mut out = Vec::new();
    for (path, code, literals, spelling) in kernel_views() {
        for mint in mint_sites(spelling, code, literals, path) {
            out.push((path, code, mint));
        }
    }
    out
}

/// The [`EPS_COUPLED_PREDICATES`] entries `minted` does not mint, as
/// the refusals a caller prints.
///
/// Shared by the pin and by its negative control below, so the control
/// exercises the pin's own decision rather than a second copy of it.
fn unminted_roster_names(roster: &[&str], minted: &[&str], searched: &str) -> Vec<String> {
    roster
        .iter()
        .filter(|name| !minted.contains(*name))
        .map(|name| {
            format!(
                "EPS_COUPLED_PREDICATES rosters {name:?}, which no `{MINT_CALL}` site in \
                 {searched} mints — either the kernel renamed it (rule 4 now applies to nothing \
                 and its family is judged by the metre rules) or it moved to a source \
                 MINT_SOURCES does not list. minted there: {minted:?}"
            )
        })
        .collect()
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
    let sites = kernel_mint_sites();
    let minted: Vec<&str> = sites.iter().map(|(_, _, m)| m.name.as_str()).collect();
    let paths: Vec<&str> = MINT_SOURCES.iter().map(|(p, _, _)| *p).collect();
    let complaints = unminted_roster_names(&EPS_COUPLED_PREDICATES, &minted, &format!("{paths:?}"));
    assert!(complaints.is_empty(), "{}", complaints.join("\n"));
}

/// The `let` the ε-scaled target is bound by, the value it must have,
/// and the constant that value names.
const TARGET: &str = "target_len";
const TARGET_BINDING: &str = "let target_len =";
const TARGET_VALUE: &str = "QUAD_TARGET_LEN_FACTOR * eps";
const FACTOR_DECL: &str = "const QUAD_TARGET_LEN_FACTOR: f64 = ";

/// Whether a margin names the ε-scaled target by any of its spellings:
/// the lanes' [`TARGET`] binding, topo's interim number-door resolution,
/// or the factor itself.
fn eps_scaled(margin: &str) -> bool {
    uses_target(margin)
        || margin.contains("interim_resolution")
        || margin.contains("QUAD_TARGET_LEN_FACTOR")
}

/// Whether `text` uses [`TARGET`] as a WHOLE identifier.
///
/// A substring test answers yes to `fixed_target_len`, which is a
/// different binding and need not be ε-scaled at all — exactly the
/// shape a de-coupling that keeps the family's name would take.
fn uses_target(text: &str) -> bool {
    let part = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    text.match_indices(TARGET).any(|(at, _)| {
        let after = at + TARGET.len();
        (at == 0 || !part(text.as_bytes()[at - 1]))
            && (after == text.len() || !part(text.as_bytes()[after]))
    })
}

/// `(signature, whole item)` byte ranges of the top-level `fn` enclosing
/// `at` in `code`.
///
/// Top-level: an item head starts at column zero, which every `fn` in
/// [`MINT_SOURCES`] holding a mint does. The item runs to the next such
/// head, so the answer over-reaches past the item's own `}` rather than
/// under-reaching past a nested block — and over-reaching can only make
/// the binding checks below stricter, never laxer. A trailing
/// `mod tests` is inside that reach only as blank bytes: [`kernel_only`]
/// has blanked it out of the view, so a test's `let target_len` can
/// neither stand in for the kernel's binding nor count as a second one.
fn enclosing_fn(code: &str, at: usize) -> (std::ops::Range<usize>, std::ops::Range<usize>) {
    let mut heads: Vec<usize> = ["\nfn ", "\npub fn ", "\npub(crate) fn "]
        .iter()
        .flat_map(|head| code.match_indices(head).map(|(off, _)| off + 1))
        .collect();
    heads.sort_unstable();
    let start = *heads
        .iter()
        .rev()
        .find(|h| **h <= at)
        .unwrap_or_else(|| panic!("a mint at byte {at} sits outside every top-level `fn`"));
    let end = heads
        .iter()
        .find(|h| **h > at)
        .copied()
        .unwrap_or(code.len());
    let head_end = start + code[start..end].find('{').expect("the `fn` opens a body");
    (start..head_end, start..end)
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
/// textually, and **at each mint rather than file-wide**:
///
/// 1. the margin uses [`TARGET`] as a WHOLE identifier — a
///    `fixed_target_len` bound beside the real one does not answer;
/// 2. the mint's own enclosing `fn` binds that identifier exactly once,
///    by [`TARGET_BINDING`], to [`TARGET_VALUE`] — so neither a second
///    binding nor one in a different function can stand in for it;
/// 3. that `fn` does not take [`TARGET`] as a PARAMETER, where its
///    value is a caller's property this walk does not follow. That
///    shape is REFUSED rather than answered: it is not a de-coupling,
///    and this pin cannot tell that it is not;
/// 4. `QUAD_TARGET_LEN_FACTOR` is a plain finite positive literal.
///
/// **Textual, and for THIS roster only.** It reads the kernel's
/// spelling, so a refactor that keeps the ε-coupling and changes the
/// spelling reds it — deliberately, per (3)'s reason. It does not
/// generalise to a roster entry the kernel has not minted yet; that
/// wants the criterion
/// `work/instr/k-lint-eps-coupled-criterion-unwritten` schedules.
#[test]
fn the_rostered_familys_margin_is_still_eps_scaled_at_its_mint() {
    let sites = kernel_mint_sites();
    let (_, complaints) = rostered_margin_complaints(&EPS_COUPLED_PREDICATES, &sites);
    assert!(complaints.is_empty(), "{}", complaints.join("\n"));
    let (path, code, _, _) = kernel_views()[0];
    let factor: f64 = code[source::sole_initializer(code, path, FACTOR_DECL)]
        .trim()
        .parse()
        .expect("QUAD_TARGET_LEN_FACTOR is a float literal");
    assert!(
        factor.is_finite() && factor > 0.0,
        "QUAD_TARGET_LEN_FACTOR is {factor}, so the target is not a positive multiple of ε"
    );
}

/// The live roster is empty, so the pin above asserts nothing over it.
/// **Its rule is read here against a synthetic roster** instead: a
/// kernel mint whose margin is no headroom against the target REDS it,
/// naming the mint, and a fixture mint metered against a correctly bound
/// `target_len` is QUIET, so the rule is neither vacuous nor a refusal
/// of everything.
#[test]
fn the_margin_rule_reds_an_unscaled_rostered_mint_and_passes_a_scaled_one() {
    let sites = kernel_mint_sites();
    let (rostered, complaints) = rostered_margin_complaints(&["props_quad_face_extent"], &sites);
    assert!(rostered > 0, "the lanes mint `props_quad_face_extent`");
    assert!(
        complaints
            .iter()
            .any(|c| c.contains("props_quad_face_extent") && c.contains("does not derive")),
        "a rostered mint metered against a perimeter, not the target, reds: {complaints:?}"
    );

    let scaled = concat!(
        "\n",
        "pub(crate) fn walk<T: Decide>(eps: f64, w: f64) -> R {\n",
        "    let target_len = QUAD_TARGET_LEN_FACTOR * eps;\n",
        "    classify_len::<T>(\"synthetic_coupled\", Margin::of(target_len - w), band)\n",
        "}\n",
    );
    let (code, literals) = (source::code_only(scaled), source::code_and_literals(scaled));
    let fixture: Vec<(&str, &str, Mint)> = mint_sites(LANE_MINTS, &code, &literals, "the fixture")
        .into_iter()
        .map(|mint| ("the fixture", code.as_str(), mint))
        .collect();
    let (rostered, complaints) = rostered_margin_complaints(&["synthetic_coupled"], &fixture);
    assert_eq!(rostered, 1, "the fixture mints the synthetic entry once");
    assert!(
        complaints.is_empty(),
        "a mint metered against a bound `target_len` passes: {complaints:?}"
    );
}

/// [`the_rostered_familys_margin_is_still_eps_scaled_at_its_mint`]'s rule
/// over `sites` under `roster`: how many sites are rostered, and the
/// refusal of each that breaks it.
fn rostered_margin_complaints(
    roster: &[&str],
    sites: &[(&str, &str, Mint)],
) -> (usize, Vec<String>) {
    let mut rostered = 0usize;
    let mut complaints = Vec::new();
    for (path, code, mint) in sites {
        if !roster.contains(&mint.name.as_str()) {
            continue;
        }
        rostered += 1;
        let (name, margin) = (&mint.name, &mint.margin);
        if !uses_target(margin) {
            complaints.push(format!(
                "{name} is rostered ε-coupled and its margin at the mint in {path} reads \
                 {margin:?}, which does not derive from the identifier `{TARGET}` — rule (4)'s \
                 premise is that this margin is a headroom against an ε-scaled target"
            ));
            continue;
        }
        let (head, item) = enclosing_fn(code, mint.at);
        if uses_target(&code[head.clone()]) {
            complaints.push(format!(
                "{name} is rostered ε-coupled and is minted inside `{}`, which takes `{TARGET}` \
                 as a PARAMETER — whether it is ε-scaled is then the caller's property, and this \
                 textual pin does not follow it. Refused rather than answered",
                code[head].split('(').next().unwrap_or_default().trim()
            ));
            continue;
        }
        let body = &code[item];
        let bindings = source::initializers(body, TARGET_BINDING);
        if bindings.len() != 1 {
            complaints.push(format!(
                "{name} is rostered ε-coupled and its enclosing `fn` in {path} binds \
                 `{TARGET_BINDING}` {} times, not once — a mint cannot be read against an \
                 ambiguous binding",
                bindings.len()
            ));
            continue;
        }
        if body[bindings[0].clone()].trim() != TARGET_VALUE {
            complaints.push(format!(
                "the `{TARGET}` in scope at {name}'s mint in {path} is not {TARGET_VALUE:?} — \
                 rule (4)'s exemption assumes this family's margin scales with ε"
            ));
        }
    }
    (rostered, complaints)
}

/// **Every mint whose margin derives from the ε-scaled target is on the
/// roster** — the ADDED direction, closed for
/// `target_len`-derived mints in [`MINT_SOURCES`] and for nothing
/// wider.
///
/// `EPS_COUPLED_PREDICATES` is a subset selected from an open
/// vocabulary by a property nothing writes down, so its completeness is
/// not checkable in general
/// (`work/instr/k-lint-eps-coupled-criterion-unwritten`). What IS
/// checkable is that property's spelling in this family's own source: a
/// margin metered against `target_len` is a headroom against
/// `QUAD_TARGET_LEN_FACTOR·ε`, which the pin above establishes for
/// every rostered mint. A mint that acquires one and is not rostered
/// gets ruled on here, instead of being judged by rules (2) and (3)
/// with nobody noticing the question was open.
#[test]
fn every_target_len_mint_is_rostered_or_excused() {
    for (path, _, mint) in kernel_mint_sites() {
        if !eps_scaled(&mint.margin) {
            continue;
        }
        let name = mint.name.as_str();
        assert!(
            EPS_COUPLED_PREDICATES.contains(&name)
                || EPS_COUPLED_UNRULED.iter().any(|(n, _)| *n == name),
            "{name:?} is minted in {path} with margin {:?} — a headroom against the ε-scaled \
             `{TARGET}`, which is the property rule (4) exempts a family for — but it is not on \
             EPS_COUPLED_PREDICATES nor excused in EPS_COUPLED_UNRULED. Rule it: a line on the \
             roster, or a line in EPS_COUPLED_UNRULED saying why the metre rules are right for it",
            mint.margin
        );
    }
    for (name, _) in EPS_COUPLED_UNRULED {
        assert!(
            !EPS_COUPLED_PREDICATES.contains(name),
            "{name:?} is both rostered and excused from the roster"
        );
    }
}

/// The locator reads a mint and not prose about it, the coverage
/// refusal fires on a respelled call, and the roster check reds on a
/// mint short a rostered name.
///
/// All three would otherwise be silent greens. A doc comment and a
/// quoted string spelling the same call sort ahead of the call itself,
/// and over the code view neither can answer; a call the parse cannot
/// read must red rather than shrink the population; a rostered name
/// absent from the mints must red. Each shortfall is constructed here
/// rather than assumed, and the roster half exercises
/// [`unminted_roster_names`] itself — the function
/// [`every_rostered_predicate_is_still_minted_by_the_kernel`] asserts
/// on — rather than a second copy of its rule. The roster the fixtures
/// are read against is the committed eras' ([`ERA_EPS_COUPLED_PREDICATES`]):
/// the live one is empty, and a roster check over it would red nothing.
#[test]
fn the_pin_reads_a_mint_a_respelling_and_a_missing_name_red_it() {
    let decoyed = concat!(
        "/// classify_len::<T>(\"decoy_doc\", Margin::of(target_len - w), band)\n",
        "const Q: &str = \"classify_len::<T>(\\\"decoy_str\\\", Margin::of(x), band)\";\n",
        "fn classify_len<T: Decide>(n: &str, m: Margin, b: Band) -> R { decide(n, m, b) }\n",
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
        LANE_MINTS,
        &source::code_only(decoyed),
        &source::code_and_literals(decoyed),
        "the decoy fixture",
    );
    let names: Vec<&str> = sites.iter().map(|m| m.name.as_str()).collect();
    assert_eq!(
        names,
        ["props_quad_converged", "props_quad_face_extent"],
        "only the two real calls answer"
    );
    assert_eq!(sites[0].margin, "Margin::of(target_len - width_len)");
    assert!(
        unminted_roster_names(&ERA_EPS_COUPLED_PREDICATES, &names, "the decoy fixture").is_empty(),
        "this fixture mints the whole roster, so the roster check must be QUIET on it — \
         otherwise the red below says nothing about a missing name"
    );

    // A call the parse cannot read is a refusal, not an absence.
    let respelled = decoyed.replace("if classify_len::<T>(", "if classify_len:: <T>(");
    let refused = std::panic::catch_unwind(|| {
        mint_sites(
            LANE_MINTS,
            &source::code_only(&respelled),
            &source::code_and_literals(&respelled),
            "the respelled fixture",
        )
    })
    .expect_err("a call the parse cannot read must refuse, not narrow the population");
    let refused = refused
        .downcast_ref::<String>()
        .expect("the refusal carries its message");
    assert!(
        refused.contains("neither a") && refused.contains("the respelled fixture"),
        "the coverage refusal names what it could not read, and where: {refused}"
    );

    // A mint short a rostered name reds the roster check, and the
    // refusal names what WAS minted — so a fixture minting some other
    // name is not interchangeable with this one.
    let short = "let x = classify_len::<T>(\"other_name\", Margin::of(m), b);";
    let minted = mint_sites(
        LANE_MINTS,
        &source::code_only(short),
        &source::code_and_literals(short),
        "the short fixture",
    );
    let names: Vec<&str> = minted.iter().map(|m| m.name.as_str()).collect();
    let complaints = unminted_roster_names(&ERA_EPS_COUPLED_PREDICATES, &names, "the short fixture");
    assert_eq!(complaints.len(), 1, "one rostered name is unminted here");
    assert!(
        complaints[0].contains("props_quad_converged")
            && complaints[0].contains("other_name")
            && complaints[0].contains("the short fixture"),
        "the refusal names the roster entry, what was minted instead, and where: {}",
        complaints[0]
    );
}

/// **The test-module exclusion swallows tests and nothing else** — red
/// if it ever eats a kernel mint.
///
/// [`kernel_only`] blanks the source file's `#[cfg(test)] mod tests`
/// so a test's call is not counted as a mint. Its failure mode is
/// silent: blank too much and a kernel mint leaves the population every
/// pin quantifies over. So the fixture puts kernel mints on every side
/// of what it blanks — before the module, after it, and after a
/// `#[cfg(test)]` STATEMENT inside a kernel `fn` (which is kernel code,
/// as the test hook in `props/quad.rs`'s `bezier_bisect` is) — and a
/// test-only spelling inside the module that the coverage check would
/// otherwise refuse.
#[test]
fn the_test_module_exclusion_blanks_the_module_and_no_kernel_mint() {
    let fixture = concat!(
        "fn before<T: Decide>() { classify_len::<T>(\"before_tests\", Margin::of(a), band); }\n",
        "fn hooked<T: Decide>() {\n",
        "    #[cfg(test)]\n",
        "    tests::HOOK.with(|h| h.set(true));\n",
        "    classify_len::<T>(\"after_a_test_hook\", Margin::of(b), band);\n",
        "}\n",
        "#[cfg(test)]\n",
        "#[allow(clippy::unwrap_used)]\n",
        "mod tests {\n",
        "    fn pins() { classify_len::<f64>(\"a_test_call\", Margin::of(c), band); }\n",
        "    fn nested() { if x { classify_len::<T>(\"in_tests\", Margin::of(d), band); } }\n",
        "}\n",
        "fn after<T: Decide>() { classify_len::<T>(\"after_tests\", Margin::of(e), band); }\n",
    );

    // Unexcluded, the test's concrete-scalar call is what reds the
    // coverage check — the failure this exclusion exists for.
    let unexcluded = std::panic::catch_unwind(|| {
        mint_sites(
            LANE_MINTS,
            &source::code_only(fixture),
            &source::code_and_literals(fixture),
            "the unexcluded fixture",
        )
    })
    .expect_err("a test's `classify_len::<f64>(` is neither a parsed site nor the declaration");
    let unexcluded = unexcluded
        .downcast_ref::<String>()
        .expect("the refusal carries its message");
    assert!(
        unexcluded.contains("classify_len::<f64>("),
        "the refusal quotes the test's spelling: {unexcluded}"
    );

    let (code, literals) = kernel_only("the fixture", fixture);
    assert_eq!(code.len(), fixture.len(), "blanked, not stripped");
    let names: Vec<String> = mint_sites(LANE_MINTS, &code, &literals, "the fixture")
        .into_iter()
        .map(|m| m.name)
        .collect();
    assert_eq!(
        names,
        ["before_tests", "after_a_test_hook", "after_tests"],
        "every kernel mint survives the exclusion, on both sides of the module, and no test \
         call does"
    );
    assert!(
        code.contains("tests::HOOK"),
        "a `#[cfg(test)]` statement in a kernel `fn` is not the test module"
    );
}
