---
id: the-chart-partition-has-a-topo-src-home-and-a-test-home
kind: issue
title: a body's faces grouped by surface key are spelled in topo's shell.rs, in topo's offset_together tests, and in sweep's tests/common
status: open
opened: 2026-09-26
needs_ev: true
priority: P4
cost: D
---



## Finding

- **Where**: `crates/topo/src/shell.rs` (`chart_groups` over
  `group_by_chart`, private), `crates/topo/src/offset_together.rs`'s
  in-crate `scope_walks::moves_of`, and
  `crates/sweep/tests/common/charts.rs` (`charts`, `charts_of`).
- **Confidence**: sure; all three read at the merge base of the batch-6
  lane that homed the third.
- **Raised by**: the batch-6 lane that closed
  `a-solids-charts-as-a-move-set-is-spelled-per-sweep-test-file`, which
  answered that row's first question — *is either of `shell.rs`'s two
  move-set builders a door the suites could call?* — with **no**: both
  build over the private `chart_groups` / `group_by_chart`, and no
  public door answers "this body's faces, grouped by the surface they
  wear".

The same file holds the ONE-chart twin too: `shell.rs::faces_wearing`
(every face wearing one surface key, in arena order) is the production
spelling of `shell10_r2_probes::chart_of`, which the fold kept as not
a member of the partition. A door decision here covers both.

So the partition every caller of `offset_charts_together` and
`offset_planes_together` has to build before it can call either door is
spelled three times: once in production (`shell.rs`), once in `topo`'s
own tests (`moves_of`, which cannot reach `sweep`'s test tree), and
once in `sweep/tests/common` (which now serves every `sweep` suite).
Each spells the same order — groups by first appearance in the face
arena, each group in arena order.

## What a taker owes

A decision first: whether "a body's charts" is a public `topo` door —
the partition the offset doors' `ChartMove` contract is stated over, so
a user driving those doors needs it as much as the suites do — or stays
the verb's private step. If it is a door, `shell.rs`,
`offset_together`'s tests and `sweep/tests/common/charts.rs` all fold
onto it (the last keeping only the `ChartMove` builders). If it is not,
this row closes with that ruling and the two test spellings stay.

**Why P4 and cost D**: the two test-side spellings are P4 on their own
and the production one has no second production twin; the door
question is a public-API decision on `topo`, which is what makes it D.

**Why this row is on dup's slate and not `shell`'s** (method item 14):
`work.py territory` puts `topo/src/shell.rs` and
`topo/src/offset_together.rs` under the `shell` program and
`sweep/tests/common/charts.rs` under `tcost` and `tint`. The finding is
the three spellings taken together — one per owner — and it is a
duplication question before it is any one owner's; filed on either
owner's slate it would name ground the other two own. It announces to
`shell` by seam, and moves there by `git mv` if `shell` claims the door
question.

## The question for Ev, and the recommendation

Ev (2026-09-28): *"that seems plausible but i do wonder if this is a symptom / if there's a better way to structure the api so the internals like this are better abstracted away"*.

Two designers weighed it independently, then read each other's reports. Both now recommend the same final state.

- **It is a symptom.** The offset doors (`offset_planes_together`, `offset_charts_together`) take `ChartMove { faces, distance }`. The caller writes the face list, which is a fact of the body, and the door re-derives it to check it. That check needs four typed refusals (`EmptyGroup`, `TogetherChartMixed`, `TogetherFaceRepeated`, `TogetherPartialSet`), copied into both doors. A fifth malformed case gets past them: one chart split across two moves, which refuses late with an error blaming the wrong thing.
- **The doors take the solids plus a rule giving one distance per chart.** The door builds the grouping (crate-private, one home in the offset module) and calls the rule once per chart through a read-only chart view: key, faces, surface, and sense (`None` when the faces disagree). One distance per chart holds by construction, with no scalar comparison, so the four refusals, `ChartMove`, `scope_of_moves` and the split-chart gap all go. `replace_faces_offset` names its chart by surface key.
- **A data-shaped "into the material by t" door sits on top** (e.g. `offset_solids_inward(body, solids, t)`). It owns the door ladder (planar / axial / chart by chart), the sense turn, and the mixed-sense refusal. It is the `offset_inward` that OFFSET-DESIGN O4 already names.
- **Callers get simpler.** `shell`'s cavity is one call per solid. `shell_open`'s lift becomes a one-line rule (`|c| if c.key() == counterpart { back } else { zero }`), where today it is about 20 lines of caller-side grouping. Sweep's hollowing rows become one call. Tests stop needing the grouping, so both test copies go.
- **What goes with it.** The tests that build deliberately malformed move sets are deleted, because those states can no longer be written. Every test of the door's own behaviour survives, rewritten as a rule.
- **Ratified decisions.** None changes. D8 binds recipes, not kernel doors, so a kernel door may take a closure; `Verb::Shell` keeps its thickness as data. The comparison-free `Real` (D9, Q1) is why a per-face distance is rejected. O4's sentence naming the door ladder is re-worded with the change.
- **Still open, for the follow-up unit.**
  - A closure rule or a door-built table: the designers prefer the closure, with a table as a fallback.
  - Whether the door ladder sits behind the rule door or only behind the inward door.
  - Whether the all-planar lift can leave the per-chart door, which it takes on purpose today.

If Ev prefers the smaller change, the sound alternative is to keep `ChartMove` and add a structural check that refuses two moves naming one surface key, with the inward door built over it. Both designers agree that this check is owed whenever `ChartMove` survives in any form.

The follow-up is a unit on `shell`'s ground, which owns the offset doors.
