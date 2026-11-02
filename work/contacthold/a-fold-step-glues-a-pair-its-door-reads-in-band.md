---
id: a-fold-step-glues-a-pair-its-door-reads-in-band
kind: issue
title: A fold step glues a pair its face-pair door reads in band, on its neighbours' records alone
status: open
opened: 2026-10-30
priority: P1
cost: M
design: true
refs: [a-glued-pair-is-re-read-through-the-declared-rung-which-bridges-in-band]
---

Since CONTACTHOLD (PR 6079) a glued pair is read through its record at
every consuming site. The n-ary fold reads A–K through the A–J and J–K
records alone, so it can glue a pair that the face-pair door, asked
directly, reads in band. Both reviewers of PR 6079 reproduced this.
The behaviour predates the PR: before it, the declared rung bridged
the same pairs.

## The scenes

- **A tilt across y** (`crates/topo/src/boolean/glued_pair_rows.rs`,
  `a_fold_step_glues_a_pair_its_door_reads_in_band_today`):
  - Three blocks in a row along x: A at `x ∈ [-10, 1]` and J at
    `[1, 2]`, both topped flat at `z = 1`, and K at `[2, 3]`, whose top
    falls `s = 0.4·ε` across y.
  - Every corner stands within `s` of every other top's plane, and the
    J–K door reads Zero over its small ball.
  - The accumulation's merged top against K's top, read by the
    face-pair door itself, is in band: the tilt is levered over the
    long ball.
  - `nary_boolean` builds.
- **A fall along x**, from the review branch
  `review/contacthold-6079-R1-fold-probe` @ 1217ceeb:
  - The same three unit blocks as
    `a_fold_step_given_two_zeros_whose_ends_stand_in_band_refuses_at_a_vertex`,
    with J and K falling `s = −0.48·ε` per unit.
  - Every far corner is inside the zero band of the flat top
    (`2|s| < ε`), while the ends' summed bound over the three blocks'
    ball is in band (`≈ 2.16|s| > ε`).
  - The pairwise door reads A–K in band, and the fold builds on J–K's
    Zero.

## The window

A tilt `t` reaches it when its lever over the neighbours' ball is
inside the zero band and its lever over the ends' ball is not:
`t·r_JK < ε ≤ t·r_AK` (with each `r` the half-extent the door levers
over). For a pair with no vertex off a face it is open for any tilt in
that interval. When a vertex stands in band of a face, the
vertex-on-face side read refuses first:
`a_fold_step_given_two_zeros_whose_ends_stand_in_band_refuses_at_a_vertex`
is the case at `0.7·ε`.

## The merge's union-find

`GluedSurfaceEq` in `crates/topo/src/merge_faces.rs` (`union`/`find`)
has the same shape. Two glued pairs `(a, b)` and `(b, c)` put `a` and
`c` in one class, and the merge then continues across `a`–`c` with no
read of its own.

## The question

`[verdicts.closeness]` (docs/DESIGN.md) says cells are one "only by
topology (a Zero verdict glued them, and from then on they are one
cell)". It calls that transitive because it is not a comparison of
values: J glued to A and K glued to J makes A, J and K one cell. D10
says a margin in band refuses.

Does the topology rule govern a pair that no op asked about, or does
the door owe the pair its own reading over the joined extent? The
first is what the code does today. Under the second, the fold step
re-reads its accumulation against the next member, and the merge
re-reads each class's ends.

This goes to the designers and Ev. The row
`a_fold_step_glues_a_pair_its_door_reads_in_band_today` pins today's
Built until it is decided.
