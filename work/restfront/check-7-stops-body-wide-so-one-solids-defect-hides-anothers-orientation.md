---
id: check-7-stops-body-wide-so-one-solids-defect-hides-anothers-orientation
kind: issue
title: tier 3's early stops are body-wide, so one solid's structural finding hides another solid's inside-out finding in the same body
status: open
opened: 2026-09-28
priority: P3
cost: M
refs: [product-per-part-gate-counts-solids-but-gates-sources]
---


Found by the GATHER designer pair weighing
`work/gather/product-per-part-gate-counts-solids-but-gates-sources.md`
(2026-09-28), and demonstrated by one of them with a scratch probe.

## The finding

`topo::validate_geometric` stops early at three points, each over the
WHOLE body rather than per solid (`crates/topo/src/validate.rs`):

- `structural_declared_via` runs `validate_closed(body)?` first, so a
  tier-1/2 finding anywhere means no tier-3 check runs on any solid;
- a `Band` failure stops everything;
- check 7 (each solid's signed volume, `check7_subjects`) runs only when
  checks 1-6, 8 and 9 found nothing ANYWHERE in the body: the
  `if errors.is_empty()` in `tier3_local_checks_marked`, and the
  composed door's `?` before the certified check 7.

`check7_subjects` is per solid, but it is never reached if any other
solid failed an earlier check. Probe at f64 on graft-built aggregates
(`geometric_cube` + `describe_as_intersections`, `transform_rigid`,
`graft_disjoint_all`):

| body | findings |
|---|---|
| cube A, one face's sense flipped | `[LoopRoleInverted]` |
| cube B, inside out (`revert`) | `[NegativeVolume]` |
| A + B grafted | `[LoopRoleInverted]` only; B's defect is not reported |
| B + a clean cube | `[NegativeVolume]` |

So a multi-solid body's refusal omits other solids' orientation
defects: a user repairs A, re-runs, and only then learns of B. This
reaches every multi-solid refusal (the product gather's, `assemble`'s,
STEP import's), and it makes "tier 3 is a LOCAL battery" (the
`product.rs` module doc and the designers' first premise) true of what
is checked but not of what is reported.

## What a fix has to respect

The tier-1/2 stop must stay body-wide: a broken arena cannot be split
into solids. The check-7 gate could be per solid (run a solid's check 7
when THAT solid's checks 1-6, 8, 9 are clean). Not measured: whether
`validate_pseudomanifold` (tier 3', the door `assemble` uses) has the
same stop.

The product gather does not wait on this: its refusal re-gates each
source on failure, which recovers the dropped findings regardless.
