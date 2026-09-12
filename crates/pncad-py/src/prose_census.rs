//! **The census over `Display` impls that render a payload through
//! `Debug`** — the mechanical half of the prose gate.
//!
//! [`crate::errors::reads_as_prose`] rejects the field-brace
//! fingerprint `" { "`, and [`crate::py::typed_err`] asserts it on
//! every raise, live under release. So a kernel refusal whose
//! `Display` renders a struct-shaped payload through `Debug` does not
//! degrade: **it panics the binding**, at the arm meant to refuse
//! gracefully.
//!
//! That assertion catches an instance when someone RUNS the door that
//! raises it. Nothing enumerated the doors, and three instances were
//! found that way against one fix. This module enumerates them.
//!
//! # Why this is a source census and not a roster of samples
//!
//! The obvious guard is a list of error values, rendered and checked.
//! **That guard is already in the tree and it is already blind**: a
//! `seeds()` roster over one kernel error looks exhaustive over the
//! enum and samples its one struct-shaped variant with the single
//! brace-FREE alternative, so it passes for the reason it should
//! fail. A roster that picks its own samples excludes the failing mode
//! by construction, and exhaustiveness over the enum does not help —
//! what decides the rendering is the variant of the PAYLOAD, one level
//! down.
//!
//! So this guard samples nothing. It reads the SITE: every
//! `{binding:?}` in every format string inside every `impl Display` in
//! the tree, resolved to the field type the binding is declared at,
//! and asked whether that type's `Debug` can carry the fingerprint. A
//! site is flagged for the type it renders, never for a value someone
//! thought to construct, so a variant nobody sampled is not a variant
//! this can miss.
//!
//! # The three verdicts, and why the third one is written down
//!
//! [`Verdict::Braced`] is a violation. [`Verdict::Prose`] is fine.
//! [`Verdict::Undecided`] is this census saying it could not answer.
//!
//! **Undecided is never a pass, and every arm that cannot answer must
//! reach it rather than guessing prose.** That is the single rule the
//! rest of this module is written against, because a wrong guess
//! toward prose is exactly the silence it exists to remove. It is also
//! the rule the first version broke in three places, each found by
//! executing a planted tree rather than by reading the code:
//!
//! * an or-pattern was resolved at its LAST alternative only, so
//!   `Loud { site: Braced } | Quiet { site: u32 }` cleared a
//!   brace-shaped payload as prose — a silent miss of a variant, which
//!   is the one thing this census promises not to do;
//! * the item-head scanner counted angle brackets with no bracket
//!   tracking and no arrow exception, so `Fn(u32) -> bool` in a bound
//!   drove it past the item and it answered prose over a named-field
//!   payload;
//! * every string literal in a `Display` body was read as a format
//!   string, so prose in an ordinary `let` invented rendering sites
//!   that do not exist.
//!
//! The first two are why [`Verdict::Undecided`] now also covers
//! disagreement: a name this census cannot place answers Undecided
//! unless every definition of it agrees, and an item this census
//! cannot parse answers Undecided rather than "no fields, so prose".
//!
//! # What a type's identity is here
//!
//! **A written type resolves to the path it is DECLARED at**, through
//! its module's `use` items and through re-exports ([`Types::lookup`]).
//! One bare name can mean two distinct types — `BlendError`,
//! `ComposeError`, `LiftRefusal`, `ReplayError` and `SplitError` each
//! name two in this tree — and a key that cannot tell them apart is a
//! key on which a `Display` found on either satisfies both. Reading the
//! declaring path also follows a re-export that RENAMES, which the bare
//! name cannot reach at all.
//!
//! **Failing to resolve falls back to the bare name**, which is the
//! reading this census had before, so a name it cannot place is still
//! judged over every rival at once and still answers Undecided when
//! they disagree. The fallback costs precision; picking one rival would
//! cost soundness, and that is the trade this module never makes.
//!
//! **[`UNDECIDED`] names every undecided site with its reason**, and
//! the row over it fails in both directions: a new site the resolver
//! cannot type reds until someone writes its line, and a line that no
//! longer names a site reds too. Reducing that list is scheduled as
//! `work/fix/prose-census-undecided-residue.md`, not left to the
//! reader of a comment.
//!
//! # What it cannot see
//!
//! * **A `Debug` rendering that does not pass through an
//!   `impl Display`.** The three known instances are all in `Display`
//!   impls; [`raise_sites_rendering_debug`] closes the other route for
//!   this crate's own raise sites, and only for the message
//!   expressions it can read — a message bound by anything other than
//!   a `let` in the same body is not one of them.
//! * **Reachability.** A brace-shaped payload that never reaches
//!   `typed_err` is cosmetic; one that does is a panic. This census
//!   over-approximates deliberately — it covers every `Display` in the
//!   tree rather than tracing raise paths — because a reachability
//!   analysis is the half that goes stale silently when a door is
//!   added.
//! * **`macro_rules!` bodies and `include!`d text**, which the shared
//!   lexer does not expand. A type declared inside a macro parses to
//!   an item with no fields, which is why an empty parse answers
//!   Undecided and not prose.
//! * **Type aliases.** A re-export is followed to its declaration,
//!   `as` renames included; `type X = Y;` is not an item this indexes,
//!   so a field declared at one resolves to no definition — Undecided.
//! * **A name reached only through a GLOB.** `use x::*` records no
//!   binding, so a type it brings into scope resolves to no declaring
//!   path and falls back to its bare name.
//! * **Name collisions WITHIN one file.** Two items of one name in one
//!   file — an inner `mod`, or one declared in a function body — share
//!   a declaring path, stay rivals, and answer Undecided when they
//!   disagree. Across files the declaring path separates them.
//! * **Scopes finer than the file.** A module path is derived from the
//!   file path, so an inline `mod` block's items are attributed to the
//!   file, and so are `use` items written inside one or inside a
//!   function body. Both can only widen what a name resolves to, and a
//!   name that resolves to no declaration falls back to the bare one.
//! * **A crate whose library name is not its directory name.** The
//!   crate segment of a declaring path is the directory's, with `-` as
//!   `_`; no crate in this tree overrides it, and one that did would
//!   resolve to nothing rather than to something wrong.
//! * **A generic argument's scope.** An argument is substituted
//!   textually into the declaration's field types and then read in the
//!   DECLARING module, not at the use site, so one spelled with a
//!   use-site alias resolves to nothing there.
//! * **A binding introduced by a pattern nested inside the one this
//!   census reads** — `slot: SlotId::Profile { .. }`, `verb: Some(v)`,
//!   `endpoints: (u, v)` — and a binding that is no match pattern at
//!   all, a closure parameter or a catch-all arm. Those reach no
//!   declared type, which is the largest reason [`UNDECIDED`] has.

// Per the workspace convention recorded in the root Cargo.toml: test
// code may allow the panic family, because panicking IS a test's
// failure mechanism.
#![allow(clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

// The shared Rust-source lexer AND its structural readers. This guard
// reads two views of every file — structure from the code-only one,
// the format string from the one that keeps literals — and spells no
// comment, quote, bracket or generic-list scanner of its own.
// `crates/test-utils/tests/reader_census.rs` carries the line that
// says so.
use test_utils::source::{
    angle_end, balanced_end, code_and_literals, code_only, crate_dir, rust_sources, top_level_split,
};

/// What a type's `Debug` rendering can contain.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Verdict {
    /// No `" { "` reachable: primitives, unit and tuple shapes whose
    /// elements are themselves prose.
    Prose,
    /// A named-field struct, or an enum with a struct variant, at any
    /// nesting depth — the fingerprint the prose gate rejects.
    Braced,
    /// This census could not answer. Censused in [`UNDECIDED`], never
    /// silently passed.
    Undecided,
}

/// One declaration of a type: where it was declared, its generic
/// parameters and its shape.
struct Declaration {
    /// The module path the item is declared in — `geom_brep::intersect`
    /// for `crates/geom-brep/src/intersect.rs`. With the item's name
    /// this is the DECLARING PATH, the key a written type resolves to,
    /// and it is also the scope the item's own field types are read in.
    module: String,
    /// The names of its generic parameters, so a use site's arguments
    /// can be substituted into its field types instead of resolving
    /// the parameter as if it were a type.
    params: Vec<String>,
    /// How its `Debug` renders.
    shape: Shape,
}

/// The declared shape of one type, as its `Debug` would render it.
enum Shape {
    /// `struct S { .. }` — renders `S { field: .. }` when it has any
    /// field at all. Carries `name: type` so a binding read out of a
    /// pattern can be resolved to what it was declared as.
    NamedStruct(Vec<(String, String)>),
    /// `struct S(A, B);` — renders `S(a, b)`, so its verdict is its
    /// elements'.
    TupleStruct(Vec<String>),
    /// `struct S;` — renders one bare token.
    UnitStruct,
    /// `enum E { .. }`, its variants keyed on their names.
    Enum(BTreeMap<String, VariantShape>),
    /// An item this census could not parse — a macro-declared type,
    /// or a head it could not read. **Not "no fields".**
    Unreadable,
}

/// The declared shape of one enum variant.
enum VariantShape {
    /// `V { .. }`, carrying `name: type` for the same reason
    /// [`Shape::NamedStruct`] does.
    Named(Vec<(String, String)>),
    /// `V(A, B)`.
    Tuple(Vec<String>),
    /// `V`.
    Unit,
}

/// The repository root, from this crate's own directory.
fn repo_root() -> PathBuf {
    crate_dir(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

/// Every `.rs` file this census reads: the library sources of every
/// crate in the repository.
///
/// **Derived from the walk, not from a roster of crates.** A `src`
/// component is what makes a file library source, so a crate that
/// lands tomorrow — at the workspace root or beside it, as
/// `interval-transcendentals/` already is — is covered the day it
/// arrives. `target` and hidden directories are excluded by name,
/// which is the same rule `reader_census.rs` uses and for the same
/// reason: a roster of trees to cover narrows silently on a rename.
fn scanned_files(root: &Path) -> Vec<PathBuf> {
    rust_sources(root)
        .into_iter()
        .filter(|path| {
            let rel = path.strip_prefix(root).unwrap_or(path);
            let parts: Vec<_> = rel.components().map(|c| c.as_os_str().to_owned()).collect();
            parts.iter().any(|c| c == "src")
                && !parts
                    .iter()
                    .any(|c| c == "target" || c.to_string_lossy().starts_with('.'))
        })
        .collect()
}

/// The path as this census names it: repository-relative, `/`-joined,
/// so a roster line reads the same on every machine.
fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/")
}

/// The module path a file's items are declared in.
///
/// `crates/geom-brep/src/intersect.rs` is `geom_brep::intersect`,
/// `crates/topo/src/boolean/mod.rs` is `topo::boolean`, and a crate
/// root is the crate alone.
///
/// **The crate name is the directory's, with `-` as `_`.** No crate in
/// this tree overrides its library name, and one that did would resolve
/// to no declaring path and fall back to the bare name — the reading
/// this census had before, never a wrong one.
fn module_path(file: &str) -> String {
    let parts: Vec<&str> = file.split('/').collect();
    let Some(src) = parts.iter().position(|part| *part == "src") else {
        return String::new();
    };
    let Some(krate) = src.checked_sub(1).and_then(|i| parts.get(i)) else {
        return String::new();
    };
    let mut out = krate.replace('-', "_");
    for part in &parts[src + 1..] {
        let part = part.strip_suffix(".rs").unwrap_or(part);
        if part.is_empty() || part == "mod" || part == "lib" || part == "main" {
            continue;
        }
        out.push_str("::");
        out.push_str(part);
    }
    out
}

/// The last segment of a written type path — the bare name.
fn bare_name(written: &str) -> &str {
    written.rsplit("::").next().unwrap_or(written)
}

/// One `use` tree flattened into the bindings it introduces.
///
/// **A glob introduces none.** `use x::*` names no binding this can
/// record, so a type reached only through one resolves to no declaring
/// path and is judged at its bare name.
fn expand_use(text: &str, prefix: &str, out: &mut BTreeMap<String, String>) {
    let text = text.trim();
    let join = |head: &str| -> String {
        match (prefix.is_empty(), head.is_empty()) {
            (true, _) => head.to_owned(),
            (_, true) => prefix.to_owned(),
            _ => format!("{prefix}::{head}"),
        }
    };
    if let Some(open) = text.find('{') {
        let Some(close) = balanced_end(text, open) else {
            return;
        };
        let head = text[..open].trim().trim_end_matches("::").trim();
        let joined = join(head);
        let inner = &text[open + 1..close];
        for range in top_level_split(inner, ',') {
            expand_use(&inner[range], &joined, out);
        }
        return;
    }
    if text.is_empty() || text.ends_with('*') {
        return;
    }
    let (path, alias) = match text.split_once(" as ") {
        Some((path, alias)) => (path.trim(), Some(alias.trim())),
        None => (text, None),
    };
    if alias == Some("_") {
        return;
    }
    // `a::b::{self, C}` binds `b`, which is the group's own prefix.
    let full = if path == "self" { join("") } else { join(path) };
    if full.is_empty() {
        return;
    }
    let name = alias.unwrap_or_else(|| bare_name(&full));
    if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return;
    }
    out.insert(name.to_owned(), full);
}

/// The names a file's `use` items bring into scope.
///
/// Read over the code view, and attributed to the FILE's module: a
/// `use` inside an inline `mod` block or a function body is recorded
/// against the file, which can only widen what a name resolves to, and
/// a name that resolves to no declaration falls back to the bare one.
fn use_bindings(code: &str) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for (at, _) in code.match_indices("use ") {
        if at > 0
            && code[..at]
                .chars()
                .next_back()
                .is_some_and(|c| c.is_alphanumeric() || c == '_')
        {
            continue;
        }
        let Some(end) = code[at..].find(';').map(|o| at + o) else {
            continue;
        };
        expand_use(&code[at + "use ".len()..end], "", &mut out);
    }
    out
}

/// The identifier starting at `from`, and one past its end.
fn ident_at(code: &str, from: usize) -> Option<(String, usize)> {
    let rest = code.get(from..)?;
    let start = rest.len() - rest.trim_start().len();
    let name: String = rest[start..]
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    (!name.is_empty() && !name.starts_with(|c: char| c.is_ascii_digit()))
        .then(|| (name.clone(), from + start + name.len()))
}

/// The head of an item declaration: its generic parameters, and the
/// offset where its body opens.
///
/// **The generic list is skipped with [`angle_end`] and the body is
/// found outside every round and square bracket.** Both halves are
/// load-bearing and neither is this module's to invent: a hand-rolled
/// angle counter reads the `>` of `-> bool` as a closer and runs past
/// the item, and a scan that accepts the first `(` reads
/// `where F: Fn(u32) -> bool` as a tuple body. `angle_end`'s own
/// documentation names both cases — this census imports it rather
/// than re-deriving it, because a third hand-rolled angle scanner in
/// one tree is how the second one's defects were re-minted.
///
/// Returns `None` when the head cannot be read, which the caller turns
/// into [`Shape::Unreadable`] rather than into an empty item.
fn item_head(code: &str, after_name: usize) -> Option<(Vec<String>, usize)> {
    let rest = code.get(after_name..)?;
    let lead = rest.len() - rest.trim_start().len();
    let mut at = after_name + lead;
    let mut params = Vec::new();
    if code.get(at..)?.starts_with('<') {
        let close = angle_end(code, at)?;
        let inner = &code[at + 1..close];
        params = top_level_split(inner, ',')
            .into_iter()
            .filter_map(|range| {
                let item = inner[range].trim();
                let item = item.split(':').next().unwrap_or(item).trim();
                let item = item.split_whitespace().last().unwrap_or(item);
                (!item.is_empty()
                    && !item.starts_with('\'')
                    && item.chars().all(|c| c.is_alphanumeric() || c == '_'))
                .then(|| item.to_owned())
            })
            .collect();
        at = close + 1;
    }
    let tail = code.get(at..)?;
    let lead = tail.len() - tail.trim_start().len();
    if tail[lead..].starts_with('(') {
        return Some((params, at + lead));
    }
    // Everything else: the body opens at the first `{` or `;` outside
    // every round and square bracket, so a `where` clause carrying
    // `Fn(u32)` cannot be mistaken for a tuple body.
    let mut brackets = 0i32;
    for (off, c) in code[at..].char_indices() {
        match c {
            '(' | '[' => brackets += 1,
            ')' | ']' => brackets -= 1,
            '{' | ';' if brackets == 0 => return Some((params, at + off)),
            _ => {}
        }
    }
    None
}

/// A field or element with its visibility removed — `pub`, and the
/// parenthesised `pub(crate)` / `pub(super)` / `pub(in path)` forms,
/// which a bare `trim_start_matches("pub")` leaves as a `(crate) …`
/// type nothing resolves.
fn without_visibility(item: &str) -> &str {
    let rest = item.trim();
    let Some(rest) = rest.strip_prefix("pub") else {
        return rest;
    };
    if rest.starts_with('(') {
        let end = balanced_end(rest, 0).map_or(rest.len(), |e| e + 1);
        return rest[end..].trim_start();
    }
    if rest.starts_with(char::is_whitespace) {
        return rest.trim_start();
    }
    item.trim()
}

/// `name: Type` pairs of a named-field body, over the code view.
fn named_fields(body: &str) -> Vec<(String, String)> {
    top_level_split(body, ',')
        .into_iter()
        .filter_map(|range| {
            let item = body[range].trim();
            let (name, ty) = item.split_once(':')?;
            let name = without_visibility(name);
            let name = name.split_whitespace().last().unwrap_or(name);
            let name = name.trim_start_matches("r#");
            (!name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_'))
                .then(|| (name.to_owned(), ty.trim().to_owned()))
        })
        .collect()
}

/// The element types of a tuple body, over the code view.
fn tuple_elements(body: &str) -> Vec<String> {
    top_level_split(body, ',')
        .into_iter()
        .map(|range| without_visibility(&body[range]).to_owned())
        .filter(|t| !t.is_empty())
        .collect()
}

/// The variants of an enum body, over the code view.
fn enum_variants(body: &str) -> BTreeMap<String, VariantShape> {
    let mut out = BTreeMap::new();
    for range in top_level_split(body, ',') {
        let item = &body[range];
        let Some((name, after)) = ident_at(item, 0) else {
            continue;
        };
        let rest = item[after..].trim_start();
        let at = item.len() - rest.len();
        let shape = if rest.starts_with('{') {
            let end = balanced_end(item, at).unwrap_or(item.len());
            VariantShape::Named(named_fields(&item[at + 1..end]))
        } else if rest.starts_with('(') {
            let end = balanced_end(item, at).unwrap_or(item.len());
            VariantShape::Tuple(tuple_elements(&item[at + 1..end]))
        } else {
            VariantShape::Unit
        };
        out.insert(name, shape);
    }
    out
}

/// Every `struct` and `enum` in the tree, indexed by DECLARING PATH
/// and by bare name, beside the `use` items of every module.
///
/// **The declaring path is the key; the bare name is the fallback.**
/// One bare name can mean two distinct types — `BlendError`,
/// `ComposeError`, `LiftRefusal`, `ReplayError` and `SplitError` each
/// name two in this tree — and a census keyed on it alone cannot say
/// which one a site wrote. [`Types::lookup`] resolves the written type
/// through its module's `use` items and through re-exports to the
/// module that declares it, and only when that fails does it fall back
/// to judging every rival declaration at once.
///
/// The fallback is the census's older reading, so failing to resolve
/// costs precision and never soundness: rivals that disagree still
/// answer [`Verdict::Undecided`].
struct Types {
    /// Every declaration, in the order the walk found them.
    all: Vec<Declaration>,
    /// Bare name to indices into [`Types::all`].
    by_name: BTreeMap<String, Vec<usize>>,
    /// Declaring path (`module::Name`) to indices into [`Types::all`].
    /// Two items of one name in one file — an inner `mod`, or one
    /// declared in a function body — share a path and stay rivals.
    by_path: BTreeMap<String, Vec<usize>>,
    /// Module path to the names its `use` items bring into scope. Its
    /// `pub use` entries are what a re-export is followed through.
    imports: BTreeMap<String, BTreeMap<String, String>>,
    /// Every module path the tree has a file for, so a written path's
    /// leading segment can be told from a crate name.
    modules: BTreeSet<String>,
}

impl Types {
    /// The declarations at one declaring path.
    fn at(&self, path: &str) -> Vec<&Declaration> {
        self.by_path
            .get(path)
            .into_iter()
            .flatten()
            .filter_map(|i| self.all.get(*i))
            .collect()
    }

    /// Every declaration of a bare name, rivals included.
    fn named(&self, name: &str) -> Vec<&Declaration> {
        self.by_name
            .get(name)
            .into_iter()
            .flatten()
            .filter_map(|i| self.all.get(*i))
            .collect()
    }

    /// A written type path rewritten against the module it was written
    /// in: `crate`, `self` and `super` expanded, and a leading segment
    /// that names a `use` binding replaced by what it names.
    fn normalize(&self, module: &str, written: &str) -> Option<String> {
        let segments: Vec<&str> = written.split("::").filter(|s| !s.is_empty()).collect();
        let (first, rest) = segments.split_first()?;
        if rest.is_empty() {
            let local = format!("{module}::{first}");
            if self.by_path.contains_key(&local) {
                return Some(local);
            }
            let imported = self.imports.get(module).and_then(|m| m.get(*first))?;
            if imported == written {
                return None;
            }
            return self.normalize(module, &imported.clone());
        }
        let mut rest = rest;
        let base = match *first {
            "crate" => module.split("::").next()?.to_owned(),
            "self" => module.to_owned(),
            "super" => {
                let mut parts: Vec<&str> = module.split("::").collect();
                parts.pop();
                while rest.first() == Some(&"super") {
                    parts.pop();
                    rest = &rest[1..];
                }
                parts.join("::")
            }
            // A leading segment is a `use` binding, a CHILD MODULE of
            // the writing one — the uniform-path spelling this tree
            // uses, `pub use refuse::Refusal` inside `viewer::session`
            // — or a crate, in that order. Reading a child module as a
            // crate is what sends a re-export to no declaration at all.
            other => {
                let imported = self.imports.get(module).and_then(|m| m.get(other)).cloned();
                let child = format!("{module}::{other}");
                match imported {
                    Some(path) => path,
                    None if self.modules.contains(&child) => child,
                    None => other.to_owned(),
                }
            }
        };
        if base.is_empty() || rest.is_empty() {
            return None;
        }
        let mut path = base;
        for segment in rest {
            path.push_str("::");
            path.push_str(segment);
        }
        Some(path)
    }

    /// The path a written type is DECLARED at, following re-exports.
    ///
    /// A path that names no declaration is looked up as a `use` item of
    /// its own prefix — which is what a `pub use` re-export is — and
    /// the walk repeats until it lands on a declaration. The bound is
    /// there because a `use` cycle this census cannot see is cheaper to
    /// give up on than to prove impossible.
    fn declaring_path(&self, module: &str, written: &str) -> Option<String> {
        let mut path = self.normalize(module, written)?;
        for _ in 0..8 {
            if self.by_path.contains_key(&path) {
                return Some(path);
            }
            // The re-export's own module is the scope its target path
            // was written in: `pub use inner::Braced` inside `payloads`
            // names `payloads::inner::Braced`, and reading it anywhere
            // else lands on nothing.
            let (prefix, last) = path.rsplit_once("::")?;
            let next = self.imports.get(prefix)?.get(last)?.clone();
            let resolved = self.normalize(prefix, &next)?;
            if resolved == path {
                return None;
            }
            path = resolved;
        }
        None
    }

    /// The declarations a written type names, and the identity the
    /// recursion records it under.
    ///
    /// The declaring path when it resolves, the bare name when it does
    /// not — so an unresolvable name is judged exactly as this census
    /// judged every name before: over all its rivals at once.
    fn lookup(&self, module: &str, written: &str) -> (String, Vec<&Declaration>) {
        match self.declaring_path(module, written) {
            Some(path) => {
                let declarations = self.at(&path);
                (path, declarations)
            }
            None => {
                let bare = bare_name(written);
                (bare.to_owned(), self.named(bare))
            }
        }
    }
}

/// Build the index over every source in the tree.
fn type_table(sources: &[Source]) -> Types {
    let mut types = Types {
        all: Vec::new(),
        by_name: BTreeMap::new(),
        by_path: BTreeMap::new(),
        imports: BTreeMap::new(),
        modules: sources.iter().map(|s| module_path(&s.file)).collect(),
    };
    for Source { file, code, .. } in sources {
        let module = module_path(file);
        types
            .imports
            .entry(module.clone())
            .or_default()
            .extend(use_bindings(code));
        let mut push = |name: String, declaration: Declaration| {
            let at = types.all.len();
            types
                .by_path
                .entry(format!("{}::{name}", declaration.module))
                .or_default()
                .push(at);
            types.by_name.entry(name).or_default().push(at);
            types.all.push(declaration);
        };
        for (keyword, is_enum) in [("struct ", false), ("enum ", true)] {
            for (at, _) in code.match_indices(keyword) {
                if at > 0
                    && code[..at]
                        .chars()
                        .next_back()
                        .is_some_and(|c| c.is_alphanumeric() || c == '_')
                {
                    continue;
                }
                let Some((name, after)) = ident_at(code, at + keyword.len()) else {
                    continue;
                };
                let Some((params, head)) = item_head(code, after) else {
                    push(
                        name,
                        Declaration {
                            module: module.clone(),
                            params: Vec::new(),
                            shape: Shape::Unreadable,
                        },
                    );
                    continue;
                };
                let shape = match &code[head..=head] {
                    ";" => Shape::UnitStruct,
                    "(" => match balanced_end(code, head) {
                        Some(end) => Shape::TupleStruct(tuple_elements(&code[head + 1..end])),
                        None => Shape::Unreadable,
                    },
                    _ => match balanced_end(code, head) {
                        Some(end) => {
                            let body = &code[head + 1..end];
                            if is_enum {
                                let variants = enum_variants(body);
                                // A parse that found NO variant is a
                                // parse that failed — a macro-declared
                                // enum is the live case — and "no
                                // variants" must not read as "nothing
                                // to render".
                                if variants.is_empty() {
                                    Shape::Unreadable
                                } else {
                                    Shape::Enum(variants)
                                }
                            } else {
                                Shape::NamedStruct(named_fields(body))
                            }
                        }
                        None => Shape::Unreadable,
                    },
                };
                push(
                    name,
                    Declaration {
                        module: module.clone(),
                        params,
                        shape,
                    },
                );
            }
        }
    }
    types
}

/// Types whose `Debug` is prose by definition, so the walk stops.
const PRIMITIVE: &[&str] = &[
    "bool", "char", "str", "String", "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize",
    "u8", "u16", "u32", "u64", "u128", "usize", "OsStr", "OsString", "Path", "PathBuf", "Duration",
    "Ordering",
];

/// Containers whose own `Debug` adds no field brace, so the verdict is
/// their parameters'. Maps belong here too: `{k: v}` carries no
/// SPACE-brace-space, so a map is braced exactly when its key or value
/// type is.
const TRANSPARENT: &[&str] = &[
    "Option", "Box", "Rc", "Arc", "Cow", "Vec", "VecDeque", "BTreeSet", "HashSet", "BTreeMap",
    "HashMap", "Reverse", "Wrapping",
];

/// A type expression split into its head and its generic arguments,
/// with references and lifetimes stripped.
///
/// **The head keeps the path it was written with.** `profile::Verb`
/// comes back whole, because the path is what says WHICH `Verb`; the
/// bare name is one [`bare_name`] away for the lookups that want it.
fn head_args(ty: &str) -> (String, Vec<String>) {
    let mut ty = ty.trim();
    loop {
        let stripped = ty.strip_prefix('&').unwrap_or(ty).trim_start();
        let stripped = stripped
            .strip_prefix("mut ")
            .unwrap_or(stripped)
            .trim_start();
        let stripped = if stripped.starts_with('\'') {
            stripped
                .split_once(char::is_whitespace)
                .map_or("", |(_, rest)| rest)
                .trim_start()
        } else {
            stripped
        };
        if stripped == ty {
            break;
        }
        ty = stripped;
    }
    if ty.starts_with('(') {
        let inner = balanced_end(ty, 0).map_or("", |end| &ty[1..end]);
        return ("(tuple)".to_owned(), tuple_elements(inner));
    }
    if ty.starts_with('[') {
        let inner = balanced_end(ty, 0).map_or("", |end| &ty[1..end]);
        let element = inner.split(';').next().unwrap_or(inner).trim().to_owned();
        return ("(slice)".to_owned(), vec![element]);
    }
    match ty.find('<') {
        Some(open) if ty.ends_with('>') => {
            let head = ty[..open].trim();
            let inner = &ty[open + 1..ty.len() - 1];
            let args = top_level_split(inner, ',')
                .into_iter()
                .map(|range| inner[range].trim().to_owned())
                .filter(|arg| !arg.is_empty() && !arg.starts_with('\''))
                .collect();
            (head.to_owned(), args)
        }
        _ => (ty.trim().to_owned(), Vec::new()),
    }
}

/// `ty` with each of `params` textually replaced by the matching
/// argument, so a wrapper's field types are read at the use site's
/// instantiation instead of at its parameter names.
fn substitute(ty: &str, params: &[String], args: &[String]) -> String {
    let mut out = ty.to_owned();
    for (param, arg) in params.iter().zip(args) {
        let mut rebuilt = String::with_capacity(out.len());
        let mut rest = out.as_str();
        while let Some(at) = rest.find(param.as_str()) {
            let before = rest[..at].chars().next_back();
            let after = rest[at + param.len()..].chars().next();
            let bounded = |c: Option<char>| !c.is_some_and(|c| c.is_alphanumeric() || c == '_');
            rebuilt.push_str(&rest[..at]);
            if bounded(before) && bounded(after) {
                rebuilt.push_str(arg);
            } else {
                rebuilt.push_str(param);
            }
            rest = &rest[at + param.len()..];
        }
        rebuilt.push_str(rest);
        out = rebuilt;
    }
    out
}

/// Whether the `Debug` rendering of `ty`, as written in `module`, can
/// carry the field-brace fingerprint the prose gate rejects.
///
/// `module` is the scope the type expression was WRITTEN in — the file
/// holding the `Display` impl for a site's own candidate, and the
/// declaring module of the owner for a field walked into. It is what
/// turns a bare `Verb` into the one the site means.
fn brace_shaped(types: &Types, module: &str, ty: &str, seen: &mut BTreeSet<String>) -> Verdict {
    let (head, args) = head_args(ty);
    let bare = bare_name(&head).to_owned();
    if bare.is_empty() || bare == "()" || bare == "!" {
        return Verdict::Prose;
    }
    if PRIMITIVE.contains(&bare.as_str()) {
        return Verdict::Prose;
    }
    // A cycle through a recursive type adds no NEW rendering: whatever
    // the outer level renders has already been judged. The identity a
    // cycle is measured at is the DECLARING PATH wherever the name
    // resolved to one, so two same-named types are two cycles.
    if bare == "(tuple)" || bare == "(slice)" || TRANSPARENT.contains(&bare.as_str()) {
        if !seen.insert(bare.clone()) {
            return Verdict::Prose;
        }
        let verdict = combine(args.iter().map(|a| brace_shaped(types, module, a, seen)));
        seen.remove(&bare);
        return verdict;
    }
    let (identity, declarations) = types.lookup(module, &head);
    if !seen.insert(identity.clone()) {
        return Verdict::Prose;
    }
    let verdict = if declarations.is_empty() {
        // Not a type this tree declares: an out-of-tree type, a type
        // alias, or a GENERIC PARAMETER — which in this workspace is
        // usually the `Real` scalar, whose interval instantiation wraps
        // a named-field struct.
        Verdict::Undecided
    } else {
        agree(
            declarations
                .iter()
                .map(|d| declaration_verdict(types, d, &args, seen)),
        )
    };
    seen.remove(&identity);
    verdict
}

/// The verdict of one declaration, with the use site's generic
/// arguments substituted into its field types.
///
/// **The field types are read in the DECLARING module**, which is the
/// scope they were written in. A generic argument substituted in came
/// from the use site instead, so an argument spelled with a use-site
/// alias resolves to nothing here and falls back to its bare name.
fn declaration_verdict(
    types: &Types,
    declaration: &Declaration,
    args: &[String],
    seen: &mut BTreeSet<String>,
) -> Verdict {
    let sub = |ty: &String| substitute(ty, &declaration.params, args);
    let module = declaration.module.as_str();
    match &declaration.shape {
        Shape::Unreadable => Verdict::Undecided,
        Shape::UnitStruct => Verdict::Prose,
        Shape::NamedStruct(fields) => {
            if fields.is_empty() {
                Verdict::Prose
            } else {
                Verdict::Braced
            }
        }
        Shape::TupleStruct(elements) => combine(
            elements
                .iter()
                .map(|e| brace_shaped(types, module, &sub(e), seen)),
        ),
        Shape::Enum(variants) => combine(variants.values().map(|variant| {
            match variant {
                VariantShape::Unit => Verdict::Prose,
                VariantShape::Named(fields) => {
                    if fields.is_empty() {
                        Verdict::Prose
                    } else {
                        Verdict::Braced
                    }
                }
                VariantShape::Tuple(elements) => combine(
                    elements
                        .iter()
                        .map(|e| brace_shaped(types, module, &sub(e), seen)),
                ),
            }
        })),
    }
}

/// The verdict over the alternatives WITHIN one type: any brace wins,
/// and an undecided alternative beats prose.
fn combine(verdicts: impl IntoIterator<Item = Verdict>) -> Verdict {
    let mut answer = Verdict::Prose;
    for verdict in verdicts {
        match verdict {
            Verdict::Braced => return Verdict::Braced,
            Verdict::Undecided => answer = Verdict::Undecided,
            Verdict::Prose => {}
        }
    }
    answer
}

/// The verdict over declarations that are RIVAL readings of one name.
///
/// **Disagreement is Undecided, not "braced wins".** Two `Verb` types
/// live in this tree — one macro-declared, one carrying a struct
/// variant — and a bare `Verb` resolved to their union reported a
/// brace-shaped payload at two sites whose values render brace-free.
/// A verdict produced by a collision is not a verdict about the type
/// the site names, and saying so is the difference between a defect
/// and a phantom.
fn agree(verdicts: impl IntoIterator<Item = Verdict>) -> Verdict {
    let mut answer: Option<Verdict> = None;
    for verdict in verdicts {
        match answer {
            None => answer = Some(verdict),
            Some(seen) if seen == verdict => {}
            Some(_) => return Verdict::Undecided,
        }
    }
    answer.unwrap_or(Verdict::Undecided)
}

/// One file in the two views this census reads.
struct Source {
    /// Repository-relative path.
    file: String,
    /// Comments blanked, literals KEPT — where the format string is
    /// read from.
    text: String,
    /// Comments and literals both blanked — where structure is read
    /// from, and the only view [`balanced_end`] may run over.
    code: String,
}

/// One `Debug` rendering inside a `Display` impl.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Site {
    /// Repository-relative path.
    file: String,
    /// One-based line, for the message only.
    line: usize,
    /// The type whose `Display` this is.
    display_type: String,
    /// The binding rendered, or [`POSITIONAL`].
    binding: String,
    /// What the payload type's `Debug` can carry.
    verdict: Verdict,
    /// The declared types this rendering resolved to — empty when it
    /// resolved to none, which is itself a reason. Carried so a
    /// diagnosis names the payload rather than only the binding.
    candidates: Vec<String>,
}

/// The binding name a positional `{:?}` is recorded under.
const POSITIONAL: &str = "<positional>";

/// A type expression AND the module it was written in, which is the
/// scope its bare names resolve in. Carried together because a type
/// read out of one crate's declaration and rendered in another's
/// `Display` means whatever its OWN file's `use` items say it means.
type Typed = (String, String);

/// The macros whose first string argument is a format string, and how
/// many arguments precede it.
const FORMATTING: &[(&str, usize)] = &[
    ("write!", 1),
    ("writeln!", 1),
    ("format!", 0),
    ("format_args!", 0),
];

/// Every arm pattern whose arm lexically contains `pos`.
///
/// **Every enclosing arm, not the nearest one.** A `write!` inside a
/// nested `match` is governed by the outer arm's bindings as well as
/// the inner one's, and the nearest-arm reading is what hid a live
/// instance: the two brace-shaped renderings in `blend`'s escalation
/// arm sit inside a nested match on the predicate.
fn arms_in_scope(code: &str, body: std::ops::Range<usize>, pos: usize) -> Vec<&str> {
    let mut out = Vec::new();
    for (off, _) in code[body.clone()].match_indices("=>") {
        let arrow = body.start + off;
        if arrow >= pos {
            break;
        }
        let after = arrow + 2;
        let rest = &code[after..body.end];
        let lead = rest.len() - rest.trim_start().len();
        let end = if rest[lead..].starts_with('{') {
            balanced_end(code, after + lead).unwrap_or(body.end)
        } else {
            let mut depth = 0i32;
            let mut end = body.end;
            for (o, c) in code[after..body.end].char_indices() {
                match c {
                    '(' | '[' | '{' => depth += 1,
                    ')' | ']' | '}' => {
                        if depth == 0 {
                            end = after + o;
                            break;
                        }
                        depth -= 1;
                    }
                    ',' if depth == 0 => {
                        end = after + o;
                        break;
                    }
                    _ => {}
                }
            }
            end
        };
        if pos < end {
            out.push(arm_pattern(code, arrow));
        }
    }
    out
}

/// The WHOLE pattern of the arm whose `=>` sits at `arrow`,
/// alternatives included.
///
/// Scanning back, a closing delimiter at depth zero belongs to the
/// pattern while nothing else has been seen yet (`Self::V { a } =>`);
/// after that it is the previous arm's block or expression ending —
/// **unless a `|` follows it**, in which case it closed one
/// ALTERNATIVE of this pattern and the scan continues.
///
/// That last clause is not a refinement. Without it
/// `Self::Loud { site } | Self::Quiet { site } =>` yields only its
/// last alternative, the binding is typed from one variant, and a
/// brace-shaped payload written anywhere but last is cleared as prose
/// — a silent miss of a variant, which is the one failure this census
/// promises it cannot have.
fn arm_pattern(code: &str, arrow: usize) -> &str {
    let bytes = code.as_bytes();
    let (mut depth, mut seen) = (0i32, false);
    let mut at = arrow;
    while at > 0 {
        at -= 1;
        match bytes[at] {
            b')' | b']' | b'}' => {
                if depth == 0 && seen && !followed_by_alternative(code, at + 1) {
                    at += 1;
                    break;
                }
                depth += 1;
                seen = true;
            }
            b'(' | b'[' | b'{' => {
                if depth == 0 {
                    at += 1;
                    break;
                }
                depth -= 1;
                seen = true;
            }
            b',' | b';' if depth == 0 => {
                at += 1;
                break;
            }
            c if !c.is_ascii_whitespace() => seen = true,
            _ => {}
        }
    }
    code[at..arrow].trim()
}

/// Whether the next non-space byte at or after `from` is a `|` that is
/// not part of `||`.
fn followed_by_alternative(code: &str, from: usize) -> bool {
    let rest = code[from..].trim_start();
    rest.starts_with('|') && !rest.starts_with("||")
}

/// The bindings a pattern introduces, as `binding -> declared types`.
///
/// A binding gets one entry per ALTERNATIVE that declares it, and the
/// caller judges every one of them.
fn pattern_bindings(
    pattern: &str,
    variants: &BTreeMap<String, Vec<(String, VariantShape)>>,
) -> BTreeMap<String, Vec<Typed>> {
    let mut out: BTreeMap<String, Vec<Typed>> = BTreeMap::new();
    for range in top_level_split(pattern, '|') {
        let alternative = pattern[range].trim();
        let Some(sep) = alternative.find("::") else {
            continue;
        };
        let Some((variant, after)) = ident_at(alternative, sep + 2) else {
            continue;
        };
        let rest = &alternative[after..];
        for (module, shape) in variants.get(&variant).into_iter().flatten() {
            match shape {
                VariantShape::Named(fields) => {
                    let inner = rest.trim_start().trim_start_matches('{');
                    for field_range in top_level_split(inner, ',') {
                        let item = inner[field_range]
                            .trim()
                            .trim_end_matches(['}', ' ', '\n'])
                            .to_owned();
                        let (field, bound) = item
                            .split_once(':')
                            .map_or((item.clone(), item.clone()), |(f, b)| {
                                (f.trim().to_owned(), b.trim().to_owned())
                            });
                        let field = field.trim_start_matches("r#").to_owned();
                        let bound = bound.trim_start_matches("r#").to_owned();
                        if let Some((_, ty)) = fields.iter().find(|(name, _)| *name == field) {
                            out.entry(bound)
                                .or_default()
                                .push((module.clone(), ty.clone()));
                        }
                    }
                }
                VariantShape::Tuple(elements) => {
                    if let Some(open) = rest.find('(') {
                        let end = balanced_end(rest, open).unwrap_or(rest.len());
                        let inner = &rest[open + 1..end];
                        for (index, element_range) in
                            top_level_split(inner, ',').into_iter().enumerate()
                        {
                            let name = inner[element_range].trim().trim_start_matches(['&', ' ']);
                            if let Some(ty) = elements.get(index)
                                && !name.is_empty()
                                && name.chars().all(|c| c.is_alphanumeric() || c == '_')
                            {
                                out.entry(name.trim_start_matches("r#").to_owned())
                                    .or_default()
                                    .push((module.clone(), ty.clone()));
                            }
                        }
                    }
                }
                VariantShape::Unit => {}
            }
        }
    }
    out
}

/// A `Debug` placeholder found in a format string.
struct Placeholder {
    /// Byte offset of its opening brace.
    at: usize,
    /// The argument name, empty for a positional one.
    name: String,
    /// Which positional argument it consumes, if it is one.
    index: usize,
}

/// Every `{…?}` in the literal spanning `range`.
///
/// A placeholder is a `Debug` rendering when its format spec ends in
/// `?`; `{{` is an escaped brace and consumes nothing.
fn debug_placeholders(text: &str, range: std::ops::Range<usize>) -> Vec<Placeholder> {
    let bytes = text.as_bytes();
    let (mut out, mut positional, mut at) = (Vec::new(), 0usize, range.start);
    while at < range.end {
        if bytes[at] != b'{' {
            at += 1;
            continue;
        }
        if bytes.get(at + 1) == Some(&b'{') {
            at += 2;
            continue;
        }
        let Some(close) = text[at..range.end].find('}').map(|o| at + o) else {
            break;
        };
        let body = &text[at + 1..close];
        if body.contains('{') {
            at += 1;
            continue;
        }
        let (name, spec) = body.split_once(':').unwrap_or((body, ""));
        let name = name.trim();
        let numeric = !name.is_empty() && name.chars().all(|c| c.is_ascii_digit());
        if spec.trim_end().ends_with('?') {
            out.push(Placeholder {
                at,
                name: if numeric {
                    String::new()
                } else {
                    name.to_owned()
                },
                index: if numeric {
                    name.parse().unwrap_or(positional)
                } else {
                    positional
                },
            });
        }
        if name.is_empty() {
            positional += 1;
        }
        at = close + 1;
    }
    out
}

/// Every file of the tree, in both views.
fn read_sources(root: &Path) -> Vec<Source> {
    scanned_files(root)
        .into_iter()
        .map(|path| {
            let raw = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("a readable source file {}: {e}", path.display()));
            Source {
                file: relative(root, &path),
                text: code_and_literals(&raw),
                code: code_only(&raw),
            }
        })
        .collect()
}

/// The variant shapes a `Display` impl's patterns can name: the
/// displayed type's own, and those of the enum types its named fields
/// are declared at.
///
/// **The second half is not an extra.** An error type whose `Display`
/// matches on `&self.kind` names the KIND's variants, not its own, and
/// a census that builds the map from the displayed type alone types
/// none of those bindings — one real brace-shaped rendering sat in the
/// undecided census for exactly that reason, and every error type with
/// a `kind` field shares the hole.
fn variants_in_scope(
    types: &Types,
    module: &str,
    display_type: &str,
) -> BTreeMap<String, Vec<(String, VariantShape)>> {
    let mut out: BTreeMap<String, Vec<(String, VariantShape)>> = BTreeMap::new();
    let absorb = |declarations: &[&Declaration],
                  out: &mut BTreeMap<String, Vec<(String, VariantShape)>>| {
        for declaration in declarations {
            if let Shape::Enum(variants) = &declaration.shape {
                for (variant, shape) in variants {
                    out.entry(variant.clone())
                        .or_default()
                        .push((declaration.module.clone(), clone_variant(shape)));
                }
            }
        }
    };
    let own = types.lookup(module, display_type).1;
    absorb(&own, &mut out);
    for declaration in &own {
        let fields: Vec<&(String, String)> = match &declaration.shape {
            Shape::NamedStruct(fields) => fields.iter().collect(),
            Shape::Enum(variants) => variants
                .values()
                .flat_map(|variant| match variant {
                    VariantShape::Named(fields) => fields.iter().collect::<Vec<_>>(),
                    _ => Vec::new(),
                })
                .collect(),
            _ => Vec::new(),
        };
        for (_, ty) in fields {
            let (head, _) = head_args(ty);
            if bare_name(&head) != display_type {
                let reached = types.lookup(&declaration.module, &head).1;
                absorb(&reached, &mut out);
            }
        }
    }
    out
}

/// `VariantShape` is not `Clone` by derive because its owner is built
/// once; one copy is needed to merge rival readings of a name.
fn clone_variant(variant: &VariantShape) -> VariantShape {
    match variant {
        VariantShape::Unit => VariantShape::Unit,
        VariantShape::Named(fields) => VariantShape::Named(fields.clone()),
        VariantShape::Tuple(elements) => VariantShape::Tuple(elements.clone()),
    }
}

/// Every `Debug` rendering inside every `impl Display` in the tree.
fn census(sources: &[Source]) -> Vec<Site> {
    let types = type_table(sources);
    let mut out = Vec::new();
    for source in sources {
        let (code, text) = (&source.code, &source.text);
        let module = module_path(&source.file);
        for (found, _) in code.match_indices("Display for ") {
            let window = found.saturating_sub(200);
            if !code[window..found].contains("impl") {
                continue;
            }
            let Some((display_type, after)) = ident_at(code, found + "Display for ".len()) else {
                continue;
            };
            let Some(open) = code[after..].find('{').map(|o| after + o) else {
                continue;
            };
            let Some(close) = balanced_end(code, open) else {
                continue;
            };
            let body = open + 1..close;
            let variants = variants_in_scope(&types, &module, &display_type);
            let mut fields: Vec<(String, Typed)> = Vec::new();
            for declaration in types.lookup(&module, &display_type).1 {
                if let Shape::NamedStruct(declared) = &declaration.shape {
                    fields.extend(declared.iter().map(|(name, ty)| {
                        (name.clone(), (declaration.module.clone(), ty.clone()))
                    }));
                }
            }
            for call in formatting_calls(source, body.clone()) {
                for placeholder in debug_placeholders(text, call.format.clone()) {
                    let mut scope: BTreeMap<String, Vec<Typed>> = BTreeMap::new();
                    for pattern in arms_in_scope(code, body.clone(), placeholder.at) {
                        for (binding, types) in pattern_bindings(pattern, &variants) {
                            scope.entry(binding).or_default().extend(types);
                        }
                    }
                    let mut candidates = resolve(&placeholder, &scope, &fields, &call, &types);
                    candidates.sort();
                    candidates.dedup();
                    let verdict = if candidates.is_empty() {
                        Verdict::Undecided
                    } else {
                        combine(candidates.iter().map(|(written_in, ty)| {
                            brace_shaped(&types, written_in, ty, &mut BTreeSet::new())
                        }))
                    };
                    let mut shown: Vec<String> =
                        candidates.iter().map(|(_, ty)| ty.clone()).collect();
                    shown.dedup();
                    out.push(Site {
                        file: source.file.clone(),
                        line: text[..placeholder.at].matches('\n').count() + 1,
                        display_type: display_type.clone(),
                        binding: if placeholder.name.is_empty() {
                            POSITIONAL.to_owned()
                        } else {
                            placeholder.name.clone()
                        },
                        verdict,
                        candidates: shown,
                    });
                }
            }
        }
    }
    out.sort();
    out
}

/// One formatting-macro call: its format string, and the arguments
/// that fill it.
struct FormattingCall {
    /// Byte range of the format-string literal.
    format: std::ops::Range<usize>,
    /// The arguments after it, as code text.
    arguments: Vec<String>,
    /// Its `name = expr` arguments.
    named: BTreeMap<String, String>,
}

/// Every formatting-macro call in `body` whose format string is a
/// literal.
///
/// **A `Display` body's other string literals are not format
/// strings.** Reading every literal as one invents rendering sites out
/// of ordinary prose — a `let` holding an explanatory sentence that
/// happens to spell `{binding:?}` produced two census rows for
/// renderings that do not exist — and it blames an author for a
/// rendering they did not write.
fn formatting_calls(source: &Source, body: std::ops::Range<usize>) -> Vec<FormattingCall> {
    let (code, text) = (&source.code, &source.text);
    let mut out = Vec::new();
    for (macro_name, precedes) in FORMATTING {
        for (at, _) in code[body.clone()].match_indices(macro_name) {
            let at = body.start + at;
            if at > 0
                && code[..at]
                    .chars()
                    .next_back()
                    .is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '!')
            {
                continue;
            }
            let after = at + macro_name.len();
            let rest = &code[after..body.end.max(after)];
            let lead = rest.len() - rest.trim_start().len();
            if !rest[lead..].starts_with('(') {
                continue;
            }
            let open = after + lead;
            let Some(end) = balanced_end(code, open) else {
                continue;
            };
            let inner_at = open + 1;
            let inner = &code[inner_at..end];
            let ranges = top_level_split(inner, ',');
            let Some(format_range) = ranges.get(*precedes) else {
                continue;
            };
            let absolute = inner_at + format_range.start..inner_at + format_range.end;
            let literal = text[absolute.clone()].trim_start();
            if !(literal.starts_with('"') || literal.starts_with('r')) {
                continue;
            }
            let arguments: Vec<String> = ranges[precedes + 1..]
                .iter()
                .map(|range| inner[range.clone()].trim().to_owned())
                .filter(|argument| !argument.is_empty())
                .collect();
            let mut named = BTreeMap::new();
            for argument in &arguments {
                if let Some((name, value)) = argument.split_once('=') {
                    let name = name.trim();
                    if !value.starts_with('=')
                        && !name.is_empty()
                        && name.chars().all(|c| c.is_alphanumeric() || c == '_')
                    {
                        named.insert(name.to_owned(), value.trim().to_owned());
                    }
                }
            }
            out.push(FormattingCall {
                format: absolute,
                arguments,
                named,
            });
        }
    }
    out
}

/// The declared types a placeholder can be rendering. Empty means this
/// census could not say.
fn resolve(
    placeholder: &Placeholder,
    scope: &BTreeMap<String, Vec<Typed>>,
    fields: &[(String, Typed)],
    call: &FormattingCall,
    types: &Types,
) -> Vec<Typed> {
    let lookup = |name: &str| -> Vec<Typed> {
        let mut out = scope.get(name).cloned().unwrap_or_default();
        out.extend(
            fields
                .iter()
                .filter(|(field, _)| field == name)
                .map(|(_, typed)| typed.clone()),
        );
        out
    };
    if !placeholder.name.is_empty() {
        let direct = lookup(&placeholder.name);
        if !direct.is_empty() {
            return direct;
        }
        // A named argument declared in the call's own list, which is
        // how a field spelled with a raw identifier (`r#loop`) reaches
        // a format string at all.
        return call
            .named
            .get(&placeholder.name)
            .map(|aliased| expression_type(aliased, &lookup, types))
            .unwrap_or_default();
    }
    call.arguments
        .iter()
        .filter(|a| !a.contains('=') || a.starts_with("=="))
        .nth(placeholder.index)
        .map(|argument| expression_type(argument, &lookup, types))
        .unwrap_or_default()
}

/// The declared types of an argument EXPRESSION, for the two shapes a
/// census can honestly type: a binding, and a tuple-struct field of
/// one. Anything else is undecided and says so.
fn expression_type(
    expression: &str,
    lookup: &impl Fn(&str) -> Vec<Typed>,
    types: &Types,
) -> Vec<Typed> {
    let expression = expression.trim().trim_start_matches(['&', '*', ' ']).trim();
    let expression = expression.trim_start_matches("r#");
    if !expression.is_empty() && expression.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return lookup(expression);
    }
    let Some((base, field)) = expression.split_once('.') else {
        return Vec::new();
    };
    if base == "self" {
        return field.split('.').next().map(&lookup).unwrap_or_default();
    }
    let Ok(index) = field.parse::<usize>() else {
        return Vec::new();
    };
    lookup(base.trim_start_matches("r#"))
        .iter()
        .flat_map(|(written_in, base)| {
            let (head, args) = head_args(base);
            types
                .lookup(written_in, &head)
                .1
                .into_iter()
                .filter_map(|declaration| match &declaration.shape {
                    Shape::TupleStruct(elements) => elements.get(index).map(|e| {
                        (
                            declaration.module.clone(),
                            substitute(e, &declaration.params, &args),
                        )
                    }),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

/// A raise site whose message is composed with a `Debug` rendering
/// rather than taken from a kernel `Display`.
///
/// The census above reads `impl Display` bodies, which is where all
/// three known instances were minted — but it is not the only route
/// into a typed refusal's message. This crate's own raise sites are
/// the other one, and inside one crate the question is local enough to
/// answer: the message argument of a [`crate::py::typed_err`] call
/// either carries a `{…?}` placeholder or it does not.
///
/// **The message is followed through a local `let`.** Two doors in
/// this crate already pass `typed_err` a bound name rather than a
/// literal, so a guard that reads only the argument expression is
/// green over the shape it exists to catch.
fn raise_sites_rendering_debug(sources: &[Source]) -> Vec<RaiseSite> {
    let mut out = Vec::new();
    for source in sources {
        if !source.file.starts_with("crates/pncad-py/src/") {
            continue;
        }
        let (code, text) = (&source.code, &source.text);
        for (found, _) in code.match_indices("typed_err(") {
            let open = found + "typed_err".len();
            let Some(close) = balanced_end(code, open) else {
                continue;
            };
            let inner_at = open + 1;
            let inner = &code[inner_at..close];
            // `typed_err(py, class, message, fields)`: the message is
            // the third argument, and the arity is the function's own.
            let ranges = top_level_split(inner, ',');
            let Some(message_range) = ranges.get(2) else {
                continue;
            };
            let absolute = inner_at + message_range.start..inner_at + message_range.end;
            let expression = text[absolute.clone()].trim().to_owned();
            let mut spans = vec![absolute.clone()];
            if expression.chars().all(|c| c.is_alphanumeric() || c == '_')
                && !expression.is_empty()
                && let Some(local) = local_binding_span(source, &expression, absolute.start)
            {
                spans.push(local);
            }
            let renders_debug = spans.iter().any(|span| {
                formatting_calls(source, span.clone())
                    .iter()
                    .any(|call| !debug_placeholders(text, call.format.clone()).is_empty())
                    || !debug_placeholders(text, span.clone()).is_empty()
            });
            if !renders_debug {
                continue;
            }
            out.push(RaiseSite {
                file: source.file.clone(),
                line: text[..absolute.start].matches('\n').count() + 1,
                message: expression.split_whitespace().collect::<Vec<_>>().join(" "),
            });
        }
    }
    out.sort();
    out.dedup();
    out
}

/// One `typed_err` call whose message is built from a `Debug`
/// rendering.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct RaiseSite {
    /// Repository-relative path.
    file: String,
    /// One-based line, for the message only.
    line: usize,
    /// The message expression, whitespace collapsed — the half of the
    /// key that makes an allowance name a SITE. Keyed on the file
    /// alone, an allowance is a permanently open door: a second
    /// `Debug` raise appended to an allowed file inherits its
    /// exemption without anyone deciding that.
    message: String,
}

impl RaiseSite {
    /// The key [`KNOWN_DEBUG_RAISES`] is written in.
    fn key(&self) -> (String, String) {
        (self.file.clone(), self.message.clone())
    }
}

/// The span of the initializer of the nearest `let <name> = …;` before
/// `before`, if there is one.
fn local_binding_span(
    source: &Source,
    name: &str,
    before: usize,
) -> Option<std::ops::Range<usize>> {
    let code = &source.code;
    let needle = format!("let {name}");
    let at = code[..before].rfind(&needle)?;
    let after = at + needle.len();
    if code[after..]
        .chars()
        .next()
        .is_some_and(|c| c.is_alphanumeric() || c == '_')
    {
        return None;
    }
    let equals = at + code[at..before].find('=')?;
    let mut depth = 0i32;
    for (off, c) in code[equals..before].char_indices() {
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            ';' if depth == 0 => return Some(equals..equals + off),
            _ => {}
        }
    }
    None
}

/// The sites whose payload type is brace-shaped and which this census
/// does not fail on today.
///
/// **Every entry is a known-live or undischarged DEFECT, not accepted
/// behaviour.** None is on this unit's ground; each is filed, or
/// reported for filing, where its owner can take it. The list exists
/// so the guard can land ahead of the repairs rather than behind them
/// — the point of the guard is the site that is not on it yet.
///
/// The count is part of the key. Without it a Display impl rendering
/// one binding at several sites collapses to one row, and repairing
/// some of them leaves the guard green over the rest.
const KNOWN_BRACED: &[(&str, &str, &str, usize, &str)] = &[
    (
        "crates/editor-core/src/edit.rs",
        "EditError",
        "path",
        1,
        "`ExprPath` is a named-field struct; DOCM's, filed at\
         work/docm/debug-in-prose-residue-after-finding-sink.md. \
         This one BLOCKS A DOOR rather than only degrading a message: \
         `path_off_tree` is what an expression-path edit refuses with, \
         so binding that edit would panic the gate exactly where the \
         door is supposed to refuse (work/lib/\
         the-expression-path-edit-cannot-refuse-as-prose.md)",
    ),
    (
        "crates/editor-core/src/edit.rs",
        "EditError",
        "slot",
        6,
        "`SlotId::Profile` is a struct variant; DOCM's, filed at\
         work/docm/debug-in-prose-residue-after-finding-sink.md",
    ),
    (
        "crates/editor-core/src/eval/mod.rs",
        "NodeErrorKind",
        "slot",
        2,
        "`SlotId::Profile` is a struct variant; DOCM's, filed at\
         work/docm/debug-in-prose-residue-after-finding-sink.md",
    ),
    (
        "crates/editor-core/src/persist/check.rs",
        "ProgramFault",
        "slot",
        1,
        "`SlotId::Profile` is a struct variant; DOCM's, filed at\
         work/docm/debug-in-prose-residue-after-finding-sink.md",
    ),
    (
        "crates/editor-core/src/program.rs",
        "ProgramRefusal",
        "slot",
        1,
        "`SlotId::Profile` is a struct variant; DOCM's, filed at\
         work/docm/debug-in-prose-residue-after-finding-sink.md",
    ),
    (
        "crates/editor-core/src/resolve/mod.rs",
        "Diagnosis",
        "param",
        1,
        "`SlotId::Profile` is a struct variant; DOCM's, filed at\
         work/docm/debug-in-prose-residue-after-finding-sink.md",
    ),
    (
        "crates/step-import/src/error.rs",
        "StepImportError",
        "source",
        2,
        "a live panic on a public door — EXCH's, filed at\
         work/issues/debug-in-prose-at-blend-and-step-import.md",
    ),
    (
        "crates/topo/src/boolean/voids.rs",
        "VoidInsertError",
        "e",
        1,
        "`RevertError` carries struct variants. Found BY this census;\
         reachability into `typed_err` not traced, so severity is undecided and\
         the site is disclosed rather than claimed",
    ),
];

/// The blind spot, written down WITH ITS REASON.
///
/// A site here is one this census could not decide. It is not a pass:
/// the row over this list fails when a site arrives that is not on it
/// and when a line no longer names a site, so the population cannot
/// grow in silence — which is the only property a blind spot can
/// honestly be given.
///
/// **Reducing it is scheduled**, at
/// `work/fix/prose-census-undecided-residue.md`. A disclosed list with
/// no schedule is a record of work done rather than an open thread.
const UNDECIDED: &[(&str, &str, &str, usize, &str)] = &[
    (
        "crates/editor-core/src/names/emit.rs",
        "NamingError",
        POSITIONAL,
        1,
        "a positional `{:?}` over an expression this census does not type",
    ),
    (
        "crates/editor-core/src/persist/check.rs",
        "ProgramFault",
        "arg",
        1,
        "the binding is introduced by a pattern NESTED inside the field pattern\
         this census reads — `slot: SlotId::Profile { .., arg }` — so no declared\
         type reaches it",
    ),
    (
        "crates/editor-core/src/persist/check.rs",
        "ProgramFault",
        "verb",
        1,
        "`profile::Verb` resolves to its declaration in a `macro_rules!` body,\
         which the shared lexer does not expand, so its shape is unreadable",
    ),
    (
        "crates/geom-brep/src/nurbs_iso.rs",
        "IsoRowError",
        "u",
        1,
        "the `Real` scalar parameter: `Interval` wraps a named-field struct, so\
         this renders a brace in an interval build and prose in a default one",
    ),
    (
        "crates/geom-brep/src/offset.rs",
        "OffsetError",
        "realized",
        1,
        "the `Real` scalar parameter: `Interval` wraps a named-field struct, so\
         this renders a brace in an interval build and prose in a default one",
    ),
    (
        "crates/geom-brep/src/offset.rs",
        "OffsetError",
        "realized_minor",
        1,
        "the `Real` scalar parameter: `Interval` wraps a named-field struct, so\
         this renders a brace in an interval build and prose in a default one",
    ),
    (
        "crates/geom-brep/src/offset_fit.rs",
        "OffsetFitError",
        POSITIONAL,
        2,
        "a positional `{:?}` over an expression this census does not type",
    ),
    (
        "crates/profile/src/path/program.rs",
        "ReplayError",
        "verb",
        1,
        "the binding is introduced by a pattern NESTED inside the field pattern\
         this census reads — `verb: Some(verb)` — so no declared type reaches it",
    ),
    (
        "crates/step-import/src/error.rs",
        "StepImportError",
        "e",
        1,
        "the binding is a closure parameter — `errors.iter().map(|e| ..)` — not a\
         match binding, and this census types patterns and fields",
    ),
    (
        "crates/sweep/src/blend/mod.rs",
        "BlendError",
        "other",
        1,
        "a catch-all arm binds the name, and a pattern naming no variant path\
         declares no field type to read it at",
    ),
    (
        "crates/topo/src/boolean/mod.rs",
        "BooleanError",
        POSITIONAL,
        8,
        "a positional `{:?}` over an expression this census does not type",
    ),
    (
        "crates/topo/src/flush.rs",
        "FlushRefusal",
        POSITIONAL,
        2,
        "a positional `{:?}` over an expression this census does not type",
    ),
    (
        "crates/topo/src/replace_face.rs",
        "ReplaceFaceError",
        "e",
        1,
        "an inner arm — `match edge { Some(e) => .. }` — whose pattern names no\
         variant path, so this census reads no field type from it",
    ),
    (
        "crates/topo/src/replace_face.rs",
        "ReplaceFaceError",
        "gap",
        3,
        "the `Real` scalar parameter: `Interval` wraps a named-field struct, so\
         this renders a brace in an interval build and prose in a default one",
    ),
    (
        "crates/topo/src/replace_face.rs",
        "ReplaceFaceError",
        "shift",
        1,
        "the `Real` scalar parameter: `Interval` wraps a named-field struct, so\
         this renders a brace in an interval build and prose in a default one",
    ),
    (
        "crates/topo/src/replace_face.rs",
        "ReplaceFaceError",
        "station_max",
        1,
        "the `Real` scalar parameter: `Interval` wraps a named-field struct, so\
         this renders a brace in an interval build and prose in a default one",
    ),
    (
        "crates/topo/src/replace_face.rs",
        "ReplaceFaceError",
        "station_min",
        1,
        "the `Real` scalar parameter: `Interval` wraps a named-field struct, so\
         this renders a brace in an interval build and prose in a default one",
    ),
    (
        "crates/topo/src/replace_face.rs",
        "ReplaceFaceError",
        "v_max",
        1,
        "the `Real` scalar parameter: `Interval` wraps a named-field struct, so\
         this renders a brace in an interval build and prose in a default one",
    ),
    (
        "crates/topo/src/replace_face.rs",
        "ReplaceFaceError",
        "v_min",
        1,
        "the `Real` scalar parameter: `Interval` wraps a named-field struct, so\
         this renders a brace in an interval build and prose in a default one",
    ),
    (
        "crates/topo/src/shell.rs",
        "ShellError",
        "gap",
        1,
        "the `Real` scalar parameter: `Interval` wraps a named-field struct, so\
         this renders a brace in an interval build and prose in a default one",
    ),
    (
        "crates/topo/src/shell.rs",
        "ShellError",
        "needed",
        1,
        "the `Real` scalar parameter: `Interval` wraps a named-field struct, so\
         this renders a brace in an interval build and prose in a default one",
    ),
    (
        "crates/topo/src/shell.rs",
        "ShellError",
        "thickness",
        1,
        "the `Real` scalar parameter: `Interval` wraps a named-field struct, so\
         this renders a brace in an interval build and prose in a default one",
    ),
    (
        "crates/topo/src/splitting/mod.rs",
        "SplitReduceError",
        "u",
        1,
        "the binding is introduced by a pattern NESTED inside the field pattern\
         this census reads — `endpoints: (u, v)` — so no declared type reaches it",
    ),
    (
        "crates/topo/src/splitting/mod.rs",
        "SplitReduceError",
        "v",
        1,
        "the binding is introduced by a pattern NESTED inside the field pattern\
         this census reads — `endpoints: (u, v)` — so no declared type reaches it",
    ),
    (
        "crates/topo/src/validate.rs",
        "CensusContact",
        POSITIONAL,
        2,
        "a positional `{:?}` over an expression this census does not type",
    ),
    (
        "crates/topo/src/validate.rs",
        "ValidationError",
        POSITIONAL,
        2,
        "a positional `{:?}` over an expression this census does not type",
    ),
    (
        "crates/viewer/src/frame.rs",
        "Disagreement",
        POSITIONAL,
        1,
        "a positional `{:?}` over an expression this census does not type",
    ),
];

/// The raise sites that compose a `Debug` rendering deliberately.
///
/// One arm, and `crates/pncad-py/src/errors.rs` already writes its
/// warning in prose: a future STRUCT variant of the kernel enum it
/// renders would trip the assertion and panic where that arm means to
/// refuse gracefully. This row is that warning made mechanical.
const KNOWN_DEBUG_RAISES: &[(&str, &str, &str)] = &[(
    "crates/pncad-py/src/py/flush.rs",
    "format!(\"a contact class this binding predates: {other:?}\")",
    "an unknown `ContactClass` has nothing but `Debug` to render it; the arm \
     and `errors.rs` both say so, and a struct variant of that enum would \
     panic here",
)];

#[cfg(test)]
mod tests {
    use super::{
        Declaration, KNOWN_BRACED, KNOWN_DEBUG_RAISES, POSITIONAL, Shape, Site, Source, UNDECIDED,
        Verdict, census, code_and_literals, code_only, raise_sites_rendering_debug, read_sources,
        repo_root, type_table,
    };
    use crate::errors::reads_as_prose;
    use std::collections::{BTreeMap, BTreeSet};

    /// A planted tree, so a planted case runs the real census rather
    /// than a second implementation of it.
    ///
    /// More than one file, because the defects around a type's IDENTITY
    /// only exist across files: two crates declaring one name, and a
    /// re-export that renames one.
    fn planted_tree(files: &[(&str, &str)]) -> Vec<Source> {
        files
            .iter()
            .map(|(file, text)| Source {
                file: (*file).to_owned(),
                text: code_and_literals(text),
                code: code_only(text),
            })
            .collect()
    }

    /// The one-file case.
    fn planted(text: &str) -> Vec<Source> {
        planted_tree(&[("crates/planted/src/lib.rs", text)])
    }

    /// Counts per `(file, display type, binding)`, the shape both tree
    /// rosters are written in.
    fn tally(sites: &[Site], want: Verdict) -> BTreeMap<(String, String, String), usize> {
        let mut out = BTreeMap::new();
        for site in sites.iter().filter(|s| s.verdict == want) {
            *out.entry((
                site.file.clone(),
                site.display_type.clone(),
                site.binding.clone(),
            ))
            .or_insert(0usize) += 1;
        }
        out
    }

    /// A roster as the same shape.
    fn roster(
        rows: &[(&str, &str, &str, usize, &str)],
    ) -> BTreeMap<(String, String, String), usize> {
        rows.iter()
            .map(|(file, ty, binding, count, _)| {
                (
                    ((*file).to_owned(), (*ty).to_owned(), (*binding).to_owned()),
                    *count,
                )
            })
            .collect()
    }

    /// The shape a value-sampling roster passes on: a payload enum
    /// with one struct variant among brace-free ones.
    const SAMPLER_BLIND_SPOT: &str = r#"
pub enum Site { Link { edge: u32 }, Joint { vertex: u32 }, Chain }
pub enum PlantedError { Escalated { site: Site } }
impl fmt::Display for PlantedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Escalated { site } => write!(f, "escalated at {site:?}"),
        }
    }
}
"#;

    /// The same door with the payload named by its own `Display`.
    const REPAIRED: &str = r#"
pub enum Site { Link { edge: u32 }, Joint { vertex: u32 }, Chain }
pub enum PlantedError { Escalated { site: Site } }
impl fmt::Display for PlantedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Escalated { site } => write!(f, "escalated at {site}"),
        }
    }
}
"#;

    /// The blend shape: the rendering sits inside a NESTED match.
    const NESTED_MATCH: &str = r#"
pub enum Site { Link { edge: u32 }, Chain }
pub enum PlantedError { Escalated { site: Site, predicate: u32 } }
impl fmt::Display for PlantedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Escalated { site, predicate } => {
                match predicate {
                    0 => write!(f, "escalated at {site:?}: no recourse"),
                    other => write!(f, "escalated at {site:?}: {other}"),
                }
            }
        }
    }
}
"#;

    /// An or-pattern binds one name from two variants, and the
    /// brace-shaped one is written FIRST.
    const OR_PATTERN_BRACED_FIRST: &str = r#"
pub struct Braced { a: u32 }
pub enum PlantedError { Loud { site: Braced }, Quiet { site: u32 } }
impl fmt::Display for PlantedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Loud { site } | Self::Quiet { site } => write!(f, "at {site:?}"),
        }
    }
}
"#;

    /// `->` in a generic bound: a hand-rolled angle counter reads its
    /// `>` as a closer and runs past the item.
    const ARROW_IN_BOUND: &str = r#"
pub struct Inner { a: u32 }
pub enum Carrier<F: Fn(u32) -> bool> { One { cause: Inner, f: F } }
pub enum PlantedError { Bad { c: Carrier<fn(u32) -> bool> } }
impl fmt::Display for PlantedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bad { c } => write!(f, "at {c:?}"),
        }
    }
}
"#;

    /// A `where` clause with a paren before the body: a scan that
    /// accepts the first `(` reads the item as a tuple struct.
    const WHERE_FN_HEAD: &str = r#"
pub struct Inner { a: u32 }
pub struct Guarded<F>
where
    F: Fn(u32) -> bool,
{
    pub cause: Inner,
    pub predicate: F,
}
pub enum PlantedError { Bad { g: Guarded<fn(u32) -> bool> } }
impl fmt::Display for PlantedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bad { g } => write!(f, "at {g:?}"),
        }
    }
}
"#;

    /// A string literal in a `Display` body that is not a format
    /// string.
    const NON_FORMAT_LITERAL: &str = r#"
pub struct Braced { a: u32 }
pub enum PlantedError { Bad { p: Braced, label: String } }
impl fmt::Display for PlantedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bad { p, label } => {
                let _hint = "the old spelling was {label:?} and {p:?}; do not restore it";
                write!(f, "at {}", p.a)
            }
        }
    }
}
"#;

    /// A `Display` that matches on a `kind` FIELD, so its patterns
    /// name the kind's variants and not its own.
    const KIND_FIELD: &str = r#"
pub struct Braced { a: u32 }
pub enum Kind { Bad { p: Braced }, Fine }
pub struct PlantedError { kind: Kind }
impl fmt::Display for PlantedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            Kind::Bad { p } => write!(f, "at {p:?}"),
            Kind::Fine => write!(f, "fine"),
        }
    }
}
"#;

    /// A generic wrapper whose argument is brace-shaped, with a unit
    /// struct in the tree named exactly like the parameter.
    const GENERIC_WRAPPER_COLLIDING: &str = r#"
struct V;
pub struct Braced { a: u32 }
pub struct Holder<V>(V);
pub enum PlantedError { Bad { h: Holder<Braced> } }
impl fmt::Display for PlantedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bad { h } => write!(f, "at {h:?}"),
        }
    }
}
"#;

    /// Two rival declarations of one name, one brace-shaped.
    const COLLIDING_NAMES: &str = r#"
pub enum Verb { Fillet { edges: u32 } }
pub enum PlantedError { Bad { v: Other } }
impl fmt::Display for PlantedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bad { v } => write!(f, "at {v:?}"),
        }
    }
}
"#;

    #[test]
    fn a_struct_shaped_payload_is_flagged_however_a_sampler_would_choose() {
        let flagged: Vec<_> = census(&planted(SAMPLER_BLIND_SPOT))
            .into_iter()
            .filter(|site| site.verdict == Verdict::Braced)
            .collect();
        assert_eq!(
            flagged.len(),
            1,
            "the census reads the SITE's declared type, so which variant a roster \
             would have sampled cannot change the answer: {flagged:?}"
        );
        assert_eq!(flagged[0].binding, "site");
    }

    #[test]
    fn naming_the_payload_through_its_display_clears_the_site() {
        assert!(
            census(&planted(REPAIRED))
                .iter()
                .all(|site| site.verdict != Verdict::Braced),
            "rendering the payload through `Display` is the repair"
        );
    }

    #[test]
    fn a_rendering_inside_a_nested_match_is_still_resolved() {
        let sites = census(&planted(NESTED_MATCH));
        assert_eq!(
            sites
                .iter()
                .filter(|site| site.verdict == Verdict::Braced)
                .count(),
            2,
            "both renderings are governed by the OUTER arm's binding: {sites:?}"
        );
    }

    #[test]
    fn an_or_pattern_is_judged_at_every_alternative_not_only_the_last() {
        assert!(
            census(&planted(OR_PATTERN_BRACED_FIRST))
                .iter()
                .any(|site| site.verdict == Verdict::Braced),
            "`Loud {{ site: Braced }} | Quiet {{ site: u32 }}` binds one name from \
             two declared types; taking only the last alternative clears a \
             brace-shaped payload as prose"
        );
    }

    #[test]
    fn an_item_head_this_census_cannot_read_is_never_answered_prose() {
        for (label, text) in [
            ("-> in a bound", ARROW_IN_BOUND),
            ("where Fn()", WHERE_FN_HEAD),
        ] {
            let sites = census(&planted(text));
            assert!(
                !sites.is_empty() && sites.iter().all(|site| site.verdict != Verdict::Prose),
                "{label}: the head parser ran past the item and answered PROSE for a \
                 payload that reaches a named-field struct. Undecided is the honest \
                 answer; prose is the one silence this module exists to remove: \
                 {sites:?}"
            );
        }
    }

    #[test]
    fn a_literal_that_is_not_a_format_string_is_not_a_rendering_site() {
        assert!(
            census(&planted(NON_FORMAT_LITERAL)).is_empty(),
            "the only rendering here is `{{}}` over `p.a`; censusing every literal \
             in the body invents rendering sites out of ordinary prose"
        );
    }

    #[test]
    fn a_display_that_matches_on_a_kind_field_is_still_resolved() {
        let sites = census(&planted(KIND_FIELD));
        assert!(
            sites.iter().any(|site| site.verdict == Verdict::Braced),
            "the arms name the KIND's variants, not the displayed type's; building \
             the variant map from the displayed type alone types none of them, and \
             every error type with a `kind` field shares that hole: {sites:?}"
        );
    }

    #[test]
    fn a_generic_argument_is_substituted_and_a_colliding_name_never_answers_prose() {
        let sites = census(&planted(GENERIC_WRAPPER_COLLIDING));
        assert!(
            !sites.is_empty() && sites.iter().all(|site| site.verdict != Verdict::Prose),
            "`Holder<Braced>` renders a named-field struct; a unit `struct V;` \
             declared elsewhere in the tree must not turn the parameter into prose: \
             {sites:?}"
        );
    }

    #[test]
    fn rival_readings_of_one_name_answer_undecided_rather_than_either() {
        let sites = census(&planted(COLLIDING_NAMES));
        assert!(
            sites.iter().all(|site| site.verdict == Verdict::Undecided),
            "`Other` is not declared here at all, so the honest answer is undecided"
        );
        let verb = type_table(&planted(COLLIDING_NAMES)).named("Verb").len();
        assert_eq!(verb, 1, "one declaration of `Verb` in this fixture");
    }

    #[test]
    fn an_unreadable_item_is_undecided_and_not_an_empty_one() {
        let table = type_table(&planted(
            "macro_rules! m { () => { pub enum Hidden { A } }; }",
        ));
        let shapes = table.named("Hidden");
        assert!(!shapes.is_empty(), "the macro body's enum is indexed");
        assert!(
            shapes.iter().all(|Declaration { shape, .. }| matches!(
                shape,
                Shape::Enum(_) | Shape::Unreadable
            )),
            "a macro-declared item either parses or is unreadable — never an item \
             with no fields, which would read as prose"
        );
    }

    #[test]
    fn the_shape_this_census_hunts_really_trips_the_prose_gate() {
        #[derive(Debug)]
        enum Site {
            Link {
                #[allow(dead_code, reason = "read by the derived `Debug`, the subject here")]
                edge: u32,
            },
        }
        let rendered = format!("escalated at {:?}", Site::Link { edge: 7 });
        assert!(
            !reads_as_prose(&rendered),
            "if this passes, the census is hunting a shape the gate does not \
             reject and one of the two is wrong: {rendered}"
        );
        assert!(reads_as_prose("escalated at the link on edge 7"));
    }

    #[test]
    fn no_display_impl_renders_a_brace_shaped_payload_through_debug() {
        let found = tally(&census(&read_sources(&repo_root())), Verdict::Braced);
        let allowed = roster(KNOWN_BRACED);
        let new: Vec<_> = found
            .iter()
            .filter(|(k, _)| !allowed.contains_key(*k))
            .collect();
        assert!(
            new.is_empty(),
            "a `Display` impl renders a struct-shaped payload through `Debug`, so \
             the refusal PANICS the binding at `typed_err` instead of raising: \
             {new:#?}\nRender the payload through its own `Display`, or give it one."
        );
        assert_eq!(
            found, allowed,
            "KNOWN_BRACED must name exactly the brace-shaped sites, with their \
             counts: an entry that no longer names one is struck in the PR that \
             repaired it, and a partial repair moves the count"
        );
    }

    #[test]
    fn every_site_this_census_cannot_decide_is_named_with_its_reason() {
        let found = tally(&census(&read_sources(&repo_root())), Verdict::Undecided);
        assert_eq!(
            found,
            roster(UNDECIDED),
            "the blind spot is a CENSUS, not a silence: a site this cannot decide \
             either gets its line here, with the reason it could not be decided, or \
             gets rewritten so it can be. Reducing the list is scheduled at \
             work/fix/prose-census-undecided-residue.md"
        );
    }

    #[test]
    fn no_raise_site_composes_a_debug_rendering_into_its_message() {
        let found = raise_sites_rendering_debug(&read_sources(&repo_root()));
        let keys: BTreeSet<_> = found.iter().map(super::RaiseSite::key).collect();
        let allowed: BTreeSet<_> = KNOWN_DEBUG_RAISES
            .iter()
            .map(|(file, message, _)| ((*file).to_owned(), (*message).to_owned()))
            .collect();
        assert_eq!(
            keys, allowed,
            "a raise site builds its human message out of a `Debug` rendering, \
             which is the other route to the panic the census above guards. The \
             allowance is keyed on the SITE, not the file: a second `Debug` raise \
             in an allowed file inherits nothing."
        );
    }

    /// One name, declared in two crates, with opposite shapes.
    const RIVAL_BRACED: &str = "pub enum Verb { Fillet { edges: u32 } }\n";
    const RIVAL_PROSE: &str = "pub enum Verb { Fillet }\n";

    /// A site rendering a `Verb` payload, importing the one it means.
    fn rival_site(import: &str) -> String {
        let head = if import.is_empty() {
            String::new()
        } else {
            format!("use {import};\n")
        };
        format!(
            r#"
{head}pub enum PlantedError {{ Bad {{ verb: Verb }} }}
impl fmt::Display for PlantedError {{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{
        match self {{
            Self::Bad {{ verb }} => write!(f, "at {{verb:?}}"),
        }}
    }}
}}
"#
        )
    }

    fn rival_tree(site: &str) -> Vec<Source> {
        planted_tree(&[
            ("crates/braced/src/lib.rs", RIVAL_BRACED),
            ("crates/prose/src/lib.rs", RIVAL_PROSE),
            ("crates/site/src/lib.rs", site),
        ])
    }

    #[test]
    fn the_declaration_a_site_imports_decides_a_name_two_crates_declare() {
        for (import, want) in [
            ("braced::Verb", Verdict::Braced),
            ("prose::Verb", Verdict::Prose),
        ] {
            let sites = census(&rival_tree(&rival_site(import)));
            assert_eq!(sites.len(), 1, "one rendering in this tree: {sites:?}");
            assert_eq!(
                sites[0].verdict, want,
                "the site imports `{import}`, so that is the declaration its \
                 payload renders. Keyed on the BARE name the two rival `Verb`s \
                 are one entry, and a census that cannot tell them apart is the \
                 census whose `Display` found on either satisfied both: {sites:?}"
            );
        }
    }

    #[test]
    fn a_name_no_import_places_is_still_judged_over_every_rival() {
        let sites = census(&rival_tree(&rival_site("")));
        assert_eq!(sites.len(), 1, "one rendering in this tree: {sites:?}");
        assert_eq!(
            sites[0].verdict,
            Verdict::Undecided,
            "no `use` places this `Verb`, so resolution fails and the reading \
             falls back to every rival at once — which disagree. Falling back \
             costs precision; picking one of them would cost soundness: {sites:?}"
        );
    }

    #[test]
    fn a_payload_renamed_by_a_re_export_reaches_its_declaration() {
        let sites = census(&planted_tree(&[
            (
                "crates/payloads/src/inner.rs",
                "pub struct Braced { a: u32 }\n",
            ),
            (
                "crates/payloads/src/lib.rs",
                "pub mod inner;\npub use inner::Braced as Payload;\n",
            ),
            (
                "crates/site/src/lib.rs",
                r#"
use payloads::Payload;
pub enum PlantedError { Bad { p: Payload } }
impl fmt::Display for PlantedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bad { p } => write!(f, "at {p:?}"),
        }
    }
}
"#,
            ),
        ]));
        assert_eq!(sites.len(), 1, "one rendering in this tree: {sites:?}");
        assert_eq!(
            sites[0].verdict,
            Verdict::Braced,
            "`Payload` is a rename of a named-field struct. A census that reads \
             the written name and stops declares a payload it cannot see, and \
             answering undecided over a brace-shaped one leaves the panic in \
             place: {sites:?}"
        );
    }

    #[test]
    fn the_census_reads_a_tree_and_not_an_empty_one() {
        let sources = read_sources(&repo_root());
        assert!(
            sources.len() > 200,
            "a census over a tree it failed to find passes by seeing nothing: {} files",
            sources.len()
        );
        let sites = census(&sources);
        assert!(
            sites.len() > 300,
            "the same vacuity one level in: {} sites",
            sites.len()
        );
        assert!(
            sites.iter().any(|site| site.binding == POSITIONAL),
            "positional renderings exist in this tree; finding none means the \
             placeholder scan stopped reading"
        );
    }
}
