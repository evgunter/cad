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
//! exactly; the format may not shift under a refactor. `contact_class`
//! writes `"rest"`/`"tangent"`, lowercase because the class vocabulary
//! was minted straight into a `with` module at the v11 break and never
//! had a derive to match.
//!
//! A module for an already-persisted type takes whatever its type's
//! bytes are today, and pins them by test before changing anything. Only a
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
//! the vocabulary by `tag` rather than repeating the pairs — so the
//! two directions cannot disagree, and neither can a refusal message
//! that quotes the same table. Every module here does this.
//!
//! **Take the vocabulary from where the type lives, when it is there
//! to take.** `contact_class` searches `topo::ContactClass::ALL` — the
//! kernel's own slice, sitting inside the impl block whose exhaustive
//! matches force a new variant through it. A `[Rest, Tangent]` literal
//! restated here would be the shape that slice exists to retire: the
//! enum is `#[non_exhaustive]`, so this crate cannot enumerate it
//! correctly even in principle, and the kernel's doc records the
//! measurement that such a literal stays GREEN under a planted third
//! variant. A local list is kept only for an enum of this crate's own
//! that offers none — and there the completeness of the list genuinely
//! is unchecked, since safe Rust cannot tie an array literal to a
//! variant list without a proc macro and the workspace has none.
//!
//! **Check the round trip on the way out, either way.** Before
//! writing, a module verifies its own `tag` reads back and refuses if
//! it does not. Where the domain comes from the kernel that is
//! belt-and-braces; where it is a local list it is the only guard the
//! gap has. It costs a lookup, so it is not worth deciding per module:
//! every one does it.

pub(crate) mod contact_class;
pub(crate) mod extrude_side;
