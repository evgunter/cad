---
id: SYM-1
kind: unit
title: the profile inside the normal form: where the E12 tier's time goes on the M10-3 slab, and what freezes
status: closed
opened: 2026-09-13
branch: sym/1-profile
refs: [symbolic-tier-costs-95-percent-of-the-m10-3-drive, derived-frame-placement-freezes-on-the-symbolic-lane]
closed: 2026-09-14
pr: 2530
---


## What

The first work in `symbolic-tier-costs-95-percent-of-the-m10-3-drive`:
the profile inside `geom_core::sym` on the M10-3 slab — which of the
three candidate cost centres (degree growth from carried denominators,
the coefficient ring past `i128`, term storage) dominates, with numbers
from callgrind and from an in-tree feature-gated profile; the freeze
and promotion population read off the counters; the method recorded so
a later unit can take a hosted before/after. No fix in this unit: it
moves nothing the tier decides, so it is outside the A/B experiment
(one review, no ordinal). Spec: `docs/SYM-1-SPEC.md`. The same
instrument reads the freeze mechanism
`derived-frame-placement-freezes-on-the-symbolic-lane` diagnosed, on
the slab and the plate, which is why that row is a ref.
