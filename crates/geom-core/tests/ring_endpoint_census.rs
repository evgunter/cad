//! **The endpoint-read census: every production read of one side of a
//! `RingInterval` bracket, counted and dispositioned.**
//!
//! # Why this row exists
//!
//! The ring's refusal used to be a NaN pair, so a consumer that read
//! one endpoint and compared it refused *by accident*: every
//! comparison against NaN is false, `f64::max(NaN, 0.0)` is `0.0`,
//! and `!x.is_finite()` caught it. The refusal is now the decoration
//! (`dec < Def`), and a refused bracket carries **ordinary
//! endpoints** — a quotient by a divisor not proven away from zero, a
//! negative integer power, or a crossing from a scalar that may not
//! certify. At each of those reads the same code now takes the
//! certifying branch on a value that does not certify, unless the
//! site asks [`geom_core::RingInterval::is_poison`] by name.
//!
//! A hand sweep found 31 such sites and missed five on its first
//! pass. This row is the sweep, executed: it walks the same file set
//! by the same rule and pins, per file, how many production lines
//! read an endpoint and how many of those sit in a function that asks
//! the refusal. **A read added anywhere in the ring's consumer set
//! moves a number here**, so it cannot land without being
//! dispositioned.
//!
//! # What the two numbers mean
//!
//! * `reads` — production lines carrying `.lo()` or `.hi()` in a file
//!   that names `RingInterval`. It is a LINE count, not a site count:
//!   a site spelled over two lines counts twice, and two reads on one
//!   line count once. That is the same reading the register was
//!   classified from, kept deliberately so the two are comparable.
//! * `asking` — of those, the lines whose enclosing function mentions
//!   `is_poison()` anywhere. The refusal being IN the function is what
//!   this can see; that it guards THIS read is the reader's judgement,
//!   at the site.
//!
//! The remainder — `reads − asking` — is dispositioned by hand, and
//! the disposition lives with the register
//! (`work/scalar/ring-nan-poison-is-load-bearing-at-unguarded-reads`,
//! and the PR that closed it). Today it is three classes, and they
//! are named per file in `ROSTER` below.
//!
//! # Blind spots, stated
//!
//! 1. **Type-blind.** It cannot tell a ring endpoint from a
//!    `Bounds::lo()` on an evaluation scalar, so `ssi/certify.rs`'s
//!    twelve `T: Bounds` reads are counted here and are not this
//!    census's subject at all — `Bounds` is decoration-blind by
//!    ratification. They are in the remainder, named below.
//! 2. **Function-scoped.** A refusal anywhere in the enclosing
//!    function counts, including one that guards a different value.
//!    Narrowing it to "before this line" would mis-read the sites that
//!    bind an endpoint and refuse on the next line, which is the
//!    ordinary spelling here.
//! 3. **A ring endpoint handed to a helper taking `f64`** is no longer
//!    a ring value where the comparison happens, and no text scan can
//!    follow it. Unchanged from the register's own blind spot.
//! 4. **A read inside a macro body, or split across two lines by
//!    rustfmt**, is missed the same way.
//! 5. The production/test cut is the register's: each `#[cfg(test)]
//!    mod` BLOCK is skipped and the walk carries on past it. It runs
//!    over `test_utils::source`'s CODE view — comments and string
//!    literals blanked, newlines kept — and matches the block's
//!    brackets with that module's own `balanced_end`, which is exact
//!    over a blanked view and is why this row rolls no reader of its
//!    own (`crates/test-utils/tests/reader_census.rs`). The register's
//!    hand command read raw lines and skipped a line beginning `//`;
//!    the two agree file for file on this tree, which is the
//!    cross-check that made the conversion safe.
//! 6. **The population is keyed on the TEXT `RingInterval`.** A file
//!    that reads a ring bracket without naming the type is outside it,
//!    and a file LEAVES it when a prose mention is deleted — which is
//!    an edit no reviewer reads as a census change. The two files in
//!    the tree that are outside the key and still belong are named in
//!    [`ALSO_WALKED`] and walked anyway; a third that arrives is
//!    detected by nothing here.
//!
//! # Where it lives, and why here
//!
//! In `geom-core/tests/` rather than beside the consumers, because the
//! subject is one type's accessor across five crates and no consumer
//! crate can see the others. `geom-core` owns the type, and this suite
//! is gated to the ring's own sources.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use test_utils::source;

test_utils::gated_to![
    "crates/geom-core/src/interval.rs",
    "crates/geom-core/src/ring_interval.rs",
    "crates/geom-brep/src/",
    "crates/geom/src/",
    "crates/mesh/src/",
    "crates/topo/src/",
];

/// Files walked **in addition** to the text-keyed population, each
/// because it reads a ring bracket without naming the type in its own
/// text (blind spot 6).
///
/// **A fixed list, not a type-keyed rule**, and the choice is forced:
/// this walk reads source as text through the shared lexer, and
/// deciding whether an `x.lo()` is a ring read needs name resolution —
/// a compiler, not a lexer. That is blind spot 1 seen from outside a
/// file instead of inside one. The cost is that the list is kept by
/// hand; what keeps it honest is that each entry's counts are pinned in
/// [`ROSTER`] like every other file's, so a read arriving in one of
/// them still reds.
const ALSO_WALKED: &[&str] = &[
    // The SSI driver's transversality read: `wu.hull()` is a
    // `RingInterval` and the file names only the window it came from.
    "crates/geom-brep/src/ssi.rs",
    // The crossing's other end. `Interval`'s endpoints are what
    // `RingInterval::from_certified` carries into the ring, and the
    // file left the text-keyed population the moment its prose mention
    // of the ring went away.
    "crates/geom-core/src/interval.rs",
];

/// One entry per file walked — every file that names `RingInterval`
/// under `crates/*/src`, plus [`ALSO_WALKED`]:
/// the path, the production endpoint-read LINES, and how many of
/// those sit in a function that asks `is_poison()`.
///
/// The remainder is dispositioned here, one note per file that has
/// one. Nothing checks these notes — they are prose beside a number
/// the walk re-derives, exactly like the ratification citations on
/// `bounds-allowlist.sh`'s file list, and they are here so a reader
/// who reds this row learns what the numbers were for.
const ROSTER: &[(&str, usize, usize, &str)] = &[
    (
        "crates/geom-brep/src/offset_fit.rs",
        8,
        8,
        "every read is in `cell_bound`, which refuses to `f64::INFINITY`",
    ),
    (
        "crates/geom-brep/src/offset_meters.rs",
        13,
        13,
        "the mignitude, the two norm assemblies and the curvature join all refuse by name",
    ),
    (
        "crates/geom-brep/src/props/quad.rs",
        17,
        12,
        "the remaining 5 are safe by construction: `cos_step`/`sin_step` and the two \
         half-angle clamps build from `pt` of a finite f64 with nonzero exact divisors, \
         so no operand can leave a domain (argued at each). Every other read goes \
         through `lo_or_refuse`/`hi_or_refuse`/`mid`, which are the guarded ones",
    ),
    (
        "crates/geom-brep/src/ssi.rs",
        8,
        2,
        "the 2 that ask are the pcurve window (`pcurve_windows` refuses both hulls by \
         name before padding them). The other 6 are the two march contexts' slab \
         corners, safe by construction: `SsiDomain::slab` is `Box3::around` of a \
         `Point3<f64>` and an `f64` half-extent, and an `f64`'s refusal IS its NaN — \
         the crossing has no decoration channel to carry a refusal in, so a refused \
         slab reads NaN at both ends exactly as it did before the newtype",
    ),
    (
        "crates/geom-brep/src/ssi/certify.rs",
        16,
        4,
        "the 4 that ask are the mignitude (`zero_free_lower_bound`). Of the other 12, \
         ten are `T: Bounds` reads on the evaluation scalar and not ring endpoints at \
         all — blind spot 1 — and two are the transversality span-hull window, safe \
         because `KnotVector::clamped` refuses degree 0: a window therefore holds at \
         least two coefficients, and `CoeffWindow::hull` folds every one after the \
         first through `RingInterval::hull`, whose refusing guard mints NaI. (The \
         hull guard alone would not do it — the fold seeds `acc` with the first \
         coefficient and would hand a one-coefficient window's refusal straight out \
         with its endpoints intact.)",
    ),
    (
        "crates/geom-brep/src/ssi/enclose.rs",
        6,
        6,
        "`Box3`'s disjointness, containment, centre and split all refuse by name",
    ),
    ("crates/geom-brep/src/ssi/exhaust.rs", 1, 1, ""),
    (
        "crates/geom-core/src/interval.rs",
        17,
        0,
        "not ring reads at all — blind spot 1. Every one is the certification \
         scalar's own `self.0.lo()`/`hi()` on the `DInterval` it wraps, where the \
         refusal is the decoration and `is_certified()` is what reads it. The file \
         is walked because it is the crossing's other end (`ALSO_WALKED`)",
    ),
    (
        "crates/geom-core/src/ring_interval.rs",
        9,
        4,
        "the type's own body: five reads are the accessors and the two trait \
         forwarders, which ARE what every site above calls",
    ),
    ("crates/geom-core/src/spline/compose/tensor.rs", 3, 3, ""),
    ("crates/geom-core/src/sym/signed.rs", 2, 2, ""),
    ("crates/geom/src/curves/nurbs.rs", 4, 4, ""),
    ("crates/mesh/src/chords.rs", 2, 2, ""),
    ("crates/mesh/src/nurbs_cert.rs", 1, 1, ""),
    ("crates/topo/src/props.rs", 8, 8, ""),
];

/// The repo root: this crate's directory, two levels up.
fn repo_root() -> std::path::PathBuf {
    let crate_dir = source::crate_dir(env!("CARGO_MANIFEST_DIR"));
    crate_dir
        .parent()
        .and_then(std::path::Path::parent)
        .map(std::path::Path::to_path_buf)
        .expect("crates/<name> sits two levels under the repo root")
}

/// Every `crates/*/src` file whose text names `RingInterval`.
fn consumer_files(root: &std::path::Path) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let crates = root.join("crates");
    let mut dirs: Vec<std::path::PathBuf> = std::fs::read_dir(&crates)
        .expect("crates/ is readable")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.join("src").is_dir())
        .collect();
    dirs.sort();
    for d in dirs {
        for path in source::rust_sources(&d.join("src")) {
            let raw = std::fs::read_to_string(&path).expect("a readable source file");
            let rel = path
                .strip_prefix(root)
                .expect("a walked file lies under the repo root")
                .to_string_lossy()
                .replace('\\', "/");
            if !raw.contains("RingInterval") && !ALSO_WALKED.contains(&rel.as_str()) {
                continue;
            }
            // **The shared lexer's CODE view**, not the raw text: a
            // `.lo()` inside a doc comment or a string literal is not
            // an endpoint read, and blanking keeps the newlines so a
            // line count over this view is a line count over the file
            // (`crates/test-utils/tests/reader_census.rs` is the
            // ledger this entry sits in).
            let text = source::code_only(&raw);
            out.push((rel, text));
        }
    }
    out.sort();
    out
}

/// The production half of the CODE view: every line outside a
/// `#[cfg(test)] mod` BLOCK, whose extent comes from the shared
/// lexer's [`source::balanced_end`] rather than from counting braces
/// by hand — over a blanked view every brace is a real brace, which is
/// exactly the precondition that helper states.
fn production(code: &str) -> Vec<String> {
    let mut skipped: Vec<(usize, usize)> = Vec::new();
    let mut at = 0usize;
    while let Some(off) = code[at..].find("#[cfg(test)]") {
        let gate = at + off;
        at = gate + "#[cfg(test)]".len();
        // **The gate is not always adjacent to the item.** Further
        // attributes may sit between them — `#[allow(clippy::…)]` on a
        // test module is the tree's commonest spelling, and reading
        // only the next token is what let a whole test module count as
        // production. Each one is skipped through the shared lexer's
        // bracket matcher, over the blanked view where that is exact.
        let mut cursor = at;
        loop {
            cursor = source::skip_ws(code, cursor);
            if !code[cursor..].starts_with("#[") {
                break;
            }
            let Some(close) = source::balanced_end(code, cursor + 1) else {
                break;
            };
            cursor = close + 1;
        }
        // Only a `mod` opens a block this walk carries on past; a
        // `#[cfg(test)]` on a function or a `use` gates no block.
        let item = &code[cursor..];
        if !(item.starts_with("mod ")
            || item.starts_with("pub mod ")
            || item.starts_with("pub(crate) mod ")
            || item.starts_with("pub(super) mod "))
        {
            continue;
        }
        let Some(open) = item.find('{') else { continue };
        let Some(end) = source::balanced_end(code, cursor + open) else {
            continue;
        };
        skipped.push((source::line(code, gate), source::line(code, end)));
        at = end;
    }
    code.lines()
        .enumerate()
        .filter(|(i, _)| {
            let ln = i + 1;
            !skipped.iter().any(|&(a, b)| ln >= a && ln <= b)
        })
        .map(|(_, l)| l.to_string())
        .collect()
}

/// Whether the line declares a function — the census's reading of
/// "enclosing function", which is the nearest such line above.
fn declares_a_fn(line: &str) -> bool {
    let mut rest = line;
    while let Some(at) = rest.find("fn ") {
        let before = rest[..at].chars().next_back();
        if before.is_none_or(|c| !c.is_alphanumeric() && c != '_') {
            return true;
        }
        rest = &rest[at + 3..];
    }
    false
}

#[test]
fn every_production_endpoint_read_is_counted_and_dispositioned() {
    let root = repo_root();
    let mut found: Vec<(String, usize, usize)> = Vec::new();
    for (name, text) in consumer_files(&root) {
        let lines = production(&text);
        let reads: Vec<usize> = (0..lines.len())
            .filter(|i| lines[*i].contains(".lo()") || lines[*i].contains(".hi()"))
            .collect();
        if reads.is_empty() {
            continue;
        }
        let mut asking = 0;
        for &i in &reads {
            let start = (0..=i)
                .rev()
                .find(|j| declares_a_fn(&lines[*j]))
                .unwrap_or(0);
            let end = (i + 1..lines.len())
                .find(|j| declares_a_fn(&lines[*j]))
                .unwrap_or(lines.len());
            if lines[start..end].iter().any(|l| l.contains("is_poison()")) {
                asking += 1;
            }
        }
        found.push((name, reads.len(), asking));
    }
    let pinned: Vec<(String, usize, usize)> = ROSTER
        .iter()
        .map(|(p, reads, asking, _)| ((*p).to_string(), *reads, *asking))
        .collect();
    assert_eq!(
        found, pinned,
        "the ring's endpoint-read census moved. This is not a number to silence. The \
         ring's refusal is its DECORATION, so a bracket that may not certify carries \
         ordinary endpoints and a one-sided read of one takes the certifying branch: \
         a new read has to ask `is_poison()` by name, or be argued safe at the site \
         and its file's remainder note above extended. A read that went AWAY needs the \
         same note shortened. The classes and the sweep's blind spots are in this \
         file's docs."
    );
    // The census is about a population, so it says how big that
    // population is: a walk that found nothing would satisfy every
    // per-file row above by finding no rows at all.
    let total: usize = found.iter().map(|(_, reads, _)| reads).sum();
    assert!(
        total > 60,
        "the walk found only {total} endpoint reads across {} files — it is reading the \
         wrong tree",
        found.len()
    );
}
