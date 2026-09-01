//! The bytes of KERNEL types, described from above the boundary.
//!
//! # Why this file exists
//!
//! Two ratified rules meet on every kernel type a document persists,
//! and both are kept:
//!
//! - the kernel crates gain NO serde dependency (the F3/G1 layering
//!   rule, workspace `Cargo.toml`, checked by
//!   `scripts/gates/kernel-serde-free.sh`) — persistence is this
//!   crate's job;
//! - a shared vocabulary is ONE enum, defined lowest and re-exported
//!   upward, never a parallel enum (CONTACT-DESIGN C4's layering
//!   ruling) — so a mirror enum carrying the
//!   derives is not available either.
//!
//! A `#[serde(with)]` module satisfies both: the TYPE stays the
//! kernel's everywhere, and only the BYTES are described here. This
//! module is where every such description lives, so the technique has
//! one home and one doc instead of one per type.
//!
//! # The rules a new module here must follow
//!
//! **Spell, do not number.** A wire spelling is a stable STRING, never
//! a discriminant: a discriminant silently re-maps when a variant
//! lands between two existing ones, and a file then reads a different
//! value than it was written with.
//!
//! **The casing is whatever the type's bytes already are — it is not a
//! style choice.** These modules are written for types that are
//! already persisted, so their job is to reproduce existing bytes
//! exactly; the format may not shift under a refactor. That is why the
//! two spellings here disagree and must keep disagreeing:
//!
//! - `boolean_op` writes `"Union"`/`"Intersect"`/`"Subtract"` —
//!   capitalised because it replaced a `#[derive(Serialize)]`, whose
//!   spelling of a unit variant is the variant's own name. Anything
//!   else would have been a format break.
//!   
//! - `contact_class` writes `"rest"`/`"tangent"` — lowercase because
//!   the class vocabulary was minted straight into a `with` module at
//!   the v11 break and never had a derive to match.
//!
//! A THIRD module inherits neither: it takes whatever its type's bytes
//! are today, and pins them by test before changing anything. Only a
//! vocabulary with no bytes yet is free to choose, and then lowercase
//! is the house style.
//!
//! **Refuse, never guess.** Both directions refuse typed on a value
//! this build has no spelling for — including the WRITE direction. A
//! file written under a guessed tag is worse than a refusal: it is
//! read back as something else.
//!
//! **Derive the inverse; never restate it.** A spelling is written
//! down once, in the module's `tag`, and the read direction searches
//! an `ALL` table by `tag` rather than repeating the pairs — so the
//! two directions cannot disagree, and neither can a refusal message
//! that quotes the same table. What no `ALL` can be checked for is
//! completeness (safe Rust cannot tie an array literal to a variant
//! list without a proc macro, and the workspace has none), so the
//! write direction pays for it at run time: it verifies the round
//! trip before writing and refuses rather than creating a file this
//! build could not open. Both modules here do exactly this, and a
//! third owes it too.

pub(crate) mod boolean_op;
pub(crate) mod contact_class;
