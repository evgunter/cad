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
//! `{binding:?}` inside every `impl Display` in the tree, resolved to
//! the field type the binding is declared at, and asked whether that
//! type's `Debug` can carry the fingerprint. A site is flagged for the
//! type it renders, never for a value someone thought to construct, so
//! a variant nobody sampled is not a variant this can miss.
//!
//! # The three verdicts, and why the third one is written down
//!
//! [`Verdict::Braced`] is a violation. [`Verdict::Prose`] is fine.
//! [`Verdict::Undecided`] is the resolver saying it could not type the
//! argument — a positional `{:?}` over an expression, a generic scalar
//! parameter, a type declared outside this tree.
//!
//! **An undecided site is censused, not skipped.** [`UNDECIDED`] names
//! every one, and the test over it fails in both directions: a new
//! site the resolver cannot type reds until someone writes its line,
//! and a line that no longer resolves to a site reds too. That is the
//! whole treatment of the blind spot — it is a stated population that
//! cannot grow silently, rather than a number in a comment.
//!
//! # What it cannot see
//!
//! * **A `Debug` rendering that does not pass through an
//!   `impl Display`** — a raise site composing `format!("{err:?}")`
//!   directly. The three known instances are all in `Display` impls;
//!   this scope is where the fingerprint has been minted, not a proof
//!   that it is the only place it can be.
//! * **Reachability.** A brace-shaped payload that never reaches
//!   `typed_err` is cosmetic; one that does is a panic. This census
//!   over-approximates deliberately — it covers every `Display` in the
//!   tree rather than tracing raise paths — because a reachability
//!   analysis is the half that goes stale silently when a door is
//!   added.
//! * **`macro_rules!` bodies and `include!`d text**, which the shared
//!   lexer does not expand.
//! * **Cross-crate name collisions.** The type table is keyed on the
//!   bare type name; two definitions that disagree resolve to
//!   [`Verdict::Braced`] if either is, and to [`Verdict::Undecided`]
//!   otherwise, so a collision cannot answer "prose" by accident.

// Per the workspace convention recorded in the root Cargo.toml: test
// code may allow the panic family, because panicking IS a test's
// failure mechanism.
#![allow(clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

// The shared Rust-source lexer. This guard reads two views of every
// file — structure from the code-only one, the format string itself
// from the one that keeps literals — and spells no comment or quote
// delimiter of its own. `crates/test-utils/tests/reader_census.rs`
// carries the line that says so.
use test_utils::source::{
    balanced_end, code_and_literals, code_only, crate_dir, rust_sources, top_level_split,
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
    /// The resolver could not type the argument. Censused in
    /// [`UNDECIDED`], never silently passed.
    Undecided,
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

/// One past the head of an item declaration: the first `{`, `(` or `;`
/// that is not inside the generic parameter list.
fn head_end(code: &str, from: usize) -> Option<usize> {
    let mut angle = 0i32;
    for (off, c) in code[from..].char_indices() {
        match c {
            '<' => angle += 1,
            '>' => angle -= 1,
            '{' | '(' | ';' if angle == 0 => return Some(from + off),
            _ => {}
        }
    }
    None
}

/// The identifier starting at `from`, if any.
fn ident_at(code: &str, from: usize) -> Option<(String, usize)> {
    let rest = &code[from..];
    let start = rest.len() - rest.trim_start().len();
    let name: String = rest[start..]
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    (!name.is_empty() && !name.starts_with(|c: char| c.is_ascii_digit()))
        .then(|| (name.clone(), from + start + name.len()))
}

/// `name: Type` pairs of a named-field body, over the code view.
fn named_fields(body: &str) -> Vec<(String, String)> {
    top_level_split(body, ',')
        .into_iter()
        .filter_map(|range| {
            let item = body[range].trim();
            let (name, ty) = item.split_once(':')?;
            let name = name.trim().trim_start_matches("pub").trim();
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
        .map(|range| {
            body[range]
                .trim()
                .trim_start_matches("pub")
                .trim()
                .to_owned()
        })
        .filter(|t| !t.is_empty())
        .collect()
}

/// The variants of an enum body, over the code view.
fn enum_variants(body: &str) -> BTreeMap<String, VariantShape> {
    let mut out = BTreeMap::new();
    for range in top_level_split(body, ',') {
        let item = &body[range.clone()];
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

/// Every `struct` and `enum` in the tree, keyed on its bare name.
///
/// A name with more than one definition keeps all of them, so
/// [`brace_shaped`] can refuse to answer "prose" on a collision it
/// cannot disambiguate.
fn type_table(sources: &[Source]) -> BTreeMap<String, Vec<Shape>> {
    let mut table: BTreeMap<String, Vec<Shape>> = BTreeMap::new();
    for Source { code, .. } in sources {
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
                let Some(head) = head_end(code, after) else {
                    continue;
                };
                let shape = match &code[head..=head] {
                    ";" => Shape::UnitStruct,
                    "(" => {
                        let end = balanced_end(code, head).unwrap_or(code.len());
                        Shape::TupleStruct(tuple_elements(&code[head + 1..end]))
                    }
                    _ => {
                        let end = balanced_end(code, head).unwrap_or(code.len());
                        let body = &code[head + 1..end];
                        if is_enum {
                            Shape::Enum(enum_variants(body))
                        } else {
                            Shape::NamedStruct(named_fields(body))
                        }
                    }
                };
                table.entry(name).or_default().push(shape);
            }
        }
    }
    table
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
/// with references, lifetimes and module paths stripped.
fn head_args(ty: &str) -> (String, Vec<String>) {
    let mut ty = ty.trim();
    loop {
        let stripped = ty
            .strip_prefix('&')
            .unwrap_or(ty)
            .trim_start()
            .strip_prefix("mut ")
            .unwrap_or_else(|| ty.strip_prefix('&').unwrap_or(ty).trim_start())
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
            let head = ty[..open].rsplit("::").next().unwrap_or(&ty[..open]).trim();
            let inner = &ty[open + 1..ty.len() - 1];
            let args = top_level_split(inner, ',')
                .into_iter()
                .map(|range| inner[range].trim().to_owned())
                .filter(|arg| !arg.is_empty() && !arg.starts_with('\''))
                .collect();
            (head.to_owned(), args)
        }
        _ => (
            ty.rsplit("::").next().unwrap_or(ty).trim().to_owned(),
            Vec::new(),
        ),
    }
}

/// Whether the `Debug` rendering of `ty` can carry the field-brace
/// fingerprint the prose gate rejects.
///
/// **`Undecided` is not `Prose`.** Every arm that cannot answer says
/// so, and the caller censuses it; the one thing this function never
/// does is guess prose, because a wrong guess in that direction is
/// exactly the silence this module exists to remove.
fn brace_shaped(
    table: &BTreeMap<String, Vec<Shape>>,
    ty: &str,
    seen: &mut BTreeSet<String>,
) -> Verdict {
    let (head, args) = head_args(ty);
    if head.is_empty() || head == "()" || head == "!" {
        return Verdict::Prose;
    }
    // A cycle through a recursive type adds no NEW rendering: whatever
    // the outer level renders has already been judged.
    if !seen.insert(head.clone()) {
        return Verdict::Prose;
    }
    let verdict = if PRIMITIVE.contains(&head.as_str()) {
        Verdict::Prose
    } else if head == "(tuple)" || head == "(slice)" || TRANSPARENT.contains(&head.as_str()) {
        combine(args.iter().map(|a| brace_shaped(table, a, seen)))
    } else {
        match table.get(&head) {
            Some(shapes) => combine(shapes.iter().map(|shape| shape_verdict(table, shape, seen))),
            // Not a type this tree declares. Two populations reach
            // here and both are honestly undecided: a type from
            // outside the workspace, and a GENERIC PARAMETER — which
            // in this workspace is the `Real` scalar, whose interval
            // instantiation wraps a named-field struct. Whether such
            // a site renders a brace therefore depends on the lane
            // the binary was built for, and a source census cannot
            // see the lane.
            None => Verdict::Undecided,
        }
    };
    seen.remove(&head);
    verdict
}

/// The verdict of one declared shape.
fn shape_verdict(
    table: &BTreeMap<String, Vec<Shape>>,
    shape: &Shape,
    seen: &mut BTreeSet<String>,
) -> Verdict {
    match shape {
        Shape::UnitStruct => Verdict::Prose,
        Shape::NamedStruct(fields) => {
            if fields.is_empty() {
                Verdict::Prose
            } else {
                Verdict::Braced
            }
        }
        Shape::TupleStruct(elements) => {
            combine(elements.iter().map(|e| brace_shaped(table, e, seen)))
        }
        Shape::Enum(variants) => combine(variants.values().map(|variant| match variant {
            VariantShape::Unit => Verdict::Prose,
            VariantShape::Named(fields) => {
                if fields.is_empty() {
                    Verdict::Prose
                } else {
                    Verdict::Braced
                }
            }
            VariantShape::Tuple(elements) => {
                combine(elements.iter().map(|e| brace_shaped(table, e, seen)))
            }
        })),
    }
}

/// The verdict over a set of alternatives: any brace wins, and an
/// undecided alternative beats prose — a payload this cannot rule out
/// is not one it may pass.
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

/// One `Debug` rendering inside a `Display` impl.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Site {
    /// Repository-relative path.
    file: String,
    /// One-based line, for the message only — the rosters are keyed
    /// without it, so an edit above a site does not churn them.
    line: usize,
    /// The type whose `Display` this is.
    display_type: String,
    /// The binding rendered, or [`POSITIONAL`].
    binding: String,
    /// What the payload type's `Debug` can carry.
    verdict: Verdict,
}

/// The binding name a positional `{:?}` is recorded under.
const POSITIONAL: &str = "<positional>";

impl Site {
    /// The key both rosters are written in: everything but the line.
    fn key(&self) -> (String, String, String) {
        (
            self.file.clone(),
            self.display_type.clone(),
            self.binding.clone(),
        )
    }
}

/// The pattern of the match arm whose `=>` sits at `arrow`.
///
/// Scanning back, a closing delimiter at depth zero belongs to the
/// PATTERN only while nothing else has been seen yet (`Self::V { a }
/// =>`); after that it is the previous arm's block or expression
/// ending, and the pattern starts there. Getting this wrong is not
/// academic: reading the previous arm's `}` as an opener walks the
/// scan off the front of the match, which is how a first attempt at
/// this census silently failed to resolve every arm that follows a
/// block-bodied one.
fn arm_pattern(code: &str, arrow: usize) -> &str {
    let bytes = code.as_bytes();
    let (mut depth, mut seen) = (0i32, false);
    let mut at = arrow;
    while at > 0 {
        at -= 1;
        match bytes[at] {
            b')' | b']' | b'}' => {
                if depth == 0 && seen {
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

/// Every arm pattern whose arm lexically contains `pos`.
///
/// **Every enclosing arm, not the nearest one.** A `write!` inside a
/// nested `match` is governed by the outer arm's bindings as well as
/// the inner one's, and the nearest-arm reading is what hid a live
/// instance: the two brace-shaped renderings in `blend`'s escalation
/// arm sit inside a nested match on the predicate, so a resolver that
/// stops at the inner arm types neither of them.
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
            balanced_end(code, after + lead).map_or(body.end, |e| e)
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

/// The bindings a pattern introduces, as `binding -> declared type`.
fn pattern_bindings(
    pattern: &str,
    variants: &BTreeMap<String, VariantShape>,
) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    let Some(sep) = pattern.find("::") else {
        return out;
    };
    let Some((variant, after)) = ident_at(pattern, sep + 2) else {
        return out;
    };
    let rest = &pattern[after..];
    match variants.get(&variant) {
        Some(VariantShape::Named(fields)) => {
            // `{ field }` and `{ field: alias }` both bind; the
            // declared type is the field's either way.
            for range in top_level_split(rest.trim_start_matches(['{', ' ', '\n']), ',') {
                let item = rest.trim_start_matches(['{', ' ', '\n'])[range]
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
                    out.insert(bound, ty.clone());
                }
            }
        }
        Some(VariantShape::Tuple(elements)) => {
            if let Some(open) = rest.find('(') {
                let end = balanced_end(rest, open).unwrap_or(rest.len());
                let inner = &rest[open + 1..end];
                for (index, range) in top_level_split(inner, ',').into_iter().enumerate() {
                    let name = inner[range].trim().trim_start_matches(['&', ' ']).trim();
                    if let Some(ty) = elements.get(index)
                        && !name.is_empty()
                        && name.chars().all(|c| c.is_alphanumeric() || c == '_')
                    {
                        out.insert(name.trim_start_matches("r#").to_owned(), ty.clone());
                    }
                }
            }
        }
        _ => {}
    }
    out
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

impl Source {
    /// Whether byte `at` lies inside a string literal.
    ///
    /// The two views are blanked byte for byte and differ on exactly
    /// the literal regions, so a byte that is blank in the code view
    /// and not blank in the other is inside one. No quote is spelled
    /// here: the shared lexer already decided, and this reads its
    /// answer off the difference.
    fn in_literal(&self, at: usize) -> bool {
        let (code, text) = (self.code.as_bytes(), self.text.as_bytes());
        code.get(at) != text.get(at)
    }

    /// The end of the literal containing `at`: the first byte after it
    /// that the code view keeps.
    ///
    /// **A NEWLINE is blanked text too.** The lexer preserves line
    /// structure, so a blanked region carries its newlines; a scan
    /// that stops at the first one stops inside every literal
    /// `rustfmt` has wrapped — which is most format strings in this
    /// tree, and which silently hid the two live `step-import`
    /// renderings from a first version of this census.
    fn literal_end(&self, at: usize, limit: usize) -> usize {
        let code = self.code.as_bytes();
        let mut end = at;
        while end < limit && (code[end] == b' ' || code[end] == b'\n') {
            end += 1;
        }
        end
    }

    /// The arguments of the macro call whose format string ends at
    /// `after`, as code text.
    fn call_arguments(&self, after: usize, limit: usize) -> Vec<String> {
        let code = &self.code;
        if !code[after..].starts_with(',') {
            return Vec::new();
        }
        let mut depth = 0i32;
        let mut end = limit;
        for (off, c) in code[after..limit].char_indices() {
            match c {
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' => {
                    if depth == 0 {
                        end = after + off;
                        break;
                    }
                    depth -= 1;
                }
                _ => {}
            }
        }
        let inner = &code[after + 1..end];
        top_level_split(inner, ',')
            .into_iter()
            .map(|range| inner[range].trim().to_owned())
            .filter(|arg| !arg.is_empty())
            .collect()
    }
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
        let consumes = name.is_empty();
        if spec.trim_end().ends_with('?') {
            let index = name.parse::<usize>().unwrap_or(positional);
            out.push(Placeholder {
                at,
                name: if name.chars().all(|c| c.is_ascii_digit()) {
                    String::new()
                } else {
                    name.to_owned()
                },
                index,
            });
        }
        if consumes {
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

/// Every `Debug` rendering inside every `impl Display` in the tree,
/// with the verdict of the type it renders.
fn census(sources: &[Source]) -> Vec<Site> {
    let table = type_table(sources);
    let mut out = Vec::new();
    for source in sources {
        let (code, text) = (&source.code, &source.text);
        for (found, _) in code.match_indices("Display for ") {
            // The head of the `impl` is what makes this a `Display`
            // implementation rather than a bound; a trait bound has no
            // `for`, so the look-back only has to exclude prose, and
            // prose is already blanked.
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
            let mut variants = BTreeMap::new();
            let mut fields: Vec<(String, String)> = Vec::new();
            for shape in table.get(&display_type).into_iter().flatten() {
                match shape {
                    Shape::Enum(declared) => {
                        for (name, variant) in declared {
                            variants.insert(name.clone(), clone_variant(variant));
                        }
                    }
                    Shape::NamedStruct(declared) => fields.extend(declared.iter().cloned()),
                    _ => {}
                }
            }
            let mut at = body.start;
            while at < body.end {
                if !(text.as_bytes()[at] == b'"' && source.in_literal(at)) {
                    at += 1;
                    continue;
                }
                let literal_end = source.literal_end(at, body.end);
                let arguments = source.call_arguments(literal_end, body.end);
                let mut named_arguments: BTreeMap<String, String> = BTreeMap::new();
                for argument in &arguments {
                    if let Some((name, value)) = argument.split_once('=') {
                        let name = name.trim();
                        if !value.starts_with('=')
                            && !name.is_empty()
                            && name.chars().all(|c| c.is_alphanumeric() || c == '_')
                        {
                            named_arguments.insert(name.to_owned(), value.trim().to_owned());
                        }
                    }
                }
                for placeholder in debug_placeholders(text, at..literal_end) {
                    let mut scope: BTreeMap<String, String> = BTreeMap::new();
                    for pattern in arms_in_scope(code, body.clone(), placeholder.at) {
                        scope.extend(pattern_bindings(pattern, &variants));
                    }
                    let declared = resolve(
                        &placeholder,
                        &scope,
                        &fields,
                        &named_arguments,
                        &arguments,
                        &table,
                    );
                    let verdict = declared.map_or(Verdict::Undecided, |ty| {
                        brace_shaped(&table, &ty, &mut BTreeSet::new())
                    });
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
                    });
                }
                at = literal_end;
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

/// `VariantShape` is not `Clone` by derive because its owner is built
/// once; one copy is needed to merge the shapes of a name with more
/// than one definition.
fn clone_variant(variant: &VariantShape) -> VariantShape {
    match variant {
        VariantShape::Unit => VariantShape::Unit,
        VariantShape::Named(fields) => VariantShape::Named(fields.clone()),
        VariantShape::Tuple(elements) => VariantShape::Tuple(elements.clone()),
    }
}

/// The declared type of what a placeholder renders, or `None` when the
/// resolver cannot say.
fn resolve(
    placeholder: &Placeholder,
    scope: &BTreeMap<String, String>,
    fields: &[(String, String)],
    named_arguments: &BTreeMap<String, String>,
    arguments: &[String],
    table: &BTreeMap<String, Vec<Shape>>,
) -> Option<String> {
    let lookup = |name: &str| -> Option<String> {
        scope.get(name).cloned().or_else(|| {
            fields
                .iter()
                .find(|(field, _)| field == name)
                .map(|(_, ty)| ty.clone())
        })
    };
    if !placeholder.name.is_empty() {
        // An inline argument names a binding in scope — or a NAMED
        // argument declared in the call's own list, which is how a
        // field spelled with a raw identifier (`r#loop`) reaches a
        // format string at all.
        return lookup(&placeholder.name).or_else(|| {
            let aliased = named_arguments.get(&placeholder.name)?;
            expression_type(aliased, &lookup, table)
        });
    }
    let argument = arguments
        .iter()
        .filter(|a| !a.contains('=') || a.starts_with("=="))
        .nth(placeholder.index)?;
    expression_type(argument, &lookup, table)
}

/// The declared type of an argument EXPRESSION, for the two shapes a
/// census can honestly type: a binding, and a tuple-struct field of
/// one. Anything else is undecided and says so.
fn expression_type(
    expression: &str,
    lookup: &impl Fn(&str) -> Option<String>,
    table: &BTreeMap<String, Vec<Shape>>,
) -> Option<String> {
    let expression = expression.trim().trim_start_matches(['&', '*', ' ']).trim();
    let expression = expression.trim_start_matches("r#");
    if expression.chars().all(|c| c.is_alphanumeric() || c == '_') && !expression.is_empty() {
        return lookup(expression);
    }
    let (base, field) = expression.split_once('.')?;
    let base = if base == "self" {
        return lookup(field.split('.').next()?);
    } else {
        lookup(base.trim_start_matches("r#"))?
    };
    let index: usize = field.parse().ok()?;
    let (head, _) = head_args(&base);
    table.get(&head)?.iter().find_map(|shape| match shape {
        Shape::TupleStruct(elements) => elements.get(index).cloned(),
        _ => None,
    })
}

/// The sites whose payload type is brace-shaped and which this census
/// does not fail on today.
///
/// **Every entry is a known-live or undischarged DEFECT, not accepted
/// behaviour.** None of them is on this unit's ground; each is filed,
/// or reported for filing, where its owner can take it. The list
/// exists so the guard can land ahead of the repairs rather than
/// behind them — the point of the guard is the site that is not on it
/// yet.
///
/// It cannot rot in either direction: an entry that no longer names a
/// brace-shaped site fails
/// [`no_display_impl_renders_a_brace_shaped_payload_through_debug`]
/// exactly as a new site does, so a repair lands with its line struck
/// in the same PR.
const KNOWN_BRACED: &[(&str, &str, &str, &str)] = &[
    (
        "crates/editor-core/src/edit.rs",
        "EditError",
        "path",
        "ExprPath is a named-field struct; work/docm/debug-in-prose-residue-after-finding-sink.md",
    ),
    (
        "crates/editor-core/src/edit.rs",
        "EditError",
        "slot",
        "SlotId::Profile is a struct variant; work/docm/debug-in-prose-residue-after-finding-sink.md",
    ),
    (
        "crates/editor-core/src/eval/mod.rs",
        "NodeErrorKind",
        "slot",
        "SlotId::Profile is a struct variant; work/docm/debug-in-prose-residue-after-finding-sink.md",
    ),
    (
        "crates/editor-core/src/persist/check.rs",
        "ProgramFault",
        "slot",
        "SlotId::Profile is a struct variant; work/docm/debug-in-prose-residue-after-finding-sink.md",
    ),
    (
        "crates/editor-core/src/persist/check.rs",
        "ProgramFault",
        "verb",
        "profile::path::Verb carries struct variants — the residue work/fix/plan.md names \
         under error-types-with-no-display-class",
    ),
    (
        "crates/editor-core/src/program.rs",
        "ProgramRefusal",
        "slot",
        "SlotId::Profile is a struct variant; work/docm/debug-in-prose-residue-after-finding-sink.md",
    ),
    (
        "crates/editor-core/src/resolve/mod.rs",
        "Diagnosis",
        "param",
        "SlotId::Profile is a struct variant; work/docm/debug-in-prose-residue-after-finding-sink.md",
    ),
    (
        "crates/geom-brep/src/offset_fit.rs",
        "OffsetFitError",
        "e",
        "SplineError's variants are struct-shaped — found BY this census, reported for filing",
    ),
    (
        "crates/step-import/src/error.rs",
        "StepImportError",
        "source",
        "a live panic on a public door; work/issues/debug-in-prose-at-blend-and-step-import.md",
    ),
    (
        "crates/sweep/src/blend/mod.rs",
        "BlendError",
        "site",
        "BlendSite::Link and ::Joint are struct variants — a live panic; \
         work/issues/debug-in-prose-at-blend-and-step-import.md",
    ),
    (
        "crates/topo/src/boolean/voids.rs",
        "VoidInsertError",
        "e",
        "RevertError carries struct variants — found BY this census, reported for filing",
    ),
    (
        "crates/viewer/src/sketch.rs",
        "PreviewError",
        "verb",
        "profile::path::Verb carries struct variants — the residue work/fix/plan.md names \
         under error-types-with-no-display-class",
    ),
];

/// The blind spot, written down.
///
/// A site here is one the resolver could not type — a positional
/// `{:?}` over an expression it does not read, or a binding whose
/// declared type is a generic parameter or comes from outside this
/// tree. **It is not a pass.** The row over this list fails when a
/// site arrives that is not on it and when a line no longer names a
/// site, so the population cannot grow in silence, which is the only
/// property a blind spot can honestly be given.
///
/// The two populations are told apart by the entry itself:
/// [`POSITIONAL`] is an expression the resolver does not read; a named
/// binding is a declared type it cannot decide, which in this
/// workspace is nearly always the `Real` scalar parameter. That second
/// class is not cosmetic — the interval instantiation of `Real` wraps
/// a NAMED-FIELD struct, so those renderings carry the fingerprint in
/// an interval build and not in a default one.
const UNDECIDED: &[(&str, &str, &str)] = &[
    (
        "crates/editor-core/src/names/emit.rs",
        "NamingError",
        POSITIONAL,
    ),
    (
        "crates/editor-core/src/persist/check.rs",
        "ProgramFault",
        "arg",
    ),
    ("crates/geom-brep/src/certify.rs", "CertifyError", "check"),
    ("crates/geom-brep/src/nurbs_iso.rs", "IsoRowError", "u"),
    ("crates/geom-brep/src/offset.rs", "OffsetError", "realized"),
    (
        "crates/geom-brep/src/offset.rs",
        "OffsetError",
        "realized_minor",
    ),
    (
        "crates/geom-brep/src/offset_fit.rs",
        "OffsetFitError",
        POSITIONAL,
    ),
    ("crates/profile/src/path/program.rs", "ReplayError", "state"),
    ("crates/profile/src/path/program.rs", "ReplayError", "verb"),
    ("crates/step-import/src/error.rs", "StepImportError", "e"),
    ("crates/sweep/src/blend/mod.rs", "BlendError", "other"),
    ("crates/topo/src/boolean/mod.rs", "BooleanError", POSITIONAL),
    ("crates/topo/src/flush.rs", "FlushRefusal", POSITIONAL),
    ("crates/topo/src/replace_face.rs", "ReplaceFaceError", "e"),
    ("crates/topo/src/replace_face.rs", "ReplaceFaceError", "gap"),
    (
        "crates/topo/src/replace_face.rs",
        "ReplaceFaceError",
        "shift",
    ),
    (
        "crates/topo/src/replace_face.rs",
        "ReplaceFaceError",
        "v_max",
    ),
    (
        "crates/topo/src/replace_face.rs",
        "ReplaceFaceError",
        "v_min",
    ),
    ("crates/topo/src/shell.rs", "ShellError", "gap"),
    ("crates/topo/src/shell.rs", "ShellError", "needed"),
    ("crates/topo/src/shell.rs", "ShellError", "thickness"),
    ("crates/topo/src/splitting/mod.rs", "SplitReduceError", "u"),
    ("crates/topo/src/splitting/mod.rs", "SplitReduceError", "v"),
    ("crates/topo/src/validate.rs", "CensusContact", POSITIONAL),
    ("crates/topo/src/validate.rs", "ValidationError", POSITIONAL),
    ("crates/verbs/src/run.rs", "VerbError", POSITIONAL),
    ("crates/viewer/src/frame.rs", "Disagreement", POSITIONAL),
];

/// A raise site whose message is composed with a `Debug` rendering
/// rather than taken from a kernel `Display`.
///
/// The census above reads `impl Display` bodies, which is where all
/// three known instances were minted — but it is not the only route
/// into a typed refusal's message. This crate's own raise sites are
/// the other one, and inside one crate the question is local enough
/// to answer: the message argument of a [`crate::py::typed_err`] call
/// either carries a `{…?}` placeholder or it does not.
fn raise_sites_rendering_debug(sources: &[Source]) -> Vec<Site> {
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
            let inner = &code[open + 1..close];
            // `typed_err(py, class, message, fields)`: the message is
            // the third argument, and the arity is fixed by the
            // function's own signature.
            let arguments = top_level_split(inner, ',');
            let Some(message) = arguments.get(2) else {
                continue;
            };
            let span = open + 1 + message.start..open + 1 + message.end;
            if debug_placeholders(text, span.clone()).is_empty() {
                continue;
            }
            out.push(Site {
                file: source.file.clone(),
                line: text[..span.start].matches('\n').count() + 1,
                display_type: "typed_err".to_owned(),
                binding: text[span.clone()]
                    .trim()
                    .split('(')
                    .next()
                    .unwrap_or("")
                    .to_owned(),
                verdict: Verdict::Braced,
            });
        }
    }
    out.sort();
    out
}

/// The raise sites that compose a `Debug` rendering deliberately.
///
/// One arm, and `crates/pncad-py/src/errors.rs` already writes its
/// warning in prose: a future STRUCT variant of the kernel enum it
/// renders would trip the assertion and panic where that arm means to
/// refuse gracefully. This row is that warning made mechanical for
/// this crate — the site stays, and it stays NAMED.
const KNOWN_DEBUG_RAISES: &[(&str, &str)] = &[(
    "crates/pncad-py/src/py/flush.rs",
    "an unknown `ContactClass` has nothing but `Debug` to render it; the arm and      `errors.rs` both say so, and a struct variant of that enum would panic here",
)];

#[cfg(test)]
mod tests {
    use super::{
        KNOWN_BRACED, KNOWN_DEBUG_RAISES, POSITIONAL, Source, UNDECIDED, Verdict, census,
        code_and_literals, code_only, raise_sites_rendering_debug, read_sources, repo_root,
    };
    use crate::errors::reads_as_prose;
    use std::collections::BTreeSet;

    /// A one-file tree, so a planted case runs the real census rather
    /// than a second implementation of it.
    fn planted(text: &str) -> Vec<Source> {
        vec![Source {
            file: "crates/planted/src/lib.rs".to_owned(),
            text: code_and_literals(text),
            code: code_only(text),
        }]
    }

    /// The shape a value-sampling roster passes on: a payload enum
    /// with one struct variant among brace-free ones. The failing
    /// value is `Link`; `Chain` is the one a sampler reaches for.
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

    /// The blend shape: the rendering sits inside a NESTED match, so
    /// the binding is governed by an arm that is not the nearest one.
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
            "rendering the payload through `Display` is the repair, and the census \
             has to see it as one"
        );
    }

    #[test]
    fn a_rendering_inside_a_nested_match_is_still_resolved() {
        let sites = census(&planted(NESTED_MATCH));
        let braced: Vec<_> = sites
            .iter()
            .filter(|site| site.verdict == Verdict::Braced)
            .collect();
        assert_eq!(
            braced.len(),
            2,
            "both renderings are governed by the OUTER arm's binding; a resolver \
             that stops at the inner arm types neither, which is how two live \
             instances stayed hidden: {sites:?}"
        );
    }

    #[test]
    fn the_shape_this_census_hunts_really_trips_the_prose_gate() {
        // The anchor between the static verdict and the runtime one:
        // a struct-shaped payload rendered through `Debug` produces
        // exactly the fingerprint `typed_err` panics on.
        #[derive(Debug)]
        enum Site {
            Link {
                #[allow(
                    dead_code,
                    reason = "read by the derived `Debug`, which is the subject"
                )]
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
        let sites = census(&read_sources(&repo_root()));
        let braced: BTreeSet<_> = sites
            .iter()
            .filter(|site| site.verdict == Verdict::Braced)
            .map(super::Site::key)
            .collect();
        let allowed: BTreeSet<_> = KNOWN_BRACED
            .iter()
            .map(|(file, ty, binding, _)| {
                ((*file).to_owned(), (*ty).to_owned(), (*binding).to_owned())
            })
            .collect();
        let new: Vec<_> = braced.difference(&allowed).collect();
        assert!(
            new.is_empty(),
            "a `Display` impl renders a struct-shaped payload through `Debug`, so \
             the refusal PANICS the binding at `typed_err` instead of raising: \
             {new:#?}\nRender the payload through its own `Display`, or give it \
             one."
        );
        let stale: Vec<_> = allowed.difference(&braced).collect();
        assert!(
            stale.is_empty(),
            "KNOWN_BRACED names a site that is no longer brace-shaped — strike the \
             line in the PR that repaired it: {stale:#?}"
        );
    }

    #[test]
    fn every_site_the_resolver_cannot_type_is_named_in_the_census() {
        let sites = census(&read_sources(&repo_root()));
        let found: BTreeSet<_> = sites
            .iter()
            .filter(|site| site.verdict == Verdict::Undecided)
            .map(super::Site::key)
            .collect();
        let declared: BTreeSet<_> = UNDECIDED
            .iter()
            .map(|(file, ty, binding)| {
                ((*file).to_owned(), (*ty).to_owned(), (*binding).to_owned())
            })
            .collect();
        assert_eq!(
            found, declared,
            "the resolver's blind spot is a CENSUS, not a silence: a site it cannot \
             type either gets its line here or gets rewritten so it can be typed. \
             A positional `{{:?}}` becomes an inline `{{binding:?}}`; a binding \
             whose type it cannot decide is the `Real` scalar, whose interval \
             instantiation wraps a named-field struct."
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
            "the same vacuity one level in — no `Display` impl found is not a clean \
             tree: {} sites",
            sites.len()
        );
        assert!(
            sites.iter().any(|site| site.binding == POSITIONAL),
            "positional renderings exist in this tree; finding none means the \
             placeholder scan stopped reading"
        );
    }

    #[test]
    fn no_raise_site_composes_a_debug_rendering_into_its_message() {
        let found = raise_sites_rendering_debug(&read_sources(&repo_root()));
        let allowed: BTreeSet<_> = KNOWN_DEBUG_RAISES.iter().map(|(file, _)| *file).collect();
        let new: Vec<_> = found
            .iter()
            .filter(|site| !allowed.contains(site.file.as_str()))
            .collect();
        assert!(
            new.is_empty(),
            "a raise site builds its human message out of a `Debug` rendering, which \
             is the other route to the panic the census above guards: {new:#?}"
        );
        let stale: Vec<_> = allowed
            .iter()
            .filter(|file| !found.iter().any(|site| site.file == **file))
            .collect();
        assert!(
            stale.is_empty(),
            "KNOWN_DEBUG_RAISES names a site that no longer renders through `Debug`: \
             {stale:#?}"
        );
    }
}
