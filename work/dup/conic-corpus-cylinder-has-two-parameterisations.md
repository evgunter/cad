---
id: conic-corpus-cylinder-has-two-parameterisations
kind: issue
title: The two conic-pruning suites cut with two spellings of one three-arc cylinder
status: open
opened: 2026-09-20
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
