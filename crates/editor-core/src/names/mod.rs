//! Persistent naming (M4 PR 3; NAMING-DESIGN N1–N4 made concrete —
//! ratified #74, binding; spec `docs/M4-PR3-SPEC.md` D1–D6).
//!
//! A [`StableName`] is a **derivation path**: the minting recipe node
//! plus an op-typed [`RolePath`] of closed-enum [`RoleSeg`]s (N1).
//! Names contain no floats, no arena keys, no bare enumeration
//! indices — geometry enters only as margined predicate VERDICTS
//! recorded in [`Qualifier`]s (N2), and the only integer payloads are
//! recipe-structural data (pattern `Instance(i)`) or the profile
//! crate's own canonical combinatorial identities (locators — see
//! [`role`](self)).
//!
//! The per-node [`NameTable`] (N4) is emitted EAGERLY by the wire
//! layer during evaluation — a mechanical, linear pass over each op's
//! output driven by kernel birth data (`Extruded`/`Revolved` maps,
//! `SplitNaming`, `BooleanNaming`, D5 provenance); nothing is ever
//! reconstructed by matching. Resolution (PR 4) is a table lookup.
//!
//! Layering (D1, G1): the kernel never sees a `StableName` — ops emit
//! birth facts; THIS module (editor-core) names things.

mod discriminate;
mod emit;
mod emit_sweep;
mod emit_topo;
mod role;
mod table;

pub use emit::NamingError;
pub(crate) use emit::{empty, name_pattern};
pub(crate) use emit_sweep::{name_extrude, name_loft, name_revolve};
pub(crate) use emit_topo::{OperandCtx, name_boolean, name_split};
pub use role::{
    CapEnd, EntityKind, MeridianEnd, ProfileEdgeRef, ProfileVertexRef, Qualifier, RolePath,
    RoleSeg, SideVerdict, SplitHalf, StableName,
};
pub use table::{EntityKey, EntityRef, Entry, NameTable};
