//! **The kernel's verb vocabulary** — one closed enum naming the
//! operations that have been migrated onto it, with the run dispatch and
//! the parameter→field flow that only the operation itself knows.
//!
//! **`Verb` here is the KERNEL's verb — an operation on a body.**
//! [`profile::Verb`] is the sketch program's — which transition a
//! [`profile::Step`] takes — and the two never meet in one signature;
//! every reader outside the crate that owns one spells the crate
//! (`verbs::Verb`, `profile::Verb`).
//!
//! # What it is NOT, yet
//!
//! It is not "every operation a recipe door can invoke", and reading it
//! that way would misjudge every later unit's cost. **Seven verbs in
//! six record families live here**: the blend pair, the two sweeps
//! (extrude and revolve), the boolean's three regularized ops, the
//! split and the shell. Every other door — transform, pattern, loft,
//! sweep along a path, measure — still runs the way it always did, and
//! is reached by `editor-core`'s lowering calling its op crate
//! directly. This crate is the SEAT the rest migrate onto, not a
//! description of where they are.
//!
//! **And a verb here need not be a verb the DOCUMENT can author.** The
//! vocabulary is complete per verb — its door, its record channel and
//! its refusal — whether or not a `Node` builds it, and every commitment
//! keyed on the vocabulary states what a kernel-only verb means rather
//! than skipping the row (`editor-core`'s content tag is an `Option`
//! for exactly that). Today every verb has a node, the shell's the last
//! to land; the shape stays for the next verb that ships kernel-first.
//!
//! The design's cost claim is scoped the same way and is not
//! demonstrated here: what these units show is that the migrated verbs
//! share the correspondence pattern and one tag function. Whether that
//! reduces the price of the NEXT verb is measured at the next verb.
//!
//! # What lives here, and what may never
//!
//! A [`Verb`] is the operation's parameters reified as plain data:
//! scalars at `T`, entity references as arena keys. Operand bodies are
//! NOT in the payload — they are borrowed at run time, and the
//! declaration states which door answers the verb instead
//! ([`VerbKind::arity`]: one body in and one out for the blends, two
//! bodies for the boolean, one validated PROFILE for the sweeps, one
//! body in and TWO sides out for the split, and the shell's own row —
//! the blends' two ends at a scalar that can certify — each behind its
//! own typed door). Everything else a verb is committed to belongs to
//! whoever owns that commitment, not here: the content-key tag beside
//! `editor-core`'s memo machinery, the wire spelling on `Node`'s serde
//! derives, the Python constructor, the viewer's tree label. This
//! crate says nothing about any of them.
//!
//! That is the layering line drawn once and enforced by
//! `tests/layer_guard.rs`: no serde, no `Expr`, no `StableName`, no
//! `RecipeNodeId`. Those are the recipe vocabulary and they live above.
//! What may sit beside the arenas is LOWERED pure data compared only
//! for identity — the `GeomSource` precedent in `topo/src/source.rs`.
//!
//! # Why a crate of its own
//!
//! The vocabulary spans crates: the blend pair and the two sweeps are
//! `sweep`'s, the boolean and the split are `topo`'s, and more
//! follow. Hosting the enum in either op crate would make one of
//! them name the other's ops, so it sits above both and below
//! `editor-core`, which is the only consumer.
//!
//! # What the birth record is for
//!
//! [`VerbOut`] carries the operation's per-entity birth record beside
//! the body, in the record channel for the verb's family
//! ([`VerbRecord`]); [`SplitOut`] carries the split's beside its two
//! sides. A verb without a birth channel cannot join this
//! enum: the record is what lets the document layer mint
//! derivation-path names for what the operation created, and an
//! operation whose output cannot be named is one no recipe can build
//! on.

pub mod flow;
mod run;
mod verb;

pub use flow::{EdgeScalar, FieldRole, FlowSource, ParamFlow, RoleFamily, ScalarParam};
pub use run::{PairOut, SplitOut, VerbError, VerbOut, VerbRecord};
pub use verb::{Arity, Verb, VerbKind};
