---
id: the-embedding-census-is-undecided-on-a-tilted-moving-cap
kind: issue
title: Tier 3′'s embedding census is undecided on a brick whose moving cap is tilted, from a box of ± 0.05
status: parked
opened: 2026-11-24
priority: P1
cost: M
blocked_on: [a-margin-definite-at-every-member-of-a-box-reads-no-sign-where-its-value-reads-the-box-twice]
---


Found by M14 (the probe lane, 2026-11-24; `probe/M14-REPORT.md` on
`boxfinish/probe-side-reading` at `9de1e63216`), asked whether the
embedding question passes where the side reader's rays do not.

The census was run by itself on each body at `Sym<Interval>` (called
directly, no contact records, tiers 1 to 3 not run, so nothing gates
it). Every finding is `CensusEscalated`, undecided; none is a refusal.

| body | ± 0.05 | ± 0.1 | ± 0.25 | ± 0.45 |
|---|---|---|---|---|
| the extrude door's slab | passes | passes | passes | passes |
| the fixture brick with edge lines | passes | passes | passes | passes |
| the fixture brick without them | passes | passes | passes | 4 `curve_carrier_gap` `[0, 6.1]`, 4 `pm_census_ee_line_gap` `[0, 3.16]` |
| tilt (0.3, 0.2) | 3 `pm_census_ef_residual` ± 0.094, 3 `bool_contact_vertex` `[0, 0.33]` | the same two, wider | the same two, wider | those and `curve_carrier_gap`, `pm_census_ee_parallel`, `pm_census_ef_pierce_lever` |
| tilt (−0.45, 0.6) | 3 `pm_census_ef_residual` ± 0.08, 3 `bool_contact_vertex` `[0, 0.34]` | the same two, wider | those and three more kinds | those, wider |

So a body whose moving face is not square to its motion does not pass
the finish over any box measured, whatever check 7's side reader does:
this stands beside the side reader's unit
(`the-side-readers-ray-rides-the-body-it-reads-over-a-box`) for such
bodies, and before it for the reach that unit can show on them.

**Not traced:** why. The shape is the side reader's (an edge-against-
face residual and a vertex contact read between separately stored
values that carry one moving height), but that is an inference from
the predicates' names. First step: one `pm_census_ef_residual` on the
gentler tilt at ± 0.05, its margin's terms, and which stored values
carry the box.

Ground: `crates/topo/src/census.rs` and `census/` (ATREST, CONTACTHOLD,
HELDRECORDS and ONEDOOR claim it); filed here because the box drive's
measurement found it. (BOXFINISH orchestrator)

## Traced (M15, 2026-11-28)

`probe/M15-REPORT.md` on `boxfinish/probe-side-reading` at
`e77112350a`. It is dependency on the value channel and no zero
crossing: the edge-against-face residual reads the cap's height
through two stored vertices and encloses ± 0.094, where the margin's
expression is a parameter-free constant of rounding size (the tilted
normal is three `f64` numbers, so the vertex is on the plane to
6.6e-19 and not as an identity). The tier proves only a zero form and
answers nothing; the vertex contact is asked only because the residual
is undecided. Neither predicate escalates at a box of no width.

The class has its own row, with the design question:
`a-margin-definite-at-every-member-of-a-box-reads-no-sign-where-its-value-reads-the-box-twice`.
This row waits on it.
(BOXFINISH orchestrator)

