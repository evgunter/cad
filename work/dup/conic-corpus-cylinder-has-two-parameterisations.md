---
id: conic-corpus-cylinder-has-two-parameterisations
kind: issue
title: The two conic-pruning suites cut with two spellings of one three-arc cylinder
status: closed
opened: 2026-09-20
priority: P4
cost: E
closed: 2026-09-26
pr: 3284
---


## Finding

- **Where**: `crates/sweep/tests/n3r1_prune.rs`'s `cylinder_at` /
  `cylinder`, and `crates/sweep/tests/s16_box_soundness.rs`'s
  `cylinder`.
- **Importance**: low-medium
- **Confidence**: sure; the two bodies were compared source-to-source
  at merge base `b29fe8bd1`
- **Raised by**: the `dup/one-line-fixture-wrappers` lane, 2026-09-20,
  as the one member of the conic-pruning corpus its fold declined

The two suites share a corpus vocabulary and, since
`dup/one-line-fixture-wrappers`, share its boxes and its rounded plate
from `crates/sweep/tests/common/operands.rs`. The three-arc cylinder
they both cut with did NOT go there, because they are not one
spelling:

| | `n3r1_prune::cylinder_at(cx)` | `s16_box_soundness::cylinder(z0, height)` |
| --- | --- | --- |
| placement | the PROFILE is translated in `x` (`p2(cx + 0.5·cosθ, 0.5·sinθ)`) | the profile is centred and the SKETCH PLANE is lifted to `z0` |
| extrusion | `Distance(1.0)`, fixed | `Distance(height)` |
| parameters | `cx` | `z0`, `height` |

Both build a radius-0.5 three-arc cylinder with six vertices at
0°/120°/240°; neither can be expressed as the other without deciding
something. **What the reconciliation decides is a verdict**: these
rows are about which edge×face pairs the conic prune EXAMINES, so
whether a rim's sketch pose is part of what they check is the
question, not a detail of it. A merged door would pick one pose for
both suites and could move a pair count silently — `n3r1_prune`'s row
asserts a hard 98.

Whoever takes it owes the same measurement the fold owed: plant the
other suite's pose and read what moves.

## Why it sits here and not on the territory owner's slate

`scripts/work.py territory` puts both files on S-TCOST's and S-TINT's
ground. The finding is a **duplication** finding — one fixture in two
parameterisations — which is S-DUP's charter and neither of theirs,
and S-DUP claims no territory by design (`plan.md`, *"this program
claims nothing and announces by seam"*). One row rather than two,
because the two suites are one object here; a program that wants it
claims it by `git mv` per `work/README.md`.

## Closed (2026-09-26, PR #3284)

**Checked first: are the two parameterisations deliberate?** Both
files said so ("which pose a rim carries is part of what these rows
check", written by a fix pass, `5a05a2992`, not ratified), and the row
asked for the measurement. Taken with a plant: `n3r1_prune`'s
`cylinder_at(cx)` rebuilt by sliding the SKETCH PLANE in `x` instead of
the profile. The bodies differ — `Debug` differs at `cx` = 1.001 and
0.3, is equal at 0 — and `n3r1_prune`'s one row, the hard 98, stays
green (1 / 1 passed). So that row does not read the pose, and the
sentence is gone from both files. The other direction has no plant:
`s16_box_soundness`'s blind-bore tool is lifted in `z`, which no
profile slide in the sketch plane can express.

**One spelling, two knobs.** `common::operands::three_arc_cylinder(cx,
z0, height, first)` slides the profile by `cx` and lifts the plane by
`z0`, so each suite builds exactly the body it built before and no
verdict has a new input: measured `Debug`-equal to both old builders
at every argument either suite uses (`n3r1`: `cx` ∈ {0, 1.001, 0.3};
`s16`: `(0, 1, 0°)`, `(0.5, 1, 0°)`, `(0, 1, 240°)`), with a 1e-12
offset as the negative control. `n3r1_prune` keeps `cylinder_at(cx)`
and `s16_box_soundness` keeps `cylinder(z0, height)` as one-line names
for their own pose; `cylinder_from` is gone (its two calls name the
door). `operands`' not-absorbed list and both files' ``NOT `common::``
markers lose their cylinder entries together, as the module's rule
requires.

**Plants** (filter: the two suites, 8 rows):

| plant | reds | per site |
| --- | --- | --- |
| the door panics at its caller | 7 / 8 | `n3r1` `cylinder_at` 1 (the 98 row); `s16` `cylinder` 6 |
| the same at `s16` ~:425, then ~:430 (the 240° starts, masked above) | 1 / 8, then 1 / 8 | `conic_pruning_never_loses_an_accepted_pair` both times |
| radius 0.5 → 0.502 | 3 / 8 | — |

The four rows the radius plant leaves green are reached (row 1) and
assert refusal or clearance classes a 0.4% radius change does not
cross; nothing here claims them unasserted.

The same construction is written about twenty more times outside this
corpus; filed as
`work/dup/the-three-arc-cylinder-is-spelled-per-suite-beyond-the-conic-corpus.md`.
