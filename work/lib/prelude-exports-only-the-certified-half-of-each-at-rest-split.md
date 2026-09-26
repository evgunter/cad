---
id: prelude-exports-only-the-certified-half-of-each-at-rest-split
kind: issue
title: The pncad prelude exports only the certified half of each certified/_structural door pair
status: open
opened: 2026-09-21
priority: P4
cost: E
---

## Finding

`crates/pncad/src/prelude.rs` re-exports `validate_pseudomanifold`,
`validate_geometric` and `mass_properties` (and the rest of the
validator ladder) but none of their `_structural` twins
(`validate_pseudomanifold_structural`, `validate_geometric_structural`,
`contact_marks_structural`, `mass_properties_structural`,
`classify_shells_structural` — `topo` exports all five). Since LANE-1
(PR 3010) the certified names are bounded on `CertifiedBounds`, so a
facade consumer at a scalar without certification rights (a `Dual`)
has no at-rest or measurement door through the prelude at all; it
must reach into `pncad::topo`. Raised by the LANE-1 R1 review as a
prelude asymmetry; the lane noted it and left the export list to LIB,
whose curated `pub use` lists `test_binding_census.py` measures.

## What to do

Decide whether the facade's contract includes a non-certifying
scalar. If it does, export the `_structural` half beside the certified
one (and let the binding census count the new rows); if it does not,
say so at the prelude's section 5 so the asymmetry reads as a choice.

## The count, re-taken (2026-09-25, ATREST-10)

The five `_structural` twins above are ten on main at ATREST-10 (PR
#3227): the eight at-rest doors in `validate.rs`'s door roster (every
certified door's twin, `_certificate` and `_declared` forms included —
two of them added by that unit) and `props`' two measurement twins. The
prelude exports none of them.
