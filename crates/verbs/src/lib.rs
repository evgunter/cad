//! **The kernel's verb vocabulary** — one closed enum naming the
//! operations that have been migrated onto it, with the run dispatch and
//! the parameter→field flow that only the operation itself knows.
//!
//! # What it is NOT, yet
//!
//! It is not "every operation a recipe door can invoke", and reading it
//! that way would misjudge every later unit's cost. **Two verbs live
//! here**: the fillet and the chamfer. Every other door — extrude,
//! revolve, boolean, split, transform, pattern, loft, sweep, shell,
//! measure — still runs the way it always did, and is reached by
//! `editor-core`'s lowering calling its op crate directly. This crate
//! is the SEAT the rest migrate onto (SEAT-5 and after), not a
//! description of where they are.
//!
//! The design's cost claim is scoped the same way and is not
//! demonstrated here: what this unit shows is that two verbs share one
//! lowering, one correspondence and one tag function. Whether that
//! reduces the price of the NEXT verb is measured at the next verb.
//!
//! # What lives here, and what may never
//!
//! A [`Verb`] is the operation's parameters reified as plain data:
//! scalars at `T`, entity references as arena keys. Operand bodies are
//! NOT in the payload — they are borrowed at run time, and the
//! declaration states the arity instead (both blend verbs take one
//! body). Everything else a verb is committed to belongs to whoever
//! owns that commitment, not here: the content-key tag beside
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
//! The vocabulary spans crates: the blend pair is `sweep`'s, the
//! boolean and split verbs are `topo`'s, and more join at their own
//! migrations. Hosting the enum in either op crate would make one of
//! them name the other's ops, so it sits above both and below
//! `editor-core`, which is the only consumer.
//!
//! # What the birth record is for
//!
//! [`VerbOut`] carries the operation's per-entity birth record beside
//! the body. A verb without a birth channel cannot join this enum: the
//! record is what lets the document layer mint derivation-path names
//! for what the operation created, and an operation whose output cannot
//! be named is one no recipe can build on.

pub mod flow;
mod run;
mod verb;

pub use flow::{FieldRole, ParamFlow, RoleFamily, ScalarParam};
pub use run::{VerbError, VerbOut};
pub use verb::{Verb, VerbKind};
