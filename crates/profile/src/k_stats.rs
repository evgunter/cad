//! Margin-statistics collection — since M2 PR 7 a **re-export** of the
//! unified recorder in [`geom_core::k_stats`].
//!
//! The recording machinery (the [`MarginSample`] sink, the [`Probe`]
//! scalar, and the [`decide`] funnel) started life in this crate (M2
//! PR 2's K-experiment hook) and moved to `geom-core` next to
//! `Band`/`Decide`/`AMBIGUITY_K` when PR 7 wired every crate's
//! predicates to one recorder. This module remains as the
//! compatibility surface for existing consumers (the promoted PR 2
//! review suites among them); new code should reach for
//! `geom_core::k_stats` directly.

pub use geom_core::k_stats::{
    MarginSample, Probe, SampleOutcome, decide, start_recording, take_samples,
};
