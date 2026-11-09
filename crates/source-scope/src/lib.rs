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
//! # How a bare name resolves
//!
//! Innermost scope first, as Rust reads it: a closure's parameters over
//! its body; a match arm's, an `if let`'s, a `while let`'s and a `for`'s
//! pattern over what it governs; a `let` from the statement after it to
//! its block's end; a block's own items (`fn`, `use`, `const`, `static`)
//! over the whole block; a fn's parameters over its body; then the
//! module's items and imports. A fn item does not see the locals of a fn
//! around it, and a `mod` does not see its parent's items except through
//! an import: a `use super::*` in an inline `mod` resolves a name against
//! the parent module in the same file, and any other glob, or one whose
//! parent is outside the file, is reported as the paths it reads
//! ([`Bound::Glob`]). A method or an associated fn in
//! an `impl` or a `trait` binds no bare name, and neither does a struct
//! field.
//!
//! # What it cannot read, stated
//!
//! - **What a macro expands to.** A macro's arguments are read as a
//!   comma-separated list of expressions where they parse as one (most
//!   `assert!`, `format!`-family and `debug_assert!` calls); where they do
//!   not, and in a `macro_rules!` body, every identifier is an event of
//!   kind [`Callee::Macro`], so a guard can list it rather than drop it.
//! - **What a glob brings in.** A glob is reported as the paths it reads
//!   ([`Bound::Glob`]); whether the module behind it defines the name is
//!   not known here.
//! - **Types and traits.** A name is resolved in the value namespace only;
//!   a method call's receiver type is not inferred.

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
    /// type's last segment), if there is one.
    Path {
        /// The qualifier.
        qualifier: Option<String>,
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
    /// An explicit `use` leaf.
    Use {
        /// Its path.
        path: Vec<String>,
        /// Its `as` name.
        alias: Option<String>,
    },
    /// No explicit binding; the innermost scope with globs has these.
    Glob(Vec<Vec<String>>),
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
    r.frames.push(Frame::new(Kind::Module));
    r.declare_items(&file.items);
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

#[derive(Debug)]
struct Frame {
    kind: Kind,
    names: HashMap<String, Bound>,
    globs: Vec<Vec<String>>,
}

impl Frame {
    fn new(kind: Kind) -> Self {
        Self {
            kind,
            names: HashMap::new(),
            globs: Vec::new(),
        }
    }
}

#[derive(Default)]
struct Reader {
    out: Source,
    frames: Vec<Frame>,
    /// The indices in `out.fns` of the fn items around the point read.
    fns: Vec<usize>,
    in_impl: usize,
    in_test: usize,
}

fn line(span: Span) -> usize {
    span.start().line
}

fn is_cfg_test(attrs: &[syn::Attribute]) -> bool {
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

    /// The binding `name` resolves to here.
    fn lookup(&self, name: &str) -> Option<Bound> {
        let mut locals = true;
        for (depth, frame) in self.frames.iter().enumerate().rev() {
            if let Some(b) = frame.names.get(name) {
                let item = matches!(b, Bound::Item | Bound::Use { .. });
                if locals || item {
                    return Some(b.clone());
                }
            }
            if !frame.globs.is_empty() && matches!(frame.kind, Kind::Module | Kind::Block) {
                // A module's `use super::*` reaches its parent's items, and
                // the parent is in this file where an outer `mod` frame is
                // open: resolve the name there, as Rust does.
                if frame.kind == Kind::Module
                    && frame.globs.iter().any(|g| g == &["super", "*"])
                    && let Some(parent) = self.frames[..depth]
                        .iter()
                        .rev()
                        .find(|f| f.kind == Kind::Module)
                    && let Some(b @ (Bound::Item | Bound::Use { .. })) = parent.names.get(name)
                {
                    return Some(b.clone());
                }
                return Some(Bound::Glob(frame.globs.clone()));
            }
            match frame.kind {
                Kind::Module => return None,
                Kind::Fn => locals = false,
                Kind::Block | Kind::Local => {}
            }
        }
        None
    }

    /// Declares a scope's items (`fn`, `const`, `static`, `use`) on the
    /// open frame, before the scope is read: an item is in scope over all
    /// of it. Records the `use` leaves.
    fn declare_items(&mut self, items: &[syn::Item]) {
        for item in items {
            match item {
                syn::Item::Fn(f) => self.bind(f.sig.ident.to_string(), Bound::Item),
                syn::Item::Const(c) => self.bind(c.ident.to_string(), Bound::Item),
                syn::Item::Static(s) => self.bind(s.ident.to_string(), Bound::Item),
                syn::Item::Use(u) => {
                    let mut leaves = Vec::new();
                    use_leaves(&u.tree, &mut Vec::new(), &mut leaves);
                    for (path, alias) in leaves {
                        if path.last().map(String::as_str) == Some("*") {
                            self.top().globs.push(path.clone());
                        } else if let Some(name) = alias.clone().or_else(|| path.last().cloned())
                            && name != "_"
                        {
                            self.bind(
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
            }
        } else if p.path.segments.len() > 1 {
            let n = p.path.segments.len();
            Callee::Path {
                qualifier: Some(p.path.segments[n - 2].ident.to_string()),
            }
        } else if p.path.leading_colon.is_some() {
            Callee::Path { qualifier: None }
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
        self.frames.push(Frame::new(Kind::Module));
        self.declare_items(items);
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
