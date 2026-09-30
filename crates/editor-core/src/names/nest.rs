//! **Every walk over a [`StableName`] carries its own stack.**
//!
//! A name nests one whole name per derivation level: a boolean's
//! survivor holds its operand's name ([`RoleSeg::FromA`]), a pattern
//! copy its master's ([`RoleSeg::Instance`]), an instantiated part's
//! entity its name in the part ([`RoleSeg::InPart`]), and so on down
//! the chain. Nothing bounds the depth: a part nested
//! [`MAX_DEPTH`](crate::eval::MAX_DEPTH) documents deep puts that many
//! `InPart` levels into every name its top instance carries, and a
//! document of a thousand booleans nests its survivors a thousand deep.
//! A walk that recursed once per level would need a stack proportional
//! to that depth, and the smallest stack a door runs on (the wasm32
//! build's one mebibyte) holds a few hundred levels at most.
//!
//! So no walk over a name recurses on its nesting. Each one here keeps
//! the names still to visit in a heap `Vec`, and the thread's stack
//! holds one level at a time:
//!
//! - [`Drop`] moves the path of every name it is the last holder of
//!   onto its own stack before that name goes, so no destructor runs
//!   inside another;
//! - [`Clone`], [`Debug`](core::fmt::Debug), [`PartialEq`],
//!   [`Hash`](core::hash::Hash) and [`Ord`] are the derived impls'
//!   answers, computed one level at a time;
//! - serde's form is the derived one, and the JSON doors
//!   ([`StableName::to_json`], [`StableName::from_json`], and
//!   `persist`'s save, load and canonical bytes) write and read it one
//!   level at a time.
//!
//! # One level at a time: the shallow walk
//!
//! [`RoleSeg`] has several dozen variants, and its derived impls are
//! the definition of what a name's rendering, order and wire form are.
//! They are kept, and run SHALLOW: while a walk over one level is
//! running, a name that level holds answers the walk without descending
//! (a hole in a rendering, "equal" in a comparison, nothing in a hash, an
//! empty copy in a clone), and the walk then visits the held names
//! itself, from its own stack, in the order the derived impl met them.
//! That order is declaration order, which [`RoleSeg::each_name`]
//! states once for every walk; the tests below check each walk against
//! a recursive reference at every variant.
//!
//! The shallow state is a thread-local ([`Shallow`]), set by the walk
//! for the span of one level and restored after it, including on
//! unwind.

use core::cmp::Ordering;
use core::fmt;
use std::cell::{Cell, RefCell};

use serde::ser::Error as _;

use super::role::{EntityKind, NameRef, Qualifier, RoleSeg, StableName};
use crate::node::RecipeNodeId;

/// Which derived walk is running one level at a time.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Walk {
    Debug,
    Eq,
    Ord,
    Hash,
    Clone,
    Ser,
    De,
}

/// Which nesting value the walk is over: a name, or a selector
/// pattern (`select`'s `NamePat`, which nests the same way).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Family {
    Name,
    Pattern,
}

thread_local! {
    static SHALLOW: Cell<Option<(Walk, Family)>> = const { Cell::new(None) };
    /// The answers a shallow [`Ord`] level met at its held names, in
    /// order: `Some` where the handle settled the pair itself (the
    /// same `Arc`, or one sealing walk's positions), `None` where the
    /// names have to be compared.
    static ORDERS: RefCell<Vec<Option<Ordering>>> = const { RefCell::new(Vec::new()) };
    /// The levels a shallow JSON level's holes stand for, in text
    /// order, and how many of them it has met.
    static HOLES: RefCell<(Vec<usize>, usize)> = const { RefCell::new((Vec::new(), 0)) };
    /// Whether a JSON door is running: names write and read their
    /// JSON one level at a time only inside one.
    static JSON_DOOR: Cell<bool> = const { Cell::new(false) };
}

/// Whether `walk` is running one level at a time over `family`.
pub(super) fn shallow(walk: Walk, family: Family) -> bool {
    SHALLOW.with(|s| s.get() == Some((walk, family)))
}

/// The span over which `walk` runs one level at a time; the state it
/// replaced comes back when this goes.
pub(super) struct Shallow(Option<(Walk, Family)>);

impl Shallow {
    pub(super) fn enter(walk: Walk, family: Family) -> Self {
        Self(SHALLOW.with(|s| s.replace(Some((walk, family)))))
    }
}

impl Drop for Shallow {
    fn drop(&mut self) {
        SHALLOW.with(|s| s.set(self.0));
    }
}

/// A held name's order, settled by its handle (`NameRef::cmp`),
/// recorded when a shallow [`Ord`] level asked for it.
pub(super) fn settled(order: Ordering) {
    if shallow(Walk::Ord, Family::Name) {
        ORDERS.with(|o| o.borrow_mut().push(Some(order)));
    }
}

/// Runs `f` as a JSON door: every name `f` writes or reads through
/// serde_json is written and read one level at a time, so its nesting
/// costs the thread's stack nothing. Outside a door a name's serde
/// impls are the derived ones, whatever the format.
pub(crate) fn json_door<R>(f: impl FnOnce() -> R) -> R {
    struct Restore(bool);
    impl Drop for Restore {
        fn drop(&mut self) {
            JSON_DOOR.with(|d| d.set(self.0));
        }
    }
    let _restore = Restore(JSON_DOOR.with(|d| d.replace(true)));
    f()
}

fn in_json_door() -> bool {
    JSON_DOOR.with(Cell::get)
}

/// A name a segment holds: through a shared handle, or by value (the
/// sets of [`RoleSeg::Merged`], [`RoleSeg::BandFace`] and the bands,
/// and a `SideOf` qualifier's partners).
#[derive(Clone, Copy)]
pub(crate) enum Held<'a> {
    Shared(&'a NameRef),
    Owned(&'a StableName),
}

impl<'a> Held<'a> {
    pub(crate) fn name(self) -> &'a StableName {
        match self {
            Held::Shared(r) => r.name(),
            Held::Owned(n) => n,
        }
    }
}

/// [`Held`], mutably.
pub(crate) enum HeldMut<'a> {
    Shared(&'a mut NameRef),
    Owned(&'a mut StableName),
}

impl RoleSeg {
    /// **Every name this segment holds, in declaration order** — the
    /// order every derived impl visits them in, and so the order every
    /// walk here pairs a level's held names with.
    ///
    /// EXHAUSTIVE, with no wildcard (the `walk_names` rule): a variant
    /// added to [`RoleSeg`] says here which names it holds, or stops
    /// the build.
    pub(crate) fn each_name<'a>(&'a self, f: &mut impl FnMut(Held<'a>)) {
        use Held::{Owned, Shared};
        match self {
            RoleSeg::FromA(n)
            | RoleSeg::FromB(n)
            | RoleSeg::FromMember { of: n, .. }
            | RoleSeg::SectionEdge { face: n, .. }
            | RoleSeg::SplitFragment { parent: n, .. }
            | RoleSeg::CrossingVertex { edge: n, .. }
            | RoleSeg::OnToolVertex { of: n, .. }
            | RoleSeg::FromTarget(n)
            | RoleSeg::BlendFace(n)
            | RoleSeg::CornerFace(n)
            | RoleSeg::BandTrim { edge: n, .. }
            | RoleSeg::BandFoot(n)
            | RoleSeg::BandCut(n)
            | RoleSeg::Inner(n)
            | RoleSeg::Rim(n)
            | RoleSeg::HoleRim { of: n, .. }
            | RoleSeg::InPart { of: n }
            | RoleSeg::Instance { of: n, .. } => f(Shared(n)),
            RoleSeg::Seam { a, b }
            | RoleSeg::TrimEdge {
                edge: a,
                support: b,
            }
            | RoleSeg::FootVertex {
                vertex: a,
                support: b,
            }
            | RoleSeg::EndArc { vertex: a, edge: b } => {
                f(Shared(a));
                f(Shared(b));
            }
            RoleSeg::Merged(set) | RoleSeg::BandFace(set) => set.iter().for_each(|n| f(Owned(n))),
            RoleSeg::BandCross { edge, band } | RoleSeg::BandSlit { edge, band } => {
                f(Shared(edge));
                band.iter().for_each(|n| f(Owned(n)));
            }
            RoleSeg::Fragment(Qualifier::SideOf(partners)) => {
                partners.iter().for_each(|(n, _)| f(Owned(n)));
            }
            RoleSeg::Fragment(Qualifier::OrderAlong { .. }) | super::name_free_seg!() => {}
        }
    }

    /// [`RoleSeg::each_name`], mutably: the same names in the same
    /// order.
    pub(crate) fn each_name_mut(&mut self, f: &mut impl FnMut(HeldMut<'_>)) {
        use HeldMut::{Owned, Shared};
        match self {
            RoleSeg::FromA(n)
            | RoleSeg::FromB(n)
            | RoleSeg::FromMember { of: n, .. }
            | RoleSeg::SectionEdge { face: n, .. }
            | RoleSeg::SplitFragment { parent: n, .. }
            | RoleSeg::CrossingVertex { edge: n, .. }
            | RoleSeg::OnToolVertex { of: n, .. }
            | RoleSeg::FromTarget(n)
            | RoleSeg::BlendFace(n)
            | RoleSeg::CornerFace(n)
            | RoleSeg::BandTrim { edge: n, .. }
            | RoleSeg::BandFoot(n)
            | RoleSeg::BandCut(n)
            | RoleSeg::Inner(n)
            | RoleSeg::Rim(n)
            | RoleSeg::HoleRim { of: n, .. }
            | RoleSeg::InPart { of: n }
            | RoleSeg::Instance { of: n, .. } => f(Shared(n)),
            RoleSeg::Seam { a, b }
            | RoleSeg::TrimEdge {
                edge: a,
                support: b,
            }
            | RoleSeg::FootVertex {
                vertex: a,
                support: b,
            }
            | RoleSeg::EndArc { vertex: a, edge: b } => {
                f(Shared(a));
                f(Shared(b));
            }
            RoleSeg::Merged(set) | RoleSeg::BandFace(set) => {
                set.iter_mut().for_each(|n| f(Owned(n)));
            }
            RoleSeg::BandCross { edge, band } | RoleSeg::BandSlit { edge, band } => {
                f(Shared(edge));
                band.iter_mut().for_each(|n| f(Owned(n)));
            }
            RoleSeg::Fragment(Qualifier::SideOf(partners)) => {
                partners.iter_mut().for_each(|(n, _)| f(Owned(n)));
            }
            RoleSeg::Fragment(Qualifier::OrderAlong { .. }) | super::name_free_seg!() => {}
        }
    }
}

impl StableName {
    /// The role path, owned (a name's fields cannot be moved out of
    /// it: its `Drop` is its own).
    pub(crate) fn into_path(mut self) -> super::RolePath {
        core::mem::take(&mut self.path)
    }

    /// Every name `self`'s path holds, in order (one level down).
    pub(crate) fn each_held<'a>(&'a self, f: &mut impl FnMut(Held<'a>)) {
        for seg in &self.path {
            seg.each_name(f);
        }
    }

    fn each_held_mut(&mut self, f: &mut impl FnMut(HeldMut<'_>)) {
        for seg in &mut self.path {
            seg.each_name_mut(f);
        }
    }

    fn held(&self) -> Vec<&StableName> {
        let mut out = Vec::new();
        self.each_held(&mut |h| out.push(h.name()));
        out
    }

    fn holds_by_value(&self) -> bool {
        let mut any = false;
        self.each_held(&mut |h| any |= matches!(h, Held::Owned(_)));
        any
    }
}

// ---------------------------------------------------------------
// Drop.
// ---------------------------------------------------------------

impl Drop for StableName {
    fn drop(&mut self) {
        // Every segment below this name whose names this is the last
        // holder of is moved here before it goes, so each name dropped
        // in the loop has an empty path and drops nothing of its own.
        let mut segs = core::mem::take(&mut self.path);
        while let Some(mut seg) = segs.pop() {
            seg.each_name_mut(&mut |h| match h {
                HeldMut::Shared(r) => {
                    if let Some(name) = r.get_mut() {
                        segs.append(&mut name.path);
                    }
                }
                HeldMut::Owned(name) => segs.append(&mut name.path),
            });
        }
    }
}

// ---------------------------------------------------------------
// Clone.
// ---------------------------------------------------------------

impl Clone for StableName {
    fn clone(&self) -> Self {
        let bare = |n: &StableName| StableName {
            kind: n.kind,
            node: n.node,
            path: Vec::new(),
        };
        if shallow(Walk::Clone, Family::Name) {
            return bare(self);
        }
        // A name held through a handle is shared, not copied, so only
        // the names held BY VALUE nest a copy; most names hold none.
        if !self.holds_by_value() {
            return StableName {
                kind: self.kind,
                node: self.node,
                path: self.path.clone(),
            };
        }
        let _shallow = Shallow::enter(Walk::Clone, Family::Name);
        // Level order: each source's by-value names follow it as one
        // run, whose first index `runs` records.
        let mut sources: Vec<&StableName> = vec![self];
        let mut copies: Vec<Option<StableName>> = Vec::new();
        let mut runs: Vec<usize> = Vec::new();
        let mut i = 0;
        while let Some(&source) = sources.get(i) {
            runs.push(sources.len());
            let mut copy = bare(source);
            copy.path.clone_from(&source.path);
            source.each_held(&mut |h| {
                if let Held::Owned(n) = h {
                    sources.push(n);
                }
            });
            copies.push(Some(copy));
            i += 1;
        }
        // Deepest first: every copy's held copies are finished before
        // it takes them in.
        for i in (0..copies.len()).rev() {
            let Some(mut copy) = copies.get_mut(i).and_then(Option::take) else {
                continue;
            };
            let mut next = runs.get(i).copied().unwrap_or(usize::MAX);
            copy.each_held_mut(&mut |h| {
                if let HeldMut::Owned(slot) = h {
                    if let Some(done) = copies.get_mut(next).and_then(Option::take) {
                        *slot = done;
                    }
                    next += 1;
                }
            });
            if let Some(cell) = copies.get_mut(i) {
                *cell = Some(copy);
            }
        }
        copies
            .into_iter()
            .next()
            .flatten()
            .unwrap_or_else(|| bare(self))
    }
}

// ---------------------------------------------------------------
// Debug.
// ---------------------------------------------------------------

/// What a held name renders as while its holder's level is rendered.
/// A NUL is never written by a derived rendering (a string or char
/// renders one escaped), so it cannot be mistaken for text.
const HOLE: char = '\0';

/// One level of a name as the derived impl renders it.
struct Level<'a>(&'a StableName);

impl fmt::Debug for Level<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StableName")
            .field("kind", &self.0.kind)
            .field("node", &self.0.node)
            .field("path", &self.0.path)
            .finish()
    }
}

impl fmt::Debug for StableName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if shallow(Walk::Debug, Family::Name) {
            return fmt::Write::write_char(f, HOLE);
        }
        render_nested(
            self,
            f,
            Family::Name,
            |n, alternate| {
                if alternate {
                    format!("{:#?}", Level(n))
                } else {
                    format!("{:?}", Level(n))
                }
            },
            StableName::held,
        )
    }
}

/// **The derived rendering of a nesting value, one level at a time.**
///
/// `level` renders one level with every held value a [`HOLE`];
/// `held` lists the held values in the order the rendering met them.
/// Each hole is filled with its value's own rendering, indented by the
/// hole's line under `{:#?}` as the derived impl's pad adapter would.
/// Only the alternate flag is carried to the levels.
pub(super) fn render_nested<'a, T>(
    root: &'a T,
    f: &mut fmt::Formatter<'_>,
    family: Family,
    level: impl Fn(&T, bool) -> String,
    held: impl Fn(&'a T) -> Vec<&'a T>,
) -> fmt::Result {
    struct Frame<'a, T> {
        text: String,
        at: usize,
        held: std::vec::IntoIter<&'a T>,
        indent: String,
    }
    let alternate = f.alternate();
    let open = |value: &'a T, indent: String| {
        let text = {
            let _shallow = Shallow::enter(Walk::Debug, family);
            level(value, alternate)
        };
        Frame {
            text,
            at: 0,
            held: held(value).into_iter(),
            indent,
        }
    };
    let mut stack = vec![open(root, String::new())];
    while let Some(top) = stack.last_mut() {
        let rest = top.text.get(top.at..).unwrap_or("");
        let Some(offset) = rest.find(HOLE) else {
            write_indented(f, rest, &top.indent)?;
            stack.pop();
            continue;
        };
        let hole = top.at + offset;
        write_indented(f, top.text.get(top.at..hole).unwrap_or(""), &top.indent)?;
        top.at = hole + HOLE.len_utf8();
        let before = top.text.get(..hole).unwrap_or("");
        let line = before
            .rfind('\n')
            .map_or(before, |nl| before.get(nl + 1..).unwrap_or(""));
        let lead = line.len() - line.trim_start_matches(' ').len();
        let indent = format!("{}{}", top.indent, " ".repeat(lead));
        // A level renders exactly the values it holds; a hole with no
        // value is a rendering this module did not write.
        let value = top.held.next().ok_or(fmt::Error)?;
        let frame = open(value, indent);
        stack.push(frame);
    }
    Ok(())
}

fn write_indented(f: &mut fmt::Formatter<'_>, text: &str, indent: &str) -> fmt::Result {
    if indent.is_empty() {
        return f.write_str(text);
    }
    for (i, line) in text.split('\n').enumerate() {
        if i > 0 {
            f.write_str("\n")?;
            f.write_str(indent)?;
        }
        f.write_str(line)?;
    }
    Ok(())
}

// ---------------------------------------------------------------
// PartialEq, Eq, Hash.
// ---------------------------------------------------------------

impl PartialEq for StableName {
    fn eq(&self, other: &Self) -> bool {
        if shallow(Walk::Eq, Family::Name) {
            return true;
        }
        let _shallow = Shallow::enter(Walk::Eq, Family::Name);
        let mut pairs = vec![(self, other)];
        while let Some((a, b)) = pairs.pop() {
            if core::ptr::eq(a, b) {
                continue;
            }
            // Held names compare equal here; equal levels hold equally
            // many, which are then compared pairwise.
            if a.kind != b.kind || a.node != b.node || a.path != b.path {
                return false;
            }
            pairs.extend(a.held().into_iter().zip(b.held()));
        }
        true
    }
}

impl Eq for StableName {}

impl core::hash::Hash for StableName {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        if shallow(Walk::Hash, Family::Name) {
            return;
        }
        // Each level's own fields, in pre-order: equal names feed the
        // same sequence, which is all `Hash` owes `Eq`.
        let _shallow = Shallow::enter(Walk::Hash, Family::Name);
        let mut names = vec![self];
        while let Some(n) = names.pop() {
            n.kind.hash(state);
            n.node.hash(state);
            n.path.hash(state);
            names.extend(n.held().into_iter().rev());
        }
    }
}

// ---------------------------------------------------------------
// Ord.
// ---------------------------------------------------------------

impl PartialOrd for StableName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StableName {
    /// The derived order — `kind`, then `node`, then the path, each
    /// lexicographic — one level at a time.
    ///
    /// A level is compared with its held names equal, which stops at
    /// the first difference among its own fields and records, in
    /// order, every held pair it met before stopping. The pairs are
    /// then compared in that order, and the first that differs is the
    /// answer; if none does, the level's own answer is. A difference
    /// anywhere settles the whole comparison, since every level above
    /// it had compared equal up to the pair holding it.
    fn cmp(&self, other: &Self) -> Ordering {
        if shallow(Walk::Ord, Family::Name) {
            ORDERS.with(|o| o.borrow_mut().push(None));
            return Ordering::Equal;
        }
        if core::ptr::eq(self, other) {
            return Ordering::Equal;
        }
        let _shallow = Shallow::enter(Walk::Ord, Family::Name);
        let mut stack = vec![OrdLevel::of(self, other)];
        while let Some(top) = stack.last_mut() {
            match top.pairs.next() {
                Some((_, Some(Ordering::Equal))) => {}
                Some((_, Some(order))) => return order,
                Some(((a, b), None)) => {
                    let frame = OrdLevel::of(a, b);
                    stack.push(frame);
                }
                None => {
                    if top.own != Ordering::Equal {
                        return top.own;
                    }
                    stack.pop();
                }
            }
        }
        Ordering::Equal
    }
}

/// One level of an [`Ord`] walk: the held pairs its own comparison met,
/// each with what its handle settled, and its own answer.
struct OrdLevel<'a> {
    pairs: std::vec::IntoIter<((&'a StableName, &'a StableName), Option<Ordering>)>,
    own: Ordering,
}

impl<'a> OrdLevel<'a> {
    /// `a` against `b` with their held names compared equal (the walk
    /// is shallow while this runs).
    fn of(a: &'a StableName, b: &'a StableName) -> Self {
        ORDERS.with(|o| o.borrow_mut().clear());
        let own = a
            .kind
            .cmp(&b.kind)
            .then_with(|| a.node.cmp(&b.node))
            .then_with(|| a.path.cmp(&b.path));
        let met = ORDERS.with(|o| core::mem::take(&mut *o.borrow_mut()));
        let pairs: Vec<_> = a.held().into_iter().zip(b.held()).zip(met).collect();
        Self {
            pairs: pairs.into_iter(),
            own,
        }
    }
}

// ---------------------------------------------------------------
// Serde.
// ---------------------------------------------------------------

/// The derived wire form of one level, written from borrowed fields.
#[derive(serde::Serialize)]
#[serde(rename = "StableName")]
struct WireOut<'a> {
    kind: &'a EntityKind,
    node: &'a RecipeNodeId,
    path: &'a [RoleSeg],
}

impl<'a> WireOut<'a> {
    fn of(name: &'a StableName) -> Self {
        Self {
            kind: &name.kind,
            node: &name.node,
            path: &name.path,
        }
    }
}

/// The derived wire form of one level, read.
#[derive(serde::Deserialize)]
#[serde(rename = "StableName", deny_unknown_fields)]
struct WireIn {
    kind: EntityKind,
    node: RecipeNodeId,
    path: Vec<RoleSeg>,
}

impl From<WireIn> for StableName {
    fn from(wire: WireIn) -> Self {
        StableName {
            kind: wire.kind,
            node: wire.node,
            path: wire.path,
        }
    }
}

/// What a held name writes as while its holder's level is written: a
/// string holding one NUL, which no name writes (a name holds no
/// text).
const HOLE_JSON: &str = "\"\\u0000\"";

impl serde::Serialize for StableName {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        if shallow(Walk::Ser, Family::Name) {
            return ser.serialize_str("\0");
        }
        if in_json_door() {
            let text = write_json(self).map_err(S::Error::custom)?;
            let raw = serde_json::value::RawValue::from_string(text).map_err(S::Error::custom)?;
            return serde::Serialize::serialize(&raw, ser);
        }
        serde::Serialize::serialize(&WireOut::of(self), ser)
    }
}

impl<'de> serde::Deserialize<'de> for StableName {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        if shallow(Walk::De, Family::Name) {
            return de.deserialize_any(HoleVisitor);
        }
        if in_json_door() {
            let raw = <Box<serde_json::value::RawValue> as serde::Deserialize>::deserialize(de)?;
            return read_json(raw.get()).map_err(D::Error::custom);
        }
        <WireIn as serde::Deserialize>::deserialize(de).map(StableName::from)
    }
}

/// **A name's compact JSON, one level at a time**: each level written
/// in the derived form with its held names as [`HOLE_JSON`], and each
/// hole filled with its name's own text, from this walk's stack.
fn write_json(root: &StableName) -> Result<String, serde_json::Error> {
    struct Frame<'a> {
        text: String,
        at: usize,
        held: std::vec::IntoIter<&'a StableName>,
    }
    fn open(name: &StableName) -> Result<Frame<'_>, serde_json::Error> {
        Ok(Frame {
            text: serde_json::to_string(&WireOut::of(name))?,
            at: 0,
            held: name.held().into_iter(),
        })
    }
    let _shallow = Shallow::enter(Walk::Ser, Family::Name);
    let mut out = String::new();
    let mut stack = vec![open(root)?];
    while let Some(top) = stack.last_mut() {
        let rest = top.text.get(top.at..).unwrap_or("");
        let Some(offset) = rest.find(HOLE_JSON) else {
            out.push_str(rest);
            stack.pop();
            continue;
        };
        out.push_str(rest.get(..offset).unwrap_or(""));
        top.at += offset + HOLE_JSON.len();
        let Some(name) = top.held.next() else {
            return Err(serde::ser::Error::custom(
                "a stable name's level wrote more held names than it holds",
            ));
        };
        let frame = open(name)?;
        stack.push(frame);
    }
    Ok(out)
}

/// What a shallow JSON level reads where it holds a name: the hole
/// [`read_json`] cut there, answered by a placeholder whose `node`
/// is the index of the level the hole stands for.
struct HoleVisitor;

impl<'de> serde::de::Visitor<'de> for HoleVisitor {
    type Value = StableName;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("struct StableName")
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<StableName, E> {
        let index = (v == "\0")
            .then(|| {
                HOLES.with(|holes| {
                    let (levels, met) = &mut *holes.borrow_mut();
                    let index = levels.get(*met).copied();
                    *met += 1;
                    index
                })
            })
            .flatten()
            .ok_or_else(|| E::invalid_type(serde::de::Unexpected::Str(v), &self))?;
        Ok(StableName {
            kind: EntityKind::Body,
            node: RecipeNodeId(index as u64),
            path: Vec::new(),
        })
    }

    // An object [`scan`] did not cut out holds none of a name's keys,
    // so the derived form refuses it — and refuses it here, where the
    // refusal is the derived one, without reading any deeper.
    fn visit_map<A: serde::de::MapAccess<'de>>(self, map: A) -> Result<StableName, A::Error> {
        use serde::de::Error as _;
        <WireIn as serde::Deserialize>::deserialize(serde::de::value::MapAccessDeserializer::new(
            map,
        ))?;
        Err(A::Error::custom(
            "a stable name's text holds a name where none was cut",
        ))
    }
}

/// A JSON object in a name's text: where it opens and closes, the
/// object enclosing it, and whether it holds any of a name's keys.
struct Object {
    open: usize,
    close: usize,
    enclosing: Option<usize>,
    named: bool,
}

/// **Every object in `text` that is a name, found in one pass.**
///
/// An object is taken for a name when it holds any of a name's three
/// keys (`kind`, `node`, `path`): no segment's form has one, so every
/// name holds them and nothing else does. Returns the objects and, for
/// each, the index of the name it lies directly inside (the root, at
/// index 0, for one inside no other).
///
/// # Errors
///
/// A string holding a NUL anywhere in the text: no name writes one, and
/// it is the hole [`read_json`] cuts.
fn scan(text: &str) -> Result<(Vec<Object>, Vec<usize>), String> {
    let bytes = text.as_bytes();
    let mut objects: Vec<Object> = Vec::new();
    // The open containers: `Some(object)` or `None` for an array.
    let mut open: Vec<Option<usize>> = Vec::new();
    let mut i = 0;
    while let Some(&b) = bytes.get(i) {
        match b {
            b'"' => {
                let start = i;
                i += 1;
                while let Some(&c) = bytes.get(i) {
                    match c {
                        b'\\' => i += 2,
                        b'"' => break,
                        _ => i += 1,
                    }
                }
                let token = text.get(start..=i).unwrap_or("");
                if token.contains("\\u0000") {
                    return Err("a stable name's text holds a NUL, which no name writes".to_owned());
                }
                let mut after = i + 1;
                while bytes.get(after).is_some_and(u8::is_ascii_whitespace) {
                    after += 1;
                }
                if bytes.get(after) == Some(&b':')
                    && let Some(Some(object)) = open.last()
                {
                    let key = if token.contains('\\') {
                        serde_json::from_str::<String>(token).map_err(|e| unplaced(&e))?
                    } else {
                        token.trim_matches('"').to_owned()
                    };
                    if matches!(key.as_str(), "kind" | "node" | "path")
                        && let Some(o) = objects.get_mut(*object)
                    {
                        o.named = true;
                    }
                }
            }
            b'{' => {
                let enclosing = open.iter().rev().find_map(|c| *c);
                open.push(Some(objects.len()));
                objects.push(Object {
                    open: i,
                    close: i,
                    enclosing,
                    named: false,
                });
            }
            b'[' => open.push(None),
            b'}' | b']' => {
                if let Some(Some(object)) = open.pop()
                    && let Some(o) = objects.get_mut(object)
                {
                    o.close = i;
                }
            }
            _ => {}
        }
        i += 1;
    }
    // The name each object lies directly inside: objects open in text
    // order, so an enclosing object's answer is known before its own.
    let mut within: Vec<Option<usize>> = Vec::with_capacity(objects.len());
    for o in &objects {
        let up = o.enclosing.and_then(|e| {
            let named = objects.get(e).is_some_and(|e| e.named);
            if named {
                Some(e)
            } else {
                within.get(e).copied().flatten()
            }
        });
        within.push(up);
    }
    Ok((
        objects,
        within.into_iter().map(|w| w.unwrap_or(0)).collect(),
    ))
}

/// **A name read from its JSON text, one level at a time.**
///
/// One pass over the text finds every name in it ([`scan`]). Each is
/// then read in the derived form from its own text with every name it
/// holds cut out and a hole in its place ([`HoleVisitor`]), so reading
/// a level reads that level's text and no deeper; the levels are then
/// assembled from the deepest up.
fn read_json(text: &str) -> Result<StableName, String> {
    let _shallow = Shallow::enter(Walk::De, Family::Name);
    if !text.starts_with('{') {
        // Not an object, so not a name: the derived form says so.
        let wire: WireIn = serde_json::from_str(text).map_err(|e| unplaced(&e))?;
        return Ok(wire.into());
    }
    let (objects, within) = scan(text)?;
    // The root is a level whether or not it holds a name's key (the
    // derived form refuses it if it does not); every other name is one.
    let named: Vec<usize> = (0..objects.len())
        .filter(|&o| o == 0 || objects.get(o).is_some_and(|o| o.named))
        .collect();
    let mut level_of = vec![usize::MAX; objects.len()];
    for (level, &o) in named.iter().enumerate() {
        if let Some(slot) = level_of.get_mut(o) {
            *slot = level;
        }
    }
    let mut holds: Vec<Vec<usize>> = vec![Vec::new(); named.len()];
    for &o in named.iter().skip(1) {
        let parent = within.get(o).and_then(|&p| level_of.get(p)).copied();
        let child = level_of.get(o).copied();
        if let (Some(parent), Some(child)) = (parent, child)
            && let Some(list) = holds.get_mut(parent)
        {
            list.push(child);
        }
    }
    let mut levels: Vec<Option<StableName>> = Vec::with_capacity(named.len());
    for (level, &o) in named.iter().enumerate() {
        let object = objects.get(o).ok_or(MISPLACED)?;
        let held = holds.get(level).cloned().unwrap_or_default();
        let mut cut = String::new();
        let mut at = object.open;
        for &child in &held {
            let span = named
                .get(child)
                .and_then(|&c| objects.get(c))
                .ok_or(MISPLACED)?;
            cut.push_str(text.get(at..span.open).ok_or(MISPLACED)?);
            cut.push_str(HOLE_JSON);
            at = span.close + 1;
        }
        cut.push_str(text.get(at..=object.close).ok_or(MISPLACED)?);
        HOLES.with(|holes| *holes.borrow_mut() = (held, 0));
        let wire: WireIn = serde_json::from_str(&cut).map_err(|e| unplaced(&e))?;
        levels.push(Some(wire.into()));
    }
    for i in (0..levels.len()).rev() {
        let mut name = levels.get_mut(i).and_then(Option::take).ok_or(MISPLACED)?;
        let mut missing = false;
        name.each_held_mut(&mut |h| {
            let (slot, index) = match h {
                HeldMut::Shared(r) => {
                    let index = r.node.0;
                    (r.get_mut(), index)
                }
                HeldMut::Owned(n) => {
                    let index = n.node.0;
                    (Some(n), index)
                }
            };
            let done = usize::try_from(index)
                .ok()
                .and_then(|j| levels.get_mut(j))
                .and_then(Option::take);
            match (slot, done) {
                (Some(slot), Some(done)) => *slot = done,
                _ => missing = true,
            }
        });
        if missing {
            return Err(MISPLACED.to_owned());
        }
        if let Some(cell) = levels.get_mut(i) {
            *cell = Some(name);
        }
    }
    levels
        .into_iter()
        .next()
        .flatten()
        .ok_or_else(|| MISPLACED.to_owned())
}

/// A level's held name that did not come back where it was met: a
/// defect of this module, never of the text.
const MISPLACED: &str = "a stable name's nested text was not read back where it was met";

/// A level's refusal without the level's own line and column: the door
/// that read the whole text places it.
fn unplaced(e: &serde_json::Error) -> String {
    let text = e.to_string();
    let suffix = format!(" at line {} column {}", e.line(), e.column());
    text.strip_suffix(&suffix).unwrap_or(&text).to_owned()
}

impl StableName {
    /// **This name's JSON text**: its one serialization, compact,
    /// written one nesting level at a time so a name of any depth
    /// writes on any stack.
    ///
    /// # Errors
    ///
    /// None in practice: a name holds no value JSON cannot carry. The
    /// writer's own error is passed through rather than assumed away.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        json_door(|| serde_json::to_string(self))
    }

    /// **A name read from its JSON text**, one nesting level at a time,
    /// so a name of any depth reads on any stack. The inverse of
    /// [`StableName::to_json`]; any spelling of the same JSON value (a
    /// saved document's pretty-printed one included) reads the same.
    ///
    /// # Errors
    ///
    /// Text that is not JSON, or JSON that is not a name.
    pub fn from_json(text: &str) -> Result<Self, serde_json::Error> {
        json_door(|| serde_json::from_str(text))
    }
}
