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
//! **Raw `ProfileLoop` construction is kernel vocabulary and stays
//! off the presented surface.** The types stay nameable — read-back hands back a
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
//!
//! What this module removes is the *authoring tier*: the named,
//! documented, prelude-carried way to mint a loop from a coordinate
//! table without the lattice's junction classification. The COMPILER
//! is what makes the removal total — `ProfileLoop`'s fields are
//! private, so a struct literal is an E0451 privacy error in every
//! crate but `profile` itself (pinned by a `compile_fail` doctest on
//! the type). Naming the type, reading it through its accessors, and
//! matching on error payloads that carry it are all untouched; the two
//! surfaces are complementary, and the funnel is the pair.
//!
//! Stated honestly, because it is a crate boundary and not a module
//! one: `profile`'s own internals build loops directly and hold the
//! invariant by their sealed-verbs discipline, not by privacy. And a
//! consumer willing to depend on `profile` directly still reaches
//! `RawLoop` — the door is off the PRESENTED surface, not out of
//! existence. `demos/tour` does exactly that, in one scene, on purpose
//! (its manifest says why).
//!
//! Authoring goes through the lattice: [`Open`], [`Start`], the
//! binders, [`circle`], [`circle_split`].

// The submodules a caller reaches for by path. `path`'s own root
// re-exports are already listed
// below, but the module hop is what the lattice's program vocabulary
// (`profile::path::program::Step`) is spelled through.
pub use ::profile::{lift, path};

// The lattice: authoring states, targets, the closed-carrier verbs.
pub use ::profile::{
    ArcCarrierScalar, ArcData, ArcLen, ArcSide, Bulge, Center, ClosedLoop, LineTarget, Open,
    PartialPath, PathError, PointLeg, Radius, ReplayError, ReplayErrorKind, Start, Step, Sweep,
    TangentArcTarget, Target, TipState, Verb, Via, circle, circle_split, replay,
};
// The §2c family's traits and arrival builders: the admissibility
// matrix (one impl per admissible (state, mode) pair) and the states a
// spec that leaves binders free completes through.
pub use ::profile::path::{
    ArrivalSpec, LegEndIncoming, PointIncoming, RadiusArrival, RadiusArrivalAt, RadiusArrivalDir,
    TangentIncoming, ViaArrival, ViaArrivalStart,
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
