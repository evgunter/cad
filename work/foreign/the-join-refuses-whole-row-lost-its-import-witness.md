---
id: the-join-refuses-whole-row-lost-its-import-witness
kind: issue
title: the join door's refuse-whole row lost its witness when import made the half-cap's pole a vertex
status: closed
opened: 2026-10-29
priority: P3
cost: E
closed: 2026-10-29
pr: 6007
---


Found by `import-normalizes-the-rim-only-cap`'s lane (FOREIGN implementer, 2026-10-29).

`crates/step-import/tests/halfcap_pole.rs` carried
`a_join_refused_in_the_band_leaves_the_body_as_found`: on
`halfcap_nosplit`'s imported body, split the meridian arc through the
pole once at an ordinary point (joinable, first in arena order) and once
a mid-band distance past the pole, then `Body::join_edges` refuses
`JoinUndecided` before any kill and leaves the body as found. Import now
makes the pole a vertex (`NormalizationKind::ChartSingularityVertex`),
so no imported arc crosses the pole, and `split_edge` refuses a split in
the band of an edge's end (`SplitParamEscalated`), which is where the
in-band vertex would now have to go. Re-describing the two halves into
one arc by hand (`kev_describing`) gives an arc whose description is no
chart iso, so its near-pole vertex is no join candidate and no reading
is taken. The row was deleted; the property (the join door refuses
whole) has no witness in the tree now.

Candidate witness: `crates/topo/tests/cleave_mint_doors.rs`'s M7-8 body,
whose split vertex's join reading is undecided at every ε, with an
ordinary joinable vertex split in ahead of it. That is topo's join door
read on a topo body, which is where the row belongs anyway.

## Closed (PR 6007 fix pass, 2026-10-29)

The review found the witness: `crates/topo/tests/cleave_mint_doors.rs`'s M7-8 body, an ordinary line edge split ahead of an M7-8 edge whose join reading is undecided at every ε. It carries the deleted row's name and meaning there, `a_join_refused_in_the_band_leaves_the_body_as_found`, in the nextest slow set (about 13 s in debug).
