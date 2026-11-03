//! **`[topo.homes]`'s engine-door row** (`crates/topo/README.md`): an
//! operation's directory (`boolean/`, `splitting/`) is a leaf. Over
//! every file of `topo/src`, production and test code alike:
//!
//! - no file outside an engine's directory names that engine, bar a
//!   `pub use` in `lib.rs` (its doors' re-exports);
//! - neither engine names the other;
//! - nothing inside an engine is visible to the whole crate without
//!   being `pub`: `pub(crate)`, `pub(in crate)`, and a `pub(super)` or
//!   `pub(in super…)` that climbs out of the engine.
//!
//! Today's violations are the committed allowlist beside this file,
//! `engine_leaves.txt`, which is the move list and may only shrink.
//!
//! # What counts as naming an engine
//!
//! The read is over [`CodeOnly`], so prose, doc comments (and the
//! doctests in them) and string literals never count. A site is one
//! path whose segment `boolean` (or `splitting`) is
//!
//! - led only by `crate`, `$crate`, `self` and `super` segments,
//!   counting the prefixes of every enclosing `use` group
//!   (`use crate::{boolean::X, splitting::{Y, Z}}` is two sites), or
//! - bare and followed by `::` (`boolean::X` through an import or a
//!   glob, or at the crate root).
//!
//! An engine type spelled through its root re-export names the engine
//! as much as its full path does, so each name `lib.rs`'s top-level
//! `pub use` exports from an engine (under its root name, `as` renames
//! read) is a site wherever a file outside that engine spells it
//! `crate::Name` or brings it in as an item of a `crate::{…}` group,
//! renamed or not. Those sites are their own kinds (`…-via-root`), so
//! respelling `crate::boolean::X` as `crate::X` moves a count between
//! two lines and reds both. `lib.rs` itself is not read for them.
//!
//! A `use` naming the engine module itself (`use crate::boolean;`,
//! `use crate::boolean as b;`) is its own site. Any `super` chain counts
//! wherever it lands, which is sound because `lib.rs` declares the only
//! module of either name; the row asserts that.
//!
//! **Blind spots.** A path an alias reaches (`b::X` after
//! `use crate::boolean as b;`, or `Op` after `use crate::BooleanOp as
//! Op;`) is not counted again; the alias's `use` is. A root name
//! reached through a glob (`use crate::*;`, or `use super::*;` at the
//! crate root's children) or through `super::Name` is not seen. Macros
//! are not expanded, so a path a macro builds from pieces is not seen.
//! `topo/tests/`, doctests and other crates reach the engines only
//! through `topo::…`, as consumers of the doors, and are outside this
//! walk. A `#[path]` module's depth is read from its file's path, so a
//! `pub(super)` in one can read as climbing out of the engine when it
//! does not: over-reporting, never under.

use super::{CodeOnly, crate_sources, src_root, tokens};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::Path;

/// The two operation directories `[topo.homes]` names.
const ENGINES: [&str; 2] = ["boolean", "splitting"];

/// The allowlist's name beside this file, under `src/`.
const ALLOWLIST: &str = "source_walk/engine_leaves.txt";

/// The names each engine exports through the crate root, by engine.
type RootNames = BTreeMap<&'static str, BTreeSet<String>>;

/// One allowlist key: the violation's kind and the file it is in,
/// relative to `src/`.
type Key = (String, String);

/// Every violation in `topo/src`, counted per (kind, file).
fn measure() -> BTreeMap<Key, usize> {
    let root = src_root();
    let lib = std::fs::read_to_string(root.join("lib.rs")).expect("a readable lib.rs");
    let exports = root_exports(CodeOnly::of(&lib).as_str());
    let mut out = BTreeMap::new();
    let mut declared = 0;
    for path in crate_sources() {
        let rel = relative(&root, &path);
        let text = std::fs::read_to_string(&path).expect("a readable source file");
        let code = CodeOnly::of(&text);
        let code = code.as_str();
        for engine in ENGINES {
            for at in tokens(code, "mod") {
                let rest = &code[at + 3..];
                if rest.trim_start().starts_with(engine)
                    && tokens(rest.trim_start(), engine).next() == Some(0)
                {
                    assert_eq!(
                        rel, "lib.rs",
                        "`{rel}` declares a module named `{engine}`: this row reads any \
                         `super` chain ending in `{engine}` as `crate::{engine}`, which \
                         holds only while `lib.rs` declares the one module of that name"
                    );
                    declared += 1;
                }
            }
        }
        for (kind, count) in violations(&rel, code, &exports) {
            *out.entry((kind, rel.clone())).or_insert(0) += count;
        }
    }
    assert_eq!(
        declared, 2,
        "`lib.rs` should declare `mod boolean` and `mod splitting` once each"
    );
    out
}

/// `path` relative to `root`, `/`-separated.
fn relative(root: &Path, path: &Path) -> String {
    let rel = path.strip_prefix(root).expect("a file under src/");
    rel.components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/")
}

/// The engine whose directory holds the file at `rel`, if any.
fn home(rel: &str) -> Option<&'static str> {
    ENGINES
        .into_iter()
        .find(|e| rel.strip_prefix(e).is_some_and(|r| r.starts_with('/')))
}

/// The violations of one file's blanked code, by kind.
fn violations(rel: &str, code: &str, exports: &RootNames) -> BTreeMap<String, usize> {
    let home = home(rel);
    let reexports = if rel == "lib.rs" {
        pub_use_spans(code)
    } else {
        Vec::new()
    };
    let mut out = BTreeMap::new();
    for engine in ENGINES {
        if home == Some(engine) {
            continue;
        }
        let n = engine_sites(code, engine)
            .into_iter()
            .filter(|at| !reexports.iter().any(|r| r.contains(at)))
            .count();
        let kind = match home {
            Some(h) => format!("{h}-names-{engine}"),
            None => format!("names-{engine}"),
        };
        if n > 0 {
            out.insert(kind.clone(), n);
        }
        if rel != "lib.rs" {
            let n = root_sites(code, &exports[engine]);
            if n > 0 {
                out.insert(format!("{kind}-via-root"), n);
            }
        }
    }
    if home.is_some() {
        let n = crate_wide(rel, code);
        if n > 0 {
            out.insert("crate-visible".to_string(), n);
        }
    }
    out
}

/// The byte ranges of every `pub use …;` declaration in `code`: a
/// bare `pub`, not `pub(crate)`.
fn pub_use_spans(code: &str) -> Vec<std::ops::Range<usize>> {
    tokens(code, "use")
        .filter(|&at| {
            let before = code[..at].trim_end();
            before.ends_with("pub") && tokens(before, "pub").last() == Some(before.len() - 3)
        })
        .map(|at| at..code[at..].find(';').map_or(code.len(), |s| at + s))
        .collect()
}

/// The names `lib.rs`'s top-level `pub use` declarations export from
/// each engine, under the name they take at the root (an `as` rename's
/// new name). A glob re-export would make this set the engine's own
/// `pub` items, which this reader does not collect, so it refuses one.
fn root_exports(lib: &str) -> RootNames {
    let nested = inline_mods(lib);
    let mut out: RootNames = ENGINES.into_iter().map(|e| (e, BTreeSet::new())).collect();
    for span in pub_use_spans(lib) {
        if nested.iter().any(|m| m.contains(&span.start)) {
            continue;
        }
        let tree = &lib[span.start + 3..span.end];
        for (path, name) in flatten(tree) {
            let path: Vec<&str> = path
                .iter()
                .map(String::as_str)
                .skip_while(|s| matches!(*s, "crate" | "self"))
                .collect();
            let Some(engine) = ENGINES.into_iter().find(|e| path.first() == Some(e)) else {
                continue;
            };
            assert!(
                name != "*",
                "lib.rs glob-re-exports from `{engine}`: this row reads the root's engine \
                 names off its named re-exports, so teach it the engine's own `pub` items"
            );
            if path.len() > 1 && name != "_" && name != "self" {
                out.get_mut(engine).expect("an engine").insert(name);
            }
        }
    }
    for (engine, names) in &out {
        assert!(
            !names.is_empty(),
            "lib.rs re-exports nothing from `{engine}`: the read is lost"
        );
    }
    out
}

/// One `use` tree, flattened: each leaf's full path and the name it
/// binds (its `as` rename, else its last segment).
fn flatten(tree: &str) -> Vec<(Vec<String>, String)> {
    let tree = tree.trim();
    if let Some(open) = tree.find('{') {
        let head = path_segments(&tree[..open]);
        let close = test_utils::source::balanced_end(tree, open).expect("a closed use group");
        let inner = &tree[open + 1..close];
        return test_utils::source::top_level_split(inner, ',')
            .into_iter()
            .filter(|r| !inner[r.clone()].trim().is_empty())
            .flat_map(|r| flatten(&inner[r]))
            .map(|(tail, name)| ([head.clone(), tail].concat(), name))
            .collect();
    }
    let tree = tree.split_whitespace().collect::<Vec<_>>().join(" ");
    let (path, alias) = match tree.split_once(" as ") {
        Some((p, a)) => (p, Some(a.trim().to_string())),
        None => (tree.as_str(), None),
    };
    let path = path_segments(path);
    let name = alias.unwrap_or_else(|| path.last().cloned().unwrap_or_default());
    vec![(path, name)]
}

/// The `::`-separated segments of a path fragment, empties dropped.
fn path_segments(text: &str) -> Vec<String> {
    text.split("::")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}

/// How many paths in `code` name one of `names` through the crate
/// root: `crate::Name`, or `Name` (renamed or not) as an item of a
/// `crate::{…}` group.
fn root_sites(code: &str, names: &BTreeSet<String>) -> usize {
    let first = |item: &str| -> bool {
        let item = item.trim();
        let end = item
            .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .unwrap_or(item.len());
        names.contains(&item[..end])
    };
    tokens(code, "crate")
        .map(|at| {
            let Some(rest) = code[at + 5..].trim_start().strip_prefix("::") else {
                return 0;
            };
            let rest = rest.trim_start();
            if rest.starts_with('{') {
                let open = code.len() - rest.len();
                let close = test_utils::source::balanced_end(code, open).expect("a closed group");
                let inner = &code[open + 1..close];
                test_utils::source::top_level_split(inner, ',')
                    .into_iter()
                    .filter(|r| first(&inner[r.clone()]))
                    .count()
            } else {
                usize::from(first(rest))
            }
        })
        .sum()
}

/// Every offset in `code` where a path names `crate::<engine>`, by the
/// rule in the module docs.
fn engine_sites(code: &str, engine: &str) -> Vec<usize> {
    tokens(code, engine)
        .filter(|&at| {
            let after = code[at + engine.len()..].trim_start();
            let before = code[..at].trim_end();
            if before.ends_with('.') {
                return false;
            }
            match path_prefix(code, at) {
                None => false,
                Some(prefix) if prefix.is_empty() => after.starts_with("::"),
                Some(_) => true,
            }
        })
        .collect()
}

/// The segments leading the path segment at `at`, enclosing `use`
/// groups' prefixes included, when every one of them is `crate`,
/// `$crate`, `self` or `super`; `None` when any is something else (a
/// type, another crate, a leading `::`).
fn path_prefix(code: &str, at: usize) -> Option<Vec<&str>> {
    let mut prefix = Vec::new();
    let mut cursor = at;
    loop {
        let before = code[..cursor].trim_end();
        if let Some(head) = before.strip_suffix("::") {
            let head = head.trim_end();
            let start = head
                .rfind(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '$'))
                .map_or(0, |i| i + 1);
            let seg = &head[start..];
            if !matches!(seg, "crate" | "$crate" | "self" | "super") {
                return None;
            }
            prefix.push(seg);
            cursor = start;
            continue;
        }
        if before.ends_with('{') || before.ends_with(',') {
            // Inside a `use` group only when the unmatched `{` that
            // holds this item follows a `::`.
            match group_open(code, before.len()) {
                Some(open) if code[..open].trim_end().ends_with("::") => {
                    cursor = open;
                    continue;
                }
                _ => return Some(prefix),
            }
        }
        return Some(prefix);
    }
}

/// The offset of the innermost unmatched opener before `end`, when it
/// is a `{`.
fn group_open(code: &str, end: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (i, c) in code[..end].char_indices().rev() {
        match c {
            ')' | ']' | '}' => depth += 1,
            '(' | '[' | '{' if depth > 0 => depth -= 1,
            '{' => return Some(i),
            '(' | '[' => return None,
            _ => {}
        }
    }
    None
}

/// How many visibility qualifiers in an engine file's `code` reach the
/// whole crate without being `pub`.
fn crate_wide(rel: &str, code: &str) -> usize {
    let own = home(rel).expect("an engine file");
    let file_depth = module_depth(rel);
    let inline = inline_mods(code);
    tokens(code, "pub")
        .filter(|&at| {
            let rest = code[at + 3..].trim_start();
            let Some(inner) = rest.strip_prefix('(') else {
                return false;
            };
            let inner: String = inner[..inner.find(')').expect("a closed `pub(`")]
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            let scope = inner.strip_prefix("in ").unwrap_or(&inner);
            let segs: Vec<&str> = scope.split("::").map(str::trim).collect();
            match segs[0] {
                "crate" => segs.get(1) != Some(&own),
                "self" => false,
                "super" => {
                    let depth = file_depth + inline.iter().filter(|r| r.contains(&at)).count();
                    let ups = segs.iter().take_while(|s| **s == "super").count();
                    ups >= depth
                }
                other => panic!("`{rel}`: a visibility this row cannot read: `pub({other}…)`"),
            }
        })
        .count()
}

/// The module depth below the crate root of the file at `rel`, read
/// from its path: `boolean/mod.rs` is 1, `boolean/zip.rs` 2.
fn module_depth(rel: &str) -> usize {
    let parts = rel.split('/').count();
    if rel.ends_with("/mod.rs") {
        parts - 1
    } else {
        parts
    }
}

/// The byte ranges of every inline `mod name { … }` in `code`.
fn inline_mods(code: &str) -> Vec<std::ops::Range<usize>> {
    tokens(code, "mod")
        .filter_map(|at| {
            let name = code[at + 3..].trim_start();
            let after = name.trim_start_matches(|c: char| c.is_ascii_alphanumeric() || c == '_');
            let open = code.len() - after.trim_start().len();
            after.trim_start().starts_with('{').then(|| {
                let close = test_utils::source::balanced_end(code, open).expect("a closed `mod {`");
                open..close
            })
        })
        .collect()
}

/// The committed allowlist, keyed as [`measure`] keys. Blank lines and
/// `#` lines are skipped; any other line is `kind count file`,
/// whitespace-separated.
fn allowlist() -> BTreeMap<Key, usize> {
    let path = src_root().join(ALLOWLIST);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the allowlist {} is unreadable: {e}", path.display()));
    let mut out = BTreeMap::new();
    for (n, line) in text.lines().enumerate() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = line.split_whitespace().collect();
        let [kind, count, file] = cols[..] else {
            panic!("{ALLOWLIST}:{}: not `kind count file`: {line:?}", n + 1);
        };
        let count: usize = count
            .parse()
            .unwrap_or_else(|_| panic!("{ALLOWLIST}:{}: count {count:?} is no number", n + 1));
        let fresh = out.insert((kind.to_string(), file.to_string()), count);
        assert!(
            fresh.is_none(),
            "{ALLOWLIST}:{}: a second line for {kind} {file}",
            n + 1
        );
    }
    out
}

/// What every red here says: where the rule is, and where the code
/// that tripped it belongs instead.
const WHERE_IT_GOES: &str = "\
`[topo.homes]` (crates/topo/README.md) makes `boolean/` and `splitting/` leaves: nothing \
outside an engine names it, neither names the other, and an engine declares nothing \
`pub(crate)` (its `pub` items are its doors, re-exported by lib.rs; `pub(super)` or \
`pub(in crate::<engine>)` for its own use). What you are reaching for has a home by its \
signature: no `Body` and no arena key -> `geom-brep`; reads or constructs over a `Body` and \
names no type declared under an engine -> a crate-root module or a `Body` method; names an \
engine's type -> that engine.";

/// **The engine directories are leaves** (`[topo.homes]`), against a
/// committed allowlist that only shrinks: a site past its line reds, and
/// so does a line the code no longer reaches.
#[test]
fn engine_directories_are_leaves() {
    let measured = measure();
    let allowed = allowlist();
    let mut grown = Vec::new();
    let mut stale = Vec::new();
    for (key, &n) in &measured {
        let listed = allowed.get(key).copied().unwrap_or(0);
        if n > listed {
            grown.push(format!(
                "  {:<32} {}: {n} site(s), {listed} allowed",
                key.0, key.1
            ));
        }
    }
    for (key, &listed) in &allowed {
        let n = measured.get(key).copied().unwrap_or(0);
        if n < listed {
            stale.push(format!(
                "  {:<32} {}: {listed} allowed, {n} left — {}",
                key.0,
                key.1,
                if n == 0 {
                    "drop the line".to_string()
                } else {
                    format!("lower it to {n}")
                }
            ));
        }
    }
    let mut table = String::new();
    let mut last = "";
    for ((kind, file), n) in &measured {
        if kind != last {
            table.push('\n');
            last = kind;
        }
        writeln!(table, "{kind:<32} {n:>4}  {file}").expect("a String takes a write");
    }
    assert!(
        grown.is_empty(),
        "new engine-door violations past src/{ALLOWLIST}:\n{}\n\n{WHERE_IT_GOES}\n\n\
         The allowlist only shrinks; do not raise it.",
        grown.join("\n")
    );
    assert!(
        stale.is_empty(),
        "src/{ALLOWLIST} lists more than the code still does — the move landed, so the \
         list shrinks with it:\n{}\n\n{WHERE_IT_GOES}\n\nToday's table, to paste in place \
         of the listed lines:\n{table}",
        stale.join("\n")
    );
}

/// The path reader and the visibility reader, one construct per row.
#[test]
fn the_reader_counts_engine_paths_and_crate_wide_visibility() {
    let sites = |text: &str| engine_sites(CodeOnly::of(text).as_str(), "boolean").len();
    for (text, n) in [
        ("use crate::boolean::X;", 1),
        ("use crate::{boolean::X, splitting::Y};", 1),
        ("use crate::{a::b, boolean::{X, Y}};", 1),
        ("use crate::{boolean, splitting};", 1),
        ("use crate::boolean as b;", 1),
        ("use super::super::boolean::X;", 1),
        ("fn f() { crate::boolean::zip::run(); boolean::g(); }", 2),
        ("fn f() { $crate::boolean::g() }", 1),
        ("fn f() { g(a, crate :: boolean :: h) }", 1),
        (
            "fn f() { let boolean = 1; x.boolean::<u8>(); Op::boolean(); }",
            0,
        ),
        ("use geom::{boolean::X};", 0),
        ("use ::boolean::X;", 0),
        (
            "// crate::boolean::X\n/// [`crate::boolean::X`]\nconst S: &str = \"crate::boolean\";",
            0,
        ),
        ("fn f() { boolean_door(); crate::booleans::x(); }", 0),
    ] {
        assert_eq!(sites(text), n, "engine sites in {text:?}");
    }
    let names: BTreeSet<String> = ["BooleanOp", "union"].map(String::from).into();
    let via_root = |text: &str| root_sites(CodeOnly::of(text).as_str(), &names);
    for (text, n) in [
        ("use crate::BooleanOp;", 1),
        ("use crate::{BooleanOp as Op, contact::union, union};", 2),
        (
            "fn f() { crate::union(a, b); $crate::BooleanOp::Union; }",
            2,
        ),
        (
            "fn f() { crate::contact::BooleanOp; crate::BooleanOperand; pub(crate) fn g() {} }",
            0,
        ),
    ] {
        assert_eq!(via_root(text), n, "root-spelled engine names in {text:?}");
    }
    let lib = "pub use boolean::{BooleanOp, ops::{union as unite, Plain}}; \
               pub use crate::splitting::Split; use boolean::Hidden; \
               pub mod t { pub use crate::boolean::Inner; }";
    let exports = root_exports(CodeOnly::of(lib).as_str());
    assert_eq!(
        (
            exports["boolean"]
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            exports["splitting"].len()
        ),
        (vec!["BooleanOp", "Plain", "unite"], 1),
        "the root's engine names read off lib.rs's top-level `pub use`"
    );
    let wide = |rel: &str, text: &str| crate_wide(rel, CodeOnly::of(text).as_str());
    for (rel, text, n) in [
        (
            "boolean/zip.rs",
            "pub(crate) fn a() {} pub(in crate) struct B;",
            2,
        ),
        (
            "boolean/zip.rs",
            "pub(super) fn a() {} pub(in crate::boolean) fn b() {}",
            0,
        ),
        ("boolean/zip.rs", "pub(in crate::splitting) fn a() {}", 1),
        (
            "boolean/zip.rs",
            "pub(in super::super) fn a() {} pub(self) fn b() {}",
            1,
        ),
        (
            "boolean/mod.rs",
            "pub(super) fn a() {} mod t { pub(super) fn b() {} }",
            1,
        ),
        (
            "boolean/zip.rs",
            "pub fn a() {} struct S { pub(crate) f: u8 }",
            1,
        ),
    ] {
        assert_eq!(
            wide(rel, text),
            n,
            "crate-visible qualifiers in {rel}: {text:?}"
        );
    }
}
