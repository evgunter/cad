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
//! 5. The production/test cut is the GATE'S, `scripts/gates/lib.sh`'s,
//!    so the two instruments cannot disagree on which files are
//!    importers: a file a test-only `mod x;` mounts is dropped whole
//!    ([`test_only_mounts`], rustc's placement, `#[path]` and inline
//!    modules included), and inside a file every item under a test-only
//!    `cfg` — a module, a function, a `use`, an `impl` — is dropped line
//!    for line as the gate's reader drops it ([`production`]), which
//!    shares that reader's one-line blind spot (a production item on the
//!    same line as a test-only one's close is dropped with it; rustfmt
//!    never writes one). The key is read per STATEMENT, as the gate
//!    reads it ([`statements`]). It all runs over `test_utils::source`'s
//!    CODE view — comments and string literals blanked, newlines kept —
//!    so every brace and `;` it counts is a real one, and this row rolls
//!    no lexer of its own (`crates/test-utils/tests/reader_census.rs`).
//! 6. **A new holder is invisible until it is listed.** A file that
//!    comes to hold a certification bracket without calling a door, and
//!    is not in [`HOLDERS`], is outside the population: a read there
//!    happens and nothing here counts it. No text key can find such a
//!    file, because the bracket's type is also the evaluation scalar —
//!    telling the two apart needs name resolution, a compiler, not a
//!    lexer. The module key is textual too, but it is the one route to
//!    the doors: the trait is sealed, reached only through its module,
//!    and re-exported nowhere — the gate's REEXPORT reds a `pub … use`
//!    of it in any production file — so a door call cannot be written
//!    without the module's name somewhere in the file. The routes the
//!    gate states it cannot see (its KNOWN GAPs 4 and 5: a public
//!    subtrait, a macro-written path) this census cannot see either.
//!
//! # Where it lives, and why here
//!
//! In `geom-core/tests/` rather than beside the consumers, because the
//! subject is one type's doors across six crates and no consumer crate
//! can see the others. `geom-core` owns the type.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use test_utils::source;

// Every crate the population walks — it reads every `crates/*/src` —
// and the gate whose allowlist the cross-read row reads; held to the
// walk by `the_suite_is_gated_to_every_crate_it_walks`.
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

/// Whether a CODE-view statement names the certification module as a
/// path — `certification::`, or `certification as` followed by
/// whitespace, at an identifier boundary, with any whitespace between
/// the word and what follows it. The gate's `CERT_KEY_RE`, over the
/// same statement it reads ([`statements`]). The bare word is also a
/// struct field elsewhere in the tree (`certification: …`), which is why
/// the key is the path and not the word.
fn names_the_certification_module(statement: &str) -> bool {
    let mut rest = statement;
    while let Some(at) = rest.find("certification") {
        let after = rest[at + "certification".len()..].trim_start();
        if source::boundary_before(rest, at)
            && (after.starts_with("::")
                || after
                    .strip_prefix("as")
                    .and_then(|a| a.chars().next())
                    .is_some_and(char::is_whitespace))
        {
            return true;
        }
        rest = &rest[at + "certification".len()..];
    }
    false
}

/// A file's production lines as `lib.sh`'s STATEMENT view has them: cut
/// at every `{`, `}` and `;`, each run of whitespace (newlines included)
/// one space. This is the view the gate keys on, so a path split across
/// lines — `certification` on one, `::Certification` on the next — is
/// one statement to both instruments rather than a key the gate reads
/// and this row does not.
fn statements(lines: &[String]) -> Vec<String> {
    lines
        .join("\n")
        .split(['{', '}', ';'])
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| !s.is_empty())
        .collect()
}

/// Whether a file's production statements reach certification
/// arithmetic: the module key, or an inherent door.
fn reaches_certification(lines: &[String]) -> bool {
    // Every needle below holds one of these identifiers whole, and no
    // identifier is split across lines, so a file without one cannot
    // match and is not joined into statements at all.
    if !lines
        .iter()
        .any(|l| l.contains("certification") || l.contains("_certified"))
    {
        return false;
    }
    statements(lines).iter().any(|s| {
        names_the_certification_module(s) || INHERENT_DOORS.iter().any(|door| s.contains(door))
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
    "crates/topo/src/props/quad_lane.rs",
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
    let sources = production_sources(root);
    let mut out = Vec::new();
    for (rel, code) in sources {
        let lines = production(&code);
        if !reaches_certification(&lines) && !HOLDERS.contains(&rel.as_str()) {
            continue;
        }
        out.push((rel, lines));
    }
    out.sort();
    out
}

/// Every `crates/*/src` file, repo-relative with `/` separators, with
/// its CODE view — less the files a test-only `mod x;` mounts
/// ([`test_only_mounts`]), which are test code however they read: the
/// production set `scripts/gates/lib.sh`'s `gate_production_sources`
/// hands the gate.
fn production_sources(root: &std::path::Path) -> Vec<(String, String)> {
    let crates = root.join("crates");
    let mut dirs: Vec<std::path::PathBuf> = std::fs::read_dir(&crates)
        .expect("crates/ is readable")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.join("src").is_dir())
        .collect();
    dirs.sort();
    let mut all = Vec::new();
    for d in dirs {
        for path in source::rust_sources(&d.join("src")) {
            let raw = std::fs::read_to_string(&path).expect("a readable source file");
            let rel = path
                .strip_prefix(root)
                .expect("a walked file lies under the repo root")
                .to_string_lossy()
                .replace('\\', "/");
            let code = source::code_only(&raw);
            all.push((rel, raw, code));
        }
    }
    let mut files = Vec::new();
    let mut dirs = Vec::new();
    for (rel, raw, code) in &all {
        for (target, own) in test_only_mounts(rel, raw, code) {
            if !own {
                files.push(target.clone());
            }
            dirs.push(format!("{}/", target.trim_end_matches(".rs")));
        }
    }
    all.into_iter()
        .filter(|(rel, _, _)| {
            !files.contains(rel) && !dirs.iter().any(|d| rel.starts_with(d.as_str()))
        })
        .map(|(rel, _, code)| (rel, code))
        .collect()
}

/// Whether a CODE-view text carries a TEST-ONLY `cfg` attribute:
/// `lib.sh`'s `GATE_CFG_TEST_RE` less its `GATE_CFG_TEST_NOT_RE`, one
/// predicate in two languages. `test` counts alone or inside an
/// `all(…)` — directly after `cfg(`, or after a `(` or a `,` and any
/// whitespace, and followed by `,` or `)`, within the attribute's first
/// `]` — and an attribute holding `any(` or `not(` anywhere in that
/// span refuses the whole text: `not(test)` marks production code and
/// `any(test, …)` an item that also exists without the test build.
fn is_test_only_cfg(text: &str) -> bool {
    let bodies: Vec<&str> = text
        .match_indices("#[cfg(")
        .map(|(at, open)| {
            let body = &text[at + open.len()..];
            &body[..body.find(']').unwrap_or(body.len())]
        })
        .collect();
    let test_at = |body: &str| {
        body.match_indices("test").any(|(q, _)| {
            let closes = matches!(body[q + 4..].chars().next(), Some(',' | ')'));
            let opens =
                q == 0 || matches!(body[..q].trim_end().chars().next_back(), Some('(' | ','));
            closes && opens
        })
    };
    bodies.iter().any(|b| test_at(b))
        && !bodies
            .iter()
            .any(|b| b.contains("any(") || b.contains("not("))
}

/// The production half of the CODE view, cut the way `lib.sh`'s reader
/// cuts it under `--skip-cfg-test`, line for line: a line carrying a
/// test-only attribute ([`is_test_only_cfg`]) opens a skip at the
/// current brace depth, and the skip ends on the line where the braces
/// it opened close again — or, if no brace opened, on the first line
/// with a `;`. So a `#[cfg(test)]` module, function, `use`, `impl` or
/// `const` is dropped whole, and so is anything that shares its last
/// line (the reader's own property, and its one-line blind spot). Over
/// a blanked view every brace is a real brace.
fn production(code: &str) -> Vec<String> {
    let mut out = Vec::new();
    let (mut depth, mut skip_depth) = (0i64, 0i64);
    let (mut skipping, mut seen_open) = (false, false);
    for line in code.lines() {
        let opens = i64::try_from(line.matches('{').count()).expect("a line's brace count");
        let closes = i64::try_from(line.matches('}').count()).expect("a line's brace count");
        if !skipping && is_test_only_cfg(line) {
            skipping = true;
            seen_open = false;
            skip_depth = depth;
        }
        if skipping {
            seen_open |= opens > 0;
            depth += opens - closes;
            if (seen_open && depth <= skip_depth) || (!seen_open && line.contains(';')) {
                skipping = false;
            }
            continue;
        }
        depth += opens - closes;
        out.push(line.to_string());
    }
    out
}

/// Where each test-only `mod x;` in one file mounts its module, as
/// `lib.sh`'s `gate_test_only_mounts` places it: the target file, and
/// whether that file is the declarer itself (`mod lib;` in `lib.rs`,
/// which excludes only the directory form). Read over the CODE view,
/// cut at `{`, `}` and `;` like the statement view, so the attributes a
/// declaration carries are the text since the last delimiter.
///
/// Rustc's resolution: `mod bar;` in a crate root or a `mod.rs` names
/// the sibling `dir/bar.rs`, in any other `dir/foo.rs` it names
/// `dir/foo/bar.rs`; each enclosing inline `mod y { … }` adds `y/`; and
/// a `#[path = "P"]` names `P`, relative to the declaring file's
/// directory at top level and to the inline module's directory inside
/// one. A declaration under a brace that is not a module is refused
/// loudly, as the gate refuses it: rustc mounts such a module only
/// through `#[path]`, and a mount this row cannot place is a production
/// set it does not know.
fn test_only_mounts(rel: &str, raw: &str, code: &str) -> Vec<(String, bool)> {
    if !code.contains("#[cfg(") {
        return Vec::new();
    }
    let dir = rel.rsplit_once('/').map_or("", |(d, _)| d);
    let file_name = rel.rsplit('/').next().unwrap_or(rel);
    let positional = if matches!(file_name, "mod.rs" | "lib.rs" | "main.rs") {
        dir.to_string()
    } else {
        rel.trim_end_matches(".rs").to_string()
    };
    let mut chain: Vec<Option<String>> = Vec::new();
    let mut pending: Option<String> = None;
    let mut start = 0usize;
    let mut out = Vec::new();
    let b = code.as_bytes();
    let mut i = 0usize;
    while i < b.len() {
        match b[i] {
            b'{' => {
                chain.push(pending.take());
                start = i + 1;
            }
            b'}' => {
                chain.pop();
                pending = None;
                start = i + 1;
            }
            b';' => {
                let stmt = &code[start..i];
                if let Some(name) = pending.take()
                    && is_test_only_cfg(stmt)
                {
                    let Some(names) = chain.iter().cloned().collect::<Option<Vec<String>>>() else {
                        panic!(
                            "{rel} declares a test-only `mod {name};` inside a brace that is not a \
                             module, where rustc mounts a non-inline module only through #[path]; \
                             where it lives was not decided, and neither is this row's production set"
                        );
                    };
                    let inner = names.join("/");
                    let target = if stmt.contains("#[path") {
                        let lits = source::blanked(source::code_and_literals, rel, raw);
                        let payload = path_payload(&lits[start..i]).unwrap_or_else(|| {
                            panic!(
                                "{rel}'s test-only `mod {name};` has a #[path] whose payload is \
                                 not a plain literal"
                            )
                        });
                        let base = if inner.is_empty() {
                            dir.to_string()
                        } else {
                            format!("{positional}/{inner}")
                        };
                        normalized(&format!("{base}/{payload}"))
                    } else if inner.is_empty() {
                        format!("{positional}/{name}.rs")
                    } else {
                        format!("{positional}/{inner}/{name}.rs")
                    };
                    let own = target == rel;
                    out.push((target, own));
                }
                pending = None;
                start = i + 1;
            }
            b'm' if code[i..].starts_with("mod")
                && source::boundary_before(code, i)
                && code[i + 3..].starts_with(char::is_whitespace) =>
            {
                let after = source::skip_ws(code, i + 3);
                let name = source::ident(code, after);
                if !name.is_empty() {
                    pending = Some(name.to_string());
                    i = after + name.len();
                    continue;
                }
            }
            _ => {}
        }
        i += 1;
    }
    out
}

/// The payload of the first `#[path = "…"]` in a literal-keeping slice.
fn path_payload(lits: &str) -> Option<&str> {
    let at = lits.find("#[path")?;
    let rest = lits[at + "#[path".len()..].trim_start().strip_prefix('=')?;
    let end = rest.find(']')?;
    source::plain_string_literal(&rest[..end])
}

/// A `/`-joined path with its `.` and `..` segments taken out, as
/// `lib.sh`'s `gate_norm_path` has it: an exclusion is compared to the
/// walked paths as text, and an unnormalised one excludes nothing.
fn normalized(path: &str) -> String {
    let mut seg: Vec<&str> = Vec::new();
    for s in path.split('/') {
        match s {
            "" | "." => {}
            ".." if seg.last().is_some_and(|l| *l != "..") => {
                seg.pop();
            }
            s => seg.push(s),
        }
    }
    seg.join("/")
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

/// This file, whose `gated_to!` [`the_suite_is_gated_to_every_crate_it_walks`]
/// reads.
const SELF: &str = "crates/geom-core/tests/certified_endpoint_census.rs";

/// The `gated_to!` at the top of this file is a list kept by hand, and
/// the change filter skips this suite on a pull request that touches
/// nothing it names. The walk reads EVERY `crates/*/src`, so a crate
/// added to the tree is walked from the day it lands — and, were the
/// list trusted, not gated: a certification read written in it would
/// wait for the nightly. So the list is held to the walk: exactly one
/// `crates/<name>/src/` per crate directory the walk reads, plus the
/// gate the cross-read row reads.
#[test]
fn the_suite_is_gated_to_every_crate_it_walks() {
    let root = repo_root();
    let text =
        std::fs::read_to_string(root.join(SELF)).expect("this suite's own source is readable");
    let code = source::blanked(source::code_only, SELF, &text);
    let lits = source::blanked(source::code_and_literals, SELF, &text);
    let marks: Vec<usize> = code.match_indices("gated_to!").map(|(at, _)| at).collect();
    assert_eq!(
        marks.len(),
        1,
        "{SELF} carries {} gated_to! invocations, not one",
        marks.len()
    );
    let open = source::skip_ws(&code, marks[0] + "gated_to!".len());
    let end = source::balanced_end(&code, open).expect("the gated_to! invocation closes");
    let mut named: Vec<String> = source::top_level_split(&code[open + 1..end], ',')
        .into_iter()
        .map(|r| {
            lits[open + 1 + r.start..open + 1 + r.end]
                .trim()
                .to_string()
        })
        .filter(|s| !s.is_empty())
        .map(|s| {
            source::plain_string_literal(&s)
                .unwrap_or_else(|| panic!("a gated_to! entry that is not a plain literal: {s}"))
                .to_string()
        })
        .collect();
    named.sort();
    let mut walked: Vec<String> = std::fs::read_dir(root.join("crates"))
        .expect("crates/ is readable")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.join("src").is_dir())
        .map(|p| {
            format!(
                "crates/{}/src/",
                p.file_name()
                    .expect("a crate directory has a name")
                    .to_string_lossy()
            )
        })
        .chain(std::iter::once(GATE.to_string()))
        .collect();
    walked.sort();
    assert!(
        walked.len() > 10,
        "the walk found {} crates — it is reading the wrong tree",
        walked.len()
    );
    assert_eq!(
        named, walked,
        "{SELF}'s gated_to! list, on the left, is not every crates/*/src this census walks plus {GATE} \
         (on the right). A crate the walk reads and the list does not name is walked on every run \
         and gated on none that touches it: name it in the list"
    );
}

/// `CERT_IMPORTERS=( … )`'s entries, sorted, from the gate's text, read
/// as bash reads an array body: each line's words up to the first one
/// that opens a `# comment`, with a quoted word's quotes taken off — so
/// an entry carrying a trailing note, or quoted, is the path bash sees
/// and not a false red. An unterminated array is a reader failure, not
/// a short list.
fn gate_importers(gate: &str) -> Vec<String> {
    let start = gate
        .lines()
        .position(|l| l.trim_start().starts_with("CERT_IMPORTERS=("))
        .expect("the gate declares CERT_IMPORTERS=(");
    let mut out = Vec::new();
    for line in gate.lines().skip(start + 1) {
        if line.trim() == ")" {
            out.sort();
            return out;
        }
        out.extend(
            line.split_whitespace()
                .take_while(|w| !w.starts_with('#'))
                .map(|w| w.trim_matches(|c| c == '\'' || c == '"').to_string()),
        );
    }
    panic!("{GATE}'s CERT_IMPORTERS array is never closed");
}

/// Every `crates/*/src` file whose production code names the
/// certification module, less the trait's home, sorted.
fn importers(root: &std::path::Path) -> Vec<String> {
    let mut out: Vec<String> = population(root)
        .into_iter()
        .filter(|(name, lines)| {
            name != TRAIT_HOME
                && statements(lines)
                    .iter()
                    .any(|s| names_the_certification_module(s))
        })
        .map(|(name, _)| name)
        .collect();
    out.sort();
    out
}
