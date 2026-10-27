//! **[`EPS_COUPLED_PREDICATES`] names predicates the kernel still
//! mints, and no production door states an ε-derived resolution** —
//! the two pins this roster carries across the cargo-root boundary
//! (the second replaced the ε-scaled target's pins when that target was
//! retired; the note above its row says why).
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
//!   value `QUAD_TARGET_LEN_FACTOR * eps`;
//! - **every mint whose margin derives from the ε-scaled target is
//!   rostered** — a margin naming `target_len` or
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

use k_lint::{EPS_COUPLED_PREDICATES, ERA_EPS_COUPLED_PREDICATES};
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

/// The lanes' spelling: `props/quad/run.rs`'s funnel wrapper over
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
/// `(path, spelling)`: a file, or a DIRECTORY (a path ending `/`) whose
/// every `.rs` file is read ([`mint_files`]), so a lane file added
/// there is under the roster without a line here.
///
/// The lanes' own mints (`props_quad_face_extent`, at the one extent gate
/// every lane asks, and `props_trim_piece_monotone`), and topo's
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
/// **Each file is parsed on its own**, and each entry must yield at
/// least one mint, so an entry that mints nothing reds instead of
/// riding another's sites — and a `let target_len` in one file can
/// never answer for a mint in another. A file under a directory entry
/// may mint nothing; the directory as a whole may not.
///
/// **A path here is also a trigger** in `.github/workflows/ci.yml`'s
/// `tools` row, since a PR editing only the kernel file is this
/// roster's subject; a new entry adds its path there.
const MINT_SOURCES: [(&str, MintSpelling); 3] = [
    ("crates/geom-brep/src/props/quad/", LANE_MINTS),
    ("crates/topo/src/props.rs", DOOR_MINTS),
    ("crates/topo/src/props/settle.rs", DOOR_MINTS),
];

/// Every file [`MINT_SOURCES`] names, as `(entry, path, text)`: a file
/// entry's own, and every `.rs` file under a directory entry, in path
/// order. Read from the tree at run time, once; an entry naming no file
/// reds here.
fn mint_files() -> &'static [(usize, String, String)] {
    static FILES: OnceLock<Vec<(usize, String, String)>> = OnceLock::new();
    FILES.get_or_init(|| {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let mut out = Vec::new();
        for (entry, (path, _)) in MINT_SOURCES.iter().enumerate() {
            let mut paths = Vec::new();
            if path.ends_with('/') {
                let mut stack = vec![root.join(path)];
                while let Some(dir) = stack.pop() {
                    for item in std::fs::read_dir(&dir)
                        .unwrap_or_else(|e| panic!("MINT_SOURCES' {path} does not read: {e}"))
                    {
                        let file = item.expect("an entry reads").path();
                        if file.is_dir() {
                            stack.push(file);
                        } else if file.extension().is_some_and(|x| x == "rs") {
                            paths.push(file);
                        }
                    }
                }
                paths.sort();
            } else {
                paths.push(root.join(path));
            }
            assert!(!paths.is_empty(), "MINT_SOURCES' {path} names no file");
            for file in paths {
                let text = std::fs::read_to_string(&file)
                    .unwrap_or_else(|e| panic!("{} does not read: {e}", file.display()));
                let rel = file
                    .strip_prefix(&root)
                    .expect("under the root")
                    .to_string_lossy()
                    .into_owned();
                out.push((entry, rel, text));
            }
        }
        out
    })
}

/// The call the lanes mint with, as the pins below name it.
///
/// [`mint_sites`] parses a source's call exactly; its bare name and
/// declaration head are what make that parse's COVERAGE checkable.
/// Every occurrence of the bare name in a source must be a parsed call
/// or the declaration itself, so a site respelled `classify_len:: <T>(`
/// — still a call, no longer the spelling — reds rather than silently
/// leaving the population the other pins quantify over.
const MINT_CALL: &str = LANE_MINTS.call;

/// A `(code_only, code_and_literals)` pair per [`mint_files`] file,
/// lexed once, with [`kernel_only`] applied, and held in `cache`.
fn source_views(cache: &'static OnceLock<Vec<(String, String)>>) -> &'static [(String, String)] {
    cache.get_or_init(|| {
        mint_files()
            .iter()
            .map(|(_, path, text)| kernel_only(path, text))
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

/// `(entry, path, code_view, literal_view, spelling)` for every
/// [`mint_files`] file.
///
/// The code view has prose AND string literals blanked — where a call
/// is LOCATED. The literal view keeps the literals — where a located
/// argument's VALUE is read. `source::blanked` refuses either unless it
/// is the original's bytes blanked in place, so an offset means the
/// same byte in both.
fn kernel_views() -> Vec<(
    usize,
    &'static str,
    &'static str,
    &'static str,
    MintSpelling,
)> {
    static VIEWS: OnceLock<Vec<(String, String)>> = OnceLock::new();
    mint_files()
        .iter()
        .zip(source_views(&VIEWS))
        .map(|((entry, path, _), (code, literals))| {
            (
                *entry,
                path.as_str(),
                code.as_str(),
                literals.as_str(),
                MINT_SOURCES[*entry].1,
            )
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
    let sites = parse_sites(spelling, code, literals, searched);
    assert!(
        !sites.is_empty(),
        "no `{}` site in {searched} — the kernel's mint spelling moved and every pin \
         over it is now vacuous",
        spelling.call
    );
    sites
}

/// [`mint_sites`]' parse and its coverage check, without the demand
/// that `code` mint at all: a file under a directory entry of
/// [`MINT_SOURCES`] may mint nothing, as long as the directory mints.
fn parse_sites(spelling: MintSpelling, code: &str, literals: &str, searched: &str) -> Vec<Mint> {
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

/// [`parse_sites`] over every [`mint_files`] file, each parsed on its
/// own, and every [`MINT_SOURCES`] entry made to mint, so a listed
/// source that mints nothing reds rather than riding another's sites.
/// One `(path, code_view, mint)` per site.
fn kernel_mint_sites() -> Vec<(&'static str, &'static str, Mint)> {
    let mut out = Vec::new();
    let mut minting = vec![false; MINT_SOURCES.len()];
    for (entry, path, code, literals, spelling) in kernel_views() {
        for mint in parse_sites(spelling, code, literals, path) {
            minting[entry] = true;
            out.push((path, code, mint));
        }
    }
    for ((path, spelling), minted) in MINT_SOURCES.iter().zip(minting) {
        assert!(
            minted,
            "no `{}` site in {path} — the kernel's mint spelling moved and every pin \
             over it is now vacuous",
            spelling.call
        );
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
    let paths: Vec<&str> = MINT_SOURCES.iter().map(|(p, _)| *p).collect();
    let complaints = unminted_roster_names(&EPS_COUPLED_PREDICATES, &minted, &format!("{paths:?}"));
    assert!(complaints.is_empty(), "{}", complaints.join("\n"));
}

// **The ε-scaled target's pins are retired** (QUAD measure door unit C,
// PR 5741). They held a rostered mint's margin to a `target_len` bound
// to `QUAD_TARGET_LEN_FACTOR * eps`, and they held every mint metered
// against that target to the roster. The constant is gone from the
// kernel (no lane reads a reporting target: a caller states its own
// resolution), the live roster is empty, and no mint can be bound to a
// value that no longer exists, so neither pin could go red. What
// carries the risk now is a production door stating an ε-derived
// resolution of its own, which would be the reporting target back under
// another name: the row below pins that.

/// **No production door states an ε-derived resolution** — where the
/// retired target pins' risk now sits. `[tolerance.measure]` has no
/// default: the caller states its resolution, so a kernel or binding
/// door that builds `Resolution(` from ε is a reporting target by
/// another name. Read over every `crates/*/src` file's code view, test
/// modules blanked ([`kernel_only`]); test-support files and
/// `topo`'s `test_support` module (the suites' `pinned_resolution`, a
/// stated test resolution) are not production and are skipped.
#[test]
fn no_production_door_states_an_eps_derived_resolution() {
    let crates = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates");
    let mut files = Vec::new();
    let mut stack = vec![crates];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("the tree reads") {
            let path = entry.expect("an entry reads").path();
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .to_owned();
            if path.is_dir() {
                if name != "tests" && name != "examples" && name != "benches" && name != "target" {
                    stack.push(path);
                }
            } else if name.ends_with(".rs")
                && path.components().any(|c| c.as_os_str() == "src")
                && !name.contains("test_support")
                && name != "tests.rs"
                && !name.ends_with("_tests.rs")
            {
                files.push(path);
            }
        }
    }
    files.sort();
    assert!(
        files.len() > 100,
        "the walk reaches the workspace's sources"
    );
    let mut found = Vec::new();
    for path in &files {
        let text = std::fs::read_to_string(path).expect("a source reads");
        let shown = path.display().to_string();
        let (mut code, _) = kernel_only(&shown, &text);
        if let Some(at) = code.find("pub mod test_support {") {
            let open = at + code[at..].find('{').expect("the module opens");
            let close = source::balanced_end(&code, open).expect("the module closes");
            let blank: String = code[at..=close]
                .chars()
                .map(|c| if c == '\n' { c } else { ' ' })
                .collect();
            code.replace_range(at..=close, &blank);
        }
        for (at, _) in code.match_indices("Resolution(") {
            let open = at + "Resolution".len();
            let Some(close) = source::balanced_end(&code, open) else {
                continue;
            };
            if code[open..close].contains("eps") {
                let line = code[..at].matches('\n').count() + 1;
                found.push(format!("{shown}:{line}: {}", code[at..=close].trim()));
            }
        }
    }
    assert!(
        found.is_empty(),
        "a production door states an ε-derived resolution, a reporting target by another \
         name: the caller states its resolution (`[tolerance.measure]`, no default):\n{}",
        found.join("\n")
    );
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
    let complaints =
        unminted_roster_names(&ERA_EPS_COUPLED_PREDICATES, &names, "the short fixture");
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
/// as the test hook in `props/quad/trimmed.rs`'s `bezier_bisect` is) — and a
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
