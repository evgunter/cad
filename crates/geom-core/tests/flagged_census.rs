//! **Every shipped call of a door that cites a ledger row names a row
//! the ledger has**, and the clause-(i) debt lane's `decide_flagged`
//! sites match the ledger's inventory (the standing rule on
//! [`geom_core::k_stats::decide_flagged`]; issue #214). Shipped means a
//! crate's `src/` tree: fixtures and demos carry prose reasons instead of
//! rows and are not counted, and an in-file `#[cfg(test)]` module is
//! counted (the safe direction; move such a fixture to `tests/`).
//!
//! **The doors are read off their home.** Every `pub fn` of
//! `geom_core::k_stats` that takes `ledger_row: &'static str` is a door,
//! its row at that parameter's index ([`home_doors`]): `unlogged`,
//! `check_unlogged`, `choose`, `decide_flagged` and its reporting form
//! today, and a door added there tomorrow with no list here to update.
//! A door of the home is that module's fn, not its verb: a call counts
//! when it goes through `k_stats` (or an `as` name of it), or bare under
//! a `use` that imports it, in a group, by glob or renamed
//! ([`spellings`]). A fn that forwards a `&'static str` parameter to a
//! door's row is a door in its own right, keyed on its name, and its
//! callers are checked ([`forwarding_door`]), to a fixed point.
//!
//! **What the census cannot read, it refuses rather than skips**: a row
//! that is not a string literal and not a forwarded parameter; a
//! forwarded parameter the body may rebind; a door named without a call
//! (a fn pointer, a binding, a macro argument); a `pub use` re-export of
//! a door; a turbofish it cannot balance. Each is a site citing no row,
//! or a panic, never a site dropped. A call a macro generates is not
//! read at all.
//!
//! The `decide_flagged` total is hand-synced and spelled ONCE, at
//! [`LEDGER_FLAGGED_SITES`]; the rows are computed from the document.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// The ledger's current shipped `decide_flagged` inventory.
///
/// **This number is hand-synced and nothing derives it.** It is kept
/// level by hand with the audit's own prose inventory; the sibling
/// assertion, [`every_shipped_site_cites_a_ledger_row_that_exists`],
/// reads its rows out of the audit and does compute.
const LEDGER_FLAGGED_SITES: usize = 5;

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
            "a door's `::<…>` turbofish at byte {at} has unbalanced angle brackets, so \
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
        "a door's turbofish at byte {at} did not resolve to a call: after its type \
         arguments the scan is at {:?}, not `(`. Angle depth is counted over `<` and `>` alone, \
         so an arrow or comparison inside the type arguments breaks it. The census stops rather \
         than skipping a shipped site.",
        &rest[k..rest.len().min(k + 24)]
    );
    k
}

/// What a door call cites as its ledger row.
enum Cite {
    /// A string literal: the row itself.
    Row(String),
    /// An identifier passed straight through, at byte `at` of the call: a
    /// wrapper's forwarding, which [`forwarding_door`] resolves.
    Forwarded { ident: String, at: usize },
}

/// The ledger row a site cites, which must be a string literal.
fn ledger_row(raw: &str) -> String {
    let cited = raw.trim();
    cited
        .strip_prefix('"')
        .and_then(|s| s.strip_suffix('"'))
        .unwrap_or_else(|| {
            panic!(
                "a door's site cites its ledger row as `{cited}`, not a string literal. \
                 The census can only verify literals; give the site its row directly, or teach \
                 this test to resolve the indirection — do not leave it unchecked."
            )
        })
        .to_string()
}

/// The home of the doors, under the crates directory, and the module a
/// call reaches them through.
const HOME: &str = "geom-core/src/k_stats.rs";
const HOME_MODULE: &str = "k_stats";

/// A door that cites a ledger row: its name, the row's argument index
/// (from 0), and, for a door of the home, the file that defines it. A
/// door of the home is reached through [`HOME_MODULE`] ([`spellings`]);
/// a forwarding wrapper ([`forwarding_door`]) is keyed on its name
/// alone, which reads every same-named call as one of its sites: the
/// loud direction.
#[derive(Clone, Debug, PartialEq)]
struct Door {
    name: String,
    cite: usize,
    home: Option<PathBuf>,
}

/// A `fn` head in a blanked view: its offset, name, parameters (name and
/// type with whitespace removed, read from the unblanked text), whether
/// it is `pub`, and its body, if it has one.
struct FnHead {
    at: usize,
    name: String,
    params: Vec<(String, String)>,
    public: bool,
    body: Option<std::ops::Range<usize>>,
}

/// Every `fn` head in `code` (the blanked view of `text`).
fn fn_heads(text: &str, code: &str) -> Vec<FnHead> {
    use test_utils::source::{ItemBody, balanced_end, boundary_before, ident, item_body};
    let mut out = Vec::new();
    for (head, _) in code.match_indices("fn ") {
        if !boundary_before(code, head) {
            continue;
        }
        let name_at = head + 3 + code[head + 3..].len() - code[head + 3..].trim_start().len();
        let name = ident(code, name_at).to_string();
        let Some(open) = code[name_at..].find('(').map(|o| name_at + o) else {
            continue;
        };
        let Some(close) = balanced_end(code, open) else {
            continue;
        };
        let params = test_utils::source::top_level_split(&code[open + 1..close], ',')
            .into_iter()
            .filter_map(|r| {
                let (n, ty) = text[open + 1..close][r].split_once(':')?;
                Some((
                    n.trim().to_string(),
                    ty.split_whitespace().collect::<String>(),
                ))
            })
            .collect();
        let before = code[..head].trim_end();
        let public = before.ends_with("pub")
            || (before.ends_with(')')
                && before
                    .rfind("pub(")
                    .is_some_and(|p| !before[p..].contains('\n')));
        let body = match item_body(code, head) {
            ItemBody::Body(b) => Some(b),
            _ => None,
        };
        out.push(FnHead {
            at: head,
            name,
            params,
            public,
            body,
        });
    }
    out
}

/// **The roster is the home's signatures**: every `pub fn` of `home`
/// that takes `ledger_row: &'static str` is a door, its row at that
/// parameter's index. A door added to the home is read here the day it
/// is written, and no list beside it names it.
fn home_doors(home: &Path, text: &str) -> Vec<Door> {
    let code = test_utils::source::code_only(text);
    let doors: Vec<Door> = fn_heads(text, &code)
        .into_iter()
        .filter(|f| f.public)
        .filter_map(|f| {
            let cite = f
                .params
                .iter()
                .position(|(n, ty)| n == "ledger_row" && ty == "&'staticstr")?;
            Some(Door {
                name: f.name,
                cite,
                home: Some(home.to_path_buf()),
            })
        })
        .collect();
    assert!(
        !doors.is_empty(),
        "{} defines no `pub fn` taking `ledger_row: &'static str`: the roster this census reads \
         came back empty",
        home.display()
    );
    doors
}

/// One leaf of a `use` tree: the path it imports, by segment, its `as`
/// name, and whether the item re-exports it.
#[derive(Debug)]
struct UseLeaf {
    path: Vec<String>,
    alias: Option<String>,
    public: bool,
}

/// The byte range of each `use` item in `code`, and its leaves: groups,
/// nested groups, `self`, globs and `as` are expanded, so a leaf is
/// matched by its path's segments and `k_stats::choose_x` is not
/// `k_stats::choose`.
fn use_items(code: &str) -> Vec<(std::ops::Range<usize>, Vec<UseLeaf>)> {
    use test_utils::source::{boundary_after, boundary_before};
    let mut out = Vec::new();
    for (at, _) in code.match_indices("use") {
        if !boundary_before(code, at)
            || !boundary_after(code, at + 3)
            || !code[at + 3..].starts_with(char::is_whitespace)
        {
            continue;
        }
        let Some(end) = code[at..].find(';').map(|e| at + e) else {
            continue;
        };
        let before = code[..at].trim_end();
        let public = before.ends_with("pub")
            || (before.ends_with(')')
                && before
                    .rfind("pub(")
                    .is_some_and(|p| !before[p..].contains('\n')));
        let mut tokens = Vec::new();
        let tree = &code[at + 3..end];
        let mut chars = tree.char_indices().peekable();
        while let Some((i, c)) = chars.next() {
            // `$` opens a macro fragment (`$crate`, `$tag`), one segment.
            if c.is_alphanumeric() || c == '_' || c == '$' {
                let mut j = i + c.len_utf8();
                while let Some(&(k, d)) = chars.peek() {
                    if d.is_alphanumeric() || d == '_' {
                        j = k + d.len_utf8();
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(tree[i..j].to_string());
            } else if c == ':' {
                if chars.next_if(|&(_, d)| d == ':').is_none() {
                    panic!("a `use` item at byte {at} holds a lone `:`: {tree:?}");
                }
                tokens.push("::".to_string());
            } else if !c.is_whitespace() {
                tokens.push(c.to_string());
            }
        }
        let mut leaves = Vec::new();
        let mut i = 0;
        use_tree(&tokens, &mut i, Vec::new(), public, &mut leaves);
        assert_eq!(
            i,
            tokens.len(),
            "a `use` item at byte {at} read only in part: {tree:?}"
        );
        out.push((at..end, leaves));
    }
    out
}

/// One `use` tree from `t[*i..]`, under `prefix`, into `out`.
fn use_tree(
    t: &[String],
    i: &mut usize,
    mut prefix: Vec<String>,
    public: bool,
    out: &mut Vec<UseLeaf>,
) {
    loop {
        let tok = t.get(*i).map(String::as_str);
        match tok {
            Some("::") => *i += 1,
            Some("*") => {
                *i += 1;
                prefix.push("*".to_string());
                out.push(UseLeaf {
                    path: prefix,
                    alias: None,
                    public,
                });
                return;
            }
            Some("{") => {
                *i += 1;
                loop {
                    if t.get(*i).map(String::as_str) == Some("}") {
                        *i += 1;
                        return;
                    }
                    use_tree(t, i, prefix.clone(), public, out);
                    match t.get(*i).map(String::as_str) {
                        Some(",") => *i += 1,
                        Some("}") => {}
                        other => panic!("a `use` group continues with {other:?}: {t:?}"),
                    }
                }
            }
            Some(seg)
                if seg
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '_' || c == '$') =>
            {
                *i += 1;
                if seg != "self" {
                    prefix.push(seg.to_string());
                }
                if t.get(*i).map(String::as_str) == Some("::") {
                    continue;
                }
                let alias = (t.get(*i).map(String::as_str) == Some("as")).then(|| {
                    *i += 2;
                    t.get(*i - 1)
                        .cloned()
                        .unwrap_or_else(|| panic!("`as` with no name: {t:?}"))
                });
                out.push(UseLeaf {
                    path: prefix,
                    alias,
                    public,
                });
                return;
            }
            other => panic!("a `use` tree holds {other:?}, which this census cannot read: {t:?}"),
        }
    }
}

/// **How a file spells a door of the home**: the names a bare call of it
/// goes by (the door's own name in the home, under an import, under a
/// glob of the module, and an `as` name), and the qualifiers a path call
/// goes through ([`HOME_MODULE`] and any `as` name of it). A re-export of
/// the door, or of the module under another name, is a route this census
/// cannot follow into its importers, so it stops rather than undercount.
fn spellings(file: &Scanned, door: &Door, in_home: bool) -> (Vec<String>, Vec<String>) {
    let mut bare = Vec::new();
    let mut quals = vec![HOME_MODULE.to_string()];
    if in_home {
        bare.push(door.name.clone());
    }
    for (_, leaves) in &file.uses {
        for leaf in leaves {
            let p = &leaf.path;
            let ends = |tail: &[&str]| p.len() >= tail.len() && p[p.len() - tail.len()..] == *tail;
            let is_door = ends(&[HOME_MODULE, &door.name]);
            let glob = ends(&[HOME_MODULE, "*"]);
            let module = ends(&[HOME_MODULE]);
            assert!(
                !(leaf.public && (is_door || glob || (module && leaf.alias.is_some()))),
                "{} re-exports `{}` (`pub use {}`): a call through the re-export reaches the door \
                 by a path this census does not read. Call the door through `{HOME_MODULE}`, or \
                 teach this census the route.",
                file.path.display(),
                door.name,
                p.join("::")
            );
            match (&leaf.alias, is_door, glob, module) {
                (Some(a), true, _, _) if a != "_" => bare.push(a.clone()),
                (None, true, _, _) | (_, _, true, _) => bare.push(door.name.clone()),
                (Some(a), _, _, true) if a != "_" => quals.push(a.clone()),
                _ => {}
            }
        }
    }
    (bare, quals)
}

/// A source file read once for every door: its text, blanked view and
/// `use` items.
struct Scanned<'a> {
    path: &'a Path,
    text: &'a str,
    code: String,
    uses: Vec<(std::ops::Range<usize>, Vec<UseLeaf>)>,
}

impl<'a> Scanned<'a> {
    fn of(path: &'a Path, text: &'a str) -> Self {
        let code = test_utils::source::code_only(text);
        let uses = use_items(&code);
        Scanned {
            path,
            text,
            code,
            uses,
        }
    }
}

/// Every call of `door` in `text` (at `path`), with what it cites at the
/// row's index. An argument `forwards` accepts is a [`Cite::Forwarded`]
/// rather than refused for not being a literal.
///
/// **A door named and not called is a site citing no row**: a fn
/// pointer (`let f = choose;`), an argument to a macro that calls it,
/// anything that reaches the door other than by a call this census can
/// read. The definition and the `use` items are not mentions.
fn door_calls_in(
    file: &Scanned,
    door: &Door,
    forwards: impl Fn(&str) -> bool,
) -> Vec<(usize, Cite)> {
    use test_utils::source::{boundary_after, boundary_before};
    let (text, code) = (file.text, &file.code);
    let (bare, quals) = match &door.home {
        Some(home) => spellings(file, door, home == file.path),
        None => (vec![door.name.clone()], Vec::new()),
    };
    let mut names = bare.clone();
    if !quals.is_empty() && !names.contains(&door.name) {
        names.push(door.name.clone());
    }
    let mut out = Vec::new();
    for name in &names {
        for (at, _) in code.match_indices(name.as_str()) {
            if !boundary_before(code, at) || !boundary_after(code, at + name.len()) {
                continue;
            }
            if file.uses.iter().any(|(u, _)| u.contains(&at)) {
                continue;
            }
            let before = code[..at].trim_end();
            if before.ends_with("fn") && boundary_before(before, before.len() - 2) {
                continue;
            }
            if door.home.is_some() {
                let qualified = *name == door.name
                    && before.strip_suffix("::").is_some_and(|q| {
                        let q = q.trim_end();
                        quals.iter().any(|m| {
                            q.ends_with(m.as_str()) && boundary_before(q, q.len() - m.len())
                        })
                    });
                let called_bare =
                    bare.contains(name) && !before.ends_with('.') && !before.ends_with("::");
                if !(qualified || called_bare) {
                    continue;
                }
            }
            let line = test_utils::source::line(code, at);
            let i = at + name.len();
            let open = skip_turbofish(&code[i..], at);
            if !code[i + open..].starts_with('(') {
                out.push((
                    line,
                    Cite::Row(format!(
                        "<names `{name}` without calling it: a fn pointer, a binding or a macro \
                         argument this census cannot follow>"
                    )),
                ));
                continue;
            }
            let paren = i + open;
            let end = test_utils::source::balanced_end(code, paren)
                .unwrap_or_else(|| panic!("unterminated {name} argument list at byte {paren}"));
            let args = paren + 1;
            let split = test_utils::source::top_level_split(&code[args..end], ',');
            let cite = split.get(door.cite).unwrap_or_else(|| {
                panic!(
                    "{name} call with fewer than {} arguments: {:.200}",
                    door.cite + 1,
                    &text[args..end]
                )
            });
            let raw = &text[args + cite.start..args + cite.end];
            out.push((
                line,
                if forwards(raw) {
                    Cite::Forwarded {
                        ident: raw.trim().to_string(),
                        at,
                    }
                } else {
                    Cite::Row(ledger_row(raw))
                },
            ));
        }
    }
    out
}

/// The sites of `doors` in `files`, wrappers resolved to a fixed point
/// ([`forwarding_door`]).
fn door_sites(files: &[(PathBuf, String)], doors: &[Door]) -> Vec<Site> {
    let identifier = |raw: &str| {
        !raw.trim().is_empty() && raw.trim().chars().all(|c| c.is_alphanumeric() || c == '_')
    };
    let scanned: Vec<Scanned> = files.iter().map(|(p, t)| Scanned::of(p, t)).collect();
    let mut known: Vec<Door> = doors.to_vec();
    let mut hits: Vec<Site> = Vec::new();
    let mut next = 0;
    while next < known.len() {
        let door = known[next].clone();
        next += 1;
        for file in &scanned {
            let path = &file.path.to_path_buf();
            for (line, found) in door_calls_in(file, &door, identifier) {
                match found {
                    Cite::Row(ledger_row) => hits.push(Site {
                        file: path.clone(),
                        line,
                        ledger_row,
                    }),
                    Cite::Forwarded { ident, at } => {
                        match forwarding_door(file.text, &file.code, at, &ident) {
                            Ok((name, cite)) => {
                                // A door of the home forwarding to another is
                                // already on the roster; any other wrapper is a
                                // door keyed on its name.
                                let own = known.iter().any(|d| {
                                    d.home.as_ref() == Some(path)
                                        && d.name == name
                                        && d.cite == cite
                                });
                                let wrapper = Door {
                                    name,
                                    cite,
                                    home: None,
                                };
                                if !own && !known.contains(&wrapper) {
                                    known.push(wrapper);
                                }
                            }
                            Err(why) => hits.push(Site {
                                file: path.clone(),
                                line,
                                ledger_row: format!("<forwards `{ident}`, {why}>"),
                            }),
                        }
                    }
                }
            }
        }
    }
    hits.sort_by(|a, b| (&a.file, a.line, &a.ledger_row).cmp(&(&b.file, b.line, &b.ledger_row)));
    // Only an identical citation is a duplicate: two door calls on one line
    // citing different rows are two sites, and each row is checked.
    hits.dedup_by(|a, b| a.file == b.file && a.line == b.line && a.ledger_row == b.ledger_row);
    hits
}

/// **The door a forwarding call makes of the fn around it**: the
/// innermost `fn` whose body holds byte `at` of `code`, where it takes
/// `ident` as a `&'static str` parameter, as its name and that
/// parameter's index. `text` is the unblanked source at the same offsets,
/// read for the parameter's type.
///
/// **The parameter must reach the call unbound again.** Any other
/// mention of `ident` in the body before the call (a `let`, a closure's
/// `|ident|`, a pattern) may rebind it, and then the caller's literal is
/// not the row the door receives: refused, as a site citing no row,
/// rather than read as a forwarding.
fn forwarding_door(
    text: &str,
    code: &str,
    at: usize,
    ident: &str,
) -> Result<(String, usize), String> {
    use test_utils::source::{boundary_after, boundary_before};
    let head = fn_heads(text, code)
        .into_iter()
        .rev()
        .find(|f| f.at < at && f.body.as_ref().is_some_and(|b| b.contains(&at)))
        .ok_or_else(|| "outside any fn".to_string())?;
    let index = head
        .params
        .iter()
        .position(|(n, ty)| n == ident && ty == "&'staticstr")
        .ok_or_else(|| "which is no `&'static str` parameter of the fn around it".to_string())?;
    let body = head.body.expect("filtered on a body");
    let rebound = code[body.start..at].match_indices(ident).any(|(k, _)| {
        let k = body.start + k;
        boundary_before(code, k) && boundary_after(code, k + ident.len())
    });
    if rebound {
        return Err(format!(
            "which the body of `{}` names before the call, so it may be rebound",
            head.name
        ));
    }
    Ok((head.name, index))
}

/// Every `.rs` file under each crate's `src/`, with its text.
fn shipped_files() -> Vec<(PathBuf, String)> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(crates_root()).expect("crates dir listing") {
        let src = entry.expect("dir entry").path().join("src");
        if src.is_dir() {
            for path in test_utils::source::rust_sources(&src) {
                let text = std::fs::read_to_string(&path).expect("readable source file");
                files.push((path, text));
            }
        }
    }
    files
}

/// geom-core sits at <workspace>/crates/geom-core.
fn crates_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates dir")
        .to_path_buf()
}

/// The home's row-citing doors, read off its signatures.
fn shipped_doors() -> Vec<Door> {
    let home = crates_root().join(HOME);
    let text = std::fs::read_to_string(&home).expect("readable k_stats home");
    home_doors(&home, &text)
}

/// The clause-(i) lane's sites: the calls of the home's `decide_flagged`
/// doors, the plain one and the one that keeps its margin, and of any
/// wrapper that forwards a row to them.
fn flagged_sites() -> Vec<Site> {
    let flagged: Vec<Door> = shipped_doors()
        .into_iter()
        .filter(|d| d.name.starts_with("decide_flagged"))
        .collect();
    assert!(
        !flagged.is_empty(),
        "the home defines no decide_flagged door"
    );
    door_sites(&shipped_files(), &flagged)
}

#[test]
fn shipped_decide_flagged_sites_match_the_ledger() {
    let hits = flagged_sites();
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
    let hits = door_sites(&shipped_files(), &shipped_doors());
    let audit = crates_root()
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

/// The `(file, row)` of each site, in [`door_sites`]' order.
fn rows_of(sites: &[Site]) -> Vec<(String, String)> {
    sites
        .iter()
        .map(|s| (s.file.display().to_string(), s.ledger_row.clone()))
        .collect()
}

fn planted(name: &str, text: &str) -> (PathBuf, String) {
    (PathBuf::from(name), text.to_string())
}

/// **The roster is the home's signatures**: a `pub fn` taking
/// `ledger_row: &'static str` is a door at that index, whatever its name;
/// a private fn, and a `pub fn` with no such parameter, are not.
#[test]
fn the_doors_are_the_homes_fns_that_take_a_ledger_row() {
    let home = PathBuf::from("k_stats.rs");
    let doors = home_doors(
        &home,
        "pub fn fresh(ledger_row: &'static str) {}\n\
         fn private(ledger_row: &'static str) {}\n\
         pub fn decide<T: Decide>(name: &'static str, m: T, band: Band) -> S { m }\n\
         pub(crate) fn wrapped<T>(\n    name: &'static str,\n    ledger_row: &'static str,\n) {}\n",
    );
    let read: Vec<(&str, usize)> = doors.iter().map(|d| (d.name.as_str(), d.cite)).collect();
    assert_eq!(
        read,
        [("fresh", 0), ("wrapped", 1)],
        "the home's row-citing doors"
    );
}

/// **A forwarding wrapper is a door, and its callers are checked**: a fn
/// that takes the row as `&'static str` and passes it on is read as a door
/// with its row at that parameter's index, a wrapper of a wrapper too. Its
/// callers' literals are the sites. A forwarding of something that is no
/// such parameter (a caller's own non-`&'static str` parameter, a `const`,
/// a closure's argument) is a site citing no row; so is a parameter the
/// body may rebind before the call, and a wrapper named without a call.
#[test]
fn a_forwarding_wrapper_is_a_door_and_its_callers_cite_their_rows() {
    let home = "k_stats.rs";
    let files = [
        planted(
            home,
            "pub fn unlogged<R>(ledger_row: &'static str, w: W) -> R {\n    w()\n}\n\
             pub fn check_unlogged<T: Decide>(name: &'static str, m: T, band: B, ledger_row: &'static str) -> S {\n    unlogged(ledger_row, || m)\n}\n",
        ),
        planted(
            "wrap.rs",
            "use geom_core::k_stats::check_unlogged;\n\
             pub fn gated(g: G, m: f64, row: &'static str) -> S {\n    check_unlogged(g.name(), m, B, row)\n}\n\
             pub fn twice(m: f64, ledger: &'static str) -> S {\n    gated(G, m, ledger)\n}\n\
             pub fn shadowed(m: f64, row: &'static str) -> S {\n    let row = pick();\n    check_unlogged(\"n\", m, B, row)\n}\n\
             pub fn by_closure(m: f64, row: &'static str) -> S {\n    [1].map(|row| check_unlogged(\"n\", m, B, row))\n}\n\
             pub fn closure_only(m: f64) -> S {\n    let f = |row| check_unlogged(\"n\", m, B, row);\n    f(\"F9\")\n}\n",
        ),
        planted(
            "use.rs",
            "use geom_core::k_stats::check_unlogged;\n\
             const ROW: &str = \"F1\";\n\
             fn a() { gated(G, 1.0, \"F1\"); }\n\
             fn b() { twice(2.0, \"F2\"); }\n\
             fn c(row: R) { check_unlogged(\"n\", 3.0, B, row); }\n\
             fn d() { shadowed(1.0, \"F3\"); by_closure(1.0, \"F4\"); }\n\
             fn e() { let f = gated; f(G, 1.0, \"BAD\"); }\n\
             fn g() { gated(G, 1.0, ROW); }\n",
        ),
    ];
    let roster = home_doors(Path::new(home), &files[0].1);
    let no_param = "which is no `&'static str` parameter of the fn around it>";
    let rebound = |f: &str| {
        format!(
            "<forwards `row`, which the body of `{f}` names before the call, so it may be rebound>"
        )
    };
    let want: Vec<(String, String)> = [
        ("use.rs", "F1".to_string()),
        ("use.rs", "F2".to_string()),
        ("use.rs", format!("<forwards `row`, {no_param}")),
        (
            "use.rs",
            "<names `gated` without calling it: a fn pointer, a binding or a macro argument this \
             census cannot follow>"
                .to_string(),
        ),
        ("use.rs", format!("<forwards `ROW`, {no_param}")),
        ("wrap.rs", rebound("shadowed")),
        ("wrap.rs", rebound("by_closure")),
        ("wrap.rs", format!("<forwards `row`, {no_param}")),
    ]
    .into_iter()
    .map(|(f, r)| (f.to_string(), r))
    .collect();
    assert_eq!(
        rows_of(&door_sites(&files, &roster)),
        want,
        "the exact site set"
    );
}

/// **A door of the home is the home module's fn, not the verb**: a call
/// through `k_stats`, or through an `as` name of it, and a bare call of a
/// name the file imports from it (in a group, nested, by glob or renamed)
/// are sites; a method, another module's fn of the same name, a longer
/// name and a bare call nothing imported are not.
#[test]
fn a_door_of_the_home_is_its_modules_fn_and_not_the_verb() {
    let home = "k_stats.rs";
    let files = [
        planted(
            home,
            "pub fn choose<T: Decide>(name: &'static str, m: M<T>, ledger_row: &'static str) -> Pick {\n    Pick::Tie\n}\n",
        ),
        planted(
            "qualified.rs",
            "fn a() { geom_core::k_stats::choose(\"n\", m, \"F1\"); }\n",
        ),
        planted(
            "imported.rs",
            "use geom_core::k_stats::{Pick, choose};\nfn b() { choose(\"n\", m, \"F2\"); }\n",
        ),
        planted(
            "glob.rs",
            "use geom_core::k_stats::*;\nfn g() { choose(\"n\", m, \"F3\"); }\n",
        ),
        planted(
            "aliased.rs",
            "use geom_core::k_stats as ks;\nfn h() { ks::choose(\"n\", m, \"F4\"); }\n",
        ),
        planted(
            "renamed.rs",
            "use geom_core::{Band, k_stats::{self, choose as pick}};\nfn i() { pick(\"n\", m, \"F5\"); choose(1, 2); }\n",
        ),
        planted(
            "elsewhere.rs",
            "use rand::seq::SliceRandom;\nfn c() { v.choose(&mut rng); }\nfn d() { other::choose(1, 2); }\n\
             fn e() { choose(n, k); }\nfn f() { k_stats::choose_x(1); my_k_stats::choose(1, 2); }\n",
        ),
    ];
    let roster = home_doors(Path::new(home), &files[0].1);
    let want: Vec<(String, String)> = [
        ("aliased.rs", "F4"),
        ("glob.rs", "F3"),
        ("imported.rs", "F2"),
        ("qualified.rs", "F1"),
        ("renamed.rs", "F5"),
    ]
    .into_iter()
    .map(|(f, r)| (f.to_string(), r.to_string()))
    .collect();
    assert_eq!(
        rows_of(&door_sites(&files, &roster)),
        want,
        "only the module's fn is the door"
    );
}

/// **A re-export of a door stops the census**: its importers call it by a
/// path the census does not read.
#[test]
#[should_panic(expected = "re-exports `choose`")]
fn a_re_exported_door_stops_the_census() {
    let home = "k_stats.rs";
    let files = [
        planted(
            home,
            "pub fn choose<T: Decide>(name: &'static str, m: M<T>, ledger_row: &'static str) -> Pick {\n    Pick::Tie\n}\n",
        ),
        planted("lib.rs", "pub use geom_core::k_stats::choose;\n"),
    ];
    let roster = home_doors(Path::new(home), &files[0].1);
    let _ = door_sites(&files, &roster);
}
