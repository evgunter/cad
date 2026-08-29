//! The clause-(i) debt lane's census (the standing rule on
//! [`geom_core::k_stats::decide_flagged`]; the lane is tracked as issue
//! #214): **no new `decide_flagged` site ships without a ledger row**
//! in `docs/predicate-dimension-audit.md`. Two assertions carry it,
//! over the SHIPPED call sites only (crate `src/` trees — fixtures and
//! demos carry prose reasons instead of rows and are not counted):
//!
//! 1. the number of sites matches the ledger's inventory — F2 ×4,
//!    F10 ×1 (one loop over seven rigidity residuals), F13 ×1, F14 ×1,
//!    F15 ×1, **8 sites**. This total is hand-synced; see
//!    [`LEDGER_FLAGGED_SITES`].
//! 2. every site's `ledger_row` argument names a row the ledger
//!    actually has. This one is computed from the document.
//!
//! (2) is what makes the fourth parameter mean something. Without it
//! the argument reaches no recorder, no column and no assertion, so a
//! site citing `"F16"` — or citing a row renumbered out from under it
//! — reads as a discharged obligation and is a string.
//!
//! Adding a site without updating BOTH the ledger and the count fails
//! the suite; retiring a flagged family (its own unit) decrements it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// The ledger's current shipped `decide_flagged` inventory.
///
/// **This number is hand-synced and nothing derives it.** It is kept
/// level by hand with the audit's own prose inventory; the sibling
/// assertion, [`every_shipped_site_cites_a_ledger_row_that_exists`],
/// reads its rows out of the audit and does compute.
const LEDGER_FLAGGED_SITES: usize = 8;

/// One shipped call site: where it is, and the row it cites.
#[derive(Debug)]
struct Site {
    file: PathBuf,
    line: usize,
    ledger_row: String,
}

/// The offset, within the text just after the identifier, of the `(` that
/// opens the call — skipping whitespace and an optional turbofish.
///
/// **A turbofish call is a call.** `decide_flagged::<f64>(…)` is the same
/// shipped site as `decide_flagged(…)`, and a scan that required `(` to
/// follow the identifier immediately would skip it silently and leave its
/// ledger row unverified — the undercount direction this file refuses
/// everywhere else. The generic *definition* is not swept up with it:
/// `pub fn decide_flagged<T: Decide>(` has a bare `<`, never `::<`.
///
/// **Anything that opens a turbofish and does not resolve to a `(` is a
/// panic, never a quiet "no call here."** Angle depth is counted over `<`
/// and `>` alone, so an arrow or a comparison inside the type arguments
/// breaks the balance — and both the unbalanced case and the
/// balanced-but-misplaced case stop the census, because a site the scan
/// cannot read is a site it must not drop. No such site exists today.
fn skip_turbofish(rest: &str, at: usize) -> usize {
    let mut k = rest.len() - rest.trim_start().len();
    if !rest[k..].starts_with("::") {
        return k;
    }
    let after_colons = k + 2;
    let angle =
        after_colons + (rest[after_colons..].len() - rest[after_colons..].trim_start().len());
    if !rest[angle..].starts_with('<') {
        return k;
    }
    let mut depth = 0usize;
    let mut end = None;
    for (off, c) in rest[angle..].char_indices() {
        match c {
            '<' => depth += 1,
            '>' => {
                depth -= 1;
                if depth == 0 {
                    end = Some(angle + off + 1);
                    break;
                }
            }
            _ => {}
        }
    }
    let end = end.unwrap_or_else(|| {
        panic!(
            "a `decide_flagged::<…>` turbofish at byte {at} has unbalanced angle brackets, so \
             this census cannot tell a call site from a mention. Rather than skip it — which \
             would undercount the shipped tree — the census stops here. Spell the site without \
             a turbofish, or teach `skip_turbofish` the form it uses."
        )
    });
    k = end + (rest[end..].len() - rest[end..].trim_start().len());
    // A turbofish was consumed, so this IS a call site. If the scan did not
    // land on its `(`, the angle balance was wrong — an arrow or a comparison
    // inside the type arguments will do it — and returning quietly here would
    // drop a shipped site. Stop instead.
    assert!(
        rest[k..].starts_with('('),
        "a `decide_flagged` turbofish at byte {at} did not resolve to a call: after its type \
         arguments the scan is at {:?}, not `(`. Angle depth is counted over `<` and `>` alone, \
         so an arrow or comparison inside the type arguments breaks it. The census stops rather \
         than skipping a shipped site.",
        &rest[k..rest.len().min(k + 24)]
    );
    k
}

/// Finds every `decide_flagged(…)` call in `text`.
///
/// **What this pattern matches**: the identifier, then an optional
/// turbofish, then `(` — under every import spelling
/// (`geom_core::k_stats::decide_flagged(`, `k_stats::decide_flagged(`,
/// and the bare `decide_flagged(` a `use` makes available) and with or
/// without explicit type arguments ([`skip_turbofish`]). Keying on a
/// path prefix instead would make the census a statement about how calls
/// are SPELLED rather than about how many there are.
///
/// **What it cannot match, and nothing here would notice:** a call
/// through a renamed import (`use …::decide_flagged as df;`) and a call
/// generated by a macro.
///
/// **The scan runs over [`test_utils::source::code_only`]**, so prose is
/// prose wherever it sits: a trailing `// decide_flagged(x)`, a mention
/// inside a `/* … */` block and one inside a string literal are all
/// blanked before the site is read. That view preserves byte offsets,
/// which is why the ledger row is still read out of the ORIGINAL text
/// at the range the scan found — the argument is a string literal and
/// the code view has emptied it.
///
/// **Three near-misses are deliberately NOT blind spots, because each
/// stops the census instead of being skipped:** a `ledger_row` that is
/// not a string literal (a `const`, a field, a table entry); a turbofish
/// whose angle brackets never close; and a turbofish that closes but does
/// not land on the call's `(`, which an arrow in the type arguments
/// causes. The asymmetry is the rule this file is built on — a census
/// that skips a site reports a smaller tree than the one that ships, so
/// anything it cannot read it refuses to pass over. Blanking runs
/// first, before any of those three, so a commented-out mention stops
/// nothing.
///
/// **`src/` is this census's proxy for "shipped", and the proxy is not
/// exact**: an in-file `#[cfg(test)]` module lives in `src/` and ships
/// in no build, so a `decide_flagged` call written there is COUNTED.
/// There are none in the tree, and the failure is the safe direction —
/// the count stops matching and the message names the file and line. The
/// fix for such a site is to move the fixture to the crate's `tests/`
/// tree, which this scan does not walk. Teaching this scan to parse Rust
/// modules is not the fix: it buys a scanner, with blind spots of its
/// own, to excuse a fixture that had no reason to sit in `src/`.
fn calls_in(text: &str) -> Vec<(usize, String)> {
    let code = test_utils::source::code_only(text);
    let bytes = code.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while let Some(rel) = code[i..].find("decide_flagged") {
        let at = i + rel;
        i = at + "decide_flagged".len();
        // Not a longer identifier ending in this name, and not the
        // definition (`fn decide_flagged<T: Decide>(`, whose next
        // character is `<`).
        let prev_ok = at == 0 || {
            let p = bytes[at - 1];
            !(p.is_ascii_alphanumeric() || p == b'_')
        };
        if !prev_ok {
            continue;
        }
        let rest = &code[i..];
        let open = skip_turbofish(rest, at);
        if rest[open..].starts_with('(') {
            let line = code[..at].lines().count();
            let args = i + open + 1;
            let end = args + args_len(&code[args..]);
            let cite = fourth_argument(&code[args..end], &text[args..end]);
            out.push((line, ledger_row(&text[args + cite.start..args + cite.end])));
            i = args;
        }
    }
    out
}

/// The byte length of a call's argument list in the blanked view, from
/// just after `(` to its matching `)`.
///
/// **No literal tracking and no comment strip.** In the code-only view
/// a bracket inside a string or a comment is a space, so bracket depth
/// is the whole of the parse. Both used to be hand-rolled here, once
/// per argument walk.
fn args_len(blanked: &str) -> usize {
    let mut depth = 1usize;
    for (off, c) in blanked.char_indices() {
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => {
                depth -= 1;
                if depth == 0 {
                    return off;
                }
            }
            _ => {}
        }
    }
    panic!("unterminated decide_flagged argument list: {blanked:.120}");
}

/// The byte range, within a call's argument list, of its FOURTH
/// top-level argument. `raw` is the same range of the original text and
/// is quoted in the failure only.
fn fourth_argument(blanked: &str, raw: &str) -> std::ops::Range<usize> {
    let mut depth = 0usize;
    let mut starts = vec![0usize];
    for (off, c) in blanked.char_indices() {
        match c {
            '(' | '[' | '{' | '<' => depth += 1,
            ')' | ']' | '}' | '>' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => starts.push(off + 1),
            _ => {}
        }
    }
    let from = *starts
        .get(3)
        .unwrap_or_else(|| panic!("decide_flagged call with fewer than 4 arguments: {raw:.200}"));
    // The argument ends at the next top-level comma, or at the closing
    // `)` when it is the last one.
    from..starts.get(4).map_or(blanked.len(), |n| n - 1)
}

/// The ledger row a site cites, which must be a string literal.
fn ledger_row(raw: &str) -> String {
    let cited = raw.trim();
    cited
        .strip_prefix('"')
        .and_then(|s| s.strip_suffix('"'))
        .unwrap_or_else(|| {
            panic!(
                "a decide_flagged site cites its ledger row as `{cited}`, not a string literal. \
                 The census can only verify literals; give the site its row directly, or teach \
                 this test to resolve the indirection — do not leave it unchecked."
            )
        })
        .to_string()
}

/// The walk is [`test_utils::source::rust_sources`], recursive and
/// shared: sharing the predicate and re-forking the traversal leaves
/// each guard free to miss a subdirectory, silently and in the green
/// direction.
fn count_in_tree(dir: &Path, hits: &mut Vec<Site>) {
    for path in test_utils::source::rust_sources(dir) {
        let text = std::fs::read_to_string(&path).expect("readable source file");
        for (line, ledger_row) in calls_in(&text) {
            hits.push(Site {
                file: path.clone(),
                line,
                ledger_row,
            });
        }
    }
}

fn shipped_sites() -> (PathBuf, Vec<Site>) {
    // geom-core sits at <workspace>/crates/geom-core.
    let crates_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates dir")
        .to_path_buf();
    let mut hits = Vec::new();
    for entry in std::fs::read_dir(&crates_root).expect("crates dir listing") {
        let src = entry.expect("dir entry").path().join("src");
        if src.is_dir() {
            count_in_tree(&src, &mut hits);
        }
    }
    hits.sort_by(|a, b| (&a.file, a.line).cmp(&(&b.file, b.line)));
    (crates_root, hits)
}

#[test]
fn shipped_decide_flagged_sites_match_the_ledger() {
    let (_, hits) = shipped_sites();
    assert_eq!(
        hits.len(),
        LEDGER_FLAGGED_SITES,
        "shipped decide_flagged sites diverged from the ledger's inventory \
         (docs/predicate-dimension-audit.md, clause-(i) section; issue #214). \
         A new site needs a ledger row FIRST, then this count moves with it. \
         Sites found: {hits:#?}"
    );
}

#[test]
fn every_shipped_site_cites_a_ledger_row_that_exists() {
    let (crates_root, hits) = shipped_sites();
    let audit = crates_root
        .parent()
        .expect("workspace root")
        .join("docs/predicate-dimension-audit.md");
    let text = std::fs::read_to_string(&audit).expect("readable predicate-dimension audit");
    // Rows are `- **F13** …` bullets; F1's heading carries a
    // parenthetical before the closing `**`, so the number is read up
    // to the first non-digit.
    let rows: BTreeSet<String> = text
        .lines()
        .filter_map(|l| l.strip_prefix("- **F"))
        .map(|r| {
            let digits: String = r.chars().take_while(char::is_ascii_digit).collect();
            format!("F{digits}")
        })
        .filter(|r| r.len() > 1)
        .collect();
    assert!(
        !rows.is_empty(),
        "no `- **FNN**` row bullets found in {}: the ledger's format changed and this check \
         became vacuous",
        audit.display()
    );
    for site in &hits {
        assert!(
            rows.contains(&site.ledger_row),
            "{}:{} cites ledger row `{}`, which {} does not have. The rule the parameter \
             stands for is that a flagged site is argued for in the audit; a citation that \
             resolves to nothing is a string. Rows present: {:?}",
            site.file.display(),
            site.line,
            site.ledger_row,
            audit.display(),
            rows
        );
    }
}
