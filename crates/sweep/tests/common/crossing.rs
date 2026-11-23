//! Which recorded predicates are the Boolean's **crossing pipeline**: the
//! one question a "this verb runs no intersection machinery" row asks of
//! a `k_stats` bracket, answered in one place.
//!
//! A K name's prefix is the module that decides its margin
//! (`[topo.homes]`, `crates/topo/README.md`), so the pipeline's names are
//! the Boolean's own (`bool_`) and those of the curve × face doors it
//! reads (`geom_brep::curve_face::decided_by_curve_face`, whose roster
//! that crate's own row holds against its sources, so a door renamed or
//! moved there needs no edit here). A face's azimuth window, which the
//! Boolean's containment trim reads, is its stored lift's and decides
//! nothing; the chord join's island winding decides its run's lift
//! through the pcurve mint's own names (`pcurve_`), which every verb
//! that mints records and so mark no crossing.
//!
//! Did NOT absorb: `verbs_shell`'s `VALIDATOR_SHARED`, the `bool_` names
//! the validator's containment walk shares with the pipeline, which one
//! suite subtracts and so keeps.

/// Whether `predicate` is one the crossing pipeline decides.
pub fn decided_by_crossing(predicate: &str) -> bool {
    predicate.starts_with("bool_") || geom_brep::curve_face::decided_by_curve_face(predicate)
}
