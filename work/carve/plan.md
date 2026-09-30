# CARVE — the plan

what a sweep verb builds and how it describes it

Re-scoped 2026-09-20 by CARVE's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

## The slate

**27.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `intersection-pair-order-is-unpinned-and-extrude-disagrees-with-itself` | D | EdgeDescription::Intersection's (s1, s2) order is unpinned, and extrude writes it one way on cap rims and another on struts |
| P0 | `loft-v-parameterization-is-the-first-strips-so-a-rolled-section-changes-the-body` | H | loft_geometry takes the whole surface's v from the first strip, so a section rolled about its own normal builds a different body |
| P0 | `self-closed-link-sharing-its-vertex-records-two-junctions` | H | walk_chains records two junctions at one vertex and closes the chain when a self-closed link shares its vertex with one other requested link |
| P0 | `self-overlapping-spines-build-and-validate` | H | A loft or sweep whose spine revisits itself (a planar arc past a full turn) builds a self-overlapping body and every validation tier says Ok |
| P0 | `skin-coincident-section-check-is-an-unbanded-f64-compare` | H | skin.rs refuses coincident loft sections by a bare f64 strict comparison (params[j-1] < params[j] → DegenerateSection) — per-pair and named, but unbanded |
| P0 | `two-section-loft-with-an-inverted-top-normal-builds` | H | A two-section loft whose top section's plane normal points DOWN (against the stacking) builds and tier 3 says Ok — nothing checks the last section's normal against the stacking direction |

## Order

`self-overlapping-spines-build-and-validate` and
`two-section-loft-with-an-inverted-top-normal-builds` first, together:
both are bodies the kernel BUILDS and validates when it should refuse,
and a shared diagnosis is likely — each is a spine or section
orientation the loft path never asks about.

Then `loft-v-parameterization-is-the-first-strips-so-a-rolled-section-changes-the-body`,
which is the same family seen from the parameterisation side, and
`skin-coincident-section-check-is-an-unbanded-f64-compare`, which is a
Q1 violation on its own terms: a bare `f64` strict comparison deciding
a topological question is the unmargined predicate the design forbids.

## Review posture

OPEN, for this program's first dispatch. CARVE inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.
