---
id: a-corpus-refusal-that-moves-needs-four-hand-edits
kind: issue
title: step-import: a corpus refusal that moves is re-pinned by hand in four places, and the render lane's copy goes red apart from the slow set
status: open
opened: 2026-11-23
priority: P3
---

## What

When a corpus file's first refusal moves, every place that spells it is
edited by hand. `nist_ftc_09`'s move from `#4950` to `#5130` (PR 7171's
fix pass; re-pinned on `germ/nist-snap-main-red`, PR 7538) touched:

- the gap, spelled in `crates/step-import/tests/tier_gate.rs`'s
  `NIST09_TANGENCY_IN_BAND` and derived again in `snap.rs`'s
  `nist09s_cylinder_5130_reads_tangent_in_the_files_band_and_refuses_at_the_snap`,
  with "56.8 µm" in four comments (`snap.rs`, `tier_gate.rs`, `wild.rs`,
  `demos/wild/src/main.rs`);
- the class fragment, in two `WILD_REFUSALS` tables:
  `crates/step-import/tests/wild.rs` (`WILD_REFUSALS`) and
  `demos/wild/src/main.rs` (`WILD_REFUSALS`).

The two tables are separate copies, and the demo's is read only by the
nightly's render lane. That is why the render lane went red on its own
(runs 44870651337 and 44927630239) as well as the three slow-set rows. One
home for a corpus file's pinned refusal, read by both the test and the
demo, would let a move be one edit and one red.

## Open question for the catalogue's owner (review NOTE 2 on PR 7538)

Edge `#5118` lies on both of its surfaces to 5e-15 m. The pair's two
crossing lines are 1.44 mm apart, about 42 ε_in, and they meet at about
9°: the file prints a clean crossing. The snap reads that pair as a
tangency in band and refuses it as the file's ambiguity. Is a tangent
reading in band the right reading for a pair the file prints as a clean
crossing, or should an edge that lies on both surfaces as a crossing
decide the pair?

Signed: (GERM orchestrator)
