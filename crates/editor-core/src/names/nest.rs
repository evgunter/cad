//! **Every walk over a [`StableName`] carries its own stack.**
//!
//! A name nests one whole name per derivation level: a boolean's
//! survivor holds its operand's name ([`RoleSeg::FromA`]), a pattern
//! copy its master's ([`RoleSeg::Instance`]), an instantiated part's
//! entity its name in the part ([`RoleSeg::InPart`]), and so on down
//! the chain. Nothing bounds the depth: a part nested
//! [`MAX_DEPTH`](crate::eval::parts::MAX_DEPTH) documents deep puts that many
//! `InPart` levels into every name its top instance carries, and a
//! document of a thousand booleans nests its survivors a thousand deep.
//! A walk that recursed once per level would need a stack proportional
//! to that depth, and the smallest stack a door runs on (the wasm32
//! build's one mebibyte) holds a few hundred levels at most.
//!
//! So the walks here keep the names still to visit in a heap `Vec`,
//! and the thread's stack holds one level at a time:
//!
//! - [`Drop`] moves the path of every name it is the last holder of
//!   onto its own stack before that name goes, so no destructor runs
//!   inside another;
//! - [`Clone`] and [`Debug`](core::fmt::Debug) are the derived impls'
//!   answers, and [`PartialEq`] and [`Ord`] the derived order, computed
//!   one level at a time; [`Hash`](core::hash::Hash) feeds a sequence
//!   that equal names feed alike, which is all it owes [`Eq`];
//! - serde's form is the derived one, and the JSON doors
//!   ([`write_door`], [`read_door`], [`StableName::to_json`],
//!   [`StableName::from_json`], and `persist`'s save, load and
//!   canonical bytes) write and read it one level at a time.
//!
//! Two walks still recurse once per level, each where it cannot meet a
//! name deeper than its reader allows: serde outside a JSON door, which
//! is the derived form through whatever format drives it (a name
//! through a non-JSON serializer is filed), and a name's level read in
//! the seq form no build writes, which serde_json's own recursion
//! limit bounds.
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
//! a copy of the derived types at every variant.
//!
//! # The thread-locals, framed
//!
//! A shallow level is told apart from a whole walk by thread-local
//! state, since the derived impls it runs take no argument of this
//! module's. That is an install/record/take scaffold, the family
//! `persist/refusal.rs` names, and it meets the same bar
//! (`docs/PERF-SCAN-2026-08.md` §2.4):
//!
//! - **Installed by a guard, never by hand.** [`Shallow`] (the walk
//!   running one level at a time), the native budget's `Out`, the
//!   writing door's and the reading door's guards each put back what
//!   they replaced in `Drop`, so a walk that panics or returns early
//!   leaves nothing armed: the next walk on the thread starts clean.
//!   The two values a level records into (`ORDERS`, `HOLES`) are reset
//!   by the walk at the start of each level it reads them for and taken
//!   at its end, so nothing one level leaves reaches the next.
//! - **Re-entrancy composes.** Each guard restores the state it found,
//!   so a walk begun inside another (a `Debug` of one name from inside
//!   a `Hasher` another name's hash is feeding) runs whole and hands the
//!   outer walk its state back; a read door opened inside another has
//!   its own frame. One case does not compose, and is stated at
//!   [`Hash`](core::hash::Hash): a name hashed from inside a `Hasher`
//!   while another's shallow hash is running feeds only its own level.
//! - **Thread-confined by the type.** Every guard is `!Send`.
//! - **Visible at both ends.** The recorders are the derived impls'
//!   holes ([`StableName`]'s impls below and `select`'s `NamePat`), and
//!   the harvesters are the walks in this file that open the guards.
//!
//! The recorded values are the shallow `Ord`'s met pairs (`ORDERS`), a
//! JSON level's holes (`HOLES`), and a read door's names and fault
//! (`READS`); each is taken by the walk that set it up.

use core::cmp::Ordering;
use core::fmt;
use core::marker::PhantomData;
use std::cell::{Cell, RefCell};

use serde::ser::Error as _;

use super::role::{EntityKind, NameRef, Qualifier, RoleSeg, StableName};
use crate::node::RecipeNodeId;
use crate::persist::jsontext;

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
    /// The holes of the JSON level being read ([`Holes`]).
    static HOLES: RefCell<Holes> = const {
        RefCell::new(Holes {
            marker: 0,
            met: Vec::new(),
            inline: Vec::new(),
        })
    };
    /// Whether a writing door is running ([`write_door`]).
    static WRITING: Cell<bool> = const { Cell::new(false) };
    /// The read doors open on this thread, innermost last
    /// ([`read_door`]).
    static READS: RefCell<Vec<ReadFrame>> = const { RefCell::new(Vec::new()) };
}

/// Makes a guard `!Send`: the state it restores is this thread's.
type ThisThread = PhantomData<*const ()>;

/// Whether `walk` is running one level at a time over `family`.
pub(super) fn shallow(walk: Walk, family: Family) -> bool {
    SHALLOW.with(|s| s.get() == Some((walk, family)))
}

/// The span over which `walk` runs one level at a time; the state it
/// replaced comes back when this goes.
pub(super) struct Shallow(Option<(Walk, Family)>, ThisThread);

impl Shallow {
    pub(super) fn enter(walk: Walk, family: Family) -> Self {
        Self(
            SHALLOW.with(|s| s.replace(Some((walk, family)))),
            PhantomData,
        )
    }
}

impl Drop for Shallow {
    fn drop(&mut self) {
        SHALLOW.with(|s| s.set(self.0));
    }
}

/// **How many levels `Eq`, `Ord` and `Hash` recurse natively** before
/// they go on from their own stack. Native recursion allocates nothing,
/// and nearly every comparison a name table makes is decided within a
/// few levels, so the walks take it while it is cheap. A native level
/// costs about 1.5 KiB of stack in a dev build, so this many take about
/// 128 KiB, which the smallest stack a door runs on holds with room to
/// spare (`the_native_levels_fit_the_smallest_stack`). Both routes give
/// the same answer, and `Hash` feeds the same sequence on both.
const NATIVE_LEVELS: u32 = 64;

thread_local! {
    /// The native levels the walk on this thread is inside.
    static NATIVE: Cell<u32> = const { Cell::new(0) };
}

fn native_levels() -> u32 {
    #[cfg(test)]
    if let Some(levels) = tests::native_levels() {
        return levels;
    }
    NATIVE_LEVELS
}

/// `f`, one native level deeper, while the budget lasts; `None` past
/// it.
fn natively<R>(f: impl FnOnce() -> R) -> Option<R> {
    struct Out(u32, ThisThread);
    impl Drop for Out {
        fn drop(&mut self) {
            NATIVE.with(|n| n.set(self.0));
        }
    }
    let depth = NATIVE.with(Cell::get);
    if depth >= native_levels() {
        return None;
    }
    let _out = Out(depth, PhantomData);
    NATIVE.with(|n| n.set(depth + 1));
    Some(f())
}

/// A held name's order, settled by its handle (`NameRef::cmp`),
/// recorded when a shallow [`Ord`] level asked for it.
pub(super) fn settled(order: Ordering) {
    if shallow(Walk::Ord, Family::Name) {
        ORDERS.with(|o| o.borrow_mut().push(Some(order)));
    }
}

/// **Runs `f` as a writing door**: every name `f` writes through
/// serde_json is written one level at a time, so its nesting costs the
/// thread's stack nothing.
///
/// Inside a door a name serializes as its compact JSON text, handed to
/// the serializer as serde_json's raw value. So what `f` writes depends
/// on the door, and every caller wants that: a name written by
/// `to_string_pretty` inside one comes out compact (save lays the whole
/// body out afterwards, `persist::jsontext`), and `serde_json::to_value`
/// inside one reads the text back with the reader's recursion limit, so
/// a deep name refuses there. Outside a door a name's serde impls are
/// the derived ones, whatever the format.
pub(crate) fn write_door<R>(f: impl FnOnce() -> R) -> R {
    struct Restore(bool, ThisThread);
    impl Drop for Restore {
        fn drop(&mut self) {
            WRITING.with(|d| d.set(self.0));
        }
    }
    let _restore = Restore(WRITING.with(|d| d.replace(true)), PhantomData);
    f()
}

/// One open read door: where the text its reader reads lies in memory,
/// the text as written where the reader reads a blanked copy, and what
/// the door has seen.
struct ReadFrame {
    /// The address of the text's first byte and its length, so a name
    /// the reader meets is placed by the address of its raw text.
    base: usize,
    len: usize,
    /// The text as written, where the reader reads a copy of the same
    /// length with bytes blanked (`persist::nesting`'s pruned body): a
    /// name is read from here, at the place the reader met it.
    written: Option<String>,
    seen: Read,
}

/// **What a read door saw**: the offset of every name its reader read
/// (the byte of its opening brace), and the first name that refused.
#[derive(Default)]
pub(crate) struct Read {
    pub(crate) names: std::collections::BTreeSet<usize>,
    pub(crate) fault: Option<NameFault>,
}

/// **A name's text refused where it is written**: the byte of the door's
/// text the reader stood at, and the reader's own classification and
/// words, without its place.
pub(crate) struct NameFault {
    pub(crate) at: usize,
    pub(crate) category: serde_json::error::Category,
    pub(crate) message: String,
}

/// **Runs `f`, which reads `text` through serde_json, as a read door**,
/// and answers what `f` returned beside what the door saw ([`Read`]).
///
/// Every name `f` reads is taken as serde_json's raw value, borrowed
/// from `text` (so `f` reads `text` itself, through a borrowing
/// reader), and read one level at a time from its own text: its nesting
/// costs the thread's stack nothing, and a refusal inside it is placed
/// where it is written, in the derived form's words. `written` is the
/// text as written when `text` is a copy of it with bytes blanked: a
/// name is then read from `written`, at the offset the reader met it.
pub(crate) fn read_door<R>(text: &str, written: Option<&str>, f: impl FnOnce() -> R) -> (R, Read) {
    /// Pops the door's frame however `f` exits; `finish` takes what it
    /// saw first.
    struct Door(bool, ThisThread);
    impl Door {
        fn finish(mut self) -> Read {
            self.0 = false;
            READS.with(|r| r.borrow_mut().pop()).map_or_else(
                || unreachable!("a read door's frame is popped by its own guard alone"),
                |frame| frame.seen,
            )
        }
    }
    impl Drop for Door {
        fn drop(&mut self) {
            if self.0 {
                READS.with(|r| r.borrow_mut().pop());
            }
        }
    }
    READS.with(|r| {
        r.borrow_mut().push(ReadFrame {
            base: text.as_ptr() as usize,
            len: text.len(),
            written: written.map(str::to_owned),
            seen: Read::default(),
        });
    });
    let door = Door(true, PhantomData);
    let out = f();
    (out, door.finish())
}

fn reading() -> bool {
    READS.with(|r| !r.borrow().is_empty())
}

/// A name a segment holds: through a shared handle, or by value (the
/// sets of [`RoleSeg::Merged`], [`RoleSeg::BandFace`] and the bands,
/// and a `Borders` qualifier's walls).
#[derive(Clone, Copy)]
pub(crate) enum Hold<'a> {
    Shared(&'a NameRef),
    Owned(&'a StableName),
}

impl<'a> Hold<'a> {
    pub(crate) fn name(self) -> &'a StableName {
        match self {
            Hold::Shared(r) => r.name(),
            Hold::Owned(n) => n,
        }
    }
}

/// [`Hold`], mutably.
pub(crate) enum HoldMut<'a> {
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
    pub(crate) fn each_name<'a>(&'a self, f: &mut impl FnMut(Hold<'a>)) {
        use Hold::{Owned, Shared};
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
            RoleSeg::Fragment(
                Qualifier::Borders(walls) | Qualifier::Keeps(walls) | Qualifier::Ends(walls),
            ) => walls.iter().for_each(|n| f(Owned(n))),
            RoleSeg::Fragment(Qualifier::OrderAlong { .. }) | super::name_free_seg!() => {}
        }
    }

    /// [`RoleSeg::each_name`], mutably: the same names in the same
    /// order.
    pub(crate) fn each_name_mut(&mut self, f: &mut impl FnMut(HoldMut<'_>)) {
        use HoldMut::{Owned, Shared};
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
            RoleSeg::Fragment(
                Qualifier::Borders(walls) | Qualifier::Keeps(walls) | Qualifier::Ends(walls),
            ) => {
                walls.iter_mut().for_each(|n| f(Owned(n)));
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
    pub(crate) fn each_held<'a>(&'a self, f: &mut impl FnMut(Hold<'a>)) {
        for seg in &self.path {
            seg.each_name(f);
        }
    }

    fn each_held_mut(&mut self, f: &mut impl FnMut(HoldMut<'_>)) {
        for seg in &mut self.path {
            seg.each_name_mut(f);
        }
    }

    fn held(&self) -> Vec<&StableName> {
        let mut out = Vec::new();
        self.each_held(&mut |h| out.push(h.name()));
        out
    }

    fn holds_names(&self) -> bool {
        let mut any = false;
        self.each_held(&mut |_| any = true);
        any
    }

    fn holds_by_value(&self) -> bool {
        let mut any = false;
        self.each_held(&mut |h| any |= matches!(h, Hold::Owned(_)));
        any
    }

    /// The names `self`'s path holds by value, in order: the ones a copy
    /// of it copies.
    fn held_by_value(&self) -> Vec<&StableName> {
        let mut out = Vec::new();
        self.each_held(&mut |h| {
            if let Hold::Owned(n) = h {
                out.push(n);
            }
        });
        out
    }
}

// ---------------------------------------------------------------
// The walks that rebuild a name from its held names' answers.
// ---------------------------------------------------------------

/// The address of a name: what [`Kept`] keys an answer by. A name held
/// through one shared handle in two places is one address, answered
/// once.
pub(super) fn address(name: &StableName) -> usize {
    core::ptr::from_ref(name) as usize
}

/// Why one level of a [`Descent`] stopped: it refused, or it asked for
/// a held name not answered yet.
pub(super) enum Stopped<'s, E> {
    Refused(E),
    Needs(&'s StableName),
}

impl<E> From<E> for Stopped<'_, E> {
    fn from(e: E) -> Self {
        Stopped::Refused(e)
    }
}

/// The answers a [`Descent`] has kept, by the address of the held name
/// each answers.
pub(super) struct Kept<T>(std::collections::BTreeMap<usize, T>);

impl<T> Kept<T> {
    /// `name`'s answer, if it is kept.
    pub(super) fn get(&self, name: &StableName) -> Option<&T> {
        #[cfg(test)]
        tests::asked();
        self.0.get(&address(name))
    }

    /// `name`'s answer, or the stop that asks for it first.
    pub(super) fn need<'s, E>(&self, name: &'s StableName) -> Result<&T, Stopped<'s, E>> {
        self.get(name).ok_or(Stopped::Needs(name))
    }
}

/// **A walk that answers a name from the answers of the names it
/// holds**, one level at a time: the rewrite through a
/// `SegRewrite` (`role`'s `rewrite_path`) and the union's collapse
/// (`emit_union`'s `collapse`). [`descend`] drives it.
pub(super) trait Descent<'s> {
    /// A level's own answer: what the root answers.
    type Level;
    /// What a held name's level is kept as, for the levels above it.
    type Kept;
    type Error;

    /// The held names `name`'s level asks for, in the order it asks,
    /// as far as it can tell before running: each is answered before
    /// the level runs.
    fn first(
        &mut self,
        name: &'s StableName,
        kept: &Kept<Self::Kept>,
        out: &mut Vec<&'s StableName>,
    );

    /// `name`'s level, every held name it asks for read from `kept`.
    ///
    /// # Errors
    ///
    /// The walk's own refusal, or the held name it asked for that is
    /// not kept yet.
    fn level(
        &mut self,
        name: &'s StableName,
        kept: &Kept<Self::Kept>,
    ) -> Result<Self::Level, Stopped<'s, Self::Error>>;

    /// A held name's level answer, as the levels above it keep it.
    ///
    /// # Errors
    ///
    /// The walk's own refusal.
    fn keep(&mut self, name: &'s StableName, level: Self::Level)
    -> Result<Self::Kept, Self::Error>;
}

/// **Drives a [`Descent`] from its own stack**: each level runs once
/// every held name it asks for is answered. The names [`Descent::first`]
/// lists are answered before their level runs, so a level runs once and
/// the walk is linear in the names it visits; a level that asks for one
/// it did not list stops ([`Stopped::Needs`]) and runs again after it,
/// once per such name.
///
/// # Errors
///
/// The walk's first refusal.
pub(super) fn descend<'s, D: Descent<'s>>(
    root: &'s StableName,
    walk: &mut D,
) -> Result<D::Level, D::Error> {
    let mut kept = Kept(std::collections::BTreeMap::new());
    // Each name on the way, and whether its first names are listed.
    let mut stack: Vec<(&'s StableName, bool)> = vec![(root, false)];
    let mut wanted = Vec::new();
    while let Some(top) = stack.last_mut() {
        let (name, listed) = *top;
        if !core::ptr::eq(name, root) && kept.get(name).is_some() {
            // Listed twice (one handle held in two places): answered.
            stack.pop();
            continue;
        }
        if !listed {
            top.1 = true;
            walk.first(name, &kept, &mut wanted);
            let before = stack.len();
            stack.extend(
                wanted
                    .drain(..)
                    .rev()
                    .filter(|n| kept.get(n).is_none())
                    .map(|n| (n, false)),
            );
            if stack.len() > before {
                continue;
            }
        }
        #[cfg(test)]
        tests::levelled();
        match walk.level(name, &kept) {
            Ok(level) if stack.len() == 1 => return Ok(level),
            Ok(level) => {
                stack.pop();
                let answer = walk.keep(name, level)?;
                kept.0.insert(address(name), answer);
            }
            Err(Stopped::Refused(e)) => return Err(e),
            Err(Stopped::Needs(held)) => stack.push((held, false)),
        }
    }
    unreachable!("a descent returns from its root's level, the last on its stack")
}

// ---------------------------------------------------------------
// Drop.
// ---------------------------------------------------------------

impl Drop for StableName {
    fn drop(&mut self) {
        // Every segment below this name whose names this is the last
        // holder of is moved here before it goes, so each name dropped
        // in the loop drops at most its own segments. A held name that
        // holds none drops no deeper than that where it is, so its
        // handle is not asked whether it is the last.
        let mut segs = core::mem::take(&mut self.path);
        while let Some(mut seg) = segs.pop() {
            seg.each_name_mut(&mut |h| match h {
                HoldMut::Shared(r) => {
                    if r.holds_names()
                        && let Some(name) = r.get_mut()
                    {
                        segs.append(&mut name.path);
                    }
                }
                HoldMut::Owned(name) => segs.append(&mut name.path),
            });
        }
    }
}

// ---------------------------------------------------------------
// Clone.
// ---------------------------------------------------------------

impl Clone for StableName {
    fn clone(&self) -> Self {
        if shallow(Walk::Clone, Family::Name) {
            let StableName { kind, node, .. } = self;
            return StableName {
                kind: *kind,
                node: *node,
                path: Vec::new(),
            };
        }
        // A name held through a handle is shared, not copied, so only
        // the names held BY VALUE nest a copy; most names hold none.
        if !self.holds_by_value() {
            let StableName { kind, node, path } = self;
            return StableName {
                kind: *kind,
                node: *node,
                path: path.clone(),
            };
        }
        let _shallow = Shallow::enter(Walk::Clone, Family::Name);
        copy_nested(self, StableName::held_by_value, |source, copies| {
            // The derived clone of the level, its by-value names empty
            // copies (the walk is shallow), each then replaced by its
            // own copy.
            let StableName { kind, node, path } = source;
            let mut copy = StableName {
                kind: *kind,
                node: *node,
                path: path.clone(),
            };
            copy.each_held_mut(&mut |h| {
                if let HoldMut::Owned(slot) = h {
                    *slot = copies.next().unwrap_or_else(|| unreachable!("{COPIES}"));
                }
            });
            copy
        })
    }
}

/// **A copy of a nesting value, one level at a time**, from this
/// walk's own stack: `held` lists the values a level holds that are
/// copied with it, in order, and `level` copies one level, taking the
/// copies of those from the iterator in that order. Deepest first, so a
/// level's held copies are the last ones made when it takes them.
///
/// The one copy walk of both nesting values: a name's by-value names
/// (above) and a selector pattern's argument patterns (`select`).
pub(super) fn copy_nested<'a, T>(
    root: &'a T,
    held: impl Fn(&'a T) -> Vec<&'a T>,
    level: impl Fn(&'a T, &mut dyn Iterator<Item = T>) -> T,
) -> T {
    let mut stack: Vec<(&'a T, bool)> = vec![(root, false)];
    let mut done: Vec<T> = Vec::new();
    while let Some((value, listed)) = stack.pop() {
        let below = held(value);
        if !listed {
            stack.push((value, true));
            stack.extend(below.into_iter().rev().map(|v| (v, false)));
            continue;
        }
        let first = done
            .len()
            .checked_sub(below.len())
            .unwrap_or_else(|| unreachable!("{COPIES}"));
        let mut copies = done.split_off(first).into_iter();
        let copy = level(value, &mut copies);
        if copies.next().is_some() {
            unreachable!("{COPIES}");
        }
        done.push(copy);
    }
    match (done.pop(), done.is_empty()) {
        (Some(copy), true) => copy,
        _ => unreachable!("{COPIES}"),
    }
}

/// [`copy_nested`]'s invariant: a level's held copies are the last ones
/// made when it runs, one per value it holds, and the root's copy is
/// the last of all.
const COPIES: &str = "a level's held copies are the last ones made when it takes them, one per \
                      value it holds, and the root's is the last of all";

// ---------------------------------------------------------------
// Debug.
// ---------------------------------------------------------------

/// What a held name renders as while its holder's level is rendered.
/// A NUL is never written by a derived rendering (a string or char
/// renders one escaped), so it cannot be mistaken for text.
pub(super) const HOLE: char = '\0';

/// One level of a name as the derived impl renders it.
struct Level<'a>(&'a StableName);

impl fmt::Debug for Level<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let StableName { kind, node, path } = self.0;
        f.debug_struct("StableName")
            .field("kind", kind)
            .field("node", node)
            .field("path", path)
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
///
/// So `{:?}` and `{:#?}` render what the derived impl renders. No other
/// flag reaches the levels: the debug-hex flags (`{:x?}`) cannot be
/// read from a formatter on stable Rust, and a width or fill carried
/// without them would render a third thing, neither the derived form
/// nor the plain one.
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
        // Every field is bound, so a field added to the name is an E0027
        // here until equality says what it does with it.
        let same = |a: &StableName, b: &StableName| {
            let StableName { kind, node, path } = a;
            let StableName {
                kind: b_kind,
                node: b_node,
                path: b_path,
            } = b;
            kind == b_kind && node == b_node && path == b_path
        };
        if let Some(answer) = natively(|| same(self, other)) {
            return answer;
        }
        eq_nested(Family::Name, self, other, same, StableName::held)
    }
}

/// **Whether two nesting values are equal, one level at a time**: each
/// pair of levels compared by `same` with the values they hold equal
/// (the walk is shallow meanwhile), and the held pairs then compared
/// from this walk's own stack. Equal levels hold equally many.
///
/// The one equality walk of both nesting values: a name past its
/// native levels (above) and a selector pattern (`select`).
pub(super) fn eq_nested<'a, T>(
    family: Family,
    a: &'a T,
    b: &'a T,
    same: impl Fn(&T, &T) -> bool,
    held: impl Fn(&'a T) -> Vec<&'a T>,
) -> bool {
    let _shallow = Shallow::enter(Walk::Eq, family);
    let mut pairs = vec![(a, b)];
    while let Some((a, b)) = pairs.pop() {
        if core::ptr::eq(a, b) {
            continue;
        }
        if !same(a, b) {
            return false;
        }
        pairs.extend(held(a).into_iter().zip(held(b)));
    }
    true
}

impl Eq for StableName {}

/// Consistent with [`Eq`], which is all `Hash` owes: equal names feed
/// the same sequence (each level's own fields, the names it holds fed
/// nothing, in pre-order). The sequence is not the derived impl's.
///
/// While a level's own fields are fed, the walk is shallow, so a name
/// hashed from inside the `Hasher` then (a `Hasher::write` that hashes
/// some other name) feeds only its own level: a hasher that hashes
/// names from inside its own writes gets a truncated hash of those.
impl core::hash::Hash for StableName {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        if shallow(Walk::Hash, Family::Name) {
            return;
        }
        hash_from(self, state);
    }
}

// The pre-order is walked natively while the budget lasts and from a
// heap stack past it, and the two feed the same sequence.

fn hash_level<H: core::hash::Hasher>(n: &StableName, state: &mut H) {
    use core::hash::Hash as _;
    let _shallow = Shallow::enter(Walk::Hash, Family::Name);
    let StableName { kind, node, path } = n;
    kind.hash(state);
    node.hash(state);
    path.hash(state);
}

fn hash_from<H: core::hash::Hasher>(root: &StableName, state: &mut H) {
    hash_level(root, state);
    root.each_held(&mut |held| {
        let held = held.name();
        if natively(|| hash_from(held, state)).is_none() {
            let mut names = vec![held];
            while let Some(n) = names.pop() {
                hash_level(n, state);
                names.extend(n.held().into_iter().rev());
            }
        }
    });
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
        let derived = || {
            let StableName { kind, node, path } = self;
            let StableName {
                kind: b_kind,
                node: b_node,
                path: b_path,
            } = other;
            (kind, node)
                .cmp(&(b_kind, b_node))
                .then_with(|| path.cmp(b_path))
        };
        if let Some(order) = natively(derived) {
            return order;
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
        // Every field is bound, so a field added to the name is an E0027
        // here until the order says where it goes.
        let StableName { kind, node, path } = a;
        let StableName {
            kind: b_kind,
            node: b_node,
            path: b_path,
        } = b;
        let own = kind
            .cmp(b_kind)
            .then_with(|| node.cmp(b_node))
            .then_with(|| path.cmp(b_path));
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
        // Every field is bound, so a field added to the name is an
        // E0027 here until the wire form says what it does with it.
        let StableName { kind, node, path } = name;
        Self { kind, node, path }
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
        let WireIn { kind, node, path } = wire;
        StableName { kind, node, path }
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
        if WRITING.with(Cell::get) {
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
        if reading() {
            let raw = <&'de serde_json::value::RawValue as serde::Deserialize>::deserialize(de)?;
            return read_in_door(raw.get()).map_err(D::Error::custom);
        }
        <WireIn as serde::Deserialize>::deserialize(de).map(StableName::from)
    }
}

/// A name the innermost read door's reader met, as the raw `text` it
/// borrowed from the door's text: read from its text as written, and
/// recorded with the door, as read or as the door's fault.
fn read_in_door(text: &str) -> Result<StableName, String> {
    let placed = READS.with(|r| {
        let frames = r.borrow();
        let frame = frames.last()?;
        let at = (text.as_ptr() as usize)
            .checked_sub(frame.base)
            .filter(|at| at + text.len() <= frame.len)?;
        let written = match &frame.written {
            Some(written) => written.get(at..at + text.len())?.to_owned(),
            None => text.to_owned(),
        };
        Some((at, written))
    });
    let Some((at, written)) = placed else {
        return Err(OUTSIDE.to_owned());
    };
    READS.with(|r| {
        if let Some(frame) = r.borrow_mut().last_mut() {
            frame.seen.names.insert(at);
        }
    });
    read_json(&written).map_err(|fault| {
        let message = unplaced(&fault.error);
        READS.with(|r| {
            if let Some(frame) = r.borrow_mut().last_mut() {
                frame.seen.fault.get_or_insert(NameFault {
                    at: at + fault.cursor,
                    category: fault.error.classify(),
                    message: message.clone(),
                });
            }
        });
        message
    })
}

/// A name a read door's reader met outside the door's text: the door
/// was handed a reader over some other text, a defect of its caller.
const OUTSIDE: &str = "a stable name was read from text outside its read door's";

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

/// The holes of the JSON level [`read_json`] is reading: the NULs that
/// open each hole's string, which holes a name's place has met, and the
/// names the level holds written in place rather than cut (the seq
/// form).
struct Holes {
    marker: usize,
    met: Vec<bool>,
    inline: Vec<StableName>,
}

impl Holes {
    /// The placeholder a level holds where it met hole or inline name
    /// `index`: holes first, then the inline names in the order met.
    fn placeholder(index: usize) -> StableName {
        StableName {
            kind: EntityKind::Body,
            node: RecipeNodeId(index as u64),
            path: Vec::new(),
        }
    }

    /// `name`, read in place, kept and answered by its placeholder.
    fn inline(name: StableName) -> StableName {
        HOLES.with(|holes| {
            let holes = &mut *holes.borrow_mut();
            holes.inline.push(name);
            Self::placeholder(holes.met.len() + holes.inline.len() - 1)
        })
    }
}

/// What a shallow JSON level reads where it holds a name.
///
/// A hole [`read_json`] cut is a string of [`Holes::marker`] NULs and
/// the hole's index: more NULs in a row than the text holds anywhere,
/// so no string written in the text reads as one. It is answered by a
/// placeholder whose `node` is that index, and marked met. Anything
/// else is read as the derived form reads a name there: an object or an
/// array (the seq form) through the derived impl, kept and answered by
/// a placeholder past the holes', anything else refused in its words.
struct HoleVisitor;

impl<'de> serde::de::Visitor<'de> for HoleVisitor {
    type Value = StableName;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("struct StableName")
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<StableName, E> {
        let index = HOLES.with(|holes| {
            let holes = &mut *holes.borrow_mut();
            let digits = v.strip_prefix(&"\0".repeat(holes.marker))?;
            if !digits.bytes().all(|b| b.is_ascii_digit()) {
                return None;
            }
            let index: usize = digits.parse().ok()?;
            let met = holes.met.get_mut(index).filter(|met| !**met)?;
            *met = true;
            Some(index)
        });
        let index = index.ok_or_else(|| E::invalid_type(serde::de::Unexpected::Str(v), &self))?;
        Ok(Holes::placeholder(index))
    }

    fn visit_map<A: serde::de::MapAccess<'de>>(self, map: A) -> Result<StableName, A::Error> {
        <WireIn as serde::Deserialize>::deserialize(serde::de::value::MapAccessDeserializer::new(
            map,
        ))
        .map(|wire| Holes::inline(wire.into()))
    }

    fn visit_seq<A: serde::de::SeqAccess<'de>>(self, seq: A) -> Result<StableName, A::Error> {
        <WireIn as serde::Deserialize>::deserialize(serde::de::value::SeqAccessDeserializer::new(
            seq,
        ))
        .map(|wire| Holes::inline(wire.into()))
    }
}

/// A JSON object in a name's text: where it opens and closes, the
/// nearest object holding a name's key that encloses it (the root for
/// none), and whether it holds one itself.
struct Object {
    open: usize,
    close: usize,
    parent: usize,
    named: bool,
}

/// **Every object in a name's `text`, in one pass**, each with the
/// nearest enclosing object that holds a name's key.
///
/// An object holding any of a name's three keys (`kind`, `node`,
/// `path`) is taken for a name: no segment's form has one, so in text
/// this build writes every name holds them and nothing else does. The
/// reader proves each where it meets it ([`read_json`]), so a text
/// that breaks the rule is read as the derived form reads it.
fn scan(text: &str) -> Vec<Object> {
    let mut objects: Vec<Object> = Vec::new();
    // The open containers: `Some(object)` or `None` for an array.
    let mut open: Vec<Option<usize>> = Vec::new();
    for t in jsontext::tokens(text) {
        match t.tok {
            jsontext::Tok::Key => {
                if let Some(Some(object)) = open.last()
                    && jsontext::key(text, t).is_ok_and(|k| matches!(&*k, "kind" | "node" | "path"))
                    && let Some(o) = objects.get_mut(*object)
                {
                    o.named = true;
                }
            }
            jsontext::Tok::Open(b'{') => {
                let enclosing = open.iter().rev().find_map(|c| *c);
                // An enclosing object's own answer is known: objects
                // open in text order, and a key comes before anything
                // its value holds.
                let parent = enclosing.map_or(0, |e| match objects.get(e) {
                    Some(o) if o.named => e,
                    Some(o) => o.parent,
                    None => 0,
                });
                open.push(Some(objects.len()));
                objects.push(Object {
                    open: t.start,
                    close: t.start,
                    parent,
                    named: false,
                });
            }
            jsontext::Tok::Open(_) => open.push(None),
            jsontext::Tok::Close(_) => {
                if let Some(Some(object)) = open.pop()
                    && let Some(o) = objects.get_mut(object)
                {
                    o.close = t.start;
                }
            }
            _ => {}
        }
    }
    objects
}

/// The NULs that open a hole's string: one more than the longest run of
/// written NULs (`\u0000` escapes) in `text`, so no string in it reads
/// as a hole.
fn hole_marker(text: &str) -> usize {
    let bytes = text.as_bytes();
    let (mut longest, mut run, mut i) = (0, 0, 0);
    while i < bytes.len() {
        if bytes.get(i..i + 6) == Some(b"\\u0000".as_slice()) {
            run += 1;
            longest = longest.max(run);
            i += 6;
        } else {
            run = 0;
            i += 1;
        }
    }
    longest + 1
}

/// One level's text as [`read_json`] reads it: the level's object with
/// each name it holds cut out and a hole in its place, and where each
/// piece came from.
struct Cut {
    text: String,
    /// Each piece: where it starts in the cut text and in the name's,
    /// and its length in each (a hole's differ).
    pieces: Vec<(usize, usize, usize, usize)>,
    /// Where each hole starts in the cut text.
    holes: Vec<usize>,
}

impl Cut {
    fn of(text: &str, objects: &[Object], level: usize, held: &[usize], marker: usize) -> Self {
        let mut cut = Cut {
            text: String::new(),
            pieces: Vec::new(),
            holes: Vec::new(),
        };
        let Some(object) = objects.get(level) else {
            return cut;
        };
        let mut at = object.open;
        let piece = |cut: &mut Cut, orig: usize, written: &str, orig_len: usize| {
            cut.pieces
                .push((cut.text.len(), orig, written.len(), orig_len));
            cut.text.push_str(written);
        };
        for (index, span) in held.iter().filter_map(|&h| objects.get(h)).enumerate() {
            let before = text.get(at..span.open).unwrap_or("");
            piece(&mut cut, at, before, before.len());
            cut.holes.push(cut.text.len());
            let hole = format!("\"{}{index}\"", "\\u0000".repeat(marker));
            piece(&mut cut, span.open, &hole, span.close + 1 - span.open);
            at = span.close + 1;
        }
        let rest = text.get(at..=object.close).unwrap_or("");
        piece(&mut cut, at, rest, rest.len());
        cut
    }

    /// The byte of the name's text the reader stood at when it stood at
    /// byte `at` of the cut text: a hole maps to its name's start, or,
    /// read past, to its end.
    fn written(&self, at: usize) -> usize {
        let piece = self.pieces.iter().rev().find(|p| p.0 <= at);
        piece.map_or(0, |&(cut, orig, cut_len, orig_len)| {
            let into = at - cut;
            if cut_len == orig_len {
                orig + into
            } else if into == 0 {
                orig
            } else {
                orig + orig_len
            }
        })
    }
}

/// Where a name's text was refused: the byte of it the reader stood at,
/// and the reader's error (whose own place is in the level's text).
struct LevelFault {
    cursor: usize,
    error: serde_json::Error,
}

impl LevelFault {
    fn defect(what: &str) -> Self {
        Self {
            cursor: 0,
            error: serde::de::Error::custom(what),
        }
    }
}

/// **A name read from its JSON text, one level at a time.**
///
/// One pass over the text finds every object in it ([`scan`]). Each
/// object taken for a name is a level, read in the derived form from
/// its own text with every name it holds cut out and a hole in its
/// place ([`HoleVisitor`]), so reading a level reads that level's text
/// and no deeper; the levels are then assembled from the deepest up.
///
/// A hole is proven by the reader: the derived form meets it where it
/// reads a name. One it meets anywhere else was not a name there, so
/// the level is read again with that object's text in place, and the
/// derived form reads it and refuses it in its own words. A refusal is
/// placed where it is written. The one reported is the first in the
/// text among the levels the derived form would have reached, which is
/// the one the derived form's own read stops at.
fn read_json(text: &str) -> Result<StableName, LevelFault> {
    let _shallow = Shallow::enter(Walk::De, Family::Name);
    let at_fault = |cut: &str, error: serde_json::Error, map: &dyn Fn(usize) -> usize| {
        let cursor = map(jsontext::cursor(cut, error.line(), error.column()));
        LevelFault { cursor, error }
    };
    if !text.starts_with('{') {
        // Not an object: the derived form reads it (the seq form) or
        // refuses it.
        return serde_json::from_str::<WireIn>(text)
            .map(StableName::from)
            .map_err(|e| at_fault(text, e, &|at| at));
    }
    let objects = scan(text);
    let marker = hole_marker(text);
    let is_level = |o: usize| o == 0 || objects.get(o).is_some_and(|o| o.named);
    // Each level's held names, nearest first, in text order.
    let mut holds: Vec<Vec<usize>> = vec![Vec::new(); objects.len()];
    for (o, object) in objects.iter().enumerate().skip(1) {
        if object.named
            && let Some(list) = holds.get_mut(object.parent)
        {
            list.push(o);
        }
    }
    let mut reached = vec![false; objects.len()];
    let mut absorbed = vec![false; objects.len()];
    if let Some(root) = reached.first_mut() {
        *root = true;
    }
    let mut levels: Vec<Option<StableName>> = (0..objects.len()).map(|_| None).collect();
    // The names each level holds written in place, by level.
    let mut inline: Vec<Vec<Option<StableName>>> = vec![Vec::new(); objects.len()];
    let mut first: Option<LevelFault> = None;
    for level in 0..objects.len() {
        if !is_level(level) || !reached.get(level).copied().unwrap_or(false) {
            continue;
        }
        if absorbed.get(level).copied().unwrap_or(false) {
            continue;
        }
        loop {
            let held = holds.get(level).cloned().unwrap_or_default();
            let cut = Cut::of(text, &objects, level, &held, marker);
            HOLES.with(|h| {
                *h.borrow_mut() = Holes {
                    marker,
                    met: vec![false; held.len()],
                    inline: Vec::new(),
                };
            });
            let result = serde_json::from_str::<WireIn>(&cut.text);
            let (met, written_in) = HOLES.with(|h| {
                let h = &mut *h.borrow_mut();
                (core::mem::take(&mut h.met), core::mem::take(&mut h.inline))
            });
            let stop = result
                .as_ref()
                .err()
                .map(|e| jsontext::cursor(&cut.text, e.line(), e.column()));
            // A hole the reader went past without meeting it at a
            // name's place held no name there.
            let misread: Vec<usize> = (0..held.len())
                .filter(|&j| {
                    !met.get(j).copied().unwrap_or(false)
                        && stop.is_none_or(|s| cut.holes.get(j).is_some_and(|&h| h < s))
                })
                .collect();
            if !misread.is_empty() {
                for &j in misread.iter().rev() {
                    let Some(&object) = held.get(j) else { continue };
                    if let Some(flag) = absorbed.get_mut(object) {
                        *flag = true;
                    }
                    let inner = holds
                        .get_mut(object)
                        .map(core::mem::take)
                        .unwrap_or_default();
                    if let Some(list) = holds.get_mut(level) {
                        list.splice(j..=j, inner);
                    }
                }
                continue;
            }
            for (j, &object) in held.iter().enumerate() {
                if met.get(j).copied().unwrap_or(false)
                    && let Some(flag) = reached.get_mut(object)
                {
                    *flag = true;
                }
            }
            match result {
                Ok(wire) => {
                    if let Some(slot) = levels.get_mut(level) {
                        *slot = Some(wire.into());
                    }
                    if let Some(slot) = inline.get_mut(level) {
                        *slot = written_in.into_iter().map(Some).collect();
                    }
                }
                Err(error) => {
                    let fault = at_fault(&cut.text, error, &|at| cut.written(at));
                    if first.as_ref().is_none_or(|f| fault.cursor < f.cursor) {
                        first = Some(fault);
                    }
                }
            }
            break;
        }
    }
    if let Some(fault) = first {
        return Err(fault);
    }
    for level in (0..objects.len()).rev() {
        let Some(mut name) = levels.get_mut(level).and_then(Option::take) else {
            continue;
        };
        let held = holds.get(level).cloned().unwrap_or_default();
        let mut written_in = inline
            .get_mut(level)
            .map(core::mem::take)
            .unwrap_or_default();
        let mut missing = false;
        name.each_held_mut(&mut |h| {
            let (slot, index) = match h {
                HoldMut::Shared(r) => {
                    let index = r.node.0;
                    (r.get_mut(), index)
                }
                HoldMut::Owned(n) => {
                    let index = n.node.0;
                    (Some(n), index)
                }
            };
            let done = usize::try_from(index).ok().and_then(|j| match held.get(j) {
                Some(&o) => levels.get_mut(o).and_then(Option::take),
                None => written_in.get_mut(j - held.len()).and_then(Option::take),
            });
            match (slot, done) {
                (Some(slot), Some(done)) => *slot = done,
                _ => missing = true,
            }
        });
        if missing {
            return Err(LevelFault::defect(MISPLACED));
        }
        if let Some(cell) = levels.get_mut(level) {
            *cell = Some(name);
        }
    }
    levels
        .into_iter()
        .next()
        .flatten()
        .ok_or_else(|| LevelFault::defect(MISPLACED))
}

/// A level's held name that did not come back where it was met: a
/// defect of this module, never of the text.
const MISPLACED: &str = "a stable name's nested text was not read back where it was met";

/// A reader's words without the place it gives them: the door that
/// read the whole text places them where they are written.
fn unplaced(e: &serde_json::Error) -> String {
    let text = e.to_string();
    let suffix = format!(" at line {} column {}", e.line(), e.column());
    text.strip_suffix(&suffix).unwrap_or(&text).to_owned()
}

/// Why [`StableName::from_json`] refused a text: the reader's words,
/// and the line and column of the text it stood at, counted as
/// serde_json counts them. A name nested in the text is refused where
/// it is written, in the words the derived form gives.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameTextError {
    line: usize,
    column: usize,
    message: String,
}

impl NameTextError {
    /// The line of the text, from 1.
    #[must_use]
    pub fn line(&self) -> usize {
        self.line
    }

    /// The bytes of the line the reader had read.
    #[must_use]
    pub fn column(&self) -> usize {
        self.column
    }
}

impl fmt::Display for NameTextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at line {} column {}",
            self.message, self.line, self.column
        )
    }
}

impl std::error::Error for NameTextError {}

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
        write_door(|| serde_json::to_string(self))
    }

    /// **A name read from its JSON text**, one nesting level at a time,
    /// so a name of any depth reads on any stack. The inverse of
    /// [`StableName::to_json`]; any spelling of the same JSON value (a
    /// saved document's pretty-printed one included) reads the same.
    ///
    /// # Errors
    ///
    /// Text that is not JSON, or JSON that is not a name, refused where
    /// it is written ([`NameTextError`]).
    pub fn from_json(text: &str) -> Result<Self, NameTextError> {
        let (read, seen) = read_door(text, None, || serde_json::from_str::<StableName>(text));
        read.map_err(|e| match seen.fault {
            Some(fault) => {
                let (line, column) = jsontext::place(text, fault.at);
                NameTextError {
                    line,
                    column,
                    message: fault.message,
                }
            }
            None => NameTextError {
                line: e.line(),
                column: e.column(),
                message: unplaced(&e),
            },
        })
    }
}

#[cfg(test)]
pub(super) mod tests {
    //! Every walk against a copy of the derived impls
    //! (`names::nest_reference`), at every variant; every walk at a depth
    //! past any stack, on the smallest stack a door runs on; and the
    //! walks that rebuild a name, linear in what they visit.
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use crate::names::SegTag;
    use crate::names::role::{CapEnd, PieceRole, ProfileEdgeRef};
    use crate::node::StepId;
    use test_utils::own_thread::{WASM_STACK, on_the_smallest_stack};

    thread_local! {
        /// The native levels a walk takes, where a row sets them.
        static LEVELS: Cell<Option<u32>> = const { Cell::new(None) };
        /// The answers a descent asked for, and the levels it ran.
        static ASKED: Cell<usize> = const { Cell::new(0) };
        static LEVELLED: Cell<usize> = const { Cell::new(0) };
    }

    pub(super) fn native_levels() -> Option<u32> {
        LEVELS.with(Cell::get)
    }

    pub(super) fn asked() {
        ASKED.with(|a| a.set(a.get() + 1));
    }

    pub(super) fn levelled() {
        LEVELLED.with(|l| l.set(l.get() + 1));
    }

    /// `f`, and the answers and levels the descents it ran counted.
    fn counted<R>(f: impl FnOnce() -> R) -> (R, usize, usize) {
        ASKED.with(|a| a.set(0));
        LEVELLED.with(|l| l.set(0));
        let out = f();
        (out, ASKED.with(Cell::get), LEVELLED.with(Cell::get))
    }

    /// `f`, answered from the walks' own stacks from the first level.
    fn iterative<R>(f: impl FnOnce() -> R) -> R {
        let was = LEVELS.with(|l| l.replace(Some(0)));
        let out = f();
        LEVELS.with(|l| l.set(was));
        out
    }

    /// Deeper than any stack holds a recursive walk over it: a drop
    /// recursed about 80 bytes a level in release and 270 in dev, a
    /// `Debug` 0.8 to 1.3 KiB, so a mebibyte ran out by 13 000 levels
    /// in release and 700 in dev.
    pub(in crate::names) const DEEP: usize = 20_000;

    /// One segment of every variant, each name it holds `a` or `b`,
    /// built in whichever family the names are: this module's, or the
    /// derived copy's.
    macro_rules! every_segment {
        ($a:expr, $b:expr) => {{
            use RoleSeg as R;
            let (a, b): (&StableName, &StableName) = ($a, $b);
            let r = |n: &StableName| NameRef::new(n.clone());
            let step = StepId(7);
            let e = ProfileEdgeRef::Piece {
                step,
                role: PieceRole::Piece(2),
            };
            let e2 = ProfileEdgeRef::Section {
                circle: SectionCircle::Bore,
                role: PieceRole::Arc,
            };
            let v = ProfileVertexRef::Piece {
                step,
                role: PieceRole::Leg,
            };
            vec![
                R::OutputBody,
                R::Cap(CapEnd::Start),
                R::Lateral(e),
                R::RimEdge(CapEnd::End, e2),
                R::LateralEdge(v),
                R::CapVertex(CapEnd::Start, v),
                R::LoftWall(vec![e, e2]),
                R::LoftSeam(vec![v]),
                R::Band(e),
                R::BandRim(v),
                R::BandRimPi(v),
                R::BandPi(e),
                R::Meridian(MeridianEnd::Seam, e),
                R::MeridianVertex(MeridianEnd::Pi, v),
                R::RevolveCap(MeridianEnd::End),
                R::Pole(v),
                R::AxisEdge(e2),
                R::FromA(r(a)),
                R::FromB(r(b)),
                R::FromMember {
                    member: RecipeNodeId(3),
                    of: r(a),
                },
                R::Seam { a: r(a), b: r(b) },
                R::Merged(vec![a.clone(), b.clone()]),
                R::Fragment(Qualifier::Borders(vec![a.clone(), b.clone()])),
                R::Fragment(Qualifier::Keeps(vec![a.clone(), b.clone()])),
                R::Fragment(Qualifier::Ends(vec![a.clone(), b.clone()])),
                R::Fragment(Qualifier::OrderAlong { rank: 1, of: 3 }),
                R::SplitBody(SplitHalf::Below),
                R::SectionFace {
                    side: SplitHalf::Above,
                    section: 2,
                },
                R::SectionEdge {
                    side: SplitHalf::Below,
                    face: r(a),
                },
                R::SplitFragment {
                    side: SplitHalf::Above,
                    parent: r(b),
                },
                R::CrossingVertex {
                    side: SplitHalf::Below,
                    edge: r(a),
                },
                R::OnToolVertex {
                    side: SplitHalf::Above,
                    of: r(b),
                },
                R::FromTarget(r(a)),
                R::BlendFace(r(b)),
                R::CornerFace(r(a)),
                R::TrimEdge {
                    edge: r(a),
                    support: r(b),
                },
                R::FootVertex {
                    vertex: r(b),
                    support: r(a),
                },
                R::EndArc {
                    vertex: r(a),
                    edge: r(b),
                },
                R::BandFace(vec![a.clone(), b.clone()]),
                R::BandTrim {
                    edge: r(a),
                    support: RimSupport::Mate,
                },
                R::BandFoot(r(b)),
                R::BandCross {
                    edge: r(a),
                    band: vec![b.clone()],
                },
                R::BandCut(r(a)),
                R::BandSlit {
                    edge: r(b),
                    band: vec![a.clone(), b.clone()],
                },
                R::Inner(r(a)),
                R::Rim(r(b)),
                R::HoleRim { of: r(a), hole: 4 },
                R::InPart { of: r(b) },
                R::Instance { i: 5, of: r(a) },
            ]
        }};
    }

    /// Names one and two levels deep over every variant, pairs of them
    /// differing only at their deepest level, and handles that share
    /// an `Arc` or were stamped by one sealing walk, in whichever family
    /// the names are.
    macro_rules! corpus {
        () => {{
            let named = |kind, node: u64, path: Vec<RoleSeg>| StableName {
                kind,
                node: RecipeNodeId(node),
                path,
            };
            let leaf = |node: u64| named(EntityKind::Face, node, vec![RoleSeg::Cap(CapEnd::End)]);
            let (x, y) = (leaf(1), leaf(2));
            let mut out = vec![x.clone(), y.clone()];
            for seg in every_segment!(&x, &y) {
                out.push(named(EntityKind::Face, 10, vec![seg]));
            }
            let over = |n: &StableName| {
                named(
                    EntityKind::Face,
                    10,
                    vec![RoleSeg::FromA(NameRef::new(n.clone()))],
                )
            };
            let (a, b) = (over(&x), over(&y));
            let rank = RoleSeg::Fragment(Qualifier::OrderAlong { rank: 0, of: 2 });
            for (p, q) in [(&a, &b), (&b, &a)] {
                for seg in every_segment!(p, q) {
                    out.push(named(EntityKind::Edge, 11, vec![seg, rank.clone()]));
                }
            }
            // One `Arc` held twice, beside two handles stamped by one
            // walk in their structural order and beside two unstamped
            // ones: the pairs a handle settles itself, ahead of a pair
            // it does not.
            let shared = NameRef::new(a.clone());
            let (low, high) = (NameRef::new(x.clone()), NameRef::new(y.clone()));
            stamp_in_order(&low, &high);
            let (x, y) = (NameRef::new(x), NameRef::new(y));
            for other in [&low, &high, &x, &y] {
                out.push(named(
                    EntityKind::Edge,
                    12,
                    vec![RoleSeg::Seam {
                        a: shared.clone(),
                        b: other.clone(),
                    }],
                ));
            }
            out
        }};
    }

    /// The names under test, built by the corpus.
    mod ours {
        pub(super) use super::super::super::role::{
            CapEnd, EntityKind, MeridianEnd, NameRef, PieceRole, ProfileEdgeRef, ProfileVertexRef,
            Qualifier, RimSupport, RoleSeg, SectionCircle, SplitHalf, StableName,
        };
        use crate::node::{RecipeNodeId, StepId};

        fn stamp_in_order(low: &NameRef, high: &NameRef) {
            let epoch = super::super::super::role::next_epoch().expect("an epoch");
            low.stamp(epoch, 0);
            high.stamp(epoch, 1);
        }

        pub(super) fn corpus() -> Vec<StableName> {
            corpus!()
        }

        pub(super) fn segments(a: &StableName, b: &StableName) -> Vec<RoleSeg> {
            every_segment!(a, b)
        }
    }

    /// The same corpus, in the derived copy's types.
    mod derived {
        pub(super) use super::super::super::nest_reference::*;

        /// A handle there carries no stamp.
        fn stamp_in_order(_: &NameRef, _: &NameRef) {}

        pub(super) fn corpus() -> Vec<StableName> {
            corpus!()
        }
    }

    fn leaf(node: u64) -> StableName {
        named(EntityKind::Face, node, vec![RoleSeg::Cap(CapEnd::End)])
    }

    fn named(kind: EntityKind, node: u64, path: Vec<RoleSeg>) -> StableName {
        StableName {
            kind,
            node: RecipeNodeId(node),
            path,
        }
    }

    fn hash<T: core::hash::Hash>(n: &T) -> u64 {
        use core::hash::BuildHasher;
        std::hash::BuildHasherDefault::<std::collections::hash_map::DefaultHasher>::default()
            .hash_one(n)
    }

    #[test]
    fn the_corpus_holds_every_variant() {
        let (x, y) = (leaf(1), leaf(2));
        let tags: std::collections::BTreeSet<SegTag> =
            ours::segments(&x, &y).iter().map(SegTag::of).collect();
        assert_eq!(tags.len(), SegTag::ALL.len(), "one segment per variant");
    }

    /// **Every walk answers what the derived impls answer**, against a
    /// copy of the types as they were derived, at every variant: `Debug`
    /// under `{:?}` and `{:#?}`, equality and order over every pair on
    /// both routes (the native budget, and the walks' own stacks from the
    /// first level), a clone, and the wire form written and read inside
    /// the JSON doors and outside them. `Hash` is not the derived one;
    /// it agrees with `Eq` and tells the corpus's names apart.
    #[test]
    fn every_walk_answers_what_the_derived_impls_answer_at_every_variant() {
        let (names, copies) = (ours::corpus(), derived::corpus());
        assert_eq!(names.len(), copies.len(), "one corpus, in both families");
        let mut hashes = std::collections::BTreeSet::new();
        let mut distinct = 0;
        for (i, (a, ra)) in names.iter().zip(&copies).enumerate() {
            let shown = format!("{ra:?}");
            assert_eq!(format!("{a:?}"), shown, "Debug");
            assert_eq!(
                format!("{a:#?}"),
                format!("{ra:#?}"),
                "pretty Debug of {shown}"
            );
            let copy = a.clone();
            assert_eq!(format!("{copy:?}"), shown, "a clone renders as its source");
            assert!(copy == *a, "a clone equals its source: {shown}");
            let text = serde_json::to_string(ra).unwrap();
            assert_eq!(
                serde_json::to_string(a).unwrap(),
                text,
                "the wire form: {shown}"
            );
            assert_eq!(a.to_json().unwrap(), text, "the writing door: {shown}");
            let pretty = serde_json::to_string_pretty(ra).unwrap();
            assert_eq!(
                serde_json::to_string_pretty(a).unwrap(),
                pretty,
                "the pretty wire form: {shown}"
            );
            for spelled in [&text, &pretty] {
                let outside: StableName = serde_json::from_str(spelled).unwrap();
                assert_eq!(
                    format!("{outside:?}"),
                    shown,
                    "read outside a door: {shown}"
                );
                let door = StableName::from_json(spelled).unwrap();
                assert_eq!(format!("{door:?}"), shown, "read in the door: {shown}");
            }
            assert_eq!(
                iterative(|| hash(a)),
                hash(a),
                "one hash on either route: {shown}"
            );
            assert_eq!(
                hash(&copy),
                hash(a),
                "a clone hashes as its source: {shown}"
            );
            if names.iter().take(i).all(|b| b != a) {
                distinct += 1;
                hashes.insert(hash(a));
            }
            for (b, rb) in names.iter().zip(&copies) {
                let (eq, cmp) = (ra == rb, ra.cmp(rb));
                assert_eq!(a == b, eq, "eq: {shown} / {rb:?}");
                assert_eq!(a.cmp(b), cmp, "cmp: {shown} / {rb:?}");
                assert_eq!(iterative(|| a == b), eq, "eq, iterative: {shown} / {rb:?}");
                assert_eq!(
                    iterative(|| a.cmp(b)),
                    cmp,
                    "cmp, iterative: {shown} / {rb:?}"
                );
                if eq {
                    assert_eq!(hash(a), hash(b), "equal names hash alike: {shown} / {rb:?}");
                }
            }
        }
        assert_eq!(
            hashes.len(),
            distinct,
            "the corpus's distinct names hash apart"
        );
    }

    /// A name `DEEP` levels deep, each level holding the one below in
    /// turn through a shared handle and by value, beside a leaf: every
    /// shape a walk descends through.
    pub(in crate::names) fn deep(bottom: u64) -> StableName {
        let mut n = leaf(bottom);
        let side = leaf(9);
        for level in 0..DEEP {
            let r = NameRef::new(n.clone());
            let seg = match level % 7 {
                0 => RoleSeg::FromA(r),
                1 => RoleSeg::Instance { i: 1, of: r },
                2 => RoleSeg::InPart { of: r },
                3 => RoleSeg::Merged(vec![n]),
                4 => RoleSeg::Fragment(Qualifier::Borders(vec![n])),
                5 => RoleSeg::Seam {
                    a: r,
                    b: NameRef::new(side.clone()),
                },
                _ => RoleSeg::BandCross {
                    edge: NameRef::new(side.clone()),
                    band: vec![n],
                },
            };
            n = named(EntityKind::Face, level as u64 + 20, vec![seg]);
        }
        n
    }

    #[test]
    fn a_name_nested_past_every_stack_walks_on_the_smallest_stack() {
        on_the_smallest_stack(|| {
            let (a, b) = (deep(1), deep(2));
            let copy = a.clone();
            assert!(a == copy, "a clone equals its source");
            assert!(a != b, "names differing at the bottom differ");
            assert_eq!(a.cmp(&b), Ordering::Less, "and order by the bottom");
            assert_eq!(hash(&a), hash(&copy), "a clone hashes as its source");
            let shown = format!("{a:?}");
            let sides = (DEEP / 7) * 2 + usize::from(DEEP % 7 > 5);
            assert_eq!(
                shown.matches("StableName {").count(),
                DEEP + 1 + sides,
                "Debug renders every level"
            );
            let text = a.to_json().unwrap();
            let back = StableName::from_json(&text).unwrap();
            assert!(back == a, "the JSON door reads back what it wrote");
            drop((a, b, copy, back));
        });
    }

    /// **The native levels fit a quarter of the smallest stack**: a
    /// comparison, an order and a hash of two names nested past the
    /// native budget, differing at the bottom, take every native level
    /// before going on from their own stacks.
    #[test]
    fn the_native_levels_fit_a_quarter_of_the_smallest_stack() {
        let tower = |bottom| wrapped(leaf(bottom), 3 * NATIVE_LEVELS as usize, 6, RoleSeg::FromA);
        let run = std::thread::Builder::new()
            .stack_size(WASM_STACK / 4)
            .spawn(move || {
                let (a, b) = (tower(1), tower(2));
                (a == b, a.cmp(&b), hash(&a) == hash(&b))
            })
            .expect("the thread starts")
            .join()
            .expect("the walks return");
        assert_eq!(run, (false, Ordering::Less, false), "the bottoms decide");
    }

    /// A panic inside a walk leaves every walk after it on the thread
    /// answering whole, and a walk begun inside another (a `Debug` from
    /// inside a hash's `Hasher`) runs whole.
    #[test]
    fn a_walk_that_panics_or_runs_inside_another_leaves_the_walks_whole() {
        struct Panicking(usize);
        impl core::hash::Hasher for Panicking {
            fn finish(&self) -> u64 {
                0
            }
            fn write(&mut self, _: &[u8]) {
                self.0 += 1;
                assert!(self.0 < 8, "the hasher panics mid-walk");
            }
        }
        struct Showing(StableName, Option<String>);
        impl core::hash::Hasher for Showing {
            fn finish(&self) -> u64 {
                0
            }
            fn write(&mut self, _: &[u8]) {
                if self.1.is_none() {
                    self.1 = Some(format!("{:?}", self.0));
                }
            }
        }
        for depth in [3, 100, 5_000] {
            let (a, b) = (
                wrapped(leaf(1), depth, 5, RoleSeg::FromA),
                wrapped(leaf(2), depth, 5, RoleSeg::FromA),
            );
            let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                core::hash::Hash::hash(&a, &mut Panicking(0));
            }));
            assert!(panicked.is_err(), "the hasher panicked at depth {depth}");
            assert!(
                a != b && a.cmp(&b) == Ordering::Less,
                "after the panic, at depth {depth}"
            );
            assert!(a.clone() == a, "a clone after the panic, at depth {depth}");
            let other = wrapped(leaf(7), depth.min(50), 5, RoleSeg::FromA);
            let whole = format!("{other:?}");
            let mut showing = Showing(other, None);
            core::hash::Hash::hash(&a, &mut showing);
            assert_eq!(
                showing.1,
                Some(whole),
                "a Debug inside a hash, at depth {depth}"
            );
        }
    }

    /// `inner` under `levels` wrappers of `seg`, each minted by `node`.
    fn wrapped(
        inner: StableName,
        levels: usize,
        node: u64,
        seg: fn(NameRef) -> RoleSeg,
    ) -> StableName {
        (0..levels).fold(inner, |n, _| StableName {
            kind: n.kind,
            node: RecipeNodeId(node),
            path: vec![seg(NameRef::new(n))],
        })
    }

    #[test]
    fn the_descent_walks_run_on_the_smallest_stack() {
        on_the_smallest_stack(|| {
            let edge = named(
                EntityKind::Edge,
                5,
                vec![RoleSeg::Seam {
                    a: NameRef::new(leaf(1)),
                    b: NameRef::new(leaf(2)),
                }],
            );
            let through = wrapped(edge, DEEP, 6, RoleSeg::FromA);
            let (a, b) = super::super::seam_pair::seam_line_pair(&through).expect("a seam pair");
            assert_eq!((a.node.0, b.node.0), (1, 2), "the seam at the foot");
            let face = wrapped(leaf(1), DEEP, 6, RoleSeg::FromB);
            assert!(
                super::super::face_descends_from(&face, &leaf(1)),
                "a face descends from its foot"
            );
            let merged = named(
                EntityKind::Face,
                4,
                vec![RoleSeg::Merged(vec![leaf(1), leaf(2)])],
            );
            let constituents = super::super::merged::constituents_through_wrappers(&wrapped(
                merged,
                DEEP,
                6,
                RoleSeg::FromA,
            ))
            .expect("a merged face under its wrappers");
            assert_eq!(
                constituents,
                vec![
                    wrapped(leaf(1), DEEP, 6, RoleSeg::FromA),
                    wrapped(leaf(2), DEEP, 6, RoleSeg::FromA)
                ],
                "each constituent re-wrapped by the whole chain"
            );
            let piece = named(
                EntityKind::Face,
                3,
                vec![RoleSeg::Lateral(ProfileEdgeRef::Piece {
                    step: StepId(7),
                    role: PieceRole::Leg,
                })],
            );
            let copy = |r: NameRef| RoleSeg::Instance { i: 1, of: r };
            assert_eq!(
                wrapped(piece, DEEP, 8, copy).piece_steps(),
                [StepId(7)].into(),
                "the step at the foot of a chain of pattern copies"
            );
        });
    }

    #[test]
    fn a_union_collapses_a_fold_name_nested_past_every_stack_on_the_smallest_stack() {
        on_the_smallest_stack(|| {
            let union = RecipeNodeId(9);
            let member = named(
                EntityKind::Face,
                9,
                vec![RoleSeg::FromMember {
                    member: RecipeNodeId(4),
                    of: NameRef::new(leaf(4)),
                }],
            );
            let folded = wrapped(member.clone(), DEEP, 9, RoleSeg::FromA);
            let collapsed = super::super::collapse_name(union, &folded).expect("it collapses");
            assert_eq!(collapsed, member, "the descent is flattened to its foot");
        });
    }

    /// **A rewrite and a collapse run each level once**, however many
    /// names one level holds: a merged set of `WIDE` members, each a
    /// survivor over a copy of a leaf, walked through every carried
    /// name, and a union's merged face of `WIDE` members collapsed. A
    /// level that stopped at each held name and was walked again from
    /// its start would ask for `WIDE²/2` answers.
    #[test]
    fn a_rewrite_and_a_collapse_run_each_level_once() {
        const WIDE: usize = 2_000;
        let piece = |i: usize| {
            named(
                EntityKind::Face,
                1,
                vec![RoleSeg::Lateral(ProfileEdgeRef::Piece {
                    step: StepId(i as u64),
                    role: PieceRole::Leg,
                })],
            )
        };
        let member = |i: usize| {
            named(
                EntityKind::Face,
                2,
                vec![RoleSeg::FromA(NameRef::new(named(
                    EntityKind::Face,
                    1,
                    vec![RoleSeg::Instance {
                        i: 0,
                        of: NameRef::new(piece(i)),
                    }],
                )))],
            )
        };
        let wide = named(
            EntityKind::Face,
            3,
            vec![RoleSeg::Merged((0..WIDE).map(member).collect())],
        );
        let (steps, asked, levels) = counted(|| wide.piece_steps());
        assert_eq!(steps.len(), WIDE, "every member's step");
        assert_eq!(
            levels,
            3 * WIDE + 1,
            "the set, each member, its copy and its leaf, once"
        );
        assert!(
            asked <= 16 * WIDE,
            "{asked} answers asked of {WIDE} members"
        );

        let union = RecipeNodeId(9);
        let merged = named(
            EntityKind::Face,
            9,
            vec![RoleSeg::Merged(
                (0..WIDE)
                    .map(|i| {
                        named(
                            EntityKind::Face,
                            9,
                            vec![RoleSeg::FromMember {
                                member: RecipeNodeId(100 + i as u64),
                                of: NameRef::new(leaf(4)),
                            }],
                        )
                    })
                    .collect(),
            )],
        );
        let (collapsed, asked, levels) =
            counted(|| super::super::collapse_name(union, &merged).expect("it collapses"));
        assert_eq!(
            collapsed, merged,
            "a merged face of member faces collapses to itself"
        );
        assert_eq!(levels, WIDE + 1, "the set and each member, once");
        assert!(asked <= 4 * WIDE, "{asked} answers asked of {WIDE} members");
    }

    #[test]
    fn a_name_pattern_nested_past_every_stack_walks_on_the_smallest_stack() {
        use crate::names::{NamePat, SegPat};
        on_the_smallest_stack(|| {
            let nest = |bottom: NamePat| {
                (0..DEEP).fold(bottom, |p, _| NamePat::any().seg(SegPat::any().of([p])))
            };
            let face = wrapped(leaf(1), DEEP, 6, RoleSeg::FromA);
            let any = nest(NamePat::any());
            let copy = any.clone();
            assert!(copy == any, "a clone equals its source");
            assert!(
                any.matches(&face),
                "a pattern as deep as the name matches it"
            );
            let edges = nest(NamePat::of_kind(EntityKind::Edge));
            assert!(any != edges, "patterns differing at the bottom differ");
            assert!(!edges.matches(&face), "and the bottom decides the match");
            assert_eq!(
                format!("{any:?}").matches("NamePat {").count(),
                DEEP + 1,
                "Debug renders every level"
            );
            drop((any, copy, edges, face));
        });
    }

    /// **A name's text is read, and refused, as the derived form reads
    /// it**: the same value, or the same words at the same line and
    /// column, for a text written by the door, by the pretty writer,
    /// with its keys reordered and respelled, in the seq form, and for
    /// every way a nested name can be malformed — a misspelled variant
    /// or an unknown field in a name nested deep, an object with no
    /// name's key or a string where a name belongs, a name's object
    /// where some other value belongs, and a string holding a NUL.
    #[test]
    fn a_names_text_reads_and_refuses_as_the_derived_form_does() {
        let deep = wrapped(leaf(1), 20, 3, RoleSeg::FromA);
        let text = deep.to_json().unwrap();
        let pretty = serde_json::to_string_pretty(&deep).unwrap();
        let wrap =
            |inner: &str| format!(r#"{{"kind":"Face","node":3,"path":[{{"FromA":{inner}}}]}}"#);
        let leaf_text = leaf(1).to_json().unwrap();
        let texts = [
            text.clone(),
            pretty.clone(),
            text.replacen("\"Cap\"", "\"Cop\"", 1),
            pretty.replacen("\"Cap\"", "\"Cop\"", 1),
            text.replacen("\"path\":[{\"Cap\"", "\"x\":0,\"path\":[{\"Cap\"", 1),
            pretty.replacen("\"node\": 1,", "\"node\": 1,\n\"y\": [1, 2],", 1),
            wrap("{}"),
            wrap("\"\\u0000\""),
            wrap("\"\\u0000\\u00000\""),
            wrap(r#"{"kind":"Face","node":1,"path":[],"x":0}"#),
            wrap(&format!(
                r#"{{"kind":"Face","node":1,"path":[{{"SectionFace":{leaf_text}}}]}}"#
            )),
            wrap(&format!(
                r#"{{"kind":"Face","node":1,"path":[{{"Cap":{leaf_text}}}]}}"#
            )),
            wrap(r#"["Face",1,[{"Cap":"End"}]]"#),
            wrap(r#"{"path":[{"Cap":"End"}],"n\u006fde":1,"kind":"Face"}"#),
            wrap(r#"{"path":[{"Cap":"End"}],"node":1e999,"kind":"Face"}"#),
        ];
        for t in &texts {
            let door = StableName::from_json(t).map(|n| format!("{n:?}"));
            let derived = serde_json::from_str::<derived::StableName>(t).map(|n| format!("{n:?}"));
            assert_eq!(
                door.map_err(|e| e.to_string()),
                derived.map_err(|e| e.to_string()),
                "the door and the derived form on {t}"
            );
        }
    }
}
