---
id: validate-passes-a-body-with-a-zero-width-slit-face
kind: issue
title: split runs no validation tier on its own outputs, so a spurred half left the op unchallenged
status: review
opened: 2026-09-24
priority: P3
cost: D
pr: 3797
branch: tquery/split-self-validate
---


## What

Before PR 3133, `topo::split` of `brick(0..1.5, 0..1, 0..1) ∪
brick(1.2..1.3, −1..2, 0.5..3)` by the plane y + z = 2 returned two
halves carrying a zero-width slit:
- the section face ran out along y = z = 1 and back;
- the Below half held two coincident vertices at each point along the
  slit.

The slit came from the pinch lane's mirrored rerun
(`splitting/mod.rs`). PR 3133's join now refuses it as
`SplitJoinError::SectionSpur`.

**Which tiers refuse those halves** (measured by PR 3133's review):
- tier-2 `validate_closed`, `validate_geometric` and the
  pseudomanifold validation all refuse them;
- tier-1 `validate` passes them. That is by design: tier 1 checks
  combinatorial consistency, not geometric degeneracy.

## The gap

`split` runs no tier on the bodies it returns. The degenerate halves
left the op as a success and surfaced only downstream, as
`NamingError::Duplicate` in editor-core's name emission, which pointed
at the wrong module.

A tier-2 or geometric check on `split`'s outputs, or a debug-assertion
one, would have caught this at the op that made the body. Unmeasured:
its cost on the split suites.

## Found by

The sweep for EMIT's `split-section-face-keeps-a-zero-area-spur-along-a-tangent-edge`,
corrected by PR 3133's review (m4).

## Re-homed to TQUERY, 2026-09-24 (ATREST orchestrator)

Moved from `work/atrest/` by `git mv`, id and body unchanged. The at-rest
tiers answered correctly here — tier 2 and every geometric door refuse
the spurred halves, and tier 1 passing them is its charter. What the
row asks for is a validation tier run by `split` ON ITS OWN OUTPUTS,
and `crates/topo/src/split.rs` is TQUERY's ground: whether an op
validates what it returns, and at what cost to the split suites, is the
op's posture to decide, not the validator's.

## Measured, 2026-10-02 (TQUERY lane, `tquery/split-self-validate`)

Instrumented `split` locally (not committed) to run a door on each
returned half and log its time and verdict, over `cargo nextest run
--release -p topo -p sweep` (3853 rows; 113 of them call `split`, 573
halves checked). The box was shared (load 11–17 on 4 cores).

| door on each half | total over the suite | median / p95 / max per half | halves refused |
|---|---|---|---|
| tier 2 `validate_closed` | 12 ms (2 runs: 12, 12) | 0.02 / 0.03 / 0.1–0.5 ms | 0 |
| `validate_geometric` (`AtRestPolicy::gate_at_rest`) | 0.76–0.96 s (2 runs) | 0.52–0.56 / 6.6–7.5 / 24–27 ms | 8 |
| pseudomanifold, no contacts (`gate_at_rest_kept` + `gate_at_rest_declared`) | 0.82–1.19 s (3 runs) | 0.56–0.67 / 5.8–10.7 / 20–24 ms | 57 |

Suite wall time ran 94–135 s with no check at all and the same with
each door; the box's noise is ~40 s, so no wall delta is readable, and
the summed check time (above) is the cost.

What the refusals are (each half's operand checked at the same door):
- **geometric, 8:** all on operands that fail the door themselves —
  Euler-op fixtures carrying scaffold edges (`ScaffoldAtRest`) in
  `review_m3_pr3_rings` and `surgery::tests`. Split takes such operands
  today, so an always-on tier-3 gate would start refusing them.
- **pseudomanifold, 45 more on operands that pass:**
  - 37 are the designed pinch halves (`m3_pr3_split::notched_block_end_to_end`,
    `review_m3_pr3_bob`, `review_m3_pr6`, the BOOL1 notch rows): pieces
    meeting along a tip line through distinct vertex copies, refused as
    undeclared `VertexVertex` contacts. `split` returns no
    `ContactRecords`, so this door cannot pass its pinch halves
    (`split-halves-have-no-contact-records-so-no-pseudomanifold-self-check`).
  - 8 are CLEAVE's `split-pairs-curved-face-crossings-across-the-wrong-arc`
    halves (`split_section_rings::a_clockwise_section_nothing_places_keeps_its_face`,
    `pis_arc_capped_poses::every_tilted_cut_wall_reads_its_truth`),
    refused as `EdgeFaceOverlap`: the census sees the cancelling 2-gons
    tier 3 passes.
- **tier 2: none.** No currently green row returns a half that is not
  a closed solid.

## Disposition

Tier 2 costs ~20 µs a half and refuses nothing green, and it is the
gate the boolean already runs on its result (`boolean::ops::gate`,
typed `BooleanError::ResultInvalid`). Split's own finish already
refuses typed for its kernel-bug nets (`TornComponent`,
`NestingContradiction`). The operand is never validated: the
reduction's one tier-2 refusal is an empty outer loop on a face that
rule (a) measures at an ON vertex (`Reduce/CorruptOperand`, via
`rules::face_extent`). So an invalid side can be reached by input and
owes a typed refusal (D2 row 5 (ii)) rather than a debug assertion. Each run of `split_direct` now
gates both sides at tier 2 and refuses `SplitFinishError::ResultInvalid`.
The original spurred halves fail tier 2 (the review's measurement
above), so this gate alone would have turned the mirrored run's slit
into the direct run's `DegenerateSection`, as `SectionSpur` now does.
Tier 3 and the pseudomanifold door stay out, for the reasons in the
refusal list.
