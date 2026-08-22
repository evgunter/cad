//! Shared vocabulary for the sweep test corpus: section authoring
//! below, and the orientation checking in [`orient`].
//!
//! **Routing rule**, so a fourth home does not appear without one.
//! `sweep` has four places a suite can share from, and an item lives at
//! the narrowest one all of its consumers can reach:
//!
//! - `sweep::test_support` — fixtures the LIBRARY can build, reachable
//!   from in-crate tests as well as from here;
//! - this module — section authoring, the profile vocabulary a suite
//!   builds a body FROM;
//! - [`orient`] — what a suite CHECKS of a body it built;
//! - `revolve_common` — the revolve suites' own, and the place `p2`
//!   and `eps` presently live despite belonging to no verb.
//!
//! A helper one suite uses stays in that suite.
//!
//! Section authoring (LIB-U3): loft/sweep sections in the profile
//! vocabulary, one copy per crate. Cross-crate constant deduplication
//! (the 1/16-offset table relation) is LIB-U6's territory,
//! deliberately not built here.

#![allow(dead_code)] // loaded once per consumer; each uses a subset
#![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza

/// Orientation checking: the face-facing probe and the two level-set
/// indexes it decides against. Split out because it is a different kind
/// of shared thing from the section authoring below — not a fixture,
/// but the check several suites make of a body they built.
pub mod orient;

use geom_core::Point2;
use profile::RawLoop;
use sweep::{ProfileLoop, ProfileVertex, Section};

/// A closed four-line quad section (one loop, four vertices) — the
/// plainest INTEGRAL profile: unit weights, no arc anywhere.
pub fn quad(pts: [(f64, f64); 4]) -> Section {
    vec![ProfileLoop::polygon(
        pts.iter().map(|&(x, y)| Point2::new(x, y)),
    )]
}

/// The M5 PR 10 review section: a square-with-an-arc loop scaled by
/// `s` — three lines and one bulge-0.25 arc, so the skin exercises
/// the rational lane.
pub fn chain(s: f64) -> Section {
    let v = |x: f64, y: f64, bulge: f64| ProfileVertex::new(Point2::new(x * s, y * s), bulge);
    vec![ProfileLoop::new(vec![
        v(0.0, 0.0, 0.0),
        v(2.0, 0.0, 0.25),
        v(2.0, 1.0, 0.0),
        v(0.0, 1.0, 0.0),
    ])]
}
