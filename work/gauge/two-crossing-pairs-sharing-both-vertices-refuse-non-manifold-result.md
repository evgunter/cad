---
id: two-crossing-pairs-sharing-both-vertices-refuse-non-manifold-result
kind: issue
title: Two crossing pairs that share both vertices refuse NonManifoldResult, reasoning from one result vertex the cone split may have overtaken
status: open
opened: 2026-11-11
priority: P3
cost: M
refs: [shared-vertex-crossings-that-tie-or-interleave-are-unprobed]
---

Found by designer C on fork `shared-vertex-pairing` (2026-11-11), reading the code; not measured.

`crates/topo/src/boolean/insert.rs` refuses `NonManifoldResult` when two crossing vertex pairs share both their vertices. Its doc reasons from a single result vertex. Since `zip::split_cones` (a pinch is one vertex per cone, Ev on PR 4057), the result's vertices are read per cone downstream, so that premise may no longer hold. Under the fork's ruling (pair along the lead vertex's In arcs), this is the one place where pairs' runs could nest across pairs. `four_pairs_battery` does not reach it.

**Done when** a witness reaches it, and the refusal is either shown honest or retired with the shape built and pinned.
