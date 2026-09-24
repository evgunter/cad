//! **The door values in the tree and the `CertifiedEnclosure` impls,
//! each counted against the door values' wiring rows** — the
//! which-scalars half of the census the deleted lane traits' identity
//! tests carried, and the which-doors half beside it.
//!
//! A door value is a `Copy` struct of private fn-pointer fields with one
//! constructor. The type system carries the which-body half: each
//! constructor's `impl` block is bounded on `CertifiedBounds` (or is
//! concrete at `f64`), so a scalar without the right cannot hold one. It
//! carries neither of the halves this census reads.
//!
//! **Which scalars.** Each door's wiring rows are a hand-written list of
//! the scalars whose door is pinned by pointer identity, one call of the
//! door's helper per scalar. A new `impl CertifiedEnclosure` would form
//! every certifying door in the tree and owe a row at each without
//! anything going red. [`every_certifying_scalar_has_a_wiring_row_and_every_row_a_scalar`]
//! is that red: every roster formed at the certifying scalars must
//! instantiate its helper at exactly the `CertifiedEnclosure` impls,
//! less the ones that are not door scalars (each with its reason,
//! below), and a roster formed at `f64` alone must call its helper and
//! owes no row anywhere else. Both directions are checked: an impl with
//! no row is a missing pin, and a row for a type with no impl is a stale
//! roster.
//!
//! **Which doors.** A door value that is not in [`ROSTERS`] at all is
//! pinned nowhere this census reads, and no count of scalars can see it
//! — the failure this half exists for is a door that appeared and
//! nothing reddened. [`every_door_value_has_a_roster_and_every_roster_a_door`]
//! reads every door constructor off the code view of every
//! `crates/*/src` file and requires a roster entry for each, in the
//! file that defines it, formed where its constructor's `impl` says it
//! is.
//!
//! **What the door reader sees, and its blind spot.** A door
//! constructor, for this reader, is a zero-parameter associated `fn`
//! returning `Self` (or the self type spelled out) at the top level of
//! an INHERENT `impl` block that either names the right in its bounds
//! (`CertifiedBounds` or `CertifiedEnclosure`, in the generic list or a
//! `where` clause, across any number of lines), or is concrete at
//! `f64`, or names the fn `certified` or `fit` — the two spellings the
//! tree's doors use. It does not see: a door whose only constructor
//! takes a parameter; a door made by a free function, a `const` item
//! or a trait impl; a door whose constructor carries neither name and
//! whose `impl` reaches the right only through a trait that implies it
//! without naming it; a door concrete at a scalar other than `f64`; and
//! anything outside `crates/*/src`. A door value's other mark — its
//! fn-pointer fields — is swept by hand in the PR that wrote this
//! reader, not here: most fn-pointer structs in the tree are not
//! certification doors, so the field shape alone cannot key a census.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};

use test_utils::source::{
    ImplHead, ItemBody, angle_end, balanced_end, code_only, ident, impl_head, item_body, line,
    repo_root, rust_sources, skip_ws, type_base, word_at,
};

/// Where a door value is formed, as its roster entry states it.
#[derive(Clone, Copy, Debug)]
enum Formed {
    /// At every scalar implementing `CertifiedEnclosure` (less
    /// [`NOT_A_DOOR_SCALAR`]): the constructor's `impl` is generic and
    /// bounded on the right, so the helper is generic and owes one row
    /// per such scalar, read off `helper::<Scalar>()`.
    CertifyingScalars,
    /// At `f64` alone, for the reason carried: the constructor's `impl`
    /// is concrete, so the helper is not generic, owes its bare call
    /// `helper()`, and no new `CertifiedEnclosure` impl owes it a row.
    F64Only(&'static str),
}

/// One door value's pin: the door, the file its constructor and its
/// wiring rows live in, the helper those rows call, and where it is
/// formed.
struct Roster {
    door: &'static str,
    file: &'static str,
    helper: &'static str,
    formed: Formed,
}

/// The rosters, one per door value in the tree.
const ROSTERS: [Roster; 4] = [
    Roster {
        door: "OffsetFitLane",
        file: "crates/geom-brep/src/offset_fit_lane.rs",
        helper: "holds_the_offset_fit",
        formed: Formed::F64Only(
            "`OffsetFitLane::fit` is concrete at `f64`: the fit is a DERIVATION written at \
             `f64`, not a certification right, so a new `CertifiedEnclosure` impl forms no \
             `OffsetFitLane` and owes this roster nothing",
        ),
    },
    Roster {
        door: "QuadLane",
        file: "crates/topo/src/props.rs",
        helper: "holds_the_certified_quadrature",
        formed: Formed::CertifyingScalars,
    },
    Roster {
        door: "RegionLane",
        file: "crates/topo/src/chart_region.rs",
        helper: "holds_the_certified_region_doors",
        formed: Formed::CertifyingScalars,
    },
    Roster {
        door: "ShellDoor",
        file: "crates/topo/src/props.rs",
        helper: "holds_the_certified_shell_door",
        formed: Formed::CertifyingScalars,
    },
];

/// The impls that are not door scalars, each with the reason no wiring
/// row is owed. An entry here is an exemption and has to earn it.
const NOT_A_DOOR_SCALAR: [(&str, &str); 1] = [(
    "RingInterval",
    "a bracket currency (the ring the certified reads hand back), not a scalar: it \
     implements no `Decide`, so no door forms at it",
)];

/// Where a door constructor's `impl` says the door is formed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ConstructorImpl {
    /// The head names `CertifiedBounds` or `CertifiedEnclosure`.
    Certifying,
    /// The self type is concrete at `f64`.
    ConcreteF64,
    /// Neither: a door-named constructor formed at every scalar.
    Unbounded,
}

/// A door constructor as the tree writes it.
#[derive(Debug)]
struct Door {
    door: String,
    file: String,
    line: usize,
    constructor: String,
    formed: ConstructorImpl,
}

/// Every `crates/*/src` file, as its repository-relative path and its
/// code view.
fn crate_sources() -> Vec<(String, String)> {
    let root = repo_root(env!("CARGO_MANIFEST_DIR"));
    let mut out = Vec::new();
    let crates = std::fs::read_dir(root.join("crates")).expect("crates/ lists");
    for entry in crates {
        let src = entry.expect("a crate entry").path().join("src");
        if !src.is_dir() {
            continue;
        }
        for file in rust_sources(&src) {
            let text = std::fs::read_to_string(&file).expect("a crate source reads");
            let rel = file
                .strip_prefix(&root)
                .expect("a crate source is under the root")
                .to_string_lossy()
                .replace('\\', "/");
            out.push((rel, code_only(&text)));
        }
    }
    assert!(!out.is_empty(), "no crate source read — the walk is broken");
    out
}

/// An `impl` block: its keyword's offset, its head, and its body
/// (both braces included).
struct ImplBlock {
    at: usize,
    head: ImplHead,
    body: std::ops::Range<usize>,
}

/// Every `impl` block in a code view, nested ones included.
///
/// An `impl` keyword that ends at a `;` is an `impl Trait` in a
/// signature, not a block. One whose body opens is read as a block even
/// when it is an `impl Trait` argument ahead of a function body: that
/// "block" names no trait and its top-level items are the function's
/// local items, so it can only add a door, never hide one.
fn impl_blocks(code: &str) -> Vec<ImplBlock> {
    let mut out = Vec::new();
    for (at, _) in code.match_indices("impl") {
        if !word_at(code, at, "impl") {
            continue;
        }
        let body = match item_body(code, at) {
            ItemBody::Body(body) => body,
            ItemBody::Declaration(_) => continue,
            ItemBody::Unterminated => panic!("an `impl` head runs to end of file unterminated"),
        };
        let Some(head) = impl_head(code, at, body.start) else {
            continue;
        };
        out.push(ImplBlock { at, head, body });
    }
    out
}

/// The offset of the first whole-word `where` in `text`.
fn where_token(text: &str) -> Option<usize> {
    text.match_indices("where")
        .map(|(i, _)| i)
        .find(|&i| word_at(text, i, "where"))
}

/// The offsets of the `fn` keywords at the top level of an item body's
/// inside — the block's own items, not the items local to their bodies.
fn top_level_fns(inside: &str) -> Vec<usize> {
    let mut depth = 0usize;
    let mut out = Vec::new();
    for (at, c) in inside.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => depth = depth.saturating_sub(1),
            'f' if depth == 0 && word_at(inside, at, "fn") => out.push(at),
            _ => {}
        }
    }
    out
}

/// The name of the `fn` at `fn_at` when it is a constructor of the
/// self type whose base is `self_base`: no parameters, and a return of
/// `Self` or of the self type spelled out.
fn constructor<'a>(inside: &'a str, fn_at: usize, self_base: &str) -> Option<&'a str> {
    let at = skip_ws(inside, fn_at + "fn".len());
    let name = ident(inside, at);
    if name.is_empty() {
        return None;
    }
    let mut at = skip_ws(inside, at + name.len());
    if inside[at..].starts_with('<') {
        at = skip_ws(inside, angle_end(inside, at)? + 1);
    }
    if !inside[at..].starts_with('(') {
        return None;
    }
    let close = balanced_end(inside, at)?;
    if !inside[at + 1..close].trim().is_empty() {
        return None;
    }
    let at = skip_ws(inside, close + 1);
    let ret = inside[at..].strip_prefix("->")?;
    let ret = &ret[..ret.find(['{', ';']).unwrap_or(ret.len())];
    let ret = where_token(ret).map_or(ret, |w| &ret[..w]).trim();
    (ret == "Self" || type_base(ret) == self_base).then_some(name)
}

/// The door constructors one file's code view defines.
fn doors_in(file: &str, code: &str) -> Vec<Door> {
    let mut out = Vec::new();
    for block in impl_blocks(code) {
        if block.head.trait_path.is_some() {
            continue;
        }
        let self_base = type_base(&block.head.self_type);
        let header = &code[block.at..block.body.start];
        let bounded = ["CertifiedBounds", "CertifiedEnclosure"]
            .iter()
            .any(|right| {
                header
                    .match_indices(right)
                    .any(|(i, _)| word_at(header, i, right))
            });
        let concrete_f64 = block
            .head
            .self_type
            .split_once('<')
            .is_some_and(|(_, args)| args.trim_end().trim_end_matches('>').trim() == "f64");
        let inside = &code[block.body.start + 1..block.body.end - 1];
        for fn_at in top_level_fns(inside) {
            let Some(name) = constructor(inside, fn_at, self_base) else {
                continue;
            };
            let named = name == "certified" || name == "fit";
            if !(bounded || concrete_f64 || named) {
                continue;
            }
            out.push(Door {
                door: self_base.to_string(),
                file: file.to_string(),
                line: line(code, block.body.start + 1 + fn_at),
                constructor: name.to_string(),
                formed: if bounded {
                    ConstructorImpl::Certifying
                } else if concrete_f64 {
                    ConstructorImpl::ConcreteF64
                } else {
                    ConstructorImpl::Unbounded
                },
            });
        }
    }
    out
}

/// Every `impl … CertifiedEnclosure for X` in the tree's crate sources,
/// as the base of `X` — and every textual `CertifiedEnclosure for` in
/// code is one of them, so a spelling the head reader cannot parse is a
/// red rather than a missing impl.
fn impls(sources: &[(String, String)]) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for (file, code) in sources {
        let mut seen = 0usize;
        for block in impl_blocks(code) {
            if block
                .head
                .trait_path
                .as_deref()
                .is_some_and(|t| type_base(t) == "CertifiedEnclosure")
            {
                seen += 1;
                out.insert(type_base(&block.head.self_type).to_string());
            }
        }
        let written = code
            .match_indices("CertifiedEnclosure")
            .filter(|&(i, m)| {
                word_at(code, i, m) && word_at(code, skip_ws(code, i + m.len()), "for")
            })
            .count();
        assert_eq!(
            seen, written,
            "{file}: {written} `CertifiedEnclosure for` in code, {seen} read as impl heads — \
             an impl this census cannot parse is one it cannot count"
        );
    }
    out
}

/// One call of a roster's helper.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Row {
    /// `helper::<Scalar>()`, as the scalar's base.
    At(String),
    /// `helper()`.
    Bare,
}

/// Every call of `helper` in `code`, and the body of its one
/// definition. Any other mention is a red: a row this census cannot
/// read is one it cannot count.
fn rows(file: &str, code: &str, helper: &str) -> (Vec<Row>, String) {
    let mut out = Vec::new();
    let mut definitions = Vec::new();
    for (at, _) in code.match_indices(helper) {
        if !word_at(code, at, helper) {
            continue;
        }
        let before = code[..at].trim_end();
        if before.ends_with("fn") && word_at(before, before.len() - 2, "fn") {
            let ItemBody::Body(body) = item_body(code, at) else {
                panic!("{file}: `fn {helper}` has no body");
            };
            definitions.push(code[body].to_string());
            continue;
        }
        let after = skip_ws(code, at + helper.len());
        if code[after..].starts_with("::<") {
            let open = after + "::".len();
            let close = angle_end(code, open)
                .unwrap_or_else(|| panic!("{file}: `{helper}::<` never closes"));
            let call = skip_ws(code, close + 1);
            assert!(
                code[call..].starts_with("()"),
                "{file}:{}: `{helper}::<…>` is not called with no arguments",
                line(code, at)
            );
            out.push(Row::At(type_base(&code[open + 1..close]).to_string()));
        } else if code[after..].starts_with("()") {
            out.push(Row::Bare);
        } else {
            panic!(
                "{file}:{}: `{helper}` is mentioned but neither defined nor called as a row",
                line(code, at)
            );
        }
    }
    assert_eq!(
        definitions.len(),
        1,
        "{file} must define `fn {helper}` exactly once"
    );
    (out, definitions.pop().expect("one definition"))
}

/// The code view of a repository-relative file.
fn code_of(file: &str) -> String {
    let path = repo_root(env!("CARGO_MANIFEST_DIR")).join(file);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{file} reads: {e}"));
    code_only(&text)
}

#[test]
fn every_door_value_has_a_roster_and_every_roster_a_door() {
    let mut found: BTreeMap<String, Vec<Door>> = BTreeMap::new();
    for (file, code) in &crate_sources() {
        for door in doors_in(file, code) {
            found.entry(door.door.clone()).or_default().push(door);
        }
    }
    let mut failures = Vec::new();
    for (name, doors) in &found {
        let Some(roster) = ROSTERS.iter().find(|r| r.door == name) else {
            failures.push(format!(
                "{name} is a door value — {doors:?} — and has no entry in ROSTERS: give it a \
                 wiring helper in the file that defines it, one row per scalar it is formed \
                 at, and its roster entry here"
            ));
            continue;
        };
        if doors.len() != 1 {
            failures.push(format!(
                "{name} has {} door constructors — {doors:?} — and a door value has one",
                doors.len()
            ));
        }
        for door in doors {
            if door.file != roster.file {
                failures.push(format!(
                    "{name}'s constructor is in {}:{} but its roster names {} — the pin lives \
                     beside the private fields it reads",
                    door.file, door.line, roster.file
                ));
            }
            let agrees = matches!(
                (roster.formed, door.formed),
                (Formed::CertifyingScalars, ConstructorImpl::Certifying)
                    | (Formed::F64Only(_), ConstructorImpl::ConcreteF64)
            );
            if !agrees {
                failures.push(format!(
                    "{name}'s roster says it is formed {:?} but its constructor `{}` at {}:{} \
                     is {:?}",
                    roster.formed, door.constructor, door.file, door.line, door.formed
                ));
            }
        }
    }
    for roster in &ROSTERS {
        if !found.contains_key(roster.door) {
            failures.push(format!(
                "ROSTERS names {} but no door constructor of that name is in crates/*/src — \
                 the roster is stale",
                roster.door
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

#[test]
fn every_certifying_scalar_has_a_wiring_row_and_every_row_a_scalar() {
    let impls = impls(&crate_sources());
    let exempt: BTreeSet<String> = NOT_A_DOOR_SCALAR
        .iter()
        .map(|(ty, _)| (*ty).to_string())
        .collect();
    for ty in &exempt {
        assert!(
            impls.contains(ty),
            "{ty} is exempted here as not a door scalar but implements `CertifiedEnclosure` \
             nowhere in the tree — the exemption is stale"
        );
    }
    let door_scalars: BTreeSet<Row> = impls.difference(&exempt).cloned().map(Row::At).collect();
    let mut failures = Vec::new();
    for roster in &ROSTERS {
        let (rows, definition) = rows(roster.file, &code_of(roster.file), roster.helper);
        if !definition
            .match_indices(roster.door)
            .any(|(i, _)| word_at(&definition, i, roster.door))
        {
            failures.push(format!(
                "{}'s `{}` never names {} — the roster's helper is another door's",
                roster.file, roster.helper, roster.door
            ));
        }
        let rows: BTreeSet<Row> = rows.into_iter().collect();
        match roster.formed {
            Formed::CertifyingScalars if rows != door_scalars => failures.push(format!(
                "{} is formed at every certifying scalar: the scalars implementing \
                 `CertifiedEnclosure` (less the exemptions {exempt:?}) are {door_scalars:?} \
                 and {}'s `{}` rows pin it at {rows:?}. An impl with no row forms the door \
                 unpinned, and a row with no impl is a stale roster: add the row in {}, or \
                 the exemption with its reason here",
                roster.door, roster.file, roster.helper, roster.file
            )),
            Formed::F64Only(reason) if rows != BTreeSet::from([Row::Bare]) => {
                failures.push(format!(
                    "{} is formed at `f64` alone ({reason}), so its `{}` in {} owes its bare \
                     call `{}()` and no row at any other scalar; the rows read are {rows:?}",
                    roster.door, roster.helper, roster.file, roster.helper
                ));
            }
            _ => {}
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// The door reader on each shape it claims to see, and on the shapes
/// beside them it must not count.
#[test]
fn the_door_reader_sees_each_constructor_shape() {
    let doors = |src: &str| -> Vec<(String, String, ConstructorImpl)> {
        doors_in("x.rs", &code_only(src))
            .into_iter()
            .map(|d| (d.door, d.constructor, d.formed))
            .collect()
    };
    let certifying = |door: &str, ctor: &str| {
        vec![(
            door.to_string(),
            ctor.to_string(),
            ConstructorImpl::Certifying,
        )]
    };
    // A bound list across lines, and a path-qualified right.
    assert_eq!(
        doors(
            "impl<\n    T: Decide\n        + geom_core::CertifiedBounds,\n> Lane<T> {\n    \
             pub const fn certified() -> Self { Self { f: g::<T> } }\n}"
        ),
        certifying("Lane", "certified")
    );
    // The right in a `where` clause, a constructor under another name,
    // and the self type spelled out as the return.
    assert_eq!(
        doors(
            "impl<T> Lane<T>\nwhere\n    T: Decide + CertifiedBounds,\n{\n    \
             pub fn formed() -> Lane<T> { todo!() }\n}"
        ),
        certifying("Lane", "formed")
    );
    // Concrete at `f64`, under any name.
    assert_eq!(
        doors("impl Fit<f64> {\n    pub const fn fit() -> Self { todo!() }\n}"),
        vec![(
            "Fit".to_string(),
            "fit".to_string(),
            ConstructorImpl::ConcreteF64
        )]
    );
    // A door-named constructor with no bound is still a door, and one
    // no roster can agree with.
    assert_eq!(
        doors("impl<T: Decide> Lane<T> {\n    pub fn certified() -> Self { todo!() }\n}"),
        vec![(
            "Lane".to_string(),
            "certified".to_string(),
            ConstructorImpl::Unbounded
        )]
    );
    // Not doors: a method taking `self`, a constructor taking a
    // parameter, a zero-argument constructor in an unbounded impl, a
    // trait impl, and a door-shaped `fn` local to a method body.
    assert!(
        doors(
            "impl<T: CertifiedBounds> Lane<T> {\n    fn certified(self) -> Self { self }\n    \
             fn with(x: u8) -> Self { todo!() }\n}\n\
             impl<T: Decide> Plain<T> {\n    fn new() -> Self { todo!() }\n    \
             fn run(self) {\n        fn certified() -> Self { todo!() }\n    }\n}\n\
             impl<T: CertifiedBounds> Default for Lane<T> {\n    \
             fn default() -> Self { todo!() }\n}"
        )
        .is_empty()
    );
}

/// The row reader on each call shape, and the definition it sets aside.
#[test]
fn the_row_reader_reads_turbofish_and_bare_calls() {
    let code = code_only(
        "fn holds<T: X>() -> Result<(), &'static str> { Door::<T>::certified(); Ok(()) }\n\
         fn a() { holds::<f64>(); holds::<geom_core::Sym<f64>>(); holds ::< Probe >(); }\n\
         fn b() { holds(); }",
    );
    let (rows, definition) = rows("x.rs", &code, "holds");
    assert_eq!(
        rows,
        vec![
            Row::At("f64".into()),
            Row::At("Sym".into()),
            Row::At("Probe".into()),
            Row::Bare
        ]
    );
    assert!(definition.contains("Door::<T>::certified()"));
}
