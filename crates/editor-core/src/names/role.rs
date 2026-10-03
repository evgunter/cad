//! The N1 role vocabulary: [`StableName`], [`RolePath`], the closed
//! per-op [`RoleSeg`] enum, and the N2 [`Qualifier`] verdicts.
//!
//! # Organization (spec D2, reported choice)
//!
//! ONE closed enum with op-grouped variants, not per-op enums
//! composed: role arguments are themselves names (N1's composition),
//! so a heterogeneous `Vec<RoleSeg>` is the natural spine and a
//! single enum keeps it flat. Variants are grouped and documented by
//! op; each group is versioned with its op's contract.
//!
//! # Kind tagging (spec D1, reported choice)
//!
//! A RUNTIME kind tag ([`EntityKind`] field), not phantom typing:
//! declared pairs, table keys, and (PR 4) hit-test returns all need
//! kind-heterogeneous collections, and the F3 serialization story
//! wants one concrete type. Kind agreement is enforced at emission
//! (the table refuses a name whose kind disagrees with its entity).
//!
//! **One caller does want the kind at COMPILE time**, and gets it from
//! a type beside the tag rather than instead of it: a mate head is a
//! [`FaceName`], a `StableName` whose tag is `Face` by construction.
//! The tag is still the runtime fact everything else reads — the
//! wrapper adds a door, it does not replace the field — and it exists
//! for the one place where what the name denotes is fixed by the
//! statement being made rather than discovered from it.
//!
//! # Locators
//!
//! [`ProfileEdgeRef`]/[`ProfileVertexRef`] name a profile piece by what
//! made it, never by its position (`names/README.md`, "N1, the profile
//! pieces"): an authored piece by the [`StepId`] its step was minted
//! with and its role in that step's fixed list, and a section a
//! kernel door builds — a tube's outer circle or bore — by its place
//! in that construction, under the node that owns it. The sweep
//! emitters iterate CANONICAL segments (`crates/profile/README.md`
//! V3), and the naming anchor (`eval::anchor`) hands them the locator
//! each canonical segment answers to.

use std::sync::Arc;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

use super::canonical;
use super::nest::{Descent, Kept, Stopped, descend};
use crate::node::{RecipeNodeId, StepId};

/// **The handle a role segment holds its argument [`StableName`] by**:
/// a shared, immutable name plus one word of ORDER CACHE.
///
/// # Why the name is shared
///
/// A role path is a derivation, so a survivor's name embeds its
/// operand's name, which embeds ITS operand's, as deep as the chain
/// runs. Held by value that costs one allocation per level of descent
/// on every copy, and a boolean step copies every surviving name.
/// Held here it is one refcount bump: a survivor's argument IS the
/// operand table's name, not a transcription of it, so cloning a name
/// costs its own path and nothing below it.
///
/// # Why it carries a stamp
///
/// The [`Ord`] a name table keys on is structural, and a structural
/// compare of two names of one chain walks to the level where they
/// first differ — the bottom, for two survivors of the same descent.
/// The stamp is the escape: [`NameTable::seal_order`] walks a finished
/// table in its own key order and writes each name's POSITION in it,
/// under an epoch identifying that walk. Two names stamped by ONE walk
/// compare by position in O(1), and the answer is the structural one
/// because the walk enumerated a structurally-ordered map. Two names
/// from different walks, or either one unstamped, fall back to the
/// structural compare.
///
/// So the stamp is a cache and never a decision: [`NameRef`]'s order
/// IS [`StableName`]'s order, at every pair, and no output can depend
/// on whether a name happened to be stamped (D9).
///
/// [`NameTable::seal_order`]: super::table::NameTable::seal_order
#[derive(Clone)]
pub struct NameRef(Arc<Held>);

/// The shared payload: the name, and its cached position in the table
/// that sealed it.
struct Held {
    name: StableName,
    /// `0` while unstamped; otherwise `(epoch << 32) | (position + 1)`
    /// — the `+ 1` keeps a stamped position 0 distinguishable from
    /// "unstamped" without a second word.
    stamp: AtomicU64,
}

/// The source of sealing epochs. Monotone and process-wide: an epoch
/// identifies ONE walk over ONE table, which is what makes "same
/// epoch" mean "positions from one structurally-ordered enumeration".
static EPOCH: AtomicU32 = AtomicU32::new(1);

/// A fresh sealing epoch, or `None` once the counter is exhausted.
///
/// **An epoch is never reused.** Reuse is the one way the stamped
/// compare could answer wrongly — two names from DIFFERENT walks
/// reading as one walk's positions — so the counter SATURATES rather
/// than wrapping, and every seal after that point declines to stamp.
/// Unstamped is always safe: those names compare structurally, which
/// is the answer the stamp is a cache of. `0` is never handed out
/// because it is the spelling of "unstamped".
pub(super) fn next_epoch() -> Option<u32> {
    EPOCH
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |e| {
            (e != u32::MAX).then_some(e + 1)
        })
        .ok()
}

impl NameRef {
    /// Shares `name`.
    #[must_use]
    pub fn new(name: StableName) -> Self {
        Self(Arc::new(Held {
            name,
            stamp: AtomicU64::new(0),
        }))
    }

    /// The name itself.
    #[must_use]
    pub fn name(&self) -> &StableName {
        &self.0.name
    }

    /// The name, mutably, when this handle is its only holder.
    pub(super) fn get_mut(&mut self) -> Option<&mut StableName> {
        Arc::get_mut(&mut self.0).map(|held| &mut held.name)
    }

    /// Records this name's `position` in the `epoch` walk, unless it
    /// already carries a stamp.
    ///
    /// Stamp-once is what keeps the invariant true when one name is
    /// shared by two tables: it keeps the positions of the walk that
    /// claimed it first, and the other table's rows simply compare
    /// structurally against it.
    pub(super) fn stamp(&self, epoch: u32, position: u32) {
        // `position + 1` occupies the LOW word and must not carry into
        // the epoch field: the single position that would is declined
        // rather than stamped, and an unstamped name compares
        // structurally, which is the answer this is a cache of.
        let Some(slot) = position.checked_add(1) else {
            return;
        };
        let packed = (u64::from(epoch) << 32) | u64::from(slot);
        let _ = self
            .0
            .stamp
            .compare_exchange(0, packed, Ordering::Relaxed, Ordering::Relaxed);
    }
}

#[cfg(test)]
impl NameRef {
    /// Whether a sealing walk has claimed this name — the one thing a
    /// test needs to see that the stamp is otherwise opaque about.
    pub(crate) fn stamped_for_tests(&self) -> bool {
        self.0.stamp.load(Ordering::Relaxed) != 0
    }
}

impl core::ops::Deref for NameRef {
    type Target = StableName;

    fn deref(&self) -> &StableName {
        &self.0.name
    }
}

impl AsRef<StableName> for NameRef {
    fn as_ref(&self) -> &StableName {
        &self.0.name
    }
}

// The table looks a bare name up in a map keyed by handles, so the
// handle borrows as the name it holds. `Ord`, `Eq` and `Hash` agree
// with the name's own on every pair (the stamp is a cache), which is
// exactly `Borrow`'s contract.
impl core::borrow::Borrow<StableName> for NameRef {
    fn borrow(&self) -> &StableName {
        &self.0.name
    }
}

impl From<StableName> for NameRef {
    fn from(name: StableName) -> Self {
        Self::new(name)
    }
}

// The five walks below read `Held` rather than `NameRef`, and each
// binds every one of its fields: a third field on `Held` is an E0027
// at all five and has to be given a rendering, a comparison or a
// stated reason to sit outside one. Without the patterns it would land
// outside every one of them with no error anywhere, which is what this
// buys — the handle's own field is the `Arc`, and reading THROUGH it
// ties these impls to nothing.
//
// **No census sees this and none can.** The arrival census keys on the
// impl's own self type, and every read here goes through `self.0` — a
// tuple index into a private inner type it does not resolve. It
// reports these impls as reading no field at all, and its own
// blind-spot list says so
// (`crates/test-utils/tests/hand_written_impl_census.rs`, which names
// this file as the standing example; whether that verdict should
// change is
// `work/tint/census-answers-no-field-read-for-a-walk-that-reads-a-field.md`).
// `Deref`, `AsRef` and `Borrow` are not in the group: each hands back
// `&StableName`, so its return TYPE names the one field it projects
// and there is no list that could be short.
//
// `Serialize` is not in the group either and NOT for that reason — a
// style review caught this sentence claiming it was. It returns
// `Result<S::Ok, S::Error>`, which names nothing, and it projects
// `self.0.name` in its body: the same shape this group is about. It
// sits outside because the handle has no wire form at all, which the
// note above its impl states; that reason is about serialization, not
// about return types.
//
// `NameRef::name()` and `stamped_for_tests()` read `Held`'s fields too
// and are covered by nothing here. The group's scope is the impls
// BELOW it, which is itself a hand-written list of impls no
// declaration holds — a sixth walk added under `Ord` would inherit
// this claim without being in it. Recorded rather than closed; the
// closing instrument would be a per-impl population, which is
// `work/tint/the-per-impl-sight-anchor-is-a-suppression-list-that-shrinks.md`.

// The rendering a `NameRef` gave: the name, with no wrapper of
// its own. Name digests are taken over this text.
impl core::fmt::Debug for NameRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // The stamp is a cache of an order, never part of the value
        // (D9), so it is outside the text a digest is taken over.
        let Held { name, stamp: _ } = &*self.0;
        name.fmt(f)
    }
}

impl core::fmt::Display for NameRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Held { name, stamp: _ } = &*self.0;
        name.fmt(f)
    }
}

impl PartialEq for NameRef {
    fn eq(&self, other: &Self) -> bool {
        // Stamp outside equality by D9: two names that differ only in
        // whether a walk stamped them are the same name.
        let Held { name, stamp: _ } = &*self.0;
        let Held {
            name: other_name,
            stamp: _,
        } = &*other.0;
        Arc::ptr_eq(&self.0, &other.0) || name == other_name
    }
}

impl Eq for NameRef {}

impl core::hash::Hash for NameRef {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        // Agrees with `PartialEq` above, so the stamp is outside this
        // for the same reason and must stay outside it.
        let Held { name, stamp: _ } = &*self.0;
        name.hash(state);
    }
}

impl Ord for NameRef {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        // An answer the handle settles itself is recorded for a name
        // walk comparing the level that holds it (`nest::settled`).
        if Arc::ptr_eq(&self.0, &other.0) {
            super::nest::settled(core::cmp::Ordering::Equal);
            return core::cmp::Ordering::Equal;
        }
        // Both fields are read here — the stamp as the O(1) cache of
        // the structural order, the name as the answer it caches.
        let Held { name, stamp } = &*self.0;
        let Held {
            name: other_name,
            stamp: other_stamp,
        } = &*other.0;
        let (a, b) = (
            stamp.load(Ordering::Relaxed),
            other_stamp.load(Ordering::Relaxed),
        );
        // One walk stamped both, so their positions ARE their
        // structural order. A zero stamp has epoch 0, which no walk
        // ever uses, so this arm cannot fire on an unstamped pair.
        if a != 0 && (a >> 32) == (b >> 32) {
            let order = (a as u32).cmp(&(b as u32));
            super::nest::settled(order);
            return order;
        }
        name.cmp(other_name)
    }
}

impl PartialOrd for NameRef {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Structurally, exactly as the boxed name serialized: the handle is a
// runtime sharing decision and has no wire form of its own (F3).
impl serde::Serialize for NameRef {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        self.0.name.serialize(ser)
    }
}

impl<'de> serde::Deserialize<'de> for NameRef {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        StableName::deserialize(de).map(Self::new)
    }
}

/// Entity kinds a [`StableName`] can denote (N1: bodies are
/// first-class alongside faces/edges/vertices, Q-h).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum EntityKind {
    /// A whole body.
    Body,
    /// A face.
    Face,
    /// An edge.
    Edge,
    /// A vertex.
    Vertex,
}

impl EntityKind {
    /// The kind as a prose noun — the one spelling every user-facing
    /// message uses, so a rendered kind never leans on `Debug`.
    pub(crate) fn noun(self) -> &'static str {
        match self {
            Self::Body => "body",
            Self::Face => "face",
            Self::Edge => "edge",
            Self::Vertex => "vertex",
        }
    }

    /// The indefinite article agreeing with [`EntityKind::noun`] — the
    /// value decides it ("an edge", "a face"), so a sentence that
    /// hard-codes one is wrong for some kind it can reach. Rendered as
    /// `"{} {}"` beside the noun, or beside a whole [`StableName`]
    /// whose kind this is. `Dimension`'s `article` is the same idiom
    /// and states the rule.
    pub(crate) fn article(self) -> &'static str {
        match self {
            Self::Body | Self::Face | Self::Vertex => "a",
            Self::Edge => "an",
        }
    }
}

/// Why a [`StableName`] could not be read as a [`FaceName`].
///
/// One field, because there is one fact: a name's kind is data on the
/// name, so the only thing the constructor can report is what it found
/// instead. `found` is the word every entity-kind refusal in this
/// crate spells its answer with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NotAFaceName {
    /// What the name denotes.
    pub found: EntityKind,
}

impl core::fmt::Display for NotAFaceName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "the name denotes {} {}, and a face is required here",
            self.found.article(),
            self.found.noun()
        )
    }
}

impl core::error::Error for NotAFaceName {}

/// **A [`StableName`] that denotes a FACE, by construction.**
///
/// Not to be confused with `tess-meter`'s `FaceName`, a validated text
/// token in a mesh report: one short name, two unrelated types, and
/// `demos/tour` uses both — which is why that binary spells each by
/// full path.
///
/// A name's kind is data on the name — readable with no product, no
/// table and no evaluation — so a caller that requires a face can
/// require it in the TYPE rather than re-asking the question at every
/// door. [`crate::SitedFace`] is the carrier a mate's heads are made
/// of, and that is what makes a mate whose head names an edge a
/// program that does not compile rather than a document some door has
/// to refuse.
///
/// **Where a `FaceName` comes from — the whole census, and its one
/// home.** Three boundaries turn DATA into a face name and each calls
/// [`FaceName::new`], which is where the kind is still a question:
///
/// - the WIRE — this type's `Deserialize`, so a file whose mate head
///   or interface crossing names an edge refuses at the load door's
///   own parse;
/// - the PYTHON binding's name-from-text door
///   (`pncad-py`'s `py::doc`), which answers the refusal as a typed
///   Python error;
/// - the VIEWER's picked face (`viewer`'s mate tool), where a pick is
///   data until the kind is asked.
///
/// Inside the crate a face name is never re-asked, only re-derived:
/// `FaceName::map_derivation` (crate-private, below) is the single
/// door, and its signature cannot change a kind. The split's remap
/// (`refactor::remap_face`) and the `Rebind` repair
/// (`Node::rebind_payload_names`) are its two callers.
///
/// The inner name is reachable by [`Deref`](core::ops::Deref) and
/// [`AsRef`], never by a public field: a field could be assigned and
/// the invariant would last exactly until someone did.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize)]
#[serde(transparent)]
pub struct FaceName(StableName);

impl FaceName {
    /// The checked constructor — the ONE way a `FaceName` is made.
    pub fn new(name: StableName) -> Result<Self, NotAFaceName> {
        if name.kind == EntityKind::Face {
            Ok(Self(name))
        } else {
            Err(NotAFaceName { found: name.kind })
        }
    }

    /// The name back out, owned.
    pub fn into_name(self) -> StableName {
        self.0
    }

    /// **This face as the instance that places its part names it** —
    /// the part's own name for the face, worn inside the one
    /// [`RoleSeg::InPart`] qualifier an instantiate node puts round
    /// every name it places, headed at that instance. What a mate head
    /// on a placed part is; [`FaceName::part_local`] is its inverse.
    pub fn in_part(&self, instance: RecipeNodeId) -> FaceName {
        Self(StableName {
            kind: self.0.kind,
            node: instance,
            path: vec![RoleSeg::InPart {
                of: self.0.clone().into(),
            }],
        })
    }

    /// **The part-local face a placed name wraps** — the row of the
    /// part's own table under the one `InPart` qualifier `instance`
    /// put round it, read INSIDE the part where no instance exists:
    /// what a `FromFace` mate frame stores. `None` when `name` is not
    /// of that shape (headed elsewhere, qualified otherwise, or not a
    /// face); [`FaceName::in_part`] is its inverse.
    pub fn part_local(name: &StableName, instance: RecipeNodeId) -> Option<FaceName> {
        if name.node != instance {
            return None;
        }
        let [RoleSeg::InPart { of }] = name.path.as_slice() else {
            return None;
        };
        FaceName::new((**of).clone()).ok()
    }

    /// **The one in-crate way a face name is re-made**: this face's
    /// DERIVATION rewritten, its kind untouched.
    ///
    /// `rewrite` is handed the minting node and the role path — the
    /// whole of what a name derives from — and answers the new pair.
    /// It is never handed the KIND and cannot return one, so a rewrite
    /// that produced a non-face is not a state this signature can
    /// express: there is no arm to refuse, assert away or call
    /// unreachable. That is the difference from [`FaceName::new`],
    /// which is the door for data whose kind is still a question.
    ///
    /// # Errors
    ///
    /// Whatever `rewrite` answers — the rewrite's own miss, carried
    /// through unchanged.
    pub(crate) fn map_derivation<E>(
        &self,
        rewrite: impl FnOnce(RecipeNodeId, &[RoleSeg]) -> Result<(RecipeNodeId, RolePath), E>,
    ) -> Result<Self, E> {
        let (node, path) = rewrite(self.0.node, &self.0.path)?;
        Ok(Self(StableName {
            kind: self.0.kind,
            node,
            path,
        }))
    }
}

impl core::ops::Deref for FaceName {
    type Target = StableName;

    fn deref(&self) -> &StableName {
        &self.0
    }
}

impl AsRef<StableName> for FaceName {
    fn as_ref(&self) -> &StableName {
        &self.0
    }
}

// The name's own rendering, forwarded: a face name reads the same
// wherever it is held, and a wrapper that re-spelled it would be a
// second vocabulary for one fact.
impl core::fmt::Display for FaceName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

// THE WIRE'S DOOR. `Deserialize` goes through [`FaceName::new`], so a
// file whose mate head names an edge is refused where the bytes are
// read — in the load door's own `PersistError::Unreadable` class,
// which is what "this build's types rejected these bytes" means — and
// no walk downstream has to re-ask the question.
impl<'de> serde::Deserialize<'de> for FaceName {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        let name = StableName::deserialize(de)?;
        Self::new(name).map_err(serde::de::Error::custom)
    }
}

/// N1's stable name: a derivation path — the minting node plus an
/// op-typed role path. Float-free and arena-key-free by construction;
/// serialization is structural (F3, PR 6).
///
/// A name nests whole names inside its segments, as deep as its
/// derivation runs, so its `Drop`, `Clone`, `Debug`, `PartialEq`,
/// `Hash`, `Ord` and serde impls are written by hand, one level at a
/// time (`names::nest`), and none recurses on the nesting. `Clone`,
/// `PartialEq`, `Ord`, serde and `Debug` under `{:?}` and `{:#?}` give
/// the derived impls' answers; `Hash` is consistent with `Eq`.
pub struct StableName {
    /// The entity kind this name denotes (N1's `K`, runtime-tagged —
    /// module docs).
    pub kind: EntityKind,
    /// The recipe node whose operation minted the entity (for
    /// pass-through ops — the set `verbatim_edge` states — the
    /// ORIGINAL minting node: those ops contribute no segment).
    pub node: RecipeNodeId,
    /// The role path within that operation.
    pub path: RolePath,
}

// The human-readable rendering: the kind (through [`EntityKind::noun`],
// never `Debug`) plus the minting node — the half of a name a user can
// act on. The role path is a derivation, not something a person reads
// mid-sentence, so prose never renders it; the typed value remains the
// machine channel for anything that needs the path. Article-free
// ("face name minted by node 3") so a sentence supplies its own
// article. Refusal prose that names a name forwards this rather than
// re-spelling it.
impl core::fmt::Display for StableName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} name minted by node {}", self.kind.noun(), self.node)
    }
}

/// A sequence of role segments (N1). Usually length 1; composition
/// (`[FromA(..), Fragment(..)]`) grows it.
pub type RolePath = Vec<RoleSeg>;

/// Which end of the sweep vector a cap face closes. The sweep vector
/// is the signed extrusion (or the stacking from first section to
/// last), so both variants hold whichever way it points; the derived
/// `Ord` is the name table's key order and the declaration order is
/// that key order alone.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum CapEnd {
    /// Where the sweep vector ends: on the sketch plane translated by
    /// it.
    End,
    /// Where the sweep vector starts: on the sketch plane it is
    /// measured from.
    Start,
}

/// **Which of its step's pieces a profile piece is** — the role half
/// of a locator, from the fixed list its verb draws. The one type is
/// the profile crate's, which records it per segment as it replays;
/// its docs give the lists, and its `Display` is the one spelling a
/// user reads.
pub use profile::PieceRole;

/// **The wire spelling of [`PieceRole`]**: serde's derive for the
/// profile crate's type, which carries no serde of its own (the kernel
/// crates are serde-free). Externally tagged, one variant per role, so
/// a role reads back as the variant it was written as.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(remote = "profile::PieceRole")]
enum PieceRoleWire {
    Leg,
    RunIn,
    Arc,
    RunOut,
    Piece(u32),
}

/// **Which circle of a kernel-built section** — the tube doors'
/// section, whose shape the node kind fixes: one circle for a solid
/// tube, the outer circle and the bore for a hollow one.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum SectionCircle {
    /// The outer circle.
    Outer,
    /// A hollow section's bore.
    Bore,
}

/// **A profile edge — one piece of a profile — by what made it**,
/// never by its position.
///
/// - [`ProfileEdgeRef::Piece`] names what an author drew: the piece
///   `role` of the step minted `step`. No loop index and no segment
///   index enters it, so a value edit, an outer/hole swap, a sense
///   flip or a `SetProgram` that keeps the step cannot move it; a role
///   the current values do not draw, or a step a `SetProgram` dropped,
///   denotes nothing (N1).
/// - [`ProfileEdgeRef::Section`] names what a kernel door built: piece
///   `role` of one circle of a section whose shape the minting node's
///   kind fixes (a tube's), so nothing can renumber it.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum ProfileEdgeRef {
    /// The piece `role` of the authored step `step`.
    Piece {
        /// The step's minted id.
        step: StepId,
        /// Which of the step's pieces.
        #[serde(with = "PieceRoleWire")]
        role: PieceRole,
    },
    /// Piece `role` of one circle of a kernel-built section.
    Section {
        /// Which circle.
        circle: SectionCircle,
        /// Which of its pieces.
        #[serde(with = "PieceRoleWire")]
        role: PieceRole,
    },
}

/// **The profile pieces one swept wall holds** (`names/README.md`, N1
/// "Swept walls over a run"): the run of pieces an extrude or a revolve
/// built one wall over, their locators in authored order, never empty.
///
/// A one-piece run is spelled as that one locator, on the wire and in
/// words, so a name minted before runs existed reads back unchanged; a
/// run of several is spelled as the list.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PieceRun(Vec<ProfileEdgeRef>);

impl core::fmt::Debug for PieceRun {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.single() {
            Some(one) => one.fmt(f),
            None => f.debug_list().entries(&self.0).finish(),
        }
    }
}

impl PieceRun {
    /// The run of `pieces`, or `None` when there are none.
    #[must_use]
    pub fn new(pieces: Vec<ProfileEdgeRef>) -> Option<Self> {
        (!pieces.is_empty()).then_some(Self(pieces))
    }

    /// The run of one piece.
    #[must_use]
    pub fn one(piece: ProfileEdgeRef) -> Self {
        Self(vec![piece])
    }

    /// The pieces, in authored order.
    #[must_use]
    pub fn pieces(&self) -> &[ProfileEdgeRef] {
        &self.0
    }

    /// The piece of a one-piece run.
    #[must_use]
    pub fn single(&self) -> Option<ProfileEdgeRef> {
        match self.0.as_slice() {
            [one] => Some(*one),
            _ => None,
        }
    }

    /// Whether `piece` is one of the run's.
    #[must_use]
    pub fn holds(&self, piece: &ProfileEdgeRef) -> bool {
        self.0.contains(piece)
    }

    /// The run with each piece passed through `f`.
    ///
    /// # Errors
    ///
    /// The first error `f` returns.
    pub fn try_map<E>(
        &self,
        mut f: impl FnMut(ProfileEdgeRef) -> Result<ProfileEdgeRef, E>,
    ) -> Result<Self, E> {
        Ok(Self(
            self.0.iter().map(|e| f(*e)).collect::<Result<_, _>>()?,
        ))
    }
}

impl From<ProfileEdgeRef> for PieceRun {
    fn from(piece: ProfileEdgeRef) -> Self {
        Self::one(piece)
    }
}

/// [`PieceRun`]'s wire spelling: the bare locator for one piece (a JSON
/// object, `ProfileEdgeRef`'s externally tagged form), the list for
/// several (a JSON array). The reader dispatches ONCE on the token it
/// meets — an object or an array — and never tries one arm and falls
/// back to the other, so a refusal inside a locator is the refusal that
/// decided the read (`persist::refusal`'s premise,
/// `scripts/gates/persist-no-backtracking.sh`).
impl serde::Serialize for PieceRun {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self.single() {
            Some(one) => one.serialize(s),
            None => self.0.serialize(s),
        }
    }
}

/// The one-dispatch reader behind [`PieceRun`]'s `Deserialize`.
struct PieceRunVisitor;

impl<'de> serde::de::Visitor<'de> for PieceRunVisitor {
    type Value = PieceRun;

    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("a profile piece locator, or a non-empty list of them")
    }

    fn visit_map<A: serde::de::MapAccess<'de>>(self, map: A) -> Result<PieceRun, A::Error> {
        let one = <ProfileEdgeRef as serde::Deserialize>::deserialize(
            serde::de::value::MapAccessDeserializer::new(map),
        )?;
        Ok(PieceRun::one(one))
    }

    fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> Result<PieceRun, A::Error> {
        let mut pieces = Vec::new();
        while let Some(piece) = seq.next_element::<ProfileEdgeRef>()? {
            pieces.push(piece);
        }
        PieceRun::new(pieces)
            .ok_or_else(|| serde::de::Error::custom("a run of profile pieces holds at least one"))
    }
}

impl<'de> serde::Deserialize<'de> for PieceRun {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(PieceRunVisitor)
    }
}

/// **A profile vertex by what made it**: the vertex where the piece of
/// the same spelling starts, in authored order ([`ProfileEdgeRef`]'s
/// two forms, read at the piece's start).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum ProfileVertexRef {
    /// Where the piece `role` of the authored step `step` starts.
    Piece {
        /// The step's minted id.
        step: StepId,
        /// Which of the step's pieces starts here.
        #[serde(with = "PieceRoleWire")]
        role: PieceRole,
    },
    /// Where piece `role` of one circle of a kernel-built section
    /// starts.
    Section {
        /// Which circle.
        circle: SectionCircle,
        /// Which of its pieces starts here.
        #[serde(with = "PieceRoleWire")]
        role: PieceRole,
    },
}

impl ProfileEdgeRef {
    /// The authored step this piece belongs to, where it is one.
    #[must_use]
    pub fn step(&self) -> Option<StepId> {
        match self {
            Self::Piece { step, .. } => Some(*step),
            Self::Section { .. } => None,
        }
    }

    /// The vertex where this piece starts.
    #[must_use]
    pub fn start(&self) -> ProfileVertexRef {
        match *self {
            Self::Piece { step, role } => ProfileVertexRef::Piece { step, role },
            Self::Section { circle, role } => ProfileVertexRef::Section { circle, role },
        }
    }
}

impl ProfileVertexRef {
    /// The authored step whose piece starts here, where it is one.
    #[must_use]
    pub fn step(&self) -> Option<StepId> {
        match self {
            Self::Piece { step, .. } => Some(*step),
            Self::Section { .. } => None,
        }
    }
}

/// Which meridian of a revolve (the M2 band/pole/seam taxonomy).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum MeridianEnd {
    /// Partial: the start-cap side (on the sketch plane).
    Start,
    /// Partial: the end-cap side (sketch plane rotated by θ).
    End,
    /// Full: the `u = 0` seam chain (F13 seam role).
    Seam,
    /// Full, wire case only: the angle-π copies (not a seam).
    Pi,
}

/// A split output half, by the tool plane's orientation (the plane's
/// normal side is `Above` — recipe-covariant: the tool is the split
/// node's own input).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum SplitHalf {
    /// Material on the tool plane's normal side.
    Above,
    /// Material on the opposite side.
    Below,
}

impl SplitHalf {
    /// Both halves, in output-body order.
    pub const ALL: [SplitHalf; 2] = [SplitHalf::Above, SplitHalf::Below];

    /// **The half's OUTPUT-BODY INDEX in a split's value** — the one
    /// definition of the mapping every reader of a split's table and
    /// value keys by: the emitter writes the rows under it, the
    /// product gather and the interrogation doors read the value by
    /// it, and a projection of one half selects by it. `Above` is
    /// body 0 and `Below` is body 1 because that is the order the
    /// value's fields are declared in; nothing else fixes it, so
    /// nothing else may restate it.
    pub fn output_body(self) -> u32 {
        match self {
            SplitHalf::Above => 0,
            SplitHalf::Below => 1,
        }
    }

    /// The half that owns output body `index`, if either does — the
    /// inverse of [`SplitHalf::output_body`], DERIVED from it rather
    /// than written a second time.
    pub fn of_output_body(index: u32) -> Option<SplitHalf> {
        SplitHalf::ALL
            .into_iter()
            .find(|half| half.output_body() == index)
    }
}

/// An N2 fragment discriminator against recipe-covariant references.
/// NO values, NO bare indices: `Borders`, `Keeps` and `Ends` cite
/// names, and `OrderAlong.rank` is an ordinal under the named
/// order-along comparison, which changes only at a recorded flip.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum Qualifier {
    /// The divider walls a boolean's or a union's face piece borders,
    /// each cited by its parent's name, sorted and deduplicated (N2,
    /// `names::borders`): the faces across its edges where it meets an
    /// obstacle — a connected part of the parent face's region no piece
    /// holds — that borders two or more pieces. A pure function of the
    /// result's topology and the boolean's record of what it discarded;
    /// the pieces the wall set does not tell apart are N2's tie.
    Borders(Vec<StableName>),
    /// The boundary edges of its parent face a Split's same-side face
    /// piece holds a stretch of, each cited by its name without piece
    /// qualifiers, sorted and deduplicated (N2). Pieces with equal sets
    /// are N2's tie.
    Keeps(Vec<StableName>),
    /// The names of an edge piece's two end vertices as the node
    /// publishes them, sorted, a repeat kept (N2): the qualifier of
    /// every piece of a parent edge its parent's name does not settle.
    /// Pieces with equal pairs are N2's tie.
    Ends(Vec<StableName>),
    /// Ordinal position of a crossing VERTEX along the edge it lies on
    /// (`name_frag_order_along` through `k_stats`): rank `rank` of `of`
    /// crossings of one edge by one face, ordered by the crossed edge's
    /// carrier parameter, the edge oriented as its operand body stores
    /// it, and a seam edge as the loop of its pair's first side runs
    /// along it (N2).
    ///
    /// A union's seam pair is in name order (`names::canonical`), and
    /// wherever putting it there swaps the pair — at the union's
    /// collapse, or at a later rewrite of the name that reorders the
    /// two sides — the rank is read from the other end.
    OrderAlong {
        /// This crossing's rank (0-based) along the crossed edge.
        rank: u32,
        /// How many crossings the ordering ranked.
        of: u32,
    },
}

/// Which support of a rim blend an entity lies on (M6-5). A pair of
/// structural ROLES, not a geometric classification: the surgery knows
/// which support is which from the chain it resolved, and a rim
/// between two supports of the SAME kind — two cones meeting at a
/// latitude circle — has no kind that tells them apart.
///
/// [`RimSupport::Host`] is the PLANAR support wherever the rim has
/// one, so a caller selecting the flat side of a plane–sphere rim
/// selects the host; a rim whose supports both curve takes the
/// resolved link's own first side.
///
/// **The planarity boundary is where this vocabulary is NOT
/// covariant, and it is load-bearing enough to state here.** Because
/// the host is defined by planarity, an edit that carries a support
/// ACROSS planarity re-decides which arc each role addresses, while
/// the rim's own name and the selection naming it stay word for word
/// the same. Every edit that does not cross that boundary leaves both
/// roles fixed. The instability is INHERITED, not introduced — the
/// retired kind vocabulary moved on exactly the same edits — but it is
/// worse in one respect and the record should say so: a renamed KIND
/// makes a stored selection stop resolving, loudly, where a swapped
/// ROLE silently retargets it to the other arc of the same rim.
/// Pinned by `blend5_r1_probes` and `blend5_r2_probes`.
///
/// # The kernel twin, and why this is not it
///
/// `sweep::blend::naming::RimSide` is the same two roles, recorded by
/// the surgery as it carves; `names::emit_blend` maps one onto the
/// other by an identity match. The duplication is deliberate and the
/// emitter's match is the SEAM.
///
/// This side is what a file remembers: persisted, so its spelling is
/// file data and cannot move without every saved document moving with
/// it. The kernel side is a birth record of arena keys with no serde,
/// free to be re-spelled with the surgery. Collapsing them would
/// either drag serde down into the kernel — which G1 layering forbids
/// the other way round too, since the kernel must not depend on
/// `editor-core` — or let a surgery refactor silently re-spell every
/// saved document. With the seam, a rename on the kernel side that
/// the emitter still maps touches no file. The fuller statement lives
/// at `RimSide`'s own declaration.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
// INERT as it stands, and kept deliberately: an externally-tagged enum
// of unit-only variants already rejects an unknown variant name
// unconditionally, so this attribute guards only a FUTURE variant that
// carries fields. Its siblings above carry it for the same reason;
// issue #1308 owns the workspace-wide disposition.
pub enum RimSupport {
    /// The HOST support: the planar one wherever the rim has one (on a
    /// ladder rim, the face carrying the rim as a ring), otherwise the
    /// resolved link's own first side.
    Host,
    /// The MATE support: the other side of the same rim (on a ladder
    /// rim, the cap the rim bounds).
    Mate,
}

/// One op-typed role segment (N1; closed enum, spec D2). Grouped by
/// op; each group versions with its op's contract.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum RoleSeg {
    // ---- Shared ----
    /// The single output body of a body-producing op (extrude,
    /// revolve, boolean). Q-h: a body's name is the node + the role
    /// that minted it.
    OutputBody,

    // ---- Extrude ----
    /// A cap face.
    Cap(CapEnd),
    /// A side-wall face swept from a run of profile pieces on one
    /// carrier (N1, "Swept walls over a run").
    Lateral(PieceRun),
    /// A cap–wall rim edge (cap end × profile segment).
    RimEdge(CapEnd, ProfileEdgeRef),
    /// A strut (join) edge swept from a profile vertex.
    LateralEdge(ProfileVertexRef),
    /// A cap vertex over a profile vertex.
    CapVertex(CapEnd, ProfileVertexRef),

    // ---- Loft ----
    /// A loft wall: the pieces the skin paired into it, one per
    /// section, in section order. A loft's caps, rims and cap vertices
    /// are the extrude's roles, each spelled with its own end
    /// section's locator.
    LoftWall(Vec<ProfileEdgeRef>),
    /// A loft seam: the wall–wall edge through the vertices the skin
    /// paired, one per section, in section order.
    LoftSeam(Vec<ProfileVertexRef>),

    // ---- Revolve (M2 band/pole/seam taxonomy) ----
    /// A wall (band) face swept from a run of profile pieces on one
    /// carrier (N1, "Swept walls over a run").
    Band(PieceRun),
    /// A latitude (rim) edge at a profile vertex (partial: wedge
    /// arcs; full: full-period rims).
    BandRim(ProfileVertexRef),
    /// Full, wire case: the π…2π band's latitude half-rim.
    BandRimPi(ProfileVertexRef),
    /// Full, wire case: a CURVED wall's π…2π band face. A plane wall is
    /// built whole and named by [`RoleSeg::Band`] alone.
    BandPi(PieceRun),
    /// A meridian edge (per meridian, per wall run: a partial revolve's
    /// stations split its meridian chains, so there it is per piece).
    Meridian(MeridianEnd, PieceRun),
    /// A meridian vertex: the copy of a profile vertex on a wedge
    /// cap plane (partial) or the surviving meridian vertex (full).
    MeridianVertex(MeridianEnd, ProfileVertexRef),
    /// A wedge cap face (partial revolve; `Start`/`End` only).
    RevolveCap(MeridianEnd),
    /// An on-axis (pole) vertex at a profile vertex on the axis.
    Pole(ProfileVertexRef),
    /// The shared axis edge of an on-axis profile segment (partial).
    AxisEdge(ProfileEdgeRef),

    // ---- Booleans ----
    /// An entity surviving from operand A (argument: its name in the
    /// A operand's table).
    FromA(NameRef),
    /// An entity surviving from operand B.
    FromB(NameRef),
    /// **An entity surviving from one MEMBER of an n-ary union**
    /// ([`crate::Node::Union`]; DM4 as amended): which member, and
    /// which entity of it.
    ///
    /// The union's value is a fold of the pair verb, so the fold's own
    /// tables carry `FromA`/`FromB` descent chains whose depth is the
    /// member's POSITION in the list. This segment is what the union's
    /// emitter mints instead: one wrapper, whatever the depth. A
    /// member's own names are therefore a function of the member's
    /// identity alone — neither its position nor how many members
    /// precede it — which is what lets a member be dropped without
    /// renaming the rest. A declaration, which names a member's entity
    /// SITED at that member and is rewritten into this wrapper at the
    /// routing door, keeps that identity through the fold's MERGES: a
    /// member's face that a declared merge has consumed resolves, at
    /// the step its pair is fed to, to the accumulation's `Merged` row
    /// whose flat constituent set holds it. A face another member
    /// contained whole leaves no row behind, and a pair naming it is
    /// satisfied. A face surviving only in pieces — split by a later
    /// member, or inside a merged row later fragmented — has no one
    /// entity to resolve to, and a pair naming it refuses, saying which
    /// of the two consumed it; once every piece is contained whole, no
    /// piece survives and the pair is satisfied instead
    /// ([`crate::Node::Union`] states the rule).
    ///
    /// That is a statement about the WRAPPER, and about nothing else.
    /// Which of a union's names exist at all is still the pair verb's
    /// answer at every step, and the pair verb is not symmetric in its
    /// two operands: a declared merge keeps operand A's carrier and
    /// splits operand A's rims, so reordering the member list moves
    /// `Fragment` rows from one member to the other and
    /// changes the merged face's carrier origin. Measured on a bare
    /// [`crate::Node::Boolean`] with no union in the picture
    /// (`work/wire/the-pair-verbs-declared-merge-is-asymmetric-in-its-operands.md`),
    /// so it is the verb's asymmetry showing through a fold rather
    /// than anything the fold or this segment adds.
    ///
    /// # Why the member EDGE and not just the inner name
    ///
    /// Because an inner name does not say which member it came from.
    /// A pass-through op mints no name of its own (N1: a transform
    /// adds no segment and `node` stays the original minter), so N
    /// placements of one prototype carry N IDENTICAL tables — which
    /// is the die's twenty-one pips exactly. Keying on the inner name
    /// alone collapses them onto one name and the emitter refuses.
    /// The member's own node id is the list edge, and it is the only
    /// thing that distinguishes one member from another.
    ///
    /// It is a bare [`RecipeNodeId`] rather than a name, which is new
    /// in this vocabulary, and it is the honest shape: what is being
    /// recorded IS a recipe edge, not another entity. `Instance { i,
    /// of }` is the precedent — a recipe-structural discriminator
    /// beside the name it qualifies — with an id where that one has an
    /// index, because a union's members are named by the DAG and a
    /// pattern's instances are counted.
    FromMember {
        /// The member node this entity came from — a DAG edge of the
        /// union, and the identity that makes the name position-free.
        member: RecipeNodeId,
        /// The entity's name in that member's own table.
        of: NameRef,
    },
    /// A zip-minted seam entity: the crossing of an A-operand entity
    /// and a B-operand entity, by their operand names. An edge is
    /// face × face. A vertex is edge × edge, edge × face or
    /// face × edge, face × face (every incident seam line agreeing on
    /// one face pair), or edge × vertex / vertex × edge (the partner
    /// read from the reduction's contact records). Several pieces of one
    /// seam edge carry a `Fragment(Ends)` after it, and a vertex pair
    /// that crosses more than once a `Fragment(OrderAlong)`. A seam
    /// JUNCTION — the vertex where k ≥ 2 seam lines meet
    /// and no operand edge does — is named by the sorted run of those
    /// lines' face × face `Seam` segments, one segment per line and
    /// nothing after them.
    ///
    /// In a pair boolean's table `a` is the A side and `b` the B side.
    /// In a UNION's published table they are not: a union has no A
    /// and B, so the two sides are in name order (`names::canonical`,
    /// at the collapse and after any rewrite), and `a` is only the
    /// lesser name.
    Seam {
        /// The A-side crossing entity's name (the lesser name, in a
        /// union's table).
        a: NameRef,
        /// The B-side crossing entity's name (the greater name, in a
        /// union's table).
        b: NameRef,
    },
    /// An F7 merged face: the sorted, FLAT set of constituent names
    /// retires into this name (N3; canonical order = name order). A
    /// constituent is never itself a BARE merged face, through any
    /// `FromA`/`FromB` wrapping — a merge of a merged face lists the
    /// faces, never the merge. The one carve-out, stated here and
    /// pointed at from every other site: a FRAGMENT of a merged face
    /// (`[Merged(set), Fragment(q)]`) is a face in its own right, a
    /// legitimate constituent, and is not nesting.
    Merged(Vec<StableName>),
    /// A fragment discriminator, composed AFTER the parent-bearing
    /// segment: `[FromA(f), Fragment(q)]` reads "the q-qualified
    /// fragment of A's face f" (N2).
    Fragment(Qualifier),

    // ---- Split ----
    /// A split output body (the half is the tool plane's own
    /// orientation — covariant).
    SplitBody(SplitHalf),
    /// A section face: the split-minted face on the tool plane,
    /// bounding the `side` half; `section` is the position in section
    /// completion order — a function of the recipe + the recorded
    /// `split_join_order_*` verdicts (the exact-order sort), so
    /// covariant in N4's sense (reorderings are recorded flips).
    SectionFace {
        /// Which output half this section face bounds.
        side: SplitHalf,
        /// Position in section completion order (covariance: module
        /// docs of `topo::splitting::order` — the sort runs through
        /// named exact-order predicates).
        section: u32,
    },
    /// A section-boundary edge: where the section meets an operand
    /// face (argument: the operand face's name), on the `side` half
    /// (each half owns its own coincident copy).
    SectionEdge {
        /// Which output half.
        side: SplitHalf,
        /// The operand face the section boundary runs across.
        face: NameRef,
    },
    /// A fragment of an operand entity carved by the split: faces cut
    /// by the section, edges crossing the plane. The side IS the N2
    /// discriminator (the kernel's own decided classification against
    /// the tool plane — the split node's recipe-covariant reference);
    /// same-side face multiplicity appends `Fragment(Keeps)`, and
    /// same-side edge multiplicity `Fragment(Ends)`.
    SplitFragment {
        /// Which output half holds this fragment.
        side: SplitHalf,
        /// The operand entity's name.
        parent: NameRef,
    },
    /// A crossing vertex minted where the tool plane crossed an
    /// operand edge (argument: the operand edge's name; each half
    /// keeps its own coincident copy — `side` names which).
    CrossingVertex {
        /// Which output half holds this copy.
        side: SplitHalf,
        /// The operand edge the plane crossed.
        edge: NameRef,
    },
    /// A per-half copy of an operand vertex the tool plane passed
    /// THROUGH (review R2): both halves keep a coincident copy, so
    /// the operand name alone would alias — the side tag (the
    /// kernel's own recorded side assignment, a verdict) is the
    /// discriminator. Fully birth-derived: the operand identity via
    /// the null-pair copy row, the side via which half owns the copy.
    OnToolVertex {
        /// Which output half holds this copy.
        side: SplitHalf,
        /// The operand vertex the plane passed through.
        of: NameRef,
    },

    // ---- Fillet (M6-5: the composition surgery's vocabulary) ----
    //
    // Every segment carries the SOURCE entity's OWN stable name, so a
    // fillet name composes covariantly under an upstream bump exactly
    // as the boolean emitter's `FromA`/`Seam` do: the target's names
    // move, and these move with them, without this emitter deciding
    // anything about geometry.
    /// An entity carried through from the op's target (argument: its
    /// name in the target's table) — a blend's shrunk support face,
    /// untouched edge or far vertex, or a shell's outer wall. The
    /// single-operand analogue of [`RoleSeg::FromA`], shared by every
    /// single-operand verb whose survivors keep their operand keys;
    /// which verb carried the entity is the minting node's business.
    FromTarget(NameRef),
    /// The blend face rounding a source edge.
    BlendFace(NameRef),
    /// The octant (sphere patch) rounding a source vertex.
    CornerFace(NameRef),
    /// A trimline: where a blend meets ONE of its two supports. Both
    /// arguments are needed — one source edge yields two trimlines,
    /// discriminated by which support they lie in.
    TrimEdge {
        /// The source edge being blended.
        edge: NameRef,
        /// The support face the trimline lies in.
        support: NameRef,
    },
    /// A blend foot: where a support's two trimlines meet, retracted
    /// from the source vertex where the band ends. One such vertex
    /// yields one foot per incident support, whether the band ends at
    /// a corner or at a transverse cap.
    FootVertex {
        /// The source vertex the band ends at.
        vertex: NameRef,
        /// The support face the foot lies in.
        support: NameRef,
    },
    /// **The arc where a blend band closes at a source vertex**, keyed
    /// by the source edge whose blend it bounds.
    ///
    /// A structural role, not a geometric classification. The octant
    /// seam — three convex edges, the arc parting the band from the
    /// octant [`RoleSeg::CornerFace`] names — and the transverse
    /// cut-off — a ruled band meeting a cap, the arc parting the band
    /// from the cap — are two CONFIGURATIONS of the one role, told
    /// apart by the body and by the minting node rather than by this
    /// word, the reading [`RimSupport`] states for its own pair.
    /// `(vertex, edge)` is unique under both: a source vertex is one
    /// configuration or the other and never both.
    EndArc {
        /// The source vertex the band closes at.
        vertex: NameRef,
        /// The source edge whose blend the arc bounds.
        edge: NameRef,
    },
    /// The one blend face a chain of several source edges is carved
    /// into — a CLOSED chain's torus band, or an open fillet's cylinder
    /// or chamfer's flat strip carved across joints where consecutive
    /// links lie on the same two faces
    /// (argument: the chain's source edges as a sorted set — a rim is a
    /// cycle with no distinguished first edge, and an open chain's walk
    /// order depends on which link seeded it, so the SET is the
    /// covariant identity; the N3 [`RoleSeg::Merged`] precedent, same
    /// canonical order). A one-link open band is a
    /// [`RoleSeg::BlendFace`].
    BandFace(Vec<StableName>),
    /// A band trimline on one support (a rim edge yields one per
    /// side).
    BandTrim {
        /// The source rim edge this arc replaces.
        edge: NameRef,
        /// Which support the arc lies on.
        support: RimSupport,
    },
    /// A band foot: the HOST-support vertex retracted from a source
    /// rim vertex. (Named by role, not kind: a rim between two curved
    /// walls has no planar support and still mints one — pinned by
    /// `blend5_r1_probes`.)
    BandFoot(NameRef),
    /// The vertex where the band's MATE-side trimline crossed a source
    /// edge running off the rim (on a ladder rim, a cap meridian). Both
    /// arguments are needed, for the reason [`RoleSeg::BandSlit`]
    /// states: two rims at the two ends of one meridian segment each
    /// cross that segment's seam, once per band.
    BandCross {
        /// The source edge the trimline crossed.
        edge: NameRef,
        /// The band whose trimline crossed it: its closed chain's
        /// source edges as a sorted set, the set that band's
        /// [`RoleSeg::BandFace`] carries.
        band: Vec<StableName>,
    },
    /// The surviving piece of a source edge the band's trimline cut —
    /// on a ladder rim a cap meridian, on a ruled band a cap rim edge.
    BandCut(NameRef),
    /// A band's SLIT: the double-traversed torus meridian that keeps
    /// the annular band RING-FREE (`sweep::blend::surgery`'s donut
    /// representation). Both arguments are needed: two rims at the two
    /// ends of one meridian segment each slit that segment's seam, so
    /// one source edge yields a slit per band, discriminated by which
    /// band slit it — the [`RoleSeg::BandTrim`] shape, with the band in
    /// the support's place.
    BandSlit {
        /// The source edge whose severed piece became it.
        edge: NameRef,
        /// The band that slit it: its closed chain's source edges as a
        /// sorted set, the set that band's [`RoleSeg::BandFace`]
        /// carries. A band has exactly one slit, so this alone is
        /// unique per slit.
        band: Vec<StableName>,
    },

    // ---- Shell (the hollowing verb's vocabulary) ----
    //
    // Every segment carries the SOURCE entity's OWN stable name from
    // the target's table, so a shell name composes covariantly under
    // an upstream bump exactly as the blend segments do: the target's
    // names move, and these move with them. The outer wall is not
    // here — a survivor keeps its operand key and is named
    // [`RoleSeg::FromTarget`], the blend's pass-through, because it IS
    // the same entity carried through one op.
    /// **The cavity twin of a source entity**: the face, edge or
    /// vertex the inward offset minted for the named one. A designated
    /// face's own twin dies in the rim surgery and is never named; its
    /// boundary's twins survive as the rim's ring and are named here,
    /// as twins of the boundary edges — a ring is a cycle of twins, not
    /// a role of its own.
    Inner(NameRef),
    /// **The annular rim a designated chart became**, named for the
    /// FIRST face designated on that chart — the face the chart's
    /// members merged onto, whose own name vanishes with the merge.
    /// `Rim(mouth)` is what a selector says for "the mouth's rim".
    Rim(NameRef),
    /// **The promoted rim of a designated face's HOLE**: the annulus
    /// between a hole's boundary and its cavity twin, one per hole, in
    /// the kernel's pairing order.
    HoleRim {
        /// The first designated face of the chart the hole is in.
        of: NameRef,
        /// The hole's index in the kernel's pairing order.
        hole: u32,
    },

    // ---- Instantiate part (ASM-2A D-4: the name bridge) ----
    /// An entity of an instantiated part's product, named under the
    /// instance (argument: its PART-LOCAL stable name).
    ///
    /// The instance's identity rides in the enclosing
    /// [`StableName::node`] — the instantiate node — and the referenced
    /// DOCUMENT's identity rides in that node's `doc_ref`, so neither is
    /// repeated here: one qualifier per instance is all the
    /// distinctness the part-local names need, exactly as
    /// [`RoleSeg::Instance`] carries a pattern's.
    InPart {
        /// The entity's name inside the referenced document's product.
        of: NameRef,
    },

    // ---- Pattern ----
    /// Instance `i` of the pattern's master (i is the D8-structural
    /// index — A8/N1; `of` is the master entity's name).
    Instance {
        /// The structural instance index.
        i: u32,
        /// The master entity this instance copy corresponds to.
        of: NameRef,
    },
}

/// **The band face swept from the profile piece `piece`** on the
/// revolve at `node` (in a full revolve's wire case, a curved wall's
/// `[0, π)` half) — [`RoleSeg::Band`] over the one-piece run.
///
/// This and its three siblings are the MINTING direction of the
/// vocabulary [`SegPat::tag`](crate::SegPat::tag) matches in. A
/// selection that is ANSWERED — [`select`](fn@crate::select),
/// [`all_faces`](fn@super::all_faces) — needs an evaluation to answer
/// from; a selection that is AUTHORED, a shell's open list or a
/// fillet's frozen selection, is written before any evaluation of the
/// minting node exists, so its names are spelled. Each builder fixes
/// the [`EntityKind`] its role always denotes, which is the field a
/// hand-spelled name gets wrong silently until emission refuses it.
#[must_use]
pub fn band(node: RecipeNodeId, piece: ProfileEdgeRef) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Band(piece.into())],
    }
}

/// **The `[π, 2π)` band face swept from the profile piece `piece`** —
/// [`band`]'s twin in the wire case, where a full revolve emits a
/// curved wall as two faces ([`RoleSeg::BandPi`]); a plane wall is one.
/// [`EntityKind::Face`], as [`band`] is.
#[must_use]
pub fn band_pi(node: RecipeNodeId, piece: ProfileEdgeRef) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::BandPi(piece.into())],
    }
}

/// **The latitude rim at the profile vertex `vertex`** —
/// [`RoleSeg::BandRim`], the edge between the bands of the piece
/// ending there and the piece starting there. An [`EntityKind::Edge`].
#[must_use]
pub fn band_rim(node: RecipeNodeId, vertex: ProfileVertexRef) -> StableName {
    StableName {
        kind: EntityKind::Edge,
        node,
        path: vec![RoleSeg::BandRim(vertex)],
    }
}

/// **The meridian vertex at `end`** — [`RoleSeg::MeridianVertex`]:
/// the copy of the profile vertex `vertex` on a wedge cap plane
/// ([`MeridianEnd::Start`], [`MeridianEnd::End`]) on a partial
/// revolve, or the surviving meridian vertex ([`MeridianEnd::Seam`])
/// on a full one. An [`EntityKind::Vertex`].
#[must_use]
pub fn meridian_vertex(
    end: MeridianEnd,
    node: RecipeNodeId,
    vertex: ProfileVertexRef,
) -> StableName {
    StableName {
        kind: EntityKind::Vertex,
        node,
        path: vec![RoleSeg::MeridianVertex(end, vertex)],
    }
}

/// **The name a survivor of `node` takes**: [`RoleSeg::FromTarget`]
/// of the name `inner` it had in the target's table — the
/// single-operand pass-through a blend's shrunk support or a shell's
/// outer wall wears one op later.
///
/// The kind is `inner`'s and cannot be anything else: a survivor is
/// the same entity carried through one op, so the wrapper renames it
/// without re-kinding it.
#[must_use]
pub fn carried(node: RecipeNodeId, inner: StableName) -> StableName {
    StableName {
        kind: inner.kind,
        node,
        path: vec![RoleSeg::FromTarget(NameRef::new(inner))],
    }
}

/// **The bare recipe-node id a segment carries, if any** — the third
/// question about a segment's payload, beside "which names does it
/// embed" and "which side does it name".
///
/// One variant answers today: [`RoleSeg::FromMember`]'s member edge.
/// It matters because such an id is a LOCAL node reference like the
/// minting one — it must be re-mapped when a subgraph is copied into
/// another document, fed to the naming key, and held to the document's
/// mint log when a file is read — and a walk that only visits
/// embedded NAMES cannot see it.
///
/// The match is EXHAUSTIVE on purpose (the `walk_names` rule): a
/// future variant carrying a node id must be classified here or the
/// compile breaks, rather than defaulting to "carries none" and
/// crossing a re-map with an id from another document's space.
pub(crate) fn member_edge(seg: &RoleSeg) -> Option<RecipeNodeId> {
    match seg {
        RoleSeg::FromMember { member, .. } => Some(*member),
        name_free_seg!() => None,
        RoleSeg::FromA(_)
        | RoleSeg::FromB(_)
        | RoleSeg::Seam { .. }
        | RoleSeg::Merged(_)
        | RoleSeg::Fragment(_)
        | RoleSeg::SectionEdge { .. }
        | RoleSeg::SplitFragment { .. }
        | RoleSeg::CrossingVertex { .. }
        | RoleSeg::OnToolVertex { .. }
        | RoleSeg::FromTarget(_)
        | RoleSeg::BlendFace(_)
        | RoleSeg::CornerFace(_)
        | RoleSeg::TrimEdge { .. }
        | RoleSeg::FootVertex { .. }
        | RoleSeg::EndArc { .. }
        | RoleSeg::BandFace(_)
        | RoleSeg::BandTrim { .. }
        | RoleSeg::BandFoot(_)
        | RoleSeg::BandCross { .. }
        | RoleSeg::BandCut(_)
        | RoleSeg::BandSlit { .. }
        | RoleSeg::Inner(_)
        | RoleSeg::Rim(_)
        | RoleSeg::HoleRim { .. }
        | RoleSeg::InPart { .. }
        | RoleSeg::Instance { .. } => None,
    }
}

/// **One name-carrying edge of the recipe** (N1): a node that adds no
/// segment to the names it carries, so every name it publishes from
/// below keeps its original minter. Which edge it is decides which of
/// the input's names come through.
#[derive(Debug)]
pub(crate) enum VerbatimEdge<'a> {
    /// Every body of `input`, body `k` in to body `k` out, moved: a
    /// [`Node::Transform`](crate::node::Node::Transform). A selection
    /// in effect above it rides through to `input` unchanged.
    Whole {
        /// The value placed.
        input: RecipeNodeId,
    },
    /// The one body of `of` that `select` names, projected: a
    /// [`Node::Part`](crate::node::Node::Part).
    Selected {
        /// The split or pattern read.
        of: RecipeNodeId,
        /// Which body of it.
        select: &'a crate::node::PartSelect,
    },
    /// The entities of the split target the split leaves intact: a
    /// [`Node::Split`](crate::node::Node::Split). Which ones those are
    /// is the geometry's answer, not the recipe's, so no walk follows
    /// this edge and it carries no input.
    Intact,
}

/// **The name-carrying edge `node` is, if any**: a transform, a part's
/// projection, a split's intact entities (N1's pass-through ops).
/// Every other node is classified as re-minting what it carries.
///
/// Two walks read the set here: the product's two-roots check
/// (`product::placed_under_two_roots`) and the mate member walk
/// (`mate::member::walk`); they differ only in where each stops. The
/// compiler holds the three together: this match is exhaustive, so a
/// new node kind does not compile until it is classified here, and
/// both walks match [`VerbatimEdge`] without a wildcard, so a new kind
/// of edge does not compile until each decides what to do with it.
///
/// What the compiler cannot hold — that this classification agrees
/// with what the evaluator actually passes through (`eval::wire`'s
/// `wire_transform`, `wire_part`, `wire_split`) — is held at runtime
/// by `tests/names_verbatim_edge_evaluator.rs`, per edge over an
/// evaluated corpus: a `Whole` or `Selected` node publishes only names
/// headed by other nodes, an `Intact` one publishes both kinds, and a
/// `None` node heads every row itself. Every node kind the corpus can
/// evaluate is sampled with rows, except the kinds that publish none
/// at all, which that suite names and holds at zero.
pub(crate) fn verbatim_edge<P>(node: &crate::node::Node<P>) -> Option<VerbatimEdge<'_>> {
    use crate::node::Node;
    match node {
        Node::Transform { input, .. } => Some(VerbatimEdge::Whole { input: *input }),
        Node::Part { of, select } => Some(VerbatimEdge::Selected { of: *of, select }),
        Node::Split { .. } => Some(VerbatimEdge::Intact),
        Node::Datum(_)
        | Node::Profile(_)
        | Node::Extrude { .. }
        | Node::Revolve { .. }
        | Node::Tube { .. }
        | Node::HollowTube { .. }
        | Node::Loft { .. }
        | Node::Sweep { .. }
        | Node::Fillet { .. }
        | Node::Chamfer { .. }
        | Node::Shell { .. }
        | Node::Boolean { .. }
        | Node::Union { .. }
        | Node::Pattern { .. }
        | Node::PlacedUnion { .. }
        | Node::InstantiatePart { .. }
        | Node::Gauge { .. }
        | Node::Mate { .. }
        | Node::Measure { .. }
        | Node::Assertion { .. } => None,
    }
}

/// The [`RoleSeg`] variants that embed no [`StableName`], as a
/// PATTERN rather than a predicate.
///
/// Several matches classify segments by this partition and each does
/// something different with the other half — the name walk visits,
/// the selector collects, the member-edge read answers `None`, and
/// the attribution walk ([`fn@super::attribute`]) stops. Only the
/// negative answer is common, so only the negative answer is shared,
/// and it is shared as an or-pattern so that none of them loses its
/// exhaustiveness: a variant added to [`RoleSeg`] and not added here
/// breaks every one of those builds, exactly as spelling the list out
/// at each site did. What changes is that classifying it name-free is
/// ONE decision at one site instead of one per match that can be made
/// differently. (No count of those matches is carried here: a count
/// in prose has gone stale before, and `rg 'name_free_seg!'` is the
/// census.)
///
/// It is the union of two lists that are each ONE decision of their
/// own: [`inert_seg`], the variants that carry neither a name nor a
/// profile locator, and [`locator_seg`], the variants that carry a
/// [`ProfileEdgeRef`] or a [`ProfileVertexRef`] directly. The one
/// rewrite walk ([`RoleSeg::rewrite`]) needs the two halves apart —
/// it passes a locator through its rewriter and an inert segment
/// verbatim — and the compiler refuses the union as an arm after the
/// locator arms (`unreachable_patterns` fires on a covered
/// alternative of an or-pattern), so the split is what lets the walk
/// share the decision rather than re-spell it.
///
/// A `fn` returning `bool` would not do: a caller may forget to call
/// a predicate, and the property this list carries is the one the
/// compiler holds.
macro_rules! name_free_seg {
    () => {
        $crate::names::inert_seg!() | $crate::names::locator_seg!()
    };
}

/// The [`RoleSeg`] variants that carry NEITHER a [`StableName`] nor a
/// profile locator: the sweep, split and section primitives a rewrite
/// of either kind crosses verbatim. One half of [`name_free_seg`].
macro_rules! inert_seg {
    () => {
        $crate::names::RoleSeg::OutputBody
            | $crate::names::RoleSeg::Cap(_)
            | $crate::names::RoleSeg::RevolveCap(_)
            | $crate::names::RoleSeg::SplitBody(_)
            | $crate::names::RoleSeg::SectionFace { .. }
    };
}

/// The [`RoleSeg`] variants that carry a [`ProfileEdgeRef`] or a
/// [`ProfileVertexRef`] DIRECTLY — the ones an extrude, revolve or
/// loft emitter mints, the ones `eval::anchor` re-anchors and the ones
/// the whole-program edit's segment map moves. The other half of
/// [`name_free_seg`]: none of them embeds a name. A variant that
/// begins to carry a locator has to be added here or every rewrite
/// walk stops compiling, which is the point.
macro_rules! locator_seg {
    () => {
        $crate::names::RoleSeg::Lateral(_)
            | $crate::names::RoleSeg::RimEdge(..)
            | $crate::names::RoleSeg::LateralEdge(_)
            | $crate::names::RoleSeg::CapVertex(..)
            | $crate::names::RoleSeg::LoftWall(_)
            | $crate::names::RoleSeg::LoftSeam(_)
            | $crate::names::RoleSeg::Band(_)
            | $crate::names::RoleSeg::BandRim(_)
            | $crate::names::RoleSeg::BandRimPi(_)
            | $crate::names::RoleSeg::BandPi(_)
            | $crate::names::RoleSeg::Meridian(..)
            | $crate::names::RoleSeg::MeridianVertex(..)
            | $crate::names::RoleSeg::Pole(_)
            | $crate::names::RoleSeg::AxisEdge(_)
    };
}

pub(crate) use inert_seg;
pub(crate) use locator_seg;
pub(crate) use name_free_seg;

/// **What a rewrite of a role path does to each thing a segment
/// carries** — the four holes in the one walk [`RoleSeg::rewrite`]
/// makes over [`RoleSeg`]'s shape.
///
/// Three walks of a name's path exist: `refactor`'s re-mapping of node
/// and step ids across a split or an inline, the union emitter's
/// citing of whole member edges, and [`StableName::piece_steps`]'s
/// collection of the steps a name spells. What differs between them is
/// only what each does with a locator, a carried name and a member
/// edge; what they share — which variant carries which, and putting
/// the rewritten path back in canonical form
/// ([`StableName::rewrite_path`], through `names::canonical`) — is the
/// walk, written once. This trait is the part that differs.
///
/// Every method defaults to the identity, because "not this rewrite's
/// concern" IS the identity: the union's citing moves member edges and
/// nothing else, the step collection reads locators and moves nothing.
/// A rewrite that descends into a carried name says so from its own
/// [`SegRewrite::name`] ([`Carry::Descend`]); the walk then rewrites
/// that name's path through the same rewriter and hands the result to
/// [`SegRewrite::descended`]. A rewriter that must not descend (the
/// union's: a member's own name is final in the member) simply does
/// not say so. The walk keeps the names it is descending on its own
/// stack ([`StableName::rewrite_path`]), since a name nests as deep as
/// its derivation.
pub(crate) trait SegRewrite {
    /// What stops the rewrite; [`core::convert::Infallible`] where
    /// nothing can.
    type Error;

    /// A profile edge locator the segment carries directly.
    ///
    /// # Errors
    ///
    /// The rewriter's own.
    fn edge(&mut self, e: ProfileEdgeRef) -> Result<ProfileEdgeRef, Self::Error> {
        Ok(e)
    }

    /// A profile vertex locator the segment carries directly.
    ///
    /// # Errors
    ///
    /// The rewriter's own.
    fn vertex(&mut self, v: ProfileVertexRef) -> Result<ProfileVertexRef, Self::Error> {
        Ok(v)
    }

    /// What becomes of a name the segment carries ([`Carry`]).
    ///
    /// # Errors
    ///
    /// The rewriter's own.
    fn name(&mut self, n: &StableName) -> Result<Carry, Self::Error> {
        let _ = n;
        Ok(Carry::Keep)
    }

    /// A carried name this rewriter descends into, once its path has
    /// been rewritten through it (`walked`): `None` leaves `n` as it
    /// is, `Some` replaces it.
    ///
    /// # Errors
    ///
    /// The rewriter's own.
    fn descended(
        &mut self,
        n: &StableName,
        walked: StableName,
    ) -> Result<Option<StableName>, Self::Error> {
        let _ = n;
        Ok(Some(walked))
    }

    /// The member edge of a [`RoleSeg::FromMember`] — a bare node id,
    /// which is not a name and which a rewrite of the document's id
    /// space has to move too.
    ///
    /// # Errors
    ///
    /// The rewriter's own.
    fn member(&mut self, m: RecipeNodeId) -> Result<RecipeNodeId, Self::Error> {
        Ok(m)
    }
}

/// What a rewrite does with a name a segment carries.
pub(crate) enum Carry {
    /// Leave it as it is (and cost no clone).
    Keep,
    /// Put this name in its place.
    Replace(StableName),
    /// Rewrite its path through the same rewriter, and put what
    /// [`SegRewrite::descended`] answers in its place.
    Descend,
}

/// A rewriter, with the answers for the carried names already
/// descended.
struct Deep<'d, 's, W: SegRewrite> {
    w: &'d mut W,
    kept: &'d Kept<Option<StableName>>,
    /// While a level lists the names it descends into first
    /// ([`Descent::first`]): a carried name not descended yet is listed
    /// and left as it is, and the walk goes on.
    listing: Option<&'d mut Vec<&'s StableName>>,
}

impl<'s, W: SegRewrite> Deep<'_, 's, W> {
    /// `n` through the rewriter: `None` where it is kept.
    fn carried(&mut self, n: &'s StableName) -> Result<Option<StableName>, Stopped<'s, W::Error>> {
        match self.w.name(n).map_err(Stopped::Refused)? {
            Carry::Keep => Ok(None),
            Carry::Replace(next) => Ok(Some(next)),
            Carry::Descend => match (self.kept.get(n), &mut self.listing) {
                (Some(answer), _) => Ok(answer.clone()),
                (None, Some(listed)) => {
                    listed.push(n);
                    Ok(None)
                }
                (None, None) => Err(Stopped::Needs(n)),
            },
        }
    }

    fn edge(&mut self, e: ProfileEdgeRef) -> Result<ProfileEdgeRef, Stopped<'s, W::Error>> {
        self.w.edge(e).map_err(Stopped::Refused)
    }

    fn vertex(&mut self, v: ProfileVertexRef) -> Result<ProfileVertexRef, Stopped<'s, W::Error>> {
        self.w.vertex(v).map_err(Stopped::Refused)
    }

    fn member(&mut self, m: RecipeNodeId) -> Result<RecipeNodeId, Stopped<'s, W::Error>> {
        self.w.member(m).map_err(Stopped::Refused)
    }
}

/// [`StableName::rewrite_path`] as a [`Descent`]: a level is its path
/// rebuilt through the rewriter, and a carried name it descends into is
/// kept as [`SegRewrite::descended`] answers it.
struct Rewriting<'w, W>(&'w mut W);

impl<'s, W: SegRewrite> Descent<'s> for Rewriting<'_, W> {
    type Level = StableName;
    type Kept = Option<StableName>;
    type Error = W::Error;

    /// The level walked as far as the rewriter lets it, every carried
    /// name it descends into listed in the order the walk meets it. A
    /// refusal ends the list: the level's own run meets it again after
    /// the names listed before it are descended, which is where a walk
    /// that recursed would have met it.
    fn first(
        &mut self,
        name: &'s StableName,
        kept: &Kept<Option<StableName>>,
        out: &mut Vec<&'s StableName>,
    ) {
        let listed = name.walk(&mut Deep {
            w: self.0,
            kept,
            listing: Some(out),
        });
        drop(listed);
    }

    fn level(
        &mut self,
        name: &'s StableName,
        kept: &Kept<Option<StableName>>,
    ) -> Result<StableName, Stopped<'s, W::Error>> {
        name.walk(&mut Deep {
            w: self.0,
            kept,
            listing: None,
        })
    }

    fn keep(
        &mut self,
        name: &'s StableName,
        walked: StableName,
    ) -> Result<Option<StableName>, W::Error> {
        self.0.descended(name, walked)
    }
}

/// One carried name through the rewriter, kept as it is where the
/// rewriter leaves it.
fn rewrite_ref<'s, W: SegRewrite>(
    n: &'s NameRef,
    w: &mut Deep<'_, 's, W>,
) -> Result<NameRef, Stopped<'s, W::Error>> {
    Ok(match w.carried(n.name())? {
        Some(next) => NameRef::new(next),
        None => n.clone(),
    })
}

/// A SET of names through the rewriter, each kept as it is where the
/// rewriter leaves it. The set's order is the path's canonical form's
/// to restore, not this walk's.
fn rewrite_set<'s, W: SegRewrite>(
    v: &'s [StableName],
    w: &mut Deep<'_, 's, W>,
) -> Result<Vec<StableName>, Stopped<'s, W::Error>> {
    v.iter()
        .map(|n| Ok(w.carried(n)?.unwrap_or_else(|| n.clone())))
        .collect()
}

impl RoleSeg {
    /// **This segment rebuilt through `w`** — the one walk over
    /// [`RoleSeg`]'s shape that every rewrite of a role path goes
    /// through ([`SegRewrite`] says which three).
    ///
    /// A segment rebuilt alone is NOT canonical: its sets, its `Borders`
    /// walls and a union seam's sides are in whatever order the
    /// rewrite left them, and a rank may lie along a line the rewrite
    /// reversed. So the walk is private to [`StableName::rewrite_path`],
    /// which rebuilds the whole path and puts it in canonical form.
    ///
    /// The match is EXHAUSTIVE on purpose, with no wildcard (the
    /// `walk_names` rule): a variant added to [`RoleSeg`] says here
    /// which of the three things it carries — a locator, a name, or
    /// neither — or stops the build. A catch-all would let a new
    /// locator cross a re-anchor stale, or a new carried name cross a
    /// split with an id from another document's space, silently. The
    /// inert half is [`inert_seg`]'s one decision; the locator half is
    /// [`locator_seg`]'s list, spelled arm by arm because each arm
    /// rebuilds its own payload.
    ///
    /// `InPart` crosses verbatim under every rewriter: its argument
    /// names ANOTHER document's nodes (the document seam), whose
    /// profiles and id space are not this document's to rewrite.
    ///
    /// # Errors
    ///
    /// Whatever `w` refuses, at the first thing it refuses.
    #[allow(clippy::too_many_lines)] // one arm per RoleSeg variant, each short
    fn rewrite<'s, W: SegRewrite>(
        &'s self,
        w: &mut Deep<'_, 's, W>,
    ) -> Result<RoleSeg, Stopped<'s, W::Error>> {
        use RoleSeg as R;
        Ok(match self {
            // Neither a locator nor a name: verbatim.
            inert_seg!() => self.clone(),
            // The locators.
            R::Lateral(run) => R::Lateral(run.try_map(|e| w.edge(e))?),
            R::RimEdge(c, e) => R::RimEdge(*c, w.edge(*e)?),
            R::LateralEdge(v) => R::LateralEdge(w.vertex(*v)?),
            R::CapVertex(c, v) => R::CapVertex(*c, w.vertex(*v)?),
            R::LoftWall(es) => {
                R::LoftWall(es.iter().map(|e| w.edge(*e)).collect::<Result<_, _>>()?)
            }
            R::LoftSeam(vs) => {
                R::LoftSeam(vs.iter().map(|v| w.vertex(*v)).collect::<Result<_, _>>()?)
            }
            R::Band(run) => R::Band(run.try_map(|e| w.edge(e))?),
            R::BandRim(v) => R::BandRim(w.vertex(*v)?),
            R::BandRimPi(v) => R::BandRimPi(w.vertex(*v)?),
            R::BandPi(run) => R::BandPi(run.try_map(|e| w.edge(e))?),
            R::Meridian(m, run) => R::Meridian(*m, run.try_map(|e| w.edge(e))?),
            R::MeridianVertex(m, v) => R::MeridianVertex(*m, w.vertex(*v)?),
            R::Pole(v) => R::Pole(w.vertex(*v)?),
            R::AxisEdge(e) => R::AxisEdge(w.edge(*e)?),
            // The carried names.
            R::FromA(n) => R::FromA(rewrite_ref(n, w)?),
            R::FromB(n) => R::FromB(rewrite_ref(n, w)?),
            // BOTH halves: the member edge is a local node id like the
            // minting one, and a rewrite of the id space moves it too.
            R::FromMember { member, of } => R::FromMember {
                member: w.member(*member)?,
                of: rewrite_ref(of, w)?,
            },
            R::Seam { a, b } => R::Seam {
                a: rewrite_ref(a, w)?,
                b: rewrite_ref(b, w)?,
            },
            R::Merged(v) => R::Merged(rewrite_set(v, w)?),
            R::Fragment(q) => R::Fragment(match q {
                Qualifier::Borders(walls) => Qualifier::Borders(rewrite_set(walls, w)?),
                Qualifier::Keeps(edges) => Qualifier::Keeps(rewrite_set(edges, w)?),
                Qualifier::Ends(ends) => Qualifier::Ends(rewrite_set(ends, w)?),
                Qualifier::OrderAlong { .. } => q.clone(),
            }),
            R::SectionEdge { side, face } => R::SectionEdge {
                side: *side,
                face: rewrite_ref(face, w)?,
            },
            R::SplitFragment { side, parent } => R::SplitFragment {
                side: *side,
                parent: rewrite_ref(parent, w)?,
            },
            R::CrossingVertex { side, edge } => R::CrossingVertex {
                side: *side,
                edge: rewrite_ref(edge, w)?,
            },
            R::OnToolVertex { side, of } => R::OnToolVertex {
                side: *side,
                of: rewrite_ref(of, w)?,
            },
            R::FromTarget(n) => R::FromTarget(rewrite_ref(n, w)?),
            R::BlendFace(n) => R::BlendFace(rewrite_ref(n, w)?),
            R::CornerFace(n) => R::CornerFace(rewrite_ref(n, w)?),
            R::TrimEdge { edge, support } => R::TrimEdge {
                edge: rewrite_ref(edge, w)?,
                support: rewrite_ref(support, w)?,
            },
            R::FootVertex { vertex, support } => R::FootVertex {
                vertex: rewrite_ref(vertex, w)?,
                support: rewrite_ref(support, w)?,
            },
            R::EndArc { vertex, edge } => R::EndArc {
                vertex: rewrite_ref(vertex, w)?,
                edge: rewrite_ref(edge, w)?,
            },
            R::BandFace(v) => R::BandFace(rewrite_set(v, w)?),
            R::BandTrim { edge, support } => R::BandTrim {
                edge: rewrite_ref(edge, w)?,
                support: *support,
            },
            R::BandFoot(n) => R::BandFoot(rewrite_ref(n, w)?),
            R::BandCross { edge, band } => R::BandCross {
                edge: rewrite_ref(edge, w)?,
                band: rewrite_set(band, w)?,
            },
            R::BandCut(n) => R::BandCut(rewrite_ref(n, w)?),
            R::BandSlit { edge, band } => R::BandSlit {
                edge: rewrite_ref(edge, w)?,
                band: rewrite_set(band, w)?,
            },
            R::Inner(n) => R::Inner(rewrite_ref(n, w)?),
            R::Rim(n) => R::Rim(rewrite_ref(n, w)?),
            R::HoleRim { of, hole } => R::HoleRim {
                of: rewrite_ref(of, w)?,
                hole: *hole,
            },
            // The document seam.
            R::InPart { .. } => self.clone(),
            R::Instance { i, of } => R::Instance {
                i: *i,
                of: rewrite_ref(of, w)?,
            },
        })
    }
}

impl StableName {
    /// This name with every segment of its path rebuilt through `w`
    /// ([`RoleSeg::rewrite`]), then put back in canonical form; the
    /// kind and the minting node are not the path's and are kept.
    ///
    /// The canonical form is `names::canonical`'s, the one the emitters
    /// mint: a rewrite that moves the names in a name-ordered position
    /// (a set, a `Borders` set, a junction's run, a union seam's two
    /// sides) can change their order, and a name the emitter would not
    /// mint for the same entity resolves to nothing. A seam whose sides
    /// come out swapped — in this name, or in a name it embeds — reverses
    /// the ranks along its line, so the rule is read from the name as it
    /// was and as it is, with the images `w` gives the seam's sides
    /// (`names::canonical::rewritten`).
    ///
    /// A carried name `w` descends into ([`Carry::Descend`]) is
    /// rewritten the same way, and a name nests as deep as its
    /// derivation, so the walk keeps the names it is descending on its
    /// own stack (`names::nest::descend`): a level first lists the
    /// carried names it descends into, each is descended, and the level
    /// is then walked once, every name already descended answered from
    /// what was kept. A rewriter is asked the same questions twice on a
    /// level, so what it answers must be a function of the question.
    ///
    /// # Errors
    ///
    /// Whatever `w` refuses, at the first thing it refuses.
    pub(crate) fn rewrite_path<W: SegRewrite>(self, w: &mut W) -> Result<StableName, W::Error> {
        descend(&self, &mut Rewriting(w))
    }

    /// One level of [`StableName::rewrite_path`]: this name's path
    /// rebuilt through `w`, in canonical form. While the level only
    /// lists what it descends into, the canonical form is left out: it
    /// asks after the path, and a name it asks for that the path did not
    /// list stops the level's own run instead.
    fn walk<'s, W: SegRewrite>(
        &'s self,
        w: &mut Deep<'_, 's, W>,
    ) -> Result<StableName, Stopped<'s, W::Error>> {
        let path = self
            .path
            .iter()
            .map(|seg| seg.rewrite(w))
            .collect::<Result<_, _>>()?;
        let StableName { kind, node, .. } = self;
        let now = StableName {
            kind: *kind,
            node: *node,
            path,
        };
        if w.listing.is_some() {
            return Ok(now);
        }
        canonical::rewritten(self, now, &mut |n| {
            Ok(w.carried(n)?.unwrap_or_else(|| n.clone()))
        })
    }

    /// **Every authored step this name spells a piece of** — in its own
    /// path and in every name it carries from this document. An
    /// `InPart` argument names ANOTHER document's steps (the document
    /// seam), so it is not read: its ids are not this document's.
    ///
    /// What a `SetProgram` that drops a step asks of each name the
    /// document holds (DM7): a step id is unique across the document,
    /// so a name that spells it is a name on that step's pieces,
    /// whichever node minted the name.
    pub(crate) fn piece_steps(&self) -> std::collections::BTreeSet<StepId> {
        self.step_pieces()
            .iter()
            .filter_map(ProfileEdgeRef::step)
            .collect()
    }

    /// **Every authored step's piece this name spells**, over the same
    /// walk as [`Self::piece_steps`]: an edge locator as itself, a
    /// vertex locator as the piece that starts there (N1: a vertex is
    /// named by the piece of the same spelling), a kernel-built
    /// section's not at all.
    ///
    /// What a `SetProgram` that keeps a step asks of each name the
    /// document holds (DM7): whether the new program still draws the
    /// piece the name spells.
    pub(crate) fn step_pieces(&self) -> std::collections::BTreeSet<ProfileEdgeRef> {
        let mut pieces = StepPieces(std::collections::BTreeSet::new());
        let Ok(_) = self.clone().rewrite_path(&mut pieces);
        pieces.0
    }
}

/// [`StableName::step_pieces`]'s walk: every step locator collected
/// as the piece it names, every carried name descended, nothing
/// rewritten.
struct StepPieces(std::collections::BTreeSet<ProfileEdgeRef>);

impl SegRewrite for StepPieces {
    type Error = core::convert::Infallible;

    fn edge(&mut self, e: ProfileEdgeRef) -> Result<ProfileEdgeRef, Self::Error> {
        if let ProfileEdgeRef::Piece { .. } = e {
            self.0.insert(e);
        }
        Ok(e)
    }

    fn vertex(&mut self, v: ProfileVertexRef) -> Result<ProfileVertexRef, Self::Error> {
        if let ProfileVertexRef::Piece { step, role } = v {
            self.0.insert(ProfileEdgeRef::Piece { step, role });
        }
        Ok(v)
    }

    fn name(&mut self, _: &StableName) -> Result<Carry, Self::Error> {
        Ok(Carry::Descend)
    }

    fn descended(
        &mut self,
        _: &StableName,
        _: StableName,
    ) -> Result<Option<StableName>, Self::Error> {
        Ok(None)
    }
}
/// The [`RoleSeg`] variants a BOOLEAN emitter never mints, as a
/// PATTERN rather than a predicate.
///
/// A union's value is a fold of the pair verb, so a fold table is a
/// boolean table and this is the same list for both. Two matches
/// classify segments by it and each does something different with the
/// half it does recognize — the rewrite descends a name's head
/// (`emit_union`'s `collapse`) and rebuilds its tail. Only the
/// negative answer is common, so only the negative answer is
/// shared, and it is shared as an or-pattern for the reason
/// [`name_free_seg`] is: neither match loses its exhaustiveness, so a
/// variant added to [`RoleSeg`] and not added here still stops the
/// build. What changes is that "the boolean emitter does not mint
/// this" is ONE decision at one site instead of two that can be made
/// differently.
///
/// The seven it leaves out are the boolean table's own vocabulary:
/// [`RoleSeg::OutputBody`], [`RoleSeg::FromA`], [`RoleSeg::FromB`],
/// [`RoleSeg::FromMember`], [`RoleSeg::Seam`], [`RoleSeg::Merged`]
/// and [`RoleSeg::Fragment`]. Each of the two matches decides those
/// for itself, because that is exactly where they differ:
/// `FromA`/`FromB` are the fold's INTERNAL space (descended through
/// by the rewrite), a `Seam` heads a name alone or as a junction's
/// run of lines, and a `Fragment` is a tail segment rather than a head
/// one.
macro_rules! never_in_a_boolean_table {
    () => {
        $crate::names::RoleSeg::Cap(_)
            | $crate::names::RoleSeg::Lateral(_)
            | $crate::names::RoleSeg::RimEdge(..)
            | $crate::names::RoleSeg::LateralEdge(_)
            | $crate::names::RoleSeg::CapVertex(..)
            | $crate::names::RoleSeg::LoftWall(_)
            | $crate::names::RoleSeg::LoftSeam(_)
            | $crate::names::RoleSeg::Band(_)
            | $crate::names::RoleSeg::BandRim(_)
            | $crate::names::RoleSeg::BandRimPi(_)
            | $crate::names::RoleSeg::BandPi(_)
            | $crate::names::RoleSeg::Meridian(..)
            | $crate::names::RoleSeg::MeridianVertex(..)
            | $crate::names::RoleSeg::RevolveCap(_)
            | $crate::names::RoleSeg::Pole(_)
            | $crate::names::RoleSeg::AxisEdge(_)
            | $crate::names::RoleSeg::SplitBody(_)
            | $crate::names::RoleSeg::SectionFace { .. }
            | $crate::names::RoleSeg::SectionEdge { .. }
            | $crate::names::RoleSeg::SplitFragment { .. }
            | $crate::names::RoleSeg::CrossingVertex { .. }
            | $crate::names::RoleSeg::OnToolVertex { .. }
            | $crate::names::RoleSeg::FromTarget(_)
            | $crate::names::RoleSeg::BlendFace(_)
            | $crate::names::RoleSeg::CornerFace(_)
            | $crate::names::RoleSeg::TrimEdge { .. }
            | $crate::names::RoleSeg::FootVertex { .. }
            | $crate::names::RoleSeg::EndArc { .. }
            | $crate::names::RoleSeg::BandFace(_)
            | $crate::names::RoleSeg::BandTrim { .. }
            | $crate::names::RoleSeg::BandFoot(_)
            | $crate::names::RoleSeg::BandCross { .. }
            | $crate::names::RoleSeg::BandCut(_)
            | $crate::names::RoleSeg::BandSlit { .. }
            | $crate::names::RoleSeg::Inner(_)
            | $crate::names::RoleSeg::Rim(_)
            | $crate::names::RoleSeg::HoleRim { .. }
            | $crate::names::RoleSeg::InPart { .. }
            | $crate::names::RoleSeg::Instance { .. }
    };
}

pub(crate) use never_in_a_boolean_table;

#[cfg(test)]
mod tests {
    use super::{
        EntityKind, MeridianEnd, NameRef, PieceRole, ProfileEdgeRef, RoleSeg, SectionCircle,
        StableName, band, band_pi, band_rim, carried, meridian_vertex,
    };
    use crate::node::{RecipeNodeId, StepId};

    /// The node every pin below mints against.
    const N: RecipeNodeId = RecipeNodeId(7);

    /// The two locator forms every pin below is written at: an
    /// authored piece and a kernel-built section's piece.
    fn edges() -> [ProfileEdgeRef; 2] {
        [
            ProfileEdgeRef::Piece {
                step: StepId(3),
                role: PieceRole::RunOut,
            },
            ProfileEdgeRef::Section {
                circle: SectionCircle::Bore,
                role: PieceRole::Piece(1),
            },
        ]
    }

    /// A builder mints EXACTLY the name a caller would spell by hand.
    /// Four pins, one per builder, each written the long way — the
    /// spelling they replace at their consumers — at both locator
    /// forms, so a builder cannot drift from the vocabulary without
    /// this file disagreeing with itself.
    #[test]
    fn band_mints_the_hand_spelled_face() {
        for e in edges() {
            assert_eq!(
                band(N, e),
                StableName {
                    kind: EntityKind::Face,
                    node: N,
                    path: vec![RoleSeg::Band(e.into())],
                }
            );
        }
    }

    #[test]
    fn band_pi_mints_the_hand_spelled_face() {
        for e in edges() {
            assert_eq!(
                band_pi(N, e),
                StableName {
                    kind: EntityKind::Face,
                    node: N,
                    path: vec![RoleSeg::BandPi(e.into())],
                }
            );
        }
    }

    #[test]
    fn band_rim_mints_the_hand_spelled_edge() {
        for e in edges() {
            assert_eq!(
                band_rim(N, e.start()),
                StableName {
                    kind: EntityKind::Edge,
                    node: N,
                    path: vec![RoleSeg::BandRim(e.start())],
                }
            );
        }
    }

    #[test]
    fn meridian_vertex_mints_the_hand_spelled_vertex() {
        for e in edges() {
            assert_eq!(
                meridian_vertex(MeridianEnd::Seam, N, e.start()),
                StableName {
                    kind: EntityKind::Vertex,
                    node: N,
                    path: vec![RoleSeg::MeridianVertex(MeridianEnd::Seam, e.start())],
                }
            );
        }
    }

    /// **A piece's vertex is spelled as the piece is**: the vertex
    /// locator a piece starts at carries the same form, step or circle,
    /// and role — and its wire text is the edge's, which is what lets
    /// one piece text name both.
    #[test]
    #[allow(clippy::expect_used)]
    fn a_pieces_start_is_spelled_as_the_piece() {
        for e in edges() {
            let v = e.start();
            assert_eq!(v.step(), e.step());
            assert_eq!(
                serde_json::to_string(&v).expect("serializes"),
                serde_json::to_string(&e).expect("serializes")
            );
        }
        assert_eq!(
            serde_json::to_string(&edges()[0]).expect("serializes"),
            r#"{"Piece":{"step":3,"role":"RunOut"}}"#
        );
        assert_eq!(
            serde_json::to_string(&edges()[1]).expect("serializes"),
            r#"{"Section":{"circle":"Bore","role":{"Piece":1}}}"#
        );
    }

    /// **A name's piece steps are its own path's and every carried
    /// name's of this document, and never an `InPart` argument's** —
    /// the other document's ids are not this one's.
    #[test]
    fn piece_steps_read_the_names_own_document_only() {
        let wall = |node: u64, step: u64| StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(node),
            path: vec![RoleSeg::Lateral(
                ProfileEdgeRef::Piece {
                    step: StepId(step),
                    role: PieceRole::Leg,
                }
                .into(),
            )],
        };
        let carried_wall = carried(RecipeNodeId(9), wall(1, 4));
        assert_eq!(
            carried_wall.piece_steps().into_iter().collect::<Vec<_>>(),
            vec![StepId(4)]
        );
        let foreign = StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(9),
            path: vec![RoleSeg::InPart {
                of: NameRef::new(wall(1, 5)),
            }],
        };
        assert!(foreign.piece_steps().is_empty());
        let section = band(N, edges()[1]);
        assert!(section.piece_steps().is_empty(), "a section has no step");
    }

    /// `carried` takes the INNER name's kind, which is the one field
    /// of a pass-through wrapper a caller can get wrong: the pin
    /// wraps an edge and asserts the wrapper is an edge.
    #[test]
    fn carried_mints_the_hand_spelled_wrapper_and_keeps_the_kind() {
        let inner = band_rim(N, edges()[0].start());
        let outer = RecipeNodeId(9);
        assert_eq!(
            carried(outer, inner.clone()),
            StableName {
                kind: EntityKind::Edge,
                node: outer,
                path: vec![RoleSeg::FromTarget(NameRef::new(inner))],
            }
        );
    }
}
