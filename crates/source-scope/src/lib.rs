//! **Scope questions about Rust source, answered off a parse.**
//!
//! A guard that asks *which binding does this name resolve to here*,
//! *what does this `use` tree import*, *which fn is this call inside*
//! needs Rust's scope rules, and a lexical reader of them is a list of
//! the shapes it knows: every shape it does not know is a blind spot, and
//! closing one has opened the next (the census in
//! `geom-core/tests/flagged_census.rs` before it moved here). This crate
//! parses the file with `syn` and resolves names with a scope stack, so a
//! guard keeps only its own question.
//!
//! Lexing — *is this text code, a comment or a literal* — stays in
//! `test_utils::source`, which is a zero-dependency leaf on purpose.
//! `test-utils/tests/reader_census.rs` is the ledger that sends a scope
//! question here and refuses a hand-rolled scope reader.
//!
//! # What [`read`] gives
//!
//! - [`Source::uses`]: every `use` leaf, groups, nesting, `self`, globs
//!   and `as` expanded.
//! - [`Source::fns`]: every `fn` item (free, in an `impl` or a `trait`),
//!   its visibility, its parameters' names and type shapes, and whether
//!   it sits under `#[cfg(test)]`.
//! - [`Source::type_aliases`]: every `type` item and the identifiers its
//!   right side names.
//! - [`Source::events`]: every call (a path call, a method call) and
//!   every mention of a name that is not a local variable, with its line,
//!   the fn it sits in, and, for a bare name, the binding it resolves to
//!   ([`Bound`]).
//!
//! # How a name resolves
//!
//! Innermost scope first, on Rust's lexical rules:
//! - a closure's parameters over its body;
//! - a match arm's, an `if let`'s (and each `let` of a let chain), a
//!   `while let`'s and a `for`'s pattern over what it governs;
//! - a `let` from the statement after it to its block's end;
//! - a block's own items (`fn`, `use`, `const`, `static`) over the whole
//!   block;
//! - a fn's parameters over its body (a fn item does not see the locals of
//!   a fn around it);
//! - then the module's items and imports. A `mod` does not see its
//!   parent's items except through an import.
//!
//! An import whose path stays in the file is **followed**: `use
//! super::name`, `use self::m::name`, a glob of `super`, `self` or an
//! inline module, and a name a parent or child module imports, resolve
//! where they point, recursively. The [`Bound::Use`] a guard sees is the
//! import that leaves the file. A path call whose head is in the file
//! (`self::f()`, `super::f()`, `m::f()`, a name an import binds) is
//! resolved the same way ([`Callee::Path`]'s `resolved`). Two item
//! bindings of one name in one scope are [`Bound::Ambiguous`], not the
//! last one read. A method or an associated fn in an `impl` or a
//! `trait` binds no bare name, and neither does a struct field.
//!
//! # What it cannot read, stated
//!
//! - **What a macro expands to.** A macro's arguments are read as a
//!   comma-separated list of expressions where they parse as one (most
//!   `assert!`, `format!`-family and `debug_assert!` calls). Where they do
//!   not (`vec![x; n]`, a custom syntax) and in a `macro_rules!` body,
//!   every identifier is an event of kind [`Callee::Macro`], so a guard
//!   can list it rather than drop it.
//! - **What a glob that leaves the file brings in.** It is reported as
//!   the paths it reads, with what the name resolves to past it
//!   ([`Bound::Glob`]); whether the module behind it defines the name is
//!   not known here.
//! - **Which `#[cfg]` arm a build takes.** Every declaration is read, and
//!   two of one name are [`Bound::Ambiguous`].
//! - **Anything outside the file.** `super::` from the file's own module,
//!   `crate::`, and another file's `mod m;` are where a path leaves; the
//!   path is reported as written.
//! - **Types, traits and macros by name.** A name is resolved in the
//!   value namespace only; a method call's receiver type is not inferred,
//!   and a path through a type (`T::f`) is not followed.

use std::collections::HashMap;

use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::visit::{self, Visit};

/// One file, read.
#[derive(Debug, Default)]
pub struct Source {
    /// Every `use` leaf, in source order.
    pub uses: Vec<UseLeaf>,
    /// Every `fn` item, in source order.
    pub fns: Vec<FnItem>,
    /// Every `type` item.
    pub type_aliases: Vec<TypeAlias>,
    /// Every call and every non-local mention, in source order.
    pub events: Vec<Event>,
}

/// One leaf of a `use` tree: the path it imports, by segment (a `self`
/// leaf is its module's path; a glob ends in `*`), its `as` name, whether
/// the item re-exports it, and the line of its `use` item.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UseLeaf {
    /// The imported path's segments.
    pub path: Vec<String>,
    /// The `as` name, if any.
    pub alias: Option<String>,
    /// Whether the `use` item is `pub` (`pub(…)` included).
    pub public: bool,
    /// The line of the `use` item.
    pub line: usize,
}

/// A `fn` item.
#[derive(Clone, Debug)]
pub struct FnItem {
    /// Its name.
    pub name: String,
    /// Its typed parameters, in order (a receiver is not one).
    pub params: Vec<Param>,
    /// Whether it is `pub` (`pub(…)` included).
    pub public: bool,
    /// Whether it is defined in an `impl` block.
    pub in_impl: bool,
    /// Whether it, or an item around it, carries `#[cfg(test)]`.
    pub in_test: bool,
    /// Whether it takes a `self` receiver.
    pub receiver: bool,
}

/// A typed parameter: its name where its pattern is one identifier, and
/// its type's shape.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param {
    /// The bound name (`mut` dropped), `None` for a pattern.
    pub name: Option<String>,
    /// The type's shape.
    pub ty: TypeShape,
}

/// What a guard reads of a type without needing `syn`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeShape {
    /// Every identifier the type names (lifetimes included, without `'`).
    pub idents: Vec<String>,
    /// Whether it is an array or a slice once references are read off.
    pub list: bool,
    /// Whether it is exactly `&'static str`.
    pub static_str: bool,
}

/// A `type` item: its name and the identifiers its right side names.
#[derive(Clone, Debug)]
pub struct TypeAlias {
    /// The alias.
    pub name: String,
    /// Every identifier of its right side.
    pub rhs: Vec<String>,
}

/// One call or mention of a name.
#[derive(Clone, Debug)]
pub struct Event {
    /// The name called or mentioned (a path's last segment, a method's
    /// name).
    pub name: String,
    /// How it is reached.
    pub callee: Callee,
    /// The call's arguments (a method call's, without the receiver), or
    /// `None` for a mention.
    pub args: Option<Vec<Arg>>,
    /// The line of the name.
    pub line: usize,
    /// The fn item around it (closures are not fns), as its index in
    /// [`Source::fns`], if any.
    pub enclosing_fn: Option<usize>,
}

/// How an [`Event`]'s name is reached.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Callee {
    /// A one-segment path, and the binding it resolves to, if any.
    Bare(Option<Bound>),
    /// A path of more than one segment, or a qualified path `<T>::name`:
    /// the segment before the name (for a qualified path, the qualifying
    /// type's last segment), if there is one, and what the path resolves
    /// to where its head is in this file (`self::`, `super::`, an inline
    /// module, or a name a `use` binds here).
    Path {
        /// The qualifier.
        qualifier: Option<String>,
        /// The binding the path reaches, followed as for a bare name;
        /// `None` where its head is not in this file, or `super::` leaves
        /// the file.
        resolved: Option<Bound>,
    },
    /// A method call `.name(…)`.
    Method,
    /// An identifier inside a macro's tokens that did not parse as
    /// expressions, or inside a `macro_rules!` body.
    Macro,
}

/// What a bare name resolves to at one point.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Bound {
    /// An explicit `use` leaf, followed: a leaf whose path stays in this
    /// file (`use super::name`, `use self::m::name`, a name a parent
    /// module imports) is resolved where it points, so `path` is the one
    /// that leaves the file's modules, as the last import in the chain
    /// spells it.
    Use {
        /// The followed path.
        path: Vec<String>,
        /// The `as` name of the leaf the name was bound by.
        alias: Option<String>,
    },
    /// No explicit binding at the scope that answered; globs reaching
    /// outside the file may bring the name in. `behind` is what the name
    /// resolves to past those globs (an outer scope's binding, or an
    /// in-file glob's), if anything: Rust takes the glob's item where the
    /// glob's module defines the name, and `behind` otherwise.
    Glob {
        /// The globs' paths, each ending in `*`.
        paths: Vec<Vec<String>>,
        /// The binding past them.
        behind: Option<Box<Bound>>,
    },
    /// More than one binding of the name in one scope: item declarations
    /// that `#[cfg]` keeps apart (`#[cfg(test)] use a::f;` beside
    /// `#[cfg(not(test))] use b::f;`), or globs that each bring it in.
    /// Each is followed; which one a build sees is not read here.
    Ambiguous(Vec<Bound>),
    /// An item of a block or module: a `fn`, a `const` or a `static`.
    Item,
    /// A parameter of the fn around it.
    Param {
        /// The fn's name.
        fn_name: String,
        /// The parameter's index.
        index: usize,
    },
    /// A `let`, a closure parameter, or a pattern's binding.
    Local,
}

impl Bound {
    /// The followed `use` paths this binding reaches through: its own,
    /// the one behind a glob, and each of an ambiguity's.
    pub fn targets(&self) -> Vec<&[String]> {
        match self {
            Bound::Use { path, .. } => vec![path.as_slice()],
            Bound::Glob {
                behind: Some(b), ..
            } => b.targets(),
            Bound::Ambiguous(all) => all.iter().flat_map(Bound::targets).collect(),
            _ => Vec::new(),
        }
    }
}

impl Event {
    /// The binding this event's name resolves to, bare or by an in-file
    /// path.
    pub fn bound(&self) -> Option<&Bound> {
        match &self.callee {
            Callee::Bare(b) | Callee::Path { resolved: b, .. } => b.as_ref(),
            Callee::Method | Callee::Macro => None,
        }
    }

    /// **The names this event can reach**: its own, and the last segment
    /// of every import it resolves through (`pick` bound by
    /// `use k::choose as pick` reaches `choose`). The one alias rule: a
    /// guard indexing events by name and a guard matching a binding's
    /// path read the same followed paths ([`Bound::targets`]).
    pub fn reaches(&self) -> Vec<&str> {
        let mut out = vec![self.name.as_str()];
        if let Some(b) = self.bound() {
            for t in b.targets() {
                if let Some(last) = t.last()
                    && !out.contains(&last.as_str())
                {
                    out.push(last);
                }
            }
        }
        out
    }
}

/// One argument of a call.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Arg {
    /// A string literal's value.
    Str(String),
    /// A one-segment path, and what it resolves to.
    Ident {
        /// The name.
        name: String,
        /// Its binding, if any.
        bound: Option<Bound>,
    },
    /// Anything else.
    Other,
}

/// **Reads `text` as one Rust file.**
///
/// # Errors
///
/// `syn`'s refusal where the text does not parse as a file.
pub fn read(text: &str) -> Result<Source, syn::Error> {
    let file = syn::parse_file(text)?;
    let mut r = Reader::default();
    let root = r.build_mod(None, &file.items);
    r.frames.push(Frame::module(root));
    for item in &file.items {
        r.visit_item(item);
    }
    Ok(r.out)
}

/// What a scope frame is, which decides what a lookup may see through it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    /// A module: items and imports; nothing outside it is seen.
    Module,
    /// A fn's parameters: outer locals are not seen past it.
    Fn,
    /// A block: its items over all of it, its `let`s from each on.
    Block,
    /// A closure's, an arm's or a loop's bindings.
    Local,
}

/// A scope on the stack. A module's names live in [`Reader::mods`], so
/// a path can reach a module that is not open; every other scope's live
/// here.
#[derive(Debug)]
struct Frame {
    kind: Kind,
    /// For a module frame, its index in [`Reader::mods`].
    module: Option<usize>,
    names: HashMap<String, Bound>,
    globs: Vec<Vec<String>>,
}

impl Frame {
    fn new(kind: Kind) -> Self {
        Self {
            kind,
            module: None,
            names: HashMap::new(),
            globs: Vec::new(),
        }
    }

    fn module(index: usize) -> Self {
        Self {
            module: Some(index),
            ..Self::new(Kind::Module)
        }
    }
}

/// A module of the file (the file itself, or an inline `mod`): its
/// value-namespace items and imports, its globs, and its in-file
/// parent and children, so that `self::`, `super::` and a child's name
/// can be followed.
#[derive(Debug, Default)]
struct ModNode {
    parent: Option<usize>,
    children: HashMap<String, usize>,
    names: HashMap<String, Bound>,
    globs: Vec<Vec<String>>,
}

/// How deep a chain of in-file imports is followed before it is called
/// unresolved: a cycle (`a::x` importing `b::x` importing `a::x`) is a
/// compile error, and this bounds the walk over one.
const FOLLOW_DEPTH: usize = 32;

/// Adds `bound` for `name` to one scope's item names: a second binding
/// of the name there is [`Bound::Ambiguous`], not a silent replacement.
fn insert_item(names: &mut HashMap<String, Bound>, name: String, bound: Bound) {
    match names.remove(&name) {
        None => {
            names.insert(name, bound);
        }
        Some(Bound::Ambiguous(mut all)) => {
            all.push(bound);
            names.insert(name, Bound::Ambiguous(all));
        }
        Some(old) => {
            names.insert(name, Bound::Ambiguous(vec![old, bound]));
        }
    }
}

/// The span start that keys an inline module in [`Reader::mod_at`].
fn mod_key(m: &syn::ItemMod) -> (usize, usize) {
    let at = m.mod_token.span.start();
    (at.line, at.column)
}

#[derive(Default)]
struct Reader {
    out: Source,
    frames: Vec<Frame>,
    /// Every module of the file the visit has reached or will reach.
    mods: Vec<ModNode>,
    /// Each inline module's index in `mods`, by its `mod` token.
    mod_at: HashMap<(usize, usize), usize>,
    /// The indices in `out.fns` of the fn items around the point read.
    fns: Vec<usize>,
    in_impl: usize,
    in_test: usize,
}

fn line(span: Span) -> usize {
    span.start().line
}

/// **Whether `attrs` hold `#[cfg(test)]`, spelled exactly so**: the
/// parse-level home of the rule the tree's text readers apply by the
/// literal `#[cfg(test)]` (`test_utils::source::attributed_extent`'s
/// callers). A compound `cfg(all(test, …))` is not read as a test item,
/// the same as there.
pub fn is_cfg_test(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|a| {
        a.path().is_ident("cfg")
            && matches!(&a.meta, syn::Meta::List(l) if l.tokens.to_string().trim() == "test")
    })
}

fn public(vis: &syn::Visibility) -> bool {
    !matches!(vis, syn::Visibility::Inherited)
}

/// The identifiers a pattern binds.
fn pat_names(pat: &syn::Pat, out: &mut Vec<String>) {
    use syn::Pat;
    match pat {
        Pat::Ident(p) => {
            out.push(p.ident.to_string());
            if let Some((_, sub)) = &p.subpat {
                pat_names(sub, out);
            }
        }
        Pat::Tuple(p) => p.elems.iter().for_each(|e| pat_names(e, out)),
        Pat::TupleStruct(p) => p.elems.iter().for_each(|e| pat_names(e, out)),
        Pat::Struct(p) => p.fields.iter().for_each(|f| pat_names(&f.pat, out)),
        Pat::Slice(p) => p.elems.iter().for_each(|e| pat_names(e, out)),
        Pat::Reference(p) => pat_names(&p.pat, out),
        Pat::Or(p) => p.cases.iter().for_each(|c| pat_names(c, out)),
        Pat::Type(p) => pat_names(&p.pat, out),
        Pat::Paren(p) => pat_names(&p.pat, out),
        _ => {}
    }
}

/// Every identifier under a syntax node.
fn idents_of<T: quote::ToTokens>(node: &T) -> Vec<String> {
    fn walk(ts: proc_macro2::TokenStream, out: &mut Vec<String>) {
        for tt in ts {
            match tt {
                proc_macro2::TokenTree::Ident(i) => out.push(i.to_string()),
                proc_macro2::TokenTree::Group(g) => walk(g.stream(), out),
                _ => {}
            }
        }
    }
    let mut out = Vec::new();
    walk(node.to_token_stream(), &mut out);
    out
}

fn shape(ty: &syn::Type) -> TypeShape {
    fn list(ty: &syn::Type) -> bool {
        match ty {
            syn::Type::Reference(r) => list(&r.elem),
            syn::Type::Paren(p) => list(&p.elem),
            syn::Type::Group(g) => list(&g.elem),
            syn::Type::Array(_) | syn::Type::Slice(_) => true,
            _ => false,
        }
    }
    let static_str = matches!(ty, syn::Type::Reference(r)
        if r.mutability.is_none()
            && r.lifetime.as_ref().is_some_and(|l| l.ident == "static")
            && matches!(&*r.elem, syn::Type::Path(p) if p.qself.is_none() && p.path.is_ident("str")));
    TypeShape {
        idents: idents_of(ty),
        list: list(ty),
        static_str,
    }
}

fn params_of(sig: &syn::Signature) -> (Vec<Param>, bool) {
    let mut receiver = false;
    let mut params = Vec::new();
    for input in &sig.inputs {
        match input {
            syn::FnArg::Receiver(_) => receiver = true,
            syn::FnArg::Typed(t) => params.push(Param {
                name: match &*t.pat {
                    syn::Pat::Ident(p) if p.subpat.is_none() => Some(p.ident.to_string()),
                    _ => None,
                },
                ty: shape(&t.ty),
            }),
        }
    }
    (params, receiver)
}

/// `tree`'s leaves under `prefix`.
fn use_leaves(
    tree: &syn::UseTree,
    prefix: &mut Vec<String>,
    out: &mut Vec<(Vec<String>, Option<String>)>,
) {
    use syn::UseTree;
    match tree {
        UseTree::Path(p) => {
            let seg = p.ident.to_string();
            let pushed = seg != "self";
            if pushed {
                prefix.push(seg);
            }
            use_leaves(&p.tree, prefix, out);
            if pushed {
                prefix.pop();
            }
        }
        UseTree::Name(n) => {
            let mut path = prefix.clone();
            if n.ident != "self" {
                path.push(n.ident.to_string());
            }
            out.push((path, None));
        }
        UseTree::Rename(r) => {
            let mut path = prefix.clone();
            if r.ident != "self" {
                path.push(r.ident.to_string());
            }
            out.push((path, Some(r.rename.to_string())));
        }
        UseTree::Glob(_) => {
            let mut path = prefix.clone();
            path.push("*".to_string());
            out.push((path, None));
        }
        UseTree::Group(g) => g.items.iter().for_each(|t| use_leaves(t, prefix, out)),
    }
}

impl Reader {
    fn top(&mut self) -> &mut Frame {
        match self.frames.last_mut() {
            Some(frame) => frame,
            // `read` opens the file's module frame first, and every push
            // has its pop: the stack is never empty while a visit runs.
            None => unreachable!("a frame is always open"),
        }
    }

    fn bind(&mut self, name: String, bound: Bound) {
        self.top().names.insert(name, bound);
    }

    fn bind_pat(&mut self, pat: &syn::Pat) {
        let mut names = Vec::new();
        pat_names(pat, &mut names);
        for n in names {
            self.bind(n, Bound::Local);
        }
    }

    /// The module the innermost module frame opens.
    fn here(&self) -> usize {
        self.frames.iter().rev().find_map(|f| f.module).unwrap_or(0)
    }

    /// **The binding `name` resolves to here**, innermost scope first.
    ///
    /// A local is seen up to the nearest fn frame; an item or an import
    /// through it. A block's globs that leave the file are held while
    /// the walk goes on outward, and wrap what it finds
    /// ([`Bound::Glob`]'s `behind`). The first module frame answers for
    /// the rest ([`Reader::in_mod`]).
    fn lookup(&self, name: &str) -> Option<Bound> {
        let here = self.here();
        let mut locals = true;
        let mut held: Vec<Vec<String>> = Vec::new();
        let wrap = |held: Vec<Vec<String>>, found: Option<Bound>| {
            if held.is_empty() {
                found
            } else {
                Some(Bound::Glob {
                    paths: held,
                    behind: found.map(Box::new),
                })
            }
        };
        for frame in self.frames.iter().rev() {
            if let Some(m) = frame.module {
                return wrap(held, self.in_mod(m, name, 0));
            }
            if let Some(b) = frame.names.get(name) {
                let item = matches!(b, Bound::Item | Bound::Use { .. } | Bound::Ambiguous(_));
                if locals || item {
                    return wrap(held, Some(self.follow(here, b.clone(), 0)));
                }
            }
            for g in &frame.globs {
                match self.module_at(here, &g[..g.len() - 1], 0) {
                    Some(m) => {
                        if let Some(found) = self.in_mod(m, name, 0) {
                            return wrap(held, Some(found));
                        }
                    }
                    None => held.push(g.clone()),
                }
            }
            if frame.kind == Kind::Fn {
                locals = false;
            }
        }
        wrap(held, None)
    }

    /// What `name` resolves to in module `m`: its own item or import
    /// (followed), else what its in-file globs reach, wrapped in the
    /// globs that leave the file.
    fn in_mod(&self, m: usize, name: &str, depth: usize) -> Option<Bound> {
        if depth > FOLLOW_DEPTH {
            return None;
        }
        let node = &self.mods[m];
        if let Some(b) = node.names.get(name) {
            return Some(self.follow(m, b.clone(), depth + 1));
        }
        let mut found: Vec<Bound> = Vec::new();
        let mut held: Vec<Vec<String>> = Vec::new();
        for g in &node.globs {
            match self.module_at(m, &g[..g.len() - 1], depth + 1) {
                Some(target) => {
                    if let Some(b) = self.in_mod(target, name, depth + 1)
                        && !found.contains(&b)
                    {
                        found.push(b);
                    }
                }
                None => held.push(g.clone()),
            }
        }
        let found = match found.len() {
            0 => None,
            1 => found.pop(),
            _ => Some(Bound::Ambiguous(found)),
        };
        if held.is_empty() {
            found
        } else {
            Some(Bound::Glob {
                paths: held,
                behind: found.map(Box::new),
            })
        }
    }

    /// `b`, bound in module `m`, followed where it is an import whose
    /// path stays in the file.
    fn follow(&self, m: usize, b: Bound, depth: usize) -> Bound {
        match b {
            Bound::Use { path, alias } => match self.in_file_path(m, &path, depth + 1) {
                Some(Bound::Use {
                    path: p,
                    alias: inner,
                }) => Bound::Use {
                    path: p,
                    alias: alias.or(inner),
                },
                Some(other) => other,
                None => Bound::Use { path, alias },
            },
            Bound::Ambiguous(all) => Bound::Ambiguous(
                all.into_iter()
                    .map(|b| self.follow(m, b, depth + 1))
                    .collect(),
            ),
            other => other,
        }
    }

    /// The in-file module `path` names from module `m` (`self`, `super`,
    /// a child by name), if it stays in the file.
    fn module_at(&self, m: usize, path: &[String], depth: usize) -> Option<usize> {
        if depth > FOLLOW_DEPTH {
            return None;
        }
        let Some((head, rest)) = path.split_first() else {
            return Some(m);
        };
        let next = match head.as_str() {
            "self" => m,
            "super" => self.mods[m].parent?,
            child => *self.mods[m].children.get(child)?,
        };
        self.module_at(next, rest, depth + 1)
    }

    /// What `path`, written in module `m`, reaches where its head is in
    /// the file: `self::`, `super::`, a child module, or a name an import
    /// of `m` binds (whose path then stands in for the head). `None`
    /// where the head is not in the file, or the path leaves it.
    fn in_file_path(&self, m: usize, path: &[String], depth: usize) -> Option<Bound> {
        if depth > FOLLOW_DEPTH {
            return None;
        }
        let (last, init) = path.split_last()?;
        if init.is_empty() {
            return None;
        }
        let (head, _) = init.split_first()?;
        if matches!(head.as_str(), "self" | "super") || self.mods[m].children.contains_key(head) {
            let target = self.module_at(m, init, depth + 1)?;
            return self.in_mod(target, last, depth + 1);
        }
        match self.mods[m].names.get(head) {
            Some(Bound::Use { path: through, .. }) => {
                let mut whole = through.clone();
                whole.extend(path[1..].iter().cloned());
                Some(
                    self.in_file_path(m, &whole, depth + 1)
                        .unwrap_or(Bound::Use {
                            path: whole,
                            alias: None,
                        }),
                )
            }
            _ => None,
        }
    }

    /// One scope's item names and globs, and its `use` leaves recorded:
    /// `fn`, `const` and `static` items, and each import.
    fn item_names(&mut self, items: &[syn::Item]) -> (HashMap<String, Bound>, Vec<Vec<String>>) {
        let mut names = HashMap::new();
        let mut globs = Vec::new();
        for item in items {
            match item {
                syn::Item::Fn(f) => insert_item(&mut names, f.sig.ident.to_string(), Bound::Item),
                syn::Item::Const(c) => insert_item(&mut names, c.ident.to_string(), Bound::Item),
                syn::Item::Static(s) => insert_item(&mut names, s.ident.to_string(), Bound::Item),
                syn::Item::Use(u) => {
                    let mut leaves = Vec::new();
                    use_leaves(&u.tree, &mut Vec::new(), &mut leaves);
                    for (path, alias) in leaves {
                        if path.last().map(String::as_str) == Some("*") {
                            globs.push(path.clone());
                        } else if let Some(name) = alias.clone().or_else(|| path.last().cloned())
                            && name != "_"
                        {
                            insert_item(
                                &mut names,
                                name,
                                Bound::Use {
                                    path: path.clone(),
                                    alias: alias.clone(),
                                },
                            );
                        }
                        self.out.uses.push(UseLeaf {
                            path,
                            alias,
                            public: public(&u.vis),
                            line: line(u.use_token.span),
                        });
                    }
                }
                _ => {}
            }
        }
        (names, globs)
    }

    /// Builds the module of `items` under `parent`, and every inline
    /// module inside it, before any of it is visited: a path may name a
    /// module the visit has not reached yet.
    fn build_mod(&mut self, parent: Option<usize>, items: &[syn::Item]) -> usize {
        let (names, globs) = self.item_names(items);
        let index = self.mods.len();
        self.mods.push(ModNode {
            parent,
            children: HashMap::new(),
            names,
            globs,
        });
        for item in items {
            if let syn::Item::Mod(m) = item
                && let Some((_, inner)) = &m.content
            {
                let child = self.build_mod(Some(index), inner);
                self.mod_at.insert(mod_key(m), child);
                self.mods[index].children.insert(m.ident.to_string(), child);
            }
        }
        index
    }

    /// Declares a block's items on the open frame before the block is
    /// read (an item is in scope over all of it), and builds the inline
    /// modules among them.
    fn declare_items(&mut self, items: &[syn::Item]) {
        let (names, globs) = self.item_names(items);
        let here = self.here();
        for item in items {
            if let syn::Item::Mod(m) = item
                && let Some((_, inner)) = &m.content
            {
                let child = self.build_mod(Some(here), inner);
                self.mod_at.insert(mod_key(m), child);
            }
        }
        let top = self.top();
        for (name, b) in names {
            insert_item(&mut top.names, name, b);
        }
        top.globs.extend(globs);
    }

    fn arg(&self, e: &syn::Expr) -> Arg {
        match e {
            syn::Expr::Lit(l) => match &l.lit {
                syn::Lit::Str(s) => Arg::Str(s.value()),
                _ => Arg::Other,
            },
            syn::Expr::Path(p)
                if p.qself.is_none()
                    && p.path.segments.len() == 1
                    && p.path.leading_colon.is_none() =>
            {
                let name = p.path.segments[0].ident.to_string();
                Arg::Ident {
                    bound: self.lookup(&name),
                    name,
                }
            }
            syn::Expr::Paren(p) => self.arg(&p.expr),
            syn::Expr::Group(g) => self.arg(&g.expr),
            _ => Arg::Other,
        }
    }

    /// What a path of more than one segment reaches from here, where its
    /// head is in this file: `self::` and `super::` from the innermost
    /// module, a child module, or a name an import binds in scope.
    fn path_binding(&self, segs: &[String]) -> Option<Bound> {
        let here = self.here();
        let head = segs.first()?;
        if let Some(found) = self.in_file_path(here, segs, 0) {
            return Some(found);
        }
        if matches!(head.as_str(), "self" | "super") {
            return None;
        }
        match self.lookup(head) {
            Some(Bound::Use { path, .. }) => {
                let mut whole = path;
                whole.extend(segs[1..].iter().cloned());
                Some(Bound::Use {
                    path: whole,
                    alias: None,
                })
            }
            _ => None,
        }
    }

    /// An event for the path `p`, called with `args` or mentioned.
    fn path_event(&mut self, p: &syn::ExprPath, args: Option<Vec<Arg>>) {
        let Some(last) = p.path.segments.last() else {
            return;
        };
        let name = last.ident.to_string();
        let callee = if let Some(q) = &p.qself {
            Callee::Path {
                qualifier: match &*q.ty {
                    syn::Type::Path(t) => t.path.segments.last().map(|s| s.ident.to_string()),
                    _ => None,
                },
                resolved: None,
            }
        } else if p.path.segments.len() > 1 && p.path.leading_colon.is_none() {
            let n = p.path.segments.len();
            let segs: Vec<String> = p
                .path
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect();
            Callee::Path {
                qualifier: Some(segs[n - 2].clone()),
                resolved: self.path_binding(&segs),
            }
        } else if p.path.segments.len() > 1 {
            let n = p.path.segments.len();
            Callee::Path {
                qualifier: Some(p.path.segments[n - 2].ident.to_string()),
                resolved: None,
            }
        } else if p.path.leading_colon.is_some() {
            Callee::Path {
                qualifier: None,
                resolved: None,
            }
        } else {
            let bound = self.lookup(&name);
            if args.is_none() && matches!(bound, Some(Bound::Local | Bound::Param { .. })) {
                return;
            }
            Callee::Bare(bound)
        };
        self.out.events.push(Event {
            name,
            callee,
            args,
            line: line(last.ident.span()),
            enclosing_fn: self.fns.last().copied(),
        });
    }

    fn fn_body(
        &mut self,
        sig: &syn::Signature,
        attrs: &[syn::Attribute],
        vis: Option<&syn::Visibility>,
        block: Option<&syn::Block>,
    ) {
        let (params, receiver) = params_of(sig);
        let test = is_cfg_test(attrs);
        if test {
            self.in_test += 1;
        }
        self.out.fns.push(FnItem {
            name: sig.ident.to_string(),
            params,
            public: vis.is_some_and(public),
            in_impl: self.in_impl > 0,
            in_test: self.in_test > 0,
            receiver,
        });
        if let Some(block) = block {
            let name = sig.ident.to_string();
            self.fns.push(self.out.fns.len() - 1);
            let mut frame = Frame::new(Kind::Fn);
            for (index, input) in sig
                .inputs
                .iter()
                .filter_map(|i| match i {
                    syn::FnArg::Typed(t) => Some(t),
                    syn::FnArg::Receiver(_) => None,
                })
                .enumerate()
            {
                match &*input.pat {
                    syn::Pat::Ident(p) if p.subpat.is_none() => {
                        frame.names.insert(
                            p.ident.to_string(),
                            Bound::Param {
                                fn_name: name.clone(),
                                index,
                            },
                        );
                    }
                    pat => {
                        let mut names = Vec::new();
                        pat_names(pat, &mut names);
                        for n in names {
                            frame.names.insert(n, Bound::Local);
                        }
                    }
                }
            }
            self.frames.push(frame);
            self.visit_block(block);
            self.frames.pop();
            self.fns.pop();
        }
        if test {
            self.in_test -= 1;
        }
    }

    /// Reads a macro's arguments as expressions where they parse as a
    /// comma-separated list, and lists its identifiers otherwise.
    fn read_macro(&mut self, mac: &syn::Macro) {
        if let Ok(exprs) =
            mac.parse_body_with(Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated)
        {
            for e in &exprs {
                self.visit_expr(e);
            }
            return;
        }
        self.macro_tokens(mac.tokens.clone());
    }

    fn macro_tokens(&mut self, ts: proc_macro2::TokenStream) {
        for tt in ts {
            match tt {
                proc_macro2::TokenTree::Ident(i) => {
                    let event = Event {
                        name: i.to_string(),
                        callee: Callee::Macro,
                        args: None,
                        line: line(i.span()),
                        enclosing_fn: self.fns.last().copied(),
                    };
                    self.out.events.push(event);
                }
                proc_macro2::TokenTree::Group(g) => self.macro_tokens(g.stream()),
                _ => {}
            }
        }
    }

    /// Reads an `if`/`while` condition, binding each `let`'s pattern on
    /// the open frame after its scrutinee is read.
    fn condition(&mut self, cond: &syn::Expr) {
        match cond {
            syn::Expr::Let(l) => {
                self.visit_expr(&l.expr);
                self.bind_pat(&l.pat);
            }
            syn::Expr::Binary(b) if matches!(b.op, syn::BinOp::And(_)) => {
                self.condition(&b.left);
                self.condition(&b.right);
            }
            other => self.visit_expr(other),
        }
    }
}

impl<'ast> Visit<'ast> for Reader {
    fn visit_item_fn(&mut self, f: &'ast syn::ItemFn) {
        self.fn_body(&f.sig, &f.attrs, Some(&f.vis), Some(&f.block));
    }

    fn visit_impl_item_fn(&mut self, f: &'ast syn::ImplItemFn) {
        self.fn_body(&f.sig, &f.attrs, Some(&f.vis), Some(&f.block));
    }

    fn visit_trait_item_fn(&mut self, f: &'ast syn::TraitItemFn) {
        self.fn_body(&f.sig, &f.attrs, None, f.default.as_ref());
    }

    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let test = is_cfg_test(&i.attrs);
        self.in_impl += 1;
        self.in_test += usize::from(test);
        for item in &i.items {
            self.visit_impl_item(item);
        }
        self.in_test -= usize::from(test);
        self.in_impl -= 1;
    }

    fn visit_item_trait(&mut self, t: &'ast syn::ItemTrait) {
        let test = is_cfg_test(&t.attrs);
        self.in_test += usize::from(test);
        for item in &t.items {
            self.visit_trait_item(item);
        }
        self.in_test -= usize::from(test);
    }

    fn visit_item_mod(&mut self, m: &'ast syn::ItemMod) {
        let Some((_, items)) = &m.content else {
            return;
        };
        let test = is_cfg_test(&m.attrs);
        self.in_test += usize::from(test);
        let saved = std::mem::take(&mut self.fns);
        let index = match self.mod_at.get(&mod_key(m)) {
            Some(&i) => i,
            None => {
                let parent = self.here();
                self.build_mod(Some(parent), items)
            }
        };
        self.frames.push(Frame::module(index));
        for item in items {
            self.visit_item(item);
        }
        self.frames.pop();
        self.fns = saved;
        self.in_test -= usize::from(test);
    }

    fn visit_item_use(&mut self, _: &'ast syn::ItemUse) {
        // Declared with its scope's items (`declare_items`).
    }

    fn visit_item_type(&mut self, t: &'ast syn::ItemType) {
        self.out.type_aliases.push(TypeAlias {
            name: t.ident.to_string(),
            rhs: idents_of(&*t.ty),
        });
        visit::visit_item_type(self, t);
    }

    fn visit_item_macro(&mut self, m: &'ast syn::ItemMacro) {
        if m.mac.path.is_ident("macro_rules") {
            self.macro_tokens(m.mac.tokens.clone());
        } else {
            self.read_macro(&m.mac);
        }
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        self.read_macro(mac);
    }

    fn visit_block(&mut self, b: &'ast syn::Block) {
        self.frames.push(Frame::new(Kind::Block));
        let items: Vec<syn::Item> = b
            .stmts
            .iter()
            .filter_map(|s| match s {
                syn::Stmt::Item(i) => Some(i.clone()),
                _ => None,
            })
            .collect();
        self.declare_items(&items);
        for stmt in &b.stmts {
            match stmt {
                syn::Stmt::Local(l) => {
                    if let Some(init) = &l.init {
                        self.visit_expr(&init.expr);
                        if let Some((_, diverge)) = &init.diverge {
                            self.visit_expr(diverge);
                        }
                    }
                    self.bind_pat(&l.pat);
                }
                other => self.visit_stmt(other),
            }
        }
        self.frames.pop();
    }

    fn visit_expr_closure(&mut self, c: &'ast syn::ExprClosure) {
        self.frames.push(Frame::new(Kind::Local));
        for input in &c.inputs {
            self.bind_pat(input);
        }
        self.visit_expr(&c.body);
        self.frames.pop();
    }

    fn visit_expr_match(&mut self, m: &'ast syn::ExprMatch) {
        self.visit_expr(&m.expr);
        for arm in &m.arms {
            self.frames.push(Frame::new(Kind::Local));
            self.bind_pat(&arm.pat);
            if let Some((_, guard)) = &arm.guard {
                self.condition(guard);
            }
            self.visit_expr(&arm.body);
            self.frames.pop();
        }
    }

    fn visit_expr_for_loop(&mut self, f: &'ast syn::ExprForLoop) {
        self.visit_expr(&f.expr);
        self.frames.push(Frame::new(Kind::Local));
        self.bind_pat(&f.pat);
        self.visit_block(&f.body);
        self.frames.pop();
    }

    fn visit_expr_if(&mut self, i: &'ast syn::ExprIf) {
        self.frames.push(Frame::new(Kind::Local));
        self.condition(&i.cond);
        self.visit_block(&i.then_branch);
        self.frames.pop();
        if let Some((_, e)) = &i.else_branch {
            self.visit_expr(e);
        }
    }

    fn visit_expr_while(&mut self, w: &'ast syn::ExprWhile) {
        self.frames.push(Frame::new(Kind::Local));
        self.condition(&w.cond);
        self.visit_block(&w.body);
        self.frames.pop();
    }

    fn visit_expr_let(&mut self, l: &'ast syn::ExprLet) {
        self.visit_expr(&l.expr);
        self.bind_pat(&l.pat);
    }

    fn visit_expr_call(&mut self, c: &'ast syn::ExprCall) {
        let args: Vec<Arg> = c.args.iter().map(|a| self.arg(a)).collect();
        match &*c.func {
            syn::Expr::Path(p) => self.path_event(p, Some(args)),
            other => self.visit_expr(other),
        }
        for a in &c.args {
            self.visit_expr(a);
        }
    }

    fn visit_expr_method_call(&mut self, m: &'ast syn::ExprMethodCall) {
        self.visit_expr(&m.receiver);
        let args: Vec<Arg> = m.args.iter().map(|a| self.arg(a)).collect();
        self.out.events.push(Event {
            name: m.method.to_string(),
            callee: Callee::Method,
            args: Some(args),
            line: line(m.method.span()),
            enclosing_fn: self.fns.last().copied(),
        });
        for a in &m.args {
            self.visit_expr(a);
        }
    }

    fn visit_expr_path(&mut self, p: &'ast syn::ExprPath) {
        self.path_event(p, None);
    }
}
