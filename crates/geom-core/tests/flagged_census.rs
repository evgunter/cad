//! **Every shipped call of a door that cites a ledger row names a row
//! the ledger has**, and the clause-(i) debt lane's `decide_flagged`
//! sites match the ledger's inventory (the standing rule on
//! [`geom_core::k_stats::decide_flagged`]; issue #214). Shipped means a
//! crate's `src/` tree: fixtures and demos carry prose reasons instead of
//! rows and are not counted, and an in-file `#[cfg(test)]` module is
//! counted (the safe direction; move such a fixture to `tests/`).
//!
//! **The doors are read off their home.** Every `pub` fn of
//! `geom_core::k_stats` (methods and `pub(…)` included, `#[cfg(test)]`
//! out, as the least-band gate reads it) that takes a row-like parameter,
//! a text type beside `name` (`&str`, `String`, an alias the home gives
//! one) or a name holding `row`, is a door, its row at
//! that parameter's index, unless [`NON_ROW_DOORS`] names it
//! ([`home_doors`]): `unlogged`, `check_unlogged`, `choose`,
//! `decide_flagged` and its reporting form today, and a door added there
//! tomorrow with no list here to update. A door of the home is that
//! module's fn, not its verb: a call counts when it goes through
//! `k_stats` (or an `as` name of it, or `self` in the home), or bare
//! under a `use` that imports it, in a group, by glob or renamed
//! ([`spellings`]); a method door's `.m(` and `Type::m(` count too. A fn
//! that forwards a `&'static str` parameter to a door's row is a door in
//! its own right, keyed on its name, and its callers are checked
//! ([`forwarding_door`]), to a fixed point.
//!
//! **What the census cannot read, it lists rather than skips**: a row
//! that is not a string literal and not a forwarded parameter; a
//! forwarded parameter the body may rebind; a door named without a call
//! (a fn pointer, a binding, a macro argument); a bare call of a door's
//! name that nothing in its file resolves (a `use super::*` child, no
//! import); a `pub use` re-export of a door or of the module under
//! another name. Each is a site citing no row. A home fn with two
//! row-like parameters, a stale [`NON_ROW_DOORS`] entry and a turbofish
//! it cannot balance stop it.
//!
//! **Blind spots, stated:** a call a macro generates; a door whose row
//! reaches it inside a struct argument, or as a type named for text
//! outside the home (the shape `k_stats`' header rules out); a bare call
//! of a door's name inside the scope of another binding of that name (an
//! import from elsewhere or a free `fn` in its block, a `let` from it to
//! its block's end, a closure parameter in that closure's body, a
//! parameter in its fn's body), which is read as that other meaning. A
//! struct field and a method bind nothing; a closure is read by its own
//! `|…|` head where an expression starts, so a match or-pattern or a
//! bitwise or is no closure. Each of these is a lexical reading of a
//! scope, and a shape it does not know is read as no binding (the loud
//! direction) unless it looks like one it knows.
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
///
/// A door of the home defined in an `impl` is a method: a `.m(` or
/// `Type::m(` call of it is a site too, its row one argument later in
/// the second form when it takes `self`.
#[derive(Clone, Debug, PartialEq)]
struct Door {
    name: String,
    cite: usize,
    home: Option<PathBuf>,
    method: bool,
    receiver: bool,
}

/// A `fn` head in a blanked view: its offset, name, parameters (name, with
/// any `mut` dropped, and type with its whitespace runs made one space, read from the
/// unblanked text), whether it is `pub` (`pub(…)` included), whether it
/// sits in an `impl` and whether in a `#[cfg(test)]` item, and its body,
/// if it has one.
struct FnHead {
    at: usize,
    name: String,
    params: Vec<(String, String)>,
    public: bool,
    in_impl: bool,
    in_test: bool,
    receiver: bool,
    body: Option<std::ops::Range<usize>>,
}

/// Every `fn` head in `code` (the blanked view of `text`).
fn fn_heads(text: &str, code: &str) -> Vec<FnHead> {
    use test_utils::source::{
        ItemBody, attributed_extent, balanced_end, boundary_after, boundary_before, ident,
        item_body,
    };
    // `impl` blocks: an `impl` that opens an item (after a `}`, `;`, `{`,
    // an attribute or the start), not `impl Trait` in a type.
    let impls: Vec<std::ops::Range<usize>> = code
        .match_indices("impl")
        .filter(|&(at, _)| {
            boundary_before(code, at)
                && boundary_after(code, at + 4)
                && code[..at]
                    .trim_end()
                    .chars()
                    .next_back()
                    .is_none_or(|c| matches!(c, '}' | ';' | '{' | ']'))
        })
        .filter_map(|(at, _)| match item_body(code, at) {
            ItemBody::Body(b) => Some(b),
            _ => None,
        })
        .collect();
    let tests: Vec<std::ops::Range<usize>> = code
        .match_indices("#[cfg(test)]")
        .filter_map(|(at, _)| attributed_extent(code, at))
        .collect();
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
        let receiver = code[open + 1..close].split(',').next().is_some_and(|p| {
            let p = p.trim().trim_start_matches('&').trim_start();
            let p = p.strip_prefix("mut ").unwrap_or(p).trim_start();
            p == "self" || p.starts_with("self:") || p.starts_with("self ")
        });
        let params = test_utils::source::top_level_split(&code[open + 1..close], ',')
            .into_iter()
            .filter_map(|r| {
                let (n, ty) = text[open + 1..close][r].split_once(':')?;
                let n = n.trim();
                Some((
                    n.strip_prefix("mut ").unwrap_or(n).trim().to_string(),
                    ty.split_whitespace().collect::<Vec<_>>().join(" "),
                ))
            })
            .collect();
        let mut before = code[..head].trim_end();
        while let Some(b) = ["const", "unsafe", "async"]
            .iter()
            .find_map(|k| before.strip_suffix(k))
        {
            before = b.trim_end();
        }
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
            in_impl: impls.iter().any(|b| b.contains(&head)),
            in_test: tests.iter().any(|t| t.contains(&head)),
            receiver,
            body,
        });
    }
    out
}

/// The home's `pub` fns that take a row-like parameter and cite no
/// ledger row: each must be one ([`home_doors`] refuses a stale entry).
const NON_ROW_DOORS: [&str; 2] = ["in_family", "identity_pass_set"];

/// Whether a parameter reads as a ledger row: a name holding `row`, or,
/// under any name but the predicate's `name`, a type that names `str` or
/// `String` (`&'static str`, `&str`, `String`, `Cow<str>`) or one of
/// `texts`, the home's aliases of those. An array or a slice of text
/// (`[&'static str; 2]`, `&[&str]`) is predicate names, as `name` is.
fn row_like((name, ty): &(String, String), texts: &[String]) -> bool {
    let names = ty.starts_with('[') || ty.trim_start_matches('&').trim_start().starts_with('[');
    name.contains("row")
        || (name != "name"
            && !names
            && ty
                .split(|c: char| !(c.is_alphanumeric() || c == '_'))
                .any(|w| w == "str" || w == "String" || texts.iter().any(|t| t == w)))
}

/// The home's `type` aliases of a text type (`str`, `String`, or another
/// such alias), to a fixed point.
fn text_aliases(code: &str) -> Vec<String> {
    let mut texts: Vec<String> = Vec::new();
    loop {
        let before = texts.len();
        for (at, _) in code.match_indices("type ") {
            if !test_utils::source::boundary_before(code, at) {
                continue;
            }
            let Some(end) = code[at..].find(';').map(|e| at + e) else {
                continue;
            };
            let Some((head, rhs)) = code[at + 5..end].split_once('=') else {
                continue;
            };
            let alias = test_utils::source::ident(head.trim_start(), 0).to_string();
            let texty = rhs
                .split(|c: char| !(c.is_alphanumeric() || c == '_'))
                .any(|w| w == "str" || w == "String" || texts.iter().any(|t| t == w));
            if texty && !alias.is_empty() && !texts.contains(&alias) {
                texts.push(alias);
            }
        }
        if texts.len() == before {
            return texts;
        }
    }
}

/// **The roster is the home's signatures**: every `pub` fn of `home`
/// (outside `#[cfg(test)]`, methods included) that takes a row-like
/// parameter ([`row_like`]) is a door, its row at that parameter's
/// index, unless [`NON_ROW_DOORS`] names it. A door added to the home is
/// read here the day it is written, whatever it calls its row, and no
/// list beside it names it. A fn with two row-like parameters is one this
/// census cannot classify, and stops it.
fn home_doors(home: &Path, text: &str) -> Vec<Door> {
    home_doors_but(home, text, &NON_ROW_DOORS)
}

/// [`home_doors`] with the non-doors `but`.
fn home_doors_but(home: &Path, text: &str, but: &[&str]) -> Vec<Door> {
    let code = test_utils::source::code_only(text);
    let texts = text_aliases(&code);
    let mut doors = Vec::new();
    let mut takers = Vec::new();
    for f in fn_heads(text, &code) {
        if !f.public || f.in_test {
            continue;
        }
        let rows: Vec<usize> = (0..f.params.len())
            .filter(|&i| row_like(&f.params[i], &texts))
            .collect();
        let cite = match rows.as_slice() {
            [] => continue,
            [one] => *one,
            _ => panic!(
                "{}'s `{}` takes two row-like parameters ({:?}): this census cannot tell which \
                 is its ledger row. Name one `ledger_row`, or teach `home_doors` the shape.",
                home.display(),
                f.name,
                f.params
            ),
        };
        takers.push(f.name.clone());
        if !but.contains(&f.name.as_str()) {
            doors.push(Door {
                name: f.name,
                cite,
                home: Some(home.to_path_buf()),
                method: f.in_impl,
                receiver: f.receiver,
            });
        }
    }
    for nd in but {
        assert!(
            takers.iter().any(|t| t == nd),
            "NON_ROW_DOORS names `{nd}`, which is no `pub` fn of {} with a row-like parameter: \
             drop the stale entry",
            home.display()
        );
    }
    assert!(
        !doors.is_empty(),
        "{} defines no `pub` fn taking a ledger row: the roster this census reads came back empty",
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

/// **How a file spells a door of the home.**
struct Spelling {
    /// The names a bare call of it goes by: its own in the home, under an
    /// import, under a glob of the module, and an `as` name.
    bare: Vec<String>,
    /// The qualifiers a path call goes through: [`HOME_MODULE`], any `as`
    /// name of it, and `self` in the home.
    quals: Vec<String>,
    /// The spans where the file gives the door's name another meaning,
    /// each the scope of one binding: an import of it from another path
    /// (its block, or the file), a `let` (from it to its block's end), a
    /// closure's `|…|` (that closure's body, [`closure_parameter`]), a
    /// free `fn` of that name (the block it is declared in; a method or an
    /// associated fn, in an `impl` or `trait`, is never reached bare); a
    /// parameter's (its fn's body) is read on demand
    /// ([`parameter_scopes`]). A bare call inside one is that, not the
    /// door. A struct field of that name binds nothing.
    elsewhere: Vec<std::ops::Range<usize>>,
    /// Each re-export of the door, or of the module under another name, as
    /// `(line, row)`: a route this census cannot follow into its
    /// importers, listed as a site citing no row.
    offences: Vec<(usize, String)>,
}

/// How `file` spells `door`, a door of the home ([`Spelling`]).
fn spellings(file: &Scanned, door: &Door, in_home: bool) -> Spelling {
    use test_utils::source::{boundary_after, boundary_before};
    let mut sp = Spelling {
        bare: Vec::new(),
        quals: vec![HOME_MODULE.to_string()],
        elsewhere: Vec::new(),
        offences: Vec::new(),
    };
    if in_home {
        sp.bare.push(door.name.clone());
        sp.quals.push("self".to_string());
    }
    for (range, leaves) in &file.uses {
        for leaf in leaves {
            let p = &leaf.path;
            let ends = |tail: &[&str]| p.len() >= tail.len() && p[p.len() - tail.len()..] == *tail;
            let is_door = ends(&[HOME_MODULE, &door.name]);
            let glob = ends(&[HOME_MODULE, "*"]);
            let module = ends(&[HOME_MODULE]);
            if leaf.public && (is_door || glob || (module && leaf.alias.is_some())) {
                let what = match (&leaf.alias, module) {
                    (Some(a), true) => format!("the module `{HOME_MODULE}` as `{a}`"),
                    _ => format!("`{}`", door.name),
                };
                sp.offences.push((
                    test_utils::source::line(&file.code, range.start),
                    format!(
                        "<re-exports {what} (`pub use {}`): its importers reach the door by a \
                         path this census does not read>",
                        p.join("::")
                    ),
                ));
            }
            match (&leaf.alias, is_door, glob, module) {
                (Some(a), true, _, _) if a != "_" => sp.bare.push(a.clone()),
                (None, true, _, _) | (_, _, true, _) => sp.bare.push(door.name.clone()),
                (Some(a), _, _, true) if a != "_" => sp.quals.push(a.clone()),
                _ => {
                    let named = leaf.alias.as_deref().or(p.last().map(String::as_str));
                    if named == Some(door.name.as_str()) {
                        sp.elsewhere.push(enclosing_block(&file.code, range.start));
                    }
                }
            }
        }
    }
    if in_home {
        return sp;
    }
    let code = &file.code;
    let n = &door.name;
    for (at, _) in code.match_indices(n.as_str()) {
        if !boundary_before(code, at) || !boundary_after(code, at + n.len()) {
            continue;
        }
        let before = code[..at].trim_end();
        let keyword =
            |k: &str| before.ends_with(k) && boundary_before(before, before.len() - k.len());
        let block = enclosing_block(code, at);
        if let Some(body) = closure_parameter(code, at, n.len()) {
            sp.elsewhere.push(body);
        } else if keyword("fn") {
            if !is_impl_or_trait_body(code, &block) {
                sp.elsewhere.push(block);
            }
        } else if keyword("let") || keyword("mut") {
            sp.elsewhere.push(at..block.end);
        }
    }
    sp
}

/// **The body of the closure whose parameter list holds `code[at..at +
/// len]`**, or `None` where it is no closure parameter. A closure is read
/// by its own head: a `|` opening where an expression starts (after `(`,
/// `[`, `{`, `,`, `=`, `;`, `=>`, `move` or `return`), parameters only
/// up to the closing `|`, then a body: a `{ … }` block, or an expression
/// up to the `,`, `;` or closer that ends it at its own depth. A match
/// or-pattern (`1 | 2 =>`) and a bitwise or (`a | b`) open no closure.
fn closure_parameter(code: &str, at: usize, len: usize) -> Option<std::ops::Range<usize>> {
    let param = |c: char| c.is_alphanumeric() || " \t\n_,:&()<>'".contains(c);
    let open = code[..at].rfind('|')?;
    if !code[open + 1..at].chars().all(param) {
        return None;
    }
    let lead = code[..open].trim_end();
    let starts = ["(", "[", "{", ",", "=", ";", "=>"]
        .iter()
        .any(|t| lead.ends_with(t))
        || ["move", "return"].iter().any(|k| {
            lead.ends_with(k) && test_utils::source::boundary_before(lead, lead.len() - k.len())
        });
    if !starts
        || lead.ends_with("==")
        || lead.ends_with("!=")
        || lead.ends_with("<=")
        || lead.ends_with(">=")
    {
        return None;
    }
    let close = at + len + code[at + len..].find('|')?;
    if !code[at + len..close].chars().all(param) {
        return None;
    }
    let from = close + 1;
    let start = test_utils::source::skip_ws(code, from);
    if code[start..].starts_with('{') {
        let end = test_utils::source::balanced_end(code, start)?;
        return Some(from..end + 1);
    }
    let mut depth = 0usize;
    for (i, c) in code[from..].char_indices() {
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' if depth == 0 => return Some(from..from + i),
            ')' | ']' | '}' => depth -= 1,
            ',' | ';' if depth == 0 => return Some(from..from + i),
            _ => {}
        }
    }
    Some(from..code.len())
}

/// Whether `block` is the body of an `impl` or a `trait`: a `fn` declared
/// there is a method or an associated fn, which a bare call never reaches.
fn is_impl_or_trait_body(code: &str, block: &std::ops::Range<usize>) -> bool {
    if block.start == 0 && block.end == code.len() {
        return false;
    }
    let head_end = block.start;
    let head_start = code[..head_end].rfind(['}', ';', '{']).map_or(0, |i| i + 1);
    let head = &code[head_start..head_end];
    head.split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .find(|w| matches!(*w, "impl" | "trait" | "fn" | "mod" | "struct" | "enum"))
        .is_some_and(|w| w == "impl" || w == "trait")
}

/// The bodies of `file`'s fns that take a parameter named `name`: where a
/// bare call of `name` is that parameter. Read only for a call nothing
/// else resolves.
fn parameter_scopes(file: &Scanned, name: &str) -> Vec<std::ops::Range<usize>> {
    fn_heads(file.text, &file.code)
        .into_iter()
        .filter(|f| f.params.iter().any(|(p, _)| p == name))
        .filter_map(|f| f.body)
        .collect()
}

/// The innermost `{ … }` of `code` that holds `at`, braces included, or
/// the whole file outside every block.
fn enclosing_block(code: &str, at: usize) -> std::ops::Range<usize> {
    let mut depth = 0usize;
    for (i, c) in code[..at].char_indices().rev() {
        match c {
            '}' => depth += 1,
            '{' if depth == 0 => {
                let end = test_utils::source::balanced_end(code, i).unwrap_or(code.len() - 1);
                return i..end + 1;
            }
            '{' => depth -= 1,
            _ => {}
        }
    }
    0..code.len()
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
    let sp = match &door.home {
        Some(home) => spellings(file, door, home == file.path),
        None => Spelling {
            bare: vec![door.name.clone()],
            quals: Vec::new(),
            elsewhere: Vec::new(),
            offences: Vec::new(),
        },
    };
    let mut out: Vec<(usize, Cite)> = sp
        .offences
        .iter()
        .map(|(line, row)| (*line, Cite::Row(row.clone())))
        .collect();
    let mut names = sp.bare.clone();
    if !names.contains(&door.name) {
        names.push(door.name.clone());
    }
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
            let calls = code[at + name.len()..].trim_start().starts_with('(')
                || code[at + name.len()..].trim_start().starts_with("::<");
            let mut cite_at = door.cite;
            if door.home.is_some() {
                let own = *name == door.name;
                let accepted = if before.ends_with('.') {
                    // A method call: the door's when the door is a method.
                    own && door.method
                } else if let Some(q) = before.strip_suffix("::") {
                    let q = q.trim_end();
                    let q = q.strip_suffix('>').unwrap_or(q).trim_end();
                    let through = sp
                        .quals
                        .iter()
                        .any(|m| q.ends_with(m.as_str()) && boundary_before(q, q.len() - m.len()));
                    let typed = door.method
                        && q.rsplit(|c: char| !(c.is_alphanumeric() || c == '_'))
                            .next()
                            .is_some_and(|seg| seg.starts_with(char::is_uppercase));
                    if own && typed && !through && door.receiver {
                        cite_at += 1;
                    }
                    own && (through || typed)
                } else if sp.bare.contains(name) {
                    true
                } else if own
                    && calls
                    && !sp.elsewhere.iter().any(|r| r.contains(&at))
                    && !parameter_scopes(file, name).iter().any(|r| r.contains(&at))
                {
                    // A bare call nothing in this file resolves: not
                    // dropped, but listed.
                    out.push((
                        test_utils::source::line(code, at),
                        Cite::Row(format!(
                            "<calls `{name}` bare where nothing in this file resolves it: import \
                             it from `{HOME_MODULE}`, or qualify it>"
                        )),
                    ));
                    continue;
                } else {
                    false
                };
                if !accepted {
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
            let cite = split.get(cite_at).unwrap_or_else(|| {
                panic!(
                    "{name} call with fewer than {} arguments: {:.200}",
                    cite_at + 1,
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
                                    method: false,
                                    receiver: false,
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
        .position(|(n, ty)| {
            n == ident && ty.split_whitespace().collect::<String>() == "&'staticstr"
        })
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

/// **The roster is the home's signatures**: every `pub` fn with a
/// row-like parameter is a door at that index, whatever the row is called
/// or typed, `pub(crate)` and methods included; a private fn, a `pub` fn
/// with none, one taking an array or slice of text (predicate names), a
/// `#[cfg(test)]` fn and a named non-door are not.
#[test]
fn the_doors_are_the_homes_fns_that_take_a_ledger_row() {
    let home = PathBuf::from("k_stats.rs");
    let doors = home_doors_but(
        &home,
        "pub fn fresh(ledger_row: &'static str) {}\n\
         fn private(ledger_row: &'static str) {}\n\
         pub fn decide<T: Decide>(name: &'static str, m: T, band: Band) -> S { m }\n\
         pub(crate) fn wrapped<T>(\n    name: &'static str,\n    ledger_row: &'static str,\n) {}\n\
         pub fn short(name: &'static str, ledger_row: &str) {}\n\
         pub fn renamed(name: &'static str, m: f64, audit: &'static str) {}\n\
         pub fn rebound(name: &'static str, mut ledger_row: &'static str) {}\n\
         pub fn by_row_name(m: f64, row: Row) {}\n\
         pub fn owned(name: &'static str, cited: String) {}\n\
         pub type Cite = &'static str;\npub type Cited = Option<Cite>;\n\
         pub fn aliased(name: &'static str, cited: Cited) {}\n\
         pub const fn constant(ledger_row: &'static str) {}\n\
         pub fn in_family(family: &'static str) {}\n\
         pub fn across(names: [&'static str; 2], band: Band) {}\n\
         pub fn listed(names: &[&str], band: Band) {}\n\
         impl Bracket {\n    pub fn cite(&self, ledger_row: &'static str) {}\n}\n\
         #[cfg(test)]\nmod tests {\n    pub fn in_a_test(ledger_row: &'static str) {}\n}\n",
        &["in_family"],
    );
    let read: Vec<(&str, usize, bool)> = doors
        .iter()
        .map(|d| (d.name.as_str(), d.cite, d.method))
        .collect();
    assert_eq!(
        read,
        [
            ("fresh", 0, false),
            ("wrapped", 1, false),
            ("short", 1, false),
            ("renamed", 2, false),
            ("rebound", 1, false),
            ("by_row_name", 1, false),
            ("owned", 1, false),
            ("aliased", 1, false),
            ("constant", 0, false),
            ("cite", 0, true),
        ],
        "the home's row-citing doors"
    );
}

/// A non-door the home no longer defines is a stale entry.
#[test]
#[should_panic(expected = "drop the stale entry")]
fn a_stale_non_door_stops_the_census() {
    let _ = home_doors_but(
        Path::new("k_stats.rs"),
        "pub fn d(ledger_row: &str) {}\n",
        &["gone"],
    );
}

/// A fn with two row-like parameters is one the census cannot classify.
#[test]
#[should_panic(expected = "two row-like parameters")]
fn two_row_like_parameters_stop_the_census() {
    let _ = home_doors_but(
        Path::new("k_stats.rs"),
        "pub fn d(name: &'static str, a: &'static str, ledger_row: &str) {}\n",
        &[],
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
    let roster = home_doors_but(Path::new(home), &files[0].1, &[]);
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
/// through `k_stats` (with or without angle brackets), through an `as`
/// name of it, or `self::` in the home, and a bare call of a name the file
/// imports from it (in a group, nested, by glob or renamed) are sites; so
/// is a method door's `.m(` and `Type::m(`. A method of another type,
/// another module's fn of the same name, a longer name, and a bare call
/// the file imports from elsewhere or binds itself are not. A bare call
/// nothing resolves (a `use super::*` child, no import at all) is a site
/// citing no row.
#[test]
fn a_door_of_the_home_is_its_modules_fn_and_not_the_verb() {
    let home = "k_stats.rs";
    let files = [
        planted(
            home,
            "pub fn choose<T: Decide>(name: &'static str, m: M<T>, ledger_row: &'static str) -> Pick {\n    Pick::Tie\n}\n\
             fn inner() { self::choose(\"n\", m, \"F0\"); }\n\
             impl Bracket {\n    pub fn cite(&self, ledger_row: &'static str) {}\n}\n",
        ),
        planted(
            "qualified.rs",
            "fn a() { geom_core::k_stats::choose(\"n\", m, \"F1\"); }\n",
        ),
        planted(
            "angled.rs",
            "fn a() { <k_stats>::choose(\"n\", m, \"F6\"); }\n",
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
            "use geom_core::{Band, k_stats::{self, choose as pick}};\nfn i() { pick(\"n\", m, \"F5\"); }\n",
        ),
        planted(
            "method.rs",
            "fn j(b: Bracket) { b.cite(\"F7\"); Bracket::cite(&b, \"F8\"); }\n",
        ),
        planted(
            "child.rs",
            "use super::*;\nfn k() { choose(\"n\", m, \"F9\"); }\n",
        ),
        planted("unimported.rs", "fn e() { choose(n, k); }\n"),
        planted(
            "elsewhere.rs",
            "use rand::seq::SliceRandom;\nuse other::choose;\nfn c() { v.choose(&mut rng); }\nfn d() { other::choose(1, 2); choose(1, 2); }\n\
             fn f() { k_stats::choose_x(1); my_k_stats::choose(1, 2); }\n",
        ),
        planted(
            "bound.rs",
            "fn l() { let choose = |a, b| a; choose(1, 2); }\nfn cite() {}\nfn m() { x.other(); }\n",
        ),
    ];
    let roster = home_doors_but(Path::new(home), &files[0].1, &[]);
    let unresolved = "<calls `choose` bare where nothing in this file resolves it: import it from \
                      `k_stats`, or qualify it>";
    let want: Vec<(String, String)> = [
        ("aliased.rs", "F4"),
        ("angled.rs", "F6"),
        ("child.rs", unresolved),
        ("glob.rs", "F3"),
        ("imported.rs", "F2"),
        (home, "F0"),
        ("method.rs", "F7"),
        ("method.rs", "F8"),
        ("qualified.rs", "F1"),
        ("renamed.rs", "F5"),
        ("unimported.rs", unresolved),
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

/// **Another meaning of a door's name holds only in its own scope**: a
/// `let`, an import or a parameter of that name elsewhere in the file, a
/// closure parameter outside its closure's body, a method of that name,
/// or a struct field of it anywhere, leaves a bare call it does not scope
/// over unresolved; so does a match or-pattern or a bitwise or, which
/// open no closure. Inside its scope the call is that other meaning.
#[test]
fn another_meaning_of_a_doors_name_holds_only_in_its_scope() {
    let home = "k_stats.rs";
    let files = [
        planted(
            home,
            "pub fn choose<T: Decide>(name: &'static str, m: M<T>, ledger_row: &'static str) -> Pick {\n    Pick::Tie\n}\n",
        ),
        planted(
            "let_elsewhere.rs",
            "use super::*;\nfn a() { let choose = 3; }\nfn b() { choose(\"n\", m, \"ROW\"); }\n",
        ),
        planted(
            "field.rs",
            "use super::*;\nstruct S { choose: bool }\nfn b() { choose(\"n\", m, \"ROW\"); }\n",
        ),
        planted(
            "import_elsewhere.rs",
            "use super::*;\nmod x { use other::choose; }\nfn b() { choose(\"n\", m, \"ROW\"); }\n",
        ),
        planted(
            "let_after.rs",
            "use super::*;\nfn b() { choose(\"n\", m, \"ROW\"); let choose = |a, b| a; choose(1, 2); }\n",
        ),
        planted(
            "or_pattern.rs",
            "use super::*;\nfn b(x: u8) { match x { 1 | 2 => g(1, choose(\"n\", m, \"ROW\")), _ => {} } }\n",
        ),
        planted(
            "bitwise_or.rs",
            "use super::*;\nfn b(a: u8) { let y = a | b; h(x, choose(\"n\", m, \"ROW\")); }\n",
        ),
        planted(
            "method.rs",
            "use super::*;\nimpl S {\n    fn choose(&self) {}\n    fn b(&self) { choose(\"n\", m, \"ROW\"); }\n}\n\
             trait T {\n    fn choose(&self);\n    fn c(&self) { choose(\"n\", m, \"ROW\"); }\n}\n",
        ),
        planted(
            "closure_scope.rs",
            "use super::*;\nfn b() { let f = |choose: u8| choose + 1; choose(\"n\", m, \"ROW\"); }\n",
        ),
        planted(
            "in_scope.rs",
            "use super::*;\nfn a(choose: F) { choose(1, 2); }\nmod x { use other::choose; fn c() { choose(1, 2); } }\n\
             fn d() { [1].map(|x, choose| choose(x)); }\nfn choose_local() {}\nmod y { fn choose(a: u8, b: u8) {} fn e() { choose(1, 2); } }\n",
        ),
    ];
    let roster = home_doors_but(Path::new(home), &files[0].1, &[]);
    let unresolved = "<calls `choose` bare where nothing in this file resolves it: import it from \
                      `k_stats`, or qualify it>";
    let want: Vec<(String, String)> = [
        "bitwise_or.rs",
        "closure_scope.rs",
        "field.rs",
        "import_elsewhere.rs",
        "let_after.rs",
        "let_elsewhere.rs",
        "method.rs",
        "method.rs",
        "or_pattern.rs",
    ]
    .into_iter()
    .map(|f| (f.to_string(), unresolved.to_string()))
    .collect();
    assert_eq!(
        rows_of(&door_sites(&files, &roster)),
        want,
        "each binding's own scope"
    );
}

/// **A re-export of a door is a site citing no row**: its importers call
/// it by a path the census does not read. The row names what was
/// re-exported, the door or the module under another name.
#[test]
fn a_re_exported_door_is_a_site_citing_no_row() {
    let home = "k_stats.rs";
    let files = [
        planted(
            home,
            "pub fn choose<T: Decide>(name: &'static str, m: M<T>, ledger_row: &'static str) -> Pick {\n    Pick::Tie\n}\n",
        ),
        planted("door.rs", "pub use geom_core::k_stats::choose;\n"),
        planted("module.rs", "pub(crate) use geom_core::k_stats as ks;\n"),
        planted("private.rs", "use geom_core::k_stats as ks;\n"),
    ];
    let roster = home_doors_but(Path::new(home), &files[0].1, &[]);
    let want: Vec<(String, String)> = [
        (
            "door.rs",
            "<re-exports `choose` (`pub use geom_core::k_stats::choose`): its importers reach the \
             door by a path this census does not read>",
        ),
        (
            "module.rs",
            "<re-exports the module `k_stats` as `ks` (`pub use geom_core::k_stats`): its \
             importers reach the door by a path this census does not read>",
        ),
    ]
    .into_iter()
    .map(|(f, r)| (f.to_string(), r.to_string()))
    .collect();
    assert_eq!(
        rows_of(&door_sites(&files, &roster)),
        want,
        "each re-export is listed"
    );
}
