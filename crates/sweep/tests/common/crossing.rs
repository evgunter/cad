//! Which recorded predicates are the Boolean's **crossing pipeline**: the
//! one question a "this verb runs no intersection machinery" row asks of
//! a `k_stats` bracket, answered in one place.
//!
//! A K name's prefix is the module that decides its margin
//! (`[topo.homes]`, `crates/topo/README.md`), so the pipeline's names are
//! the Boolean's own (`bool_`), those of the curve × face doors it reads
//! (`geom_brep::curve_face::decided_by_a_door`, whose roster that crate's
//! own row holds against its sources, so a door renamed or moved there
//! needs no edit here), and those of the shared readings its crossing
//! arms call that are neither, which [`SHARED_READINGS`] lists.
//!
//! Did NOT absorb: `verbs_shell`'s `VALIDATOR_SHARED`, the `bool_` names
//! the validator's containment walk shares with the pipeline, which one
//! suite subtracts and so keeps.

/// The prefixes of the shared readings the crossing pipeline decides
/// through that are not a curve × face door's, each a crate-root module
/// of `topo`:
/// - `chart_window_` — a face's azimuth window on its chart, which the
///   Boolean's containment trim and the chord join read
///   (`topo::chart_window`).
pub const SHARED_READINGS: &[&str] = &["chart_window_"];

/// Whether `predicate` is one the crossing pipeline decides.
pub fn decided_by_crossing(predicate: &str) -> bool {
    predicate.starts_with("bool_")
        || geom_brep::curve_face::decided_by_a_door(predicate)
        || SHARED_READINGS.iter().any(|p| predicate.starts_with(p))
}
