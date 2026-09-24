//! **The endpoint-read census: every production read of one side of a
//! certification bracket, counted and dispositioned.**
//!
//! # Why this row exists
//!
//! A certification bracket's refusal is its DECORATION (`dec < Def`,
//! asked as `!Interval::is_certified()`), not a NaN pair: a quotient by a
//! divisor not proven away from zero, a negative integer power, and a
//! crossing from a scalar that may not certify all carry **ordinary
//! endpoints**. A consumer that reads one endpoint and compares it
//! therefore takes the certifying branch on a value that does not
//! certify, unless the site asks [`geom_core::Interval::is_certified`] by
//! name — and `Real::is_poison`, which at the interval scalar asks only
//! NaI or empty, is not that question.
//!
//! This row is the sweep, executed: it walks the certification code by
//! one rule and pins, per file, how many production lines read an
//! endpoint and how many of those sit in a function that asks the
//! refusal. **A read added anywhere in that code moves a number here**,
//! so it cannot land without being dispositioned.
//!
//! # What the two numbers mean
//!
//! * `reads` — production lines carrying `.lo()` or `.hi()` in a file of
//!   the population below. It is a LINE count, not a site count: a site
//!   spelled over two lines counts twice, and two reads on one line
//!   count once.
//! * `asking` — of those, the lines whose enclosing function mentions
//!   `is_certified()` anywhere. The refusal being IN the function is what
//!   this can see; that it guards THIS read is the reader's judgement,
//!   at the site.
//!
//! The remainder — `reads − asking` — is dispositioned by hand, and the
//! disposition is named per file in [`ROSTER`] below.
//!
//! # The population
//!
//! Two parts, walked as one:
//!
//! * every `crates/*/src` file whose PRODUCTION code reaches the
//!   certification arithmetic ([`reaches_certification`]): it names the
//!   `geom_core::interval::certification` module — the only route to the
//!   [`Certification`](geom_core::interval::certification::Certification)
//!   doors, which are that trait's and are listed there, not here — or it
//!   crosses into certification (`Interval::from_certified`) or refuses a
//!   bracket (`.is_certified()`). The key is what the code does, read off
//!   the same CODE view the counts are, so a file enters when it starts
//!   building or refusing certification brackets and leaves only when it
//!   stops — never because a comment moved. The module half of the key
//!   is `scripts/gates/certification-doors.sh`'s key, and
//!   [`the_door_importers_are_the_gate_s_allowlist`] holds the two
//!   instruments to one population;
//! * every file in [`HOLDERS`]: the files that hold certification
//!   brackets, by name, whether or not they call a door. A file that
//!   reads a bracket it was handed calls no door, and the door key alone
//!   cannot see it.
//!
//! # Blind spots, stated
//!
//! 1. **Type-blind.** It cannot tell a certification endpoint from a
//!    `Bounds::lo()` on an evaluation scalar, so `ssi/certify.rs`'s
//!    `T: Bounds` reads are counted here and are not this census's
//!    subject at all — `Bounds` is decoration-blind by ratification.
//!    They are in the remainder, named below.
//! 2. **Function-scoped.** A refusal anywhere in the enclosing
//!    function counts, including one that guards a different value.
//!    Narrowing it to "before this line" would mis-read the sites that
//!    bind an endpoint and refuse on the next line, which is the
//!    ordinary spelling here.
//! 3. **A certification endpoint handed to a helper taking `f64`** is no
//!    longer an enclosure where the comparison happens, and no text scan
//!    can follow it.
//! 4. **A read inside a macro body, or split across two lines by
//!    rustfmt**, is missed the same way.
//! 5. The production/test cut: each `#[cfg(test)] mod` BLOCK is skipped
//!    and the walk carries on past it. It runs over
//!    `test_utils::source`'s CODE view — comments and string literals
//!    blanked, newlines kept — and matches the block's brackets with that
//!    module's own `balanced_end`, which is exact over a blanked view and
//!    is why this row rolls no reader of its own
//!    (`crates/test-utils/tests/reader_census.rs`).
//! 6. **A new holder is invisible until it is listed.** A file that
//!    comes to hold a certification bracket without calling a door, and
//!    is not in [`HOLDERS`], is outside the population: a read there
//!    happens and nothing here counts it. No text key can find such a
//!    file, because the bracket's type is also the evaluation scalar —
//!    telling the two apart needs name resolution, a compiler, not a
//!    lexer. The module key is textual too, but it is the one route to
//!    the doors: the trait is sealed, reached only through its module,
//!    and re-exported nowhere, so a door call cannot be written without
//!    the module's name somewhere in the file (a `pub use` of the trait
//!    under another name is the gate's KNOWN GAP 4, and the census
//!    shares it).
//!
//! # Where it lives, and why here
//!
//! In `geom-core/tests/` rather than beside the consumers, because the
//! subject is one type's doors across six crates and no consumer crate
//! can see the others. `geom-core` owns the type.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use test_utils::source;

// Every crate the population walks — it reads every `crates/*/src` —
// and the gate whose allowlist the cross-read row reads.
test_utils::gated_to![
    "crates/bvh/src/",
    "crates/editor-core/src/",
    "crates/geom/src/",
    "crates/geom-brep/src/",
    "crates/geom-core/src/",
    "crates/mesh/src/",
    "crates/pncad/src/",
    "crates/pncad-py/src/",
    "crates/profile/src/",
    "crates/quantity/src/",
    "crates/step-export/src/",
    "crates/step-import/src/",
    "crates/stl/src/",
    "crates/sweep/src/",
    "crates/test-utils/src/",
    "crates/topo/src/",
    "crates/verbs/src/",
    "crates/viewer/src/",
    "scripts/gates/certification-doors.sh",
];

/// The inherent doors whose call puts a file in the population besides
/// the module key: the crossing into certification arithmetic and the
/// predicate that refuses a bracket, as they are spelled at a call site
/// in the CODE view. The trait's doors need no entry: a file cannot call
/// one without naming the module ([`names_the_certification_module`]).
const INHERENT_DOORS: &[&str] = &["Interval::from_certified(", ".is_certified()"];

/// The trait's home: it defines the doors rather than importing them.
const TRAIT_HOME: &str = "crates/geom-core/src/interval/certification.rs";

/// The gate that holds the importers, whose allowlist
/// [`the_door_importers_are_the_gate_s_allowlist`] reads.
const GATE: &str = "scripts/gates/certification-doors.sh";

/// Whether a CODE-view line names the certification module as a path —
/// `certification::` or `certification as`, at an identifier boundary.
/// The gate's `CERT_KEY_RE`, over one line. The bare word is also a
/// struct field elsewhere in the tree (`certification: …`), which is why
/// the key is the path and not the word.
fn names_the_certification_module(line: &str) -> bool {
    let mut rest = line;
    while let Some(at) = rest.find("certification") {
        let before = rest[..at].chars().next_back();
        let after = rest[at + "certification".len()..].trim_start();
        if before.is_none_or(|c| !c.is_alphanumeric() && c != '_')
            && (after.starts_with("::") || after.starts_with("as "))
        {
            return true;
        }
        rest = &rest[at + "certification".len()..];
    }
    false
}

/// Whether a file's production lines reach certification arithmetic:
/// the module key, or an inherent door.
fn reaches_certification(lines: &[String]) -> bool {
    lines.iter().any(|l| {
        names_the_certification_module(l) || INHERENT_DOORS.iter().any(|door| l.contains(door))
    })
}

/// The files that hold certification brackets, walked whether or not
/// their production code calls a door: every `crates/*/src` file that
/// named the certification type when it was a type of its own, plus the
/// two that read its brackets without naming it (the SSI driver's
/// pcurve windows, and the scalar the crossing starts from).
///
/// **A fixed list, not a rule**, and the choice is forced (blind spot 6):
/// deciding whether a file holds certification brackets needs name
/// resolution. What keeps it honest is that each entry's counts are
/// pinned in [`ROSTER`] like every other file's, so a read arriving in
/// one of them reds, and [`every_listed_holder_exists`] reds when an
/// entry names a file that is gone — a list that silently shrinks is
/// the failure a list invites.
const HOLDERS: &[&str] = &[
    "crates/geom-brep/src/offset_fit.rs",
    "crates/geom-brep/src/offset_meters.rs",
    "crates/geom-brep/src/patch_bound.rs",
    "crates/geom-brep/src/props/mod.rs",
    "crates/geom-brep/src/props/quad.rs",
    "crates/geom-brep/src/ssi.rs",
    "crates/geom-brep/src/ssi/certify.rs",
    "crates/geom-brep/src/ssi/enclose.rs",
    "crates/geom-brep/src/ssi/exhaust.rs",
    "crates/geom-core/src/interval.rs",
    "crates/geom-core/src/lib.rs",
    "crates/geom-core/src/real.rs",
    "crates/geom-core/src/spline/algebra.rs",
    "crates/geom-core/src/spline/compose.rs",
    "crates/geom-core/src/spline/compose/patch.rs",
    "crates/geom-core/src/spline/compose/tensor.rs",
    "crates/geom-core/src/spline/hull.rs",
    "crates/geom-core/src/spline/net.rs",
    "crates/geom-core/src/sym/signed.rs",
    "crates/geom/src/curves/nurbs.rs",
    "crates/geom/src/net.rs",
    "crates/geom/src/surfaces/nurbs.rs",
    "crates/mesh/src/chords.rs",
    "crates/mesh/src/nurbs_cert.rs",
    "crates/step-import/src/recognize_curve.rs",
    "crates/topo/src/props.rs",
];

/// One entry per file walked with at least one endpoint read — every
/// `crates/*/src` file whose production code reaches certification
/// arithmetic ([`reaches_certification`]), and every file in [`HOLDERS`]:
/// the path, the production endpoint-read LINES, and how many of those
/// sit in a function that asks `is_certified()`.
///
/// The remainder is dispositioned here, one note per file that has one.
/// Nothing checks these notes — they are prose beside a number the walk
/// re-derives, exactly like the ratification citations on
/// `bounds-allowlist.sh`'s file list, and they are here so a reader who
/// reds this row learns what the numbers were for.
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
        "crates/geom-brep/src/patch_bound.rs",
        2,
        2,
        "both are in `rational_cells`, and only one of them is guarded by the \
         `is_certified()` this counts — blind spot 2. That one is the refined weight \
         licence, which asks by name before reading `lo`. The other is the cell \
         CENTROID, safe because a centre is a translation choice and no enclosure \
         rests on it: the site takes each refined control point's midpoint and \
         contributes `0` for a non-finite one, so a refused point still yields a \
         finite centre and the widened hulls report the trouble",
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
         slab reads NaN at both ends",
    ),
    (
        "crates/geom-brep/src/ssi/certify.rs",
        12,
        4,
        "the 4 that ask are the transversality span-hull window (2: \
         `probe_tube_chart` refuses either window \
         hull by name before reading it — a refused hull is NaI, and a NaN window end \
         would land on the first span), and two `T: Bounds` reads of the pcurve's \
         tangent that share that function and count only by blind spot 2. The other \
         8 are `T: Bounds` reads on the evaluation scalar and not certification \
         endpoints at all — blind spot 1",
    ),
    (
        "crates/geom-brep/src/ssi/enclose.rs",
        10,
        10,
        "`Box3`'s disjointness, containment, centre and split all refuse by name, and \
         so does the mignitude (`zero_free_lower_bound`, 4)",
    ),
    ("crates/geom-brep/src/ssi/exhaust.rs", 1, 1, ""),
    (
        "crates/geom-core/src/interval.rs",
        16,
        2,
        "the type's own body. The 2 that ask are the two refusal doors \
         (`certified_bracket`, `sign_within`). The other 14 are not certification \
         reads at all — blind spot 1: they are the evaluation scalar's own \
         implementation reads of the `DInterval` it wraps (the `Bounds` forwarders, \
         `repr_bits`, `copysign` and the kink selectors), which test NaI and empty \
         themselves and carry the decoration forward rather than certifying",
    ),
    (
        "crates/geom-core/src/interval/certification.rs",
        3,
        3,
        "the doors that read an endpoint (`clamped_to`, `width`, `mag`), each refusing first",
    ),
    ("crates/geom-core/src/spline/compose/tensor.rs", 3, 3, ""),
    ("crates/geom-core/src/sym/signed.rs", 2, 2, ""),
    ("crates/geom/src/curves/nurbs.rs", 4, 4, ""),
    ("crates/mesh/src/chords.rs", 2, 2, ""),
    ("crates/mesh/src/nurbs_cert.rs", 1, 1, ""),
    ("crates/topo/src/props/quad_lane.rs", 5, 5, ""),
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

/// Every `crates/*/src` file whose production code reaches certification
/// arithmetic, and every file in [`HOLDERS`], with its production lines. **The shared lexer's CODE view**, not the raw
/// text: a `.lo()` or a door inside a doc comment or a string literal is
/// neither a read nor a call, and blanking keeps the newlines so a line
/// count over this view is a line count over the file
/// (`crates/test-utils/tests/reader_census.rs` is the ledger this entry
/// sits in).
fn population(root: &std::path::Path) -> Vec<(String, Vec<String>)> {
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
            let lines = production(&source::code_only(&raw));
            let rel = path
                .strip_prefix(root)
                .expect("a walked file lies under the repo root")
                .to_string_lossy()
                .replace('\\', "/");
            if !reaches_certification(&lines) && !HOLDERS.contains(&rel.as_str()) {
                continue;
            }
            out.push((rel, lines));
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
    for (name, lines) in population(&root) {
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
            if lines[start..end]
                .iter()
                .any(|l| l.contains("is_certified()"))
            {
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
        "the certification endpoint-read census moved. This is not a number to \
         silence. A certification bracket's refusal is its DECORATION, so a bracket \
         that may not certify carries ordinary endpoints and a one-sided read of one \
         takes the certifying branch: a new read has to ask `is_certified()` by name, \
         or be argued safe at the site and its file's remainder note above extended. \
         A read that went AWAY needs the same note shortened. The classes and the \
         sweep's blind spots are in this file's docs."
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

/// [`HOLDERS`] is a list kept by hand, so it is held to the tree: an
/// entry whose file was renamed or deleted would drop out of the walk
/// with no count moving, and the population would shrink unannounced.
#[test]
fn every_listed_holder_exists() {
    let root = repo_root();
    let gone: Vec<&str> = HOLDERS
        .iter()
        .copied()
        .filter(|p| !root.join(p).is_file())
        .collect();
    assert!(
        gone.is_empty(),
        "{gone:?} in HOLDERS no longer exist. A renamed file takes its entry with it, \
         under its new path; a deleted one leaves the list, and the certification \
         brackets it held are either gone or now live somewhere that has to be listed"
    );
}

/// The census and the gate are two instruments over one population: the
/// files whose production code names the certification module are
/// exactly `scripts/gates/certification-doors.sh`'s `CERT_IMPORTERS`.
/// Read out of the gate's own text, so neither list can gain or lose a
/// file without the other — and read loudly: an array that parses to
/// nothing is a broken reader, not an empty population.
#[test]
fn the_door_importers_are_the_gate_s_allowlist() {
    let root = repo_root();
    let gate = std::fs::read_to_string(root.join(GATE)).expect("the gate is readable");
    let allowlist = gate_importers(&gate);
    assert!(
        allowlist.len() >= 10,
        "read {} entries out of {GATE}'s CERT_IMPORTERS array — the array moved or its \
         reader broke, and a cross-read over too few entries proves nothing",
        allowlist.len()
    );
    let importers = importers(&root);
    assert!(
        importers.len() >= 10,
        "the walk found {} files naming the certification module — it is reading the \
         wrong tree",
        importers.len()
    );
    assert_eq!(
        importers, allowlist,
        "the files whose production code names geom_core::interval::certification (left) \
         are not {GATE}'s CERT_IMPORTERS (right). The two are one population: a file that \
         imports the certification doors is added to the gate's list in the change that \
         makes it import them, and leaves it in the change that stops"
    );
}

/// `CERT_IMPORTERS=( … )`'s entries, in order, from the gate's text: one
/// path per line, comments and blank lines skipped. An unterminated
/// array is a reader failure, not a short list.
fn gate_importers(gate: &str) -> Vec<String> {
    let start = gate
        .lines()
        .position(|l| l.trim_start().starts_with("CERT_IMPORTERS=("))
        .expect("the gate declares CERT_IMPORTERS=(");
    let mut out = Vec::new();
    for line in gate.lines().skip(start + 1) {
        let t = line.trim();
        if t == ")" {
            let mut sorted = out.clone();
            sorted.sort();
            return sorted;
        }
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        out.push(t.to_string());
    }
    panic!("{GATE}'s CERT_IMPORTERS array is never closed");
}

/// Every `crates/*/src` file whose production code names the
/// certification module, less the trait's home, sorted.
fn importers(root: &std::path::Path) -> Vec<String> {
    let mut out: Vec<String> = population(root)
        .into_iter()
        .filter(|(name, lines)| {
            name != TRAIT_HOME && lines.iter().any(|l| names_the_certification_module(l))
        })
        .map(|(name, _)| name)
        .collect();
    out.sort();
    out
}
