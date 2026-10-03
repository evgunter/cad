# PIN — the plan

The boolean lane's preconditions, probes and census premises that nothing holds to account.

Opened 2026-09-20 by REACH's priority-seam cut (`work/README.md`,
Track size). Nothing dispatched yet.

## The slate

**15.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P3 | `D284` | E | boolean/join.rs's two Err(_) wildcards classify unnamed SectionError variants as Desync |
| P3 | `S234` | E | Add per-door rows that execute the box roster's direction column instead of reciting it |
| P3 | `axial-radial-takes-an-unchecked-unit-axis` | E | implicit.rs axial_radial takes a unit axis it does not check — carrier-derived, so no caller holds the witness yet |
| P3 | `boolean-predicates-read-a-direction-as-unit-by-prose` | E | three boolean predicates read a direction as unit by prose — contfp's plane normal, the germ facing sense, the sector directions |
| P3 | `boolean-smooth-arm-rebuilds-line-carrier` | E | boolean's smooth-arm stale-flat branch rebuilds line carriers via line_between instead of restating the certified carrier |
| P3 | `census-at-rest-two-boolean-lane-premises` | D | Census at rest - two more boolean-lane-derived premises the face rung leaves standing |
| P3 | `edge-chord-len-defaults-to-one-metre` | E | edge_chord_len's None defaults to a 1 m arm at two plane-identity sites |
| P3 | `flush-pair-relation-has-no-caller` | E | The planar flush-pair door has no in-tree caller since the detector widened |
| P3 | `join-probe-charts-hand-roll-the-across-axis-reference` | E | join's probe charts hand-roll an across-axis reference instead of calling Vec3::orthonormal_basis |
| P3 | `point-in-solid-refusal-names-faces-zero` | E | point_in_solid's body-scoped refusals carry faces[0] under a payload doc that says 'the face being tested' |
| P3 | `r2-union-wall-probe-only-prints` | E | r2_the_two_union_walls_on_my_operands prints four union refusals and asserts none — its stated claim about operand order is unchecked |
| P3 | `rows-do-not-cross-a-boolean-remap` | E | declaration rows do not cross a boolean's key remap (unreachable today: an instance carrying one is multi-solid) |
| P4 | `boolean-mod-doc-links-a-feature-gated-variant` | E | boolean/mod.rs:31 links SweepStrategy::Idealized, a feature-gated variant — rustdoc with CI's lints errors on a default build |
| P4 | `kernel-verbs-teapot-paragraph-predates-the-canal` | E | docs/KERNEL-VERBS.md's teapot paragraph names two wrong boolean pairs and a spout the scene no longer builds |

## Order

**By cost, not by subject.** Thirteen of the fourteen rows are class
`E` and several are one commit. Take them as drive-bys where you are
already in the file (`work/README.md`, "The tracker is not
comprehensive") and dispatch whatever is left as one or two batched
units rather than fourteen PRs.

The two that are worth their own read are
`r2-union-wall-probe-only-prints` (a probe that prints four union
refusals and asserts none, so it has never been able to fail) and
`D284` (two `Err(_)` wildcards classifying every unnamed
`SectionError` as `Desync`, which is a misdiagnosis rather than a
gap). `census-at-rest-two-boolean-lane-premises` is the only `D`.

## Review posture

OPEN, for this program's first dispatch. Nothing here is a kernel
unit in v7's sense — the rows are guards, probes and preconditions —
so a batched style review is the likely answer, on the S-TCOST
posture. The first orchestrator confirms it.
