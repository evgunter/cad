---
id: an-exact-plug-in-a-subtracted-hole-is-undecided-flush
kind: issue
title: An exact plug in a subtracted ring's hole is CensusUndecidable for its top and bottom: Door 2 escalates and nothing reads the faces
status: open
opened: 2026-10-23
---


`crates/topo/src/census.rs` `flush_at_rest` decides two same-oriented
coplanar faces of different groups through Door 2
(`chart_region.rs` `declared_pair_overlap`). Where the region walk
refuses `TouchingBoundary`, the flush arm reads the faces itself
(`planar_interiors_meet`) and writes the flush row the two faces earn.
Where Door 2 answers `Escalated`, the arm stops at `CensusUndecidable`.

The witness: `topo::subtract` a hole `[1, 2]²` from a plate `[0, 3]²`
(z in `[0, 1]`), then graft a plug that fills the hole exactly. The
plug's top and bottom lie flush with the ring's, their boundaries
meeting along the hole's rim and their interiors apart. Door 2 escalates
on the boolean-made ring, and both pairs stand as `CensusUndecidable`
where they are flush rows. The U-shaped twin with its notch filled,
built by prisms, writes its rows. Pinned in
`crates/topo/tests/review_census_fixpass_probes.rs`
`fixpass_exact_fills_are_flush`, which accepts a row or a finding for
the ring.

Owed: the `Escalated` arm consults `planar_interiors_meet` as the
`TouchingBoundary` arm does, provided that reading certifies apart
whatever the walk escalated on. Or a row states why an escalated walk
must stay undecided. Then tighten the pin to two rows.

Filed from PR 5605's fix-pass review.
