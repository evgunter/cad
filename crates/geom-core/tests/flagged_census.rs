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
//! where the name resolves to an import of it, in a group, by glob or
//! renamed; a method door's `.m(` and `Type::m(` count too. A fn that
//! forwards a `&'static str` parameter to a door's row is a door in its
//! own right, keyed on its name, and its callers are checked
//! ([`forwarding_door`]), to a fixed point.
//!
//! **Scopes are read off a parse** ([`source_scope`]): a bare name
//! resolves to its innermost binding, a `use`, a `let`, a closure's or a
//! pattern's binding, a parameter or an item, as Rust resolves it, so a
//! call inside another binding of the door's name is that other meaning
//! and a field or a method binds nothing. This census asks only its own
//! question of what the parse resolves.
//!
//! **What the census cannot read, it lists rather than skips**: a row
//! that is not a string literal and not a forwarded parameter; a
//! forwarded parameter something rebinds before the call; a door named
//! without a call (a fn pointer, a binding); a door's name inside a
//! macro whose arguments do not parse as expressions, or in a
//! `macro_rules!` body; a bare call of a door's name that nothing in its
//! file resolves (a `use super::*` child, no import); a `pub use`
//! re-export of a door or of the module under another name. Each is a
//! site citing no row. A home fn with two row-like parameters, a stale
//! [`NON_ROW_DOORS`] entry and a file that does not parse stop it.
//!
//! **Blind spots, stated:** a call a macro generates; a door whose row
//! reaches it inside a struct argument, or as a type named for text
//! outside the home (the shape `k_stats`' header rules out); and what a
//! glob other than `k_stats::*` brings in, which is read as binding
//! nothing.
//!
//! The `decide_flagged` total is hand-synced and spelled ONCE, at
//! [`LEDGER_FLAGGED_SITES`]; the rows are computed from the document.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use source_scope::{Arg, Bound, Callee, Event, Source};

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

/// What a door call cites as its ledger row.
enum Cite {
    /// A string literal: the row itself.
    Row(String),
    /// An identifier passed straight through: a wrapper's forwarding,
    /// which [`forwarding_door`] resolves.
    Forwarded {
        ident: String,
        bound: Option<Bound>,
        around: Option<usize>,
    },
}

/// The home of the doors, under the crates directory, and the module a
/// call reaches them through.
const HOME: &str = "geom-core/src/k_stats.rs";
const HOME_MODULE: &str = "k_stats";

/// A door that cites a ledger row: its name, the row's argument index
/// (from 0), and, for a door of the home, the file that defines it. A
/// door of the home is reached through [`HOME_MODULE`] or an import of
/// it ([`door_calls_in`]); a forwarding wrapper ([`forwarding_door`]) is
/// keyed on its name alone, which reads every same-named call as one of
/// its sites: the loud direction.
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

/// The home's `pub` fns that take a row-like parameter and cite no
/// ledger row: each must be one ([`home_doors`] refuses a stale entry).
const NON_ROW_DOORS: [&str; 2] = ["in_family", "identity_pass_set"];

/// Whether a parameter reads as a ledger row: a name holding `row`, or,
/// under any name but the predicate's `name`, a type that names `str` or
/// `String` (`&'static str`, `&str`, `String`, `Cow<str>`) or one of
/// `texts`, the home's aliases of those, and is no list. A list of text,
/// `[&'static str; 2]` or `&'static [String]`, is predicate names, as
/// `name` is, unless its name holds `row`.
fn row_like(p: &source_scope::Param, texts: &[String]) -> bool {
    let Some(name) = p.name.as_deref() else {
        return false;
    };
    name.contains("row")
        || (name != "name"
            && !p.ty.list
            && p.ty
                .idents
                .iter()
                .any(|w| w == "str" || w == "String" || texts.contains(w)))
}

/// The home's `type` aliases of a text type (`str`, `String`, or another
/// such alias), to a fixed point.
fn text_aliases(src: &Source) -> Vec<String> {
    let mut texts: Vec<String> = Vec::new();
    loop {
        let before = texts.len();
        for alias in &src.type_aliases {
            let texty = alias
                .rhs
                .iter()
                .any(|w| w == "str" || w == "String" || texts.contains(w));
            if texty && !texts.contains(&alias.name) {
                texts.push(alias.name.clone());
            }
        }
        if texts.len() == before {
            return texts;
        }
    }
}

/// `text`, parsed: a file that does not parse stops the census.
fn parse(path: &Path, text: &str) -> Source {
    source_scope::read(text).unwrap_or_else(|e| {
        panic!(
            "{} does not parse ({e}): this census reads scopes off a parse, so a file it \
             cannot parse is a file it cannot check",
            path.display()
        )
    })
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
    let src = parse(home, text);
    let texts = text_aliases(&src);
    let mut doors = Vec::new();
    let mut takers = Vec::new();
    for f in &src.fns {
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
                f.params.iter().map(|p| p.name.clone()).collect::<Vec<_>>()
            ),
        };
        takers.push(f.name.clone());
        if !but.contains(&f.name.as_str()) {
            doors.push(Door {
                name: f.name.clone(),
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

/// A source file read once for every door, its events indexed by the
/// name they reach: their own, and for a bare name bound by an import,
/// the imported item's (`choose as pick` is found under `choose`).
struct Scanned<'a> {
    path: &'a Path,
    src: Source,
    by_name: std::collections::HashMap<String, Vec<usize>>,
}

impl<'a> Scanned<'a> {
    fn of(path: &'a Path, text: &str) -> Self {
        let src = parse(path, text);
        let mut by_name: std::collections::HashMap<String, Vec<usize>> =
            std::collections::HashMap::new();
        for (i, ev) in src.events.iter().enumerate() {
            by_name.entry(ev.name.clone()).or_default().push(i);
            if let Callee::Bare(Some(Bound::Use { path, .. })) = &ev.callee
                && let Some(last) = path.last().filter(|l| **l != ev.name)
            {
                by_name.entry(last.clone()).or_default().push(i);
            }
        }
        Scanned { path, src, by_name }
    }
}

/// Whether `path` ends with the segments `tail`.
fn ends(path: &[String], tail: &[&str]) -> bool {
    path.len() >= tail.len() && path[path.len() - tail.len()..].iter().eq(tail.iter())
}

/// How one event reaches a door.
enum Reach {
    /// A site of the door, its row at this argument index.
    Site(usize),
    /// A bare call of the door's own name that nothing in the file
    /// resolves.
    Unresolved,
    /// The door's name inside macro tokens this census cannot parse.
    InMacro,
    /// Not the door.
    No,
}

/// How `ev` reaches `door`, given the file's module qualifiers `quals`.
fn reach(ev: &Event, door: &Door, quals: &[String], in_home: bool) -> Reach {
    let own = ev.name == door.name;
    if door.home.is_none() {
        return match (own, &ev.callee) {
            (false, _) => Reach::No,
            (true, Callee::Macro) => Reach::InMacro,
            (true, _) => Reach::Site(door.cite),
        };
    }
    match &ev.callee {
        Callee::Bare(Some(Bound::Use { path, .. })) if ends(path, &[HOME_MODULE, &door.name]) => {
            Reach::Site(door.cite)
        }
        Callee::Bare(Some(Bound::Glob(globs)))
            if own && globs.iter().any(|g| ends(g, &[HOME_MODULE, "*"])) =>
        {
            Reach::Site(door.cite)
        }
        Callee::Bare(Some(Bound::Item)) if own && in_home => Reach::Site(door.cite),
        Callee::Bare(None | Some(Bound::Glob(_))) if own && ev.args.is_some() => Reach::Unresolved,
        Callee::Path { qualifier } if own => {
            let q = qualifier.as_deref().unwrap_or("");
            let through = quals.iter().any(|m| m == q);
            let typed = door.method && q.starts_with(char::is_uppercase);
            match (through, typed) {
                (true, _) => Reach::Site(door.cite),
                (false, true) => Reach::Site(door.cite + usize::from(door.receiver)),
                (false, false) => Reach::No,
            }
        }
        Callee::Method if own && door.method => Reach::Site(door.cite),
        Callee::Macro if own => Reach::InMacro,
        _ => Reach::No,
    }
}

/// Every site of `door` in `file`, with what each cites at the row's
/// index; an identifier is a [`Cite::Forwarded`].
///
/// **A door named and not called is a site citing no row**: a fn
/// pointer (`let f = choose;`), anything that reaches the door other than
/// by a call this census can read. A `pub use` re-export of the door, or
/// of the module under another name, is one too: its importers reach the
/// door by a path this census does not read.
fn door_calls_in(file: &Scanned, door: &Door) -> Vec<(usize, Cite)> {
    let in_home = door.home.as_deref() == Some(file.path);
    let mut out = Vec::new();
    let mut quals = vec![HOME_MODULE.to_string()];
    if in_home {
        quals.push("self".to_string());
    }
    if door.home.is_some() {
        for leaf in &file.src.uses {
            let module = ends(&leaf.path, &[HOME_MODULE]);
            let is_door = ends(&leaf.path, &[HOME_MODULE, &door.name]);
            let glob = ends(&leaf.path, &[HOME_MODULE, "*"]);
            if let (Some(a), true) = (&leaf.alias, module)
                && a != "_"
            {
                quals.push(a.clone());
            }
            if !in_home && leaf.public && (is_door || glob || (module && leaf.alias.is_some())) {
                let what = match (&leaf.alias, module) {
                    (Some(a), true) => format!("the module `{HOME_MODULE}` as `{a}`"),
                    _ => format!("`{}`", door.name),
                };
                out.push((
                    leaf.line,
                    Cite::Row(format!(
                        "<re-exports {what} (`pub use {}`): its importers reach the door by a \
                         path this census does not read>",
                        leaf.path.join("::")
                    )),
                ));
            }
        }
    }
    let events = file.by_name.get(&door.name).map_or(&[][..], Vec::as_slice);
    for ev in events.iter().map(|&i| &file.src.events[i]) {
        let name = &ev.name;
        let cite_at = match reach(ev, door, &quals, in_home) {
            Reach::No => continue,
            Reach::Unresolved => {
                out.push((
                    ev.line,
                    Cite::Row(format!(
                        "<calls `{name}` bare where nothing in this file resolves it: import \
                         it from `{HOME_MODULE}`, or qualify it>"
                    )),
                ));
                continue;
            }
            Reach::InMacro => {
                out.push((
                    ev.line,
                    Cite::Row(format!(
                        "<names `{name}` inside a macro whose arguments this census cannot \
                         parse>"
                    )),
                ));
                continue;
            }
            Reach::Site(at) => at,
        };
        let Some(args) = &ev.args else {
            out.push((
                ev.line,
                Cite::Row(format!(
                    "<names `{name}` without calling it: a fn pointer, a binding or a macro \
                     argument this census cannot follow>"
                )),
            ));
            continue;
        };
        let cited = args.get(cite_at).unwrap_or_else(|| {
            panic!(
                "{}:{}: `{name}` call with fewer than {} arguments",
                file.path.display(),
                ev.line,
                cite_at + 1
            )
        });
        out.push((
            ev.line,
            match cited {
                Arg::Str(row) => Cite::Row(row.clone()),
                Arg::Ident { name, bound } => Cite::Forwarded {
                    ident: name.clone(),
                    bound: bound.clone(),
                    around: ev.enclosing_fn,
                },
                Arg::Other => panic!(
                    "{}:{}: a door's site cites its ledger row as an expression, not a string \
                     literal. The census can only verify literals; give the site its row \
                     directly, or teach this test to resolve the indirection — do not leave it \
                     unchecked.",
                    file.path.display(),
                    ev.line
                ),
            },
        ));
    }
    out
}

/// The sites of `doors` in `files`, wrappers resolved to a fixed point
/// ([`forwarding_door`]).
fn door_sites(files: &[(PathBuf, String)], doors: &[Door]) -> Vec<Site> {
    let scanned: Vec<Scanned> = files.iter().map(|(p, t)| Scanned::of(p, t)).collect();
    let mut known: Vec<Door> = doors.to_vec();
    let mut hits: Vec<Site> = Vec::new();
    let mut next = 0;
    while next < known.len() {
        let door = known[next].clone();
        next += 1;
        for file in &scanned {
            let path = &file.path.to_path_buf();
            for (line, found) in door_calls_in(file, &door) {
                match found {
                    Cite::Row(ledger_row) => hits.push(Site {
                        file: path.clone(),
                        line,
                        ledger_row,
                    }),
                    Cite::Forwarded {
                        ident,
                        bound,
                        around,
                    } => {
                        let around = around.map(|i| &file.src.fns[i]);
                        match forwarding_door(around, &ident, bound.as_ref()) {
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

/// **The door a forwarding call makes of the fn around it** (`around`):
/// where that fn takes `ident` as a `&'static str` parameter, its name
/// and that parameter's index.
///
/// **The parameter must be what the call's `ident` resolves to.** A
/// `let`, a closure's parameter or a pattern that binds `ident` again
/// between the fn's head and the call (`bound`, as the parse resolves it)
/// means the caller's literal is not the row the door receives: refused,
/// as a site citing no row, rather than read as a forwarding.
fn forwarding_door(
    around: Option<&source_scope::FnItem>,
    ident: &str,
    bound: Option<&Bound>,
) -> Result<(String, usize), String> {
    let f = around.ok_or_else(|| "outside any fn".to_string())?;
    let index = f
        .params
        .iter()
        .position(|p| p.name.as_deref() == Some(ident) && p.ty.static_str)
        .ok_or_else(|| "which is no `&'static str` parameter of the fn around it".to_string())?;
    match bound {
        Some(Bound::Param { fn_name, index: i }) if *fn_name == f.name && *i == index => {
            Ok((f.name.clone(), index))
        }
        _ => Err(format!(
            "which the body of `{}` names before the call, so it may be rebound",
            f.name
        )),
    }
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
         pub fn one(label: [&'static str; 1]) {}\n\
         pub fn owned_list(labels: &[String]) {}\n\
         pub fn static_list(labels: &'static [&'static str]) {}\n\
         pub fn row_list(rows: &'static [&'static str]) {}\n\
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
            ("row_list", 0, false),
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

/// **A pattern's binding holds over what the pattern governs, and no
/// further**: an `if let`, a `for`, a match arm and a tuple `let` each
/// bind `choose` somewhere in these files, and a call outside that scope
/// is still a door call. A parameter of one fn does not shadow an
/// import in another. Inside each scope the name is the local, and
/// nothing is reported. A lexical scope reader read each of these
/// shapes as file-wide or block-wide and dropped the door call beside
/// it. That is the census's silent direction, and these shapes are what
/// the move onto a parse closed.
#[test]
fn a_patterns_binding_holds_only_over_what_the_pattern_governs() {
    let home = "k_stats.rs";
    let files = [
        planted(
            home,
            "pub fn choose<T: Decide>(name: &'static str, m: M<T>, ledger_row: &'static str) -> Pick {\n    Pick::Tie\n}\n",
        ),
        planted(
            "if_let_beside.rs",
            "use super::*;\nfn b(x: O) {\n    if let Some(choose) = x { g(choose); }\n    choose(\"n\", m, \"ROW\");\n}\n",
        ),
        planted(
            "for_beside.rs",
            "use super::*;\nfn b() {\n    for choose in xs { g(choose); }\n    choose(\"n\", m, \"ROW\");\n}\n",
        ),
        planted(
            "arm_beside.rs",
            "use super::*;\nfn b(x: O) {\n    match x { Some(choose) => g(choose), None => {} }\n    choose(\"n\", m, \"ROW\");\n}\n",
        ),
        planted(
            "tuple_let_after.rs",
            "use super::*;\nfn b() {\n    choose(\"n\", m, \"ROW\");\n    let (choose, y) = (1, 2);\n}\n",
        ),
        planted(
            "param_elsewhere.rs",
            "use crate::k_stats::choose;\nfn a(choose: F) { choose(1, 2); }\nfn b() { choose(\"n\", m, \"ROW\"); }\n",
        ),
        planted(
            "in_scope.rs",
            "use crate::k_stats::choose;\nfn a(x: O, fs: V) {\n    if let Some(choose) = x { choose(1, 2); }\n    \
             for choose in fs { choose(1, 2); }\n    match x { Some(choose) => choose(1, 2), None => {} }\n    \
             let (choose, y) = (f, 2);\n    choose(1, 2);\n}\n",
        ),
    ];
    let roster = home_doors_but(Path::new(home), &files[0].1, &[]);
    let unresolved = "<calls `choose` bare where nothing in this file resolves it: import it from \
                      `k_stats`, or qualify it>";
    let mut want: Vec<(String, String)> = [
        "arm_beside.rs",
        "for_beside.rs",
        "if_let_beside.rs",
        "tuple_let_after.rs",
    ]
    .into_iter()
    .map(|f| (f.to_string(), unresolved.to_string()))
    .collect();
    want.push(("param_elsewhere.rs".to_string(), "ROW".to_string()));
    want.sort();
    let mut got = rows_of(&door_sites(&files, &roster));
    got.sort();
    assert_eq!(got, want, "each pattern's own scope");
}
