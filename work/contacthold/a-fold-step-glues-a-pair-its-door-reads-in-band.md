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

## More evidence: chains past K·ε (REFS orchestrator, 2026-11-02)

This came from a research lane answering Ev's question on PR 6342 ("a long chain of things that are all <eps off but they stack up? we don't want to be able to snap that by more than eps").

The probe is on branch `reads/chain-drift-probe`, files `probes/chain-drift/chain_drift_probe.rs` and `chain_probe.log`, run at ε = 1e-9, K = 10, f64. It was run against the topo test-support doors and was not committed to the suite.

| Scene | Total drift | Result |
|---|---|---|
| n-ary union of slab chains, each adjacent pair Zero | 0.9ε | builds, valid |
| same | 1.8ε to 19.8ε | refused at member 3, `VertexOnFace` in band |
| `merge_coplanar_faces` on one faceted top | 0.9ε | merges |
| same | 1.8ε and 4.5ε | refused in band |
| same | 19.8ε and 82.8ε | refused **as a kernel defect** ("there is no way through: this is a kernel defect; report it") |
| **Lever**: flat block A; block B tilted 0.4ε per unit; long block C continuing B's plane, its far corner 40ε off A's plane; n-ary union | — | refused as `EdgeUndescribed` / `ResidualExceeded`, also kernel-bug wording |
| same lever, binary `(A∪B)∪C` | — | builds a valid body, keeping C's top as its own face |

Nothing moved a point by more than ε silently. But:

- a legal input that drifts past K·ε gets the kernel-bug wording instead of a typed refusal that blames the input;
- the n-ary fold refuses the lever while the binary fold builds it.

No test pins a chain that drifts past K·ε. This belongs to this row's question: does the fold or merge owe a read of a glued class's ends?
