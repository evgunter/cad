//! 2-D profile authoring: loops, vertices, sketch planes, the PATHS
//! authoring lattice, and the validation tiers — **curated**.
//!
//! Every other kernel crate is re-exported whole from the façade root
//! (`pub use mesh;`, `pub use sweep;`, …): they carry geometry, and a
//! whole-crate re-export of geometry hands out nothing a caller should
//! not have. `profile` is the exception, and this module is why.
//!
//! # The boundary this module draws
//!
//! Evan's ruling on #413 (LIB-RETTAIL): **raw `ProfileLoop`
//! construction is kernel vocabulary and leaves the presented
//! surface.** The types stay nameable — read-back hands back a
//! `ProfileLoop`, `ProfileError` payloads carry `SegmentRef`s,
//! `ValidatedProfile` is what every body operation consumes — but the
//! minting doors go.
//!
//! That is only enforceable because the minting doors are trait
//! methods, not inherent ones. `profile::RawLoop` carries `new` and
//! `polygon`; inherent methods would have travelled with the type
//! through any re-export that made it nameable, and the type must be
//! nameable. This module re-exports everything in `profile`'s root
//! EXCEPT `RawLoop`, so `pncad::profile::ProfileLoop::polygon(…)` does
//! not resolve: the trait is not in scope and there is no path to it.
//! (`pub use profile;` did resolve it — that is the measurement this
//! narrowing answers, and it is the LB13 precedent: a curated module in
//! place of a whole-crate re-export, aimed at one nameability.)
//!
//! RESIDUE, stated rather than glossed: `ProfileLoop` and
//! `ProfileVertex` are plain data with public fields, so
//! `ProfileLoop { vertices, tangent_joints }` still type-checks
//! wherever the type is nameable. Sealing that means private fields
//! plus accessors, which is a change to the plain-data convention and
//! not a housekeeping edit. What this module removes is the *authoring
//! tier*: the named, documented, prelude-carried way to mint a loop
//! from a coordinate table without the lattice's junction
//! classification.
//!
//! Authoring goes through the lattice: [`Open`], [`Start`], the
//! binders, [`circle`], [`circle_split`].

// The submodules a caller reaches for by path. `test_support` is gone
// (LIB-RETTAIL) and `path`'s own root re-exports are already listed
// below, but the module hop is what the lattice's program vocabulary
// (`profile::path::program::Step`) is spelled through.
pub use ::profile::{k_stats, lift, path};

// The lattice: authoring states, targets, the closed-carrier verbs.
pub use ::profile::{
    ArcCarrierScalar, ArcCenterTarget, ArcTarget, ArcViaTarget, ClosedLoop, LineTarget, Open,
    PartialPath, PathError, ReplayError, ReplayErrorKind, Start, Step, TangentArcTarget, Target,
    TipState, Verb, circle, circle_split, replay,
};

// The data types (nameable, not mintable) and the bulge sugar that
// computes a single segment's parameter — sugar over ARITHMETIC, not a
// loop-minting door.
pub use ::profile::{
    ArcSweep, FilletLegShape, Profile, ProfileLoop, ProfileVertex, SketchPlane, bulge_from_center,
    bulge_from_via,
};

// Validation: the gate, its typed refusals, and the canonical output.
pub use ::profile::{
    ContactKind, EscalationSite, FilletLeg, FilletLegCarrier, LoopRole, NoCornerReason,
    ProfileError, SegmentKind, SegmentRef, ValidatedLoop, ValidatedProfile, ValidatedSegment,
};

// The lift door (recorded programs back to loops) and its verdicts.
pub use ::profile::{Fidelity, LiftOutcome, LiftRefusal, lift_checked};
// `lift` the FUNCTION shares its name with `lift` the module above; one
// `pub use` covers both namespaces.
