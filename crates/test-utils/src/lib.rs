//! Shared test scaffolding for the whole tree — the pieces several
//! suites would otherwise each hand-roll a copy of.
//!
//! Today it holds:
//!
//! - [`fuzz`], the harness every randomized falsification sweep draws
//!   its RNG, its per-run seed and its EFFORT dial from.
//! - [`source`], the SHARED Rust lexer for guards that pin a claim
//!   about the code against the code — three views of a file (code
//!   only, code with literals, prose alone) plus the traversals and
//!   balanced-text operations that read them. The readers still
//!   outside it are enumerated in `tests/reader_census.rs`.
//! - [`vacuity`], the **anti-vacuity floor** — a statement of how much a
//!   sampling guard actually exercised, printed every run and asserted,
//!   so a run that exercised nothing goes red instead of green.
//! - [`tightness`], its companion for a certified bound: the CEILING a
//!   `bound >= truth` row cannot state, measured per site, plus the
//!   check that the ceiling sits below the scale at which the
//!   enclosure has degenerated to the whole object.
//!
//! # DEV-ONLY, by convention
//!
//! Nothing depends on this crate outside `[dev-dependencies]`, and
//! nothing should: a crate that no production manifest names cannot be
//! reached from a production build path.
//!
//! Being a LEAF with ZERO dependencies is the other half of the point.
//! `interval-transcendentals/` is its own workspace root and is
//! path-depended on by `geom-core` (the `interval` feature's backend),
//! so its tests could never have used a harness living in `geom-core`
//! without inverting the layering. Below everything, there is no cycle
//! to create.

#[cfg(test)]
mod panic_capture;

pub mod fuzz;
pub mod source;
pub mod tightness;
pub mod vacuity;

/// Declares the source paths a randomized or otherwise expensive suite is
/// SPECIFIC TO, so a pull-request gate can skip it when none of them moved.
///
/// The invocation is read out of the SOURCE TEXT by
/// `scripts/ci-filter.py`, which runs in a job with no toolchain and
/// therefore cannot ask the compiler anything. What the macro itself buys is
/// the half a text scan cannot: a misspelt marker is a compile error rather
/// than a silently ungated suite. It expands to nothing.
///
/// # Where it goes, and what it gates
///
/// One invocation per file, at the top of it.
///
/// * `crates/<c>/tests/<suite>.rs` — gates every test in that suite, which
///   is one `#[path]` module of the crate's aggregated `tests/all.rs`.
/// * `crates/<c>/src/<path>.rs` — gates every test in that file's module,
///   whole-file granularity. A file that also carries production code puts
///   the invocation INSIDE that file's `#[cfg(test)]` module, because this
///   crate is a dev-dependency and does not exist in a production build —
///   and because a bare top-level `#[cfg(test)]` line is what this tree's
///   source censuses read as the production/test cut, so a second one makes
///   that cut ambiguous (`mesh`'s `the_eps_inventory_is_pinned` asserts
///   there is at most one). A file whose whole module is already
///   `#[cfg(test)] mod …` in its parent needs neither.
///
/// # What the paths mean
///
/// Repo-relative, files or directories; a trailing `/` means "anything
/// under". THE FILE CARRYING THE MARKER IS ALWAYS AN IMPLICIT MEMBER, so it
/// is never listed. Every listed path must exist —
/// `scripts/gates/gated-suite-paths.sh` fails the `discipline` row when one
/// does not, which is what stops a rename from turning a gate into "never
/// runs on a pull request".
///
/// ERR TOWARD NAMING MORE. The cost of a path set that is too wide is a run
/// the change did not need; the cost of one that is too narrow is a break
/// that waits for the nightly, which runs the whole gated set ungated. An
/// upstream file whose change would plausibly break the suite belongs in the
/// set even when the suite does not name it.
#[macro_export]
macro_rules! gated_to {
    ($($path:literal),+ $(,)?) => {};
}
