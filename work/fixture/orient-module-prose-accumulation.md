---
id: orient-module-prose-accumulation
kind: issue
title: `sweep`'s orient.rs has no governor on prose accumulation
status: open
opened: 2026-09-15
priority: P4
cost: E
---


## Finding

**Raised by the style review on #2624 (S391), reviewer's Q8 read.**

`crates/sweep/tests/common/orient.rs` is 824 lines. It carries **two
complete containment oracles** (the fixed-chord index around
`loft_contains`, and `LevelIndex`), a **self-declared copy** of the
tree's outermost ray-parity implementation (`level_set_contains`, whose
header argues reuse is blocked in four directions), **ten tuning
constants each with a multi-paragraph derivation**
(`FIXED_AXIS_GUARD_COS`, `CHORD_ERR_BOUND`, `MONOTONE_SCAN`, `LEVELS`,
`CONTINUITY_COS` and their neighbours), and a **~120-line `LevelIndex`
doc** carrying a measured 3x4 table of cosines over the helix fixtures.

**No single addition was unreasonable, and that is the finding.** The
module header states a routing rule — *an item lives at the narrowest
home all of its consumers can reach* — and it governs where ITEMS live.
Nothing governs where PROSE lives. So each unit that touched the module
added an argument at the site it was working on, and the argument for
"what the fixed-chord index needs, and why a turning body has none" is
now stated at six sites across three files: `FIXED_AXIS_GUARD_COS`'s
doc, `level_plane`'s doc, `first_wall_chord`'s doc, `LevelIndex`'s
doc, `m8_14_long_turn_sweep.rs`'s anti-restatement bullet, and
`turning_orientation.rs`'s module header. #2624 trimmed one of those
six back to what only it can say; the other five are as they were, and
nothing stops a seventh.

The cost is the one this module's own routing rule was written against,
one level up from code: a reader looking for the canonical statement of
a condition finds five near-copies and cannot tell which is load
bearing, and an editor who corrects one leaves four stale. #2624 is the
receipt — its sweep declared the file "read in full" and still left two
sentences describing the fixed chord as a section-to-section chord,
eight lines under a line the same diff edited.

**What a fix looks like** (not settled, and the row's first question):
either the routing rule is extended to prose — one canonical home per
argument, every other site a cross-reference — or the module is split
so that each oracle's argument has somewhere of its own to live. The
second is the larger change and touches `tests/all.rs`'s aggregation.

**Confidence**: sure (read). **Where**:
`crates/sweep/tests/common/orient.rs`, whole module; the six sites
named above.

**Verdict:**

## The same shape, one file over: `sweep/src/test_support.rs` (2026-09-15, SUITE/S392)

Added by the reviewer of `S392`'s PR #2650, which grew the file it
measures.

`crates/sweep/src/test_support.rs` is **1825 lines, about 42% of them
doc comment**, opening with a five-section module header near 100 lines
long: the feature gate's argument, the routing rule, the extrusion
family, the loft family, and "editing a fixture here re-authors
committed bytes". Every section was added by a unit that had a real
reason, and no single one was unreasonable — which is this row's
finding, one file over from the one it was filed about.

It is the same mechanism and the same cost: `orient.rs`'s routing rule
governs where ITEMS live and nothing governs where PROSE lives, and
`test_support.rs` is governed by that same rule and has the same gap.
Two receipts from #2650 specifically:

- The module header **acquired a paragraph arguing that no
  hand-written census is kept here**, and thirty lines below it a later
  section of the same header carried a hand-written numeric census
  ("thirteen constructions want the solid, six want the sections"), one
  of whose two numbers was already wrong when the reviewer read it.
  Both were written in the same diff. The header is now long enough
  that its own rule was invisible from inside it.
- The **`_at` suffix** carries two meanings across eleven doors with
  nothing in the header saying so; that half is filed on
  `sweep-test-support-two-wrapper-conventions`, which is the row that
  asks for a header rule.

What this adds to the fix above: whatever governor is written for
`orient.rs` should be written for a **module**, not for that file, and
`test_support.rs` is the second subject to apply it to. It is also the
harder one, because it is `src/` and its header is the one document
that tells a crate outside `sweep` how to join the fixture home — so
"split the module" costs more here than it does in `tests/common`.
