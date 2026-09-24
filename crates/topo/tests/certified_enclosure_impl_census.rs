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
//! is. Every failure a run finds is listed in its one red.
//!
//! A type the door reader matches that is not a door value goes in
//! [`NOT_A_DOOR`] with its reason, instead of a roster; an entry there
//! that the reader no longer matches is a red, like a stale roster.
//!
//! **Which fields.** A door's helper pins the door only as far as it
//! compares its fields. [`every_door_helper_compares_every_fn_pointer_field`]
//! reads each rostered door's `struct` in its roster's file, takes its
//! fields whose type is a fn pointer (`fn(…)`, across lines), and
//! requires the helper's body to hand each one to a
//! `std::ptr::fn_addr_eq` call as a field access (`lane.remap`,
//! `Door::<T>::certified().open`). It does not see: whether the other
//! argument is the RIGHT target, or whether the comparison's answer
//! reaches an `Err` at all (a compared-and-discarded field counts); a
//! fn pointer behind a type alias, inside another type
//! (`Option<fn(…)>`) or in a nested struct; and a `struct` defined
//! outside the roster's file.
//!
//! **What the door reader sees, and its blind spot.** A door
//! constructor, for this reader, is a zero-parameter associated `fn`
//! returning `Self` or the self type spelled out — bare, or as the
//! first argument of an `Option<…>` or a `Result<…, _>` — at the top
//! level of an INHERENT `impl` block that either names the right in its
//! bounds (`CertifiedBounds` or `CertifiedEnclosure`, in the generic
//! list or a `where` clause, across any number of lines), or is
//! concrete at `f64` (its first type argument after any lifetimes), or
//! names the fn `certified` or `fit` — the two spellings the tree's
//! doors use. An `impl` head the shared head reader cannot parse, or
//! one that never closes, is a red rather than a skipped block. It
//! does not see: a right bound under a renamed import (`CertifiedBounds
//! as Certifying`) or reached only through a trait that implies it
//! without naming it, unless the constructor carries one of the two
//! names; a self type written as a type alias; a door made by an
//! associated `const`, a `const` item, a free function or a trait impl;
//! a door whose only constructor takes a parameter; a constructor
//! returning the door in any other wrapper; a door concrete at a scalar
//! other than `f64`; and anything outside `crates/*/src`. Finding a
//! door is keyed on its constructor, not on its other mark, the
//! fn-pointer fields: most fn-pointer structs in the tree are not
//! certification doors, so that shape is read only for a door already
//! rostered (above), and a door with neither constructor shape is found
//! by no row here.
//!
//! **Every row is read as TEXT, and `cfg` is invisible to it.** A
//! helper call under `#[cfg(feature = "…")]` counts whether or not any
//! build compiles it, so a row gated on a feature the manifest no
//! longer declares is a pin in no build that this census still counts.
//! What catches that one is the `unexpected_cfgs` lint under clippy's
//! `-D warnings`, which reds a `cfg` on a name the manifest does not
//! declare; a row under a declared feature compiles in every build
//! that turns the feature on. A `cfg` that is well-formed but never
//! true (`cfg(any())`) is caught by neither.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};

use test_utils::source::{
    ImplHead, ItemBody, angle_end, balanced_end, code_only, ident, impl_head, item_body, line,
    repo_root, rust_sources, skip_ws, top_level_split, type_base, where_at, word_at,
};

/// This file, for the messages that send an author here.
const CENSUS: &str = "crates/topo/tests/certified_enclosure_impl_census.rs";

/// What a door's wiring helper is, for the messages that ask for one.
const HELPER_SHAPE: &str = "a wiring helper is a `#[cfg(test)]` fn in the file that defines \
     the door, returning `Result<(), &'static str>`: it forms the door by its constructor, \
     compares EVERY fn-pointer field by `std::ptr::fn_addr_eq` against the function that field \
     is meant to hold, and returns `Err` naming the first field that differs, `Ok(())` \
     otherwise. One `#[test]` row calls it per scalar in the roster's formed set — \
     `helper::<Scalar>()` at each certifying scalar, or the bare `helper()` for a door formed \
     at `f64` alone. `holds_the_certified_region_doors` in crates/topo/src/chart_region.rs is \
     the model";

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

/// The types the door reader matches that are not door values, each
/// with the reason no roster is owed. An entry here is an exemption and
/// has to earn it; one the reader no longer matches is stale, and red.
const NOT_A_DOOR: [(&str, &str); 0] = [];

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

/// Every `impl` block in one file's code view, nested ones included;
/// each block this census cannot read is a failure pushed to
/// `failures`, and the walk goes on to the next.
///
/// An `impl` keyword that ends at a `;` is an `impl Trait` in a
/// signature, not a block. One whose body opens is read as a block even
/// when it is an `impl Trait` argument ahead of a function body: that
/// "block" names no trait and its top-level items are the function's
/// local items, so it can only add a door, never hide one. A head that
/// never closes, or that the shared head reader cannot parse, could be
/// an inherent `impl` holding a door, so it is a red and not a skip.
fn impl_blocks(file: &str, code: &str, failures: &mut Vec<String>) -> Vec<ImplBlock> {
    let mut out = Vec::new();
    for (at, _) in code.match_indices("impl") {
        if !word_at(code, at, "impl") {
            continue;
        }
        let body = match item_body(code, at) {
            ItemBody::Body(body) => body,
            ItemBody::Declaration(_) => continue,
            ItemBody::Unterminated => {
                failures.push(format!(
                    "{file}:{}: an `impl` head runs to end of file unterminated — a block this \
                     census cannot read is one it cannot read a door or an impl off",
                    line(code, at)
                ));
                continue;
            }
        };
        let Some(head) = impl_head(code, at, body.start) else {
            failures.push(format!(
                "{file}:{}: an `impl` head the shared head reader cannot parse (its generic \
                 list does not close before its body) — a block this census cannot read is \
                 one it cannot read a door or an impl off",
                line(code, at)
            ));
            continue;
        };
        out.push(ImplBlock { at, head, body });
    }
    out
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

/// Whether a return spelling hands back the self type whose base is
/// `self_base`: `Self` or the type spelled out, bare or as the first
/// argument of an `Option<…>` or a `Result<…, _>`.
fn returns_self(ret: &str, self_base: &str) -> bool {
    let ret = ret.trim();
    if ret == "Self" || type_base(ret) == self_base {
        return true;
    }
    if !matches!(type_base(ret), "Option" | "Result") {
        return false;
    }
    let Some(open) = ret.find('<') else {
        return false;
    };
    let Some(close) = angle_end(ret, open) else {
        return false;
    };
    let args = &ret[open + 1..close];
    let first = top_level_split(args, ',').into_iter().next();
    first.is_some_and(|first| {
        let first = args[first].trim();
        first == "Self" || type_base(first) == self_base
    })
}

/// Whether a self type is concrete at `f64`: its first generic
/// argument after any lifetimes is `f64` (`Fit<f64>`, `Fit<'a, f64>`,
/// `Fit<f64, 3>`).
fn concrete_at_f64(self_type: &str) -> bool {
    let Some(open) = self_type.find('<') else {
        return false;
    };
    let Some(close) = angle_end(self_type, open) else {
        return false;
    };
    let args = &self_type[open + 1..close];
    top_level_split(args, ',')
        .into_iter()
        .map(|r| args[r].trim())
        .find(|arg| !arg.starts_with('\''))
        .is_some_and(|arg| arg == "f64")
}

/// The name of the `fn` at `fn_at` when it is a constructor of the
/// self type whose base is `self_base`: no parameters, and a return
/// [`returns_self`] reads as that type.
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
    let ret = where_at(ret).map_or(ret, |w| &ret[..w]);
    returns_self(ret, self_base).then_some(name)
}

/// The door constructors one file's code view defines; each `impl`
/// block it cannot read is a failure pushed to `failures`.
fn doors_in(file: &str, code: &str, failures: &mut Vec<String>) -> Vec<Door> {
    let mut out = Vec::new();
    for block in impl_blocks(file, code, failures) {
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
        let concrete_f64 = concrete_at_f64(&block.head.self_type);
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
/// failure pushed to `failures` rather than a missing impl.
fn impls(sources: &[(String, String)], failures: &mut Vec<String>) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for (file, code) in sources {
        let mut seen = 0usize;
        for block in impl_blocks(file, code, failures) {
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
        if seen != written {
            failures.push(format!(
                "{file}: {written} `CertifiedEnclosure for` in code, {seen} read as impl heads — \
                 an impl this census cannot parse is one it cannot count"
            ));
        }
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

/// A roster's helper as its file writes it: every call, and the body
/// of its one definition.
struct Helper {
    rows: Vec<Row>,
    definition: String,
}

/// Every call of `helper` in `code`, and the body of its one
/// definition — or every reason it cannot be read. Any mention that is
/// neither is a failure: a row this census cannot read is one it cannot
/// count.
fn rows(file: &str, code: &str, helper: &str) -> Result<Helper, Vec<String>> {
    let mut out = Vec::new();
    let mut definitions = Vec::new();
    let mut failures = Vec::new();
    for (at, _) in code.match_indices(helper) {
        if !word_at(code, at, helper) {
            continue;
        }
        let before = code[..at].trim_end();
        if before.ends_with("fn") && word_at(before, before.len() - 2, "fn") {
            match item_body(code, at) {
                ItemBody::Body(body) => definitions.push(code[body].to_string()),
                _ => failures.push(format!(
                    "{file}:{}: `fn {helper}` has no body",
                    line(code, at)
                )),
            }
            continue;
        }
        let after = skip_ws(code, at + helper.len());
        if code[after..].starts_with("::<") {
            let open = after + "::".len();
            let Some(close) = angle_end(code, open) else {
                failures.push(format!(
                    "{file}:{}: `{helper}::<` never closes",
                    line(code, at)
                ));
                continue;
            };
            if !code[skip_ws(code, close + 1)..].starts_with("()") {
                failures.push(format!(
                    "{file}:{}: `{helper}::<…>` is not called with no arguments",
                    line(code, at)
                ));
                continue;
            }
            out.push(Row::At(type_base(&code[open + 1..close]).to_string()));
        } else if code[after..].starts_with("()") {
            out.push(Row::Bare);
        } else {
            failures.push(format!(
                "{file}:{}: `{helper}` is mentioned but neither defined nor called as a row",
                line(code, at)
            ));
        }
    }
    if definitions.len() != 1 {
        failures.push(format!(
            "{file} must define `fn {helper}` exactly once and defines it {} times; \
             {HELPER_SHAPE}",
            definitions.len()
        ));
    }
    match definitions.pop() {
        Some(definition) if failures.is_empty() => Ok(Helper {
            rows: out,
            definition,
        }),
        _ => Err(failures),
    }
}

/// The code view of a repository-relative file, or why it cannot be
/// read.
fn code_of(file: &str) -> Result<String, String> {
    let path = repo_root(env!("CARGO_MANIFEST_DIR")).join(file);
    std::fs::read_to_string(&path)
        .map(|text| code_only(&text))
        .map_err(|e| format!("{file} does not read ({e}), so nothing rostered there is checked"))
}

/// A roster's helper, read off its file — or every reason it cannot
/// be, each ending in what was skipped because of it.
fn helper_of(roster: &Roster) -> Result<(String, Helper), Vec<String>> {
    let code = code_of(roster.file).map_err(|e| vec![e])?;
    let helper = rows(roster.file, &code, roster.helper).map_err(|mut failures| {
        failures.push(format!(
            "{}'s `{}` cannot be read, so its rows and its fields are not checked",
            roster.door, roster.helper
        ));
        failures
    })?;
    Ok((code, helper))
}

/// The byte offset of the first whole-word `word` in `text` outside
/// every round, square and angle bracket.
fn top_level_word(text: &str, word: &str) -> Option<usize> {
    let mut depth = 0usize;
    for (at, c) in text.char_indices() {
        match c {
            '(' | '[' | '<' => depth += 1,
            ')' | ']' | '>' => depth = depth.saturating_sub(1),
            _ if depth == 0 && word_at(text, at, word) => return Some(at),
            _ => {}
        }
    }
    None
}

/// The names of the fn-pointer fields of `struct {door}` in `code` —
/// the fields whose type is a `fn(…)` at its own top level — or why
/// they cannot be read.
fn fn_pointer_fields(file: &str, code: &str, door: &str) -> Result<Vec<String>, String> {
    let defined: Vec<usize> = code
        .match_indices(door)
        .map(|(at, _)| at)
        .filter(|&at| {
            let before = code[..at].trim_end();
            word_at(code, at, door)
                && before.ends_with("struct")
                && word_at(before, before.len() - "struct".len(), "struct")
        })
        .collect();
    let [at] = defined[..] else {
        return Err(format!(
            "{file} must define `struct {door}` exactly once and defines it {} times — its \
             fn-pointer fields are read there",
            defined.len()
        ));
    };
    let ItemBody::Body(body) = item_body(code, at) else {
        return Err(format!(
            "{file}:{}: `struct {door}` has no braced field list — a door value's fields are \
             named, and this census reads them by name",
            line(code, at)
        ));
    };
    // An arrow's `>` is not an angle bracket; blanking it keeps the
    // offsets and the bracket depth both true.
    let inside = code[body.start + 1..body.end - 1].replace("->", "  ");
    let mut out = Vec::new();
    for field in top_level_split(&inside, ',') {
        let field = &inside[field];
        let mut at = skip_ws(field, 0);
        while field[at..].starts_with('#') {
            let open = skip_ws(field, at + 1);
            let close = balanced_end(field, open)
                .ok_or_else(|| format!("{file}: an attribute on a `{door}` field never closes"))?;
            at = skip_ws(field, close + 1);
        }
        if word_at(field, at, "pub") {
            at = skip_ws(field, at + "pub".len());
            if field[at..].starts_with('(') {
                let close = balanced_end(field, at)
                    .ok_or_else(|| format!("{file}: a `{door}` field's `pub(` never closes"))?;
                at = skip_ws(field, close + 1);
            }
        }
        let name = ident(field, at);
        if name.is_empty() {
            if field[at..].trim().is_empty() {
                continue;
            }
            return Err(format!(
                "{file}: a `{door}` field this census cannot read: `{}`",
                field.trim()
            ));
        }
        let colon = skip_ws(field, at + name.len());
        if !field[colon..].starts_with(':') {
            return Err(format!(
                "{file}: `{door}`'s field `{name}` has no `:` after its name"
            ));
        }
        if top_level_word(&field[colon + 1..], "fn").is_some() {
            out.push(name.to_string());
        }
    }
    if out.is_empty() {
        return Err(format!(
            "{file}: `struct {door}` has no field read as a fn pointer — a door value holds \
             them, so the field reader has missed their spelling"
        ));
    }
    Ok(out)
}

/// The fields a helper's body hands to `std::ptr::fn_addr_eq` as a
/// field access: every argument of every call that ends in `.name`.
fn compared_fields(definition: &str) -> BTreeSet<String> {
    let body = definition.replace("->", "  ");
    let mut out = BTreeSet::new();
    for (at, m) in body.match_indices("fn_addr_eq") {
        let open = skip_ws(&body, at + m.len());
        if !word_at(&body, at, m) || !body[open..].starts_with('(') {
            continue;
        }
        let Some(close) = balanced_end(&body, open) else {
            continue;
        };
        let args = &body[open + 1..close];
        for arg in top_level_split(args, ',') {
            if let Some((_, field)) = args[arg].trim().rsplit_once('.') {
                if !field.is_empty() && ident(field, 0) == field {
                    out.insert(field.to_string());
                }
            }
        }
    }
    out
}

#[test]
fn every_door_value_has_a_roster_and_every_roster_a_door() {
    let mut failures = Vec::new();
    let mut found: BTreeMap<String, Vec<Door>> = BTreeMap::new();
    for (file, code) in &crate_sources() {
        for door in doors_in(file, code, &mut failures) {
            found.entry(door.door.clone()).or_default().push(door);
        }
    }
    for (name, reason) in NOT_A_DOOR {
        if !found.contains_key(name) {
            failures.push(format!(
                "NOT_A_DOOR exempts {name} ({reason}) but the door reader no longer matches a \
                 constructor of that name in crates/*/src — the exemption is stale"
            ));
        }
        if ROSTERS.iter().any(|r| r.door == name) {
            failures.push(format!(
                "{name} is both rostered and exempted in NOT_A_DOOR — it is one or the other"
            ));
        }
    }
    for (name, doors) in &found {
        if NOT_A_DOOR.iter().any(|(ty, _)| ty == name) {
            continue;
        }
        let Some(roster) = ROSTERS.iter().find(|r| r.door == name) else {
            failures.push(format!(
                "{name} is a door value — {doors:?} — and has no entry in ROSTERS in {CENSUS}. \
                 Give it a wiring helper beside its constructor and an entry in ROSTERS naming \
                 that helper; {HELPER_SHAPE}. If it is not a door value, exempt it in \
                 NOT_A_DOOR in {CENSUS} with the reason instead"
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
    let mut failures = Vec::new();
    let impls = impls(&crate_sources(), &mut failures);
    let exempt: BTreeSet<String> = NOT_A_DOOR_SCALAR
        .iter()
        .map(|(ty, _)| (*ty).to_string())
        .collect();
    for ty in &exempt {
        if !impls.contains(ty) {
            failures.push(format!(
                "{ty} is exempted here as not a door scalar but implements \
                 `CertifiedEnclosure` nowhere in the tree — the exemption is stale"
            ));
        }
    }
    let door_scalars: BTreeSet<Row> = impls.difference(&exempt).cloned().map(Row::At).collect();
    for roster in &ROSTERS {
        let (_, helper) = match helper_of(roster) {
            Ok(read) => read,
            Err(mut unread) => {
                failures.append(&mut unread);
                continue;
            }
        };
        if !helper
            .definition
            .match_indices(roster.door)
            .any(|(i, _)| word_at(&helper.definition, i, roster.door))
        {
            failures.push(format!(
                "{}'s `{}` never names {} — the roster's helper is another door's",
                roster.file, roster.helper, roster.door
            ));
        }
        let rows: BTreeSet<Row> = helper.rows.into_iter().collect();
        match roster.formed {
            Formed::CertifyingScalars if rows != door_scalars => failures.push(format!(
                "{} is formed at every certifying scalar: the scalars implementing \
                 `CertifiedEnclosure` (less the exemptions {exempt:?}) are {door_scalars:?} \
                 and {}'s `{}` rows pin it at {rows:?}. An impl with no row forms the door \
                 unpinned, and a row with no impl is a stale roster: add the row in {}, or \
                 the exemption with its reason in {CENSUS}",
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

#[test]
fn every_door_helper_compares_every_fn_pointer_field() {
    let mut failures = Vec::new();
    for roster in &ROSTERS {
        let (code, helper) = match helper_of(roster) {
            Ok(read) => read,
            Err(mut unread) => {
                failures.append(&mut unread);
                continue;
            }
        };
        let fields = match fn_pointer_fields(roster.file, &code, roster.door) {
            Ok(fields) => fields,
            Err(unread) => {
                failures.push(format!(
                    "{unread}; so {}'s `{}` is not checked field by field",
                    roster.door, roster.helper
                ));
                continue;
            }
        };
        let compared = compared_fields(&helper.definition);
        let missing: Vec<&String> = fields.iter().filter(|f| !compared.contains(*f)).collect();
        if !missing.is_empty() {
            failures.push(format!(
                "{}'s fn-pointer fields are {fields:?}, and `{}` in {} hands {missing:?} to no \
                 `std::ptr::fn_addr_eq` — a field it does not compare can be re-pointed with \
                 every row green. Compare each against the function it is meant to hold and \
                 return `Err` naming it; {HELPER_SHAPE}",
                roster.door, roster.helper, roster.file
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// The door reader on each shape it claims to see, and on the shapes
/// beside them it must not count.
#[test]
fn the_door_reader_sees_each_constructor_shape() {
    let doors = |src: &str| -> Vec<(String, String, ConstructorImpl)> {
        let mut failures = Vec::new();
        let doors = doors_in("x.rs", &code_only(src), &mut failures);
        assert!(failures.is_empty(), "{failures:?}");
        doors
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
    let concrete = |door: &str, ctor: &str| {
        vec![(
            door.to_string(),
            ctor.to_string(),
            ConstructorImpl::ConcreteF64,
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
    // The door handed back inside an `Option` or a `Result`, as `Self`
    // or spelled out.
    assert_eq!(
        doors("impl<T: CertifiedBounds> Lane<T> {\n    fn try_new() -> Option<Self> { None }\n}"),
        certifying("Lane", "try_new")
    );
    assert_eq!(
        doors(
            "impl<T: CertifiedEnclosure> Lane<T> {\n    \
             fn built() -> core::result::Result<Lane<T>, Error<T>> { todo!() }\n}"
        ),
        certifying("Lane", "built")
    );
    // Concrete at `f64`, under any name, and with a lifetime or a
    // const argument beside the scalar.
    assert_eq!(
        doors("impl Fit<f64> {\n    pub const fn fit() -> Self { todo!() }\n}"),
        concrete("Fit", "fit")
    );
    assert_eq!(
        doors("impl Fit<'static, f64> {\n    pub fn derive() -> Self { todo!() }\n}"),
        concrete("Fit", "derive")
    );
    assert_eq!(
        doors("impl<'a> Fit<'a, f64, 3> {\n    pub fn derive() -> Self { todo!() }\n}"),
        concrete("Fit", "derive")
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
    // parameter, a return that is not the self type (bare, in an
    // `Option`, or in another wrapper), a zero-argument constructor in
    // an unbounded impl, a trait impl, a door-shaped `fn` local to a
    // method body, and an `impl` generic over a parameter spelled `F64`.
    assert!(
        doors(
            "impl<T: CertifiedBounds> Lane<T> {\n    fn certified(self) -> Self { self }\n    \
             fn with(x: u8) -> Self { todo!() }\n    fn count() -> Option<u8> { None }\n    \
             fn all() -> Vec<Self> { todo!() }\n}\n\
             impl<T: Decide> Plain<T> {\n    fn new() -> Self { todo!() }\n    \
             fn run(self) {\n        fn certified() -> Self { todo!() }\n    }\n}\n\
             impl<T: CertifiedBounds> Default for Lane<T> {\n    \
             fn default() -> Self { todo!() }\n}\n\
             impl<F64> Grid<F64> {\n    fn new() -> Self { todo!() }\n}"
        )
        .is_empty()
    );
}

/// A head the shared reader cannot parse, and one that never closes,
/// are each a failure the walk reports and walks past — not a skip,
/// and not an abort that hides the next.
#[test]
fn an_unreadable_impl_head_is_a_red_and_the_walk_goes_on() {
    let code = code_only(
        "impl<T: Tr<{ 1 }>> Lane<T> {\n    fn certified() -> Self { todo!() }\n}\n\
         impl<T: CertifiedBounds> Quad<T> {\n    fn certified() -> Self { todo!() }\n}\n\
         impl Broken<T> {",
    );
    let mut failures = Vec::new();
    let doors = doors_in("x.rs", &code, &mut failures);
    assert_eq!(
        doors.iter().map(|d| d.door.as_str()).collect::<Vec<_>>(),
        ["Quad"]
    );
    assert_eq!(failures.len(), 2, "{failures:?}");
    assert!(
        failures[0].starts_with("x.rs:") && failures[0].contains("cannot parse"),
        "{failures:?}"
    );
    assert!(
        failures[1].starts_with("x.rs:") && failures[1].contains("unterminated"),
        "{failures:?}"
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
    let helper = rows("x.rs", &code, "holds").unwrap();
    assert_eq!(
        helper.rows,
        vec![
            Row::At("f64".into()),
            Row::At("Sym".into()),
            Row::At("Probe".into()),
            Row::Bare
        ]
    );
    assert!(helper.definition.contains("Door::<T>::certified()"));
}

/// Every reason a helper cannot be read is listed, not just the first,
/// and a missing definition says what the helper must be.
#[test]
fn the_row_reader_lists_every_unreadable_mention() {
    let code = code_only("fn a() { holds::<f64>(1); let f = holds; }");
    let failures = rows("x.rs", &code, "holds").err().unwrap();
    assert_eq!(failures.len(), 3, "{failures:?}");
    assert!(failures[0].contains("not called with no arguments"));
    assert!(failures[1].contains("neither defined nor called"));
    assert!(failures[2].contains("exactly once") && failures[2].contains(HELPER_SHAPE));
}

/// The field reader takes the fn-pointer fields and only those, across
/// lines and past attributes and visibility; the comparison reader
/// takes the fields handed to `fn_addr_eq` and only those.
#[test]
fn the_field_readers_see_fn_pointers_and_their_comparisons() {
    let code = code_only(
        "pub struct Lane<T: Decide> {\n    /// Doc.\n    #[allow(clippy::type_complexity)]\n    \
         pub(crate) fit: fn(&Body<T>, Tol) -> Result<Fitted<T>, Error>,\n    \
         remap: fn(\n        &Body<T>,\n        Band,\n    ) -> Result<(), Error>,\n    \
         mint: unsafe fn() -> u8,\n    n: u8,\n    maybe: Option<fn() -> u8>,\n    \
         boxed: Box<dyn Fn(u8) -> u8>,\n}\n\
         struct LaneMirror { x: u8 }",
    );
    assert_eq!(
        fn_pointer_fields("x.rs", &code, "Lane").unwrap(),
        ["fit", "remap", "mint"]
    );
    assert!(
        fn_pointer_fields("x.rs", &code, "LaneMirror")
            .unwrap_err()
            .contains("no field read as a fn pointer")
    );
    assert!(
        fn_pointer_fields("x.rs", &code, "Absent")
            .unwrap_err()
            .contains("defines it 0 times")
    );
    let helper = code_only(
        "{ let lane = Lane::<T>::certified();\n  \
         if !std::ptr::fn_addr_eq(lane.fit, fit::<T> as fn(_, _) -> _) { return Err(\"fit\"); }\n  \
         if !fn_addr_eq(\n      Lane::<T>::certified().remap,\n      remap as fn(_, _) -> _,\n  ) \
         { return Err(\"remap\"); }\n  let _ = lane.mint;\n  Ok(()) }",
    );
    assert_eq!(
        compared_fields(&helper),
        BTreeSet::from(["fit".to_string(), "remap".to_string()])
    );
}
