---
id: revalue-pairs-the-ray-casters-decisions-only-to-their-first-difference
kind: issue
title: revalue_corpus pairs the ray caster's decisions only up to their first difference, so a laundering in a later ray goes uncounted
status: open
priority: P3
opened: 2026-10-19
---

Review R1 of PR 5119's second delta (item 5 of fix pass 3, `likely`,
read). `editor-core`'s `revalue_corpus` splits each build's decisions
into the document's and the side reader's ray caster's
(`compare_decisions`, `crates/editor-core/tests/revalue_corpus.rs`),
and pairs the caster's only up to their own first difference. A
laundering in a ray past that point is never re-valued.

**Measured: pairing by key does not answer it.** Pairing the `k`-th
decision of each predicate in one build with the `k`-th in the other,
across the whole stream, reported launderings that are none:
`die_chamfer` (`point_in_loop_advance`), `plate_param` apart to its
parting (`point_in_loop_advance`) and, counting the flips' wakes too,
`boss_union` (`bool_pair_gap`, `point_in_arc_loop_arm`). Where one build
aims a ray the other does not, every later decision of the same
predicate pairs with a different ray's.

**The fix is a ray's identity on the record**: the symbolic recorder
tags a decision with its family (`geom_core::sym::revalue::Decision::family`,
from `k_stats::in_family`); it would also need the aim it belongs to
(the aimed face and the ray's ordinal, scoped by `side_reader::cast_aims`
per ray), so the harness pairs a ray's decisions with the same ray's.
Then `a_side_reader_laundering_is_caught_and_its_flips_part_nothing`
extends with a laundering past a divergence.
